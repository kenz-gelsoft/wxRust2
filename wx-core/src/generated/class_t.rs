use super::*;

// wxTGAHandler
wxwidgets! {
    /// This is the image handler for the TGA format.
    /// - [`TGAHandler`] represents a C++ `wxTGAHandler` class instance which your code has ownership, [`TGAHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`TGAHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTGAHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html) for more details.
    #[doc(alias = "wxTGAHandler")]
    #[doc(alias = "TGAHandler")]
    class TGAHandler
        = TGAHandlerInRust<true>(wxTGAHandler) impl
        TGAHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TGAHandlerInRust<OWNED> {
    /// Default constructor for wxTGAHandler.
    ///
    /// See [C++ `wxTGAHandler::wxTGAHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html#a5258c1de737fd77d505c06a7f822000a).
    pub fn new() -> TGAHandlerInRust<OWNED> {
        unsafe { TGAHandlerInRust(ffi::wxTGAHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TGAHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TGAHandlerInRust<OWNED>> for ImageHandlerInRust<OWNED> {
    fn from(o: TGAHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TGAHandlerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TGAHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TGAHandlerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTGAHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TGAHandlerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTIFFHandler
wxwidgets! {
    /// This is the image handler for the TIFF format.
    /// - [`TIFFHandler`] represents a C++ `wxTIFFHandler` class instance which your code has ownership, [`TIFFHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`TIFFHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTIFFHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html) for more details.
    #[doc(alias = "wxTIFFHandler")]
    #[doc(alias = "TIFFHandler")]
    class TIFFHandler
        = TIFFHandlerInRust<true>(wxTIFFHandler) impl
        TIFFHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TIFFHandlerInRust<OWNED> {
    /// Default constructor for wxTIFFHandler.
    ///
    /// See [C++ `wxTIFFHandler::wxTIFFHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html#a8ff101a3e846cdfaf3170d54805d05c8).
    pub fn new() -> TIFFHandlerInRust<OWNED> {
        unsafe { TIFFHandlerInRust(ffi::wxTIFFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TIFFHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TIFFHandlerInRust<OWNED>> for ImageHandlerInRust<OWNED> {
    fn from(o: TIFFHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TIFFHandlerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TIFFHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TIFFHandlerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTIFFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TIFFHandlerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> ImageHandlerMethods for TIFFHandlerInRust<OWNED> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxTaskBarIcon
wxwidgets! {
    /// This class represents a taskbar icon.
    /// - [`TaskBarIcon`] represents a C++ `wxTaskBarIcon` class instance which your code has ownership, [`TaskBarIconInRust`]`<false>` represents one which don't own.
    /// - Use [`TaskBarIcon`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTaskBarIcon` class's documentation](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html) for more details.
    #[doc(alias = "wxTaskBarIcon")]
    #[doc(alias = "TaskBarIcon")]
    class TaskBarIcon
        = TaskBarIconInRust<true>(wxTaskBarIcon) impl
        TaskBarIconMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TaskBarIconInRust<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarIcon()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TaskBarIconInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TaskBarIconInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TaskBarIconInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TaskBarIconInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TaskBarIconInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TaskBarIconInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTaskBarIcon_CLASSINFO()) }
    }
}

// wxTaskBarIconEvent
wxwidgets! {
    /// The event class used by wxTaskBarIcon.
    /// - [`TaskBarIconEvent`] represents a C++ `wxTaskBarIconEvent` class instance which your code has ownership, [`TaskBarIconEventInRust`]`<false>` represents one which don't own.
    /// - Use [`TaskBarIconEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTaskBarIconEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon_event.html) for more details.
    #[doc(alias = "wxTaskBarIconEvent")]
    #[doc(alias = "TaskBarIconEvent")]
    class TaskBarIconEvent
        = TaskBarIconEventInRust<true>(wxTaskBarIconEvent) impl
        TaskBarIconEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TaskBarIconEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxTaskBarIconEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TaskBarIconEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TaskBarIconEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: TaskBarIconEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TaskBarIconEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TaskBarIconEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TaskBarIconEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTaskBarIconEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TaskBarIconEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTextAttr
wxwidgets! {
    /// wxTextAttr represents the character and paragraph attributes, or style, for a range of text in a wxTextCtrl or wxRichTextCtrl.
    /// - [`TextAttr`] represents a C++ `wxTextAttr` class instance which your code has ownership, [`TextAttrInRust`]`<false>` represents one which don't own.
    /// - Use [`TextAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html) for more details.
    #[doc(alias = "wxTextAttr")]
    #[doc(alias = "TextAttr")]
    class TextAttr
        = TextAttrInRust<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const OWNED: bool> TextAttrInRust<OWNED> {
    /// Constructors.
    ///
    /// See [C++ `wxTextAttr::wxTextAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a7870ad57a4430b83a9478adeda20c6ec).
    pub fn new() -> TextAttrInRust<OWNED> {
        unsafe { TextAttrInRust(ffi::wxTextAttr_new()) }
    }
    // NOT_SUPPORTED: fn wxTextAttr1()
    ///
    /// See [C++ `wxTextAttr::wxTextAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a56e913249eb032aa4184f26ebdbca454).
    pub fn new_with_textattr<T: TextAttrMethods>(attr: &T) -> TextAttrInRust<OWNED> {
        unsafe {
            let attr = attr.as_ptr();
            TextAttrInRust(ffi::wxTextAttr_new2(attr))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextAttrInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TextAttrInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextAttr_delete(self.0) }
        }
    }
}

// wxTextCtrl
wxwidgets! {
    /// A text control allows text to be displayed and edited.
    /// - [`TextCtrl`] represents a C++ `wxTextCtrl` class instance which your code has ownership, [`TextCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`TextCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html) for more details.
    #[doc(alias = "wxTextCtrl")]
    #[doc(alias = "TextCtrl")]
    class TextCtrl
        = TextCtrlInRust<true>(wxTextCtrl) impl
        TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextCtrlInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxTextCtrl::wxTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a4d01be66f9a9e95501aa55d54f94d54f).
    pub fn new_2step() -> TextCtrlInRust<OWNED> {
        unsafe { TextCtrlInRust(ffi::wxTextCtrl_new()) }
    }
    /// Constructor, creating and showing a text control.
    ///
    /// See [C++ `wxTextCtrl::wxTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a4f398e4800a904fdf225fabc366d7910).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TextCtrlInRust<OWNED> {
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
            TextCtrlInRust(ffi::wxTextCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TextCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: TextCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TextCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TextCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TextCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTextCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTextCtrl
impl<const OWNED: bool> TextEntryMethods for TextCtrlInRust<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxTextCtrl_AsTextEntry(self.as_ptr()) }
    }
}

// wxTextDataObject
wxwidgets! {
    /// wxTextDataObject is a specialization of wxDataObjectSimple for text data.
    /// - [`TextDataObject`] represents a C++ `wxTextDataObject` class instance which your code has ownership, [`TextDataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`TextDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html) for more details.
    #[doc(alias = "wxTextDataObject")]
    #[doc(alias = "TextDataObject")]
    class TextDataObject
        = TextDataObjectInRust<true>(wxTextDataObject) impl
        TextDataObjectMethods,
        // DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> TextDataObjectInRust<OWNED> {
    /// Constructor, may be used to initialise the text (otherwise SetText() should be used later).
    ///
    /// See [C++ `wxTextDataObject::wxTextDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html#a1d1c7457cb2b89f2df0a11129344c078).
    pub fn new(text: &str) -> TextDataObjectInRust<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            TextDataObjectInRust(ffi::wxTextDataObject_new(text))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextDataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextDataObjectInRust<OWNED>> for DataObjectSimpleInRust<OWNED> {
    fn from(o: TextDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextDataObjectInRust<OWNED>> for DataObjectInRust<OWNED> {
    fn from(o: TextDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextDataObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextDataObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> DataObjectSimpleMethods for TextDataObjectInRust<OWNED> {
    // BLOCKED: fn GetFormat()
}

// wxTextDropTarget
wxwidgets! {
    /// A predefined drop target for dealing with text data.
    /// - [`TextDropTarget`] represents a C++ `wxTextDropTarget` class instance which your code has ownership, [`TextDropTargetInRust`]`<false>` represents one which don't own.
    /// - Use [`TextDropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_drop_target.html) for more details.
    #[doc(alias = "wxTextDropTarget")]
    #[doc(alias = "TextDropTarget")]
    class TextDropTarget
        = TextDropTargetInRust<true>(wxTextDropTarget) impl
        TextDropTargetMethods,
        DropTargetMethods
}
impl<const OWNED: bool> TextDropTargetInRust<OWNED> {
    // BLOCKED: fn wxTextDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextDropTargetInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextDropTargetInRust<OWNED>> for DropTargetInRust<OWNED> {
    fn from(o: TextDropTargetInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TextDropTargetInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextDropTarget_delete(self.0) }
        }
    }
}

// wxTextEntry
wxwidgets! {
    /// Common base class for single line text entry fields.
    /// - [`TextEntry`] represents a C++ `wxTextEntry` class instance which your code has ownership, [`TextEntryInRust`]`<false>` represents one which don't own.
    /// - Use [`TextEntry`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextEntry` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry.html) for more details.
    #[doc(alias = "wxTextEntry")]
    #[doc(alias = "TextEntry")]
    class TextEntry
        = TextEntryInRust<true>(wxTextEntry) impl
        TextEntryMethods
}
impl<const OWNED: bool> TextEntryInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextEntryInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TextEntryInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTextEntry_delete(self.0) }
        }
    }
}

// wxTextEntryDialog
wxwidgets! {
    /// This class represents a dialog that requests a one-line text string from the user.
    /// - [`TextEntryDialog`] represents a C++ `wxTextEntryDialog` class instance which your code has ownership, [`TextEntryDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`TextEntryDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextEntryDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html) for more details.
    #[doc(alias = "wxTextEntryDialog")]
    #[doc(alias = "TextEntryDialog")]
    class TextEntryDialog
        = TextEntryDialogInRust<true>(wxTextEntryDialog) impl
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextEntryDialogInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTextEntryDialog::wxTextEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#ac6f3a153bafcc98078217e0201e6b834).
    pub fn new_2step() -> TextEntryDialogInRust<OWNED> {
        unsafe { TextEntryDialogInRust(ffi::wxTextEntryDialog_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxTextEntryDialog::wxTextEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#ae8c8bf35b7fa14e4a0b6203d38d4dc3d).
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        value: &str,
        style: c_long,
        pos: &P,
    ) -> TextEntryDialogInRust<OWNED> {
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
            TextEntryDialogInRust(ffi::wxTextEntryDialog_new1(
                parent, message, caption, value, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TextEntryDialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextEntryDialogInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: TextEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: TextEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: TextEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TextEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TextEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextEntryDialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TextEntryDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextEntryDialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTextEntryDialog_CLASSINFO()) }
    }
}

// wxTextValidator
wxwidgets! {
    /// wxTextValidator validates text controls, providing a variety of filtering behaviours.
    /// - [`TextValidator`] represents a C++ `wxTextValidator` class instance which your code has ownership, [`TextValidatorInRust`]`<false>` represents one which don't own.
    /// - Use [`TextValidator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html) for more details.
    #[doc(alias = "wxTextValidator")]
    #[doc(alias = "TextValidator")]
    class TextValidator
        = TextValidatorInRust<true>(wxTextValidator) impl
        TextValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TextValidatorInRust<OWNED> {
    /// Copy constructor.
    ///
    /// See [C++ `wxTextValidator::wxTextValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a383375eef8f5897386b0cbc8d5eccfe5).
    pub fn new_with_textvalidator<T: TextValidatorMethods>(
        validator: &T,
    ) -> TextValidatorInRust<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            TextValidatorInRust(ffi::wxTextValidator_new(validator))
        }
    }
    /// Constructor taking a style and optional pointer to a wxString variable.
    ///
    /// See [C++ `wxTextValidator::wxTextValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#aae2135bd984152d4f5d006b012b83992).
    pub fn new_with_long(style: c_long, val_ptr: *mut c_void) -> TextValidatorInRust<OWNED> {
        unsafe { TextValidatorInRust(ffi::wxTextValidator_new1(style, val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TextValidatorInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TextValidatorInRust<OWNED>> for ValidatorInRust<OWNED> {
    fn from(o: TextValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextValidatorInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TextValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TextValidatorInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TextValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TextValidatorInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTextValidator_CLASSINFO()) }
    }
}

// wxThreadEvent
wxwidgets! {
    /// This class adds some simple functionality to wxEvent to facilitate inter-thread communication.
    /// - [`ThreadEvent`] represents a C++ `wxThreadEvent` class instance which your code has ownership, [`ThreadEventInRust`]`<false>` represents one which don't own.
    /// - Use [`ThreadEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxThreadEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_thread_event.html) for more details.
    #[doc(alias = "wxThreadEvent")]
    #[doc(alias = "ThreadEvent")]
    class ThreadEvent
        = ThreadEventInRust<true>(wxThreadEvent) impl
        ThreadEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ThreadEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxThreadEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ThreadEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ThreadEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: ThreadEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ThreadEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ThreadEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ThreadEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxThreadEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ThreadEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTimePickerCtrl
wxwidgets! {
    /// This control allows the user to enter time.
    /// - [`TimePickerCtrl`] represents a C++ `wxTimePickerCtrl` class instance which your code has ownership, [`TimePickerCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`TimePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTimePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html) for more details.
    #[doc(alias = "wxTimePickerCtrl")]
    #[doc(alias = "TimePickerCtrl")]
    class TimePickerCtrl
        = TimePickerCtrlInRust<true>(wxTimePickerCtrl) impl
        TimePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimePickerCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTimePickerCtrl::wxTimePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#a500865526b4290d8ffb1c588b146712a).
    pub fn new_2step() -> TimePickerCtrlInRust<OWNED> {
        unsafe { TimePickerCtrlInRust(ffi::wxTimePickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxTimePickerCtrl::wxTimePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#adb47aa561d4fd65849d85ac08139806e).
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
    ) -> TimePickerCtrlInRust<OWNED> {
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
            TimePickerCtrlInRust(ffi::wxTimePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TimePickerCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TimePickerCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: TimePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TimePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TimePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimePickerCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TimePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimePickerCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTimePickerCtrl_CLASSINFO()) }
    }
}

// wxTipProvider
wxwidgets! {
    /// This is the class used together with wxShowTip() function.
    /// - [`TipProvider`] represents a C++ `wxTipProvider` class instance which your code has ownership, [`TipProviderInRust`]`<false>` represents one which don't own.
    /// - Use [`TipProvider`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTipProvider` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html) for more details.
    #[doc(alias = "wxTipProvider")]
    #[doc(alias = "TipProvider")]
    class TipProvider
        = TipProviderInRust<true>(wxTipProvider) impl
        TipProviderMethods
}
impl<const OWNED: bool> TipProviderInRust<OWNED> {
    // BLOCKED: fn wxTipProvider()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TipProviderInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TipProviderInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTipProvider_delete(self.0) }
        }
    }
}

// wxTipWindow
wxwidgets! {
    /// Shows simple text in a popup tip window on creation.
    /// - [`TipWindow`] represents a C++ `wxTipWindow` class instance which your code has ownership, [`TipWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`TipWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTipWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tip_window.html) for more details.
    #[doc(alias = "wxTipWindow")]
    #[doc(alias = "TipWindow")]
    class TipWindow
        = TipWindowInRust<true>(wxTipWindow) impl
        TipWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TipWindowInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxTipWindow::wxTipWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tip_window.html#a080acf702dedf627f278050b90a9e292).
    pub fn new<W: WindowMethods, T: TipWindowMethods, R: RectMethods>(
        parent: Option<&W>,
        text: &str,
        max_length: c_int,
        window_ptr: Option<&T>,
        rect_bounds: Option<&R>,
    ) -> TipWindowInRust<OWNED> {
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
            TipWindowInRust(ffi::wxTipWindow_new(
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
impl<const OWNED: bool> Clone for TipWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TipWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TipWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TipWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TipWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TipWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TipWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TipWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTipWindow_CLASSINFO()) }
    }
}

// wxToggleButton
wxwidgets! {
    /// wxToggleButton is a button that stays pressed when clicked by the user.
    /// - [`ToggleButton`] represents a C++ `wxToggleButton` class instance which your code has ownership, [`ToggleButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`ToggleButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToggleButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html) for more details.
    #[doc(alias = "wxToggleButton")]
    #[doc(alias = "ToggleButton")]
    class ToggleButton
        = ToggleButtonInRust<true>(wxToggleButton) impl
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToggleButtonInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxToggleButton::wxToggleButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#ae80157529dec7b6db6d02641f34512ab).
    pub fn new_2step() -> ToggleButtonInRust<OWNED> {
        unsafe { ToggleButtonInRust(ffi::wxToggleButton_new()) }
    }
    /// Constructor, creating and showing a toggle button.
    ///
    /// See [C++ `wxToggleButton::wxToggleButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#af0d1a2dcae93ee68a23a5a2b77aced5b).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
        name: &str,
    ) -> ToggleButtonInRust<OWNED> {
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
            ToggleButtonInRust(ffi::wxToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ToggleButtonInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ToggleButtonInRust<OWNED>> for AnyButtonInRust<OWNED> {
    fn from(o: ToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToggleButtonInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToggleButtonInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxToggleButton_CLASSINFO()) }
    }
}

// wxToolBar
wxwidgets! {
    /// A toolbar is a bar of buttons and/or other controls usually placed below the menu bar in a wxFrame.
    /// - [`ToolBar`] represents a C++ `wxToolBar` class instance which your code has ownership, [`ToolBarInRust`]`<false>` represents one which don't own.
    /// - Use [`ToolBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html) for more details.
    #[doc(alias = "wxToolBar")]
    #[doc(alias = "ToolBar")]
    class ToolBar
        = ToolBarInRust<true>(wxToolBar) impl
        ToolBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolBarInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxToolBar::wxToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a1d03fd30dfb9c0af2e65164008eca1a3).
    pub fn new_2step() -> ToolBarInRust<OWNED> {
        unsafe { ToolBarInRust(ffi::wxToolBar_new()) }
    }
    /// Constructs a toolbar.
    ///
    /// See [C++ `wxToolBar::wxToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a1c5d3a2853b41a2ef728dad18c321e1f).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolBarInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolBarInRust(ffi::wxToolBar_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ToolBarInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ToolBarInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ToolBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ToolBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ToolBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolBarInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ToolBarInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolBarInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxToolBar_CLASSINFO()) }
    }
}

// wxToolTip
wxwidgets! {
    /// This class holds information about a tooltip associated with a window (see wxWindow::SetToolTip()).
    /// - [`ToolTip`] represents a C++ `wxToolTip` class instance which your code has ownership, [`ToolTipInRust`]`<false>` represents one which don't own.
    /// - Use [`ToolTip`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolTip` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html) for more details.
    #[doc(alias = "wxToolTip")]
    #[doc(alias = "ToolTip")]
    class ToolTip
        = ToolTipInRust<true>(wxToolTip) impl
        ToolTipMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolTipInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxToolTip::wxToolTip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a64d0a8d8439571dfae42b2558559c8fb).
    pub fn new(tip: &str) -> ToolTipInRust<OWNED> {
        unsafe {
            let tip = WxString::from(tip);
            let tip = tip.as_ptr();
            ToolTipInRust(ffi::wxToolTip_new(tip))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ToolTipInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ToolTipInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ToolTipInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolTipInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxToolTip_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ToolTipInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxToolbook
wxwidgets! {
    /// wxToolbook is a class similar to wxNotebook but which uses a wxToolBar to show the labels instead of the tabs.
    /// - [`Toolbook`] represents a C++ `wxToolbook` class instance which your code has ownership, [`ToolbookInRust`]`<false>` represents one which don't own.
    /// - Use [`Toolbook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolbook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html) for more details.
    #[doc(alias = "wxToolbook")]
    #[doc(alias = "Toolbook")]
    class Toolbook
        = ToolbookInRust<true>(wxToolbook) impl
        ToolbookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolbookInRust<OWNED> {
    /// Constructs a choicebook control.
    ///
    /// See [C++ `wxToolbook::wxToolbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#a8d988e8cdd0a495d737603418af5da34).
    pub fn new_2step() -> ToolbookInRust<OWNED> {
        unsafe { ToolbookInRust(ffi::wxToolbook_new()) }
    }
    ///
    /// See [C++ `wxToolbook::wxToolbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#a9587b9f1a9234c9ae366003a6551ac0e).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> ToolbookInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolbookInRust(ffi::wxToolbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ToolbookInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ToolbookInRust<OWNED>> for BookCtrlBaseInRust<OWNED> {
    fn from(o: ToolbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ToolbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ToolbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ToolbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ToolbookInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ToolbookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ToolbookInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxToolbook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for ToolbookInRust<OWNED> {
    /// Create the tool book control that has already been constructed with the default constructor.
    ///
    /// See [C++ `wxToolbook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#a2138b2b4a597fc51dd28f1b9e9710b1c).
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
    /// - [`TopLevelWindow`] represents a C++ `wxTopLevelWindow` class instance which your code has ownership, [`TopLevelWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`TopLevelWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTopLevelWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html) for more details.
    #[doc(alias = "wxTopLevelWindow")]
    #[doc(alias = "TopLevelWindow")]
    class TopLevelWindow
        = TopLevelWindowInRust<true>(wxTopLevelWindow) impl
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TopLevelWindowInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxTopLevelWindow::wxTopLevelWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#abb919ad585057de6fce94bb7d5497256).
    pub fn new_2step() -> TopLevelWindowInRust<OWNED> {
        unsafe { TopLevelWindowInRust(ffi::wxTopLevelWindow_new()) }
    }
    /// Constructor creating the top level window.
    ///
    /// See [C++ `wxTopLevelWindow::wxTopLevelWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#abddbaa0d42b46847b679464b485654be).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TopLevelWindowInRust<OWNED> {
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
            TopLevelWindowInRust(ffi::wxTopLevelWindow_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TopLevelWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TopLevelWindowInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: TopLevelWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TopLevelWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TopLevelWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TopLevelWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TopLevelWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TopLevelWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTopLevelWindow_CLASSINFO()) }
    }
}

// wxTreeCtrl
wxwidgets! {
    /// A tree control presents information as a hierarchy, with items that may be expanded to show further items.
    /// - [`TreeCtrl`] represents a C++ `wxTreeCtrl` class instance which your code has ownership, [`TreeCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html) for more details.
    #[doc(alias = "wxTreeCtrl")]
    #[doc(alias = "TreeCtrl")]
    class TreeCtrl
        = TreeCtrlInRust<true>(wxTreeCtrl) impl
        TreeCtrlMethods,
        // ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeCtrlInRust<OWNED> {
    /// Default Constructor.
    ///
    /// See [C++ `wxTreeCtrl::wxTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ac4a07aa9c3ef92d6663ace3c20aa1576).
    pub fn new_2step() -> TreeCtrlInRust<OWNED> {
        unsafe { TreeCtrlInRust(ffi::wxTreeCtrl_new()) }
    }
    /// Constructor, creating and showing a tree control.
    ///
    /// See [C++ `wxTreeCtrl::wxTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a298f48e824d12a43539a018e2cc06999).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> TreeCtrlInRust<OWNED> {
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
            TreeCtrlInRust(ffi::wxTreeCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TreeCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: TreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTreeCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for TreeCtrlInRust<OWNED> {
    /// Creates the tree control.
    ///
    /// See [C++ `wxTreeCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#adf40716643252f78e72fcb95dbd347ae).
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
impl<const OWNED: bool> WindowMethods for TreeCtrlInRust<OWNED> {
    /// Sets the mode flags associated with the display of the tree control.
    ///
    /// See [C++ `wxTreeCtrl::SetWindowStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a00974d17198e3883bfe4cca9d7ab52ba).
    fn set_window_style(&self, styles: c_long) {
        unsafe { ffi::wxTreeCtrl_SetWindowStyle(self.as_ptr(), styles) }
    }
}

// wxTreeEvent
wxwidgets! {
    /// A tree event holds information about events associated with wxTreeCtrl objects.
    /// - [`TreeEvent`] represents a C++ `wxTreeEvent` class instance which your code has ownership, [`TreeEventInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_event.html) for more details.
    #[doc(alias = "wxTreeEvent")]
    #[doc(alias = "TreeEvent")]
    class TreeEvent
        = TreeEventInRust<true>(wxTreeEvent) impl
        TreeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxTreeEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: TreeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: TreeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: TreeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TreeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTreeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TreeEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTreeItemData
wxwidgets! {
    /// wxTreeItemData is some (arbitrary) user class associated with some item.
    /// - [`TreeItemData`] represents a C++ `wxTreeItemData` class instance which your code has ownership, [`TreeItemDataInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeItemData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeItemData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html) for more details.
    #[doc(alias = "wxTreeItemData")]
    #[doc(alias = "TreeItemData")]
    class TreeItemData
        = TreeItemDataInRust<true>(wxTreeItemData) impl
        TreeItemDataMethods,
        ClientDataMethods
}
impl<const OWNED: bool> TreeItemDataInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreeItemData::wxTreeItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html#a8ebdd1027310183289833bcbb18dba6a).
    pub fn new() -> TreeItemDataInRust<OWNED> {
        unsafe { TreeItemDataInRust(ffi::wxTreeItemData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeItemDataInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeItemDataInRust<OWNED>> for ClientDataInRust<OWNED> {
    fn from(o: TreeItemDataInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for TreeItemDataInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeItemData_delete(self.0) }
        }
    }
}

// wxTreeItemId
wxwidgets! {
    /// An opaque reference to a tree item.
    /// - [`TreeItemId`] represents a C++ `wxTreeItemId` class instance which your code has ownership, [`TreeItemIdInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeItemId`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeItemId` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html) for more details.
    #[doc(alias = "wxTreeItemId")]
    #[doc(alias = "TreeItemId")]
    class TreeItemId
        = TreeItemIdInRust<true>(wxTreeItemId) impl
        TreeItemIdMethods
}
impl<const OWNED: bool> TreeItemIdInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreeItemId::wxTreeItemId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html#ac1c310e544edbfd2428d1ff91d0e3fb6).
    pub fn new() -> TreeItemIdInRust<OWNED> {
        unsafe { TreeItemIdInRust(ffi::wxTreeItemId_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeItemIdInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TreeItemIdInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeItemId_delete(self.0) }
        }
    }
}

// wxTreeListCtrl
wxwidgets! {
    /// A control combining wxTreeCtrl and wxListCtrl features.
    /// - [`TreeListCtrl`] represents a C++ `wxTreeListCtrl` class instance which your code has ownership, [`TreeListCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html) for more details.
    #[doc(alias = "wxTreeListCtrl")]
    #[doc(alias = "TreeListCtrl")]
    class TreeListCtrl
        = TreeListCtrlInRust<true>(wxTreeListCtrl) impl
        TreeListCtrlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreeListCtrlInRust<OWNED> {
    /// Default constructor, call Create() later.
    ///
    /// See [C++ `wxTreeListCtrl::wxTreeListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a844c05440a63f6cf04b36aab91e9449e).
    pub fn new_2step() -> TreeListCtrlInRust<OWNED> {
        unsafe { TreeListCtrlInRust(ffi::wxTreeListCtrl_new()) }
    }
    /// Full constructing, creating the object and its window.
    ///
    /// See [C++ `wxTreeListCtrl::wxTreeListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aafa1dff7beddda23c685f1c19c68aefb).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TreeListCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreeListCtrlInRust(ffi::wxTreeListCtrl_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TreeListCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreeListCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TreeListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeListCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TreeListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreeListCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TreeListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreeListCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTreeListCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for TreeListCtrlInRust<OWNED> {
    /// Create the control window.
    ///
    /// See [C++ `wxTreeListCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aca79c6e9b0f49eac98450d8104de8c95).
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
    /// - [`TreeListItem`] represents a C++ `wxTreeListItem` class instance which your code has ownership, [`TreeListItemInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeListItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html) for more details.
    #[doc(alias = "wxTreeListItem")]
    #[doc(alias = "TreeListItem")]
    class TreeListItem
        = TreeListItemInRust<true>(wxTreeListItem) impl
        TreeListItemMethods
}
impl<const OWNED: bool> TreeListItemInRust<OWNED> {
    /// Only the default constructor is publicly accessible.
    ///
    /// See [C++ `wxTreeListItem::wxTreeListItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html#ae2de6943754cad1454ab8d797bc3ada0).
    pub fn new() -> TreeListItemInRust<OWNED> {
        unsafe { TreeListItemInRust(ffi::wxTreeListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeListItemInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TreeListItemInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeListItem_delete(self.0) }
        }
    }
}

// wxTreeListItemComparator
wxwidgets! {
    /// Class defining sort order for the items in wxTreeListCtrl.
    /// - [`TreeListItemComparator`] represents a C++ `wxTreeListItemComparator` class instance which your code has ownership, [`TreeListItemComparatorInRust`]`<false>` represents one which don't own.
    /// - Use [`TreeListItemComparator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListItemComparator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item_comparator.html) for more details.
    #[doc(alias = "wxTreeListItemComparator")]
    #[doc(alias = "TreeListItemComparator")]
    class TreeListItemComparator
        = TreeListItemComparatorInRust<true>(wxTreeListItemComparator) impl
        TreeListItemComparatorMethods
}
impl<const OWNED: bool> TreeListItemComparatorInRust<OWNED> {
    // BLOCKED: fn wxTreeListItemComparator()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeListItemComparatorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for TreeListItemComparatorInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTreeListItemComparator_delete(self.0) }
        }
    }
}

// wxTreebook
wxwidgets! {
    /// This class is an extension of the wxNotebook class that allows a tree structured set of pages to be shown in a control.
    /// - [`Treebook`] represents a C++ `wxTreebook` class instance which your code has ownership, [`TreebookInRust`]`<false>` represents one which don't own.
    /// - Use [`Treebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html) for more details.
    #[doc(alias = "wxTreebook")]
    #[doc(alias = "Treebook")]
    class Treebook
        = TreebookInRust<true>(wxTreebook) impl
        TreebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TreebookInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreebook::wxTreebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html#acf521d7cb242cd792756ff6959fec797).
    pub fn new_2step() -> TreebookInRust<OWNED> {
        unsafe { TreebookInRust(ffi::wxTreebook_new()) }
    }
    /// Creates an empty wxTreebook.
    ///
    /// See [C++ `wxTreebook::wxTreebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html#a9206f1a4a98bc1defb9244284bb01ab8).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> TreebookInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreebookInRust(ffi::wxTreebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for TreebookInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<TreebookInRust<OWNED>> for BookCtrlBaseInRust<OWNED> {
    fn from(o: TreebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: TreebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: TreebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: TreebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TreebookInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: TreebookInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TreebookInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxTreebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for TreebookInRust<OWNED> {
    /// Creates a treebook control.
    ///
    /// See [C++ `wxTreebook::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html#ac6ff8c68c20d71c2c7d4b96bbe7714ce).
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
