#![allow(non_upper_case_globals)]

use super::*;

// wxTextAttr
wx_class! { TextAttr =
    TextAttrIsOwned<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const OWNED: bool> TextAttrIsOwned<OWNED> {
    pub fn new() -> TextAttrIsOwned<OWNED> {
        unsafe { TextAttrIsOwned(ffi::wxTextAttr_new()) }
    }
    // NOT_SUPPORTED: fn wxTextAttr1()
    pub fn new_with_textattr<T: TextAttrMethods>(attr: &T) -> TextAttrIsOwned<OWNED> {
        unsafe {
            let attr = attr.as_ptr();
            TextAttrIsOwned(ffi::wxTextAttr_new2(attr))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TextAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextAttr_delete(self.0) }
        }
    }
}

// wxTextCtrl
wx_class! { TextCtrl =
    TextCtrlIsOwned<true>(wxTextCtrl) impl
        TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TextCtrlIsOwned<OWNED> {
        unsafe { TextCtrlIsOwned(ffi::wxTextCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TextCtrlIsOwned<OWNED> {
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
            TextCtrlIsOwned(ffi::wxTextCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TextCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTextCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTextCtrl
impl<const OWNED: bool> TextEntryMethods for TextCtrlIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxTextCtrl_AsTextEntry(self.as_ptr()) }
    }
}

// wxTextEntry
wx_class! { TextEntry =
    TextEntryIsOwned<true>(wxTextEntry) impl
        TextEntryMethods
}
impl<const OWNED: bool> TextEntryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TextEntryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextEntry_delete(self.0) }
        }
    }
}

// wxTextEntryDialog
wx_class! { TextEntryDialog =
    TextEntryDialogIsOwned<true>(wxTextEntryDialog) impl
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextEntryDialogIsOwned<OWNED> {
    pub fn new_2step() -> TextEntryDialogIsOwned<OWNED> {
        unsafe { TextEntryDialogIsOwned(ffi::wxTextEntryDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        value: &str,
        style: c_long,
        pos: &P,
    ) -> TextEntryDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            TextEntryDialogIsOwned(ffi::wxTextEntryDialog_new1(
                parent, message, caption, value, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TextEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextEntryDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTextEntryDialog_CLASSINFO()) }
    }
}

// wxTimePickerCtrl
wx_class! { TimePickerCtrl =
    TimePickerCtrlIsOwned<true>(wxTimePickerCtrl) impl
        TimePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> TimePickerCtrlIsOwned<OWNED> {
        unsafe { TimePickerCtrlIsOwned(ffi::wxTimePickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        dt: &D,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TimePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TimePickerCtrlIsOwned(ffi::wxTimePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TimePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTimePickerCtrl_CLASSINFO()) }
    }
}

// wxToggleButton
wx_class! { ToggleButton =
    ToggleButtonIsOwned<true>(wxToggleButton) impl
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToggleButtonIsOwned<OWNED> {
    pub fn new_2step() -> ToggleButtonIsOwned<OWNED> {
        unsafe { ToggleButtonIsOwned(ffi::wxToggleButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
        name: &str,
    ) -> ToggleButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let val = val.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToggleButtonIsOwned(ffi::wxToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToggleButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToggleButton_CLASSINFO()) }
    }
}

// wxToolBar
wx_class! { ToolBar =
    ToolBarIsOwned<true>(wxToolBar) impl
        ToolBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolBarIsOwned<OWNED> {
    pub fn new_2step() -> ToolBarIsOwned<OWNED> {
        unsafe { ToolBarIsOwned(ffi::wxToolBar_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolBarIsOwned(ffi::wxToolBar_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToolBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToolBar_CLASSINFO()) }
    }
}

// wxTopLevelWindow
wx_class! { TopLevelWindow =
    TopLevelWindowIsOwned<true>(wxTopLevelWindow) impl
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TopLevelWindowIsOwned<OWNED> {
    pub fn new_2step() -> TopLevelWindowIsOwned<OWNED> {
        unsafe { TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TopLevelWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TopLevelWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TopLevelWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTopLevelWindow_CLASSINFO()) }
    }
}
