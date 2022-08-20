use super::*;

extern "C" {

    // wxJPEGHandler
    pub fn wxJPEGHandler_CLASSINFO() -> *mut c_void;
    pub fn wxJPEGHandler_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxJPEGHandler_GetLibraryVersionInfo() -> wxVersionInfo;

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
