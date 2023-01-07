use super::*;

// wxFileCtrl
wxwidgets! {
    /// This control allows the user to select a file.
    /// - [`FileCtrl`] represents a C++ `wxFileCtrl` class instance which your code has ownership, [`FileCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`FileCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html) for more details.
    #[doc(alias = "wxFileCtrl")]
    #[doc(alias = "FileCtrl")]
    class FileCtrl
        = FileCtrlInRust<true>(wxFileCtrl) impl
        FileCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FileCtrlInRust<IN_RUST> {
    ///
    /// See [C++ `wxFileCtrl::wxFileCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#ad0f3935004f16b380571ead2d1b4c04c).
    pub fn new_2step() -> FileCtrlInRust<IN_RUST> {
        unsafe { FileCtrlInRust(ffi::wxFileCtrl_new()) }
    }
    /// Constructs the window.
    ///
    /// See [C++ `wxFileCtrl::wxFileCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a3a4611ef0c3e9087e3768869c53ab5cc).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        default_directory: &str,
        default_filename: &str,
        wild_card: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileCtrlInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let default_directory = WxString::from(default_directory);
            let default_directory = default_directory.as_ptr();
            let default_filename = WxString::from(default_filename);
            let default_filename = default_filename.as_ptr();
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileCtrlInRust(ffi::wxFileCtrl_new1(
                parent,
                id,
                default_directory,
                default_filename,
                wild_card,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FileCtrlInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileCtrlInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: FileCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileCtrlInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FileCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileCtrlInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FileCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileCtrlInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FileCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FileCtrlInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFileCtrl_CLASSINFO()) }
    }
}

// wxFileCtrlEvent
wxwidgets! {
    /// A file control event holds information about events associated with wxFileCtrl objects.
    /// - [`FileCtrlEvent`] represents a C++ `wxFileCtrlEvent` class instance which your code has ownership, [`FileCtrlEventInRust`]`<false>` represents one which don't own.
    /// - Use [`FileCtrlEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileCtrlEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html) for more details.
    #[doc(alias = "wxFileCtrlEvent")]
    #[doc(alias = "FileCtrlEvent")]
    class FileCtrlEvent
        = FileCtrlEventInRust<true>(wxFileCtrlEvent) impl
        FileCtrlEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FileCtrlEventInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxFileCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileCtrlEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileCtrlEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: FileCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileCtrlEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: FileCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileCtrlEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FileCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FileCtrlEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFileCtrlEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FileCtrlEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDataObject
wxwidgets! {
    /// wxFileDataObject is a specialization of wxDataObject for file names.
    /// - [`FileDataObject`] represents a C++ `wxFileDataObject` class instance which your code has ownership, [`FileDataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`FileDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html) for more details.
    #[doc(alias = "wxFileDataObject")]
    #[doc(alias = "FileDataObject")]
    class FileDataObject
        = FileDataObjectInRust<true>(wxFileDataObject) impl
        FileDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const IN_RUST: bool> FileDataObjectInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxFileDataObject::wxFileDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html#a7c7cb54a6cf9114de7dec67755ac749e).
    pub fn new() -> FileDataObjectInRust<IN_RUST> {
        unsafe { FileDataObjectInRust(ffi::wxFileDataObject_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileDataObjectInRust<IN_RUST>> for DataObjectSimpleInRust<IN_RUST> {
    fn from(o: FileDataObjectInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDataObjectInRust<IN_RUST>> for DataObjectInRust<IN_RUST> {
    fn from(o: FileDataObjectInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for FileDataObjectInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxFileDataObject_delete(self.0) }
        }
    }
}

// wxFileDialog
wxwidgets! {
    /// This class represents the file chooser dialog.
    /// - [`FileDialog`] represents a C++ `wxFileDialog` class instance which your code has ownership, [`FileDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`FileDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html) for more details.
    #[doc(alias = "wxFileDialog")]
    #[doc(alias = "FileDialog")]
    class FileDialog
        = FileDialogInRust<true>(wxFileDialog) impl
        FileDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FileDialogInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxFileDialog::wxFileDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#af3ff2981229bd2f892df0fa96fb9265d).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        message: &str,
        default_dir: &str,
        default_file: &str,
        wildcard: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> FileDialogInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let default_dir = WxString::from(default_dir);
            let default_dir = default_dir.as_ptr();
            let default_file = WxString::from(default_file);
            let default_file = default_file.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FileDialogInRust(ffi::wxFileDialog_new(
                parent,
                message,
                default_dir,
                default_file,
                wildcard,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FileDialogInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileDialogInRust<IN_RUST>> for DialogInRust<IN_RUST> {
    fn from(o: FileDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDialogInRust<IN_RUST>> for TopLevelWindowInRust<IN_RUST> {
    fn from(o: FileDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDialogInRust<IN_RUST>> for NonOwnedWindowInRust<IN_RUST> {
    fn from(o: FileDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDialogInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FileDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDialogInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FileDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDialogInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FileDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FileDialogInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFileDialog_CLASSINFO()) }
    }
}

// wxFileDirPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxFilePickerCtrl and by wxDirPickerCtrl.
    /// - [`FileDirPickerEvent`] represents a C++ `wxFileDirPickerEvent` class instance which your code has ownership, [`FileDirPickerEventInRust`]`<false>` represents one which don't own.
    /// - Use [`FileDirPickerEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDirPickerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html) for more details.
    #[doc(alias = "wxFileDirPickerEvent")]
    #[doc(alias = "FileDirPickerEvent")]
    class FileDirPickerEvent
        = FileDirPickerEventInRust<true>(wxFileDirPickerEvent) impl
        FileDirPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FileDirPickerEventInRust<IN_RUST> {
    ///
    /// See [C++ `wxFileDirPickerEvent::wxFileDirPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html#a311f6e70d669ca9ef6f4425c7778f215).
    pub fn new() -> FileDirPickerEventInRust<IN_RUST> {
        unsafe { FileDirPickerEventInRust(ffi::wxFileDirPickerEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxFileDirPickerEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDirPickerEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileDirPickerEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: FileDirPickerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDirPickerEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: FileDirPickerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FileDirPickerEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FileDirPickerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FileDirPickerEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFileDirPickerEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FileDirPickerEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDropTarget
wxwidgets! {
    /// This is a drop target which accepts files (dragged from File Manager or Explorer).
    /// - [`FileDropTarget`] represents a C++ `wxFileDropTarget` class instance which your code has ownership, [`FileDropTargetInRust`]`<false>` represents one which don't own.
    /// - Use [`FileDropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_drop_target.html) for more details.
    #[doc(alias = "wxFileDropTarget")]
    #[doc(alias = "FileDropTarget")]
    class FileDropTarget
        = FileDropTargetInRust<true>(wxFileDropTarget) impl
        FileDropTargetMethods,
        DropTargetMethods
}
impl<const IN_RUST: bool> FileDropTargetInRust<IN_RUST> {
    // BLOCKED: fn wxFileDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDropTargetInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileDropTargetInRust<IN_RUST>> for DropTargetInRust<IN_RUST> {
    fn from(o: FileDropTargetInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for FileDropTargetInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxFileDropTarget_delete(self.0) }
        }
    }
}

// wxFileHistory
wxwidgets! {
    /// The wxFileHistory encapsulates a user interface convenience, the list of most recently visited files as shown on a menu (usually the File menu).
    /// - [`FileHistory`] represents a C++ `wxFileHistory` class instance which your code has ownership, [`FileHistoryInRust`]`<false>` represents one which don't own.
    /// - Use [`FileHistory`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileHistory` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html) for more details.
    #[doc(alias = "wxFileHistory")]
    #[doc(alias = "FileHistory")]
    class FileHistory
        = FileHistoryInRust<true>(wxFileHistory) impl
        FileHistoryMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FileHistoryInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxFileHistory::wxFileHistory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a30e3a3a1f92fc253cc0fc69eb6f27fd8).
    pub fn new(max_files: usize, id_base: c_int) -> FileHistoryInRust<IN_RUST> {
        unsafe { FileHistoryInRust(ffi::wxFileHistory_new(max_files, id_base)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileHistoryInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FileHistoryInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FileHistoryInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FileHistoryInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFileHistory_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FileHistoryInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFilePickerCtrl
wxwidgets! {
    /// This control allows the user to select a file.
    /// - [`FilePickerCtrl`] represents a C++ `wxFilePickerCtrl` class instance which your code has ownership, [`FilePickerCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`FilePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFilePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html) for more details.
    #[doc(alias = "wxFilePickerCtrl")]
    #[doc(alias = "FilePickerCtrl")]
    class FilePickerCtrl
        = FilePickerCtrlInRust<true>(wxFilePickerCtrl) impl
        FilePickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FilePickerCtrlInRust<IN_RUST> {
    ///
    /// See [C++ `wxFilePickerCtrl::wxFilePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#afe16cc740765fb8ec68c9df250a07812).
    pub fn new_2step() -> FilePickerCtrlInRust<IN_RUST> {
        unsafe { FilePickerCtrlInRust(ffi::wxFilePickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxFilePickerCtrl::wxFilePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#a2f9d8631a622d88cf820719a33879a4f).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        wildcard: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FilePickerCtrlInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let wildcard = WxString::from(wildcard);
            let wildcard = wildcard.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FilePickerCtrlInRust(ffi::wxFilePickerCtrl_new1(
                parent, id, path, message, wildcard, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FilePickerCtrlInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FilePickerCtrlInRust<IN_RUST>> for PickerBaseInRust<IN_RUST> {
    fn from(o: FilePickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FilePickerCtrlInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: FilePickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FilePickerCtrlInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FilePickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FilePickerCtrlInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FilePickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FilePickerCtrlInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FilePickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FilePickerCtrlInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFilePickerCtrl_CLASSINFO()) }
    }
}

// wxFindDialogEvent
wxwidgets! {
    /// wxFindReplaceDialog events.
    /// - [`FindDialogEvent`] represents a C++ `wxFindDialogEvent` class instance which your code has ownership, [`FindDialogEventInRust`]`<false>` represents one which don't own.
    /// - Use [`FindDialogEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFindDialogEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html) for more details.
    #[doc(alias = "wxFindDialogEvent")]
    #[doc(alias = "FindDialogEvent")]
    class FindDialogEvent
        = FindDialogEventInRust<true>(wxFindDialogEvent) impl
        FindDialogEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FindDialogEventInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxFindDialogEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FindDialogEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FindDialogEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: FindDialogEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindDialogEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: FindDialogEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindDialogEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FindDialogEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FindDialogEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFindDialogEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FindDialogEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceData
wxwidgets! {
    /// wxFindReplaceData holds the data for wxFindReplaceDialog.
    /// - [`FindReplaceData`] represents a C++ `wxFindReplaceData` class instance which your code has ownership, [`FindReplaceDataInRust`]`<false>` represents one which don't own.
    /// - Use [`FindReplaceData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFindReplaceData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html) for more details.
    #[doc(alias = "wxFindReplaceData")]
    #[doc(alias = "FindReplaceData")]
    class FindReplaceData
        = FindReplaceDataInRust<true>(wxFindReplaceData) impl
        FindReplaceDataMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FindReplaceDataInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxFindReplaceData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FindReplaceDataInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FindReplaceDataInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FindReplaceDataInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FindReplaceDataInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFindReplaceData_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FindReplaceDataInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceDialog
wxwidgets! {
    /// wxFindReplaceDialog is a standard modeless dialog which is used to allow the user to search for some text (and possibly replace it with something else).
    /// - [`FindReplaceDialog`] represents a C++ `wxFindReplaceDialog` class instance which your code has ownership, [`FindReplaceDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`FindReplaceDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFindReplaceDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html) for more details.
    #[doc(alias = "wxFindReplaceDialog")]
    #[doc(alias = "FindReplaceDialog")]
    class FindReplaceDialog
        = FindReplaceDialogInRust<true>(wxFindReplaceDialog) impl
        FindReplaceDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FindReplaceDialogInRust<IN_RUST> {
    ///
    /// See [C++ `wxFindReplaceDialog::wxFindReplaceDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#a10601867d5793096323ce0979f7993cd).
    pub fn new_2step() -> FindReplaceDialogInRust<IN_RUST> {
        unsafe { FindReplaceDialogInRust(ffi::wxFindReplaceDialog_new()) }
    }
    /// After using default constructor Create() must be called.
    ///
    /// See [C++ `wxFindReplaceDialog::wxFindReplaceDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#a27c6a7da82dd935ec7a9812ea6bf73c3).
    pub fn new<W: WindowMethods, F: FindReplaceDataMethods>(
        parent: Option<&W>,
        data: Option<&F>,
        title: &str,
        style: c_int,
    ) -> FindReplaceDialogInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            FindReplaceDialogInRust(ffi::wxFindReplaceDialog_new1(parent, data, title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FindReplaceDialogInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FindReplaceDialogInRust<IN_RUST>> for DialogInRust<IN_RUST> {
    fn from(o: FindReplaceDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindReplaceDialogInRust<IN_RUST>> for TopLevelWindowInRust<IN_RUST> {
    fn from(o: FindReplaceDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindReplaceDialogInRust<IN_RUST>> for NonOwnedWindowInRust<IN_RUST> {
    fn from(o: FindReplaceDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindReplaceDialogInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FindReplaceDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindReplaceDialogInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FindReplaceDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FindReplaceDialogInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FindReplaceDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FindReplaceDialogInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFindReplaceDialog_CLASSINFO()) }
    }
}

// wxFlexGridSizer
wxwidgets! {
    /// A flex grid sizer is a sizer which lays out its children in a two-dimensional table with all table fields in one row having the same height and all fields in one column having the same width, but all rows or all columns are not necessarily the same height or width as in the wxGridSizer.
    /// - [`FlexGridSizer`] represents a C++ `wxFlexGridSizer` class instance which your code has ownership, [`FlexGridSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`FlexGridSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFlexGridSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html) for more details.
    #[doc(alias = "wxFlexGridSizer")]
    #[doc(alias = "FlexGridSizer")]
    class FlexGridSizer
        = FlexGridSizerInRust<true>(wxFlexGridSizer) impl
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FlexGridSizerInRust<IN_RUST> {
    /// wxFlexGridSizer constructors.
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a1b4a8cb492c6d89e3e3755c9b8d31f03).
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> FlexGridSizerInRust<IN_RUST> {
        unsafe { FlexGridSizerInRust(ffi::wxFlexGridSizer_new(cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a4c2fef6d9eca9c1d3ee3ee0ef41a4307).
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> FlexGridSizerInRust<IN_RUST> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerInRust(ffi::wxFlexGridSizer_new1(cols, gap))
        }
    }
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a12b3b94cf1fe8ea687c74c84b8eb892f).
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> FlexGridSizerInRust<IN_RUST> {
        unsafe { FlexGridSizerInRust(ffi::wxFlexGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a49c35e0580fec338a47c5f0f348515e7).
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> FlexGridSizerInRust<IN_RUST> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerInRust(ffi::wxFlexGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FlexGridSizerInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FlexGridSizerInRust<IN_RUST>> for GridSizerInRust<IN_RUST> {
    fn from(o: FlexGridSizerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FlexGridSizerInRust<IN_RUST>> for SizerInRust<IN_RUST> {
    fn from(o: FlexGridSizerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FlexGridSizerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FlexGridSizerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FlexGridSizerInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFlexGridSizer_CLASSINFO()) }
    }
}

// wxFocusEvent
wxwidgets! {
    /// A focus event is sent when a window's focus changes.
    /// - [`FocusEvent`] represents a C++ `wxFocusEvent` class instance which your code has ownership, [`FocusEventInRust`]`<false>` represents one which don't own.
    /// - Use [`FocusEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFocusEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_focus_event.html) for more details.
    #[doc(alias = "wxFocusEvent")]
    #[doc(alias = "FocusEvent")]
    class FocusEvent
        = FocusEventInRust<true>(wxFocusEvent) impl
        FocusEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FocusEventInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxFocusEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FocusEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FocusEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: FocusEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FocusEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FocusEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FocusEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFocusEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FocusEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFont
wxwidgets! {
    /// A font is an object which determines the appearance of text.
    /// - [`Font`] represents a C++ `wxFont` class instance which your code has ownership, [`FontInRust`]`<false>` represents one which don't own.
    /// - Use [`Font`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFont` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font.html) for more details.
    #[doc(alias = "wxFont")]
    #[doc(alias = "Font")]
    class Font
        = FontInRust<true>(wxFont) impl
        FontMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FontInRust<IN_RUST> {
    /// Default ctor.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#af183c2709f2b8f03e8fe88f28826612c).
    pub fn new() -> FontInRust<IN_RUST> {
        unsafe { FontInRust(ffi::wxFont_new()) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#ad77ff719ea7cf27f32d1de7e14dc76c1).
    pub fn new_with_font<F: FontMethods>(font: &F) -> FontInRust<IN_RUST> {
        unsafe {
            let font = font.as_ptr();
            FontInRust(ffi::wxFont_new1(font))
        }
    }
    /// Creates a font object using the specified font description.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a019f22fdd833bf6bfc16f7b795a68a45).
    pub fn new_with_fontinfo(font_info: *const c_void) -> FontInRust<IN_RUST> {
        unsafe { FontInRust(ffi::wxFont_new2(font_info)) }
    }
    // NOT_SUPPORTED: fn wxFont3()
    // NOT_SUPPORTED: fn wxFont4()
    /// Constructor from font description string.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a9d43775aaeafc628064b0e1b63730567).
    pub fn new_with_str(native_info_string: &str) -> FontInRust<IN_RUST> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            FontInRust(ffi::wxFont_new5(native_info_string))
        }
    }
    /// Construct font from a native font info structure.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a17d85e1cac77bf812182915f3a017976).
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        native_info: &N,
    ) -> FontInRust<IN_RUST> {
        unsafe {
            let native_info = native_info.as_ptr();
            FontInRust(ffi::wxFont_new6(native_info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FontInRust<IN_RUST>> for GDIObjectInRust<IN_RUST> {
    fn from(o: FontInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FontInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FontInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFont_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FontInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontData
wxwidgets! {
    /// This class holds a variety of information related to font dialogs.
    /// - [`FontData`] represents a C++ `wxFontData` class instance which your code has ownership, [`FontDataInRust`]`<false>` represents one which don't own.
    /// - Use [`FontData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html) for more details.
    #[doc(alias = "wxFontData")]
    #[doc(alias = "FontData")]
    class FontData
        = FontDataInRust<true>(wxFontData) impl
        FontDataMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FontDataInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxFontData::wxFontData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a7475bad933f864387b91f41ec26efd44).
    pub fn new() -> FontDataInRust<IN_RUST> {
        unsafe { FontDataInRust(ffi::wxFontData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontDataInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FontDataInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FontDataInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FontDataInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFontData_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FontDataInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontDialog
wxwidgets! {
    /// This class represents the font chooser dialog.
    /// - [`FontDialog`] represents a C++ `wxFontDialog` class instance which your code has ownership, [`FontDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`FontDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html) for more details.
    #[doc(alias = "wxFontDialog")]
    #[doc(alias = "FontDialog")]
    class FontDialog
        = FontDialogInRust<true>(wxFontDialog) impl
        FontDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FontDialogInRust<IN_RUST> {
    /// Default ctor.
    ///
    /// See [C++ `wxFontDialog::wxFontDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#acf0f377d6df63ea86e72df3972d5c1c2).
    pub fn new_2step() -> FontDialogInRust<IN_RUST> {
        unsafe { FontDialogInRust(ffi::wxFontDialog_new()) }
    }
    // BLOCKED: fn wxFontDialog1()
    /// Constructor.
    ///
    /// See [C++ `wxFontDialog::wxFontDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#a1dafd790fcdea244ceca846c6e62ab2e).
    pub fn new<W: WindowMethods, F: FontDataMethods>(
        parent: Option<&W>,
        data: &F,
    ) -> FontDialogInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = data.as_ptr();
            FontDialogInRust(ffi::wxFontDialog_new2(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FontDialogInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FontDialogInRust<IN_RUST>> for DialogInRust<IN_RUST> {
    fn from(o: FontDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontDialogInRust<IN_RUST>> for TopLevelWindowInRust<IN_RUST> {
    fn from(o: FontDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontDialogInRust<IN_RUST>> for NonOwnedWindowInRust<IN_RUST> {
    fn from(o: FontDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontDialogInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FontDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontDialogInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FontDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontDialogInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FontDialogInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FontDialogInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFontDialog_CLASSINFO()) }
    }
}

// wxFontEnumerator
wxwidgets! {
    /// wxFontEnumerator enumerates either all available fonts on the system or only the ones with given attributes - either only fixed-width (suited for use in programs such as terminal emulators and the like) or the fonts available in the given encoding).
    /// - [`FontEnumerator`] represents a C++ `wxFontEnumerator` class instance which your code has ownership, [`FontEnumeratorInRust`]`<false>` represents one which don't own.
    /// - Use [`FontEnumerator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontEnumerator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html) for more details.
    #[doc(alias = "wxFontEnumerator")]
    #[doc(alias = "FontEnumerator")]
    class FontEnumerator
        = FontEnumeratorInRust<true>(wxFontEnumerator) impl
        FontEnumeratorMethods
}
impl<const IN_RUST: bool> FontEnumeratorInRust<IN_RUST> {
    ///
    /// See [C++ `wxFontEnumerator::wxFontEnumerator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a4ea35d567889f590df8ea37d48b3bc98).
    pub fn new() -> FontEnumeratorInRust<IN_RUST> {
        unsafe { FontEnumeratorInRust(ffi::wxFontEnumerator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontEnumeratorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for FontEnumeratorInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxFontEnumerator_delete(self.0) }
        }
    }
}

// wxFontList
wxwidgets! {
    /// A font list is a list containing all fonts which have been created.
    /// - [`FontList`] represents a C++ `wxFontList` class instance which your code has ownership, [`FontListInRust`]`<false>` represents one which don't own.
    /// - Use [`FontList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_list.html) for more details.
    #[doc(alias = "wxFontList")]
    #[doc(alias = "FontList")]
    class FontList
        = FontListInRust<true>(wxFontList) impl
        FontListMethods
}
impl<const IN_RUST: bool> FontListInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxFontList::wxFontList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_list.html#a1ca7f23958bc81f12893d1602b3a037d).
    pub fn new() -> FontListInRust<IN_RUST> {
        unsafe { FontListInRust(ffi::wxFontList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontListInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for FontListInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxFontList_delete(self.0) }
        }
    }
}

// wxFontMapper
wxwidgets! {
    /// wxFontMapper manages user-definable correspondence between logical font names and the fonts present on the machine.
    /// - [`FontMapper`] represents a C++ `wxFontMapper` class instance which your code has ownership, [`FontMapperInRust`]`<false>` represents one which don't own.
    /// - Use [`FontMapper`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontMapper` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html) for more details.
    #[doc(alias = "wxFontMapper")]
    #[doc(alias = "FontMapper")]
    class FontMapper
        = FontMapperInRust<true>(wxFontMapper) impl
        FontMapperMethods
}
impl<const IN_RUST: bool> FontMapperInRust<IN_RUST> {
    /// Default ctor.
    ///
    /// See [C++ `wxFontMapper::wxFontMapper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a9eb3ae0550d0c858ba994a7d7a020441).
    pub fn new() -> FontMapperInRust<IN_RUST> {
        unsafe { FontMapperInRust(ffi::wxFontMapper_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontMapperInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for FontMapperInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxFontMapper_delete(self.0) }
        }
    }
}

// wxFontPickerCtrl
wxwidgets! {
    /// This control allows the user to select a font.
    /// - [`FontPickerCtrl`] represents a C++ `wxFontPickerCtrl` class instance which your code has ownership, [`FontPickerCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`FontPickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontPickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html) for more details.
    #[doc(alias = "wxFontPickerCtrl")]
    #[doc(alias = "FontPickerCtrl")]
    class FontPickerCtrl
        = FontPickerCtrlInRust<true>(wxFontPickerCtrl) impl
        FontPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FontPickerCtrlInRust<IN_RUST> {
    ///
    /// See [C++ `wxFontPickerCtrl::wxFontPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a391e4a0d0935941152501cce7d492fb6).
    pub fn new_2step() -> FontPickerCtrlInRust<IN_RUST> {
        unsafe { FontPickerCtrlInRust(ffi::wxFontPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxFontPickerCtrl::wxFontPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a840bd203045d1be1249cef9348839951).
    pub fn new<
        W: WindowMethods,
        F: FontMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        font: &F,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> FontPickerCtrlInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            FontPickerCtrlInRust(ffi::wxFontPickerCtrl_new1(
                parent, id, font, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FontPickerCtrlInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FontPickerCtrlInRust<IN_RUST>> for PickerBaseInRust<IN_RUST> {
    fn from(o: FontPickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontPickerCtrlInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: FontPickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontPickerCtrlInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FontPickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontPickerCtrlInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FontPickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontPickerCtrlInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FontPickerCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FontPickerCtrlInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFontPickerCtrl_CLASSINFO()) }
    }
}

// wxFontPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxFontPickerCtrl.
    /// - [`FontPickerEvent`] represents a C++ `wxFontPickerEvent` class instance which your code has ownership, [`FontPickerEventInRust`]`<false>` represents one which don't own.
    /// - Use [`FontPickerEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontPickerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html) for more details.
    #[doc(alias = "wxFontPickerEvent")]
    #[doc(alias = "FontPickerEvent")]
    class FontPickerEvent
        = FontPickerEventInRust<true>(wxFontPickerEvent) impl
        FontPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FontPickerEventInRust<IN_RUST> {
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxFontPickerEvent::wxFontPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html#a59e707304d36f12937605e1bff5df798).
    pub fn new<O: ObjectMethods, F: FontMethods>(
        generator: Option<&O>,
        id: c_int,
        font: &F,
    ) -> FontPickerEventInRust<IN_RUST> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            FontPickerEventInRust(ffi::wxFontPickerEvent_new(generator, id, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontPickerEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FontPickerEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: FontPickerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontPickerEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: FontPickerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FontPickerEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FontPickerEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FontPickerEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFontPickerEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for FontPickerEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFrame
wxwidgets! {
    /// A frame is a window whose size and position can (usually) be changed by the user.
    /// - [`Frame`] represents a C++ `wxFrame` class instance which your code has ownership, [`FrameInRust`]`<false>` represents one which don't own.
    /// - Use [`Frame`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFrame` class's documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html) for more details.
    #[doc(alias = "wxFrame")]
    #[doc(alias = "Frame")]
    class Frame
        = FrameInRust<true>(wxFrame) impl
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> FrameInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxFrame::wxFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc).
    pub fn new_2step() -> FrameInRust<IN_RUST> {
        unsafe { FrameInRust(ffi::wxFrame_new()) }
    }
    /// Constructor, creating the window.
    ///
    /// See [C++ `wxFrame::wxFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a01b53ac2d4a5e6b0773ecbcf7b5f6af8).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> FrameInRust<IN_RUST> {
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
            FrameInRust(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for FrameInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<FrameInRust<IN_RUST>> for TopLevelWindowInRust<IN_RUST> {
    fn from(o: FrameInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FrameInRust<IN_RUST>> for NonOwnedWindowInRust<IN_RUST> {
    fn from(o: FrameInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FrameInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: FrameInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FrameInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: FrameInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<FrameInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: FrameInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for FrameInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxFrame_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> TopLevelWindowMethods for FrameInRust<IN_RUST> {
    /// Used in two-step frame construction.
    ///
    /// See [C++ `wxFrame::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a6541d1aab71fc90041bfdde6e8705add).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
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
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl<const IN_RUST: bool> WindowMethods for FrameInRust<IN_RUST> {
    /// Centres the frame on the display.
    ///
    /// See [C++ `wxFrame::Centre()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a39b18ed552aabaf2a1bc4af7cc924a0f).
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr(), direction) }
    }
}
