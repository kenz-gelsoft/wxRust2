use super::*;

// wxJPEGHandler
pub trait JPEGHandlerMethods: ImageHandlerMethods {}

// wxJoystickEvent
pub trait JoystickEventMethods: EventMethods {
    fn button_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonDown(self.as_ptr(), button) }
    }
    fn button_is_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonIsDown(self.as_ptr(), button) }
    }
    fn button_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonUp(self.as_ptr(), button) }
    }
    fn get_button_change(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonChange(self.as_ptr()) }
    }
    fn get_button_ordinal(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonOrdinal(self.as_ptr()) }
    }
    fn get_button_state(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonState(self.as_ptr()) }
    }
    fn get_joystick(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetJoystick(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxJoystickEvent_GetPosition(self.as_ptr())) }
    }
    fn get_z_position(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetZPosition(self.as_ptr()) }
    }
    fn is_button(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsButton(self.as_ptr()) }
    }
    fn is_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsMove(self.as_ptr()) }
    }
    fn is_z_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsZMove(self.as_ptr()) }
    }
}
