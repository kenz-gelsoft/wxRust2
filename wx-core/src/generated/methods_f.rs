use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

pub use wx_base::methods::*;

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
