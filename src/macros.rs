macro_rules! wx_class {
    (
        $type:ident($wxType:ident) impl $($methods:ident),*
    ) => {
        #[derive(Clone)]
        pub struct $type(*mut ffi::$wxType);
        $(
            impl $methods for $type {}
        )*
        impl ObjectMethods for $type {
            unsafe fn as_ptr(&self) -> UnsafeAnyPtr { self.0 as _ }
        }
    };
}
pub(crate) use wx_class;
