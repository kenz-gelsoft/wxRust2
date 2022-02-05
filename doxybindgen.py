import xml.etree.ElementTree as ET
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

class RustClassBinding:
    def __init__(self, model):
        self.__model = model

    def ffi_methods(self):
        template = '''\

        // CLASS: %s
        type %s;'''
        yield template % (
            self.__model.name,
            self.__model.name
        )
        indent = ' ' * 4 * 2
        for method in self.__model.methods:
            binding = RustMethodBinding(method)
            for line in binding.in_rust():
                yield '%s%s' % (indent, line)

    def ffi_ctors(self):
        indent = ' ' * 4 * 2
        for ctor in self.__model.ctors():
            for line in ctor.ffi_lines():
                yield '%s%s' % (indent, line)

    def safer_binding(self):
        rs_template = '''\
// %s
wx_class! { %s(%s) impl
    %sMethods
}'''
        yield rs_template % (
            self.__model.name,
            self.__model.unprefixed(),
            self.__model.name,
            self.__model.unprefixed(),
        )
        for chunk in self._ctors_for_rs():
            yield chunk
        for chunk in self._methods_for_rs():
            yield chunk

    def _ctors_for_rs(self):
        yield 'impl %s {' % (self.__model.unprefixed(),)
        for ctor in self.__model.ctors():
            yield ctor.for_rs()
        yield '}'

    def _methods_for_rs(self):
        yield 'trait %sMethods: WxRustMethods {' % (self.__model.unprefixed(),)
        for method in self.__model.methods:
            if method.is_ctor:
                continue
            yield method.for_rs()
        yield '}\n'

class CxxClassBinding:
    def __init__(self, model):
        self.__model = model

    def ctors_for_h(self):
        yield '// CLASS: %s' % (self.__model.name,)
        for ctor in self.__model.ctors():
            yield ctor.for_h()
        yield ''
    
    def ctors_for_cc(self):
        yield '// CLASS: %s' % (self.__model.name,)
        for ctor in self.__model.ctors():
            yield ctor.for_cc()
        yield ''

class Class:
    def in_xml(xmlfile, blocklist):
        tree = ET.parse(xmlfile)
        root = tree.getroot()
        for cls in root.findall(".//compounddef[@kind='class']"):
            yield Class(cls, blocklist)

    def __init__(self, e, blocklist):
        self.name = e.findtext('compoundname')
        self.methods = []
        self.__blocklist = blocklist.get(self.name)
        for method in e.findall(".//memberdef[@kind='function']"):
            m = Method(self, method)
            if not m.is_public:
                continue
            self.methods.append(m)

    def unprefixed(self):
        return self.name[2:]

    def ctors(self):
        for method in self.methods:
            if method.is_ctor:
                yield method
    
    def blocks(self, name):
        if self.__blocklist is None:
            return False
        return name in self.__blocklist

class RustMethodBinding:
    def __init__(self, model):
        self.__model = model
    
    def in_rust(self):
        body = '%sfn %s(%s)%s;' % (
            self.__model.unsafe_or_not(),
            self.__model.name,
            self.__model.rust_params(),
            self._returns_or_not(),
        )
        suppressed = self.__model.suppressed_reason()
        if suppressed:
            return ['// %s: %s' % (suppressed, body)]
        lines = [body]
        overload = self.__model.rename()
        if overload:
            lines.insert(0, overload)
        # print(lines)
        return lines

    def _returns_or_not(self):
        returns = self.__model.returns.in_rust()
        if returns in ['void', '']:
            returns = ''
        else:
            returns = ' -> %s' % (returns,)
        return returns
    

class Method:
    def __init__(self, cls, e):
        self.is_public = e.get('prot') == 'public'
        self.__is_static = e.get('static') == 'yes'
        self.returns = CxxType(''.join(e.find('type').itertext()))
        self.__class = cls
        self.name = e.findtext('name')
        self.__index = self._overload_index()
        self.is_ctor = self.name == cls.name
        self.__is_dtor = self.name.startswith('~')
        const = e.get('const') == 'yes'
        if self.is_ctor:
            self.returns = SelfType(cls.name, const, ctor_retval=True)
        self.__self_param = Param(SelfType(cls.name, const), 'self')
        self.__params = []
        for param in e.findall('param'):
            ptype = ''.join(param.find('type').itertext())
            pname = param.findtext('declname')
            self.__params.append(Param(CxxType(ptype), pname))

    def _overload_index(self):
        return sum(m.name == self.name for m in self.__class.methods)

    def _is_method_call(self):
        return not (self.__is_static or self.is_ctor)

    def rust_params(self, with_ffi=False, binding=False):
        params = self.__params.copy()
        if self._is_method_call():
            params.insert(0, self.__self_param)
        return ', '.join(p.in_rust(with_ffi, binding) for p in params)
    
    def _cxx_params(self):
        return ', '.join(p.in_cxx() for p in self.__params)
    
    def _call_params(self):
        return ', '.join(p.name for p in self.__params)

    def rename(self):
        if self.__index == 0:
            return ''
        return '#[rust_name = "%s"]' % (self._overload_name(),)

    def _overload_name(self):
        name = self.name
        if self.is_ctor:
            name = 'New%s' % (self.__class.unprefixed(),)
        index = self.__index
        if self.__index == 0:
            index = ''
        return '%s%s' % (name, index)
    
    def unsafe_or_not(self):
        return 'unsafe ' if self._uses_ptr_type() else ''
    
    def _uses_ptr_type(self):
        return any(p.type.is_ptr() for p in self.__params)

    def suppressed_reason(self, suppress_ctor=True):
        if suppress_ctor and self.is_ctor:
            return 'CTOR'
        if self.__is_dtor:
            return 'DTOR'
        if self._uses_unsupported_type():
            return 'CXX_UNSUPPORTED'
        if self.__class.blocks(self.name):
            return 'BLOCKED'
        return None
    
    def _uses_unsupported_type(self):
        if self.returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.__params)

    def ffi_lines(self):
        rs_template = '%sfn %s(%s) -> *mut %s;'
        lines = [rs_template % (
            self.unsafe_or_not(),
            self._overload_name(),
            self.rust_params(),
            self.__class.name,
        )]
        overload = self.rename()
        if overload:
            lines.insert(0, overload)
        return lines

    def for_rs(self):
        suppress = self.suppressed_reason(suppress_ctor=False)
        if suppress:
            return '// %s: fn %s()' % (suppress, self.name)
        rs_template = '''\
    %sfn %s(%s)%s {
        %s
    }'''
        unprefixed = self.__class.unprefixed()
        call = '%s(%s)' % (
            prefixed(self._overload_name(), with_ffi=True),
            self._call_params(),
        )
        if self._is_method_call():
            call = 'self.pinned::<ffi::%s>().as_mut().%s(%s)' % (
                self.__class.name,
                self._overload_name(),
                self._call_params(),
            )
        returns_or_not = ''
        if not self.returns.is_void():
            returns_or_not = ' -> %s' % (self.returns.in_rust(with_ffi=True),)
        is_method = self._is_method_call()
        return rs_template % (
            '' if is_method else 'pub ',
            self._rust_method_name(),
            self.rust_params(with_ffi=True, binding=True),
            returns_or_not,
            self._wrap_if_unsafe(
                self._wrap_return_type(
                    unprefixed, call
                )
            ),
        )

    def _wrap_return_type(self, type, body):
        if self.is_ctor:
            return '%s(%s)' % (type, body)
        return body

    def _rust_method_name(self):
        method_name = self.name
        if self.is_ctor:
            method_name = 'new'
        if self.__index > 0:
            method_name += str(self.__index)
        return method_name


    def _wrap_if_unsafe(self, t):
        if self._uses_ptr_type():
            return 'unsafe { %s }' % (t,)
        return t

    def for_h(self):
        body = '%s *%s(%s);' % (
            self.name,
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
    
    def in_rust(self, with_ffi, binding):
        if binding and isinstance(self.type, SelfType):
            return '&self'
        return '%s: %s' % (
            self.name,
            self.type.in_rust(with_ffi)
        )
    
    def in_cxx(self):
        return '%s %s' % (self.type.in_cxx(), self.name)
    
    def is_self(self):
        return self.name == 'self'

class SelfType:
    def __init__(self, s, const, ctor_retval=False):
        self.type = s
        self.const = const
        self.__ctor_retval = ctor_retval

    def in_rust(self, with_ffi=False):
        t = self.type
        if self.__ctor_retval:
            return t[2:]
        t = prefixed(t, with_ffi)
        if self.const:
            return '&%s' % (t)
        return 'Pin<&mut %s>' % (t,)
    
    def not_supported(self):
        return False
    
    def is_void(self):
        return False

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
            mut = 'mut ' if self.__is_mut else ''
            if ref.startswith('*') and not self.__is_mut:
                mut = 'const '
        if ref.startswith('&') and self.__is_mut:
            return 'Pin<&mut %s>' % (t,)
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
    
    def is_void(self):
        if self.is_ptr():
            return False
        return self.__typename == 'void'

RUST_PRIMITIVES = [
    'bool',
    'f64',
    'i32',
    'i64',
    'u8',
]
def prefixed(t, with_ffi=False):
    if t in RUST_PRIMITIVES:
        return t
    elif with_ffi:
        t = 'ffi::%s' % (t,)
    return t

