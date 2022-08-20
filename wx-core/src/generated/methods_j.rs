use super::*;

// wxJPEGHandler
pub trait JPEGHandlerMethods: ImageHandlerMethods {}

// wxJoystick
pub trait JoystickMethods: ObjectMethods {
    // DTOR: fn ~wxJoystick()
    fn get_button_state(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetButtonState(self.as_ptr()) }
    }
    fn get_button_state_uint(&self, id: c_uint) -> bool {
        unsafe { ffi::wxJoystick_GetButtonState1(self.as_ptr(), id) }
    }
    fn get_manufacturer_id(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetManufacturerId(self.as_ptr()) }
    }
    fn get_movement_threshold(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetMovementThreshold(self.as_ptr()) }
    }
    fn get_number_axes(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetNumberAxes(self.as_ptr()) }
    }
    fn get_number_buttons(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetNumberButtons(self.as_ptr()) }
    }
    fn get_povcts_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPOVCTSPosition(self.as_ptr()) }
    }
    fn get_pov_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPOVPosition(self.as_ptr()) }
    }
    fn get_polling_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPollingMax(self.as_ptr()) }
    }
    fn get_polling_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPollingMin(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxJoystick_GetPosition(self.as_ptr())) }
    }
    fn get_position_uint(&self, axis: c_uint) -> c_int {
        unsafe { ffi::wxJoystick_GetPosition1(self.as_ptr(), axis) }
    }
    fn get_product_id(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetProductId(self.as_ptr()) }
    }
    fn get_product_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxJoystick_GetProductName(self.as_ptr())).into() }
    }
    fn get_rudder_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetRudderMax(self.as_ptr()) }
    }
    fn get_rudder_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetRudderMin(self.as_ptr()) }
    }
    fn get_rudder_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetRudderPosition(self.as_ptr()) }
    }
    fn get_u_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetUMax(self.as_ptr()) }
    }
    fn get_u_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetUMin(self.as_ptr()) }
    }
    fn get_u_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetUPosition(self.as_ptr()) }
    }
    fn get_v_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetVMax(self.as_ptr()) }
    }
    fn get_v_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetVMin(self.as_ptr()) }
    }
    fn get_v_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetVPosition(self.as_ptr()) }
    }
    fn get_x_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetXMax(self.as_ptr()) }
    }
    fn get_x_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetXMin(self.as_ptr()) }
    }
    fn get_y_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetYMax(self.as_ptr()) }
    }
    fn get_y_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetYMin(self.as_ptr()) }
    }
    fn get_z_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetZMax(self.as_ptr()) }
    }
    fn get_z_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetZMin(self.as_ptr()) }
    }
    fn get_z_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetZPosition(self.as_ptr()) }
    }
    fn has_pov(&self) -> bool {
        unsafe { ffi::wxJoystick_HasPOV(self.as_ptr()) }
    }
    fn has_po_v4_dir(&self) -> bool {
        unsafe { ffi::wxJoystick_HasPOV4Dir(self.as_ptr()) }
    }
    fn has_povcts(&self) -> bool {
        unsafe { ffi::wxJoystick_HasPOVCTS(self.as_ptr()) }
    }
    fn has_rudder(&self) -> bool {
        unsafe { ffi::wxJoystick_HasRudder(self.as_ptr()) }
    }
    fn has_u(&self) -> bool {
        unsafe { ffi::wxJoystick_HasU(self.as_ptr()) }
    }
    fn has_v(&self) -> bool {
        unsafe { ffi::wxJoystick_HasV(self.as_ptr()) }
    }
    fn has_z(&self) -> bool {
        unsafe { ffi::wxJoystick_HasZ(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxJoystick_IsOk(self.as_ptr()) }
    }
    fn release_capture(&self) -> bool {
        unsafe { ffi::wxJoystick_ReleaseCapture(self.as_ptr()) }
    }
    fn set_capture<W: WindowMethods>(&self, win: Option<&W>, polling_freq: c_int) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxJoystick_SetCapture(self.as_ptr(), win, polling_freq)
        }
    }
    fn set_movement_threshold(&self, threshold: c_int) {
        unsafe { ffi::wxJoystick_SetMovementThreshold(self.as_ptr(), threshold) }
    }
    fn get_number_joysticks() -> c_int {
        unsafe { ffi::wxJoystick_GetNumberJoysticks() }
    }
}

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
