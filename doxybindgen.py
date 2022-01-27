import re

CXX2CXX = {
    'long': 'int32_t',
}

CXX2RUST = {
    'double': 'f64',
    'int': 'i32',
    'long': 'i32',
    'unsigned int': 'u32',
    'wxByte': 'u8',
    'wxCoord': 'i32',
    'wxWindowID': 'i32',
}

class Class:
    def __init__(self, e):
        self.name = e.findtext('compoundname')
        self.methods = []
        for method in e.findall(".//memberdef[@kind='function']"):
            m = Method(self, method)
            if not m.is_public:
                continue
            self.methods.append(m)
    
    def print_ffi_methods(self, f):
        indent = ' ' * 4 * 2
        print(file=f)
        print('%s// CLASS: %s' % (indent, self.name),
                file=f)
        print('%stype %s;' % (indent, self.name),
                file=f)
        for method in self.methods:
            for line in method.in_rust():
                print('%s%s' % (indent, line),
                        file=f)

    def print_ffi_ctors(self, f):
        indent = ' ' * 4 * 2
        for ctor in self._ctors():
            for line in ctor.ffi_lines():
                print('%s%s' % (indent, line),
                        file=f)

    def print_safer_binding(self, f):
        rs_template = '''\
// %s
wx_class! { %s(%s) impl
}'''
        without_wx = self.name[2:]
        print(rs_template % (
            self.name,
            without_wx,
            self.name,
        ), file=f)
        self.print_ctors_to_rs(f)

    def print_ctors_to_rs(self, f):
        without_wx = self.name[2:]
        print('impl %s {' % (without_wx,),
                file=f)
        for ctor in self._ctors():
            print(ctor.for_rs(), file=f)
        print('}', file=f)
        print(file=f)

    def print_ctors_to_h(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for ctor in self._ctors():
            print(ctor.for_h(), file=f)
        print(file=f)
    
    def print_ctors_to_cc(self, f):
        print('// CLASS: %s' % (self.name,),
                file=f)
        for ctor in self._ctors():
            print(ctor.for_cc(), file=f)
        print(file=f)

    def _ctors(self):
        for method in self.methods:
            if method.is_ctor:
                yield method

BLOCKLIST = {
    'wxObject': [
        'operator delete',
    ],
    'wxEvtHandler': [
        'AddFilter',
        'Bind',
        'CallAfter',
        'GetClientData',
        'RemoveFilter',
        'SetClientData',
        'Unbind',
    ],
    'wxWindow': [
        # TODO: treat long correctly
        'Create',
        # wxWindowBase's methods
        'AddChild',
        'FindFocus',
        'FindWindow',
        'FindWindowById',
        'FindWindowByLabel',
        'FindWindowByName',
        'FromDIP',
        'GetCapture',
        'GetExtraStyle',
        'GetWindowStyle',
        'GetWindowStyleFlag',
        'IsDescendant',
        'IsExposed',
        'NewControlId',
        'RemoveChild',
        'Reparent',
        'SetExtraStyle',
        'SetSize',
        'SetWindowStyle',
        'SetWindowStyleFlag',
        'ToDIP',
        'UnreserveControlId',
        'UpdateWindowUI',
    ],
    'wxControl': [
        # TODO: treat long correctly
        'Create',
    ],
    'wxButton': [
        # TODO: treat long correctly
        'Create',
    ],
}
class Method:
    def __init__(self, cls, e):
        self.is_public = e.get('prot') == 'public'
        self.__is_static = e.get('static') == 'yes'
        self.__returns = CxxType(''.join(e.find('type').itertext()))
        self.__class = cls
        self.__name = e.findtext('name')
        self.__index = self._overload_index()
        self.is_ctor = self.__name == cls.name
        const = e.get('const') == 'yes'
        self.__self_param = Param(SelfType(cls.name, const), 'self')
        self.__params = []
        for param in e.findall('param'):
            ptype = ''.join(param.find('type').itertext())
            pname = param.findtext('declname')
            self.__params.append(Param(CxxType(ptype), pname))

    def _overload_index(self):
        return sum(m.__name == self.__name for m in self.__class.methods)

    def _is_blocked(self):
        if self.__class.name not in BLOCKLIST:
            return False
        blocked_methods = BLOCKLIST[self.__class.name]
        if blocked_methods:
            return self.__name in blocked_methods
        return False

    def _rust_params(self, with_ffi=False):
        params = self.__params.copy()
        if not (self.__is_static or self.is_ctor):
            params.insert(0, self.__self_param)
        return ', '.join(p.in_rust(with_ffi) for p in params)
    
    def _cxx_params(self):
        return ', '.join(p.in_cxx() for p in self.__params)
    
    def _call_params(self):
        return ', '.join(p.name for p in self.__params)

    def in_rust(self):
        body = '%sfn %s(%s)%s;' % (
            self._unsafe_or_not(),
            self.__name,
            self._rust_params(),
            self._returns_or_not(),
        )
        suppressed = self._suppressed_reason()
        if suppressed:
            return ['// %s: %s' % (suppressed, body)]
        lines = [body]
        overload = self._rename()
        if overload:
            lines.insert(0, overload)
        # print(lines)
        return lines

    def _rename(self):
        if self.__index == 0:
            return ''
        return '#[rust_name = "%s"]' % (self._overload_name(),)

    def _overload_name(self):
        name = self.__name
        if self.is_ctor:
            without_wx = self.__class.name[2:]
            name = 'New%s' % (without_wx,)
        index = self.__index
        if self.__index == 0:
            index = ''
        return '%s%s' % (name, index)
    
    def _unsafe_or_not(self):
        return self._uses_ptr_type() and 'unsafe ' or ''
    
    def _uses_ptr_type(self):
        if self.__returns.is_ptr():
            return True
        return any(p.type.is_ptr() for p in self.__params)

    def _returns_or_not(self):
        returns = self.__returns.in_rust()
        if returns in ['void', '']:
            returns = ''
        else:
            returns = ' -> %s' % (returns,)
        return returns
    
    def _suppressed_reason(self):
        if self.is_ctor:
            return 'CTOR'
        if self._uses_unsupported_type():
            return 'CXX_UNSUPPORTED'
        if self._is_blocked():
            return 'BLOCKED'
        return None
    
    def _uses_unsupported_type(self):
        if self.__returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.__params)

    def ffi_lines(self):
        rs_template = '%sfn %s(%s) -> *mut %s;'
        lines = [rs_template % (
            self._unsafe_or_not(),
            self._overload_name(),
            self._rust_params(),
            self.__class.name,
        )]
        overload = self._rename()
        if overload:
            lines.insert(0, overload)
        return lines

    def for_rs(self):
        rs_template = '''\
    pub fn %s(%s) -> %s {
        %s
    }'''
        new_name = 'new'
        if self.__index > 0:
            new_name += str(self.__index)
        without_wx = self.__class.name[2:]
        body = '%s(ffi::%s(%s))' % (
            without_wx,
            self._overload_name(),
            self._call_params(),
        )
        return rs_template % (
            new_name,
            self._rust_params(with_ffi=True),
            without_wx,
            self._wrap_if_unsafe(body),
        )
    
    def _wrap_if_unsafe(self, t):
        if self._uses_ptr_type():
            return 'unsafe { %s }' % (t,)
        return t

    def for_h(self):
        body = '%s *%s(%s);' % (
            self.__name,
            self._overload_name(),
            self._cxx_params(),
        )
        return body
    
    def for_cc(self):
        cc_template = '''\
%s *%s(%s) {
    return new %s(%s);
}'''
        return cc_template % (
            self.__class.name,
            self._overload_name(),
            self._cxx_params(),
            self.__class.name,
            self._call_params(),
        )

class Param:
    def __init__(self, type, name):
        self.type = type
        self.name = name
    
    def in_rust(self, with_ffi=False):
        return '%s: %s' % (
            self.name,
            self.type.in_rust(with_ffi)
        )
    
    def in_cxx(self):
        return '%s %s' % (self.type.in_cxx(), self.name)

class SelfType:
    def __init__(self, s, const):
        self.type = s
        self.const = const

    def in_rust(self, with_ffi=False):
        t = self.type
        t = prefixed(t, with_ffi)
        if self.const:
            return '&%s' % (t)
        return 'Pin<&mut %s>' % (t,)

OS_UNSUPPORTED_TYPES = [
    'wxAccessible',
]
CXX_SUPPORTED_VALUE_TYPES = [
    'bool',
    'void',
]
class CxxType:
    def __init__(self, s):
        self.__srctype = s
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', s)
        self.__typename = None
        if matched:
            self.__is_mut = matched.group(1) is None
            self.__typename = matched.group(2).strip()
            self.__indirection = matched.group(3)
        if self.__indirection is None:
            self.__indirection = ''
    
    def in_cxx(self):
        if self.__srctype in CXX2CXX:
            return CXX2CXX[self.__srctype]
        return self.__srctype

    def in_rust(self, with_ffi=False):
        t = self.__typename
        if t in CXX2RUST:
            t = CXX2RUST[t]
        t = prefixed(t, with_ffi)
        ref = self.__indirection
        mut = ''
        if ref:
            mut = self.__is_mut and 'mut ' or ''
            if ref.startswith('*') and not self.__is_mut:
                mut = 'const '
        if ref.startswith('&') and self.__is_mut:
            return 'Pin<%smut %s>' % (ref, t)
        return '%s%s%s' % (ref, mut, t)
    
    def not_supported(self):
        if self.__typename in OS_UNSUPPORTED_TYPES:
            return True
        if not self._is_cxx_supported_value_type():
            return not self.__indirection
        return False
    
    def _is_cxx_supported_value_type(self):
        if self.__typename in CXX_SUPPORTED_VALUE_TYPES:
            return True
        if self.__typename in CXX2RUST:
            return True
        return False
    
    def is_ptr(self):
        return self.__indirection.startswith('*')

RUST_PRIMITIVES = [
    'i32',
    'i64',
]
def prefixed(t, with_ffi):
    if t in RUST_PRIMITIVES:
        return t
    if with_ffi:
        t = 'ffi::%s' % (t,)
    return t
