#[macro_export]
macro_rules! wxwidgets {
    (
        $(#[doc = $docComment:tt])*
        $(#[doc($docAttrKey:ident = $docAttrValue:tt)])*
        class $type:ident
            = $typeIsOwned:ident<true>($wxType:ident) impl
            $($methods:ident),*
    ) => {
        $(#[doc = $docComment])*
        $(#[doc($docAttrKey = $docAttrValue)])*
        pub struct $typeIsOwned<const OWNED: bool>(*mut c_void);
        $(#[doc = $docComment])*
        pub type $type = $typeIsOwned<true>;
        $(
            impl<const OWNED: bool> $methods for $typeIsOwned<OWNED> {}
        )*
        impl<const OWNED: bool> WxRustMethods for $typeIsOwned<OWNED> {
            type Unowned = $typeIsOwned<false>;
            unsafe fn as_ptr(&self) -> *mut c_void { self.0 }
            unsafe fn from_ptr(ptr: *mut c_void) -> Self {
                $typeIsOwned(ptr)
            }
            unsafe fn from_unowned_ptr(ptr: *mut c_void) -> Self::Unowned {
                $typeIsOwned::<false>(ptr)
            }
            unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F) {
                let tmp = Self(ptr);
                closure(&tmp);
                mem::forget(tmp);
            }
        }
    };
}
