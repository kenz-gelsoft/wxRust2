use super::*;

// wxRefCounter
pub trait RefCounterMethods: WxRustMethods {
    fn as_ref_counter(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    fn dec_ref(&self) {
        unsafe { ffi::wxRefCounter_DecRef(self.as_ref_counter()) }
    }
    fn get_ref_count(&self) -> c_int {
        unsafe { ffi::wxRefCounter_GetRefCount(self.as_ref_counter()) }
    }
    fn inc_ref(&self) {
        unsafe { ffi::wxRefCounter_IncRef(self.as_ref_counter()) }
    }
}
