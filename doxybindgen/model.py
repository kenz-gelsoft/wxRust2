from .constants import Enum
import xml.etree.ElementTree as ET
import re


CXX2RUST = {
    'double': 'c_double',
    'int': 'c_int',
    'long': 'c_long',
    'wxByte': 'c_uchar',
    'wxCoord': 'c_int',
    'wxEllipsizeMode': 'c_int',
    'wxItemKind': 'c_int',
    'wxWindowID': 'c_int',
}
CXX_PRIMITIVES = [
    'bool',
    'void',
]
RUST_PRIMITIVES = [
    'bool',
    'c_double',
    'c_int',
    'c_long',
    'c_uchar',
]


class Class:
    def in_xml(manager, xmlfile, config):
        tree = ET.parse(xmlfile)
        root = tree.getroot()
        for cls in root.findall(".//compounddef[@kind='class']"):
            yield Class(manager, cls, config)

    def __init__(self, manager, e, config):
        self.manager = manager
        self.name = e.findtext('compoundname')
        self.base = e.findtext('basecompoundref')
        self.enums = []
        self.methods = []
        config = config.get(self.name) or {}
        self.__blocklist = config.get('blocklist') or []
        self.config = config
        self.library = self._find_libname(e)
        for enum in e.findall(".//memberdef[@kind='enum']"):
            enum = Enum(enum)
            self.enums.append(enum)
        for method in e.findall(".//memberdef[@kind='function']"):
            m = Method(self, method)
            if not m.is_public:
                continue
            if m.is_virtual_override:
                continue
            self.methods.append(m)

    def _find_libname(self, e):
        library = self.config.get('library')
        if library:
            return library
        for ref in e.findall('./detaileddescription//ref'):
            if ref.get('refid').startswith('page_libs_'):
                return ref.text.lower()[2:]

    def unprefixed(self):
        return self.name[2:]

    def is_blocked_method(self, name):
        return name in self.__blocklist


class Method:
    def __init__(self, cls, e):
        self.is_public = e.get('prot') == 'public'
        self.is_static = e.get('static') == 'yes'
        self.returns = CxxType(cls.manager, e.find('type'))
        self.cls = cls
        self.__name = e.findtext('name')
        self.overload_index = self._overload_index()
        self.is_ctor = self.__name == cls.name
        self.is_instance_method = not (self.is_ctor or self.is_static)
        self.const = e.get('const') == 'yes'
        if self.is_ctor:
            self.returns = RustType(cls.name, self.const)
        self.params = []
        for param in e.findall('param'):
            ptype = CxxType(cls.manager, param.find('type'))
            pname = param.findtext('declname')
            self.params.append(Param(ptype, pname))
        is_virtual = e.get('virt') == 'virtual'
        is_override = e.find('reimplements') is not None
        self.is_virtual_override = is_virtual and is_override

    def suppressed_reason(self):
        if self.is_blocked():
            return 'BLOCKED'
        if self.__name.startswith('~'):
            return 'DTOR'
        if self.uses_not_supported_type():
            return 'NOT_SUPPORTED'
        return None

    def uses_not_supported_type(self):
        if self.returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.params)

    def is_blocked(self):
        return self.cls.is_blocked_method(self.name())

    def find_condition(self, conditions):
        for cond_name, condition in conditions.items():
            cond_list = self.cls.config.get(cond_name) or []
            if self.name() in cond_list:
                return condition
        return None

    def _overload_index(self):
        return sum(m.__name == self.__name for m in self.cls.methods)

    def name(self, for_ffi=False, without_index=False):
        name = self.__name
        if for_ffi:
            if self.is_ctor:
                name = 'new'
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

    def wrapped_return_type(self, allows_ptr):
        if (self.is_ctor or
            self.returns_new(allows_ptr) or 
            allows_ptr and self.returns.is_ptr_to_binding()):
            return self.returns.typename
        else:
            return None

    def returns_new(self, allows_ptr):
        if self.is_blocked():
            return False
        if self.returns.is_str():
            return True
        if self.returns.needs_new():
            return True
        return False

    def cxx_signature(self):
        items = []
        returns = self.returns.normalized()
        if returns:
            items.append(returns)
        items.append('%s(%s)' % (
            self.__name,
            ', '.join(p.type.normalized() for p in self.params)
        ))
        return ' '.join(items)

    def is_non_virtual_override(self, cls):
        if cls == self.cls:
            return False
        signature = self.cxx_signature()
        return any(m.cxx_signature() == signature for m in cls.methods)
    
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
        return '%s.as_ptr()' % (name,)


class RustType:
    def __init__(self, s, const):
        self.typename = s
        self.const = const
        self.generic_name = None

    def marshal(self, param):
        return None

    def in_rust(self, for_ffi=False):
        mut = 'const' if self.const else 'mut'
        return '*%s c_void' % (mut,)
    
    def in_cxx(self):
        t = '%s *' % (self.typename,)
        if self.const:
            t = 'const %s' % (t,)
        return t

    def is_ptr_to_binding(self):
        return False

    def not_supported(self):
        return False

    def needs_new(self):
        return False
    
    def is_void(self):
        return False

    def is_str(self):
        return self.typename == 'wxString'

    def normalized(self):
        return '%s%s*' % (
            'const ' if self.const else '',
            self.typename,
        )

OS_UNSUPPORTED_TYPES = [
    'wxAccessible',
]


class ClassInfo:
    def __init__(self, cls):
        self.cls = cls
        self.ancestors = None


class ClassManager:
    def __init__(self):
        self.__all = None
        self.__by_name = None

    def all(self):
        return (i.cls for i in self.__all)
    
    def in_lib(self, libname, generated):
        all_classes = self.all()
        if libname is None:
            return (cls for cls in all_classes if cls.library not in generated)
        return (cls for cls in all_classes if cls.library == libname)
    
    def by_name(self, name):
        info = self.__by_name.get(name)
        return info.cls if info else None

    def register(self, classes):
        self.__all = [ClassInfo(cls) for cls in classes]
        dict = {}
        for info in self.__all:
            dict[info.cls.name] = info
        self.__by_name = dict

    def is_binding_type(self, name):
        assert self.__by_name is not None
        return name in self.__by_name.keys()
    
    def ancestors_of(self, cls):
        info = self.__by_name.get(cls.name)
        if info.ancestors is None:
            info.ancestors = self._find_ancestors(cls)
        return info.ancestors

    def _find_ancestors(self, cls):
        base_classes = []
        current = cls
        while current:
            base_classes.append(current)
            current = self.by_name(current.base)
        return base_classes

    def is_a(self, cls, ancestor):
        return any(c.name == ancestor for c in self.ancestors_of(cls))


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
    
    def __hash__(self) -> int:
        return hash(self.normalized())

    def __eq__(self, other: object) -> bool:
        return self.normalized() == other.normalized()
    
    def __repr__(self) -> str:
        return '`%s`' % (self.normalized(),)
    
    def in_overload_name(self):
        if self.is_str():
            return 'str'
        t = self.typename
        if t == 'size_t':
            t = 'sz'
        elif t.startswith('wx'):
            t = t[2:]
        return t.lower()
    
    def in_cxx(self):
        if self.is_ref():
            const_or_not = '' if self.__is_mut else 'const '
            new_type = '%s%s *' % (
                const_or_not,
                self.typename,
            )
            return new_type
        return self.__srctype
    
    def marshal(self, param):
        name = camel_to_snake(param.name)
        if self._is_const_ref_to_string():
            yield 'let %s = wx_base::wx_string_from(%s);' % (
                name,
                name,
            )
        if self._is_const_ref_to_binding():
            yield 'let %s = %s;' % (
                name,
                param.rust_ffi_ref(),
            )
        if self.is_ptr_to_binding():
            yield 'let %s = match %s {' % (
                name,
                name,
            )
            yield '    Some(r) => %s,' % (
                param.rust_ffi_ref(rename='r'),
            )
            yield '    None => ptr::null_mut(),'
            yield '};'

    def in_rust(self, for_ffi=False):
        t = self.typename
        if not for_ffi:
            if self._is_const_ref_to_string():
                return '&str'
            if self._is_const_ref_to_binding():
                return '&%s' % (t[2:])
            if self.is_ptr_to_binding():
                return 'Option<&%s>' % (t[2:])
        if t in CXX2RUST:
            t = CXX2RUST[t]
        if self.__indirection:
            mut = 'mut' if self.__is_mut else 'const'
            return '*%s c_void' % (mut,)
        return prefixed(t, with_ffi=not for_ffi)
    
    def is_ptr_to_binding(self):
        # TODO: consider mutability
        return (self.is_ptr() and
                self._is_binding_type())

    def _is_const_ref_to_string(self):
        return self._is_const_ref() and self.typename == 'wxString'

    def _is_const_ref_to_binding(self):
        return self._is_const_ref() and self._is_binding_type()

    def _is_const_ref(self):
        if self.__is_mut:
            return False
        return self.is_ref()
    
    def is_ref(self):
        return self.__indirection == '&'

    def not_supported(self):
        if self.typename in OS_UNSUPPORTED_TYPES:
            return True
        if self.is_str():
            return False
        if self._is_value_type():
            return False
        if self.__indirection:
            return False
        if self.needs_new():
            return False
        return True
    
    def needs_new(self):
        return self._is_binding_type() and not self.__indirection
    
    def _is_binding_type(self):
        return self.__manager.is_binding_type(self.typename)

    def _is_value_type(self):
        if self.typename in CXX_PRIMITIVES:
            return True
        if self.typename in CXX2RUST:
            return True
        return False
        
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

    def normalized(self):
        return '%s%s%s' % (
            '' if self.__is_mut else 'const ',
            self.typename,
            self.__indirection,
        )


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

