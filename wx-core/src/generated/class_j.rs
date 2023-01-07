use super::*;

// wxJPEGHandler
wxwidgets! {
    /// This is the image handler for the JPEG format.
    /// - [`JPEGHandler`] represents a C++ `wxJPEGHandler` class instance which your code has ownership, [`JPEGHandlerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`JPEGHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxJPEGHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html) for more details.
    #[doc(alias = "wxJPEGHandler")]
    #[doc(alias = "JPEGHandler")]
    class JPEGHandler
        = JPEGHandlerFromCpp<true>(wxJPEGHandler) impl
        JPEGHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> JPEGHandlerFromCpp<FROM_CPP> {
    /// Default constructor for wxJPEGHandler.
    ///
    /// See [C++ `wxJPEGHandler::wxJPEGHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_j_p_e_g_handler.html#a2935d0b968967a673b0b6cef75dcbc04).
    pub fn new() -> JPEGHandlerFromCpp<FROM_CPP> {
        unsafe { JPEGHandlerFromCpp(ffi::wxJPEGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for JPEGHandlerFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<JPEGHandlerFromCpp<FROM_CPP>> for ImageHandlerFromCpp<FROM_CPP> {
    fn from(o: JPEGHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<JPEGHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: JPEGHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for JPEGHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxJPEGHandler_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for JPEGHandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ImageHandlerMethods for JPEGHandlerFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxJoystickEvent
wxwidgets! {
    /// This event class contains information about joystick events, particularly events received by windows.
    /// - [`JoystickEvent`] represents a C++ `wxJoystickEvent` class instance which your code has ownership, [`JoystickEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`JoystickEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxJoystickEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_joystick_event.html) for more details.
    #[doc(alias = "wxJoystickEvent")]
    #[doc(alias = "JoystickEvent")]
    class JoystickEvent
        = JoystickEventFromCpp<true>(wxJoystickEvent) impl
        JoystickEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> JoystickEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxJoystickEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for JoystickEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<JoystickEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: JoystickEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<JoystickEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: JoystickEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for JoystickEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxJoystickEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for JoystickEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
