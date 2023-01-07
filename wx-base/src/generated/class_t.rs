use super::*;

// wxTimer
wxwidgets! {
    /// The wxTimer class allows you to execute code at specified intervals.
    /// - [`Timer`] represents a C++ `wxTimer` class instance which your code has ownership, [`TimerInRust`]`<false>` represents one which don't own.
    /// - Use [`Timer`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxTimer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html) for more details.
    #[doc(alias = "wxTimer")]
    #[doc(alias = "Timer")]
    class Timer
        = TimerInRust<true>(wxTimer) impl
        TimerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#a0560894a1ab57792f52992ffbc58e735).
    pub fn new() -> TimerInRust<OWNED> {
        unsafe { TimerInRust(ffi::wxTimer_new()) }
    }
    /// Creates a timer and associates it with owner.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#aeba595e67731d9f8ce07e4ac69a0ca65).
    pub fn new_with_evthandler<E: EvtHandlerMethods>(
        owner: Option<&E>,
        id: c_int,
    ) -> TimerInRust<OWNED> {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TimerInRust(ffi::wxTimer_new1(owner, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TimerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TimerInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TimerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TimerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTimer_CLASSINFO()) }
    }
}

// wxTimerEvent
wxwidgets! {
    /// wxTimerEvent object is passed to the event handler of timer events (see wxTimer::SetOwner).
    /// - [`TimerEvent`] represents a C++ `wxTimerEvent` class instance which your code has ownership, [`TimerEventInRust`]`<false>` represents one which don't own.
    /// - Use [`TimerEvent`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxTimerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html) for more details.
    #[doc(alias = "wxTimerEvent")]
    #[doc(alias = "TimerEvent")]
    class TimerEvent
        = TimerEventInRust<true>(wxTimerEvent) impl
        TimerEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerEventInRust<OWNED> {
    ///
    /// See [C++ `wxTimerEvent::wxTimerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html#aedca4f26719c256c2f8d9ef8486d2f86).
    pub fn new<T: TimerMethods>(timer: &T) -> TimerEventInRust<OWNED> {
        unsafe {
            let timer = timer.as_ptr();
            TimerEventInRust(ffi::wxTimerEvent_new(timer))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TimerEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TimerEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: TimerEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimerEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TimerEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimerEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTimerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TimerEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
