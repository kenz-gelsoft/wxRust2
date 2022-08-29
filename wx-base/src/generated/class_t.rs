use super::*;

// wxTimer
wxwidgets! {
    /// The wxTimer class allows you to execute code at specified intervals.
    /// - [`Timer`] represents a C++ `wxTimer` class instance which your code has ownership, [`TimerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Timer`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxTimer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html) for more details.
    #[doc(alias = "wxTimer")]
    #[doc(alias = "Timer")]
    class Timer
        = TimerIsOwned<true>(wxTimer) impl
        TimerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#a0560894a1ab57792f52992ffbc58e735).
    pub fn new() -> TimerIsOwned<OWNED> {
        unsafe { TimerIsOwned(ffi::wxTimer_new()) }
    }
    /// Creates a timer and associates it with owner.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#aeba595e67731d9f8ce07e4ac69a0ca65).
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
wxwidgets! {
    /// wxTimerEvent object is passed to the event handler of timer events (see wxTimer::SetOwner).
    /// - [`TimerEvent`] represents a C++ `wxTimerEvent` class instance which your code has ownership, [`TimerEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TimerEvent`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxTimerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html) for more details.
    #[doc(alias = "wxTimerEvent")]
    #[doc(alias = "TimerEvent")]
    class TimerEvent
        = TimerEventIsOwned<true>(wxTimerEvent) impl
        TimerEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerEventIsOwned<OWNED> {
    ///
    /// See [C++ `wxTimerEvent::wxTimerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html#aedca4f26719c256c2f8d9ef8486d2f86).
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
