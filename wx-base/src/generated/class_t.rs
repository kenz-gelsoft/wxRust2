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
impl<const IN_RUST: bool> TimerInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#a0560894a1ab57792f52992ffbc58e735).
    pub fn new() -> TimerInRust<IN_RUST> {
        unsafe { TimerInRust(ffi::wxTimer_new()) }
    }
    /// Creates a timer and associates it with owner.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#aeba595e67731d9f8ce07e4ac69a0ca65).
    pub fn new_with_evthandler<E: EvtHandlerMethods>(
        owner: Option<&E>,
        id: c_int,
    ) -> TimerInRust<IN_RUST> {
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
impl<const IN_RUST: bool> Clone for TimerInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<TimerInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: TimerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<TimerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: TimerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for TimerInRust<IN_RUST> {
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
impl<const IN_RUST: bool> TimerEventInRust<IN_RUST> {
    ///
    /// See [C++ `wxTimerEvent::wxTimerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html#aedca4f26719c256c2f8d9ef8486d2f86).
    pub fn new<T: TimerMethods>(timer: &T) -> TimerEventInRust<IN_RUST> {
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
impl<const IN_RUST: bool> From<TimerEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: TimerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<TimerEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: TimerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for TimerEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTimerEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for TimerEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
