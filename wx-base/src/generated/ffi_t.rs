use super::*;

extern "C" {

    // wxTimer
    pub fn wxTimer_CLASSINFO() -> *mut c_void;
    pub fn wxTimer_new() -> *mut c_void;
    pub fn wxTimer_new1(owner: *mut c_void, id: c_int) -> *mut c_void;
    // DTOR: pub fn wxTimer_~wxTimer(self_: *mut c_void);
    pub fn wxTimer_GetId(self_: *const c_void) -> c_int;
    pub fn wxTimer_GetInterval(self_: *const c_void) -> c_int;
    pub fn wxTimer_GetOwner(self_: *const c_void) -> *mut c_void;
    pub fn wxTimer_IsOneShot(self_: *const c_void) -> bool;
    pub fn wxTimer_IsRunning(self_: *const c_void) -> bool;
    pub fn wxTimer_Notify(self_: *mut c_void);
    pub fn wxTimer_SetOwner(self_: *mut c_void, owner: *mut c_void, id: c_int);
    pub fn wxTimer_Start(self_: *mut c_void, milliseconds: c_int, one_shot: bool) -> bool;
    pub fn wxTimer_StartOnce(self_: *mut c_void, milliseconds: c_int) -> bool;
    pub fn wxTimer_Stop(self_: *mut c_void);

    // wxTimerEvent
    pub fn wxTimerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxTimerEvent_new(timer: *mut c_void) -> *mut c_void;
    pub fn wxTimerEvent_GetInterval(self_: *const c_void) -> c_int;
    pub fn wxTimerEvent_GetTimer(self_: *const c_void) -> *mut c_void;

}
