use super::*;

extern "C" {

    // wxFileCtrl
    pub fn wxFileCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxFileCtrl_new() -> *mut c_void;
    pub fn wxFileCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        default_directory: *const c_void,
        default_filename: *const c_void,
        wild_card: *const c_void,
        style: c_long,
        pos: *const c_void,
        size: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        default_directory: *const c_void,
        default_filename: *const c_void,
        wild_card: *const c_void,
        style: c_long,
        pos: *const c_void,
        size: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxFileCtrl_GetDirectory(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_GetFilename(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_GetFilenames(self_: *const c_void, filenames: *mut c_void);
    pub fn wxFileCtrl_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxFileCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_GetPaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxFileCtrl_GetWildcard(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrl_SetDirectory(self_: *mut c_void, directory: *const c_void) -> bool;
    pub fn wxFileCtrl_SetFilename(self_: *mut c_void, filename: *const c_void) -> bool;
    pub fn wxFileCtrl_SetPath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxFileCtrl_SetFilterIndex(self_: *mut c_void, filter_index: c_int);
    pub fn wxFileCtrl_SetWildcard(self_: *mut c_void, wild_card: *const c_void);
    pub fn wxFileCtrl_ShowHidden(self_: *mut c_void, show: bool);

    // wxFileCtrlEvent
    pub fn wxFileCtrlEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileCtrlEvent_new(type_: wxEventType, evt_object: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetDirectory(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetFile(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetFiles(self_: *const c_void) -> *mut c_void;
    pub fn wxFileCtrlEvent_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxFileCtrlEvent_SetFiles(self_: *mut c_void, files: *const c_void);
    pub fn wxFileCtrlEvent_SetDirectory(self_: *mut c_void, directory: *const c_void);
    pub fn wxFileCtrlEvent_SetFilterIndex(self_: *mut c_void, index: c_int);

    // wxFileDataObject
    pub fn wxFileDataObject_delete(self_: *mut c_void);
    pub fn wxFileDataObject_new() -> *mut c_void;
    pub fn wxFileDataObject_AddFile(self_: *mut c_void, file: *const c_void);
    pub fn wxFileDataObject_GetFilenames(self_: *const c_void) -> *mut c_void;

    // wxFileDialog
    pub fn wxFileDialog_CLASSINFO() -> *mut c_void;
    pub fn wxFileDialog_new(
        parent: *mut c_void,
        message: *const c_void,
        default_dir: *const c_void,
        default_file: *const c_void,
        wildcard: *const c_void,
        style: c_long,
        pos: *const c_void,
        size: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxFileDialog_~wxFileDialog(self_: *mut c_void);
    pub fn wxFileDialog_GetCurrentlySelectedFilename(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_GetCurrentlySelectedFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxFileDialog_GetDirectory(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_GetExtraControl(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_GetFilename(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_GetFilenames(self_: *const c_void, filenames: *mut c_void);
    pub fn wxFileDialog_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxFileDialog_GetMessage(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_GetPaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxFileDialog_GetWildcard(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDialog_SetCustomizeHook(self_: *mut c_void, customize_hook: *mut c_void) -> bool;
    pub fn wxFileDialog_SetDirectory(self_: *mut c_void, directory: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileDialog_SetExtraControlCreator(self_: *mut c_void, creator: ExtraControlCreatorFunction) -> bool;
    pub fn wxFileDialog_SetFilename(self_: *mut c_void, setfilename: *const c_void);
    pub fn wxFileDialog_SetFilterIndex(self_: *mut c_void, filter_index: c_int);
    pub fn wxFileDialog_SetMessage(self_: *mut c_void, message: *const c_void);
    pub fn wxFileDialog_SetPath(self_: *mut c_void, path: *const c_void);
    pub fn wxFileDialog_SetWildcard(self_: *mut c_void, wild_card: *const c_void);

    // wxFileDirPickerEvent
    pub fn wxFileDirPickerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxFileDirPickerEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileDirPickerEvent_new1(type_: wxEventType, generator: *mut c_void, id: c_int, path: *const c_void) -> *mut c_void;
    pub fn wxFileDirPickerEvent_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileDirPickerEvent_SetPath(self_: *mut c_void, path: *const c_void);

    // wxFileDropTarget
    pub fn wxFileDropTarget_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxFileDropTarget_new() -> *mut c_void;
    pub fn wxFileDropTarget_OnDropFiles(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        filenames: *const c_void,
    ) -> bool;

    // wxFileHistory
    pub fn wxFileHistory_CLASSINFO() -> *mut c_void;
    pub fn wxFileHistory_new(max_files: usize, id_base: c_int) -> *mut c_void;
    // DTOR: pub fn wxFileHistory_~wxFileHistory(self_: *mut c_void);
    pub fn wxFileHistory_AddFileToHistory(self_: *mut c_void, filename: *const c_void);
    pub fn wxFileHistory_AddFilesToMenu(self_: *mut c_void);
    pub fn wxFileHistory_AddFilesToMenu1(self_: *mut c_void, menu: *mut c_void);
    pub fn wxFileHistory_GetBaseId(self_: *const c_void) -> c_int;
    pub fn wxFileHistory_GetCount(self_: *const c_void) -> usize;
    pub fn wxFileHistory_GetHistoryFile(self_: *const c_void, index: usize) -> *mut c_void;
    pub fn wxFileHistory_GetMaxFiles(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxFileHistory_GetMenus(self_: *const c_void) -> *const c_void;
    pub fn wxFileHistory_Load(self_: *mut c_void, config: *const c_void);
    pub fn wxFileHistory_RemoveFileFromHistory(self_: *mut c_void, i: usize);
    pub fn wxFileHistory_RemoveMenu(self_: *mut c_void, menu: *mut c_void);
    pub fn wxFileHistory_Save(self_: *mut c_void, config: *mut c_void);
    pub fn wxFileHistory_SetBaseId(self_: *mut c_void, base_id: c_int);
    pub fn wxFileHistory_UseMenu(self_: *mut c_void, menu: *mut c_void);
    // NOT_SUPPORTED: pub fn wxFileHistory_SetMenuPathStyle(self_: *mut c_void, style: wxFileHistoryMenuPathStyle);
    // NOT_SUPPORTED: pub fn wxFileHistory_GetMenuPathStyle(self_: *const c_void) -> wxFileHistoryMenuPathStyle;

    // wxFilePickerCtrl
    pub fn wxFilePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxFilePickerCtrl_new() -> *mut c_void;
    pub fn wxFilePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        wildcard: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFilePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        wildcard: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxFilePickerCtrl_GetFileName(self_: *const c_void) -> *mut c_void;
    pub fn wxFilePickerCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFilePickerCtrl_SetFileName(self_: *mut c_void, filename: *const c_void);
    pub fn wxFilePickerCtrl_SetInitialDirectory(self_: *mut c_void, dir: *const c_void);
    pub fn wxFilePickerCtrl_SetPath(self_: *mut c_void, filename: *const c_void);

    // wxFindDialogEvent
    pub fn wxFindDialogEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFindDialogEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxFindDialogEvent_GetDialog(self_: *const c_void) -> *mut c_void;
    pub fn wxFindDialogEvent_GetFindString(self_: *const c_void) -> *mut c_void;
    pub fn wxFindDialogEvent_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxFindDialogEvent_GetReplaceString(self_: *const c_void) -> *mut c_void;

    // wxFindReplaceData
    pub fn wxFindReplaceData_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFindReplaceData_new(flags: wxUint32) -> *mut c_void;
    pub fn wxFindReplaceData_GetFindString(self_: *const c_void) -> *mut c_void;
    pub fn wxFindReplaceData_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxFindReplaceData_GetReplaceString(self_: *const c_void) -> *mut c_void;
    pub fn wxFindReplaceData_SetFindString(self_: *mut c_void, str: *const c_void);
    // NOT_SUPPORTED: pub fn wxFindReplaceData_SetFlags(self_: *mut c_void, flags: wxUint32);
    pub fn wxFindReplaceData_SetReplaceString(self_: *mut c_void, str: *const c_void);

    // wxFindReplaceDialog
    pub fn wxFindReplaceDialog_CLASSINFO() -> *mut c_void;
    pub fn wxFindReplaceDialog_new() -> *mut c_void;
    pub fn wxFindReplaceDialog_new1(
        parent: *mut c_void,
        data: *mut c_void,
        title: *const c_void,
        style: c_int,
    ) -> *mut c_void;
    // DTOR: pub fn wxFindReplaceDialog_~wxFindReplaceDialog(self_: *mut c_void);
    pub fn wxFindReplaceDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        data: *mut c_void,
        title: *const c_void,
        style: c_int,
    ) -> bool;
    pub fn wxFindReplaceDialog_GetData(self_: *const c_void) -> *mut c_void;

    // wxFlexGridSizer
    pub fn wxFlexGridSizer_CLASSINFO() -> *mut c_void;
    pub fn wxFlexGridSizer_new(cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxFlexGridSizer_new1(cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxFlexGridSizer_new2(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxFlexGridSizer_new3(rows: c_int, cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxFlexGridSizer_AddGrowableCol(self_: *mut c_void, idx: usize, proportion: c_int);
    pub fn wxFlexGridSizer_AddGrowableRow(self_: *mut c_void, idx: usize, proportion: c_int);
    pub fn wxFlexGridSizer_GetFlexibleDirection(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxFlexGridSizer_GetNonFlexibleGrowMode(self_: *const c_void) -> wxFlexSizerGrowMode;
    pub fn wxFlexGridSizer_IsColGrowable(self_: *mut c_void, idx: usize) -> bool;
    pub fn wxFlexGridSizer_IsRowGrowable(self_: *mut c_void, idx: usize) -> bool;
    pub fn wxFlexGridSizer_RemoveGrowableCol(self_: *mut c_void, idx: usize);
    pub fn wxFlexGridSizer_RemoveGrowableRow(self_: *mut c_void, idx: usize);
    pub fn wxFlexGridSizer_SetFlexibleDirection(self_: *mut c_void, direction: c_int);
    // NOT_SUPPORTED: pub fn wxFlexGridSizer_SetNonFlexibleGrowMode(self_: *mut c_void, mode: wxFlexSizerGrowMode);
    pub fn wxFlexGridSizer_GetRowHeights(self_: *const c_void) -> *mut c_void;
    pub fn wxFlexGridSizer_GetColWidths(self_: *const c_void) -> *mut c_void;

    // wxFocusEvent
    pub fn wxFocusEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFocusEvent_new(event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxFocusEvent_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxFocusEvent_SetWindow(self_: *mut c_void, win: *mut c_void);

    // wxFont
    pub fn wxFont_CLASSINFO() -> *mut c_void;
    pub fn wxFont_GetBaseFont(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetEncoding(self_: *const c_void) -> wxFontEncoding;
    pub fn wxFont_GetFaceName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetFamily(self_: *const c_void) -> wxFontFamily;
    pub fn wxFont_GetNativeFontInfoDesc(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_GetNativeFontInfoUserDesc(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_GetNativeFontInfo(self_: *const c_void) -> *const c_void;
    pub fn wxFont_GetPointSize(self_: *const c_void) -> c_int;
    pub fn wxFont_GetFractionalPointSize(self_: *const c_void) -> c_double;
    pub fn wxFont_GetPixelSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetStyle(self_: *const c_void) -> wxFontStyle;
    pub fn wxFont_GetUnderlined(self_: *const c_void) -> bool;
    pub fn wxFont_GetStrikethrough(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFont_GetWeight(self_: *const c_void) -> wxFontWeight;
    pub fn wxFont_GetNumericWeight(self_: *const c_void) -> c_int;
    pub fn wxFont_IsFixedWidth(self_: *const c_void) -> bool;
    pub fn wxFont_IsOk(self_: *const c_void) -> bool;
    pub fn wxFont_AddPrivateFont(filename: *const c_void) -> bool;
    pub fn wxFont_Bold(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Italic(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Larger(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Smaller(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Underlined(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_Strikethrough(self_: *const c_void) -> *mut c_void;
    pub fn wxFont_MakeBold(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeItalic(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeLarger(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeSmaller(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeUnderlined(self_: *mut c_void) -> *mut c_void;
    pub fn wxFont_MakeStrikethrough(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_Scale(self_: *mut c_void, x: float) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_Scaled(self_: *const c_void, x: float) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_SetEncoding(self_: *mut c_void, encoding: wxFontEncoding);
    pub fn wxFont_SetFaceName(self_: *mut c_void, face_name: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFont_SetFamily(self_: *mut c_void, family: wxFontFamily);
    pub fn wxFont_SetNativeFontInfo(self_: *mut c_void, info: *const c_void) -> bool;
    pub fn wxFont_SetNativeFontInfoUserDesc(self_: *mut c_void, info: *const c_void) -> bool;
    pub fn wxFont_SetNativeFontInfo1(self_: *mut c_void, info: *const c_void);
    pub fn wxFont_SetPointSize(self_: *mut c_void, point_size: c_int);
    pub fn wxFont_SetFractionalPointSize(self_: *mut c_void, point_size: c_double);
    pub fn wxFont_SetPixelSize(self_: *mut c_void, pixel_size: *const c_void);
    // NOT_SUPPORTED: pub fn wxFont_SetStyle(self_: *mut c_void, style: wxFontStyle);
    // NOT_SUPPORTED: pub fn wxFont_SetSymbolicSize(self_: *mut c_void, size: wxFontSymbolicSize);
    // NOT_SUPPORTED: pub fn wxFont_SetSymbolicSizeRelativeTo(self_: *mut c_void, size: wxFontSymbolicSize, base: c_int);
    pub fn wxFont_SetUnderlined(self_: *mut c_void, underlined: bool);
    pub fn wxFont_SetStrikethrough(self_: *mut c_void, strikethrough: bool);
    // NOT_SUPPORTED: pub fn wxFont_SetWeight(self_: *mut c_void, weight: wxFontWeight);
    pub fn wxFont_SetNumericWeight(self_: *mut c_void, weight: c_int);
    // BLOCKED: pub fn wxFont_operator!=(self_: *const c_void, font: *const c_void) -> bool;
    // BLOCKED: pub fn wxFont_operator==(self_: *const c_void, font: *const c_void) -> bool;
    // BLOCKED: pub fn wxFont_operator=(self_: *mut c_void, font: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_GetDefaultEncoding() -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFont_SetDefaultEncoding(encoding: wxFontEncoding);
    // NOT_SUPPORTED: pub fn wxFont_GetNumericWeightOf(weight: wxFontWeight) -> c_int;
    // NOT_SUPPORTED: pub fn wxFont_New(point_size: c_int, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_New1(point_size: c_int, family: wxFontFamily, flags: c_int, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_New2(pixel_size: *const c_void, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_New3(pixel_size: *const c_void, family: wxFontFamily, flags: c_int, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFont_New4(native_info: *const c_void) -> *mut c_void;
    pub fn wxFont_New5(native_info_string: *const c_void) -> *mut c_void;
    pub fn wxFont_new() -> *mut c_void;
    pub fn wxFont_new1(font: *const c_void) -> *mut c_void;
    pub fn wxFont_new2(font_info: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_new3(point_size: c_int, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFont_new4(pixel_size: *const c_void, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, face_name: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFont_new5(native_info_string: *const c_void) -> *mut c_void;
    pub fn wxFont_new6(native_info: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxFont_~wxFont(self_: *mut c_void);

    // wxFontData
    pub fn wxFontData_CLASSINFO() -> *mut c_void;
    pub fn wxFontData_new() -> *mut c_void;
    pub fn wxFontData_EnableEffects(self_: *mut c_void, enable: bool);
    pub fn wxFontData_GetAllowSymbols(self_: *const c_void) -> bool;
    pub fn wxFontData_GetChosenFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontData_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxFontData_GetEnableEffects(self_: *const c_void) -> bool;
    pub fn wxFontData_GetRestrictSelection(self_: *const c_void) -> c_int;
    pub fn wxFontData_GetInitialFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontData_GetShowHelp(self_: *const c_void) -> bool;
    pub fn wxFontData_RestrictSelection(self_: *mut c_void, flags: c_int);
    pub fn wxFontData_SetAllowSymbols(self_: *mut c_void, allow_symbols: bool);
    pub fn wxFontData_SetChosenFont(self_: *mut c_void, font: *const c_void);
    pub fn wxFontData_SetColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxFontData_SetInitialFont(self_: *mut c_void, font: *const c_void);
    pub fn wxFontData_SetRange(self_: *mut c_void, min: c_int, max: c_int);
    pub fn wxFontData_SetShowHelp(self_: *mut c_void, show_help: bool);
    // BLOCKED: pub fn wxFontData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;

    // wxFontDialog
    pub fn wxFontDialog_CLASSINFO() -> *mut c_void;
    pub fn wxFontDialog_new() -> *mut c_void;
    // BLOCKED: pub fn wxFontDialog_new1(parent: *mut c_void) -> *mut c_void;
    pub fn wxFontDialog_new2(parent: *mut c_void, data: *const c_void) -> *mut c_void;
    pub fn wxFontDialog_Create(self_: *mut c_void, parent: *mut c_void) -> bool;
    pub fn wxFontDialog_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        data: *const c_void,
    ) -> bool;
    // BLOCKED: pub fn wxFontDialog_GetFontData(self_: *const c_void) -> *mut c_void;
    pub fn wxFontDialog_GetFontData1(self_: *mut c_void) -> *mut c_void;

    // wxFontEnumerator
    pub fn wxFontEnumerator_delete(self_: *mut c_void);
    pub fn wxFontEnumerator_new() -> *mut c_void;
    // DTOR: pub fn wxFontEnumerator_~wxFontEnumerator(self_: *mut c_void);
    pub fn wxFontEnumerator_EnumerateEncodings(self_: *mut c_void, font: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFontEnumerator_EnumerateFacenames(self_: *mut c_void, encoding: wxFontEncoding, fixed_width_only: bool) -> bool;
    pub fn wxFontEnumerator_OnFacename(self_: *mut c_void, font: *const c_void) -> bool;
    pub fn wxFontEnumerator_OnFontEncoding(
        self_: *mut c_void,
        font: *const c_void,
        encoding: *const c_void,
    ) -> bool;
    pub fn wxFontEnumerator_GetEncodings(facename: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontEnumerator_GetFacenames(encoding: wxFontEncoding, fixed_width_only: bool) -> *mut c_void;
    pub fn wxFontEnumerator_IsValidFacename(facename: *const c_void) -> bool;
    pub fn wxFontEnumerator_InvalidateCache();

    // wxFontList
    pub fn wxFontList_delete(self_: *mut c_void);
    pub fn wxFontList_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontList_FindOrCreateFont(self_: *mut c_void, point_size: c_int, family: wxFontFamily, style: wxFontStyle, weight: wxFontWeight, underline: bool, facename: *const c_void, encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFontList_FindOrCreateFont1(
        self_: *mut c_void,
        font_info: *const c_void,
    ) -> *mut c_void;

    // wxFontMapper
    pub fn wxFontMapper_delete(self_: *mut c_void);
    pub fn wxFontMapper_new() -> *mut c_void;
    // DTOR: pub fn wxFontMapper_~wxFontMapper(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxFontMapper_CharsetToEncoding(self_: *mut c_void, charset: *const c_void, interactive: bool) -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetAltForEncoding(self_: *mut c_void, encoding: wxFontEncoding, info: *mut c_void, facename: *const c_void, interactive: bool) -> bool;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetAltForEncoding1(self_: *mut c_void, encoding: wxFontEncoding, alt_encoding: *mut c_void, facename: *const c_void, interactive: bool) -> bool;
    // NOT_SUPPORTED: pub fn wxFontMapper_IsEncodingAvailable(self_: *mut c_void, encoding: wxFontEncoding, facename: *const c_void) -> bool;
    pub fn wxFontMapper_SetConfigPath(self_: *mut c_void, prefix: *const c_void);
    pub fn wxFontMapper_SetDialogParent(self_: *mut c_void, parent: *mut c_void);
    pub fn wxFontMapper_SetDialogTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxFontMapper_Get() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetAllEncodingNames(encoding: wxFontEncoding) -> *const c_void;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncoding(n: usize) -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncodingDescription(encoding: wxFontEncoding) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncodingFromName(encoding: *const c_void) -> wxFontEncoding;
    // NOT_SUPPORTED: pub fn wxFontMapper_GetEncodingName(encoding: wxFontEncoding) -> *mut c_void;
    pub fn wxFontMapper_GetSupportedEncodingsCount() -> usize;
    pub fn wxFontMapper_Set(mapper: *mut c_void) -> *mut c_void;

    // wxFontPickerCtrl
    pub fn wxFontPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxFontPickerCtrl_new() -> *mut c_void;
    pub fn wxFontPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        font: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFontPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        font: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxFontPickerCtrl_GetMaxPointSize(self_: *const c_void) -> c_uint;
    pub fn wxFontPickerCtrl_GetMinPointSize(self_: *const c_void) -> c_uint;
    pub fn wxFontPickerCtrl_GetSelectedColour(self_: *const c_void) -> *mut c_void;
    pub fn wxFontPickerCtrl_GetSelectedFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontPickerCtrl_SetMaxPointSize(self_: *mut c_void, max: c_uint);
    pub fn wxFontPickerCtrl_SetMinPointSize(self_: *mut c_void, min: c_uint);
    pub fn wxFontPickerCtrl_SetSelectedColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxFontPickerCtrl_SetSelectedFont(self_: *mut c_void, font: *const c_void);

    // wxFontPickerEvent
    pub fn wxFontPickerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxFontPickerEvent_new(
        generator: *mut c_void,
        id: c_int,
        font: *const c_void,
    ) -> *mut c_void;
    pub fn wxFontPickerEvent_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxFontPickerEvent_SetFont(self_: *mut c_void, f: *const c_void);

    // wxFrame
    pub fn wxFrame_CLASSINFO() -> *mut c_void;
    pub fn wxFrame_new() -> *mut c_void;
    pub fn wxFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxFrame_~wxFrame(self_: *mut c_void);
    pub fn wxFrame_Centre(self_: *mut c_void, direction: c_int);
    pub fn wxFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxFrame_CreateStatusBar(
        self_: *mut c_void,
        number: c_int,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_CreateToolBar(
        self_: *mut c_void,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_DoGiveHelp(self_: *mut c_void, text: *const c_void, show: bool);
    pub fn wxFrame_GetMenuBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_GetStatusBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_GetStatusBarPane(self_: *const c_void) -> c_int;
    pub fn wxFrame_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_OnCreateStatusBar(
        self_: *mut c_void,
        number: c_int,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_OnCreateToolBar(
        self_: *mut c_void,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_ProcessCommand(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxFrame_SetMenuBar(self_: *mut c_void, menu_bar: *mut c_void);
    pub fn wxFrame_SetStatusBar(self_: *mut c_void, status_bar: *mut c_void);
    pub fn wxFrame_SetStatusBarPane(self_: *mut c_void, n: c_int);
    pub fn wxFrame_SetStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
    pub fn wxFrame_SetStatusWidths(self_: *mut c_void, n: c_int, widths_field: *const c_void);
    pub fn wxFrame_SetToolBar(self_: *mut c_void, tool_bar: *mut c_void);
    pub fn wxFrame_MSWGetTaskBarButton(self_: *mut c_void) -> *mut c_void;
    pub fn wxFrame_PushStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
    pub fn wxFrame_PopStatusText(self_: *mut c_void, number: c_int);

}
