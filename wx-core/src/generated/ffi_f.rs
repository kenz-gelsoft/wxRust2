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
