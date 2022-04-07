#[macro_export]
macro_rules! wx_class {
    (
        $type:ident($wxType:ident) impl $($methods:ident),*
    ) => {
        #[derive(Clone)]
        pub struct $type(*mut c_void);
        $(
            impl $methods for $type {}
        )*
        impl WxRustMethods for $type {
            unsafe fn as_ptr(&self) -> *mut c_void { self.0 }
            unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F) {
                let tmp = Self(ptr);
                closure(&tmp);
                mem::forget(tmp);
            }
        }
    };
}
