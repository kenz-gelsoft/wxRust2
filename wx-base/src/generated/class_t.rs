use super::*;

// wxTimer
wx_class! { Timer =
    TimerIsOwned<true>(wxTimer) impl
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
impl<const OWNED: bool> Trackable<TimerIsOwned<false>> for TimerIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<TimerIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxTimerEvent
wx_class! { TimerEvent =
    TimerEventIsOwned<true>(wxTimerEvent) impl
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
