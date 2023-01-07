use super::*;

// wxFileCtrl
/// This trait represents [C++ `wxFileCtrl` class](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html)'s methods and inheritance.
///
/// See [`FileCtrlFromCpp`] documentation for the class usage.
pub trait FileCtrlMethods: ControlMethods {
    /// Create function for two-step construction.
    ///
    /// See [C++ `wxFileCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#adfd64c940ac90a72a5b5931404c12c79).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        default_directory: &str,
        default_filename: &str,
        wild_card: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> bool {
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
            ffi::wxFileCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                default_directory,
                default_filename,
                wild_card,
                style,
                pos,
                size,
                name,
            )
        }
    }
    /// Returns the current directory of the file control (i.e. the directory shown by it).
    ///
    /// See [C++ `wxFileCtrl::GetDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a660189a738b48e42f682e616295a9657).
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetDirectory(self.as_ptr())).into() }
    }
    /// Returns the currently selected filename.
    ///
    /// See [C++ `wxFileCtrl::GetFilename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#aaf4b92dbfc7c0df9d55c90722872720c).
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetFilename(self.as_ptr())).into() }
    }
    /// Fills the array filenames with the filenames only of selected items.
    ///
    /// See [C++ `wxFileCtrl::GetFilenames()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a4e4018fdaccdb32cae774fec077432f9).
    fn get_filenames<A: ArrayStringMethods>(&self, filenames: &A) {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileCtrl_GetFilenames(self.as_ptr(), filenames)
        }
    }
    /// Returns the zero-based index of the currently selected filter.
    ///
    /// See [C++ `wxFileCtrl::GetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#af94c3a066652014972eb2090b6fd0b2d).
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileCtrl_GetFilterIndex(self.as_ptr()) }
    }
    /// Returns the full path (directory and filename) of the currently selected file.
    ///
    /// See [C++ `wxFileCtrl::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a3ba040d726dd6478c1f8c622e275cdd5).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetPath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the full paths of the files chosen.
    ///
    /// See [C++ `wxFileCtrl::GetPaths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a51aa11d5765bb4ce7050496e03774a8c).
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxFileCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Returns the current wildcard.
    ///
    /// See [C++ `wxFileCtrl::GetWildcard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a5c5f14c67967b6eedbbec191897f8ca5).
    fn get_wildcard(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetWildcard(self.as_ptr())).into() }
    }
    /// Sets(changes) the current directory displayed in the control.
    ///
    /// See [C++ `wxFileCtrl::SetDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#afc5c972073949ff303acd9f9834b0543).
    fn set_directory(&self, directory: &str) -> bool {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileCtrl_SetDirectory(self.as_ptr(), directory)
        }
    }
    /// Selects a certain file.
    ///
    /// See [C++ `wxFileCtrl::SetFilename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#aa6672f8ae14c8d59eb8312d8935eebd6).
    fn set_filename(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFileCtrl_SetFilename(self.as_ptr(), filename)
        }
    }
    /// Changes to a certain directory and selects a certain file.
    ///
    /// See [C++ `wxFileCtrl::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a588d1276adfe6ab5b53f41c7cf67a30d).
    fn set_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileCtrl_SetPath(self.as_ptr(), path)
        }
    }
    /// Sets the current filter index, starting from zero.
    ///
    /// See [C++ `wxFileCtrl::SetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a3e69759abe6cf646c4aff13d4bb32145).
    fn set_filter_index(&self, filter_index: c_int) {
        unsafe { ffi::wxFileCtrl_SetFilterIndex(self.as_ptr(), filter_index) }
    }
    /// Sets the wildcard, which can contain multiple file types, for example: "BMP files (*.bmp)|*.bmp|GIF files (*.gif)|*.gif".
    ///
    /// See [C++ `wxFileCtrl::SetWildcard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a7599cc5356e0d0f2b851374d591d9aeb).
    fn set_wildcard(&self, wild_card: &str) {
        unsafe {
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            ffi::wxFileCtrl_SetWildcard(self.as_ptr(), wild_card)
        }
    }
    /// Sets whether hidden files and folders are shown or not.
    ///
    /// See [C++ `wxFileCtrl::ShowHidden()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl.html#a062c1bbdd9c31f647286db8cac2990b8).
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxFileCtrl_ShowHidden(self.as_ptr(), show) }
    }
}

// wxFileCtrlEvent
/// This trait represents [C++ `wxFileCtrlEvent` class](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html)'s methods and inheritance.
///
/// See [`FileCtrlEventFromCpp`] documentation for the class usage.
pub trait FileCtrlEventMethods: CommandEventMethods {
    /// Returns the current directory.
    ///
    /// See [C++ `wxFileCtrlEvent::GetDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#a49aaa8fa4695ac25354f6bce528c1ace).
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrlEvent_GetDirectory(self.as_ptr())).into() }
    }
    /// Returns the file selected (assuming it is only one file).
    ///
    /// See [C++ `wxFileCtrlEvent::GetFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#a8cdf46598822faaff51128e20c2df43e).
    fn get_file(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrlEvent_GetFile(self.as_ptr())).into() }
    }
    /// Returns the files selected.
    ///
    /// See [C++ `wxFileCtrlEvent::GetFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#a615897eec0a3bb9a963cd5bd892d8c85).
    fn get_files(&self) -> ArrayString {
        unsafe { ArrayString::from_ptr(ffi::wxFileCtrlEvent_GetFiles(self.as_ptr())) }
    }
    /// Returns the current file filter index.
    ///
    /// See [C++ `wxFileCtrlEvent::GetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#a241db981f38e2cda43c6e2cbb7a1398f).
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileCtrlEvent_GetFilterIndex(self.as_ptr()) }
    }
    /// Sets the files changed by this event.
    ///
    /// See [C++ `wxFileCtrlEvent::SetFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#aa318626624419fedcfdb66e99f162371).
    fn set_files<A: ArrayStringMethods>(&self, files: &A) {
        unsafe {
            let files = files.as_ptr();
            ffi::wxFileCtrlEvent_SetFiles(self.as_ptr(), files)
        }
    }
    /// Sets the directory of this event.
    ///
    /// See [C++ `wxFileCtrlEvent::SetDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#a17fb78dfee5c02fc9c51bbd8dac02481).
    fn set_directory(&self, directory: &str) {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileCtrlEvent_SetDirectory(self.as_ptr(), directory)
        }
    }
    /// Sets the filter index changed by this event.
    ///
    /// See [C++ `wxFileCtrlEvent::SetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_ctrl_event.html#a24e26be83fabc7368fc53134789da2db).
    fn set_filter_index(&self, index: c_int) {
        unsafe { ffi::wxFileCtrlEvent_SetFilterIndex(self.as_ptr(), index) }
    }
}

// wxFileDataObject
/// This trait represents [C++ `wxFileDataObject` class](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html)'s methods and inheritance.
///
/// See [`FileDataObjectFromCpp`] documentation for the class usage.
pub trait FileDataObjectMethods: DataObjectSimpleMethods {
    /// Adds a file to the file list represented by this data object (Windows only).
    ///
    /// See [C++ `wxFileDataObject::AddFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html#af2e24d5d093a189b1a410a7f862123ba).
    fn add_file(&self, file: &str) {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileDataObject_AddFile(self.as_ptr(), file)
        }
    }
    /// Returns the array of file names.
    ///
    /// See [C++ `wxFileDataObject::GetFilenames()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_data_object.html#a40727417bb94728bb759b63314138b5b).
    fn get_filenames(&self) -> ArrayStringFromCpp<false> {
        unsafe { ArrayStringFromCpp::from_ptr(ffi::wxFileDataObject_GetFilenames(self.as_ptr())) }
    }
}

// wxFileDialog
/// This trait represents [C++ `wxFileDialog` class](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html)'s methods and inheritance.
///
/// See [`FileDialogFromCpp`] documentation for the class usage.
pub trait FileDialogMethods: DialogMethods {
    // DTOR: fn ~wxFileDialog()
    /// Returns the path of the file currently selected in dialog.
    ///
    /// See [C++ `wxFileDialog::GetCurrentlySelectedFilename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a05107400272572b7abec4ac831ef50c0).
    fn get_currently_selected_filename(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileDialog_GetCurrentlySelectedFilename(
                self.as_ptr(),
            ))
            .into()
        }
    }
    /// Returns the file type filter index currently selected in dialog.
    ///
    /// See [C++ `wxFileDialog::GetCurrentlySelectedFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#aec472cd5877d704244bb9b770bf5d910).
    fn get_currently_selected_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileDialog_GetCurrentlySelectedFilterIndex(self.as_ptr()) }
    }
    /// Returns the default directory.
    ///
    /// See [C++ `wxFileDialog::GetDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#ac9db32a1d92c89c00270dc2a2a865135).
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetDirectory(self.as_ptr())).into() }
    }
    /// If functions SetExtraControlCreator() and ShowModal() were called, returns the extra window.
    ///
    /// See [C++ `wxFileDialog::GetExtraControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a6ca6f9056340144945754b32f77ea838).
    fn get_extra_control(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxFileDialog_GetExtraControl(self.as_ptr())) }
    }
    /// Returns the default filename.
    ///
    /// See [C++ `wxFileDialog::GetFilename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a4d0948d01b050738c4b00dc11a26400b).
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetFilename(self.as_ptr())).into() }
    }
    /// Fills the array filenames with the names of the files chosen.
    ///
    /// See [C++ `wxFileDialog::GetFilenames()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#ac2c5811b6231008687e07b5a735e6063).
    fn get_filenames<A: ArrayStringMethods>(&self, filenames: &A) {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileDialog_GetFilenames(self.as_ptr(), filenames)
        }
    }
    /// Returns the index into the list of filters supplied, optionally, in the wildcard parameter.
    ///
    /// See [C++ `wxFileDialog::GetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#abf751784b7fff13f62c9679ec0963df0).
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileDialog_GetFilterIndex(self.as_ptr()) }
    }
    /// Returns the message that will be displayed on the dialog.
    ///
    /// See [C++ `wxFileDialog::GetMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#ad79729f7d758ac5b25a5991dbae94bdb).
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetMessage(self.as_ptr())).into() }
    }
    /// Returns the full path (directory and filename) of the selected file.
    ///
    /// See [C++ `wxFileDialog::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a9b81a1a4a9c6e6d22b5edd635862aff5).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetPath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the full paths of the files chosen.
    ///
    /// See [C++ `wxFileDialog::GetPaths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a0bc1a3ed7ef7c91720b64acc778e2c5c).
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxFileDialog_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Returns the file dialog wildcard.
    ///
    /// See [C++ `wxFileDialog::GetWildcard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#afb4ad06b7cc2df4c160bbfcb174e93c8).
    fn get_wildcard(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetWildcard(self.as_ptr())).into() }
    }
    /// Set the hook to be used for customizing the dialog contents.
    ///
    /// See [C++ `wxFileDialog::SetCustomizeHook()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#ab02b1018e1e045904656e1d094010bc2).
    fn set_customize_hook(&self, customize_hook: *mut c_void) -> bool {
        unsafe { ffi::wxFileDialog_SetCustomizeHook(self.as_ptr(), customize_hook) }
    }
    /// Sets the default directory.
    ///
    /// See [C++ `wxFileDialog::SetDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#ac9d3622241e377b3d7dae6fc80acdf88).
    fn set_directory(&self, directory: &str) {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileDialog_SetDirectory(self.as_ptr(), directory)
        }
    }
    // NOT_SUPPORTED: fn SetExtraControlCreator()
    /// Sets the default filename.
    ///
    /// See [C++ `wxFileDialog::SetFilename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#afb7fe41fee49450402a598dfa3526a61).
    fn set_filename(&self, setfilename: &str) {
        unsafe {
            let setfilename = WxString::from(setfilename);
            let setfilename = setfilename.as_ptr();
            ffi::wxFileDialog_SetFilename(self.as_ptr(), setfilename)
        }
    }
    /// Sets the default filter index, starting from zero.
    ///
    /// See [C++ `wxFileDialog::SetFilterIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a49a27f0f2550b9bc79ad26c9a19eaf28).
    fn set_filter_index(&self, filter_index: c_int) {
        unsafe { ffi::wxFileDialog_SetFilterIndex(self.as_ptr(), filter_index) }
    }
    /// Sets the message that will be displayed on the dialog.
    ///
    /// See [C++ `wxFileDialog::SetMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#a6b3ac95e95750179164111b36ec4e0ec).
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxFileDialog_SetMessage(self.as_ptr(), message)
        }
    }
    /// Sets the path (the combined directory and filename that will be returned when the dialog is dismissed).
    ///
    /// See [C++ `wxFileDialog::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#af81f0017c66440e2987648e5bc64f856).
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileDialog_SetPath(self.as_ptr(), path)
        }
    }
    /// Sets the wildcard, which can contain multiple file types, for example: "BMP files (*.bmp)|*.bmp|GIF files (*.gif)|*.gif".
    ///
    /// See [C++ `wxFileDialog::SetWildcard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dialog.html#aeb0137b721770bb390043b77cf2c09ff).
    fn set_wildcard(&self, wild_card: &str) {
        unsafe {
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            ffi::wxFileDialog_SetWildcard(self.as_ptr(), wild_card)
        }
    }
}

// wxFileDirPickerEvent
/// This trait represents [C++ `wxFileDirPickerEvent` class](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html)'s methods and inheritance.
///
/// See [`FileDirPickerEventFromCpp`] documentation for the class usage.
pub trait FileDirPickerEventMethods: CommandEventMethods {
    /// Retrieve the absolute path of the file/directory the user has just selected.
    ///
    /// See [C++ `wxFileDirPickerEvent::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html#ae6259bdc805313ceb5cdb826794aafe7).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDirPickerEvent_GetPath(self.as_ptr())).into() }
    }
    /// Set the absolute path of the file/directory associated with the event.
    ///
    /// See [C++ `wxFileDirPickerEvent::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_dir_picker_event.html#a4bdc140322bb16344da7ecf51aa216ce).
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileDirPickerEvent_SetPath(self.as_ptr(), path)
        }
    }
}

// wxFileDropTarget
/// This trait represents [C++ `wxFileDropTarget` class](https://docs.wxwidgets.org/3.2/classwx_file_drop_target.html)'s methods and inheritance.
///
/// See [`FileDropTargetFromCpp`] documentation for the class usage.
pub trait FileDropTargetMethods: DropTargetMethods {
    /// Override this function to receive dropped files.
    ///
    /// See [C++ `wxFileDropTarget::OnDropFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_drop_target.html#ad4f15d965332e232a59fe985e37d48f9).
    fn on_drop_files<A: ArrayStringMethods>(&self, x: c_int, y: c_int, filenames: &A) -> bool {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileDropTarget_OnDropFiles(self.as_ptr(), x, y, filenames)
        }
    }
}

// wxFileHistory
/// This trait represents [C++ `wxFileHistory` class](https://docs.wxwidgets.org/3.2/classwx_file_history.html)'s methods and inheritance.
///
/// See [`FileHistoryFromCpp`] documentation for the class usage.
pub trait FileHistoryMethods: ObjectMethods {
    // DTOR: fn ~wxFileHistory()
    /// Adds a file to the file history list, if the object has a pointer to an appropriate file menu.
    ///
    /// See [C++ `wxFileHistory::AddFileToHistory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a835839c473dfa42feecc78bb4798c79d).
    fn add_file_to_history(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFileHistory_AddFileToHistory(self.as_ptr(), filename)
        }
    }
    /// Appends the files in the history list, to all menus managed by the file history object.
    ///
    /// See [C++ `wxFileHistory::AddFilesToMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a8c77c1d51e18be0b54639953745481a6).
    fn add_files_to_menu(&self) {
        unsafe { ffi::wxFileHistory_AddFilesToMenu(self.as_ptr()) }
    }
    /// Appends the files in the history list, to the given menu only.
    ///
    /// See [C++ `wxFileHistory::AddFilesToMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a285f936b4c746f5214059a69357d049f).
    fn add_files_to_menu_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileHistory_AddFilesToMenu1(self.as_ptr(), menu)
        }
    }
    /// Returns the base identifier for the range used for appending items.
    ///
    /// See [C++ `wxFileHistory::GetBaseId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#ad4ac1e52985ede2e653d2b0cbc3d0fe9).
    fn get_base_id(&self) -> c_int {
        unsafe { ffi::wxFileHistory_GetBaseId(self.as_ptr()) }
    }
    /// Returns the number of files currently stored in the file history.
    ///
    /// See [C++ `wxFileHistory::GetCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a27cf5fe852e6fae1fb4cd390c8b48ff4).
    fn get_count(&self) -> usize {
        unsafe { ffi::wxFileHistory_GetCount(self.as_ptr()) }
    }
    /// Returns the file at this index (zero-based).
    ///
    /// See [C++ `wxFileHistory::GetHistoryFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#ad6e3e5fc63641cb3066ac41641e6a905).
    fn get_history_file(&self, index: usize) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileHistory_GetHistoryFile(self.as_ptr(), index)).into()
        }
    }
    /// Returns the maximum number of files that can be stored.
    ///
    /// See [C++ `wxFileHistory::GetMaxFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#aecb9ad3cb5e1953a8d6731672b84e37f).
    fn get_max_files(&self) -> c_int {
        unsafe { ffi::wxFileHistory_GetMaxFiles(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenus()
    /// Loads the file history from the given config object.
    ///
    /// See [C++ `wxFileHistory::Load()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a2537201495686a3a6a26701e158ecfb3).
    fn load(&self, config: *const c_void) {
        unsafe { ffi::wxFileHistory_Load(self.as_ptr(), config) }
    }
    /// Removes the specified file from the history.
    ///
    /// See [C++ `wxFileHistory::RemoveFileFromHistory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#ad9e2db9f45b538c19a240c55ccd60448).
    fn remove_file_from_history(&self, i: usize) {
        unsafe { ffi::wxFileHistory_RemoveFileFromHistory(self.as_ptr(), i) }
    }
    /// Removes this menu from the list of those managed by this object.
    ///
    /// See [C++ `wxFileHistory::RemoveMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a5ed5eb530eb2fce7207ff8688535423c).
    fn remove_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileHistory_RemoveMenu(self.as_ptr(), menu)
        }
    }
    /// Saves the file history into the given config object.
    ///
    /// See [C++ `wxFileHistory::Save()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#a0552296e381e7a3500e0813d2263930e).
    fn save(&self, config: *mut c_void) {
        unsafe { ffi::wxFileHistory_Save(self.as_ptr(), config) }
    }
    /// Sets the base identifier for the range used for appending items.
    ///
    /// See [C++ `wxFileHistory::SetBaseId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#ae778a6e4ade91d08b048082c3ec3566f).
    fn set_base_id(&self, base_id: c_int) {
        unsafe { ffi::wxFileHistory_SetBaseId(self.as_ptr(), base_id) }
    }
    /// Adds this menu to the list of those menus that are managed by this file history object.
    ///
    /// See [C++ `wxFileHistory::UseMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_history.html#ad73800aba67b30fc9d0dc94f17c4cc09).
    fn use_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileHistory_UseMenu(self.as_ptr(), menu)
        }
    }
    // NOT_SUPPORTED: fn SetMenuPathStyle()
    // NOT_SUPPORTED: fn GetMenuPathStyle()
}

// wxFilePickerCtrl
/// This trait represents [C++ `wxFilePickerCtrl` class](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html)'s methods and inheritance.
///
/// See [`FilePickerCtrlFromCpp`] documentation for the class usage.
pub trait FilePickerCtrlMethods: PickerBaseMethods {
    /// Creates this widget with the given parameters.
    ///
    /// See [C++ `wxFilePickerCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#a8e643314daae74a584fc1b25ade9c5bd).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
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
    ) -> bool {
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
            ffi::wxFilePickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                path,
                message,
                wildcard,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    /// Similar to GetPath() but returns the path of the currently selected file as a wxFileName object.
    ///
    /// See [C++ `wxFilePickerCtrl::GetFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#a252a2d16ef4de7649d77e013bf1a5f51).
    fn get_file_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFilePickerCtrl_GetFileName(self.as_ptr())) }
    }
    /// Returns the absolute path of the currently selected file.
    ///
    /// See [C++ `wxFilePickerCtrl::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#a38c4aa4c15e6b001aabebcb91f176a89).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFilePickerCtrl_GetPath(self.as_ptr())).into() }
    }
    /// This method does the same thing as SetPath() but takes a wxFileName object instead of a string.
    ///
    /// See [C++ `wxFilePickerCtrl::SetFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#ae191b11ea4a93273b84aba880a97e122).
    fn set_file_name<F: FileNameMethods>(&self, filename: &F) {
        unsafe {
            let filename = filename.as_ptr();
            ffi::wxFilePickerCtrl_SetFileName(self.as_ptr(), filename)
        }
    }
    /// Set the directory to show when starting to browse for files.
    ///
    /// See [C++ `wxFilePickerCtrl::SetInitialDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#ab36700ece10509deb7384f30ebf0baea).
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFilePickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    /// Sets the absolute path of the currently selected file.
    ///
    /// See [C++ `wxFilePickerCtrl::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_picker_ctrl.html#aa18c0c1598d60bc6ef526f7749c1e156).
    fn set_path(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFilePickerCtrl_SetPath(self.as_ptr(), filename)
        }
    }
}

// wxFindDialogEvent
/// This trait represents [C++ `wxFindDialogEvent` class](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html)'s methods and inheritance.
///
/// See [`FindDialogEventFromCpp`] documentation for the class usage.
pub trait FindDialogEventMethods: CommandEventMethods {
    /// Return the pointer to the dialog which generated this event.
    ///
    /// See [C++ `wxFindDialogEvent::GetDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html#a95a25700a04a3459f587648875bfc98c).
    fn get_dialog(&self) -> WeakRef<FindReplaceDialog> {
        unsafe {
            WeakRef::<FindReplaceDialog>::from(ffi::wxFindDialogEvent_GetDialog(self.as_ptr()))
        }
    }
    /// Return the string to find (never empty).
    ///
    /// See [C++ `wxFindDialogEvent::GetFindString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html#a45c2ae8d7dd483e6e5a3503512aaabb1).
    fn get_find_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindDialogEvent_GetFindString(self.as_ptr())).into() }
    }
    /// Get the currently selected flags: this is the combination of the wxFindReplaceFlags enumeration values.
    ///
    /// See [C++ `wxFindDialogEvent::GetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html#affe96aed3118b675aa372db502071022).
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFindDialogEvent_GetFlags(self.as_ptr()) }
    }
    /// Return the string to replace the search string with (only for replace and replace all events).
    ///
    /// See [C++ `wxFindDialogEvent::GetReplaceString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_dialog_event.html#a513ab8b8cfbb0f1eac38f4ef2f40c65d).
    fn get_replace_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindDialogEvent_GetReplaceString(self.as_ptr())).into() }
    }
}

// wxFindReplaceData
/// This trait represents [C++ `wxFindReplaceData` class](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html)'s methods and inheritance.
///
/// See [`FindReplaceDataFromCpp`] documentation for the class usage.
pub trait FindReplaceDataMethods: ObjectMethods {
    /// Get the string to find.
    ///
    /// See [C++ `wxFindReplaceData::GetFindString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html#af47ad3d2d9a9af5bac9a05f0a0a13259).
    fn get_find_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindReplaceData_GetFindString(self.as_ptr())).into() }
    }
    /// Get the combination of wxFindReplaceFlags values.
    ///
    /// See [C++ `wxFindReplaceData::GetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html#a7699fbfe922095e879801fcdce59ad0e).
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFindReplaceData_GetFlags(self.as_ptr()) }
    }
    /// Get the replacement string.
    ///
    /// See [C++ `wxFindReplaceData::GetReplaceString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html#a6a71a17d3e59180e44a693df725800af).
    fn get_replace_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindReplaceData_GetReplaceString(self.as_ptr())).into() }
    }
    /// Set the string to find (used as initial value by the dialog).
    ///
    /// See [C++ `wxFindReplaceData::SetFindString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html#ae8762e9353ddf76a6e1887b5f6b53d11).
    fn set_find_string(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxFindReplaceData_SetFindString(self.as_ptr(), str)
        }
    }
    // NOT_SUPPORTED: fn SetFlags()
    /// Set the replacement string (used as initial value by the dialog).
    ///
    /// See [C++ `wxFindReplaceData::SetReplaceString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_data.html#ae9371cf3a1e68a288cd4508f67f2a8d9).
    fn set_replace_string(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxFindReplaceData_SetReplaceString(self.as_ptr(), str)
        }
    }
}

// wxFindReplaceDialog
/// This trait represents [C++ `wxFindReplaceDialog` class](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html)'s methods and inheritance.
///
/// See [`FindReplaceDialogFromCpp`] documentation for the class usage.
pub trait FindReplaceDialogMethods: DialogMethods {
    // DTOR: fn ~wxFindReplaceDialog()
    /// Creates the dialog; use wxWindow::Show to show it on screen.
    ///
    /// See [C++ `wxFindReplaceDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#aa2f0a45997f9f2380eae2e3e635d6c94).
    fn create_findreplacedata<W: WindowMethods, F: FindReplaceDataMethods>(
        &self,
        parent: Option<&W>,
        data: Option<&F>,
        title: &str,
        style: c_int,
    ) -> bool {
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
            ffi::wxFindReplaceDialog_Create(self.as_ptr(), parent, data, title, style)
        }
    }
    /// Get the wxFindReplaceData object used by this dialog.
    ///
    /// See [C++ `wxFindReplaceDialog::GetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_find_replace_dialog.html#afa6c61f0e1b6af69cc6f2ec062440e68).
    fn get_data(&self) -> Option<FindReplaceDataFromCpp<false>> {
        unsafe { FindReplaceData::option_from(ffi::wxFindReplaceDialog_GetData(self.as_ptr())) }
    }
}

// wxFlexGridSizer
/// This trait represents [C++ `wxFlexGridSizer` class](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html)'s methods and inheritance.
///
/// See [`FlexGridSizerFromCpp`] documentation for the class usage.
pub trait FlexGridSizerMethods: GridSizerMethods {
    /// Specifies that column idx (starting from zero) should be grown if there is extra space available to the sizer.
    ///
    /// See [C++ `wxFlexGridSizer::AddGrowableCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a7d22045257180999e3705fbcd5585b6e).
    fn add_growable_col(&self, idx: usize, proportion: c_int) {
        unsafe { ffi::wxFlexGridSizer_AddGrowableCol(self.as_ptr(), idx, proportion) }
    }
    /// Specifies that row idx (starting from zero) should be grown if there is extra space available to the sizer.
    ///
    /// See [C++ `wxFlexGridSizer::AddGrowableRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a7c91a92ad1e01aac56222d36c4342d00).
    fn add_growable_row(&self, idx: usize, proportion: c_int) {
        unsafe { ffi::wxFlexGridSizer_AddGrowableRow(self.as_ptr(), idx, proportion) }
    }
    /// Returns a wxOrientation value that specifies whether the sizer flexibly resizes its columns, rows, or both (default).
    ///
    /// See [C++ `wxFlexGridSizer::GetFlexibleDirection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a27b2da0ec57edc710bc0ad5c7d4d04d5).
    fn get_flexible_direction(&self) -> c_int {
        unsafe { ffi::wxFlexGridSizer_GetFlexibleDirection(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetNonFlexibleGrowMode()
    /// Returns true if column idx is growable.
    ///
    /// See [C++ `wxFlexGridSizer::IsColGrowable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a04d611af7165db02e9971f9b19fad799).
    fn is_col_growable(&self, idx: usize) -> bool {
        unsafe { ffi::wxFlexGridSizer_IsColGrowable(self.as_ptr(), idx) }
    }
    /// Returns true if row idx is growable.
    ///
    /// See [C++ `wxFlexGridSizer::IsRowGrowable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a80e787496ee23a9dd5801f06d1dda55b).
    fn is_row_growable(&self, idx: usize) -> bool {
        unsafe { ffi::wxFlexGridSizer_IsRowGrowable(self.as_ptr(), idx) }
    }
    /// Specifies that the idx column index is no longer growable.
    ///
    /// See [C++ `wxFlexGridSizer::RemoveGrowableCol()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#aa25bb73aba1e3bba89ab65854692026b).
    fn remove_growable_col(&self, idx: usize) {
        unsafe { ffi::wxFlexGridSizer_RemoveGrowableCol(self.as_ptr(), idx) }
    }
    /// Specifies that the idx row index is no longer growable.
    ///
    /// See [C++ `wxFlexGridSizer::RemoveGrowableRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a8247a1ee496f60efe4e56b6460ed5ec8).
    fn remove_growable_row(&self, idx: usize) {
        unsafe { ffi::wxFlexGridSizer_RemoveGrowableRow(self.as_ptr(), idx) }
    }
    /// Specifies whether the sizer should flexibly resize its columns, rows, or both.
    ///
    /// See [C++ `wxFlexGridSizer::SetFlexibleDirection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a5c206d868699abb1e550cdbaa6cb19bc).
    fn set_flexible_direction(&self, direction: c_int) {
        unsafe { ffi::wxFlexGridSizer_SetFlexibleDirection(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn SetNonFlexibleGrowMode()
    /// Returns a read-only array containing the heights of the rows in the sizer.
    ///
    /// See [C++ `wxFlexGridSizer::GetRowHeights()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a6552b94de617ab2c022e2b9758c6e4f4).
    fn get_row_heights(&self) -> ArrayIntFromCpp<false> {
        unsafe { ArrayIntFromCpp::from_ptr(ffi::wxFlexGridSizer_GetRowHeights(self.as_ptr())) }
    }
    /// Returns a read-only array containing the widths of the columns in the sizer.
    ///
    /// See [C++ `wxFlexGridSizer::GetColWidths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_flex_grid_sizer.html#a719895a5bc97030744fdeed198ab6507).
    fn get_col_widths(&self) -> ArrayIntFromCpp<false> {
        unsafe { ArrayIntFromCpp::from_ptr(ffi::wxFlexGridSizer_GetColWidths(self.as_ptr())) }
    }
}

// wxFocusEvent
/// This trait represents [C++ `wxFocusEvent` class](https://docs.wxwidgets.org/3.2/classwx_focus_event.html)'s methods and inheritance.
///
/// See [`FocusEventFromCpp`] documentation for the class usage.
pub trait FocusEventMethods: EventMethods {
    /// Returns the window associated with this event, that is the window which had the focus before for the wxEVT_SET_FOCUS event and the window which is going to receive focus for the wxEVT_KILL_FOCUS one.
    ///
    /// See [C++ `wxFocusEvent::GetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_focus_event.html#a395fb0d49e650fc1900be64cfafedc7c).
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxFocusEvent_GetWindow(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxFocusEvent::SetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_focus_event.html#a0761d1e6d4d6e987ad4e2a8ee9e63f32).
    fn set_window<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFocusEvent_SetWindow(self.as_ptr(), win)
        }
    }
}

// wxFont
/// This trait represents [C++ `wxFont` class](https://docs.wxwidgets.org/3.2/classwx_font.html)'s methods and inheritance.
///
/// See [`FontFromCpp`] documentation for the class usage.
pub trait FontMethods: GDIObjectMethods {
    /// Returns a font with the same face/size as the given one but with normal weight and style and not underlined nor stricken through.
    ///
    /// See [C++ `wxFont::GetBaseFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#acdf46cdb37733a76e8a5024331095df9).
    fn get_base_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_GetBaseFont(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEncoding()
    /// Returns the face name associated with the font, or the empty string if there is no face information.
    ///
    /// See [C++ `wxFont::GetFaceName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a589ad627e9594e5dfeabdd5c063dc01d).
    fn get_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFamily()
    /// Returns the platform-dependent string completely describing this font.
    ///
    /// See [C++ `wxFont::GetNativeFontInfoDesc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a0668df773eaf781cd9979ec6707efe4e).
    fn get_native_font_info_desc(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetNativeFontInfoDesc(self.as_ptr())).into() }
    }
    /// Returns a user-friendly string for this font object.
    ///
    /// See [C++ `wxFont::GetNativeFontInfoUserDesc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a52abf8643ff20cbc7364de2a59cadf0c).
    fn get_native_font_info_user_desc(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetNativeFontInfoUserDesc(self.as_ptr())).into() }
    }
    /// Returns a font with the same face/size as the given one but with normal weight and style and not underlined nor stricken through.
    ///
    /// See [C++ `wxFont::GetNativeFontInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a4141a77677772ea283c1ce737695c77d).
    fn get_native_font_info(&self) -> Option<NativeFontInfoFromCpp<false>> {
        unsafe { NativeFontInfo::option_from(ffi::wxFont_GetNativeFontInfo(self.as_ptr())) }
    }
    /// Gets the point size as an integer number.
    ///
    /// See [C++ `wxFont::GetPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a0bb323d97e6628d1caffe4bd4f472623).
    fn get_point_size(&self) -> c_int {
        unsafe { ffi::wxFont_GetPointSize(self.as_ptr()) }
    }
    /// Gets the point size as a floating number.
    ///
    /// See [C++ `wxFont::GetFractionalPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a796687d506688f24128559423da7ac5a).
    fn get_fractional_point_size(&self) -> c_double {
        unsafe { ffi::wxFont_GetFractionalPointSize(self.as_ptr()) }
    }
    /// Gets the pixel size.
    ///
    /// See [C++ `wxFont::GetPixelSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a5c052abc341453c2ef1002c61df383b9).
    fn get_pixel_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxFont_GetPixelSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    /// Returns true if the font is underlined, false otherwise.
    ///
    /// See [C++ `wxFont::GetUnderlined()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#aa12d8be740d3bff6001e578aa0c97349).
    fn get_underlined(&self) -> bool {
        unsafe { ffi::wxFont_GetUnderlined(self.as_ptr()) }
    }
    /// Returns true if the font is stricken-through, false otherwise.
    ///
    /// See [C++ `wxFont::GetStrikethrough()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#aa11f9f946a14876d0736545e33c43645).
    fn get_strikethrough(&self) -> bool {
        unsafe { ffi::wxFont_GetStrikethrough(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWeight()
    /// Gets the font weight as an integer value.
    ///
    /// See [C++ `wxFont::GetNumericWeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a38a79006439e19d247fb680745f3cc7c).
    fn get_numeric_weight(&self) -> c_int {
        unsafe { ffi::wxFont_GetNumericWeight(self.as_ptr()) }
    }
    /// Returns true if the font is a fixed width (or monospaced) font, false if it is a proportional one or font is invalid.
    ///
    /// See [C++ `wxFont::IsFixedWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#adb4f2ad5f09f178b76f0822574500c39).
    fn is_fixed_width(&self) -> bool {
        unsafe { ffi::wxFont_IsFixedWidth(self.as_ptr()) }
    }
    /// Returns true if this object is a valid font, false otherwise.
    ///
    /// See [C++ `wxFont::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#aa5a818ce69a9867f9ca05bdf57b7badf).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFont_IsOk(self.as_ptr()) }
    }
    /// Specify the name of a file containing a TrueType font to be made available to the current application.
    ///
    /// See [C++ `wxFont::AddPrivateFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#ab96266bba385bc770a40ec4c7d1ec620).
    fn add_private_font(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFont_AddPrivateFont(filename)
        }
    }
    /// Returns a bold version of this font.
    ///
    /// See [C++ `wxFont::Bold()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#ad7b49b796335499ce86dffa8d02e30c1).
    fn bold(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Bold(self.as_ptr())) }
    }
    /// Returns an italic version of this font.
    ///
    /// See [C++ `wxFont::Italic()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a986c623afc3441b32c4fff892cb6a7ed).
    fn italic(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Italic(self.as_ptr())) }
    }
    /// Returns a larger version of this font.
    ///
    /// See [C++ `wxFont::Larger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a5ae0c8f11dd3b78b84be30f4aadb1c68).
    fn larger(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Larger(self.as_ptr())) }
    }
    /// Returns a smaller version of this font.
    ///
    /// See [C++ `wxFont::Smaller()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#acfeb26bc0fe1bce3d21c0a06daef1625).
    fn smaller(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Smaller(self.as_ptr())) }
    }
    /// Returns underlined version of this font.
    ///
    /// See [C++ `wxFont::Underlined()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a662846a3dfb53bc7d4f97ffe9b0b3897).
    fn underlined(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Underlined(self.as_ptr())) }
    }
    /// Returns stricken-through version of this font.
    ///
    /// See [C++ `wxFont::Strikethrough()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#af8b9ec6118e82bfa11ef43028d834512).
    fn strikethrough(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Strikethrough(self.as_ptr())) }
    }
    /// Changes this font to be bold.
    ///
    /// See [C++ `wxFont::MakeBold()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a80a2a8ee3d458e5e76179bc52dbd6bad).
    fn make_bold(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeBold(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be italic.
    ///
    /// See [C++ `wxFont::MakeItalic()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#afa489cd41ca406afdfa4ba42aa6c7458).
    fn make_italic(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeItalic(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be larger.
    ///
    /// See [C++ `wxFont::MakeLarger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a2aad6f665ebc3a97419758fb372e3511).
    fn make_larger(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeLarger(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be smaller.
    ///
    /// See [C++ `wxFont::MakeSmaller()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a181bba83280f4e898c69a273ba9c4055).
    fn make_smaller(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeSmaller(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be underlined.
    ///
    /// See [C++ `wxFont::MakeUnderlined()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a8d41bd009bf9f630d8efa312ccfdc388).
    fn make_underlined(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeUnderlined(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be stricken-through.
    ///
    /// See [C++ `wxFont::MakeStrikethrough()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#ad0fc68e7e9af9221f5be84d0beb8edaa).
    fn make_strikethrough(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeStrikethrough(self.as_ptr());
            &self
        }
    }
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Scaled()
    // NOT_SUPPORTED: fn SetEncoding()
    /// Sets the facename for the font.
    ///
    /// See [C++ `wxFont::SetFaceName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a97456a61332a1b6d44a76f0b57b01709).
    fn set_face_name(&self, face_name: &str) -> bool {
        unsafe {
            let face_name = WxString::from(face_name);
            let face_name = face_name.as_ptr();
            ffi::wxFont_SetFaceName(self.as_ptr(), face_name)
        }
    }
    // NOT_SUPPORTED: fn SetFamily()
    /// Creates the font corresponding to the given native font description string which must have been previously returned by GetNativeFontInfoDesc().
    ///
    /// See [C++ `wxFont::SetNativeFontInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#aaef8dc8fc6c8b81246af000a201b52c5).
    fn set_native_font_info_str(&self, info: &str) -> bool {
        unsafe {
            let info = WxString::from(info);
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfo(self.as_ptr(), info)
        }
    }
    /// Creates the font corresponding to the given native font description string and returns true if the creation was successful.
    ///
    /// See [C++ `wxFont::SetNativeFontInfoUserDesc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#afa8eb67298aea006e3a262bd7b0c8493).
    fn set_native_font_info_user_desc(&self, info: &str) -> bool {
        unsafe {
            let info = WxString::from(info);
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfoUserDesc(self.as_ptr(), info)
        }
    }
    /// Sets the encoding for this font.
    ///
    /// See [C++ `wxFont::SetNativeFontInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a31bb8005da8e4172a0f8e3fecc04efef).
    fn set_native_font_info_nativefontinfo<N: NativeFontInfoMethods>(&self, info: &N) {
        unsafe {
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfo1(self.as_ptr(), info)
        }
    }
    /// Sets the font size in points to an integer value.
    ///
    /// See [C++ `wxFont::SetPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a32d54af99749f180991c4cdab9f9a7dd).
    fn set_point_size(&self, point_size: c_int) {
        unsafe { ffi::wxFont_SetPointSize(self.as_ptr(), point_size) }
    }
    /// Sets the font size in points.
    ///
    /// See [C++ `wxFont::SetFractionalPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#aee03b736fa49851362444b74794c2c60).
    fn set_fractional_point_size(&self, point_size: c_double) {
        unsafe { ffi::wxFont_SetFractionalPointSize(self.as_ptr(), point_size) }
    }
    /// Sets the pixel size.
    ///
    /// See [C++ `wxFont::SetPixelSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a94f364735647de9ddd065dfa992ad4e4).
    fn set_pixel_size<S: SizeMethods>(&self, pixel_size: &S) {
        unsafe {
            let pixel_size = pixel_size.as_ptr();
            ffi::wxFont_SetPixelSize(self.as_ptr(), pixel_size)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    // NOT_SUPPORTED: fn SetSymbolicSize()
    // NOT_SUPPORTED: fn SetSymbolicSizeRelativeTo()
    /// Sets underlining.
    ///
    /// See [C++ `wxFont::SetUnderlined()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a2a8a3a71090bfa4ed957dfd1ffcc524c).
    fn set_underlined(&self, underlined: bool) {
        unsafe { ffi::wxFont_SetUnderlined(self.as_ptr(), underlined) }
    }
    /// Sets strike-through attribute of the font.
    ///
    /// See [C++ `wxFont::SetStrikethrough()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a074f14f7bea2d493d7ec14a65f76a3b7).
    fn set_strikethrough(&self, strikethrough: bool) {
        unsafe { ffi::wxFont_SetStrikethrough(self.as_ptr(), strikethrough) }
    }
    // NOT_SUPPORTED: fn SetWeight()
    /// Sets the font weight using an integer value.
    ///
    /// See [C++ `wxFont::SetNumericWeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#ad527c7be417f4a1d4789ad0f60ceaec0).
    fn set_numeric_weight(&self, weight: c_int) {
        unsafe { ffi::wxFont_SetNumericWeight(self.as_ptr(), weight) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator=()
    // NOT_SUPPORTED: fn GetDefaultEncoding()
    // NOT_SUPPORTED: fn SetDefaultEncoding()
    // NOT_SUPPORTED: fn GetNumericWeightOf()
    // NOT_SUPPORTED: fn New()
    // NOT_SUPPORTED: fn New1()
    // NOT_SUPPORTED: fn New2()
    // NOT_SUPPORTED: fn New3()
    /// Sets the encoding for this font.
    ///
    /// See [C++ `wxFont::New()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a554845d97040706a2c3407dd968e9f62).
    fn new_nativefontinfo<N: NativeFontInfoMethods>(native_info: &N) -> Option<FontFromCpp<false>> {
        unsafe {
            let native_info = native_info.as_ptr();
            Font::option_from(ffi::wxFont_New4(native_info))
        }
    }
    /// Sets the encoding for this font.
    ///
    /// See [C++ `wxFont::New()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font.html#a6cfbe4bb51490bec78637204d3f18edd).
    fn new_str(native_info_string: &str) -> Option<FontFromCpp<false>> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            Font::option_from(ffi::wxFont_New5(native_info_string))
        }
    }
    // DTOR: fn ~wxFont()
}

// wxFontData
/// This trait represents [C++ `wxFontData` class](https://docs.wxwidgets.org/3.2/classwx_font_data.html)'s methods and inheritance.
///
/// See [`FontDataFromCpp`] documentation for the class usage.
pub trait FontDataMethods: ObjectMethods {
    /// Enables or disables "effects" under Windows or generic only.
    ///
    /// See [C++ `wxFontData::EnableEffects()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a5d2430c6959bea4a023a87c0e606c31a).
    fn enable_effects(&self, enable: bool) {
        unsafe { ffi::wxFontData_EnableEffects(self.as_ptr(), enable) }
    }
    /// Under Windows, returns a flag determining whether symbol fonts can be selected.
    ///
    /// See [C++ `wxFontData::GetAllowSymbols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a0804c60917cf51d289f004838a8fc0ef).
    fn get_allow_symbols(&self) -> bool {
        unsafe { ffi::wxFontData_GetAllowSymbols(self.as_ptr()) }
    }
    /// Gets the font chosen by the user if the user pressed OK (wxFontDialog::ShowModal() returned wxID_OK).
    ///
    /// See [C++ `wxFontData::GetChosenFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a066dead81cc01854df744ba711ac606e).
    fn get_chosen_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontData_GetChosenFont(self.as_ptr())) }
    }
    /// Gets the colour associated with the font dialog.
    ///
    /// See [C++ `wxFontData::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#ab64b3bc54a2c518d2124f93197646ff8).
    fn get_colour(&self) -> ColourFromCpp<false> {
        unsafe { ColourFromCpp::from_ptr(ffi::wxFontData_GetColour(self.as_ptr())) }
    }
    /// Determines whether "effects" are enabled under Windows.
    ///
    /// See [C++ `wxFontData::GetEnableEffects()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a3a5598ba073c197a65e8ca6f29a3a528).
    fn get_enable_effects(&self) -> bool {
        unsafe { ffi::wxFontData_GetEnableEffects(self.as_ptr()) }
    }
    /// Returns the state of the flags restricting the selection.
    ///
    /// See [C++ `wxFontData::GetRestrictSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a74416a26717eb9e8ea8e461c88047e65).
    fn get_restrict_selection(&self) -> c_int {
        unsafe { ffi::wxFontData_GetRestrictSelection(self.as_ptr()) }
    }
    /// Gets the font that will be initially used by the font dialog.
    ///
    /// See [C++ `wxFontData::GetInitialFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a92124db2663fba43472ea374182ee664).
    fn get_initial_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontData_GetInitialFont(self.as_ptr())) }
    }
    /// Returns true if the Help button will be shown (Windows only).
    ///
    /// See [C++ `wxFontData::GetShowHelp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#aa5fc469cbbc12ab6d61588a617b66f4a).
    fn get_show_help(&self) -> bool {
        unsafe { ffi::wxFontData_GetShowHelp(self.as_ptr()) }
    }
    /// Restricts the selection to a subset of the available fonts.
    ///
    /// See [C++ `wxFontData::RestrictSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a2cdfe592ee6b3ba9dced7bc4e08fc0be).
    fn restrict_selection(&self, flags: c_int) {
        unsafe { ffi::wxFontData_RestrictSelection(self.as_ptr(), flags) }
    }
    /// Under Windows, determines whether symbol fonts can be selected.
    ///
    /// See [C++ `wxFontData::SetAllowSymbols()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a0bd2d264044fcf568c1d731e9d19c32c).
    fn set_allow_symbols(&self, allow_symbols: bool) {
        unsafe { ffi::wxFontData_SetAllowSymbols(self.as_ptr(), allow_symbols) }
    }
    /// Sets the font that will be returned to the user (for internal use only).
    ///
    /// See [C++ `wxFontData::SetChosenFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a316d4574ef7100e4e5f3e12bff7d98dc).
    fn set_chosen_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontData_SetChosenFont(self.as_ptr(), font)
        }
    }
    /// Sets the colour that will be used for the font foreground colour.
    ///
    /// See [C++ `wxFontData::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a52e4308bf17d933b5b2948f6eb469608).
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxFontData_SetColour(self.as_ptr(), colour)
        }
    }
    /// Sets the font that will be initially used by the font dialog.
    ///
    /// See [C++ `wxFontData::SetInitialFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#a24fd79c0a3eb6764cc7d525acba9aaeb).
    fn set_initial_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontData_SetInitialFont(self.as_ptr(), font)
        }
    }
    /// Sets the valid range for the font point size (Windows only).
    ///
    /// See [C++ `wxFontData::SetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#ac3d2f78a59cd50d5e3a045775a83fff5).
    fn set_range(&self, min: c_int, max: c_int) {
        unsafe { ffi::wxFontData_SetRange(self.as_ptr(), min, max) }
    }
    /// Determines whether the Help button will be displayed in the font dialog (Windows only).
    ///
    /// See [C++ `wxFontData::SetShowHelp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_data.html#ad203acb97b8278907994f254f96f63ce).
    fn set_show_help(&self, show_help: bool) {
        unsafe { ffi::wxFontData_SetShowHelp(self.as_ptr(), show_help) }
    }
    // BLOCKED: fn operator=()
}

// wxFontDialog
/// This trait represents [C++ `wxFontDialog` class](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html)'s methods and inheritance.
///
/// See [`FontDialogFromCpp`] documentation for the class usage.
pub trait FontDialogMethods: DialogMethods {
    /// Creates the dialog if the wxFontDialog object had been initialized using the default constructor.
    ///
    /// See [C++ `wxFontDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#aa03d1bdc247fee02c20360373a4bb228).
    fn create<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFontDialog_Create(self.as_ptr(), parent)
        }
    }
    /// Creates the dialog if the wxFontDialog object had been initialized using the default constructor.
    ///
    /// See [C++ `wxFontDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#a3d1125778e41dea1ea2dd357f74c2749).
    fn create_fontdata<W: WindowMethods, F: FontDataMethods>(
        &self,
        parent: Option<&W>,
        data: &F,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = data.as_ptr();
            ffi::wxFontDialog_Create1(self.as_ptr(), parent, data)
        }
    }
    // BLOCKED: fn GetFontData()
    ///
    /// See [C++ `wxFontDialog::GetFontData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_dialog.html#a5c0f427480a66edca8bef13eccd67b2c).
    fn get_font_data(&self) -> FontDataFromCpp<false> {
        unsafe { FontDataFromCpp::from_ptr(ffi::wxFontDialog_GetFontData1(self.as_ptr())) }
    }
}

// wxFontEnumerator
/// This trait represents [C++ `wxFontEnumerator` class](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html)'s methods and inheritance.
///
/// See [`FontEnumeratorFromCpp`] documentation for the class usage.
pub trait FontEnumeratorMethods: WxRustMethods {
    // DTOR: fn ~wxFontEnumerator()
    /// Call OnFontEncoding() for each encoding supported by the given font - or for each encoding supported by at least some font if font is not specified.
    ///
    /// See [C++ `wxFontEnumerator::EnumerateEncodings()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a38dfb993bdb5ba19359679eb37f4296b).
    fn enumerate_encodings(&self, font: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxFontEnumerator_EnumerateEncodings(self.as_ptr(), font)
        }
    }
    // NOT_SUPPORTED: fn EnumerateFacenames()
    /// Called by EnumerateFacenames() for each match.
    ///
    /// See [C++ `wxFontEnumerator::OnFacename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#ad9566d37c45e857c7e885f6d23a66848).
    fn on_facename(&self, font: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxFontEnumerator_OnFacename(self.as_ptr(), font)
        }
    }
    /// Called by EnumerateEncodings() for each match.
    ///
    /// See [C++ `wxFontEnumerator::OnFontEncoding()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a5a100fb6f9cf812d0982ab83a0c07644).
    fn on_font_encoding(&self, font: &str, encoding: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            let encoding = WxString::from(encoding);
            let encoding = encoding.as_ptr();
            ffi::wxFontEnumerator_OnFontEncoding(self.as_ptr(), font, encoding)
        }
    }
    /// Return array of strings containing all encodings found by EnumerateEncodings().
    ///
    /// See [C++ `wxFontEnumerator::GetEncodings()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a66503923926dbdc9bcf050fe3e15af06).
    fn get_encodings(facename: &str) -> ArrayString {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ArrayString::from_ptr(ffi::wxFontEnumerator_GetEncodings(facename))
        }
    }
    // NOT_SUPPORTED: fn GetFacenames()
    /// Returns true if the given string is valid face name, i.e.
    ///
    /// See [C++ `wxFontEnumerator::IsValidFacename()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#a8e11919b1a3bbbd662ba6087c7bc9943).
    fn is_valid_facename(facename: &str) -> bool {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ffi::wxFontEnumerator_IsValidFacename(facename)
        }
    }
    /// Invalidate cache used by some of the methods of this class internally.
    ///
    /// See [C++ `wxFontEnumerator::InvalidateCache()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_enumerator.html#aad3227af9b6479ae40b833d255622d2f).
    fn invalidate_cache() {
        unsafe { ffi::wxFontEnumerator_InvalidateCache() }
    }
}

// wxFontList
/// This trait represents [C++ `wxFontList` class](https://docs.wxwidgets.org/3.2/classwx_font_list.html)'s methods and inheritance.
///
/// See [`FontListFromCpp`] documentation for the class usage.
pub trait FontListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreateFont()
    /// Finds a font of the given specification, or creates one and adds it to the list.
    ///
    /// See [C++ `wxFontList::FindOrCreateFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_list.html#a13af1bba1c223887cefc59544a91cbcc).
    fn find_or_create_font(&self, font_info: *const c_void) -> Option<FontFromCpp<false>> {
        unsafe { Font::option_from(ffi::wxFontList_FindOrCreateFont1(self.as_ptr(), font_info)) }
    }
}

// wxFontMapper
/// This trait represents [C++ `wxFontMapper` class](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html)'s methods and inheritance.
///
/// See [`FontMapperFromCpp`] documentation for the class usage.
pub trait FontMapperMethods: WxRustMethods {
    // DTOR: fn ~wxFontMapper()
    // NOT_SUPPORTED: fn CharsetToEncoding()
    // NOT_SUPPORTED: fn GetAltForEncoding()
    // NOT_SUPPORTED: fn GetAltForEncoding1()
    // NOT_SUPPORTED: fn IsEncodingAvailable()
    /// Set the root config path to use (should be an absolute path).
    ///
    /// See [C++ `wxFontMapper::SetConfigPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a5999b3950e0a78e3e8890d96abe37a97).
    fn set_config_path(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFontMapper_SetConfigPath(self.as_ptr(), prefix)
        }
    }
    /// The parent window for modal dialogs.
    ///
    /// See [C++ `wxFontMapper::SetDialogParent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a2fc0db252f9a52356f1484c684c6f8b0).
    fn set_dialog_parent<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFontMapper_SetDialogParent(self.as_ptr(), parent)
        }
    }
    /// The title for the dialogs (note that default is quite reasonable).
    ///
    /// See [C++ `wxFontMapper::SetDialogTitle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a7459de236727f25d40a57414f10850c1).
    fn set_dialog_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxFontMapper_SetDialogTitle(self.as_ptr(), title)
        }
    }
    /// Get the current font mapper object.
    ///
    /// See [C++ `wxFontMapper::Get()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#ae0718e57b4e91c1b2f38cac352c0a9d5).
    fn get() -> Option<FontMapperFromCpp<false>> {
        unsafe { FontMapper::option_from(ffi::wxFontMapper_Get()) }
    }
    // NOT_SUPPORTED: fn GetAllEncodingNames()
    // NOT_SUPPORTED: fn GetEncoding()
    // NOT_SUPPORTED: fn GetEncodingDescription()
    // NOT_SUPPORTED: fn GetEncodingFromName()
    // NOT_SUPPORTED: fn GetEncodingName()
    /// Returns the number of the font encodings supported by this class.
    ///
    /// See [C++ `wxFontMapper::GetSupportedEncodingsCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a49c41e77de4bb68d8ceb2c22d182a819).
    fn get_supported_encodings_count() -> usize {
        unsafe { ffi::wxFontMapper_GetSupportedEncodingsCount() }
    }
    /// Set the current font mapper object and return previous one (may be NULL).
    ///
    /// See [C++ `wxFontMapper::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_mapper.html#a41b0051189bfe57f3fabde0f42c5635c).
    fn set<F: FontMapperMethods>(mapper: Option<&F>) -> Option<FontMapperFromCpp<false>> {
        unsafe {
            let mapper = match mapper {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            FontMapper::option_from(ffi::wxFontMapper_Set(mapper))
        }
    }
}

// wxFontPickerCtrl
/// This trait represents [C++ `wxFontPickerCtrl` class](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html)'s methods and inheritance.
///
/// See [`FontPickerCtrlFromCpp`] documentation for the class usage.
pub trait FontPickerCtrlMethods: PickerBaseMethods {
    /// Creates this widget with given parameters.
    ///
    /// See [C++ `wxFontPickerCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#aee60a47fb3de5b61f4c79823fcf2bd29).
    fn create_font<
        W: WindowMethods,
        F: FontMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        font: &F,
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
            let font = font.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFontPickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                font,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    /// Returns the maximum point size value allowed for the user-chosen font.
    ///
    /// See [C++ `wxFontPickerCtrl::GetMaxPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a800e8e8c3d682a62be54090a01c7da84).
    fn get_max_point_size(&self) -> c_uint {
        unsafe { ffi::wxFontPickerCtrl_GetMaxPointSize(self.as_ptr()) }
    }
    /// Returns the minimum point size value allowed for the user-chosen font.
    ///
    /// See [C++ `wxFontPickerCtrl::GetMinPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a6558efa57fcbe1c49881587e5c0ef213).
    fn get_min_point_size(&self) -> c_uint {
        unsafe { ffi::wxFontPickerCtrl_GetMinPointSize(self.as_ptr()) }
    }
    /// Returns the currently selected colour.
    ///
    /// See [C++ `wxFontPickerCtrl::GetSelectedColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a9f92eb7430015cc4c3e2193e7a0c3362).
    fn get_selected_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxFontPickerCtrl_GetSelectedColour(self.as_ptr())) }
    }
    /// Returns the currently selected font.
    ///
    /// See [C++ `wxFontPickerCtrl::GetSelectedFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a853e238c49133786fe421f1093e5695a).
    fn get_selected_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontPickerCtrl_GetSelectedFont(self.as_ptr())) }
    }
    /// Sets the maximum point size value allowed for the user-chosen font.
    ///
    /// See [C++ `wxFontPickerCtrl::SetMaxPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a63b4c361a3b9162ce101e00673828721).
    fn set_max_point_size(&self, max: c_uint) {
        unsafe { ffi::wxFontPickerCtrl_SetMaxPointSize(self.as_ptr(), max) }
    }
    /// Sets the minimum point size value allowed for the user-chosen font.
    ///
    /// See [C++ `wxFontPickerCtrl::SetMinPointSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#ad0177fdae6d9ed31a4a4672cd872e2b1).
    fn set_min_point_size(&self, min: c_uint) {
        unsafe { ffi::wxFontPickerCtrl_SetMinPointSize(self.as_ptr(), min) }
    }
    /// Sets the font colour.
    ///
    /// See [C++ `wxFontPickerCtrl::SetSelectedColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#a37e28bb0369f95bcb5da71302f643307).
    fn set_selected_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxFontPickerCtrl_SetSelectedColour(self.as_ptr(), colour)
        }
    }
    /// Sets the currently selected font.
    ///
    /// See [C++ `wxFontPickerCtrl::SetSelectedFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_ctrl.html#ada07e418ed99431e5012fb9409b729a7).
    fn set_selected_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontPickerCtrl_SetSelectedFont(self.as_ptr(), font)
        }
    }
}

// wxFontPickerEvent
/// This trait represents [C++ `wxFontPickerEvent` class](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html)'s methods and inheritance.
///
/// See [`FontPickerEventFromCpp`] documentation for the class usage.
pub trait FontPickerEventMethods: CommandEventMethods {
    /// Retrieve the font the user has just selected.
    ///
    /// See [C++ `wxFontPickerEvent::GetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html#a1bbde3a3dbd884754c7d18a7a5b52354).
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontPickerEvent_GetFont(self.as_ptr())) }
    }
    /// Set the font associated with the event.
    ///
    /// See [C++ `wxFontPickerEvent::SetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_font_picker_event.html#a4731df639d37c8056b96f9dec9db52d0).
    fn set_font<F: FontMethods>(&self, f: &F) {
        unsafe {
            let f = f.as_ptr();
            ffi::wxFontPickerEvent_SetFont(self.as_ptr(), f)
        }
    }
}

// wxFrame
/// This trait represents [C++ `wxFrame` class](https://docs.wxwidgets.org/3.2/classwx_frame.html)'s methods and inheritance.
///
/// See [`FrameFromCpp`] documentation for the class usage.
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    /// Creates a status bar at the bottom of the frame.
    ///
    /// See [C++ `wxFrame::CreateStatusBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a76709944aa2a1bae45c48bf2be4908b2).
    fn create_status_bar(
        &self,
        number: c_int,
        style: c_long,
        id: c_int,
        name: &str,
    ) -> WeakRef<StatusBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<StatusBar>::from(ffi::wxFrame_CreateStatusBar(
                self.as_ptr(),
                number,
                style,
                id,
                name,
            ))
        }
    }
    /// Creates a toolbar at the top or left of the frame.
    ///
    /// See [C++ `wxFrame::CreateToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#ab133b1f25421d2932f1bafcc3a8fd1f1).
    fn create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<ToolBar>::from(ffi::wxFrame_CreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    /// Method used to show help string of the selected menu toolbar item.
    ///
    /// See [C++ `wxFrame::DoGiveHelp()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#ab316b7ec4a5c63810cdb05318d5aeb26).
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_DoGiveHelp(self.as_ptr(), text, show)
        }
    }
    /// Returns a pointer to the menubar currently associated with the frame (if any).
    ///
    /// See [C++ `wxFrame::GetMenuBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a468c573ad848c17543dddc4550cae351).
    fn get_menu_bar(&self) -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxFrame_GetMenuBar(self.as_ptr())) }
    }
    /// Returns a pointer to the status bar currently associated with the frame (if any).
    ///
    /// See [C++ `wxFrame::GetStatusBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a075a4c41c80819bc252c8cbd27258ef3).
    fn get_status_bar(&self) -> WeakRef<StatusBar> {
        unsafe { WeakRef::<StatusBar>::from(ffi::wxFrame_GetStatusBar(self.as_ptr())) }
    }
    /// Returns the status bar pane used to display menu and toolbar help.
    ///
    /// See [C++ `wxFrame::GetStatusBarPane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#ad36e13c10ea3675c18f2fe480cd7b952).
    fn get_status_bar_pane(&self) -> c_int {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr()) }
    }
    /// Returns a pointer to the toolbar currently associated with the frame (if any).
    ///
    /// See [C++ `wxFrame::GetToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#aa6c57209ee6b5efb6a4661ac0d9923f4).
    fn get_tool_bar(&self) -> WeakRef<ToolBar> {
        unsafe { WeakRef::<ToolBar>::from(ffi::wxFrame_GetToolBar(self.as_ptr())) }
    }
    /// Virtual function called when a status bar is requested by CreateStatusBar().
    ///
    /// See [C++ `wxFrame::OnCreateStatusBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#adb6d9e361ae0fe2f0cb17cf9d5030dcf).
    fn on_create_status_bar(
        &self,
        number: c_int,
        style: c_long,
        id: c_int,
        name: &str,
    ) -> WeakRef<StatusBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<StatusBar>::from(ffi::wxFrame_OnCreateStatusBar(
                self.as_ptr(),
                number,
                style,
                id,
                name,
            ))
        }
    }
    /// Virtual function called when a toolbar is requested by CreateToolBar().
    ///
    /// See [C++ `wxFrame::OnCreateToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a2c23ede57762d475a8d0a6741983ab9f).
    fn on_create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<ToolBar>::from(ffi::wxFrame_OnCreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    /// Simulate a menu command.
    ///
    /// See [C++ `wxFrame::ProcessCommand()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a3dfab71a356ba08da4f065ca6c108b74).
    fn process_command(&self, id: c_int) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr(), id) }
    }
    /// Tells the frame to show the given menu bar.
    ///
    /// See [C++ `wxFrame::SetMenuBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a83e63c48e1a3f14e661de3ccd434cbbf).
    fn set_menu_bar<M: MenuBarMethods>(&self, menu_bar: Option<&M>) {
        unsafe {
            let menu_bar = match menu_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetMenuBar(self.as_ptr(), menu_bar)
        }
    }
    /// Associates a status bar with the frame.
    ///
    /// See [C++ `wxFrame::SetStatusBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a1c3714836fab3f8b892e18b45eb011b0).
    fn set_status_bar<S: StatusBarMethods>(&self, status_bar: Option<&S>) {
        unsafe {
            let status_bar = match status_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetStatusBar(self.as_ptr(), status_bar)
        }
    }
    /// Set the status bar pane used to display menu and toolbar help.
    ///
    /// See [C++ `wxFrame::SetStatusBarPane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a8a82de139a9a44ce638fe978c00ac07f).
    fn set_status_bar_pane(&self, n: c_int) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr(), n) }
    }
    /// Sets the status bar text and updates the status bar display.
    ///
    /// See [C++ `wxFrame::SetStatusText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a0026c883df35e1d1f8818e229c41249f).
    fn set_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_SetStatusText(self.as_ptr(), text, number)
        }
    }
    /// Sets the widths of the fields in the status bar.
    ///
    /// See [C++ `wxFrame::SetStatusWidths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a5e872935a702d136a4dafb808c4a2456).
    fn set_status_widths(&self, n: c_int, widths_field: *const c_void) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr(), n, widths_field) }
    }
    /// Associates a toolbar with the frame.
    ///
    /// See [C++ `wxFrame::SetToolBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#ab4017f727aa97560f51457d7302b0ca5).
    fn set_tool_bar<T: ToolBarMethods>(&self, tool_bar: Option<&T>) {
        unsafe {
            let tool_bar = match tool_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetToolBar(self.as_ptr(), tool_bar)
        }
    }
    /// MSW-specific function for accessing the taskbar button under Windows 7 or later.
    ///
    /// See [C++ `wxFrame::MSWGetTaskBarButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a555acbda4f137bedaec8895262f1617a).
    fn msw_get_task_bar_button(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_MSWGetTaskBarButton(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxFrame::PushStatusText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#a65346afc0ded65a824c13fedca1bf7a5).
    fn push_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_PushStatusText(self.as_ptr(), text, number)
        }
    }
    ///
    /// See [C++ `wxFrame::PopStatusText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_frame.html#aec7e735264adfd53e99fb0ccb506f87c).
    fn pop_status_text(&self, number: c_int) {
        unsafe { ffi::wxFrame_PopStatusText(self.as_ptr(), number) }
    }
}
