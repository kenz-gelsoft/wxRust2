use super::*;

// wxFileCtrl
pub trait FileCtrlMethods: ControlMethods {
    /// Create function for two-step construction.
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
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetDirectory(self.as_ptr())).into() }
    }
    /// Returns the currently selected filename.
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetFilename(self.as_ptr())).into() }
    }
    /// Fills the array filenames with the filenames only of selected items.
    fn get_filenames<A: ArrayStringMethods>(&self, filenames: &A) {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileCtrl_GetFilenames(self.as_ptr(), filenames)
        }
    }
    /// Returns the zero-based index of the currently selected filter.
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileCtrl_GetFilterIndex(self.as_ptr()) }
    }
    /// Returns the full path (directory and filename) of the currently selected file.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetPath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the full paths of the files chosen.
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxFileCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Returns the current wildcard.
    fn get_wildcard(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetWildcard(self.as_ptr())).into() }
    }
    /// Sets(changes) the current directory displayed in the control.
    fn set_directory(&self, directory: &str) -> bool {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileCtrl_SetDirectory(self.as_ptr(), directory)
        }
    }
    /// Selects a certain file.
    fn set_filename(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFileCtrl_SetFilename(self.as_ptr(), filename)
        }
    }
    /// Changes to a certain directory and selects a certain file.
    fn set_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileCtrl_SetPath(self.as_ptr(), path)
        }
    }
    /// Sets the current filter index, starting from zero.
    fn set_filter_index(&self, filter_index: c_int) {
        unsafe { ffi::wxFileCtrl_SetFilterIndex(self.as_ptr(), filter_index) }
    }
    /// Sets the wildcard, which can contain multiple file types, for example: "BMP files (*.bmp)|*.bmp|GIF files (*.gif)|*.gif".
    fn set_wildcard(&self, wild_card: &str) {
        unsafe {
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            ffi::wxFileCtrl_SetWildcard(self.as_ptr(), wild_card)
        }
    }
    /// Sets whether hidden files and folders are shown or not.
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxFileCtrl_ShowHidden(self.as_ptr(), show) }
    }
}

// wxFileCtrlEvent
pub trait FileCtrlEventMethods: CommandEventMethods {
    /// Returns the current directory.
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrlEvent_GetDirectory(self.as_ptr())).into() }
    }
    /// Returns the file selected (assuming it is only one file).
    fn get_file(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrlEvent_GetFile(self.as_ptr())).into() }
    }
    /// Returns the files selected.
    fn get_files(&self) -> ArrayString {
        unsafe { ArrayString::from_ptr(ffi::wxFileCtrlEvent_GetFiles(self.as_ptr())) }
    }
    /// Returns the current file filter index.
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileCtrlEvent_GetFilterIndex(self.as_ptr()) }
    }
    /// Sets the files changed by this event.
    fn set_files<A: ArrayStringMethods>(&self, files: &A) {
        unsafe {
            let files = files.as_ptr();
            ffi::wxFileCtrlEvent_SetFiles(self.as_ptr(), files)
        }
    }
    /// Sets the directory of this event.
    fn set_directory(&self, directory: &str) {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileCtrlEvent_SetDirectory(self.as_ptr(), directory)
        }
    }
    /// Sets the filter index changed by this event.
    fn set_filter_index(&self, index: c_int) {
        unsafe { ffi::wxFileCtrlEvent_SetFilterIndex(self.as_ptr(), index) }
    }
}

// wxFileDataObject
pub trait FileDataObjectMethods: DataObjectSimpleMethods {
    /// Adds a file to the file list represented by this data object (Windows only).
    fn add_file(&self, file: &str) {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileDataObject_AddFile(self.as_ptr(), file)
        }
    }
    /// Returns the array of file names.
    fn get_filenames(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxFileDataObject_GetFilenames(self.as_ptr())) }
    }
}

// wxFileDialog
pub trait FileDialogMethods: DialogMethods {
    // DTOR: fn ~wxFileDialog()
    /// Returns the path of the file currently selected in dialog.
    fn get_currently_selected_filename(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileDialog_GetCurrentlySelectedFilename(
                self.as_ptr(),
            ))
            .into()
        }
    }
    /// Returns the file type filter index currently selected in dialog.
    fn get_currently_selected_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileDialog_GetCurrentlySelectedFilterIndex(self.as_ptr()) }
    }
    /// Returns the default directory.
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetDirectory(self.as_ptr())).into() }
    }
    /// If functions SetExtraControlCreator() and ShowModal() were called, returns the extra window.
    fn get_extra_control(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxFileDialog_GetExtraControl(self.as_ptr())) }
    }
    /// Returns the default filename.
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetFilename(self.as_ptr())).into() }
    }
    /// Fills the array filenames with the names of the files chosen.
    fn get_filenames<A: ArrayStringMethods>(&self, filenames: &A) {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileDialog_GetFilenames(self.as_ptr(), filenames)
        }
    }
    /// Returns the index into the list of filters supplied, optionally, in the wildcard parameter.
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileDialog_GetFilterIndex(self.as_ptr()) }
    }
    /// Returns the message that will be displayed on the dialog.
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetMessage(self.as_ptr())).into() }
    }
    /// Returns the full path (directory and filename) of the selected file.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetPath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the full paths of the files chosen.
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxFileDialog_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Returns the file dialog wildcard.
    fn get_wildcard(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetWildcard(self.as_ptr())).into() }
    }
    /// Set the hook to be used for customizing the dialog contents.
    fn set_customize_hook(&self, customize_hook: *mut c_void) -> bool {
        unsafe { ffi::wxFileDialog_SetCustomizeHook(self.as_ptr(), customize_hook) }
    }
    /// Sets the default directory.
    fn set_directory(&self, directory: &str) {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileDialog_SetDirectory(self.as_ptr(), directory)
        }
    }
    // NOT_SUPPORTED: fn SetExtraControlCreator()
    /// Sets the default filename.
    fn set_filename(&self, setfilename: &str) {
        unsafe {
            let setfilename = WxString::from(setfilename);
            let setfilename = setfilename.as_ptr();
            ffi::wxFileDialog_SetFilename(self.as_ptr(), setfilename)
        }
    }
    /// Sets the default filter index, starting from zero.
    fn set_filter_index(&self, filter_index: c_int) {
        unsafe { ffi::wxFileDialog_SetFilterIndex(self.as_ptr(), filter_index) }
    }
    /// Sets the message that will be displayed on the dialog.
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxFileDialog_SetMessage(self.as_ptr(), message)
        }
    }
    /// Sets the path (the combined directory and filename that will be returned when the dialog is dismissed).
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileDialog_SetPath(self.as_ptr(), path)
        }
    }
    /// Sets the wildcard, which can contain multiple file types, for example: "BMP files (*.bmp)|*.bmp|GIF files (*.gif)|*.gif".
    fn set_wildcard(&self, wild_card: &str) {
        unsafe {
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            ffi::wxFileDialog_SetWildcard(self.as_ptr(), wild_card)
        }
    }
}

// wxFileDirPickerEvent
pub trait FileDirPickerEventMethods: CommandEventMethods {
    /// Retrieve the absolute path of the file/directory the user has just selected.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDirPickerEvent_GetPath(self.as_ptr())).into() }
    }
    /// Set the absolute path of the file/directory associated with the event.
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileDirPickerEvent_SetPath(self.as_ptr(), path)
        }
    }
}

// wxFileDropTarget
pub trait FileDropTargetMethods: DropTargetMethods {
    /// Override this function to receive dropped files.
    fn on_drop_files<A: ArrayStringMethods>(&self, x: c_int, y: c_int, filenames: &A) -> bool {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileDropTarget_OnDropFiles(self.as_ptr(), x, y, filenames)
        }
    }
}

// wxFileHistory
pub trait FileHistoryMethods: ObjectMethods {
    // DTOR: fn ~wxFileHistory()
    /// Adds a file to the file history list, if the object has a pointer to an appropriate file menu.
    fn add_file_to_history(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFileHistory_AddFileToHistory(self.as_ptr(), filename)
        }
    }
    /// Appends the files in the history list, to all menus managed by the file history object.
    fn add_files_to_menu(&self) {
        unsafe { ffi::wxFileHistory_AddFilesToMenu(self.as_ptr()) }
    }
    /// Appends the files in the history list, to the given menu only.
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
    fn get_base_id(&self) -> c_int {
        unsafe { ffi::wxFileHistory_GetBaseId(self.as_ptr()) }
    }
    /// Returns the number of files currently stored in the file history.
    fn get_count(&self) -> usize {
        unsafe { ffi::wxFileHistory_GetCount(self.as_ptr()) }
    }
    /// Returns the file at this index (zero-based).
    fn get_history_file(&self, index: usize) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileHistory_GetHistoryFile(self.as_ptr(), index)).into()
        }
    }
    /// Returns the maximum number of files that can be stored.
    fn get_max_files(&self) -> c_int {
        unsafe { ffi::wxFileHistory_GetMaxFiles(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenus()
    /// Loads the file history from the given config object.
    fn load(&self, config: *const c_void) {
        unsafe { ffi::wxFileHistory_Load(self.as_ptr(), config) }
    }
    /// Removes the specified file from the history.
    fn remove_file_from_history(&self, i: usize) {
        unsafe { ffi::wxFileHistory_RemoveFileFromHistory(self.as_ptr(), i) }
    }
    /// Removes this menu from the list of those managed by this object.
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
    fn save(&self, config: *mut c_void) {
        unsafe { ffi::wxFileHistory_Save(self.as_ptr(), config) }
    }
    /// Sets the base identifier for the range used for appending items.
    fn set_base_id(&self, base_id: c_int) {
        unsafe { ffi::wxFileHistory_SetBaseId(self.as_ptr(), base_id) }
    }
    /// Adds this menu to the list of those menus that are managed by this file history object.
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
pub trait FilePickerCtrlMethods: PickerBaseMethods {
    /// Creates this widget with the given parameters.
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
    fn get_file_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFilePickerCtrl_GetFileName(self.as_ptr())) }
    }
    /// Returns the absolute path of the currently selected file.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFilePickerCtrl_GetPath(self.as_ptr())).into() }
    }
    /// This method does the same thing as SetPath() but takes a wxFileName object instead of a string.
    fn set_file_name<F: FileNameMethods>(&self, filename: &F) {
        unsafe {
            let filename = filename.as_ptr();
            ffi::wxFilePickerCtrl_SetFileName(self.as_ptr(), filename)
        }
    }
    /// Set the directory to show when starting to browse for files.
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFilePickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    /// Sets the absolute path of the currently selected file.
    fn set_path(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFilePickerCtrl_SetPath(self.as_ptr(), filename)
        }
    }
}

// wxFindDialogEvent
pub trait FindDialogEventMethods: CommandEventMethods {
    /// Return the pointer to the dialog which generated this event.
    fn get_dialog(&self) -> WeakRef<FindReplaceDialog> {
        unsafe {
            WeakRef::<FindReplaceDialog>::from(ffi::wxFindDialogEvent_GetDialog(self.as_ptr()))
        }
    }
    /// Return the string to find (never empty).
    fn get_find_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindDialogEvent_GetFindString(self.as_ptr())).into() }
    }
    /// Get the currently selected flags: this is the combination of the wxFindReplaceFlags enumeration values.
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFindDialogEvent_GetFlags(self.as_ptr()) }
    }
    /// Return the string to replace the search string with (only for replace and replace all events).
    fn get_replace_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindDialogEvent_GetReplaceString(self.as_ptr())).into() }
    }
}

// wxFindReplaceData
pub trait FindReplaceDataMethods: ObjectMethods {
    /// Get the string to find.
    fn get_find_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindReplaceData_GetFindString(self.as_ptr())).into() }
    }
    /// Get the combination of wxFindReplaceFlags values.
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFindReplaceData_GetFlags(self.as_ptr()) }
    }
    /// Get the replacement string.
    fn get_replace_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindReplaceData_GetReplaceString(self.as_ptr())).into() }
    }
    /// Set the string to find (used as initial value by the dialog).
    fn set_find_string(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxFindReplaceData_SetFindString(self.as_ptr(), str)
        }
    }
    // NOT_SUPPORTED: fn SetFlags()
    /// Set the replacement string (used as initial value by the dialog).
    fn set_replace_string(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxFindReplaceData_SetReplaceString(self.as_ptr(), str)
        }
    }
}

// wxFindReplaceDialog
pub trait FindReplaceDialogMethods: DialogMethods {
    // DTOR: fn ~wxFindReplaceDialog()
    /// Creates the dialog; use wxWindow::Show to show it on screen.
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
    fn get_data(&self) -> Option<FindReplaceDataIsOwned<false>> {
        unsafe { FindReplaceData::option_from(ffi::wxFindReplaceDialog_GetData(self.as_ptr())) }
    }
}

// wxFlexGridSizer
pub trait FlexGridSizerMethods: GridSizerMethods {
    /// Specifies that column idx (starting from zero) should be grown if there is extra space available to the sizer.
    fn add_growable_col(&self, idx: usize, proportion: c_int) {
        unsafe { ffi::wxFlexGridSizer_AddGrowableCol(self.as_ptr(), idx, proportion) }
    }
    /// Specifies that row idx (starting from zero) should be grown if there is extra space available to the sizer.
    fn add_growable_row(&self, idx: usize, proportion: c_int) {
        unsafe { ffi::wxFlexGridSizer_AddGrowableRow(self.as_ptr(), idx, proportion) }
    }
    /// Returns a wxOrientation value that specifies whether the sizer flexibly resizes its columns, rows, or both (default).
    fn get_flexible_direction(&self) -> c_int {
        unsafe { ffi::wxFlexGridSizer_GetFlexibleDirection(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetNonFlexibleGrowMode()
    /// Returns true if column idx is growable.
    fn is_col_growable(&self, idx: usize) -> bool {
        unsafe { ffi::wxFlexGridSizer_IsColGrowable(self.as_ptr(), idx) }
    }
    /// Returns true if row idx is growable.
    fn is_row_growable(&self, idx: usize) -> bool {
        unsafe { ffi::wxFlexGridSizer_IsRowGrowable(self.as_ptr(), idx) }
    }
    /// Specifies that the idx column index is no longer growable.
    fn remove_growable_col(&self, idx: usize) {
        unsafe { ffi::wxFlexGridSizer_RemoveGrowableCol(self.as_ptr(), idx) }
    }
    /// Specifies that the idx row index is no longer growable.
    fn remove_growable_row(&self, idx: usize) {
        unsafe { ffi::wxFlexGridSizer_RemoveGrowableRow(self.as_ptr(), idx) }
    }
    /// Specifies whether the sizer should flexibly resize its columns, rows, or both.
    fn set_flexible_direction(&self, direction: c_int) {
        unsafe { ffi::wxFlexGridSizer_SetFlexibleDirection(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn SetNonFlexibleGrowMode()
    /// Returns a read-only array containing the heights of the rows in the sizer.
    fn get_row_heights(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxFlexGridSizer_GetRowHeights(self.as_ptr())) }
    }
    /// Returns a read-only array containing the widths of the columns in the sizer.
    fn get_col_widths(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxFlexGridSizer_GetColWidths(self.as_ptr())) }
    }
}

// wxFocusEvent
pub trait FocusEventMethods: EventMethods {
    /// Returns the window associated with this event, that is the window which had the focus before for the wxEVT_SET_FOCUS event and the window which is going to receive focus for the wxEVT_KILL_FOCUS one.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxFocusEvent_GetWindow(self.as_ptr())) }
    }
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
pub trait FontMethods: GDIObjectMethods {
    /// Returns a font with the same face/size as the given one but with normal weight and style and not underlined nor stricken through.
    fn get_base_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_GetBaseFont(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEncoding()
    /// Returns the face name associated with the font, or the empty string if there is no face information.
    fn get_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFamily()
    /// Returns the platform-dependent string completely describing this font.
    fn get_native_font_info_desc(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetNativeFontInfoDesc(self.as_ptr())).into() }
    }
    /// Returns a user-friendly string for this font object.
    fn get_native_font_info_user_desc(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetNativeFontInfoUserDesc(self.as_ptr())).into() }
    }
    /// Returns a font with the same face/size as the given one but with normal weight and style and not underlined nor stricken through.
    fn get_native_font_info(&self) -> Option<NativeFontInfoIsOwned<false>> {
        unsafe { NativeFontInfo::option_from(ffi::wxFont_GetNativeFontInfo(self.as_ptr())) }
    }
    /// Gets the point size as an integer number.
    fn get_point_size(&self) -> c_int {
        unsafe { ffi::wxFont_GetPointSize(self.as_ptr()) }
    }
    /// Gets the point size as a floating number.
    fn get_fractional_point_size(&self) -> c_double {
        unsafe { ffi::wxFont_GetFractionalPointSize(self.as_ptr()) }
    }
    /// Gets the pixel size.
    fn get_pixel_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxFont_GetPixelSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    /// Returns true if the font is underlined, false otherwise.
    fn get_underlined(&self) -> bool {
        unsafe { ffi::wxFont_GetUnderlined(self.as_ptr()) }
    }
    /// Returns true if the font is stricken-through, false otherwise.
    fn get_strikethrough(&self) -> bool {
        unsafe { ffi::wxFont_GetStrikethrough(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWeight()
    /// Gets the font weight as an integer value.
    fn get_numeric_weight(&self) -> c_int {
        unsafe { ffi::wxFont_GetNumericWeight(self.as_ptr()) }
    }
    /// Returns true if the font is a fixed width (or monospaced) font, false if it is a proportional one or font is invalid.
    fn is_fixed_width(&self) -> bool {
        unsafe { ffi::wxFont_IsFixedWidth(self.as_ptr()) }
    }
    /// Returns true if this object is a valid font, false otherwise.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFont_IsOk(self.as_ptr()) }
    }
    /// Specify the name of a file containing a TrueType font to be made available to the current application.
    fn add_private_font(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFont_AddPrivateFont(filename)
        }
    }
    /// Returns a bold version of this font.
    fn bold(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Bold(self.as_ptr())) }
    }
    /// Returns an italic version of this font.
    fn italic(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Italic(self.as_ptr())) }
    }
    /// Returns a larger version of this font.
    fn larger(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Larger(self.as_ptr())) }
    }
    /// Returns a smaller version of this font.
    fn smaller(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Smaller(self.as_ptr())) }
    }
    /// Returns underlined version of this font.
    fn underlined(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Underlined(self.as_ptr())) }
    }
    /// Returns stricken-through version of this font.
    fn strikethrough(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Strikethrough(self.as_ptr())) }
    }
    /// Changes this font to be bold.
    fn make_bold(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeBold(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be italic.
    fn make_italic(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeItalic(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be larger.
    fn make_larger(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeLarger(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be smaller.
    fn make_smaller(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeSmaller(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be underlined.
    fn make_underlined(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeUnderlined(self.as_ptr());
            &self
        }
    }
    /// Changes this font to be stricken-through.
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
    fn set_face_name(&self, face_name: &str) -> bool {
        unsafe {
            let face_name = WxString::from(face_name);
            let face_name = face_name.as_ptr();
            ffi::wxFont_SetFaceName(self.as_ptr(), face_name)
        }
    }
    // NOT_SUPPORTED: fn SetFamily()
    /// Creates the font corresponding to the given native font description string which must have been previously returned by GetNativeFontInfoDesc().
    fn set_native_font_info_str(&self, info: &str) -> bool {
        unsafe {
            let info = WxString::from(info);
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfo(self.as_ptr(), info)
        }
    }
    /// Creates the font corresponding to the given native font description string and returns true if the creation was successful.
    fn set_native_font_info_user_desc(&self, info: &str) -> bool {
        unsafe {
            let info = WxString::from(info);
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfoUserDesc(self.as_ptr(), info)
        }
    }
    /// Sets the encoding for this font.
    fn set_native_font_info_nativefontinfo<N: NativeFontInfoMethods>(&self, info: &N) {
        unsafe {
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfo1(self.as_ptr(), info)
        }
    }
    /// Sets the font size in points to an integer value.
    fn set_point_size(&self, point_size: c_int) {
        unsafe { ffi::wxFont_SetPointSize(self.as_ptr(), point_size) }
    }
    /// Sets the font size in points.
    fn set_fractional_point_size(&self, point_size: c_double) {
        unsafe { ffi::wxFont_SetFractionalPointSize(self.as_ptr(), point_size) }
    }
    /// Sets the pixel size.
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
    fn set_underlined(&self, underlined: bool) {
        unsafe { ffi::wxFont_SetUnderlined(self.as_ptr(), underlined) }
    }
    /// Sets strike-through attribute of the font.
    fn set_strikethrough(&self, strikethrough: bool) {
        unsafe { ffi::wxFont_SetStrikethrough(self.as_ptr(), strikethrough) }
    }
    // NOT_SUPPORTED: fn SetWeight()
    /// Sets the font weight using an integer value.
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
    fn new_nativefontinfo<N: NativeFontInfoMethods>(native_info: &N) -> Option<FontIsOwned<false>> {
        unsafe {
            let native_info = native_info.as_ptr();
            Font::option_from(ffi::wxFont_New4(native_info))
        }
    }
    /// Sets the encoding for this font.
    fn new_str(native_info_string: &str) -> Option<FontIsOwned<false>> {
        unsafe {
            let native_info_string = WxString::from(native_info_string);
            let native_info_string = native_info_string.as_ptr();
            Font::option_from(ffi::wxFont_New5(native_info_string))
        }
    }
    // DTOR: fn ~wxFont()
}

// wxFontData
pub trait FontDataMethods: ObjectMethods {
    /// Enables or disables "effects" under Windows or generic only.
    fn enable_effects(&self, enable: bool) {
        unsafe { ffi::wxFontData_EnableEffects(self.as_ptr(), enable) }
    }
    /// Under Windows, returns a flag determining whether symbol fonts can be selected.
    fn get_allow_symbols(&self) -> bool {
        unsafe { ffi::wxFontData_GetAllowSymbols(self.as_ptr()) }
    }
    /// Gets the font chosen by the user if the user pressed OK (wxFontDialog::ShowModal() returned wxID_OK).
    fn get_chosen_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontData_GetChosenFont(self.as_ptr())) }
    }
    /// Gets the colour associated with the font dialog.
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxFontData_GetColour(self.as_ptr())) }
    }
    /// Determines whether "effects" are enabled under Windows.
    fn get_enable_effects(&self) -> bool {
        unsafe { ffi::wxFontData_GetEnableEffects(self.as_ptr()) }
    }
    /// Returns the state of the flags restricting the selection.
    fn get_restrict_selection(&self) -> c_int {
        unsafe { ffi::wxFontData_GetRestrictSelection(self.as_ptr()) }
    }
    /// Gets the font that will be initially used by the font dialog.
    fn get_initial_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontData_GetInitialFont(self.as_ptr())) }
    }
    /// Returns true if the Help button will be shown (Windows only).
    fn get_show_help(&self) -> bool {
        unsafe { ffi::wxFontData_GetShowHelp(self.as_ptr()) }
    }
    /// Restricts the selection to a subset of the available fonts.
    fn restrict_selection(&self, flags: c_int) {
        unsafe { ffi::wxFontData_RestrictSelection(self.as_ptr(), flags) }
    }
    /// Under Windows, determines whether symbol fonts can be selected.
    fn set_allow_symbols(&self, allow_symbols: bool) {
        unsafe { ffi::wxFontData_SetAllowSymbols(self.as_ptr(), allow_symbols) }
    }
    /// Sets the font that will be returned to the user (for internal use only).
    fn set_chosen_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontData_SetChosenFont(self.as_ptr(), font)
        }
    }
    /// Sets the colour that will be used for the font foreground colour.
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxFontData_SetColour(self.as_ptr(), colour)
        }
    }
    /// Sets the font that will be initially used by the font dialog.
    fn set_initial_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontData_SetInitialFont(self.as_ptr(), font)
        }
    }
    /// Sets the valid range for the font point size (Windows only).
    fn set_range(&self, min: c_int, max: c_int) {
        unsafe { ffi::wxFontData_SetRange(self.as_ptr(), min, max) }
    }
    /// Determines whether the Help button will be displayed in the font dialog (Windows only).
    fn set_show_help(&self, show_help: bool) {
        unsafe { ffi::wxFontData_SetShowHelp(self.as_ptr(), show_help) }
    }
    // BLOCKED: fn operator=()
}

// wxFontDialog
pub trait FontDialogMethods: DialogMethods {
    /// Creates the dialog if the wxFontDialog object had been initialized using the default constructor.
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
    fn get_font_data(&self) -> FontDataIsOwned<false> {
        unsafe { FontDataIsOwned::from_ptr(ffi::wxFontDialog_GetFontData1(self.as_ptr())) }
    }
}

// wxFontEnumerator
pub trait FontEnumeratorMethods: WxRustMethods {
    // DTOR: fn ~wxFontEnumerator()
    /// Call OnFontEncoding() for each encoding supported by the given font - or for each encoding supported by at least some font if font is not specified.
    fn enumerate_encodings(&self, font: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxFontEnumerator_EnumerateEncodings(self.as_ptr(), font)
        }
    }
    // NOT_SUPPORTED: fn EnumerateFacenames()
    /// Called by EnumerateFacenames() for each match.
    fn on_facename(&self, font: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxFontEnumerator_OnFacename(self.as_ptr(), font)
        }
    }
    /// Called by EnumerateEncodings() for each match.
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
    fn get_encodings(facename: &str) -> ArrayString {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ArrayString::from_ptr(ffi::wxFontEnumerator_GetEncodings(facename))
        }
    }
    // NOT_SUPPORTED: fn GetFacenames()
    /// Returns true if the given string is valid face name, i.e.
    fn is_valid_facename(facename: &str) -> bool {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ffi::wxFontEnumerator_IsValidFacename(facename)
        }
    }
    /// Invalidate cache used by some of the methods of this class internally.
    fn invalidate_cache() {
        unsafe { ffi::wxFontEnumerator_InvalidateCache() }
    }
}

// wxFontList
pub trait FontListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreateFont()
    /// Finds a font of the given specification, or creates one and adds it to the list.
    fn find_or_create_font(&self, font_info: *const c_void) -> Option<FontIsOwned<false>> {
        unsafe { Font::option_from(ffi::wxFontList_FindOrCreateFont1(self.as_ptr(), font_info)) }
    }
}

// wxFontMapper
pub trait FontMapperMethods: WxRustMethods {
    // DTOR: fn ~wxFontMapper()
    // NOT_SUPPORTED: fn CharsetToEncoding()
    // NOT_SUPPORTED: fn GetAltForEncoding()
    // NOT_SUPPORTED: fn GetAltForEncoding1()
    // NOT_SUPPORTED: fn IsEncodingAvailable()
    /// Set the root config path to use (should be an absolute path).
    fn set_config_path(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFontMapper_SetConfigPath(self.as_ptr(), prefix)
        }
    }
    /// The parent window for modal dialogs.
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
    fn set_dialog_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxFontMapper_SetDialogTitle(self.as_ptr(), title)
        }
    }
    /// Get the current font mapper object.
    fn get() -> Option<FontMapperIsOwned<false>> {
        unsafe { FontMapper::option_from(ffi::wxFontMapper_Get()) }
    }
    // NOT_SUPPORTED: fn GetAllEncodingNames()
    // NOT_SUPPORTED: fn GetEncoding()
    // NOT_SUPPORTED: fn GetEncodingDescription()
    // NOT_SUPPORTED: fn GetEncodingFromName()
    // NOT_SUPPORTED: fn GetEncodingName()
    /// Returns the number of the font encodings supported by this class.
    fn get_supported_encodings_count() -> usize {
        unsafe { ffi::wxFontMapper_GetSupportedEncodingsCount() }
    }
    /// Set the current font mapper object and return previous one (may be NULL).
    fn set<F: FontMapperMethods>(mapper: Option<&F>) -> Option<FontMapperIsOwned<false>> {
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
pub trait FontPickerCtrlMethods: PickerBaseMethods {
    /// Creates this widget with given parameters.
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
    fn get_max_point_size(&self) -> c_uint {
        unsafe { ffi::wxFontPickerCtrl_GetMaxPointSize(self.as_ptr()) }
    }
    /// Returns the minimum point size value allowed for the user-chosen font.
    fn get_min_point_size(&self) -> c_uint {
        unsafe { ffi::wxFontPickerCtrl_GetMinPointSize(self.as_ptr()) }
    }
    /// Returns the currently selected colour.
    fn get_selected_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxFontPickerCtrl_GetSelectedColour(self.as_ptr())) }
    }
    /// Returns the currently selected font.
    fn get_selected_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontPickerCtrl_GetSelectedFont(self.as_ptr())) }
    }
    /// Sets the maximum point size value allowed for the user-chosen font.
    fn set_max_point_size(&self, max: c_uint) {
        unsafe { ffi::wxFontPickerCtrl_SetMaxPointSize(self.as_ptr(), max) }
    }
    /// Sets the minimum point size value allowed for the user-chosen font.
    fn set_min_point_size(&self, min: c_uint) {
        unsafe { ffi::wxFontPickerCtrl_SetMinPointSize(self.as_ptr(), min) }
    }
    /// Sets the font colour.
    fn set_selected_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxFontPickerCtrl_SetSelectedColour(self.as_ptr(), colour)
        }
    }
    /// Sets the currently selected font.
    fn set_selected_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontPickerCtrl_SetSelectedFont(self.as_ptr(), font)
        }
    }
}

// wxFontPickerEvent
pub trait FontPickerEventMethods: CommandEventMethods {
    /// Retrieve the font the user has just selected.
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontPickerEvent_GetFont(self.as_ptr())) }
    }
    /// Set the font associated with the event.
    fn set_font<F: FontMethods>(&self, f: &F) {
        unsafe {
            let f = f.as_ptr();
            ffi::wxFontPickerEvent_SetFont(self.as_ptr(), f)
        }
    }
}

// wxFrame
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    /// Creates a status bar at the bottom of the frame.
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
    fn create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<ToolBar>::from(ffi::wxFrame_CreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    /// Method used to show help string of the selected menu toolbar item.
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_DoGiveHelp(self.as_ptr(), text, show)
        }
    }
    /// Returns a pointer to the menubar currently associated with the frame (if any).
    fn get_menu_bar(&self) -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxFrame_GetMenuBar(self.as_ptr())) }
    }
    /// Returns a pointer to the status bar currently associated with the frame (if any).
    fn get_status_bar(&self) -> WeakRef<StatusBar> {
        unsafe { WeakRef::<StatusBar>::from(ffi::wxFrame_GetStatusBar(self.as_ptr())) }
    }
    /// Returns the status bar pane used to display menu and toolbar help.
    fn get_status_bar_pane(&self) -> c_int {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr()) }
    }
    /// Returns a pointer to the toolbar currently associated with the frame (if any).
    fn get_tool_bar(&self) -> WeakRef<ToolBar> {
        unsafe { WeakRef::<ToolBar>::from(ffi::wxFrame_GetToolBar(self.as_ptr())) }
    }
    /// Virtual function called when a status bar is requested by CreateStatusBar().
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
    fn on_create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<ToolBar>::from(ffi::wxFrame_OnCreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    /// Simulate a menu command.
    fn process_command(&self, id: c_int) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr(), id) }
    }
    /// Tells the frame to show the given menu bar.
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
    fn set_status_bar_pane(&self, n: c_int) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr(), n) }
    }
    /// Sets the status bar text and updates the status bar display.
    fn set_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_SetStatusText(self.as_ptr(), text, number)
        }
    }
    /// Sets the widths of the fields in the status bar.
    fn set_status_widths(&self, n: c_int, widths_field: *const c_void) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr(), n, widths_field) }
    }
    /// Associates a toolbar with the frame.
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
    fn msw_get_task_bar_button(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_MSWGetTaskBarButton(self.as_ptr()) }
    }
    fn push_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_PushStatusText(self.as_ptr(), text, number)
        }
    }
    fn pop_status_text(&self, number: c_int) {
        unsafe { ffi::wxFrame_PopStatusText(self.as_ptr(), number) }
    }
}
