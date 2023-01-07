use super::*;

// wxTimer
wxwidgets! {
    /// The wxTimer class allows you to execute code at specified intervals.
    /// - [`Timer`] represents a C++ `wxTimer` class instance which your code has ownership, [`TimerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Timer`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxTimer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html) for more details.
    #[doc(alias = "wxTimer")]
    #[doc(alias = "Timer")]
    class Timer
        = TimerFromCpp<true>(wxTimer) impl
        TimerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TimerFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#a0560894a1ab57792f52992ffbc58e735).
    pub fn new() -> TimerFromCpp<FROM_CPP> {
        unsafe { TimerFromCpp(ffi::wxTimer_new()) }
    }
    /// Creates a timer and associates it with owner.
    ///
    /// See [C++ `wxTimer::wxTimer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer.html#aeba595e67731d9f8ce07e4ac69a0ca65).
    pub fn new_with_evthandler<E: EvtHandlerMethods>(
        owner: Option<&E>,
        id: c_int,
    ) -> TimerFromCpp<FROM_CPP> {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TimerFromCpp(ffi::wxTimer_new1(owner, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TimerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TimerFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TimerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TimerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TimerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TimerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTimer_CLASSINFO()) }
    }
}

// wxTimerEvent
wxwidgets! {
    /// wxTimerEvent object is passed to the event handler of timer events (see wxTimer::SetOwner).
    /// - [`TimerEvent`] represents a C++ `wxTimerEvent` class instance which your code has ownership, [`TimerEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TimerEvent`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxTimerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html) for more details.
    #[doc(alias = "wxTimerEvent")]
    #[doc(alias = "TimerEvent")]
    class TimerEvent
        = TimerEventFromCpp<true>(wxTimerEvent) impl
        TimerEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TimerEventFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxTimerEvent::wxTimerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_timer_event.html#aedca4f26719c256c2f8d9ef8486d2f86).
    pub fn new<T: TimerMethods>(timer: &T) -> TimerEventFromCpp<FROM_CPP> {
        unsafe {
            let timer = timer.as_ptr();
            TimerEventFromCpp(ffi::wxTimerEvent_new(timer))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TimerEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TimerEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: TimerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TimerEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TimerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TimerEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTimerEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for TimerEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
