use super::*;

// wxLayoutAlgorithm
/// This trait represents [C++ `wxLayoutAlgorithm` class](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html)'s methods and inheritance.
///
/// See [`LayoutAlgorithmFromCpp`] documentation for the class usage.
pub trait LayoutAlgorithmMethods: ObjectMethods {
    // DTOR: fn ~wxLayoutAlgorithm()
    /// Lays out the children of a normal frame.
    ///
    /// See [C++ `wxLayoutAlgorithm::LayoutFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html#aec4fbde5e0443135361f134509d6f256).
    fn layout_frame<F: FrameMethods, W: WindowMethods>(
        &self,
        frame: Option<&F>,
        main_window: Option<&W>,
    ) -> bool {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_window = match main_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLayoutAlgorithm_LayoutFrame(self.as_ptr(), frame, main_window)
        }
    }
    // BLOCKED: fn LayoutMDIFrame()
    /// Lays out the children of a normal frame or other window.
    ///
    /// See [C++ `wxLayoutAlgorithm::LayoutWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_layout_algorithm.html#a92a081ca21f6faeef4b19c0d248e972e).
    fn layout_window<W: WindowMethods, W2: WindowMethods>(
        &self,
        parent: Option<&W>,
        main_window: Option<&W2>,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_window = match main_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLayoutAlgorithm_LayoutWindow(self.as_ptr(), parent, main_window)
        }
    }
}

// wxListBox
/// This trait represents [C++ `wxListBox` class](https://docs.wxwidgets.org/3.2/classwx_list_box.html)'s methods and inheritance.
///
/// See [`ListBoxFromCpp`] documentation for the class usage.
pub trait ListBoxMethods: ControlMethods {
    // DTOR: fn ~wxListBox()
    // NOT_SUPPORTED: fn Create()
    ///
    /// See [C++ `wxListBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a6ad98ad3ff84204342b2dc0ce6301efb).
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxListBox_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    /// Deselects an item in the list box.
    ///
    /// See [C++ `wxListBox::Deselect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a892bf1c85f1ede9c22143cbb9fc22217).
    fn deselect(&self, n: c_int) {
        unsafe { ffi::wxListBox_Deselect(self.as_ptr(), n) }
    }
    ///
    /// See [C++ `wxListBox::SetStringSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#aea7400ba03316f4c437aaaada228e598).
    fn set_string_selection_bool(&self, s: &str, select: bool) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxListBox_SetStringSelection(self.as_ptr(), s, select)
        }
    }
    ///
    /// See [C++ `wxListBox::SetStringSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a708da5aa578b62b86e497f56e272a9a5).
    fn set_string_selection(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxListBox_SetStringSelection1(self.as_ptr(), s)
        }
    }
    /// Fill an array of ints with the positions of the currently selected items.
    ///
    /// See [C++ `wxListBox::GetSelections()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a3035584e9c22b1192c67e8f02df45064).
    fn get_selections<A: ArrayIntMethods>(&self, selections: &A) -> c_int {
        unsafe {
            let selections = selections.as_ptr();
            ffi::wxListBox_GetSelections(self.as_ptr(), selections)
        }
    }
    /// Returns the item located at point, or wxNOT_FOUND if there is no item located at point.
    ///
    /// See [C++ `wxListBox::HitTest()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#aa595879f186d29e6d97f8bfb6074fff0).
    fn hit_test_point<P: PointMethods>(&self, point: &P) -> c_int {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListBox_HitTest(self.as_ptr(), point)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxListBox::HitTest()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a637cf9c39cee2f7d3875535b5aafa051).
    fn hit_test_int(&self, x: c_int, y: c_int) -> c_int {
        unsafe { ffi::wxListBox_HitTest1(self.as_ptr(), x, y) }
    }
    // BLOCKED: fn InsertItems()
    /// Insert the given number of strings before the specified position.
    ///
    /// See [C++ `wxListBox::InsertItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#aac3af32320ec14ee349c00547abb36ab).
    fn insert_items<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxListBox_InsertItems1(self.as_ptr(), items, pos)
        }
    }
    /// Determines whether an item is selected.
    ///
    /// See [C++ `wxListBox::IsSelected()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a9e2ec76968d24f67d69e62378896058a).
    fn is_selected(&self, n: c_int) -> bool {
        unsafe { ffi::wxListBox_IsSelected(self.as_ptr(), n) }
    }
    /// Set the specified item to be the first visible item.
    ///
    /// See [C++ `wxListBox::SetFirstItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#ad51cf8a4f71c5525ad85a7217af56e74).
    fn set_first_item_int(&self, n: c_int) {
        unsafe { ffi::wxListBox_SetFirstItem(self.as_ptr(), n) }
    }
    /// Set the specified item to be the first visible item.
    ///
    /// See [C++ `wxListBox::SetFirstItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a53ca0780ad9bccb92bd1cc2a87c29f41).
    fn set_first_item_str(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxListBox_SetFirstItem1(self.as_ptr(), string)
        }
    }
    /// Ensure that the item with the given index is currently shown.
    ///
    /// See [C++ `wxListBox::EnsureVisible()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a152fc29cfc7877e8bdb1f0ec21b67014).
    fn ensure_visible(&self, n: c_int) {
        unsafe { ffi::wxListBox_EnsureVisible(self.as_ptr(), n) }
    }
    /// Return true if the listbox has wxLB_SORT style.
    ///
    /// See [C++ `wxListBox::IsSorted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a216e4e0a02bc9bf63a4b07b3a1b8427c).
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxListBox_IsSorted(self.as_ptr()) }
    }
    /// Return the number of items that can fit vertically in the visible area of the listbox.
    ///
    /// See [C++ `wxListBox::GetCountPerPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#ad9fe8006950c0aae6e2dde1c488a9ecd).
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListBox_GetCountPerPage(self.as_ptr()) }
    }
    /// Return the index of the topmost visible item.
    ///
    /// See [C++ `wxListBox::GetTopItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_box.html#a4fb690ea63ecb640111a562339ca689e).
    fn get_top_item(&self) -> c_int {
        unsafe { ffi::wxListBox_GetTopItem(self.as_ptr()) }
    }
    // BLOCKED: fn MSWSetTabStops()
}

// wxListCtrl
/// This trait represents [C++ `wxListCtrl` class](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html)'s methods and inheritance.
///
/// See [`ListCtrlFromCpp`] documentation for the class usage.
pub trait ListCtrlMethods: ControlMethods {
    // DTOR: fn ~wxListCtrl()
    // NOT_SUPPORTED: fn AppendColumn()
    /// Arranges the items in icon or small icon view.
    ///
    /// See [C++ `wxListCtrl::Arrange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a18017016afaf8f9b564df7eb692e5e5f).
    fn arrange(&self, flag: c_int) -> bool {
        unsafe { ffi::wxListCtrl_Arrange(self.as_ptr(), flag) }
    }
    /// Sets the image list associated with the control and takes ownership of it.
    ///
    /// See [C++ `wxListCtrl::AssignImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#abea2212faca2067fc5a8b8ce261906f2).
    fn assign_image_list<I: ImageListMethods>(&self, image_list: Option<&I>, which: c_int) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxListCtrl_AssignImageList(self.as_ptr(), image_list, which)
        }
    }
    /// Deletes all items and all columns.
    ///
    /// See [C++ `wxListCtrl::ClearAll()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a834e6ee38695323f882ad2ee5cc7a979).
    fn clear_all(&self) {
        unsafe { ffi::wxListCtrl_ClearAll(self.as_ptr()) }
    }
    /// Delete all columns in the list control.
    ///
    /// See [C++ `wxListCtrl::DeleteAllColumns()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a02a7ce5ad3eb09cd1b5af937f118a9e4).
    fn delete_all_columns(&self) -> bool {
        unsafe { ffi::wxListCtrl_DeleteAllColumns(self.as_ptr()) }
    }
    /// Deletes all items in the list control.
    ///
    /// See [C++ `wxListCtrl::DeleteAllItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aa9fe817f4a8fd36fca7e8cd5e9818291).
    fn delete_all_items(&self) -> bool {
        unsafe { ffi::wxListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Deletes a column.
    ///
    /// See [C++ `wxListCtrl::DeleteColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a6d30022b48a69999ceafa8c6420bba5e).
    fn delete_column(&self, col: c_int) -> bool {
        unsafe { ffi::wxListCtrl_DeleteColumn(self.as_ptr(), col) }
    }
    /// Deletes the specified item.
    ///
    /// See [C++ `wxListCtrl::DeleteItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aa4d9e2fd4e35137f970b65f0d60b6058).
    fn delete_item(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_DeleteItem(self.as_ptr(), item) }
    }
    /// Starts editing the label of the given item.
    ///
    /// See [C++ `wxListCtrl::EditLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a40011b6ced0490a8b396bdb53a682faa).
    fn edit_label<C: ClassInfoMethods>(
        &self,
        item: c_long,
        text_control_class: Option<&C>,
    ) -> WeakRef<TextCtrl> {
        unsafe {
            let text_control_class = match text_control_class {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<TextCtrl>::from(ffi::wxListCtrl_EditLabel(
                self.as_ptr(),
                item,
                text_control_class,
            ))
        }
    }
    /// Enable alternating row background colours (also called zebra striping).
    ///
    /// See [C++ `wxListCtrl::EnableAlternateRowColours()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a274e98c1c5101725bbdca9b19d8678a4).
    fn enable_alternate_row_colours(&self, enable: bool) {
        unsafe { ffi::wxListCtrl_EnableAlternateRowColours(self.as_ptr(), enable) }
    }
    /// Enable or disable a beep if there is no match for the currently entered text when searching for the item from keyboard.
    ///
    /// See [C++ `wxListCtrl::EnableBellOnNoMatch()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a88057252d356c0b6f9a6eaaac7a91ceb).
    fn enable_bell_on_no_match(&self, on: bool) {
        unsafe { ffi::wxListCtrl_EnableBellOnNoMatch(self.as_ptr(), on) }
    }
    /// Finish editing the label.
    ///
    /// See [C++ `wxListCtrl::EndEditLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a037b5c94e79661110663d542f093d3a4).
    fn end_edit_label(&self, cancel: bool) -> bool {
        unsafe { ffi::wxListCtrl_EndEditLabel(self.as_ptr(), cancel) }
    }
    /// Ensures this item is visible.
    ///
    /// See [C++ `wxListCtrl::EnsureVisible()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a59373c7c7b3b02522c40f97dd6493534).
    fn ensure_visible(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_EnsureVisible(self.as_ptr(), item) }
    }
    /// Find an item whose label matches this string, starting from start or the beginning if start is -1.
    ///
    /// See [C++ `wxListCtrl::FindItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ad8d64b6d1b4af0df31e61ae9521388d4).
    fn find_item_str(&self, start: c_long, str: &str, partial: bool) -> c_long {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxListCtrl_FindItem(self.as_ptr(), start, str, partial)
        }
    }
    // NOT_SUPPORTED: fn FindItem1()
    /// Find an item nearest this position in the specified direction, starting from start or the beginning if start is -1.
    ///
    /// See [C++ `wxListCtrl::FindItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ae46c0aa927816ae5b10afeea7a040e47).
    fn find_item_point<P: PointMethods>(&self, start: c_long, pt: &P, direction: c_int) -> c_long {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxListCtrl_FindItem2(self.as_ptr(), start, pt, direction)
        }
    }
    /// Gets information about this column.
    ///
    /// See [C++ `wxListCtrl::GetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a48e4808a48ba5f9b349ac6f725a6d8a4).
    fn get_column<L: ListItemMethods>(&self, col: c_int, item: &L) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListCtrl_GetColumn(self.as_ptr(), col, item)
        }
    }
    /// Returns the number of columns.
    ///
    /// See [C++ `wxListCtrl::GetColumnCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a48f2f5b4f062e3372d10c38542fdfc1a).
    fn get_column_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnCount(self.as_ptr()) }
    }
    /// Gets the column index from its position in visual order.
    ///
    /// See [C++ `wxListCtrl::GetColumnIndexFromOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a79411a2c63b0bc9bd6d9d48838aa86c1).
    fn get_column_index_from_order(&self, pos: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnIndexFromOrder(self.as_ptr(), pos) }
    }
    /// Gets the column visual order position.
    ///
    /// See [C++ `wxListCtrl::GetColumnOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aab1126b7921769b13f634a7f3a9a88a9).
    fn get_column_order(&self, col: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnOrder(self.as_ptr(), col) }
    }
    /// Gets the column width (report view only).
    ///
    /// See [C++ `wxListCtrl::GetColumnWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a1dee53dfab57448b307e50394caa581d).
    fn get_column_width(&self, col: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnWidth(self.as_ptr(), col) }
    }
    /// Returns the array containing the orders of all columns.
    ///
    /// See [C++ `wxListCtrl::GetColumnsOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aa239433ed39608fba50692a2100c17cc).
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxListCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    /// Gets the number of items that can fit vertically in the visible area of the list control (list or report view) or the total number of items in the list control (icon or small icon view).
    ///
    /// See [C++ `wxListCtrl::GetCountPerPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#af3e656c293986ee6cc397bdc517ce5f7).
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetCountPerPage(self.as_ptr()) }
    }
    /// Returns the edit control being currently used to edit a label.
    ///
    /// See [C++ `wxListCtrl::GetEditControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a3c563c1b426da437b195dca3a4adb695).
    fn get_edit_control(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxListCtrl_GetEditControl(self.as_ptr())) }
    }
    /// Returns the specified image list.
    ///
    /// See [C++ `wxListCtrl::GetImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aed8cdb5d1e453c91a764dd202b50b8b6).
    fn get_image_list(&self, which: c_int) -> Option<ImageListFromCpp<false>> {
        unsafe { ImageList::option_from(ffi::wxListCtrl_GetImageList(self.as_ptr(), which)) }
    }
    /// Gets information about the item.
    ///
    /// See [C++ `wxListCtrl::GetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a74369e6c808beefe37ff5a6fe141784f).
    fn get_item<L: ListItemMethods>(&self, info: &L) -> bool {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_GetItem(self.as_ptr(), info)
        }
    }
    /// Returns the colour for this item.
    ///
    /// See [C++ `wxListCtrl::GetItemBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a6f95294168e46663aaa3ab1712429649).
    fn get_item_background_colour(&self, item: c_long) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetItemBackgroundColour(self.as_ptr(), item)) }
    }
    /// Returns the number of items in the list control.
    ///
    /// See [C++ `wxListCtrl::GetItemCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a8d09520c767e1b09db6586bea6afa77a).
    fn get_item_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    /// Returns the item's font.
    ///
    /// See [C++ `wxListCtrl::GetItemFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac51bdea8ff28404780f43d8d12bbf39f).
    fn get_item_font(&self, item: c_long) -> Font {
        unsafe { Font::from_ptr(ffi::wxListCtrl_GetItemFont(self.as_ptr(), item)) }
    }
    /// Returns the position of the item, in icon or small icon view.
    ///
    /// See [C++ `wxListCtrl::GetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a0215890b6e8ac6ae49730d01f3f8ffca).
    fn get_item_position<P: PointMethods>(&self, item: c_long, pos: &P) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxListCtrl_GetItemPosition(self.as_ptr(), item, pos)
        }
    }
    /// Returns the rectangle representing the item's size and position, in physical coordinates.
    ///
    /// See [C++ `wxListCtrl::GetItemRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a7672f86c151cfa91c0df7d31c6509fea).
    fn get_item_rect<R: RectMethods>(&self, item: c_long, rect: &R, code: c_int) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxListCtrl_GetItemRect(self.as_ptr(), item, rect, code)
        }
    }
    /// Retrieves the spacing between icons in pixels: horizontal spacing is returned as x component of the wxSize object and the vertical spacing as its y component.
    ///
    /// See [C++ `wxListCtrl::GetItemSpacing()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a2b50b4871a486561192e5ecb16658dc5).
    fn get_item_spacing(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxListCtrl_GetItemSpacing(self.as_ptr())) }
    }
    /// Gets the item state.
    ///
    /// See [C++ `wxListCtrl::GetItemState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#adfe1f96d1262dbfb51a79c31f4c6a77f).
    fn get_item_state(&self, item: c_long, state_mask: c_long) -> c_int {
        unsafe { ffi::wxListCtrl_GetItemState(self.as_ptr(), item, state_mask) }
    }
    /// Gets the item text for this item.
    ///
    /// See [C++ `wxListCtrl::GetItemText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#af6030eb517baa0c331aa4424f53e915f).
    fn get_item_text(&self, item: c_long, col: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxListCtrl_GetItemText(self.as_ptr(), item, col)).into() }
    }
    /// Returns the colour for this item.
    ///
    /// See [C++ `wxListCtrl::GetItemTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a52400b14563d1ff8fa7e9728a5bbf3ae).
    fn get_item_text_colour(&self, item: c_long) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetItemTextColour(self.as_ptr(), item)) }
    }
    /// Searches for an item with the given geometry or state, starting from item but excluding the item itself.
    ///
    /// See [C++ `wxListCtrl::GetNextItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a4d6793dc14feadac68516da9bf21cd67).
    fn get_next_item(&self, item: c_long, geometry: c_int, state: c_int) -> c_long {
        unsafe { ffi::wxListCtrl_GetNextItem(self.as_ptr(), item, geometry, state) }
    }
    /// Returns the number of selected items in the list control.
    ///
    /// See [C++ `wxListCtrl::GetSelectedItemCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#abfeaae84557fbbbf67ae1703dc4d61d9).
    fn get_selected_item_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetSelectedItemCount(self.as_ptr()) }
    }
    /// Returns the rectangle representing the size and position, in physical coordinates, of the given subitem, i.e.
    ///
    /// See [C++ `wxListCtrl::GetSubItemRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac528fabd73e6947a0760f804d53e9a30).
    fn get_sub_item_rect<R: RectMethods>(
        &self,
        item: c_long,
        sub_item: c_long,
        rect: &R,
        code: c_int,
    ) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxListCtrl_GetSubItemRect(self.as_ptr(), item, sub_item, rect, code)
        }
    }
    /// Gets the text colour of the list control.
    ///
    /// See [C++ `wxListCtrl::GetTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a4ea334b0fdc8b16e5aaf5ab3d5a10395).
    fn get_text_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetTextColour(self.as_ptr())) }
    }
    /// Gets the index of the topmost visible item when in list or report view.
    ///
    /// See [C++ `wxListCtrl::GetTopItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a837666df6594a128cb6d1eca00fadbb7).
    fn get_top_item(&self) -> c_long {
        unsafe { ffi::wxListCtrl_GetTopItem(self.as_ptr()) }
    }
    /// Returns the rectangle taken by all items in the control.
    ///
    /// See [C++ `wxListCtrl::GetViewRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ab1032138f5ff418bbe2e13ab5ea5d659).
    fn get_view_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxListCtrl_GetViewRect(self.as_ptr())) }
    }
    /// Set the alternative row background colour to a specific colour.
    ///
    /// See [C++ `wxListCtrl::SetAlternateRowColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a7ed18be1794a1db88f264cf641b738da).
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxListCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    /// Get the alternative row background colour.
    ///
    /// See [C++ `wxListCtrl::GetAlternateRowColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac38c2f5ac8d2fcfee3f4c4d1885c9dde).
    fn get_alternate_row_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetAlternateRowColour(self.as_ptr())) }
    }
    /// Determines which item (if any) is at the specified point, giving details in flags.
    ///
    /// See [C++ `wxListCtrl::HitTest()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac75506f2203ffa9fbebcd8ecf240f27c).
    fn hit_test<P: PointMethods>(
        &self,
        point: &P,
        flags: *mut c_void,
        ptr_sub_item: *mut c_void,
    ) -> c_long {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListCtrl_HitTest(self.as_ptr(), point, flags, ptr_sub_item)
        }
    }
    /// Returns true if the control is currently using wxLC_REPORT style.
    ///
    /// See [C++ `wxListCtrl::InReportView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ad05564e94d55f56bf7b10f960acb3e20).
    fn in_report_view(&self) -> bool {
        unsafe { ffi::wxListCtrl_InReportView(self.as_ptr()) }
    }
    /// For report view mode (only), inserts a column.
    ///
    /// See [C++ `wxListCtrl::InsertColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a75337fbb43f767eafc82d47966943141).
    fn insert_column_listitem<L: ListItemMethods>(&self, col: c_long, info: &L) -> c_long {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_InsertColumn(self.as_ptr(), col, info)
        }
    }
    /// For report view mode (only), inserts a column.
    ///
    /// See [C++ `wxListCtrl::InsertColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#abd8d55b0f89e9a63372e3837c19c32d8).
    fn insert_column_str(&self, col: c_long, heading: &str, format: c_int, width: c_int) -> c_long {
        unsafe {
            let heading = WxString::from(heading);
            let heading = heading.as_ptr();
            ffi::wxListCtrl_InsertColumn1(self.as_ptr(), col, heading, format, width)
        }
    }
    /// Inserts an item, returning the index of the new item if successful, -1 otherwise.
    ///
    /// See [C++ `wxListCtrl::InsertItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aa65a5ca4e7bd7c0e60a0522a9092a693).
    fn insert_item_listitem<L: ListItemMethods>(&self, info: &L) -> c_long {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_InsertItem(self.as_ptr(), info)
        }
    }
    /// Insert a string item.
    ///
    /// See [C++ `wxListCtrl::InsertItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a66128e3a4806a7b545e83b6de0f49ed5).
    fn insert_item_long_str(&self, index: c_long, label: &str) -> c_long {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_InsertItem1(self.as_ptr(), index, label)
        }
    }
    /// Insert an image item.
    ///
    /// See [C++ `wxListCtrl::InsertItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a5a01ced78c631b056d781cf6f2ae6626).
    fn insert_item_long_int(&self, index: c_long, image_index: c_int) -> c_long {
        unsafe { ffi::wxListCtrl_InsertItem2(self.as_ptr(), index, image_index) }
    }
    /// Insert an image/string item.
    ///
    /// See [C++ `wxListCtrl::InsertItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ad06b98bdb32f5d3b15703db8e8f5f5c3).
    fn insert_item_long_str_int(&self, index: c_long, label: &str, image_index: c_int) -> c_long {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_InsertItem3(self.as_ptr(), index, label, image_index)
        }
    }
    /// Returns true if the control doesn't currently contain any items.
    ///
    /// See [C++ `wxListCtrl::IsEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a8f3364477562fde3dc5e05b07daa6d42).
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsEmpty(self.as_ptr()) }
    }
    /// Returns true if the control is currently in virtual report view.
    ///
    /// See [C++ `wxListCtrl::IsVirtual()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a69ec74781f9eeee8b1edda6f984be85d).
    fn is_virtual(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsVirtual(self.as_ptr()) }
    }
    /// Redraws the given item.
    ///
    /// See [C++ `wxListCtrl::RefreshItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ab8a88dac752a883ad4e02bb42a7d5015).
    fn refresh_item(&self, item: c_long) {
        unsafe { ffi::wxListCtrl_RefreshItem(self.as_ptr(), item) }
    }
    /// Redraws the items between itemFrom and itemTo.
    ///
    /// See [C++ `wxListCtrl::RefreshItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aaa6b0180a9dcdf46be8ec4baffc61efc).
    fn refresh_items(&self, item_from: c_long, item_to: c_long) {
        unsafe { ffi::wxListCtrl_RefreshItems(self.as_ptr(), item_from, item_to) }
    }
    /// Scrolls the list control.
    ///
    /// See [C++ `wxListCtrl::ScrollList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a7e620aa7cc9953d5e98846b130034135).
    fn scroll_list(&self, dx: c_int, dy: c_int) -> bool {
        unsafe { ffi::wxListCtrl_ScrollList(self.as_ptr(), dx, dy) }
    }
    /// Sets information about this column.
    ///
    /// See [C++ `wxListCtrl::SetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a61244b9721214359b0ad3992135920c4).
    fn set_column<L: ListItemMethods>(&self, col: c_int, item: &L) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListCtrl_SetColumn(self.as_ptr(), col, item)
        }
    }
    /// Sets the column width.
    ///
    /// See [C++ `wxListCtrl::SetColumnWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac5818d43ce3913f7f70a6a759485cd6c).
    fn set_column_width(&self, col: c_int, width: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetColumnWidth(self.as_ptr(), col, width) }
    }
    /// Changes the order in which the columns are shown.
    ///
    /// See [C++ `wxListCtrl::SetColumnsOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a2a499df49ebcefb3451a44146f76457d).
    fn set_columns_order<A: ArrayIntMethods>(&self, orders: &A) -> bool {
        unsafe {
            let orders = orders.as_ptr();
            ffi::wxListCtrl_SetColumnsOrder(self.as_ptr(), orders)
        }
    }
    /// Change the font and the colours used for the list control header.
    ///
    /// See [C++ `wxListCtrl::SetHeaderAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a2156d06bdb31314e1d179c23d786c5a8).
    fn set_header_attr(&self, attr: *const c_void) -> bool {
        unsafe { ffi::wxListCtrl_SetHeaderAttr(self.as_ptr(), attr) }
    }
    /// Sets the image list associated with the control.
    ///
    /// See [C++ `wxListCtrl::SetImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a9a6b0ebe6f4b0a8fbef31c4e17fe235f).
    fn set_image_list<I: ImageListMethods>(&self, image_list: Option<&I>, which: c_int) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxListCtrl_SetImageList(self.as_ptr(), image_list, which)
        }
    }
    /// Sets the images to use when showing large, normal icons in this control.
    ///
    /// See [C++ `wxListCtrl::SetNormalImages()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac317f95793cbb9f1b5be99cc84360d20).
    fn set_normal_images(&self, images: *const c_void) {
        unsafe { ffi::wxListCtrl_SetNormalImages(self.as_ptr(), images) }
    }
    /// Sets the images to use when showing small icons in this control.
    ///
    /// See [C++ `wxListCtrl::SetSmallImages()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a07ed585d078008433b52e8cadebaa928).
    fn set_small_images(&self, images: *const c_void) {
        unsafe { ffi::wxListCtrl_SetSmallImages(self.as_ptr(), images) }
    }
    /// Check if the item is visible.
    ///
    /// See [C++ `wxListCtrl::IsVisible()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ac426bbaed62604c032698bf0d26b4324).
    fn is_visible(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_IsVisible(self.as_ptr(), item) }
    }
    /// Sets the data of an item.
    ///
    /// See [C++ `wxListCtrl::SetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a65a4d9f5fa5bc363e4a93317777d57b4).
    fn set_item_listitem<L: ListItemMethods>(&self, info: &L) -> bool {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_SetItem(self.as_ptr(), info)
        }
    }
    /// Sets an item string field at a particular column.
    ///
    /// See [C++ `wxListCtrl::SetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a1068a0171489a2a5d322a290da3dd10d).
    fn set_item_long(&self, index: c_long, column: c_int, label: &str, image_id: c_int) -> bool {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_SetItem1(self.as_ptr(), index, column, label, image_id)
        }
    }
    /// Sets the background colour for this item.
    ///
    /// See [C++ `wxListCtrl::SetItemBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#adf07b82511dc77d64097c0ffefdc9a30).
    fn set_item_background_colour<C: ColourMethods>(&self, item: c_long, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetItemBackgroundColour(self.as_ptr(), item, col)
        }
    }
    /// Sets the image associated with the item.
    ///
    /// See [C++ `wxListCtrl::SetItemColumnImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a83d08f3aefb0a20b25fad05f19fad67d).
    fn set_item_column_image(&self, item: c_long, column: c_long, image: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetItemColumnImage(self.as_ptr(), item, column, image) }
    }
    /// This method can only be used with virtual list controls.
    ///
    /// See [C++ `wxListCtrl::SetItemCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a5ded9342e4969eeeb60d1305f75995fd).
    fn set_item_count(&self, count: c_long) {
        unsafe { ffi::wxListCtrl_SetItemCount(self.as_ptr(), count) }
    }
    /// Associates application-defined data with this item.
    ///
    /// See [C++ `wxListCtrl::SetItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a54825c8c66e7fb59cee6076d6554f8ec).
    fn set_item_data(&self, item: c_long, data: c_long) -> bool {
        unsafe { ffi::wxListCtrl_SetItemData(self.as_ptr(), item, data) }
    }
    /// Sets the item's font.
    ///
    /// See [C++ `wxListCtrl::SetItemFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#acfc640f1aba5634ab25186e71ffc278f).
    fn set_item_font<F: FontMethods>(&self, item: c_long, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxListCtrl_SetItemFont(self.as_ptr(), item, font)
        }
    }
    /// Sets the unselected and selected images associated with the item.
    ///
    /// See [C++ `wxListCtrl::SetItemImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#afe006faba05c34bbc70539f100e72c86).
    fn set_item_image(&self, item: c_long, image: c_int, sel_image: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetItemImage(self.as_ptr(), item, image, sel_image) }
    }
    /// Sets the position of the item, in icon or small icon view.
    ///
    /// See [C++ `wxListCtrl::SetItemPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a13525147c745a26eeb54fe55dfdbad46).
    fn set_item_position<P: PointMethods>(&self, item: c_long, pos: &P) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxListCtrl_SetItemPosition(self.as_ptr(), item, pos)
        }
    }
    // NOT_SUPPORTED: fn SetItemPtrData()
    /// Sets the item state.
    ///
    /// See [C++ `wxListCtrl::SetItemState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a172d78854092aec2b784b8519bf5be08).
    fn set_item_state(&self, item: c_long, state: c_long, state_mask: c_long) -> bool {
        unsafe { ffi::wxListCtrl_SetItemState(self.as_ptr(), item, state, state_mask) }
    }
    /// Sets the item text for this item.
    ///
    /// See [C++ `wxListCtrl::SetItemText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#af4d70a5a6a15011590fa0b94254ecbd9).
    fn set_item_text(&self, item: c_long, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxListCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
    /// Sets the colour for this item.
    ///
    /// See [C++ `wxListCtrl::SetItemTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a069801dda4c0df09ddb0c7cdefcdaf11).
    fn set_item_text_colour<C: ColourMethods>(&self, item: c_long, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetItemTextColour(self.as_ptr(), item, col)
        }
    }
    /// Adds or removes a single window style.
    ///
    /// See [C++ `wxListCtrl::SetSingleStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a20daa8d70dc41ba23d00bcf6a652ab28).
    fn set_single_style(&self, style: c_long, add: bool) {
        unsafe { ffi::wxListCtrl_SetSingleStyle(self.as_ptr(), style, add) }
    }
    /// Sets the text colour of the list control.
    ///
    /// See [C++ `wxListCtrl::SetTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#af3e129f2ac6412c8bd602f286b95300f).
    fn set_text_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetTextColour(self.as_ptr(), col)
        }
    }
    // NOT_SUPPORTED: fn SortItems()
    /// Returns true if checkboxes are enabled for list items.
    ///
    /// See [C++ `wxListCtrl::HasCheckBoxes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a04cd9e9a27891d7c2a80b3913c54de4b).
    fn has_check_boxes(&self) -> bool {
        unsafe { ffi::wxListCtrl_HasCheckBoxes(self.as_ptr()) }
    }
    /// Enable or disable checkboxes for list items.
    ///
    /// See [C++ `wxListCtrl::EnableCheckBoxes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a3503862366737d341be95fd25c93abcb).
    fn enable_check_boxes(&self, enable: bool) -> bool {
        unsafe { ffi::wxListCtrl_EnableCheckBoxes(self.as_ptr(), enable) }
    }
    /// Return true if the checkbox for the given wxListItem is checked.
    ///
    /// See [C++ `wxListCtrl::IsItemChecked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ae4c5c73fc41dc60722facff4682dd4dd).
    fn is_item_checked(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_IsItemChecked(self.as_ptr(), item) }
    }
    /// Check or uncheck a wxListItem in a control using checkboxes.
    ///
    /// See [C++ `wxListCtrl::CheckItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a4377a54ed8eb209b4cae6eb18f2d31ea).
    fn check_item(&self, item: c_long, check: bool) {
        unsafe { ffi::wxListCtrl_CheckItem(self.as_ptr(), item, check) }
    }
    /// Extend rules and alternate rows background to the entire client area.
    ///
    /// See [C++ `wxListCtrl::ExtendRulesAndAlternateColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aae3a67d7ce557e1948d66a2421da8525).
    fn extend_rules_and_alternate_colour(&self, extend: bool) {
        unsafe { ffi::wxListCtrl_ExtendRulesAndAlternateColour(self.as_ptr(), extend) }
    }
    /// Show the sort indicator of a specific column in a specific direction.
    ///
    /// See [C++ `wxListCtrl::ShowSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#afad8a23566cb0ad94b6fa9c65ffce393).
    fn show_sort_indicator(&self, col: c_int, ascending: bool) {
        unsafe { ffi::wxListCtrl_ShowSortIndicator(self.as_ptr(), col, ascending) }
    }
    /// Remove the sort indicator from the column being used as sort key.
    ///
    /// See [C++ `wxListCtrl::RemoveSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a108e4866e67e8e21499a3c446fade60c).
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxListCtrl_RemoveSortIndicator(self.as_ptr()) }
    }
    /// Returns the column that shows the sort indicator.
    ///
    /// See [C++ `wxListCtrl::GetSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#ab4477feda03c38625948f5c177c4efcc).
    fn get_sort_indicator(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetSortIndicator(self.as_ptr()) }
    }
    /// Returns the new value to use for sort indicator after clicking a column.
    ///
    /// See [C++ `wxListCtrl::GetUpdatedAscendingSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#a4e5233990e755f189fec6dac344e0264).
    fn get_updated_ascending_sort_indicator(&self, col: c_int) -> bool {
        unsafe { ffi::wxListCtrl_GetUpdatedAscendingSortIndicator(self.as_ptr(), col) }
    }
    /// Returns true if the sort indicator direction is ascending, false when the direction is descending.
    ///
    /// See [C++ `wxListCtrl::IsAscendingSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_ctrl.html#aec16d750ec7a93021573cc8acead72f3).
    fn is_ascending_sort_indicator(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsAscendingSortIndicator(self.as_ptr()) }
    }
}

// wxListEvent
/// This trait represents [C++ `wxListEvent` class](https://docs.wxwidgets.org/3.2/classwx_list_event.html)'s methods and inheritance.
///
/// See [`ListEventFromCpp`] documentation for the class usage.
pub trait ListEventMethods: NotifyEventMethods {
    /// For EVT_LIST_CACHE_HINT event only: return the first item which the list control advises us to cache.
    ///
    /// See [C++ `wxListEvent::GetCacheFrom()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a5e643c082b091490d5a3fa4eaca63582).
    fn get_cache_from(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetCacheFrom(self.as_ptr()) }
    }
    /// For EVT_LIST_CACHE_HINT event only: return the last item (inclusive) which the list control advises us to cache.
    ///
    /// See [C++ `wxListEvent::GetCacheTo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a89d6e5bfec88d1da90d1f95a24b8a0ba).
    fn get_cache_to(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetCacheTo(self.as_ptr()) }
    }
    /// The column position: it is only used with COL events.
    ///
    /// See [C++ `wxListEvent::GetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a299056538a8aa23f318bd49420cb80ca).
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetColumn(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetData()
    /// The image.
    ///
    /// See [C++ `wxListEvent::GetImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a466b53ba7611757a327b8fe9d8600b3a).
    fn get_image(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetImage(self.as_ptr()) }
    }
    /// The item index.
    ///
    /// See [C++ `wxListEvent::GetIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#adbfa912ed37faa860c988d4ebe0a976f).
    fn get_index(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetIndex(self.as_ptr()) }
    }
    /// An item object, used by some events.
    ///
    /// See [C++ `wxListEvent::GetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#aa855597658bab6bbdf4ab0d85223724d).
    fn get_item(&self) -> ListItemFromCpp<false> {
        unsafe { ListItemFromCpp::from_ptr(ffi::wxListEvent_GetItem(self.as_ptr())) }
    }
    /// Key code if the event is a keypress event.
    ///
    /// See [C++ `wxListEvent::GetKeyCode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#aae42616e1339d2a3716943bacf36cb43).
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetKeyCode(self.as_ptr()) }
    }
    /// The (new) item label for EVT_LIST_END_LABEL_EDIT event.
    ///
    /// See [C++ `wxListEvent::GetLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a4ca8208521d7989f5f3207f247882e67).
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListEvent_GetLabel(self.as_ptr())).into() }
    }
    /// The mask.
    ///
    /// See [C++ `wxListEvent::GetMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#aae0fdeb8a268a8554df044616cfe8e70).
    fn get_mask(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetMask(self.as_ptr()) }
    }
    /// The position of the mouse pointer if the event is a drag event.
    ///
    /// See [C++ `wxListEvent::GetPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#ae67f9aaec8dddd51224948e6a27527ce).
    fn get_point(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxListEvent_GetPoint(self.as_ptr())) }
    }
    /// The text.
    ///
    /// See [C++ `wxListEvent::GetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#adb157213ff27b811d974e611fe1355e0).
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListEvent_GetText(self.as_ptr())).into() }
    }
    /// This method only makes sense for EVT_LIST_END_LABEL_EDIT message and returns true if it the label editing has been cancelled by the user (GetLabel() returns an empty string in this case but it doesn't allow the application to distinguish between really cancelling the edit and the admittedly rare case when the user wants to rename it to an empty string).
    ///
    /// See [C++ `wxListEvent::IsEditCancelled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a5777db14d2fdf3f0e45940d525239ca6).
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxListEvent_IsEditCancelled(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxListEvent::SetKeyCode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#ab0f010e03c879a4339022d55f8225a55).
    fn set_key_code(&self, code: c_int) {
        unsafe { ffi::wxListEvent_SetKeyCode(self.as_ptr(), code) }
    }
    ///
    /// See [C++ `wxListEvent::SetIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a5d37c0181944bf5a7fba2c16a4ef697c).
    fn set_index(&self, index: c_long) {
        unsafe { ffi::wxListEvent_SetIndex(self.as_ptr(), index) }
    }
    ///
    /// See [C++ `wxListEvent::SetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#acc4827b22db3c75be8385b1fd154ec64).
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxListEvent_SetColumn(self.as_ptr(), col) }
    }
    ///
    /// See [C++ `wxListEvent::SetPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a940ea698bd2d49f0185beccb293495f4).
    fn set_point<P: PointMethods>(&self, point: &P) {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListEvent_SetPoint(self.as_ptr(), point)
        }
    }
    ///
    /// See [C++ `wxListEvent::SetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#ab74d9e8fc977e0f01dfa046a53748ea6).
    fn set_item<L: ListItemMethods>(&self, item: &L) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListEvent_SetItem(self.as_ptr(), item)
        }
    }
    ///
    /// See [C++ `wxListEvent::SetCacheFrom()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#ab2af06d15f3dcf206c96a43ed493c339).
    fn set_cache_from(&self, cache_from: c_long) {
        unsafe { ffi::wxListEvent_SetCacheFrom(self.as_ptr(), cache_from) }
    }
    ///
    /// See [C++ `wxListEvent::SetCacheTo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_event.html#a204a2d5f615e94e099148aeac3cf2c17).
    fn set_cache_to(&self, cache_to: c_long) {
        unsafe { ffi::wxListEvent_SetCacheTo(self.as_ptr(), cache_to) }
    }
}

// wxListItem
/// This trait represents [C++ `wxListItem` class](https://docs.wxwidgets.org/3.2/classwx_list_item.html)'s methods and inheritance.
///
/// See [`ListItemFromCpp`] documentation for the class usage.
pub trait ListItemMethods: ObjectMethods {
    /// Resets the item state to the default.
    ///
    /// See [C++ `wxListItem::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a42119c1285a87375beca516b0bfa1879).
    fn clear(&self) {
        unsafe { ffi::wxListItem_Clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAlign()
    /// Returns the background colour for this item.
    ///
    /// See [C++ `wxListItem::GetBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#af547f42466edce3a9c4f7afe40109947).
    fn get_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListItem_GetBackgroundColour(self.as_ptr())) }
    }
    /// Returns the zero-based column; meaningful only in report mode.
    ///
    /// See [C++ `wxListItem::GetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a176b3053aae9c4d5148d3325cec807fe).
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxListItem_GetColumn(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetData()
    /// Returns the font used to display the item.
    ///
    /// See [C++ `wxListItem::GetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#acccc743a259f64a03b7378f53435cd5b).
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxListItem_GetFont(self.as_ptr())) }
    }
    /// Returns the zero-based item position.
    ///
    /// See [C++ `wxListItem::GetId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#af6c503004269a32e0f60cb57b1d53ca6).
    fn get_id(&self) -> c_long {
        unsafe { ffi::wxListItem_GetId(self.as_ptr()) }
    }
    /// Returns the zero-based index of the image associated with the item into the image list.
    ///
    /// See [C++ `wxListItem::GetImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#ae4e4708662ba3e071e5afa590c4e3021).
    fn get_image(&self) -> c_int {
        unsafe { ffi::wxListItem_GetImage(self.as_ptr()) }
    }
    /// Returns a bit mask indicating which fields of the structure are valid.
    ///
    /// See [C++ `wxListItem::GetMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#ab09bea19faf49127e61958e6c1c86b9a).
    fn get_mask(&self) -> c_long {
        unsafe { ffi::wxListItem_GetMask(self.as_ptr()) }
    }
    /// Returns a bit field representing the state of the item.
    ///
    /// See [C++ `wxListItem::GetState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a1995f4e9fecf37d78dacb2eb55aca4a9).
    fn get_state(&self) -> c_long {
        unsafe { ffi::wxListItem_GetState(self.as_ptr()) }
    }
    /// Returns the label/header text.
    ///
    /// See [C++ `wxListItem::GetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a52e959d5da4459c6f64babbf2f99bc26).
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListItem_GetText(self.as_ptr())).into() }
    }
    /// Returns the text colour.
    ///
    /// See [C++ `wxListItem::GetTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a00db1511d4fbe22d3a1a0435be9011ac).
    fn get_text_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListItem_GetTextColour(self.as_ptr())) }
    }
    /// Meaningful only for column headers in report mode.
    ///
    /// See [C++ `wxListItem::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a24d055a0ab8c9825693006c174898af8).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxListItem_GetWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAlign()
    /// Sets the background colour for the item.
    ///
    /// See [C++ `wxListItem::SetBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a0bebe32d7a97890ace77ee349b9011b5).
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxListItem_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    /// Sets the zero-based column.
    ///
    /// See [C++ `wxListItem::SetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#ac6c4c2a7471d8ed69de4d13ce62daa0c).
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxListItem_SetColumn(self.as_ptr(), col) }
    }
    /// Sets client data for the item.
    ///
    /// See [C++ `wxListItem::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a2fbe1d24df53a081c5b8d4a93248bfd7).
    fn set_data_long(&self, data: c_long) {
        unsafe { ffi::wxListItem_SetData(self.as_ptr(), data) }
    }
    ///
    /// See [C++ `wxListItem::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#afe3c57a6238f1fd9bcf467304a0afc74).
    fn set_data_void(&self, data: *mut c_void) {
        unsafe { ffi::wxListItem_SetData1(self.as_ptr(), data) }
    }
    /// Sets the font for the item.
    ///
    /// See [C++ `wxListItem::SetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a10a0a06869c8e1e55458cc1109f17a28).
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxListItem_SetFont(self.as_ptr(), font)
        }
    }
    /// Sets the zero-based item position.
    ///
    /// See [C++ `wxListItem::SetId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#acc586bd9e92693a7405366ed7ab5cdc0).
    fn set_id(&self, id: c_long) {
        unsafe { ffi::wxListItem_SetId(self.as_ptr(), id) }
    }
    /// Sets the zero-based index of the image associated with the item into the image list.
    ///
    /// See [C++ `wxListItem::SetImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#ad0b1de6940857d814a7f175ee0076dc1).
    fn set_image(&self, image: c_int) {
        unsafe { ffi::wxListItem_SetImage(self.as_ptr(), image) }
    }
    /// Sets the mask of valid fields.
    ///
    /// See [C++ `wxListItem::SetMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a2b9395037783f0f44f87f835c1b071f6).
    fn set_mask(&self, mask: c_long) {
        unsafe { ffi::wxListItem_SetMask(self.as_ptr(), mask) }
    }
    /// Sets the item state flags (note that the valid state flags are influenced by the value of the state mask, see wxListItem::SetStateMask).
    ///
    /// See [C++ `wxListItem::SetState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a6194edc9db9f4f521e1873d5ae6f6a42).
    fn set_state(&self, state: c_long) {
        unsafe { ffi::wxListItem_SetState(self.as_ptr(), state) }
    }
    /// Sets the bitmask that is used to determine which of the state flags are to be set.
    ///
    /// See [C++ `wxListItem::SetStateMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#aaa3a97f86342d4dddd2ddf3b2b29fb01).
    fn set_state_mask(&self, state_mask: c_long) {
        unsafe { ffi::wxListItem_SetStateMask(self.as_ptr(), state_mask) }
    }
    /// Sets the text label for the item.
    ///
    /// See [C++ `wxListItem::SetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#aa3c69c9507833b6081edc280df17d6e0).
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxListItem_SetText(self.as_ptr(), text)
        }
    }
    /// Sets the text colour for the item.
    ///
    /// See [C++ `wxListItem::SetTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#a9e971c1815de2f34e6ef132ea71e457a).
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxListItem_SetTextColour(self.as_ptr(), col_text)
        }
    }
    /// Meaningful only for column headers in report mode.
    ///
    /// See [C++ `wxListItem::SetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_item.html#abd2490fdc76ee4bdfdc8d7450f36f680).
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxListItem_SetWidth(self.as_ptr(), width) }
    }
}

// wxListView
/// This trait represents [C++ `wxListView` class](https://docs.wxwidgets.org/3.2/classwx_list_view.html)'s methods and inheritance.
///
/// See [`ListViewFromCpp`] documentation for the class usage.
pub trait ListViewMethods: ListCtrlMethods {
    // DTOR: fn ~wxListView()
    /// Resets the column image  after calling this function, no image will be shown.
    ///
    /// See [C++ `wxListView::ClearColumnImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#ae37c58d623cf113d7cef451eb38bdf78).
    fn clear_column_image(&self, col: c_int) {
        unsafe { ffi::wxListView_ClearColumnImage(self.as_ptr(), col) }
    }
    /// Sets focus to the item with the given index.
    ///
    /// See [C++ `wxListView::Focus()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a5468e6e52d0cca76b1584dbb2a552adc).
    fn focus(&self, index: c_long) {
        unsafe { ffi::wxListView_Focus(self.as_ptr(), index) }
    }
    /// Returns the first selected item in a (presumably) multiple selection control.
    ///
    /// See [C++ `wxListView::GetFirstSelected()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a64681fac38de90260dd61bbb1e46ee79).
    fn get_first_selected(&self) -> c_long {
        unsafe { ffi::wxListView_GetFirstSelected(self.as_ptr()) }
    }
    /// Returns the currently focused item or -1 if none.
    ///
    /// See [C++ `wxListView::GetFocusedItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a3d0d0071b2c212b687a040ac9bebc035).
    fn get_focused_item(&self) -> c_long {
        unsafe { ffi::wxListView_GetFocusedItem(self.as_ptr()) }
    }
    /// Used together with GetFirstSelected() to iterate over all selected items in the control.
    ///
    /// See [C++ `wxListView::GetNextSelected()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a00a6445da758cabfe05952c0b87d939a).
    fn get_next_selected(&self, item: c_long) -> c_long {
        unsafe { ffi::wxListView_GetNextSelected(self.as_ptr(), item) }
    }
    /// Returns true if the item with the given index is selected, false otherwise.
    ///
    /// See [C++ `wxListView::IsSelected()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a6147a2b53877e75a93bc9cd8522e7807).
    fn is_selected(&self, index: c_long) -> bool {
        unsafe { ffi::wxListView_IsSelected(self.as_ptr(), index) }
    }
    /// Selects or unselects the given item.
    ///
    /// See [C++ `wxListView::Select()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#a02966936da5643b2d80a857676482b2b).
    fn select(&self, n: c_long, on: bool) {
        unsafe { ffi::wxListView_Select(self.as_ptr(), n, on) }
    }
    /// Sets the column image for the specified column.
    ///
    /// See [C++ `wxListView::SetColumnImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_list_view.html#ab4f169a074fbe2d6bf9c2880c84f3dee).
    fn set_column_image(&self, col: c_int, image: c_int) {
        unsafe { ffi::wxListView_SetColumnImage(self.as_ptr(), col, image) }
    }
}

// wxListbook
/// This trait represents [C++ `wxListbook` class](https://docs.wxwidgets.org/3.2/classwx_listbook.html)'s methods and inheritance.
///
/// See [`ListbookFromCpp`] documentation for the class usage.
pub trait ListbookMethods: BookCtrlBaseMethods {
    /// Returns the wxListView associated with the control.
    ///
    /// See [C++ `wxListbook::GetListView()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_listbook.html#acfe82a1f24b20d7e699b2cc94a739cc0).
    fn get_list_view(&self) -> WeakRef<ListView> {
        unsafe { WeakRef::<ListView>::from(ffi::wxListbook_GetListView(self.as_ptr())) }
    }
}
