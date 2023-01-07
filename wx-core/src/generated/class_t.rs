use super::*;

// wxTGAHandler
wxwidgets! {
    /// This is the image handler for the TGA format.
    /// - [`TGAHandler`] represents a C++ `wxTGAHandler` class instance which your code has ownership, [`TGAHandlerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TGAHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTGAHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html) for more details.
    #[doc(alias = "wxTGAHandler")]
    #[doc(alias = "TGAHandler")]
    class TGAHandler
        = TGAHandlerFromCpp<true>(wxTGAHandler) impl
        TGAHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TGAHandlerFromCpp<FROM_CPP> {
    /// Default constructor for wxTGAHandler.
    ///
    /// See [C++ `wxTGAHandler::wxTGAHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html#a5258c1de737fd77d505c06a7f822000a).
    pub fn new() -> TGAHandlerFromCpp<FROM_CPP> {
        unsafe { TGAHandlerFromCpp(ffi::wxTGAHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TGAHandlerFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TGAHandlerFromCpp<FROM_CPP>> for ImageHandlerFromCpp<FROM_CPP> {
    fn from(o: TGAHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TGAHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TGAHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TGAHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTGAHandler_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for TGAHandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTIFFHandler
wxwidgets! {
    /// This is the image handler for the TIFF format.
    /// - [`TIFFHandler`] represents a C++ `wxTIFFHandler` class instance which your code has ownership, [`TIFFHandlerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TIFFHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTIFFHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html) for more details.
    #[doc(alias = "wxTIFFHandler")]
    #[doc(alias = "TIFFHandler")]
    class TIFFHandler
        = TIFFHandlerFromCpp<true>(wxTIFFHandler) impl
        TIFFHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TIFFHandlerFromCpp<FROM_CPP> {
    /// Default constructor for wxTIFFHandler.
    ///
    /// See [C++ `wxTIFFHandler::wxTIFFHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html#a8ff101a3e846cdfaf3170d54805d05c8).
    pub fn new() -> TIFFHandlerFromCpp<FROM_CPP> {
        unsafe { TIFFHandlerFromCpp(ffi::wxTIFFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TIFFHandlerFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TIFFHandlerFromCpp<FROM_CPP>> for ImageHandlerFromCpp<FROM_CPP> {
    fn from(o: TIFFHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TIFFHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TIFFHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TIFFHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTIFFHandler_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for TIFFHandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> ImageHandlerMethods for TIFFHandlerFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxTaskBarIcon
wxwidgets! {
    /// This class represents a taskbar icon.
    /// - [`TaskBarIcon`] represents a C++ `wxTaskBarIcon` class instance which your code has ownership, [`TaskBarIconFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TaskBarIcon`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTaskBarIcon` class's documentation](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html) for more details.
    #[doc(alias = "wxTaskBarIcon")]
    #[doc(alias = "TaskBarIcon")]
    class TaskBarIcon
        = TaskBarIconFromCpp<true>(wxTaskBarIcon) impl
        TaskBarIconMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TaskBarIconFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxTaskBarIcon()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TaskBarIconFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TaskBarIconFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TaskBarIconFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TaskBarIconFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TaskBarIconFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TaskBarIconFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTaskBarIcon_CLASSINFO()) }
    }
}

// wxTaskBarIconEvent
wxwidgets! {
    /// The event class used by wxTaskBarIcon.
    /// - [`TaskBarIconEvent`] represents a C++ `wxTaskBarIconEvent` class instance which your code has ownership, [`TaskBarIconEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TaskBarIconEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTaskBarIconEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon_event.html) for more details.
    #[doc(alias = "wxTaskBarIconEvent")]
    #[doc(alias = "TaskBarIconEvent")]
    class TaskBarIconEvent
        = TaskBarIconEventFromCpp<true>(wxTaskBarIconEvent) impl
        TaskBarIconEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TaskBarIconEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxTaskBarIconEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TaskBarIconEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TaskBarIconEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: TaskBarIconEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TaskBarIconEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TaskBarIconEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TaskBarIconEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTaskBarIconEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for TaskBarIconEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTextAttr
wxwidgets! {
    /// wxTextAttr represents the character and paragraph attributes, or style, for a range of text in a wxTextCtrl or wxRichTextCtrl.
    /// - [`TextAttr`] represents a C++ `wxTextAttr` class instance which your code has ownership, [`TextAttrFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html) for more details.
    #[doc(alias = "wxTextAttr")]
    #[doc(alias = "TextAttr")]
    class TextAttr
        = TextAttrFromCpp<true>(wxTextAttr) impl
        TextAttrMethods
}
impl<const FROM_CPP: bool> TextAttrFromCpp<FROM_CPP> {
    /// Constructors.
    ///
    /// See [C++ `wxTextAttr::wxTextAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a7870ad57a4430b83a9478adeda20c6ec).
    pub fn new() -> TextAttrFromCpp<FROM_CPP> {
        unsafe { TextAttrFromCpp(ffi::wxTextAttr_new()) }
    }
    // NOT_SUPPORTED: fn wxTextAttr1()
    ///
    /// See [C++ `wxTextAttr::wxTextAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a56e913249eb032aa4184f26ebdbca454).
    pub fn new_with_textattr<T: TextAttrMethods>(attr: &T) -> TextAttrFromCpp<FROM_CPP> {
        unsafe {
            let attr = attr.as_ptr();
            TextAttrFromCpp(ffi::wxTextAttr_new2(attr))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextAttrFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for TextAttrFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTextAttr_delete(self.0) }
        }
    }
}

// wxTextCtrl
wxwidgets! {
    /// A text control allows text to be displayed and edited.
    /// - [`TextCtrl`] represents a C++ `wxTextCtrl` class instance which your code has ownership, [`TextCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html) for more details.
    #[doc(alias = "wxTextCtrl")]
    #[doc(alias = "TextCtrl")]
    class TextCtrl
        = TextCtrlFromCpp<true>(wxTextCtrl) impl
        TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TextCtrlFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxTextCtrl::wxTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a4d01be66f9a9e95501aa55d54f94d54f).
    pub fn new_2step() -> TextCtrlFromCpp<FROM_CPP> {
        unsafe { TextCtrlFromCpp(ffi::wxTextCtrl_new()) }
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
    ) -> TextCtrlFromCpp<FROM_CPP> {
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
            TextCtrlFromCpp(ffi::wxTextCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TextCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TextCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: TextCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TextCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TextCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TextCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TextCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTextCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTextCtrl
impl<const FROM_CPP: bool> TextEntryMethods for TextCtrlFromCpp<FROM_CPP> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxTextCtrl_AsTextEntry(self.as_ptr()) }
    }
}

// wxTextDataObject
wxwidgets! {
    /// wxTextDataObject is a specialization of wxDataObjectSimple for text data.
    /// - [`TextDataObject`] represents a C++ `wxTextDataObject` class instance which your code has ownership, [`TextDataObjectFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html) for more details.
    #[doc(alias = "wxTextDataObject")]
    #[doc(alias = "TextDataObject")]
    class TextDataObject
        = TextDataObjectFromCpp<true>(wxTextDataObject) impl
        TextDataObjectMethods,
        // DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> TextDataObjectFromCpp<FROM_CPP> {
    /// Constructor, may be used to initialise the text (otherwise SetText() should be used later).
    ///
    /// See [C++ `wxTextDataObject::wxTextDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html#a1d1c7457cb2b89f2df0a11129344c078).
    pub fn new(text: &str) -> TextDataObjectFromCpp<FROM_CPP> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            TextDataObjectFromCpp(ffi::wxTextDataObject_new(text))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextDataObjectFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TextDataObjectFromCpp<FROM_CPP>>
    for DataObjectSimpleFromCpp<FROM_CPP>
{
    fn from(o: TextDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextDataObjectFromCpp<FROM_CPP>> for DataObjectFromCpp<FROM_CPP> {
    fn from(o: TextDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for TextDataObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTextDataObject_delete(self.0) }
        }
    }
}
impl<const FROM_CPP: bool> DataObjectSimpleMethods for TextDataObjectFromCpp<FROM_CPP> {
    // BLOCKED: fn GetFormat()
}

// wxTextDropTarget
wxwidgets! {
    /// A predefined drop target for dealing with text data.
    /// - [`TextDropTarget`] represents a C++ `wxTextDropTarget` class instance which your code has ownership, [`TextDropTargetFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextDropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_drop_target.html) for more details.
    #[doc(alias = "wxTextDropTarget")]
    #[doc(alias = "TextDropTarget")]
    class TextDropTarget
        = TextDropTargetFromCpp<true>(wxTextDropTarget) impl
        TextDropTargetMethods,
        DropTargetMethods
}
impl<const FROM_CPP: bool> TextDropTargetFromCpp<FROM_CPP> {
    // BLOCKED: fn wxTextDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextDropTargetFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TextDropTargetFromCpp<FROM_CPP>> for DropTargetFromCpp<FROM_CPP> {
    fn from(o: TextDropTargetFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for TextDropTargetFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTextDropTarget_delete(self.0) }
        }
    }
}

// wxTextEntry
wxwidgets! {
    /// Common base class for single line text entry fields.
    /// - [`TextEntry`] represents a C++ `wxTextEntry` class instance which your code has ownership, [`TextEntryFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextEntry`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextEntry` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry.html) for more details.
    #[doc(alias = "wxTextEntry")]
    #[doc(alias = "TextEntry")]
    class TextEntry
        = TextEntryFromCpp<true>(wxTextEntry) impl
        TextEntryMethods
}
impl<const FROM_CPP: bool> TextEntryFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TextEntryFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for TextEntryFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTextEntry_delete(self.0) }
        }
    }
}

// wxTextEntryDialog
wxwidgets! {
    /// This class represents a dialog that requests a one-line text string from the user.
    /// - [`TextEntryDialog`] represents a C++ `wxTextEntryDialog` class instance which your code has ownership, [`TextEntryDialogFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextEntryDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextEntryDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html) for more details.
    #[doc(alias = "wxTextEntryDialog")]
    #[doc(alias = "TextEntryDialog")]
    class TextEntryDialog
        = TextEntryDialogFromCpp<true>(wxTextEntryDialog) impl
        TextEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TextEntryDialogFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxTextEntryDialog::wxTextEntryDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#ac6f3a153bafcc98078217e0201e6b834).
    pub fn new_2step() -> TextEntryDialogFromCpp<FROM_CPP> {
        unsafe { TextEntryDialogFromCpp(ffi::wxTextEntryDialog_new()) }
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
    ) -> TextEntryDialogFromCpp<FROM_CPP> {
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
            TextEntryDialogFromCpp(ffi::wxTextEntryDialog_new1(
                parent, message, caption, value, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TextEntryDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TextEntryDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: TextEntryDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextEntryDialogFromCpp<FROM_CPP>>
    for TopLevelWindowFromCpp<FROM_CPP>
{
    fn from(o: TextEntryDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextEntryDialogFromCpp<FROM_CPP>>
    for NonOwnedWindowFromCpp<FROM_CPP>
{
    fn from(o: TextEntryDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextEntryDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TextEntryDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextEntryDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TextEntryDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextEntryDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TextEntryDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TextEntryDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTextEntryDialog_CLASSINFO()) }
    }
}

// wxTextValidator
wxwidgets! {
    /// wxTextValidator validates text controls, providing a variety of filtering behaviours.
    /// - [`TextValidator`] represents a C++ `wxTextValidator` class instance which your code has ownership, [`TextValidatorFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TextValidator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTextValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html) for more details.
    #[doc(alias = "wxTextValidator")]
    #[doc(alias = "TextValidator")]
    class TextValidator
        = TextValidatorFromCpp<true>(wxTextValidator) impl
        TextValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TextValidatorFromCpp<FROM_CPP> {
    /// Copy constructor.
    ///
    /// See [C++ `wxTextValidator::wxTextValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a383375eef8f5897386b0cbc8d5eccfe5).
    pub fn new_with_textvalidator<T: TextValidatorMethods>(
        validator: &T,
    ) -> TextValidatorFromCpp<FROM_CPP> {
        unsafe {
            let validator = validator.as_ptr();
            TextValidatorFromCpp(ffi::wxTextValidator_new(validator))
        }
    }
    /// Constructor taking a style and optional pointer to a wxString variable.
    ///
    /// See [C++ `wxTextValidator::wxTextValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#aae2135bd984152d4f5d006b012b83992).
    pub fn new_with_long(style: c_long, val_ptr: *mut c_void) -> TextValidatorFromCpp<FROM_CPP> {
        unsafe { TextValidatorFromCpp(ffi::wxTextValidator_new1(style, val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TextValidatorFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TextValidatorFromCpp<FROM_CPP>> for ValidatorFromCpp<FROM_CPP> {
    fn from(o: TextValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextValidatorFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TextValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TextValidatorFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TextValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TextValidatorFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTextValidator_CLASSINFO()) }
    }
}

// wxThreadEvent
wxwidgets! {
    /// This class adds some simple functionality to wxEvent to facilitate inter-thread communication.
    /// - [`ThreadEvent`] represents a C++ `wxThreadEvent` class instance which your code has ownership, [`ThreadEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ThreadEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxThreadEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_thread_event.html) for more details.
    #[doc(alias = "wxThreadEvent")]
    #[doc(alias = "ThreadEvent")]
    class ThreadEvent
        = ThreadEventFromCpp<true>(wxThreadEvent) impl
        ThreadEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ThreadEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxThreadEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ThreadEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ThreadEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: ThreadEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ThreadEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ThreadEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ThreadEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxThreadEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ThreadEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTimePickerCtrl
wxwidgets! {
    /// This control allows the user to enter time.
    /// - [`TimePickerCtrl`] represents a C++ `wxTimePickerCtrl` class instance which your code has ownership, [`TimePickerCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TimePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTimePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html) for more details.
    #[doc(alias = "wxTimePickerCtrl")]
    #[doc(alias = "TimePickerCtrl")]
    class TimePickerCtrl
        = TimePickerCtrlFromCpp<true>(wxTimePickerCtrl) impl
        TimePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TimePickerCtrlFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxTimePickerCtrl::wxTimePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#a500865526b4290d8ffb1c588b146712a).
    pub fn new_2step() -> TimePickerCtrlFromCpp<FROM_CPP> {
        unsafe { TimePickerCtrlFromCpp(ffi::wxTimePickerCtrl_new()) }
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
    ) -> TimePickerCtrlFromCpp<FROM_CPP> {
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
            TimePickerCtrlFromCpp(ffi::wxTimePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TimePickerCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TimePickerCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: TimePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TimePickerCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TimePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TimePickerCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TimePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TimePickerCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TimePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TimePickerCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTimePickerCtrl_CLASSINFO()) }
    }
}

// wxTipProvider
wxwidgets! {
    /// This is the class used together with wxShowTip() function.
    /// - [`TipProvider`] represents a C++ `wxTipProvider` class instance which your code has ownership, [`TipProviderFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TipProvider`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTipProvider` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html) for more details.
    #[doc(alias = "wxTipProvider")]
    #[doc(alias = "TipProvider")]
    class TipProvider
        = TipProviderFromCpp<true>(wxTipProvider) impl
        TipProviderMethods
}
impl<const FROM_CPP: bool> TipProviderFromCpp<FROM_CPP> {
    // BLOCKED: fn wxTipProvider()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TipProviderFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for TipProviderFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTipProvider_delete(self.0) }
        }
    }
}

// wxTipWindow
wxwidgets! {
    /// Shows simple text in a popup tip window on creation.
    /// - [`TipWindow`] represents a C++ `wxTipWindow` class instance which your code has ownership, [`TipWindowFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TipWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTipWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tip_window.html) for more details.
    #[doc(alias = "wxTipWindow")]
    #[doc(alias = "TipWindow")]
    class TipWindow
        = TipWindowFromCpp<true>(wxTipWindow) impl
        TipWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TipWindowFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxTipWindow::wxTipWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tip_window.html#a080acf702dedf627f278050b90a9e292).
    pub fn new<W: WindowMethods, T: TipWindowMethods, R: RectMethods>(
        parent: Option<&W>,
        text: &str,
        max_length: c_int,
        window_ptr: Option<&T>,
        rect_bounds: Option<&R>,
    ) -> TipWindowFromCpp<FROM_CPP> {
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
            TipWindowFromCpp(ffi::wxTipWindow_new(
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
impl<const FROM_CPP: bool> Clone for TipWindowFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TipWindowFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TipWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TipWindowFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TipWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TipWindowFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TipWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TipWindowFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTipWindow_CLASSINFO()) }
    }
}

// wxToggleButton
wxwidgets! {
    /// wxToggleButton is a button that stays pressed when clicked by the user.
    /// - [`ToggleButton`] represents a C++ `wxToggleButton` class instance which your code has ownership, [`ToggleButtonFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ToggleButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToggleButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html) for more details.
    #[doc(alias = "wxToggleButton")]
    #[doc(alias = "ToggleButton")]
    class ToggleButton
        = ToggleButtonFromCpp<true>(wxToggleButton) impl
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ToggleButtonFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxToggleButton::wxToggleButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#ae80157529dec7b6db6d02641f34512ab).
    pub fn new_2step() -> ToggleButtonFromCpp<FROM_CPP> {
        unsafe { ToggleButtonFromCpp(ffi::wxToggleButton_new()) }
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
    ) -> ToggleButtonFromCpp<FROM_CPP> {
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
            ToggleButtonFromCpp(ffi::wxToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ToggleButtonFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ToggleButtonFromCpp<FROM_CPP>> for AnyButtonFromCpp<FROM_CPP> {
    fn from(o: ToggleButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToggleButtonFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ToggleButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToggleButtonFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ToggleButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToggleButtonFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ToggleButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToggleButtonFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ToggleButtonFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ToggleButtonFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxToggleButton_CLASSINFO()) }
    }
}

// wxToolBar
wxwidgets! {
    /// A toolbar is a bar of buttons and/or other controls usually placed below the menu bar in a wxFrame.
    /// - [`ToolBar`] represents a C++ `wxToolBar` class instance which your code has ownership, [`ToolBarFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ToolBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html) for more details.
    #[doc(alias = "wxToolBar")]
    #[doc(alias = "ToolBar")]
    class ToolBar
        = ToolBarFromCpp<true>(wxToolBar) impl
        ToolBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ToolBarFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxToolBar::wxToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a1d03fd30dfb9c0af2e65164008eca1a3).
    pub fn new_2step() -> ToolBarFromCpp<FROM_CPP> {
        unsafe { ToolBarFromCpp(ffi::wxToolBar_new()) }
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
    ) -> ToolBarFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolBarFromCpp(ffi::wxToolBar_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ToolBarFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ToolBarFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ToolBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolBarFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ToolBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolBarFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ToolBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolBarFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ToolBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ToolBarFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxToolBar_CLASSINFO()) }
    }
}

// wxToolTip
wxwidgets! {
    /// This class holds information about a tooltip associated with a window (see wxWindow::SetToolTip()).
    /// - [`ToolTip`] represents a C++ `wxToolTip` class instance which your code has ownership, [`ToolTipFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ToolTip`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolTip` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html) for more details.
    #[doc(alias = "wxToolTip")]
    #[doc(alias = "ToolTip")]
    class ToolTip
        = ToolTipFromCpp<true>(wxToolTip) impl
        ToolTipMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ToolTipFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxToolTip::wxToolTip()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a64d0a8d8439571dfae42b2558559c8fb).
    pub fn new(tip: &str) -> ToolTipFromCpp<FROM_CPP> {
        unsafe {
            let tip = WxString::from(tip);
            let tip = tip.as_ptr();
            ToolTipFromCpp(ffi::wxToolTip_new(tip))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ToolTipFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ToolTipFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ToolTipFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ToolTipFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxToolTip_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ToolTipFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxToolbook
wxwidgets! {
    /// wxToolbook is a class similar to wxNotebook but which uses a wxToolBar to show the labels instead of the tabs.
    /// - [`Toolbook`] represents a C++ `wxToolbook` class instance which your code has ownership, [`ToolbookFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Toolbook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxToolbook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html) for more details.
    #[doc(alias = "wxToolbook")]
    #[doc(alias = "Toolbook")]
    class Toolbook
        = ToolbookFromCpp<true>(wxToolbook) impl
        ToolbookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ToolbookFromCpp<FROM_CPP> {
    /// Constructs a choicebook control.
    ///
    /// See [C++ `wxToolbook::wxToolbook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#a8d988e8cdd0a495d737603418af5da34).
    pub fn new_2step() -> ToolbookFromCpp<FROM_CPP> {
        unsafe { ToolbookFromCpp(ffi::wxToolbook_new()) }
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
    ) -> ToolbookFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ToolbookFromCpp(ffi::wxToolbook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for ToolbookFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ToolbookFromCpp<FROM_CPP>> for BookCtrlBaseFromCpp<FROM_CPP> {
    fn from(o: ToolbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolbookFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: ToolbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolbookFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: ToolbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolbookFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: ToolbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<ToolbookFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ToolbookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ToolbookFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxToolbook_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for ToolbookFromCpp<FROM_CPP> {
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
    /// - [`TopLevelWindow`] represents a C++ `wxTopLevelWindow` class instance which your code has ownership, [`TopLevelWindowFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TopLevelWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTopLevelWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html) for more details.
    #[doc(alias = "wxTopLevelWindow")]
    #[doc(alias = "TopLevelWindow")]
    class TopLevelWindow
        = TopLevelWindowFromCpp<true>(wxTopLevelWindow) impl
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TopLevelWindowFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxTopLevelWindow::wxTopLevelWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#abb919ad585057de6fce94bb7d5497256).
    pub fn new_2step() -> TopLevelWindowFromCpp<FROM_CPP> {
        unsafe { TopLevelWindowFromCpp(ffi::wxTopLevelWindow_new()) }
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
    ) -> TopLevelWindowFromCpp<FROM_CPP> {
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
            TopLevelWindowFromCpp(ffi::wxTopLevelWindow_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TopLevelWindowFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TopLevelWindowFromCpp<FROM_CPP>>
    for NonOwnedWindowFromCpp<FROM_CPP>
{
    fn from(o: TopLevelWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TopLevelWindowFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TopLevelWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TopLevelWindowFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TopLevelWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TopLevelWindowFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TopLevelWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TopLevelWindowFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTopLevelWindow_CLASSINFO()) }
    }
}

// wxTreeCtrl
wxwidgets! {
    /// A tree control presents information as a hierarchy, with items that may be expanded to show further items.
    /// - [`TreeCtrl`] represents a C++ `wxTreeCtrl` class instance which your code has ownership, [`TreeCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html) for more details.
    #[doc(alias = "wxTreeCtrl")]
    #[doc(alias = "TreeCtrl")]
    class TreeCtrl
        = TreeCtrlFromCpp<true>(wxTreeCtrl) impl
        TreeCtrlMethods,
        // ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TreeCtrlFromCpp<FROM_CPP> {
    /// Default Constructor.
    ///
    /// See [C++ `wxTreeCtrl::wxTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ac4a07aa9c3ef92d6663ace3c20aa1576).
    pub fn new_2step() -> TreeCtrlFromCpp<FROM_CPP> {
        unsafe { TreeCtrlFromCpp(ffi::wxTreeCtrl_new()) }
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
    ) -> TreeCtrlFromCpp<FROM_CPP> {
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
            TreeCtrlFromCpp(ffi::wxTreeCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TreeCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TreeCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: TreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TreeCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTreeCtrl_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> ControlMethods for TreeCtrlFromCpp<FROM_CPP> {
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
impl<const FROM_CPP: bool> WindowMethods for TreeCtrlFromCpp<FROM_CPP> {
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
    /// - [`TreeEvent`] represents a C++ `wxTreeEvent` class instance which your code has ownership, [`TreeEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_event.html) for more details.
    #[doc(alias = "wxTreeEvent")]
    #[doc(alias = "TreeEvent")]
    class TreeEvent
        = TreeEventFromCpp<true>(wxTreeEvent) impl
        TreeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TreeEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxTreeEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TreeEventFromCpp<FROM_CPP>> for NotifyEventFromCpp<FROM_CPP> {
    fn from(o: TreeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: TreeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: TreeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TreeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TreeEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTreeEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for TreeEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTreeItemData
wxwidgets! {
    /// wxTreeItemData is some (arbitrary) user class associated with some item.
    /// - [`TreeItemData`] represents a C++ `wxTreeItemData` class instance which your code has ownership, [`TreeItemDataFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeItemData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeItemData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html) for more details.
    #[doc(alias = "wxTreeItemData")]
    #[doc(alias = "TreeItemData")]
    class TreeItemData
        = TreeItemDataFromCpp<true>(wxTreeItemData) impl
        TreeItemDataMethods,
        ClientDataMethods
}
impl<const FROM_CPP: bool> TreeItemDataFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreeItemData::wxTreeItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html#a8ebdd1027310183289833bcbb18dba6a).
    pub fn new() -> TreeItemDataFromCpp<FROM_CPP> {
        unsafe { TreeItemDataFromCpp(ffi::wxTreeItemData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeItemDataFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TreeItemDataFromCpp<FROM_CPP>> for ClientDataFromCpp<FROM_CPP> {
    fn from(o: TreeItemDataFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for TreeItemDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTreeItemData_delete(self.0) }
        }
    }
}

// wxTreeItemId
wxwidgets! {
    /// An opaque reference to a tree item.
    /// - [`TreeItemId`] represents a C++ `wxTreeItemId` class instance which your code has ownership, [`TreeItemIdFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeItemId`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeItemId` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html) for more details.
    #[doc(alias = "wxTreeItemId")]
    #[doc(alias = "TreeItemId")]
    class TreeItemId
        = TreeItemIdFromCpp<true>(wxTreeItemId) impl
        TreeItemIdMethods
}
impl<const FROM_CPP: bool> TreeItemIdFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreeItemId::wxTreeItemId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html#ac1c310e544edbfd2428d1ff91d0e3fb6).
    pub fn new() -> TreeItemIdFromCpp<FROM_CPP> {
        unsafe { TreeItemIdFromCpp(ffi::wxTreeItemId_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeItemIdFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for TreeItemIdFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTreeItemId_delete(self.0) }
        }
    }
}

// wxTreeListCtrl
wxwidgets! {
    /// A control combining wxTreeCtrl and wxListCtrl features.
    /// - [`TreeListCtrl`] represents a C++ `wxTreeListCtrl` class instance which your code has ownership, [`TreeListCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html) for more details.
    #[doc(alias = "wxTreeListCtrl")]
    #[doc(alias = "TreeListCtrl")]
    class TreeListCtrl
        = TreeListCtrlFromCpp<true>(wxTreeListCtrl) impl
        TreeListCtrlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TreeListCtrlFromCpp<FROM_CPP> {
    /// Default constructor, call Create() later.
    ///
    /// See [C++ `wxTreeListCtrl::wxTreeListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a844c05440a63f6cf04b36aab91e9449e).
    pub fn new_2step() -> TreeListCtrlFromCpp<FROM_CPP> {
        unsafe { TreeListCtrlFromCpp(ffi::wxTreeListCtrl_new()) }
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
    ) -> TreeListCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreeListCtrlFromCpp(ffi::wxTreeListCtrl_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TreeListCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TreeListCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TreeListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeListCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TreeListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreeListCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TreeListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TreeListCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTreeListCtrl_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for TreeListCtrlFromCpp<FROM_CPP> {
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
    /// - [`TreeListItem`] represents a C++ `wxTreeListItem` class instance which your code has ownership, [`TreeListItemFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeListItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html) for more details.
    #[doc(alias = "wxTreeListItem")]
    #[doc(alias = "TreeListItem")]
    class TreeListItem
        = TreeListItemFromCpp<true>(wxTreeListItem) impl
        TreeListItemMethods
}
impl<const FROM_CPP: bool> TreeListItemFromCpp<FROM_CPP> {
    /// Only the default constructor is publicly accessible.
    ///
    /// See [C++ `wxTreeListItem::wxTreeListItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html#ae2de6943754cad1454ab8d797bc3ada0).
    pub fn new() -> TreeListItemFromCpp<FROM_CPP> {
        unsafe { TreeListItemFromCpp(ffi::wxTreeListItem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeListItemFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for TreeListItemFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTreeListItem_delete(self.0) }
        }
    }
}

// wxTreeListItemComparator
wxwidgets! {
    /// Class defining sort order for the items in wxTreeListCtrl.
    /// - [`TreeListItemComparator`] represents a C++ `wxTreeListItemComparator` class instance which your code has ownership, [`TreeListItemComparatorFromCpp`]`<false>` represents one which don't own.
    /// - Use [`TreeListItemComparator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreeListItemComparator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_tree_list_item_comparator.html) for more details.
    #[doc(alias = "wxTreeListItemComparator")]
    #[doc(alias = "TreeListItemComparator")]
    class TreeListItemComparator
        = TreeListItemComparatorFromCpp<true>(wxTreeListItemComparator) impl
        TreeListItemComparatorMethods
}
impl<const FROM_CPP: bool> TreeListItemComparatorFromCpp<FROM_CPP> {
    // BLOCKED: fn wxTreeListItemComparator()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for TreeListItemComparatorFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for TreeListItemComparatorFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxTreeListItemComparator_delete(self.0) }
        }
    }
}

// wxTreebook
wxwidgets! {
    /// This class is an extension of the wxNotebook class that allows a tree structured set of pages to be shown in a control.
    /// - [`Treebook`] represents a C++ `wxTreebook` class instance which your code has ownership, [`TreebookFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Treebook`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxTreebook` class's documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html) for more details.
    #[doc(alias = "wxTreebook")]
    #[doc(alias = "Treebook")]
    class Treebook
        = TreebookFromCpp<true>(wxTreebook) impl
        TreebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> TreebookFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxTreebook::wxTreebook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_treebook.html#acf521d7cb242cd792756ff6959fec797).
    pub fn new_2step() -> TreebookFromCpp<FROM_CPP> {
        unsafe { TreebookFromCpp(ffi::wxTreebook_new()) }
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
    ) -> TreebookFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            TreebookFromCpp(ffi::wxTreebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for TreebookFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<TreebookFromCpp<FROM_CPP>> for BookCtrlBaseFromCpp<FROM_CPP> {
    fn from(o: TreebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreebookFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: TreebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreebookFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: TreebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreebookFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: TreebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<TreebookFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: TreebookFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for TreebookFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxTreebook_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for TreebookFromCpp<FROM_CPP> {
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
