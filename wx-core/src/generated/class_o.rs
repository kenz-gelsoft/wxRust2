use super::*;

// wxOverlay
wxwidgets! {
    /// Creates an overlay over an existing window, allowing for manipulations like rubberbanding, etc.
    /// - [`Overlay`] represents a C++ `wxOverlay` class instance which your code has ownership, [`OverlayInRust`]`<false>` represents one which don't own.
    /// - Use [`Overlay`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxOverlay` class's documentation](https://docs.wxwidgets.org/3.2/classwx_overlay.html) for more details.
    #[doc(alias = "wxOverlay")]
    #[doc(alias = "Overlay")]
    class Overlay
        = OverlayInRust<true>(wxOverlay) impl
        OverlayMethods
}
impl<const OWNED: bool> OverlayInRust<OWNED> {
    ///
    /// See [C++ `wxOverlay::wxOverlay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_overlay.html#aa99ff227f54e8eaa85d2d7966e317a35).
    pub fn new() -> OverlayInRust<OWNED> {
        unsafe { OverlayInRust(ffi::wxOverlay_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for OverlayInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for OverlayInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxOverlay_delete(self.0) }
        }
    }
}

// wxOwnerDrawnComboBox
wxwidgets! {
    /// wxOwnerDrawnComboBox is a combobox with owner-drawn list items.
    /// - [`OwnerDrawnComboBox`] represents a C++ `wxOwnerDrawnComboBox` class instance which your code has ownership, [`OwnerDrawnComboBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`OwnerDrawnComboBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxOwnerDrawnComboBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html) for more details.
    #[doc(alias = "wxOwnerDrawnComboBox")]
    #[doc(alias = "OwnerDrawnComboBox")]
    class OwnerDrawnComboBox
        = OwnerDrawnComboBoxInRust<true>(wxOwnerDrawnComboBox) impl
        OwnerDrawnComboBoxMethods,
        // ComboCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> OwnerDrawnComboBoxInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxOwnerDrawnComboBox::wxOwnerDrawnComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#af949423a0a5e212ee125ad82ecb5941e).
    pub fn new_2step() -> OwnerDrawnComboBoxInRust<OWNED> {
        unsafe { OwnerDrawnComboBoxInRust(ffi::wxOwnerDrawnComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxOwnerDrawnComboBox1()
    /// Constructor, creating and showing a owner-drawn combobox.
    ///
    /// See [C++ `wxOwnerDrawnComboBox::wxOwnerDrawnComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#a2eda25541f678c9dfa9bc4c97bcafc29).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> OwnerDrawnComboBoxInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            OwnerDrawnComboBoxInRust(ffi::wxOwnerDrawnComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for OwnerDrawnComboBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxInRust<OWNED>> for ComboCtrlInRust<OWNED> {
    fn from(o: OwnerDrawnComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: OwnerDrawnComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: OwnerDrawnComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: OwnerDrawnComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<OwnerDrawnComboBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: OwnerDrawnComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for OwnerDrawnComboBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxOwnerDrawnComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxOwnerDrawnComboBox
impl<const OWNED: bool> ItemContainerMethods for OwnerDrawnComboBoxInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for OwnerDrawnComboBoxInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for OwnerDrawnComboBoxInRust<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ComboCtrlMethods for OwnerDrawnComboBoxInRust<OWNED> {
    /// Creates the combobox for two-step construction.
    ///
    /// See [C++ `wxOwnerDrawnComboBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#a92bdeed6e785aecb9ee37181b2eb3a3e).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxOwnerDrawnComboBox_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
}
