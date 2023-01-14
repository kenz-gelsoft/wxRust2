use super::*;

// wxVListBox
wxwidgets! {
    /// wxVListBox is a wxListBox-like control with the following two main differences from a regular wxListBox: it can have an arbitrarily huge number of items because it doesn't store them itself but uses the OnDrawItem() callback to draw them (so it is a virtual listbox) and its items can have variable height as determined by OnMeasureItem() (so it is also a listbox with the lines of variable height).
    /// - [`VListBox`] represents a C++ `wxVListBox` class instance which your code has ownership, [`VListBoxFromCpp`]`<true>` represents one which don't own.
    /// - Use [`VListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxVListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html) for more details.
    #[doc(alias = "wxVListBox")]
    #[doc(alias = "VListBox")]
    class VListBox
        = VListBoxFromCpp<false>(wxVListBox) impl
        VListBoxMethods,
        VScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> VListBoxFromCpp<FROM_CPP> {
    // BLOCKED: fn wxVListBox()
    // BLOCKED: fn wxVListBox1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for VListBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<VListBoxFromCpp<FROM_CPP>> for VScrolledWindowFromCpp<FROM_CPP> {
    fn from(o: VListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VListBoxFromCpp<FROM_CPP>> for PanelFromCpp<FROM_CPP> {
    fn from(o: VListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VListBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: VListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VListBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: VListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VListBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: VListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for VListBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxVListBox_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for VListBoxFromCpp<FROM_CPP> {
    /// Creates the control.
    ///
    /// See [C++ `wxVListBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#ab04914d5db45af7c3c032e19fa2b2615).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxVListBox_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxVScrolledWindow
wxwidgets! {
    /// In the name of this class, "V" may stand for "variable" because it can be used for scrolling rows of variable heights; "virtual", because it is not necessary to know the heights of all rows in advance  only those which are shown on the screen need to be measured; or even "vertical", because this class only supports scrolling vertically.
    /// - [`VScrolledWindow`] represents a C++ `wxVScrolledWindow` class instance which your code has ownership, [`VScrolledWindowFromCpp`]`<true>` represents one which don't own.
    /// - Use [`VScrolledWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxVScrolledWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_v_scrolled_window.html) for more details.
    #[doc(alias = "wxVScrolledWindow")]
    #[doc(alias = "VScrolledWindow")]
    class VScrolledWindow
        = VScrolledWindowFromCpp<false>(wxVScrolledWindow) impl
        VScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> VScrolledWindowFromCpp<FROM_CPP> {
    // BLOCKED: fn wxVScrolledWindow()
    // BLOCKED: fn wxVScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for VScrolledWindowFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<VScrolledWindowFromCpp<FROM_CPP>> for PanelFromCpp<FROM_CPP> {
    fn from(o: VScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VScrolledWindowFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: VScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VScrolledWindowFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: VScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<VScrolledWindowFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: VScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for VScrolledWindowFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxVScrolledWindow_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for VScrolledWindowFromCpp<FROM_CPP> {
    /// Same as the non-default constructor, but returns a status code: true if ok, false if the window couldn't be created.
    ///
    /// See [C++ `wxVScrolledWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_v_scrolled_window.html#ac6e7a6ace37133efb091b1bf69d09a90).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxValidator
wxwidgets! {
    /// wxValidator is the base class for a family of validator classes that mediate between a class of control, and application data.
    /// - [`Validator`] represents a C++ `wxValidator` class instance which your code has ownership, [`ValidatorFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Validator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_validator.html) for more details.
    #[doc(alias = "wxValidator")]
    #[doc(alias = "Validator")]
    class Validator
        = ValidatorFromCpp<false>(wxValidator) impl
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ValidatorFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxValidator::wxValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_validator.html#aac102bc64513a0f8bd38e9db81a3d833).
    pub fn new() -> ValidatorFromCpp<FROM_CPP> {
        unsafe { ValidatorFromCpp(ffi::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ValidatorFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ValidatorFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ValidatorFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ValidatorFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxValidator_CLASSINFO()) }
    }
}
