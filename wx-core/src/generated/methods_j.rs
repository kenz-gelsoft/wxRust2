use super::*;

// wxJPEGHandler
pub trait JPEGHandlerMethods: ImageHandlerMethods {}

// wxJoystickEvent
pub trait JoystickEventMethods: EventMethods {
    /// Returns true if the event was a down event from the specified button (or any button).
    fn button_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonDown(self.as_ptr(), button) }
    }
    /// Returns true if the specified button (or any button) was in a down state.
    fn button_is_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonIsDown(self.as_ptr(), button) }
    }
    /// Returns true if the event was an up event from the specified button (or any button).
    fn button_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonUp(self.as_ptr(), button) }
    }
    /// Returns the identifier of the button changing state.
    fn get_button_change(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonChange(self.as_ptr()) }
    }
    /// Returns the 0-indexed ordinal of the button changing state.
    fn get_button_ordinal(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonOrdinal(self.as_ptr()) }
    }
    /// Returns the down state of the buttons.
    fn get_button_state(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonState(self.as_ptr()) }
    }
    /// Returns the identifier of the joystick generating the event - one of wxJOYSTICK1 and wxJOYSTICK2.
    fn get_joystick(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetJoystick(self.as_ptr()) }
    }
    /// Returns the x, y position of the joystick event.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxJoystickEvent_GetPosition(self.as_ptr())) }
    }
    /// Returns the z position of the joystick event.
    fn get_z_position(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetZPosition(self.as_ptr()) }
    }
    /// Returns true if this was a button up or down event (not 'is any button down?').
    fn is_button(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsButton(self.as_ptr()) }
    }
    /// Returns true if this was an x, y move event.
    fn is_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsMove(self.as_ptr()) }
    }
    /// Returns true if this was a z move event.
    fn is_z_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsZMove(self.as_ptr()) }
    }
}
