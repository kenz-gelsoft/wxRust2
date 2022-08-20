#![allow(non_upper_case_globals)]

use super::*;

// wxKeyEvent
wx_class! { KeyEvent =
    KeyEventIsOwned<true>(wxKeyEvent) impl
        KeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> KeyEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxKeyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<KeyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: KeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<KeyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: KeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for KeyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxKeyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for KeyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
