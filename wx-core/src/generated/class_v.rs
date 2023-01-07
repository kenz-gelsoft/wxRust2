use super::*;

// wxVListBox
wxwidgets! {
    /// wxVListBox is a wxListBox-like control with the following two main differences from a regular wxListBox: it can have an arbitrarily huge number of items because it doesn't store them itself but uses the OnDrawItem() callback to draw them (so it is a virtual listbox) and its items can have variable height as determined by OnMeasureItem() (so it is also a listbox with the lines of variable height).
    /// - [`VListBox`] represents a C++ `wxVListBox` class instance which your code has ownership, [`VListBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`VListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxVListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html) for more details.
    #[doc(alias = "wxVListBox")]
    #[doc(alias = "VListBox")]
    class VListBox
        = VListBoxInRust<true>(wxVListBox) impl
        VListBoxMethods,
        VScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> VListBoxInRust<OWNED> {
    // BLOCKED: fn wxVListBox()
    // BLOCKED: fn wxVListBox1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for VListBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<VListBoxInRust<OWNED>> for VScrolledWindowInRust<OWNED> {
    fn from(o: VListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxInRust<OWNED>> for PanelInRust<OWNED> {
    fn from(o: VListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: VListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: VListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VListBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: VListBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for VListBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxVListBox_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for VListBoxInRust<OWNED> {
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
    /// - [`VScrolledWindow`] represents a C++ `wxVScrolledWindow` class instance which your code has ownership, [`VScrolledWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`VScrolledWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxVScrolledWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_v_scrolled_window.html) for more details.
    #[doc(alias = "wxVScrolledWindow")]
    #[doc(alias = "VScrolledWindow")]
    class VScrolledWindow
        = VScrolledWindowInRust<true>(wxVScrolledWindow) impl
        VScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> VScrolledWindowInRust<OWNED> {
    // BLOCKED: fn wxVScrolledWindow()
    // BLOCKED: fn wxVScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for VScrolledWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<VScrolledWindowInRust<OWNED>> for PanelInRust<OWNED> {
    fn from(o: VScrolledWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VScrolledWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: VScrolledWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VScrolledWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: VScrolledWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<VScrolledWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: VScrolledWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for VScrolledWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxVScrolledWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for VScrolledWindowInRust<OWNED> {
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
    /// - [`Validator`] represents a C++ `wxValidator` class instance which your code has ownership, [`ValidatorInRust`]`<false>` represents one which don't own.
    /// - Use [`Validator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_validator.html) for more details.
    #[doc(alias = "wxValidator")]
    #[doc(alias = "Validator")]
    class Validator
        = ValidatorInRust<true>(wxValidator) impl
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ValidatorInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxValidator::wxValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_validator.html#aac102bc64513a0f8bd38e9db81a3d833).
    pub fn new() -> ValidatorInRust<OWNED> {
        unsafe { ValidatorInRust(ffi::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ValidatorInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ValidatorInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ValidatorInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ValidatorInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxValidator_CLASSINFO()) }
    }
}
