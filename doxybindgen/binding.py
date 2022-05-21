from collections import OrderedDict
from .model import Param, RustType, non_keyword_name, prefixed, pascal_to_snake


class RustClassBinding:
    def __init__(self, model):
        self.__model = model
        self.overloads = OverloadTree(model)
        self.overloads.print_tree()
        self.__methods = [RustMethodBinding(self, m) for m in model.methods]
    
    def is_a(self, base):
        return self.__model.manager.is_a(self.__model, base)

    def lines(self, for_ffi=False, for_methods=False):
        yield ''
        yield '// %s' % (
            self.__model.name,
        )
        if for_ffi:
            if not self.is_a('wxObject'):
                yield 'pub fn %s_delete(self_: *mut c_void);' % (
                    self.__model.name,
                )
            for method in self.__methods:
                for line in method.lines(for_ffi=True):
                    yield line
        elif for_methods:
            for line in self._trait_with_methods():
                yield line
        else:
            unprefixed = self.__model.unprefixed()
            yield 'wx_class! { %s = ' % (unprefixed,)
            yield '    %sIsOwned<true>(%s) impl' % (
                unprefixed,
                self.__model.name,
            )
            yield ',\n'.join(self._ancestor_methods())
            yield '}'
            for line in self._impl_with_ctors():
                yield line
            for line in self._impl_drop_if_needed():
                yield line
            for line in self._impl_non_virtual_overrides():
                yield line
    
    def _ancestor_methods(self):
        for ancestor in self.__model.manager.ancestors_of(self.__model):
            comment_or_not = ''
            if any(m.is_non_virtual_override(ancestor) for m in self.__methods):
                comment_or_not = '// '
            yield '        %s%sMethods' % (
                comment_or_not,
                ancestor.name[2:],
            )

    def _impl_with_ctors(self):
        unprefixed = self.__model.unprefixed()
        yield 'impl<const OWNED: bool> %sIsOwned<OWNED> {' % (unprefixed,)
        for enum in self.__model.enums:
            for line in enum.generate():
                yield '    %s' % (line,)
            yield ''
        for ctor in self._ctors():
            for line in ctor.lines():
                yield '    %s' % (line,)
        yield "    pub fn none() -> Option<&'static Self> {"
        yield '        None'
        yield '    }'
        yield '}'
    
    def _impl_drop_if_needed(self):
        if (self.is_a('wxEvtHandler') or
            self.is_a('wxSizer')):
            return
        deleter_class = self.__model.name
        if self.is_a('wxObject'):
            deleter_class = 'wxObject'
        yield 'impl<const OWNED: bool> Drop for %sIsOwned<OWNED> {' % (self.__model.unprefixed(),)
        yield '    fn drop(&mut self) {'
        yield '        if OWNED {'
        yield '            unsafe { ffi::%s_delete(self.0) }' % (deleter_class,)
        yield '        }'
        yield '    }'
        yield '}'
    
    def _impl_non_virtual_overrides(self):
        for ancestor in self.__model.manager.ancestors_of(self.__model):
            methods = [m for m in self.__methods if m.is_non_virtual_override(ancestor)]
            if not methods:
                continue
            yield 'impl<const OWNED: bool> %sMethods for %sIsOwned<OWNED> {' % (
                ancestor.unprefixed(),
                self.__model.unprefixed(),
            )
            for method in methods:
                for line in method.lines():
                    yield '    %s' % (line)
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
        ancestors = self.__model.manager.ancestors_of(self.__model)
        for method in self.__methods:
            if method.is_ctor:
                continue
            if any(method.is_non_virtual_override(c) for c in ancestors):
                continue
            for line in method.lines():
                yield '%s%s' % (indent, line)
        yield '}'


class Overload:
    def __init__(self, name):
        self.name = name
        self.method = None
        self.items = OrderedDict()


class OverloadTree:
    def __init__(self, cls):
        self.__cls = cls
        self.__root = Overload("")

        for m in cls.methods:
            self._add(m)
    
    def _path(self, method):
        path = []
        path.append('%s.%s' % (
            self.__cls.name,
            method.name(without_index=True),
        ))
        for p in method.params:
            path.append(p.type)
        return path
    
    def _add(self, method):
        if method.suppressed_reason():
            return
        path = self._path(method)
        node = self.__root
        for item in path:
            items = node.items
            if item not in items:
                items[item] = Overload(item)
            node = items[item]
        node.method = method
    
    def has_overload(self, method):
        if method.suppressed_reason():
            return False
        by_name = self._path(method)[0]
        node = self.__root.items[by_name]
        return self._count_in_subtree(node) > 1
    
    def args_to_disambiguate(self, method):
        if method.suppressed_reason():
            return []
        result = []
        path = self._path(method)
        prev_count = None
        current = self.__root
        for item in path:
            current = current.items[item]
            count = self._count_in_subtree(current)
            if prev_count is None or count < prev_count:
                result.append(item)
            if count < 2:
                break
            prev_count = count
        return [arg.in_overload_name() for arg in result[1:]]
    
    def _count_in_subtree(self, node):
        count = 0
        if node.method is not None:
            count += 1
        for k, v in node.items.items():
            count += self._count_in_subtree(v)
        return count
    
    def print_tree(self):
        self.print_node(self.__root, 0)

    def print_node(self, node, level):
        indent = '    ' * level
        for k, v in node.items.items():
            count = self._count_in_subtree(v)
            if level == 0 and count == 1:
                continue
            
            args = ''
            method = v.method
            if method is not None:
                args = self.args_to_disambiguate(method)
                args = '(%s)' % (', '.join(args),)
                method = method.name()
            print("%s- %s: %s: %s %s" % (indent, count, k, method, args))
            self.print_node(v, level + 1)


class RustMethodBinding:
    def __init__(self, cls, model):
        self.__cls = cls
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')
        # must be name neither self or this
        self.__ffi_self = Param(RustType(model.cls.name, model.const), 'self_')
        self.__generic_params = GenericParams(self.__model.params)

    def is_blocked(self):
        return self.__model.is_blocked()

    def _returns_or_not(self, for_ffi=False):
        if self.__model.returns.is_void():
            return ''
        returns = self.__model.returns.in_rust(for_ffi=True)
        wrapped = self.__model.wrapped_return_type(allows_ptr=True)
        if self.__model.maybe_returns_self():
            if for_ffi:
                returns = '*mut c_void'
            else:
                returns = '&Self'
        elif wrapped:
            if for_ffi:
                returns = '*mut c_void'
            else:
                returns = wrapped[2:]
                if (self.is_ctor or
                    self.__model.returns.is_ptr_to_binding()):
                    if self.is_ctor:
                        returns = '%sIsOwned<OWNED>' % (returns,)
                    elif not self.__model.returns_owned():
                        if self.__model.returns_trackable():
                            returns = 'WeakRef<%s>' % (returns,)
                        else:
                            returns = 'Option<%sIsOwned<false>>' % (returns,)
                if self.__model.returns.is_str():
                    returns = 'String'
        return ' -> %s' % (returns,)
    
    def lines(self, for_ffi=False):
        pub_or_not = 'pub '
        gen_params = ''
        name = self.__model.name(for_ffi=True)
        if not for_ffi:
            if not self.is_ctor:
                pub_or_not = '' if not self.is_ctor else 'pub '
            name = self._rust_method_name()
            if self.__generic_params.names:
                gen_params = '<%s>' % (
                    ', '.join('%s: %s' % p for p in self.__generic_params.names),
                )
        signature = '%sfn %s%s(%s)%s' % (
            pub_or_not,
            name,
            gen_params,
            self._rust_params(for_ffi=for_ffi),
            self._returns_or_not(for_ffi=for_ffi),
        )
        suppressed = self.__model.suppressed_reason()
        if suppressed:
            if for_ffi:
                body = '%s;' % (signature,)
            else:
                body = 'fn %s()' % (self.__model.name(),)
            yield '// %s: %s' % (
                suppressed,
                body,
            )
            return
        if for_ffi:
            yield '%s;' % (signature,)
        else:
            yield '%s {' % (signature,)
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
        name = prefixed(self.__model.name(for_ffi=True), with_ffi=True)
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
        params = [p.name for p in params]
        if self_to_insert:
            params.insert(0, self_to_insert)
        return ', '.join(params)

    def _rust_method_name(self):
        method_name = pascal_to_snake(self.__model.name(
            without_index=True,
        ))
        overloads = self.__cls.overloads
        if self.__model.is_ctor:
            method_name = 'new'
        if overloads.has_overload(self.__model):
            splitter = '_'
            arg_types = overloads.args_to_disambiguate(self.__model)
            if self.__model.is_ctor:
                if self.__cls.is_a('wxWindow'):
                    return 'new_2step' if len(arg_types) == 0 else 'new'
                splitter = '_with_'
            if len(arg_types) > 0:
                method_name += splitter + '_'.join(arg_types)
        method_name = non_keyword_name(method_name)
        return method_name
    
    def _rust_params(self, for_ffi=False):
        params = self.__model.params.copy()
        if self.__model.is_instance_method:
            if for_ffi:
                params.insert(0, self.__ffi_self)
            else:
                params.insert(0, self.__self_param)
        return ', '.join(self._rust_param(p, for_ffi) for p in params)

    def _rust_param(self, param, for_ffi):
        typename = param.type.in_rust(for_ffi=for_ffi)
        if not for_ffi:
            if param.is_self():
                return '&self'
            elif param.type.generic_name:
                typename = '&%s' % (param.type.generic_name,)
                if param.type.generic_option:
                    typename = 'Option<%s>' % (typename,)
        return '%s: %s' % (
            param.name,
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
        if self.__model.returns.is_str():
            return 'wx_base::from_wx_string(%s)' % (call,)
        if self.__model.maybe_returns_self():
            return '%s; &self' % (call,)
        wrapped = self.__model.wrapped_return_type(allows_ptr=False)
        if wrapped:
            return '%sIsOwned(%s)' % (wrapped[2:], call)
        wrapped = self.__model.wrapped_return_type(allows_ptr=True)
        if wrapped:
            if self.__model.returns_owned():
                return '%s::from_ptr(%s)' % (wrapped[2:], call)
            elif self.__model.returns_trackable():
                return 'WeakRef::<%s>::from(%s)' % (wrapped[2:], call)
            else:
                return '%s::option_from(%s)' % (wrapped[2:], call)
        return call

    def _uses_ptr_type(self):
        return any(p.type.is_ptr() for p in self.__model.params)
    
    def is_non_virtual_override(self, cls):
        return self.__model.is_non_virtual_override(cls)


class GenericParams:
    def __init__(self, params):
        self.__used_names = dict()
        self.names = self._make_params_generic(params)
    
    def _make_params_generic(self, params):
        self.names = []
        for param in params:
            is_ptr_to_binding = param.type.is_ptr_to_binding()
            if (is_ptr_to_binding or
                param.type.is_const_ref_to_binding()):
                name = self._new_name_for(param.type)
                self.names.append(param.type.make_generic(
                    name,
                    is_option=is_ptr_to_binding,
                ))
        return self.names

    def _new_name_for(self, param_type):
        name = param_type.in_overload_name()[0].upper()
        used = self.__used_names
        if name not in used:
            used[name] = 0
        n = used[name]
        n += 1
        used[name] = n
        if n > 1:
            name = '%s%s' % (name, n)
        return name


class CxxClassBinding:
    def __init__(self, model, config):
        self.__model = model
        self.conditions = config.get('conditions')
        self.__methods = [CxxMethodBinding(self, m) for m in model.methods]
    
    def lines(self, is_cc=False):
        yield '// CLASS: %s' % (self.__model.name,)
        for line in self._dtor_lines(is_cc):
            yield line
        self.in_condition = None
        for method in self.__methods:
            for line in method.lines(is_cc):
                yield line
        if self.in_condition:
            yield '#endif'
        yield ''

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)
    
    def _dtor_lines(self, is_cc):
        if self.__model.manager.is_a(self.__model, 'wxObject'):
            return
        signature = 'void %s_delete(%s *self)' % (
            self.__model.name,
            self.__model.name,
        )
        if is_cc:
            yield '%s {' % (signature,)
            yield '    delete self;'
            yield '}'
        else:
            yield '%s;' % (signature,)


class CxxMethodBinding:
    def __init__(self, cls, model):
        self.__cls = cls
        self.__model = model
        self.is_ctor = model.is_ctor
        self.__self_param = Param(RustType(model.cls.name, model.const), 'self')

    def lines(self, is_cc):
        if self.__model.suppressed_reason():
            return
        wrapped = self.__model.wrapped_return_type(allows_ptr=False)
        returns = self.__model.returns.in_cxx() + ' '
        if wrapped:
            returns = '%s *' % (
                wrapped,
            )
        signature = '%s%s(%s)' % (
            returns,
            self.__model.name(for_ffi=True),
            self._cxx_params(),
        )
        condition = self.__model.find_condition(self.__cls.conditions)
        if self.__cls.in_condition != condition:
            if self.__cls.in_condition:
                yield '#endif'
            self.__cls.in_condition = condition
            if condition:
                yield condition.get('cxx')
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
                self.__model.name(without_index=True),
                new_params_or_expr,
            )
        if self.__model.maybe_returns_self():
            yield '    return &(%s);' % (new_params_or_expr,)
        elif wrapped:
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

