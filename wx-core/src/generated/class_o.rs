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
impl<const IN_RUST: bool> OverlayInRust<IN_RUST> {
    ///
    /// See [C++ `wxOverlay::wxOverlay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_overlay.html#aa99ff227f54e8eaa85d2d7966e317a35).
    pub fn new() -> OverlayInRust<IN_RUST> {
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
impl<const IN_RUST: bool> Drop for OverlayInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
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
impl<const IN_RUST: bool> OwnerDrawnComboBoxInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxOwnerDrawnComboBox::wxOwnerDrawnComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#af949423a0a5e212ee125ad82ecb5941e).
    pub fn new_2step() -> OwnerDrawnComboBoxInRust<IN_RUST> {
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
    ) -> OwnerDrawnComboBoxInRust<IN_RUST> {
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
impl<const IN_RUST: bool> Clone for OwnerDrawnComboBoxInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<OwnerDrawnComboBoxInRust<IN_RUST>> for ComboCtrlInRust<IN_RUST> {
    fn from(o: OwnerDrawnComboBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<OwnerDrawnComboBoxInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: OwnerDrawnComboBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<OwnerDrawnComboBoxInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: OwnerDrawnComboBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<OwnerDrawnComboBoxInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: OwnerDrawnComboBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<OwnerDrawnComboBoxInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: OwnerDrawnComboBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for OwnerDrawnComboBoxInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxOwnerDrawnComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxOwnerDrawnComboBox
impl<const IN_RUST: bool> ItemContainerMethods for OwnerDrawnComboBoxInRust<IN_RUST> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const IN_RUST: bool> ItemContainerImmutableMethods for OwnerDrawnComboBoxInRust<IN_RUST> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const IN_RUST: bool> TextEntryMethods for OwnerDrawnComboBoxInRust<IN_RUST> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxOwnerDrawnComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const IN_RUST: bool> ComboCtrlMethods for OwnerDrawnComboBoxInRust<IN_RUST> {
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
