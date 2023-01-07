use super::*;

// wxKeyEvent
wxwidgets! {
    /// This event class contains information about key press and release events.
    /// - [`KeyEvent`] represents a C++ `wxKeyEvent` class instance which your code has ownership, [`KeyEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`KeyEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxKeyEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_key_event.html) for more details.
    #[doc(alias = "wxKeyEvent")]
    #[doc(alias = "KeyEvent")]
    class KeyEvent
        = KeyEventFromCpp<false>(wxKeyEvent) impl
        KeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> KeyEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxKeyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for KeyEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<KeyEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: KeyEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<KeyEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: KeyEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for KeyEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxKeyEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for KeyEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
