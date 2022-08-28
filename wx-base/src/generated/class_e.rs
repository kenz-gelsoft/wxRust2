use super::*;

// wxEvent
wxwidgets! {
    /// An event is a structure holding information about an event passed to a callback or member function.
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
    #[doc(alias = "wxEvtHandler")]
    #[doc(alias = "EvtHandler")]
    class EvtHandler
        = EvtHandlerIsOwned<true>(wxEvtHandler) impl
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EvtHandlerIsOwned<OWNED> {
    /// Constructor.
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
