#[macro_export]
macro_rules! wx_class {
    (
        $type:ident = $typeIsOwned:ident<true>($wxType:ident) impl $($methods:ident),*
    ) => {
        #[derive(Clone)]
        pub struct $typeIsOwned<const OWNED: bool>(*mut c_void);
        pub type $type = $typeIsOwned<true>;
        $(
            impl<const OWNED: bool> $methods for $typeIsOwned<OWNED> {}
        )*
        impl<const OWNED: bool> WxRustMethods for $typeIsOwned<OWNED> {
            unsafe fn as_ptr(&self) -> *mut c_void { self.0 }
            unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F) {
                let tmp = Self(ptr);
                closure(&tmp);
                mem::forget(tmp);
            }
        }
    };
}
