use super::*;

// wxVListBox
pub trait VListBoxMethods: VScrolledWindowMethods {
    // DTOR: fn ~wxVListBox()
    /// Deletes all items from the control.
    fn clear(&self) {
        unsafe { ffi::wxVListBox_Clear(self.as_ptr()) }
    }
    /// Deselects all the items in the listbox.
    fn deselect_all(&self) -> bool {
        unsafe { ffi::wxVListBox_DeselectAll(self.as_ptr()) }
    }
    /// Returns the index of the first selected item in the listbox or wxNOT_FOUND if no items are currently selected.
    fn get_first_selected(&self, cookie: *mut c_void) -> c_int {
        unsafe { ffi::wxVListBox_GetFirstSelected(self.as_ptr(), cookie) }
    }
    /// Get the number of items in the control.
    fn get_item_count(&self) -> usize {
        unsafe { ffi::wxVListBox_GetItemCount(self.as_ptr()) }
    }
    /// Returns the margins used by the control.
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxVListBox_GetMargins(self.as_ptr())) }
    }
    /// Returns the rectangle occupied by this item in physical coordinates.
    fn get_item_rect(&self, item: usize) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxVListBox_GetItemRect(self.as_ptr(), item)) }
    }
    /// Returns the index of the next selected item or wxNOT_FOUND if there are no more.
    fn get_next_selected(&self, cookie: *mut c_void) -> c_int {
        unsafe { ffi::wxVListBox_GetNextSelected(self.as_ptr(), cookie) }
    }
    /// Returns the number of the items currently selected.
    fn get_selected_count(&self) -> usize {
        unsafe { ffi::wxVListBox_GetSelectedCount(self.as_ptr()) }
    }
    /// Get the currently selected item or wxNOT_FOUND if there is no selection.
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxVListBox_GetSelection(self.as_ptr()) }
    }
    /// Returns the background colour used for the selected cells.
    fn get_selection_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxVListBox_GetSelectionBackground(self.as_ptr())) }
    }
    /// Returns true if the listbox was created with wxLB_MULTIPLE style and so supports multiple selection or false if it is a single selection listbox.
    fn has_multiple_selection(&self) -> bool {
        unsafe { ffi::wxVListBox_HasMultipleSelection(self.as_ptr()) }
    }
    /// Returns true if this item is the current one, false otherwise.
    fn is_current(&self, item: usize) -> bool {
        unsafe { ffi::wxVListBox_IsCurrent(self.as_ptr(), item) }
    }
    /// Returns true if this item is selected, false otherwise.
    fn is_selected(&self, item: usize) -> bool {
        unsafe { ffi::wxVListBox_IsSelected(self.as_ptr(), item) }
    }
    /// Selects or deselects the specified item which must be valid (i.e. not equal to wxNOT_FOUND).
    fn select(&self, item: usize, select: bool) -> bool {
        unsafe { ffi::wxVListBox_Select(self.as_ptr(), item, select) }
    }
    /// Selects all the items in the listbox.
    fn select_all(&self) -> bool {
        unsafe { ffi::wxVListBox_SelectAll(self.as_ptr()) }
    }
    /// Selects all items in the specified range which may be given in any order.
    fn select_range(&self, from: usize, to: usize) -> bool {
        unsafe { ffi::wxVListBox_SelectRange(self.as_ptr(), from, to) }
    }
    /// Set the number of items to be shown in the control.
    fn set_item_count(&self, count: usize) {
        unsafe { ffi::wxVListBox_SetItemCount(self.as_ptr(), count) }
    }
    /// Set the margins: horizontal margin is the distance between the window border and the item contents while vertical margin is half of the distance between items.
    fn set_margins_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxVListBox_SetMargins(self.as_ptr(), pt)
        }
    }
    fn set_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxVListBox_SetMargins1(self.as_ptr(), x, y) }
    }
    /// Set the selection to the specified item, if it is -1 the selection is unset.
    fn set_selection(&self, selection: c_int) {
        unsafe { ffi::wxVListBox_SetSelection(self.as_ptr(), selection) }
    }
    /// Sets the colour to be used for the selected cells background.
    fn set_selection_background<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxVListBox_SetSelectionBackground(self.as_ptr(), col)
        }
    }
    /// Toggles the state of the specified item, i.e. selects it if it was unselected and deselects it if it was selected.
    fn toggle(&self, item: usize) {
        unsafe { ffi::wxVListBox_Toggle(self.as_ptr(), item) }
    }
}

// wxVScrolledWindow
pub trait VScrolledWindowMethods: PanelMethods {}

// wxValidator
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    /// All validator classes must implement the Clone() function, which returns an identical copy of itself.
    fn clone(&self) -> Object {
        unsafe { Object::from_ptr(ffi::wxValidator_Clone(self.as_ptr())) }
    }
    /// Returns the window associated with the validator.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxValidator_GetWindow(self.as_ptr())) }
    }
    /// Associates a window with the validator.
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
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr()) }
    }
    /// This overridable function is called when the value associated with the validator must be transferred to the window.
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr()) }
    }
    /// This overridable function is called when the value in the associated window must be validated.
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
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    /// Returns if the error sound is currently disabled.
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}
