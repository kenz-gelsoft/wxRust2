use super::*;

// wxRadioBox
wxwidgets! {
    /// A radio box item is used to select one of number of mutually exclusive choices.
    /// - [`RadioBox`] represents a C++ `wxRadioBox` class instance which your code has ownership, [`RadioBoxFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RadioBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRadioBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_radio_box.html) for more details.
    #[doc(alias = "wxRadioBox")]
    #[doc(alias = "RadioBox")]
    class RadioBox
        = RadioBoxFromCpp<true>(wxRadioBox) impl
        RadioBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RadioBoxFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRadioBox::wxRadioBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a2821fb98e7a2b2f52b0966c784bf4864).
    pub fn new_2step() -> RadioBoxFromCpp<FROM_CPP> {
        unsafe { RadioBoxFromCpp(ffi::wxRadioBox_new()) }
    }
    // NOT_SUPPORTED: fn wxRadioBox1()
    /// Constructor, creating and showing a radiobox.
    ///
    /// See [C++ `wxRadioBox::wxRadioBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_radio_box.html#a4bf8c8ac111aa1f4a6657458e49aa152).
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
    ) -> RadioBoxFromCpp<FROM_CPP> {
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
            RadioBoxFromCpp(ffi::wxRadioBox_new2(
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
impl<const FROM_CPP: bool> Clone for RadioBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RadioBoxFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: RadioBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RadioBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: RadioBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RadioBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: RadioBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RadioBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RadioBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RadioBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRadioBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRadioBox
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for RadioBoxFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRadioBox_AsItemContainerImmutable(self.as_ptr()) }
    }
}

// wxRadioButton
wxwidgets! {
    /// A radio button item is a button which usually denotes one of several mutually exclusive options.
    /// - [`RadioButton`] represents a C++ `wxRadioButton` class instance which your code has ownership, [`RadioButtonFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RadioButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRadioButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_radio_button.html) for more details.
    #[doc(alias = "wxRadioButton")]
    #[doc(alias = "RadioButton")]
    class RadioButton
        = RadioButtonFromCpp<true>(wxRadioButton) impl
        RadioButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RadioButtonFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRadioButton::wxRadioButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#a295e680547c57d9ad5bfbc835770ed2b).
    pub fn new_2step() -> RadioButtonFromCpp<FROM_CPP> {
        unsafe { RadioButtonFromCpp(ffi::wxRadioButton_new()) }
    }
    /// Constructor, creating and showing a radio button.
    ///
    /// See [C++ `wxRadioButton::wxRadioButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_radio_button.html#ab4b4b8a8b3532558204dec6b34396bb2).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> RadioButtonFromCpp<FROM_CPP> {
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
            RadioButtonFromCpp(ffi::wxRadioButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for RadioButtonFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RadioButtonFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: RadioButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RadioButtonFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: RadioButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RadioButtonFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: RadioButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RadioButtonFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RadioButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RadioButtonFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRadioButton_CLASSINFO()) }
    }
}

// wxRealPoint
wxwidgets! {
    /// A wxRealPoint is a useful data structure for graphics operations.
    /// - [`RealPoint`] represents a C++ `wxRealPoint` class instance which your code has ownership, [`RealPointFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RealPoint`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRealPoint` class's documentation](https://docs.wxwidgets.org/3.2/classwx_real_point.html) for more details.
    #[doc(alias = "wxRealPoint")]
    #[doc(alias = "RealPoint")]
    class RealPoint
        = RealPointFromCpp<true>(wxRealPoint) impl
        RealPointMethods
}
impl<const FROM_CPP: bool> RealPointFromCpp<FROM_CPP> {
    /// Initializes to zero the x and y members.
    ///
    /// See [C++ `wxRealPoint::wxRealPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_real_point.html#a9c4d38e144bb23d0e5ce94f7653e7887).
    pub fn new() -> RealPointFromCpp<FROM_CPP> {
        unsafe { RealPointFromCpp(ffi::wxRealPoint_new()) }
    }
    /// Initializes the point with the given coordinates.
    ///
    /// See [C++ `wxRealPoint::wxRealPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_real_point.html#a672ffa73a517579bfee8c32198c5d948).
    pub fn new_with_double(x: c_double, y: c_double) -> RealPointFromCpp<FROM_CPP> {
        unsafe { RealPointFromCpp(ffi::wxRealPoint_new1(x, y)) }
    }
    /// Converts the given wxPoint (with integer coordinates) to a wxRealPoint.
    ///
    /// See [C++ `wxRealPoint::wxRealPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_real_point.html#ae09522b031b035db4472500e48a0e2d3).
    pub fn new_with_point<P: PointMethods>(pt: &P) -> RealPointFromCpp<FROM_CPP> {
        unsafe {
            let pt = pt.as_ptr();
            RealPointFromCpp(ffi::wxRealPoint_new2(pt))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RealPointFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for RealPointFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxRealPoint_delete(self.0) }
        }
    }
}

// wxRearrangeCtrl
wxwidgets! {
    /// A composite control containing a wxRearrangeList and the buttons allowing to move the items in it.
    /// - [`RearrangeCtrl`] represents a C++ `wxRearrangeCtrl` class instance which your code has ownership, [`RearrangeCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RearrangeCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRearrangeCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_ctrl.html) for more details.
    #[doc(alias = "wxRearrangeCtrl")]
    #[doc(alias = "RearrangeCtrl")]
    class RearrangeCtrl
        = RearrangeCtrlFromCpp<true>(wxRearrangeCtrl) impl
        RearrangeCtrlMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RearrangeCtrlFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRearrangeCtrl::wxRearrangeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_ctrl.html#ad47b3bbeddb36e799feb7d463376c929).
    pub fn new_2step() -> RearrangeCtrlFromCpp<FROM_CPP> {
        unsafe { RearrangeCtrlFromCpp(ffi::wxRearrangeCtrl_new()) }
    }
    /// Constructor really creating the control.
    ///
    /// See [C++ `wxRearrangeCtrl::wxRearrangeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_ctrl.html#a20803f2a9c123596b055fafd90ecff6a).
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
    ) -> RearrangeCtrlFromCpp<FROM_CPP> {
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
            RearrangeCtrlFromCpp(ffi::wxRearrangeCtrl_new1(
                parent, id, pos, size, order, items, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for RearrangeCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RearrangeCtrlFromCpp<FROM_CPP>> for PanelFromCpp<FROM_CPP> {
    fn from(o: RearrangeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: RearrangeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: RearrangeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RearrangeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RearrangeCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRearrangeCtrl_CLASSINFO()) }
    }
}

// wxRearrangeDialog
wxwidgets! {
    /// A dialog allowing the user to rearrange the specified items.
    /// - [`RearrangeDialog`] represents a C++ `wxRearrangeDialog` class instance which your code has ownership, [`RearrangeDialogFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RearrangeDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRearrangeDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html) for more details.
    #[doc(alias = "wxRearrangeDialog")]
    #[doc(alias = "RearrangeDialog")]
    class RearrangeDialog
        = RearrangeDialogFromCpp<true>(wxRearrangeDialog) impl
        RearrangeDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RearrangeDialogFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRearrangeDialog::wxRearrangeDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html#ad230c6d3baf27e7d3f516a37e62e0d58).
    pub fn new_2step() -> RearrangeDialogFromCpp<FROM_CPP> {
        unsafe { RearrangeDialogFromCpp(ffi::wxRearrangeDialog_new()) }
    }
    /// Constructor creating the dialog.
    ///
    /// See [C++ `wxRearrangeDialog::wxRearrangeDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_dialog.html#ad91c63bf702f87b4727994d255743719).
    pub fn new<W: WindowMethods, A: ArrayIntMethods, A2: ArrayStringMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        title: &str,
        order: &A,
        items: &A2,
        pos: &P,
        name: &str,
    ) -> RearrangeDialogFromCpp<FROM_CPP> {
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
            RearrangeDialogFromCpp(ffi::wxRearrangeDialog_new1(
                parent, message, title, order, items, pos, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for RearrangeDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RearrangeDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: RearrangeDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeDialogFromCpp<FROM_CPP>>
    for TopLevelWindowFromCpp<FROM_CPP>
{
    fn from(o: RearrangeDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeDialogFromCpp<FROM_CPP>>
    for NonOwnedWindowFromCpp<FROM_CPP>
{
    fn from(o: RearrangeDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: RearrangeDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: RearrangeDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RearrangeDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RearrangeDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRearrangeDialog_CLASSINFO()) }
    }
}

// wxRearrangeList
wxwidgets! {
    /// A listbox-like control allowing the user to rearrange the items and to enable or disable them.
    /// - [`RearrangeList`] represents a C++ `wxRearrangeList` class instance which your code has ownership, [`RearrangeListFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RearrangeList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRearrangeList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html) for more details.
    #[doc(alias = "wxRearrangeList")]
    #[doc(alias = "RearrangeList")]
    class RearrangeList
        = RearrangeListFromCpp<true>(wxRearrangeList) impl
        RearrangeListMethods,
        CheckListBoxMethods,
        ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RearrangeListFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRearrangeList::wxRearrangeList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#ab211f32339f81867de12d14dc962d76c).
    pub fn new_2step() -> RearrangeListFromCpp<FROM_CPP> {
        unsafe { RearrangeListFromCpp(ffi::wxRearrangeList_new()) }
    }
    /// Constructor really creating the control.
    ///
    /// See [C++ `wxRearrangeList::wxRearrangeList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rearrange_list.html#a3cf5cbfc0a24b5a58cd3683c80d303ba).
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
    ) -> RearrangeListFromCpp<FROM_CPP> {
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
            RearrangeListFromCpp(ffi::wxRearrangeList_new1(
                parent, id, pos, size, order, items, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for RearrangeListFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RearrangeListFromCpp<FROM_CPP>> for CheckListBoxFromCpp<FROM_CPP> {
    fn from(o: RearrangeListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeListFromCpp<FROM_CPP>> for ListBoxFromCpp<FROM_CPP> {
    fn from(o: RearrangeListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeListFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: RearrangeListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeListFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: RearrangeListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeListFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: RearrangeListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RearrangeListFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RearrangeListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RearrangeListFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRearrangeList_CLASSINFO()) }
    }
}
// Mix-in(s) to wxRearrangeList
impl<const FROM_CPP: bool> ItemContainerMethods for RearrangeListFromCpp<FROM_CPP> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for RearrangeListFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxRearrangeList_AsItemContainer(self.as_ptr()) }
    }
}

// wxRect
wxwidgets! {
    /// Represents a rectangle with integer coordinates.
    /// - [`Rect`] represents a C++ `wxRect` class instance which your code has ownership, [`RectFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Rect`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRect` class's documentation](https://docs.wxwidgets.org/3.2/classwx_rect.html) for more details.
    #[doc(alias = "wxRect")]
    #[doc(alias = "Rect")]
    class Rect
        = RectFromCpp<true>(wxRect) impl
        RectMethods
}
impl<const FROM_CPP: bool> RectFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRect::wxRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rect.html#a19b68679edc9030cba102a8602febded).
    pub fn new() -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::wxRect_new()) }
    }
    /// Creates a wxRect object from x, y, width and height values.
    ///
    /// See [C++ `wxRect::wxRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rect.html#aefb674bdc6d9d66a5c3746e5acc845d4).
    pub fn new_with_int(x: c_int, y: c_int, width: c_int, height: c_int) -> RectFromCpp<FROM_CPP> {
        unsafe { RectFromCpp(ffi::wxRect_new1(x, y, width, height)) }
    }
    /// Creates a wxRect object from top-left and bottom-right points.
    ///
    /// See [C++ `wxRect::wxRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rect.html#a8d62df83787d4e1acdcf7a12fc97e5e6).
    pub fn new_with_point_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RectFromCpp<FROM_CPP> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RectFromCpp(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    /// Creates a wxRect object from position pos and size values.
    ///
    /// See [C++ `wxRect::wxRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rect.html#ae46a17423adfbb2faa21020b7cab8bc8).
    pub fn new_with_point_size<P: PointMethods, S: SizeMethods>(
        pos: &P,
        size: &S,
    ) -> RectFromCpp<FROM_CPP> {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            RectFromCpp(ffi::wxRect_new3(pos, size))
        }
    }
    /// Creates a wxRect object from size values at the origin.
    ///
    /// See [C++ `wxRect::wxRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rect.html#a806ffe715a971157753890bab59d7d16).
    pub fn new_with_size<S: SizeMethods>(size: &S) -> RectFromCpp<FROM_CPP> {
        unsafe {
            let size = size.as_ptr();
            RectFromCpp(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RectFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for RectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxRect_delete(self.0) }
        }
    }
}

// wxRegion
wxwidgets! {
    /// A wxRegion represents a simple or complex region on a device context or window.
    /// - [`Region`] represents a C++ `wxRegion` class instance which your code has ownership, [`RegionFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Region`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRegion` class's documentation](https://docs.wxwidgets.org/3.2/classwx_region.html) for more details.
    #[doc(alias = "wxRegion")]
    #[doc(alias = "Region")]
    class Region
        = RegionFromCpp<true>(wxRegion) impl
        RegionMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RegionFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#a1edc6768118cf02749b46774a0ca37f9).
    pub fn new() -> RegionFromCpp<FROM_CPP> {
        unsafe { RegionFromCpp(ffi::wxRegion_new()) }
    }
    /// Constructs a rectangular region with the given position and size.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#af2b51dd977107b0dd162db6e4e9e9c5c).
    pub fn new_with_coord(
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> RegionFromCpp<FROM_CPP> {
        unsafe { RegionFromCpp(ffi::wxRegion_new1(x, y, width, height)) }
    }
    /// Constructs a rectangular region from the top left point and the bottom right point.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#a2cebf12d74122ce220f3ad48fa686e97).
    pub fn new_with_point<P: PointMethods, P2: PointMethods>(
        top_left: &P,
        bottom_right: &P2,
    ) -> RegionFromCpp<FROM_CPP> {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            RegionFromCpp(ffi::wxRegion_new2(top_left, bottom_right))
        }
    }
    /// Constructs a rectangular region a wxRect object.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#a66708b42c8bce4a76ada65cb312b60f9).
    pub fn new_with_rect<R: RectMethods>(rect: &R) -> RegionFromCpp<FROM_CPP> {
        unsafe {
            let rect = rect.as_ptr();
            RegionFromCpp(ffi::wxRegion_new3(rect))
        }
    }
    /// Copy constructor, uses Reference Counting.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#ad88c11689e65f02bb99acea6d59a6d9f).
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionFromCpp<FROM_CPP> {
        unsafe {
            let region = region.as_ptr();
            RegionFromCpp(ffi::wxRegion_new4(region))
        }
    }
    // NOT_SUPPORTED: fn wxRegion5()
    /// Constructs a region using a bitmap.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#ad7bfbeb79fe8dc281b91cfba43e8bd0a).
    pub fn new_with_bitmap<B: BitmapMethods>(bmp: &B) -> RegionFromCpp<FROM_CPP> {
        unsafe {
            let bmp = bmp.as_ptr();
            RegionFromCpp(ffi::wxRegion_new6(bmp))
        }
    }
    /// Constructs a region using the non-transparent pixels of a bitmap.
    ///
    /// See [C++ `wxRegion::wxRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region.html#aab62039edc53c55b833042b7c83e9068).
    pub fn new_with_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        bmp: &B,
        trans_colour: &C,
        tolerance: c_int,
    ) -> RegionFromCpp<FROM_CPP> {
        unsafe {
            let bmp = bmp.as_ptr();
            let trans_colour = trans_colour.as_ptr();
            RegionFromCpp(ffi::wxRegion_new7(bmp, trans_colour, tolerance))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RegionFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RegionFromCpp<FROM_CPP>> for GDIObjectFromCpp<FROM_CPP> {
    fn from(o: RegionFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<RegionFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RegionFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RegionFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRegion_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for RegionFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRegionIterator
wxwidgets! {
    /// This class is used to iterate through the rectangles in a region, typically when examining the damaged regions of a window within an OnPaint call.
    /// - [`RegionIterator`] represents a C++ `wxRegionIterator` class instance which your code has ownership, [`RegionIteratorFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RegionIterator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRegionIterator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html) for more details.
    #[doc(alias = "wxRegionIterator")]
    #[doc(alias = "RegionIterator")]
    class RegionIterator
        = RegionIteratorFromCpp<true>(wxRegionIterator) impl
        RegionIteratorMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> RegionIteratorFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxRegionIterator::wxRegionIterator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a1a2d17672c9585f86e26b8ee054e08e5).
    pub fn new() -> RegionIteratorFromCpp<FROM_CPP> {
        unsafe { RegionIteratorFromCpp(ffi::wxRegionIterator_new()) }
    }
    /// Creates an iterator object given a region.
    ///
    /// See [C++ `wxRegionIterator::wxRegionIterator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_region_iterator.html#a01b1845c8ecb190d244dfcc97d358f99).
    pub fn new_with_region<R: RegionMethods>(region: &R) -> RegionIteratorFromCpp<FROM_CPP> {
        unsafe {
            let region = region.as_ptr();
            RegionIteratorFromCpp(ffi::wxRegionIterator_new1(region))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RegionIteratorFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<RegionIteratorFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: RegionIteratorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for RegionIteratorFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxRegionIterator_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for RegionIteratorFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRendererNative
wxwidgets! {
    /// First, a brief introduction to wxRendererNative and why it is needed.
    /// - [`RendererNative`] represents a C++ `wxRendererNative` class instance which your code has ownership, [`RendererNativeFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RendererNative`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRendererNative` class's documentation](https://docs.wxwidgets.org/3.2/classwx_renderer_native.html) for more details.
    #[doc(alias = "wxRendererNative")]
    #[doc(alias = "RendererNative")]
    class RendererNative
        = RendererNativeFromCpp<true>(wxRendererNative) impl
        RendererNativeMethods
}
impl<const FROM_CPP: bool> RendererNativeFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RendererNativeFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for RendererNativeFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxRendererNative_delete(self.0) }
        }
    }
}

// wxRichToolTip
wxwidgets! {
    /// Allows showing a tool tip with more customizations than wxToolTip.
    /// - [`RichToolTip`] represents a C++ `wxRichToolTip` class instance which your code has ownership, [`RichToolTipFromCpp`]`<false>` represents one which don't own.
    /// - Use [`RichToolTip`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxRichToolTip` class's documentation](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html) for more details.
    #[doc(alias = "wxRichToolTip")]
    #[doc(alias = "RichToolTip")]
    class RichToolTip
        = RichToolTipFromCpp<true>(wxRichToolTip) impl
        RichToolTipMethods
}
impl<const FROM_CPP: bool> RichToolTipFromCpp<FROM_CPP> {
    /// Constructor must specify the tooltip title and main message.
    ///
    /// See [C++ `wxRichToolTip::wxRichToolTip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_rich_tool_tip.html#ada22072c75eaca6de3de2e89e66a352f).
    pub fn new(title: &str, message: &str) -> RichToolTipFromCpp<FROM_CPP> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            RichToolTipFromCpp(ffi::wxRichToolTip_new(title, message))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for RichToolTipFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for RichToolTipFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxRichToolTip_delete(self.0) }
        }
    }
}
