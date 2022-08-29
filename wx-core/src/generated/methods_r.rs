use super::*;

// wxRadioBox
/// This trait represents C++ [`wxRadioBox`](https://docs.wxwidgets.org/3.2/classwx_radio_box.html) class's methods and inheritance.
///
/// See [`RadioBoxIsOwned`] documentation for the class usage.
pub trait RadioBoxMethods: ControlMethods {
    // DTOR: fn ~wxRadioBox()
    // NOT_SUPPORTED: fn Create()
    /// Creates the radiobox for two-step construction.
    ///
    /// [See `wxRadioBox::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a866a228778655e03f9ca7777e558935d)
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
    ///
    /// [See `wxRadioBox::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a0e297404226d3737560ddd15eebcdccc)
    fn enable_uint(&self, n: c_uint, enable: bool) -> bool {
        unsafe { ffi::wxRadioBox_Enable(self.as_ptr(), n, enable) }
    }
    /// Returns the number of columns in the radiobox.
    ///
    /// [See `wxRadioBox::GetColumnCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#aebc5d922d224641c5f0a0574e4adbbdd)
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetColumnCount(self.as_ptr()) }
    }
    /// Returns a radio box item under the point, a zero-based item index, or wxNOT_FOUND if no item is under the point.
    ///
    /// [See `wxRadioBox::GetItemFromPoint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a6298b56942086da3d3275b37d7bae3ea)
    fn get_item_from_point<P: PointMethods>(&self, pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRadioBox_GetItemFromPoint(self.as_ptr(), pt)
        }
    }
    /// Returns the helptext associated with the specified item if any or wxEmptyString.
    ///
    /// [See `wxRadioBox::GetItemHelpText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a46c5a252960ba84c10074f462bb6499f)
    fn get_item_help_text(&self, item: c_uint) -> String {
        unsafe { WxString::from_ptr(ffi::wxRadioBox_GetItemHelpText(self.as_ptr(), item)).into() }
    }
    /// Returns the tooltip associated with the specified item if any or NULL.
    ///
    /// [See `wxRadioBox::GetItemToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#aae17bd6b2707de2d93ab1fba78850a41)
    fn get_item_tool_tip(&self, item: c_uint) -> Option<ToolTipIsOwned<false>> {
        unsafe { ToolTip::option_from(ffi::wxRadioBox_GetItemToolTip(self.as_ptr(), item)) }
    }
    /// Returns the number of rows in the radiobox.
    ///
    /// [See `wxRadioBox::GetRowCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#ab2f7947e95c6fbc11d3bb66e1c4b1954)
    fn get_row_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetRowCount(self.as_ptr()) }
    }
    /// Returns true if the item is enabled or false if it was disabled using Enable(n, false).
    ///
    /// [See `wxRadioBox::IsItemEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#aa58c438a4eac517b97550704f65e8b5d)
    fn is_item_enabled(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemEnabled(self.as_ptr(), n) }
    }
    /// Returns true if the item is currently shown or false if it was hidden using Show(n, false).
    ///
    /// [See `wxRadioBox::IsItemShown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#afc8200cb62219fd3818452592c4acc05)
    fn is_item_shown(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemShown(self.as_ptr(), n) }
    }
    /// Sets the helptext for an item.
    ///
    /// [See `wxRadioBox::SetItemHelpText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a2c7cbd294ddc0ce169947d9b447ce19f)
    fn set_item_help_text(&self, item: c_uint, helptext: &str) {
        unsafe {
            let helptext = WxString::from(helptext);
            let helptext = helptext.as_ptr();
            ffi::wxRadioBox_SetItemHelpText(self.as_ptr(), item, helptext)
        }
    }
    /// Sets the tooltip text for the specified item in the radio group.
    ///
    /// [See `wxRadioBox::SetItemToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a4eba2a89154b6ea56d3f51b1844bbde1)
    fn set_item_tool_tip(&self, item: c_uint, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxRadioBox_SetItemToolTip(self.as_ptr(), item, text)
        }
    }
    /// Shows or hides individual buttons.
    ///
    /// [See `wxRadioBox::Show()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a1ad6078e61378f77034b3a93d6d7b0d6)
    fn show_uint(&self, item: c_uint, show: bool) -> bool {
        unsafe { ffi::wxRadioBox_Show(self.as_ptr(), item, show) }
    }
}

// wxRadioButton
/// This trait represents C++ [`wxRadioButton`](https://docs.wxwidgets.org/3.2/classwx_radio_button.html) class's methods and inheritance.
///
/// See [`RadioButtonIsOwned`] documentation for the class usage.
pub trait RadioButtonMethods: ControlMethods {
    // DTOR: fn ~wxRadioButton()
    /// Creates the choice for two-step construction.
    ///
    /// [See `wxRadioButton::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#ab81787ff8cacbf7c5fb334b46bdf4495)
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
    ///
    /// [See `wxRadioButton::GetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#abd3a16d45529e8fdea724ce63ea6af48)
    fn get_value(&self) -> bool {
        unsafe { ffi::wxRadioButton_GetValue(self.as_ptr()) }
    }
    /// Sets the radio button to checked or unchecked status.
    ///
    /// [See `wxRadioButton::SetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#ac234464fbe7554b79dda8e1d025fe50b)
    fn set_value(&self, value: bool) {
        unsafe { ffi::wxRadioButton_SetValue(self.as_ptr(), value) }
    }
    /// Returns the first button of the radio button group this button belongs to.
    ///
    /// [See `wxRadioButton::GetFirstInGroup()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#ada71e1f7e252fccbd654bdf838a288ee)
    fn get_first_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetFirstInGroup(self.as_ptr())) }
    }
    /// Returns the last button of the radio button group this button belongs to.
    ///
    /// [See `wxRadioButton::GetLastInGroup()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#a630ce9db12750ab46be4a599ec15874f)
    fn get_last_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetLastInGroup(self.as_ptr())) }
    }
    /// Returns the previous radio button in the same group.
    ///
    /// [See `wxRadioButton::GetPreviousInGroup()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#a6e6397ea074d108310a11f5f63d2f09a)
    fn get_previous_in_group(&self) -> WeakRef<RadioButton> {
        unsafe {
            WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetPreviousInGroup(self.as_ptr()))
        }
    }
    /// Returns the next radio button in the same group.
    ///
    /// [See `wxRadioButton::GetNextInGroup()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#a66b46c7008321ba95083732f53703245)
    fn get_next_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetNextInGroup(self.as_ptr())) }
    }
}

// wxRealPoint
/// This trait represents C++ [`wxRealPoint`](https://docs.wxwidgets.org/3.2/classwx_real_point.html) class's methods and inheritance.
///
/// See [`RealPointIsOwned`] documentation for the class usage.
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
/// This trait represents C++ [`wxRearrangeCtrl`](https://docs.wxwidgets.org/3.2/classwx_rearrange_ctrl.html) class's methods and inheritance.
///
/// See [`RearrangeCtrlIsOwned`] documentation for the class usage.
pub trait RearrangeCtrlMethods: PanelMethods {
    /// Effectively creates the window for an object created using the default constructor.
    ///
    /// [See `wxRearrangeCtrl::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_ctrl.html#ab6b616278ce3cc9a095fbbd3e02620d9)
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
    ///
    /// [See `wxRearrangeCtrl::GetList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_ctrl.html#a0b5c2b7b228e5c0c64e674a516354182)
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeCtrl_GetList(self.as_ptr())) }
    }
}

// wxRearrangeDialog
/// This trait represents C++ [`wxRearrangeDialog`](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html) class's methods and inheritance.
///
/// See [`RearrangeDialogIsOwned`] documentation for the class usage.
pub trait RearrangeDialogMethods: DialogMethods {
    /// Effectively creates the dialog for an object created using the default constructor.
    ///
    /// [See `wxRearrangeDialog::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html#ae6bddbc5d1fc41031f518a266f62591b)
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
    ///
    /// [See `wxRearrangeDialog::AddExtraControls()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html#af3f804a35d4df4c22546d80c691474d3)
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
    ///
    /// [See `wxRearrangeDialog::GetList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html#a31e5432f50ad6fca9214b7ba10b8f444)
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeDialog_GetList(self.as_ptr())) }
    }
    /// Return the array describing the order of items after it was modified by the user.
    ///
    /// [See `wxRearrangeDialog::GetOrder()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html#a106cb37c33d53812c8df2886284fa87e)
    fn get_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxRearrangeDialog_GetOrder(self.as_ptr())) }
    }
}

// wxRearrangeList
/// This trait represents C++ [`wxRearrangeList`](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html) class's methods and inheritance.
///
/// See [`RearrangeListIsOwned`] documentation for the class usage.
pub trait RearrangeListMethods: CheckListBoxMethods {
    /// Effectively creates the window for an object created using the default constructor.
    ///
    /// [See `wxRearrangeList::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a48b402d920f8afaa5e84aa2638d514a1)
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
    ///
    /// [See `wxRearrangeList::GetCurrentOrder()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a7d9953adf768501bf27bdaf2c3b6e20d)
    fn get_current_order(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxRearrangeList_GetCurrentOrder(self.as_ptr())) }
    }
    /// Return true if the currently selected item can be moved up.
    ///
    /// [See `wxRearrangeList::CanMoveCurrentUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a892d04a926dbf32fb41408f2f6876098)
    fn can_move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentUp(self.as_ptr()) }
    }
    /// Return true if the currently selected item can be moved down.
    ///
    /// [See `wxRearrangeList::CanMoveCurrentDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a126f986e9eb985e99691088454888200)
    fn can_move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentDown(self.as_ptr()) }
    }
    /// Move the currently selected item one position above.
    ///
    /// [See `wxRearrangeList::MoveCurrentUp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a03703035ef32e9d1596138f3a8595070)
    fn move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentUp(self.as_ptr()) }
    }
    /// Move the currently selected item one position below.
    ///
    /// [See `wxRearrangeList::MoveCurrentDown()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a71a00acdbae45613486d562b7bc17470)
    fn move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentDown(self.as_ptr()) }
    }
}

// wxRect
/// This trait represents C++ [`wxRect`](https://docs.wxwidgets.org/3.2/classwx_rect.html) class's methods and inheritance.
///
/// See [`RectIsOwned`] documentation for the class usage.
pub trait RectMethods: WxRustMethods {
    /// Returns the rectangle having the same size as this one but centered relatively to the given rectangle r.
    ///
    /// [See `wxRect::CentreIn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a7ea7c06a0a4136736009e05fa6eb006f)
    fn centre_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect::from_ptr(ffi::wxRect_CentreIn(self.as_ptr(), r, dir))
        }
    }
    ///
    /// [See `wxRect::CenterIn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#ab89e40c6c82cd7271d0a304f4fe5ab4b)
    fn center_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect::from_ptr(ffi::wxRect_CenterIn(self.as_ptr(), r, dir))
        }
    }
    /// Returns true if the given point is inside the rectangle (or on its boundary) and false otherwise.
    ///
    /// [See `wxRect::Contains()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#af9dbfc78b8b630d04e70240c6c5d691b)
    fn contains_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr(), x, y) }
    }
    /// Returns true if the given point is inside the rectangle (or on its boundary) and false otherwise.
    ///
    /// [See `wxRect::Contains()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a742168b406772696f9b491c1b3f1b82a)
    fn contains_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Contains1(self.as_ptr(), pt)
        }
    }
    /// Returns true if the given rectangle is completely inside this rectangle (or touches its boundary) and false otherwise.
    ///
    /// [See `wxRect::Contains()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a200f82b29580067c6052162784608897)
    fn contains_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Contains2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    ///
    /// [See `wxRect::Deflate()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#aa9475a49c56f1e5fac550dbfd0b7a5b8)
    fn deflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRect_Deflate3(self.as_ptr(), dx, dy)) }
    }
    /// Gets the bottom point of the rectangle.
    ///
    /// [See `wxRect::GetBottom()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a362f06d7a036726a31b73464224c4b24)
    fn get_bottom(&self) -> c_int {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr()) }
    }
    /// Gets the position of the bottom left corner.
    ///
    /// [See `wxRect::GetBottomLeft()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a3343305c7af694260f1124b79014f230)
    fn get_bottom_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomLeft(self.as_ptr())) }
    }
    /// Gets the position of the bottom right corner.
    ///
    /// [See `wxRect::GetBottomRight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a63597674ae5446df85e6cd82724495a6)
    fn get_bottom_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomRight(self.as_ptr())) }
    }
    /// Gets the height member.
    ///
    /// [See `wxRect::GetHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a15431ee34e69823f7a5098f8ff3cf929)
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr()) }
    }
    /// Gets the left point of the rectangle (the same as GetX()).
    ///
    /// [See `wxRect::GetLeft()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a903b6b2ea94ce0266b86f4e8e7c88e52)
    fn get_left(&self) -> c_int {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr()) }
    }
    /// Gets the position.
    ///
    /// [See `wxRect::GetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a3b46232998f8c0bf44e3544fe8cbad20)
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetPosition(self.as_ptr())) }
    }
    /// Gets the right point of the rectangle.
    ///
    /// [See `wxRect::GetRight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#af6808c0e7a663411b570867cd8ed98b3)
    fn get_right(&self) -> c_int {
        unsafe { ffi::wxRect_GetRight(self.as_ptr()) }
    }
    /// Gets the size.
    ///
    /// [See `wxRect::GetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a3cc4e113de2759c6b7cdbb3597b728f1)
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxRect_GetSize(self.as_ptr())) }
    }
    /// Gets the top point of the rectangle (the same as GetY()).
    ///
    /// [See `wxRect::GetTop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a6419a3672446e9589d7fb14d24fa867b)
    fn get_top(&self) -> c_int {
        unsafe { ffi::wxRect_GetTop(self.as_ptr()) }
    }
    /// Gets the position of the top left corner of the rectangle, same as GetPosition().
    ///
    /// [See `wxRect::GetTopLeft()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a9be63d55cc56520955f72541c06ce27c)
    fn get_top_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopLeft(self.as_ptr())) }
    }
    /// Gets the position of the top right corner.
    ///
    /// [See `wxRect::GetTopRight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a9a8d3422539928a31772080026d1f855)
    fn get_top_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopRight(self.as_ptr())) }
    }
    /// Gets the width member.
    ///
    /// [See `wxRect::GetWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a45e10fe0f67517b3700cc079520dff31)
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr()) }
    }
    /// Gets the x member.
    ///
    /// [See `wxRect::GetX()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#ad7dc3b867d63e2908710f354baacddc3)
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRect_GetX(self.as_ptr()) }
    }
    /// Gets the y member.
    ///
    /// [See `wxRect::GetY()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a2dc9bc934a126f3568642f826db0cbbb)
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRect_GetY(self.as_ptr()) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    ///
    /// [See `wxRect::Inflate()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#acc9c9e81ad5d6519d6544434ac636ba1)
    fn inflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRect_Inflate3(self.as_ptr(), dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    /// Returns the overlapping portion of this rectangle and the one passed in as parameter.
    ///
    /// [See `wxRect::Intersect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a0b739fa8d09d3d3eed4791372e27f418)
    fn intersect<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect::from_ptr(ffi::wxRect_Intersect1(self.as_ptr(), rect))
        }
    }
    /// Returns true if this rectangle has a non-empty intersection with the rectangle rect and false otherwise.
    ///
    /// [See `wxRect::Intersects()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a66bf0638e9b845e20116f5292c456a27)
    fn intersects<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Intersects(self.as_ptr(), rect)
        }
    }
    /// Returns true if this rectangle has a width or height less than or equal to 0 and false otherwise.
    ///
    /// [See `wxRect::IsEmpty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#ae7cebd6daeb13297ba836239aef945e7)
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr()) }
    }
    /// Moves the rectangle by the specified offset.
    ///
    /// [See `wxRect::Offset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a90f2b9450cf08cf81afe66e502814902)
    fn offset_coord(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxRect_Offset(self.as_ptr(), dx, dy) }
    }
    ///
    /// [See `wxRect::Offset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a7533abc7c86cb6089c64231db9c9c219)
    fn offset_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Offset1(self.as_ptr(), pt)
        }
    }
    /// Sets the height.
    ///
    /// [See `wxRect::SetHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a6a089334755230eedf89e06794571fca)
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr(), height) }
    }
    /// Sets the position.
    ///
    /// [See `wxRect::SetPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a252234c32c36504805cb2bd9f2e3a59e)
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxRect_SetPosition(self.as_ptr(), pos)
        }
    }
    /// Sets the size.
    ///
    /// [See `wxRect::SetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a2e3502710b2987209bde881ac59521fc)
    fn set_size<S: SizeMethods>(&self, s: &S) {
        unsafe {
            let s = s.as_ptr();
            ffi::wxRect_SetSize(self.as_ptr(), s)
        }
    }
    /// Sets the width.
    ///
    /// [See `wxRect::SetWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a653179ac2e602486ecef4f5266b6878b)
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr(), width) }
    }
    /// Sets the x position.
    ///
    /// [See `wxRect::SetX()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a878bff0acba1bb8183db8d1e5d4c4d53)
    fn set_x(&self, x: c_int) {
        unsafe { ffi::wxRect_SetX(self.as_ptr(), x) }
    }
    /// Sets the y position.
    ///
    /// [See `wxRect::SetY()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a55b3573519cf8748d3ec1af9c10cd94d)
    fn set_y(&self, y: c_int) {
        unsafe { ffi::wxRect_SetY(self.as_ptr(), y) }
    }
    /// Set the left side of the rectangle.
    ///
    /// [See `wxRect::SetLeft()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a234b22dbe1ee1a6e9f711492d9e5bce8)
    fn set_left(&self, left: c_int) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr(), left) }
    }
    /// Set the right side of the rectangle.
    ///
    /// [See `wxRect::SetRight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#abc3f50f43628a64c6689889b6e86876e)
    fn set_right(&self, right: c_int) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr(), right) }
    }
    /// Set the top edge of the rectangle.
    ///
    /// [See `wxRect::SetTop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#afaa648c7f822e3504d858e292b889a3d)
    fn set_top(&self, top: c_int) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr(), top) }
    }
    /// Set the bottom edge of the rectangle.
    ///
    /// [See `wxRect::SetBottom()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a1df4b9602d009c0b7b7af7e29bc045ea)
    fn set_bottom(&self, bottom: c_int) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr(), bottom) }
    }
    /// Set the top-left point of the rectangle.
    ///
    /// [See `wxRect::SetTopLeft()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#ae7a6eef4e3d0242077be2b38a4c60927)
    fn set_top_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopLeft(self.as_ptr(), p)
        }
    }
    /// Set the bottom-right point of the rectangle.
    ///
    /// [See `wxRect::SetBottomRight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#ad35b1402cbe77d7feed98d25370fe33d)
    fn set_bottom_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomRight(self.as_ptr(), p)
        }
    }
    /// Set the top-right point of the rectangle.
    ///
    /// [See `wxRect::SetTopRight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#abdb7cdbf0361e4b1af955c4840ebec5a)
    fn set_top_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopRight(self.as_ptr(), p)
        }
    }
    /// Set the bottom-left point of the rectangle.
    ///
    /// [See `wxRect::SetBottomLeft()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a2b8fe9507b3979ec6c930e710f331337)
    fn set_bottom_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomLeft(self.as_ptr(), p)
        }
    }
    /// Modifies the rectangle to contain the bounding box of this rectangle and the one passed in as parameter.
    ///
    /// [See `wxRect::Union()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rect.html#a0647e48228840a668b379933f298a1a8)
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
/// This trait represents C++ [`wxRegion`](https://docs.wxwidgets.org/3.2/classwx_region.html) class's methods and inheritance.
///
/// See [`RegionIsOwned`] documentation for the class usage.
pub trait RegionMethods: GDIObjectMethods {
    // DTOR: fn ~wxRegion()
    /// Clears the current region.
    ///
    /// [See `wxRegion::Clear()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a3e2c4b0910da7fadc36bd6ced199a673)
    fn clear(&self) {
        unsafe { ffi::wxRegion_Clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    // NOT_SUPPORTED: fn Contains2()
    // NOT_SUPPORTED: fn Contains3()
    /// Convert the region to a black and white bitmap with the white pixels being inside the region.
    ///
    /// [See `wxRegion::ConvertToBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a8f7a823bddae015b38c9fc825ffe9c0c)
    fn convert_to_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxRegion_ConvertToBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetBox()
    ///
    /// [See `wxRegion::GetBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a783a4def265fbab702c89d82d60a5469)
    fn get_box(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegion_GetBox1(self.as_ptr())) }
    }
    /// Finds the intersection of this region and another, rectangular region, specified using position and size.
    ///
    /// [See `wxRegion::Intersect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a241f47ba24eab7f10fe6acf2b6c0b7f3)
    fn intersect_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Intersect(self.as_ptr(), x, y, width, height) }
    }
    /// Finds the intersection of this region and another, rectangular region.
    ///
    /// [See `wxRegion::Intersect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a2aaa03bd30c4dad1f3aca89149933a22)
    fn intersect_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Intersect1(self.as_ptr(), rect)
        }
    }
    /// Finds the intersection of this region and another region.
    ///
    /// [See `wxRegion::Intersect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ad94d3469777fc5d8e9246acaa3ca7afd)
    fn intersect_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Intersect2(self.as_ptr(), region)
        }
    }
    /// Returns true if the region is empty, false otherwise.
    ///
    /// [See `wxRegion::IsEmpty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a113466114416705a3fc05305b7137883)
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRegion_IsEmpty(self.as_ptr()) }
    }
    /// Returns true if the region is equal to, i.e. covers the same area as, another one.
    ///
    /// [See `wxRegion::IsEqual()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ab88ad2031a01d2970c7b4a7b06a46e44)
    fn is_equal<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_IsEqual(self.as_ptr(), region)
        }
    }
    /// Moves the region by the specified offsets in horizontal and vertical directions.
    ///
    /// [See `wxRegion::Offset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#abe953e65820362a0796b7bbe5a5ff35d)
    fn offset_coord(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRegion_Offset(self.as_ptr(), x, y) }
    }
    ///
    /// [See `wxRegion::Offset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#afad58d401a561baeff26b2413cef9532)
    fn offset_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRegion_Offset1(self.as_ptr(), pt)
        }
    }
    /// Subtracts a rectangular region from this region.
    ///
    /// [See `wxRegion::Subtract()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a569a77247771375b06dc159a739b79b8)
    fn subtract_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Subtract(self.as_ptr(), rect)
        }
    }
    /// Subtracts a region from this region.
    ///
    /// [See `wxRegion::Subtract()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ac1ed0ca194bc1e9a46d32581a16203e4)
    fn subtract_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Subtract1(self.as_ptr(), region)
        }
    }
    /// Finds the union of this region and another, rectangular region, specified using position and size.
    ///
    /// [See `wxRegion::Union()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ae8ea15425722b296b6cb6974d7306e60)
    fn union_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Union(self.as_ptr(), x, y, width, height) }
    }
    /// Finds the union of this region and another, rectangular region.
    ///
    /// [See `wxRegion::Union()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a4a78ad08c47361b8cfa95dabc54487e8)
    fn union_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Union1(self.as_ptr(), rect)
        }
    }
    /// Finds the union of this region and another region.
    ///
    /// [See `wxRegion::Union()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ac4edc555b9bef0263675b8afefdf0578)
    fn union_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Union2(self.as_ptr(), region)
        }
    }
    /// Finds the union of this region and the non-transparent pixels of a bitmap.
    ///
    /// [See `wxRegion::Union()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a9b8729b4d3acf9e77188f92ebdfaf168)
    fn union_bitmap<B: BitmapMethods>(&self, bmp: &B) -> bool {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxRegion_Union3(self.as_ptr(), bmp)
        }
    }
    /// Finds the union of this region and the non-transparent pixels of a bitmap.
    ///
    /// [See `wxRegion::Union()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ad33ab31d06beebe15b9b7fbd3f2978e9)
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
    ///
    /// [See `wxRegion::Xor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#aafeebc72cda33bc9b1285d1554be9ec3)
    fn xor_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxRegion_Xor(self.as_ptr(), x, y, width, height) }
    }
    /// Finds the Xor of this region and another, rectangular region.
    ///
    /// [See `wxRegion::Xor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#ad9afb74d82763343326a4a60c397ea6e)
    fn xor_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRegion_Xor1(self.as_ptr(), rect)
        }
    }
    /// Finds the Xor of this region and another region.
    ///
    /// [See `wxRegion::Xor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region.html#a908a56969729732832b3c274ffae8071)
    fn xor_region<R: RegionMethods>(&self, region: &R) -> bool {
        unsafe {
            let region = region.as_ptr();
            ffi::wxRegion_Xor2(self.as_ptr(), region)
        }
    }
    // BLOCKED: fn operator=()
}

// wxRegionIterator
/// This trait represents C++ [`wxRegionIterator`](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html) class's methods and inheritance.
///
/// See [`RegionIteratorIsOwned`] documentation for the class usage.
pub trait RegionIteratorMethods: ObjectMethods {
    /// An alias for GetHeight().
    ///
    /// [See `wxRegionIterator::GetH()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#ae671dcc358493ef70f8059128616f0b1)
    fn get_h(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetH(self.as_ptr()) }
    }
    /// Returns the height value for the current region.
    ///
    /// [See `wxRegionIterator::GetHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#adbc1ea9abd548a7f1dbdf8eb03e6b899)
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetHeight(self.as_ptr()) }
    }
    /// Returns the current rectangle.
    ///
    /// [See `wxRegionIterator::GetRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a06ed0d29b8e4349ff7ac0fa6a3f4699f)
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegionIterator_GetRect(self.as_ptr())) }
    }
    /// An alias for GetWidth().
    ///
    /// [See `wxRegionIterator::GetW()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a96345f65ab2f22b8b73ec8f479d73287)
    fn get_w(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetW(self.as_ptr()) }
    }
    /// Returns the width value for the current region.
    ///
    /// [See `wxRegionIterator::GetWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a2757a359b51ec5d3aa7c1c9867295096)
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetWidth(self.as_ptr()) }
    }
    /// Returns the x value for the current region.
    ///
    /// [See `wxRegionIterator::GetX()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a5cb58e87ac2d9df1ef0329dca4c8d90c)
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetX(self.as_ptr()) }
    }
    /// Returns the y value for the current region.
    ///
    /// [See `wxRegionIterator::GetY()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a2bc88470886e165366a7ac0ee70d9c30)
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetY(self.as_ptr()) }
    }
    /// Returns true if there are still some rectangles; otherwise returns false.
    ///
    /// [See `wxRegionIterator::HaveRects()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a042beb24aec97f34600ee2181245a199)
    fn have_rects(&self) -> bool {
        unsafe { ffi::wxRegionIterator_HaveRects(self.as_ptr()) }
    }
    /// Resets the iterator to the beginning of the rectangles.
    ///
    /// [See `wxRegionIterator::Reset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#ac2facb5b4ea2c8188c43474350207fea)
    fn reset(&self) {
        unsafe { ffi::wxRegionIterator_Reset(self.as_ptr()) }
    }
    /// Resets the iterator to the given region.
    ///
    /// [See `wxRegionIterator::Reset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a147ea0f4fcce9b5662925e6b6ffc92f3)
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
/// This trait represents C++ [`wxRendererNative`](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html) class's methods and inheritance.
///
/// See [`RendererNativeIsOwned`] documentation for the class usage.
pub trait RendererNativeMethods: WxRustMethods {
    // DTOR: fn ~wxRendererNative()
    /// Draw a check box.
    ///
    /// [See `wxRendererNative::DrawCheckBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#af5a80f13e6b7a20affd26b0becd9856b)
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
    ///
    /// [See `wxRendererNative::DrawComboBoxDropButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a9495db526d637f854a3436e1d17bb472)
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
    ///
    /// [See `wxRendererNative::DrawDropArrow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a0c16016c310cec4d3d2f53574741df05)
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
    ///
    /// [See `wxRendererNative::DrawFocusRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a365bc24e598ce64e3a8d2b9559a8c4c9)
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
    ///
    /// [See `wxRendererNative::DrawGauge()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#ab28f91fa303ae5c9786b399ec5e5cb2e)
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
    ///
    /// [See `wxRendererNative::DrawItemSelectionRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a5d0ab3cbc0dbcf90a91a4e6ed400ff8a)
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
    ///
    /// [See `wxRendererNative::DrawItemText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#af1a78ca9f745650102ae35ac0e53fe3e)
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
    ///
    /// [See `wxRendererNative::DrawPushButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a866a6e77f5f302e7fbb50819268de331)
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
    ///
    /// [See `wxRendererNative::DrawCollapseButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#ae30bcfb0f1598e8a53a0713ae4d234e7)
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
    ///
    /// [See `wxRendererNative::GetCollapseButtonSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a31951fd60a2f0ba94083f03c1179a118)
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
    ///
    /// [See `wxRendererNative::DrawSplitterBorder()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a42ec07b63b3c38ab7309fe87932cfcb3)
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
    ///
    /// [See `wxRendererNative::DrawTreeItemButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a2b66a74d44ffc6a36a8bd1e15c83e77c)
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
    ///
    /// [See `wxRendererNative::DrawChoice()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a54bc38596a1ff8f37aa2a4f97d208a2d)
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
    ///
    /// [See `wxRendererNative::DrawComboBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#ab9e724dd0441e511a75aee96fb7ddcd2)
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
    ///
    /// [See `wxRendererNative::DrawTextCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a35e49075d3a7e748953660101252a892)
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
    ///
    /// [See `wxRendererNative::DrawRadioBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#af62a5a4bd85f1d2f26ae72d5feb4469c)
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
    ///
    /// [See `wxRendererNative::DrawCheckMark()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#aac01bd3904fd4e79d1d58359e71fe654)
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
    ///
    /// [See `wxRendererNative::GetCheckBoxSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#ada3f575549bf73edce0f60d2251a6fe3)
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
    ///
    /// [See `wxRendererNative::GetCheckMarkSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#aea79135c845ed05f16ceddab22fcf48b)
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
    ///
    /// [See `wxRendererNative::GetExpanderSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a10a08ecbe1cc4044d0382ff718d1b503)
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
    ///
    /// [See `wxRendererNative::GetHeaderButtonHeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a7318ee160cf6810d5fbcfa3383a0562f)
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
    ///
    /// [See `wxRendererNative::GetHeaderButtonMargin()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#afede2d2f77c54cd443b1be09dbe2f9e4)
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
    ///
    /// [See `wxRendererNative::Get()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a9526c5b65e880112597ce89f82f7c79a)
    fn get() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_Get()) }
    }
    /// Return the default (native) implementation for this platform  this is also the one used by default but this may be changed by calling Set() in which case the return value of this method may be different from the return value of Get().
    ///
    /// [See `wxRendererNative::GetDefault()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#ab0da0d81fc83357893336e9c6789f2e9)
    fn get_default() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetDefault()) }
    }
    /// Return the generic implementation of the renderer.
    ///
    /// [See `wxRendererNative::GetGeneric()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a30c27eccbf079dbab0b5c51c909d423b)
    fn get_generic() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetGeneric()) }
    }
    /// Load the renderer from the specified DLL, the returned pointer must be deleted by caller if not NULL when it is not used any more.
    ///
    /// [See `wxRendererNative::Load()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#a3370fada68174ae45bbf1ce52757bf0b)
    fn load(name: &str) -> Option<RendererNativeIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            RendererNative::option_from(ffi::wxRendererNative_Load(name))
        }
    }
    /// Set the renderer to use, passing NULL reverts to using the default renderer (the global renderer must always exist).
    ///
    /// [See `wxRendererNative::Set()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html#afc00cb4f831444446a275bf990448efe)
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
/// This trait represents C++ [`wxRichToolTip`](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html) class's methods and inheritance.
///
/// See [`RichToolTipIsOwned`] documentation for the class usage.
pub trait RichToolTipMethods: WxRustMethods {
    /// Set the background colour.
    ///
    /// [See `wxRichToolTip::SetBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html#abc2ac6aff7ca4404622b8e5a24a7d6df)
    fn set_background_colour<C: ColourMethods, C2: ColourMethods>(&self, col: &C, col_end: &C2) {
        unsafe {
            let col = col.as_ptr();
            let col_end = col_end.as_ptr();
            ffi::wxRichToolTip_SetBackgroundColour(self.as_ptr(), col, col_end)
        }
    }
    /// Set the small icon to show.
    ///
    /// [See `wxRichToolTip::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html#a51e540bf634f3e0fdd457f49abb11021)
    fn set_icon_int(&self, icon: c_int) {
        unsafe { ffi::wxRichToolTip_SetIcon(self.as_ptr(), icon) }
    }
    ///
    /// [See `wxRichToolTip::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html#a88881abdea0691e6151da6a7f081533e)
    fn set_icon_bitmapbundle<B: BitmapBundleMethods>(&self, icon: &B) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxRichToolTip_SetIcon1(self.as_ptr(), icon)
        }
    }
    // NOT_SUPPORTED: fn SetTimeout()
    // NOT_SUPPORTED: fn SetTipKind()
    /// Set the title text font.
    ///
    /// [See `wxRichToolTip::SetTitleFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html#a3268339723ba6bf65c0eb3809b9914eb)
    fn set_title_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxRichToolTip_SetTitleFont(self.as_ptr(), font)
        }
    }
    /// Show the tooltip for the given window and optionally specify where to show the tooltip.
    ///
    /// [See `wxRichToolTip::ShowFor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html#a29712741409510978cbddec974c9b87b)
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
