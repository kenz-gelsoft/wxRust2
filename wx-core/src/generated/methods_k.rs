use super::*;

// wxKeyEvent
pub trait KeyEventMethods: EventMethods {
    /// Returns the key code of the key that generated this event.
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetKeyCode(self.as_ptr()) }
    }
    /// Returns true if the key is in the given key category.
    fn is_key_in_category(&self, category: c_int) -> bool {
        unsafe { ffi::wxKeyEvent_IsKeyInCategory(self.as_ptr(), category) }
    }
    /// Returns true if this event is an auto-repeat of the key, false if this is the initial key press.
    fn is_auto_repeat(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsAutoRepeat(self.as_ptr()) }
    }
    /// Obtains the position (in client coordinates) at which the key was pressed.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxKeyEvent_GetPosition(self.as_ptr())) }
    }
    fn get_position_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxKeyEvent_GetPosition1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetRawKeyCode()
    // NOT_SUPPORTED: fn GetRawKeyFlags()
    // NOT_SUPPORTED: fn GetUnicodeKey()
    /// Returns the X position (in client coordinates) of the event.
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetX(self.as_ptr()) }
    }
    /// Returns the Y position (in client coordinates) of the event.
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetY(self.as_ptr()) }
    }
    /// Allow normal key events generation.
    fn do_allow_next_event(&self) {
        unsafe { ffi::wxKeyEvent_DoAllowNextEvent(self.as_ptr()) }
    }
    /// Returns true if DoAllowNextEvent() had been called, false by default.
    fn is_next_event_allowed(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsNextEventAllowed(self.as_ptr()) }
    }
}
