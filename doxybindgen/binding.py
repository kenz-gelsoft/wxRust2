from .model import Param, SelfType, prefixed

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
            binding = RustMethodBinding(ctor)
            for line in binding.ffi_lines():
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
            binding = RustMethodBinding(ctor)
            yield binding.for_rs()
        yield '}'

    def _methods_for_rs(self):
        yield 'trait %sMethods: WxRustMethods {' % (self.__model.unprefixed(),)
        for method in self.__model.methods:
            if method.is_ctor:
                continue
            binding = RustMethodBinding(method)
            yield binding.for_rs()
        yield '}\n'

class CxxClassBinding:
    def __init__(self, model):
        self.__model = model

    def ctors_for_h(self):
        yield '// CLASS: %s' % (self.__model.name,)
        for ctor in self.__model.ctors():
            binding = CxxMethodBinding(ctor)
            yield binding.for_h()
        yield ''
    
    def ctors_for_cc(self):
        yield '// CLASS: %s' % (self.__model.name,)
        for ctor in self.__model.ctors():
            binding = CxxMethodBinding(ctor)
            yield binding.for_cc()
        yield ''

class RustMethodBinding:
    def __init__(self, model):
        self.__model = model
        self.__is_dtor = model.name.startswith('~')
        self.__is_method_call = not (model.is_static or model.is_ctor)
        self.__self_param = Param(SelfType(model.cls.name, model.const), 'self')
    
    def in_rust(self):
        body = '%sfn %s(%s)%s;' % (
            self._unsafe_or_not(),
            self.__model.name,
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

    def _returns_or_not(self):
        returns = self.__model.returns.in_rust()
        if returns in ['void', '']:
            returns = ''
        else:
            returns = ' -> %s' % (returns,)
        return returns
    
    def ffi_lines(self):
        rs_template = '%sfn %s(%s) -> *mut %s;'
        lines = [rs_template % (
            self._unsafe_or_not(),
            self.__model.overload_name(),
            self._rust_params(),
            self.__model.cls.name,
        )]
        overload = self._rename()
        if overload:
            lines.insert(0, overload)
        return lines

    def _unsafe_or_not(self):
        return 'unsafe ' if self._uses_ptr_type() else ''
    
    def _rename(self):
        if self.__model.overload_index == 0:
            return ''
        return '#[rust_name = "%s"]' % (self.__model.overload_name(),)

    def for_rs(self):
        suppress = self._suppressed_reason(suppress_ctor=False)
        if suppress:
            return '// %s: fn %s()' % (suppress, self.__model.name)
        rs_template = '''\
    %sfn %s(%s)%s {
        %s
    }'''
        unprefixed = self.__model.cls.unprefixed()
        call = '%s(%s)' % (
            prefixed(self.__model.overload_name(), with_ffi=True),
            self.__model.call_params(),
        )
        if self.__is_method_call:
            call = 'self.pinned::<ffi::%s>().as_mut().%s(%s)' % (
                self.__model.cls.name,
                self.__model.overload_name(),
                self.__model.call_params(),
            )
        returns_or_not = ''
        if not self.__model.returns.is_void():
            returns_or_not = ' -> %s' % (self.__model.returns.in_rust(with_ffi=True),)
        is_method = self.__is_method_call
        return rs_template % (
            '' if is_method else 'pub ',
            self._rust_method_name(),
            self._rust_params(with_ffi=True, binding=True),
            returns_or_not,
            self._wrap_if_unsafe(
                self._wrap_return_type(
                    unprefixed, call
                )
            ),
        )

    def _suppressed_reason(self, suppress_ctor=True):
        if suppress_ctor and self.__model.is_ctor:
            return 'CTOR'
        if self.__is_dtor:
            return 'DTOR'
        if self._uses_unsupported_type():
            return 'CXX_UNSUPPORTED'
        if self.__model.cls.blocks(self.__model.name):
            return 'BLOCKED'
        return None
    
    def _uses_unsupported_type(self):
        if self.__model.returns.not_supported():
            return True
        return any(p.type.not_supported() for p in self.__model.params)

    def _rust_method_name(self):
        method_name = self.__model.name
        if self.__model.is_ctor:
            method_name = 'new'
        if self.__model.overload_index > 0:
            method_name += str(self.__model.overload_index)
        return method_name

    def _rust_params(self, with_ffi=False, binding=False):
        params = self.__model.params.copy()
        if self.__is_method_call:
            params.insert(0, self.__self_param)
        return ', '.join(self._rust_param(p, with_ffi, binding) for p in params)

    def _rust_param(self, param, with_ffi, binding):
        if binding and isinstance(param.type, SelfType):
            return '&self'
        return '%s: %s' % (
            param.name,
            param.type.in_rust(with_ffi)
        )

    def _wrap_if_unsafe(self, t):
        if self._uses_ptr_type():
            return 'unsafe { %s }' % (t,)
        return t

    def _wrap_return_type(self, type, body):
        if self.__model.is_ctor:
            return '%s(%s)' % (type, body)
        return body

    def _uses_ptr_type(self):
        return any(p.type.is_ptr() for p in self.__model.params)

class CxxMethodBinding:
    def __init__(self, model):
        self.__model = model

    def for_h(self):
        body = '%s *%s(%s);' % (
            self.__model.name,
            self.__model.overload_name(),
            self._cxx_params(),
        )
        return body
    
    def for_cc(self):
        cc_template = '''\
%s *%s(%s) {
    return new %s(%s);
}'''
        return cc_template % (
            self.__model.cls.name,
            self.__model.overload_name(),
            self._cxx_params(),
            self.__model.cls.name,
            self.__model.call_params(),
        )

    def _cxx_params(self):
        return ', '.join(self._cxx_param(p) for p in self.__model.params)

    def _cxx_param(self, param):
        return '%s %s' % (
            param.type.in_cxx(),
            param.name,
        )
