use super::*;

// wxKeyEvent
wxwidgets! {
    /// This event class contains information about key press and release events.
    /// - [`KeyEvent`] represents a C++ `wxKeyEvent` class instance which your code has ownership, [`KeyEventInRust`]`<false>` represents one which don't own.
    /// - Use [`KeyEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxKeyEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_key_event.html) for more details.
    #[doc(alias = "wxKeyEvent")]
    #[doc(alias = "KeyEvent")]
    class KeyEvent
        = KeyEventInRust<true>(wxKeyEvent) impl
        KeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> KeyEventInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxKeyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for KeyEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<KeyEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: KeyEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<KeyEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: KeyEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for KeyEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxKeyEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for KeyEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
