use super::*;

// wxTGAHandler
wxwidgets! {
    /// This is the image handler for the TGA format.
    /// - [`TGAHandler`] represents a C++ `wxTGAHandler` class instance which your code has ownership, [`TGAHandlerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TGAHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTGAHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html) for more details.
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
    ///
    /// See [C++ `wxTGAHandler::wxTGAHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html#a5258c1de737fd77d505c06a7f822000a).
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
    /// - [`TIFFHandler`] represents a C++ `wxTIFFHandler` class instance which your code has ownership, [`TIFFHandlerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TIFFHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTIFFHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html) for more details.
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
    ///
    /// See [C++ `wxTIFFHandler::wxTIFFHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html#a8ff101a3e846cdfaf3170d54805d05c8).
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
    /// - [`TaskBarIcon`] represents a C++ `wxTaskBarIcon` class instance which your code has ownership, [`TaskBarIconIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TaskBarIcon`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTaskBarIcon` class's documentation](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html) for more details.
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
    /// - [`TaskBarIconEvent`] represents a C++ `wxTaskBarIconEvent` class instance which your code has ownership, [`TaskBarIconEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TaskBarIconEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTaskBarIconEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon_event.html) for more details.
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
    /// - [`TextAttr`] represents a C++ `wxTextAttr` class instance which your code has ownership, [`TextAttrIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html) for more details.
    #[doc(alias = "wxTextAttr")]
    #[doc(alias = "TextAttr")]
    class TextAttr
        = TextAttrIsOwned<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const OWNED: bool> TextAttrIsOwned<OWNED> {
    /// Constructors.
    ///
    /// See [C++ `wxTextAttr::wxTextAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a7870ad57a4430b83a9478adeda20c6ec).
    pub fn new() -> TextAttrIsOwned<OWNED> {
        unsafe { TextAttrIsOwned(ffi::wxTextAttr_new()) }
    }
    // NOT_SUPPORTED: fn wxTextAttr1()
    ///
    /// See [C++ `wxTextAttr::wxTextAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a56e913249eb032aa4184f26ebdbca454).
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
    /// - [`TextCtrl`] represents a C++ `wxTextCtrl` class instance which your code has ownership, [`TextCtrlIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html) for more details.
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
    ///
    /// See [C++ `wxTextCtrl::wxTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a4d01be66f9a9e95501aa55d54f94d54f).
    pub fn new_2step() -> TextCtrlIsOwned<OWNED> {
        unsafe { TextCtrlIsOwned(ffi::wxTextCtrl_new()) }
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
    /// - [`TextDataObject`] represents a C++ `wxTextDataObject` class instance which your code has ownership, [`TextDataObjectIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html) for more details.
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
    ///
    /// See [C++ `wxTextDataObject::wxTextDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html#a1d1c7457cb2b89f2df0a11129344c078).
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
    /// - [`TextDropTarget`] represents a C++ `wxTextDropTarget` class instance which your code has ownership, [`TextDropTargetIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextDropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_drop_target.html) for more details.
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
    /// - [`TextEntry`] represents a C++ `wxTextEntry` class instance which your code has ownership, [`TextEntryIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextEntry`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextEntry` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry.html) for more details.
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
    /// - [`TextEntryDialog`] represents a C++ `wxTextEntryDialog` class instance which your code has ownership, [`TextEntryDialogIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextEntryDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextEntryDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html) for more details.
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
    ///
    /// See [C++ `wxTextEntryDialog::wxTextEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#ac6f3a153bafcc98078217e0201e6b834).
    pub fn new_2step() -> TextEntryDialogIsOwned<OWNED> {
        unsafe { TextEntryDialogIsOwned(ffi::wxTextEntryDialog_new()) }
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
    /// - [`TextValidator`] represents a C++ `wxTextValidator` class instance which your code has ownership, [`TextValidatorIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TextValidator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html) for more details.
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
    ///
    /// See [C++ `wxTextValidator::wxTextValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a383375eef8f5897386b0cbc8d5eccfe5).
    pub fn new_with_textvalidator<T: TextValidatorMethods>(
        validator: &T,
    ) -> TextValidatorIsOwned<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            TextValidatorIsOwned(ffi::wxTextValidator_new(validator))
        }
    }
    /// Constructor taking a style and optional pointer to a wxString variable.
    ///
    /// See [C++ `wxTextValidator::wxTextValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#aae2135bd984152d4f5d006b012b83992).
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
    /// - [`ThreadEvent`] represents a C++ `wxThreadEvent` class instance which your code has ownership, [`ThreadEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ThreadEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxThreadEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_thread_event.html) for more details.
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
    /// - [`TimePickerCtrl`] represents a C++ `wxTimePickerCtrl` class instance which your code has ownership, [`TimePickerCtrlIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TimePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTimePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html) for more details.
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
    ///
    /// See [C++ `wxTimePickerCtrl::wxTimePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#a500865526b4290d8ffb1c588b146712a).
    pub fn new_2step() -> TimePickerCtrlIsOwned<OWNED> {
        unsafe { TimePickerCtrlIsOwned(ffi::wxTimePickerCtrl_new()) }
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
    /// - [`TipProvider`] represents a C++ `wxTipProvider` class instance which your code has ownership, [`TipProviderIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TipProvider`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTipProvider` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html) for more details.
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
    /// - [`TipWindow`] represents a C++ `wxTipWindow` class instance which your code has ownership, [`TipWindowIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TipWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTipWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tip_window.html) for more details.
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
    ///
    /// See [C++ `wxTipWindow::wxTipWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tip_window.html#a080acf702dedf627f278050b90a9e292).
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
    /// - [`ToggleButton`] represents a C++ `wxToggleButton` class instance which your code has ownership, [`ToggleButtonIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ToggleButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToggleButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html) for more details.
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
    ///
    /// See [C++ `wxToggleButton::wxToggleButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#ae80157529dec7b6db6d02641f34512ab).
    pub fn new_2step() -> ToggleButtonIsOwned<OWNED> {
        unsafe { ToggleButtonIsOwned(ffi::wxToggleButton_new()) }
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
    /// - [`ToolBar`] represents a C++ `wxToolBar` class instance which your code has ownership, [`ToolBarIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ToolBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html) for more details.
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
    ///
    /// See [C++ `wxToolBar::wxToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a1d03fd30dfb9c0af2e65164008eca1a3).
    pub fn new_2step() -> ToolBarIsOwned<OWNED> {
        unsafe { ToolBarIsOwned(ffi::wxToolBar_new()) }
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
    /// - [`ToolTip`] represents a C++ `wxToolTip` class instance which your code has ownership, [`ToolTipIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ToolTip`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolTip` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html) for more details.
    #[doc(alias = "wxToolTip")]
    #[doc(alias = "ToolTip")]
    class ToolTip
        = ToolTipIsOwned<true>(wxToolTip) impl
        ToolTipMethods,
        ObjectMethods
}
impl<const OWNED: bool> ToolTipIsOwned<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxToolTip::wxToolTip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a64d0a8d8439571dfae42b2558559c8fb).
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
    /// - [`Toolbook`] represents a C++ `wxToolbook` class instance which your code has ownership, [`ToolbookIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Toolbook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolbook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html) for more details.
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
    ///
    /// See [C++ `wxToolbook::wxToolbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#a8d988e8cdd0a495d737603418af5da34).
    pub fn new_2step() -> ToolbookIsOwned<OWNED> {
        unsafe { ToolbookIsOwned(ffi::wxToolbook_new()) }
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
    /// - [`TopLevelWindow`] represents a C++ `wxTopLevelWindow` class instance which your code has ownership, [`TopLevelWindowIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TopLevelWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTopLevelWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html) for more details.
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
    ///
    /// See [C++ `wxTopLevelWindow::wxTopLevelWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#abb919ad585057de6fce94bb7d5497256).
    pub fn new_2step() -> TopLevelWindowIsOwned<OWNED> {
        unsafe { TopLevelWindowIsOwned(ffi::wxTopLevelWindow_new()) }
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
    /// - [`TreeCtrl`] represents a C++ `wxTreeCtrl` class instance which your code has ownership, [`TreeCtrlIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html) for more details.
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
    ///
    /// See [C++ `wxTreeCtrl::wxTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ac4a07aa9c3ef92d6663ace3c20aa1576).
    pub fn new_2step() -> TreeCtrlIsOwned<OWNED> {
        unsafe { TreeCtrlIsOwned(ffi::wxTreeCtrl_new()) }
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
impl<const OWNED: bool> WindowMethods for TreeCtrlIsOwned<OWNED> {
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
    /// - [`TreeEvent`] represents a C++ `wxTreeEvent` class instance which your code has ownership, [`TreeEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_event.html) for more details.
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
    /// - [`TreeItemData`] represents a C++ `wxTreeItemData` class instance which your code has ownership, [`TreeItemDataIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeItemData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeItemData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html) for more details.
    #[doc(alias = "wxTreeItemData")]
    #[doc(alias = "TreeItemData")]
    class TreeItemData
        = TreeItemDataIsOwned<true>(wxTreeItemData) impl
        TreeItemDataMethods,
        ClientDataMethods
}
impl<const OWNED: bool> TreeItemDataIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreeItemData::wxTreeItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html#a8ebdd1027310183289833bcbb18dba6a).
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
    /// - [`TreeItemId`] represents a C++ `wxTreeItemId` class instance which your code has ownership, [`TreeItemIdIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeItemId`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeItemId` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html) for more details.
    #[doc(alias = "wxTreeItemId")]
    #[doc(alias = "TreeItemId")]
    class TreeItemId
        = TreeItemIdIsOwned<true>(wxTreeItemId) impl
        TreeItemIdMethods
}
impl<const OWNED: bool> TreeItemIdIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreeItemId::wxTreeItemId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html#ac1c310e544edbfd2428d1ff91d0e3fb6).
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
    /// - [`TreeListCtrl`] represents a C++ `wxTreeListCtrl` class instance which your code has ownership, [`TreeListCtrlIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html) for more details.
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
    ///
    /// See [C++ `wxTreeListCtrl::wxTreeListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a844c05440a63f6cf04b36aab91e9449e).
    pub fn new_2step() -> TreeListCtrlIsOwned<OWNED> {
        unsafe { TreeListCtrlIsOwned(ffi::wxTreeListCtrl_new()) }
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
    /// - [`TreeListItem`] represents a C++ `wxTreeListItem` class instance which your code has ownership, [`TreeListItemIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeListItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html) for more details.
    #[doc(alias = "wxTreeListItem")]
    #[doc(alias = "TreeListItem")]
    class TreeListItem
        = TreeListItemIsOwned<true>(wxTreeListItem) impl
        TreeListItemMethods
}
impl<const OWNED: bool> TreeListItemIsOwned<OWNED> {
    /// Only the default constructor is publicly accessible.
    ///
    /// See [C++ `wxTreeListItem::wxTreeListItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html#ae2de6943754cad1454ab8d797bc3ada0).
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
    /// - [`TreeListItemComparator`] represents a C++ `wxTreeListItemComparator` class instance which your code has ownership, [`TreeListItemComparatorIsOwned`]`<false>` represents one which don't own.
    /// - Use [`TreeListItemComparator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListItemComparator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item_comparator.html) for more details.
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
    /// - [`Treebook`] represents a C++ `wxTreebook` class instance which your code has ownership, [`TreebookIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Treebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html) for more details.
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
    ///
    /// See [C++ `wxTreebook::wxTreebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html#acf521d7cb242cd792756ff6959fec797).
    pub fn new_2step() -> TreebookIsOwned<OWNED> {
        unsafe { TreebookIsOwned(ffi::wxTreebook_new()) }
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
