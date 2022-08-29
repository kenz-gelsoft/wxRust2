use super::*;

// wxTGAHandler
/// This is the image handler for the TGA format.
///
/// [See `wxTGAHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_t_g_a_handler.html)
pub trait TGAHandlerMethods: ImageHandlerMethods {}

// wxTIFFHandler
/// This is the image handler for the TIFF format.
///
/// [See `wxTIFFHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_t_i_f_f_handler.html)
pub trait TIFFHandlerMethods: ImageHandlerMethods {}

// wxTaskBarIcon
/// This class represents a taskbar icon.
///
/// [See `wxTaskBarIcon`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html)
pub trait TaskBarIconMethods: EvtHandlerMethods {
    // DTOR: fn ~wxTaskBarIcon()
    /// This method is similar to wxWindow::Destroy and can be used to schedule the task bar icon object for the delayed destruction: it will be deleted during the next event loop iteration, which allows the task bar icon to process any pending events for it before being destroyed.
    ///
    /// [See `wxTaskBarIcon::Destroy()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#a6f70b76af5364d616d67fb0610724780)
    fn destroy(&self) {
        unsafe { ffi::wxTaskBarIcon_Destroy(self.as_ptr()) }
    }
    /// Returns true if SetIcon() was called with no subsequent RemoveIcon().
    ///
    /// [See `wxTaskBarIcon::IsIconInstalled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#a12134ef096825bf7bf71297b8876dbd9)
    fn is_icon_installed(&self) -> bool {
        unsafe { ffi::wxTaskBarIcon_IsIconInstalled(self.as_ptr()) }
    }
    /// Returns true if the object initialized successfully.
    ///
    /// [See `wxTaskBarIcon::IsOk()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#a6956273d5188617b623f705bb7191d27)
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxTaskBarIcon_IsOk(self.as_ptr()) }
    }
    /// Pops up a menu at the current mouse position.
    ///
    /// [See `wxTaskBarIcon::PopupMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#a5f67b426e34077c690c49f4e6dcc6919)
    fn popup_menu<M: MenuMethods>(&self, menu: Option<&M>) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTaskBarIcon_PopupMenu(self.as_ptr(), menu)
        }
    }
    /// Removes the icon previously set with SetIcon().
    ///
    /// [See `wxTaskBarIcon::RemoveIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#ae2144fbd2527eff10fe4b7ec8e96ecdc)
    fn remove_icon(&self) -> bool {
        unsafe { ffi::wxTaskBarIcon_RemoveIcon(self.as_ptr()) }
    }
    /// Sets the icon, and optional tooltip text.
    ///
    /// [See `wxTaskBarIcon::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#a7a77fb5c59a7be91aa8954f582f0f8a1)
    fn set_icon<B: BitmapBundleMethods>(&self, icon: &B, tooltip: &str) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            let tooltip = WxString::from(tooltip);
            let tooltip = tooltip.as_ptr();
            ffi::wxTaskBarIcon_SetIcon(self.as_ptr(), icon, tooltip)
        }
    }
    /// Returns true if system tray is available in the desktop environment the app runs under.
    ///
    /// [See `wxTaskBarIcon::IsAvailable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon.html#a287bb3303f01651f50c8de17e314a147)
    fn is_available() -> bool {
        unsafe { ffi::wxTaskBarIcon_IsAvailable() }
    }
}

// wxTaskBarIconEvent
/// The event class used by wxTaskBarIcon.
///
/// [See `wxTaskBarIconEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_task_bar_icon_event.html)
pub trait TaskBarIconEventMethods: EventMethods {}

// wxTextAttr
/// wxTextAttr represents the character and paragraph attributes, or style, for a range of text in a wxTextCtrl or wxRichTextCtrl.
///
/// [See `wxTextAttr`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html)
pub trait TextAttrMethods: WxRustMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    /// Returns the background colour.
    ///
    /// [See `wxTextAttr::GetBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a8e4e02b11235e6a907c3dc79c2eaf459)
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetBackgroundColour(self.as_ptr())) }
    }
    /// Returns a string containing the name of the font associated with the bullet symbol.
    ///
    /// [See `wxTextAttr::GetBulletFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a0e8ae61c6d922a0a117280eed86f5cba)
    fn get_bullet_font(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetBulletFont(self.as_ptr())).into() }
    }
    /// Returns the standard bullet name, applicable if the bullet style is wxTEXT_ATTR_BULLET_STYLE_STANDARD.
    ///
    /// [See `wxTextAttr::GetBulletName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a0d7734a6ccfd2ef8941187ad3f14cbcc)
    fn get_bullet_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetBulletName(self.as_ptr())).into() }
    }
    /// Returns the bullet number.
    ///
    /// [See `wxTextAttr::GetBulletNumber()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a02eb1501badea0b1e87ea4c1532fa907)
    fn get_bullet_number(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetBulletNumber(self.as_ptr()) }
    }
    /// Returns the bullet style.
    ///
    /// [See `wxTextAttr::GetBulletStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#af01f5df0c258aae0e041c1008e0f9382)
    fn get_bullet_style(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetBulletStyle(self.as_ptr()) }
    }
    /// Returns the bullet text, which could be a symbol, or (for example) cached outline text.
    ///
    /// [See `wxTextAttr::GetBulletText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac1afc5716128bd268657522f0b73bdd7)
    fn get_bullet_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetBulletText(self.as_ptr())).into() }
    }
    /// Returns the name of the character style.
    ///
    /// [See `wxTextAttr::GetCharacterStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a315ee7c6b189327ab31c106b792662a1)
    fn get_character_style_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetCharacterStyleName(self.as_ptr())).into() }
    }
    /// Returns flags indicating which attributes are applicable.
    ///
    /// [See `wxTextAttr::GetFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a5e80b19e60410e0c1d57ab76b103128c)
    fn get_flags(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetFlags(self.as_ptr()) }
    }
    /// Creates and returns a font specified by the font attributes in the wxTextAttr object.
    ///
    /// [See `wxTextAttr::GetFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a77addc57112a3d7c474648771616d7a3)
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxTextAttr_GetFont(self.as_ptr())) }
    }
    /// Gets the font attributes from the given font, using only the attributes specified by flags.
    ///
    /// [See `wxTextAttr::GetFontAttributes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a1d7e71c2dbee4ceb320befbef52db965)
    fn get_font_attributes<F: FontMethods>(&self, font: &F, flags: c_int) -> bool {
        unsafe {
            let font = font.as_ptr();
            ffi::wxTextAttr_GetFontAttributes(self.as_ptr(), font, flags)
        }
    }
    // NOT_SUPPORTED: fn GetFontEncoding()
    /// Returns the font face name.
    ///
    /// [See `wxTextAttr::GetFontFaceName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a3f4a6cc1a0017c3bf5546d38c334a13d)
    fn get_font_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetFontFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFontFamily()
    /// Returns the font size in points.
    ///
    /// [See `wxTextAttr::GetFontSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ae48b943d09e4df7b0d05258931e8ff8e)
    fn get_font_size(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetFontSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontStyle()
    /// Returns true if the font is underlined.
    ///
    /// [See `wxTextAttr::GetFontUnderlined()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#af3b3bc97137977c7c1e3858fe30aa49f)
    fn get_font_underlined(&self) -> bool {
        unsafe { ffi::wxTextAttr_GetFontUnderlined(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetUnderlineType()
    /// Returns the underline color used.
    ///
    /// [See `wxTextAttr::GetUnderlineColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aa7d3a1bd71a4bd65c62d4e07c36bdb3a)
    fn get_underline_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetUnderlineColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetFontWeight()
    /// Returns the left indent in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::GetLeftIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a7ee8ef4eeea7c7c9d7f6c4351a78e61b)
    fn get_left_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetLeftIndent(self.as_ptr()) }
    }
    /// Returns the left sub-indent in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::GetLeftSubIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a4a9cd45f3aa6c2370f3b06ecc9b8e637)
    fn get_left_sub_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetLeftSubIndent(self.as_ptr()) }
    }
    /// Returns the line spacing value, one of wxTextAttrLineSpacing values.
    ///
    /// [See `wxTextAttr::GetLineSpacing()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac908a34938fa782acd2dafa6cc326e63)
    fn get_line_spacing(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetLineSpacing(self.as_ptr()) }
    }
    /// Returns the name of the list style.
    ///
    /// [See `wxTextAttr::GetListStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#afd4c9e6165d76f0ba4f651b34f73ba32)
    fn get_list_style_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetListStyleName(self.as_ptr())).into() }
    }
    /// Returns the outline level.
    ///
    /// [See `wxTextAttr::GetOutlineLevel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a71c4d9cdc2281c783f6d14ee6f597127)
    fn get_outline_level(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetOutlineLevel(self.as_ptr()) }
    }
    /// Returns the space in tenths of a millimeter after the paragraph.
    ///
    /// [See `wxTextAttr::GetParagraphSpacingAfter()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ab919f46fb92b00ce0d26233e65237250)
    fn get_paragraph_spacing_after(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetParagraphSpacingAfter(self.as_ptr()) }
    }
    /// Returns the space in tenths of a millimeter before the paragraph.
    ///
    /// [See `wxTextAttr::GetParagraphSpacingBefore()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a14e7ac14917bc3353f875e1273ede195)
    fn get_paragraph_spacing_before(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetParagraphSpacingBefore(self.as_ptr()) }
    }
    /// Returns the name of the paragraph style.
    ///
    /// [See `wxTextAttr::GetParagraphStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a3734989f897978b78b5680ebd53b424f)
    fn get_paragraph_style_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetParagraphStyleName(self.as_ptr())).into() }
    }
    /// Returns the right indent in tenths of a millimeter.
    ///
    /// [See `wxTextAttr::GetRightIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a231026bb23f90d0a52b56fdc32ab4cfe)
    fn get_right_indent(&self) -> c_long {
        unsafe { ffi::wxTextAttr_GetRightIndent(self.as_ptr()) }
    }
    /// Returns an array of tab stops, each expressed in tenths of a millimeter.
    ///
    /// [See `wxTextAttr::GetTabs()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a58d46d58d8bd430078d86360381d30f1)
    fn get_tabs(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxTextAttr_GetTabs(self.as_ptr())) }
    }
    /// Returns the text foreground colour.
    ///
    /// [See `wxTextAttr::GetTextColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a3ee332af4536a29a894d202a532f9f89)
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxTextAttr_GetTextColour(self.as_ptr())) }
    }
    /// Returns the text effect bits of interest.
    ///
    /// [See `wxTextAttr::GetTextEffectFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ad111329eb3c7db34ec4727834f1c570a)
    fn get_text_effect_flags(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetTextEffectFlags(self.as_ptr()) }
    }
    /// Returns the text effects, a bit list of styles.
    ///
    /// [See `wxTextAttr::GetTextEffects()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a9d2638af4d70f9a9bd7ba740032bffa5)
    fn get_text_effects(&self) -> c_int {
        unsafe { ffi::wxTextAttr_GetTextEffects(self.as_ptr()) }
    }
    /// Returns the URL for the content.
    ///
    /// [See `wxTextAttr::GetURL()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a97d58feb54d09cf3ea03a2754f4e5af5)
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextAttr_GetURL(self.as_ptr())).into() }
    }
    /// Returns true if the attribute object specifies alignment.
    ///
    /// [See `wxTextAttr::HasAlignment()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a811af45f03c213b05884c3db0e99566d)
    fn has_alignment(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasAlignment(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a background colour.
    ///
    /// [See `wxTextAttr::HasBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a11906776a4ea17da2b7cdd7cfe666c19)
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBackgroundColour(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a standard bullet name.
    ///
    /// [See `wxTextAttr::HasBulletName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aecd64878adea5b06d70372d30d94952c)
    fn has_bullet_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletName(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a bullet number.
    ///
    /// [See `wxTextAttr::HasBulletNumber()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a9950fbc6c9dca6fdaf20e93579a3914b)
    fn has_bullet_number(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletNumber(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a bullet style.
    ///
    /// [See `wxTextAttr::HasBulletStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a21c5629c4ad6af42b779266be2b7e772)
    fn has_bullet_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletStyle(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies bullet text (usually specifying a symbol).
    ///
    /// [See `wxTextAttr::HasBulletText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a6d76728cbf4ca1f5a63a3ba2051882e2)
    fn has_bullet_text(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasBulletText(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a character style name.
    ///
    /// [See `wxTextAttr::HasCharacterStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a2642afa237a1f435b1b903456d420f61)
    fn has_character_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasCharacterStyleName(self.as_ptr()) }
    }
    /// Returns true if the flag is present in the attribute object's flag bitlist.
    ///
    /// [See `wxTextAttr::HasFlag()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a2a04e327ad7baa9541a5364f30f40ba3)
    fn has_flag(&self, flag: c_long) -> bool {
        unsafe { ffi::wxTextAttr_HasFlag(self.as_ptr(), flag) }
    }
    /// Returns true if the attribute object specifies any font attributes.
    ///
    /// [See `wxTextAttr::HasFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#abd60e04738892bf95b19357267454d05)
    fn has_font(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFont(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies an encoding.
    ///
    /// [See `wxTextAttr::HasFontEncoding()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a592574138c768142684651257c2fe6fe)
    fn has_font_encoding(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontEncoding(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a font face name.
    ///
    /// [See `wxTextAttr::HasFontFaceName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ad56a9936370c194bdb57aeaba3af0150)
    fn has_font_face_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontFaceName(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a font family.
    ///
    /// [See `wxTextAttr::HasFontFamily()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a6144fac535a5b2aefbe53cb903519ec6)
    fn has_font_family(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontFamily(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies italic style.
    ///
    /// [See `wxTextAttr::HasFontItalic()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ace03c90a8d5260242ea5ca4ac33497ac)
    fn has_font_italic(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontItalic(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a font point or pixel size.
    ///
    /// [See `wxTextAttr::HasFontSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a494331f1a74f3fad1b6dc73ffe0d3d23)
    fn has_font_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontSize(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a font point size.
    ///
    /// [See `wxTextAttr::HasFontPointSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ab0690abb3b671a2bbbd523e85668de90)
    fn has_font_point_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontPointSize(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a font pixel size.
    ///
    /// [See `wxTextAttr::HasFontPixelSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aa8e274c8233e617c6dd9ae3714e33758)
    fn has_font_pixel_size(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontPixelSize(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies either underlining or no underlining.
    ///
    /// [See `wxTextAttr::HasFontUnderlined()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac5726748a70161324b89beb8f90ccfc9)
    fn has_font_underlined(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontUnderlined(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies font weight (bold, light or normal).
    ///
    /// [See `wxTextAttr::HasFontWeight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a67ad510446a330c251dd14741d6b4bdf)
    fn has_font_weight(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasFontWeight(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a left indent.
    ///
    /// [See `wxTextAttr::HasLeftIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aae2a52b53e6a7571e4f33edd3b997a52)
    fn has_left_indent(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasLeftIndent(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies line spacing.
    ///
    /// [See `wxTextAttr::HasLineSpacing()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a5ca224c927865add3af7a3e6d224e791)
    fn has_line_spacing(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasLineSpacing(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a list style name.
    ///
    /// [See `wxTextAttr::HasListStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac2307f68de4e58a8e66c3323b1615f0b)
    fn has_list_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasListStyleName(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies an outline level.
    ///
    /// [See `wxTextAttr::HasOutlineLevel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a25d81120bdfed25f55f82eea2446b504)
    fn has_outline_level(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasOutlineLevel(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a page break before this paragraph.
    ///
    /// [See `wxTextAttr::HasPageBreak()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ae9ee4ecc100fe8bc9506e3c8b10e86fe)
    fn has_page_break(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasPageBreak(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies spacing after a paragraph.
    ///
    /// [See `wxTextAttr::HasParagraphSpacingAfter()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aa09f0c32acc98ff5db3256538782038c)
    fn has_paragraph_spacing_after(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphSpacingAfter(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies spacing before a paragraph.
    ///
    /// [See `wxTextAttr::HasParagraphSpacingBefore()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ae5e7d21ac5ff421c1432a9d03fe204c7)
    fn has_paragraph_spacing_before(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphSpacingBefore(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a paragraph style name.
    ///
    /// [See `wxTextAttr::HasParagraphStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a998e6f5c580a08773c397ecfa8afad62)
    fn has_paragraph_style_name(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasParagraphStyleName(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a right indent.
    ///
    /// [See `wxTextAttr::HasRightIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac351274da47a699500209d807d6d2e86)
    fn has_right_indent(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasRightIndent(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies tab stops.
    ///
    /// [See `wxTextAttr::HasTabs()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aee1562b0ca9aac4f585fce2b3d3e01f1)
    fn has_tabs(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTabs(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a text foreground colour.
    ///
    /// [See `wxTextAttr::HasTextColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a271b460ea7a59eb33b16d454089a1888)
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTextColour(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies text effects.
    ///
    /// [See `wxTextAttr::HasTextEffects()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a1c53ce88e7f9b693f096ca32488f89dd)
    fn has_text_effects(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasTextEffects(self.as_ptr()) }
    }
    /// Returns true if the attribute object specifies a URL.
    ///
    /// [See `wxTextAttr::HasURL()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a4aa1da1de9259cd08afd259ba608b666)
    fn has_url(&self) -> bool {
        unsafe { ffi::wxTextAttr_HasURL(self.as_ptr()) }
    }
    /// Returns true if the object represents a character style, that is, the flags specify a font or a text background or foreground colour.
    ///
    /// [See `wxTextAttr::IsCharacterStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac7ad29aaefb48ca5b157fc59e6a82279)
    fn is_character_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsCharacterStyle(self.as_ptr()) }
    }
    /// Returns false if we have any attributes set, true otherwise.
    ///
    /// [See `wxTextAttr::IsDefault()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a1889daa0cac5498416c927bf3234d5f6)
    fn is_default(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsDefault(self.as_ptr()) }
    }
    /// Returns true if the object represents a paragraph style, that is, the flags specify alignment, indentation, tabs, paragraph spacing, or bullet style.
    ///
    /// [See `wxTextAttr::IsParagraphStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a59dd96564776775d6f9b8d77eb99b71b)
    fn is_paragraph_style(&self) -> bool {
        unsafe { ffi::wxTextAttr_IsParagraphStyle(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    /// Sets the background colour.
    ///
    /// [See `wxTextAttr::SetBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a3d904c5febccbd613c732715df6cd21a)
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxTextAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    /// Sets the name of the font associated with the bullet symbol.
    ///
    /// [See `wxTextAttr::SetBulletFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a274986a65fd546e4218fa6a44aae7598)
    fn set_bullet_font(&self, font: &str) {
        unsafe {
            let font = WxString::from(font);
            let font = font.as_ptr();
            ffi::wxTextAttr_SetBulletFont(self.as_ptr(), font)
        }
    }
    /// Sets the standard bullet name, applicable if the bullet style is wxTEXT_ATTR_BULLET_STYLE_STANDARD.
    ///
    /// [See `wxTextAttr::SetBulletName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a3b8e03167ff4decd0191c5bff1772edd)
    fn set_bullet_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetBulletName(self.as_ptr(), name)
        }
    }
    /// Sets the bullet number.
    ///
    /// [See `wxTextAttr::SetBulletNumber()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ad6ac93c015e64cc539d319b69b8cc5da)
    fn set_bullet_number(&self, n: c_int) {
        unsafe { ffi::wxTextAttr_SetBulletNumber(self.as_ptr(), n) }
    }
    /// Sets the bullet style.
    ///
    /// [See `wxTextAttr::SetBulletStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a90b42b4243c71f3c9a7eb5a2bbdbbb3f)
    fn set_bullet_style(&self, style: c_int) {
        unsafe { ffi::wxTextAttr_SetBulletStyle(self.as_ptr(), style) }
    }
    /// Sets the bullet text, which could be a symbol, or (for example) cached outline text.
    ///
    /// [See `wxTextAttr::SetBulletText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ab11b0110911a8fe9784da56080ae1da2)
    fn set_bullet_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTextAttr_SetBulletText(self.as_ptr(), text)
        }
    }
    /// Sets the character style name.
    ///
    /// [See `wxTextAttr::SetCharacterStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ac1bff41f48307ba58f2cc5713ca10bd3)
    fn set_character_style_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetCharacterStyleName(self.as_ptr(), name)
        }
    }
    /// Sets the flags determining which styles are being specified.
    ///
    /// [See `wxTextAttr::SetFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a315ebbc5a41840a0777a7716fc19f5f0)
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxTextAttr_SetFlags(self.as_ptr(), flags) }
    }
    /// Sets the attributes for the given font.
    ///
    /// [See `wxTextAttr::SetFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aafa9ced6fc22a506b827636be3cefd00)
    fn set_font<F: FontMethods>(&self, font: &F, flags: c_int) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxTextAttr_SetFont(self.as_ptr(), font, flags)
        }
    }
    // NOT_SUPPORTED: fn SetFontEncoding()
    /// Sets the font face name.
    ///
    /// [See `wxTextAttr::SetFontFaceName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a5465b67ad2924cbc4730a6c2343e6562)
    fn set_font_face_name(&self, face_name: &str) {
        unsafe {
            let face_name = WxString::from(face_name);
            let face_name = face_name.as_ptr();
            ffi::wxTextAttr_SetFontFaceName(self.as_ptr(), face_name)
        }
    }
    // NOT_SUPPORTED: fn SetFontFamily()
    /// Sets the font size in points.
    ///
    /// [See `wxTextAttr::SetFontSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a914d5bcb7725c7893d6fb938d9bf5e85)
    fn set_font_size(&self, point_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontSize(self.as_ptr(), point_size) }
    }
    /// Sets the font size in points.
    ///
    /// [See `wxTextAttr::SetFontPointSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ad184b2eef5ff569c7a4d95bd7ff994d6)
    fn set_font_point_size(&self, point_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontPointSize(self.as_ptr(), point_size) }
    }
    /// Sets the font size in pixels.
    ///
    /// [See `wxTextAttr::SetFontPixelSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aec4a709e538976888c5db8a43f2e1685)
    fn set_font_pixel_size(&self, pixel_size: c_int) {
        unsafe { ffi::wxTextAttr_SetFontPixelSize(self.as_ptr(), pixel_size) }
    }
    // NOT_SUPPORTED: fn SetFontStyle()
    /// Sets the font underlining (solid line, text colour).
    ///
    /// [See `wxTextAttr::SetFontUnderlined()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a9bdb4ac861b4892d57069d976c552fdc)
    fn set_font_underlined(&self, underlined: bool) {
        unsafe { ffi::wxTextAttr_SetFontUnderlined(self.as_ptr(), underlined) }
    }
    // NOT_SUPPORTED: fn SetFontUnderlined1()
    // NOT_SUPPORTED: fn SetFontWeight()
    /// Sets the left indent and left subindent in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::SetLeftIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a090d97a13d48d9455d5d9e449f7a4072)
    fn set_left_indent(&self, indent: c_int, sub_indent: c_int) {
        unsafe { ffi::wxTextAttr_SetLeftIndent(self.as_ptr(), indent, sub_indent) }
    }
    /// Sets the line spacing.
    ///
    /// [See `wxTextAttr::SetLineSpacing()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a469fe33ac4acd9f83a00d621c2bd6cf6)
    fn set_line_spacing(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetLineSpacing(self.as_ptr(), spacing) }
    }
    /// Sets the list style name.
    ///
    /// [See `wxTextAttr::SetListStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a9b7939948ed563c3ab0c02aa832cc743)
    fn set_list_style_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetListStyleName(self.as_ptr(), name)
        }
    }
    /// Specifies the outline level.
    ///
    /// [See `wxTextAttr::SetOutlineLevel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a24d5f8a632dcd1a09eb3910207816963)
    fn set_outline_level(&self, level: c_int) {
        unsafe { ffi::wxTextAttr_SetOutlineLevel(self.as_ptr(), level) }
    }
    /// Specifies a page break before this paragraph.
    ///
    /// [See `wxTextAttr::SetPageBreak()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a47bf936ecf8d900eb6b5798cd45d8474)
    fn set_page_break(&self, page_break: bool) {
        unsafe { ffi::wxTextAttr_SetPageBreak(self.as_ptr(), page_break) }
    }
    /// Sets the spacing after a paragraph, in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::SetParagraphSpacingAfter()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aaf87a0f02c53ded0a2b8a34e6ff6ff1c)
    fn set_paragraph_spacing_after(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetParagraphSpacingAfter(self.as_ptr(), spacing) }
    }
    /// Sets the spacing before a paragraph, in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::SetParagraphSpacingBefore()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a78f2cfaa80010b4470b42404d5fa7cb8)
    fn set_paragraph_spacing_before(&self, spacing: c_int) {
        unsafe { ffi::wxTextAttr_SetParagraphSpacingBefore(self.as_ptr(), spacing) }
    }
    /// Sets the name of the paragraph style.
    ///
    /// [See `wxTextAttr::SetParagraphStyleName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#adfa8e1c11e933576314546316d28e37d)
    fn set_paragraph_style_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxTextAttr_SetParagraphStyleName(self.as_ptr(), name)
        }
    }
    /// Sets the right indent in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::SetRightIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a7fd4f5eb19852c6908ca173c4e350887)
    fn set_right_indent(&self, indent: c_int) {
        unsafe { ffi::wxTextAttr_SetRightIndent(self.as_ptr(), indent) }
    }
    /// Sets the tab stops, expressed in tenths of a millimetre.
    ///
    /// [See `wxTextAttr::SetTabs()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ad21e66c498c5a60e3603d71938b8c772)
    fn set_tabs<A: ArrayIntMethods>(&self, tabs: &A) {
        unsafe {
            let tabs = tabs.as_ptr();
            ffi::wxTextAttr_SetTabs(self.as_ptr(), tabs)
        }
    }
    /// Sets the text foreground colour.
    ///
    /// [See `wxTextAttr::SetTextColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ae3d74ce2e7057146911eb0360d451082)
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxTextAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    /// Sets the text effect bits of interest.
    ///
    /// [See `wxTextAttr::SetTextEffectFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aaa361390da3d932aaa6a1c67875aff02)
    fn set_text_effect_flags(&self, flags: c_int) {
        unsafe { ffi::wxTextAttr_SetTextEffectFlags(self.as_ptr(), flags) }
    }
    /// Sets the text effects, a bit list of styles.
    ///
    /// [See `wxTextAttr::SetTextEffects()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a218521c1b968212fc97e474dd5a563ef)
    fn set_text_effects(&self, effects: c_int) {
        unsafe { ffi::wxTextAttr_SetTextEffects(self.as_ptr(), effects) }
    }
    /// Sets the URL for the content.
    ///
    /// [See `wxTextAttr::SetURL()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#aad62bd5f3d6f1b9ea78360f06947a8c5)
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxTextAttr_SetURL(self.as_ptr(), url)
        }
    }
    // BLOCKED: fn operator=()
    /// Applies the attributes in style to the original object, but not those attributes from style that are the same as those in compareWith (if passed).
    ///
    /// [See `wxTextAttr::Apply()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a9601bfb3fb57a4595f122617c525d9fc)
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
    /// Copies all defined/valid properties from overlay to current object.
    ///
    /// [See `wxTextAttr::Merge()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a591d90d4a675dcbdcd57bbef5557669b)
    fn merge<T: TextAttrMethods>(&self, overlay: &T) {
        unsafe {
            let overlay = overlay.as_ptr();
            ffi::wxTextAttr_Merge(self.as_ptr(), overlay)
        }
    }
    /// Partial equality test.
    ///
    /// [See `wxTextAttr::EqPartial()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#a7a479bbcae7149fc2aa69001d7f934ad)
    fn eq_partial<T: TextAttrMethods>(&self, attr: &T, weak_test: bool) -> bool {
        unsafe {
            let attr = attr.as_ptr();
            ffi::wxTextAttr_EqPartial(self.as_ptr(), attr, weak_test)
        }
    }
    /// Creates a new wxTextAttr which is a merge of base and overlay.
    ///
    /// [See `wxTextAttr::Merge()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_attr.html#ad51cefa334bbfa1bdcff33c05bf0c6ef)
    fn merge_textattr<T: TextAttrMethods, T2: TextAttrMethods>(base: &T, overlay: &T2) -> TextAttr {
        unsafe {
            let base = base.as_ptr();
            let overlay = overlay.as_ptr();
            TextAttr::from_ptr(ffi::wxTextAttr_Merge1(base, overlay))
        }
    }
}

// wxTextCtrl
/// A text control allows text to be displayed and edited.
///
/// [See `wxTextCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html)
pub trait TextCtrlMethods: ControlMethods {
    /// Enable the automatic replacement of new lines characters in a single-line text field with spaces under macOS.
    ///
    /// [See `wxTextCtrl::OSXEnableNewLineReplacement()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a83a2cb6c159119696b0aee341afb90bc)
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
    /// Creates the text control for two-step construction.
    ///
    /// [See `wxTextCtrl::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a4216ca88aa1610edc00c69e26eb92fbd)
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
    /// Resets the internal modified flag as if the current changes had been saved.
    ///
    /// [See `wxTextCtrl::DiscardEdits()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#ab9653877885738aeacacc731e0b759d5)
    fn discard_edits(&self) {
        unsafe { ffi::wxTextCtrl_DiscardEdits(self.as_ptr()) }
    }
    /// Delete the undo history.
    ///
    /// [See `wxTextCtrl::EmptyUndoBuffer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a4699805aa84d45cfbad44ba29fe303dc)
    fn empty_undo_buffer(&self) {
        unsafe { ffi::wxTextCtrl_EmptyUndoBuffer(self.as_ptr()) }
    }
    /// This function inserts into the control the character which would have been inserted if the given key event had occurred in the text control.
    ///
    /// [See `wxTextCtrl::EmulateKeyPress()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#aac3e7f2124aafd67df6f6efbc285a40b)
    fn emulate_key_press<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxTextCtrl_EmulateKeyPress(self.as_ptr(), event)
        }
    }
    /// Enable or disable native spell checking on this text control if it is available on the current platform.
    ///
    /// [See `wxTextCtrl::EnableProofCheck()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#ae59dab77ddefd0df3b3b2e6495c43653)
    fn enable_proof_check(&self, options: *const c_void) -> bool {
        unsafe { ffi::wxTextCtrl_EnableProofCheck(self.as_ptr(), options) }
    }
    /// Returns the style currently used for the new text.
    ///
    /// [See `wxTextCtrl::GetDefaultStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a922bfcaf404dbaff10837c80b40744bf)
    fn get_default_style(&self) -> TextAttrIsOwned<false> {
        unsafe { TextAttrIsOwned::from_ptr(ffi::wxTextCtrl_GetDefaultStyle(self.as_ptr())) }
    }
    /// Gets the length of the specified line, not including any trailing newline character(s).
    ///
    /// [See `wxTextCtrl::GetLineLength()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a26e637b26508d7e97c209d22db32efb6)
    fn get_line_length(&self, line_no: c_long) -> c_int {
        unsafe { ffi::wxTextCtrl_GetLineLength(self.as_ptr(), line_no) }
    }
    /// Returns the contents of a given line in the text control, not including any trailing newline character(s).
    ///
    /// [See `wxTextCtrl::GetLineText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#afaa68cea1c13d82d5e98e99d60d5cc81)
    fn get_line_text(&self, line_no: c_long) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextCtrl_GetLineText(self.as_ptr(), line_no)).into() }
    }
    /// Returns the number of lines in the text control buffer.
    ///
    /// [See `wxTextCtrl::GetNumberOfLines()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#acc92cde8ae6d479f30dd4f1e877f7a16)
    fn get_number_of_lines(&self) -> c_int {
        unsafe { ffi::wxTextCtrl_GetNumberOfLines(self.as_ptr()) }
    }
    /// Returns the style at this position in the text control.
    ///
    /// [See `wxTextCtrl::GetStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a762bc2bc4875c45776a6df10a88d1b1d)
    fn get_style<T: TextAttrMethods>(&self, position: c_long, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_GetStyle(self.as_ptr(), position, style)
        }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    /// Returns true if the text has been modified by user.
    ///
    /// [See `wxTextCtrl::IsModified()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a08ad1dc6f4dbb77ecd60d9e1e61b1bd6)
    fn is_modified(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsModified(self.as_ptr()) }
    }
    /// Returns true if this is a multi line edit control and false otherwise.
    ///
    /// [See `wxTextCtrl::IsMultiLine()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a39ce8a2dfdb3510752e42e0e908f7710)
    fn is_multi_line(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsMultiLine(self.as_ptr()) }
    }
    /// Returns true if this is a single line edit control and false otherwise.
    ///
    /// [See `wxTextCtrl::IsSingleLine()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a021dd82ebcec882520e65baa9ac9be1f)
    fn is_single_line(&self) -> bool {
        unsafe { ffi::wxTextCtrl_IsSingleLine(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetProofCheckOptions()
    /// Loads and displays the named file, if it exists.
    ///
    /// [See `wxTextCtrl::LoadFile()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a40e67be01fca0933542d471aadae04fd)
    fn load_file(&self, filename: &str, file_type: c_int) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxTextCtrl_LoadFile(self.as_ptr(), filename, file_type)
        }
    }
    /// Mark text as modified (dirty).
    ///
    /// [See `wxTextCtrl::MarkDirty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#ab89671b5cd9e2da9a978f14e0f9931cf)
    fn mark_dirty(&self) {
        unsafe { ffi::wxTextCtrl_MarkDirty(self.as_ptr()) }
    }
    /// This event handler function implements default drag and drop behaviour, which is to load the first dropped file into the control.
    ///
    /// [See `wxTextCtrl::OnDropFiles()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#aaa898f4c8d488e17201555ba02411e53)
    fn on_drop_files<D: DropFilesEventMethods>(&self, event: &D) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxTextCtrl_OnDropFiles(self.as_ptr(), event)
        }
    }
    /// Converts given position to a zero-based column, line number pair.
    ///
    /// [See `wxTextCtrl::PositionToXY()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a898c3d89764271cf8bee160e2d2bbf18)
    fn position_to_xy(&self, pos: c_long, x: *mut c_void, y: *mut c_void) -> bool {
        unsafe { ffi::wxTextCtrl_PositionToXY(self.as_ptr(), pos, x, y) }
    }
    /// Converts given text position to client coordinates in pixels.
    ///
    /// [See `wxTextCtrl::PositionToCoords()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#ac7e8926d052dd0dd7210cebab061237e)
    fn position_to_coords(&self, pos: c_long) -> Point {
        unsafe { Point::from_ptr(ffi::wxTextCtrl_PositionToCoords(self.as_ptr(), pos)) }
    }
    /// Saves the contents of the control in a text file.
    ///
    /// [See `wxTextCtrl::SaveFile()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a5cedd577ca31598235244b992229e411)
    fn save_file(&self, filename: &str, file_type: c_int) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxTextCtrl_SaveFile(self.as_ptr(), filename, file_type)
        }
    }
    /// Changes the default style to use for the new text which is going to be added to the control.
    ///
    /// [See `wxTextCtrl::SetDefaultStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a1c0ead0e3a2faa1bc16b1898e418f9e0)
    fn set_default_style<T: TextAttrMethods>(&self, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_SetDefaultStyle(self.as_ptr(), style)
        }
    }
    /// Marks the control as being modified by the user or not.
    ///
    /// [See `wxTextCtrl::SetModified()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a73a48def8fe76ef303d7adc332d71ce7)
    fn set_modified(&self, modified: bool) {
        unsafe { ffi::wxTextCtrl_SetModified(self.as_ptr(), modified) }
    }
    /// Changes the style of the given range.
    ///
    /// [See `wxTextCtrl::SetStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a0d972d3007e23ed9270d16b6bb326e80)
    fn set_style<T: TextAttrMethods>(&self, start: c_long, end: c_long, style: &T) -> bool {
        unsafe {
            let style = style.as_ptr();
            ffi::wxTextCtrl_SetStyle(self.as_ptr(), start, end, style)
        }
    }
    /// Makes the line containing the given position visible.
    ///
    /// [See `wxTextCtrl::ShowPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#ad4044ce304755d06fcbe5bd432b56783)
    fn show_position(&self, pos: c_long) {
        unsafe { ffi::wxTextCtrl_ShowPosition(self.as_ptr(), pos) }
    }
    /// Converts the given zero based column and line number to a position.
    ///
    /// [See `wxTextCtrl::XYToPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_ctrl.html#a535bbd06fd08beee9055d5c05946b484)
    fn xy_to_position(&self, x: c_long, y: c_long) -> c_long {
        unsafe { ffi::wxTextCtrl_XYToPosition(self.as_ptr(), x, y) }
    }
}

// wxTextDataObject
/// wxTextDataObject is a specialization of wxDataObjectSimple for text data.
///
/// [See `wxTextDataObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html)
pub trait TextDataObjectMethods: DataObjectSimpleMethods {
    /// Returns the text associated with the data object.
    ///
    /// [See `wxTextDataObject::GetText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html#a7dcbc7745b8cc16cdaedd3bddc40a7fb)
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextDataObject_GetText(self.as_ptr())).into() }
    }
    /// Returns the data size.
    ///
    /// [See `wxTextDataObject::GetTextLength()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html#a13bf9f66e8ab5628edcaf5d8773b5a32)
    fn get_text_length(&self) -> usize {
        unsafe { ffi::wxTextDataObject_GetTextLength(self.as_ptr()) }
    }
    /// Sets the text associated with the data object.
    ///
    /// [See `wxTextDataObject::SetText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_data_object.html#ac6754f3ad536bd0fa99928604ec48e7a)
    fn set_text(&self, str_text: &str) {
        unsafe {
            let str_text = WxString::from(str_text);
            let str_text = str_text.as_ptr();
            ffi::wxTextDataObject_SetText(self.as_ptr(), str_text)
        }
    }
}

// wxTextDropTarget
/// A predefined drop target for dealing with text data.
///
/// [See `wxTextDropTarget`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_drop_target.html)
pub trait TextDropTargetMethods: DropTargetMethods {
    /// Override this function to receive dropped text.
    ///
    /// [See `wxTextDropTarget::OnDropText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_drop_target.html#a91ba08b0cf219b402e97fb608bc9e852)
    fn on_drop_text(&self, x: c_int, y: c_int, data: &str) -> bool {
        unsafe {
            let data = WxString::from(data);
            let data = data.as_ptr();
            ffi::wxTextDropTarget_OnDropText(self.as_ptr(), x, y, data)
        }
    }
}

// wxTextEntry
/// Common base class for single line text entry fields.
///
/// [See `wxTextEntry`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html)
pub trait TextEntryMethods: WxRustMethods {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    /// Appends the text to the end of the text control.
    ///
    /// [See `wxTextEntry::AppendText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#af80b5a51906ca9c65fa6cdaa9640768b)
    fn append_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTextEntry_AppendText(self.as_text_entry(), text)
        }
    }
    /// Call this function to enable auto-completion of the text typed in a single-line text control using the given choices.
    ///
    /// [See `wxTextEntry::AutoComplete()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ad911d59d6b381a20b0a6c34df2ac1d9f)
    fn auto_complete_arraystring<A: ArrayStringMethods>(&self, choices: &A) -> bool {
        unsafe {
            let choices = choices.as_ptr();
            ffi::wxTextEntry_AutoComplete(self.as_text_entry(), choices)
        }
    }
    /// Enable auto-completion using the provided completer object.
    ///
    /// [See `wxTextEntry::AutoComplete()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ae8ca40185ba6bbaacb4715039d73342b)
    fn auto_complete_textcompleter(&self, completer: *mut c_void) -> bool {
        unsafe { ffi::wxTextEntry_AutoComplete1(self.as_text_entry(), completer) }
    }
    /// Call this function to enable auto-completion of the text typed in a single-line text control using all valid file system paths.
    ///
    /// [See `wxTextEntry::AutoCompleteFileNames()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ad40d7e35d8bb9c9ab8e4ffa1b801a5d5)
    fn auto_complete_file_names(&self) -> bool {
        unsafe { ffi::wxTextEntry_AutoCompleteFileNames(self.as_text_entry()) }
    }
    /// Call this function to enable auto-completion of the text using the file system directories.
    ///
    /// [See `wxTextEntry::AutoCompleteDirectories()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ab02338d68d51f103551454298578851c)
    fn auto_complete_directories(&self) -> bool {
        unsafe { ffi::wxTextEntry_AutoCompleteDirectories(self.as_text_entry()) }
    }
    /// Returns true if the selection can be copied to the clipboard.
    ///
    /// [See `wxTextEntry::CanCopy()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a620eaa062c530c194dd3a079991c1167)
    fn can_copy(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanCopy(self.as_text_entry()) }
    }
    /// Returns true if the selection can be cut to the clipboard.
    ///
    /// [See `wxTextEntry::CanCut()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a5f680c7b00d718f295e09288d3a16b66)
    fn can_cut(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanCut(self.as_text_entry()) }
    }
    /// Returns true if the contents of the clipboard can be pasted into the text control.
    ///
    /// [See `wxTextEntry::CanPaste()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ae03fe0c746bd7230fc02eb7df8d7650c)
    fn can_paste(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanPaste(self.as_text_entry()) }
    }
    /// Returns true if there is a redo facility available and the last operation can be redone.
    ///
    /// [See `wxTextEntry::CanRedo()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ab99240d67d8c82511ea3954a1c68ca77)
    fn can_redo(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanRedo(self.as_text_entry()) }
    }
    /// Returns true if there is an undo facility available and the last operation can be undone.
    ///
    /// [See `wxTextEntry::CanUndo()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a7c7e533085c5e4cf61450ccf2ea7a5ed)
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxTextEntry_CanUndo(self.as_text_entry()) }
    }
    /// Sets the new text control value.
    ///
    /// [See `wxTextEntry::ChangeValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a8c52ab71f51c8f80556c2c8e763cbca1)
    fn change_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntry_ChangeValue(self.as_text_entry(), value)
        }
    }
    /// Clears the text in the control.
    ///
    /// [See `wxTextEntry::Clear()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a98b8e3fab8a5ac9944c2cefbb09ab3a7)
    fn clear(&self) {
        unsafe { ffi::wxTextEntry_Clear(self.as_text_entry()) }
    }
    /// Copies the selected text to the clipboard.
    ///
    /// [See `wxTextEntry::Copy()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a94675646b91a2d5a18e3ca7395cafe77)
    fn copy(&self) {
        unsafe { ffi::wxTextEntry_Copy(self.as_text_entry()) }
    }
    /// Copies the selected text to the clipboard and removes it from the control.
    ///
    /// [See `wxTextEntry::Cut()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a571332a18ed5cdecd76a259ee97ae5a1)
    fn cut(&self) {
        unsafe { ffi::wxTextEntry_Cut(self.as_text_entry()) }
    }
    /// Convert all text entered into the control to upper case.
    ///
    /// [See `wxTextEntry::ForceUpper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a19199a1f6c9a94095dad2a93f846ed12)
    fn force_upper(&self) {
        unsafe { ffi::wxTextEntry_ForceUpper(self.as_text_entry()) }
    }
    /// Returns the insertion point, or cursor, position.
    ///
    /// [See `wxTextEntry::GetInsertionPoint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#aec1a523f158549abdef5271c55906902)
    fn get_insertion_point(&self) -> c_long {
        unsafe { ffi::wxTextEntry_GetInsertionPoint(self.as_text_entry()) }
    }
    // NOT_SUPPORTED: fn GetLastPosition()
    /// Returns the string containing the text starting in the positions from and up to to in the control.
    ///
    /// [See `wxTextEntry::GetRange()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ae568ccd892e44bf89f9918f9ed187af3)
    fn get_range(&self, from: c_long, to: c_long) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxTextEntry_GetRange(self.as_text_entry(), from, to)).into()
        }
    }
    /// Gets the current selection span.
    ///
    /// [See `wxTextEntry::GetSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#af8d6ff4ff8376bb762987069a69b984b)
    fn get_selection_long(&self, from: *mut c_void, to: *mut c_void) {
        unsafe { ffi::wxTextEntry_GetSelection(self.as_text_entry(), from, to) }
    }
    /// Gets the text currently selected in the control.
    ///
    /// [See `wxTextEntry::GetStringSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#adac8cae1eef10e198d3f8777d91b2607)
    fn get_string_selection(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxTextEntry_GetStringSelection(self.as_text_entry())).into()
        }
    }
    /// Gets the contents of the control.
    ///
    /// [See `wxTextEntry::GetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a39335d9009b2053b5daf850c7b9d2974)
    fn get_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextEntry_GetValue(self.as_text_entry())).into() }
    }
    /// Returns true if the controls contents may be edited by user (note that it always can be changed by the program).
    ///
    /// [See `wxTextEntry::IsEditable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ac7191f01863c91397ac844883e588b43)
    fn is_editable(&self) -> bool {
        unsafe { ffi::wxTextEntry_IsEditable(self.as_text_entry()) }
    }
    /// Returns true if the control is currently empty.
    ///
    /// [See `wxTextEntry::IsEmpty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ad8353636538ddf3552a074e597b14c2d)
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxTextEntry_IsEmpty(self.as_text_entry()) }
    }
    /// Pastes text from the clipboard to the text item.
    ///
    /// [See `wxTextEntry::Paste()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a5889d0d240e603e32c11bd580b65b3d7)
    fn paste(&self) {
        unsafe { ffi::wxTextEntry_Paste(self.as_text_entry()) }
    }
    /// If there is a redo facility and the last operation can be redone, redoes the last operation.
    ///
    /// [See `wxTextEntry::Redo()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a3aa0d89ddb8a922c941818dafe4852a4)
    fn redo(&self) {
        unsafe { ffi::wxTextEntry_Redo(self.as_text_entry()) }
    }
    /// Removes the text starting at the first given position up to (but not including) the character at the last position.
    ///
    /// [See `wxTextEntry::Remove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ab38f2786becabdacf27c7e31a2922bcc)
    fn remove(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxTextEntry_Remove(self.as_text_entry(), from, to) }
    }
    /// Replaces the text starting at the first position up to (but not including) the character at the last position with the given text.
    ///
    /// [See `wxTextEntry::Replace()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a1fb3ac76d270b2c64cff595497815f8d)
    fn replace(&self, from: c_long, to: c_long, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntry_Replace(self.as_text_entry(), from, to, value)
        }
    }
    /// Makes the text item editable or read-only, overriding the wxTE_READONLY flag.
    ///
    /// [See `wxTextEntry::SetEditable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a7d95c0f42b5e1dc0559ae1ec56cb8b86)
    fn set_editable(&self, editable: bool) {
        unsafe { ffi::wxTextEntry_SetEditable(self.as_text_entry(), editable) }
    }
    /// Sets the insertion point at the given position.
    ///
    /// [See `wxTextEntry::SetInsertionPoint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a6e5460ec6e893ecb3e3ce90300373de8)
    fn set_insertion_point(&self, pos: c_long) {
        unsafe { ffi::wxTextEntry_SetInsertionPoint(self.as_text_entry(), pos) }
    }
    /// Sets the insertion point at the end of the text control.
    ///
    /// [See `wxTextEntry::SetInsertionPointEnd()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a954a065a2f20da350ae830faff1fff95)
    fn set_insertion_point_end(&self) {
        unsafe { ffi::wxTextEntry_SetInsertionPointEnd(self.as_text_entry()) }
    }
    // NOT_SUPPORTED: fn SetMaxLength()
    /// Selects the text starting at the first position up to (but not including) the character at the last position.
    ///
    /// [See `wxTextEntry::SetSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#af7e298bc2a34bd646328f53efab766aa)
    fn set_selection_long(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxTextEntry_SetSelection(self.as_text_entry(), from, to) }
    }
    /// Selects all text in the control.
    ///
    /// [See `wxTextEntry::SelectAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a2f7cb6fe4a1c2d1cd79edec6391ea91e)
    fn select_all(&self) {
        unsafe { ffi::wxTextEntry_SelectAll(self.as_text_entry()) }
    }
    /// Deselects selected text in the control.
    ///
    /// [See `wxTextEntry::SelectNone()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a7ab161521fee2982118f109cfeaa4f22)
    fn select_none(&self) {
        unsafe { ffi::wxTextEntry_SelectNone(self.as_text_entry()) }
    }
    /// Sets a hint shown in an empty unfocused text control.
    ///
    /// [See `wxTextEntry::SetHint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a4e9dfe958dbd1918c54b45be83f1bed4)
    fn set_hint(&self, hint: &str) -> bool {
        unsafe {
            let hint = WxString::from(hint);
            let hint = hint.as_ptr();
            ffi::wxTextEntry_SetHint(self.as_text_entry(), hint)
        }
    }
    /// Returns the current hint string.
    ///
    /// [See `wxTextEntry::GetHint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ae05d111ea78f08ee38d1b404c3dcf9ae)
    fn get_hint(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextEntry_GetHint(self.as_text_entry())).into() }
    }
    /// Attempts to set the control margins.
    ///
    /// [See `wxTextEntry::SetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#af2f9684123d3f4d7233945016b2d5c1d)
    fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxTextEntry_SetMargins(self.as_text_entry(), pt)
        }
    }
    ///
    /// [See `wxTextEntry::SetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ad8dc1eb9633c73d26c968b337525c6c2)
    fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
        unsafe { ffi::wxTextEntry_SetMargins1(self.as_text_entry(), left, top) }
    }
    /// Returns the margins used by the control.
    ///
    /// [See `wxTextEntry::GetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#aff48d044d76f1cc8708bd9abb5968fa8)
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxTextEntry_GetMargins(self.as_text_entry())) }
    }
    /// Sets the new text control value.
    ///
    /// [See `wxTextEntry::SetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#a90f876b2dd83ba5c97ba0c193b386e9f)
    fn set_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntry_SetValue(self.as_text_entry(), value)
        }
    }
    /// If there is an undo facility and the last operation can be undone, undoes the last operation.
    ///
    /// [See `wxTextEntry::Undo()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#ad3784d539a2554c6eec76c80aa6d97de)
    fn undo(&self) {
        unsafe { ffi::wxTextEntry_Undo(self.as_text_entry()) }
    }
    /// Writes the text into the text control at the current insertion position.
    ///
    /// [See `wxTextEntry::WriteText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry.html#aa1b9419f95878c44234ff812b528c17b)
    fn write_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTextEntry_WriteText(self.as_text_entry(), text)
        }
    }
}

// wxTextEntryDialog
/// This class represents a dialog that requests a one-line text string from the user.
///
/// [See `wxTextEntryDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html)
pub trait TextEntryDialogMethods: DialogMethods {
    ///
    /// [See `wxTextEntryDialog::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#a765b5d5cfca07be990d787a381c165f3)
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
    /// Returns the text that the user has entered if the user has pressed OK, or the original value if the user has pressed Cancel.
    ///
    /// [See `wxTextEntryDialog::GetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#ae9fb8eb071f15a9aba096470f2fd09c8)
    fn get_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextEntryDialog_GetValue(self.as_ptr())).into() }
    }
    /// Associate a validator with the text control used by the dialog.
    ///
    /// [See `wxTextEntryDialog::SetTextValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#a6ced03c03673b4d74b64596825d3a5a3)
    fn set_text_validator<T: TextValidatorMethods>(&self, validator: &T) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxTextEntryDialog_SetTextValidator(self.as_ptr(), validator)
        }
    }
    // NOT_SUPPORTED: fn SetTextValidator1()
    // NOT_SUPPORTED: fn SetMaxLength()
    /// Sets the default text value.
    ///
    /// [See `wxTextEntryDialog::SetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#a50a247a73ac092ffc46fd2a526f8d376)
    fn set_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxTextEntryDialog_SetValue(self.as_ptr(), value)
        }
    }
    /// Convert all text entered into the text control used by the dialog to upper case.
    ///
    /// [See `wxTextEntryDialog::ForceUpper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_entry_dialog.html#a8d03287c3fd94689bc9a5c6ca4ed6452)
    fn force_upper(&self) {
        unsafe { ffi::wxTextEntryDialog_ForceUpper(self.as_ptr()) }
    }
}

// wxTextValidator
/// wxTextValidator validates text controls, providing a variety of filtering behaviours.
///
/// [See `wxTextValidator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html)
pub trait TextValidatorMethods: ValidatorMethods {
    /// Returns a copy of the exclude char list (the list of invalid characters).
    ///
    /// [See `wxTextValidator::GetCharExcludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a62c6db9d30541b1720a942eb1fb82972)
    fn get_char_excludes(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextValidator_GetCharExcludes(self.as_ptr())).into() }
    }
    /// Returns a copy of the include char list (the list of additional valid characters).
    ///
    /// [See `wxTextValidator::GetCharIncludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a1fba4cb2eb69546a1ac7fb7235ea2b3e)
    fn get_char_includes(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTextValidator_GetCharIncludes(self.as_ptr())).into() }
    }
    /// Returns a const reference to the exclude list (the list of invalid values).
    ///
    /// [See `wxTextValidator::GetExcludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a470219e1cac3953dc2ff015e43891665)
    fn get_excludes(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxTextValidator_GetExcludes(self.as_ptr())) }
    }
    /// Returns a const reference to the include list (the list of valid values).
    ///
    /// [See `wxTextValidator::GetIncludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a5466e57dff6e7edcd0957ac7a2150beb)
    fn get_includes(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxTextValidator_GetIncludes(self.as_ptr())) }
    }
    /// Returns the validator style.
    ///
    /// [See `wxTextValidator::GetStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a5410e686b5a2a70d6c7e15bc8055188c)
    fn get_style(&self) -> c_long {
        unsafe { ffi::wxTextValidator_GetStyle(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HasFlag()
    /// Receives character input from the window and filters it according to the current validator style.
    ///
    /// [See `wxTextValidator::OnChar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a125b03603df03e2d45d6bbb3ec079499)
    fn on_char<K: KeyEventMethods>(&self, event: &K) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxTextValidator_OnChar(self.as_ptr(), event)
        }
    }
    /// Sets the exclude list (invalid values for the user input).
    ///
    /// [See `wxTextValidator::SetExcludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#aca577f30a7bb9dfec8a5ae6e11413891)
    fn set_excludes<A: ArrayStringMethods>(&self, string_list: &A) {
        unsafe {
            let string_list = string_list.as_ptr();
            ffi::wxTextValidator_SetExcludes(self.as_ptr(), string_list)
        }
    }
    /// Sets the exclude char list (invalid characters for the user input).
    ///
    /// [See `wxTextValidator::SetCharExcludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a8a461648fd10e0ce15021e3d913f24d4)
    fn set_char_excludes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_SetCharExcludes(self.as_ptr(), chars)
        }
    }
    /// Sets the include list (valid values for the user input).
    ///
    /// [See `wxTextValidator::SetIncludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a82e11147d76becf3c1dd5805b92f5b74)
    fn set_includes<A: ArrayStringMethods>(&self, string_list: &A) {
        unsafe {
            let string_list = string_list.as_ptr();
            ffi::wxTextValidator_SetIncludes(self.as_ptr(), string_list)
        }
    }
    /// Sets the include char list (additional valid values for the user input).
    ///
    /// [See `wxTextValidator::SetCharIncludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a73b9a9a92761b34503777381226d30a3)
    fn set_char_includes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_SetCharIncludes(self.as_ptr(), chars)
        }
    }
    /// Adds exclude to the list of excluded values.
    ///
    /// [See `wxTextValidator::AddExclude()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#ab4bab3f3d26d9874d93aebcbe8572847)
    fn add_exclude(&self, exclude: &str) {
        unsafe {
            let exclude = WxString::from(exclude);
            let exclude = exclude.as_ptr();
            ffi::wxTextValidator_AddExclude(self.as_ptr(), exclude)
        }
    }
    /// Adds include to the list of included values.
    ///
    /// [See `wxTextValidator::AddInclude()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a6324e041079dc0c6a64ea0bc5639333c)
    fn add_include(&self, include: &str) {
        unsafe {
            let include = WxString::from(include);
            let include = include.as_ptr();
            ffi::wxTextValidator_AddInclude(self.as_ptr(), include)
        }
    }
    /// Adds chars to the list of excluded characters.
    ///
    /// [See `wxTextValidator::AddCharExcludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a5841025e9d33a204c1c63ac0e35a6350)
    fn add_char_excludes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_AddCharExcludes(self.as_ptr(), chars)
        }
    }
    /// Adds chars to the list of included characters.
    ///
    /// [See `wxTextValidator::AddCharIncludes()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#ab8bec6a5a5a1b7c33d8aa369ceefa6ce)
    fn add_char_includes(&self, chars: &str) {
        unsafe {
            let chars = WxString::from(chars);
            let chars = chars.as_ptr();
            ffi::wxTextValidator_AddCharIncludes(self.as_ptr(), chars)
        }
    }
    /// Sets the validator style which must be a combination of one or more of the wxTextValidatorStyle values.
    ///
    /// [See `wxTextValidator::SetStyle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a30d9deab4413db1039539dc7c062f9fa)
    fn set_style(&self, style: c_long) {
        unsafe { ffi::wxTextValidator_SetStyle(self.as_ptr(), style) }
    }
    /// Returns the error message if the contents of val are invalid or the empty string if val is valid.
    ///
    /// [See `wxTextValidator::IsValid()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_text_validator.html#a77dd17dbc8f6c23a23add534051d8129)
    fn is_valid(&self, val: &str) -> String {
        unsafe {
            let val = WxString::from(val);
            let val = val.as_ptr();
            WxString::from_ptr(ffi::wxTextValidator_IsValid(self.as_ptr(), val)).into()
        }
    }
}

// wxThreadEvent
/// This class adds some simple functionality to wxEvent to facilitate inter-thread communication.
///
/// [See `wxThreadEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html)
pub trait ThreadEventMethods: EventMethods {
    // BLOCKED: fn SetPayload()
    // NOT_SUPPORTED: fn GetPayload()
    /// Returns extra information integer value.
    ///
    /// [See `wxThreadEvent::GetExtraLong()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html#a82a12193461d981861a8d9c4b6706a15)
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxThreadEvent_GetExtraLong(self.as_ptr()) }
    }
    /// Returns stored integer value.
    ///
    /// [See `wxThreadEvent::GetInt()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html#a7da703415668fd4d715f25c678f1ff33)
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxThreadEvent_GetInt(self.as_ptr()) }
    }
    /// Returns stored string value.
    ///
    /// [See `wxThreadEvent::GetString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html#afe47d1a134a5119d489ac6d1cda5dd22)
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxThreadEvent_GetString(self.as_ptr())).into() }
    }
    /// Sets the extra information value.
    ///
    /// [See `wxThreadEvent::SetExtraLong()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html#a177554d331bfd911de8d9abcc35660e9)
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxThreadEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    /// Sets the integer value.
    ///
    /// [See `wxThreadEvent::SetInt()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html#a6f3846d1860ddbb5fe8f7cdaa5f3c1fa)
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxThreadEvent_SetInt(self.as_ptr(), int_command) }
    }
    /// Sets the string value.
    ///
    /// [See `wxThreadEvent::SetString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_thread_event.html#a72092085fc4e0837d8e56666cb45f4d3)
    fn set_string(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxThreadEvent_SetString(self.as_ptr(), string)
        }
    }
}

// wxTimePickerCtrl
/// This control allows the user to enter time.
///
/// [See `wxTimePickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html)
pub trait TimePickerCtrlMethods: ControlMethods {
    /// Create the control window.
    ///
    /// [See `wxTimePickerCtrl::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#a7a5895f0173dc6556e3a15c26e783704)
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
    /// Returns the currently entered time as hours, minutes and seconds.
    ///
    /// [See `wxTimePickerCtrl::GetTime()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#ac54166395c084e3fbb3cf652fc219323)
    fn get_time(&self, hour: *mut c_void, min: *mut c_void, sec: *mut c_void) -> bool {
        unsafe { ffi::wxTimePickerCtrl_GetTime(self.as_ptr(), hour, min, sec) }
    }
    /// Returns the currently entered time.
    ///
    /// [See `wxTimePickerCtrl::GetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#a55a817cc6fdb4287f5dc8e67024e0f40)
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxTimePickerCtrl_GetValue(self.as_ptr())) }
    }
    /// Changes the current time of the control.
    ///
    /// [See `wxTimePickerCtrl::SetTime()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#a361ddb73bd005759c1f9acff7433d098)
    fn set_time(&self, hour: c_int, min: c_int, sec: c_int) -> bool {
        unsafe { ffi::wxTimePickerCtrl_SetTime(self.as_ptr(), hour, min, sec) }
    }
    /// Changes the current value of the control.
    ///
    /// [See `wxTimePickerCtrl::SetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_time_picker_ctrl.html#aa1369853e4cc0f71191ddbca1e1e39a1)
    fn set_value<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxTimePickerCtrl_SetValue(self.as_ptr(), dt)
        }
    }
}

// wxTipProvider
/// This is the class used together with wxShowTip() function.
///
/// [See `wxTipProvider`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html)
pub trait TipProviderMethods: WxRustMethods {
    // DTOR: fn ~wxTipProvider()
    /// Returns the index of the current tip (i.e. the one which would be returned by GetTip()).
    ///
    /// [See `wxTipProvider::GetCurrentTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html#a5fa87aa0139640587613a2f3c80d0b3e)
    fn get_current_tip(&self) -> usize {
        unsafe { ffi::wxTipProvider_GetCurrentTip(self.as_ptr()) }
    }
    /// Return the text of the current tip and pass to the next one.
    ///
    /// [See `wxTipProvider::GetTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_provider.html#a82f7aea7a77b67707bc265bab365eb49)
    fn get_tip(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTipProvider_GetTip(self.as_ptr())).into() }
    }
}

// wxTipWindow
/// Shows simple text in a popup tip window on creation.
///
/// [See `wxTipWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_window.html)
pub trait TipWindowMethods: WindowMethods {
    /// By default, the tip window disappears when the user clicks the mouse or presses a keyboard key or if it loses focus in any other way - for example because the user switched to another application window.
    ///
    /// [See `wxTipWindow::SetBoundingRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_window.html#ab4d4159ecae1da6f70c86521ecb8af6a)
    fn set_bounding_rect<R: RectMethods>(&self, rect_bound: &R) {
        unsafe {
            let rect_bound = rect_bound.as_ptr();
            ffi::wxTipWindow_SetBoundingRect(self.as_ptr(), rect_bound)
        }
    }
    /// When the tip window closes itself (which may happen at any moment and unexpectedly to the caller) it may NULL out the pointer pointed to by windowPtr.
    ///
    /// [See `wxTipWindow::SetTipWindowPtr()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tip_window.html#ac70d4d8ef1082c316f9b1a1a978c93d1)
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
/// wxToggleButton is a button that stays pressed when clicked by the user.
///
/// [See `wxToggleButton`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html)
pub trait ToggleButtonMethods: AnyButtonMethods {
    // DTOR: fn ~wxToggleButton()
    /// Creates the toggle button for two-step construction.
    ///
    /// [See `wxToggleButton::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#a1a965baed27860a6c04829a85c7efcdb)
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
    /// Gets the state of the toggle button.
    ///
    /// [See `wxToggleButton::GetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#a470729e8ec01eae3993741631c8ffa85)
    fn get_value(&self) -> bool {
        unsafe { ffi::wxToggleButton_GetValue(self.as_ptr()) }
    }
    /// Sets the toggle button to the given state.
    ///
    /// [See `wxToggleButton::SetValue()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toggle_button.html#ab75d4ad3bbbbf5a827c5aed373ff26db)
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxToggleButton_SetValue(self.as_ptr(), state) }
    }
}

// wxToolBar
/// A toolbar is a bar of buttons and/or other controls usually placed below the menu bar in a wxFrame.
///
/// [See `wxToolBar`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html)
pub trait ToolBarMethods: ControlMethods {
    // DTOR: fn ~wxToolBar()
    /// Adds a new check (or toggle) tool to the toolbar.
    ///
    /// [See `wxToolBar::AddCheckTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a583ea04a37600fbea6156fdb1a28ef45)
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
    /// Adds any control to the toolbar, typically e.g. a wxComboBox.
    ///
    /// [See `wxToolBar::AddControl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a8d72ff220adeba82d48185cdea316834)
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
    /// Adds a new radio tool to the toolbar.
    ///
    /// [See `wxToolBar::AddRadioTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#aef47fe780e81fb5128aad3929c9e77c9)
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
    /// Adds a separator for spacing groups of tools.
    ///
    /// [See `wxToolBar::AddSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a12ec2524b5f4a236b4e880813315fb14)
    fn add_separator(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddSeparator(self.as_ptr()) }
    }
    /// Adds a stretchable space to the toolbar.
    ///
    /// [See `wxToolBar::AddStretchableSpace()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#aa7cd894f82105782c4847dd0a8a35914)
    fn add_stretchable_space(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddStretchableSpace(self.as_ptr()) }
    }
    /// Adds a tool to the toolbar.
    ///
    /// [See `wxToolBar::AddTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a47f5a68ad5f1182ab241dd3abfe7ac6c)
    fn add_tool_toolbartoolbase(&self, tool: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxToolBar_AddTool(self.as_ptr(), tool) }
    }
    /// Adds a tool to the toolbar.
    ///
    /// [See `wxToolBar::AddTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#abefedb5432f9c53da9c7c97c7dfaa092)
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
    /// Adds a tool to the toolbar.
    ///
    /// [See `wxToolBar::AddTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ad46bf8ee6e8c603670ad8d90be728c04)
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
    /// Deletes all the tools in the toolbar.
    ///
    /// [See `wxToolBar::ClearTools()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ad431696ec829540e2f9889588f6589a5)
    fn clear_tools(&self) {
        unsafe { ffi::wxToolBar_ClearTools(self.as_ptr()) }
    }
    /// Removes the specified tool from the toolbar and deletes it.
    ///
    /// [See `wxToolBar::DeleteTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#aea3c91f9b3d7505e4b511f678883c73d)
    fn delete_tool(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_DeleteTool(self.as_ptr(), tool_id) }
    }
    /// This function behaves like DeleteTool() but it deletes the tool at the specified position and not the one with the given id.
    ///
    /// [See `wxToolBar::DeleteToolByPos()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a441f430c309c1fa8e43803fe09c2fa25)
    fn delete_tool_by_pos(&self, pos: usize) -> bool {
        unsafe { ffi::wxToolBar_DeleteToolByPos(self.as_ptr(), pos) }
    }
    /// Enables or disables the tool.
    ///
    /// [See `wxToolBar::EnableTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ad6b3481a2a6d71cad7b0923adf15c9d0)
    fn enable_tool(&self, tool_id: c_int, enable: bool) {
        unsafe { ffi::wxToolBar_EnableTool(self.as_ptr(), tool_id, enable) }
    }
    /// Returns a pointer to the tool identified by id or NULL if no corresponding tool is found.
    ///
    /// [See `wxToolBar::FindById()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ad86c6f25ec98723091a6665e180eaf6d)
    fn find_by_id(&self, id: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_FindById(self.as_ptr(), id) }
    }
    /// Returns a pointer to the control identified by id or NULL if no corresponding control is found.
    ///
    /// [See `wxToolBar::FindControl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ad0f91a97e94b2f8cb3f22b21ff3ef778)
    fn find_control(&self, id: c_int) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxToolBar_FindControl(self.as_ptr(), id)) }
    }
    /// Finds a tool for the given mouse position.
    ///
    /// [See `wxToolBar::FindToolForPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a7fc94c044ce9a2773bd4cceb49d86e1e)
    fn find_tool_for_position(&self, x: c_int, y: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_FindToolForPosition(self.as_ptr(), x, y) }
    }
    /// Returns the left/right and top/bottom margins, which are also used for inter-toolspacing.
    ///
    /// [See `wxToolBar::GetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a3aae6e73af007736a5194604e9e21017)
    fn get_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetMargins(self.as_ptr())) }
    }
    /// Returns the size of bitmap that the toolbar expects to have.
    ///
    /// [See `wxToolBar::GetToolBitmapSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a57443a2b2a10eeef84a4f139f7ff6636)
    fn get_tool_bitmap_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetToolBitmapSize(self.as_ptr())) }
    }
    // BLOCKED: fn GetToolByPos()
    ///
    /// [See `wxToolBar::GetToolByPos()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a3ee0a2ac30adce5aff479ec8a22299ce)
    fn get_tool_by_pos(&self, pos: c_int) -> *const c_void {
        unsafe { ffi::wxToolBar_GetToolByPos1(self.as_ptr(), pos) }
    }
    /// Get any client data associated with the tool.
    ///
    /// [See `wxToolBar::GetToolClientData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a7b4027b1a5f6e91f356a7bc4dc024e32)
    fn get_tool_client_data(&self, tool_id: c_int) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxToolBar_GetToolClientData(self.as_ptr(), tool_id)) }
    }
    /// Called to determine whether a tool is enabled (responds to user input).
    ///
    /// [See `wxToolBar::GetToolEnabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a3546d1a97a87c5a93f6e16f2eee01505)
    fn get_tool_enabled(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_GetToolEnabled(self.as_ptr(), tool_id) }
    }
    /// Returns the long help for the given tool.
    ///
    /// [See `wxToolBar::GetToolLongHelp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a40730e2e72a303c703fe456c0fa73321)
    fn get_tool_long_help(&self, tool_id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxToolBar_GetToolLongHelp(self.as_ptr(), tool_id)).into() }
    }
    /// Returns the value used for packing tools.
    ///
    /// [See `wxToolBar::GetToolPacking()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a7dd9ebb2753306f8a53685e1f674a30f)
    fn get_tool_packing(&self) -> c_int {
        unsafe { ffi::wxToolBar_GetToolPacking(self.as_ptr()) }
    }
    /// Returns the tool position in the toolbar, or wxNOT_FOUND if the tool is not found.
    ///
    /// [See `wxToolBar::GetToolPos()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a47f1fa0520fac74fb4def2987efbb304)
    fn get_tool_pos(&self, tool_id: c_int) -> c_int {
        unsafe { ffi::wxToolBar_GetToolPos(self.as_ptr(), tool_id) }
    }
    /// Returns the default separator size.
    ///
    /// [See `wxToolBar::GetToolSeparation()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a7e37ff492283f3c650edaf2e4c3479c3)
    fn get_tool_separation(&self) -> c_int {
        unsafe { ffi::wxToolBar_GetToolSeparation(self.as_ptr()) }
    }
    /// Returns the short help for the given tool.
    ///
    /// [See `wxToolBar::GetToolShortHelp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ac87ac63a3f1958a57481474c49b1584b)
    fn get_tool_short_help(&self, tool_id: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxToolBar_GetToolShortHelp(self.as_ptr(), tool_id)).into()
        }
    }
    /// Returns the size of a whole button, which is usually larger than a tool bitmap because of added 3D effects.
    ///
    /// [See `wxToolBar::GetToolSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ac394cec1c8511caec6dd635f8d62db51)
    fn get_tool_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetToolSize(self.as_ptr())) }
    }
    /// Gets the on/off state of a toggle tool.
    ///
    /// [See `wxToolBar::GetToolState()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a67e571b18871d9ee4a88ffc98ecea9ed)
    fn get_tool_state(&self, tool_id: c_int) -> bool {
        unsafe { ffi::wxToolBar_GetToolState(self.as_ptr(), tool_id) }
    }
    /// Returns the number of tools in the toolbar.
    ///
    /// [See `wxToolBar::GetToolsCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a3aa14e58881f2f2631a23a3afab2adc1)
    fn get_tools_count(&self) -> usize {
        unsafe { ffi::wxToolBar_GetToolsCount(self.as_ptr()) }
    }
    /// Inserts the control into the toolbar at the given position.
    ///
    /// [See `wxToolBar::InsertControl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a1167c3e201fe769b78b183539933aef1)
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
    /// Inserts the separator into the toolbar at the given position.
    ///
    /// [See `wxToolBar::InsertSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#aa8c20c9271c613f5da787afefddddf9d)
    fn insert_separator(&self, pos: usize) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertSeparator(self.as_ptr(), pos) }
    }
    /// Inserts a stretchable space at the given position.
    ///
    /// [See `wxToolBar::InsertStretchableSpace()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ab5be7309ff14d548dea18a73c4980964)
    fn insert_stretchable_space(&self, pos: usize) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertStretchableSpace(self.as_ptr(), pos) }
    }
    /// Inserts the tool with the specified attributes into the toolbar at the given position.
    ///
    /// [See `wxToolBar::InsertTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a4e662a923f34ccbc0d8141312cc3e1be)
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
    ///
    /// [See `wxToolBar::InsertTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a7df7bd046e1626f6be3cbdcee4b157e1)
    fn insert_tool_toolbartoolbase(&self, pos: usize, tool: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxToolBar_InsertTool1(self.as_ptr(), pos, tool) }
    }
    /// Called when the user clicks on a tool with the left mouse button.
    ///
    /// [See `wxToolBar::OnLeftClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ab25e206f57a873d909cf5c45d2f91971)
    fn on_left_click(&self, tool_id: c_int, toggle_down: bool) -> bool {
        unsafe { ffi::wxToolBar_OnLeftClick(self.as_ptr(), tool_id, toggle_down) }
    }
    /// This is called when the mouse cursor moves into a tool or out of the toolbar.
    ///
    /// [See `wxToolBar::OnMouseEnter()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a11fb9b5c65661ea63d28198ce156cf4d)
    fn on_mouse_enter(&self, tool_id: c_int) {
        unsafe { ffi::wxToolBar_OnMouseEnter(self.as_ptr(), tool_id) }
    }
    ///
    /// [See `wxToolBar::OnRightClick()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#add255ea52fdf9d5bfde64a22bf94dab6)
    fn on_right_click(&self, tool_id: c_int, x: c_long, y: c_long) {
        unsafe { ffi::wxToolBar_OnRightClick(self.as_ptr(), tool_id, x, y) }
    }
    /// This function should be called after you have added tools.
    ///
    /// [See `wxToolBar::Realize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#abb31ce337a2d3c1e30e8e3748905ecbf)
    fn realize(&self) -> bool {
        unsafe { ffi::wxToolBar_Realize(self.as_ptr()) }
    }
    /// Removes the given tool from the toolbar but doesn't delete it.
    ///
    /// [See `wxToolBar::RemoveTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a32ccf94df61f21e2b1853b9fdcfad1e1)
    fn remove_tool(&self, id: c_int) -> *mut c_void {
        unsafe { ffi::wxToolBar_RemoveTool(self.as_ptr(), id) }
    }
    /// Sets the dropdown menu for the tool given by its id.
    ///
    /// [See `wxToolBar::SetDropdownMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a77e36541602e9c0f7daa4e9bf8b6e33e)
    fn set_dropdown_menu<M: MenuMethods>(&self, id: c_int, menu: Option<&M>) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_SetDropdownMenu(self.as_ptr(), id, menu)
        }
    }
    /// Set the values to be used as margins for the toolbar.
    ///
    /// [See `wxToolBar::SetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#af35993e6b491f619b810fbcf084f44e3)
    fn set_margins_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxToolBar_SetMargins(self.as_ptr(), x, y) }
    }
    /// Set the margins for the toolbar.
    ///
    /// [See `wxToolBar::SetMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a7f793b01108f804ab076d05420b420da)
    fn set_margins_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxToolBar_SetMargins1(self.as_ptr(), size)
        }
    }
    /// Sets the default size of each tool bitmap.
    ///
    /// [See `wxToolBar::SetToolBitmapSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a57398893cb553fad90ffa7cd18cb1882)
    fn set_tool_bitmap_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxToolBar_SetToolBitmapSize(self.as_ptr(), size)
        }
    }
    /// Sets the client data associated with the tool.
    ///
    /// [See `wxToolBar::SetToolClientData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a4e3c80eeecb4723c9d4f090caab5151f)
    fn set_tool_client_data<O: ObjectMethods>(&self, id: c_int, client_data: Option<&O>) {
        unsafe {
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxToolBar_SetToolClientData(self.as_ptr(), id, client_data)
        }
    }
    /// Sets the bitmap to be used by the tool with the given ID when the tool is in a disabled state.
    ///
    /// [See `wxToolBar::SetToolDisabledBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a3846b3ed1b5167190b35fa15b59f80fd)
    fn set_tool_disabled_bitmap<B: BitmapBundleMethods>(&self, id: c_int, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxToolBar_SetToolDisabledBitmap(self.as_ptr(), id, bitmap)
        }
    }
    /// Sets the long help for the given tool.
    ///
    /// [See `wxToolBar::SetToolLongHelp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#abaf8db63f2bc75e0de67280b7e83711d)
    fn set_tool_long_help(&self, tool_id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxToolBar_SetToolLongHelp(self.as_ptr(), tool_id, help_string)
        }
    }
    /// Sets the bitmap to be used by the tool with the given ID.
    ///
    /// [See `wxToolBar::SetToolNormalBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a8a6149e119f2f674fa82bbcb73a2b9a6)
    fn set_tool_normal_bitmap<B: BitmapBundleMethods>(&self, id: c_int, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxToolBar_SetToolNormalBitmap(self.as_ptr(), id, bitmap)
        }
    }
    /// Sets the value used for spacing tools.
    ///
    /// [See `wxToolBar::SetToolPacking()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a26ddfe7e52a93fbc4c169159863c629c)
    fn set_tool_packing(&self, packing: c_int) {
        unsafe { ffi::wxToolBar_SetToolPacking(self.as_ptr(), packing) }
    }
    /// Sets the default separator size.
    ///
    /// [See `wxToolBar::SetToolSeparation()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a163c9bfdd0f9f30707688bf498ab788c)
    fn set_tool_separation(&self, separation: c_int) {
        unsafe { ffi::wxToolBar_SetToolSeparation(self.as_ptr(), separation) }
    }
    /// Sets the short help for the given tool.
    ///
    /// [See `wxToolBar::SetToolShortHelp()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a28ad34823b70faeae9a64c6152b8fcd1)
    fn set_tool_short_help(&self, tool_id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxToolBar_SetToolShortHelp(self.as_ptr(), tool_id, help_string)
        }
    }
    /// Toggles a tool on or off.
    ///
    /// [See `wxToolBar::ToggleTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a9a7a9fb115b35f267d4b10a293667542)
    fn toggle_tool(&self, tool_id: c_int, toggle: bool) {
        unsafe { ffi::wxToolBar_ToggleTool(self.as_ptr(), tool_id, toggle) }
    }
    /// Factory function to create a new toolbar tool.
    ///
    /// [See `wxToolBar::CreateTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#ab0354a5e07e1cc5df112f493186d62ad)
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
    /// Factory function to create a new control toolbar tool.
    ///
    /// [See `wxToolBar::CreateTool()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#aeadad4478ff6e2cbc892a77e0e786fbf)
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
    /// Factory function to create a new separator toolbar tool.
    ///
    /// [See `wxToolBar::CreateSeparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_bar.html#a96f3adc566c3d1330389dd954ed62fab)
    fn create_separator(&self) -> *mut c_void {
        unsafe { ffi::wxToolBar_CreateSeparator(self.as_ptr()) }
    }
}

// wxToolTip
/// This class holds information about a tooltip associated with a window (see wxWindow::SetToolTip()).
///
/// [See `wxToolTip`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html)
pub trait ToolTipMethods: ObjectMethods {
    /// Get the tooltip text.
    ///
    /// [See `wxToolTip::GetTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a74c26e6d9078fb678c5e9fb0ac129816)
    fn get_tip(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxToolTip_GetTip(self.as_ptr())).into() }
    }
    /// Get the associated window.
    ///
    /// [See `wxToolTip::GetWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#ae4d4fd80f826f57582a5174074f06d85)
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxToolTip_GetWindow(self.as_ptr())) }
    }
    /// Set the tooltip text.
    ///
    /// [See `wxToolTip::SetTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#af398833cd15ef69ef8d82fc5b41edea0)
    fn set_tip(&self, tip: &str) {
        unsafe {
            let tip = WxString::from(tip);
            let tip = tip.as_ptr();
            ffi::wxToolTip_SetTip(self.as_ptr(), tip)
        }
    }
    /// Enable or disable tooltips globally.
    ///
    /// [See `wxToolTip::Enable()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a527208d12631dfb3e6dea71011b887d6)
    fn enable(flag: bool) {
        unsafe { ffi::wxToolTip_Enable(flag) }
    }
    /// Set the delay after which the tooltip disappears or how long a tooltip remains visible.
    ///
    /// [See `wxToolTip::SetAutoPop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a6e947da99007aa96e2bd351c8ed08a21)
    fn set_auto_pop(msecs: c_long) {
        unsafe { ffi::wxToolTip_SetAutoPop(msecs) }
    }
    /// Set the delay after which the tooltip appears.
    ///
    /// [See `wxToolTip::SetDelay()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a5db5357f084afef9c4602baf0cb89105)
    fn set_delay(msecs: c_long) {
        unsafe { ffi::wxToolTip_SetDelay(msecs) }
    }
    /// Set tooltip maximal width in pixels.
    ///
    /// [See `wxToolTip::SetMaxWidth()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a244b83c88e7d4559ed7e74e8173c5b42)
    fn set_max_width(width: c_int) {
        unsafe { ffi::wxToolTip_SetMaxWidth(width) }
    }
    /// Set the delay between subsequent tooltips to appear.
    ///
    /// [See `wxToolTip::SetReshow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tool_tip.html#a10e1eb761dca8e28583250265aa22ad3)
    fn set_reshow(msecs: c_long) {
        unsafe { ffi::wxToolTip_SetReshow(msecs) }
    }
}

// wxToolbook
/// wxToolbook is a class similar to wxNotebook but which uses a wxToolBar to show the labels instead of the tabs.
///
/// [See `wxToolbook`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toolbook.html)
pub trait ToolbookMethods: BookCtrlBaseMethods {
    /// Returns the wxToolBarBase associated with the control.
    ///
    /// [See `wxToolbook::GetToolBar()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#ab425d7c19215f294fb9041b4bdc2aef2)
    fn get_tool_bar(&self) -> *mut c_void {
        unsafe { ffi::wxToolbook_GetToolBar(self.as_ptr()) }
    }
    /// Enables or disables the specified page.
    ///
    /// [See `wxToolbook::EnablePage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#a6b4c25bbb81ec70cfd0ab533acae7b08)
    fn enable_page_sz(&self, page: usize, enable: bool) -> bool {
        unsafe { ffi::wxToolbook_EnablePage(self.as_ptr(), page, enable) }
    }
    /// Enables or disables the specified page.
    ///
    /// [See `wxToolbook::EnablePage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_toolbook.html#aa6027f6754f74b9e6661b9a7a56e8778)
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
/// wxTopLevelWindow is a common base class for wxDialog and wxFrame.
///
/// [See `wxTopLevelWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html)
pub trait TopLevelWindowMethods: NonOwnedWindowMethods {
    // DTOR: fn ~wxTopLevelWindow()
    /// Creates the top level window.
    ///
    /// [See `wxTopLevelWindow::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a3a03f383ee3a98f62b3419463f27f346)
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
    /// A synonym for CentreOnScreen().
    ///
    /// [See `wxTopLevelWindow::CenterOnScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#adaaa8f83cde59e1c4c563c058b381ac4)
    fn center_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.as_ptr(), direction) }
    }
    /// Centres the window on screen.
    ///
    /// [See `wxTopLevelWindow::CentreOnScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a1d28df1d739af6a3d5d2ec9e43b03c29)
    fn centre_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.as_ptr(), direction) }
    }
    /// Enables or disables the Close button (most often in the right upper corner of a dialog) and the Close entry of the system menu (most often in the left upper corner of the dialog).
    ///
    /// [See `wxTopLevelWindow::EnableCloseButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a78a9441746702894fba7653c2d98cb8d)
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.as_ptr(), enable) }
    }
    /// Enables or disables the Maximize button (in the right or left upper corner of a frame or dialog).
    ///
    /// [See `wxTopLevelWindow::EnableMaximizeButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a0891e79cd5fca827aaa7e06c683f048c)
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr(), enable) }
    }
    /// Enables or disables the Minimize button (in the right or left upper corner of a frame or dialog).
    ///
    /// [See `wxTopLevelWindow::EnableMinimizeButton()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a6c035ec449accd9ea87c22dd5581c51c)
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr(), enable) }
    }
    /// Returns a pointer to the button which is the default for this window, or  NULL.
    ///
    /// [See `wxTopLevelWindow::GetDefaultItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#aa23b789ea5c3b15f191d9e7680480855)
    fn get_default_item(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTopLevelWindow_GetDefaultItem(self.as_ptr())) }
    }
    /// Returns the standard icon of the window.
    ///
    /// [See `wxTopLevelWindow::GetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a9e922563d44423c2cb4097326358caf0)
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxTopLevelWindow_GetIcon(self.as_ptr())) }
    }
    // BLOCKED: fn GetIcons()
    /// Gets a string containing the window title.
    ///
    /// [See `wxTopLevelWindow::GetTitle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#ab6ac6ef93ded0703c105332861ebe6f5)
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTopLevelWindow_GetTitle(self.as_ptr())).into() }
    }
    /// Iconizes or restores the window.
    ///
    /// [See `wxTopLevelWindow::Iconize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#ae461559192e33ef111c3f147999a26bf)
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.as_ptr(), iconize) }
    }
    /// Returns true if this window is currently active, i.e. if the user is currently working with it.
    ///
    /// [See `wxTopLevelWindow::IsActive()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#afa58f411cf488eb9279683b1d20a74b6)
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.as_ptr()) }
    }
    /// Returns true if this window is expected to be always maximized, either due to platform policy or due to local policy regarding particular class.
    ///
    /// [See `wxTopLevelWindow::IsAlwaysMaximized()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a84a05afb2ce3ff0f56c9b16c9a656d3e)
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr()) }
    }
    /// Returns true if the window is in fullscreen mode.
    ///
    /// [See `wxTopLevelWindow::IsFullScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a865e3244aca58679dab610df06c90f02)
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(self.as_ptr()) }
    }
    /// Returns true if the window is iconized.
    ///
    /// [See `wxTopLevelWindow::IsIconized()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a558559389a38bbba4333c4d6d56840aa)
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(self.as_ptr()) }
    }
    /// Returns true if the window is maximized.
    ///
    /// [See `wxTopLevelWindow::IsMaximized()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a20143d255165f6780d20f6611b67057e)
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(self.as_ptr()) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    /// Maximizes or restores the window.
    ///
    /// [See `wxTopLevelWindow::Maximize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#addb1612e9f00ac82c2e19c2bac6ffc4f)
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.as_ptr(), maximize) }
    }
    /// MSW-specific function for accessing the system menu.
    ///
    /// [See `wxTopLevelWindow::MSWGetSystemMenu()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a3f5d1dd4bf61bea390802ae6997a9ee5)
    fn msw_get_system_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxTopLevelWindow_MSWGetSystemMenu(self.as_ptr())) }
    }
    /// Use a system-dependent way to attract users attention to the window when it is in background.
    ///
    /// [See `wxTopLevelWindow::RequestUserAttention()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a749d23ea08f6273fb93fe2c003b650b3)
    fn request_user_attention(&self, flags: c_int) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.as_ptr(), flags) }
    }
    /// Restore a previously iconized or maximized window to its normal state.
    ///
    /// [See `wxTopLevelWindow::Restore()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a21e2d58a530c431c26ff098e4f75f249)
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.as_ptr()) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    /// Changes the default item for the panel, usually win is a button.
    ///
    /// [See `wxTopLevelWindow::SetDefaultItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#ae7c2b3a50037896115ace0ab75f67d4d)
    fn set_default_item<W: WindowMethods>(&self, win: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxTopLevelWindow_SetDefaultItem(self.as_ptr(), win))
        }
    }
    ///
    /// [See `wxTopLevelWindow::SetTmpDefaultItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a23c168237b5d1711d593884a90b8216b)
    fn set_tmp_default_item<W: WindowMethods>(&self, win: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr(), win))
        }
    }
    ///
    /// [See `wxTopLevelWindow::GetTmpDefaultItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#ab2d89dc35a6b1105eac22d948390028c)
    fn get_tmp_default_item(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr())) }
    }
    /// Sets the icon for this window.
    ///
    /// [See `wxTopLevelWindow::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a33dc013a7cb33384f631b2764ca53b06)
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxTopLevelWindow_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Sets several icons of different sizes for this window: this allows using different icons for different situations (e.g.
    ///
    /// [See `wxTopLevelWindow::SetIcons()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a34270084ea5a46c12309abbb99f2004c)
    fn set_icons<I: IconBundleMethods>(&self, icons: &I) {
        unsafe {
            let icons = icons.as_ptr();
            ffi::wxTopLevelWindow_SetIcons(self.as_ptr(), icons)
        }
    }
    /// Sets the window title.
    ///
    /// [See `wxTopLevelWindow::SetTitle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#afb944376c19d735a8251654ee4ac276f)
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxTopLevelWindow_SetTitle(self.as_ptr(), title)
        }
    }
    /// This virtual function is not meant to be called directly but can be overridden to return false (it returns true by default) to allow the application to close even if this, presumably not very important, window is still opened.
    ///
    /// [See `wxTopLevelWindow::ShouldPreventAppExit()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a2491f400227cedfc5140f68dbea46f27)
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr()) }
    }
    /// This function sets the wxTopLevelWindow's modified state on macOS, which currently draws a black dot in the wxTopLevelWindow's close button.
    ///
    /// [See `wxTopLevelWindow::OSXSetModified()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a6d102c9180b115a7a72b2e21f56cbc3e)
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.as_ptr(), modified) }
    }
    /// Returns the current modified state of the wxTopLevelWindow on macOS.
    ///
    /// [See `wxTopLevelWindow::OSXIsModified()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#aa5f93bc8d4d9ecf132126e10f2e4bffd)
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(self.as_ptr()) }
    }
    /// Sets the file name represented by this wxTopLevelWindow.
    ///
    /// [See `wxTopLevelWindow::SetRepresentedFilename()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a82afa6f09dd5faef6330807ef59374e8)
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr(), filename)
        }
    }
    /// Show the wxTopLevelWindow, but do not give it keyboard focus.
    ///
    /// [See `wxTopLevelWindow::ShowWithoutActivating()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#a03e526f505716568318d601318527bd0)
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr()) }
    }
    /// Enables the zoom button to toggle full screen mode.
    ///
    /// [See `wxTopLevelWindow::EnableFullScreenView()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#abcc1ee1837105b622512b3bc3347770c)
    fn enable_full_screen_view(&self, enable: bool, style: c_long) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.as_ptr(), enable, style) }
    }
    /// Depending on the value of show parameter the window is either shown full screen or restored to its normal state.
    ///
    /// [See `wxTopLevelWindow::ShowFullScreen()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#ab4089f1274bcb74e5f7763d1fb84ee28)
    fn show_full_screen(&self, show: bool, style: c_long) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.as_ptr(), show, style) }
    }
    // NOT_SUPPORTED: fn GetContentProtection()
    // NOT_SUPPORTED: fn SetContentProtection()
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    /// Get the default size for a new top level window.
    ///
    /// [See `wxTopLevelWindow::GetDefaultSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_top_level_window.html#ab096a9937afef5bc150686b6a819c64a)
    fn get_default_size() -> Size {
        unsafe { Size::from_ptr(ffi::wxTopLevelWindow_GetDefaultSize()) }
    }
}

// wxTreeCtrl
/// A tree control presents information as a hierarchy, with items that may be expanded to show further items.
///
/// [See `wxTreeCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html)
pub trait TreeCtrlMethods: ControlMethods {
    // DTOR: fn ~wxTreeCtrl()
    /// Adds the root node to the tree, returning the new item.
    ///
    /// [See `wxTreeCtrl::AddRoot()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a1fa4eecee7fca8ea6d69903be8e6c285)
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
    /// Appends an item to the end of the branch identified by parent, return a new item id.
    ///
    /// [See `wxTreeCtrl::AppendItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a0a798a2a06f21bcb5bef2c4b4b7cc3a5)
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
    /// Sets the buttons image list.
    ///
    /// [See `wxTreeCtrl::AssignButtonsImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a2a24d50d475f0c487a9162f477f6e69e)
    fn assign_buttons_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_AssignButtonsImageList(self.as_ptr(), image_list)
        }
    }
    /// Sets the state image list.
    ///
    /// [See `wxTreeCtrl::AssignStateImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a6f42485bcb6ff7445862b08b78f01f33)
    fn assign_state_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_AssignStateImageList(self.as_ptr(), image_list)
        }
    }
    /// Collapses the given item.
    ///
    /// [See `wxTreeCtrl::Collapse()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a6be5e621a37cd9a82e68eed0a083c211)
    fn collapse<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Collapse(self.as_ptr(), item)
        }
    }
    /// Collapses the root item.
    ///
    /// [See `wxTreeCtrl::CollapseAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a954bf5a6b748ac7b7b60ee18156467db)
    fn collapse_all(&self) {
        unsafe { ffi::wxTreeCtrl_CollapseAll(self.as_ptr()) }
    }
    /// Collapses this item and all of its children, recursively.
    ///
    /// [See `wxTreeCtrl::CollapseAllChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#aeef3170b6bb0fc53b79351183bd595d2)
    fn collapse_all_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_CollapseAllChildren(self.as_ptr(), item)
        }
    }
    /// Collapses the given item and removes all children.
    ///
    /// [See `wxTreeCtrl::CollapseAndReset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a15b56333132fbcdb6ade6590adf0ad1c)
    fn collapse_and_reset<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_CollapseAndReset(self.as_ptr(), item)
        }
    }
    /// Deletes the specified item.
    ///
    /// [See `wxTreeCtrl::Delete()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a53a2292e2b7b6c13ba3e2d542d8bc1d2)
    fn delete<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Delete(self.as_ptr(), item)
        }
    }
    /// Deletes all items in the control.
    ///
    /// [See `wxTreeCtrl::DeleteAllItems()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a76de2019d0fcc73f4c302dff5d392c66)
    fn delete_all_items(&self) {
        unsafe { ffi::wxTreeCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Deletes all children of the given item (but not the item itself).
    ///
    /// [See `wxTreeCtrl::DeleteChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a56b91fbfa520892a23b546432c4428b6)
    fn delete_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_DeleteChildren(self.as_ptr(), item)
        }
    }
    /// Starts editing the label of the given item.
    ///
    /// [See `wxTreeCtrl::EditLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab2b489f19ae42dc39bc90f2ac837105a)
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
    /// Enable or disable a beep if there is no match for the currently entered text when searching for the item from keyboard.
    ///
    /// [See `wxTreeCtrl::EnableBellOnNoMatch()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ac6e1b3f9c7fa6dcbd001b4807ec6cff2)
    fn enable_bell_on_no_match(&self, on: bool) {
        unsafe { ffi::wxTreeCtrl_EnableBellOnNoMatch(self.as_ptr(), on) }
    }
    /// Ends label editing.
    ///
    /// [See `wxTreeCtrl::EndEditLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a548331161e0b9f620a4663d84ec642b2)
    fn end_edit_label<T: TreeItemIdMethods>(&self, item: &T, discard_changes: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_EndEditLabel(self.as_ptr(), item, discard_changes)
        }
    }
    /// Scrolls and/or expands items to ensure that the given item is visible.
    ///
    /// [See `wxTreeCtrl::EnsureVisible()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a6440663f2dcf7652635ec7347d023f83)
    fn ensure_visible<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_EnsureVisible(self.as_ptr(), item)
        }
    }
    /// Expands the given item.
    ///
    /// [See `wxTreeCtrl::Expand()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a36e5c8826d223336df5b807700f4bded)
    fn expand<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Expand(self.as_ptr(), item)
        }
    }
    /// Expands all items in the tree.
    ///
    /// [See `wxTreeCtrl::ExpandAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a8767f4e10e5fb007a77701e27d3927e3)
    fn expand_all(&self) {
        unsafe { ffi::wxTreeCtrl_ExpandAll(self.as_ptr()) }
    }
    /// Expands the given item and all its children recursively.
    ///
    /// [See `wxTreeCtrl::ExpandAllChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a41b109ce1f196c2ed77a7f1c3dc3e7ff)
    fn expand_all_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ExpandAllChildren(self.as_ptr(), item)
        }
    }
    /// Retrieves the rectangle bounding the item.
    ///
    /// [See `wxTreeCtrl::GetBoundingRect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#af4419b412f1791c5ef2388a0f52b9f13)
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
    /// Returns the buttons image list (from which application-defined button images are taken).
    ///
    /// [See `wxTreeCtrl::GetButtonsImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a439e5fe7606b5b33f04655edcad45760)
    fn get_buttons_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxTreeCtrl_GetButtonsImageList(self.as_ptr())) }
    }
    /// Returns the number of items in the branch.
    ///
    /// [See `wxTreeCtrl::GetChildrenCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a696bd8930cfabfd60cbbd89ef0e5732b)
    fn get_children_count<T: TreeItemIdMethods>(&self, item: &T, recursively: bool) -> usize {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_GetChildrenCount(self.as_ptr(), item, recursively)
        }
    }
    /// Returns the number of items in the control.
    ///
    /// [See `wxTreeCtrl::GetCount()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a0e8464d26adaa134598397eee2015e82)
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxTreeCtrl_GetCount(self.as_ptr()) }
    }
    /// Returns the edit control being currently used to edit a label.
    ///
    /// [See `wxTreeCtrl::GetEditControl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#af36f849a82411b7a81179b562c48111b)
    fn get_edit_control(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxTreeCtrl_GetEditControl(self.as_ptr())) }
    }
    /// Returns the first child; call GetNextChild() for the next child.
    ///
    /// [See `wxTreeCtrl::GetFirstChild()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a30546111962c66cc761b3ec717738e8a)
    fn get_first_child<T: TreeItemIdMethods>(&self, item: &T, cookie: *mut c_void) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetFirstChild(self.as_ptr(), item, cookie))
        }
    }
    /// Returns the first visible item.
    ///
    /// [See `wxTreeCtrl::GetFirstVisibleItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a5f9168352fcb511c803c7bd158997300)
    fn get_first_visible_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetFirstVisibleItem(self.as_ptr())) }
    }
    /// Returns the item last clicked or otherwise selected.
    ///
    /// [See `wxTreeCtrl::GetFocusedItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab65e39216b3e02e4e94345391ca8eba8)
    fn get_focused_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetFocusedItem(self.as_ptr())) }
    }
    /// Clears the currently focused item.
    ///
    /// [See `wxTreeCtrl::ClearFocusedItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a37114ccd8ab49b27991ccba2b786672a)
    fn clear_focused_item(&self) {
        unsafe { ffi::wxTreeCtrl_ClearFocusedItem(self.as_ptr()) }
    }
    /// Sets the currently focused item.
    ///
    /// [See `wxTreeCtrl::SetFocusedItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a37890d9d52a21c58342560906b658f1c)
    fn set_focused_item<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetFocusedItem(self.as_ptr(), item)
        }
    }
    /// Returns the current tree control indentation.
    ///
    /// [See `wxTreeCtrl::GetIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a450408747bde58d6cb6f2f18832be747)
    fn get_indent(&self) -> c_uint {
        unsafe { ffi::wxTreeCtrl_GetIndent(self.as_ptr()) }
    }
    /// Returns the current tree control spacing.
    ///
    /// [See `wxTreeCtrl::GetSpacing()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a2687d9a15b5d44f19230c2ee942e6961)
    fn get_spacing(&self) -> c_uint {
        unsafe { ffi::wxTreeCtrl_GetSpacing(self.as_ptr()) }
    }
    /// Returns the background colour of the item.
    ///
    /// [See `wxTreeCtrl::GetItemBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a0d9004aa9a61e38f4802b26e135a7380)
    fn get_item_background_colour<T: TreeItemIdMethods>(&self, item: &T) -> Colour {
        unsafe {
            let item = item.as_ptr();
            Colour::from_ptr(ffi::wxTreeCtrl_GetItemBackgroundColour(self.as_ptr(), item))
        }
    }
    /// Returns the tree item data associated with the item.
    ///
    /// [See `wxTreeCtrl::GetItemData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ae87589185698639e796b166aeeaaacb6)
    fn get_item_data<T: TreeItemIdMethods>(&self, item: &T) -> Option<TreeItemDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            TreeItemData::option_from(ffi::wxTreeCtrl_GetItemData(self.as_ptr(), item))
        }
    }
    /// Returns the font of the item label.
    ///
    /// [See `wxTreeCtrl::GetItemFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a2bf7a0ad3f935de5da95559a5bc1b21e)
    fn get_item_font<T: TreeItemIdMethods>(&self, item: &T) -> Font {
        unsafe {
            let item = item.as_ptr();
            Font::from_ptr(ffi::wxTreeCtrl_GetItemFont(self.as_ptr(), item))
        }
    }
    // NOT_SUPPORTED: fn GetItemImage()
    /// Returns the item's parent.
    ///
    /// [See `wxTreeCtrl::GetItemParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#af76c5a2dab0092624cd9082ea810f38c)
    fn get_item_parent<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetItemParent(self.as_ptr(), item))
        }
    }
    /// Gets the specified item state.
    ///
    /// [See `wxTreeCtrl::GetItemState()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a0f15135b810cf28cdda9ea35d2f2ef74)
    fn get_item_state<T: TreeItemIdMethods>(&self, item: &T) -> c_int {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_GetItemState(self.as_ptr(), item)
        }
    }
    /// Returns the item label.
    ///
    /// [See `wxTreeCtrl::GetItemText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a3666ef743b222e4abfd28b51c9232928)
    fn get_item_text<T: TreeItemIdMethods>(&self, item: &T) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxTreeCtrl_GetItemText(self.as_ptr(), item)).into()
        }
    }
    /// Returns the colour of the item label.
    ///
    /// [See `wxTreeCtrl::GetItemTextColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a6fe94ddffcaaaa44c9ded45394b633f2)
    fn get_item_text_colour<T: TreeItemIdMethods>(&self, item: &T) -> Colour {
        unsafe {
            let item = item.as_ptr();
            Colour::from_ptr(ffi::wxTreeCtrl_GetItemTextColour(self.as_ptr(), item))
        }
    }
    /// Returns the last child of the item (or an invalid tree item if this item has no children).
    ///
    /// [See `wxTreeCtrl::GetLastChild()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ae0fc3ef235c7f97eac0271c587e09231)
    fn get_last_child<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetLastChild(self.as_ptr(), item))
        }
    }
    /// Returns the next child; call GetFirstChild() for the first child.
    ///
    /// [See `wxTreeCtrl::GetNextChild()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ac8b1a72cd624b3e93228af4242488856)
    fn get_next_child<T: TreeItemIdMethods>(&self, item: &T, cookie: *mut c_void) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetNextChild(self.as_ptr(), item, cookie))
        }
    }
    /// Returns the next sibling of the specified item; call GetPrevSibling() for the previous sibling.
    ///
    /// [See `wxTreeCtrl::GetNextSibling()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a05e302a9ff2af6ef7ae1ef18f2624194)
    fn get_next_sibling_treeitemid<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetNextSibling(self.as_ptr(), item))
        }
    }
    /// Returns the next visible item or an invalid item if this item is the last visible one.
    ///
    /// [See `wxTreeCtrl::GetNextVisible()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a3d48b36dbae179429073016df1336c6c)
    fn get_next_visible<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetNextVisible(self.as_ptr(), item))
        }
    }
    /// Returns the previous sibling of the specified item; call GetNextSibling() for the next sibling.
    ///
    /// [See `wxTreeCtrl::GetPrevSibling()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a5b66e8d79ef9aa2ee5c1fba3bd2d619a)
    fn get_prev_sibling_treeitemid<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetPrevSibling(self.as_ptr(), item))
        }
    }
    /// Returns the previous visible item or an invalid item if this item is the first visible one.
    ///
    /// [See `wxTreeCtrl::GetPrevVisible()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a2dc88b8c9abeaa4cdc9ae95619006b87)
    fn get_prev_visible<T: TreeItemIdMethods>(&self, item: &T) -> TreeItemId {
        unsafe {
            let item = item.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_GetPrevVisible(self.as_ptr(), item))
        }
    }
    /// Returns true if the control will use a quick calculation for the best size, looking only at the first and last items.
    ///
    /// [See `wxTreeCtrl::GetQuickBestSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a1b9c71340e324aa106f8cde522cb2a33)
    fn get_quick_best_size(&self) -> bool {
        unsafe { ffi::wxTreeCtrl_GetQuickBestSize(self.as_ptr()) }
    }
    /// Returns the root item for the tree control.
    ///
    /// [See `wxTreeCtrl::GetRootItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab5d7ec551901a78a79888106c580ab79)
    fn get_root_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetRootItem(self.as_ptr())) }
    }
    /// Returns the selection, or an invalid item if there is no selection.
    ///
    /// [See `wxTreeCtrl::GetSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a70a03f43aed60956d1fd63d2eb72ae2e)
    fn get_selection(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeCtrl_GetSelection(self.as_ptr())) }
    }
    /// Fills the array of tree items passed in with the currently selected items.
    ///
    /// [See `wxTreeCtrl::GetSelections()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a5d243137947e75cc3ef73e07bc183137)
    fn get_selections(&self, selection: *mut c_void) -> usize {
        unsafe { ffi::wxTreeCtrl_GetSelections(self.as_ptr(), selection) }
    }
    /// Returns the state image list (from which application-defined state images are taken).
    ///
    /// [See `wxTreeCtrl::GetStateImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#aeb27fd93365bb9cd9a9b88f32c386941)
    fn get_state_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxTreeCtrl_GetStateImageList(self.as_ptr())) }
    }
    /// Calculates which (if any) item is under the given point, returning the tree item id at this point plus extra information flags.
    ///
    /// [See `wxTreeCtrl::HitTest()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab76927cdfbd23296e4817a200dc7aee7)
    fn hit_test<P: PointMethods>(&self, point: &P, flags: *mut c_void) -> TreeItemId {
        unsafe {
            let point = point.as_ptr();
            TreeItemId::from_ptr(ffi::wxTreeCtrl_HitTest(self.as_ptr(), point, flags))
        }
    }
    /// Inserts an item after a given one (previous).
    ///
    /// [See `wxTreeCtrl::InsertItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a06e1e6b25a7121922412eea510bd3109)
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
    /// Inserts an item before one identified by its position (pos).
    ///
    /// [See `wxTreeCtrl::InsertItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a52e7baec34632a75ce8060695cfa976a)
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
    /// Returns true if the given item is in bold state.
    ///
    /// [See `wxTreeCtrl::IsBold()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a08877c0319a358bb02e37543a61c7fe8)
    fn is_bold<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsBold(self.as_ptr(), item)
        }
    }
    /// Returns true if the control is empty (i.e. has no items, even no root one).
    ///
    /// [See `wxTreeCtrl::IsEmpty()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a1ee6c7fe6489c47e44f962d050626b9e)
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxTreeCtrl_IsEmpty(self.as_ptr()) }
    }
    /// Returns true if the item is expanded (only makes sense if it has children).
    ///
    /// [See `wxTreeCtrl::IsExpanded()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a4143652eab75a62965e9896f1dfcadbc)
    fn is_expanded<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsExpanded(self.as_ptr(), item)
        }
    }
    /// Returns true if the item is selected.
    ///
    /// [See `wxTreeCtrl::IsSelected()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a72beea097880e83337bf0784104264e3)
    fn is_selected<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsSelected(self.as_ptr(), item)
        }
    }
    /// Returns true if the item is visible on the screen.
    ///
    /// [See `wxTreeCtrl::IsVisible()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a29724d678d683786a1f3f2ac8f160291)
    fn is_visible<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_IsVisible(self.as_ptr(), item)
        }
    }
    /// Returns true if the item has children.
    ///
    /// [See `wxTreeCtrl::ItemHasChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab1881d51d96807f1ea775dc8f9f4390d)
    fn item_has_children<T: TreeItemIdMethods>(&self, item: &T) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ItemHasChildren(self.as_ptr(), item)
        }
    }
    /// Override this function in the derived class to change the sort order of the items in the tree control.
    ///
    /// [See `wxTreeCtrl::OnCompareItems()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab90a465793c291ca7aa827a576b7d146)
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
    /// Appends an item as the first child of parent, return a new item id.
    ///
    /// [See `wxTreeCtrl::PrependItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab7ec4164ca8eb1675646f0c16b681566)
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
    /// Scrolls the specified item into view.
    ///
    /// [See `wxTreeCtrl::ScrollTo()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a5448656d47293d10348971c56aba61ab)
    fn scroll_to<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ScrollTo(self.as_ptr(), item)
        }
    }
    /// Selects the given item.
    ///
    /// [See `wxTreeCtrl::SelectItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ab6ed9a6e7c3d101ec51a496ef71b422e)
    fn select_item<T: TreeItemIdMethods>(&self, item: &T, select: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SelectItem(self.as_ptr(), item, select)
        }
    }
    /// Sets the buttons image list (from which application-defined button images are taken).
    ///
    /// [See `wxTreeCtrl::SetButtonsImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#aa8ce6bdc82a7966d1cba0fdf5a79f6d0)
    fn set_buttons_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_SetButtonsImageList(self.as_ptr(), image_list)
        }
    }
    /// Sets the indentation for the tree control.
    ///
    /// [See `wxTreeCtrl::SetIndent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a1db2c8ef0b28483515a2585a0312dc43)
    fn set_indent(&self, indent: c_uint) {
        unsafe { ffi::wxTreeCtrl_SetIndent(self.as_ptr(), indent) }
    }
    /// Sets the spacing for the tree control.
    ///
    /// [See `wxTreeCtrl::SetSpacing()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ad328c1d84bafad142faf5eb70b91515f)
    fn set_spacing(&self, spacing: c_uint) {
        unsafe { ffi::wxTreeCtrl_SetSpacing(self.as_ptr(), spacing) }
    }
    /// Sets the colour of the item's background.
    ///
    /// [See `wxTreeCtrl::SetItemBackgroundColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ae55f83069faef927b8d4a4d3e45daae6)
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
    /// Makes item appear in bold font if bold parameter is true or resets it to the normal state.
    ///
    /// [See `wxTreeCtrl::SetItemBold()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a52744e373d5405e384928eab5fb9c991)
    fn set_item_bold<T: TreeItemIdMethods>(&self, item: &T, bold: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemBold(self.as_ptr(), item, bold)
        }
    }
    /// Sets the item client data.
    ///
    /// [See `wxTreeCtrl::SetItemData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#adb74a81b41ab25bd38884311bb4b3b46)
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
    /// Gives the item the visual feedback for Drag'n'Drop actions, which is useful if something is dragged from the outside onto the tree control (as opposed to a DnD operation within the tree control, which already is implemented internally).
    ///
    /// [See `wxTreeCtrl::SetItemDropHighlight()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a03fd2789f5bdb6c645b381739772c673)
    fn set_item_drop_highlight<T: TreeItemIdMethods>(&self, item: &T, highlight: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemDropHighlight(self.as_ptr(), item, highlight)
        }
    }
    /// Sets the item's font.
    ///
    /// [See `wxTreeCtrl::SetItemFont()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ae8283e7314d164b7a69056fa182d2386)
    fn set_item_font<T: TreeItemIdMethods, F: FontMethods>(&self, item: &T, font: &F) {
        unsafe {
            let item = item.as_ptr();
            let font = font.as_ptr();
            ffi::wxTreeCtrl_SetItemFont(self.as_ptr(), item, font)
        }
    }
    /// Force appearance of the button next to the item.
    ///
    /// [See `wxTreeCtrl::SetItemHasChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a26211b6424ab2b4108311a2b733eb7e5)
    fn set_item_has_children<T: TreeItemIdMethods>(&self, item: &T, has_children: bool) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemHasChildren(self.as_ptr(), item, has_children)
        }
    }
    // NOT_SUPPORTED: fn SetItemImage()
    /// Sets the specified item state.
    ///
    /// [See `wxTreeCtrl::SetItemState()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a8424e24cb8d4ceedffd11f5bc6218038)
    fn set_item_state<T: TreeItemIdMethods>(&self, item: &T, state: c_int) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SetItemState(self.as_ptr(), item, state)
        }
    }
    /// Sets the item label.
    ///
    /// [See `wxTreeCtrl::SetItemText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#aab82b6353f72d0f06c6e65a6924a1df1)
    fn set_item_text<T: TreeItemIdMethods>(&self, item: &T, text: &str) {
        unsafe {
            let item = item.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTreeCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
    /// Sets the colour of the item's text.
    ///
    /// [See `wxTreeCtrl::SetItemTextColour()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a319b4ca62f34f0a4377f4ec77cb92974)
    fn set_item_text_colour<T: TreeItemIdMethods, C: ColourMethods>(&self, item: &T, col: &C) {
        unsafe {
            let item = item.as_ptr();
            let col = col.as_ptr();
            ffi::wxTreeCtrl_SetItemTextColour(self.as_ptr(), item, col)
        }
    }
    /// If true is passed, specifies that the control will use a quick calculation for the best size, looking only at the first and last items.
    ///
    /// [See `wxTreeCtrl::SetQuickBestSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a54a5d1e4cd35b5c693d0d701a04ff358)
    fn set_quick_best_size(&self, quick_best_size: bool) {
        unsafe { ffi::wxTreeCtrl_SetQuickBestSize(self.as_ptr(), quick_best_size) }
    }
    /// Sets the state image list (from which application-defined state images are taken).
    ///
    /// [See `wxTreeCtrl::SetStateImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#aabb789a3eaa97608659e50afbd7a46ff)
    fn set_state_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeCtrl_SetStateImageList(self.as_ptr(), image_list)
        }
    }
    /// Sorts the children of the given item using OnCompareItems().
    ///
    /// [See `wxTreeCtrl::SortChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#aa59a3c3081b78ffe644fee0a4bcd1ca3)
    fn sort_children<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_SortChildren(self.as_ptr(), item)
        }
    }
    /// Toggles the given item between collapsed and expanded states.
    ///
    /// [See `wxTreeCtrl::Toggle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a618a620d9a434bacf91a51202ab807b1)
    fn toggle<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_Toggle(self.as_ptr(), item)
        }
    }
    /// Toggles the given item between selected and unselected states.
    ///
    /// [See `wxTreeCtrl::ToggleItemSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a458eab144e16efbaf283748bacc475f7)
    fn toggle_item_selection<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_ToggleItemSelection(self.as_ptr(), item)
        }
    }
    /// Removes the selection from the currently selected item (if any).
    ///
    /// [See `wxTreeCtrl::Unselect()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a86a5a649501f02a635ce193afb9d9f0f)
    fn unselect(&self) {
        unsafe { ffi::wxTreeCtrl_Unselect(self.as_ptr()) }
    }
    /// This function either behaves the same as Unselect() if the control doesn't have wxTR_MULTIPLE style, or removes the selection from all items if it does have this style.
    ///
    /// [See `wxTreeCtrl::UnselectAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#afc7bb2bb77f3589abdca5b4c1d99094d)
    fn unselect_all(&self) {
        unsafe { ffi::wxTreeCtrl_UnselectAll(self.as_ptr()) }
    }
    /// Unselects the given item.
    ///
    /// [See `wxTreeCtrl::UnselectItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#ad5eb938fd64cd961a2e50a90211974ec)
    fn unselect_item<T: TreeItemIdMethods>(&self, item: &T) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxTreeCtrl_UnselectItem(self.as_ptr(), item)
        }
    }
    /// Select all the immediate children of the given parent.
    ///
    /// [See `wxTreeCtrl::SelectChildren()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_ctrl.html#a3d43ef6217902a3a8f4c774de6be13ef)
    fn select_children<T: TreeItemIdMethods>(&self, parent: &T) {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxTreeCtrl_SelectChildren(self.as_ptr(), parent)
        }
    }
}

// wxTreeEvent
/// A tree event holds information about events associated with wxTreeCtrl objects.
///
/// [See `wxTreeEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html)
pub trait TreeEventMethods: NotifyEventMethods {
    /// Returns the item.
    ///
    /// [See `wxTreeEvent::GetItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#a7f692e2866427d04c9e64cb09b794d91)
    fn get_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeEvent_GetItem(self.as_ptr())) }
    }
    /// Returns the key code if the event is a key event.
    ///
    /// [See `wxTreeEvent::GetKeyCode()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#ab59f8026a657a304df5bc41be91e6b5a)
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxTreeEvent_GetKeyCode(self.as_ptr()) }
    }
    /// Returns the key event for EVT_TREE_KEY_DOWN events.
    ///
    /// [See `wxTreeEvent::GetKeyEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#a7d37b3a4be2c44a4640b193fad237a6f)
    fn get_key_event(&self) -> KeyEventIsOwned<false> {
        unsafe { KeyEventIsOwned::from_ptr(ffi::wxTreeEvent_GetKeyEvent(self.as_ptr())) }
    }
    /// Returns the label if the event is a begin or end edit label event.
    ///
    /// [See `wxTreeEvent::GetLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#a832e3dd7a013f13bcb360a1434667f34)
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxTreeEvent_GetLabel(self.as_ptr())).into() }
    }
    /// Returns the old item index (valid for EVT_TREE_SEL_CHANGING and EVT_TREE_SEL_CHANGED events).
    ///
    /// [See `wxTreeEvent::GetOldItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#ae639a05875f67fecca8d09ec4dc57a03)
    fn get_old_item(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxTreeEvent_GetOldItem(self.as_ptr())) }
    }
    /// Returns the position of the mouse pointer if the event is a drag or menu-context event.
    ///
    /// [See `wxTreeEvent::GetPoint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#a7759cd79d7d0bbb4f70a51d9b03e0944)
    fn get_point(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxTreeEvent_GetPoint(self.as_ptr())) }
    }
    /// Returns true if the label edit was cancelled.
    ///
    /// [See `wxTreeEvent::IsEditCancelled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#a05189c5269a904db8a61ebb5116f8124)
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxTreeEvent_IsEditCancelled(self.as_ptr()) }
    }
    /// Set the tooltip for the item (valid for EVT_TREE_ITEM_GETTOOLTIP events).
    ///
    /// [See `wxTreeEvent::SetToolTip()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_event.html#aa7e87d07fb9292ee3d72b857485bf342)
    fn set_tool_tip(&self, tooltip: &str) {
        unsafe {
            let tooltip = WxString::from(tooltip);
            let tooltip = tooltip.as_ptr();
            ffi::wxTreeEvent_SetToolTip(self.as_ptr(), tooltip)
        }
    }
}

// wxTreeItemData
/// wxTreeItemData is some (arbitrary) user class associated with some item.
///
/// [See `wxTreeItemData`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html)
pub trait TreeItemDataMethods: ClientDataMethods {
    // DTOR: fn ~wxTreeItemData()
    /// Returns the item associated with this node.
    ///
    /// [See `wxTreeItemData::GetId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html#ad29ad85ec3390e8900a3c3f5b2b22eb0)
    fn get_id(&self) -> TreeItemIdIsOwned<false> {
        unsafe { TreeItemIdIsOwned::from_ptr(ffi::wxTreeItemData_GetId(self.as_ptr())) }
    }
    /// Sets the item associated with this node.
    ///
    /// [See `wxTreeItemData::SetId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_data.html#a527fa24ac4c80f1df25f51e24990ec44)
    fn set_id<T: TreeItemIdMethods>(&self, id: &T) {
        unsafe {
            let id = id.as_ptr();
            ffi::wxTreeItemData_SetId(self.as_ptr(), id)
        }
    }
}

// wxTreeItemId
/// An opaque reference to a tree item.
///
/// [See `wxTreeItemId`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html)
pub trait TreeItemIdMethods: WxRustMethods {
    /// Returns true if this instance is referencing a valid tree item.
    ///
    /// [See `wxTreeItemId::IsOk()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html#a56c80ce39762069bf2194bcf4bb7e941)
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxTreeItemId_IsOk(self.as_ptr()) }
    }
    ///
    /// [See `wxTreeItemId::GetID()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html#a85a6ebec469cfb7fb77ccde6805e0928)
    fn get_id(&self) -> *mut c_void {
        unsafe { ffi::wxTreeItemId_GetID(self.as_ptr()) }
    }
    ///
    /// [See `wxTreeItemId::Unset()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_item_id.html#a3ffea23cbc04a325ab67cedb2ce9fa0a)
    fn unset(&self) {
        unsafe { ffi::wxTreeItemId_Unset(self.as_ptr()) }
    }
}

// wxTreeListCtrl
/// A control combining wxTreeCtrl and wxListCtrl features.
///
/// [See `wxTreeListCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html)
pub trait TreeListCtrlMethods: WindowMethods {
    /// Sets the image list and gives its ownership to the control.
    ///
    /// [See `wxTreeListCtrl::AssignImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aec3d84f8fa4122f9aa5dd6a90106514c)
    fn assign_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_AssignImageList(self.as_ptr(), image_list)
        }
    }
    /// Sets the image list.
    ///
    /// [See `wxTreeListCtrl::SetImageList()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a2a9d3607f9e507f50187080518ab5d3d)
    fn set_image_list<I: ImageListMethods>(&self, image_list: Option<&I>) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_SetImageList(self.as_ptr(), image_list)
        }
    }
    /// Add a column with the given title and attributes.
    ///
    /// [See `wxTreeListCtrl::AppendColumn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a8464b3bf449606d5da7849be6b0986a5)
    fn append_column(&self, title: &str, width: c_int, align: c_int, flags: c_int) -> c_int {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxTreeListCtrl_AppendColumn(self.as_ptr(), title, width, align, flags)
        }
    }
    // NOT_SUPPORTED: fn GetColumnCount()
    // NOT_SUPPORTED: fn DeleteColumn()
    /// Delete all columns.
    ///
    /// [See `wxTreeListCtrl::ClearColumns()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aa2eb8bfe9d49270b720b902b42a2ed98)
    fn clear_columns(&self) {
        unsafe { ffi::wxTreeListCtrl_ClearColumns(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetColumnWidth()
    // NOT_SUPPORTED: fn GetColumnWidth()
    /// Get the width appropriate for showing the given text.
    ///
    /// [See `wxTreeListCtrl::WidthFor()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#ab36f813c68db8d927f106ec623c617d5)
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
    /// Delete all tree items.
    ///
    /// [See `wxTreeListCtrl::DeleteAllItems()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aaed015b3a0dbafc634f4c11c35beb250)
    fn delete_all_items(&self) {
        unsafe { ffi::wxTreeListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Return the (never shown) root item.
    ///
    /// [See `wxTreeListCtrl::GetRootItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aef255f7cd382ae41933abb17d7bbde36)
    fn get_root_item(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetRootItem(self.as_ptr())) }
    }
    // BLOCKED: fn GetItemParent()
    // BLOCKED: fn GetFirstChild()
    // BLOCKED: fn GetNextSibling()
    /// Return the first item in the tree.
    ///
    /// [See `wxTreeListCtrl::GetFirstItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a3ab57a3f0124bcef6965d28970d4fee2)
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
    /// Return the currently selected item.
    ///
    /// [See `wxTreeListCtrl::GetSelection()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#aa1267a9fd9f0f18eebb61dd711c3afe6)
    fn get_selection(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetSelection(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetSelections()
    // BLOCKED: fn Select()
    // BLOCKED: fn Unselect()
    // BLOCKED: fn IsSelected()
    /// Select all the control items.
    ///
    /// [See `wxTreeListCtrl::SelectAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a1c3cd2ca3a68462d16e845e6e7521052)
    fn select_all(&self) {
        unsafe { ffi::wxTreeListCtrl_SelectAll(self.as_ptr()) }
    }
    /// Deselect all the control items.
    ///
    /// [See `wxTreeListCtrl::UnselectAll()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a33d0d5a9ac453a895bf0469935926dd7)
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
    /// Return the column currently used for sorting, if any.
    ///
    /// [See `wxTreeListCtrl::GetSortColumn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#a707c2f1d1f2eee8bcb207e007ed54e68)
    fn get_sort_column(&self, col: *mut c_void, ascending_order: *mut c_void) -> bool {
        unsafe { ffi::wxTreeListCtrl_GetSortColumn(self.as_ptr(), col, ascending_order) }
    }
    /// Set the object to use for comparing the items.
    ///
    /// [See `wxTreeListCtrl::SetItemComparator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#addcdef889d35ca918f1a0303cc6ef751)
    fn set_item_comparator<T: TreeListItemComparatorMethods>(&self, comparator: Option<&T>) {
        unsafe {
            let comparator = match comparator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_SetItemComparator(self.as_ptr(), comparator)
        }
    }
    /// Return the view part of this control as a wxWindow.
    ///
    /// [See `wxTreeListCtrl::GetView()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#afb0af50bf9310ae2e8ef7f15a68f08a9)
    fn get_view(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxTreeListCtrl_GetView(self.as_ptr())) }
    }
    /// Return the view part of this control as wxDataViewCtrl.
    ///
    /// [See `wxTreeListCtrl::GetDataView()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_ctrl.html#ad3b775a79cc7051fac5557c77dc0344b)
    fn get_data_view(&self) -> WeakRef<DataViewCtrl> {
        unsafe { WeakRef::<DataViewCtrl>::from(ffi::wxTreeListCtrl_GetDataView(self.as_ptr())) }
    }
}

// wxTreeListItem
/// Unique identifier of an item in wxTreeListCtrl.
///
/// [See `wxTreeListItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html)
pub trait TreeListItemMethods: WxRustMethods {
    /// Return true if the item is valid.
    ///
    /// [See `wxTreeListItem::IsOk()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_item.html#af53649cd5f5305b49993b89a74763d8b)
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxTreeListItem_IsOk(self.as_ptr()) }
    }
}

// wxTreeListItemComparator
/// Class defining sort order for the items in wxTreeListCtrl.
///
/// [See `wxTreeListItemComparator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_tree_list_item_comparator.html)
pub trait TreeListItemComparatorMethods: WxRustMethods {
    // NOT_SUPPORTED: fn Compare()
    // DTOR: fn ~wxTreeListItemComparator()
}

// wxTreebook
/// This class is an extension of the wxNotebook class that allows a tree structured set of pages to be shown in a control.
///
/// [See `wxTreebook`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html)
pub trait TreebookMethods: BookCtrlBaseMethods {
    // DTOR: fn ~wxTreebook()
    /// Adds a new child-page to the last top-level page.
    ///
    /// [See `wxTreebook::AddSubPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html#a409e957bacff40d46c3ba9a6606fc0d2)
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
    /// Shortcut for ExpandNode( pageId, false ).
    ///
    /// [See `wxTreebook::CollapseNode()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html#ae71d6c62bb452d6a808cf289b98ef30b)
    fn collapse_node(&self, page_id: usize) -> bool {
        unsafe { ffi::wxTreebook_CollapseNode(self.as_ptr(), page_id) }
    }
    /// Expands (collapses) the pageId node.
    ///
    /// [See `wxTreebook::ExpandNode()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html#add0b76a06599dc37b6275dc4b35ead79)
    fn expand_node(&self, page_id: usize, expand: bool) -> bool {
        unsafe { ffi::wxTreebook_ExpandNode(self.as_ptr(), page_id, expand) }
    }
    /// Returns the parent page of the given one or wxNOT_FOUND if this is a top-level page.
    ///
    /// [See `wxTreebook::GetPageParent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html#a244b279a4b1a6bf9091109ea2982f7fc)
    fn get_page_parent(&self, page: usize) -> c_int {
        unsafe { ffi::wxTreebook_GetPageParent(self.as_ptr(), page) }
    }
    /// Inserts a sub page under the specified page.
    ///
    /// [See `wxTreebook::InsertSubPage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html#ae3c46fd64891e13d511b4840b275143c)
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
    /// Returns true if the page represented by pageId is expanded.
    ///
    /// [See `wxTreebook::IsNodeExpanded()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_treebook.html#ae2364a8292b2022369df53e74f586ef7)
    fn is_node_expanded(&self, page_id: usize) -> bool {
        unsafe { ffi::wxTreebook_IsNodeExpanded(self.as_ptr(), page_id) }
    }
}
