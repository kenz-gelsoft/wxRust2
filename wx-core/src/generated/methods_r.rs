use super::*;

// wxRadioBox
pub trait RadioBoxMethods: ControlMethods {
    // DTOR: fn ~wxRadioBox()
    // NOT_SUPPORTED: fn Create()
    /// Creates the radiobox for two-step construction.
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
    /// Enables or disables an individual button in the radiobox.
    fn enable_uint(&self, n: c_uint, enable: bool) -> bool {
        unsafe { ffi::wxRadioBox_Enable(self.as_ptr(), n, enable) }
    }
    /// Returns the number of columns in the radiobox.
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetColumnCount(self.as_ptr()) }
    }
    /// Returns a radio box item under the point, a zero-based item index, or wxNOT_FOUND if no item is under the point.
    fn get_item_from_point<P: PointMethods>(&self, pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRadioBox_GetItemFromPoint(self.as_ptr(), pt)
        }
    }
    /// Returns the helptext associated with the specified item if any or wxEmptyString.
    fn get_item_help_text(&self, item: c_uint) -> String {
        unsafe { WxString::from_ptr(ffi::wxRadioBox_GetItemHelpText(self.as_ptr(), item)).into() }
    }
    /// Returns the tooltip associated with the specified item if any or NULL.
    fn get_item_tool_tip(&self, item: c_uint) -> Option<ToolTipIsOwned<false>> {
        unsafe { ToolTip::option_from(ffi::wxRadioBox_GetItemToolTip(self.as_ptr(), item)) }
    }
    /// Returns the number of rows in the radiobox.
    fn get_row_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetRowCount(self.as_ptr()) }
    }
    /// Returns true if the item is enabled or false if it was disabled using Enable(n, false).
    fn is_item_enabled(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemEnabled(self.as_ptr(), n) }
    }
    /// Returns true if the item is currently shown or false if it was hidden using Show(n, false).
    fn is_item_shown(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemShown(self.as_ptr(), n) }
    }
    /// Sets the helptext for an item.
    fn set_item_help_text(&self, item: c_uint, helptext: &str) {
        unsafe {
            let helptext = WxString::from(helptext);
            let helptext = helptext.as_ptr();
            ffi::wxRadioBox_SetItemHelpText(self.as_ptr(), item, helptext)
        }
    }
    /// Sets the tooltip text for the specified item in the radio group.
    fn set_item_tool_tip(&self, item: c_uint, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxRadioBox_SetItemToolTip(self.as_ptr(), item, text)
        }
    }
    /// Shows or hides individual buttons.
    fn show_uint(&self, item: c_uint, show: bool) -> bool {
        unsafe { ffi::wxRadioBox_Show(self.as_ptr(), item, show) }
    }
}

// wxRadioButton
pub trait RadioButtonMethods: ControlMethods {
    // DTOR: fn ~wxRadioButton()
    /// Creates the choice for two-step construction.
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
    /// Returns true if the radio button is checked, false otherwise.
    fn get_value(&self) -> bool {
        unsafe { ffi::wxRadioButton_GetValue(self.as_ptr()) }
    }
    /// Sets the radio button to checked or unchecked status.
    fn set_value(&self, value: bool) {
        unsafe { ffi::wxRadioButton_SetValue(self.as_ptr(), value) }
    }
    /// Returns the first button of the radio button group this button belongs to.
    fn get_first_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetFirstInGroup(self.as_ptr())) }
    }
    /// Returns the last button of the radio button group this button belongs to.
    fn get_last_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetLastInGroup(self.as_ptr())) }
    }
    /// Returns the previous radio button in the same group.
    fn get_previous_in_group(&self) -> WeakRef<RadioButton> {
        unsafe {
            WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetPreviousInGroup(self.as_ptr()))
        }
    }
    /// Returns the next radio button in the same group.
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
    /// Effectively creates the window for an object created using the default constructor.
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
    /// Return the listbox which is the main part of this control.
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeCtrl_GetList(self.as_ptr())) }
    }
}

// wxRearrangeDialog
pub trait RearrangeDialogMethods: DialogMethods {
    /// Effectively creates the dialog for an object created using the default constructor.
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
    /// Customize the dialog by adding extra controls to it.
    fn add_extra_controls<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRearrangeDialog_AddExtraControls(self.as_ptr(), win)
        }
    }
    /// Return the list control used by the dialog.
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeDialog_GetList(self.as_ptr())) }
    }
    /// Return the array describing the order of items after it was modified by the user.
    fn get_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxRearrangeDialog_GetOrder(self.as_ptr())) }
    }
}

// wxRearrangeList
pub trait RearrangeListMethods: CheckListBoxMethods {
    /// Effectively creates the window for an object created using the default constructor.
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
    /// Return the current order of the items.
    fn get_current_order(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxRearrangeList_GetCurrentOrder(self.as_ptr())) }
    }
    /// Return true if the currently selected item can be moved up.
    fn can_move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentUp(self.as_ptr()) }
    }
    /// Return true if the currently selected item can be moved down.
    fn can_move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentDown(self.as_ptr()) }
    }
    /// Move the currently selected item one position above.
    fn move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentUp(self.as_ptr()) }
    }
    /// Move the currently selected item one position below.
    fn move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentDown(self.as_ptr()) }
    }
}

// wxRect
pub trait RectMethods: WxRustMethods {
    /// Returns the rectangle having the same size as this one but centered relatively to the given rectangle r.
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
    /// Returns true if the given point is inside the rectangle (or on its boundary) and false otherwise.
    fn contains_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr(), x, y) }
    }
    /// Returns true if the given point is inside the rectangle (or on its boundary) and false otherwise.
    fn contains_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Contains1(self.as_ptr(), pt)
        }
    }
    /// Returns true if the given rectangle is completely inside this rectangle (or touches its boundary) and false otherwise.
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
    /// Gets the bottom point of the rectangle.
    fn get_bottom(&self) -> c_int {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr()) }
    }
    /// Gets the position of the bottom left corner.
    fn get_bottom_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomLeft(self.as_ptr())) }
    }
    /// Gets the position of the bottom right corner.
    fn get_bottom_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomRight(self.as_ptr())) }
    }
    /// Gets the height member.
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr()) }
    }
    /// Gets the left point of the rectangle (the same as GetX()).
    fn get_left(&self) -> c_int {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr()) }
    }
    /// Gets the position.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetPosition(self.as_ptr())) }
    }
    /// Gets the right point of the rectangle.
    fn get_right(&self) -> c_int {
        unsafe { ffi::wxRect_GetRight(self.as_ptr()) }
    }
    /// Gets the size.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxRect_GetSize(self.as_ptr())) }
    }
    /// Gets the top point of the rectangle (the same as GetY()).
    fn get_top(&self) -> c_int {
        unsafe { ffi::wxRect_GetTop(self.as_ptr()) }
    }
    /// Gets the position of the top left corner of the rectangle, same as GetPosition().
    fn get_top_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopLeft(self.as_ptr())) }
    }
    /// Gets the position of the top right corner.
    fn get_top_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopRight(self.as_ptr())) }
    }
    /// Gets the width member.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr()) }
    }
    /// Gets the x member.
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRect_GetX(self.as_ptr()) }
    }
    /// Gets the y member.
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
    /// Returns the overlapping portion of this rectangle and the one passed in as parameter.
    fn intersect<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect::from_ptr(ffi::wxRect_Intersect1(self.as_ptr(), rect))
        }
    }
    /// Returns true if this rectangle has a non-empty intersection with the rectangle rect and false otherwise.
    fn intersects<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Intersects(self.as_ptr(), rect)
        }
    }
    /// Returns true if this rectangle has a width or height less than or equal to 0 and false otherwise.
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr()) }
    }
    /// Moves the rectangle by the specified offset.
    fn offset_coord(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxRect_Offset(self.as_ptr(), dx, dy) }
    }
    fn offset_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Offset1(self.as_ptr(), pt)
        }
    }
    /// Sets the height.
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr(), height) }
    }
    /// Sets the position.
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxRect_SetPosition(self.as_ptr(), pos)
        }
    }
    /// Sets the size.
    fn set_size<S: SizeMethods>(&self, s: &S) {
        unsafe {
            let s = s.as_ptr();
            ffi::wxRect_SetSize(self.as_ptr(), s)
        }
    }
    /// Sets the width.
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr(), width) }
    }
    /// Sets the x position.
    fn set_x(&self, x: c_int) {
        unsafe { ffi::wxRect_SetX(self.as_ptr(), x) }
    }
    /// Sets the y position.
    fn set_y(&self, y: c_int) {
        unsafe { ffi::wxRect_SetY(self.as_ptr(), y) }
    }
    /// Set the left side of the rectangle.
    fn set_left(&self, left: c_int) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr(), left) }
    }
    /// Set the right side of the rectangle.
    fn set_right(&self, right: c_int) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr(), right) }
    }
    /// Set the top edge of the rectangle.
    fn set_top(&self, top: c_int) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr(), top) }
    }
    /// Set the bottom edge of the rectangle.
    fn set_bottom(&self, bottom: c_int) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr(), bottom) }
    }
    /// Set the top-left point of the rectangle.
    fn set_top_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopLeft(self.as_ptr(), p)
        }
    }
    /// Set the bottom-right point of the rectangle.
    fn set_bottom_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomRight(self.as_ptr(), p)
        }
    }
    /// Set the top-right point of the rectangle.
    fn set_top_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopRight(self.as_ptr(), p)
        }
    }
    /// Set the bottom-left point of the rectangle.
    fn set_bottom_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomLeft(self.as_ptr(), p)
        }
    }
    /// Modifies the rectangle to contain the bounding box of this rectangle and the one passed in as parameter.
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

// wxRegion
pub trait RegionMethods: GDIObjectMethods {
    // DTOR: fn ~wxRegion()
    /// Clears the current region.
    fn clear(&self) {
        unsafe { ffi::wxRegion_Clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    // NOT_SUPPORTED: fn Contains2()
    // NOT_SUPPORTED: fn Contains3()
    /// Convert the region to a black and white bitmap with the white pixels being inside the region.
    fn convert_to_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxRegion_ConvertToBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetBox()
    fn get_box(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegion_GetBox1(self.as_ptr())) }
    }
    /// Finds the intersection of this region and another, rectangular region, specified using position and size.
    fn intersect_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Intersect(self.as_ptr(), x, y, width, height) }
    }
    /// Finds the intersection of this region and another, rectangular region.
    fn intersect_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Intersect1(self.as_ptr(), rect)
        }
    }
    /// Finds the intersection of this region and another region.
    fn intersect_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Intersect2(self.as_ptr(), region)
        }
    }
    /// Returns true if the region is empty, false otherwise.
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRegion_IsEmpty(self.as_ptr()) }
    }
    /// Returns true if the region is equal to, i.e. covers the same area as, another one.
    fn is_equal<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_IsEqual(self.as_ptr(), region)
        }
    }
    /// Moves the region by the specified offsets in horizontal and vertical directions.
    fn offset_coord(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRegion_Offset(self.as_ptr(), x, y) }
    }
    fn offset_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRegion_Offset1(self.as_ptr(), pt)
        }
    }
    /// Subtracts a rectangular region from this region.
    fn subtract_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Subtract(self.as_ptr(), rect)
        }
    }
    /// Subtracts a region from this region.
    fn subtract_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Subtract1(self.as_ptr(), region)
        }
    }
    /// Finds the union of this region and another, rectangular region, specified using position and size.
    fn union_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Union(self.as_ptr(), x, y, width, height) }
    }
    /// Finds the union of this region and another, rectangular region.
    fn union_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Union1(self.as_ptr(), rect)
        }
    }
    /// Finds the union of this region and another region.
    fn union_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Union2(self.as_ptr(), region)
        }
    }
    /// Finds the union of this region and the non-transparent pixels of a bitmap.
    fn union_bitmap<B: BitmapMethods>(&self, bmp: &B) -> bool {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxRegion_Union3(self.as_ptr(), bmp)
        }
    }
    /// Finds the union of this region and the non-transparent pixels of a bitmap.
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
    /// Finds the Xor of this region and another, rectangular region, specified using position and size.
    fn xor_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Xor(self.as_ptr(), x, y, width, height) }
    }
    /// Finds the Xor of this region and another, rectangular region.
    fn xor_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Xor1(self.as_ptr(), rect)
        }
    }
    /// Finds the Xor of this region and another region.
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
    /// An alias for GetHeight().
    fn get_h(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetH(self.as_ptr()) }
    }
    /// Returns the height value for the current region.
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetHeight(self.as_ptr()) }
    }
    /// Returns the current rectangle.
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegionIterator_GetRect(self.as_ptr())) }
    }
    /// An alias for GetWidth().
    fn get_w(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetW(self.as_ptr()) }
    }
    /// Returns the width value for the current region.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetWidth(self.as_ptr()) }
    }
    /// Returns the x value for the current region.
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetX(self.as_ptr()) }
    }
    /// Returns the y value for the current region.
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetY(self.as_ptr()) }
    }
    /// Returns true if there are still some rectangles; otherwise returns false.
    fn have_rects(&self) -> bool {
        unsafe { ffi::wxRegionIterator_HaveRects(self.as_ptr()) }
    }
    /// Resets the iterator to the beginning of the rectangles.
    fn reset(&self) {
        unsafe { ffi::wxRegionIterator_Reset(self.as_ptr()) }
    }
    /// Resets the iterator to the given region.
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
    /// Draw a check box.
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
    /// Draw a button like the one used by wxComboBox to show a drop down window.
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
    /// Draw a drop down arrow that is suitable for use outside a combo box.
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
    /// Draw a focus rectangle using the specified rectangle.
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
    /// Draw a progress bar in the specified rectangle.
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
    /// Draw a selection rectangle underneath the text as used e.g.
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
    /// Draw item text in the correct color based on selection status.
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
    /// Draw a blank push button that looks very similar to wxButton.
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
    /// Draw a collapse button.
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
    /// Returns the size of a collapse button.
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
    /// Draw the border for sash window: this border must be such that the sash drawn by DrawSplitterSash() blends into it well.
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
    /// Draw the expanded/collapsed icon for a tree control item.
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
    /// Draw a native wxChoice.
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
    /// Draw a native wxComboBox.
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
    /// Draw a native wxTextCtrl frame.
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
    /// Draw a native wxRadioButton bitmap.
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
    /// Draw a check mark.
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
    /// Returns the size of a check box.
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
    /// Returns the size of a check mark.
    fn get_check_mark_size<W: WindowMethods>(&self, win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetCheckMarkSize(self.as_ptr(), win))
        }
    }
    /// Returns the size of the expander used in tree-like controls.
    fn get_expander_size<W: WindowMethods>(&self, win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetExpanderSize(self.as_ptr(), win))
        }
    }
    /// Returns the height of a header button, either a fixed platform height if available, or a generic height based on the win window's font.
    fn get_header_button_height<W: WindowMethods>(&self, win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRendererNative_GetHeaderButtonHeight(self.as_ptr(), win)
        }
    }
    /// Returns the horizontal margin on the left and right sides of header button's label.
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
    /// Return the currently used renderer.
    fn get() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_Get()) }
    }
    /// Return the default (native) implementation for this platform  this is also the one used by default but this may be changed by calling Set() in which case the return value of this method may be different from the return value of Get().
    fn get_default() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetDefault()) }
    }
    /// Return the generic implementation of the renderer.
    fn get_generic() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetGeneric()) }
    }
    /// Load the renderer from the specified DLL, the returned pointer must be deleted by caller if not NULL when it is not used any more.
    fn load(name: &str) -> Option<RendererNativeIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            RendererNative::option_from(ffi::wxRendererNative_Load(name))
        }
    }
    /// Set the renderer to use, passing NULL reverts to using the default renderer (the global renderer must always exist).
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
    /// Set the background colour.
    fn set_background_colour<C: ColourMethods, C2: ColourMethods>(&self, col: &C, col_end: &C2) {
        unsafe {
            let col = col.as_ptr();
            let col_end = col_end.as_ptr();
            ffi::wxRichToolTip_SetBackgroundColour(self.as_ptr(), col, col_end)
        }
    }
    /// Set the small icon to show.
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
    /// Set the title text font.
    fn set_title_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxRichToolTip_SetTitleFont(self.as_ptr(), font)
        }
    }
    /// Show the tooltip for the given window and optionally specify where to show the tooltip.
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
