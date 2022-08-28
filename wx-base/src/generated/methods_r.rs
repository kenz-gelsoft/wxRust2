use super::*;

// wxRefCounter
pub trait RefCounterMethods: WxRustMethods {
    /// Decrements the reference count associated with this shared data and, if it reaches zero, destroys this instance of wxRefCounter releasing its memory.
    fn dec_ref(&self) {
        unsafe { ffi::wxRefCounter_DecRef(self.as_ptr()) }
    }
    /// Returns the reference count associated with this shared data.
    fn get_ref_count(&self) -> c_int {
        unsafe { ffi::wxRefCounter_GetRefCount(self.as_ptr()) }
    }
    /// Increments the reference count associated with this shared data.
    fn inc_ref(&self) {
        unsafe { ffi::wxRefCounter_IncRef(self.as_ptr()) }
    }
}
