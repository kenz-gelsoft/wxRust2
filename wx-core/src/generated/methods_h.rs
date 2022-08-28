use super::*;

// wxHScrolledWindow
pub trait HScrolledWindowMethods: PanelMethods {}

// wxHTMLDataObject
pub trait HTMLDataObjectMethods: DataObjectSimpleMethods {
    /// Returns the HTML string.
    fn get_html(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHTMLDataObject_GetHTML(self.as_ptr())).into() }
    }
    /// Sets the HTML string.
    fn set_html(&self, html: &str) {
        unsafe {
            let html = WxString::from(html);
            let html = html.as_ptr();
            ffi::wxHTMLDataObject_SetHTML(self.as_ptr(), html)
        }
    }
}

// wxHVScrolledWindow
pub trait HVScrolledWindowMethods: PanelMethods {}

// wxHeaderColumn
pub trait HeaderColumnMethods: WxRustMethods {
    /// Get the text shown in the column header.
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHeaderColumn_GetTitle(self.as_ptr())).into() }
    }
    /// This function exists only for backwards compatibility, it's recommended to override GetBitmapBundle() in the new code and override this one to do nothing, as it will never be called if GetBitmapBundle() is overridden.
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxHeaderColumn_GetBitmap(self.as_ptr())) }
    }
    /// Returns the bitmap in the header of the column, if any.
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxHeaderColumn_GetBitmapBundle(self.as_ptr())) }
    }
    /// Returns the current width of the column.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetWidth(self.as_ptr()) }
    }
    /// Return the minimal column width.
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetMinWidth(self.as_ptr()) }
    }
    /// Returns the current column alignment.
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetAlignment(self.as_ptr()) }
    }
    /// Get the column flags.
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetFlags(self.as_ptr()) }
    }
    /// Return true if the specified flag is currently set for this column.
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxHeaderColumn_HasFlag(self.as_ptr(), flag) }
    }
    /// Return true if the column can be resized by the user.
    fn is_resizeable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsResizeable(self.as_ptr()) }
    }
    /// Returns true if the column can be clicked by user to sort the control contents by the field in this column.
    fn is_sortable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortable(self.as_ptr()) }
    }
    /// Returns true if the column can be dragged by user to change its order.
    fn is_reorderable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsReorderable(self.as_ptr()) }
    }
    /// Returns true if the column is currently hidden.
    fn is_hidden(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsHidden(self.as_ptr()) }
    }
    /// Returns true if the column is currently shown.
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsShown(self.as_ptr()) }
    }
    /// Returns true if the column is currently used for sorting.
    fn is_sort_key(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortKey(self.as_ptr()) }
    }
    /// Returns true, if the sort order is ascending.
    fn is_sort_order_ascending(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortOrderAscending(self.as_ptr()) }
    }
}

// wxHeaderColumnSimple
pub trait HeaderColumnSimpleMethods: SettableHeaderColumnMethods {}

// wxHeaderCtrl
pub trait HeaderCtrlMethods: ControlMethods {
    /// Set the number of columns in the control.
    fn set_column_count(&self, count: c_uint) {
        unsafe { ffi::wxHeaderCtrl_SetColumnCount(self.as_ptr(), count) }
    }
    /// Return the number of columns in the control.
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnCount(self.as_ptr()) }
    }
    /// Return whether the control has any columns.
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_IsEmpty(self.as_ptr()) }
    }
    /// Update the column with the given index.
    fn update_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrl_UpdateColumn(self.as_ptr(), idx) }
    }
    /// Change the columns display order.
    fn set_columns_order<A: ArrayIntMethods>(&self, order: &A) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_SetColumnsOrder(self.as_ptr(), order)
        }
    }
    /// Return the array describing the columns display order.
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxHeaderCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    /// Return the index of the column displayed at the given position.
    fn get_column_at(&self, pos: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnAt(self.as_ptr(), pos) }
    }
    /// Get the position at which this column is currently displayed.
    fn get_column_pos(&self, idx: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnPos(self.as_ptr(), idx) }
    }
    /// Reset the columns order to the natural one.
    fn reset_columns_order(&self) {
        unsafe { ffi::wxHeaderCtrl_ResetColumnsOrder(self.as_ptr()) }
    }
    /// Show the popup menu allowing the user to show or hide the columns.
    fn show_columns_menu<P: PointMethods>(&self, pt: &P, title: &str) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxHeaderCtrl_ShowColumnsMenu(self.as_ptr(), pt, title)
        }
    }
    /// Helper function appending the checkable items corresponding to all the columns to the given menu.
    fn add_columns_items<M: MenuMethods>(&self, menu: &M, id_columns_base: c_int) {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxHeaderCtrl_AddColumnsItems(self.as_ptr(), menu, id_columns_base)
        }
    }
    /// Show the column customization dialog.
    fn show_customize_dialog(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_ShowCustomizeDialog(self.as_ptr()) }
    }
    /// Returns width needed for given column's title.
    fn get_column_title_width_headercolumn<H: HeaderColumnMethods>(&self, col: &H) -> c_int {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrl_GetColumnTitleWidth(self.as_ptr(), col)
        }
    }
    /// Returns width needed for the column with the given index.
    fn get_column_title_width_uint(&self, idx: c_uint) -> c_int {
        unsafe { ffi::wxHeaderCtrl_GetColumnTitleWidth1(self.as_ptr(), idx) }
    }
    /// Helper function to manipulate the array of column indices.
    fn move_column_in_order_array<A: ArrayIntMethods>(order: &A, idx: c_uint, pos: c_uint) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_MoveColumnInOrderArray(order, idx, pos)
        }
    }
}

// wxHeaderCtrlEvent
pub trait HeaderCtrlEventMethods: NotifyEventMethods {
    /// Return the index of the column affected by this event.
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetColumn(self.as_ptr()) }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetColumn(self.as_ptr(), col) }
    }
    /// Return the current width of the column.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetWidth(self.as_ptr()) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetWidth(self.as_ptr(), width) }
    }
    /// Return the new order of the column.
    fn get_new_order(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrlEvent_GetNewOrder(self.as_ptr()) }
    }
    fn set_new_order(&self, order: c_uint) {
        unsafe { ffi::wxHeaderCtrlEvent_SetNewOrder(self.as_ptr(), order) }
    }
}

// wxHeaderCtrlSimple
pub trait HeaderCtrlSimpleMethods: HeaderCtrlMethods {
    /// Insert the column at the given position.
    fn insert_column<H: HeaderColumnSimpleMethods>(&self, col: &H, idx: c_uint) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_InsertColumn(self.as_ptr(), col, idx)
        }
    }
    /// Append the column to the end of the control.
    fn append_column<H: HeaderColumnSimpleMethods>(&self, col: &H) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_AppendColumn(self.as_ptr(), col)
        }
    }
    /// Delete the column at the given position.
    fn delete_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_DeleteColumn(self.as_ptr(), idx) }
    }
    /// Show or hide the column.
    fn show_column(&self, idx: c_uint, show: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowColumn(self.as_ptr(), idx, show) }
    }
    /// Hide the column with the given index.
    fn hide_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_HideColumn(self.as_ptr(), idx) }
    }
    /// Update the column sort indicator.
    fn show_sort_indicator(&self, idx: c_uint, sort_order: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowSortIndicator(self.as_ptr(), idx, sort_order) }
    }
    /// Remove the sort indicator from the column being used as sort key.
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxHeaderCtrlSimple_RemoveSortIndicator(self.as_ptr()) }
    }
}

// wxHelpEvent
pub trait HelpEventMethods: CommandEventMethods {
    // NOT_SUPPORTED: fn GetOrigin()
    /// Returns the left-click position of the mouse, in screen coordinates.
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxHelpEvent_GetPosition(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetOrigin()
    /// Sets the left-click position of the mouse, in screen coordinates.
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxHelpEvent_SetPosition(self.as_ptr(), pt)
        }
    }
}

// wxHyperlinkCtrl
pub trait HyperlinkCtrlMethods: ControlMethods {
    /// Creates the hyperlink control.
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
    fn get_hover_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetHoverColour(self.as_ptr())) }
    }
    /// Returns the colour used to print the label when the link has never been clicked before (i.e. the link has not been visited) and the mouse is not over the control.
    fn get_normal_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetNormalColour(self.as_ptr())) }
    }
    /// Returns the URL associated with the hyperlink.
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkCtrl_GetURL(self.as_ptr())).into() }
    }
    /// Returns true if the hyperlink has already been clicked by the user at least one time.
    fn get_visited(&self) -> bool {
        unsafe { ffi::wxHyperlinkCtrl_GetVisited(self.as_ptr()) }
    }
    /// Returns the colour used to print the label when the mouse is not over the control and the link has already been clicked before (i.e. the link has been visited).
    fn get_visited_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetVisitedColour(self.as_ptr())) }
    }
    /// Sets the colour used to print the label of the hyperlink when the mouse is over the control.
    fn set_hover_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetHoverColour(self.as_ptr(), colour)
        }
    }
    /// Sets the colour used to print the label when the link has never been clicked before (i.e. the link has not been visited) and the mouse is not over the control.
    fn set_normal_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetNormalColour(self.as_ptr(), colour)
        }
    }
    /// Sets the URL associated with the hyperlink.
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkCtrl_SetURL(self.as_ptr(), url)
        }
    }
    /// Marks the hyperlink as visited (see wxHyperlinkCtrl::SetVisitedColour).
    fn set_visited(&self, visited: bool) {
        unsafe { ffi::wxHyperlinkCtrl_SetVisited(self.as_ptr(), visited) }
    }
    /// Sets the colour used to print the label when the mouse is not over the control and the link has already been clicked before (i.e. the link has been visited).
    fn set_visited_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetVisitedColour(self.as_ptr(), colour)
        }
    }
}

// wxHyperlinkEvent
pub trait HyperlinkEventMethods: CommandEventMethods {
    /// Returns the URL of the hyperlink where the user has just clicked.
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkEvent_GetURL(self.as_ptr())).into() }
    }
    /// Sets the URL associated with the event.
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkEvent_SetURL(self.as_ptr(), url)
        }
    }
}
