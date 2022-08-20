use super::*;

// wxLayoutAlgorithm
pub trait LayoutAlgorithmMethods: ObjectMethods {
    // DTOR: fn ~wxLayoutAlgorithm()
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
    fn layout_mdi_frame<R: RectMethods>(&self, frame: *mut c_void, rect: Option<&R>) -> bool {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLayoutAlgorithm_LayoutMDIFrame(self.as_ptr(), frame, rect)
        }
    }
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
pub trait ListBoxMethods: ControlMethods {
    // DTOR: fn ~wxListBox()
    // NOT_SUPPORTED: fn Create()
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
    fn deselect(&self, n: c_int) {
        unsafe { ffi::wxListBox_Deselect(self.as_ptr(), n) }
    }
    fn set_string_selection_bool(&self, s: &str, select: bool) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxListBox_SetStringSelection(self.as_ptr(), s, select)
        }
    }
    fn set_string_selection(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxListBox_SetStringSelection1(self.as_ptr(), s)
        }
    }
    fn get_selections<A: ArrayIntMethods>(&self, selections: &A) -> c_int {
        unsafe {
            let selections = selections.as_ptr();
            ffi::wxListBox_GetSelections(self.as_ptr(), selections)
        }
    }
    fn hit_test_point<P: PointMethods>(&self, point: &P) -> c_int {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListBox_HitTest(self.as_ptr(), point)
        }
    }
    fn hit_test_int(&self, x: c_int, y: c_int) -> c_int {
        unsafe { ffi::wxListBox_HitTest1(self.as_ptr(), x, y) }
    }
    // BLOCKED: fn InsertItems()
    fn insert_items<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxListBox_InsertItems1(self.as_ptr(), items, pos)
        }
    }
    fn is_selected(&self, n: c_int) -> bool {
        unsafe { ffi::wxListBox_IsSelected(self.as_ptr(), n) }
    }
    fn set_first_item_int(&self, n: c_int) {
        unsafe { ffi::wxListBox_SetFirstItem(self.as_ptr(), n) }
    }
    fn set_first_item_str(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxListBox_SetFirstItem1(self.as_ptr(), string)
        }
    }
    fn ensure_visible(&self, n: c_int) {
        unsafe { ffi::wxListBox_EnsureVisible(self.as_ptr(), n) }
    }
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxListBox_IsSorted(self.as_ptr()) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListBox_GetCountPerPage(self.as_ptr()) }
    }
    fn get_top_item(&self) -> c_int {
        unsafe { ffi::wxListBox_GetTopItem(self.as_ptr()) }
    }
    // BLOCKED: fn MSWSetTabStops()
}

// wxListCtrl
pub trait ListCtrlMethods: ControlMethods {
    // DTOR: fn ~wxListCtrl()
    // NOT_SUPPORTED: fn AppendColumn()
    fn arrange(&self, flag: c_int) -> bool {
        unsafe { ffi::wxListCtrl_Arrange(self.as_ptr(), flag) }
    }
    fn assign_image_list<I: ImageListMethods>(&self, image_list: Option<&I>, which: c_int) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxListCtrl_AssignImageList(self.as_ptr(), image_list, which)
        }
    }
    fn clear_all(&self) {
        unsafe { ffi::wxListCtrl_ClearAll(self.as_ptr()) }
    }
    fn delete_all_columns(&self) -> bool {
        unsafe { ffi::wxListCtrl_DeleteAllColumns(self.as_ptr()) }
    }
    fn delete_all_items(&self) -> bool {
        unsafe { ffi::wxListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn delete_column(&self, col: c_int) -> bool {
        unsafe { ffi::wxListCtrl_DeleteColumn(self.as_ptr(), col) }
    }
    fn delete_item(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_DeleteItem(self.as_ptr(), item) }
    }
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
    fn enable_alternate_row_colours(&self, enable: bool) {
        unsafe { ffi::wxListCtrl_EnableAlternateRowColours(self.as_ptr(), enable) }
    }
    fn enable_bell_on_no_match(&self, on: bool) {
        unsafe { ffi::wxListCtrl_EnableBellOnNoMatch(self.as_ptr(), on) }
    }
    fn end_edit_label(&self, cancel: bool) -> bool {
        unsafe { ffi::wxListCtrl_EndEditLabel(self.as_ptr(), cancel) }
    }
    fn ensure_visible(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_EnsureVisible(self.as_ptr(), item) }
    }
    fn find_item_str(&self, start: c_long, str: &str, partial: bool) -> c_long {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxListCtrl_FindItem(self.as_ptr(), start, str, partial)
        }
    }
    // NOT_SUPPORTED: fn FindItem1()
    fn find_item_point<P: PointMethods>(&self, start: c_long, pt: &P, direction: c_int) -> c_long {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxListCtrl_FindItem2(self.as_ptr(), start, pt, direction)
        }
    }
    fn get_column<L: ListItemMethods>(&self, col: c_int, item: &L) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListCtrl_GetColumn(self.as_ptr(), col, item)
        }
    }
    fn get_column_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn get_column_index_from_order(&self, pos: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnIndexFromOrder(self.as_ptr(), pos) }
    }
    fn get_column_order(&self, col: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnOrder(self.as_ptr(), col) }
    }
    fn get_column_width(&self, col: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnWidth(self.as_ptr(), col) }
    }
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxListCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetCountPerPage(self.as_ptr()) }
    }
    fn get_edit_control(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxListCtrl_GetEditControl(self.as_ptr())) }
    }
    fn get_image_list(&self, which: c_int) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxListCtrl_GetImageList(self.as_ptr(), which)) }
    }
    fn get_item<L: ListItemMethods>(&self, info: &L) -> bool {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_GetItem(self.as_ptr(), info)
        }
    }
    fn get_item_background_colour(&self, item: c_long) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetItemBackgroundColour(self.as_ptr(), item)) }
    }
    fn get_item_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    fn get_item_font(&self, item: c_long) -> Font {
        unsafe { Font::from_ptr(ffi::wxListCtrl_GetItemFont(self.as_ptr(), item)) }
    }
    fn get_item_position<P: PointMethods>(&self, item: c_long, pos: &P) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxListCtrl_GetItemPosition(self.as_ptr(), item, pos)
        }
    }
    fn get_item_rect<R: RectMethods>(&self, item: c_long, rect: &R, code: c_int) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxListCtrl_GetItemRect(self.as_ptr(), item, rect, code)
        }
    }
    fn get_item_spacing(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxListCtrl_GetItemSpacing(self.as_ptr())) }
    }
    fn get_item_state(&self, item: c_long, state_mask: c_long) -> c_int {
        unsafe { ffi::wxListCtrl_GetItemState(self.as_ptr(), item, state_mask) }
    }
    fn get_item_text(&self, item: c_long, col: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxListCtrl_GetItemText(self.as_ptr(), item, col)).into() }
    }
    fn get_item_text_colour(&self, item: c_long) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetItemTextColour(self.as_ptr(), item)) }
    }
    fn get_next_item(&self, item: c_long, geometry: c_int, state: c_int) -> c_long {
        unsafe { ffi::wxListCtrl_GetNextItem(self.as_ptr(), item, geometry, state) }
    }
    fn get_selected_item_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetSelectedItemCount(self.as_ptr()) }
    }
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
    fn get_text_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetTextColour(self.as_ptr())) }
    }
    fn get_top_item(&self) -> c_long {
        unsafe { ffi::wxListCtrl_GetTopItem(self.as_ptr()) }
    }
    fn get_view_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxListCtrl_GetViewRect(self.as_ptr())) }
    }
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxListCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    fn get_alternate_row_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetAlternateRowColour(self.as_ptr())) }
    }
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
    fn in_report_view(&self) -> bool {
        unsafe { ffi::wxListCtrl_InReportView(self.as_ptr()) }
    }
    fn insert_column_listitem<L: ListItemMethods>(&self, col: c_long, info: &L) -> c_long {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_InsertColumn(self.as_ptr(), col, info)
        }
    }
    fn insert_column_str(&self, col: c_long, heading: &str, format: c_int, width: c_int) -> c_long {
        unsafe {
            let heading = WxString::from(heading);
            let heading = heading.as_ptr();
            ffi::wxListCtrl_InsertColumn1(self.as_ptr(), col, heading, format, width)
        }
    }
    fn insert_item_listitem<L: ListItemMethods>(&self, info: &L) -> c_long {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_InsertItem(self.as_ptr(), info)
        }
    }
    fn insert_item_long_str(&self, index: c_long, label: &str) -> c_long {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_InsertItem1(self.as_ptr(), index, label)
        }
    }
    fn insert_item_long_int(&self, index: c_long, image_index: c_int) -> c_long {
        unsafe { ffi::wxListCtrl_InsertItem2(self.as_ptr(), index, image_index) }
    }
    fn insert_item_long_str_int(&self, index: c_long, label: &str, image_index: c_int) -> c_long {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_InsertItem3(self.as_ptr(), index, label, image_index)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsEmpty(self.as_ptr()) }
    }
    fn is_virtual(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsVirtual(self.as_ptr()) }
    }
    fn refresh_item(&self, item: c_long) {
        unsafe { ffi::wxListCtrl_RefreshItem(self.as_ptr(), item) }
    }
    fn refresh_items(&self, item_from: c_long, item_to: c_long) {
        unsafe { ffi::wxListCtrl_RefreshItems(self.as_ptr(), item_from, item_to) }
    }
    fn scroll_list(&self, dx: c_int, dy: c_int) -> bool {
        unsafe { ffi::wxListCtrl_ScrollList(self.as_ptr(), dx, dy) }
    }
    fn set_column<L: ListItemMethods>(&self, col: c_int, item: &L) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListCtrl_SetColumn(self.as_ptr(), col, item)
        }
    }
    fn set_column_width(&self, col: c_int, width: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetColumnWidth(self.as_ptr(), col, width) }
    }
    fn set_columns_order<A: ArrayIntMethods>(&self, orders: &A) -> bool {
        unsafe {
            let orders = orders.as_ptr();
            ffi::wxListCtrl_SetColumnsOrder(self.as_ptr(), orders)
        }
    }
    fn set_header_attr(&self, attr: *const c_void) -> bool {
        unsafe { ffi::wxListCtrl_SetHeaderAttr(self.as_ptr(), attr) }
    }
    fn set_image_list<I: ImageListMethods>(&self, image_list: Option<&I>, which: c_int) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxListCtrl_SetImageList(self.as_ptr(), image_list, which)
        }
    }
    fn set_normal_images(&self, images: *const c_void) {
        unsafe { ffi::wxListCtrl_SetNormalImages(self.as_ptr(), images) }
    }
    fn set_small_images(&self, images: *const c_void) {
        unsafe { ffi::wxListCtrl_SetSmallImages(self.as_ptr(), images) }
    }
    fn is_visible(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_IsVisible(self.as_ptr(), item) }
    }
    fn set_item_listitem<L: ListItemMethods>(&self, info: &L) -> bool {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_SetItem(self.as_ptr(), info)
        }
    }
    fn set_item_long(&self, index: c_long, column: c_int, label: &str, image_id: c_int) -> bool {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_SetItem1(self.as_ptr(), index, column, label, image_id)
        }
    }
    fn set_item_background_colour<C: ColourMethods>(&self, item: c_long, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetItemBackgroundColour(self.as_ptr(), item, col)
        }
    }
    fn set_item_column_image(&self, item: c_long, column: c_long, image: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetItemColumnImage(self.as_ptr(), item, column, image) }
    }
    fn set_item_count(&self, count: c_long) {
        unsafe { ffi::wxListCtrl_SetItemCount(self.as_ptr(), count) }
    }
    fn set_item_data(&self, item: c_long, data: c_long) -> bool {
        unsafe { ffi::wxListCtrl_SetItemData(self.as_ptr(), item, data) }
    }
    fn set_item_font<F: FontMethods>(&self, item: c_long, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxListCtrl_SetItemFont(self.as_ptr(), item, font)
        }
    }
    fn set_item_image(&self, item: c_long, image: c_int, sel_image: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetItemImage(self.as_ptr(), item, image, sel_image) }
    }
    fn set_item_position<P: PointMethods>(&self, item: c_long, pos: &P) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxListCtrl_SetItemPosition(self.as_ptr(), item, pos)
        }
    }
    // NOT_SUPPORTED: fn SetItemPtrData()
    fn set_item_state(&self, item: c_long, state: c_long, state_mask: c_long) -> bool {
        unsafe { ffi::wxListCtrl_SetItemState(self.as_ptr(), item, state, state_mask) }
    }
    fn set_item_text(&self, item: c_long, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxListCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
    fn set_item_text_colour<C: ColourMethods>(&self, item: c_long, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetItemTextColour(self.as_ptr(), item, col)
        }
    }
    fn set_single_style(&self, style: c_long, add: bool) {
        unsafe { ffi::wxListCtrl_SetSingleStyle(self.as_ptr(), style, add) }
    }
    fn set_text_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetTextColour(self.as_ptr(), col)
        }
    }
    // NOT_SUPPORTED: fn SortItems()
    fn has_check_boxes(&self) -> bool {
        unsafe { ffi::wxListCtrl_HasCheckBoxes(self.as_ptr()) }
    }
    fn enable_check_boxes(&self, enable: bool) -> bool {
        unsafe { ffi::wxListCtrl_EnableCheckBoxes(self.as_ptr(), enable) }
    }
    fn is_item_checked(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_IsItemChecked(self.as_ptr(), item) }
    }
    fn check_item(&self, item: c_long, check: bool) {
        unsafe { ffi::wxListCtrl_CheckItem(self.as_ptr(), item, check) }
    }
    fn extend_rules_and_alternate_colour(&self, extend: bool) {
        unsafe { ffi::wxListCtrl_ExtendRulesAndAlternateColour(self.as_ptr(), extend) }
    }
    fn show_sort_indicator(&self, col: c_int, ascending: bool) {
        unsafe { ffi::wxListCtrl_ShowSortIndicator(self.as_ptr(), col, ascending) }
    }
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxListCtrl_RemoveSortIndicator(self.as_ptr()) }
    }
    fn get_sort_indicator(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetSortIndicator(self.as_ptr()) }
    }
    fn get_updated_ascending_sort_indicator(&self, col: c_int) -> bool {
        unsafe { ffi::wxListCtrl_GetUpdatedAscendingSortIndicator(self.as_ptr(), col) }
    }
    fn is_ascending_sort_indicator(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsAscendingSortIndicator(self.as_ptr()) }
    }
}

// wxListEvent
pub trait ListEventMethods: NotifyEventMethods {
    fn get_cache_from(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetCacheFrom(self.as_ptr()) }
    }
    fn get_cache_to(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetCacheTo(self.as_ptr()) }
    }
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetColumn(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetData()
    fn get_image(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetImage(self.as_ptr()) }
    }
    fn get_index(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetIndex(self.as_ptr()) }
    }
    fn get_item(&self) -> ListItemIsOwned<false> {
        unsafe { ListItemIsOwned::from_ptr(ffi::wxListEvent_GetItem(self.as_ptr())) }
    }
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetKeyCode(self.as_ptr()) }
    }
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListEvent_GetLabel(self.as_ptr())).into() }
    }
    fn get_mask(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetMask(self.as_ptr()) }
    }
    fn get_point(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxListEvent_GetPoint(self.as_ptr())) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListEvent_GetText(self.as_ptr())).into() }
    }
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxListEvent_IsEditCancelled(self.as_ptr()) }
    }
    fn set_key_code(&self, code: c_int) {
        unsafe { ffi::wxListEvent_SetKeyCode(self.as_ptr(), code) }
    }
    fn set_index(&self, index: c_long) {
        unsafe { ffi::wxListEvent_SetIndex(self.as_ptr(), index) }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxListEvent_SetColumn(self.as_ptr(), col) }
    }
    fn set_point<P: PointMethods>(&self, point: &P) {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListEvent_SetPoint(self.as_ptr(), point)
        }
    }
    fn set_item<L: ListItemMethods>(&self, item: &L) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListEvent_SetItem(self.as_ptr(), item)
        }
    }
    fn set_cache_from(&self, cache_from: c_long) {
        unsafe { ffi::wxListEvent_SetCacheFrom(self.as_ptr(), cache_from) }
    }
    fn set_cache_to(&self, cache_to: c_long) {
        unsafe { ffi::wxListEvent_SetCacheTo(self.as_ptr(), cache_to) }
    }
}

// wxListItem
pub trait ListItemMethods: ObjectMethods {
    fn clear(&self) {
        unsafe { ffi::wxListItem_Clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAlign()
    fn get_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListItem_GetBackgroundColour(self.as_ptr())) }
    }
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxListItem_GetColumn(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetData()
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxListItem_GetFont(self.as_ptr())) }
    }
    fn get_id(&self) -> c_long {
        unsafe { ffi::wxListItem_GetId(self.as_ptr()) }
    }
    fn get_image(&self) -> c_int {
        unsafe { ffi::wxListItem_GetImage(self.as_ptr()) }
    }
    fn get_mask(&self) -> c_long {
        unsafe { ffi::wxListItem_GetMask(self.as_ptr()) }
    }
    fn get_state(&self) -> c_long {
        unsafe { ffi::wxListItem_GetState(self.as_ptr()) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListItem_GetText(self.as_ptr())).into() }
    }
    fn get_text_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListItem_GetTextColour(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxListItem_GetWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAlign()
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxListItem_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxListItem_SetColumn(self.as_ptr(), col) }
    }
    fn set_data_long(&self, data: c_long) {
        unsafe { ffi::wxListItem_SetData(self.as_ptr(), data) }
    }
    fn set_data_void(&self, data: *mut c_void) {
        unsafe { ffi::wxListItem_SetData1(self.as_ptr(), data) }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxListItem_SetFont(self.as_ptr(), font)
        }
    }
    fn set_id(&self, id: c_long) {
        unsafe { ffi::wxListItem_SetId(self.as_ptr(), id) }
    }
    fn set_image(&self, image: c_int) {
        unsafe { ffi::wxListItem_SetImage(self.as_ptr(), image) }
    }
    fn set_mask(&self, mask: c_long) {
        unsafe { ffi::wxListItem_SetMask(self.as_ptr(), mask) }
    }
    fn set_state(&self, state: c_long) {
        unsafe { ffi::wxListItem_SetState(self.as_ptr(), state) }
    }
    fn set_state_mask(&self, state_mask: c_long) {
        unsafe { ffi::wxListItem_SetStateMask(self.as_ptr(), state_mask) }
    }
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxListItem_SetText(self.as_ptr(), text)
        }
    }
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxListItem_SetTextColour(self.as_ptr(), col_text)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxListItem_SetWidth(self.as_ptr(), width) }
    }
}

// wxListView
pub trait ListViewMethods: ListCtrlMethods {
    // DTOR: fn ~wxListView()
    fn clear_column_image(&self, col: c_int) {
        unsafe { ffi::wxListView_ClearColumnImage(self.as_ptr(), col) }
    }
    fn focus(&self, index: c_long) {
        unsafe { ffi::wxListView_Focus(self.as_ptr(), index) }
    }
    fn get_first_selected(&self) -> c_long {
        unsafe { ffi::wxListView_GetFirstSelected(self.as_ptr()) }
    }
    fn get_focused_item(&self) -> c_long {
        unsafe { ffi::wxListView_GetFocusedItem(self.as_ptr()) }
    }
    fn get_next_selected(&self, item: c_long) -> c_long {
        unsafe { ffi::wxListView_GetNextSelected(self.as_ptr(), item) }
    }
    fn is_selected(&self, index: c_long) -> bool {
        unsafe { ffi::wxListView_IsSelected(self.as_ptr(), index) }
    }
    fn select(&self, n: c_long, on: bool) {
        unsafe { ffi::wxListView_Select(self.as_ptr(), n, on) }
    }
    fn set_column_image(&self, col: c_int, image: c_int) {
        unsafe { ffi::wxListView_SetColumnImage(self.as_ptr(), col, image) }
    }
}

// wxListbook
pub trait ListbookMethods: BookCtrlBaseMethods {
    fn get_list_view(&self) -> WeakRef<ListView> {
        unsafe { WeakRef::<ListView>::from(ffi::wxListbook_GetListView(self.as_ptr())) }
    }
}
