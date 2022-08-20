use super::*;

// wxKeyEvent
pub trait KeyEventMethods: EventMethods {
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetKeyCode(self.as_ptr()) }
    }
    fn is_key_in_category(&self, category: c_int) -> bool {
        unsafe { ffi::wxKeyEvent_IsKeyInCategory(self.as_ptr(), category) }
    }
    fn is_auto_repeat(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsAutoRepeat(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxKeyEvent_GetPosition(self.as_ptr())) }
    }
    fn get_position_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxKeyEvent_GetPosition1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetRawKeyCode()
    // NOT_SUPPORTED: fn GetRawKeyFlags()
    // NOT_SUPPORTED: fn GetUnicodeKey()
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetY(self.as_ptr()) }
    }
    fn do_allow_next_event(&self) {
        unsafe { ffi::wxKeyEvent_DoAllowNextEvent(self.as_ptr()) }
    }
    fn is_next_event_allowed(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsNextEventAllowed(self.as_ptr()) }
    }
}
