use super::*;

// wxTimer
wx_class! {
    #[doc(alias = "wxTimer")]
    #[doc(alias = "Timer")]
    type Timer = TimerIsOwned<true>(wxTimer) impl
        TimerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerIsOwned<OWNED> {
    pub fn new() -> TimerIsOwned<OWNED> {
        unsafe { TimerIsOwned(ffi::wxTimer_new()) }
    }
    pub fn new_with_evthandler<E: EvtHandlerMethods>(
        owner: Option<&E>,
        id: c_int,
    ) -> TimerIsOwned<OWNED> {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TimerIsOwned(ffi::wxTimer_new1(owner, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TimerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TimerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TimerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TimerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTimer_CLASSINFO()) }
    }
}

// wxTimerEvent
wx_class! {
    #[doc(alias = "wxTimerEvent")]
    #[doc(alias = "TimerEvent")]
    type TimerEvent = TimerEventIsOwned<true>(wxTimerEvent) impl
        TimerEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerEventIsOwned<OWNED> {
    pub fn new<T: TimerMethods>(timer: &T) -> TimerEventIsOwned<OWNED> {
        unsafe {
            let timer = timer.as_ptr();
            TimerEventIsOwned(ffi::wxTimerEvent_new(timer))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TimerEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TimerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TimerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TimerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTimerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TimerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
