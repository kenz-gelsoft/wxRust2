from collections import OrderedDict
from .model import Param, RustType, non_keyword_name, prefixed, pascal_to_snake

MIXIN_CXX_CLASS = {
    # MSW: Some wxTextEntry-derived classes actually derived from its base
    'wxTextEntry': 'wxTextEntryBase',
}

class RustClassBinding:
    def __init__(self, model, overload_tree_md):
        self.__model = model
        self.overloads = OverloadTree(model)
        self.overloads.print_tree(overload_tree_md)
        self.__methods = [RustMethodBinding(self, m) for m in model.methods]
    
    def has_initial(self, i):
        return self.__model.has_initial(i)
    
    def is_a(self, base):
        return self.__model.manager.is_a(self.__model, base)
    
    def mixed_into(self):
        return self.__model.manager.mixed_into(self.__model.name)
    
    def as_mixin(self):
        if not self.mixed_into():
            return None
        return 'as_%s' % (
            pascal_to_snake(self.__model.unprefixed()),
        )

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
            else:
                yield 'pub fn %s_CLASSINFO() -> *mut c_void;' % (
                    self.__model.name,
                )
            for method in self.__methods:
                for line in method.lines(for_ffi=True):
                    yield line
            for line in self._mixin_ffi_lines():
                yield line
        elif for_methods:
            for line in self._trait_with_methods():
                yield line
        else:
            unprefixed = self.__model.unprefixed()
            yield 'wxwidgets! {'
            yield '    /// %s' % (self.__model.doc,)
            yield "    /// - [`%s`] represents a C++ `%s` class instance which your code has ownership, [`%sFromCpp`]`<false>` represents one which don't own." % (
                unprefixed,
                self.__model.name,
                unprefixed,
            )
            buildable_doc = ''
            if self.__model.library != 'base':
                buildable_doc = ' or [`Buildable::builder()`] (if available)'
            yield "    /// - Use [`%s`]'s `new()`%s to create an instance of this class." % (
                unprefixed,
                buildable_doc,
            )
            yield "    /// - See [C++ `%s` class's documentation](%s) for more details." % (
                self.__model.name,
                self.__model.doc_url(),
            )
            for alias in (self.__model.name, unprefixed):
                yield '    #[doc(alias = "%s")]' % (alias,)
            yield '    class %s' % (unprefixed,)
            yield '        = %sFromCpp<true>(%s) impl' % (
                unprefixed,
                self.__model.name,
            )
            ancestors = list(reversed(list(self._ancestors_with_overloads())))
            ancestor_methods = list(self._ancestor_methods(ancestors))
            last_methods_if_commented_out = None
            if '//' in ancestor_methods[-1]:
                last_methods_if_commented_out = ancestor_methods.pop(-1)
            yield ',\n'.join(ancestor_methods)
            if last_methods_if_commented_out:
                yield last_methods_if_commented_out
            yield '}'
            for line in self._impl_with_ctors():
                yield line
            for line in self._impl_clone():
                yield line                
            for line in self._impl_from_ancestors():
                yield line
            for line in self._impl_dynamic_cast_if_needed():
                yield line
            for line in self._impl_drop_if_needed():
                yield line
            for line in self._impl_mixin_if_needed():
                yield line
            for line in self._impl_non_virtual_overrides(ancestors):
                yield line
    
    def _ancestors_with_overloads(self):
        generated = []
        for ancestor in reversed(self.__model.manager.ancestors_of(self.__model)):
            overloads = [m for m in self.__methods if (
                m.is_non_virtual_override(ancestor) and
                m not in generated
            )]
            generated.extend(overloads)
            yield [ancestor, overloads]
    
    def _ancestor_methods(self, ancestors):
        for ancestor, overloads in ancestors:
            comment_or_not = ''
            if overloads:
                comment_or_not = '// '
            yield '        %s%sMethods' % (
                comment_or_not,
                ancestor.name[2:],
            )

    def _impl_with_ctors(self):
        unprefixed = self.__model.unprefixed()
        yield 'impl<const FROM_CPP: bool> %sFromCpp<FROM_CPP> {' % (unprefixed,)
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

    def _has_drop(self):
        if (self.is_a('wxEvtHandler') or
            self.is_a('wxSizer')):
            return False
        return True

    def _impl_clone(self):
        in_rust = '%sFromCpp' % (self.__model.unprefixed(),)
        if self._has_drop():
            yield 'impl Clone for %s<false> {' % (in_rust,)
        else:
            yield 'impl<const FROM_CPP: bool> Clone for %s<FROM_CPP> {' % (in_rust,)
        yield '    fn clone(&self) -> Self {'
        yield '        Self(self.0)'
        yield '    }'
        yield '}'

    def _impl_from_ancestors(self):
        unprefixed = self.__model.unprefixed()
        for ancestor in self.__model.manager.ancestors_of(self.__model):
            unprefixed_ancestor = ancestor.name[2:]
            if unprefixed == unprefixed_ancestor:
                continue
            yield 'impl<const FROM_CPP: bool> From<%sFromCpp<FROM_CPP>> for %sFromCpp<FROM_CPP> {' % (
                unprefixed,
                unprefixed_ancestor,
            )
            yield '    fn from(o: %sFromCpp<FROM_CPP>) -> Self {' % (unprefixed,)
            yield '        unsafe { Self::from_ptr(o.as_ptr()) }'
            yield '    }'
            yield '}'

    def _impl_dynamic_cast_if_needed(self):
        if not self.is_a('wxObject'):
            return
        yield 'impl<const FROM_CPP: bool> DynamicCast for %sFromCpp<FROM_CPP> {' % (self.__model.unprefixed(),)
        yield '    fn class_info() -> ClassInfoFromCpp<false> {'
        yield '        unsafe { ClassInfoFromCpp::from_ptr(ffi::%s_CLASSINFO()) }' % (self.__model.name)
        yield '    }'
        yield '}'
    
    def _impl_drop_if_needed(self):
        if (self.is_a('wxEvtHandler') or
            self.is_a('wxSizer')):
            return
        deleter_class = self.__model.name
        if self.is_a('wxObject'):
            deleter_class = 'wxObject'
        yield 'impl<const FROM_CPP: bool> Drop for %sFromCpp<FROM_CPP> {' % (self.__model.unprefixed(),)
        yield '    fn drop(&mut self) {'
        yield '        if FROM_CPP {'
        yield '            unsafe { ffi::%s_delete(self.0) }' % (deleter_class,)
        yield '        }'
        yield '    }'
        yield '}'
    

    def _mixin_ffi_lines(self):
        mixins = list(self.__model.mixins())
        if not mixins:
            return
        yield '// Mix-in(s) to %s' % (self.__model.name,)
        for mixin in mixins:
            yield 'pub fn %s_As%s(obj: *mut c_void) -> *mut c_void;' % (
                self.__model.name,
                mixin[2:],
            )
    
    def _impl_mixin_if_needed(self):
        mixins = list(self.__model.mixins())
        if not mixins:
            return
        yield '// Mix-in(s) to %s' % (self.__model.name,)
        for mixin in mixins:
            for ancestor in self._ancestors_names_of(mixin):
                ancestor_unprefixed = ancestor[2:]
                yield 'impl<const FROM_CPP: bool> %sMethods for %sFromCpp<FROM_CPP> {' % (
                    ancestor_unprefixed,
                    self.__model.unprefixed(),
                )
                yield '    fn as_%s(&self) -> *mut c_void {' % (
                    pascal_to_snake(ancestor_unprefixed),
                )
                yield '        unsafe { ffi::%s_As%s(self.as_ptr()) }' % (
                    self.__model.name,
                    mixin[2:],
                )
                yield '    }'
                yield '}'
    
    def _ancestors_names_of(self, name):
        cm = self.__model.manager
        return (a.name for a in cm.ancestors_of(cm.by_name(name)))

    def _impl_non_virtual_overrides(self, non_virtual_overrides):
        for ancestor, overloads in non_virtual_overrides:
            if not overloads:
                continue
            yield 'impl<const FROM_CPP: bool> %sMethods for %sFromCpp<FROM_CPP> {' % (
                ancestor.unprefixed(),
                self.__model.unprefixed(),
            )
            ancestor_overloads = OverloadTree(ancestor)
            for method in overloads:
                for line in method.lines(
                    with_overloads=ancestor_overloads,
                ):
                    yield '    %s' % (line)
            yield '}'

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)
    
    def _trait_with_methods(self):
        indent = ' ' * 4 * 1
        base = self.__model.primary_base()
        if not base:
            base = '__WxRust'
        yield "    /// This trait represents [C++ `%s` class](%s)'s methods and inheritance." % (
            self.__model.name,
            self.__model.doc_url(),
        )
        yield '    ///'
        yield '    /// See [`%sFromCpp`] documentation for the class usage.' % (
            self.__model.unprefixed(),
        )
        yield 'pub trait %sMethods: %sMethods {' % (
            self.__model.unprefixed(),
            base[2:],
        )
        if self.mixed_into():
            yield '    fn %s(&self) -> *mut c_void {' % (
                self.as_mixin(),
            )
            yield '        unsafe { self.as_ptr() }'
            yield '    }'
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

        self._add_ancestors_methods(cls)
        
        # If `cls` is mixin class,
        mixed_into = cls.manager.mixed_into(cls.name)
        for mixed_in in mixed_into:
            # Consider mixed-in classes
            mixed_in = cls.manager.by_name(mixed_in)
            self._add_ancestors_methods(mixed_in)

            # and those (other) mix-ins to resolve overload methods 
            for mixin in mixed_in.mixins():
                mixin = cls.manager.by_name(mixin)
                self._add_ancestors_methods(mixin)
    
    def _add_ancestors_methods(self, cls):
        for c in self.__cls.manager.ancestors_of(cls):
            for m in c.methods:
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
        # print(method.cls.name)
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
    
    def print_tree(self, tofile):
        self.print_node(tofile, self.__root, 0)

    def print_node(self, tofile, node, level):
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
            print("%s- %s: %s: %s %s" % (indent, count, k, method, args),
                  file=tofile)
            self.print_node(tofile, v, level + 1)


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
        wrapped = self.__model.wrap_return_type(allows_ptr=True)
        if self.__model.maybe_returns_self():
            if for_ffi:
                returns = '*mut c_void'
            else:
                returns = '&Self'
        elif wrapped:
            if for_ffi:
                returns = '*mut c_void'
            else:
                returns = wrapped.returns()
        return ' -> %s' % (returns,)
    
    def lines(self, for_ffi=False, with_overloads=None):
        pub_or_not = 'pub '
        gen_params = ''
        name = self.__model.name(for_ffi=True)
        if not for_ffi:
            if not self.is_ctor:
                pub_or_not = '' if not self.is_ctor else 'pub '
            name = self._rust_method_name(with_overloads)
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
            doc = self.__model.doc
            if doc:
                yield '/// %s' % (doc,)
            yield '///'
            yield "/// See [C++ `%s::%s()`'s documentation](%s)." % (
                self.__model.cls.name,
                self.__model.name(without_index=True),
                self.__model.doc_url(),
            )
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
            self_param = self.__self_param.rust_ffi_ref(
                as_mixin=self.__cls.as_mixin(),
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
        # print(self.__model.name(), params)
        return ', '.join(params)

    def _rust_method_name(self, with_overloads):
        method_name = pascal_to_snake(self.__model.name(
            without_index=True,
        ))
        overloads = self.__cls.overloads
        if with_overloads:
            overloads = with_overloads
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
        if self.__model.maybe_returns_self():
            return '%s; &self' % (call,)
        wrapped = self.__model.wrap_return_type(allows_ptr=True)
        if wrapped:
            return wrapped.call(call)
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
                param.type.is_ref_to_binding()):
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

    def has_initial(self, i):
        return self.__model.has_initial(i)

    def include(self):
        condition = ''
        cond_name = self.__model.config.get('condition')
        if cond_name:
            condition = self.conditions[cond_name].get('cxx')
        return (
            '#include <%s>' % (self.__model.include,),
            condition,
        )

    def lines(self, is_cc=False):
        yield '// CLASS: %s' % (self.__model.name,)
        for line in self._dtor_lines(is_cc):
            yield line
        for line in self._class_info_lines(is_cc):
            yield line
        self.in_condition = None
        for method in self.__methods:
            for line in method.lines(is_cc):
                yield line
        if self.in_condition:
            yield '#endif'
        mixins = list(self.__model.mixins())
        if mixins:
            yield '// Mix-in(s) to %s' % (self.__model.name,)
        for mixin in mixins:
            for line in self._mixin_lines(mixin, is_cc=is_cc):
                yield line
        yield ''

    def _mixin_lines(self, mixin, is_cc):
        cxx_class = mixin
        if cxx_class in MIXIN_CXX_CLASS:
            cxx_class = MIXIN_CXX_CLASS[cxx_class]
        signature = '%s *%s_As%s(%s* obj)' % (
            cxx_class,
            self.__model.name,
            mixin[2:], # TODO: unprefixed
            self.__model.name,
        )
        if is_cc:
            yield '%s {' % (signature,)
            yield '    return static_cast<%s*>(obj);' % (cxx_class,)
            yield '}'
        else:
            yield '%s;' % (signature,)

    def _ctors(self):
        return (m for m in self.__methods if m.is_ctor)
    
    def _dtor_lines(self, is_cc):
        if (self.__model.manager.is_a(self.__model, 'wxObject') or
            self.__model.manager.is_a(self.__model, 'wxRefCounter') or
            # FIXME: wxObjectRefData is a typedef of wxRefCounter
            self.__model.manager.is_a(self.__model, 'wxObjectRefData') or
            self.__model.manager.is_a(self.__model, 'wxSharedClientDataContainer')):
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
    
    def _class_info_lines(self, is_cc):
        if not self.__model.manager.is_a(self.__model, 'wxObject'):
            return
        signature = 'wxClassInfo *%s_CLASSINFO()' % (
            self.__model.name,
        )
        if is_cc:
            yield '%s {' % (signature,)
            yield '    return wxCLASSINFO(%s);' % (self.__model.name,)
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
        wrapped = self.__model.wrap_return_type(allows_ptr=False)
        returns = self.__model.returns.in_cxx() + ' '
        if wrapped:
            returns = '%s *' % (
                wrapped.in_cxx(),
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
        if (wrapped or
              self.__model.returns.is_const_ref_to_binding()):
            yield '    return new %s(%s);' % (wrapped.in_cxx(), new_params_or_expr)
        elif (self.__model.maybe_returns_self() or
              self.__model.returns.is_ref_to_binding()):
            yield '    return &(%s);' % (new_params_or_expr,)
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

