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

// wxRadioButton
pub trait RadioButtonMethods: ControlMethods {
    // DTOR: fn ~wxRadioButton()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
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
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRadioButton_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_value(&self) -> bool {
        unsafe { ffi::wxRadioButton_GetValue(self.as_ptr()) }
    }
    fn set_value(&self, value: bool) {
        unsafe { ffi::wxRadioButton_SetValue(self.as_ptr(), value) }
    }
    fn get_first_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetFirstInGroup(self.as_ptr())) }
    }
    fn get_last_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetLastInGroup(self.as_ptr())) }
    }
    fn get_previous_in_group(&self) -> WeakRef<RadioButton> {
        unsafe {
            WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetPreviousInGroup(self.as_ptr()))
        }
    }
    fn get_next_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetNextInGroup(self.as_ptr())) }
    }
}

// wxRealPoint
pub trait RealPointMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+2()
    // BLOCKED: fn operator-2()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxRearrangeCtrl
pub trait RearrangeCtrlMethods: PanelMethods {
    fn create_arrayint<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
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
            let order = order.as_ptr();
            let items = items.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRearrangeCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                order,
                items,
                style,
                validator,
                name,
            )
        }
    }
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeCtrl_GetList(self.as_ptr())) }
    }
}

// wxRearrangeDialog
pub trait RearrangeDialogMethods: DialogMethods {
    fn create_str<W: WindowMethods, A: ArrayIntMethods, A2: ArrayStringMethods, P: PointMethods>(
        &self,
        parent: Option<&W>,
        message: &str,
        title: &str,
        order: &A,
        items: &A2,
        pos: &P,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            let order = order.as_ptr();
            let items = items.as_ptr();
            let pos = pos.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRearrangeDialog_Create(
                self.as_ptr(),
                parent,
                message,
                title,
                order,
                items,
                pos,
                name,
            )
        }
    }
    fn add_extra_controls<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRearrangeDialog_AddExtraControls(self.as_ptr(), win)
        }
    }
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeDialog_GetList(self.as_ptr())) }
    }
    fn get_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxRearrangeDialog_GetOrder(self.as_ptr())) }
    }
}

// wxRearrangeList
pub trait RearrangeListMethods: CheckListBoxMethods {
    fn create_arrayint<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
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
            let order = order.as_ptr();
            let items = items.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRearrangeList_Create(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                order,
                items,
                style,
                validator,
                name,
            )
        }
    }
    fn get_current_order(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxRearrangeList_GetCurrentOrder(self.as_ptr())) }
    }
    fn can_move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentUp(self.as_ptr()) }
    }
    fn can_move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentDown(self.as_ptr()) }
    }
    fn move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentUp(self.as_ptr()) }
    }
    fn move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentDown(self.as_ptr()) }
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

// wxRect
pub trait RectMethods: WxRustMethods {}

// wxRegion
pub trait RegionMethods: GDIObjectMethods {
    // DTOR: fn ~wxRegion()
    fn clear(&self) {
        unsafe { ffi::wxRegion_Clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    // NOT_SUPPORTED: fn Contains2()
    // NOT_SUPPORTED: fn Contains3()
    fn convert_to_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxRegion_ConvertToBitmap(self.as_ptr())) }
    }
    fn get_box_coord(
        &self,
        x: *mut c_void,
        y: *mut c_void,
        width: *mut c_void,
        height: *mut c_void,
    ) {
        unsafe { ffi::wxRegion_GetBox(self.as_ptr(), x, y, width, height) }
    }
    fn get_box(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegion_GetBox1(self.as_ptr())) }
    }
    fn intersect_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Intersect(self.as_ptr(), x, y, width, height) }
    }
    fn intersect_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Intersect1(self.as_ptr(), rect)
        }
    }
    fn intersect_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Intersect2(self.as_ptr(), region)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRegion_IsEmpty(self.as_ptr()) }
    }
    fn is_equal<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_IsEqual(self.as_ptr(), region)
        }
    }
    fn offset_coord(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRegion_Offset(self.as_ptr(), x, y) }
    }
    fn offset_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRegion_Offset1(self.as_ptr(), pt)
        }
    }
    fn subtract_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Subtract(self.as_ptr(), rect)
        }
    }
    fn subtract_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Subtract1(self.as_ptr(), region)
        }
    }
    fn union_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Union(self.as_ptr(), x, y, width, height) }
    }
    fn union_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Union1(self.as_ptr(), rect)
        }
    }
    fn union_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Union2(self.as_ptr(), region)
        }
    }
    fn union_bitmap<B: BitmapMethods>(&self, bmp: &B) -> bool {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxRegion_Union3(self.as_ptr(), bmp)
        }
    }
    fn union_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        &self,
        bmp: &B,
        trans_colour: &C,
        tolerance: c_int,
    ) -> bool {
        unsafe {
            let bmp = bmp.as_ptr();
            let trans_colour = trans_colour.as_ptr();
            ffi::wxRegion_Union4(self.as_ptr(), bmp, trans_colour, tolerance)
        }
    }
    fn xor_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Xor(self.as_ptr(), x, y, width, height) }
    }
    fn xor_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Xor1(self.as_ptr(), rect)
        }
    }
    fn xor_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Xor2(self.as_ptr(), region)
        }
    }
    // BLOCKED: fn operator=()
}

// wxRegionIterator
pub trait RegionIteratorMethods: ObjectMethods {
    fn get_h(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetH(self.as_ptr()) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetHeight(self.as_ptr()) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegionIterator_GetRect(self.as_ptr())) }
    }
    fn get_w(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetW(self.as_ptr()) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetWidth(self.as_ptr()) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetY(self.as_ptr()) }
    }
    fn have_rects(&self) -> bool {
        unsafe { ffi::wxRegionIterator_HaveRects(self.as_ptr()) }
    }
    fn reset(&self) {
        unsafe { ffi::wxRegionIterator_Reset(self.as_ptr()) }
    }
    fn reset_region<R: RegionMethods>(&self, region: &R) {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegionIterator_Reset1(self.as_ptr(), region)
        }
    }
    // BLOCKED: fn operator++()
    // NOT_SUPPORTED: fn operator bool()
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

// wxRichToolTip
pub trait RichToolTipMethods: WxRustMethods {
    fn set_background_colour<C: ColourMethods, C2: ColourMethods>(&self, col: &C, col_end: &C2) {
        unsafe {
            let col = col.as_ptr();
            let col_end = col_end.as_ptr();
            ffi::wxRichToolTip_SetBackgroundColour(self.as_ptr(), col, col_end)
        }
    }
    fn set_icon_int(&self, icon: c_int) {
        unsafe { ffi::wxRichToolTip_SetIcon(self.as_ptr(), icon) }
    }
    fn set_icon_bitmapbundle<B: BitmapBundleMethods>(&self, icon: &B) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxRichToolTip_SetIcon1(self.as_ptr(), icon)
        }
    }
    // NOT_SUPPORTED: fn SetTimeout()
    // NOT_SUPPORTED: fn SetTipKind()
    fn set_title_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxRichToolTip_SetTitleFont(self.as_ptr(), font)
        }
    }
    fn show_for<W: WindowMethods, R: RectMethods>(&self, win: Option<&W>, rect: Option<&R>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRichToolTip_ShowFor(self.as_ptr(), win, rect)
        }
    }
    // DTOR: fn ~wxRichToolTip()
}
