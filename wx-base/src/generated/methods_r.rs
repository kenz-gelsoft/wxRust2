use super::*;

// wxRefCounter
pub trait RefCounterMethods: WxRustMethods {
    fn dec_ref(&self) {
        unsafe { ffi::wxRefCounter_DecRef(self.as_ptr()) }
    }
    fn get_ref_count(&self) -> c_int {
        unsafe { ffi::wxRefCounter_GetRefCount(self.as_ptr()) }
    }
    fn inc_ref(&self) {
        unsafe { ffi::wxRefCounter_IncRef(self.as_ptr()) }
    }
}
