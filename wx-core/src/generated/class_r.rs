use super::*;

// wxRadioBox
wxwidgets! {
    /// A radio box item is used to select one of number of mutually exclusive choices.
    #[doc(alias = "wxRadioBox")]
    #[doc(alias = "RadioBox")]
    class RadioBox
        = RadioBoxIsOwned<true>(wxRadioBox) impl
        RadioBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RadioBoxIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> RadioBoxIsOwned<OWNED> {
        unsafe { RadioBoxIsOwned(ffi::wxRadioBox_new()) }
    }
    // NOT_SUPPORTED: fn wxRadioBox1()
    /// Constructor, creating and showing a radiobox.
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
impl<const OWNED: bool> Clone for RadioBoxIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A radio button item is a button which usually denotes one of several mutually exclusive options.
    #[doc(alias = "wxRadioButton")]
    #[doc(alias = "RadioButton")]
    class RadioButton
        = RadioButtonIsOwned<true>(wxRadioButton) impl
        RadioButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RadioButtonIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> RadioButtonIsOwned<OWNED> {
        unsafe { RadioButtonIsOwned(ffi::wxRadioButton_new()) }
    }
    /// Constructor, creating and showing a radio button.
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
impl<const OWNED: bool> Clone for RadioButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A wxRealPoint is a useful data structure for graphics operations.
    #[doc(alias = "wxRealPoint")]
    #[doc(alias = "RealPoint")]
    class RealPoint
        = RealPointIsOwned<true>(wxRealPoint) impl
        RealPointMethods
}
impl<const OWNED: bool> RealPointIsOwned<OWNED> {
    /// Initializes to zero the x and y members.
    pub fn new() -> RealPointIsOwned<OWNED> {
        unsafe { RealPointIsOwned(ffi::wxRealPoint_new()) }
    }
    /// Initializes the point with the given coordinates.
    pub fn new_with_double(x: c_double, y: c_double) -> RealPointIsOwned<OWNED> {
        unsafe { RealPointIsOwned(ffi::wxRealPoint_new1(x, y)) }
    }
    /// Converts the given wxPoint (with integer coordinates) to a wxRealPoint.
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
impl Clone for RealPointIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A composite control containing a wxRearrangeList and the buttons allowing to move the items in it.
    #[doc(alias = "wxRearrangeCtrl")]
    #[doc(alias = "RearrangeCtrl")]
    class RearrangeCtrl
        = RearrangeCtrlIsOwned<true>(wxRearrangeCtrl) impl
        RearrangeCtrlMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeCtrlIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> RearrangeCtrlIsOwned<OWNED> {
        unsafe { RearrangeCtrlIsOwned(ffi::wxRearrangeCtrl_new()) }
    }
    /// Constructor really creating the control.
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
impl<const OWNED: bool> Clone for RearrangeCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A dialog allowing the user to rearrange the specified items.
    #[doc(alias = "wxRearrangeDialog")]
    #[doc(alias = "RearrangeDialog")]
    class RearrangeDialog
        = RearrangeDialogIsOwned<true>(wxRearrangeDialog) impl
        RearrangeDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeDialogIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> RearrangeDialogIsOwned<OWNED> {
        unsafe { RearrangeDialogIsOwned(ffi::wxRearrangeDialog_new()) }
    }
    /// Constructor creating the dialog.
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
impl<const OWNED: bool> Clone for RearrangeDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A listbox-like control allowing the user to rearrange the items and to enable or disable them.
    #[doc(alias = "wxRearrangeList")]
    #[doc(alias = "RearrangeList")]
    class RearrangeList
        = RearrangeListIsOwned<true>(wxRearrangeList) impl
        RearrangeListMethods,
        CheckListBoxMethods,
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> RearrangeListIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> RearrangeListIsOwned<OWNED> {
        unsafe { RearrangeListIsOwned(ffi::wxRearrangeList_new()) }
    }
    /// Constructor really creating the control.
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
impl<const OWNED: bool> Clone for RearrangeListIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// Represents a rectangle with integer coordinates.
    #[doc(alias = "wxRect")]
    #[doc(alias = "Rect")]
    class Rect
        = RectIsOwned<true>(wxRect) impl
        RectMethods
}
impl<const OWNED: bool> RectIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new()) }
    }
    /// Creates a wxRect object from x, y, width and height values.
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectIsOwned<OWNED> {
        unsafe { RectIsOwned(ffi::wxRect_new1(x, y, width, height)) }
    }
    /// Creates a wxRect object from top-left and bottom-right points.
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
    /// Creates a wxRect object from position pos and size values.
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
    /// Creates a wxRect object from size values at the origin.
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
impl Clone for RectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for RectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRect_delete(self.0) }
        }
    }
}

// wxRegion
wxwidgets! {
    /// A wxRegion represents a simple or complex region on a device context or window.
    #[doc(alias = "wxRegion")]
    #[doc(alias = "Region")]
    class Region
        = RegionIsOwned<true>(wxRegion) impl
        RegionMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> RegionIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> RegionIsOwned<OWNED> {
        unsafe { RegionIsOwned(ffi::wxRegion_new()) }
    }
    /// Constructs a rectangular region with the given position and size.
    pub fn new_with_coord(x: c_int, y: c_int, width: c_int, height: c_int) -> RegionIsOwned<OWNED> {
        unsafe { RegionIsOwned(ffi::wxRegion_new1(x, y, width, height)) }
    }
    /// Constructs a rectangular region from the top left point and the bottom right point.
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
    /// Constructs a rectangular region a wxRect object.
    pub fn new_with_rect<R: RectMethods>(rect: &R) -> RegionIsOwned<OWNED> {
        unsafe {
            let rect = rect.as_ptr();
            RegionIsOwned(ffi::wxRegion_new3(rect))
        }
    }
    /// Copy constructor, uses Reference Counting.
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionIsOwned<OWNED> {
        unsafe {
            let region = region.as_ptr();
            RegionIsOwned(ffi::wxRegion_new4(region))
        }
    }
    // NOT_SUPPORTED: fn wxRegion5()
    /// Constructs a region using a bitmap.
    pub fn new_with_bitmap<B: BitmapMethods>(bmp: &B) -> RegionIsOwned<OWNED> {
        unsafe {
            let bmp = bmp.as_ptr();
            RegionIsOwned(ffi::wxRegion_new6(bmp))
        }
    }
    /// Constructs a region using the non-transparent pixels of a bitmap.
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
impl Clone for RegionIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This class is used to iterate through the rectangles in a region, typically when examining the damaged regions of a window within an OnPaint call.
    #[doc(alias = "wxRegionIterator")]
    #[doc(alias = "RegionIterator")]
    class RegionIterator
        = RegionIteratorIsOwned<true>(wxRegionIterator) impl
        RegionIteratorMethods,
        ObjectMethods
}
impl<const OWNED: bool> RegionIteratorIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> RegionIteratorIsOwned<OWNED> {
        unsafe { RegionIteratorIsOwned(ffi::wxRegionIterator_new()) }
    }
    /// Creates an iterator object given a region.
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
impl Clone for RegionIteratorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// First, a brief introduction to wxRendererNative and why it is needed.
    #[doc(alias = "wxRendererNative")]
    #[doc(alias = "RendererNative")]
    class RendererNative
        = RendererNativeIsOwned<true>(wxRendererNative) impl
        RendererNativeMethods
}
impl<const OWNED: bool> RendererNativeIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RendererNativeIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// Allows showing a tool tip with more customizations than wxToolTip.
    #[doc(alias = "wxRichToolTip")]
    #[doc(alias = "RichToolTip")]
    class RichToolTip
        = RichToolTipIsOwned<true>(wxRichToolTip) impl
        RichToolTipMethods
}
impl<const OWNED: bool> RichToolTipIsOwned<OWNED> {
    /// Constructor must specify the tooltip title and main message.
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
impl Clone for RichToolTipIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for RichToolTipIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRichToolTip_delete(self.0) }
        }
    }
}
