from .model import Param, RustType, prefixed
import re

# Known, and problematic
RUST_KEYWORDS = [
    'move',
    'ref',
]

class RustClassBinding:
    def __init__(self, model):
        self.__model = model
        self.__methods = [RustMethodBinding(m) for m in model.methods]

    def cxx_auto_bound_methods(self, is_cxx):
        if is_cxx:
            yield ''
        yield '// CLASS: %s' % (
            self.__model.name,
        )
        if is_cxx:
            yield 'type %s;' % (
                self.__model.name,
            )
        for method in self.__methods:
            for line in method.cxx_auto_binding(is_cxx=is_cxx):
                yield line

    def safer_binding(self, classes):
        yield '// %s' % (
            self.__model.name,            
        )
        yield 'wx_class! { %s(%s) impl' % (
            self.__model.unprefixed(),
            self.__model.name,
        )
        yield ',\n'.join(self._ancestor_methods(classes))
        yield '}'
        for line in self._generate_impl_with_ctors():
            yield line
        for line in self._generate_trait_with_methods():
            yield line
    
    def _ancestor_methods(self, classes):
        for ancestor in self._find_ancestors(classes):
            yield '    %sMethods' % (
                ancestor[2:],
            )

    def _find_ancestors(self, classes):
        base_classes = []
        current = self.__model
        while current:
            base_classes.append(current.name)
            current = self._class_by_name(current.base, classes)
        return base_classes

    def _class_by_name(self, name, classes):
        for cls in classes:
            if cls.name == name:
                return cls
        return None

    def _generate_impl_with_ctors(self):
        yield 'impl %s {' % (self.__model.unprefixed(),)
        for ctor in self._ctors():
            for line in ctor.binding():
                yield '    %s' % (line,)
        yield "    pub fn none() -> Option<&'static Self> {"
        yield '        None'
        yield '    }'
        yield '}'

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)
    
    def _generate_trait_with_methods(self):
        indent = ' ' * 4 * 1
        base = self.__model.base
        if not base:
            base = '__WxRust'
        yield 'pub trait %sMethods: %sMethods {' % (
            self.__model.unprefixed(),
            base[2:],
        )
        for method in self.__methods:
            if method.is_ctor:
                continue
            for line in method.binding():
                yield '%s%s' % (indent, line)
        yield '}\n'

class RustMethodBinding:
    def __init__(self, model):
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__is_dtor = model.name.startswith('~')
        self.__is_instance_method = not (model.is_static or model.is_ctor)
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')
        self.__this_param = Param(RustType(model.cls.name, model.const), 'arg0')
        self.__generic_params = self._make_params_generic()
    
    def cxx_auto_binding(self, is_cxx):
        if not (is_cxx or self.__model.generates()):
            return
        with_this = not is_cxx and self.__model.returns_new()
        body = '%sfn %s(%s)%s;' % (
            self._unsafe_or_not(),
            self.__model.overload_name(
                without_index=True,
                cxx_name=is_cxx,
            ),
            self._rust_params(with_this=with_this),
            self._returns_or_not(is_cxx=is_cxx),
        )
        suppressed = self._suppressed_reason()
        if is_cxx and suppressed:
            yield '// %s: %s' % (suppressed, body)
            return
        overload = self._rename()
        if overload:
            yield overload
        yield body

    def _returns_or_not(self, is_cxx):
        if is_cxx:
            returns = self.__model.returns.in_rust()
        else:
            wrapped = self.__model.wrapped_return_type()
            if wrapped:
                returns = '*mut %s' % (wrapped,)
        if returns in ['void', '']:
            return ''
        else:
            return ' -> %s' % (returns,)
    
    def _returns_or_not_binding(self):
        if self.__model.returns_new():
            returns = self.__model.returns.in_rust()[2:]
        else:
            returns = self.__model.returns.in_rust(with_ffi=True)
        if self.__model.returns.is_void():
            return ''
        else:
            return ' -> %s' % (returns,)
    
    def _unsafe_or_not(self):
        return 'unsafe ' if self._uses_ptr_type() else ''
    
    def _rename(self):
        if self.__model.overload_index == 0:
            return ''
        return '#[rust_name = "%s"]' % (
            self.__model.overload_name(),
        )
    
    def _make_params_generic(self):
        generic_params = []
        for param in self.__model.params:
            if param.type.is_ptr_to_binding():
                count = len(generic_params)
                name = chr(ord('T') + count)
                generic_params.append(param.type.make_generic(name))
        return generic_params

    def binding(self):
        suppress = self._suppressed_reason(
            suppress_ctor=False,
            suppress_returns_new=False,
        )
        if suppress:
            yield '// %s: fn %s()' % (suppress, self.__model.name)
            return
        gen_params = ''
        if self.__generic_params:
            gen_params = '<%s>' % (
                ', '.join('%s: %s' % p for p in self.__generic_params),
            )
        yield '%sfn %s%s(%s)%s {' % (
            '' if self.__is_instance_method else 'pub ',
            self._rust_method_name(),
            gen_params,
            self._rust_params(with_ffi=True, binding=True),
            self._returns_or_not_binding(),
        )
        body_lines = list(self._binding_body())
        for line in self._wrap_if_unsafe(body_lines):
            yield '    %s' % (line,)
        yield '}'
    
    def _binding_body(self):
        for param in self.__model.params:
            marshalling = param.type.marshal(camel_to_snake(param.name))
            if marshalling:
                for line in marshalling:
                    yield '%s' % (line,)
        call = '%s(%s)' % (
            prefixed(self.__model.overload_name(), with_ffi=True),
            self._call_params(),
        )
        if self.__is_instance_method:
            self_param = 'self.pinned::<ffi::%s>().as_mut()' % (
                self.__model.cls.name,
            )
            if self.__model.returns_new():
                if self.__model.const:
                    self_param = '&' + self_param
                params = ', '.join([self_param, self._call_params()])
                call = 'ffi::%s(%s)' % (
                    self.__model.overload_name(),
                    params,
                )
            else:
                call = '%s.%s(%s)' % (
                    self_param,
                    self.__model.overload_name(),
                    self._call_params(),
                )
        yield self._wrap_return_type(call)
    
    def _call_params(self):
        return ', '.join(camel_to_snake(p.name) for p in self.__model.params)

    def _suppressed_reason(self, suppress_ctor=True, suppress_returns_new=True):
        if suppress_ctor and self.__model.is_ctor:
            return 'CTOR'
        if self.__is_dtor:
            return 'DTOR'
        if self.__model.is_static:
            # TODO: handle static methods specially
            return 'STATIC'
        if self._uses_unsupported_type():
            if self.__model.returns_new():
                if suppress_returns_new:
                    return 'GENERATED'
            else:
                return 'CXX_UNSUPPORTED'
        if self.__model.is_blocked():
            return 'BLOCKED'
        return None
    
    def _uses_unsupported_type(self):
        if self.__model.returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.__model.params)

    def _rust_method_name(self):
        method_name = pascal_to_snake(self.__model.name)
        if self.__model.is_ctor:
            method_name = 'new'
        if self.__model.overload_index > 0:
            method_name += str(self.__model.overload_index)
        if method_name in RUST_KEYWORDS:
            method_name += '_'
        return method_name
    
    def _rust_params(self, with_ffi=False, binding=False, with_this=False):
        params = self.__model.params.copy()
        if self.__is_instance_method:
            if with_this:
                params.insert(0, self.__this_param)
            else:
                params.insert(0, self.__self_param)
        return ', '.join(self._rust_param(p, with_ffi, binding) for p in params)

    def _rust_param(self, param, with_ffi, binding):
        typename = param.type.in_rust(with_ffi, binding)
        if binding:
            if param.is_self():
                return '&self'
            elif param.type.generic_name:
                typename = 'Option<&%s>' % (param.type.generic_name,)
        return '%s: %s' % (
            camel_to_snake(param.name),
            typename,
        )

    def _wrap_if_unsafe(self, lines):
        to_be_generated = lines
        if self._uses_ptr_type():
            to_be_generated = self._unsafe_lines(lines)
        for line in to_be_generated:
            yield line
    
    def _unsafe_lines(self, lines):
        if len(lines) < 2:
            yield 'unsafe { %s }' % (lines[0],)
        else:
            yield 'unsafe {'
            for line in lines:
                yield '    %s' % (line,)
            yield '}'

    def _wrap_return_type(self, call):
        wrapped = self.__model.wrapped_return_type()
        if wrapped:
            return '%s(%s)' % (wrapped[2:], call)
        return call

    def _uses_ptr_type(self):
        return any(p.type.is_ptr() for p in self.__model.params)

class CxxClassBinding:
    def __init__(self, model):
        self.__model = model
        self.__methods = [CxxMethodBinding(m) for m in model.methods]
    
    def defs(self):
        yield '// CLASS: %s' % (self.__model.name,)
        for method in self.__methods:
            for line in method.definition():
                yield line
        yield ''

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)
    
class CxxMethodBinding:
    def __init__(self, model):
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')

    def definition(self):
        if not self.__model.generates():
            return
        yield 'inline %s *%s(%s) {' % (
            self.__model.wrapped_return_type(),
            self.__model.overload_name(without_index=True),
            self._cxx_params(),
        )
        new_params_or_expr = self._call_params()
        if self.__model.returns_new():
            new_params_or_expr = 'self.%s(%s)' % (
                self.__model.overload_name(
                    without_index=True,
                    cxx_name=True,
                ),
                new_params_or_expr,
            )
        yield '    return new %s(%s);' % (
            self.__model.wrapped_return_type(),
            new_params_or_expr,
        )
        yield '}'

    def _cxx_params(self):
        params = self.__model.params.copy()
        if self.__model.returns_new():
            params.insert(0, self.__self_param)
        return ', '.join(self._cxx_param(p) for p in params)

    def _cxx_param(self, param):
        return '%s %s' % (
            param.type.in_cxx(),
            param.name,
        )

    def _call_params(self):
        return ', '.join(p.name for p in self.__model.params)

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
