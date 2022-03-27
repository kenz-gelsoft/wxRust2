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
        }
    };
}
pub(crate) use wx_class;
