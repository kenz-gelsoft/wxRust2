use super::*;

// wxHScrolledWindow
/// This trait represents [C++ `wxHScrolledWindow` class](https://docs.wxwidgets.org/3.2/classwx_h_scrolled_window.html)'s methods and inheritance.
///
/// See [`HScrolledWindowFromCpp`] documentation for the class usage.
pub trait HScrolledWindowMethods: PanelMethods {}

// wxHTMLDataObject
/// This trait represents [C++ `wxHTMLDataObject` class](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html)'s methods and inheritance.
///
/// See [`HTMLDataObjectFromCpp`] documentation for the class usage.
pub trait HTMLDataObjectMethods: DataObjectSimpleMethods {
    /// Returns the HTML string.
    ///
    /// See [C++ `wxHTMLDataObject::GetHTML()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html#a40610295fed2754d98a2d753b1f4527c).
    fn get_html(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHTMLDataObject_GetHTML(self.as_ptr())).into() }
    }
    /// Sets the HTML string.
    ///
    /// See [C++ `wxHTMLDataObject::SetHTML()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html#aaa23c38c3e4af0f62c78fb6e76d41be3).
    fn set_html(&self, html: &str) {
        unsafe {
            let html = WxString::from(html);
            let html = html.as_ptr();
            ffi::wxHTMLDataObject_SetHTML(self.as_ptr(), html)
        }
    }
}

// wxHVScrolledWindow
/// This trait represents [C++ `wxHVScrolledWindow` class](https://docs.wxwidgets.org/3.2/classwx_h_v_scrolled_window.html)'s methods and inheritance.
///
/// See [`HVScrolledWindowFromCpp`] documentation for the class usage.
pub trait HVScrolledWindowMethods: PanelMethods {}

// wxHeaderColumn
/// This trait represents [C++ `wxHeaderColumn` class](https://docs.wxwidgets.org/3.2/classwx_header_column.html)'s methods and inheritance.
///
/// See [`HeaderColumnFromCpp`] documentation for the class usage.
pub trait HeaderColumnMethods: WxRustMethods {
    /// Get the text shown in the column header.
    ///
    /// See [C++ `wxHeaderColumn::GetTitle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a47d57c9babac44df16e1f5581156a3ce).
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHeaderColumn_GetTitle(self.as_ptr())).into() }
    }
    /// This function exists only for backwards compatibility, it's recommended to override GetBitmapBundle() in the new code and override this one to do nothing, as it will never be called if GetBitmapBundle() is overridden.
    ///
    /// See [C++ `wxHeaderColumn::GetBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#aca4d183177667dd1f6f20a61127109af).
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxHeaderColumn_GetBitmap(self.as_ptr())) }
    }
    /// Returns the bitmap in the header of the column, if any.
    ///
    /// See [C++ `wxHeaderColumn::GetBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#aa44eca61f73dec37bf1434041f8dccd8).
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxHeaderColumn_GetBitmapBundle(self.as_ptr())) }
    }
    /// Returns the current width of the column.
    ///
    /// See [C++ `wxHeaderColumn::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a992a31984563a35b0222adf5a06b0a77).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetWidth(self.as_ptr()) }
    }
    /// Return the minimal column width.
    ///
    /// See [C++ `wxHeaderColumn::GetMinWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a8a7c17e7f634570246b734cf2eb63fbb).
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetMinWidth(self.as_ptr()) }
    }
    /// Returns the current column alignment.
    ///
    /// See [C++ `wxHeaderColumn::GetAlignment()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#afc7728e0def849ecb6a2b074f2ea0eb8).
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetAlignment(self.as_ptr()) }
    }
    /// Get the column flags.
    ///
    /// See [C++ `wxHeaderColumn::GetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a5026f8cf3dde67ab2003da8ec8a892ab).
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetFlags(self.as_ptr()) }
    }
    /// Return true if the specified flag is currently set for this column.
    ///
    /// See [C++ `wxHeaderColumn::HasFlag()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#ad9ca1aaba092ae89b0e4efcacc243b12).
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxHeaderColumn_HasFlag(self.as_ptr(), flag) }
    }
    /// Return true if the column can be resized by the user.
    ///
    /// See [C++ `wxHeaderColumn::IsResizeable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a5d5bade6f1ade844355fe2a4ee680471).
    fn is_resizeable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsResizeable(self.as_ptr()) }
    }
    /// Returns true if the column can be clicked by user to sort the control contents by the field in this column.
    ///
    /// See [C++ `wxHeaderColumn::IsSortable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a48e7b0b4c63cfc778b9fc87f0074fb62).
    fn is_sortable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortable(self.as_ptr()) }
    }
    /// Returns true if the column can be dragged by user to change its order.
    ///
    /// See [C++ `wxHeaderColumn::IsReorderable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a40f94597100d0d1852b61124d32ab871).
    fn is_reorderable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsReorderable(self.as_ptr()) }
    }
    /// Returns true if the column is currently hidden.
    ///
    /// See [C++ `wxHeaderColumn::IsHidden()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a010ab3ccff0b3c58a1e19b93b0eb09ac).
    fn is_hidden(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsHidden(self.as_ptr()) }
    }
    /// Returns true if the column is currently shown.
    ///
    /// See [C++ `wxHeaderColumn::IsShown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a14d520b2b990069ea2df289cc346ba44).
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsShown(self.as_ptr()) }
    }
    /// Returns true if the column is currently used for sorting.
    ///
    /// See [C++ `wxHeaderColumn::IsSortKey()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a95645523fc113008d66d626bc06c7dd1).
    fn is_sort_key(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortKey(self.as_ptr()) }
    }
    /// Returns true, if the sort order is ascending.
    ///
    /// See [C++ `wxHeaderColumn::IsSortOrderAscending()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html#a30e41f28c017812c4f53e4b66c62cd4c).
    fn is_sort_order_ascending(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortOrderAscending(self.as_ptr()) }
    }
}

// wxHeaderColumnSimple
/// This trait represents [C++ `wxHeaderColumnSimple` class](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html)'s methods and inheritance.
///
/// See [`HeaderColumnSimpleFromCpp`] documentation for the class usage.
pub trait HeaderColumnSimpleMethods: SettableHeaderColumnMethods {}

// wxHeaderCtrl
/// This trait represents [C++ `wxHeaderCtrl` class](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html)'s methods and inheritance.
///
/// See [`HeaderCtrlFromCpp`] documentation for the class usage.
pub trait HeaderCtrlMethods: ControlMethods {
    /// Set the number of columns in the control.
    ///
    /// See [C++ `wxHeaderCtrl::SetColumnCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a41ff1c0ad9b13a79b17e61555e81eb25).
    fn set_column_count(&self, count: c_uint) {
        unsafe { ffi::wxHeaderCtrl_SetColumnCount(self.as_ptr(), count) }
    }
    /// Return the number of columns in the control.
    ///
    /// See [C++ `wxHeaderCtrl::GetColumnCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a42690da34b54c1ae7a575c3924297dd1).
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnCount(self.as_ptr()) }
    }
    /// Return whether the control has any columns.
    ///
    /// See [C++ `wxHeaderCtrl::IsEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a12706f2e9263701a189b776e2877458d).
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_IsEmpty(self.as_ptr()) }
    }
    /// Update the column with the given index.
    ///
    /// See [C++ `wxHeaderCtrl::UpdateColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a39d92436446d06da3ed011e43fd4d249).
    fn update_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrl_UpdateColumn(self.as_ptr(), idx) }
    }
    /// Change the columns display order.
    ///
    /// See [C++ `wxHeaderCtrl::SetColumnsOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a561d03dc7ce13d2deeff41b0df0b8990).
    fn set_columns_order<A: ArrayIntMethods>(&self, order: &A) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_SetColumnsOrder(self.as_ptr(), order)
        }
    }
    /// Return the array describing the columns display order.
    ///
    /// See [C++ `wxHeaderCtrl::GetColumnsOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a9cc12b69fb41bd8bf7c3b106078a8915).
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxHeaderCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    /// Return the index of the column displayed at the given position.
    ///
    /// See [C++ `wxHeaderCtrl::GetColumnAt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#ab86cd1ea2dafcafc38a23b3a182f0953).
    fn get_column_at(&self, pos: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnAt(self.as_ptr(), pos) }
    }
    /// Get the position at which this column is currently displayed.
    ///
    /// See [C++ `wxHeaderCtrl::GetColumnPos()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#af3cadca8793de042fe71666f8c3a8c66).
    fn get_column_pos(&self, idx: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnPos(self.as_ptr(), idx) }
    }
    /// Reset the columns order to the natural one.
    ///
    /// See [C++ `wxHeaderCtrl::ResetColumnsOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#aec79d0137d1c577741bb883dbb897f0e).
    fn reset_columns_order(&self) {
        unsafe { ffi::wxHeaderCtrl_ResetColumnsOrder(self.as_ptr()) }
    }
    /// Show the popup menu allowing the user to show or hide the columns.
    ///
    /// See [C++ `wxHeaderCtrl::ShowColumnsMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a1662ff7b43136b1ae710d11cbe9e3295).
    fn show_columns_menu<P: PointMethods>(&self, pt: &P, title: &str) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxHeaderCtrl_ShowColumnsMenu(self.as_ptr(), pt, title)
        }
    }
    /// Helper function appending the checkable items corresponding to all the columns to the given menu.
    ///
    /// See [C++ `wxHeaderCtrl::AddColumnsItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a22396560f002df58ecc7c6ed62feaf6e).
    fn add_columns_items<M: MenuMethods>(&self, menu: &M, id_columns_base: c_int) {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxHeaderCtrl_AddColumnsItems(self.as_ptr(), menu, id_columns_base)
        }
    }
    /// Show the column customization dialog.
    ///
    /// See [C++ `wxHeaderCtrl::ShowCustomizeDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a47f99bb2cfe60decf996f43138c1b32f).
    fn show_customize_dialog(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_ShowCustomizeDialog(self.as_ptr()) }
    }
    /// Returns width needed for given column's title.
    ///
    /// See [C++ `wxHeaderCtrl::GetColumnTitleWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a317a54720263d266356c685c4c024869).
    fn get_column_title_width_headercolumn<H: HeaderColumnMethods>(&self, col: &H) -> c_int {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrl_GetColumnTitleWidth(self.as_ptr(), col)
        }
    }
    /// Returns width needed for the column with the given index.
    ///
    /// See [C++ `wxHeaderCtrl::GetColumnTitleWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a67efc79baa684de193d725b35db91956).
    fn get_column_title_width_uint(&self, idx: c_uint) -> c_int {
        unsafe { ffi::wxHeaderCtrl_GetColumnTitleWidth1(self.as_ptr(), idx) }
    }
    /// Helper function to manipulate the array of column indices.
    ///
    /// See [C++ `wxHeaderCtrl::MoveColumnInOrderArray()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a8046ddebbcdf8a6d82f1da144cf868ea).
    fn move_column_in_order_array<A: ArrayIntMethods>(order: &A, idx: c_uint, pos: c_uint) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_MoveColumnInOrderArray(order, idx, pos)
        }
    }
}

// wxHeaderCtrlEvent
/// This trait represents [C++ `wxHeaderCtrlEvent` class](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html)'s methods and inheritance.
///
/// See [`HeaderCtrlEventFromCpp`] documentation for the class usage.
pub trait HeaderCtrlEventMethods: NotifyEventMethods {
    /// Return the index of the column affected by this event.
    ///
    /// See [C++ `wxHeaderCtrlEvent::GetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#af7eff8f64cb52cf69d32c7e84eba76b2).
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetColumn(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxHeaderCtrlEvent::SetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#a8fb7846e9799d28468bd14a7c505f40d).
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetColumn(self.as_ptr(), col) }
    }
    /// Return the current width of the column.
    ///
    /// See [C++ `wxHeaderCtrlEvent::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#a7b31e1098c094499703be53f8f5ef55d).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetWidth(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxHeaderCtrlEvent::SetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#a71bcd3bfaf4d89e586b5bc5bde1820ae).
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetWidth(self.as_ptr(), width) }
    }
    /// Return the new order of the column.
    ///
    /// See [C++ `wxHeaderCtrlEvent::GetNewOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#ab34f345e08ce51a50f914724369cfe70).
    fn get_new_order(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrlEvent_GetNewOrder(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxHeaderCtrlEvent::SetNewOrder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#aeac162a3d7df259e8e34a1c28d5f73ca).
    fn set_new_order(&self, order: c_uint) {
        unsafe { ffi::wxHeaderCtrlEvent_SetNewOrder(self.as_ptr(), order) }
    }
}

// wxHeaderCtrlSimple
/// This trait represents [C++ `wxHeaderCtrlSimple` class](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html)'s methods and inheritance.
///
/// See [`HeaderCtrlSimpleFromCpp`] documentation for the class usage.
pub trait HeaderCtrlSimpleMethods: HeaderCtrlMethods {
    /// Insert the column at the given position.
    ///
    /// See [C++ `wxHeaderCtrlSimple::InsertColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a2da87a48d6020d62649d660a53778098).
    fn insert_column<H: HeaderColumnSimpleMethods>(&self, col: &H, idx: c_uint) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_InsertColumn(self.as_ptr(), col, idx)
        }
    }
    /// Append the column to the end of the control.
    ///
    /// See [C++ `wxHeaderCtrlSimple::AppendColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a953d8f6fcc305cea02182a10eb47cc08).
    fn append_column<H: HeaderColumnSimpleMethods>(&self, col: &H) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_AppendColumn(self.as_ptr(), col)
        }
    }
    /// Delete the column at the given position.
    ///
    /// See [C++ `wxHeaderCtrlSimple::DeleteColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a6b3ca586ed2da4ac964bd57ef0753509).
    fn delete_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_DeleteColumn(self.as_ptr(), idx) }
    }
    /// Show or hide the column.
    ///
    /// See [C++ `wxHeaderCtrlSimple::ShowColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#aca35f2f868e3b5116f132ea5a7ef3bd7).
    fn show_column(&self, idx: c_uint, show: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowColumn(self.as_ptr(), idx, show) }
    }
    /// Hide the column with the given index.
    ///
    /// See [C++ `wxHeaderCtrlSimple::HideColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a39735d2c9a2ab8ba709a142757c47287).
    fn hide_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_HideColumn(self.as_ptr(), idx) }
    }
    /// Update the column sort indicator.
    ///
    /// See [C++ `wxHeaderCtrlSimple::ShowSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a18c357a62620cf6171ab4a8fa281eee2).
    fn show_sort_indicator(&self, idx: c_uint, sort_order: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowSortIndicator(self.as_ptr(), idx, sort_order) }
    }
    /// Remove the sort indicator from the column being used as sort key.
    ///
    /// See [C++ `wxHeaderCtrlSimple::RemoveSortIndicator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a3ad67fca36b463042d91a7d4b0e265ef).
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxHeaderCtrlSimple_RemoveSortIndicator(self.as_ptr()) }
    }
}

// wxHelpEvent
/// This trait represents [C++ `wxHelpEvent` class](https://docs.wxwidgets.org/3.2/classwx_help_event.html)'s methods and inheritance.
///
/// See [`HelpEventFromCpp`] documentation for the class usage.
pub trait HelpEventMethods: CommandEventMethods {
    // NOT_SUPPORTED: fn GetOrigin()
    /// Returns the left-click position of the mouse, in screen coordinates.
    ///
    /// See [C++ `wxHelpEvent::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_help_event.html#a28bd5a0701bb9d0537dced60743a2292).
    fn get_position(&self) -> PointFromCpp<true> {
        unsafe { PointFromCpp::from_ptr(ffi::wxHelpEvent_GetPosition(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetOrigin()
    /// Sets the left-click position of the mouse, in screen coordinates.
    ///
    /// See [C++ `wxHelpEvent::SetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_help_event.html#aceef2981ed1c48fa1ba55cff9c185e81).
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxHelpEvent_SetPosition(self.as_ptr(), pt)
        }
    }
}

// wxHyperlinkCtrl
/// This trait represents [C++ `wxHyperlinkCtrl` class](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html)'s methods and inheritance.
///
/// See [`HyperlinkCtrlFromCpp`] documentation for the class usage.
pub trait HyperlinkCtrlMethods: ControlMethods {
    /// Creates the hyperlink control.
    ///
    /// See [C++ `wxHyperlinkCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#aa8aa95908e1624daffd352bb404a1113).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHyperlinkCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                url,
                pos,
                size,
                style,
                name,
            )
        }
    }
    /// Returns the colour used to print the label of the hyperlink when the mouse is over the control.
    ///
    /// See [C++ `wxHyperlinkCtrl::GetHoverColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#af61ed9ee14d41ea8c28af63da7cdd9cb).
    fn get_hover_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetHoverColour(self.as_ptr())) }
    }
    /// Returns the colour used to print the label when the link has never been clicked before (i.e. the link has not been visited) and the mouse is not over the control.
    ///
    /// See [C++ `wxHyperlinkCtrl::GetNormalColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#acf74c7d92a32d32efb84a35698033a90).
    fn get_normal_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetNormalColour(self.as_ptr())) }
    }
    /// Returns the URL associated with the hyperlink.
    ///
    /// See [C++ `wxHyperlinkCtrl::GetURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a35d423be99fe82de8f12c97692d58ade).
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkCtrl_GetURL(self.as_ptr())).into() }
    }
    /// Returns true if the hyperlink has already been clicked by the user at least one time.
    ///
    /// See [C++ `wxHyperlinkCtrl::GetVisited()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a61cc41599177ba3e05c41d7a6b1173e1).
    fn get_visited(&self) -> bool {
        unsafe { ffi::wxHyperlinkCtrl_GetVisited(self.as_ptr()) }
    }
    /// Returns the colour used to print the label when the mouse is not over the control and the link has already been clicked before (i.e. the link has been visited).
    ///
    /// See [C++ `wxHyperlinkCtrl::GetVisitedColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a1e81064f74b98b5814e56c1f1d7c0f49).
    fn get_visited_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetVisitedColour(self.as_ptr())) }
    }
    /// Sets the colour used to print the label of the hyperlink when the mouse is over the control.
    ///
    /// See [C++ `wxHyperlinkCtrl::SetHoverColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a0b96709dc5b7874b4f7ecf5301bc553b).
    fn set_hover_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetHoverColour(self.as_ptr(), colour)
        }
    }
    /// Sets the colour used to print the label when the link has never been clicked before (i.e. the link has not been visited) and the mouse is not over the control.
    ///
    /// See [C++ `wxHyperlinkCtrl::SetNormalColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a6081b5336124b2851bf1d33dd48b97f3).
    fn set_normal_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetNormalColour(self.as_ptr(), colour)
        }
    }
    /// Sets the URL associated with the hyperlink.
    ///
    /// See [C++ `wxHyperlinkCtrl::SetURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a1cdaf835c456f8395d1574d5d73ec988).
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkCtrl_SetURL(self.as_ptr(), url)
        }
    }
    /// Marks the hyperlink as visited (see wxHyperlinkCtrl::SetVisitedColour).
    ///
    /// See [C++ `wxHyperlinkCtrl::SetVisited()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a6d2b3cefefa9047fd1bebe8c677a1f12).
    fn set_visited(&self, visited: bool) {
        unsafe { ffi::wxHyperlinkCtrl_SetVisited(self.as_ptr(), visited) }
    }
    /// Sets the colour used to print the label when the mouse is not over the control and the link has already been clicked before (i.e. the link has been visited).
    ///
    /// See [C++ `wxHyperlinkCtrl::SetVisitedColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#add059ac942622177ae7870d341a11054).
    fn set_visited_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetVisitedColour(self.as_ptr(), colour)
        }
    }
}

// wxHyperlinkEvent
/// This trait represents [C++ `wxHyperlinkEvent` class](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html)'s methods and inheritance.
///
/// See [`HyperlinkEventFromCpp`] documentation for the class usage.
pub trait HyperlinkEventMethods: CommandEventMethods {
    /// Returns the URL of the hyperlink where the user has just clicked.
    ///
    /// See [C++ `wxHyperlinkEvent::GetURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html#a8179561f6e053ba8dffb5e689dc45d8b).
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkEvent_GetURL(self.as_ptr())).into() }
    }
    /// Sets the URL associated with the event.
    ///
    /// See [C++ `wxHyperlinkEvent::SetURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html#a49a363a38d5fede85e6e3b17694694ce).
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkEvent_SetURL(self.as_ptr(), url)
        }
    }
}
