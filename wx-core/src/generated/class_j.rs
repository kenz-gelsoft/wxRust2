use super::*;

// wxJPEGHandler
wxwidgets! {
    /// This is the image handler for the JPEG format.
    /// - [`JPEGHandler`] represents a C++ `wxJPEGHandler` class instance which your code has ownership, [`JPEGHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`JPEGHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxJPEGHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html) for more details.
    #[doc(alias = "wxJPEGHandler")]
    #[doc(alias = "JPEGHandler")]
    class JPEGHandler
        = JPEGHandlerInRust<true>(wxJPEGHandler) impl
        JPEGHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> JPEGHandlerInRust<IN_RUST> {
    /// Default constructor for wxJPEGHandler.
    ///
    /// See [C++ `wxJPEGHandler::wxJPEGHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html#a2935d0b968967a673b0b6cef75dcbc04).
    pub fn new() -> JPEGHandlerInRust<IN_RUST> {
        unsafe { JPEGHandlerInRust(ffi::wxJPEGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for JPEGHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<JPEGHandlerInRust<IN_RUST>> for ImageHandlerInRust<IN_RUST> {
    fn from(o: JPEGHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<JPEGHandlerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: JPEGHandlerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for JPEGHandlerInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxJPEGHandler_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for JPEGHandlerInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const IN_RUST: bool> ImageHandlerMethods for JPEGHandlerInRust<IN_RUST> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxJoystickEvent
wxwidgets! {
    /// This event class contains information about joystick events, particularly events received by windows.
    /// - [`JoystickEvent`] represents a C++ `wxJoystickEvent` class instance which your code has ownership, [`JoystickEventInRust`]`<false>` represents one which don't own.
    /// - Use [`JoystickEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxJoystickEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html) for more details.
    #[doc(alias = "wxJoystickEvent")]
    #[doc(alias = "JoystickEvent")]
    class JoystickEvent
        = JoystickEventInRust<true>(wxJoystickEvent) impl
        JoystickEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> JoystickEventInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxJoystickEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for JoystickEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<JoystickEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: JoystickEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<JoystickEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: JoystickEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for JoystickEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxJoystickEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for JoystickEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
