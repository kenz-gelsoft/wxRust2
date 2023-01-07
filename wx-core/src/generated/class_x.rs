use super::*;

// wxXPMHandler
wxwidgets! {
    /// This is the image handler for the XPM format.
    /// - [`XPMHandler`] represents a C++ `wxXPMHandler` class instance which your code has ownership, [`XPMHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`XPMHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxXPMHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_x_p_m_handler.html) for more details.
    #[doc(alias = "wxXPMHandler")]
    #[doc(alias = "XPMHandler")]
    class XPMHandler
        = XPMHandlerInRust<true>(wxXPMHandler) impl
        XPMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> XPMHandlerInRust<OWNED> {
    /// Default constructor for wxXPMHandler.
    ///
    /// See [C++ `wxXPMHandler::wxXPMHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_x_p_m_handler.html#a4b2e1b268d849c5abcbb83c120638d15).
    pub fn new() -> XPMHandlerInRust<OWNED> {
        unsafe { XPMHandlerInRust(ffi::wxXPMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for XPMHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<XPMHandlerInRust<OWNED>> for ImageHandlerInRust<OWNED> {
    fn from(o: XPMHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<XPMHandlerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: XPMHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for XPMHandlerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxXPMHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for XPMHandlerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
