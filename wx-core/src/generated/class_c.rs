use super::*;

// wxCalculateLayoutEvent
wxwidgets! {
    /// This event is sent by wxLayoutAlgorithm to calculate the amount of the remaining client area that the window should occupy.
    ///
    /// [See `wxCalculateLayoutEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html)
    #[doc(alias = "wxCalculateLayoutEvent")]
    #[doc(alias = "CalculateLayoutEvent")]
    class CalculateLayoutEvent
        = CalculateLayoutEventIsOwned<true>(wxCalculateLayoutEvent) impl
        CalculateLayoutEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalculateLayoutEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxCalculateLayoutEvent::wxCalculateLayoutEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a132da85194408f1ee9e57929a63e4af0)
    pub fn new(id: c_int) -> CalculateLayoutEventIsOwned<OWNED> {
        unsafe { CalculateLayoutEventIsOwned(ffi::wxCalculateLayoutEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalculateLayoutEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CalculateLayoutEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CalculateLayoutEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalculateLayoutEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CalculateLayoutEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalculateLayoutEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCalculateLayoutEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CalculateLayoutEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCalendarCtrl
wxwidgets! {
    /// The calendar control allows the user to pick a date.
    ///
    /// [See `wxCalendarCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html)
    #[doc(alias = "wxCalendarCtrl")]
    #[doc(alias = "CalendarCtrl")]
    class CalendarCtrl
        = CalendarCtrlIsOwned<true>(wxCalendarCtrl) impl
        CalendarCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalendarCtrlIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCalendarCtrl::wxCalendarCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ab25f66cdb24c24f0e21ccb4761a5aff1)
    pub fn new_2step() -> CalendarCtrlIsOwned<OWNED> {
        unsafe { CalendarCtrlIsOwned(ffi::wxCalendarCtrl_new()) }
    }
    /// Does the same as Create() method.
    ///
    /// [See `wxCalendarCtrl::wxCalendarCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#aaa594799b72c7b7fbbd80346d202f582)
    pub fn new<W: WindowMethods, D: DateTimeMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        date: &D,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> CalendarCtrlIsOwned<OWNED> {
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
            CalendarCtrlIsOwned(ffi::wxCalendarCtrl_new1(
                parent, id, date, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CalendarCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CalendarCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalendarCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCalendarCtrl_CLASSINFO()) }
    }
}

// wxCalendarDateAttr
wxwidgets! {
    /// wxCalendarDateAttr is a custom attributes for a calendar date.
    ///
    /// [See `wxCalendarDateAttr`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html)
    #[doc(alias = "wxCalendarDateAttr")]
    #[doc(alias = "CalendarDateAttr")]
    class CalendarDateAttr
        = CalendarDateAttrIsOwned<true>(wxCalendarDateAttr) impl
        CalendarDateAttrMethods
}
impl<const OWNED: bool> CalendarDateAttrIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCalendarDateAttr()
    // NOT_SUPPORTED: fn wxCalendarDateAttr1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalendarDateAttrIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for CalendarDateAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCalendarDateAttr_delete(self.0) }
        }
    }
}

// wxCalendarEvent
wxwidgets! {
    /// The wxCalendarEvent class is used together with wxCalendarCtrl.
    ///
    /// [See `wxCalendarEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html)
    #[doc(alias = "wxCalendarEvent")]
    #[doc(alias = "CalendarEvent")]
    class CalendarEvent
        = CalendarEventIsOwned<true>(wxCalendarEvent) impl
        CalendarEventMethods,
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CalendarEventIsOwned<OWNED> {
    ///
    /// [See `wxCalendarEvent::wxCalendarEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html#af4042d0201f1f5a593ec298b908187f8)
    pub fn new() -> CalendarEventIsOwned<OWNED> {
        unsafe { CalendarEventIsOwned(ffi::wxCalendarEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxCalendarEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CalendarEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for DateEventIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CalendarEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CalendarEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CalendarEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCalendarEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CalendarEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCaret
wxwidgets! {
    /// A caret is a blinking cursor showing the position where the typed text will appear.
    ///
    /// [See `wxCaret`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_caret.html)
    #[doc(alias = "wxCaret")]
    #[doc(alias = "Caret")]
    class Caret
        = CaretIsOwned<true>(wxCaret) impl
        CaretMethods
}
impl<const OWNED: bool> CaretIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCaret::wxCaret()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_caret.html#a07b320d7d5296d378fa8d6350e2ecf22)
    pub fn new() -> CaretIsOwned<OWNED> {
        unsafe { CaretIsOwned(ffi::wxCaret_new()) }
    }
    /// Creates a caret with the given size (in pixels) and associates it with the window.
    ///
    /// [See `wxCaret::wxCaret()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_caret.html#a69612cbfbe2bd14a244b9c347db5e142)
    pub fn new_with_window_int<W: WindowMethods>(
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> CaretIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CaretIsOwned(ffi::wxCaret_new1(window, width, height))
        }
    }
    ///
    /// [See `wxCaret::wxCaret()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_caret.html#a60b70ae60ae73f9e3c86bfb08c628e64)
    pub fn new_with_window_size<W: WindowMethods, S: SizeMethods>(
        window: Option<&W>,
        size: &S,
    ) -> CaretIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            CaretIsOwned(ffi::wxCaret_new2(window, size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CaretIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for CaretIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCaret_delete(self.0) }
        }
    }
}

// wxCheckBox
wxwidgets! {
    /// A checkbox is a labelled box which by default is either on (checkmark is visible) or off (no checkmark).
    ///
    /// [See `wxCheckBox`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_box.html)
    #[doc(alias = "wxCheckBox")]
    #[doc(alias = "CheckBox")]
    class CheckBox
        = CheckBoxIsOwned<true>(wxCheckBox) impl
        CheckBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckBoxIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCheckBox::wxCheckBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a2b239bc0db88dcae8b1dfe2ac3e7f96f)
    pub fn new_2step() -> CheckBoxIsOwned<OWNED> {
        unsafe { CheckBoxIsOwned(ffi::wxCheckBox_new()) }
    }
    /// Constructor, creating and showing a checkbox.
    ///
    /// [See `wxCheckBox::wxCheckBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a5ab183aef8c69afd5ca12de0ce41dbc4)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CheckBoxIsOwned<OWNED> {
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
            CheckBoxIsOwned(ffi::wxCheckBox_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CheckBoxIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CheckBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CheckBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCheckBox_CLASSINFO()) }
    }
}

// wxCheckListBox
wxwidgets! {
    /// A wxCheckListBox is like a wxListBox, but allows items to be checked or unchecked.
    ///
    /// [See `wxCheckListBox`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html)
    #[doc(alias = "wxCheckListBox")]
    #[doc(alias = "CheckListBox")]
    class CheckListBox
        = CheckListBoxIsOwned<true>(wxCheckListBox) impl
        CheckListBoxMethods,
        // ListBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CheckListBoxIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCheckListBox::wxCheckListBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a7ef1af0cad44ed9b4ab99daa3469e10b)
    pub fn new_2step() -> CheckListBoxIsOwned<OWNED> {
        unsafe { CheckListBoxIsOwned(ffi::wxCheckListBox_new()) }
    }
    // NOT_SUPPORTED: fn wxCheckListBox1()
    /// Constructor, creating and showing a list box.
    ///
    /// [See `wxCheckListBox::wxCheckListBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#abe40b7afe1da92affd48c6522f02c762)
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
    ) -> CheckListBoxIsOwned<OWNED> {
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
            CheckListBoxIsOwned(ffi::wxCheckListBox_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CheckListBoxIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for ListBoxIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CheckListBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CheckListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CheckListBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCheckListBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxCheckListBox
impl<const OWNED: bool> ItemContainerMethods for CheckListBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for CheckListBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxCheckListBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ListBoxMethods for CheckListBoxIsOwned<OWNED> {
    // NOT_SUPPORTED: fn Create()
    ///
    /// [See `wxCheckListBox::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a1ff3c075762c1be1321c5d6e9a71bd1e)
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
    ///
    /// [See `wxChildFocusEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html)
    #[doc(alias = "wxChildFocusEvent")]
    #[doc(alias = "ChildFocusEvent")]
    class ChildFocusEvent
        = ChildFocusEventIsOwned<true>(wxChildFocusEvent) impl
        ChildFocusEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChildFocusEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxChildFocusEvent::wxChildFocusEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html#ad630ddc1fb95a86a74efdbe04e6105b6)
    pub fn new<W: WindowMethods>(win: Option<&W>) -> ChildFocusEventIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ChildFocusEventIsOwned(ffi::wxChildFocusEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ChildFocusEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ChildFocusEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ChildFocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChildFocusEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ChildFocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChildFocusEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ChildFocusEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChildFocusEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxChildFocusEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ChildFocusEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxChoice
wxwidgets! {
    /// A choice item is used to select one of a list of strings.
    ///
    /// [See `wxChoice`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choice.html)
    #[doc(alias = "wxChoice")]
    #[doc(alias = "Choice")]
    class Choice
        = ChoiceIsOwned<true>(wxChoice) impl
        ChoiceMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChoiceIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxChoice::wxChoice()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choice.html#abe4fa74cf09a21608e90cd55ca96fabd)
    pub fn new_2step() -> ChoiceIsOwned<OWNED> {
        unsafe { ChoiceIsOwned(ffi::wxChoice_new()) }
    }
    // NOT_SUPPORTED: fn wxChoice1()
    /// Constructor, creating and showing a choice.
    ///
    /// [See `wxChoice::wxChoice()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choice.html#a2bb4542d8803a4c91de478831f6ed560)
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
    ) -> ChoiceIsOwned<OWNED> {
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
            ChoiceIsOwned(ffi::wxChoice_new2(
                parent, id, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ChoiceIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoiceIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ChoiceIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChoiceIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxChoice_CLASSINFO()) }
    }
}
// Mix-in(s) to wxChoice
impl<const OWNED: bool> ItemContainerMethods for ChoiceIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ChoiceIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxChoice_AsItemContainer(self.as_ptr()) }
    }
}

// wxChoicebook
wxwidgets! {
    /// wxChoicebook is a class similar to wxNotebook, but uses a wxChoice control to show the labels instead of the tabs.
    ///
    /// [See `wxChoicebook`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choicebook.html)
    #[doc(alias = "wxChoicebook")]
    #[doc(alias = "Choicebook")]
    class Choicebook
        = ChoicebookIsOwned<true>(wxChoicebook) impl
        ChoicebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ChoicebookIsOwned<OWNED> {
    /// Constructs a choicebook control.
    ///
    /// [See `wxChoicebook::wxChoicebook()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#aa06ad50928a63e7067e9e496e4fbcf08)
    pub fn new_2step() -> ChoicebookIsOwned<OWNED> {
        unsafe { ChoicebookIsOwned(ffi::wxChoicebook_new()) }
    }
    ///
    /// [See `wxChoicebook::wxChoicebook()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#ab4054d2f57aebdc7d6991798c4a3376f)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ChoicebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ChoicebookIsOwned(ffi::wxChoicebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ChoicebookIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ChoicebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ChoicebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ChoicebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxChoicebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for ChoicebookIsOwned<OWNED> {
    /// Create the choicebook control that has already been constructed with the default constructor.
    ///
    /// [See `wxChoicebook::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#a101f60164662715ef3a95a9ce3709037)
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
    ///
    /// [See `wxClientDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html)
    #[doc(alias = "wxClientDC")]
    #[doc(alias = "ClientDC")]
    class ClientDC
        = ClientDCIsOwned<true>(wxClientDC) impl
        ClientDCMethods,
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClientDCIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxClientDC::wxClientDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html#ac2deeb91c3d7f8dd755e6f6166159501)
    pub fn new<W: WindowMethods>(window: Option<&W>) -> ClientDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ClientDCIsOwned(ffi::wxClientDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClientDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ClientDCIsOwned<OWNED>> for WindowDCIsOwned<OWNED> {
    fn from(o: ClientDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClientDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: ClientDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClientDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClientDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClientDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClientDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClientDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboard
wxwidgets! {
    /// A class for manipulating the clipboard.
    ///
    /// [See `wxClipboard`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_clipboard.html)
    #[doc(alias = "wxClipboard")]
    #[doc(alias = "Clipboard")]
    class Clipboard
        = ClipboardIsOwned<true>(wxClipboard) impl
        ClipboardMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClipboardIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxClipboard::wxClipboard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a8d61a457ae71f52f718e0225ba3e8bb4)
    pub fn new() -> ClipboardIsOwned<OWNED> {
        unsafe { ClipboardIsOwned(ffi::wxClipboard_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClipboardIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ClipboardIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClipboardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClipboardIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClipboard_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClipboardIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClipboardTextEvent
wxwidgets! {
    /// This class represents the events generated by a control (typically a wxTextCtrl but other windows can generate these events as well) when its content gets copied or cut to, or pasted from the clipboard.
    ///
    /// [See `wxClipboardTextEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_clipboard_text_event.html)
    #[doc(alias = "wxClipboardTextEvent")]
    #[doc(alias = "ClipboardTextEvent")]
    class ClipboardTextEvent
        = ClipboardTextEventIsOwned<true>(wxClipboardTextEvent) impl
        ClipboardTextEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClipboardTextEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxClipboardTextEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClipboardTextEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ClipboardTextEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ClipboardTextEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClipboardTextEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ClipboardTextEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ClipboardTextEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClipboardTextEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClipboardTextEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClipboardTextEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClipboardTextEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCloseEvent
wxwidgets! {
    /// This event class contains information about window and session close events.
    ///
    /// [See `wxCloseEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_close_event.html)
    #[doc(alias = "wxCloseEvent")]
    #[doc(alias = "CloseEvent")]
    class CloseEvent
        = CloseEventIsOwned<true>(wxCloseEvent) impl
        CloseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CloseEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCloseEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CloseEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CloseEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CloseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CloseEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CloseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CloseEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCloseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CloseEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCollapsiblePane
wxwidgets! {
    /// A collapsible pane is a container with an embedded button-like control which can be used by the user to collapse or expand the pane's contents.
    ///
    /// [See `wxCollapsiblePane`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html)
    #[doc(alias = "wxCollapsiblePane")]
    #[doc(alias = "CollapsiblePane")]
    class CollapsiblePane
        = CollapsiblePaneIsOwned<true>(wxCollapsiblePane) impl
        CollapsiblePaneMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsiblePaneIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCollapsiblePane::wxCollapsiblePane()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#afe29ca6c7b230a05d63022c3adb7348a)
    pub fn new_2step() -> CollapsiblePaneIsOwned<OWNED> {
        unsafe { CollapsiblePaneIsOwned(ffi::wxCollapsiblePane_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// [See `wxCollapsiblePane::wxCollapsiblePane()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#a920561522d4b28c08b8d693047c3aa38)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> CollapsiblePaneIsOwned<OWNED> {
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
            CollapsiblePaneIsOwned(ffi::wxCollapsiblePane_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CollapsiblePaneIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CollapsiblePaneIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsiblePaneIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCollapsiblePane_CLASSINFO()) }
    }
}

// wxCollapsiblePaneEvent
wxwidgets! {
    /// This event class is used for the events generated by wxCollapsiblePane.
    ///
    /// [See `wxCollapsiblePaneEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html)
    #[doc(alias = "wxCollapsiblePaneEvent")]
    #[doc(alias = "CollapsiblePaneEvent")]
    class CollapsiblePaneEvent
        = CollapsiblePaneEventIsOwned<true>(wxCollapsiblePaneEvent) impl
        CollapsiblePaneEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CollapsiblePaneEventIsOwned<OWNED> {
    /// The constructor is not normally used by the user code.
    ///
    /// [See `wxCollapsiblePaneEvent::wxCollapsiblePaneEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html#a4b2a4c27e9908f892be38971b0ddf555)
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        collapsed: bool,
    ) -> CollapsiblePaneEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            CollapsiblePaneEventIsOwned(ffi::wxCollapsiblePaneEvent_new(generator, id, collapsed))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CollapsiblePaneEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: CollapsiblePaneEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CollapsiblePaneEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CollapsiblePaneEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CollapsiblePaneEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CollapsiblePaneEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCollapsiblePaneEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CollapsiblePaneEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColour
wxwidgets! {
    /// A colour is an object representing a combination of Red, Green, and Blue (RGB) intensity values and an Alpha value, and is used to determine drawing colours.
    ///
    /// [See `wxColour`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour.html)
    #[doc(alias = "wxColour")]
    #[doc(alias = "Colour")]
    class Colour
        = ColourIsOwned<true>(wxColour) impl
        ColourMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxColour::wxColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour.html#aac96e7922132d672a1f83d59ecf07343)
    pub fn new() -> ColourIsOwned<OWNED> {
        unsafe { ColourIsOwned(ffi::wxColour_new()) }
    }
    // NOT_SUPPORTED: fn wxColour1()
    ///
    /// [See `wxColour::wxColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour.html#ae114ff88fe3a07477d111baa01c1b325)
    pub fn new_with_str(colour_name: &str) -> ColourIsOwned<OWNED> {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            ColourIsOwned(ffi::wxColour_new2(colour_name))
        }
    }
    // NOT_SUPPORTED: fn wxColour3()
    /// Copy constructor.
    ///
    /// [See `wxColour::wxColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour.html#a0fbc51432a57424a0538d0932af6bf78)
    pub fn new_with_colour<C: ColourMethods>(colour: &C) -> ColourIsOwned<OWNED> {
        unsafe {
            let colour = colour.as_ptr();
            ColourIsOwned(ffi::wxColour_new4(colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColour_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourData
wxwidgets! {
    /// This class holds a variety of information related to colour dialogs.
    ///
    /// [See `wxColourData`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_data.html)
    #[doc(alias = "wxColourData")]
    #[doc(alias = "ColourData")]
    class ColourData
        = ColourDataIsOwned<true>(wxColourData) impl
        ColourDataMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDataIsOwned<OWNED> {
    //  ENUM: @6
    pub const NUM_CUSTOM: c_int = 16;

    /// Constructor.
    ///
    /// [See `wxColourData::wxColourData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a970e44b3f6ee89a4f4621bd76eb3738c)
    pub fn new() -> ColourDataIsOwned<OWNED> {
        unsafe { ColourDataIsOwned(ffi::wxColourData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourDataIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDataIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourData_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxColourDatabase
wxwidgets! {
    /// wxWidgets maintains a database of standard RGB colours for a predefined set of named colours.
    ///
    /// [See `wxColourDatabase`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_database.html)
    #[doc(alias = "wxColourDatabase")]
    #[doc(alias = "ColourDatabase")]
    class ColourDatabase
        = ColourDatabaseIsOwned<true>(wxColourDatabase) impl
        ColourDatabaseMethods
}
impl<const OWNED: bool> ColourDatabaseIsOwned<OWNED> {
    /// Constructs the colour database.
    ///
    /// [See `wxColourDatabase::wxColourDatabase()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_database.html#a66269122cb725da348b3a1c396002e41)
    pub fn new() -> ColourDatabaseIsOwned<OWNED> {
        unsafe { ColourDatabaseIsOwned(ffi::wxColourDatabase_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourDatabaseIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ColourDatabaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxColourDatabase_delete(self.0) }
        }
    }
}

// wxColourDialog
wxwidgets! {
    /// This class represents the colour chooser dialog.
    ///
    /// [See `wxColourDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html)
    #[doc(alias = "wxColourDialog")]
    #[doc(alias = "ColourDialog")]
    class ColourDialog
        = ColourDialogIsOwned<true>(wxColourDialog) impl
        ColourDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourDialogIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxColourDialog::wxColourDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html#a357b4f19cd3757c6f74c44cd49a1d90e)
    pub fn new<W: WindowMethods, C: ColourDataMethods>(
        parent: Option<&W>,
        data: Option<&C>,
    ) -> ColourDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ColourDialogIsOwned(ffi::wxColourDialog_new(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ColourDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourDialog_CLASSINFO()) }
    }
}

// wxColourPickerCtrl
wxwidgets! {
    /// This control allows the user to select a colour.
    ///
    /// [See `wxColourPickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html)
    #[doc(alias = "wxColourPickerCtrl")]
    #[doc(alias = "ColourPickerCtrl")]
    class ColourPickerCtrl
        = ColourPickerCtrlIsOwned<true>(wxColourPickerCtrl) impl
        ColourPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerCtrlIsOwned<OWNED> {
    ///
    /// [See `wxColourPickerCtrl::wxColourPickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a610182eb11286c6df2fbd195b98868cd)
    pub fn new_2step() -> ColourPickerCtrlIsOwned<OWNED> {
        unsafe { ColourPickerCtrlIsOwned(ffi::wxColourPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// [See `wxColourPickerCtrl::wxColourPickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a3763437a33dd948c3ae0d7a2210b7c0e)
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
    ) -> ColourPickerCtrlIsOwned<OWNED> {
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
            ColourPickerCtrlIsOwned(ffi::wxColourPickerCtrl_new1(
                parent, id, colour, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ColourPickerCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourPickerCtrl_CLASSINFO()) }
    }
}

// wxColourPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxColourPickerCtrl.
    ///
    /// [See `wxColourPickerEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html)
    #[doc(alias = "wxColourPickerEvent")]
    #[doc(alias = "ColourPickerEvent")]
    class ColourPickerEvent
        = ColourPickerEventIsOwned<true>(wxColourPickerEvent) impl
        ColourPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ColourPickerEventIsOwned<OWNED> {
    ///
    /// [See `wxColourPickerEvent::wxColourPickerEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#afa8edf53f69e85cbaa6a1f2e91c22f2e)
    pub fn new() -> ColourPickerEventIsOwned<OWNED> {
        unsafe { ColourPickerEventIsOwned(ffi::wxColourPickerEvent_new()) }
    }
    /// The constructor is not normally used by the user code.
    ///
    /// [See `wxColourPickerEvent::wxColourPickerEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#ab9d9e7ef4204c0dc7511eddf4fc4eb0c)
    pub fn new_with_object<O: ObjectMethods, C: ColourMethods>(
        generator: Option<&O>,
        id: c_int,
        colour: &C,
    ) -> ColourPickerEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            ColourPickerEventIsOwned(ffi::wxColourPickerEvent_new1(generator, id, colour))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ColourPickerEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ColourPickerEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ColourPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ColourPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ColourPickerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ColourPickerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ColourPickerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxColourPickerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ColourPickerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxComboBox
wxwidgets! {
    /// A combobox is like a combination of an edit control and a listbox.
    ///
    /// [See `wxComboBox`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_box.html)
    #[doc(alias = "wxComboBox")]
    #[doc(alias = "ComboBox")]
    class ComboBox
        = ComboBoxIsOwned<true>(wxComboBox) impl
        ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ComboBoxIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxComboBox::wxComboBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#aac1f4d94c40a0a675dd5b2c72376057a)
    pub fn new_2step() -> ComboBoxIsOwned<OWNED> {
        unsafe { ComboBoxIsOwned(ffi::wxComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxComboBox1()
    /// Constructor, creating and showing a combobox.
    ///
    /// [See `wxComboBox::wxComboBox()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#ade11ed7d2b64bd1f50cdb34f162e120f)
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
    ) -> ComboBoxIsOwned<OWNED> {
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
            ComboBoxIsOwned(ffi::wxComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ComboBoxIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ComboBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboBox
impl<const OWNED: bool> ItemContainerMethods for ComboBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ComboBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for ComboBoxIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboBox_AsTextEntry(self.as_ptr()) }
    }
}

// wxComboCtrl
wxwidgets! {
    /// A combo control is a generic combobox that allows totally custom popup.
    ///
    /// [See `wxComboCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html)
    #[doc(alias = "wxComboCtrl")]
    #[doc(alias = "ComboCtrl")]
    class ComboCtrl
        = ComboCtrlIsOwned<true>(wxComboCtrl) impl
        ComboCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ComboCtrlIsOwned<OWNED> {
    //  ENUM: @9
    pub const ShowBelow: c_int = 0x0000;
    pub const ShowAbove: c_int = 0x0001;
    pub const CanDeferShow: c_int = 0x0002;

    /// Default constructor.
    ///
    /// [See `wxComboCtrl::wxComboCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#af31b505e73047689dc4d2948eaa13a3d)
    pub fn new_2step() -> ComboCtrlIsOwned<OWNED> {
        unsafe { ComboCtrlIsOwned(ffi::wxComboCtrl_new()) }
    }
    /// Constructor, creating and showing a combo control.
    ///
    /// [See `wxComboCtrl::wxComboCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a0f9d6b21d3728f135dbe0d01ee4bf865)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ComboCtrlIsOwned<OWNED> {
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
            ComboCtrlIsOwned(ffi::wxComboCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ComboCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ComboCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ComboCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ComboCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxComboCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxComboCtrl
impl<const OWNED: bool> TextEntryMethods for ComboCtrlIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxComboCtrl_AsTextEntry(self.as_ptr()) }
    }
}

// wxComboPopup
wxwidgets! {
    /// In order to use a custom popup with wxComboCtrl, an interface class must be derived from wxComboPopup.
    ///
    /// [See `wxComboPopup`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html)
    #[doc(alias = "wxComboPopup")]
    #[doc(alias = "ComboPopup")]
    class ComboPopup
        = ComboPopupIsOwned<true>(wxComboPopup) impl
        ComboPopupMethods
}
impl<const OWNED: bool> ComboPopupIsOwned<OWNED> {
    // BLOCKED: fn wxComboPopup()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ComboPopupIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ComboPopupIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxComboPopup_delete(self.0) }
        }
    }
}

// wxCommand
wxwidgets! {
    /// wxCommand is a base class for modelling an application command, which is an action usually performed by selecting a menu item, pressing a toolbar button or any other means provided by the application to change the data or view.
    ///
    /// [See `wxCommand`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command.html)
    #[doc(alias = "wxCommand")]
    #[doc(alias = "Command")]
    class Command
        = CommandIsOwned<true>(wxCommand) impl
        CommandMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandIsOwned<OWNED> {
    // BLOCKED: fn wxCommand()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommand_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandEvent
wxwidgets! {
    /// This event class contains information about command events, which originate from a variety of simple controls.
    ///
    /// [See `wxCommandEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command_event.html)
    #[doc(alias = "wxCommandEvent")]
    #[doc(alias = "CommandEvent")]
    class CommandEvent
        = CommandEventIsOwned<true>(wxCommandEvent) impl
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCommandEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: CommandEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommandEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCommandLinkButton
wxwidgets! {
    /// Objects of this class are similar in appearance to the normal wxButtons but are similar to the links in a web page in functionality.
    ///
    /// [See `wxCommandLinkButton`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html)
    #[doc(alias = "wxCommandLinkButton")]
    #[doc(alias = "CommandLinkButton")]
    class CommandLinkButton
        = CommandLinkButtonIsOwned<true>(wxCommandLinkButton) impl
        CommandLinkButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandLinkButtonIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCommandLinkButton::wxCommandLinkButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a05be41a7e1fd702cee7662e1a6ae9293)
    pub fn new_2step() -> CommandLinkButtonIsOwned<OWNED> {
        unsafe { CommandLinkButtonIsOwned(ffi::wxCommandLinkButton_new()) }
    }
    /// Constructor really creating a command Link button.
    ///
    /// [See `wxCommandLinkButton::wxCommandLinkButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a193413ad8afa7895bdb0ef133a454234)
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
    ) -> CommandLinkButtonIsOwned<OWNED> {
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
            CommandLinkButtonIsOwned(ffi::wxCommandLinkButton_new1(
                parent, id, main_label, note, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for CommandLinkButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for ButtonIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CommandLinkButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandLinkButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandLinkButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommandLinkButton_CLASSINFO()) }
    }
}

// wxCommandProcessor
wxwidgets! {
    /// wxCommandProcessor is a class that maintains a history of wxCommands, with undo/redo functionality built-in.
    ///
    /// [See `wxCommandProcessor`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command_processor.html)
    #[doc(alias = "wxCommandProcessor")]
    #[doc(alias = "CommandProcessor")]
    class CommandProcessor
        = CommandProcessorIsOwned<true>(wxCommandProcessor) impl
        CommandProcessorMethods,
        ObjectMethods
}
impl<const OWNED: bool> CommandProcessorIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxCommandProcessor::wxCommandProcessor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a8f0b55885b6ea95037e81bbe76e28d74)
    pub fn new(max_commands: c_int) -> CommandProcessorIsOwned<OWNED> {
        unsafe { CommandProcessorIsOwned(ffi::wxCommandProcessor_new(max_commands)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CommandProcessorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CommandProcessorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CommandProcessorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CommandProcessorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCommandProcessor_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CommandProcessorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxContextMenuEvent
wxwidgets! {
    /// This class is used for context menu events, sent to give the application a chance to show a context (popup) menu for a wxWindow.
    ///
    /// [See `wxContextMenuEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_context_menu_event.html)
    #[doc(alias = "wxContextMenuEvent")]
    #[doc(alias = "ContextMenuEvent")]
    class ContextMenuEvent
        = ContextMenuEventIsOwned<true>(wxContextMenuEvent) impl
        ContextMenuEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ContextMenuEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxContextMenuEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ContextMenuEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ContextMenuEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ContextMenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextMenuEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ContextMenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ContextMenuEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ContextMenuEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ContextMenuEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxContextMenuEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ContextMenuEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxControl
wxwidgets! {
    /// This is the base class for a control or "widget".
    ///
    /// [See `wxControl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_control.html)
    #[doc(alias = "wxControl")]
    #[doc(alias = "Control")]
    class Control
        = ControlIsOwned<true>(wxControl) impl
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlIsOwned<OWNED> {
    /// Constructs a control.
    ///
    /// [See `wxControl::wxControl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_control.html#adb8f3edf807efa9159de826bf92d6a44)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ControlIsOwned<OWNED> {
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
            ControlIsOwned(ffi::wxControl_new(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    /// Default constructor to allow 2-phase creation.
    ///
    /// [See `wxControl::wxControl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_control.html#a08428de2ba5cc988a86fe17071d49522)
    pub fn new_2step() -> ControlIsOwned<OWNED> {
        unsafe { ControlIsOwned(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ControlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ControlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ControlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ControlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ControlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ControlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxControl_CLASSINFO()) }
    }
}

// wxControlWithItems
wxwidgets! {
    /// This is convenience class that derives from both wxControl and wxItemContainer.
    ///
    /// [See `wxControlWithItems`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_control_with_items.html)
    #[doc(alias = "wxControlWithItems")]
    #[doc(alias = "ControlWithItems")]
    class ControlWithItems
        = ControlWithItemsIsOwned<true>(wxControlWithItems) impl
        ControlWithItemsMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ControlWithItemsIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ControlWithItemsIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ControlWithItemsIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ControlWithItemsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ControlWithItemsIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxControlWithItems_CLASSINFO()) }
    }
}
// Mix-in(s) to wxControlWithItems
impl<const OWNED: bool> ItemContainerMethods for ControlWithItemsIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for ControlWithItemsIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxControlWithItems_AsItemContainer(self.as_ptr()) }
    }
}

// wxCursor
wxwidgets! {
    /// A cursor is a small bitmap usually used for denoting where the mouse pointer is, with a picture that might indicate the interpretation of a mouse click.
    ///
    /// [See `wxCursor`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_cursor.html)
    #[doc(alias = "wxCursor")]
    #[doc(alias = "Cursor")]
    class Cursor
        = CursorIsOwned<true>(wxCursor) impl
        CursorMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> CursorIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxCursor::wxCursor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a21735e923430faf941c440c67f13859d)
    pub fn new() -> CursorIsOwned<OWNED> {
        unsafe { CursorIsOwned(ffi::wxCursor_new()) }
    }
    // NOT_SUPPORTED: fn wxCursor1()
    // NOT_SUPPORTED: fn wxCursor2()
    // NOT_SUPPORTED: fn wxCursor3()
    /// Constructs a cursor from a wxImage.
    ///
    /// [See `wxCursor::wxCursor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_cursor.html#adaf6e394c7306a3999bdc314d7b307a7)
    pub fn new_with_image<I: ImageMethods>(image: &I) -> CursorIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            CursorIsOwned(ffi::wxCursor_new4(image))
        }
    }
    /// Constructs a cursor from XPM data.
    ///
    /// [See `wxCursor::wxCursor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_cursor.html#abb43549ff688898c79f48112ff85f7d7)
    pub fn new_with_char(xpm_data: *const c_void) -> CursorIsOwned<OWNED> {
        unsafe { CursorIsOwned(ffi::wxCursor_new5(xpm_data)) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// [See `wxCursor::wxCursor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a20f3123d9d012427ae1f614e10b9cfb9)
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> CursorIsOwned<OWNED> {
        unsafe {
            let cursor = cursor.as_ptr();
            CursorIsOwned(ffi::wxCursor_new6(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CursorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CursorIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: CursorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CursorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: CursorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for CursorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxCursor_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for CursorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCustomDataObject
wxwidgets! {
    /// wxCustomDataObject is a specialization of wxDataObjectSimple for some application-specific data in arbitrary (either custom or one of the standard ones).
    ///
    /// [See `wxCustomDataObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html)
    #[doc(alias = "wxCustomDataObject")]
    #[doc(alias = "CustomDataObject")]
    class CustomDataObject
        = CustomDataObjectIsOwned<true>(wxCustomDataObject) impl
        CustomDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> CustomDataObjectIsOwned<OWNED> {
    /// The constructor accepts a format argument which specifies the (single) format supported by this object.
    ///
    /// [See `wxCustomDataObject::wxCustomDataObject()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a667ea5eae3e91095d79cb6fe9e548695)
    pub fn new<D: DataFormatMethods>(format: &D) -> CustomDataObjectIsOwned<OWNED> {
        unsafe {
            let format = format.as_ptr();
            CustomDataObjectIsOwned(ffi::wxCustomDataObject_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for CustomDataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<CustomDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: CustomDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<CustomDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: CustomDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for CustomDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCustomDataObject_delete(self.0) }
        }
    }
}
