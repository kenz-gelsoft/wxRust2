use super::*;

// wxTGAHandler
wxwidgets! {
    /// This is the image handler for the TGA format.
    ///
    /// [See `wxTGAHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html)
    #[doc(alias = "wxTGAHandler")]
    #[doc(alias = "TGAHandler")]
    class TGAHandler
        = TGAHandlerIsOwned<true>(wxTGAHandler) impl
        TGAHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TGAHandlerIsOwned<OWNED> {
    /// Default constructor for wxTGAHandler.
    pub fn new() -> TGAHandlerIsOwned<OWNED> {
        unsafe { TGAHandlerIsOwned(ffi::wxTGAHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TGAHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TGAHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: TGAHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TGAHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TGAHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TGAHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTGAHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TGAHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTIFFHandler
wxwidgets! {
    /// This is the image handler for the TIFF format.
    ///
    /// [See `wxTIFFHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html)
    #[doc(alias = "wxTIFFHandler")]
    #[doc(alias = "TIFFHandler")]
    class TIFFHandler
        = TIFFHandlerIsOwned<true>(wxTIFFHandler) impl
        TIFFHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TIFFHandlerIsOwned<OWNED> {
    /// Default constructor for wxTIFFHandler.
    pub fn new() -> TIFFHandlerIsOwned<OWNED> {
        unsafe { TIFFHandlerIsOwned(ffi::wxTIFFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TIFFHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TIFFHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: TIFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TIFFHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TIFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TIFFHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTIFFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TIFFHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> ImageHandlerMethods for TIFFHandlerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxTaskBarIcon
wxwidgets! {
    /// This class represents a taskbar icon.
    ///
    /// [See `wxTaskBarIcon`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html)
    #[doc(alias = "wxTaskBarIcon")]
    #[doc(alias = "TaskBarIcon")]
    class TaskBarIcon
        = TaskBarIconIsOwned<true>(wxTaskBarIcon) impl
        TaskBarIconMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TaskBarIconIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarIcon()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TaskBarIconIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TaskBarIconIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TaskBarIconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TaskBarIconIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TaskBarIconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TaskBarIconIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTaskBarIcon_CLASSINFO()) }
    }
}

// wxTaskBarIconEvent
wxwidgets! {
    /// The event class used by wxTaskBarIcon.
    ///
    /// [See `wxTaskBarIconEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon_event.html)
    #[doc(alias = "wxTaskBarIconEvent")]
    #[doc(alias = "TaskBarIconEvent")]
    class TaskBarIconEvent
        = TaskBarIconEventIsOwned<true>(wxTaskBarIconEvent) impl
        TaskBarIconEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TaskBarIconEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarIconEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TaskBarIconEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TaskBarIconEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TaskBarIconEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TaskBarIconEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TaskBarIconEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TaskBarIconEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTaskBarIconEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TaskBarIconEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTextAttr
wxwidgets! {
    /// wxTextAttr represents the character and paragraph attributes, or style, for a range of text in a wxTextCtrl or wxRichTextCtrl.
    ///
    /// [See `wxTextAttr`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html)
    #[doc(alias = "wxTextAttr")]
    #[doc(alias = "TextAttr")]
    class TextAttr
        = TextAttrIsOwned<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const OWNED: bool> TextAttrIsOwned<OWNED> {
    /// Constructors.
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
impl Clone for TextAttrIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A text control allows text to be displayed and edited.
    ///
    /// [See `wxTextCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html)
    #[doc(alias = "wxTextCtrl")]
    #[doc(alias = "TextCtrl")]
    class TextCtrl
        = TextCtrlIsOwned<true>(wxTextCtrl) impl
        TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextCtrlIsOwned<OWNED> {
    /// Default ctor.
    pub fn new_2step() -> TextCtrlIsOwned<OWNED> {
        unsafe { TextCtrlIsOwned(ffi::wxTextCtrl_new()) }
    }
    /// Constructor, creating and showing a text control.
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
impl<const OWNED: bool> Clone for TextCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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

// wxTextDataObject
wxwidgets! {
    /// wxTextDataObject is a specialization of wxDataObjectSimple for text data.
    ///
    /// [See `wxTextDataObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html)
    #[doc(alias = "wxTextDataObject")]
    #[doc(alias = "TextDataObject")]
    class TextDataObject
        = TextDataObjectIsOwned<true>(wxTextDataObject) impl
        TextDataObjectMethods,
        // DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> TextDataObjectIsOwned<OWNED> {
    /// Constructor, may be used to initialise the text (otherwise SetText() should be used later).
    pub fn new(text: &str) -> TextDataObjectIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            TextDataObjectIsOwned(ffi::wxTextDataObject_new(text))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextDataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: TextDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: TextDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextDataObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> DataObjectSimpleMethods for TextDataObjectIsOwned<OWNED> {
    // BLOCKED: fn GetFormat()
}

// wxTextDropTarget
wxwidgets! {
    /// A predefined drop target for dealing with text data.
    ///
    /// [See `wxTextDropTarget`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_drop_target.html)
    #[doc(alias = "wxTextDropTarget")]
    #[doc(alias = "TextDropTarget")]
    class TextDropTarget
        = TextDropTargetIsOwned<true>(wxTextDropTarget) impl
        TextDropTargetMethods,
        DropTargetMethods
}
impl<const OWNED: bool> TextDropTargetIsOwned<OWNED> {
    // BLOCKED: fn wxTextDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextDropTargetIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextDropTargetIsOwned<OWNED>> for DropTargetIsOwned<OWNED> {
    fn from(o: TextDropTargetIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextDropTargetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextDropTarget_delete(self.0) }
        }
    }
}

// wxTextEntry
wxwidgets! {
    /// Common base class for single line text entry fields.
    ///
    /// [See `wxTextEntry`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html)
    #[doc(alias = "wxTextEntry")]
    #[doc(alias = "TextEntry")]
    class TextEntry
        = TextEntryIsOwned<true>(wxTextEntry) impl
        TextEntryMethods
}
impl<const OWNED: bool> TextEntryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextEntryIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This class represents a dialog that requests a one-line text string from the user.
    ///
    /// [See `wxTextEntryDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html)
    #[doc(alias = "wxTextEntryDialog")]
    #[doc(alias = "TextEntryDialog")]
    class TextEntryDialog
        = TextEntryDialogIsOwned<true>(wxTextEntryDialog) impl
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextEntryDialogIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> TextEntryDialogIsOwned<OWNED> {
        unsafe { TextEntryDialogIsOwned(ffi::wxTextEntryDialog_new()) }
    }
    /// Constructor.
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
impl<const OWNED: bool> Clone for TextEntryDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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

// wxTextValidator
wxwidgets! {
    /// wxTextValidator validates text controls, providing a variety of filtering behaviours.
    ///
    /// [See `wxTextValidator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html)
    #[doc(alias = "wxTextValidator")]
    #[doc(alias = "TextValidator")]
    class TextValidator
        = TextValidatorIsOwned<true>(wxTextValidator) impl
        TextValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextValidatorIsOwned<OWNED> {
    /// Copy constructor.
    pub fn new_with_textvalidator<T: TextValidatorMethods>(
        validator: &T,
    ) -> TextValidatorIsOwned<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            TextValidatorIsOwned(ffi::wxTextValidator_new(validator))
        }
    }
    /// Constructor taking a style and optional pointer to a wxString variable.
    pub fn new_with_long(style: c_long, val_ptr: *mut c_void) -> TextValidatorIsOwned<OWNED> {
        unsafe { TextValidatorIsOwned(ffi::wxTextValidator_new1(style, val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TextValidatorIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextValidatorIsOwned<OWNED>> for ValidatorIsOwned<OWNED> {
    fn from(o: TextValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextValidatorIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TextValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextValidatorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TextValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextValidatorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTextValidator_CLASSINFO()) }
    }
}

// wxThreadEvent
wxwidgets! {
    /// This class adds some simple functionality to wxEvent to facilitate inter-thread communication.
    ///
    /// [See `wxThreadEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html)
    #[doc(alias = "wxThreadEvent")]
    #[doc(alias = "ThreadEvent")]
    class ThreadEvent
        = ThreadEventIsOwned<true>(wxThreadEvent) impl
        ThreadEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ThreadEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxThreadEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ThreadEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ThreadEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ThreadEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ThreadEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ThreadEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ThreadEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxThreadEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ThreadEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTimePickerCtrl
wxwidgets! {
    /// This control allows the user to enter time.
    ///
    /// [See `wxTimePickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html)
    #[doc(alias = "wxTimePickerCtrl")]
    #[doc(alias = "TimePickerCtrl")]
    class TimePickerCtrl
        = TimePickerCtrlIsOwned<true>(wxTimePickerCtrl) impl
        TimePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimePickerCtrlIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> TimePickerCtrlIsOwned<OWNED> {
        unsafe { TimePickerCtrlIsOwned(ffi::wxTimePickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
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
impl<const OWNED: bool> Clone for TimePickerCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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

// wxTipProvider
wxwidgets! {
    /// This is the class used together with wxShowTip() function.
    ///
    /// [See `wxTipProvider`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html)
    #[doc(alias = "wxTipProvider")]
    #[doc(alias = "TipProvider")]
    class TipProvider
        = TipProviderIsOwned<true>(wxTipProvider) impl
        TipProviderMethods
}
impl<const OWNED: bool> TipProviderIsOwned<OWNED> {
    // BLOCKED: fn wxTipProvider()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TipProviderIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TipProviderIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTipProvider_delete(self.0) }
        }
    }
}

// wxTipWindow
wxwidgets! {
    /// Shows simple text in a popup tip window on creation.
    ///
    /// [See `wxTipWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_window.html)
    #[doc(alias = "wxTipWindow")]
    #[doc(alias = "TipWindow")]
    class TipWindow
        = TipWindowIsOwned<true>(wxTipWindow) impl
        TipWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TipWindowIsOwned<OWNED> {
    /// Constructor.
    pub fn new<W: WindowMethods, T: TipWindowMethods, R: RectMethods>(
        parent: Option<&W>,
        text: &str,
        max_length: c_int,
        window_ptr: Option<&T>,
        rect_bounds: Option<&R>,
    ) -> TipWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let window_ptr = match window_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect_bounds = match rect_bounds {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TipWindowIsOwned(ffi::wxTipWindow_new(
                parent,
                text,
                max_length,
                window_ptr,
                rect_bounds,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TipWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TipWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TipWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TipWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TipWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TipWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TipWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TipWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTipWindow_CLASSINFO()) }
    }
}

// wxToggleButton
wxwidgets! {
    /// wxToggleButton is a button that stays pressed when clicked by the user.
    ///
    /// [See `wxToggleButton`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html)
    #[doc(alias = "wxToggleButton")]
    #[doc(alias = "ToggleButton")]
    class ToggleButton
        = ToggleButtonIsOwned<true>(wxToggleButton) impl
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToggleButtonIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> ToggleButtonIsOwned<OWNED> {
        unsafe { ToggleButtonIsOwned(ffi::wxToggleButton_new()) }
    }
    /// Constructor, creating and showing a toggle button.
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
impl<const OWNED: bool> Clone for ToggleButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A toolbar is a bar of buttons and/or other controls usually placed below the menu bar in a wxFrame.
    ///
    /// [See `wxToolBar`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html)
    #[doc(alias = "wxToolBar")]
    #[doc(alias = "ToolBar")]
    class ToolBar
        = ToolBarIsOwned<true>(wxToolBar) impl
        ToolBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolBarIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> ToolBarIsOwned<OWNED> {
        unsafe { ToolBarIsOwned(ffi::wxToolBar_new()) }
    }
    /// Constructs a toolbar.
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
impl<const OWNED: bool> Clone for ToolBarIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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

// wxToolTip
wxwidgets! {
    /// This class holds information about a tooltip associated with a window (see wxWindow::SetToolTip()).
    ///
    /// [See `wxToolTip`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html)
    #[doc(alias = "wxToolTip")]
    #[doc(alias = "ToolTip")]
    class ToolTip
        = ToolTipIsOwned<true>(wxToolTip) impl
        ToolTipMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolTipIsOwned<OWNED> {
    /// Constructor.
    pub fn new(tip: &str) -> ToolTipIsOwned<OWNED> {
        unsafe {
            let tip = WxString::from(tip);
            let tip = tip.as_ptr();
            ToolTipIsOwned(ffi::wxToolTip_new(tip))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ToolTipIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ToolTipIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToolTipIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolTipIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToolTip_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ToolTipIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxToolbook
wxwidgets! {
    /// wxToolbook is a class similar to wxNotebook but which uses a wxToolBar to show the labels instead of the tabs.
    ///
    /// [See `wxToolbook`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toolbook.html)
    #[doc(alias = "wxToolbook")]
    #[doc(alias = "Toolbook")]
    class Toolbook
        = ToolbookIsOwned<true>(wxToolbook) impl
        ToolbookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolbookIsOwned<OWNED> {
    /// Constructs a choicebook control.
    pub fn new_2step() -> ToolbookIsOwned<OWNED> {
        unsafe { ToolbookIsOwned(ffi::wxToolbook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolbookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolbookIsOwned(ffi::wxToolbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ToolbookIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ToolbookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolbookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxToolbook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for ToolbookIsOwned<OWNED> {
    /// Create the tool book control that has already been constructed with the default constructor.
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
            ffi::wxToolbook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxTopLevelWindow
wxwidgets! {
    /// wxTopLevelWindow is a common base class for wxDialog and wxFrame.
    ///
    /// [See `wxTopLevelWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html)
    #[doc(alias = "wxTopLevelWindow")]
    #[doc(alias = "TopLevelWindow")]
    class TopLevelWindow
        = TopLevelWindowIsOwned<true>(wxTopLevelWindow) impl
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TopLevelWindowIsOwned<OWNED> {
    /// Default ctor.
    pub fn new_2step() -> TopLevelWindowIsOwned<OWNED> {
        unsafe { TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new()) }
    }
    /// Constructor creating the top level window.
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
impl<const OWNED: bool> Clone for TopLevelWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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

// wxTreeCtrl
wxwidgets! {
    /// A tree control presents information as a hierarchy, with items that may be expanded to show further items.
    ///
    /// [See `wxTreeCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html)
    #[doc(alias = "wxTreeCtrl")]
    #[doc(alias = "TreeCtrl")]
    class TreeCtrl
        = TreeCtrlIsOwned<true>(wxTreeCtrl) impl
        TreeCtrlMethods,
        // ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeCtrlIsOwned<OWNED> {
    /// Default Constructor.
    pub fn new_2step() -> TreeCtrlIsOwned<OWNED> {
        unsafe { TreeCtrlIsOwned(ffi::wxTreeCtrl_new()) }
    }
    /// Constructor, creating and showing a tree control.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TreeCtrlIsOwned<OWNED> {
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
            TreeCtrlIsOwned(ffi::wxTreeCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TreeCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreeCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for TreeCtrlIsOwned<OWNED> {
    /// Creates the tree control.
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTreeCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}
impl<const OWNED: bool> WindowMethods for TreeCtrlIsOwned<OWNED> {
    /// Sets the mode flags associated with the display of the tree control.
    fn set_window_style(&self, styles: c_long) {
        unsafe { ffi::wxTreeCtrl_SetWindowStyle(self.as_ptr(), styles) }
    }
}

// wxTreeEvent
wxwidgets! {
    /// A tree event holds information about events associated with wxTreeCtrl objects.
    ///
    /// [See `wxTreeEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html)
    #[doc(alias = "wxTreeEvent")]
    #[doc(alias = "TreeEvent")]
    class TreeEvent
        = TreeEventIsOwned<true>(wxTreeEvent) impl
        TreeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxTreeEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TreeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTreeItemData
wxwidgets! {
    /// wxTreeItemData is some (arbitrary) user class associated with some item.
    ///
    /// [See `wxTreeItemData`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html)
    #[doc(alias = "wxTreeItemData")]
    #[doc(alias = "TreeItemData")]
    class TreeItemData
        = TreeItemDataIsOwned<true>(wxTreeItemData) impl
        TreeItemDataMethods,
        ClientDataMethods
}
impl<const OWNED: bool> TreeItemDataIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> TreeItemDataIsOwned<OWNED> {
        unsafe { TreeItemDataIsOwned(ffi::wxTreeItemData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeItemDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeItemDataIsOwned<OWNED>> for ClientDataIsOwned<OWNED> {
    fn from(o: TreeItemDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TreeItemDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeItemData_delete(self.0) }
        }
    }
}

// wxTreeItemId
wxwidgets! {
    /// An opaque reference to a tree item.
    ///
    /// [See `wxTreeItemId`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html)
    #[doc(alias = "wxTreeItemId")]
    #[doc(alias = "TreeItemId")]
    class TreeItemId
        = TreeItemIdIsOwned<true>(wxTreeItemId) impl
        TreeItemIdMethods
}
impl<const OWNED: bool> TreeItemIdIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> TreeItemIdIsOwned<OWNED> {
        unsafe { TreeItemIdIsOwned(ffi::wxTreeItemId_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeItemIdIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TreeItemIdIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeItemId_delete(self.0) }
        }
    }
}

// wxTreeListCtrl
wxwidgets! {
    /// A control combining wxTreeCtrl and wxListCtrl features.
    ///
    /// [See `wxTreeListCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html)
    #[doc(alias = "wxTreeListCtrl")]
    #[doc(alias = "TreeListCtrl")]
    class TreeListCtrl
        = TreeListCtrlIsOwned<true>(wxTreeListCtrl) impl
        TreeListCtrlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeListCtrlIsOwned<OWNED> {
    /// Default constructor, call Create() later.
    pub fn new_2step() -> TreeListCtrlIsOwned<OWNED> {
        unsafe { TreeListCtrlIsOwned(ffi::wxTreeListCtrl_new()) }
    }
    /// Full constructing, creating the object and its window.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TreeListCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreeListCtrlIsOwned(ffi::wxTreeListCtrl_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TreeListCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeListCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TreeListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeListCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TreeListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeListCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreeListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeListCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreeListCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for TreeListCtrlIsOwned<OWNED> {
    /// Create the control window.
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
            ffi::wxTreeListCtrl_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxTreeListItem
wxwidgets! {
    /// Unique identifier of an item in wxTreeListCtrl.
    ///
    /// [See `wxTreeListItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html)
    #[doc(alias = "wxTreeListItem")]
    #[doc(alias = "TreeListItem")]
    class TreeListItem
        = TreeListItemIsOwned<true>(wxTreeListItem) impl
        TreeListItemMethods
}
impl<const OWNED: bool> TreeListItemIsOwned<OWNED> {
    /// Only the default constructor is publicly accessible.
    pub fn new() -> TreeListItemIsOwned<OWNED> {
        unsafe { TreeListItemIsOwned(ffi::wxTreeListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeListItemIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TreeListItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeListItem_delete(self.0) }
        }
    }
}

// wxTreeListItemComparator
wxwidgets! {
    /// Class defining sort order for the items in wxTreeListCtrl.
    ///
    /// [See `wxTreeListItemComparator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_item_comparator.html)
    #[doc(alias = "wxTreeListItemComparator")]
    #[doc(alias = "TreeListItemComparator")]
    class TreeListItemComparator
        = TreeListItemComparatorIsOwned<true>(wxTreeListItemComparator) impl
        TreeListItemComparatorMethods
}
impl<const OWNED: bool> TreeListItemComparatorIsOwned<OWNED> {
    // BLOCKED: fn wxTreeListItemComparator()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeListItemComparatorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TreeListItemComparatorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeListItemComparator_delete(self.0) }
        }
    }
}

// wxTreebook
wxwidgets! {
    /// This class is an extension of the wxNotebook class that allows a tree structured set of pages to be shown in a control.
    ///
    /// [See `wxTreebook`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html)
    #[doc(alias = "wxTreebook")]
    #[doc(alias = "Treebook")]
    class Treebook
        = TreebookIsOwned<true>(wxTreebook) impl
        TreebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreebookIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> TreebookIsOwned<OWNED> {
        unsafe { TreebookIsOwned(ffi::wxTreebook_new()) }
    }
    /// Creates an empty wxTreebook.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TreebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreebookIsOwned(ffi::wxTreebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TreebookIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TreebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTreebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for TreebookIsOwned<OWNED> {
    /// Creates a treebook control.
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
            ffi::wxTreebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}
