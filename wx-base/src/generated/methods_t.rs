use super::*;

// wxTimer
pub trait TimerMethods: EvtHandlerMethods {
    // DTOR: fn ~wxTimer()
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxTimer_GetId(self.as_ptr()) }
    }
    fn get_interval(&self) -> c_int {
        unsafe { ffi::wxTimer_GetInterval(self.as_ptr()) }
    }
    fn get_owner(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxTimer_GetOwner(self.as_ptr())) }
    }
    fn is_one_shot(&self) -> bool {
        unsafe { ffi::wxTimer_IsOneShot(self.as_ptr()) }
    }
    fn is_running(&self) -> bool {
        unsafe { ffi::wxTimer_IsRunning(self.as_ptr()) }
    }
    fn notify(&self) {
        unsafe { ffi::wxTimer_Notify(self.as_ptr()) }
    }
    fn set_owner<E: EvtHandlerMethods>(&self, owner: Option<&E>, id: c_int) {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTimer_SetOwner(self.as_ptr(), owner, id)
        }
    }
    fn start(&self, milliseconds: c_int, one_shot: bool) -> bool {
        unsafe { ffi::wxTimer_Start(self.as_ptr(), milliseconds, one_shot) }
    }
    fn start_once(&self, milliseconds: c_int) -> bool {
        unsafe { ffi::wxTimer_StartOnce(self.as_ptr(), milliseconds) }
    }
    fn stop(&self) {
        unsafe { ffi::wxTimer_Stop(self.as_ptr()) }
    }
}

// wxTimerEvent
pub trait TimerEventMethods: EventMethods {
    fn get_interval(&self) -> c_int {
        unsafe { ffi::wxTimerEvent_GetInterval(self.as_ptr()) }
    }
    fn get_timer(&self) -> TimerIsOwned<false> {
        unsafe { TimerIsOwned::from_ptr(ffi::wxTimerEvent_GetTimer(self.as_ptr())) }
    }
}
