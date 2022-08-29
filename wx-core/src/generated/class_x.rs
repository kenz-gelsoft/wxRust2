use super::*;

// wxXPMHandler
wxwidgets! {
    /// This is the image handler for the XPM format.
    /// - [`XPMHandler`] represents a C++ `wxXPMHandler` class instance which your code has ownership, [`XPMHandlerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`XPMHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxXPMHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_x_p_m_handler.html) for more details.
    #[doc(alias = "wxXPMHandler")]
    #[doc(alias = "XPMHandler")]
    class XPMHandler
        = XPMHandlerIsOwned<true>(wxXPMHandler) impl
        XPMHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> XPMHandlerIsOwned<OWNED> {
    /// Default constructor for wxXPMHandler.
    ///
    /// See [C++ `wxXPMHandler::wxXPMHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_x_p_m_handler.html#a4b2e1b268d849c5abcbb83c120638d15).
    pub fn new() -> XPMHandlerIsOwned<OWNED> {
        unsafe { XPMHandlerIsOwned(ffi::wxXPMHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for XPMHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<XPMHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: XPMHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<XPMHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: XPMHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for XPMHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxXPMHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for XPMHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
