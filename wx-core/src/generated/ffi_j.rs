use super::*;

extern "C" {

    // wxJPEGHandler
    pub fn wxJPEGHandler_CLASSINFO() -> *mut c_void;
    pub fn wxJPEGHandler_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxJPEGHandler_GetLibraryVersionInfo() -> wxVersionInfo;

    // wxJoystick
    pub fn wxJoystick_CLASSINFO() -> *mut c_void;
    pub fn wxJoystick_new(joystick: c_int) -> *mut c_void;
    // DTOR: pub fn wxJoystick_~wxJoystick(self_: *mut c_void);
    pub fn wxJoystick_GetButtonState(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetButtonState1(self_: *const c_void, id: c_uint) -> bool;
    pub fn wxJoystick_GetManufacturerId(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetMovementThreshold(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetNumberAxes(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetNumberButtons(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPOVCTSPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPOVPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPollingMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPollingMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxJoystick_GetPosition1(self_: *const c_void, axis: c_uint) -> c_int;
    pub fn wxJoystick_GetProductId(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetProductName(self_: *const c_void) -> *mut c_void;
    pub fn wxJoystick_GetRudderMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetRudderMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetRudderPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetUMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetUMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetUPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetVMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetVMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetVPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetXMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetXMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetYMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetYMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetZMax(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetZMin(self_: *const c_void) -> c_int;
    pub fn wxJoystick_GetZPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystick_HasPOV(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasPOV4Dir(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasPOVCTS(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasRudder(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasU(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasV(self_: *const c_void) -> bool;
    pub fn wxJoystick_HasZ(self_: *const c_void) -> bool;
    pub fn wxJoystick_IsOk(self_: *const c_void) -> bool;
    pub fn wxJoystick_ReleaseCapture(self_: *mut c_void) -> bool;
    pub fn wxJoystick_SetCapture(self_: *mut c_void, win: *mut c_void, polling_freq: c_int)
        -> bool;
    pub fn wxJoystick_SetMovementThreshold(self_: *mut c_void, threshold: c_int);
    pub fn wxJoystick_GetNumberJoysticks() -> c_int;

    // wxJoystickEvent
    pub fn wxJoystickEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxJoystickEvent_new(event_type: wxEventType, state: c_int, joystick: c_int, change: c_int) -> *mut c_void;
    pub fn wxJoystickEvent_ButtonDown(self_: *const c_void, button: c_int) -> bool;
    pub fn wxJoystickEvent_ButtonIsDown(self_: *const c_void, button: c_int) -> bool;
    pub fn wxJoystickEvent_ButtonUp(self_: *const c_void, button: c_int) -> bool;
    pub fn wxJoystickEvent_GetButtonChange(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetButtonOrdinal(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetButtonState(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetJoystick(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxJoystickEvent_GetZPosition(self_: *const c_void) -> c_int;
    pub fn wxJoystickEvent_IsButton(self_: *const c_void) -> bool;
    pub fn wxJoystickEvent_IsMove(self_: *const c_void) -> bool;
    pub fn wxJoystickEvent_IsZMove(self_: *const c_void) -> bool;

}
