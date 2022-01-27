import re

cxx_to_cxx = {
    'long': 'int32_t',
}

cxx_to_rust = {
    'double': 'f64',
    'int': 'i32',
    'long': 'i32',
    'unsigned int': 'u32',
    'wxByte': 'u8',
    'wxCoord': 'i32',
    'wxWindowID': 'i32',
}

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

os_unsupported_types = [
    'wxAccessible',
]
cxx_unsupported_types = [
]
cxx_supported_value_types = [
    'bool',
    'void',
]
class CxxType:
    def __init__(self, s):
        self.origtype = s
        # print('parsing: |%s|' % (s,))
        matched = re.match(r'(const )?([^*&]*)([*&]+)?', s)
        self.basetype = None
        if matched:
            self.mut = matched.group(1) is None
            self.basetype = matched.group(2).strip()
            self.indirection = matched.group(3)
        if self.indirection is None:
            self.indirection = ''
    
    def in_cxx(self):
        if self.origtype in cxx_to_cxx:
            return cxx_to_cxx[self.origtype]
        return self.origtype

    def in_rust(self, with_ffi=False):
        t = self.basetype
        if t in cxx_to_rust:
            t = cxx_to_rust[t]
        t = prefixed(t, with_ffi)
        ref = self.indirection
        mut = ''
        if ref:
            mut = self.mut and 'mut ' or ''
            if ref.startswith('*') and not self.mut:
                mut = 'const '
        if ref.startswith('&') and self.mut:
            return 'Pin<%smut %s>' % (ref, t)
        return '%s%s%s' % (ref, mut, t)
    
    def not_supported(self):
        if self.basetype in os_unsupported_types:
            return True
        if self.basetype in cxx_unsupported_types:
            return True
        if not self._is_cxx_supported_value_type():
            return not self.indirection
        return False
    
    def _is_cxx_supported_value_type(self):
        if self.basetype in cxx_supported_value_types:
            return True
        if self.basetype in cxx_to_rust:
            return True
        return False
    
    def is_ptr(self):
        return self.indirection.startswith('*')

rust_primitives = [
    'i32',
    'i64',
]
def prefixed(t, with_ffi):
    if t in rust_primitives:
        return t
    if with_ffi:
        t = 'ffi::%s' % (t,)
    return t

