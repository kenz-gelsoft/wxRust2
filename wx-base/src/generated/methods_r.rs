use super::*;

// wxRefCounter
/// This class is used to manage reference-counting providing a simple interface and a counter.
///
/// [See `wxRefCounter`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_ref_counter.html)
pub trait RefCounterMethods: WxRustMethods {
    /// Decrements the reference count associated with this shared data and, if it reaches zero, destroys this instance of wxRefCounter releasing its memory.
    ///
    /// [See `wxRefCounter::DecRef()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_ref_counter.html#a803eb5be907b1a342082ceb59c01d8c5)
    fn dec_ref(&self) {
        unsafe { ffi::wxRefCounter_DecRef(self.as_ptr()) }
    }
    /// Returns the reference count associated with this shared data.
    ///
    /// [See `wxRefCounter::GetRefCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_ref_counter.html#a98ca5fc3fa6219d993dd3654925c46aa)
    fn get_ref_count(&self) -> c_int {
        unsafe { ffi::wxRefCounter_GetRefCount(self.as_ptr()) }
    }
    /// Increments the reference count associated with this shared data.
    ///
    /// [See `wxRefCounter::IncRef()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_ref_counter.html#a9fec1fb6f778d9df7a8c046ad6a2d887)
    fn inc_ref(&self) {
        unsafe { ffi::wxRefCounter_IncRef(self.as_ptr()) }
    }
}
