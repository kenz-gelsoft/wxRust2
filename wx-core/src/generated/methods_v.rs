use super::*;

// wxVListBox
/// This trait represents C++ [`wxVListBox`](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html) class's methods and inheritance.
///
/// See [`VListBoxIsOwned`] documentation for the class usage.
pub trait VListBoxMethods: VScrolledWindowMethods {
    // DTOR: fn ~wxVListBox()
    /// Deletes all items from the control.
    ///
    /// [See `wxVListBox::Clear()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#aaf92fc38649bd6efc5b1f72c508a5f6f)
    fn clear(&self) {
        unsafe { ffi::wxVListBox_Clear(self.as_ptr()) }
    }
    /// Deselects all the items in the listbox.
    ///
    /// [See `wxVListBox::DeselectAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a3d922a858e2af9eb94026a5360e2c443)
    fn deselect_all(&self) -> bool {
        unsafe { ffi::wxVListBox_DeselectAll(self.as_ptr()) }
    }
    /// Returns the index of the first selected item in the listbox or wxNOT_FOUND if no items are currently selected.
    ///
    /// [See `wxVListBox::GetFirstSelected()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a6fe25d924067f6417d0df219d0d2bf30)
    fn get_first_selected(&self, cookie: *mut c_void) -> c_int {
        unsafe { ffi::wxVListBox_GetFirstSelected(self.as_ptr(), cookie) }
    }
    /// Get the number of items in the control.
    ///
    /// [See `wxVListBox::GetItemCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a678e0068cc41361701097f9ebdc63fa5)
    fn get_item_count(&self) -> usize {
        unsafe { ffi::wxVListBox_GetItemCount(self.as_ptr()) }
    }
    /// Returns the margins used by the control.
    ///
    /// [See `wxVListBox::GetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a021b8a9a3148623508f555f4f3cd0d24)
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxVListBox_GetMargins(self.as_ptr())) }
    }
    /// Returns the rectangle occupied by this item in physical coordinates.
    ///
    /// [See `wxVListBox::GetItemRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#ac42c91e4dc7e877f3062108bcee27693)
    fn get_item_rect(&self, item: usize) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxVListBox_GetItemRect(self.as_ptr(), item)) }
    }
    /// Returns the index of the next selected item or wxNOT_FOUND if there are no more.
    ///
    /// [See `wxVListBox::GetNextSelected()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#af683034a20f3760b4a253510cc320789)
    fn get_next_selected(&self, cookie: *mut c_void) -> c_int {
        unsafe { ffi::wxVListBox_GetNextSelected(self.as_ptr(), cookie) }
    }
    /// Returns the number of the items currently selected.
    ///
    /// [See `wxVListBox::GetSelectedCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a0e7a8edc564fa5a309cb09f07258cfe5)
    fn get_selected_count(&self) -> usize {
        unsafe { ffi::wxVListBox_GetSelectedCount(self.as_ptr()) }
    }
    /// Get the currently selected item or wxNOT_FOUND if there is no selection.
    ///
    /// [See `wxVListBox::GetSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#acc5665a51a4cefd6291cbf3c02e338ba)
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxVListBox_GetSelection(self.as_ptr()) }
    }
    /// Returns the background colour used for the selected cells.
    ///
    /// [See `wxVListBox::GetSelectionBackground()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a6fc9b1d5737d51c001148f1563a7a737)
    fn get_selection_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxVListBox_GetSelectionBackground(self.as_ptr())) }
    }
    /// Returns true if the listbox was created with wxLB_MULTIPLE style and so supports multiple selection or false if it is a single selection listbox.
    ///
    /// [See `wxVListBox::HasMultipleSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a8d0144a90340ca18f5be907fd4dfc96a)
    fn has_multiple_selection(&self) -> bool {
        unsafe { ffi::wxVListBox_HasMultipleSelection(self.as_ptr()) }
    }
    /// Returns true if this item is the current one, false otherwise.
    ///
    /// [See `wxVListBox::IsCurrent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a84098ec4b9227a619a1dcc4a2427f582)
    fn is_current(&self, item: usize) -> bool {
        unsafe { ffi::wxVListBox_IsCurrent(self.as_ptr(), item) }
    }
    /// Returns true if this item is selected, false otherwise.
    ///
    /// [See `wxVListBox::IsSelected()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a331d39ab137d9319163f29f5659b0dca)
    fn is_selected(&self, item: usize) -> bool {
        unsafe { ffi::wxVListBox_IsSelected(self.as_ptr(), item) }
    }
    /// Selects or deselects the specified item which must be valid (i.e. not equal to wxNOT_FOUND).
    ///
    /// [See `wxVListBox::Select()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a0b49818db23a5fbd987438ca5ea0f607)
    fn select(&self, item: usize, select: bool) -> bool {
        unsafe { ffi::wxVListBox_Select(self.as_ptr(), item, select) }
    }
    /// Selects all the items in the listbox.
    ///
    /// [See `wxVListBox::SelectAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a46d5153f9c50bff13663d1768818255e)
    fn select_all(&self) -> bool {
        unsafe { ffi::wxVListBox_SelectAll(self.as_ptr()) }
    }
    /// Selects all items in the specified range which may be given in any order.
    ///
    /// [See `wxVListBox::SelectRange()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#ab148ec397eacecb1a71e829c2b5aa13a)
    fn select_range(&self, from: usize, to: usize) -> bool {
        unsafe { ffi::wxVListBox_SelectRange(self.as_ptr(), from, to) }
    }
    /// Set the number of items to be shown in the control.
    ///
    /// [See `wxVListBox::SetItemCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a205615c79836dc7999d20e083a4a8ffb)
    fn set_item_count(&self, count: usize) {
        unsafe { ffi::wxVListBox_SetItemCount(self.as_ptr(), count) }
    }
    /// Set the margins: horizontal margin is the distance between the window border and the item contents while vertical margin is half of the distance between items.
    ///
    /// [See `wxVListBox::SetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#aeabe37f2b1a1887f995c431eb5522921)
    fn set_margins_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxVListBox_SetMargins(self.as_ptr(), pt)
        }
    }
    ///
    /// [See `wxVListBox::SetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#a20ef3b7013828036d0c3901500d23cf3)
    fn set_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxVListBox_SetMargins1(self.as_ptr(), x, y) }
    }
    /// Set the selection to the specified item, if it is -1 the selection is unset.
    ///
    /// [See `wxVListBox::SetSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#aee1df263dbb96e49dbc44684fb7d9dad)
    fn set_selection(&self, selection: c_int) {
        unsafe { ffi::wxVListBox_SetSelection(self.as_ptr(), selection) }
    }
    /// Sets the colour to be used for the selected cells background.
    ///
    /// [See `wxVListBox::SetSelectionBackground()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#aad25b40caedb9a864ab347d50e8c5e8b)
    fn set_selection_background<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxVListBox_SetSelectionBackground(self.as_ptr(), col)
        }
    }
    /// Toggles the state of the specified item, i.e. selects it if it was unselected and deselects it if it was selected.
    ///
    /// [See `wxVListBox::Toggle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_v_list_box.html#ae530f18573bd96ea497d64b1b1b18905)
    fn toggle(&self, item: usize) {
        unsafe { ffi::wxVListBox_Toggle(self.as_ptr(), item) }
    }
}

// wxVScrolledWindow
/// This trait represents C++ [`wxVScrolledWindow`](https://docs.wxwidgets.org/3.2/classwx_v_scrolled_window.html) class's methods and inheritance.
///
/// See [`VScrolledWindowIsOwned`] documentation for the class usage.
pub trait VScrolledWindowMethods: PanelMethods {}

// wxValidator
/// This trait represents C++ [`wxValidator`](https://docs.wxwidgets.org/3.2/classwx_validator.html) class's methods and inheritance.
///
/// See [`ValidatorIsOwned`] documentation for the class usage.
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    /// All validator classes must implement the Clone() function, which returns an identical copy of itself.
    ///
    /// [See `wxValidator::Clone()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#a06c78c305b279cd8cad1a31376e6c713)
    fn clone(&self) -> Object {
        unsafe { Object::from_ptr(ffi::wxValidator_Clone(self.as_ptr())) }
    }
    /// Returns the window associated with the validator.
    ///
    /// [See `wxValidator::GetWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#a7568622187a3ffa022ba53ef7e976b9c)
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxValidator_GetWindow(self.as_ptr())) }
    }
    /// Associates a window with the validator.
    ///
    /// [See `wxValidator::SetWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#af909586ba6a6c8f665f684a4a83334eb)
    fn set_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.as_ptr(), window)
        }
    }
    /// This overridable function is called when the value in the window must be transferred to the validator.
    ///
    /// [See `wxValidator::TransferFromWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#acffa9472b2f741ab29dbfad3b80934da)
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr()) }
    }
    /// This overridable function is called when the value associated with the validator must be transferred to the window.
    ///
    /// [See `wxValidator::TransferToWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#aa09f9ae3bace5de7a8e577206b75aeae)
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr()) }
    }
    /// This overridable function is called when the value in the associated window must be validated.
    ///
    /// [See `wxValidator::Validate()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#abe48368bac7f540f0c20b1436e5c71af)
    fn validate<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.as_ptr(), parent)
        }
    }
    /// This functions switches on or turns off the error sound produced by the validators if an invalid key is pressed.
    ///
    /// [See `wxValidator::SuppressBellOnError()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#a810c6c099c730a13b88f04d46ebe720e)
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    /// Returns if the error sound is currently disabled.
    ///
    /// [See `wxValidator::IsSilent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_validator.html#a50671f3be232b0fd30a0877d3273a366)
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}
