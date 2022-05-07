#[macro_export]
macro_rules! wx_class {
    (
        $type:ident($wxType:ident) impl $($methods:ident),*
    ) => {
        #[derive(Clone)]
        pub struct $type<const Owned: bool>(*mut c_void);
        $(
            impl<const Owned: bool> $methods for $type<Owned> {}
        )*
        impl<const Owned: bool> WxRustMethods for $type<Owned> {
            unsafe fn as_ptr(&self) -> *mut c_void { self.0 }
            unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F) {
                let tmp = Self(ptr);
                closure(&tmp);
                mem::forget(tmp);
            }
        }
    };
}
