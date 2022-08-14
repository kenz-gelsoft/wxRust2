use super::*;

// wxRadioBox
pub trait RadioBoxMethods: ControlMethods {
    // DTOR: fn ~wxRadioBox()
    // NOT_SUPPORTED: fn Create()
    fn create_str<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        choices: &A,
        major_dimension: c_int,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRadioBox_Create1(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                choices,
                major_dimension,
                style,
                validator,
                name,
            )
        }
    }
    fn enable_uint(&self, n: c_uint, enable: bool) -> bool {
        unsafe { ffi::wxRadioBox_Enable(self.as_ptr(), n, enable) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetColumnCount(self.as_ptr()) }
    }
    fn get_item_from_point<P: PointMethods>(&self, pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRadioBox_GetItemFromPoint(self.as_ptr(), pt)
        }
    }
    fn get_item_help_text(&self, item: c_uint) -> String {
        unsafe { WxString::from_ptr(ffi::wxRadioBox_GetItemHelpText(self.as_ptr(), item)).into() }
    }
    fn get_item_tool_tip(&self, item: c_uint) -> *mut c_void {
        unsafe { ffi::wxRadioBox_GetItemToolTip(self.as_ptr(), item) }
    }
    fn get_row_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetRowCount(self.as_ptr()) }
    }
    fn is_item_enabled(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemEnabled(self.as_ptr(), n) }
    }
    fn is_item_shown(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemShown(self.as_ptr(), n) }
    }
    fn set_item_help_text(&self, item: c_uint, helptext: &str) {
        unsafe {
            let helptext = WxString::from(helptext);
            let helptext = helptext.as_ptr();
            ffi::wxRadioBox_SetItemHelpText(self.as_ptr(), item, helptext)
        }
    }
    fn set_item_tool_tip(&self, item: c_uint, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxRadioBox_SetItemToolTip(self.as_ptr(), item, text)
        }
    }
    fn show_uint(&self, item: c_uint, show: bool) -> bool {
        unsafe { ffi::wxRadioBox_Show(self.as_ptr(), item, show) }
    }
}

// wxRect
pub trait RectMethods: WxRustMethods {
    fn centre_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect::from_ptr(ffi::wxRect_CentreIn(self.as_ptr(), r, dir))
        }
    }
    fn center_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect::from_ptr(ffi::wxRect_CenterIn(self.as_ptr(), r, dir))
        }
    }
    fn contains_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr(), x, y) }
    }
    fn contains_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Contains1(self.as_ptr(), pt)
        }
    }
    fn contains_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Contains2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRect_Deflate3(self.as_ptr(), dx, dy)) }
    }
    fn get_bottom(&self) -> c_int {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr()) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomLeft(self.as_ptr())) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomRight(self.as_ptr())) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr()) }
    }
    fn get_left(&self) -> c_int {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetPosition(self.as_ptr())) }
    }
    fn get_right(&self) -> c_int {
        unsafe { ffi::wxRect_GetRight(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxRect_GetSize(self.as_ptr())) }
    }
    fn get_top(&self) -> c_int {
        unsafe { ffi::wxRect_GetTop(self.as_ptr()) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopLeft(self.as_ptr())) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopRight(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr()) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRect_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRect_GetY(self.as_ptr()) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRect_Inflate3(self.as_ptr(), dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect::from_ptr(ffi::wxRect_Intersect1(self.as_ptr(), rect))
        }
    }
    fn intersects<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Intersects(self.as_ptr(), rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr()) }
    }
    fn offset_coord(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxRect_Offset(self.as_ptr(), dx, dy) }
    }
    fn offset_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Offset1(self.as_ptr(), pt)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr(), height) }
    }
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxRect_SetPosition(self.as_ptr(), pos)
        }
    }
    fn set_size<S: SizeMethods>(&self, s: &S) {
        unsafe {
            let s = s.as_ptr();
            ffi::wxRect_SetSize(self.as_ptr(), s)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr(), width) }
    }
    fn set_x(&self, x: c_int) {
        unsafe { ffi::wxRect_SetX(self.as_ptr(), x) }
    }
    fn set_y(&self, y: c_int) {
        unsafe { ffi::wxRect_SetY(self.as_ptr(), y) }
    }
    fn set_left(&self, left: c_int) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr(), left) }
    }
    fn set_right(&self, right: c_int) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr(), right) }
    }
    fn set_top(&self, top: c_int) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr(), top) }
    }
    fn set_bottom(&self, bottom: c_int) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr(), bottom) }
    }
    fn set_top_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopLeft(self.as_ptr(), p)
        }
    }
    fn set_bottom_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomRight(self.as_ptr(), p)
        }
    }
    fn set_top_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopRight(self.as_ptr(), p)
        }
    }
    fn set_bottom_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomLeft(self.as_ptr(), p)
        }
    }
    fn union<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect::from_ptr(ffi::wxRect_Union(self.as_ptr(), rect))
        }
    }
    // BLOCKED: fn Union1()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxRendererNative
pub trait RendererNativeMethods: WxRustMethods {
    // DTOR: fn ~wxRendererNative()
    fn draw_check_box<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawCheckBox(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_combo_box_drop_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawComboBoxDropButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_drop_arrow<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawDropArrow(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_focus_rect<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawFocusRect(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_gauge<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        value: c_int,
        max: c_int,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawGauge(self.as_ptr(), win, dc, rect, value, max, flags)
        }
    }
    // NOT_SUPPORTED: fn DrawHeaderButton()
    // NOT_SUPPORTED: fn DrawHeaderButtonContents()
    fn draw_item_selection_rect<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawItemSelectionRect(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_item_text<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        text: &str,
        rect: &R,
        align: c_int,
        flags: c_int,
        ellipsize_mode: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawItemText(
                self.as_ptr(),
                win,
                dc,
                text,
                rect,
                align,
                flags,
                ellipsize_mode,
            )
        }
    }
    fn draw_push_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawPushButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_collapse_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawCollapseButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn get_collapse_button_size<W: WindowMethods, D: DCMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
    ) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            Size::from_ptr(ffi::wxRendererNative_GetCollapseButtonSize(
                self.as_ptr(),
                win,
                dc,
            ))
        }
    }
    fn draw_splitter_border<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawSplitterBorder(self.as_ptr(), win, dc, rect, flags)
        }
    }
    // NOT_SUPPORTED: fn DrawSplitterSash()
    fn draw_tree_item_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawTreeItemButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_choice<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawChoice(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_combo_box<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawComboBox(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_text_ctrl<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawTextCtrl(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_radio_bitmap<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawRadioBitmap(self.as_ptr(), win, dc, rect, flags)
        }
    }
    // NOT_SUPPORTED: fn DrawTitleBarBitmap()
    fn draw_check_mark<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawCheckMark(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn get_check_box_size<W: WindowMethods>(&self, win: Option<&W>, flags: c_int) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetCheckBoxSize(
                self.as_ptr(),
                win,
                flags,
            ))
        }
    }
    fn get_check_mark_size<W: WindowMethods>(&self, win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetCheckMarkSize(self.as_ptr(), win))
        }
    }
    fn get_expander_size<W: WindowMethods>(&self, win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetExpanderSize(self.as_ptr(), win))
        }
    }
    fn get_header_button_height<W: WindowMethods>(&self, win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRendererNative_GetHeaderButtonHeight(self.as_ptr(), win)
        }
    }
    fn get_header_button_margin<W: WindowMethods>(&self, win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRendererNative_GetHeaderButtonMargin(self.as_ptr(), win)
        }
    }
    // NOT_SUPPORTED: fn GetSplitterParams()
    // NOT_SUPPORTED: fn GetVersion()
    fn get() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_Get()) }
    }
    fn get_default() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetDefault()) }
    }
    fn get_generic() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetGeneric()) }
    }
    fn load(name: &str) -> Option<RendererNativeIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            RendererNative::option_from(ffi::wxRendererNative_Load(name))
        }
    }
    fn set<R: RendererNativeMethods>(renderer: Option<&R>) -> Option<RendererNativeIsOwned<false>> {
        unsafe {
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            RendererNative::option_from(ffi::wxRendererNative_Set(renderer))
        }
    }
}
