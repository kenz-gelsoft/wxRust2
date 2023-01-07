use super::*;

// wxCalculateLayoutEvent
wxwidgets! {
    /// This event is sent by wxLayoutAlgorithm to calculate the amount of the remaining client area that the window should occupy.
    /// - [`CalculateLayoutEvent`] represents a C++ `wxCalculateLayoutEvent` class instance which your code has ownership, [`CalculateLayoutEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CalculateLayoutEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalculateLayoutEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html) for more details.
    #[doc(alias = "wxCalculateLayoutEvent")]
    #[doc(alias = "CalculateLayoutEvent")]
    class CalculateLayoutEvent
        = CalculateLayoutEventFromCpp<false>(wxCalculateLayoutEvent) impl
        CalculateLayoutEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CalculateLayoutEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxCalculateLayoutEvent::wxCalculateLayoutEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a132da85194408f1ee9e57929a63e4af0).
    pub fn new(id: c_int) -> CalculateLayoutEventFromCpp<FROM_CPP> {
        unsafe { CalculateLayoutEventFromCpp(ffi::wxCalculateLayoutEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalculateLayoutEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CalculateLayoutEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: CalculateLayoutEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalculateLayoutEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CalculateLayoutEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CalculateLayoutEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCalculateLayoutEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CalculateLayoutEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCalendarCtrl
wxwidgets! {
    /// The calendar control allows the user to pick a date.
    /// - [`CalendarCtrl`] represents a C++ `wxCalendarCtrl` class instance which your code has ownership, [`CalendarCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CalendarCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalendarCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html) for more details.
    #[doc(alias = "wxCalendarCtrl")]
    #[doc(alias = "CalendarCtrl")]
    class CalendarCtrl
        = CalendarCtrlFromCpp<false>(wxCalendarCtrl) impl
        CalendarCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CalendarCtrlFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCalendarCtrl::wxCalendarCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ab25f66cdb24c24f0e21ccb4761a5aff1).
    pub fn new_2step() -> CalendarCtrlFromCpp<FROM_CPP> {
        unsafe { CalendarCtrlFromCpp(ffi::wxCalendarCtrl_new()) }
    }
    /// Does the same as Create() method.
    ///
    /// See [C++ `wxCalendarCtrl::wxCalendarCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#aaa594799b72c7b7fbbd80346d202f582).
    pub fn new<W: WindowMethods, D: DateTimeMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        date: &D,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> CalendarCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let date = date.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CalendarCtrlFromCpp(ffi::wxCalendarCtrl_new1(
                parent, id, date, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for CalendarCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CalendarCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: CalendarCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalendarCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: CalendarCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalendarCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: CalendarCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalendarCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CalendarCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CalendarCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCalendarCtrl_CLASSINFO()) }
    }
}

// wxCalendarDateAttr
wxwidgets! {
    /// wxCalendarDateAttr is a custom attributes for a calendar date.
    /// - [`CalendarDateAttr`] represents a C++ `wxCalendarDateAttr` class instance which your code has ownership, [`CalendarDateAttrFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CalendarDateAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalendarDateAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html) for more details.
    #[doc(alias = "wxCalendarDateAttr")]
    #[doc(alias = "CalendarDateAttr")]
    class CalendarDateAttr
        = CalendarDateAttrFromCpp<false>(wxCalendarDateAttr) impl
        CalendarDateAttrMethods
}
impl<const FROM_CPP: bool> CalendarDateAttrFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxCalendarDateAttr()
    // NOT_SUPPORTED: fn wxCalendarDateAttr1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalendarDateAttrFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for CalendarDateAttrFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxCalendarDateAttr_delete(self.0) }
        }
    }
}

// wxCalendarEvent
wxwidgets! {
    /// The wxCalendarEvent class is used together with wxCalendarCtrl.
    /// - [`CalendarEvent`] represents a C++ `wxCalendarEvent` class instance which your code has ownership, [`CalendarEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CalendarEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCalendarEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html) for more details.
    #[doc(alias = "wxCalendarEvent")]
    #[doc(alias = "CalendarEvent")]
    class CalendarEvent
        = CalendarEventFromCpp<false>(wxCalendarEvent) impl
        CalendarEventMethods,
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CalendarEventFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxCalendarEvent::wxCalendarEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html#af4042d0201f1f5a593ec298b908187f8).
    pub fn new() -> CalendarEventFromCpp<FROM_CPP> {
        unsafe { CalendarEventFromCpp(ffi::wxCalendarEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxCalendarEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalendarEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CalendarEventFromCpp<FROM_CPP>> for DateEventFromCpp<FROM_CPP> {
    fn from(o: CalendarEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalendarEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: CalendarEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalendarEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: CalendarEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CalendarEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CalendarEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CalendarEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCalendarEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CalendarEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCaret
wxwidgets! {
    /// A caret is a blinking cursor showing the position where the typed text will appear.
    /// - [`Caret`] represents a C++ `wxCaret` class instance which your code has ownership, [`CaretFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Caret`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCaret` class's documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html) for more details.
    #[doc(alias = "wxCaret")]
    #[doc(alias = "Caret")]
    class Caret
        = CaretFromCpp<false>(wxCaret) impl
        CaretMethods
}
impl<const FROM_CPP: bool> CaretFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCaret::wxCaret()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a07b320d7d5296d378fa8d6350e2ecf22).
    pub fn new() -> CaretFromCpp<FROM_CPP> {
        unsafe { CaretFromCpp(ffi::wxCaret_new()) }
    }
    /// Creates a caret with the given size (in pixels) and associates it with the window.
    ///
    /// See [C++ `wxCaret::wxCaret()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a69612cbfbe2bd14a244b9c347db5e142).
    pub fn new_with_window_int<W: WindowMethods>(
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> CaretFromCpp<FROM_CPP> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CaretFromCpp(ffi::wxCaret_new1(window, width, height))
        }
    }
    ///
    /// See [C++ `wxCaret::wxCaret()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a60b70ae60ae73f9e3c86bfb08c628e64).
    pub fn new_with_window_size<W: WindowMethods, S: SizeMethods>(
        window: Option<&W>,
        size: &S,
    ) -> CaretFromCpp<FROM_CPP> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            CaretFromCpp(ffi::wxCaret_new2(window, size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CaretFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for CaretFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxCaret_delete(self.0) }
        }
    }
}

// wxCheckBox
wxwidgets! {
    /// A checkbox is a labelled box which by default is either on (checkmark is visible) or off (no checkmark).
    /// - [`CheckBox`] represents a C++ `wxCheckBox` class instance which your code has ownership, [`CheckBoxFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CheckBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCheckBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html) for more details.
    #[doc(alias = "wxCheckBox")]
    #[doc(alias = "CheckBox")]
    class CheckBox
        = CheckBoxFromCpp<false>(wxCheckBox) impl
        CheckBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CheckBoxFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCheckBox::wxCheckBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a2b239bc0db88dcae8b1dfe2ac3e7f96f).
    pub fn new_2step() -> CheckBoxFromCpp<FROM_CPP> {
        unsafe { CheckBoxFromCpp(ffi::wxCheckBox_new()) }
    }
    /// Constructor, creating and showing a checkbox.
    ///
    /// See [C++ `wxCheckBox::wxCheckBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a5ab183aef8c69afd5ca12de0ce41dbc4).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckBoxFromCpp<FROM_CPP> {
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
            CheckBoxFromCpp(ffi::wxCheckBox_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for CheckBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CheckBoxFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: CheckBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: CheckBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: CheckBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CheckBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CheckBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCheckBox_CLASSINFO()) }
    }
}

// wxCheckListBox
wxwidgets! {
    /// A wxCheckListBox is like a wxListBox, but allows items to be checked or unchecked.
    /// - [`CheckListBox`] represents a C++ `wxCheckListBox` class instance which your code has ownership, [`CheckListBoxFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CheckListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCheckListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html) for more details.
    #[doc(alias = "wxCheckListBox")]
    #[doc(alias = "CheckListBox")]
    class CheckListBox
        = CheckListBoxFromCpp<false>(wxCheckListBox) impl
        CheckListBoxMethods,
        // ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CheckListBoxFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCheckListBox::wxCheckListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a7ef1af0cad44ed9b4ab99daa3469e10b).
    pub fn new_2step() -> CheckListBoxFromCpp<FROM_CPP> {
        unsafe { CheckListBoxFromCpp(ffi::wxCheckListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxCheckListBox1()
    /// Constructor, creating and showing a list box.
    ///
    /// See [C++ `wxCheckListBox::wxCheckListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#abe40b7afe1da92affd48c6522f02c762).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckListBoxFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CheckListBoxFromCpp(ffi::wxCheckListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for CheckListBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CheckListBoxFromCpp<FROM_CPP>> for ListBoxFromCpp<FROM_CPP> {
    fn from(o: CheckListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckListBoxFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: CheckListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckListBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: CheckListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckListBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: CheckListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CheckListBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CheckListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CheckListBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCheckListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCheckListBox
impl<const FROM_CPP: bool> ItemContainerMethods for CheckListBoxFromCpp<FROM_CPP> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for CheckListBoxFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ListBoxMethods for CheckListBoxFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn Create()
    ///
    /// See [C++ `wxCheckListBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a1ff3c075762c1be1321c5d6e9a71bd1e).
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
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
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCheckListBox_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
}

// wxChildFocusEvent
wxwidgets! {
    /// A child focus event is sent to a (parent-)window when one of its child windows gains focus, so that the window could restore the focus back to its corresponding child if it loses it now and regains later.
    /// - [`ChildFocusEvent`] represents a C++ `wxChildFocusEvent` class instance which your code has ownership, [`ChildFocusEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ChildFocusEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxChildFocusEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html) for more details.
    #[doc(alias = "wxChildFocusEvent")]
    #[doc(alias = "ChildFocusEvent")]
    class ChildFocusEvent
        = ChildFocusEventFromCpp<false>(wxChildFocusEvent) impl
        ChildFocusEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ChildFocusEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxChildFocusEvent::wxChildFocusEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html#ad630ddc1fb95a86a74efdbe04e6105b6).
    pub fn new<W: WindowMethods>(win: Option<&W>) -> ChildFocusEventFromCpp<FROM_CPP> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ChildFocusEventFromCpp(ffi::wxChildFocusEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ChildFocusEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ChildFocusEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: ChildFocusEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChildFocusEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ChildFocusEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChildFocusEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ChildFocusEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ChildFocusEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxChildFocusEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ChildFocusEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxChoice
wxwidgets! {
    /// A choice item is used to select one of a list of strings.
    /// - [`Choice`] represents a C++ `wxChoice` class instance which your code has ownership, [`ChoiceFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Choice`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxChoice` class's documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html) for more details.
    #[doc(alias = "wxChoice")]
    #[doc(alias = "Choice")]
    class Choice
        = ChoiceFromCpp<false>(wxChoice) impl
        ChoiceMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ChoiceFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxChoice::wxChoice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#abe4fa74cf09a21608e90cd55ca96fabd).
    pub fn new_2step() -> ChoiceFromCpp<FROM_CPP> {
        unsafe { ChoiceFromCpp(ffi::wxChoice_new()) }
    }
    // NOT_SUPPORTED: fn wxChoice1()
    /// Constructor, creating and showing a choice.
    ///
    /// See [C++ `wxChoice::wxChoice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#a2bb4542d8803a4c91de478831f6ed560).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ChoiceFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoiceFromCpp(ffi::wxChoice_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ChoiceFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ChoiceFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ChoiceFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoiceFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ChoiceFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoiceFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ChoiceFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoiceFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ChoiceFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ChoiceFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxChoice_CLASSINFO()) }
    }
}
// Mix-in(s) to wxChoice
impl<const FROM_CPP: bool> ItemContainerMethods for ChoiceFromCpp<FROM_CPP> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for ChoiceFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}

// wxChoicebook
wxwidgets! {
    /// wxChoicebook is a class similar to wxNotebook, but uses a wxChoice control to show the labels instead of the tabs.
    /// - [`Choicebook`] represents a C++ `wxChoicebook` class instance which your code has ownership, [`ChoicebookFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Choicebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxChoicebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html) for more details.
    #[doc(alias = "wxChoicebook")]
    #[doc(alias = "Choicebook")]
    class Choicebook
        = ChoicebookFromCpp<false>(wxChoicebook) impl
        ChoicebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ChoicebookFromCpp<FROM_CPP> {
    /// Constructs a choicebook control.
    ///
    /// See [C++ `wxChoicebook::wxChoicebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#aa06ad50928a63e7067e9e496e4fbcf08).
    pub fn new_2step() -> ChoicebookFromCpp<FROM_CPP> {
        unsafe { ChoicebookFromCpp(ffi::wxChoicebook_new()) }
    }
    ///
    /// See [C++ `wxChoicebook::wxChoicebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#ab4054d2f57aebdc7d6991798c4a3376f).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ChoicebookFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoicebookFromCpp(ffi::wxChoicebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ChoicebookFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ChoicebookFromCpp<FROM_CPP>> for BookCtrlBaseFromCpp<FROM_CPP> {
    fn from(o: ChoicebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoicebookFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ChoicebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoicebookFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ChoicebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoicebookFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ChoicebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ChoicebookFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ChoicebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ChoicebookFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxChoicebook_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for ChoicebookFromCpp<FROM_CPP> {
    /// Create the choicebook control that has already been constructed with the default constructor.
    ///
    /// See [C++ `wxChoicebook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#a101f60164662715ef3a95a9ce3709037).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxChoicebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxClientDC
wxwidgets! {
    /// wxClientDC is primarily useful for obtaining information about the window from outside EVT_PAINT() handler.
    /// - [`ClientDC`] represents a C++ `wxClientDC` class instance which your code has ownership, [`ClientDCFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ClientDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxClientDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html) for more details.
    #[doc(alias = "wxClientDC")]
    #[doc(alias = "ClientDC")]
    class ClientDC
        = ClientDCFromCpp<false>(wxClientDC) impl
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ClientDCFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxClientDC::wxClientDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html#ac2deeb91c3d7f8dd755e6f6166159501).
    pub fn new<W: WindowMethods>(window: Option<&W>) -> ClientDCFromCpp<FROM_CPP> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ClientDCFromCpp(ffi::wxClientDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClientDCFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ClientDCFromCpp<FROM_CPP>> for WindowDCFromCpp<FROM_CPP> {
    fn from(o: ClientDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ClientDCFromCpp<FROM_CPP>> for DCFromCpp<FROM_CPP> {
    fn from(o: ClientDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ClientDCFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ClientDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ClientDCFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxClientDC_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ClientDCFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboard
wxwidgets! {
    /// A class for manipulating the clipboard.
    /// - [`Clipboard`] represents a C++ `wxClipboard` class instance which your code has ownership, [`ClipboardFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Clipboard`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxClipboard` class's documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html) for more details.
    #[doc(alias = "wxClipboard")]
    #[doc(alias = "Clipboard")]
    class Clipboard
        = ClipboardFromCpp<false>(wxClipboard) impl
        ClipboardMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ClipboardFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxClipboard::wxClipboard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a8d61a457ae71f52f718e0225ba3e8bb4).
    pub fn new() -> ClipboardFromCpp<FROM_CPP> {
        unsafe { ClipboardFromCpp(ffi::wxClipboard_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClipboardFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ClipboardFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ClipboardFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ClipboardFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxClipboard_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ClipboardFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboardTextEvent
wxwidgets! {
    /// This class represents the events generated by a control (typically a wxTextCtrl but other windows can generate these events as well) when its content gets copied or cut to, or pasted from the clipboard.
    /// - [`ClipboardTextEvent`] represents a C++ `wxClipboardTextEvent` class instance which your code has ownership, [`ClipboardTextEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ClipboardTextEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxClipboardTextEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard_text_event.html) for more details.
    #[doc(alias = "wxClipboardTextEvent")]
    #[doc(alias = "ClipboardTextEvent")]
    class ClipboardTextEvent
        = ClipboardTextEventFromCpp<false>(wxClipboardTextEvent) impl
        ClipboardTextEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ClipboardTextEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxClipboardTextEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClipboardTextEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ClipboardTextEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: ClipboardTextEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ClipboardTextEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ClipboardTextEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ClipboardTextEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ClipboardTextEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ClipboardTextEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxClipboardTextEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ClipboardTextEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCloseEvent
wxwidgets! {
    /// This event class contains information about window and session close events.
    /// - [`CloseEvent`] represents a C++ `wxCloseEvent` class instance which your code has ownership, [`CloseEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CloseEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCloseEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html) for more details.
    #[doc(alias = "wxCloseEvent")]
    #[doc(alias = "CloseEvent")]
    class CloseEvent
        = CloseEventFromCpp<false>(wxCloseEvent) impl
        CloseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CloseEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxCloseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CloseEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CloseEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: CloseEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CloseEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CloseEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CloseEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCloseEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CloseEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCollapsiblePane
wxwidgets! {
    /// A collapsible pane is a container with an embedded button-like control which can be used by the user to collapse or expand the pane's contents.
    /// - [`CollapsiblePane`] represents a C++ `wxCollapsiblePane` class instance which your code has ownership, [`CollapsiblePaneFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CollapsiblePane`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCollapsiblePane` class's documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html) for more details.
    #[doc(alias = "wxCollapsiblePane")]
    #[doc(alias = "CollapsiblePane")]
    class CollapsiblePane
        = CollapsiblePaneFromCpp<false>(wxCollapsiblePane) impl
        CollapsiblePaneMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CollapsiblePaneFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCollapsiblePane::wxCollapsiblePane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#afe29ca6c7b230a05d63022c3adb7348a).
    pub fn new_2step() -> CollapsiblePaneFromCpp<FROM_CPP> {
        unsafe { CollapsiblePaneFromCpp(ffi::wxCollapsiblePane_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxCollapsiblePane::wxCollapsiblePane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#a920561522d4b28c08b8d693047c3aa38).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CollapsiblePaneFromCpp<FROM_CPP> {
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
            CollapsiblePaneFromCpp(ffi::wxCollapsiblePane_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for CollapsiblePaneFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: CollapsiblePaneFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: CollapsiblePaneFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: CollapsiblePaneFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CollapsiblePaneFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CollapsiblePaneFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCollapsiblePane_CLASSINFO()) }
    }
}

// wxCollapsiblePaneEvent
wxwidgets! {
    /// This event class is used for the events generated by wxCollapsiblePane.
    /// - [`CollapsiblePaneEvent`] represents a C++ `wxCollapsiblePaneEvent` class instance which your code has ownership, [`CollapsiblePaneEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CollapsiblePaneEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCollapsiblePaneEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html) for more details.
    #[doc(alias = "wxCollapsiblePaneEvent")]
    #[doc(alias = "CollapsiblePaneEvent")]
    class CollapsiblePaneEvent
        = CollapsiblePaneEventFromCpp<false>(wxCollapsiblePaneEvent) impl
        CollapsiblePaneEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CollapsiblePaneEventFromCpp<FROM_CPP> {
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxCollapsiblePaneEvent::wxCollapsiblePaneEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html#a4b2a4c27e9908f892be38971b0ddf555).
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        collapsed: bool,
    ) -> CollapsiblePaneEventFromCpp<FROM_CPP> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CollapsiblePaneEventFromCpp(ffi::wxCollapsiblePaneEvent_new(generator, id, collapsed))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CollapsiblePaneEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: CollapsiblePaneEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: CollapsiblePaneEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CollapsiblePaneEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CollapsiblePaneEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CollapsiblePaneEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCollapsiblePaneEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CollapsiblePaneEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColour
wxwidgets! {
    /// A colour is an object representing a combination of Red, Green, and Blue (RGB) intensity values and an Alpha value, and is used to determine drawing colours.
    /// - [`Colour`] represents a C++ `wxColour` class instance which your code has ownership, [`ColourFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Colour`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColour` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html) for more details.
    #[doc(alias = "wxColour")]
    #[doc(alias = "Colour")]
    class Colour
        = ColourFromCpp<false>(wxColour) impl
        ColourMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ColourFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxColour::wxColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#aac96e7922132d672a1f83d59ecf07343).
    pub fn new() -> ColourFromCpp<FROM_CPP> {
        unsafe { ColourFromCpp(ffi::wxColour_new()) }
    }
    // NOT_SUPPORTED: fn wxColour1()
    ///
    /// See [C++ `wxColour::wxColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#ae114ff88fe3a07477d111baa01c1b325).
    pub fn new_with_str(colour_name: &str) -> ColourFromCpp<FROM_CPP> {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            ColourFromCpp(ffi::wxColour_new2(colour_name))
        }
    }
    // NOT_SUPPORTED: fn wxColour3()
    /// Copy constructor.
    ///
    /// See [C++ `wxColour::wxColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a0fbc51432a57424a0538d0932af6bf78).
    pub fn new_with_colour<C: ColourMethods>(colour: &C) -> ColourFromCpp<FROM_CPP> {
        unsafe {
            let colour = colour.as_ptr();
            ColourFromCpp(ffi::wxColour_new4(colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ColourFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ColourFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ColourFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxColour_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ColourFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourData
wxwidgets! {
    /// This class holds a variety of information related to colour dialogs.
    /// - [`ColourData`] represents a C++ `wxColourData` class instance which your code has ownership, [`ColourDataFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ColourData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html) for more details.
    #[doc(alias = "wxColourData")]
    #[doc(alias = "ColourData")]
    class ColourData
        = ColourDataFromCpp<false>(wxColourData) impl
        ColourDataMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ColourDataFromCpp<FROM_CPP> {
    //  ENUM: @6
    pub const NUM_CUSTOM: c_int = 16;

    /// Constructor.
    ///
    /// See [C++ `wxColourData::wxColourData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a970e44b3f6ee89a4f4621bd76eb3738c).
    pub fn new() -> ColourDataFromCpp<FROM_CPP> {
        unsafe { ColourDataFromCpp(ffi::wxColourData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourDataFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ColourDataFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ColourDataFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ColourDataFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxColourData_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ColourDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourDatabase
wxwidgets! {
    /// wxWidgets maintains a database of standard RGB colours for a predefined set of named colours.
    /// - [`ColourDatabase`] represents a C++ `wxColourDatabase` class instance which your code has ownership, [`ColourDatabaseFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ColourDatabase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourDatabase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html) for more details.
    #[doc(alias = "wxColourDatabase")]
    #[doc(alias = "ColourDatabase")]
    class ColourDatabase
        = ColourDatabaseFromCpp<false>(wxColourDatabase) impl
        ColourDatabaseMethods
}
impl<const FROM_CPP: bool> ColourDatabaseFromCpp<FROM_CPP> {
    /// Constructs the colour database.
    ///
    /// See [C++ `wxColourDatabase::wxColourDatabase()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html#a66269122cb725da348b3a1c396002e41).
    pub fn new() -> ColourDatabaseFromCpp<FROM_CPP> {
        unsafe { ColourDatabaseFromCpp(ffi::wxColourDatabase_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourDatabaseFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ColourDatabaseFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxColourDatabase_delete(self.0) }
        }
    }
}

// wxColourDialog
wxwidgets! {
    /// This class represents the colour chooser dialog.
    /// - [`ColourDialog`] represents a C++ `wxColourDialog` class instance which your code has ownership, [`ColourDialogFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ColourDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html) for more details.
    #[doc(alias = "wxColourDialog")]
    #[doc(alias = "ColourDialog")]
    class ColourDialog
        = ColourDialogFromCpp<false>(wxColourDialog) impl
        ColourDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ColourDialogFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxColourDialog::wxColourDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html#a357b4f19cd3757c6f74c44cd49a1d90e).
    pub fn new<W: WindowMethods, C: ColourDataMethods>(
        parent: Option<&W>,
        data: Option<&C>,
    ) -> ColourDialogFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ColourDialogFromCpp(ffi::wxColourDialog_new(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ColourDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ColourDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: ColourDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourDialogFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: ColourDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourDialogFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: ColourDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ColourDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ColourDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ColourDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ColourDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxColourDialog_CLASSINFO()) }
    }
}

// wxColourPickerCtrl
wxwidgets! {
    /// This control allows the user to select a colour.
    /// - [`ColourPickerCtrl`] represents a C++ `wxColourPickerCtrl` class instance which your code has ownership, [`ColourPickerCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ColourPickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourPickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html) for more details.
    #[doc(alias = "wxColourPickerCtrl")]
    #[doc(alias = "ColourPickerCtrl")]
    class ColourPickerCtrl
        = ColourPickerCtrlFromCpp<false>(wxColourPickerCtrl) impl
        ColourPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ColourPickerCtrlFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxColourPickerCtrl::wxColourPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a610182eb11286c6df2fbd195b98868cd).
    pub fn new_2step() -> ColourPickerCtrlFromCpp<FROM_CPP> {
        unsafe { ColourPickerCtrlFromCpp(ffi::wxColourPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxColourPickerCtrl::wxColourPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a3763437a33dd948c3ae0d7a2210b7c0e).
    pub fn new<
        W: WindowMethods,
        C: ColourMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        colour: &C,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ColourPickerCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ColourPickerCtrlFromCpp(ffi::wxColourPickerCtrl_new1(
                parent, id, colour, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ColourPickerCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ColourPickerCtrlFromCpp<FROM_CPP>> for PickerBaseFromCpp<FROM_CPP> {
    fn from(o: ColourPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourPickerCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ColourPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourPickerCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ColourPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourPickerCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ColourPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourPickerCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ColourPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ColourPickerCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxColourPickerCtrl_CLASSINFO()) }
    }
}

// wxColourPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxColourPickerCtrl.
    /// - [`ColourPickerEvent`] represents a C++ `wxColourPickerEvent` class instance which your code has ownership, [`ColourPickerEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ColourPickerEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxColourPickerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html) for more details.
    #[doc(alias = "wxColourPickerEvent")]
    #[doc(alias = "ColourPickerEvent")]
    class ColourPickerEvent
        = ColourPickerEventFromCpp<false>(wxColourPickerEvent) impl
        ColourPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ColourPickerEventFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxColourPickerEvent::wxColourPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#afa8edf53f69e85cbaa6a1f2e91c22f2e).
    pub fn new() -> ColourPickerEventFromCpp<FROM_CPP> {
        unsafe { ColourPickerEventFromCpp(ffi::wxColourPickerEvent_new()) }
    }
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxColourPickerEvent::wxColourPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#ab9d9e7ef4204c0dc7511eddf4fc4eb0c).
    pub fn new_with_object<O: ObjectMethods, C: ColourMethods>(
        generator: Option<&O>,
        id: c_int,
        colour: &C,
    ) -> ColourPickerEventFromCpp<FROM_CPP> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            ColourPickerEventFromCpp(ffi::wxColourPickerEvent_new1(generator, id, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourPickerEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ColourPickerEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: ColourPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourPickerEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ColourPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ColourPickerEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ColourPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ColourPickerEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxColourPickerEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ColourPickerEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxComboBox
wxwidgets! {
    /// A combobox is like a combination of an edit control and a listbox.
    /// - [`ComboBox`] represents a C++ `wxComboBox` class instance which your code has ownership, [`ComboBoxFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ComboBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxComboBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html) for more details.
    #[doc(alias = "wxComboBox")]
    #[doc(alias = "ComboBox")]
    class ComboBox
        = ComboBoxFromCpp<false>(wxComboBox) impl
        ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ComboBoxFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxComboBox::wxComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#aac1f4d94c40a0a675dd5b2c72376057a).
    pub fn new_2step() -> ComboBoxFromCpp<FROM_CPP> {
        unsafe { ComboBoxFromCpp(ffi::wxComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxComboBox1()
    /// Constructor, creating and showing a combobox.
    ///
    /// See [C++ `wxComboBox::wxComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#ade11ed7d2b64bd1f50cdb34f162e120f).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboBoxFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ComboBoxFromCpp(ffi::wxComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ComboBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ComboBoxFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ComboBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ComboBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ComboBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ComboBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ComboBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ComboBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ComboBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ComboBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboBox
impl<const FROM_CPP: bool> ItemContainerMethods for ComboBoxFromCpp<FROM_CPP> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for ComboBoxFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> TextEntryMethods for ComboBoxFromCpp<FROM_CPP> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsTextEntry(self.as_ptr()) }
    }
}

// wxComboCtrl
wxwidgets! {
    /// A combo control is a generic combobox that allows totally custom popup.
    /// - [`ComboCtrl`] represents a C++ `wxComboCtrl` class instance which your code has ownership, [`ComboCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ComboCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxComboCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html) for more details.
    #[doc(alias = "wxComboCtrl")]
    #[doc(alias = "ComboCtrl")]
    class ComboCtrl
        = ComboCtrlFromCpp<false>(wxComboCtrl) impl
        ComboCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ComboCtrlFromCpp<FROM_CPP> {
    //  ENUM: @9
    pub const ShowBelow: c_int = 0x0000;
    pub const ShowAbove: c_int = 0x0001;
    pub const CanDeferShow: c_int = 0x0002;

    /// Default constructor.
    ///
    /// See [C++ `wxComboCtrl::wxComboCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#af31b505e73047689dc4d2948eaa13a3d).
    pub fn new_2step() -> ComboCtrlFromCpp<FROM_CPP> {
        unsafe { ComboCtrlFromCpp(ffi::wxComboCtrl_new()) }
    }
    /// Constructor, creating and showing a combo control.
    ///
    /// See [C++ `wxComboCtrl::wxComboCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a0f9d6b21d3728f135dbe0d01ee4bf865).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ComboCtrlFromCpp(ffi::wxComboCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ComboCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ComboCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ComboCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ComboCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ComboCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ComboCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ComboCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ComboCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ComboCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ComboCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxComboCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboCtrl
impl<const FROM_CPP: bool> TextEntryMethods for ComboCtrlFromCpp<FROM_CPP> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboCtrl_AsTextEntry(self.as_ptr()) }
    }
}

// wxComboPopup
wxwidgets! {
    /// In order to use a custom popup with wxComboCtrl, an interface class must be derived from wxComboPopup.
    /// - [`ComboPopup`] represents a C++ `wxComboPopup` class instance which your code has ownership, [`ComboPopupFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ComboPopup`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxComboPopup` class's documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html) for more details.
    #[doc(alias = "wxComboPopup")]
    #[doc(alias = "ComboPopup")]
    class ComboPopup
        = ComboPopupFromCpp<false>(wxComboPopup) impl
        ComboPopupMethods
}
impl<const FROM_CPP: bool> ComboPopupFromCpp<FROM_CPP> {
    // BLOCKED: fn wxComboPopup()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ComboPopupFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ComboPopupFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxComboPopup_delete(self.0) }
        }
    }
}

// wxCommand
wxwidgets! {
    /// wxCommand is a base class for modelling an application command, which is an action usually performed by selecting a menu item, pressing a toolbar button or any other means provided by the application to change the data or view.
    /// - [`Command`] represents a C++ `wxCommand` class instance which your code has ownership, [`CommandFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Command`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommand` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command.html) for more details.
    #[doc(alias = "wxCommand")]
    #[doc(alias = "Command")]
    class Command
        = CommandFromCpp<false>(wxCommand) impl
        CommandMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CommandFromCpp<FROM_CPP> {
    // BLOCKED: fn wxCommand()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CommandFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CommandFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CommandFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCommand_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CommandFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandEvent
wxwidgets! {
    /// This event class contains information about command events, which originate from a variety of simple controls.
    /// - [`CommandEvent`] represents a C++ `wxCommandEvent` class instance which your code has ownership, [`CommandEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CommandEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommandEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html) for more details.
    #[doc(alias = "wxCommandEvent")]
    #[doc(alias = "CommandEvent")]
    class CommandEvent
        = CommandEventFromCpp<false>(wxCommandEvent) impl
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CommandEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxCommandEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CommandEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: CommandEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CommandEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CommandEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CommandEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCommandEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CommandEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandLinkButton
wxwidgets! {
    /// Objects of this class are similar in appearance to the normal wxButtons but are similar to the links in a web page in functionality.
    /// - [`CommandLinkButton`] represents a C++ `wxCommandLinkButton` class instance which your code has ownership, [`CommandLinkButtonFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CommandLinkButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommandLinkButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html) for more details.
    #[doc(alias = "wxCommandLinkButton")]
    #[doc(alias = "CommandLinkButton")]
    class CommandLinkButton
        = CommandLinkButtonFromCpp<false>(wxCommandLinkButton) impl
        CommandLinkButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CommandLinkButtonFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCommandLinkButton::wxCommandLinkButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a05be41a7e1fd702cee7662e1a6ae9293).
    pub fn new_2step() -> CommandLinkButtonFromCpp<FROM_CPP> {
        unsafe { CommandLinkButtonFromCpp(ffi::wxCommandLinkButton_new()) }
    }
    /// Constructor really creating a command Link button.
    ///
    /// See [C++ `wxCommandLinkButton::wxCommandLinkButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a193413ad8afa7895bdb0ef133a454234).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        main_label: &str,
        note: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CommandLinkButtonFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            CommandLinkButtonFromCpp(ffi::wxCommandLinkButton_new1(
                parent, id, main_label, note, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for CommandLinkButtonFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CommandLinkButtonFromCpp<FROM_CPP>> for ButtonFromCpp<FROM_CPP> {
    fn from(o: CommandLinkButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CommandLinkButtonFromCpp<FROM_CPP>> for AnyButtonFromCpp<FROM_CPP> {
    fn from(o: CommandLinkButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CommandLinkButtonFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: CommandLinkButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CommandLinkButtonFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: CommandLinkButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CommandLinkButtonFromCpp<FROM_CPP>>
    for EvtHandlerFromCpp<FROM_CPP>
{
    fn from(o: CommandLinkButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CommandLinkButtonFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CommandLinkButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CommandLinkButtonFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCommandLinkButton_CLASSINFO()) }
    }
}

// wxCommandProcessor
wxwidgets! {
    /// wxCommandProcessor is a class that maintains a history of wxCommands, with undo/redo functionality built-in.
    /// - [`CommandProcessor`] represents a C++ `wxCommandProcessor` class instance which your code has ownership, [`CommandProcessorFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CommandProcessor`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCommandProcessor` class's documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html) for more details.
    #[doc(alias = "wxCommandProcessor")]
    #[doc(alias = "CommandProcessor")]
    class CommandProcessor
        = CommandProcessorFromCpp<false>(wxCommandProcessor) impl
        CommandProcessorMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CommandProcessorFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxCommandProcessor::wxCommandProcessor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a8f0b55885b6ea95037e81bbe76e28d74).
    pub fn new(max_commands: c_int) -> CommandProcessorFromCpp<FROM_CPP> {
        unsafe { CommandProcessorFromCpp(ffi::wxCommandProcessor_new(max_commands)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandProcessorFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CommandProcessorFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CommandProcessorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CommandProcessorFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCommandProcessor_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CommandProcessorFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxContextMenuEvent
wxwidgets! {
    /// This class is used for context menu events, sent to give the application a chance to show a context (popup) menu for a wxWindow.
    /// - [`ContextMenuEvent`] represents a C++ `wxContextMenuEvent` class instance which your code has ownership, [`ContextMenuEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ContextMenuEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxContextMenuEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_context_menu_event.html) for more details.
    #[doc(alias = "wxContextMenuEvent")]
    #[doc(alias = "ContextMenuEvent")]
    class ContextMenuEvent
        = ContextMenuEventFromCpp<false>(wxContextMenuEvent) impl
        ContextMenuEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ContextMenuEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxContextMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ContextMenuEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ContextMenuEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: ContextMenuEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ContextMenuEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ContextMenuEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ContextMenuEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ContextMenuEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ContextMenuEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxContextMenuEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ContextMenuEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxControl
wxwidgets! {
    /// This is the base class for a control or "widget".
    /// - [`Control`] represents a C++ `wxControl` class instance which your code has ownership, [`ControlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Control`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxControl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_control.html) for more details.
    #[doc(alias = "wxControl")]
    #[doc(alias = "Control")]
    class Control
        = ControlFromCpp<false>(wxControl) impl
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ControlFromCpp<FROM_CPP> {
    /// Constructs a control.
    ///
    /// See [C++ `wxControl::wxControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#adb8f3edf807efa9159de826bf92d6a44).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ControlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ControlFromCpp(ffi::wxControl_new(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    /// Default constructor to allow 2-phase creation.
    ///
    /// See [C++ `wxControl::wxControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a08428de2ba5cc988a86fe17071d49522).
    pub fn new_2step() -> ControlFromCpp<FROM_CPP> {
        unsafe { ControlFromCpp(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ControlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ControlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ControlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ControlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ControlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ControlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxControl_CLASSINFO()) }
    }
}

// wxControlWithItems
wxwidgets! {
    /// This is convenience class that derives from both wxControl and wxItemContainer.
    /// - [`ControlWithItems`] represents a C++ `wxControlWithItems` class instance which your code has ownership, [`ControlWithItemsFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ControlWithItems`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxControlWithItems` class's documentation](https://docs.wxwidgets.org/3.2/classwx_control_with_items.html) for more details.
    #[doc(alias = "wxControlWithItems")]
    #[doc(alias = "ControlWithItems")]
    class ControlWithItems
        = ControlWithItemsFromCpp<false>(wxControlWithItems) impl
        ControlWithItemsMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ControlWithItemsFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ControlWithItemsFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ControlWithItemsFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ControlWithItemsFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlWithItemsFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ControlWithItemsFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlWithItemsFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ControlWithItemsFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ControlWithItemsFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ControlWithItemsFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ControlWithItemsFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxControlWithItems_CLASSINFO()) }
    }
}
// Mix-in(s) to wxControlWithItems
impl<const FROM_CPP: bool> ItemContainerMethods for ControlWithItemsFromCpp<FROM_CPP> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> ItemContainerImmutableMethods for ControlWithItemsFromCpp<FROM_CPP> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}

// wxCursor
wxwidgets! {
    /// A cursor is a small bitmap usually used for denoting where the mouse pointer is, with a picture that might indicate the interpretation of a mouse click.
    /// - [`Cursor`] represents a C++ `wxCursor` class instance which your code has ownership, [`CursorFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Cursor`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCursor` class's documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html) for more details.
    #[doc(alias = "wxCursor")]
    #[doc(alias = "Cursor")]
    class Cursor
        = CursorFromCpp<false>(wxCursor) impl
        CursorMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> CursorFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a21735e923430faf941c440c67f13859d).
    pub fn new() -> CursorFromCpp<FROM_CPP> {
        unsafe { CursorFromCpp(ffi::wxCursor_new()) }
    }
    // NOT_SUPPORTED: fn wxCursor1()
    // NOT_SUPPORTED: fn wxCursor2()
    // NOT_SUPPORTED: fn wxCursor3()
    /// Constructs a cursor from a wxImage.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#adaf6e394c7306a3999bdc314d7b307a7).
    pub fn new_with_image<I: ImageMethods>(image: &I) -> CursorFromCpp<FROM_CPP> {
        unsafe {
            let image = image.as_ptr();
            CursorFromCpp(ffi::wxCursor_new4(image))
        }
    }
    /// Constructs a cursor from XPM data.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#abb43549ff688898c79f48112ff85f7d7).
    pub fn new_with_char(xpm_data: *const c_void) -> CursorFromCpp<FROM_CPP> {
        unsafe { CursorFromCpp(ffi::wxCursor_new5(xpm_data)) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// See [C++ `wxCursor::wxCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a20f3123d9d012427ae1f614e10b9cfb9).
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> CursorFromCpp<FROM_CPP> {
        unsafe {
            let cursor = cursor.as_ptr();
            CursorFromCpp(ffi::wxCursor_new6(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CursorFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CursorFromCpp<FROM_CPP>> for GDIObjectFromCpp<FROM_CPP> {
    fn from(o: CursorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CursorFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: CursorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for CursorFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxCursor_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for CursorFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCustomDataObject
wxwidgets! {
    /// wxCustomDataObject is a specialization of wxDataObjectSimple for some application-specific data in arbitrary (either custom or one of the standard ones).
    /// - [`CustomDataObject`] represents a C++ `wxCustomDataObject` class instance which your code has ownership, [`CustomDataObjectFromCpp`]`<true>` represents one which don't own.
    /// - Use [`CustomDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxCustomDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html) for more details.
    #[doc(alias = "wxCustomDataObject")]
    #[doc(alias = "CustomDataObject")]
    class CustomDataObject
        = CustomDataObjectFromCpp<false>(wxCustomDataObject) impl
        CustomDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> CustomDataObjectFromCpp<FROM_CPP> {
    /// The constructor accepts a format argument which specifies the (single) format supported by this object.
    ///
    /// See [C++ `wxCustomDataObject::wxCustomDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a667ea5eae3e91095d79cb6fe9e548695).
    pub fn new<D: DataFormatMethods>(format: &D) -> CustomDataObjectFromCpp<FROM_CPP> {
        unsafe {
            let format = format.as_ptr();
            CustomDataObjectFromCpp(ffi::wxCustomDataObject_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CustomDataObjectFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<CustomDataObjectFromCpp<FROM_CPP>>
    for DataObjectSimpleFromCpp<FROM_CPP>
{
    fn from(o: CustomDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<CustomDataObjectFromCpp<FROM_CPP>> for DataObjectFromCpp<FROM_CPP> {
    fn from(o: CustomDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for CustomDataObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxCustomDataObject_delete(self.0) }
        }
    }
}
