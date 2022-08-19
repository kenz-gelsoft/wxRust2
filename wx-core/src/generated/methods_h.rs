use super::*;

// wxHScrolledWindow
pub trait HScrolledWindowMethods: PanelMethods {}

// wxHTMLDataObject
pub trait HTMLDataObjectMethods: DataObjectSimpleMethods {
    fn get_html(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHTMLDataObject_GetHTML(self.as_ptr())).into() }
    }
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
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHeaderColumn_GetTitle(self.as_ptr())).into() }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxHeaderColumn_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxHeaderColumn_GetBitmapBundle(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetWidth(self.as_ptr()) }
    }
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetMinWidth(self.as_ptr()) }
    }
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetAlignment(self.as_ptr()) }
    }
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetFlags(self.as_ptr()) }
    }
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxHeaderColumn_HasFlag(self.as_ptr(), flag) }
    }
    fn is_resizeable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsResizeable(self.as_ptr()) }
    }
    fn is_sortable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortable(self.as_ptr()) }
    }
    fn is_reorderable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsReorderable(self.as_ptr()) }
    }
    fn is_hidden(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsHidden(self.as_ptr()) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsShown(self.as_ptr()) }
    }
    fn is_sort_key(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortKey(self.as_ptr()) }
    }
    fn is_sort_order_ascending(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortOrderAscending(self.as_ptr()) }
    }
}

// wxHeaderColumnSimple
pub trait HeaderColumnSimpleMethods: SettableHeaderColumnMethods {}

// wxHeaderCtrl
pub trait HeaderCtrlMethods: ControlMethods {
    fn set_column_count(&self, count: c_uint) {
        unsafe { ffi::wxHeaderCtrl_SetColumnCount(self.as_ptr(), count) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_IsEmpty(self.as_ptr()) }
    }
    fn update_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrl_UpdateColumn(self.as_ptr(), idx) }
    }
    fn set_columns_order<A: ArrayIntMethods>(&self, order: &A) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_SetColumnsOrder(self.as_ptr(), order)
        }
    }
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxHeaderCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    fn get_column_at(&self, pos: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnAt(self.as_ptr(), pos) }
    }
    fn get_column_pos(&self, idx: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnPos(self.as_ptr(), idx) }
    }
    fn reset_columns_order(&self) {
        unsafe { ffi::wxHeaderCtrl_ResetColumnsOrder(self.as_ptr()) }
    }
    fn show_columns_menu<P: PointMethods>(&self, pt: &P, title: &str) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxHeaderCtrl_ShowColumnsMenu(self.as_ptr(), pt, title)
        }
    }
    fn add_columns_items<M: MenuMethods>(&self, menu: &M, id_columns_base: c_int) {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxHeaderCtrl_AddColumnsItems(self.as_ptr(), menu, id_columns_base)
        }
    }
    fn show_customize_dialog(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_ShowCustomizeDialog(self.as_ptr()) }
    }
    fn get_column_title_width_headercolumn<H: HeaderColumnMethods>(&self, col: &H) -> c_int {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrl_GetColumnTitleWidth(self.as_ptr(), col)
        }
    }
    fn get_column_title_width_uint(&self, idx: c_uint) -> c_int {
        unsafe { ffi::wxHeaderCtrl_GetColumnTitleWidth1(self.as_ptr(), idx) }
    }
    fn move_column_in_order_array<A: ArrayIntMethods>(order: &A, idx: c_uint, pos: c_uint) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_MoveColumnInOrderArray(order, idx, pos)
        }
    }
}

// wxHeaderCtrlEvent
pub trait HeaderCtrlEventMethods: NotifyEventMethods {
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetColumn(self.as_ptr()) }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetColumn(self.as_ptr(), col) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetWidth(self.as_ptr()) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetWidth(self.as_ptr(), width) }
    }
    fn get_new_order(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrlEvent_GetNewOrder(self.as_ptr()) }
    }
    fn set_new_order(&self, order: c_uint) {
        unsafe { ffi::wxHeaderCtrlEvent_SetNewOrder(self.as_ptr(), order) }
    }
}

// wxHeaderCtrlSimple
pub trait HeaderCtrlSimpleMethods: HeaderCtrlMethods {
    fn insert_column<H: HeaderColumnSimpleMethods>(&self, col: &H, idx: c_uint) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_InsertColumn(self.as_ptr(), col, idx)
        }
    }
    fn append_column<H: HeaderColumnSimpleMethods>(&self, col: &H) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_AppendColumn(self.as_ptr(), col)
        }
    }
    fn delete_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_DeleteColumn(self.as_ptr(), idx) }
    }
    fn show_column(&self, idx: c_uint, show: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowColumn(self.as_ptr(), idx, show) }
    }
    fn hide_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_HideColumn(self.as_ptr(), idx) }
    }
    fn show_sort_indicator(&self, idx: c_uint, sort_order: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowSortIndicator(self.as_ptr(), idx, sort_order) }
    }
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxHeaderCtrlSimple_RemoveSortIndicator(self.as_ptr()) }
    }
}

// wxHelpEvent
pub trait HelpEventMethods: CommandEventMethods {
    // NOT_SUPPORTED: fn GetOrigin()
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxHelpEvent_GetPosition(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetOrigin()
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxHelpEvent_SetPosition(self.as_ptr(), pt)
        }
    }
}

// wxHelpProvider
pub trait HelpProviderMethods: WxRustMethods {
    // DTOR: fn ~wxHelpProvider()
    fn add_help_window<W: WindowMethods>(&self, window: Option<&W>, text: &str) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxHelpProvider_AddHelp(self.as_ptr(), window, text)
        }
    }
    fn add_help_windowid(&self, id: c_int, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxHelpProvider_AddHelp1(self.as_ptr(), id, text)
        }
    }
    fn get_help<W: WindowMethods>(&self, window: Option<&W>) -> String {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WxString::from_ptr(ffi::wxHelpProvider_GetHelp(self.as_ptr(), window)).into()
        }
    }
    fn remove_help<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxHelpProvider_RemoveHelp(self.as_ptr(), window)
        }
    }
    fn show_help<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxHelpProvider_ShowHelp(self.as_ptr(), window)
        }
    }
    // NOT_SUPPORTED: fn ShowHelpAtPoint()
    fn get() -> Option<HelpProviderIsOwned<false>> {
        unsafe { HelpProvider::option_from(ffi::wxHelpProvider_Get()) }
    }
    fn set<H: HelpProviderMethods>(
        help_provider: Option<&H>,
    ) -> Option<HelpProviderIsOwned<false>> {
        unsafe {
            let help_provider = match help_provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HelpProvider::option_from(ffi::wxHelpProvider_Set(help_provider))
        }
    }
}

// wxHyperlinkCtrl
pub trait HyperlinkCtrlMethods: ControlMethods {
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
    fn get_hover_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetHoverColour(self.as_ptr())) }
    }
    fn get_normal_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetNormalColour(self.as_ptr())) }
    }
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkCtrl_GetURL(self.as_ptr())).into() }
    }
    fn get_visited(&self) -> bool {
        unsafe { ffi::wxHyperlinkCtrl_GetVisited(self.as_ptr()) }
    }
    fn get_visited_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetVisitedColour(self.as_ptr())) }
    }
    fn set_hover_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetHoverColour(self.as_ptr(), colour)
        }
    }
    fn set_normal_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetNormalColour(self.as_ptr(), colour)
        }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkCtrl_SetURL(self.as_ptr(), url)
        }
    }
    fn set_visited(&self, visited: bool) {
        unsafe { ffi::wxHyperlinkCtrl_SetVisited(self.as_ptr(), visited) }
    }
    fn set_visited_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetVisitedColour(self.as_ptr(), colour)
        }
    }
}

// wxHyperlinkEvent
pub trait HyperlinkEventMethods: CommandEventMethods {
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkEvent_GetURL(self.as_ptr())).into() }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkEvent_SetURL(self.as_ptr(), url)
        }
    }
}
