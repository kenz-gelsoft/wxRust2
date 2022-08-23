use super::*;

// wxTGAHandler
pub trait TGAHandlerMethods: ImageHandlerMethods {}

// wxTIFFHandler
pub trait TIFFHandlerMethods: ImageHandlerMethods {}

// wxTaskBarIcon
pub trait TaskBarIconMethods: EvtHandlerMethods {
    // DTOR: fn ~wxTaskBarIcon()
    fn destroy(&self) {
        unsafe { ffi::wxTaskBarIcon_Destroy(self.as_ptr()) }
    }
    fn is_icon_installed(&self) -> bool {
        unsafe { ffi::wxTaskBarIcon_IsIconInstalled(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxTaskBarIcon_IsOk(self.as_ptr()) }
    }
    fn popup_menu<M: MenuMethods>(&self, menu: Option<&M>) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTaskBarIcon_PopupMenu(self.as_ptr(), menu)
        }
    }
    fn remove_icon(&self) -> bool {
        unsafe { ffi::wxTaskBarIcon_RemoveIcon(self.as_ptr()) }
    }
    fn set_icon<B: BitmapBundleMethods>(&self, icon: &B, tooltip: &str) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            let tooltip = WxString::from(tooltip);
            let tooltip = tooltip.as_ptr();
            ffi::wxTaskBarIcon_SetIcon(self.as_ptr(), icon, tooltip)
        }
    }
    fn is_available() -> bool {
        unsafe { ffi::wxTaskBarIcon_IsAvailable() }
    }
}

// wxTaskBarIconEvent
pub trait TaskBarIconEventMethods: EventMethods {}

// wxTextAttr
pub trait TextAttrMethods: WxRustMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetBackgroundColour(self.as_ptr())) }
    }
    fn get_bullet_font(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetBulletFont(self.as_ptr())).into() }
    }
    fn get_bullet_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetBulletName(self.as_ptr())).into() }
    }
    fn get_bullet_number(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetBulletNumber(self.as_ptr()) }
    }
    fn get_bullet_style(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetBulletStyle(self.as_ptr()) }
    }
    fn get_bullet_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetBulletText(self.as_ptr())).into() }
    }
    fn get_character_style_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetCharacterStyleName(self.as_ptr())).into() }
    }
    fn get_flags(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetFlags(self.as_ptr()) }
    }
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxTextAttr_GetFont(self.as_ptr())) }
    }
    fn get_font_attributes<F: FontMethods>(&self, font: &F, flags: c_int) -> bool {
        unsafe {
            let font = font.as_ptr();
            ffi::wxTextAttr_GetFontAttributes(self.as_ptr(), font, flags)
        }
    }
    // NOT_SUPPORTED: fn GetFontEncoding()
    fn get_font_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetFontFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFontFamily()
    fn get_font_size(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetFontSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontStyle()
    fn get_font_underlined(&self) -> bool {
        unsafe { ffi::wxTextAttr_GetFontUnderlined(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetUnderlineType()
    fn get_underline_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetUnderlineColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetFontWeight()
    fn get_left_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetLeftIndent(self.as_ptr()) }
    }
    fn get_left_sub_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetLeftSubIndent(self.as_ptr()) }
    }
    fn get_line_spacing(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetLineSpacing(self.as_ptr()) }
    }
    fn get_list_style_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetListStyleName(self.as_ptr())).into() }
    }
    fn get_outline_level(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetOutlineLevel(self.as_ptr()) }
    }
    fn get_paragraph_spacing_after(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetParagraphSpacingAfter(self.as_ptr()) }
    }
    fn get_paragraph_spacing_before(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetParagraphSpacingBefore(self.as_ptr()) }
    }
    fn get_paragraph_style_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetParagraphStyleName(self.as_ptr())).into() }
    }
    fn get_right_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetRightIndent(self.as_ptr()) }
    }
    fn get_tabs(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxTextAttr_GetTabs(self.as_ptr())) }
    }
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetTextColour(self.as_ptr())) }
    }
    fn get_text_effect_flags(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetTextEffectFlags(self.as_ptr()) }
    }
    fn get_text_effects(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetTextEffects(self.as_ptr()) }
    }
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetURL(self.as_ptr())).into() }
    }
    fn has_alignment(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasAlignment(self.as_ptr()) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn has_bullet_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletName(self.as_ptr()) }
    }
    fn has_bullet_number(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletNumber(self.as_ptr()) }
    }
    fn has_bullet_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletStyle(self.as_ptr()) }
    }
    fn has_bullet_text(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletText(self.as_ptr()) }
    }
    fn has_character_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasCharacterStyleName(self.as_ptr()) }
    }
    fn has_flag(&self, flag: c_long) -> bool {
        unsafe { ffi::wxTextAttr_HasFlag(self.as_ptr(), flag) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFont(self.as_ptr()) }
    }
    fn has_font_encoding(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontEncoding(self.as_ptr()) }
    }
    fn has_font_face_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontFaceName(self.as_ptr()) }
    }
    fn has_font_family(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontFamily(self.as_ptr()) }
    }
    fn has_font_italic(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontItalic(self.as_ptr()) }
    }
    fn has_font_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontSize(self.as_ptr()) }
    }
    fn has_font_point_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontPointSize(self.as_ptr()) }
    }
    fn has_font_pixel_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontPixelSize(self.as_ptr()) }
    }
    fn has_font_underlined(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontUnderlined(self.as_ptr()) }
    }
    fn has_font_weight(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontWeight(self.as_ptr()) }
    }
    fn has_left_indent(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasLeftIndent(self.as_ptr()) }
    }
    fn has_line_spacing(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasLineSpacing(self.as_ptr()) }
    }
    fn has_list_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasListStyleName(self.as_ptr()) }
    }
    fn has_outline_level(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasOutlineLevel(self.as_ptr()) }
    }
    fn has_page_break(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasPageBreak(self.as_ptr()) }
    }
    fn has_paragraph_spacing_after(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphSpacingAfter(self.as_ptr()) }
    }
    fn has_paragraph_spacing_before(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphSpacingBefore(self.as_ptr()) }
    }
    fn has_paragraph_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphStyleName(self.as_ptr()) }
    }
    fn has_right_indent(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasRightIndent(self.as_ptr()) }
    }
    fn has_tabs(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTabs(self.as_ptr()) }
    }
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTextColour(self.as_ptr()) }
    }
    fn has_text_effects(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTextEffects(self.as_ptr()) }
    }
    fn has_url(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasURL(self.as_ptr()) }
    }
    fn is_character_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsCharacterStyle(self.as_ptr()) }
    }
    fn is_default(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsDefault(self.as_ptr()) }
    }
    fn is_paragraph_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsParagraphStyle(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxTextAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    fn set_bullet_font(&self, font: &str) {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxTextAttr_SetBulletFont(self.as_ptr(), font)
        }
    }
    fn set_bullet_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetBulletName(self.as_ptr(), name)
        }
    }
    fn set_bullet_number(&self, n: c_int) {
        unsafe { ffi::wxTextAttr_SetBulletNumber(self.as_ptr(), n) }
    }
    fn set_bullet_style(&self, style: c_int) {
        unsafe { ffi::wxTextAttr_SetBulletStyle(self.as_ptr(), style) }
    }
    fn set_bullet_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTextAttr_SetBulletText(self.as_ptr(), text)
        }
    }
    fn set_character_style_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetCharacterStyleName(self.as_ptr(), name)
        }
    }
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxTextAttr_SetFlags(self.as_ptr(), flags) }
    }
    fn set_font<F: FontMethods>(&self, font: &F, flags: c_int) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxTextAttr_SetFont(self.as_ptr(), font, flags)
        }
    }
    // NOT_SUPPORTED: fn SetFontEncoding()
    fn set_font_face_name(&self, face_name: &str) {
        unsafe {
            let face_name = WxString::from(face_name);
            let face_name = face_name.as_ptr();
            ffi::wxTextAttr_SetFontFaceName(self.as_ptr(), face_name)
        }
    }
    // NOT_SUPPORTED: fn SetFontFamily()
    fn set_font_size(&self, point_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontSize(self.as_ptr(), point_size) }
    }
    fn set_font_point_size(&self, point_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontPointSize(self.as_ptr(), point_size) }
    }
    fn set_font_pixel_size(&self, pixel_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontPixelSize(self.as_ptr(), pixel_size) }
    }
    // NOT_SUPPORTED: fn SetFontStyle()
    fn set_font_underlined(&self, underlined: bool) {
        unsafe { ffi::wxTextAttr_SetFontUnderlined(self.as_ptr(), underlined) }
    }
    // NOT_SUPPORTED: fn SetFontUnderlined1()
    // NOT_SUPPORTED: fn SetFontWeight()
    fn set_left_indent(&self, indent: c_int, sub_indent: c_int) {
        unsafe { ffi::wxTextAttr_SetLeftIndent(self.as_ptr(), indent, sub_indent) }
    }
    fn set_line_spacing(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetLineSpacing(self.as_ptr(), spacing) }
    }
    fn set_list_style_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetListStyleName(self.as_ptr(), name)
        }
    }
    fn set_outline_level(&self, level: c_int) {
        unsafe { ffi::wxTextAttr_SetOutlineLevel(self.as_ptr(), level) }
    }
    fn set_page_break(&self, page_break: bool) {
        unsafe { ffi::wxTextAttr_SetPageBreak(self.as_ptr(), page_break) }
    }
    fn set_paragraph_spacing_after(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetParagraphSpacingAfter(self.as_ptr(), spacing) }
    }
    fn set_paragraph_spacing_before(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetParagraphSpacingBefore(self.as_ptr(), spacing) }
    }
    fn set_paragraph_style_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetParagraphStyleName(self.as_ptr(), name)
        }
    }
    fn set_right_indent(&self, indent: c_int) {
        unsafe { ffi::wxTextAttr_SetRightIndent(self.as_ptr(), indent) }
    }
    fn set_tabs<A: ArrayIntMethods>(&self, tabs: &A) {
        unsafe {
            let tabs = tabs.as_ptr();
            ffi::wxTextAttr_SetTabs(self.as_ptr(), tabs)
        }
    }
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxTextAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    fn set_text_effect_flags(&self, flags: c_int) {
        unsafe { ffi::wxTextAttr_SetTextEffectFlags(self.as_ptr(), flags) }
    }
    fn set_text_effects(&self, effects: c_int) {
        unsafe { ffi::wxTextAttr_SetTextEffects(self.as_ptr(), effects) }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxTextAttr_SetURL(self.as_ptr(), url)
        }
    }
    // BLOCKED: fn operator=()
    fn apply<T: TextAttrMethods, T2: TextAttrMethods>(
        &self,
        style: &T,
        compare_with: Option<&T2>,
    ) -> bool {
        unsafe {
            let style = style.as_ptr();
            let compare_with = match compare_with {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTextAttr_Apply(self.as_ptr(), style, compare_with)
        }
    }
    fn merge<T: TextAttrMethods>(&self, overlay: &T) {
        unsafe {
            let overlay = overlay.as_ptr();
            ffi::wxTextAttr_Merge(self.as_ptr(), overlay)
        }
    }
    fn eq_partial<T: TextAttrMethods>(&self, attr: &T, weak_test: bool) -> bool {
        unsafe {
            let attr = attr.as_ptr();
            ffi::wxTextAttr_EqPartial(self.as_ptr(), attr, weak_test)
        }
    }
    fn merge_textattr<T: TextAttrMethods, T2: TextAttrMethods>(base: &T, overlay: &T2) -> TextAttr {
        unsafe {
            let base = base.as_ptr();
            let overlay = overlay.as_ptr();
            TextAttr::from_ptr(ffi::wxTextAttr_Merge1(base, overlay))
        }
    }
}

// wxTextCtrl
pub trait TextCtrlMethods: ControlMethods {
    fn osx_enable_new_line_replacement(&self, enable: bool) {
        unsafe { ffi::wxTextCtrl_OSXEnableNewLineReplacement(self.as_ptr(), enable) }
    }
    // BLOCKED: fn operator<<()
    // BLOCKED: fn operator<<1()
    // BLOCKED: fn operator<<2()
    // NOT_SUPPORTED: fn operator<<3()
    // BLOCKED: fn operator<<4()
    // NOT_SUPPORTED: fn operator<<5()
    // NOT_SUPPORTED: fn operator<<6()
    // DTOR: fn ~wxTextCtrl()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
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
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn discard_edits(&self) {
        unsafe { ffi::wxTextCtrl_DiscardEdits(self.as_ptr()) }
    }
    fn empty_undo_buffer(&self) {
        unsafe { ffi::wxTextCtrl_EmptyUndoBuffer(self.as_ptr()) }
    }
    fn emulate_key_press<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxTextCtrl_EmulateKeyPress(self.as_ptr(), event)
        }
    }
    fn enable_proof_check(&self, options: *const c_void) -> bool {
        unsafe { ffi::wxTextCtrl_EnableProofCheck(self.as_ptr(), options) }
    }
    fn get_default_style(&self) -> TextAttrIsOwned<false> {
        unsafe { TextAttrIsOwned::from_ptr(ffi::wxTextCtrl_GetDefaultStyle(self.as_ptr())) }
    }
    fn get_line_length(&self, line_no: c_long) -> c_int {
        unsafe { ffi::wxTextCtrl_GetLineLength(self.as_ptr(), line_no) }
    }
    fn get_line_text(&self, line_no: c_long) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextCtrl_GetLineText(self.as_ptr(), line_no)).into() }
    }
    fn get_number_of_lines(&self) -> c_int {
        unsafe { ffi::wxTextCtrl_GetNumberOfLines(self.as_ptr()) }
    }
    fn get_style<T: TextAttrMethods>(&self, position: c_long, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_GetStyle(self.as_ptr(), position, style)
        }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    fn is_modified(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsModified(self.as_ptr()) }
    }
    fn is_multi_line(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsMultiLine(self.as_ptr()) }
    }
    fn is_single_line(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsSingleLine(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetProofCheckOptions()
    fn load_file(&self, filename: &str, file_type: c_int) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxTextCtrl_LoadFile(self.as_ptr(), filename, file_type)
        }
    }
    fn mark_dirty(&self) {
        unsafe { ffi::wxTextCtrl_MarkDirty(self.as_ptr()) }
    }
    fn on_drop_files<D: DropFilesEventMethods>(&self, event: &D) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxTextCtrl_OnDropFiles(self.as_ptr(), event)
        }
    }
    fn position_to_xy(&self, pos: c_long, x: *mut c_void, y: *mut c_void) -> bool {
        unsafe { ffi::wxTextCtrl_PositionToXY(self.as_ptr(), pos, x, y) }
    }
    fn position_to_coords(&self, pos: c_long) -> Point {
        unsafe { Point::from_ptr(ffi::wxTextCtrl_PositionToCoords(self.as_ptr(), pos)) }
    }
    fn save_file(&self, filename: &str, file_type: c_int) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxTextCtrl_SaveFile(self.as_ptr(), filename, file_type)
        }
    }
    fn set_default_style<T: TextAttrMethods>(&self, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_SetDefaultStyle(self.as_ptr(), style)
        }
    }
    fn set_modified(&self, modified: bool) {
        unsafe { ffi::wxTextCtrl_SetModified(self.as_ptr(), modified) }
    }
    fn set_style<T: TextAttrMethods>(&self, start: c_long, end: c_long, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_SetStyle(self.as_ptr(), start, end, style)
        }
    }
    fn show_position(&self, pos: c_long) {
        unsafe { ffi::wxTextCtrl_ShowPosition(self.as_ptr(), pos) }
    }
    fn xy_to_position(&self, x: c_long, y: c_long) -> c_long {
        unsafe { ffi::wxTextCtrl_XYToPosition(self.as_ptr(), x, y) }
    }
}

// wxTextDataObject
pub trait TextDataObjectMethods: DataObjectSimpleMethods {
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextDataObject_GetText(self.as_ptr())).into() }
    }
    fn get_text_length(&self) -> usize {
        unsafe { ffi::wxTextDataObject_GetTextLength(self.as_ptr()) }
    }
    fn set_text(&self, str_text: &str) {
        unsafe {
            let str_text = WxString::from(str_text);
            let str_text = str_text.as_ptr();
            ffi::wxTextDataObject_SetText(self.as_ptr(), str_text)
        }
    }
}

// wxTextDropTarget
pub trait TextDropTargetMethods: DropTargetMethods {
    fn on_drop_text(&self, x: c_int, y: c_int, data: &str) -> bool {
        unsafe {
            let data = WxString::from(data);
            let data = data.as_ptr();
            ffi::wxTextDropTarget_OnDropText(self.as_ptr(), x, y, data)
        }
    }
}

// wxTextEntry
pub trait TextEntryMethods: WxRustMethods {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    fn append_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTextEntry_AppendText(self.as_text_entry(), text)
        }
    }
    fn auto_complete_arraystring<A: ArrayStringMethods>(&self, choices: &A) -> bool {
        unsafe {
            let choices = choices.as_ptr();
            ffi::wxTextEntry_AutoComplete(self.as_text_entry(), choices)
        }
    }
    fn auto_complete_textcompleter(&self, completer: *mut c_void) -> bool {
        unsafe { ffi::wxTextEntry_AutoComplete1(self.as_text_entry(), completer) }
    }
    fn auto_complete_file_names(&self) -> bool {
        unsafe { ffi::wxTextEntry_AutoCompleteFileNames(self.as_text_entry()) }
    }
    fn auto_complete_directories(&self) -> bool {
        unsafe { ffi::wxTextEntry_AutoCompleteDirectories(self.as_text_entry()) }
    }
    fn can_copy(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanCopy(self.as_text_entry()) }
    }
    fn can_cut(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanCut(self.as_text_entry()) }
    }
    fn can_paste(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanPaste(self.as_text_entry()) }
    }
    fn can_redo(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanRedo(self.as_text_entry()) }
    }
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanUndo(self.as_text_entry()) }
    }
    fn change_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntry_ChangeValue(self.as_text_entry(), value)
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxTextEntry_Clear(self.as_text_entry()) }
    }
    fn copy(&self) {
        unsafe { ffi::wxTextEntry_Copy(self.as_text_entry()) }
    }
    fn cut(&self) {
        unsafe { ffi::wxTextEntry_Cut(self.as_text_entry()) }
    }
    fn force_upper(&self) {
        unsafe { ffi::wxTextEntry_ForceUpper(self.as_text_entry()) }
    }
    fn get_insertion_point(&self) -> c_long {
        unsafe { ffi::wxTextEntry_GetInsertionPoint(self.as_text_entry()) }
    }
    // NOT_SUPPORTED: fn GetLastPosition()
    fn get_range(&self, from: c_long, to: c_long) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxTextEntry_GetRange(self.as_text_entry(), from, to)).into()
        }
    }
    fn get_selection_long(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { ffi::wxTextEntry_GetSelection(self.as_text_entry(), from, to) }
    }
    fn get_string_selection(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxTextEntry_GetStringSelection(self.as_text_entry())).into()
        }
    }
    fn get_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextEntry_GetValue(self.as_text_entry())).into() }
    }
    fn is_editable(&self) -> bool {
        unsafe { ffi::wxTextEntry_IsEditable(self.as_text_entry()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxTextEntry_IsEmpty(self.as_text_entry()) }
    }
    fn paste(&self) {
        unsafe { ffi::wxTextEntry_Paste(self.as_text_entry()) }
    }
    fn redo(&self) {
        unsafe { ffi::wxTextEntry_Redo(self.as_text_entry()) }
    }
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxTextEntry_Remove(self.as_text_entry(), from, to) }
    }
    fn replace(&self, from: c_long, to: c_long, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntry_Replace(self.as_text_entry(), from, to, value)
        }
    }
    fn set_editable(&self, editable: bool) {
        unsafe { ffi::wxTextEntry_SetEditable(self.as_text_entry(), editable) }
    }
    fn set_insertion_point(&self, pos: c_long) {
        unsafe { ffi::wxTextEntry_SetInsertionPoint(self.as_text_entry(), pos) }
    }
    fn set_insertion_point_end(&self) {
        unsafe { ffi::wxTextEntry_SetInsertionPointEnd(self.as_text_entry()) }
    }
    // NOT_SUPPORTED: fn SetMaxLength()
    fn set_selection_long(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxTextEntry_SetSelection(self.as_text_entry(), from, to) }
    }
    fn select_all(&self) {
        unsafe { ffi::wxTextEntry_SelectAll(self.as_text_entry()) }
    }
    fn select_none(&self) {
        unsafe { ffi::wxTextEntry_SelectNone(self.as_text_entry()) }
    }
    fn set_hint(&self, hint: &str) -> bool {
        unsafe {
            let hint = WxString::from(hint);
            let hint = hint.as_ptr();
            ffi::wxTextEntry_SetHint(self.as_text_entry(), hint)
        }
    }
    fn get_hint(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextEntry_GetHint(self.as_text_entry())).into() }
    }
    fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxTextEntry_SetMargins(self.as_text_entry(), pt)
        }
    }
    fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
        unsafe { ffi::wxTextEntry_SetMargins1(self.as_text_entry(), left, top) }
    }
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxTextEntry_GetMargins(self.as_text_entry())) }
    }
    fn set_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntry_SetValue(self.as_text_entry(), value)
        }
    }
    fn undo(&self) {
        unsafe { ffi::wxTextEntry_Undo(self.as_text_entry()) }
    }
    fn write_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTextEntry_WriteText(self.as_text_entry(), text)
        }
    }
}

// wxTextEntryDialog
pub trait TextEntryDialogMethods: DialogMethods {
    fn create_str<W: WindowMethods, P: PointMethods>(
        &self,
        parent: Option<&W>,
        message: &str,
        caption: &str,
        value: &str,
        style: c_long,
        pos: &P,
    ) -> bool {
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
            ffi::wxTextEntryDialog_Create(
                self.as_ptr(),
                parent,
                message,
                caption,
                value,
                style,
                pos,
            )
        }
    }
    // DTOR: fn ~wxTextEntryDialog()
    fn get_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextEntryDialog_GetValue(self.as_ptr())).into() }
    }
    fn set_text_validator<T: TextValidatorMethods>(&self, validator: &T) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxTextEntryDialog_SetTextValidator(self.as_ptr(), validator)
        }
    }
    // NOT_SUPPORTED: fn SetTextValidator1()
    // NOT_SUPPORTED: fn SetMaxLength()
    fn set_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntryDialog_SetValue(self.as_ptr(), value)
        }
    }
    fn force_upper(&self) {
        unsafe { ffi::wxTextEntryDialog_ForceUpper(self.as_ptr()) }
    }
}

// wxTextValidator
pub trait TextValidatorMethods: ValidatorMethods {
    fn get_char_excludes(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextValidator_GetCharExcludes(self.as_ptr())).into() }
    }
    fn get_char_includes(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextValidator_GetCharIncludes(self.as_ptr())).into() }
    }
    fn get_excludes(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxTextValidator_GetExcludes(self.as_ptr())) }
    }
    fn get_includes(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxTextValidator_GetIncludes(self.as_ptr())) }
    }
    fn get_style(&self) -> c_long {
        unsafe { ffi::wxTextValidator_GetStyle(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HasFlag()
    fn on_char<K: KeyEventMethods>(&self, event: &K) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxTextValidator_OnChar(self.as_ptr(), event)
        }
    }
    fn set_excludes<A: ArrayStringMethods>(&self, string_list: &A) {
        unsafe {
            let string_list = string_list.as_ptr();
            ffi::wxTextValidator_SetExcludes(self.as_ptr(), string_list)
        }
    }
    fn set_char_excludes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_SetCharExcludes(self.as_ptr(), chars)
        }
    }
    fn set_includes<A: ArrayStringMethods>(&self, string_list: &A) {
        unsafe {
            let string_list = string_list.as_ptr();
            ffi::wxTextValidator_SetIncludes(self.as_ptr(), string_list)
        }
    }
    fn set_char_includes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_SetCharIncludes(self.as_ptr(), chars)
        }
    }
    fn add_exclude(&self, exclude: &str) {
        unsafe {
            let exclude = WxString::from(exclude);
            let exclude = exclude.as_ptr();
            ffi::wxTextValidator_AddExclude(self.as_ptr(), exclude)
        }
    }
    fn add_include(&self, include: &str) {
        unsafe {
            let include = WxString::from(include);
            let include = include.as_ptr();
            ffi::wxTextValidator_AddInclude(self.as_ptr(), include)
        }
    }
    fn add_char_excludes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_AddCharExcludes(self.as_ptr(), chars)
        }
    }
    fn add_char_includes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_AddCharIncludes(self.as_ptr(), chars)
        }
    }
    fn set_style(&self, style: c_long) {
        unsafe { ffi::wxTextValidator_SetStyle(self.as_ptr(), style) }
    }
    fn is_valid(&self, val: &str) -> String {
        unsafe {
            let val = WxString::from(val);
            let val = val.as_ptr();
            WxString::from_ptr(ffi::wxTextValidator_IsValid(self.as_ptr(), val)).into()
        }
    }
}

// wxThreadEvent
pub trait ThreadEventMethods: EventMethods {
    // BLOCKED: fn SetPayload()
    // NOT_SUPPORTED: fn GetPayload()
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxThreadEvent_GetExtraLong(self.as_ptr()) }
    }
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxThreadEvent_GetInt(self.as_ptr()) }
    }
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxThreadEvent_GetString(self.as_ptr())).into() }
    }
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxThreadEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxThreadEvent_SetInt(self.as_ptr(), int_command) }
    }
    fn set_string(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxThreadEvent_SetString(self.as_ptr(), string)
        }
    }
}

// wxTimePickerCtrl
pub trait TimePickerCtrlMethods: ControlMethods {
    fn create_datetime<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        dt: &D,
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
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTimePickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                dt,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_time(&self, hour: *mut c_void, min: *mut c_void, sec: *mut c_void) -> bool {
        unsafe { ffi::wxTimePickerCtrl_GetTime(self.as_ptr(), hour, min, sec) }
    }
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxTimePickerCtrl_GetValue(self.as_ptr())) }
    }
    fn set_time(&self, hour: c_int, min: c_int, sec: c_int) -> bool {
        unsafe { ffi::wxTimePickerCtrl_SetTime(self.as_ptr(), hour, min, sec) }
    }
    fn set_value<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxTimePickerCtrl_SetValue(self.as_ptr(), dt)
        }
    }
}

// wxTipProvider
pub trait TipProviderMethods: WxRustMethods {
    // DTOR: fn ~wxTipProvider()
    fn get_current_tip(&self) -> usize {
        unsafe { ffi::wxTipProvider_GetCurrentTip(self.as_ptr()) }
    }
    fn get_tip(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTipProvider_GetTip(self.as_ptr())).into() }
    }
}

// wxTipWindow
pub trait TipWindowMethods: WindowMethods {
    fn set_bounding_rect<R: RectMethods>(&self, rect_bound: &R) {
        unsafe {
            let rect_bound = rect_bound.as_ptr();
            ffi::wxTipWindow_SetBoundingRect(self.as_ptr(), rect_bound)
        }
    }
    fn set_tip_window_ptr<T: TipWindowMethods>(&self, window_ptr: Option<&T>) {
        unsafe {
            let window_ptr = match window_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTipWindow_SetTipWindowPtr(self.as_ptr(), window_ptr)
        }
    }
}

// wxToggleButton
pub trait ToggleButtonMethods: AnyButtonMethods {
    // DTOR: fn ~wxToggleButton()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
        name: &str,
    ) -> bool {
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
            ffi::wxToggleButton_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                val,
                name,
            )
        }
    }
    fn get_value(&self) -> bool {
        unsafe { ffi::wxToggleButton_GetValue(self.as_ptr()) }
    }
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxToggleButton_SetValue(self.as_ptr(), state) }
    }
}

// wxToolBar
pub trait ToolBarMethods: ControlMethods {
    // DTOR: fn ~wxToolBar()
    fn add_check_tool<B: BitmapBundleMethods, B2: BitmapBundleMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap1: &B,
        bmp_disabled: &B2,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bitmap1 = bitmap1.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            let long_help = WxString::from(long_help);
            let long_help = long_help.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_AddCheckTool(
                self.as_ptr(),
                tool_id,
                label,
                bitmap1,
                bmp_disabled,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn add_control<C: ControlMethods>(&self, control: Option<&C>, label: &str) -> *mut c_void {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxToolBar_AddControl(self.as_ptr(), control, label)
        }
    }
    fn add_radio_tool<B: BitmapBundleMethods, B2: BitmapBundleMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap1: &B,
        bmp_disabled: &B2,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bitmap1 = bitmap1.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            let long_help = WxString::from(long_help);
            let long_help = long_help.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_AddRadioTool(
                self.as_ptr(),
                tool_id,
                label,
                bitmap1,
                bmp_disabled,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn add_separator(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddSeparator(self.as_ptr()) }
    }
    fn add_stretchable_space(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddStretchableSpace(self.as_ptr()) }
    }
    fn add_tool_toolbartoolbase(&self, tool: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddTool(self.as_ptr(), tool) }
    }
    fn add_tool_int_str<B: BitmapBundleMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        short_help: &str,
        kind: c_int,
    ) -> *mut c_void {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bitmap = bitmap.as_ptr();
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            ffi::wxToolBar_AddTool1(self.as_ptr(), tool_id, label, bitmap, short_help, kind)
        }
    }
    fn add_tool_int_bitmapbundle<
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
        O: ObjectMethods,
    >(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        bmp_disabled: &B2,
        kind: c_int,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bitmap = bitmap.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            let long_help = WxString::from(long_help);
            let long_help = long_help.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_AddTool2(
                self.as_ptr(),
                tool_id,
                label,
                bitmap,
                bmp_disabled,
                kind,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn clear_tools(&self) {
        unsafe { ffi::wxToolBar_ClearTools(self.as_ptr()) }
    }
    fn delete_tool(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_DeleteTool(self.as_ptr(), tool_id) }
    }
    fn delete_tool_by_pos(&self, pos: usize) -> bool {
        unsafe { ffi::wxToolBar_DeleteToolByPos(self.as_ptr(), pos) }
    }
    fn enable_tool(&self, tool_id: c_int, enable: bool) {
        unsafe { ffi::wxToolBar_EnableTool(self.as_ptr(), tool_id, enable) }
    }
    fn find_by_id(&self, id: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_FindById(self.as_ptr(), id) }
    }
    fn find_control(&self, id: c_int) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxToolBar_FindControl(self.as_ptr(), id)) }
    }
    fn find_tool_for_position(&self, x: c_int, y: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_FindToolForPosition(self.as_ptr(), x, y) }
    }
    fn get_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetMargins(self.as_ptr())) }
    }
    fn get_tool_bitmap_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetToolBitmapSize(self.as_ptr())) }
    }
    // BLOCKED: fn GetToolByPos()
    fn get_tool_by_pos(&self, pos: c_int) -> *const c_void {
        unsafe { ffi::wxToolBar_GetToolByPos1(self.as_ptr(), pos) }
    }
    fn get_tool_client_data(&self, tool_id: c_int) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxToolBar_GetToolClientData(self.as_ptr(), tool_id)) }
    }
    fn get_tool_enabled(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_GetToolEnabled(self.as_ptr(), tool_id) }
    }
    fn get_tool_long_help(&self, tool_id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxToolBar_GetToolLongHelp(self.as_ptr(), tool_id)).into() }
    }
    fn get_tool_packing(&self) -> c_int {
        unsafe { ffi::wxToolBar_GetToolPacking(self.as_ptr()) }
    }
    fn get_tool_pos(&self, tool_id: c_int) -> c_int {
        unsafe { ffi::wxToolBar_GetToolPos(self.as_ptr(), tool_id) }
    }
    fn get_tool_separation(&self) -> c_int {
        unsafe { ffi::wxToolBar_GetToolSeparation(self.as_ptr()) }
    }
    fn get_tool_short_help(&self, tool_id: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxToolBar_GetToolShortHelp(self.as_ptr(), tool_id)).into()
        }
    }
    fn get_tool_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetToolSize(self.as_ptr())) }
    }
    fn get_tool_state(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_GetToolState(self.as_ptr(), tool_id) }
    }
    fn get_tools_count(&self) -> usize {
        unsafe { ffi::wxToolBar_GetToolsCount(self.as_ptr()) }
    }
    fn insert_control<C: ControlMethods>(
        &self,
        pos: usize,
        control: Option<&C>,
        label: &str,
    ) -> *mut c_void {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxToolBar_InsertControl(self.as_ptr(), pos, control, label)
        }
    }
    fn insert_separator(&self, pos: usize) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertSeparator(self.as_ptr(), pos) }
    }
    fn insert_stretchable_space(&self, pos: usize) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertStretchableSpace(self.as_ptr(), pos) }
    }
    fn insert_tool_int<B: BitmapBundleMethods, B2: BitmapBundleMethods, O: ObjectMethods>(
        &self,
        pos: usize,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        bmp_disabled: &B2,
        kind: c_int,
        short_help: &str,
        long_help: &str,
        client_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bitmap = bitmap.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            let long_help = WxString::from(long_help);
            let long_help = long_help.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_InsertTool(
                self.as_ptr(),
                pos,
                tool_id,
                label,
                bitmap,
                bmp_disabled,
                kind,
                short_help,
                long_help,
                client_data,
            )
        }
    }
    fn insert_tool_toolbartoolbase(&self, pos: usize, tool: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertTool1(self.as_ptr(), pos, tool) }
    }
    fn on_left_click(&self, tool_id: c_int, toggle_down: bool) -> bool {
        unsafe { ffi::wxToolBar_OnLeftClick(self.as_ptr(), tool_id, toggle_down) }
    }
    fn on_mouse_enter(&self, tool_id: c_int) {
        unsafe { ffi::wxToolBar_OnMouseEnter(self.as_ptr(), tool_id) }
    }
    fn on_right_click(&self, tool_id: c_int, x: c_long, y: c_long) {
        unsafe { ffi::wxToolBar_OnRightClick(self.as_ptr(), tool_id, x, y) }
    }
    fn realize(&self) -> bool {
        unsafe { ffi::wxToolBar_Realize(self.as_ptr()) }
    }
    fn remove_tool(&self, id: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_RemoveTool(self.as_ptr(), id) }
    }
    fn set_dropdown_menu<M: MenuMethods>(&self, id: c_int, menu: Option<&M>) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_SetDropdownMenu(self.as_ptr(), id, menu)
        }
    }
    fn set_margins_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxToolBar_SetMargins(self.as_ptr(), x, y) }
    }
    fn set_margins_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxToolBar_SetMargins1(self.as_ptr(), size)
        }
    }
    fn set_tool_bitmap_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxToolBar_SetToolBitmapSize(self.as_ptr(), size)
        }
    }
    fn set_tool_client_data<O: ObjectMethods>(&self, id: c_int, client_data: Option<&O>) {
        unsafe {
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_SetToolClientData(self.as_ptr(), id, client_data)
        }
    }
    fn set_tool_disabled_bitmap<B: BitmapBundleMethods>(&self, id: c_int, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxToolBar_SetToolDisabledBitmap(self.as_ptr(), id, bitmap)
        }
    }
    fn set_tool_long_help(&self, tool_id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxToolBar_SetToolLongHelp(self.as_ptr(), tool_id, help_string)
        }
    }
    fn set_tool_normal_bitmap<B: BitmapBundleMethods>(&self, id: c_int, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxToolBar_SetToolNormalBitmap(self.as_ptr(), id, bitmap)
        }
    }
    fn set_tool_packing(&self, packing: c_int) {
        unsafe { ffi::wxToolBar_SetToolPacking(self.as_ptr(), packing) }
    }
    fn set_tool_separation(&self, separation: c_int) {
        unsafe { ffi::wxToolBar_SetToolSeparation(self.as_ptr(), separation) }
    }
    fn set_tool_short_help(&self, tool_id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxToolBar_SetToolShortHelp(self.as_ptr(), tool_id, help_string)
        }
    }
    fn toggle_tool(&self, tool_id: c_int, toggle: bool) {
        unsafe { ffi::wxToolBar_ToggleTool(self.as_ptr(), tool_id, toggle) }
    }
    fn create_tool_int<B: BitmapBundleMethods, B2: BitmapBundleMethods, O: ObjectMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bmp_normal: &B,
        bmp_disabled: &B2,
        kind: c_int,
        client_data: Option<&O>,
        short_help: &str,
        long_help: &str,
    ) -> *mut c_void {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bmp_normal = bmp_normal.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            let long_help = WxString::from(long_help);
            let long_help = long_help.as_ptr();
            ffi::wxToolBar_CreateTool(
                self.as_ptr(),
                tool_id,
                label,
                bmp_normal,
                bmp_disabled,
                kind,
                client_data,
                short_help,
                long_help,
            )
        }
    }
    fn create_tool_control<C: ControlMethods>(
        &self,
        control: Option<&C>,
        label: &str,
    ) -> *mut c_void {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxToolBar_CreateTool1(self.as_ptr(), control, label)
        }
    }
    fn create_separator(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_CreateSeparator(self.as_ptr()) }
    }
}

// wxToolTip
pub trait ToolTipMethods: ObjectMethods {
    fn get_tip(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxToolTip_GetTip(self.as_ptr())).into() }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxToolTip_GetWindow(self.as_ptr())) }
    }
    fn set_tip(&self, tip: &str) {
        unsafe {
            let tip = WxString::from(tip);
            let tip = tip.as_ptr();
            ffi::wxToolTip_SetTip(self.as_ptr(), tip)
        }
    }
    fn enable(flag: bool) {
        unsafe { ffi::wxToolTip_Enable(flag) }
    }
    fn set_auto_pop(msecs: c_long) {
        unsafe { ffi::wxToolTip_SetAutoPop(msecs) }
    }
    fn set_delay(msecs: c_long) {
        unsafe { ffi::wxToolTip_SetDelay(msecs) }
    }
    fn set_max_width(width: c_int) {
        unsafe { ffi::wxToolTip_SetMaxWidth(width) }
    }
    fn set_reshow(msecs: c_long) {
        unsafe { ffi::wxToolTip_SetReshow(msecs) }
    }
}

// wxToolbook
pub trait ToolbookMethods: BookCtrlBaseMethods {
    fn get_tool_bar(&self) -> *mut c_void {
        unsafe { ffi::wxToolbook_GetToolBar(self.as_ptr()) }
    }
    fn enable_page_sz(&self, page: usize, enable: bool) -> bool {
        unsafe { ffi::wxToolbook_EnablePage(self.as_ptr(), page, enable) }
    }
    fn enable_page_window<W: WindowMethods>(&self, page: Option<&W>, enable: bool) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolbook_EnablePage1(self.as_ptr(), page, enable)
        }
    }
}

// wxTopLevelWindow
pub trait TopLevelWindowMethods: NonOwnedWindowMethods {
    // DTOR: fn ~wxTopLevelWindow()
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
            ffi::wxTopLevelWindow_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    fn center_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.as_ptr(), direction) }
    }
    fn centre_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.as_ptr(), direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.as_ptr(), enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr(), enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr(), enable) }
    }
    fn get_default_item(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTopLevelWindow_GetDefaultItem(self.as_ptr())) }
    }
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxTopLevelWindow_GetIcon(self.as_ptr())) }
    }
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTopLevelWindow_GetTitle(self.as_ptr())).into() }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.as_ptr(), iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.as_ptr()) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr()) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(self.as_ptr()) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(self.as_ptr()) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(self.as_ptr()) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.as_ptr(), maximize) }
    }
    fn msw_get_system_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxTopLevelWindow_MSWGetSystemMenu(self.as_ptr())) }
    }
    fn request_user_attention(&self, flags: c_int) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.as_ptr(), flags) }
    }
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.as_ptr()) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<W: WindowMethods>(&self, win: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxTopLevelWindow_SetDefaultItem(self.as_ptr(), win))
        }
    }
    fn set_tmp_default_item<W: WindowMethods>(&self, win: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr(), win))
        }
    }
    fn get_tmp_default_item(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr())) }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxTopLevelWindow_SetIcon(self.as_ptr(), icon)
        }
    }
    fn set_icons<I: IconBundleMethods>(&self, icons: &I) {
        unsafe {
            let icons = icons.as_ptr();
            ffi::wxTopLevelWindow_SetIcons(self.as_ptr(), icons)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxTopLevelWindow_SetTitle(self.as_ptr(), title)
        }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr()) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.as_ptr(), modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(self.as_ptr()) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr(), filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr()) }
    }
    fn enable_full_screen_view(&self, enable: bool, style: c_long) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.as_ptr(), enable, style) }
    }
    fn show_full_screen(&self, show: bool, style: c_long) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.as_ptr(), show, style) }
    }
    // NOT_SUPPORTED: fn GetContentProtection()
    // NOT_SUPPORTED: fn SetContentProtection()
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    fn get_default_size() -> Size {
        unsafe { Size::from_ptr(ffi::wxTopLevelWindow_GetDefaultSize()) }
    }
}

// wxTreeCtrl
pub trait TreeCtrlMethods: ControlMethods {
    // DTOR: fn ~wxTreeCtrl()
    fn add_root<T: TreeItemDataMethods>(
        &self,
        text: &str,
        image: c_int,
        sel_image: c_int,
        data: Option<&T>,
    ) -> TreeItemId {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeItemId::from_ptr(ffi::wxTreeCtrl_AddRoot(
                self.as_ptr(),
                text,
                image,
                sel_image,
                data,
            ))
        }
    }
    fn append_item<T: TreeItemIdMethods, T2: TreeItemDataMethods>(
        &self,
        parent: &T,
        text: &str,
        image: c_int,
        sel_image: c_int,
        data: Option<&T2>,
    ) -> TreeItemId {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeItemId::from_ptr(ffi::wxTreeCtrl_AppendItem(
                self.as_ptr(),
                parent,
                text,
                image,
                sel_image,
                data,
            ))
        }
    }
    fn assign_buttons_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_AssignButtonsImageList(self.as_ptr(), image_list)
        }
    }
    fn assign_state_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_AssignStateImageList(self.as_ptr(), image_list)
        }
    }
    fn collapse<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Collapse(self.as_ptr(), item)
        }
    }
    fn collapse_all(&self) {
        unsafe { ffi::wxTreeCtrl_CollapseAll(self.as_ptr()) }
    }
    fn collapse_all_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_CollapseAllChildren(self.as_ptr(), item)
        }
    }
    fn collapse_and_reset<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_CollapseAndReset(self.as_ptr(), item)
        }
    }
    fn delete<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Delete(self.as_ptr(), item)
        }
    }
    fn delete_all_items(&self) {
        unsafe { ffi::wxTreeCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn delete_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_DeleteChildren(self.as_ptr(), item)
        }
    }
    fn edit_label<T: TreeItemIdMethods, C: ClassInfoMethods>(
        &self,
        item: &T,
        text_ctrl_class: Option<&C>,
    ) -> WeakRef<TextCtrl> {
        unsafe {
            let item = item.as_ptr();
            let text_ctrl_class = match text_ctrl_class {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<TextCtrl>::from(ffi::wxTreeCtrl_EditLabel(
                self.as_ptr(),
                item,
                text_ctrl_class,
            ))
        }
    }
    fn enable_bell_on_no_match(&self, on: bool) {
        unsafe { ffi::wxTreeCtrl_EnableBellOnNoMatch(self.as_ptr(), on) }
    }
    fn end_edit_label<T: TreeItemIdMethods>(&self, item: &T, discard_changes: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_EndEditLabel(self.as_ptr(), item, discard_changes)
        }
    }
    fn ensure_visible<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_EnsureVisible(self.as_ptr(), item)
        }
    }
    fn expand<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Expand(self.as_ptr(), item)
        }
    }
    fn expand_all(&self) {
        unsafe { ffi::wxTreeCtrl_ExpandAll(self.as_ptr()) }
    }
    fn expand_all_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ExpandAllChildren(self.as_ptr(), item)
        }
    }
    fn get_bounding_rect<T: TreeItemIdMethods, R: RectMethods>(
        &self,
        item: &T,
        rect: &R,
        text_only: bool,
    ) -> bool {
        unsafe {
            let item = item.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxTreeCtrl_GetBoundingRect(self.as_ptr(), item, rect, text_only)
        }
    }
    fn get_buttons_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxTreeCtrl_GetButtonsImageList(self.as_ptr())) }
    }
    fn get_children_count<T: TreeItemIdMethods>(&self, item: &T, recursively: bool) -> usize {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_GetChildrenCount(self.as_ptr(), item, recursively)
        }
    }
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxTreeCtrl_GetCount(self.as_ptr()) }
    }
    fn get_edit_control(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxTreeCtrl_GetEditControl(self.as_ptr())) }
    }
    fn get_first_child<T: TreeItemIdMethods>(&self, item: &T, cookie: *mut c_void) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetFirstChild(self.as_ptr(), item, cookie))
        }
    }
    fn get_first_visible_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetFirstVisibleItem(self.as_ptr())) }
    }
    fn get_focused_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetFocusedItem(self.as_ptr())) }
    }
    fn clear_focused_item(&self) {
        unsafe { ffi::wxTreeCtrl_ClearFocusedItem(self.as_ptr()) }
    }
    fn set_focused_item<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetFocusedItem(self.as_ptr(), item)
        }
    }
    fn get_indent(&self) -> c_uint {
        unsafe { ffi::wxTreeCtrl_GetIndent(self.as_ptr()) }
    }
    fn get_spacing(&self) -> c_uint {
        unsafe { ffi::wxTreeCtrl_GetSpacing(self.as_ptr()) }
    }
    fn get_item_background_colour<T: TreeItemIdMethods>(&self, item: &T) -> Colour {
        unsafe {
            let item = item.as_ptr();
            Colour::from_ptr(ffi::wxTreeCtrl_GetItemBackgroundColour(self.as_ptr(), item))
        }
    }
    fn get_item_data<T: TreeItemIdMethods>(&self, item: &T) -> Option<TreeItemDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            TreeItemData::option_from(ffi::wxTreeCtrl_GetItemData(self.as_ptr(), item))
        }
    }
    fn get_item_font<T: TreeItemIdMethods>(&self, item: &T) -> Font {
        unsafe {
            let item = item.as_ptr();
            Font::from_ptr(ffi::wxTreeCtrl_GetItemFont(self.as_ptr(), item))
        }
    }
    // NOT_SUPPORTED: fn GetItemImage()
    fn get_item_parent<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetItemParent(self.as_ptr(), item))
        }
    }
    fn get_item_state<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_GetItemState(self.as_ptr(), item)
        }
    }
    fn get_item_text<T: TreeItemIdMethods>(&self, item: &T) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxTreeCtrl_GetItemText(self.as_ptr(), item)).into()
        }
    }
    fn get_item_text_colour<T: TreeItemIdMethods>(&self, item: &T) -> Colour {
        unsafe {
            let item = item.as_ptr();
            Colour::from_ptr(ffi::wxTreeCtrl_GetItemTextColour(self.as_ptr(), item))
        }
    }
    fn get_last_child<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetLastChild(self.as_ptr(), item))
        }
    }
    fn get_next_child<T: TreeItemIdMethods>(&self, item: &T, cookie: *mut c_void) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetNextChild(self.as_ptr(), item, cookie))
        }
    }
    fn get_next_sibling_treeitemid<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetNextSibling(self.as_ptr(), item))
        }
    }
    fn get_next_visible<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetNextVisible(self.as_ptr(), item))
        }
    }
    fn get_prev_sibling_treeitemid<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetPrevSibling(self.as_ptr(), item))
        }
    }
    fn get_prev_visible<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetPrevVisible(self.as_ptr(), item))
        }
    }
    fn get_quick_best_size(&self) -> bool {
        unsafe { ffi::wxTreeCtrl_GetQuickBestSize(self.as_ptr()) }
    }
    fn get_root_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetRootItem(self.as_ptr())) }
    }
    fn get_selection(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetSelection(self.as_ptr())) }
    }
    fn get_selections(&self, selection: *mut c_void) -> usize {
        unsafe { ffi::wxTreeCtrl_GetSelections(self.as_ptr(), selection) }
    }
    fn get_state_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxTreeCtrl_GetStateImageList(self.as_ptr())) }
    }
    fn hit_test<P: PointMethods>(&self, point: &P, flags: *mut c_void) -> TreeItemId {
        unsafe {
            let point = point.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_HitTest(self.as_ptr(), point, flags))
        }
    }
    fn insert_item_treeitemid<
        T: TreeItemIdMethods,
        T2: TreeItemIdMethods,
        T3: TreeItemDataMethods,
    >(
        &self,
        parent: &T,
        previous: &T2,
        text: &str,
        image: c_int,
        sel_image: c_int,
        data: Option<&T3>,
    ) -> TreeItemId {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeItemId::from_ptr(ffi::wxTreeCtrl_InsertItem(
                self.as_ptr(),
                parent,
                previous,
                text,
                image,
                sel_image,
                data,
            ))
        }
    }
    fn insert_item_sz<T: TreeItemIdMethods, T2: TreeItemDataMethods>(
        &self,
        parent: &T,
        pos: usize,
        text: &str,
        image: c_int,
        sel_image: c_int,
        data: Option<&T2>,
    ) -> TreeItemId {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeItemId::from_ptr(ffi::wxTreeCtrl_InsertItem1(
                self.as_ptr(),
                parent,
                pos,
                text,
                image,
                sel_image,
                data,
            ))
        }
    }
    fn is_bold<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsBold(self.as_ptr(), item)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxTreeCtrl_IsEmpty(self.as_ptr()) }
    }
    fn is_expanded<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsExpanded(self.as_ptr(), item)
        }
    }
    fn is_selected<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsSelected(self.as_ptr(), item)
        }
    }
    fn is_visible<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsVisible(self.as_ptr(), item)
        }
    }
    fn item_has_children<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ItemHasChildren(self.as_ptr(), item)
        }
    }
    fn on_compare_items<T: TreeItemIdMethods, T2: TreeItemIdMethods>(
        &self,
        item1: &T,
        item2: &T2,
    ) -> c_int {
        unsafe {
            let item1 = item1.as_ptr();
            let item2 = item2.as_ptr();
            ffi::wxTreeCtrl_OnCompareItems(self.as_ptr(), item1, item2)
        }
    }
    fn prepend_item<T: TreeItemIdMethods, T2: TreeItemDataMethods>(
        &self,
        parent: &T,
        text: &str,
        image: c_int,
        sel_image: c_int,
        data: Option<&T2>,
    ) -> TreeItemId {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeItemId::from_ptr(ffi::wxTreeCtrl_PrependItem(
                self.as_ptr(),
                parent,
                text,
                image,
                sel_image,
                data,
            ))
        }
    }
    fn scroll_to<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ScrollTo(self.as_ptr(), item)
        }
    }
    fn select_item<T: TreeItemIdMethods>(&self, item: &T, select: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SelectItem(self.as_ptr(), item, select)
        }
    }
    fn set_buttons_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_SetButtonsImageList(self.as_ptr(), image_list)
        }
    }
    fn set_indent(&self, indent: c_uint) {
        unsafe { ffi::wxTreeCtrl_SetIndent(self.as_ptr(), indent) }
    }
    fn set_spacing(&self, spacing: c_uint) {
        unsafe { ffi::wxTreeCtrl_SetSpacing(self.as_ptr(), spacing) }
    }
    fn set_item_background_colour<T: TreeItemIdMethods, C: ColourMethods>(
        &self,
        item: &T,
        col: &C,
    ) {
        unsafe {
            let item = item.as_ptr();
            let col = col.as_ptr();
            ffi::wxTreeCtrl_SetItemBackgroundColour(self.as_ptr(), item, col)
        }
    }
    fn set_item_bold<T: TreeItemIdMethods>(&self, item: &T, bold: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemBold(self.as_ptr(), item, bold)
        }
    }
    fn set_item_data<T: TreeItemIdMethods, T2: TreeItemDataMethods>(
        &self,
        item: &T,
        data: Option<&T2>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_SetItemData(self.as_ptr(), item, data)
        }
    }
    fn set_item_drop_highlight<T: TreeItemIdMethods>(&self, item: &T, highlight: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemDropHighlight(self.as_ptr(), item, highlight)
        }
    }
    fn set_item_font<T: TreeItemIdMethods, F: FontMethods>(&self, item: &T, font: &F) {
        unsafe {
            let item = item.as_ptr();
            let font = font.as_ptr();
            ffi::wxTreeCtrl_SetItemFont(self.as_ptr(), item, font)
        }
    }
    fn set_item_has_children<T: TreeItemIdMethods>(&self, item: &T, has_children: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemHasChildren(self.as_ptr(), item, has_children)
        }
    }
    // NOT_SUPPORTED: fn SetItemImage()
    fn set_item_state<T: TreeItemIdMethods>(&self, item: &T, state: c_int) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemState(self.as_ptr(), item, state)
        }
    }
    fn set_item_text<T: TreeItemIdMethods>(&self, item: &T, text: &str) {
        unsafe {
            let item = item.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTreeCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
    fn set_item_text_colour<T: TreeItemIdMethods, C: ColourMethods>(&self, item: &T, col: &C) {
        unsafe {
            let item = item.as_ptr();
            let col = col.as_ptr();
            ffi::wxTreeCtrl_SetItemTextColour(self.as_ptr(), item, col)
        }
    }
    fn set_quick_best_size(&self, quick_best_size: bool) {
        unsafe { ffi::wxTreeCtrl_SetQuickBestSize(self.as_ptr(), quick_best_size) }
    }
    fn set_state_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_SetStateImageList(self.as_ptr(), image_list)
        }
    }
    fn sort_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SortChildren(self.as_ptr(), item)
        }
    }
    fn toggle<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Toggle(self.as_ptr(), item)
        }
    }
    fn toggle_item_selection<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ToggleItemSelection(self.as_ptr(), item)
        }
    }
    fn unselect(&self) {
        unsafe { ffi::wxTreeCtrl_Unselect(self.as_ptr()) }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxTreeCtrl_UnselectAll(self.as_ptr()) }
    }
    fn unselect_item<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_UnselectItem(self.as_ptr(), item)
        }
    }
    fn select_children<T: TreeItemIdMethods>(&self, parent: &T) {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxTreeCtrl_SelectChildren(self.as_ptr(), parent)
        }
    }
}

// wxTreeEvent
pub trait TreeEventMethods: NotifyEventMethods {
    fn get_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeEvent_GetItem(self.as_ptr())) }
    }
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxTreeEvent_GetKeyCode(self.as_ptr()) }
    }
    fn get_key_event(&self) -> KeyEventIsOwned<false> {
        unsafe { KeyEventIsOwned::from_ptr(ffi::wxTreeEvent_GetKeyEvent(self.as_ptr())) }
    }
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTreeEvent_GetLabel(self.as_ptr())).into() }
    }
    fn get_old_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeEvent_GetOldItem(self.as_ptr())) }
    }
    fn get_point(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxTreeEvent_GetPoint(self.as_ptr())) }
    }
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxTreeEvent_IsEditCancelled(self.as_ptr()) }
    }
    fn set_tool_tip(&self, tooltip: &str) {
        unsafe {
            let tooltip = WxString::from(tooltip);
            let tooltip = tooltip.as_ptr();
            ffi::wxTreeEvent_SetToolTip(self.as_ptr(), tooltip)
        }
    }
}

// wxTreeItemData
pub trait TreeItemDataMethods: ClientDataMethods {
    // DTOR: fn ~wxTreeItemData()
    fn get_id(&self) -> TreeItemIdIsOwned<false> {
        unsafe { TreeItemIdIsOwned::from_ptr(ffi::wxTreeItemData_GetId(self.as_ptr())) }
    }
    fn set_id<T: TreeItemIdMethods>(&self, id: &T) {
        unsafe {
            let id = id.as_ptr();
            ffi::wxTreeItemData_SetId(self.as_ptr(), id)
        }
    }
}

// wxTreeItemId
pub trait TreeItemIdMethods: WxRustMethods {
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxTreeItemId_IsOk(self.as_ptr()) }
    }
    fn get_id(&self) -> *mut c_void {
        unsafe { ffi::wxTreeItemId_GetID(self.as_ptr()) }
    }
    fn unset(&self) {
        unsafe { ffi::wxTreeItemId_Unset(self.as_ptr()) }
    }
}

// wxTreeListCtrl
pub trait TreeListCtrlMethods: WindowMethods {
    fn assign_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_AssignImageList(self.as_ptr(), image_list)
        }
    }
    fn set_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_SetImageList(self.as_ptr(), image_list)
        }
    }
    fn append_column(&self, title: &str, width: c_int, align: c_int, flags: c_int) -> c_int {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxTreeListCtrl_AppendColumn(self.as_ptr(), title, width, align, flags)
        }
    }
    // NOT_SUPPORTED: fn GetColumnCount()
    // NOT_SUPPORTED: fn DeleteColumn()
    fn clear_columns(&self) {
        unsafe { ffi::wxTreeListCtrl_ClearColumns(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetColumnWidth()
    // NOT_SUPPORTED: fn GetColumnWidth()
    fn width_for(&self, text: &str) -> c_int {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTreeListCtrl_WidthFor(self.as_ptr(), text)
        }
    }
    // BLOCKED: fn AppendItem()
    // BLOCKED: fn InsertItem()
    // BLOCKED: fn PrependItem()
    // BLOCKED: fn DeleteItem()
    fn delete_all_items(&self) {
        unsafe { ffi::wxTreeListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn get_root_item(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetRootItem(self.as_ptr())) }
    }
    // BLOCKED: fn GetItemParent()
    // BLOCKED: fn GetFirstChild()
    // BLOCKED: fn GetNextSibling()
    fn get_first_item(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetFirstItem(self.as_ptr())) }
    }
    // BLOCKED: fn GetNextItem()
    // NOT_SUPPORTED: fn GetItemText()
    // NOT_SUPPORTED: fn SetItemText()
    // BLOCKED: fn SetItemText1()
    // BLOCKED: fn SetItemImage()
    // BLOCKED: fn GetItemData()
    // BLOCKED: fn SetItemData()
    // BLOCKED: fn Expand()
    // BLOCKED: fn Collapse()
    // BLOCKED: fn IsExpanded()
    fn get_selection(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetSelection(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetSelections()
    // BLOCKED: fn Select()
    // BLOCKED: fn Unselect()
    // BLOCKED: fn IsSelected()
    fn select_all(&self) {
        unsafe { ffi::wxTreeListCtrl_SelectAll(self.as_ptr()) }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxTreeListCtrl_UnselectAll(self.as_ptr()) }
    }
    // BLOCKED: fn EnsureVisible()
    // BLOCKED: fn CheckItem()
    // BLOCKED: fn CheckItemRecursively()
    // BLOCKED: fn UncheckItem()
    // BLOCKED: fn UpdateItemParentStateRecursively()
    // BLOCKED: fn GetCheckedState()
    // BLOCKED: fn AreAllChildrenInState()
    // NOT_SUPPORTED: fn SetSortColumn()
    fn get_sort_column(&self, col: *mut c_void, ascending_order: *mut c_void) -> bool {
        unsafe { ffi::wxTreeListCtrl_GetSortColumn(self.as_ptr(), col, ascending_order) }
    }
    fn set_item_comparator<T: TreeListItemComparatorMethods>(&self, comparator: Option<&T>) {
        unsafe {
            let comparator = match comparator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_SetItemComparator(self.as_ptr(), comparator)
        }
    }
    fn get_view(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTreeListCtrl_GetView(self.as_ptr())) }
    }
    fn get_data_view(&self) -> WeakRef<DataViewCtrl> {
        unsafe { WeakRef::<DataViewCtrl>::from(ffi::wxTreeListCtrl_GetDataView(self.as_ptr())) }
    }
}

// wxTreeListItem
pub trait TreeListItemMethods: WxRustMethods {
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxTreeListItem_IsOk(self.as_ptr()) }
    }
}

// wxTreeListItemComparator
pub trait TreeListItemComparatorMethods: WxRustMethods {
    // NOT_SUPPORTED: fn Compare()
    // DTOR: fn ~wxTreeListItemComparator()
}

// wxTreebook
pub trait TreebookMethods: BookCtrlBaseMethods {
    // DTOR: fn ~wxTreebook()
    fn add_sub_page<W: WindowMethods>(
        &self,
        page: Option<&W>,
        text: &str,
        b_select: bool,
        image_id: c_int,
    ) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTreebook_AddSubPage(self.as_ptr(), page, text, b_select, image_id)
        }
    }
    fn collapse_node(&self, page_id: usize) -> bool {
        unsafe { ffi::wxTreebook_CollapseNode(self.as_ptr(), page_id) }
    }
    fn expand_node(&self, page_id: usize, expand: bool) -> bool {
        unsafe { ffi::wxTreebook_ExpandNode(self.as_ptr(), page_id, expand) }
    }
    fn get_page_parent(&self, page: usize) -> c_int {
        unsafe { ffi::wxTreebook_GetPageParent(self.as_ptr(), page) }
    }
    fn insert_sub_page<W: WindowMethods>(
        &self,
        page_pos: usize,
        page: Option<&W>,
        text: &str,
        b_select: bool,
        image_id: c_int,
    ) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTreebook_InsertSubPage(self.as_ptr(), page_pos, page, text, b_select, image_id)
        }
    }
    fn is_node_expanded(&self, page_id: usize) -> bool {
        unsafe { ffi::wxTreebook_IsNodeExpanded(self.as_ptr(), page_id) }
    }
}
