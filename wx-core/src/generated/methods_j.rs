use super::*;

// wxJPEGHandler
/// This trait represents C++ [`wxJPEGHandler`](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html) class's methods and inheritance.
///
/// See [`JPEGHandlerIsOwned`] documentation for the class usage.
pub trait JPEGHandlerMethods: ImageHandlerMethods {}

// wxJoystickEvent
/// This trait represents C++ [`wxJoystickEvent`](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html) class's methods and inheritance.
///
/// See [`JoystickEventIsOwned`] documentation for the class usage.
pub trait JoystickEventMethods: EventMethods {
    /// Returns true if the event was a down event from the specified button (or any button).
    ///
    /// [See `wxJoystickEvent::ButtonDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a6aba0f71ebfbd97dfb1fc913e2c6ea25)
    fn button_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonDown(self.as_ptr(), button) }
    }
    /// Returns true if the specified button (or any button) was in a down state.
    ///
    /// [See `wxJoystickEvent::ButtonIsDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#abf913a82532d130fb927cbe86cca9f75)
    fn button_is_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonIsDown(self.as_ptr(), button) }
    }
    /// Returns true if the event was an up event from the specified button (or any button).
    ///
    /// [See `wxJoystickEvent::ButtonUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a725afccfc686c1e9f9cd749a623bed5c)
    fn button_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonUp(self.as_ptr(), button) }
    }
    /// Returns the identifier of the button changing state.
    ///
    /// [See `wxJoystickEvent::GetButtonChange()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a57d3db808310762c48110cbc0a037e84)
    fn get_button_change(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonChange(self.as_ptr()) }
    }
    /// Returns the 0-indexed ordinal of the button changing state.
    ///
    /// [See `wxJoystickEvent::GetButtonOrdinal()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#ad3533a7786c45b18e2876ec8fe00387d)
    fn get_button_ordinal(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonOrdinal(self.as_ptr()) }
    }
    /// Returns the down state of the buttons.
    ///
    /// [See `wxJoystickEvent::GetButtonState()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#ab29565e693a57421fb093d2e9a92062a)
    fn get_button_state(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonState(self.as_ptr()) }
    }
    /// Returns the identifier of the joystick generating the event - one of wxJOYSTICK1 and wxJOYSTICK2.
    ///
    /// [See `wxJoystickEvent::GetJoystick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a040f8a4ba089ccb9aef7e900bcb787b4)
    fn get_joystick(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetJoystick(self.as_ptr()) }
    }
    /// Returns the x, y position of the joystick event.
    ///
    /// [See `wxJoystickEvent::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a9c7b6c38ec5b0e6289ca81354570f7a3)
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxJoystickEvent_GetPosition(self.as_ptr())) }
    }
    /// Returns the z position of the joystick event.
    ///
    /// [See `wxJoystickEvent::GetZPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a48a676ed39ed44f3da674756009e0753)
    fn get_z_position(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetZPosition(self.as_ptr()) }
    }
    /// Returns true if this was a button up or down event (not 'is any button down?').
    ///
    /// [See `wxJoystickEvent::IsButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a85300818ce1dbe756ebd7ed78a1d1806)
    fn is_button(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsButton(self.as_ptr()) }
    }
    /// Returns true if this was an x, y move event.
    ///
    /// [See `wxJoystickEvent::IsMove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#a6c403c643f37ac83d5bb690a012dee76)
    fn is_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsMove(self.as_ptr()) }
    }
    /// Returns true if this was a z move event.
    ///
    /// [See `wxJoystickEvent::IsZMove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html#ac7e5dabc99330135cb5ef31317217053)
    fn is_z_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsZMove(self.as_ptr()) }
    }
}
