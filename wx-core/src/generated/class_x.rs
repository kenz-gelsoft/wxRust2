use super::*;

// wxXPMHandler
wxwidgets! {
    /// This is the image handler for the XPM format.
    /// - [`XPMHandler`] represents a C++ `wxXPMHandler` class instance which your code has ownership, [`XPMHandlerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`XPMHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxXPMHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_x_p_m_handler.html) for more details.
    #[doc(alias = "wxXPMHandler")]
    #[doc(alias = "XPMHandler")]
    class XPMHandler
        = XPMHandlerFromCpp<true>(wxXPMHandler) impl
        XPMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> XPMHandlerFromCpp<FROM_CPP> {
    /// Default constructor for wxXPMHandler.
    ///
    /// See [C++ `wxXPMHandler::wxXPMHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_x_p_m_handler.html#a4b2e1b268d849c5abcbb83c120638d15).
    pub fn new() -> XPMHandlerFromCpp<FROM_CPP> {
        unsafe { XPMHandlerFromCpp(ffi::wxXPMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for XPMHandlerFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<XPMHandlerFromCpp<FROM_CPP>> for ImageHandlerFromCpp<FROM_CPP> {
    fn from(o: XPMHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<XPMHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: XPMHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for XPMHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxXPMHandler_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for XPMHandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
