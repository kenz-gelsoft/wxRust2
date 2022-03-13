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

    def ffi_lines(self, for_shim=False):
        if not for_shim:
            yield ''
        yield '// CLASS: %s' % (
            self.__model.name,
        )
        if not for_shim:
            handwritten = ''
            if self.__model.is_trivial():
                handwritten = ' = crate::%s' % (self.__model.name,)
            yield 'type %s%s;' % (
                self.__model.name,
                handwritten,
            )
            internal_base = self.__model.internal_base()
            if internal_base:
                yield 'type %s;' % (
                    internal_base,
                )
        for method in self.__methods:
            for line in method.ffi_lines(for_shim=for_shim):
                yield line

    def binding_lines(self, classes):
        yield '// %s' % (
            self.__model.name,
        )
        if self.__model.is_trivial():
            yield 'pub struct %s(ffi::%s);' % (
                self.__model.unprefixed(),
                self.__model.name,
            )
        else:
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
        if not self.__model.is_trivial():
            yield '}'

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor and not m.is_blocked())
    
    def _trait_with_methods(self):
        indent = ' ' * 4 * 1
        if not self.__model.is_trivial():
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
        self.__is_dtor = model.name(for_shim=False).startswith('~')
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')
        # must be name neither self or this
        self.__shim_self = Param(RustType(model.cls.name, model.const), 'self_')
        self.__generic_params = self._make_params_generic()
    
    def is_blocked(self):
        return self.__model.is_blocked()

    def ffi_lines(self, for_shim):
        if for_shim and not self.__model.needs_shim():
            return
        body = '%sfn %s(%s)%s;' % (
            self._unsafe_or_not(),
            self.__model.name(for_shim=for_shim, without_index=True),
            self._rust_params(for_shim=for_shim),
            self._returns_or_not(for_shim=for_shim),
        )
        suppressed = self._suppressed_reason()
        if not for_shim and suppressed:
            yield '// %s: %s' % (suppressed, body)
            return
        overload = self._rename()
        if overload:
            yield overload
        yield body

    def _returns_or_not(self, for_shim=True, binding=False):
        if self.__model.returns.is_void():
            return ''
        returns = self.__model.returns.in_rust(with_ffi=binding)
        if self.__model.returns.is_str():
            returns = 'String'
        wrapped = self.__model.wrapped_return_type()
        if wrapped:
            if binding:
                returns = wrapped[2:]
            elif for_shim:
                if self.__model.returns.is_trivial():
                    returns = wrapped
                else:
                    returns = '*mut %s' % (wrapped,)
        return ' -> %s' % (returns,)
    
    def _unsafe_or_not(self):
        return 'unsafe ' if self._uses_ptr_type() else ''
    
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
        suppress = self._suppressed_reason(
            suppress_shim=False,
        )
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
            self._rust_params(with_ffi=True, binding=True),
            self._returns_or_not(binding=True),
        )
        body_lines = list(self._binding_body())
        for line in self._wrap_if_unsafe(body_lines):
            yield '    %s' % (line,)
        yield '}'
    
    def _binding_body(self):
        rule = None
        internal_base = self.__model.internal_base()
        if internal_base:
            rule = { self.__model.cls.name: internal_base }
        
        params = [p.rewrite(rule) for p in self.__model.params]
        for param in params:
            marshalling = param.marshal()
            if marshalling:
                for line in marshalling:
                    yield '%s' % (line,)
        name = prefixed(self.__model.name(for_shim=True), with_ffi=True)
        self_to_insert = None
        if self.__model.is_instance_method:
            self_param = self.__self_param.rewrite(rule).rust_ffi_ref()
            if self.__model.needs_shim():
                if self.__model.const:
                    self_param = '&' + self_param
                self_to_insert = self_param
            else:
                name = '%s.%s' % (
                    self_param,
                    self.__model.name(for_shim=True),
                )
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

    def _suppressed_reason(self, suppress_shim=True):
        if self.__model.is_blocked():
            return 'BLOCKED'
        if self.__model.is_ctor:
            if suppress_shim:
                return 'CTOR'
        if self.__is_dtor:
            return 'DTOR'
        if self.__model.needs_shim():
            if suppress_shim:
                return 'GENERATED'
        if self.__model.uses_unsupported_type():
            return 'CXX_UNSUPPORTED'
        return None
    
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
    
    def _rust_params(self, with_ffi=False, binding=False, for_shim=False):
        params = self.__model.params.copy()
        if self.__model.is_instance_method:
            if for_shim and self.__model.needs_shim():
                params.insert(0, self.__shim_self)
            else:
                params.insert(0, self.__self_param)
        return ', '.join(self._rust_param(p, with_ffi, binding) for p in params)

    def _rust_param(self, param, with_ffi, binding):
        rule = None
        internal_base = self.__model.internal_base()
        if internal_base:
            rule = { self.__model.cls.name: internal_base }
        param = param.rewrite(rule)
        typename = param.type.in_rust(with_ffi, binding)
        if binding:
            if param.is_self():
                if param.type.is_trivial() and not param.type.const:
                    return '&mut self'
                else:
                    return '&self'
            elif param.type.generic_name:
                typename = 'Option<&%s>' % (param.type.generic_name,)
        return '%s: %s' % (
            self.non_keyword_name(param.name),
            typename,
        )

    def _wrap_if_unsafe(self, lines):
        if not self._uses_ptr_type():
            for line in lines:
                yield line
            return
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
    
    def shims(self):
        yield '// CLASS: %s' % (self.__model.name,)
        for method in self.__methods:
            for line in method.shim():
                yield line
        yield ''

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)


class CxxMethodBinding:
    def __init__(self, model):
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')

    def shim(self):
        if not self.__model.needs_shim():
            return
        wrapped = self.__model.wrapped_return_type()
        returns = self.__model.returns.in_cxx() + ' '
        if self.__model.returns.is_str():
            returns = 'rust::String '
        if wrapped:
            ptr_or_not = '' if self.__model.returns.is_trivial() else '*'
            returns = '%s %s' % (
                wrapped,
                ptr_or_not,
            )
        yield 'inline %s%s(%s) {' % (
            returns,
            self.__model.name(for_shim=True, without_index=True),
            self._cxx_params(),
        )
        new_params_or_expr = self._call_params()
        if not self.is_ctor:
            self_or_class = 'self.'
            if self.__model.is_static:
                self_or_class = '%s::' % (self.__model.cls.name,)
            new_params_or_expr = '%s%s(%s)' % (
                self_or_class,
                self.__model.name(for_shim=False, without_index=True),
                new_params_or_expr,
            )
        if self.__model.returns.is_str():
            new_params_or_expr = 'rust::String(%s.utf8_str())' % (
                new_params_or_expr,
            )
        if wrapped and (self.is_ctor or not self.__model.returns.is_trivial()):
            new_or_not = '' if self.__model.returns.is_trivial() else 'new '
            yield '    return %s%s(%s);' % (new_or_not, wrapped, new_params_or_expr)
        else:
            yield '    return %s;' % (new_params_or_expr,)
        yield '}'

    def _cxx_params(self):
        params = self.__model.params.copy()
        if self.__model.needs_shim() and self.__model.is_instance_method:
            params.insert(0, self.__self_param)
        return ', '.join(self._cxx_param(p) for p in params)

    def _cxx_param(self, param):
        return '%s %s' % (
            param.type.in_cxx(),
            param.name,
        )

    def _call_params(self):
        return ', '.join(p.name for p in self.__model.params)

