import xml.etree.ElementTree as ET
import copy, re

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
    'wxEllipsizeMode': 'i32',
    'wxWindowID': 'i32',
}

class Class:
    def in_xml(type_manager, xmlfile, config):
        tree = ET.parse(xmlfile)
        root = tree.getroot()
        for cls in root.findall(".//compounddef[@kind='class']"):
            yield Class(type_manager, cls, config)

    def __init__(self, type_manager, e, config):
        self.type_manager = type_manager
        self.name = e.findtext('compoundname')
        self.base = e.findtext('basecompoundref')
        self.methods = []
        config = config.get(self.name) or {}
        self.__blocklist = config.get('blocklist') or []
        self.config = config
        for method in e.findall(".//memberdef[@kind='function']"):
            m = Method(self, method)
            if not m.is_public:
                continue
            self.methods.append(m)

    def unprefixed(self):
        return self.name[2:]

    def is_blocked_method(self, name):
        return name in self.__blocklist
    
    def is_trivial(self):
        return self.name in CXX_TRIVIAL_EXTERN_TYPES

    def uses_shim_for(self, name):
        methods = self.config.get('use_shim') or []
        return name in methods


class Method:
    def __init__(self, cls, e):
        self.is_public = e.get('prot') == 'public'
        self.is_static = e.get('static') == 'yes'
        self.returns = CxxType(cls.type_manager, e.find('type'))
        self.cls = cls
        self.__name = e.findtext('name')
        self.overload_index = self._overload_index()
        self.is_ctor = self.__name == cls.name
        self.is_instance_method = not (self.is_ctor or self.is_static)
        self.const = e.get('const') == 'yes'
        if self.is_ctor:
            self.returns = RustType(cls.name, self.const, ctor_retval=True)
        self.params = []
        for param in e.findall('param'):
            ptype = CxxType(cls.type_manager, param.find('type'))
            pname = param.findtext('declname')
            self.params.append(Param(ptype, pname))

    def needs_shim(self):
        if self.is_blocked() or self.uses_unsupported_type():
            return False
        return True
    
    def uses_unsupported_type(self):
        if self.returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.params)

    def is_blocked(self):
        return self.cls.is_blocked_method(self.name(for_shim=False))

    def _overload_index(self):
        return sum(m.__name == self.__name for m in self.cls.methods)

    def name(self, for_shim, without_index=False):
        name = self.__name
        if for_shim:
            if self.is_ctor:
                name = 'New%s' % (self.cls.unprefixed(),)
            else:
                name = '_'.join((
                    self.cls.name,
                    name,
                ))
        if without_index:
            return name
        return self.overload_indexed(name)
    
    def overload_indexed(self, name):
        index = self.overload_index
        if index == 0:
            index = ''
        return '%s%s' % (name, index)

    def wrapped_return_type(self):
        if (self.is_ctor or
            self.returns_new() or
            self.returns.is_trivial()):
            return self.returns.typename
        else:
            return None

    def returns_new(self):
        if self.is_blocked():
            return False
        if self.returns.is_str():
            return True
        return self.returns.not_supported_value_type(check_generated=True)

    
class Param:
    def __init__(self, type, name):
        self.type = type
        self.name = camel_to_snake(name)
    
    def is_self(self):
        return self.name == 'self'
    
    def marshal(self):
        return self.type.marshal(self)

    def rust_ffi_ref(self, rename=None, is_mut_self=False):
        name = rename if rename else self.name
        as_mut_or_not = '.as_mut()' if self.is_self() else ''
        return '%s.pinned::<ffi::%s>()%s' % (
            name,
            self.type.typename,
            as_mut_or_not,
        )


class RustType:
    def __init__(self, s, const, ctor_retval=False):
        self.typename = s
        self.const = const
        self.__ctor_retval = ctor_retval
        self.generic_name = None

    def marshal(self, param):
        return None

    def in_rust(self, with_ffi=False, binding=False):
        t = self.typename
        if self.__ctor_retval:
            return t[2:]
        t = prefixed(t, with_ffi)
        ref = '&'
        mut = '' if self.const else 'mut '
        return self._pin_if_needed('%s%s%s' % (ref, mut, t))
    
    def _pin_if_needed(self, t):
        if t.startswith('&mut '):
            return 'Pin<%s>' % (t,)
        return t

    def in_cxx(self):
        t = '%s &' % (self.typename,)
        if self.const:
            t = 'const %s' % (t,)
        return t

    def is_ptr_to_binding(self):
        return False

    def is_trivial(self):
        return self.typename in CXX_TRIVIAL_EXTERN_TYPES

    def not_supported(self):
        return False

    def not_supported_value_type(self, check_generated=False):
        return False

    def is_void(self):
        return False

    def is_str(self):
        return self.typename == 'wxString'

OS_UNSUPPORTED_TYPES = [
    'wxAccessible',
]
CXX_SUPPORTED_VALUE_TYPES = [
    'bool',
    'void',
]
CXX_TRIVIAL_EXTERN_TYPES = [
    'wxPoint',
    'wxRect',
    'wxSize',
]


class TypeManager:
    def __init__(self):
        self.known_bindings = None
        pass

    def is_binding_type(self, name):
        assert self.known_bindings is not None
        return name in self.known_bindings


class CxxType:
    def __init__(self, manager, e):
        self.__manager = manager
        self.__srctype = ''.join(e.itertext())
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', self.__srctype)
        self.typename = None
        self.generic_name = None
        if matched:
            self.__is_mut = matched.group(1) is None
            self.typename = matched.group(2).strip()
            self.__indirection = matched.group(3)
        if self.__indirection is None:
            self.__indirection = ''
    
    def in_cxx(self):
        if self.__srctype in CXX2CXX:
            return CXX2CXX[self.__srctype]
        return self.__srctype
    
    def marshal(self, param):
        name = camel_to_snake(param.name)
        if self._is_const_ref_to_string():
            yield 'let %s = &crate::ffi::NewString(%s);' % (
                name,
                name,
            )
        if self._is_const_ref_to_binding():
            yield 'let %s = &%s;' % (
                name,
                param.rust_ffi_ref(),
            )
        if self.is_ptr_to_binding():
            yield 'let %s = match %s {' % (
                name,
                name,
            )
            yield '    Some(r) => Pin::<&mut ffi::%s>::into_inner_unchecked(%s),' % (
                self.typename,
                param.rust_ffi_ref(rename='r'),
            )
            yield '    None => ptr::null_mut(),'
            yield '};'

    def in_rust(self, with_ffi=False, binding=False):
        t = self.typename
        if binding:
            if self._is_const_ref_to_string():
                return '&str'
            if self._is_const_ref_to_binding():
                return '&%s' % (t[2:])
            if self.is_ptr_to_binding():
                return 'Option<&%s>' % (t[2:])
        if t in CXX2RUST:
            t = CXX2RUST[t]
        t = prefixed(t, with_ffi)
        ref = self.__indirection
        mut = ''
        if ref:
            mut = 'mut ' if self.__is_mut else ''
            if ref.startswith('*') and not self.__is_mut:
                mut = 'const '
        return self._pin_if_needed('%s%s%s' % (ref, mut, t))
    
    def _pin_if_needed(self, t):
        if t.startswith('&mut '):
            return 'Pin<%s>' % (t,)
        return t

    def is_ptr_to_binding(self):
        # TODO: consider mutability
        return (self.is_ptr() and
                self._is_binding_type() and
                not self.is_trivial())

    def _is_const_ref_to_string(self):
        return self._is_const_ref() and self.typename == 'wxString'

    def _is_const_ref_to_binding(self):
        return self._is_const_ref() and self._is_binding_type()

    def _is_const_ref(self):
        if self.__is_mut:
            return False
        return self.__indirection.startswith('&')

    def not_supported(self):
        if self.typename in OS_UNSUPPORTED_TYPES:
            return True
        return self.not_supported_value_type()
    
    def not_supported_value_type(self, check_generated=False):
        if check_generated and not self._is_binding_type():
            return False
        if self.is_str():
            return False
        if not self._is_cxx_supported_value_type():
            return not self.__indirection
        return False
    
    def _is_binding_type(self):
        return self.__manager.is_binding_type(self.typename)

    def _is_cxx_supported_value_type(self):
        if self.typename in CXX_SUPPORTED_VALUE_TYPES:
            return True
        if self.typename in CXX2RUST:
            return True
        if self.is_trivial():
            return True 
        return False
    
    def is_trivial(self):
        return self.typename in CXX_TRIVIAL_EXTERN_TYPES
        
    def is_ptr(self):
        return self.__indirection.startswith('*')
    
    def is_void(self):
        if self.is_ptr():
            return False
        return self.typename in ['void', '']
    
    def is_str(self):
        return self.typename == 'wxString'
    
    def make_generic(self, generic_name):
        self.generic_name = generic_name
        return (generic_name, self.typename[2:] + 'Methods')

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


def pascal_to_snake(pascal_case):
    def concat_caps(words):
        buf = ''
        for word in words:
            if len(word) == 1:
                buf += word
                continue
            if buf:
                yield buf
                buf = ''
            yield word
        if buf:
            yield buf
    words = re.findall(r'[A-Z][^A-Z]*', pascal_case)
    if words:
        snake_cased = '_'.join(w.lower() for w in concat_caps(words))
        return snake_cased
    return pascal_case


def camel_to_snake(camel_case):
    if camel_case is None:
        return None
    pascal_case = camel_case[0].upper() + camel_case[1:]
    return pascal_to_snake(pascal_case)

