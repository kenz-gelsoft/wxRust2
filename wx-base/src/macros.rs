#[macro_export]
macro_rules! wxwidgets {
    (
        $(#[doc = $docComment:tt])*
        $(#[doc($docAttrKey:ident = $docAttrValue:tt)])*
        class $type:ident
            = $typeInRust:ident<true>($wxType:ident) impl
            $($methods:ident),*
    ) => {
        $(#[doc = $docComment])*
        $(#[doc($docAttrKey = $docAttrValue)])*
        pub struct $typeInRust<const IN_RUST: bool>(*mut c_void);
        $(#[doc = $docComment])*
        pub type $type = $typeInRust<true>;
        $(
            impl<const IN_RUST: bool> $methods for $typeInRust<IN_RUST> {}
        )*
        impl<const IN_RUST: bool> WxRustMethods for $typeInRust<IN_RUST> {
            type CppManaged = $typeInRust<false>;
            unsafe fn as_ptr(&self) -> *mut c_void { self.0 }
            unsafe fn from_ptr(ptr: *mut c_void) -> Self {
                $typeInRust(ptr)
            }
            unsafe fn from_unowned_ptr(ptr: *mut c_void) -> Self::CppManaged {
                $typeInRust::<false>(ptr)
            }
            unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F) {
                let tmp = Self(ptr);
                closure(&tmp);
                mem::forget(tmp);
            }
        }
    };
}
