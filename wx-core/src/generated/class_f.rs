use super::*;

// wxFileCtrl
wxwidgets! {
    /// This control allows the user to select a file.
    /// - [`FileCtrl`] represents a C++ `wxFileCtrl` class instance which your code has ownership, [`FileCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html) for more details.
    #[doc(alias = "wxFileCtrl")]
    #[doc(alias = "FileCtrl")]
    class FileCtrl
        = FileCtrlFromCpp<true>(wxFileCtrl) impl
        FileCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FileCtrlFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxFileCtrl::wxFileCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#ad0f3935004f16b380571ead2d1b4c04c).
    pub fn new_2step() -> FileCtrlFromCpp<FROM_CPP> {
        unsafe { FileCtrlFromCpp(ffi::wxFileCtrl_new()) }
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
    ) -> FileCtrlFromCpp<FROM_CPP> {
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
            FileCtrlFromCpp(ffi::wxFileCtrl_new1(
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
impl<const FROM_CPP: bool> Clone for FileCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: FileCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FileCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: FileCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FileCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FileCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFileCtrl_CLASSINFO()) }
    }
}

// wxFileCtrlEvent
wxwidgets! {
    /// A file control event holds information about events associated with wxFileCtrl objects.
    /// - [`FileCtrlEvent`] represents a C++ `wxFileCtrlEvent` class instance which your code has ownership, [`FileCtrlEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileCtrlEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileCtrlEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html) for more details.
    #[doc(alias = "wxFileCtrlEvent")]
    #[doc(alias = "FileCtrlEvent")]
    class FileCtrlEvent
        = FileCtrlEventFromCpp<true>(wxFileCtrlEvent) impl
        FileCtrlEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FileCtrlEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxFileCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileCtrlEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileCtrlEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: FileCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileCtrlEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: FileCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileCtrlEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FileCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FileCtrlEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFileCtrlEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FileCtrlEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDataObject
wxwidgets! {
    /// wxFileDataObject is a specialization of wxDataObject for file names.
    /// - [`FileDataObject`] represents a C++ `wxFileDataObject` class instance which your code has ownership, [`FileDataObjectFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html) for more details.
    #[doc(alias = "wxFileDataObject")]
    #[doc(alias = "FileDataObject")]
    class FileDataObject
        = FileDataObjectFromCpp<true>(wxFileDataObject) impl
        FileDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> FileDataObjectFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxFileDataObject::wxFileDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html#a7c7cb54a6cf9114de7dec67755ac749e).
    pub fn new() -> FileDataObjectFromCpp<FROM_CPP> {
        unsafe { FileDataObjectFromCpp(ffi::wxFileDataObject_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDataObjectFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileDataObjectFromCpp<FROM_CPP>>
    for DataObjectSimpleFromCpp<FROM_CPP>
{
    fn from(o: FileDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDataObjectFromCpp<FROM_CPP>> for DataObjectFromCpp<FROM_CPP> {
    fn from(o: FileDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for FileDataObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxFileDataObject_delete(self.0) }
        }
    }
}

// wxFileDialog
wxwidgets! {
    /// This class represents the file chooser dialog.
    /// - [`FileDialog`] represents a C++ `wxFileDialog` class instance which your code has ownership, [`FileDialogFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html) for more details.
    #[doc(alias = "wxFileDialog")]
    #[doc(alias = "FileDialog")]
    class FileDialog
        = FileDialogFromCpp<true>(wxFileDialog) impl
        FileDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FileDialogFromCpp<FROM_CPP> {
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
    ) -> FileDialogFromCpp<FROM_CPP> {
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
            FileDialogFromCpp(ffi::wxFileDialog_new(
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
impl<const FROM_CPP: bool> Clone for FileDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: FileDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDialogFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: FileDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDialogFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: FileDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FileDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: FileDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FileDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FileDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFileDialog_CLASSINFO()) }
    }
}

// wxFileDirPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxFilePickerCtrl and by wxDirPickerCtrl.
    /// - [`FileDirPickerEvent`] represents a C++ `wxFileDirPickerEvent` class instance which your code has ownership, [`FileDirPickerEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileDirPickerEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDirPickerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html) for more details.
    #[doc(alias = "wxFileDirPickerEvent")]
    #[doc(alias = "FileDirPickerEvent")]
    class FileDirPickerEvent
        = FileDirPickerEventFromCpp<true>(wxFileDirPickerEvent) impl
        FileDirPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FileDirPickerEventFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxFileDirPickerEvent::wxFileDirPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html#a311f6e70d669ca9ef6f4425c7778f215).
    pub fn new() -> FileDirPickerEventFromCpp<FROM_CPP> {
        unsafe { FileDirPickerEventFromCpp(ffi::wxFileDirPickerEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxFileDirPickerEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDirPickerEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileDirPickerEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: FileDirPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDirPickerEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: FileDirPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FileDirPickerEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FileDirPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FileDirPickerEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFileDirPickerEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FileDirPickerEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileDropTarget
wxwidgets! {
    /// This is a drop target which accepts files (dragged from File Manager or Explorer).
    /// - [`FileDropTarget`] represents a C++ `wxFileDropTarget` class instance which your code has ownership, [`FileDropTargetFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileDropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_drop_target.html) for more details.
    #[doc(alias = "wxFileDropTarget")]
    #[doc(alias = "FileDropTarget")]
    class FileDropTarget
        = FileDropTargetFromCpp<true>(wxFileDropTarget) impl
        FileDropTargetMethods,
        DropTargetMethods
}
impl<const FROM_CPP: bool> FileDropTargetFromCpp<FROM_CPP> {
    // BLOCKED: fn wxFileDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileDropTargetFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileDropTargetFromCpp<FROM_CPP>> for DropTargetFromCpp<FROM_CPP> {
    fn from(o: FileDropTargetFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for FileDropTargetFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxFileDropTarget_delete(self.0) }
        }
    }
}

// wxFileHistory
wxwidgets! {
    /// The wxFileHistory encapsulates a user interface convenience, the list of most recently visited files as shown on a menu (usually the File menu).
    /// - [`FileHistory`] represents a C++ `wxFileHistory` class instance which your code has ownership, [`FileHistoryFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FileHistory`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFileHistory` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html) for more details.
    #[doc(alias = "wxFileHistory")]
    #[doc(alias = "FileHistory")]
    class FileHistory
        = FileHistoryFromCpp<true>(wxFileHistory) impl
        FileHistoryMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FileHistoryFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxFileHistory::wxFileHistory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a30e3a3a1f92fc253cc0fc69eb6f27fd8).
    pub fn new(max_files: usize, id_base: c_int) -> FileHistoryFromCpp<FROM_CPP> {
        unsafe { FileHistoryFromCpp(ffi::wxFileHistory_new(max_files, id_base)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileHistoryFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FileHistoryFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FileHistoryFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FileHistoryFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFileHistory_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FileHistoryFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFilePickerCtrl
wxwidgets! {
    /// This control allows the user to select a file.
    /// - [`FilePickerCtrl`] represents a C++ `wxFilePickerCtrl` class instance which your code has ownership, [`FilePickerCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FilePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFilePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html) for more details.
    #[doc(alias = "wxFilePickerCtrl")]
    #[doc(alias = "FilePickerCtrl")]
    class FilePickerCtrl
        = FilePickerCtrlFromCpp<true>(wxFilePickerCtrl) impl
        FilePickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FilePickerCtrlFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxFilePickerCtrl::wxFilePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#afe16cc740765fb8ec68c9df250a07812).
    pub fn new_2step() -> FilePickerCtrlFromCpp<FROM_CPP> {
        unsafe { FilePickerCtrlFromCpp(ffi::wxFilePickerCtrl_new()) }
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
    ) -> FilePickerCtrlFromCpp<FROM_CPP> {
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
            FilePickerCtrlFromCpp(ffi::wxFilePickerCtrl_new1(
                parent, id, path, message, wildcard, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for FilePickerCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FilePickerCtrlFromCpp<FROM_CPP>> for PickerBaseFromCpp<FROM_CPP> {
    fn from(o: FilePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FilePickerCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: FilePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FilePickerCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FilePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FilePickerCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: FilePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FilePickerCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FilePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FilePickerCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFilePickerCtrl_CLASSINFO()) }
    }
}

// wxFindDialogEvent
wxwidgets! {
    /// wxFindReplaceDialog events.
    /// - [`FindDialogEvent`] represents a C++ `wxFindDialogEvent` class instance which your code has ownership, [`FindDialogEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FindDialogEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFindDialogEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html) for more details.
    #[doc(alias = "wxFindDialogEvent")]
    #[doc(alias = "FindDialogEvent")]
    class FindDialogEvent
        = FindDialogEventFromCpp<true>(wxFindDialogEvent) impl
        FindDialogEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FindDialogEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxFindDialogEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FindDialogEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FindDialogEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: FindDialogEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindDialogEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: FindDialogEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindDialogEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FindDialogEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FindDialogEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFindDialogEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FindDialogEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceData
wxwidgets! {
    /// wxFindReplaceData holds the data for wxFindReplaceDialog.
    /// - [`FindReplaceData`] represents a C++ `wxFindReplaceData` class instance which your code has ownership, [`FindReplaceDataFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FindReplaceData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFindReplaceData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html) for more details.
    #[doc(alias = "wxFindReplaceData")]
    #[doc(alias = "FindReplaceData")]
    class FindReplaceData
        = FindReplaceDataFromCpp<true>(wxFindReplaceData) impl
        FindReplaceDataMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FindReplaceDataFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxFindReplaceData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FindReplaceDataFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDataFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FindReplaceDataFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FindReplaceDataFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFindReplaceData_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FindReplaceDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFindReplaceDialog
wxwidgets! {
    /// wxFindReplaceDialog is a standard modeless dialog which is used to allow the user to search for some text (and possibly replace it with something else).
    /// - [`FindReplaceDialog`] represents a C++ `wxFindReplaceDialog` class instance which your code has ownership, [`FindReplaceDialogFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FindReplaceDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFindReplaceDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html) for more details.
    #[doc(alias = "wxFindReplaceDialog")]
    #[doc(alias = "FindReplaceDialog")]
    class FindReplaceDialog
        = FindReplaceDialogFromCpp<true>(wxFindReplaceDialog) impl
        FindReplaceDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FindReplaceDialogFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxFindReplaceDialog::wxFindReplaceDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#a10601867d5793096323ce0979f7993cd).
    pub fn new_2step() -> FindReplaceDialogFromCpp<FROM_CPP> {
        unsafe { FindReplaceDialogFromCpp(ffi::wxFindReplaceDialog_new()) }
    }
    /// After using default constructor Create() must be called.
    ///
    /// See [C++ `wxFindReplaceDialog::wxFindReplaceDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#a27c6a7da82dd935ec7a9812ea6bf73c3).
    pub fn new<W: WindowMethods, F: FindReplaceDataMethods>(
        parent: Option<&W>,
        data: Option<&F>,
        title: &str,
        style: c_int,
    ) -> FindReplaceDialogFromCpp<FROM_CPP> {
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
            FindReplaceDialogFromCpp(ffi::wxFindReplaceDialog_new1(parent, data, title, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for FindReplaceDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: FindReplaceDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDialogFromCpp<FROM_CPP>>
    for TopLevelWindowFromCpp<FROM_CPP>
{
    fn from(o: FindReplaceDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDialogFromCpp<FROM_CPP>>
    for NonOwnedWindowFromCpp<FROM_CPP>
{
    fn from(o: FindReplaceDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FindReplaceDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDialogFromCpp<FROM_CPP>>
    for EvtHandlerFromCpp<FROM_CPP>
{
    fn from(o: FindReplaceDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FindReplaceDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FindReplaceDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FindReplaceDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFindReplaceDialog_CLASSINFO()) }
    }
}

// wxFlexGridSizer
wxwidgets! {
    /// A flex grid sizer is a sizer which lays out its children in a two-dimensional table with all table fields in one row having the same height and all fields in one column having the same width, but all rows or all columns are not necessarily the same height or width as in the wxGridSizer.
    /// - [`FlexGridSizer`] represents a C++ `wxFlexGridSizer` class instance which your code has ownership, [`FlexGridSizerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FlexGridSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFlexGridSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html) for more details.
    #[doc(alias = "wxFlexGridSizer")]
    #[doc(alias = "FlexGridSizer")]
    class FlexGridSizer
        = FlexGridSizerFromCpp<true>(wxFlexGridSizer) impl
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FlexGridSizerFromCpp<FROM_CPP> {
    /// wxFlexGridSizer constructors.
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a1b4a8cb492c6d89e3e3755c9b8d31f03).
    pub fn new_with_int_int(
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> FlexGridSizerFromCpp<FROM_CPP> {
        unsafe { FlexGridSizerFromCpp(ffi::wxFlexGridSizer_new(cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a4c2fef6d9eca9c1d3ee3ee0ef41a4307).
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> FlexGridSizerFromCpp<FROM_CPP> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerFromCpp(ffi::wxFlexGridSizer_new1(cols, gap))
        }
    }
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a12b3b94cf1fe8ea687c74c84b8eb892f).
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> FlexGridSizerFromCpp<FROM_CPP> {
        unsafe { FlexGridSizerFromCpp(ffi::wxFlexGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxFlexGridSizer::wxFlexGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a49c35e0580fec338a47c5f0f348515e7).
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> FlexGridSizerFromCpp<FROM_CPP> {
        unsafe {
            let gap = gap.as_ptr();
            FlexGridSizerFromCpp(ffi::wxFlexGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for FlexGridSizerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FlexGridSizerFromCpp<FROM_CPP>> for GridSizerFromCpp<FROM_CPP> {
    fn from(o: FlexGridSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FlexGridSizerFromCpp<FROM_CPP>> for SizerFromCpp<FROM_CPP> {
    fn from(o: FlexGridSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FlexGridSizerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FlexGridSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FlexGridSizerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFlexGridSizer_CLASSINFO()) }
    }
}

// wxFocusEvent
wxwidgets! {
    /// A focus event is sent when a window's focus changes.
    /// - [`FocusEvent`] represents a C++ `wxFocusEvent` class instance which your code has ownership, [`FocusEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FocusEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFocusEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_focus_event.html) for more details.
    #[doc(alias = "wxFocusEvent")]
    #[doc(alias = "FocusEvent")]
    class FocusEvent
        = FocusEventFromCpp<true>(wxFocusEvent) impl
        FocusEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FocusEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxFocusEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FocusEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FocusEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: FocusEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FocusEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FocusEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FocusEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFocusEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FocusEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFont
wxwidgets! {
    /// A font is an object which determines the appearance of text.
    /// - [`Font`] represents a C++ `wxFont` class instance which your code has ownership, [`FontFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Font`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFont` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font.html) for more details.
    #[doc(alias = "wxFont")]
    #[doc(alias = "Font")]
    class Font
        = FontFromCpp<true>(wxFont) impl
        FontMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FontFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#af183c2709f2b8f03e8fe88f28826612c).
    pub fn new() -> FontFromCpp<FROM_CPP> {
        unsafe { FontFromCpp(ffi::wxFont_new()) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#ad77ff719ea7cf27f32d1de7e14dc76c1).
    pub fn new_with_font<F: FontMethods>(font: &F) -> FontFromCpp<FROM_CPP> {
        unsafe {
            let font = font.as_ptr();
            FontFromCpp(ffi::wxFont_new1(font))
        }
    }
    /// Creates a font object using the specified font description.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a019f22fdd833bf6bfc16f7b795a68a45).
    pub fn new_with_fontinfo(font_info: *const c_void) -> FontFromCpp<FROM_CPP> {
        unsafe { FontFromCpp(ffi::wxFont_new2(font_info)) }
    }
    // NOT_SUPPORTED: fn wxFont3()
    // NOT_SUPPORTED: fn wxFont4()
    /// Constructor from font description string.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a9d43775aaeafc628064b0e1b63730567).
    pub fn new_with_str(native_info_string: &str) -> FontFromCpp<FROM_CPP> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            FontFromCpp(ffi::wxFont_new5(native_info_string))
        }
    }
    /// Construct font from a native font info structure.
    ///
    /// See [C++ `wxFont::wxFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a17d85e1cac77bf812182915f3a017976).
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        native_info: &N,
    ) -> FontFromCpp<FROM_CPP> {
        unsafe {
            let native_info = native_info.as_ptr();
            FontFromCpp(ffi::wxFont_new6(native_info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FontFromCpp<FROM_CPP>> for GDIObjectFromCpp<FROM_CPP> {
    fn from(o: FontFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FontFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FontFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFont_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FontFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontData
wxwidgets! {
    /// This class holds a variety of information related to font dialogs.
    /// - [`FontData`] represents a C++ `wxFontData` class instance which your code has ownership, [`FontDataFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontData`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html) for more details.
    #[doc(alias = "wxFontData")]
    #[doc(alias = "FontData")]
    class FontData
        = FontDataFromCpp<true>(wxFontData) impl
        FontDataMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FontDataFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxFontData::wxFontData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a7475bad933f864387b91f41ec26efd44).
    pub fn new() -> FontDataFromCpp<FROM_CPP> {
        unsafe { FontDataFromCpp(ffi::wxFontData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontDataFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FontDataFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FontDataFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FontDataFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFontData_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FontDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFontDialog
wxwidgets! {
    /// This class represents the font chooser dialog.
    /// - [`FontDialog`] represents a C++ `wxFontDialog` class instance which your code has ownership, [`FontDialogFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html) for more details.
    #[doc(alias = "wxFontDialog")]
    #[doc(alias = "FontDialog")]
    class FontDialog
        = FontDialogFromCpp<true>(wxFontDialog) impl
        FontDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FontDialogFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxFontDialog::wxFontDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#acf0f377d6df63ea86e72df3972d5c1c2).
    pub fn new_2step() -> FontDialogFromCpp<FROM_CPP> {
        unsafe { FontDialogFromCpp(ffi::wxFontDialog_new()) }
    }
    // BLOCKED: fn wxFontDialog1()
    /// Constructor.
    ///
    /// See [C++ `wxFontDialog::wxFontDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#a1dafd790fcdea244ceca846c6e62ab2e).
    pub fn new<W: WindowMethods, F: FontDataMethods>(
        parent: Option<&W>,
        data: &F,
    ) -> FontDialogFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = data.as_ptr();
            FontDialogFromCpp(ffi::wxFontDialog_new2(parent, data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for FontDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FontDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: FontDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontDialogFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: FontDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontDialogFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: FontDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FontDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: FontDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FontDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FontDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFontDialog_CLASSINFO()) }
    }
}

// wxFontEnumerator
wxwidgets! {
    /// wxFontEnumerator enumerates either all available fonts on the system or only the ones with given attributes - either only fixed-width (suited for use in programs such as terminal emulators and the like) or the fonts available in the given encoding).
    /// - [`FontEnumerator`] represents a C++ `wxFontEnumerator` class instance which your code has ownership, [`FontEnumeratorFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontEnumerator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontEnumerator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html) for more details.
    #[doc(alias = "wxFontEnumerator")]
    #[doc(alias = "FontEnumerator")]
    class FontEnumerator
        = FontEnumeratorFromCpp<true>(wxFontEnumerator) impl
        FontEnumeratorMethods
}
impl<const FROM_CPP: bool> FontEnumeratorFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxFontEnumerator::wxFontEnumerator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a4ea35d567889f590df8ea37d48b3bc98).
    pub fn new() -> FontEnumeratorFromCpp<FROM_CPP> {
        unsafe { FontEnumeratorFromCpp(ffi::wxFontEnumerator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontEnumeratorFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for FontEnumeratorFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxFontEnumerator_delete(self.0) }
        }
    }
}

// wxFontList
wxwidgets! {
    /// A font list is a list containing all fonts which have been created.
    /// - [`FontList`] represents a C++ `wxFontList` class instance which your code has ownership, [`FontListFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_list.html) for more details.
    #[doc(alias = "wxFontList")]
    #[doc(alias = "FontList")]
    class FontList
        = FontListFromCpp<true>(wxFontList) impl
        FontListMethods
}
impl<const FROM_CPP: bool> FontListFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxFontList::wxFontList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_list.html#a1ca7f23958bc81f12893d1602b3a037d).
    pub fn new() -> FontListFromCpp<FROM_CPP> {
        unsafe { FontListFromCpp(ffi::wxFontList_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontListFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for FontListFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxFontList_delete(self.0) }
        }
    }
}

// wxFontMapper
wxwidgets! {
    /// wxFontMapper manages user-definable correspondence between logical font names and the fonts present on the machine.
    /// - [`FontMapper`] represents a C++ `wxFontMapper` class instance which your code has ownership, [`FontMapperFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontMapper`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontMapper` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html) for more details.
    #[doc(alias = "wxFontMapper")]
    #[doc(alias = "FontMapper")]
    class FontMapper
        = FontMapperFromCpp<true>(wxFontMapper) impl
        FontMapperMethods
}
impl<const FROM_CPP: bool> FontMapperFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxFontMapper::wxFontMapper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a9eb3ae0550d0c858ba994a7d7a020441).
    pub fn new() -> FontMapperFromCpp<FROM_CPP> {
        unsafe { FontMapperFromCpp(ffi::wxFontMapper_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontMapperFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for FontMapperFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxFontMapper_delete(self.0) }
        }
    }
}

// wxFontPickerCtrl
wxwidgets! {
    /// This control allows the user to select a font.
    /// - [`FontPickerCtrl`] represents a C++ `wxFontPickerCtrl` class instance which your code has ownership, [`FontPickerCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontPickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontPickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html) for more details.
    #[doc(alias = "wxFontPickerCtrl")]
    #[doc(alias = "FontPickerCtrl")]
    class FontPickerCtrl
        = FontPickerCtrlFromCpp<true>(wxFontPickerCtrl) impl
        FontPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FontPickerCtrlFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxFontPickerCtrl::wxFontPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a391e4a0d0935941152501cce7d492fb6).
    pub fn new_2step() -> FontPickerCtrlFromCpp<FROM_CPP> {
        unsafe { FontPickerCtrlFromCpp(ffi::wxFontPickerCtrl_new()) }
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
    ) -> FontPickerCtrlFromCpp<FROM_CPP> {
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
            FontPickerCtrlFromCpp(ffi::wxFontPickerCtrl_new1(
                parent, id, font, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for FontPickerCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FontPickerCtrlFromCpp<FROM_CPP>> for PickerBaseFromCpp<FROM_CPP> {
    fn from(o: FontPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontPickerCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: FontPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontPickerCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FontPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontPickerCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: FontPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontPickerCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FontPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FontPickerCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFontPickerCtrl_CLASSINFO()) }
    }
}

// wxFontPickerEvent
wxwidgets! {
    /// This event class is used for the events generated by wxFontPickerCtrl.
    /// - [`FontPickerEvent`] represents a C++ `wxFontPickerEvent` class instance which your code has ownership, [`FontPickerEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`FontPickerEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFontPickerEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html) for more details.
    #[doc(alias = "wxFontPickerEvent")]
    #[doc(alias = "FontPickerEvent")]
    class FontPickerEvent
        = FontPickerEventFromCpp<true>(wxFontPickerEvent) impl
        FontPickerEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FontPickerEventFromCpp<FROM_CPP> {
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxFontPickerEvent::wxFontPickerEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html#a59e707304d36f12937605e1bff5df798).
    pub fn new<O: ObjectMethods, F: FontMethods>(
        generator: Option<&O>,
        id: c_int,
        font: &F,
    ) -> FontPickerEventFromCpp<FROM_CPP> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let font = font.as_ptr();
            FontPickerEventFromCpp(ffi::wxFontPickerEvent_new(generator, id, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FontPickerEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FontPickerEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: FontPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontPickerEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: FontPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FontPickerEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FontPickerEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FontPickerEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFontPickerEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for FontPickerEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFrame
wxwidgets! {
    /// A frame is a window whose size and position can (usually) be changed by the user.
    /// - [`Frame`] represents a C++ `wxFrame` class instance which your code has ownership, [`FrameFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Frame`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxFrame` class's documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html) for more details.
    #[doc(alias = "wxFrame")]
    #[doc(alias = "Frame")]
    class Frame
        = FrameFromCpp<true>(wxFrame) impl
        FrameMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> FrameFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxFrame::wxFrame()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#af80368ba23c71c5d947c3178b8fe10fc).
    pub fn new_2step() -> FrameFromCpp<FROM_CPP> {
        unsafe { FrameFromCpp(ffi::wxFrame_new()) }
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
    ) -> FrameFromCpp<FROM_CPP> {
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
            FrameFromCpp(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for FrameFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<FrameFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: FrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FrameFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: FrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FrameFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: FrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FrameFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: FrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<FrameFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: FrameFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for FrameFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxFrame_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> TopLevelWindowMethods for FrameFromCpp<FROM_CPP> {
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
impl<const FROM_CPP: bool> WindowMethods for FrameFromCpp<FROM_CPP> {
    /// Centres the frame on the display.
    ///
    /// See [C++ `wxFrame::Centre()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a39b18ed552aabaf2a1bc4af7cc924a0f).
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr(), direction) }
    }
}
