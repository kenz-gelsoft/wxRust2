#[macro_export]
macro_rules! wxwidgets {
    (
        $(#[doc = $docComment:tt])*
        $(#[doc($docAttrKey:ident = $docAttrValue:tt)])*
        class $type:ident
            = $typeFromCpp:ident<true>($wxType:ident) impl
            $($methods:ident),*
    ) => {
        $(#[doc = $docComment])*
        $(#[doc($docAttrKey = $docAttrValue)])*
        pub struct $typeFromCpp<const FROM_CPP: bool>(*mut c_void);
        $(#[doc = $docComment])*
        pub type $type = $typeFromCpp<true>;
        $(
            impl<const FROM_CPP: bool> $methods for $typeFromCpp<FROM_CPP> {}
        )*
        impl<const FROM_CPP: bool> WxRustMethods for $typeFromCpp<FROM_CPP> {
            type CppManaged = $typeFromCpp<false>;
            unsafe fn as_ptr(&self) -> *mut c_void { self.0 }
            unsafe fn from_ptr(ptr: *mut c_void) -> Self {
                $typeFromCpp(ptr)
            }
            unsafe fn from_cpp_managed_ptr(ptr: *mut c_void) -> Self::CppManaged {
                $typeFromCpp::<false>(ptr)
            }
            unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F) {
                let tmp = Self(ptr);
                closure(&tmp);
                mem::forget(tmp);
            }
        }
    };
}
