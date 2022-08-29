use super::*;

// wxOverlay
/// This trait represents C++ [`wxOverlay`](https://docs.wxwidgets.org/3.2/classwx_overlay.html) class's methods and inheritance.
///
/// See [`OverlayIsOwned`] documentation for the class usage.
pub trait OverlayMethods: WxRustMethods {
    // DTOR: fn ~wxOverlay()
    /// Clears the overlay without restoring the former state.
    ///
    /// [See `wxOverlay::Reset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_overlay.html#a1e3141974ae247441862ab3604dfff2f)
    fn reset(&self) {
        unsafe { ffi::wxOverlay_Reset(self.as_ptr()) }
    }
}

// wxOwnerDrawnComboBox
/// This trait represents C++ [`wxOwnerDrawnComboBox`](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html) class's methods and inheritance.
///
/// See [`OwnerDrawnComboBoxIsOwned`] documentation for the class usage.
pub trait OwnerDrawnComboBoxMethods: ComboCtrlMethods {
    // DTOR: fn ~wxOwnerDrawnComboBox()
    // NOT_SUPPORTED: fn Create1()
    ///
    /// [See `wxOwnerDrawnComboBox::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#a8117083f01a57671d333d6df72d23abf)
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
    ///
    /// [See `wxOwnerDrawnComboBox::IsListEmpty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#a71dd15c398465339e360cf12d73361e1)
    fn is_list_empty(&self) -> bool {
        unsafe { ffi::wxOwnerDrawnComboBox_IsListEmpty(self.as_ptr()) }
    }
    /// Returns true if the text of the combobox is empty.
    ///
    /// [See `wxOwnerDrawnComboBox::IsTextEmpty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#a09355ba2a58d49676661ea85c0b0c3fe)
    fn is_text_empty(&self) -> bool {
        unsafe { ffi::wxOwnerDrawnComboBox_IsTextEmpty(self.as_ptr()) }
    }
    /// Returns index to the widest item in the list.
    ///
    /// [See `wxOwnerDrawnComboBox::GetWidestItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#aada8ca321c213d2e4a28c01aa1c56c35)
    fn get_widest_item(&self) -> c_int {
        unsafe { ffi::wxOwnerDrawnComboBox_GetWidestItem(self.as_ptr()) }
    }
    /// Returns width of the widest item in the list.
    ///
    /// [See `wxOwnerDrawnComboBox::GetWidestItemWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_owner_drawn_combo_box.html#a6bff15424b9ea43df6c4b9efe7bc1a78)
    fn get_widest_item_width(&self) -> c_int {
        unsafe { ffi::wxOwnerDrawnComboBox_GetWidestItemWidth(self.as_ptr()) }
    }
}
