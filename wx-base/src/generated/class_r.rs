use super::*;

// wxRefCounter
wx_class! { RefCounter =
    RefCounterIsOwned<true>(wxRefCounter) impl
        RefCounterMethods
}
impl<const OWNED: bool> RefCounterIsOwned<OWNED> {
    pub fn new() -> RefCounterIsOwned<OWNED> {
        unsafe { RefCounterIsOwned(ffi::wxRefCounter_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RefCounterIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for RefCounterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRefCounter_delete(self.0) }
        }
    }
}
