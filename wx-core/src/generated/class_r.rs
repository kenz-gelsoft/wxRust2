#![allow(non_upper_case_globals)]

use super::*;

// wxRadioBox
wx_class! { RadioBox =
    RadioBoxIsOwned<true>(wxRadioBox) impl
        RadioBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RadioBoxIsOwned<OWNED> {
    pub fn new_2step() -> RadioBoxIsOwned<OWNED> {
        unsafe { RadioBoxIsOwned(ffi::wxRadioBox_new()) }
    }
    // NOT_SUPPORTED: fn wxRadioBox1()
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
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
    ) -> RadioBoxIsOwned<OWNED> {
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
            RadioBoxIsOwned(ffi::wxRadioBox_new2(
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
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RadioBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RadioBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRadioBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRadioBox
impl<const OWNED: bool> ItemContainerImmutableMethods for RadioBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRadioBox_AsItemContainerImmutable(self.as_ptr()) }
    }
}

// wxRadioButton
wx_class! { RadioButton =
    RadioButtonIsOwned<true>(wxRadioButton) impl
        RadioButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RadioButtonIsOwned<OWNED> {
    pub fn new_2step() -> RadioButtonIsOwned<OWNED> {
        unsafe { RadioButtonIsOwned(ffi::wxRadioButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RadioButtonIsOwned<OWNED> {
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
            RadioButtonIsOwned(ffi::wxRadioButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RadioButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RadioButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RadioButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRadioButton_CLASSINFO()) }
    }
}

// wxRealPoint
wx_class! { RealPoint =
    RealPointIsOwned<true>(wxRealPoint) impl
        RealPointMethods
}
impl<const OWNED: bool> RealPointIsOwned<OWNED> {
    pub fn new() -> RealPointIsOwned<OWNED> {
        unsafe { RealPointIsOwned(ffi::wxRealPoint_new()) }
    }
    pub fn new_with_double(x: c_double, y: c_double) -> RealPointIsOwned<OWNED> {
        unsafe { RealPointIsOwned(ffi::wxRealPoint_new1(x, y)) }
    }
    pub fn new_with_point<P: PointMethods>(pt: &P) -> RealPointIsOwned<OWNED> {
        unsafe {
            let pt = pt.as_ptr();
            RealPointIsOwned(ffi::wxRealPoint_new2(pt))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RealPointIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRealPoint_delete(self.0) }
        }
    }
}

// wxRearrangeCtrl
wx_class! { RearrangeCtrl =
    RearrangeCtrlIsOwned<true>(wxRearrangeCtrl) impl
        RearrangeCtrlMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeCtrlIsOwned<OWNED> {
    pub fn new_2step() -> RearrangeCtrlIsOwned<OWNED> {
        unsafe { RearrangeCtrlIsOwned(ffi::wxRearrangeCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RearrangeCtrlIsOwned<OWNED> {
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
            RearrangeCtrlIsOwned(ffi::wxRearrangeCtrl_new1(
                parent, id, pos, size, order, items, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RearrangeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RearrangeCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRearrangeCtrl_CLASSINFO()) }
    }
}

// wxRearrangeDialog
wx_class! { RearrangeDialog =
    RearrangeDialogIsOwned<true>(wxRearrangeDialog) impl
        RearrangeDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeDialogIsOwned<OWNED> {
    pub fn new_2step() -> RearrangeDialogIsOwned<OWNED> {
        unsafe { RearrangeDialogIsOwned(ffi::wxRearrangeDialog_new()) }
    }
    pub fn new<W: WindowMethods, A: ArrayIntMethods, A2: ArrayStringMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        title: &str,
        order: &A,
        items: &A2,
        pos: &P,
        name: &str,
    ) -> RearrangeDialogIsOwned<OWNED> {
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
            RearrangeDialogIsOwned(ffi::wxRearrangeDialog_new1(
                parent, message, title, order, items, pos, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RearrangeDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RearrangeDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRearrangeDialog_CLASSINFO()) }
    }
}

// wxRearrangeList
wx_class! { RearrangeList =
    RearrangeListIsOwned<true>(wxRearrangeList) impl
        RearrangeListMethods,
        CheckListBoxMethods,
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeListIsOwned<OWNED> {
    pub fn new_2step() -> RearrangeListIsOwned<OWNED> {
        unsafe { RearrangeListIsOwned(ffi::wxRearrangeList_new()) }
    }
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RearrangeListIsOwned<OWNED> {
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
            RearrangeListIsOwned(ffi::wxRearrangeList_new1(
                parent, id, pos, size, order, items, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for CheckListBoxIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for ListBoxIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RearrangeListIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RearrangeListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RearrangeListIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRearrangeList_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRearrangeList
impl<const OWNED: bool> ItemContainerMethods for RearrangeListIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for RearrangeListIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsItemContainer(self.as_ptr()) }
    }
}

// wxRect
wx_class! { Rect =
    RectIsOwned<true>(wxRect) impl
        RectMethods
}
impl<const OWNED: bool> RectIsOwned<OWNED> {
    pub fn new() -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new_with_point_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RectIsOwned(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_point_size<P: PointMethods, S: SizeMethods>(
        pos: &P,
        size: &S,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new_with_size<S: SizeMethods>(size: &S) -> RectIsOwned<OWNED> {
        unsafe {
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRect_delete(self.0) }
        }
    }
}

// wxRect
wx_class! { Rect =
    RectIsOwned<true>(wxRect) impl

        // RectMethods
}
impl<const OWNED: bool> RectIsOwned<OWNED> {
    pub fn new() -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new_with_point_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RectIsOwned(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_point_size<P: PointMethods, S: SizeMethods>(
        pos: &P,
        size: &S,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new_with_size<S: SizeMethods>(size: &S) -> RectIsOwned<OWNED> {
        unsafe {
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRect_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> RectMethods for RectIsOwned<OWNED> {
    pub fn new() -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new()) }
    }
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new_with_point_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RectIsOwned(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_point_size<P: PointMethods, S: SizeMethods>(
        pos: &P,
        size: &S,
    ) -> RectIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new_with_size<S: SizeMethods>(size: &S) -> RectIsOwned<OWNED> {
        unsafe {
            let size = size.as_ptr();
            RectIsOwned(ffi::wxRect_new4(size))
        }
    }
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

// wxRegion
wx_class! { Region =
    RegionIsOwned<true>(wxRegion) impl
        RegionMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> RegionIsOwned<OWNED> {
    pub fn new() -> RegionIsOwned<OWNED> {
        unsafe { RegionIsOwned(ffi::wxRegion_new()) }
    }
    pub fn new_with_coord(x: c_int, y: c_int, width: c_int, height: c_int) -> RegionIsOwned<OWNED> {
        unsafe { RegionIsOwned(ffi::wxRegion_new1(x, y, width, height)) }
    }
    pub fn new_with_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RegionIsOwned<OWNED> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RegionIsOwned(ffi::wxRegion_new2(top_left, bottom_right))
        }
    }
    pub fn new_with_rect<R: RectMethods>(rect: &R) -> RegionIsOwned<OWNED> {
        unsafe {
            let rect = rect.as_ptr();
            RegionIsOwned(ffi::wxRegion_new3(rect))
        }
    }
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionIsOwned<OWNED> {
        unsafe {
            let region = region.as_ptr();
            RegionIsOwned(ffi::wxRegion_new4(region))
        }
    }
    // NOT_SUPPORTED: fn wxRegion5()
    pub fn new_with_bitmap<B: BitmapMethods>(bmp: &B) -> RegionIsOwned<OWNED> {
        unsafe {
            let bmp = bmp.as_ptr();
            RegionIsOwned(ffi::wxRegion_new6(bmp))
        }
    }
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bmp: &B,
        trans_colour: &C,
        tolerance: c_int,
    ) -> RegionIsOwned<OWNED> {
        unsafe {
            let bmp = bmp.as_ptr();
            let trans_colour = trans_colour.as_ptr();
            RegionIsOwned(ffi::wxRegion_new7(bmp, trans_colour, tolerance))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RegionIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: RegionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<RegionIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RegionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RegionIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRegion_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for RegionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRegionIterator
wx_class! { RegionIterator =
    RegionIteratorIsOwned<true>(wxRegionIterator) impl
        RegionIteratorMethods,
        ObjectMethods
}
impl<const OWNED: bool> RegionIteratorIsOwned<OWNED> {
    pub fn new() -> RegionIteratorIsOwned<OWNED> {
        unsafe { RegionIteratorIsOwned(ffi::wxRegionIterator_new()) }
    }
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionIteratorIsOwned<OWNED> {
        unsafe {
            let region = region.as_ptr();
            RegionIteratorIsOwned(ffi::wxRegionIterator_new1(region))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<RegionIteratorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: RegionIteratorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for RegionIteratorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxRegionIterator_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for RegionIteratorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRendererNative
wx_class! { RendererNative =
    RendererNativeIsOwned<true>(wxRendererNative) impl
        RendererNativeMethods
}
impl<const OWNED: bool> RendererNativeIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RendererNativeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRendererNative_delete(self.0) }
        }
    }
}

// wxRichToolTip
wx_class! { RichToolTip =
    RichToolTipIsOwned<true>(wxRichToolTip) impl
        RichToolTipMethods
}
impl<const OWNED: bool> RichToolTipIsOwned<OWNED> {
    pub fn new(title: &str, message: &str) -> RichToolTipIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            RichToolTipIsOwned(ffi::wxRichToolTip_new(title, message))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RichToolTipIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRichToolTip_delete(self.0) }
        }
    }
}
