use super::*;

extern "C" {

    // wxKeyEvent
    pub fn wxKeyEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxKeyEvent_new(key_event_type: wxEventType) -> *mut c_void;
    pub fn wxKeyEvent_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxKeyEvent_IsKeyInCategory(self_: *const c_void, category: c_int) -> bool;
    pub fn wxKeyEvent_IsAutoRepeat(self_: *const c_void) -> bool;
    pub fn wxKeyEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxKeyEvent_GetPosition1(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    // NOT_SUPPORTED: pub fn wxKeyEvent_GetRawKeyCode(self_: *const c_void) -> wxUint32;
    // NOT_SUPPORTED: pub fn wxKeyEvent_GetRawKeyFlags(self_: *const c_void) -> wxUint32;
    // NOT_SUPPORTED: pub fn wxKeyEvent_GetUnicodeKey(self_: *const c_void) -> wxChar;
    pub fn wxKeyEvent_GetX(self_: *const c_void) -> c_int;
    pub fn wxKeyEvent_GetY(self_: *const c_void) -> c_int;
    pub fn wxKeyEvent_DoAllowNextEvent(self_: *mut c_void);
    pub fn wxKeyEvent_IsNextEventAllowed(self_: *const c_void) -> bool;

}
