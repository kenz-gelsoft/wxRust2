use super::*;

// wxEvent
wxwidgets! {
    /// An event is a structure holding information about an event passed to a callback or member function.
    /// - [`Event`] represents a C++ `wxEvent` class instance which your code has ownership, [`EventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Event`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_event.html) for more details.
    #[doc(alias = "wxEvent")]
    #[doc(alias = "Event")]
    class Event
        = EventFromCpp<false>(wxEvent) impl
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> EventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for EventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<EventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: EventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for EventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for EventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEvtHandler
wxwidgets! {
    /// A class that can handle events from the windowing system.
    /// - [`EvtHandler`] represents a C++ `wxEvtHandler` class instance which your code has ownership, [`EvtHandlerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`EvtHandler`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxEvtHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html) for more details.
    #[doc(alias = "wxEvtHandler")]
    #[doc(alias = "EvtHandler")]
    class EvtHandler
        = EvtHandlerFromCpp<false>(wxEvtHandler) impl
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> EvtHandlerFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxEvtHandler::wxEvtHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a3f0166c4154227d05575b01eb2c8d4be).
    pub fn new() -> EvtHandlerFromCpp<FROM_CPP> {
        unsafe { EvtHandlerFromCpp(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for EvtHandlerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<EvtHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: EvtHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for EvtHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxEvtHandler_CLASSINFO()) }
    }
}
