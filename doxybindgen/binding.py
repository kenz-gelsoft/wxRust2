from .model import Param, RustType, prefixed, pascal_to_snake

# Known, and problematic
RUST_KEYWORDS = [
    'move',
    'ref',
    'type',
]


class RustClassBinding:
    def __init__(self, model):
        self.__model = model
        self.__methods = [RustMethodBinding(m) for m in model.methods]

    def ffi_lines(self):
        yield '// CLASS: %s' % (
            self.__model.name,
        )
        for method in self.__methods:
            for line in method.ffi_lines():
                yield line

    def binding_lines(self, classes):
        yield '// %s' % (
            self.__model.name,
        )
        yield 'wx_class! { %s(%s) impl' % (
            self.__model.unprefixed(),
            self.__model.name,
        )
        yield ',\n'.join(self._ancestor_methods(classes))
        yield '}'
        for line in self._impl_with_ctors():
            yield line
        for line in self._trait_with_methods():
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

    def _impl_with_ctors(self):
        yield 'impl %s {' % (self.__model.unprefixed(),)
        for ctor in self._ctors():
            for line in ctor.binding_lines():
                yield '    %s' % (line,)
        yield "    pub fn none() -> Option<&'static Self> {"
        yield '        None'
        yield '    }'
        yield '}'

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)
    
    def _trait_with_methods(self):
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
            for line in method.binding_lines():
                yield '%s%s' % (indent, line)
        yield '}\n'


class RustMethodBinding:
    def __init__(self, model):
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')
        # must be name neither self or this
        self.__shim_self = Param(RustType(model.cls.name, model.const), 'self_')
        self.__generic_params = self._make_params_generic()

    def is_blocked(self):
        return self.__model.is_blocked()

    def ffi_lines(self):
        body = 'pub fn %s(%s)%s;' % (
            self.__model.name(for_shim=True),
            self._rust_params(binding=False),
            self._returns_or_not(),
        )
        suppressed = self.__model.suppressed_reason()
        if suppressed:
            yield '// %s: %s' % (suppressed, body)
            return
        yield body

    def _returns_or_not(self, binding=False):
        if self.__model.returns.is_void():
            return ''
        returns = self.__model.returns.in_rust(with_ffi=binding)
        wrapped = self.__model.wrapped_return_type()
        if wrapped:
            if binding:
                returns = wrapped[2:]
                if self.__model.returns.is_str():
                    returns = 'String'
            else:
                returns = '*mut c_void'
        return ' -> %s' % (returns,)
    
    def _rename(self):
        if self.__model.overload_index == 0:
            return ''
        return '#[rust_name = "%s"]' % (
            self.__model.name(for_shim=True),
        )
    
    def _make_params_generic(self):
        generic_params = []
        for param in self.__model.params:
            if param.type.is_ptr_to_binding():
                count = len(generic_params)
                name = chr(ord('T') + count)
                generic_params.append(param.type.make_generic(name))
        return generic_params

    def binding_lines(self):
        suppress = self.__model.suppressed_reason()
        if suppress:
            yield '// %s: fn %s()' % (
                suppress,
                self.__model.name(for_shim=False),
            )
            return
        gen_params = ''
        if self.__generic_params:
            gen_params = '<%s>' % (
                ', '.join('%s: %s' % p for p in self.__generic_params),
            )
        yield '%sfn %s%s(%s)%s {' % (
            'pub ' if self.is_ctor else '',
            self._rust_method_name(),
            gen_params,
            self._rust_params(binding=True),
            self._returns_or_not(binding=True),
        )
        body_lines = list(self._binding_body())
        for line in self._wrap_unsafe(body_lines):
            yield '    %s' % (line,)
        yield '}'
    
    def _binding_body(self):
        params = self.__model.params
        for param in params:
            marshalling = param.marshal()
            if marshalling:
                for line in marshalling:
                    yield '%s' % (line,)
        name = prefixed(self.__model.name(for_shim=True), with_ffi=True)
        self_to_insert = None
        if self.__model.is_instance_method:
            is_mut_self = not self.__model.const
            self_param = self.__self_param.rust_ffi_ref(
                is_mut_self=is_mut_self,
            )
            self_to_insert = self_param
        call = '%s(%s)' % (
            name,
            self._call_params(params, self_to_insert),
        )
        yield self._wrap_return_type(call)
    
    def _call_params(self, params, self_to_insert):
        params = [self.non_keyword_name(p.name) for p in params]
        if self_to_insert:
            params.insert(0, self_to_insert)
        return ', '.join(params)

    def non_keyword_name(self, name):
        while name in RUST_KEYWORDS:
            name += '_'
        return name

    def _rust_method_name(self):
        method_name = pascal_to_snake(self.__model.name(
            for_shim=False,
            without_index=True,
        ))
        if self.__model.is_ctor:
            method_name = 'new'
        method_name = self.__model.overload_indexed(method_name)
        method_name = self.non_keyword_name(method_name)
        return method_name
    
    def _rust_params(self, binding):
        params = self.__model.params.copy()
        if self.__model.is_instance_method:
            if binding:
                params.insert(0, self.__self_param)
            else:
                params.insert(0, self.__shim_self)
        return ', '.join(self._rust_param(p, binding) for p in params)

    def _rust_param(self, param, binding):
        typename = param.type.in_rust(
            with_ffi=binding,
            binding=binding
        )
        if binding:
            if param.is_self():
                return '&self'
            elif param.type.generic_name:
                typename = 'Option<&%s>' % (param.type.generic_name,)
        return '%s: %s' % (
            self.non_keyword_name(param.name),
            typename,
        )

    def _wrap_unsafe(self, lines):
        if len(lines) < 2:
            yield 'unsafe { %s }' % (lines[0],)
        else:
            yield 'unsafe {'
            for line in lines:
                yield '    %s' % (line,)
            yield '}'

    def _wrap_return_type(self, call):
        wrapped = self.__model.wrapped_return_type()
        if self.__model.returns.is_str():
            return 'crate::from_wx_string(%s)' % (call,)
        if wrapped:
            return '%s(%s)' % (wrapped[2:], call)
        return call

    def _uses_ptr_type(self):
        return any(p.type.is_ptr() for p in self.__model.params)


class CxxClassBinding:
    def __init__(self, model):
        self.__model = model
        self.__methods = [CxxMethodBinding(m) for m in model.methods]
    
    def shims(self, is_cc=False):
        yield '// CLASS: %s' % (self.__model.name,)
        for method in self.__methods:
            for line in method.shim(is_cc):
                yield line
        yield ''

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)


class CxxMethodBinding:
    def __init__(self, model):
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')

    def shim(self, is_cc):
        if self.__model.suppressed_reason():
            return
        wrapped = self.__model.wrapped_return_type()
        returns = self.__model.returns.in_cxx() + ' '
        if wrapped:
            returns = '%s *' % (
                wrapped,
            )
        signature = '%s%s(%s)' % (
            returns,
            self.__model.name(for_shim=True),
            self._cxx_params(),
        )
        if is_cc:
            yield '%s {' % (signature,)
        else:
            yield '%s;' % (signature,)
            return
        new_params_or_expr = self._call_params()
        if not self.is_ctor:
            self_or_class = 'self->'
            if self.__model.is_static:
                self_or_class = '%s::' % (self.__model.cls.name,)
            new_params_or_expr = '%s%s(%s)' % (
                self_or_class,
                self.__model.name(for_shim=False, without_index=True),
                new_params_or_expr,
            )
        if wrapped:
            yield '    return new %s(%s);' % (wrapped, new_params_or_expr)
        else:
            yield '    return %s;' % (new_params_or_expr,)
        yield '}'

    def _cxx_params(self):
        params = self.__model.params.copy()
        if self.__model.is_instance_method:
            params.insert(0, self.__self_param)
        return ', '.join(self._cxx_param(p) for p in params)

    def _cxx_param(self, param):
        return '%s %s' % (
            param.type.in_cxx(),
            param.name,
        )

    def _call_params(self):
        return ', '.join(self._deref_if_needed(p) for p in self.__model.params)
    
    def _deref_if_needed(self, param):
        if param.type.is_ref():
            return '*%s' % (param.name,)
        return param.name

