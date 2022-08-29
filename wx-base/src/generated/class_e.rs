use super::*;

// wxEvent
wxwidgets! {
    /// An event is a structure holding information about an event passed to a callback or member function.
    /// - [`Event`] represents a C++ `wxEvent` class instance which your code has ownership, [`EventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Event`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_event.html) for more details.
    #[doc(alias = "wxEvent")]
    #[doc(alias = "Event")]
    class Event
        = EventIsOwned<true>(wxEvent) impl
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> EventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for EventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<EventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for EventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEvtHandler
wxwidgets! {
    /// A class that can handle events from the windowing system.
    /// - [`EvtHandler`] represents a C++ `wxEvtHandler` class instance which your code has ownership, [`EvtHandlerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`EvtHandler`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxEvtHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html) for more details.
    #[doc(alias = "wxEvtHandler")]
    #[doc(alias = "EvtHandler")]
    class EvtHandler
        = EvtHandlerIsOwned<true>(wxEvtHandler) impl
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EvtHandlerIsOwned<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxEvtHandler::wxEvtHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_evt_handler.html#a3f0166c4154227d05575b01eb2c8d4be).
    pub fn new() -> EvtHandlerIsOwned<OWNED> {
        unsafe { EvtHandlerIsOwned(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for EvtHandlerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<EvtHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EvtHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EvtHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEvtHandler_CLASSINFO()) }
    }
}
