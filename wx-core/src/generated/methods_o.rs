use super::*;

// wxOverlay
pub trait OverlayMethods: WxRustMethods {
    // DTOR: fn ~wxOverlay()
    /// Clears the overlay without restoring the former state.
    fn reset(&self) {
        unsafe { ffi::wxOverlay_Reset(self.as_ptr()) }
    }
}

// wxOwnerDrawnComboBox
pub trait OwnerDrawnComboBoxMethods: ComboCtrlMethods {
    // DTOR: fn ~wxOwnerDrawnComboBox()
    // NOT_SUPPORTED: fn Create1()
    fn create_str_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
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
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxOwnerDrawnComboBox_Create2(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    /// Returns true if the list of combobox choices is empty.
    fn is_list_empty(&self) -> bool {
        unsafe { ffi::wxOwnerDrawnComboBox_IsListEmpty(self.as_ptr()) }
    }
    /// Returns true if the text of the combobox is empty.
    fn is_text_empty(&self) -> bool {
        unsafe { ffi::wxOwnerDrawnComboBox_IsTextEmpty(self.as_ptr()) }
    }
    /// Returns index to the widest item in the list.
    fn get_widest_item(&self) -> c_int {
        unsafe { ffi::wxOwnerDrawnComboBox_GetWidestItem(self.as_ptr()) }
    }
    /// Returns width of the widest item in the list.
    fn get_widest_item_width(&self) -> c_int {
        unsafe { ffi::wxOwnerDrawnComboBox_GetWidestItemWidth(self.as_ptr()) }
    }
}
