use super::*;

// wxFileCtrl
pub trait FileCtrlMethods: ControlMethods {
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
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetDirectory(self.as_ptr())).into() }
    }
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetFilename(self.as_ptr())).into() }
    }
    fn get_filenames<A: ArrayStringMethods>(&self, filenames: &A) {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileCtrl_GetFilenames(self.as_ptr(), filenames)
        }
    }
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileCtrl_GetFilterIndex(self.as_ptr()) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetPath(self.as_ptr())).into() }
    }
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxFileCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    fn get_wildcard(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrl_GetWildcard(self.as_ptr())).into() }
    }
    fn set_directory(&self, directory: &str) -> bool {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileCtrl_SetDirectory(self.as_ptr(), directory)
        }
    }
    fn set_filename(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFileCtrl_SetFilename(self.as_ptr(), filename)
        }
    }
    fn set_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileCtrl_SetPath(self.as_ptr(), path)
        }
    }
    fn set_filter_index(&self, filter_index: c_int) {
        unsafe { ffi::wxFileCtrl_SetFilterIndex(self.as_ptr(), filter_index) }
    }
    fn set_wildcard(&self, wild_card: &str) {
        unsafe {
            let wild_card = WxString::from(wild_card);
            let wild_card = wild_card.as_ptr();
            ffi::wxFileCtrl_SetWildcard(self.as_ptr(), wild_card)
        }
    }
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxFileCtrl_ShowHidden(self.as_ptr(), show) }
    }
}

// wxFileCtrlEvent
pub trait FileCtrlEventMethods: CommandEventMethods {
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrlEvent_GetDirectory(self.as_ptr())).into() }
    }
    fn get_file(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileCtrlEvent_GetFile(self.as_ptr())).into() }
    }
    fn get_files(&self) -> ArrayString {
        unsafe { ArrayString::from_ptr(ffi::wxFileCtrlEvent_GetFiles(self.as_ptr())) }
    }
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileCtrlEvent_GetFilterIndex(self.as_ptr()) }
    }
    fn set_files<A: ArrayStringMethods>(&self, files: &A) {
        unsafe {
            let files = files.as_ptr();
            ffi::wxFileCtrlEvent_SetFiles(self.as_ptr(), files)
        }
    }
    fn set_directory(&self, directory: &str) {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileCtrlEvent_SetDirectory(self.as_ptr(), directory)
        }
    }
    fn set_filter_index(&self, index: c_int) {
        unsafe { ffi::wxFileCtrlEvent_SetFilterIndex(self.as_ptr(), index) }
    }
}

// wxFileDataObject
pub trait FileDataObjectMethods: DataObjectSimpleMethods {
    fn add_file(&self, file: &str) {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileDataObject_AddFile(self.as_ptr(), file)
        }
    }
    fn get_filenames(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxFileDataObject_GetFilenames(self.as_ptr())) }
    }
}

// wxFileDialog
pub trait FileDialogMethods: DialogMethods {
    // DTOR: fn ~wxFileDialog()
    fn get_currently_selected_filename(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileDialog_GetCurrentlySelectedFilename(
                self.as_ptr(),
            ))
            .into()
        }
    }
    fn get_currently_selected_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileDialog_GetCurrentlySelectedFilterIndex(self.as_ptr()) }
    }
    fn get_directory(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetDirectory(self.as_ptr())).into() }
    }
    fn get_extra_control(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxFileDialog_GetExtraControl(self.as_ptr())) }
    }
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetFilename(self.as_ptr())).into() }
    }
    fn get_filenames<A: ArrayStringMethods>(&self, filenames: &A) {
        unsafe {
            let filenames = filenames.as_ptr();
            ffi::wxFileDialog_GetFilenames(self.as_ptr(), filenames)
        }
    }
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxFileDialog_GetFilterIndex(self.as_ptr()) }
    }
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetMessage(self.as_ptr())).into() }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetPath(self.as_ptr())).into() }
    }
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxFileDialog_GetPaths(self.as_ptr(), paths)
        }
    }
    fn get_wildcard(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDialog_GetWildcard(self.as_ptr())).into() }
    }
    fn set_customize_hook(&self, customize_hook: *mut c_void) -> bool {
        unsafe { ffi::wxFileDialog_SetCustomizeHook(self.as_ptr(), customize_hook) }
    }
    fn set_directory(&self, directory: &str) {
        unsafe {
            let directory = WxString::from(directory);
            let directory = directory.as_ptr();
            ffi::wxFileDialog_SetDirectory(self.as_ptr(), directory)
        }
    }
    // NOT_SUPPORTED: fn SetExtraControlCreator()
    fn set_filename(&self, setfilename: &str) {
        unsafe {
            let setfilename = WxString::from(setfilename);
            let setfilename = setfilename.as_ptr();
            ffi::wxFileDialog_SetFilename(self.as_ptr(), setfilename)
        }
    }
    fn set_filter_index(&self, filter_index: c_int) {
        unsafe { ffi::wxFileDialog_SetFilterIndex(self.as_ptr(), filter_index) }
    }
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxFileDialog_SetMessage(self.as_ptr(), message)
        }
    }
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileDialog_SetPath(self.as_ptr(), path)
        }
    }
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
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileDirPickerEvent_GetPath(self.as_ptr())).into() }
    }
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
    fn add_file_to_history(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFileHistory_AddFileToHistory(self.as_ptr(), filename)
        }
    }
    fn add_files_to_menu(&self) {
        unsafe { ffi::wxFileHistory_AddFilesToMenu(self.as_ptr()) }
    }
    fn add_files_to_menu_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileHistory_AddFilesToMenu1(self.as_ptr(), menu)
        }
    }
    fn get_base_id(&self) -> c_int {
        unsafe { ffi::wxFileHistory_GetBaseId(self.as_ptr()) }
    }
    fn get_count(&self) -> usize {
        unsafe { ffi::wxFileHistory_GetCount(self.as_ptr()) }
    }
    fn get_history_file(&self, index: usize) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxFileHistory_GetHistoryFile(self.as_ptr(), index)).into()
        }
    }
    fn get_max_files(&self) -> c_int {
        unsafe { ffi::wxFileHistory_GetMaxFiles(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenus()
    fn load(&self, config: *const c_void) {
        unsafe { ffi::wxFileHistory_Load(self.as_ptr(), config) }
    }
    fn remove_file_from_history(&self, i: usize) {
        unsafe { ffi::wxFileHistory_RemoveFileFromHistory(self.as_ptr(), i) }
    }
    fn remove_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileHistory_RemoveMenu(self.as_ptr(), menu)
        }
    }
    fn save(&self, config: *mut c_void) {
        unsafe { ffi::wxFileHistory_Save(self.as_ptr(), config) }
    }
    fn set_base_id(&self, base_id: c_int) {
        unsafe { ffi::wxFileHistory_SetBaseId(self.as_ptr(), base_id) }
    }
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
    fn get_file_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFilePickerCtrl_GetFileName(self.as_ptr())) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFilePickerCtrl_GetPath(self.as_ptr())).into() }
    }
    fn set_file_name<F: FileNameMethods>(&self, filename: &F) {
        unsafe {
            let filename = filename.as_ptr();
            ffi::wxFilePickerCtrl_SetFileName(self.as_ptr(), filename)
        }
    }
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFilePickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
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
    fn get_dialog(&self) -> WeakRef<FindReplaceDialog> {
        unsafe {
            WeakRef::<FindReplaceDialog>::from(ffi::wxFindDialogEvent_GetDialog(self.as_ptr()))
        }
    }
    fn get_find_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindDialogEvent_GetFindString(self.as_ptr())).into() }
    }
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFindDialogEvent_GetFlags(self.as_ptr()) }
    }
    fn get_replace_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindDialogEvent_GetReplaceString(self.as_ptr())).into() }
    }
}

// wxFindReplaceData
pub trait FindReplaceDataMethods: ObjectMethods {
    fn get_find_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindReplaceData_GetFindString(self.as_ptr())).into() }
    }
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxFindReplaceData_GetFlags(self.as_ptr()) }
    }
    fn get_replace_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFindReplaceData_GetReplaceString(self.as_ptr())).into() }
    }
    fn set_find_string(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxFindReplaceData_SetFindString(self.as_ptr(), str)
        }
    }
    // NOT_SUPPORTED: fn SetFlags()
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
    fn get_data(&self) -> Option<FindReplaceDataIsOwned<false>> {
        unsafe { FindReplaceData::option_from(ffi::wxFindReplaceDialog_GetData(self.as_ptr())) }
    }
}

// wxFlexGridSizer
pub trait FlexGridSizerMethods: GridSizerMethods {
    fn add_growable_col(&self, idx: usize, proportion: c_int) {
        unsafe { ffi::wxFlexGridSizer_AddGrowableCol(self.as_ptr(), idx, proportion) }
    }
    fn add_growable_row(&self, idx: usize, proportion: c_int) {
        unsafe { ffi::wxFlexGridSizer_AddGrowableRow(self.as_ptr(), idx, proportion) }
    }
    fn get_flexible_direction(&self) -> c_int {
        unsafe { ffi::wxFlexGridSizer_GetFlexibleDirection(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetNonFlexibleGrowMode()
    fn is_col_growable(&self, idx: usize) -> bool {
        unsafe { ffi::wxFlexGridSizer_IsColGrowable(self.as_ptr(), idx) }
    }
    fn is_row_growable(&self, idx: usize) -> bool {
        unsafe { ffi::wxFlexGridSizer_IsRowGrowable(self.as_ptr(), idx) }
    }
    fn remove_growable_col(&self, idx: usize) {
        unsafe { ffi::wxFlexGridSizer_RemoveGrowableCol(self.as_ptr(), idx) }
    }
    fn remove_growable_row(&self, idx: usize) {
        unsafe { ffi::wxFlexGridSizer_RemoveGrowableRow(self.as_ptr(), idx) }
    }
    fn set_flexible_direction(&self, direction: c_int) {
        unsafe { ffi::wxFlexGridSizer_SetFlexibleDirection(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn SetNonFlexibleGrowMode()
    fn get_row_heights(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxFlexGridSizer_GetRowHeights(self.as_ptr())) }
    }
    fn get_col_widths(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxFlexGridSizer_GetColWidths(self.as_ptr())) }
    }
}

// wxFocusEvent
pub trait FocusEventMethods: EventMethods {
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
    fn get_base_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_GetBaseFont(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEncoding()
    fn get_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFamily()
    fn get_native_font_info_desc(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetNativeFontInfoDesc(self.as_ptr())).into() }
    }
    fn get_native_font_info_user_desc(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFont_GetNativeFontInfoUserDesc(self.as_ptr())).into() }
    }
    fn get_native_font_info(&self) -> *const c_void {
        unsafe { ffi::wxFont_GetNativeFontInfo(self.as_ptr()) }
    }
    fn get_point_size(&self) -> c_int {
        unsafe { ffi::wxFont_GetPointSize(self.as_ptr()) }
    }
    fn get_fractional_point_size(&self) -> c_double {
        unsafe { ffi::wxFont_GetFractionalPointSize(self.as_ptr()) }
    }
    fn get_pixel_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxFont_GetPixelSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    fn get_underlined(&self) -> bool {
        unsafe { ffi::wxFont_GetUnderlined(self.as_ptr()) }
    }
    fn get_strikethrough(&self) -> bool {
        unsafe { ffi::wxFont_GetStrikethrough(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWeight()
    fn get_numeric_weight(&self) -> c_int {
        unsafe { ffi::wxFont_GetNumericWeight(self.as_ptr()) }
    }
    fn is_fixed_width(&self) -> bool {
        unsafe { ffi::wxFont_IsFixedWidth(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFont_IsOk(self.as_ptr()) }
    }
    fn add_private_font(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxFont_AddPrivateFont(filename)
        }
    }
    fn bold(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Bold(self.as_ptr())) }
    }
    fn italic(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Italic(self.as_ptr())) }
    }
    fn larger(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Larger(self.as_ptr())) }
    }
    fn smaller(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Smaller(self.as_ptr())) }
    }
    fn underlined(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Underlined(self.as_ptr())) }
    }
    fn strikethrough(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFont_Strikethrough(self.as_ptr())) }
    }
    fn make_bold(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeBold(self.as_ptr());
            &self
        }
    }
    fn make_italic(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeItalic(self.as_ptr());
            &self
        }
    }
    fn make_larger(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeLarger(self.as_ptr());
            &self
        }
    }
    fn make_smaller(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeSmaller(self.as_ptr());
            &self
        }
    }
    fn make_underlined(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeUnderlined(self.as_ptr());
            &self
        }
    }
    fn make_strikethrough(&self) -> &Self {
        unsafe {
            ffi::wxFont_MakeStrikethrough(self.as_ptr());
            &self
        }
    }
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Scaled()
    // NOT_SUPPORTED: fn SetEncoding()
    fn set_face_name(&self, face_name: &str) -> bool {
        unsafe {
            let face_name = WxString::from(face_name);
            let face_name = face_name.as_ptr();
            ffi::wxFont_SetFaceName(self.as_ptr(), face_name)
        }
    }
    // NOT_SUPPORTED: fn SetFamily()
    fn set_native_font_info_str(&self, info: &str) -> bool {
        unsafe {
            let info = WxString::from(info);
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfo(self.as_ptr(), info)
        }
    }
    fn set_native_font_info_user_desc(&self, info: &str) -> bool {
        unsafe {
            let info = WxString::from(info);
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfoUserDesc(self.as_ptr(), info)
        }
    }
    fn set_native_font_info_nativefontinfo(&self, info: *const c_void) {
        unsafe { ffi::wxFont_SetNativeFontInfo1(self.as_ptr(), info) }
    }
    fn set_point_size(&self, point_size: c_int) {
        unsafe { ffi::wxFont_SetPointSize(self.as_ptr(), point_size) }
    }
    fn set_fractional_point_size(&self, point_size: c_double) {
        unsafe { ffi::wxFont_SetFractionalPointSize(self.as_ptr(), point_size) }
    }
    fn set_pixel_size<S: SizeMethods>(&self, pixel_size: &S) {
        unsafe {
            let pixel_size = pixel_size.as_ptr();
            ffi::wxFont_SetPixelSize(self.as_ptr(), pixel_size)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    // NOT_SUPPORTED: fn SetSymbolicSize()
    // NOT_SUPPORTED: fn SetSymbolicSizeRelativeTo()
    fn set_underlined(&self, underlined: bool) {
        unsafe { ffi::wxFont_SetUnderlined(self.as_ptr(), underlined) }
    }
    fn set_strikethrough(&self, strikethrough: bool) {
        unsafe { ffi::wxFont_SetStrikethrough(self.as_ptr(), strikethrough) }
    }
    // NOT_SUPPORTED: fn SetWeight()
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
    fn new_nativefontinfo(native_info: *const c_void) -> Option<FontIsOwned<false>> {
        unsafe { Font::option_from(ffi::wxFont_New4(native_info)) }
    }
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
    fn enable_effects(&self, enable: bool) {
        unsafe { ffi::wxFontData_EnableEffects(self.as_ptr(), enable) }
    }
    fn get_allow_symbols(&self) -> bool {
        unsafe { ffi::wxFontData_GetAllowSymbols(self.as_ptr()) }
    }
    fn get_chosen_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontData_GetChosenFont(self.as_ptr())) }
    }
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxFontData_GetColour(self.as_ptr())) }
    }
    fn get_enable_effects(&self) -> bool {
        unsafe { ffi::wxFontData_GetEnableEffects(self.as_ptr()) }
    }
    fn get_restrict_selection(&self) -> c_int {
        unsafe { ffi::wxFontData_GetRestrictSelection(self.as_ptr()) }
    }
    fn get_initial_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontData_GetInitialFont(self.as_ptr())) }
    }
    fn get_show_help(&self) -> bool {
        unsafe { ffi::wxFontData_GetShowHelp(self.as_ptr()) }
    }
    fn restrict_selection(&self, flags: c_int) {
        unsafe { ffi::wxFontData_RestrictSelection(self.as_ptr(), flags) }
    }
    fn set_allow_symbols(&self, allow_symbols: bool) {
        unsafe { ffi::wxFontData_SetAllowSymbols(self.as_ptr(), allow_symbols) }
    }
    fn set_chosen_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontData_SetChosenFont(self.as_ptr(), font)
        }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxFontData_SetColour(self.as_ptr(), colour)
        }
    }
    fn set_initial_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontData_SetInitialFont(self.as_ptr(), font)
        }
    }
    fn set_range(&self, min: c_int, max: c_int) {
        unsafe { ffi::wxFontData_SetRange(self.as_ptr(), min, max) }
    }
    fn set_show_help(&self, show_help: bool) {
        unsafe { ffi::wxFontData_SetShowHelp(self.as_ptr(), show_help) }
    }
    // BLOCKED: fn operator=()
}

// wxFontDialog
pub trait FontDialogMethods: DialogMethods {
    fn create<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFontDialog_Create(self.as_ptr(), parent)
        }
    }
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
    fn enumerate_encodings(&self, font: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxFontEnumerator_EnumerateEncodings(self.as_ptr(), font)
        }
    }
    // NOT_SUPPORTED: fn EnumerateFacenames()
    fn on_facename(&self, font: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxFontEnumerator_OnFacename(self.as_ptr(), font)
        }
    }
    fn on_font_encoding(&self, font: &str, encoding: &str) -> bool {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            let encoding = WxString::from(encoding);
            let encoding = encoding.as_ptr();
            ffi::wxFontEnumerator_OnFontEncoding(self.as_ptr(), font, encoding)
        }
    }
    fn get_encodings(facename: &str) -> ArrayString {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ArrayString::from_ptr(ffi::wxFontEnumerator_GetEncodings(facename))
        }
    }
    // NOT_SUPPORTED: fn GetFacenames()
    fn is_valid_facename(facename: &str) -> bool {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ffi::wxFontEnumerator_IsValidFacename(facename)
        }
    }
    fn invalidate_cache() {
        unsafe { ffi::wxFontEnumerator_InvalidateCache() }
    }
}

// wxFontList
pub trait FontListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreateFont()
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
    fn set_config_path(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFontMapper_SetConfigPath(self.as_ptr(), prefix)
        }
    }
    fn set_dialog_parent<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFontMapper_SetDialogParent(self.as_ptr(), parent)
        }
    }
    fn set_dialog_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxFontMapper_SetDialogTitle(self.as_ptr(), title)
        }
    }
    fn get() -> Option<FontMapperIsOwned<false>> {
        unsafe { FontMapper::option_from(ffi::wxFontMapper_Get()) }
    }
    // NOT_SUPPORTED: fn GetAllEncodingNames()
    // NOT_SUPPORTED: fn GetEncoding()
    // NOT_SUPPORTED: fn GetEncodingDescription()
    // NOT_SUPPORTED: fn GetEncodingFromName()
    // NOT_SUPPORTED: fn GetEncodingName()
    fn get_supported_encodings_count() -> usize {
        unsafe { ffi::wxFontMapper_GetSupportedEncodingsCount() }
    }
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
    fn get_max_point_size(&self) -> c_uint {
        unsafe { ffi::wxFontPickerCtrl_GetMaxPointSize(self.as_ptr()) }
    }
    fn get_min_point_size(&self) -> c_uint {
        unsafe { ffi::wxFontPickerCtrl_GetMinPointSize(self.as_ptr()) }
    }
    fn get_selected_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxFontPickerCtrl_GetSelectedColour(self.as_ptr())) }
    }
    fn get_selected_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontPickerCtrl_GetSelectedFont(self.as_ptr())) }
    }
    fn set_max_point_size(&self, max: c_uint) {
        unsafe { ffi::wxFontPickerCtrl_SetMaxPointSize(self.as_ptr(), max) }
    }
    fn set_min_point_size(&self, min: c_uint) {
        unsafe { ffi::wxFontPickerCtrl_SetMinPointSize(self.as_ptr(), min) }
    }
    fn set_selected_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxFontPickerCtrl_SetSelectedColour(self.as_ptr(), colour)
        }
    }
    fn set_selected_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxFontPickerCtrl_SetSelectedFont(self.as_ptr(), font)
        }
    }
}

// wxFontPickerEvent
pub trait FontPickerEventMethods: CommandEventMethods {
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxFontPickerEvent_GetFont(self.as_ptr())) }
    }
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
    fn create_status_bar(
        &self,
        number: c_int,
        style: c_long,
        id: c_int,
        name: &str,
    ) -> *mut c_void {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFrame_CreateStatusBar(self.as_ptr(), number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<ToolBar>::from(ffi::wxFrame_CreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_DoGiveHelp(self.as_ptr(), text, show)
        }
    }
    fn get_menu_bar(&self) -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxFrame_GetMenuBar(self.as_ptr())) }
    }
    fn get_status_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetStatusBar(self.as_ptr()) }
    }
    fn get_status_bar_pane(&self) -> c_int {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr()) }
    }
    fn get_tool_bar(&self) -> WeakRef<ToolBar> {
        unsafe { WeakRef::<ToolBar>::from(ffi::wxFrame_GetToolBar(self.as_ptr())) }
    }
    fn on_create_status_bar(
        &self,
        number: c_int,
        style: c_long,
        id: c_int,
        name: &str,
    ) -> *mut c_void {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFrame_OnCreateStatusBar(self.as_ptr(), number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> WeakRef<ToolBar> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<ToolBar>::from(ffi::wxFrame_OnCreateToolBar(self.as_ptr(), style, id, name))
        }
    }
    fn process_command(&self, id: c_int) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr(), id) }
    }
    fn set_menu_bar<M: MenuBarMethods>(&self, menu_bar: Option<&M>) {
        unsafe {
            let menu_bar = match menu_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetMenuBar(self.as_ptr(), menu_bar)
        }
    }
    fn set_status_bar(&self, status_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetStatusBar(self.as_ptr(), status_bar) }
    }
    fn set_status_bar_pane(&self, n: c_int) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr(), n) }
    }
    fn set_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxFrame_SetStatusText(self.as_ptr(), text, number)
        }
    }
    fn set_status_widths(&self, n: c_int, widths_field: *const c_void) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr(), n, widths_field) }
    }
    fn set_tool_bar<T: ToolBarMethods>(&self, tool_bar: Option<&T>) {
        unsafe {
            let tool_bar = match tool_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetToolBar(self.as_ptr(), tool_bar)
        }
    }
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

// wxFullScreenEvent
pub trait FullScreenEventMethods: EventMethods {
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxFullScreenEvent_IsFullScreen(self.as_ptr()) }
    }
}
