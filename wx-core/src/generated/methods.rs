use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

pub use wx_base::methods::*;

// wxAboutDialogInfo
pub trait AboutDialogInfoMethods: WxRustMethods {
    fn add_artist(&self, artist: &str) {
        unsafe {
            let artist = WxString::from(artist);
            let artist = artist.as_ptr();
            ffi::wxAboutDialogInfo_AddArtist(self.as_ptr(), artist)
        }
    }
    fn add_developer(&self, developer: &str) {
        unsafe {
            let developer = WxString::from(developer);
            let developer = developer.as_ptr();
            ffi::wxAboutDialogInfo_AddDeveloper(self.as_ptr(), developer)
        }
    }
    fn add_doc_writer(&self, docwriter: &str) {
        unsafe {
            let docwriter = WxString::from(docwriter);
            let docwriter = docwriter.as_ptr();
            ffi::wxAboutDialogInfo_AddDocWriter(self.as_ptr(), docwriter)
        }
    }
    fn add_translator(&self, translator: &str) {
        unsafe {
            let translator = WxString::from(translator);
            let translator = translator.as_ptr();
            ffi::wxAboutDialogInfo_AddTranslator(self.as_ptr(), translator)
        }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetName(self.as_ptr())).into() }
    }
    fn has_description(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDescription(self.as_ptr()) }
    }
    fn get_description(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetDescription(self.as_ptr())).into() }
    }
    fn has_copyright(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasCopyright(self.as_ptr()) }
    }
    fn get_copyright(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetCopyright(self.as_ptr())).into() }
    }
    fn set_artists<A: ArrayStringMethods>(&self, artists: &A) {
        unsafe {
            let artists = artists.as_ptr();
            ffi::wxAboutDialogInfo_SetArtists(self.as_ptr(), artists)
        }
    }
    fn set_copyright(&self, copyright: &str) {
        unsafe {
            let copyright = WxString::from(copyright);
            let copyright = copyright.as_ptr();
            ffi::wxAboutDialogInfo_SetCopyright(self.as_ptr(), copyright)
        }
    }
    fn set_description(&self, desc: &str) {
        unsafe {
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxAboutDialogInfo_SetDescription(self.as_ptr(), desc)
        }
    }
    fn set_developers<A: ArrayStringMethods>(&self, developers: &A) {
        unsafe {
            let developers = developers.as_ptr();
            ffi::wxAboutDialogInfo_SetDevelopers(self.as_ptr(), developers)
        }
    }
    fn set_doc_writers<A: ArrayStringMethods>(&self, docwriters: &A) {
        unsafe {
            let docwriters = docwriters.as_ptr();
            ffi::wxAboutDialogInfo_SetDocWriters(self.as_ptr(), docwriters)
        }
    }
    fn has_icon(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasIcon(self.as_ptr()) }
    }
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxAboutDialogInfo_GetIcon(self.as_ptr())) }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxAboutDialogInfo_SetIcon(self.as_ptr(), icon)
        }
    }
    fn has_licence(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasLicence(self.as_ptr()) }
    }
    fn get_licence(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetLicence(self.as_ptr())).into() }
    }
    fn set_licence(&self, licence: &str) {
        unsafe {
            let licence = WxString::from(licence);
            let licence = licence.as_ptr();
            ffi::wxAboutDialogInfo_SetLicence(self.as_ptr(), licence)
        }
    }
    fn set_license(&self, licence: &str) {
        unsafe {
            let licence = WxString::from(licence);
            let licence = licence.as_ptr();
            ffi::wxAboutDialogInfo_SetLicense(self.as_ptr(), licence)
        }
    }
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxAboutDialogInfo_SetName(self.as_ptr(), name)
        }
    }
    fn set_translators<A: ArrayStringMethods>(&self, translators: &A) {
        unsafe {
            let translators = translators.as_ptr();
            ffi::wxAboutDialogInfo_SetTranslators(self.as_ptr(), translators)
        }
    }
    fn set_version(&self, version: &str, long_version: &str) {
        unsafe {
            let version = WxString::from(version);
            let version = version.as_ptr();
            let long_version = WxString::from(long_version);
            let long_version = long_version.as_ptr();
            ffi::wxAboutDialogInfo_SetVersion(self.as_ptr(), version, long_version)
        }
    }
    fn get_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetVersion(self.as_ptr())).into() }
    }
    fn get_long_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetLongVersion(self.as_ptr())).into() }
    }
    fn has_web_site(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasWebSite(self.as_ptr()) }
    }
    fn get_web_site_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetWebSiteURL(self.as_ptr())).into() }
    }
    fn get_web_site_description(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxAboutDialogInfo_GetWebSiteDescription(self.as_ptr())).into()
        }
    }
    fn set_web_site(&self, url: &str, desc: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxAboutDialogInfo_SetWebSite(self.as_ptr(), url, desc)
        }
    }
    fn has_developers(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDevelopers(self.as_ptr()) }
    }
    fn get_developers(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetDevelopers(self.as_ptr())) }
    }
    fn has_doc_writers(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDocWriters(self.as_ptr()) }
    }
    fn get_doc_writers(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetDocWriters(self.as_ptr())) }
    }
    fn has_artists(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasArtists(self.as_ptr()) }
    }
    fn get_artists(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetArtists(self.as_ptr())) }
    }
    fn has_translators(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasTranslators(self.as_ptr()) }
    }
    fn get_translators(&self) -> ArrayStringIsOwned<false> {
        unsafe {
            ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetTranslators(self.as_ptr()))
        }
    }
}

// wxAcceleratorEntry
pub trait AcceleratorEntryMethods: WxRustMethods {
    fn get_command(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetCommand(self.as_ptr()) }
    }
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetFlags(self.as_ptr()) }
    }
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetKeyCode(self.as_ptr()) }
    }
    fn get_menu_item(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxAcceleratorEntry_GetMenuItem(self.as_ptr())) }
    }
    fn set<M: MenuItemMethods>(&self, flags: c_int, key_code: c_int, cmd: c_int, item: Option<&M>) {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxAcceleratorEntry_Set(self.as_ptr(), flags, key_code, cmd, item)
        }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxAcceleratorEntry_IsOk(self.as_ptr()) }
    }
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAcceleratorEntry_ToString(self.as_ptr())).into() }
    }
    fn to_raw_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAcceleratorEntry_ToRawString(self.as_ptr())).into() }
    }
    fn from_string(&self, str: &str) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxAcceleratorEntry_FromString(self.as_ptr(), str)
        }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
}

// wxActivateEvent
pub trait ActivateEventMethods: EventMethods {
    fn get_active(&self) -> bool {
        unsafe { ffi::wxActivateEvent_GetActive(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetActivationReason()
}

// wxActivityIndicator
pub trait ActivityIndicatorMethods: ControlMethods {
    fn start(&self) {
        unsafe { ffi::wxActivityIndicator_Start(self.as_ptr()) }
    }
    fn stop(&self) {
        unsafe { ffi::wxActivityIndicator_Stop(self.as_ptr()) }
    }
    fn is_running(&self) -> bool {
        unsafe { ffi::wxActivityIndicator_IsRunning(self.as_ptr()) }
    }
}

// wxAddRemoveCtrl
pub trait AddRemoveCtrlMethods: PanelMethods {
    fn set_adaptor(&self, adaptor: *mut c_void) {
        unsafe { ffi::wxAddRemoveCtrl_SetAdaptor(self.as_ptr(), adaptor) }
    }
    fn set_buttons_tool_tips(&self, addtip: &str, removetip: &str) {
        unsafe {
            let addtip = WxString::from(addtip);
            let addtip = addtip.as_ptr();
            let removetip = WxString::from(removetip);
            let removetip = removetip.as_ptr();
            ffi::wxAddRemoveCtrl_SetButtonsToolTips(self.as_ptr(), addtip, removetip)
        }
    }
}

// wxAnimationCtrl
pub trait AnimationCtrlMethods: ControlMethods {
    fn create_animation<W: WindowMethods, A: AnimationMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        anim: &A,
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
            let anim = anim.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxAnimationCtrl_Create(self.as_ptr(), parent, id, anim, pos, size, style, name)
        }
    }
    fn create_animation(&self) -> Animation {
        unsafe { Animation::from_ptr(ffi::wxAnimationCtrl_CreateAnimation(self.as_ptr())) }
    }
    fn get_animation(&self) -> Animation {
        unsafe { Animation::from_ptr(ffi::wxAnimationCtrl_GetAnimation(self.as_ptr())) }
    }
    fn get_inactive_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnimationCtrl_GetInactiveBitmap(self.as_ptr())) }
    }
    fn is_playing(&self) -> bool {
        unsafe { ffi::wxAnimationCtrl_IsPlaying(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn Load()
    fn play(&self) -> bool {
        unsafe { ffi::wxAnimationCtrl_Play(self.as_ptr()) }
    }
    fn set_animation<A: AnimationMethods>(&self, anim: &A) {
        unsafe {
            let anim = anim.as_ptr();
            ffi::wxAnimationCtrl_SetAnimation(self.as_ptr(), anim)
        }
    }
    fn set_inactive_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxAnimationCtrl_SetInactiveBitmap(self.as_ptr(), bmp)
        }
    }
    fn stop(&self) {
        unsafe { ffi::wxAnimationCtrl_Stop(self.as_ptr()) }
    }
    fn create_compatible_animation() -> Animation {
        unsafe { Animation::from_ptr(ffi::wxAnimationCtrl_CreateCompatibleAnimation()) }
    }
}

// wxAnyButton
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_current(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapCurrent(self.as_ptr())) }
    }
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapDisabled(self.as_ptr())) }
    }
    fn get_bitmap_focus(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapFocus(self.as_ptr())) }
    }
    fn get_bitmap_label(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapLabel(self.as_ptr())) }
    }
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapPressed(self.as_ptr())) }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B, dir: c_int) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmap(self.as_ptr(), bitmap, dir)
        }
    }
    fn set_bitmap_current<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_disabled<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_focus<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapFocus(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_label<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapLabel(self.as_ptr(), bitmap)
        }
    }
    fn set_bitmap_pressed<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapPressed(self.as_ptr(), bitmap)
        }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr())) }
    }
    fn set_bitmap_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr(), x, y) }
    }
    fn set_bitmap_margins_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr(), sz)
        }
    }
    fn set_bitmap_position(&self, dir: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapPosition(self.as_ptr(), dir) }
    }
}

// wxArtProvider
pub trait ArtProviderMethods: ObjectMethods {
    // DTOR: fn ~wxArtProvider()
    fn delete<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Delete(provider)
        }
    }
    fn get_bitmap<S: SizeMethods>(id: &str, client: &str, size: &S) -> Bitmap {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            let size = size.as_ptr();
            Bitmap::from_ptr(ffi::wxArtProvider_GetBitmap(id, client, size))
        }
    }
    fn get_bitmap_bundle<S: SizeMethods>(id: &str, client: &str, size: &S) -> BitmapBundle {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            let size = size.as_ptr();
            BitmapBundle::from_ptr(ffi::wxArtProvider_GetBitmapBundle(id, client, size))
        }
    }
    fn get_icon<S: SizeMethods>(id: &str, client: &str, size: &S) -> Icon {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxArtProvider_GetIcon(id, client, size))
        }
    }
    fn get_native_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetNativeDIPSizeHint(client))
        }
    }
    fn get_native_size_hint<W: WindowMethods>(client: &str, win: Option<&W>) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxArtProvider_GetNativeSizeHint(client, win))
        }
    }
    fn get_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetDIPSizeHint(client))
        }
    }
    fn get_size_hint<W: WindowMethods>(client: &str, win: Option<&W>) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxArtProvider_GetSizeHint(client, win))
        }
    }
    fn get_icon_bundle(id: &str, client: &str) -> IconBundle {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            IconBundle::from_ptr(ffi::wxArtProvider_GetIconBundle(id, client))
        }
    }
    fn has_native_provider() -> bool {
        unsafe { ffi::wxArtProvider_HasNativeProvider() }
    }
    // BLOCKED: fn Insert()
    fn pop() -> bool {
        unsafe { ffi::wxArtProvider_Pop() }
    }
    fn push<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Push(provider)
        }
    }
    fn push_back<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_PushBack(provider)
        }
    }
    fn remove<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Remove(provider)
        }
    }
    fn get_message_box_icon_id(flags: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxArtProvider_GetMessageBoxIconId(flags)).into() }
    }
    fn get_message_box_icon(flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxArtProvider_GetMessageBoxIcon(flags)) }
    }
}

// wxAutoBufferedPaintDC
pub trait AutoBufferedPaintDCMethods: BufferedPaintDCMethods {}

// wxBannerWindow
pub trait BannerWindowMethods: WindowMethods {
    fn create_direction<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
        dir: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBannerWindow_Create(self.as_ptr(), parent, winid, dir, pos, size, style, name)
        }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxBannerWindow_SetBitmap(self.as_ptr(), bmp)
        }
    }
    fn set_text(&self, title: &str, message: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxBannerWindow_SetText(self.as_ptr(), title, message)
        }
    }
    fn set_gradient<C: ColourMethods, C2: ColourMethods>(&self, start: &C, end: &C2) {
        unsafe {
            let start = start.as_ptr();
            let end = end.as_ptr();
            ffi::wxBannerWindow_SetGradient(self.as_ptr(), start, end)
        }
    }
}

// wxBitmap
pub trait BitmapMethods: GDIObjectMethods {
    // DTOR: fn ~wxBitmap()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    // NOT_SUPPORTED: fn ConvertToImage()
    fn copy_from_icon<I: IconMethods>(&self, icon: &I) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxBitmap_CopyFromIcon(self.as_ptr(), icon)
        }
    }
    fn create_int_int(&self, width: c_int, height: c_int, depth: c_int) -> bool {
        unsafe { ffi::wxBitmap_Create(self.as_ptr(), width, height, depth) }
    }
    fn create_size<S: SizeMethods>(&self, sz: &S, depth: c_int) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxBitmap_Create1(self.as_ptr(), sz, depth)
        }
    }
    fn create_int_dc<D: DCMethods>(&self, width: c_int, height: c_int, dc: &D) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxBitmap_Create2(self.as_ptr(), width, height, dc)
        }
    }
    fn create_with_dip_size_size<S: SizeMethods>(
        &self,
        size: &S,
        scale: c_double,
        depth: c_int,
    ) -> bool {
        unsafe {
            let size = size.as_ptr();
            ffi::wxBitmap_CreateWithDIPSize(self.as_ptr(), size, scale, depth)
        }
    }
    fn create_with_dip_size_int(
        &self,
        width: c_int,
        height: c_int,
        scale: c_double,
        depth: c_int,
    ) -> bool {
        unsafe { ffi::wxBitmap_CreateWithDIPSize1(self.as_ptr(), width, height, scale, depth) }
    }
    fn create_scaled(
        &self,
        width: c_int,
        height: c_int,
        depth: c_int,
        logical_scale: c_double,
    ) -> bool {
        unsafe { ffi::wxBitmap_CreateScaled(self.as_ptr(), width, height, depth, logical_scale) }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetDepth(self.as_ptr()) }
    }
    fn get_dip_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetDIPSize(self.as_ptr())) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetHeight(self.as_ptr()) }
    }
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalHeight(self.as_ptr()) }
    }
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetLogicalSize(self.as_ptr())) }
    }
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalWidth(self.as_ptr()) }
    }
    fn get_mask(&self) -> Option<MaskIsOwned<false>> {
        unsafe { Mask::option_from(ffi::wxBitmap_GetMask(self.as_ptr())) }
    }
    fn get_palette(&self) -> Option<PaletteIsOwned<false>> {
        unsafe { Palette::option_from(ffi::wxBitmap_GetPalette(self.as_ptr())) }
    }
    fn get_sub_bitmap<R: RectMethods>(&self, rect: &R) -> Bitmap {
        unsafe {
            let rect = rect.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmap_GetSubBitmap(self.as_ptr(), rect))
        }
    }
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaleFactor(self.as_ptr()) }
    }
    fn get_scaled_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledHeight(self.as_ptr()) }
    }
    fn get_scaled_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetScaledSize(self.as_ptr())) }
    }
    fn get_scaled_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledWidth(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetSize(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetWidth(self.as_ptr()) }
    }
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxBitmap_HasAlpha(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmap_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // BLOCKED: fn ResetAlpha()
    // NOT_SUPPORTED: fn SaveFile()
    fn set_depth(&self, depth: c_int) {
        unsafe { ffi::wxBitmap_SetDepth(self.as_ptr(), depth) }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxBitmap_SetHeight(self.as_ptr(), height) }
    }
    fn set_scale_factor(&self, scale: c_double) {
        unsafe { ffi::wxBitmap_SetScaleFactor(self.as_ptr(), scale) }
    }
    fn set_mask<M: MaskMethods>(&self, mask: Option<&M>) {
        unsafe {
            let mask = match mask {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmap_SetMask(self.as_ptr(), mask)
        }
    }
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxBitmap_SetPalette(self.as_ptr(), palette)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxBitmap_SetWidth(self.as_ptr(), width) }
    }
    // BLOCKED: fn UseAlpha()
    fn add_handler<B: BitmapHandlerMethods>(handler: Option<&B>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmap_AddHandler(handler)
        }
    }
    fn clean_up_handlers() {
        unsafe { ffi::wxBitmap_CleanUpHandlers() }
    }
    fn find_handler(name: &str) -> Option<BitmapHandlerIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapHandler::option_from(ffi::wxBitmap_FindHandler(name))
        }
    }
    // NOT_SUPPORTED: fn FindHandler1()
    // NOT_SUPPORTED: fn FindHandler2()
    // BLOCKED: fn GetHandlers()
    fn init_standard_handlers() {
        unsafe { ffi::wxBitmap_InitStandardHandlers() }
    }
    fn insert_handler<B: BitmapHandlerMethods>(handler: Option<&B>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmap_InsertHandler(handler)
        }
    }
    fn new_from_png_data(data: *const c_void, size: usize) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmap_NewFromPNGData(data, size)) }
    }
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmap_RemoveHandler(name)
        }
    }
    fn rescale<B: BitmapMethods, S: SizeMethods>(bmp: &B, size_needed: &S) {
        unsafe {
            let bmp = bmp.as_ptr();
            let size_needed = size_needed.as_ptr();
            ffi::wxBitmap_Rescale(bmp, size_needed)
        }
    }
}

// wxBitmapBundle
pub trait BitmapBundleMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    fn clear(&self) {
        unsafe { ffi::wxBitmapBundle_Clear(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmapBundle_IsOk(self.as_ptr()) }
    }
    fn get_default_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapBundle_GetDefaultSize(self.as_ptr())) }
    }
    fn get_preferred_bitmap_size_at_scale(&self, scale: c_double) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredBitmapSizeAtScale(
                self.as_ptr(),
                scale,
            ))
        }
    }
    fn get_preferred_bitmap_size_for<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredBitmapSizeFor(
                self.as_ptr(),
                window,
            ))
        }
    }
    fn get_preferred_logical_size_for<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredLogicalSizeFor(
                self.as_ptr(),
                window,
            ))
        }
    }
    fn get_bitmap<S: SizeMethods>(&self, size: &S) -> Bitmap {
        unsafe {
            let size = size.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmap(self.as_ptr(), size))
        }
    }
    fn get_bitmap_for<W: WindowMethods>(&self, window: Option<&W>) -> Bitmap {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmapFor(self.as_ptr(), window))
        }
    }
    fn get_icon<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxBitmapBundle_GetIcon(self.as_ptr(), size))
        }
    }
    fn get_icon_for<W: WindowMethods>(&self, window: Option<&W>) -> Icon {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Icon::from_ptr(ffi::wxBitmapBundle_GetIconFor(self.as_ptr(), window))
        }
    }
    fn is_same_as<B: BitmapBundleMethods>(&self, other: &B) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxBitmapBundle_IsSameAs(self.as_ptr(), other)
        }
    }
    // BLOCKED: fn FromBitmaps()
    fn from_bitmaps<B: BitmapMethods, B2: BitmapMethods>(
        bitmap1: &B,
        bitmap2: &B2,
    ) -> BitmapBundle {
        unsafe {
            let bitmap1 = bitmap1.as_ptr();
            let bitmap2 = bitmap2.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromBitmaps1(bitmap1, bitmap2))
        }
    }
    fn from_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundle {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromBitmap(bitmap))
        }
    }
    fn from_icon_bundle<I: IconBundleMethods>(icon_bundle: &I) -> BitmapBundle {
        unsafe {
            let icon_bundle = icon_bundle.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromIconBundle(icon_bundle))
        }
    }
    fn from_image(image: *const c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImage(image)) }
    }
    fn from_impl(impl_: *mut c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImpl(impl_)) }
    }
    fn from_resources(name: &str) -> BitmapBundle {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromResources(name))
        }
    }
    fn from_files_str(path: &str, filename: &str, extension: &str) -> BitmapBundle {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromFiles(path, filename, extension))
        }
    }
    fn from_files(fullpathname: &str) -> BitmapBundle {
        unsafe {
            let fullpathname = WxString::from(fullpathname);
            let fullpathname = fullpathname.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromFiles1(fullpathname))
        }
    }
    // BLOCKED: fn FromSVG()
    fn from_svg<S: SizeMethods>(data: *const c_void, size_def: &S) -> BitmapBundle {
        unsafe {
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVG1(data, size_def))
        }
    }
    fn from_svg_file<S: SizeMethods>(path: &str, size_def: &S) -> BitmapBundle {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVGFile(path, size_def))
        }
    }
    fn from_svg_resource<S: SizeMethods>(name: &str, size_def: &S) -> BitmapBundle {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVGResource(name, size_def))
        }
    }
}

// wxBitmapButton
pub trait BitmapButtonMethods: ButtonMethods {
    fn create_bitmapbundle<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        bitmap: &B,
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
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapButton_Create(
                self.as_ptr(),
                parent,
                id,
                bitmap,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn create_close_button<W: WindowMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapButton_CreateCloseButton(self.as_ptr(), parent, winid, name)
        }
    }
    fn new_close_button<W: WindowMethods>(
        parent: Option<&W>,
        winid: c_int,
        name: &str,
    ) -> WeakRef<BitmapButton> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<BitmapButton>::from(ffi::wxBitmapButton_NewCloseButton(parent, winid, name))
        }
    }
}

// wxBitmapComboBox
pub trait BitmapComboBoxMethods: ComboBoxMethods {
    // DTOR: fn ~wxBitmapComboBox()
    fn append<B: BitmapMethods>(&self, item: &str, bitmap: &B) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Append(self.as_ptr(), item, bitmap)
        }
    }
    fn append_void<B: BitmapMethods>(
        &self,
        item: &str,
        bitmap: &B,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Append1(self.as_ptr(), item, bitmap, client_data)
        }
    }
    fn append_clientdata<B: BitmapMethods, C: ClientDataMethods>(
        &self,
        item: &str,
        bitmap: &B,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmapComboBox_Append2(self.as_ptr(), item, bitmap, client_data)
        }
    }
    fn get_bitmap_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapComboBox_GetBitmapSize(self.as_ptr())) }
    }
    fn get_item_bitmap(&self, n: c_uint) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmapComboBox_GetItemBitmap(self.as_ptr(), n)) }
    }
    fn insert<B: BitmapMethods>(&self, item: &str, bitmap: &B, pos: c_uint) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Insert(self.as_ptr(), item, bitmap, pos)
        }
    }
    fn insert_void<B: BitmapMethods>(
        &self,
        item: &str,
        bitmap: &B,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Insert1(self.as_ptr(), item, bitmap, pos, client_data)
        }
    }
    fn insert_clientdata<B: BitmapMethods, C: ClientDataMethods>(
        &self,
        item: &str,
        bitmap: &B,
        pos: c_uint,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmapComboBox_Insert2(self.as_ptr(), item, bitmap, pos, client_data)
        }
    }
    fn set_item_bitmap<B: BitmapBundleMethods>(&self, n: c_uint, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_SetItemBitmap(self.as_ptr(), n, bitmap)
        }
    }
}

// wxBitmapHandler
pub trait BitmapHandlerMethods: ObjectMethods {
    // DTOR: fn ~wxBitmapHandler()
    // NOT_SUPPORTED: fn Create()
    fn get_extension(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxBitmapHandler_GetExtension(self.as_ptr())).into() }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxBitmapHandler_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn SaveFile()
    fn set_extension(&self, extension: &str) {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            ffi::wxBitmapHandler_SetExtension(self.as_ptr(), extension)
        }
    }
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapHandler_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetType()
}

// wxBitmapToggleButton
pub trait BitmapToggleButtonMethods: ToggleButtonMethods {
    fn create_bitmapbundle<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &B,
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
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let val = val.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmapToggleButton_Create(
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
}

// wxBookCtrlBase
pub trait BookCtrlBaseMethods: ControlMethods {
    fn get_page_image(&self, n_page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetPageImage(self.as_ptr(), n_page) }
    }
    fn set_page_image(&self, page: usize, image: c_int) -> bool {
        unsafe { ffi::wxBookCtrlBase_SetPageImage(self.as_ptr(), page, image) }
    }
    fn get_page_text(&self, n_page: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxBookCtrlBase_GetPageText(self.as_ptr(), n_page)).into() }
    }
    fn set_page_text(&self, page: usize, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxBookCtrlBase_SetPageText(self.as_ptr(), page, text)
        }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetSelection(self.as_ptr()) }
    }
    fn get_current_page(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetCurrentPage(self.as_ptr())) }
    }
    fn set_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_SetSelection(self.as_ptr(), page) }
    }
    fn advance_selection(&self, forward: bool) {
        unsafe { ffi::wxBookCtrlBase_AdvanceSelection(self.as_ptr(), forward) }
    }
    fn change_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_ChangeSelection(self.as_ptr(), page) }
    }
    fn find_page<W: WindowMethods>(&self, page: Option<&W>) -> c_int {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBookCtrlBase_FindPage(self.as_ptr(), page)
        }
    }
    fn set_page_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxBookCtrlBase_SetPageSize(self.as_ptr(), size)
        }
    }
    fn hit_test<P: PointMethods>(&self, pt: &P, flags: *mut c_void) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxBookCtrlBase_HitTest(self.as_ptr(), pt, flags)
        }
    }
    fn add_page<W: WindowMethods>(
        &self,
        page: Option<&W>,
        text: &str,
        select: bool,
        image_id: c_int,
    ) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxBookCtrlBase_AddPage(self.as_ptr(), page, text, select, image_id)
        }
    }
    fn delete_all_pages(&self) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeleteAllPages(self.as_ptr()) }
    }
    fn delete_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeletePage(self.as_ptr(), page) }
    }
    fn insert_page<W: WindowMethods>(
        &self,
        index: usize,
        page: Option<&W>,
        text: &str,
        select: bool,
        image_id: c_int,
    ) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxBookCtrlBase_InsertPage(self.as_ptr(), index, page, text, select, image_id)
        }
    }
    fn remove_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_RemovePage(self.as_ptr(), page) }
    }
    fn get_page_count(&self) -> usize {
        unsafe { ffi::wxBookCtrlBase_GetPageCount(self.as_ptr()) }
    }
    fn get_page(&self, page: usize) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetPage(self.as_ptr(), page)) }
    }
}

// wxBookCtrlEvent
pub trait BookCtrlEventMethods: NotifyEventMethods {
    fn get_old_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetOldSelection(self.as_ptr()) }
    }
    fn set_old_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetOldSelection(self.as_ptr(), page) }
    }
    fn set_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetSelection(self.as_ptr(), page) }
    }
}

// wxBoxSizer
pub trait BoxSizerMethods: SizerMethods {
    fn get_orientation(&self) -> c_int {
        unsafe { ffi::wxBoxSizer_GetOrientation(self.as_ptr()) }
    }
    fn set_orientation(&self, orient: c_int) {
        unsafe { ffi::wxBoxSizer_SetOrientation(self.as_ptr(), orient) }
    }
}

// wxBrush
pub trait BrushMethods: GDIObjectMethods {
    // DTOR: fn ~wxBrush()
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxBrush_GetColour(self.as_ptr())) }
    }
    fn get_stipple(&self) -> Option<BitmapIsOwned<false>> {
        unsafe { Bitmap::option_from(ffi::wxBrush_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    fn is_hatch(&self) -> bool {
        unsafe { ffi::wxBrush_IsHatch(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBrush_IsOk(self.as_ptr()) }
    }
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxBrush_IsNonTransparent(self.as_ptr()) }
    }
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxBrush_IsTransparent(self.as_ptr()) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxBrush_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    fn set_stipple<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxBrush_SetStipple(self.as_ptr(), bitmap)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxBrushList
pub trait BrushListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreateBrush()
}

// wxBufferedDC
pub trait BufferedDCMethods: MemoryDCMethods {
    // DTOR: fn ~wxBufferedDC()
    fn init_size<D: DCMethods, S: SizeMethods>(&self, dc: Option<&D>, area: &S, style: c_int) {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let area = area.as_ptr();
            ffi::wxBufferedDC_Init(self.as_ptr(), dc, area, style)
        }
    }
    fn init_bitmap<D: DCMethods, B: BitmapMethods>(
        &self,
        dc: Option<&D>,
        buffer: &B,
        style: c_int,
    ) {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            ffi::wxBufferedDC_Init1(self.as_ptr(), dc, buffer, style)
        }
    }
    fn un_mask(&self) {
        unsafe { ffi::wxBufferedDC_UnMask(self.as_ptr()) }
    }
    fn set_style(&self, style: c_int) {
        unsafe { ffi::wxBufferedDC_SetStyle(self.as_ptr(), style) }
    }
    fn get_style(&self) -> c_int {
        unsafe { ffi::wxBufferedDC_GetStyle(self.as_ptr()) }
    }
}

// wxBufferedPaintDC
pub trait BufferedPaintDCMethods: BufferedDCMethods {
    // DTOR: fn ~wxBufferedPaintDC()
}

// wxBusyCursor
pub trait BusyCursorMethods: WxRustMethods {
    // DTOR: fn ~wxBusyCursor()
}

// wxBusyInfo
pub trait BusyInfoMethods: WxRustMethods {
    fn update_text(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxBusyInfo_UpdateText(self.as_ptr(), str)
        }
    }
    fn update_label(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxBusyInfo_UpdateLabel(self.as_ptr(), str)
        }
    }
    // DTOR: fn ~wxBusyInfo()
}

// wxButton
pub trait ButtonMethods: AnyButtonMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxButton_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr()) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr(), needed) }
    }
    fn set_default(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxButton_SetDefault(self.as_ptr())) }
    }
    fn get_default_size<W: WindowMethods>(win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxButton_GetDefaultSize(win))
        }
    }
}

// wxCalculateLayoutEvent
pub trait CalculateLayoutEventMethods: EventMethods {
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxCalculateLayoutEvent_GetFlags(self.as_ptr()) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxCalculateLayoutEvent_GetRect(self.as_ptr())) }
    }
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxCalculateLayoutEvent_SetFlags(self.as_ptr(), flags) }
    }
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxCalculateLayoutEvent_SetRect(self.as_ptr(), rect)
        }
    }
}

// wxCalendarCtrl
pub trait CalendarCtrlMethods: ControlMethods {
    fn set_date_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        lowerdate: &D,
        upperdate: &D2,
    ) -> bool {
        unsafe {
            let lowerdate = lowerdate.as_ptr();
            let upperdate = upperdate.as_ptr();
            ffi::wxCalendarCtrl_SetDateRange(self.as_ptr(), lowerdate, upperdate)
        }
    }
    fn get_date_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        lowerdate: Option<&D>,
        upperdate: Option<&D2>,
    ) -> bool {
        unsafe {
            let lowerdate = match lowerdate {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let upperdate = match upperdate {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCalendarCtrl_GetDateRange(self.as_ptr(), lowerdate, upperdate)
        }
    }
    // DTOR: fn ~wxCalendarCtrl()
    fn create_datetime<W: WindowMethods, D: DateTimeMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        date: &D,
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
            let date = date.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCalendarCtrl_Create(self.as_ptr(), parent, id, date, pos, size, style, name)
        }
    }
    fn enable_holiday_display(&self, display: bool) {
        unsafe { ffi::wxCalendarCtrl_EnableHolidayDisplay(self.as_ptr(), display) }
    }
    fn enable_month_change(&self, enable: bool) -> bool {
        unsafe { ffi::wxCalendarCtrl_EnableMonthChange(self.as_ptr(), enable) }
    }
    fn enable_year_change(&self, enable: bool) {
        unsafe { ffi::wxCalendarCtrl_EnableYearChange(self.as_ptr(), enable) }
    }
    fn get_attr(&self, day: usize) -> Option<CalendarDateAttrIsOwned<false>> {
        unsafe { CalendarDateAttr::option_from(ffi::wxCalendarCtrl_GetAttr(self.as_ptr(), day)) }
    }
    fn get_date(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxCalendarCtrl_GetDate(self.as_ptr())) }
    }
    fn get_header_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourBg(self.as_ptr())) }
    }
    fn get_header_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourFg(self.as_ptr())) }
    }
    fn get_highlight_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourBg(self.as_ptr())) }
    }
    fn get_highlight_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourFg(self.as_ptr())) }
    }
    fn get_holiday_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourBg(self.as_ptr())) }
    }
    fn get_holiday_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourFg(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn HitTest()
    fn reset_attr(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_ResetAttr(self.as_ptr(), day) }
    }
    fn set_attr<C: CalendarDateAttrMethods>(&self, day: usize, attr: Option<&C>) {
        unsafe {
            let attr = match attr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCalendarCtrl_SetAttr(self.as_ptr(), day, attr)
        }
    }
    fn set_date<D: DateTimeMethods>(&self, date: &D) -> bool {
        unsafe {
            let date = date.as_ptr();
            ffi::wxCalendarCtrl_SetDate(self.as_ptr(), date)
        }
    }
    fn set_header_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHeaderColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    fn set_highlight_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHighlightColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    fn set_holiday(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_SetHoliday(self.as_ptr(), day) }
    }
    fn set_holiday_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHolidayColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    fn mark(&self, day: usize, mark: bool) {
        unsafe { ffi::wxCalendarCtrl_Mark(self.as_ptr(), day, mark) }
    }
}

// wxCalendarDateAttr
pub trait CalendarDateAttrMethods: WxRustMethods {
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    // NOT_SUPPORTED: fn GetBorder()
    fn get_border_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetBorderColour(self.as_ptr())) }
    }
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetFont(self.as_ptr())) }
    }
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetTextColour(self.as_ptr())) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn has_border(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorder(self.as_ptr()) }
    }
    fn has_border_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorderColour(self.as_ptr()) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasFont(self.as_ptr()) }
    }
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasTextColour(self.as_ptr()) }
    }
    fn is_holiday(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_IsHoliday(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxCalendarDateAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    // NOT_SUPPORTED: fn SetBorder()
    fn set_border_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxCalendarDateAttr_SetBorderColour(self.as_ptr(), col)
        }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxCalendarDateAttr_SetFont(self.as_ptr(), font)
        }
    }
    fn set_holiday(&self, holiday: bool) {
        unsafe { ffi::wxCalendarDateAttr_SetHoliday(self.as_ptr(), holiday) }
    }
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxCalendarDateAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    fn get_mark() -> CalendarDateAttrIsOwned<false> {
        unsafe { CalendarDateAttrIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetMark()) }
    }
    fn set_mark<C: CalendarDateAttrMethods>(m: &C) {
        unsafe {
            let m = m.as_ptr();
            ffi::wxCalendarDateAttr_SetMark(m)
        }
    }
}

// wxCalendarEvent
pub trait CalendarEventMethods: DateEventMethods {
    // NOT_SUPPORTED: fn GetWeekDay()
    // NOT_SUPPORTED: fn SetWeekDay()
}

// wxCaret
pub trait CaretMethods: WxRustMethods {
    fn create_int<W: WindowMethods>(
        &self,
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCaret_Create(self.as_ptr(), window, width, height)
        }
    }
    fn create_size<W: WindowMethods, S: SizeMethods>(&self, window: Option<&W>, size: &S) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxCaret_Create1(self.as_ptr(), window, size)
        }
    }
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxCaret_GetPosition(self.as_ptr(), x, y) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCaret_GetPosition1(self.as_ptr())) }
    }
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxCaret_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxCaret_GetSize1(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCaret_GetWindow(self.as_ptr())) }
    }
    fn hide(&self) {
        unsafe { ffi::wxCaret_Hide(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCaret_IsOk(self.as_ptr()) }
    }
    fn is_visible(&self) -> bool {
        unsafe { ffi::wxCaret_IsVisible(self.as_ptr()) }
    }
    fn move_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxCaret_Move(self.as_ptr(), x, y) }
    }
    fn move_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxCaret_Move1(self.as_ptr(), pt)
        }
    }
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxCaret_SetSize(self.as_ptr(), width, height) }
    }
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxCaret_SetSize1(self.as_ptr(), size)
        }
    }
    fn show(&self, show: bool) {
        unsafe { ffi::wxCaret_Show(self.as_ptr(), show) }
    }
    fn get_blink_time() -> c_int {
        unsafe { ffi::wxCaret_GetBlinkTime() }
    }
    fn set_blink_time(milliseconds: c_int) {
        unsafe { ffi::wxCaret_SetBlinkTime(milliseconds) }
    }
}

// wxCheckBox
pub trait CheckBoxMethods: ControlMethods {
    // DTOR: fn ~wxCheckBox()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCheckBox_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_value(&self) -> bool {
        unsafe { ffi::wxCheckBox_GetValue(self.as_ptr()) }
    }
    fn get3_state_value(&self) -> c_int {
        unsafe { ffi::wxCheckBox_Get3StateValue(self.as_ptr()) }
    }
    fn is3_state(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3State(self.as_ptr()) }
    }
    fn is3rd_state_allowed_for_user(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3rdStateAllowedForUser(self.as_ptr()) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCheckBox_IsChecked(self.as_ptr()) }
    }
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxCheckBox_SetValue(self.as_ptr(), state) }
    }
    fn set3_state_value(&self, state: c_int) {
        unsafe { ffi::wxCheckBox_Set3StateValue(self.as_ptr(), state) }
    }
}

// wxCheckListBox
pub trait CheckListBoxMethods: ListBoxMethods {
    // DTOR: fn ~wxCheckListBox()
    fn check(&self, item: c_uint, check: bool) {
        unsafe { ffi::wxCheckListBox_Check(self.as_ptr(), item, check) }
    }
    fn is_checked(&self, item: c_uint) -> bool {
        unsafe { ffi::wxCheckListBox_IsChecked(self.as_ptr(), item) }
    }
    fn get_checked_items<A: ArrayIntMethods>(&self, checked_items: &A) -> c_uint {
        unsafe {
            let checked_items = checked_items.as_ptr();
            ffi::wxCheckListBox_GetCheckedItems(self.as_ptr(), checked_items)
        }
    }
}

// wxChildFocusEvent
pub trait ChildFocusEventMethods: CommandEventMethods {
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxChildFocusEvent_GetWindow(self.as_ptr())) }
    }
}

// wxChoice
pub trait ChoiceMethods: ControlMethods {
    // DTOR: fn ~wxChoice()
    // NOT_SUPPORTED: fn Create()
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxChoice_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    fn get_columns(&self) -> c_int {
        unsafe { ffi::wxChoice_GetColumns(self.as_ptr()) }
    }
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxChoice_GetCurrentSelection(self.as_ptr()) }
    }
    fn set_columns(&self, n: c_int) {
        unsafe { ffi::wxChoice_SetColumns(self.as_ptr(), n) }
    }
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxChoice_IsSorted(self.as_ptr()) }
    }
}

// wxChoicebook
pub trait ChoicebookMethods: BookCtrlBaseMethods {
    fn get_choice_ctrl(&self) -> WeakRef<Choice> {
        unsafe { WeakRef::<Choice>::from(ffi::wxChoicebook_GetChoiceCtrl(self.as_ptr())) }
    }
}

// wxClientDC
pub trait ClientDCMethods: WindowDCMethods {}

// wxClipboard
pub trait ClipboardMethods: ObjectMethods {
    // DTOR: fn ~wxClipboard()
    fn add_data(&self, data: *mut c_void) -> bool {
        unsafe { ffi::wxClipboard_AddData(self.as_ptr(), data) }
    }
    fn clear(&self) {
        unsafe { ffi::wxClipboard_Clear(self.as_ptr()) }
    }
    fn close(&self) {
        unsafe { ffi::wxClipboard_Close(self.as_ptr()) }
    }
    fn flush(&self) -> bool {
        unsafe { ffi::wxClipboard_Flush(self.as_ptr()) }
    }
    fn get_data(&self, data: *mut c_void) -> bool {
        unsafe { ffi::wxClipboard_GetData(self.as_ptr(), data) }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxClipboard_IsOpened(self.as_ptr()) }
    }
    fn is_supported(&self, format: *const c_void) -> bool {
        unsafe { ffi::wxClipboard_IsSupported(self.as_ptr(), format) }
    }
    fn is_using_primary_selection(&self) -> bool {
        unsafe { ffi::wxClipboard_IsUsingPrimarySelection(self.as_ptr()) }
    }
    fn open(&self) -> bool {
        unsafe { ffi::wxClipboard_Open(self.as_ptr()) }
    }
    fn set_data(&self, data: *mut c_void) -> bool {
        unsafe { ffi::wxClipboard_SetData(self.as_ptr(), data) }
    }
    fn use_primary_selection(&self, primary: bool) {
        unsafe { ffi::wxClipboard_UsePrimarySelection(self.as_ptr(), primary) }
    }
    fn get() -> Option<ClipboardIsOwned<false>> {
        unsafe { Clipboard::option_from(ffi::wxClipboard_Get()) }
    }
}

// wxClipboardTextEvent
pub trait ClipboardTextEventMethods: CommandEventMethods {}

// wxCloseEvent
pub trait CloseEventMethods: EventMethods {
    fn can_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_CanVeto(self.as_ptr()) }
    }
    fn get_logging_off(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetLoggingOff(self.as_ptr()) }
    }
    fn set_can_veto(&self, can_veto: bool) {
        unsafe { ffi::wxCloseEvent_SetCanVeto(self.as_ptr(), can_veto) }
    }
    fn set_logging_off(&self, logging_off: bool) {
        unsafe { ffi::wxCloseEvent_SetLoggingOff(self.as_ptr(), logging_off) }
    }
    fn veto(&self, veto: bool) {
        unsafe { ffi::wxCloseEvent_Veto(self.as_ptr(), veto) }
    }
    fn get_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetVeto(self.as_ptr()) }
    }
}

// wxCollapsibleHeaderCtrl
pub trait CollapsibleHeaderCtrlMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCollapsibleHeaderCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn set_collapsed(&self, collapsed: bool) {
        unsafe { ffi::wxCollapsibleHeaderCtrl_SetCollapsed(self.as_ptr(), collapsed) }
    }
    fn is_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsibleHeaderCtrl_IsCollapsed(self.as_ptr()) }
    }
}

// wxCollapsiblePane
pub trait CollapsiblePaneMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCollapsiblePane_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn collapse(&self, collapse: bool) {
        unsafe { ffi::wxCollapsiblePane_Collapse(self.as_ptr(), collapse) }
    }
    fn expand(&self) {
        unsafe { ffi::wxCollapsiblePane_Expand(self.as_ptr()) }
    }
    fn get_pane(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCollapsiblePane_GetPane(self.as_ptr())) }
    }
    fn is_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsCollapsed(self.as_ptr()) }
    }
    fn is_expanded(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsExpanded(self.as_ptr()) }
    }
}

// wxCollapsiblePaneEvent
pub trait CollapsiblePaneEventMethods: CommandEventMethods {
    fn get_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePaneEvent_GetCollapsed(self.as_ptr()) }
    }
    fn set_collapsed(&self, collapsed: bool) {
        unsafe { ffi::wxCollapsiblePaneEvent_SetCollapsed(self.as_ptr(), collapsed) }
    }
}

// wxColour
pub trait ColourMethods: ObjectMethods {
    // NOT_SUPPORTED: fn Alpha()
    // NOT_SUPPORTED: fn Blue()
    fn get_alpha(&self) -> c_uint {
        unsafe { ffi::wxColour_GetAlpha(self.as_ptr()) }
    }
    fn get_blue(&self) -> c_uint {
        unsafe { ffi::wxColour_GetBlue(self.as_ptr()) }
    }
    fn get_green(&self) -> c_uint {
        unsafe { ffi::wxColour_GetGreen(self.as_ptr()) }
    }
    fn get_red(&self) -> c_uint {
        unsafe { ffi::wxColour_GetRed(self.as_ptr()) }
    }
    fn get_as_string(&self, flags: c_long) -> String {
        unsafe { WxString::from_ptr(ffi::wxColour_GetAsString(self.as_ptr(), flags)).into() }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGBA()
    // NOT_SUPPORTED: fn GetRGB()
    // NOT_SUPPORTED: fn GetRGBA()
    fn get_luminance(&self) -> c_double {
        unsafe { ffi::wxColour_GetLuminance(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    // NOT_SUPPORTED: fn Green()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxColour_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Red()
    fn is_solid(&self) -> bool {
        unsafe { ffi::wxColour_IsSolid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Set()
    // NOT_SUPPORTED: fn Set1()
    fn set(&self, str: &str) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxColour_Set2(self.as_ptr(), str)
        }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // NOT_SUPPORTED: fn MakeDisabled()
    // BLOCKED: fn ChangeLightness()
    fn make_mono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool) {
        unsafe { ffi::wxColour_MakeMono(r, g, b, on) }
    }
    // NOT_SUPPORTED: fn MakeDisabled1()
    fn make_grey(r: *mut c_void, g: *mut c_void, b: *mut c_void) {
        unsafe { ffi::wxColour_MakeGrey(r, g, b) }
    }
    fn make_grey_double(
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    ) {
        unsafe { ffi::wxColour_MakeGrey1(r, g, b, weight_r, weight_g, weight_b) }
    }
    // NOT_SUPPORTED: fn AlphaBlend()
    fn change_lightness(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int) {
        unsafe { ffi::wxColour_ChangeLightness1(r, g, b, ialpha) }
    }
}

// wxColourData
pub trait ColourDataMethods: ObjectMethods {
    // DTOR: fn ~wxColourData()
    fn get_choose_full(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseFull(self.as_ptr()) }
    }
    fn get_choose_alpha(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseAlpha(self.as_ptr()) }
    }
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxColourData_GetColour(self.as_ptr())) }
    }
    fn get_custom_colour(&self, i: c_int) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourData_GetCustomColour(self.as_ptr(), i)) }
    }
    fn set_choose_full(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseFull(self.as_ptr(), flag) }
    }
    fn set_choose_alpha(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseAlpha(self.as_ptr(), flag) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetColour(self.as_ptr(), colour)
        }
    }
    fn set_custom_colour<C: ColourMethods>(&self, i: c_int, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetCustomColour(self.as_ptr(), i, colour)
        }
    }
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxColourData_ToString(self.as_ptr())).into() }
    }
    fn from_string(&self, str: &str) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxColourData_FromString(self.as_ptr(), str)
        }
    }
    // BLOCKED: fn operator=()
}

// wxColourDatabase
pub trait ColourDatabaseMethods: WxRustMethods {
    fn add_colour<C: ColourMethods>(&self, colour_name: &str, colour: &C) {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxColourDatabase_AddColour(self.as_ptr(), colour_name, colour)
        }
    }
    fn find(&self, colour_name: &str) -> Colour {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            Colour::from_ptr(ffi::wxColourDatabase_Find(self.as_ptr(), colour_name))
        }
    }
    fn find_name<C: ColourMethods>(&self, colour: &C) -> String {
        unsafe {
            let colour = colour.as_ptr();
            WxString::from_ptr(ffi::wxColourDatabase_FindName(self.as_ptr(), colour)).into()
        }
    }
}

// wxColourDialogEvent
pub trait ColourDialogEventMethods: CommandEventMethods {
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourDialogEvent_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourDialogEvent_SetColour(self.as_ptr(), colour)
        }
    }
}

// wxColourPickerCtrl
pub trait ColourPickerCtrlMethods: PickerBaseMethods {
    fn create_colour<
        W: WindowMethods,
        C: ColourMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        colour: &C,
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
            let colour = colour.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxColourPickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                colour,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerCtrl_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxColourPickerCtrl_SetColour(self.as_ptr(), col)
        }
    }
    // BLOCKED: fn SetColour1()
}

// wxColourPickerEvent
pub trait ColourPickerEventMethods: CommandEventMethods {
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerEvent_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, pos: &C) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxColourPickerEvent_SetColour(self.as_ptr(), pos)
        }
    }
}

// wxComboBox
pub trait ComboBoxMethods: ControlMethods {
    // DTOR: fn ~wxComboBox()
    // NOT_SUPPORTED: fn Create()
    fn create_str<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
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
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxComboBox_Create1(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxComboBox_GetCurrentSelection(self.as_ptr()) }
    }
    fn is_list_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsListEmpty(self.as_ptr()) }
    }
    fn is_text_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsTextEmpty(self.as_ptr()) }
    }
    fn popup(&self) {
        unsafe { ffi::wxComboBox_Popup(self.as_ptr()) }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxComboBox_Dismiss(self.as_ptr()) }
    }
}

// wxCommandEvent
pub trait CommandEventMethods: EventMethods {
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientData(self.as_ptr()) }
    }
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe { ClientData::option_from(ffi::wxCommandEvent_GetClientObject(self.as_ptr())) }
    }
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxCommandEvent_GetExtraLong(self.as_ptr()) }
    }
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetInt(self.as_ptr()) }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetSelection(self.as_ptr()) }
    }
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandEvent_GetString(self.as_ptr())).into() }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsChecked(self.as_ptr()) }
    }
    fn is_selection(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsSelection(self.as_ptr()) }
    }
    fn set_client_data(&self, client_data: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientData(self.as_ptr(), client_data) }
    }
    fn set_client_object<C: ClientDataMethods>(&self, client_object: Option<&C>) {
        unsafe {
            let client_object = match client_object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandEvent_SetClientObject(self.as_ptr(), client_object)
        }
    }
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxCommandEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxCommandEvent_SetInt(self.as_ptr(), int_command) }
    }
    fn set_string(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxCommandEvent_SetString(self.as_ptr(), string)
        }
    }
}

// wxCommandLinkButton
pub trait CommandLinkButtonMethods: ButtonMethods {
    fn create_str_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        main_label: &str,
        note: &str,
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
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCommandLinkButton_Create(
                self.as_ptr(),
                parent,
                id,
                main_label,
                note,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn set_main_label_and_note(&self, main_label: &str, note: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabelAndNote(self.as_ptr(), main_label, note)
        }
    }
    fn set_main_label(&self, main_label: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabel(self.as_ptr(), main_label)
        }
    }
    fn set_note(&self, note: &str) {
        unsafe {
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetNote(self.as_ptr(), note)
        }
    }
    fn get_main_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetMainLabel(self.as_ptr())).into() }
    }
    fn get_note(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetNote(self.as_ptr())).into() }
    }
}

// wxContextHelp
pub trait ContextHelpMethods: ObjectMethods {
    // DTOR: fn ~wxContextHelp()
    fn begin_context_help<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxContextHelp_BeginContextHelp(self.as_ptr(), window)
        }
    }
    fn end_context_help(&self) -> bool {
        unsafe { ffi::wxContextHelp_EndContextHelp(self.as_ptr()) }
    }
}

// wxContextHelpButton
pub trait ContextHelpButtonMethods: BitmapButtonMethods {}

// wxContextMenuEvent
pub trait ContextMenuEventMethods: CommandEventMethods {
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxContextMenuEvent_GetPosition(self.as_ptr())) }
    }
    fn set_position<P: PointMethods>(&self, point: &P) {
        unsafe {
            let point = point.as_ptr();
            ffi::wxContextMenuEvent_SetPosition(self.as_ptr(), point)
        }
    }
}

// wxControl
pub trait ControlMethods: WindowMethods {
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxControl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
    fn command<C: CommandEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxControl_Command(self.as_ptr(), event)
        }
    }
    fn get_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxControl_GetLabelText(self.as_ptr())).into() }
    }
    fn get_size_from_text_size_int(&self, xlen: c_int, ylen: c_int) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize(
                self.as_ptr(),
                xlen,
                ylen,
            ))
        }
    }
    fn get_size_from_text_size_size<S: SizeMethods>(&self, tsize: &S) -> Size {
        unsafe {
            let tsize = tsize.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr(), tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromText(self.as_ptr(), text))
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxControl_SetLabelText(self.as_ptr(), text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = WxString::from(markup);
            let markup = markup.as_ptr();
            ffi::wxControl_SetLabelMarkup(self.as_ptr(), markup)
        }
    }
    fn get_label_text_str(label: &str) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WxString::from_ptr(ffi::wxControl_GetLabelText1(label)).into()
        }
    }
    fn remove_mnemonics(str: &str) -> String {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            WxString::from_ptr(ffi::wxControl_RemoveMnemonics(str)).into()
        }
    }
    fn escape_mnemonics(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxControl_EscapeMnemonics(text)).into()
        }
    }
    fn ellipsize<D: DCMethods>(
        label: &str,
        dc: &D,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let dc = dc.as_ptr();
            WxString::from_ptr(ffi::wxControl_Ellipsize(label, dc, mode, max_width, flags)).into()
        }
    }
}

// wxControlWithItems
pub trait ControlWithItemsMethods: ControlMethods {}

// wxCursor
pub trait CursorMethods: GDIObjectMethods {
    // DTOR: fn ~wxCursor()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCursor_IsOk(self.as_ptr()) }
    }
    fn get_hot_spot(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCursor_GetHotSpot(self.as_ptr())) }
    }
    // BLOCKED: fn operator=()
}

// wxDC
pub trait DCMethods: ObjectMethods {
    fn device_to_logical_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalX(self.as_ptr(), x) }
    }
    fn device_to_logical_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalXRel(self.as_ptr(), x) }
    }
    fn device_to_logical_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalY(self.as_ptr(), y) }
    }
    fn device_to_logical_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalYRel(self.as_ptr(), y) }
    }
    fn logical_to_device_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceX(self.as_ptr(), x) }
    }
    fn logical_to_device_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceXRel(self.as_ptr(), x) }
    }
    fn logical_to_device_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceY(self.as_ptr(), y) }
    }
    fn logical_to_device_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceYRel(self.as_ptr(), y) }
    }
    fn device_to_logical_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_DeviceToLogical(self.as_ptr(), x, y)) }
    }
    fn device_to_logical_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_DeviceToLogical1(self.as_ptr(), pt))
        }
    }
    fn device_to_logical_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_DeviceToLogicalRel(self.as_ptr(), x, y)) }
    }
    fn device_to_logical_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_DeviceToLogicalRel1(self.as_ptr(), dim))
        }
    }
    fn logical_to_device_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_LogicalToDevice(self.as_ptr(), x, y)) }
    }
    fn logical_to_device_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_LogicalToDevice1(self.as_ptr(), pt))
        }
    }
    fn logical_to_device_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_LogicalToDeviceRel(self.as_ptr(), x, y)) }
    }
    fn logical_to_device_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_LogicalToDeviceRel1(self.as_ptr(), dim))
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxDC_Clear(self.as_ptr()) }
    }
    fn draw_arc_coord(
        &self,
        x_start: c_int,
        y_start: c_int,
        x_end: c_int,
        y_end: c_int,
        xc: c_int,
        yc: c_int,
    ) {
        unsafe { ffi::wxDC_DrawArc(self.as_ptr(), x_start, y_start, x_end, y_end, xc, yc) }
    }
    fn draw_arc_point<P: PointMethods, P2: PointMethods, P3: PointMethods>(
        &self,
        pt_start: &P,
        pt_end: &P2,
        centre: &P3,
    ) {
        unsafe {
            let pt_start = pt_start.as_ptr();
            let pt_end = pt_end.as_ptr();
            let centre = centre.as_ptr();
            ffi::wxDC_DrawArc1(self.as_ptr(), pt_start, pt_end, centre)
        }
    }
    fn draw_bitmap_coord<B: BitmapMethods>(&self, bitmap: &B, x: c_int, y: c_int, use_mask: bool) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDC_DrawBitmap(self.as_ptr(), bitmap, x, y, use_mask)
        }
    }
    fn draw_bitmap_point<B: BitmapMethods, P: PointMethods>(
        &self,
        bmp: &B,
        pt: &P,
        use_mask: bool,
    ) {
        unsafe {
            let bmp = bmp.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawBitmap1(self.as_ptr(), bmp, pt, use_mask)
        }
    }
    fn draw_check_mark_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawCheckMark(self.as_ptr(), x, y, width, height) }
    }
    fn draw_check_mark_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawCheckMark1(self.as_ptr(), rect)
        }
    }
    fn draw_circle_coord(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { ffi::wxDC_DrawCircle(self.as_ptr(), x, y, radius) }
    }
    fn draw_circle_point<P: PointMethods>(&self, pt: &P, radius: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawCircle1(self.as_ptr(), pt, radius)
        }
    }
    fn draw_ellipse_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawEllipse(self.as_ptr(), x, y, width, height) }
    }
    fn draw_ellipse_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, size: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let size = size.as_ptr();
            ffi::wxDC_DrawEllipse1(self.as_ptr(), pt, size)
        }
    }
    fn draw_ellipse_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawEllipse2(self.as_ptr(), rect)
        }
    }
    fn draw_elliptic_arc_coord(
        &self,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        start: c_double,
        end: c_double,
    ) {
        unsafe { ffi::wxDC_DrawEllipticArc(self.as_ptr(), x, y, width, height, start, end) }
    }
    fn draw_elliptic_arc_point<P: PointMethods, S: SizeMethods>(
        &self,
        pt: &P,
        sz: &S,
        sa: c_double,
        ea: c_double,
    ) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawEllipticArc1(self.as_ptr(), pt, sz, sa, ea)
        }
    }
    fn draw_icon_coord<I: IconMethods>(&self, icon: &I, x: c_int, y: c_int) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDC_DrawIcon(self.as_ptr(), icon, x, y)
        }
    }
    fn draw_icon_point<I: IconMethods, P: PointMethods>(&self, icon: &I, pt: &P) {
        unsafe {
            let icon = icon.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawIcon1(self.as_ptr(), icon, pt)
        }
    }
    fn draw_label_bitmap<B: BitmapMethods, R: RectMethods, R2: RectMethods>(
        &self,
        text: &str,
        bitmap: &B,
        rect: &R,
        alignment: c_int,
        index_accel: c_int,
        rect_bounding: Option<&R2>,
    ) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let bitmap = bitmap.as_ptr();
            let rect = rect.as_ptr();
            let rect_bounding = match rect_bounding {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_DrawLabel(
                self.as_ptr(),
                text,
                bitmap,
                rect,
                alignment,
                index_accel,
                rect_bounding,
            )
        }
    }
    fn draw_label_rect<R: RectMethods>(
        &self,
        text: &str,
        rect: &R,
        alignment: c_int,
        index_accel: c_int,
    ) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxDC_DrawLabel1(self.as_ptr(), text, rect, alignment, index_accel)
        }
    }
    fn draw_line_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { ffi::wxDC_DrawLine(self.as_ptr(), x1, y1, x2, y2) }
    }
    fn draw_line_point<P: PointMethods, P2: PointMethods>(&self, pt1: &P, pt2: &P2) {
        unsafe {
            let pt1 = pt1.as_ptr();
            let pt2 = pt2.as_ptr();
            ffi::wxDC_DrawLine1(self.as_ptr(), pt1, pt2)
        }
    }
    // NOT_SUPPORTED: fn DrawLines()
    fn draw_lines(&self, points: *const c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { ffi::wxDC_DrawLines1(self.as_ptr(), points, xoffset, yoffset) }
    }
    fn draw_point_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_DrawPoint(self.as_ptr(), x, y) }
    }
    fn draw_point_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawPoint1(self.as_ptr(), pt)
        }
    }
    // NOT_SUPPORTED: fn DrawPolygon()
    // NOT_SUPPORTED: fn DrawPolygon1()
    // NOT_SUPPORTED: fn DrawPolyPolygon()
    fn draw_rectangle_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawRectangle(self.as_ptr(), x, y, width, height) }
    }
    fn draw_rectangle_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawRectangle1(self.as_ptr(), pt, sz)
        }
    }
    fn draw_rectangle_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRectangle2(self.as_ptr(), rect)
        }
    }
    fn draw_rotated_text_coord(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawRotatedText(self.as_ptr(), text, x, y, angle)
        }
    }
    fn draw_rotated_text_point<P: PointMethods>(&self, text: &str, point: &P, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let point = point.as_ptr();
            ffi::wxDC_DrawRotatedText1(self.as_ptr(), text, point, angle)
        }
    }
    fn draw_rounded_rectangle_coord(
        &self,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        radius: c_double,
    ) {
        unsafe { ffi::wxDC_DrawRoundedRectangle(self.as_ptr(), x, y, width, height, radius) }
    }
    fn draw_rounded_rectangle_point<P: PointMethods, S: SizeMethods>(
        &self,
        pt: &P,
        sz: &S,
        radius: c_double,
    ) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawRoundedRectangle1(self.as_ptr(), pt, sz, radius)
        }
    }
    fn draw_rounded_rectangle_rect<R: RectMethods>(&self, rect: &R, radius: c_double) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRoundedRectangle2(self.as_ptr(), rect, radius)
        }
    }
    // NOT_SUPPORTED: fn DrawSpline()
    fn draw_spline_pointlist(&self, points: *const c_void) {
        unsafe { ffi::wxDC_DrawSpline1(self.as_ptr(), points) }
    }
    fn draw_spline_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int) {
        unsafe { ffi::wxDC_DrawSpline2(self.as_ptr(), x1, y1, x2, y2, x3, y3) }
    }
    fn draw_text_coord(&self, text: &str, x: c_int, y: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawText(self.as_ptr(), text, x, y)
        }
    }
    fn draw_text_point<P: PointMethods>(&self, text: &str, pt: &P) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawText1(self.as_ptr(), text, pt)
        }
    }
    fn gradient_fill_concentric<R: RectMethods, C: ColourMethods, C2: ColourMethods>(
        &self,
        rect: &R,
        initial_colour: &C,
        dest_colour: &C2,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            let initial_colour = initial_colour.as_ptr();
            let dest_colour = dest_colour.as_ptr();
            ffi::wxDC_GradientFillConcentric(self.as_ptr(), rect, initial_colour, dest_colour)
        }
    }
    fn gradient_fill_concentric_point<
        R: RectMethods,
        C: ColourMethods,
        C2: ColourMethods,
        P: PointMethods,
    >(
        &self,
        rect: &R,
        initial_colour: &C,
        dest_colour: &C2,
        circle_center: &P,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            let initial_colour = initial_colour.as_ptr();
            let dest_colour = dest_colour.as_ptr();
            let circle_center = circle_center.as_ptr();
            ffi::wxDC_GradientFillConcentric1(
                self.as_ptr(),
                rect,
                initial_colour,
                dest_colour,
                circle_center,
            )
        }
    }
    fn gradient_fill_linear<R: RectMethods, C: ColourMethods, C2: ColourMethods>(
        &self,
        rect: &R,
        initial_colour: &C,
        dest_colour: &C2,
        n_direction: c_int,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            let initial_colour = initial_colour.as_ptr();
            let dest_colour = dest_colour.as_ptr();
            ffi::wxDC_GradientFillLinear(
                self.as_ptr(),
                rect,
                initial_colour,
                dest_colour,
                n_direction,
            )
        }
    }
    // NOT_SUPPORTED: fn FloodFill()
    // NOT_SUPPORTED: fn FloodFill1()
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CrossHair(self.as_ptr(), x, y) }
    }
    fn cross_hair_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_CrossHair1(self.as_ptr(), pt)
        }
    }
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxDC_DestroyClippingRegion(self.as_ptr()) }
    }
    fn get_clipping_box_coord(
        &self,
        x: *mut c_void,
        y: *mut c_void,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxDC_GetClippingBox(self.as_ptr(), x, y, width, height) }
    }
    fn get_clipping_box_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_GetClippingBox1(self.as_ptr(), rect)
        }
    }
    fn set_clipping_region_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_SetClippingRegion(self.as_ptr(), x, y, width, height) }
    }
    fn set_clipping_region_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_SetClippingRegion1(self.as_ptr(), pt, sz)
        }
    }
    fn set_clipping_region_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_SetClippingRegion2(self.as_ptr(), rect)
        }
    }
    fn set_device_clipping_region(&self, region: *const c_void) {
        unsafe { ffi::wxDC_SetDeviceClippingRegion(self.as_ptr(), region) }
    }
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharHeight(self.as_ptr()) }
    }
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontMetrics()
    fn get_multi_line_text_extent_coord<F: FontMethods>(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        height_line: *mut c_void,
        font: Option<&F>,
    ) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            let font = match font {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetMultiLineTextExtent(self.as_ptr(), string, w, h, height_line, font)
        }
    }
    fn get_multi_line_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetMultiLineTextExtent1(self.as_ptr(), string))
        }
    }
    fn get_partial_text_extents<A: ArrayIntMethods>(&self, text: &str, widths: &A) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let widths = widths.as_ptr();
            ffi::wxDC_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    fn get_text_extent_coord<F: FontMethods>(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: Option<&F>,
    ) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            let font = match font {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetTextExtent(self.as_ptr(), string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetTextExtent1(self.as_ptr(), string))
        }
    }
    fn get_background_mode(&self) -> c_int {
        unsafe { ffi::wxDC_GetBackgroundMode(self.as_ptr()) }
    }
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxDC_GetFont(self.as_ptr())) }
    }
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxDC_GetLayoutDirection(self.as_ptr()) }
    }
    fn get_text_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextBackground(self.as_ptr())) }
    }
    fn get_text_foreground(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextForeground(self.as_ptr())) }
    }
    fn set_background_mode(&self, mode: c_int) {
        unsafe { ffi::wxDC_SetBackgroundMode(self.as_ptr(), mode) }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDC_SetFont(self.as_ptr(), font)
        }
    }
    fn set_text_background<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextBackground(self.as_ptr(), colour)
        }
    }
    fn set_text_foreground<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextForeground(self.as_ptr(), colour)
        }
    }
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxDC_SetLayoutDirection(self.as_ptr(), dir) }
    }
    fn calc_bounding_box(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CalcBoundingBox(self.as_ptr(), x, y) }
    }
    fn max_x(&self) -> c_int {
        unsafe { ffi::wxDC_MaxX(self.as_ptr()) }
    }
    fn max_y(&self) -> c_int {
        unsafe { ffi::wxDC_MaxY(self.as_ptr()) }
    }
    fn min_x(&self) -> c_int {
        unsafe { ffi::wxDC_MinX(self.as_ptr()) }
    }
    fn min_y(&self) -> c_int {
        unsafe { ffi::wxDC_MinY(self.as_ptr()) }
    }
    fn reset_bounding_box(&self) {
        unsafe { ffi::wxDC_ResetBoundingBox(self.as_ptr()) }
    }
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDC_StartDoc(self.as_ptr(), message)
        }
    }
    fn start_page(&self) {
        unsafe { ffi::wxDC_StartPage(self.as_ptr()) }
    }
    fn end_doc(&self) {
        unsafe { ffi::wxDC_EndDoc(self.as_ptr()) }
    }
    fn end_page(&self) {
        unsafe { ffi::wxDC_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Blit()
    // NOT_SUPPORTED: fn StretchBlit()
    fn get_background(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBackground(self.as_ptr())) }
    }
    fn get_brush(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBrush(self.as_ptr())) }
    }
    fn get_pen(&self) -> PenIsOwned<false> {
        unsafe { PenIsOwned::from_ptr(ffi::wxDC_GetPen(self.as_ptr())) }
    }
    fn set_background<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBackground(self.as_ptr(), brush)
        }
    }
    fn set_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBrush(self.as_ptr(), brush)
        }
    }
    fn set_pen<P: PenMethods>(&self, pen: &P) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxDC_SetPen(self.as_ptr(), pen)
        }
    }
    fn copy_attributes<D: DCMethods>(&self, dc: &D) {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxDC_CopyAttributes(self.as_ptr(), dc)
        }
    }
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDC_GetContentScaleFactor(self.as_ptr()) }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxDC_GetDepth(self.as_ptr()) }
    }
    fn get_device_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetDeviceOrigin(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetLogicalFunction()
    // NOT_SUPPORTED: fn GetMapMode()
    fn get_pixel<C: ColourMethods>(&self, x: c_int, y: c_int, colour: Option<&C>) -> bool {
        unsafe {
            let colour = match colour {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetPixel(self.as_ptr(), x, y, colour)
        }
    }
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetPPI(self.as_ptr())) }
    }
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_FromDIP(self.as_ptr(), sz))
        }
    }
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_FromDIP1(self.as_ptr(), pt))
        }
    }
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_FromDIP2(self.as_ptr(), d) }
    }
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_ToDIP(self.as_ptr(), sz))
        }
    }
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_ToDIP1(self.as_ptr(), pt))
        }
    }
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_ToDIP2(self.as_ptr(), d) }
    }
    fn get_size_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSize1(self.as_ptr())) }
    }
    fn get_size_mm_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSizeMM(self.as_ptr(), width, height) }
    }
    fn get_size_mm(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSizeMM1(self.as_ptr())) }
    }
    fn get_user_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetUserScale(self.as_ptr(), x, y) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDC_IsOk(self.as_ptr()) }
    }
    fn set_axis_orientation(&self, x_left_right: bool, y_bottom_up: bool) {
        unsafe { ffi::wxDC_SetAxisOrientation(self.as_ptr(), x_left_right, y_bottom_up) }
    }
    fn set_device_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetDeviceOrigin(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn SetMapMode()
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxDC_SetPalette(self.as_ptr(), palette)
        }
    }
    fn set_user_scale(&self, x_scale: c_double, y_scale: c_double) {
        unsafe { ffi::wxDC_SetUserScale(self.as_ptr(), x_scale, y_scale) }
    }
    fn can_use_transform_matrix(&self) -> bool {
        unsafe { ffi::wxDC_CanUseTransformMatrix(self.as_ptr()) }
    }
    fn set_transform_matrix(&self, matrix: *const c_void) -> bool {
        unsafe { ffi::wxDC_SetTransformMatrix(self.as_ptr(), matrix) }
    }
    // NOT_SUPPORTED: fn GetTransformMatrix()
    fn reset_transform_matrix(&self) {
        unsafe { ffi::wxDC_ResetTransformMatrix(self.as_ptr()) }
    }
    fn can_draw_bitmap(&self) -> bool {
        unsafe { ffi::wxDC_CanDrawBitmap(self.as_ptr()) }
    }
    fn can_get_text_extent(&self) -> bool {
        unsafe { ffi::wxDC_CanGetTextExtent(self.as_ptr()) }
    }
    fn get_handle(&self) -> *mut c_void {
        unsafe { ffi::wxDC_GetHandle(self.as_ptr()) }
    }
    fn get_as_bitmap<R: RectMethods>(&self, subrect: Option<&R>) -> Bitmap {
        unsafe {
            let subrect = match subrect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Bitmap::from_ptr(ffi::wxDC_GetAsBitmap(self.as_ptr(), subrect))
        }
    }
    fn set_logical_scale(&self, x: c_double, y: c_double) {
        unsafe { ffi::wxDC_SetLogicalScale(self.as_ptr(), x, y) }
    }
    fn get_logical_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalScale(self.as_ptr(), x, y) }
    }
    fn set_logical_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetLogicalOrigin(self.as_ptr(), x, y) }
    }
    fn get_logical_origin_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalOrigin(self.as_ptr(), x, y) }
    }
    fn get_logical_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetLogicalOrigin1(self.as_ptr())) }
    }
    fn get_graphics_context(&self) -> *mut c_void {
        unsafe { ffi::wxDC_GetGraphicsContext(self.as_ptr()) }
    }
    fn set_graphics_context(&self, ctx: *mut c_void) {
        unsafe { ffi::wxDC_SetGraphicsContext(self.as_ptr(), ctx) }
    }
}

// wxDCBrushChanger
pub trait DCBrushChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCBrushChanger()
}

// wxDCClipper
pub trait DCClipperMethods: WxRustMethods {
    // DTOR: fn ~wxDCClipper()
}

// wxDCFontChanger
pub trait DCFontChangerMethods: WxRustMethods {
    fn set<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDCFontChanger_Set(self.as_ptr(), font)
        }
    }
    // DTOR: fn ~wxDCFontChanger()
}

// wxDCOverlay
pub trait DCOverlayMethods: WxRustMethods {
    // DTOR: fn ~wxDCOverlay()
    fn clear(&self) {
        unsafe { ffi::wxDCOverlay_Clear(self.as_ptr()) }
    }
}

// wxDCPenChanger
pub trait DCPenChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCPenChanger()
}

// wxDCTextBgColourChanger
pub trait DCTextBgColourChangerMethods: WxRustMethods {
    fn set<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxDCTextBgColourChanger_Set(self.as_ptr(), col)
        }
    }
    // DTOR: fn ~wxDCTextBgColourChanger()
}

// wxDCTextBgModeChanger
pub trait DCTextBgModeChangerMethods: WxRustMethods {}

// wxDCTextColourChanger
pub trait DCTextColourChangerMethods: WxRustMethods {
    fn set<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxDCTextColourChanger_Set(self.as_ptr(), col)
        }
    }
    // DTOR: fn ~wxDCTextColourChanger()
}

// wxDPIChangedEvent
pub trait DPIChangedEventMethods: EventMethods {
    fn get_old_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDPIChangedEvent_GetOldDPI(self.as_ptr())) }
    }
    fn get_new_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDPIChangedEvent_GetNewDPI(self.as_ptr())) }
    }
    fn scale(&self, sz: ffi::wxSize) -> Size {
        unsafe { Size::from_ptr(ffi::wxDPIChangedEvent_Scale(self.as_ptr(), sz)) }
    }
    fn scale_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDPIChangedEvent_ScaleX(self.as_ptr(), x) }
    }
    fn scale_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDPIChangedEvent_ScaleY(self.as_ptr(), y) }
    }
}

// wxDataViewColumn
pub trait DataViewColumnMethods: SettableHeaderColumnMethods {
    fn get_model_column(&self) -> c_uint {
        unsafe { ffi::wxDataViewColumn_GetModelColumn(self.as_ptr()) }
    }
    fn get_owner(&self) -> WeakRef<DataViewCtrl> {
        unsafe { WeakRef::<DataViewCtrl>::from(ffi::wxDataViewColumn_GetOwner(self.as_ptr())) }
    }
    fn get_renderer(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewColumn_GetRenderer(self.as_ptr()) }
    }
}

// wxDataViewCtrl
pub trait DataViewCtrlMethods: ControlMethods {
    // DTOR: fn ~wxDataViewCtrl()
    fn allow_multi_column_sort(&self, allow: bool) -> bool {
        unsafe { ffi::wxDataViewCtrl_AllowMultiColumnSort(self.as_ptr(), allow) }
    }
    fn append_column<D: DataViewColumnMethods>(&self, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_AppendColumn(self.as_ptr(), col)
        }
    }
    fn prepend_column<D: DataViewColumnMethods>(&self, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_PrependColumn(self.as_ptr(), col)
        }
    }
    fn insert_column<D: DataViewColumnMethods>(&self, pos: c_uint, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_InsertColumn(self.as_ptr(), pos, col)
        }
    }
    // NOT_SUPPORTED: fn AppendBitmapColumn()
    // NOT_SUPPORTED: fn AppendBitmapColumn1()
    // NOT_SUPPORTED: fn PrependBitmapColumn()
    // NOT_SUPPORTED: fn PrependBitmapColumn1()
    // NOT_SUPPORTED: fn AppendDateColumn()
    // NOT_SUPPORTED: fn AppendDateColumn1()
    // NOT_SUPPORTED: fn PrependDateColumn()
    // NOT_SUPPORTED: fn PrependDateColumn1()
    // NOT_SUPPORTED: fn AppendIconTextColumn()
    // NOT_SUPPORTED: fn AppendIconTextColumn1()
    // NOT_SUPPORTED: fn PrependIconTextColumn()
    // NOT_SUPPORTED: fn PrependIconTextColumn1()
    // NOT_SUPPORTED: fn AppendProgressColumn()
    // NOT_SUPPORTED: fn AppendProgressColumn1()
    // NOT_SUPPORTED: fn PrependProgressColumn()
    // NOT_SUPPORTED: fn PrependProgressColumn1()
    // NOT_SUPPORTED: fn AppendTextColumn()
    // NOT_SUPPORTED: fn AppendTextColumn1()
    // NOT_SUPPORTED: fn PrependTextColumn()
    // NOT_SUPPORTED: fn PrependTextColumn1()
    // NOT_SUPPORTED: fn AppendToggleColumn()
    // NOT_SUPPORTED: fn AppendToggleColumn1()
    // NOT_SUPPORTED: fn PrependToggleColumn()
    // NOT_SUPPORTED: fn PrependToggleColumn1()
    fn associate_model(&self, model: *mut c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_AssociateModel(self.as_ptr(), model) }
    }
    fn clear_columns(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_ClearColumns(self.as_ptr()) }
    }
    fn collapse<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Collapse(self.as_ptr(), item)
        }
    }
    fn delete_column<D: DataViewColumnMethods>(&self, column: Option<&D>) -> bool {
        unsafe {
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_DeleteColumn(self.as_ptr(), column)
        }
    }
    fn edit_item<D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        item: &D,
        column: Option<&D2>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_EditItem(self.as_ptr(), item, column)
        }
    }
    fn enable_drag_source(&self, format: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_EnableDragSource(self.as_ptr(), format) }
    }
    fn enable_drop_targets(&self, formats: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_EnableDropTargets(self.as_ptr(), formats) }
    }
    fn enable_drop_target(&self, format: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_EnableDropTarget(self.as_ptr(), format) }
    }
    fn ensure_visible<D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        item: &D,
        column: Option<&D2>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_EnsureVisible(self.as_ptr(), item, column)
        }
    }
    fn expand<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Expand(self.as_ptr(), item)
        }
    }
    fn expand_ancestors<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandAncestors(self.as_ptr(), item)
        }
    }
    fn expand_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandChildren(self.as_ptr(), item)
        }
    }
    fn get_column(&self, pos: c_uint) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetColumn(self.as_ptr(), pos)) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn get_column_position<D: DataViewColumnMethods>(&self, column: Option<&D>) -> c_int {
        unsafe {
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_GetColumnPosition(self.as_ptr(), column)
        }
    }
    fn get_expander_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetExpanderColumn(self.as_ptr())) }
    }
    fn get_current_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetCurrentItem(self.as_ptr())) }
    }
    fn get_current_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetCurrentColumn(self.as_ptr())) }
    }
    fn get_indent(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetIndent(self.as_ptr()) }
    }
    fn get_item_rect<D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        item: &D,
        col: Option<&D2>,
    ) -> Rect {
        unsafe {
            let item = item.as_ptr();
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Rect::from_ptr(ffi::wxDataViewCtrl_GetItemRect(self.as_ptr(), item, col))
        }
    }
    fn get_main_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDataViewCtrl_GetMainWindow(self.as_ptr())) }
    }
    fn get_model(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewCtrl_GetModel(self.as_ptr()) }
    }
    fn get_selected_items_count(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelectedItemsCount(self.as_ptr()) }
    }
    fn get_selection(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetSelection(self.as_ptr())) }
    }
    fn get_selections(&self, sel: *mut c_void) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelections(self.as_ptr(), sel) }
    }
    fn get_sorting_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetSortingColumn(self.as_ptr())) }
    }
    fn get_sorting_columns(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewCtrl_GetSortingColumns(self.as_ptr()) }
    }
    fn has_selection(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_HasSelection(self.as_ptr()) }
    }
    fn hit_test<P: PointMethods, D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        point: &P,
        item: &D,
        col: Option<&D2>,
    ) {
        unsafe {
            let point = point.as_ptr();
            let item = item.as_ptr();
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_HitTest(self.as_ptr(), point, item, col)
        }
    }
    fn is_expanded<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsExpanded(self.as_ptr(), item)
        }
    }
    fn is_multi_column_sort_allowed(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_IsMultiColumnSortAllowed(self.as_ptr()) }
    }
    fn is_selected<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsSelected(self.as_ptr(), item)
        }
    }
    fn select<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Select(self.as_ptr(), item)
        }
    }
    fn select_all(&self) {
        unsafe { ffi::wxDataViewCtrl_SelectAll(self.as_ptr()) }
    }
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    fn set_expander_column<D: DataViewColumnMethods>(&self, col: Option<&D>) {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_SetExpanderColumn(self.as_ptr(), col)
        }
    }
    fn set_current_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_SetCurrentItem(self.as_ptr(), item)
        }
    }
    fn set_header_attr<I: ItemAttrMethods>(&self, attr: &I) -> bool {
        unsafe {
            let attr = attr.as_ptr();
            ffi::wxDataViewCtrl_SetHeaderAttr(self.as_ptr(), attr)
        }
    }
    fn set_indent(&self, indent: c_int) {
        unsafe { ffi::wxDataViewCtrl_SetIndent(self.as_ptr(), indent) }
    }
    fn set_selections(&self, sel: *const c_void) {
        unsafe { ffi::wxDataViewCtrl_SetSelections(self.as_ptr(), sel) }
    }
    fn unselect<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Unselect(self.as_ptr(), item)
        }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxDataViewCtrl_UnselectAll(self.as_ptr()) }
    }
    fn set_row_height(&self, row_height: c_int) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetRowHeight(self.as_ptr(), row_height) }
    }
    fn toggle_sort_by_column(&self, column: c_int) {
        unsafe { ffi::wxDataViewCtrl_ToggleSortByColumn(self.as_ptr(), column) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetCountPerPage(self.as_ptr()) }
    }
    fn get_top_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetTopItem(self.as_ptr())) }
    }
}

// wxDataViewIconText
pub trait DataViewIconTextMethods: ObjectMethods {
    fn get_bitmap_bundle(&self) -> BitmapBundleIsOwned<false> {
        unsafe {
            BitmapBundleIsOwned::from_ptr(ffi::wxDataViewIconText_GetBitmapBundle(self.as_ptr()))
        }
    }
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxDataViewIconText_GetIcon(self.as_ptr())) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconText_GetText(self.as_ptr())).into() }
    }
    fn set_bitmap_bundle<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDataViewIconText_SetBitmapBundle(self.as_ptr(), bitmap)
        }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDataViewIconText_SetIcon(self.as_ptr(), icon)
        }
    }
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDataViewIconText_SetText(self.as_ptr(), text)
        }
    }
}

// wxDataViewItem
pub trait DataViewItemMethods: WxRustMethods {
    fn get_id(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewItem_GetID(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDataViewItem_IsOk(self.as_ptr()) }
    }
}

// wxDataViewItemAttr
pub trait DataViewItemAttrMethods: WxRustMethods {
    fn set_bold(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetBold(self.as_ptr(), set) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetColour(self.as_ptr(), colour)
        }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn set_italic(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetItalic(self.as_ptr(), set) }
    }
    fn set_strikethrough(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetStrikethrough(self.as_ptr(), set) }
    }
    fn has_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasColour(self.as_ptr()) }
    }
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetColour(self.as_ptr())) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasFont(self.as_ptr()) }
    }
    fn get_bold(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetBold(self.as_ptr()) }
    }
    fn get_italic(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetItalic(self.as_ptr()) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    fn is_default(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_IsDefault(self.as_ptr()) }
    }
    fn get_effective_font<F: FontMethods>(&self, font: &F) -> Font {
        unsafe {
            let font = font.as_ptr();
            Font::from_ptr(ffi::wxDataViewItemAttr_GetEffectiveFont(
                self.as_ptr(),
                font,
            ))
        }
    }
}

// wxDataViewTreeCtrl
pub trait DataViewTreeCtrlMethods: DataViewCtrlMethods {
    // DTOR: fn ~wxDataViewTreeCtrl()
    fn append_container<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_AppendContainer(
                self.as_ptr(),
                parent,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn append_item<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_AppendItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            ffi::wxDataViewTreeCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator)
        }
    }
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteChildren(self.as_ptr(), item)
        }
    }
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteItem(self.as_ptr(), item)
        }
    }
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeCtrl_GetChildCount(self.as_ptr(), parent)
        }
    }
    fn get_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxDataViewTreeCtrl_GetImageList(self.as_ptr())) }
    }
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            ClientData::option_from(ffi::wxDataViewTreeCtrl_GetItemData(self.as_ptr(), item))
        }
    }
    fn get_item_expanded_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemExpandedIcon(
                self.as_ptr(),
                item,
            ))
        }
    }
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemIcon(self.as_ptr(), item))
        }
    }
    fn get_item_parent(&self, item: ffi::wxDataViewItem) -> DataViewItem {
        unsafe {
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_GetItemParent(self.as_ptr(), item))
        }
    }
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeCtrl_GetItemText(self.as_ptr(), item)).into()
        }
    }
    fn get_nth_child<D: DataViewItemMethods>(&self, parent: &D, pos: c_uint) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_GetNthChild(
                self.as_ptr(),
                parent,
                pos,
            ))
        }
    }
    fn get_store(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewTreeCtrl_GetStore(self.as_ptr()) }
    }
    fn get_store(&self) -> *const c_void {
        unsafe { ffi::wxDataViewTreeCtrl_GetStore1(self.as_ptr()) }
    }
    fn insert_container<D: DataViewItemMethods, D2: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_InsertContainer(
                self.as_ptr(),
                parent,
                previous,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn insert_item<D: DataViewItemMethods, D2: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_InsertItem(
                self.as_ptr(),
                parent,
                previous,
                text,
                icon,
                data,
            ))
        }
    }
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_IsContainer(self.as_ptr(), item)
        }
    }
    fn prepend_container<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_PrependContainer(
                self.as_ptr(),
                parent,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn prepend_item<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_PrependItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    fn set_image_list<I: ImageListMethods>(&self, imagelist: Option<&I>) {
        unsafe {
            let imagelist = match imagelist {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewTreeCtrl_SetImageList(self.as_ptr(), imagelist)
        }
    }
    fn set_item_data<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        item: &D,
        data: Option<&C>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewTreeCtrl_SetItemData(self.as_ptr(), item, data)
        }
    }
    fn set_item_expanded_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(
        &self,
        item: &D,
        icon: &B,
    ) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemExpandedIcon(self.as_ptr(), item, icon)
        }
    }
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
    fn set_item_text<D: DataViewItemMethods>(&self, item: &D, text: &str) {
        unsafe {
            let item = item.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
}

// wxDataViewValueAdjuster
pub trait DataViewValueAdjusterMethods: WxRustMethods {
    // NOT_SUPPORTED: fn MakeHighlighted()
}

// wxDateEvent
pub trait DateEventMethods: CommandEventMethods {
    fn get_date(&self) -> DateTimeIsOwned<false> {
        unsafe { DateTimeIsOwned::from_ptr(ffi::wxDateEvent_GetDate(self.as_ptr())) }
    }
    fn set_date<D: DateTimeMethods>(&self, date: &D) {
        unsafe {
            let date = date.as_ptr();
            ffi::wxDateEvent_SetDate(self.as_ptr(), date)
        }
    }
}

// wxDatePickerCtrl
pub trait DatePickerCtrlMethods: ControlMethods {
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
            ffi::wxDatePickerCtrl_Create(
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
    fn get_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        dt1: Option<&D>,
        dt2: Option<&D2>,
    ) -> bool {
        unsafe {
            let dt1 = match dt1 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt2 = match dt2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDatePickerCtrl_GetRange(self.as_ptr(), dt1, dt2)
        }
    }
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDatePickerCtrl_GetValue(self.as_ptr())) }
    }
    fn set_null_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDatePickerCtrl_SetNullText(self.as_ptr(), text)
        }
    }
    fn set_range<D: DateTimeMethods, D2: DateTimeMethods>(&self, dt1: &D, dt2: &D2) {
        unsafe {
            let dt1 = dt1.as_ptr();
            let dt2 = dt2.as_ptr();
            ffi::wxDatePickerCtrl_SetRange(self.as_ptr(), dt1, dt2)
        }
    }
    fn set_value<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDatePickerCtrl_SetValue(self.as_ptr(), dt)
        }
    }
}

// wxDelegateRendererNative
pub trait DelegateRendererNativeMethods: RendererNativeMethods {}

// wxDirPickerCtrl
pub trait DirPickerCtrlMethods: PickerBaseMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDirPickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                path,
                message,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_dir_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxDirPickerCtrl_GetDirName(self.as_ptr())) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirPickerCtrl_GetPath(self.as_ptr())).into() }
    }
    fn set_dir_name<F: FileNameMethods>(&self, dirname: &F) {
        unsafe {
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetDirName(self.as_ptr(), dirname)
        }
    }
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDirPickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    fn set_path(&self, dirname: &str) {
        unsafe {
            let dirname = WxString::from(dirname);
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetPath(self.as_ptr(), dirname)
        }
    }
}

// wxDisplay
pub trait DisplayMethods: WxRustMethods {
    // DTOR: fn ~wxDisplay()
    fn change_mode(&self, mode: *const c_void) -> bool {
        unsafe { ffi::wxDisplay_ChangeMode(self.as_ptr(), mode) }
    }
    fn get_client_area(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetClientArea(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetCurrentMode()
    fn get_geometry(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetGeometry(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetModes()
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDisplay_GetName(self.as_ptr())).into() }
    }
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetPPI(self.as_ptr())) }
    }
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDisplay_GetScaleFactor(self.as_ptr()) }
    }
    fn is_primary(&self) -> bool {
        unsafe { ffi::wxDisplay_IsPrimary(self.as_ptr()) }
    }
    fn get_count() -> c_uint {
        unsafe { ffi::wxDisplay_GetCount() }
    }
    fn get_from_point<P: PointMethods>(pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDisplay_GetFromPoint(pt)
        }
    }
    fn get_from_window<W: WindowMethods>(win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDisplay_GetFromWindow(win)
        }
    }
    fn get_std_ppi_value() -> c_int {
        unsafe { ffi::wxDisplay_GetStdPPIValue() }
    }
    fn get_std_ppi() -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetStdPPI()) }
    }
}

// wxDisplayChangedEvent
pub trait DisplayChangedEventMethods: EventMethods {}

// wxDocChildFrame
pub trait DocChildFrameMethods: FrameMethods {
    // DTOR: fn ~wxDocChildFrame()
    fn get_document(&self) -> *mut c_void {
        unsafe { ffi::wxDocChildFrame_GetDocument(self.as_ptr()) }
    }
    fn get_view(&self) -> *mut c_void {
        unsafe { ffi::wxDocChildFrame_GetView(self.as_ptr()) }
    }
    fn set_document(&self, doc: *mut c_void) {
        unsafe { ffi::wxDocChildFrame_SetDocument(self.as_ptr(), doc) }
    }
    fn set_view(&self, view: *mut c_void) {
        unsafe { ffi::wxDocChildFrame_SetView(self.as_ptr(), view) }
    }
}

// wxDocMDIChildFrame
pub trait DocMDIChildFrameMethods: MDIChildFrameMethods {
    // DTOR: fn ~wxDocMDIChildFrame()
    fn get_document(&self) -> *mut c_void {
        unsafe { ffi::wxDocMDIChildFrame_GetDocument(self.as_ptr()) }
    }
    fn get_view(&self) -> *mut c_void {
        unsafe { ffi::wxDocMDIChildFrame_GetView(self.as_ptr()) }
    }
    fn set_document(&self, doc: *mut c_void) {
        unsafe { ffi::wxDocMDIChildFrame_SetDocument(self.as_ptr(), doc) }
    }
    fn set_view(&self, view: *mut c_void) {
        unsafe { ffi::wxDocMDIChildFrame_SetView(self.as_ptr(), view) }
    }
}

// wxDocMDIParentFrame
pub trait DocMDIParentFrameMethods: MDIParentFrameMethods {
    // DTOR: fn ~wxDocMDIParentFrame()
    fn create_docmanager<F: FrameMethods, P: PointMethods, S: SizeMethods>(
        &self,
        manager: *mut c_void,
        parent: Option<&F>,
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
            ffi::wxDocMDIParentFrame_Create(
                self.as_ptr(),
                manager,
                parent,
                id,
                title,
                pos,
                size,
                style,
                name,
            )
        }
    }
}

// wxDocParentFrame
pub trait DocParentFrameMethods: FrameMethods {
    // DTOR: fn ~wxDocParentFrame()
    fn create_docmanager<F: FrameMethods, P: PointMethods, S: SizeMethods>(
        &self,
        manager: *mut c_void,
        parent: Option<&F>,
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
            ffi::wxDocParentFrame_Create(
                self.as_ptr(),
                manager,
                parent,
                id,
                title,
                pos,
                size,
                style,
                name,
            )
        }
    }
    fn get_document_manager(&self) -> *mut c_void {
        unsafe { ffi::wxDocParentFrame_GetDocumentManager(self.as_ptr()) }
    }
}

// wxDragImage
pub trait DragImageMethods: ObjectMethods {
    fn begin_drag_bool<P: PointMethods, W: WindowMethods, R: RectMethods>(
        &self,
        hotspot: &P,
        window: Option<&W>,
        full_screen: bool,
        rect: Option<&R>,
    ) -> bool {
        unsafe {
            let hotspot = hotspot.as_ptr();
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDragImage_BeginDrag(self.as_ptr(), hotspot, window, full_screen, rect)
        }
    }
    fn begin_drag_window<P: PointMethods, W: WindowMethods, W2: WindowMethods>(
        &self,
        hotspot: &P,
        window: Option<&W>,
        bounding_window: Option<&W2>,
    ) -> bool {
        unsafe {
            let hotspot = hotspot.as_ptr();
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bounding_window = match bounding_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDragImage_BeginDrag1(self.as_ptr(), hotspot, window, bounding_window)
        }
    }
    fn do_draw_image<D: DCMethods, P: PointMethods>(&self, dc: &D, pos: &P) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxDragImage_DoDrawImage(self.as_ptr(), dc, pos)
        }
    }
    fn end_drag(&self) -> bool {
        unsafe { ffi::wxDragImage_EndDrag(self.as_ptr()) }
    }
    fn get_image_rect<P: PointMethods>(&self, pos: &P) -> Rect {
        unsafe {
            let pos = pos.as_ptr();
            Rect::from_ptr(ffi::wxDragImage_GetImageRect(self.as_ptr(), pos))
        }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxDragImage_Hide(self.as_ptr()) }
    }
    fn move_<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDragImage_Move(self.as_ptr(), pt)
        }
    }
    fn show(&self) -> bool {
        unsafe { ffi::wxDragImage_Show(self.as_ptr()) }
    }
    fn update_backing_from_window<
        D: DCMethods,
        M: MemoryDCMethods,
        R: RectMethods,
        R2: RectMethods,
    >(
        &self,
        window_dc: &D,
        dest_dc: &M,
        source_rect: &R,
        dest_rect: &R2,
    ) -> bool {
        unsafe {
            let window_dc = window_dc.as_ptr();
            let dest_dc = dest_dc.as_ptr();
            let source_rect = source_rect.as_ptr();
            let dest_rect = dest_rect.as_ptr();
            ffi::wxDragImage_UpdateBackingFromWindow(
                self.as_ptr(),
                window_dc,
                dest_dc,
                source_rect,
                dest_rect,
            )
        }
    }
}

// wxDropTarget
pub trait DropTargetMethods: WxRustMethods {
    // DTOR: fn ~wxDropTarget()
    fn get_data(&self) -> bool {
        unsafe { ffi::wxDropTarget_GetData(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn OnData()
    // NOT_SUPPORTED: fn OnDragOver()
    fn on_drop(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxDropTarget_OnDrop(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn OnEnter()
    fn on_leave(&self) {
        unsafe { ffi::wxDropTarget_OnLeave(self.as_ptr()) }
    }
    fn get_data_object(&self) -> *mut c_void {
        unsafe { ffi::wxDropTarget_GetDataObject(self.as_ptr()) }
    }
    fn set_data_object(&self, data: *mut c_void) {
        unsafe { ffi::wxDropTarget_SetDataObject(self.as_ptr(), data) }
    }
    // NOT_SUPPORTED: fn SetDefaultAction()
    // NOT_SUPPORTED: fn GetDefaultAction()
}

// wxEditableListBox
pub trait EditableListBoxMethods: PanelMethods {
    // DTOR: fn ~wxEditableListBox()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxEditableListBox_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn set_strings<A: ArrayStringMethods>(&self, strings: &A) {
        unsafe {
            let strings = strings.as_ptr();
            ffi::wxEditableListBox_SetStrings(self.as_ptr(), strings)
        }
    }
    fn get_strings<A: ArrayStringMethods>(&self, strings: &A) {
        unsafe {
            let strings = strings.as_ptr();
            ffi::wxEditableListBox_GetStrings(self.as_ptr(), strings)
        }
    }
}

// wxEraseEvent
pub trait EraseEventMethods: EventMethods {
    fn get_dc(&self) -> Option<DCIsOwned<false>> {
        unsafe { DC::option_from(ffi::wxEraseEvent_GetDC(self.as_ptr())) }
    }
}

// wxEventBlocker
pub trait EventBlockerMethods: EvtHandlerMethods {
    // DTOR: fn ~wxEventBlocker()
    // NOT_SUPPORTED: fn Block()
}

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

// wxFileDialogCustomize
pub trait FileDialogCustomizeMethods: WxRustMethods {
    fn add_button(&self, label: &str) -> WeakRef<FileDialogButton> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WeakRef::<FileDialogButton>::from(ffi::wxFileDialogCustomize_AddButton(
                self.as_ptr(),
                label,
            ))
        }
    }
    fn add_check_box(&self, label: &str) -> WeakRef<FileDialogCheckBox> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WeakRef::<FileDialogCheckBox>::from(ffi::wxFileDialogCustomize_AddCheckBox(
                self.as_ptr(),
                label,
            ))
        }
    }
    fn add_radio_button(&self, label: &str) -> WeakRef<FileDialogRadioButton> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WeakRef::<FileDialogRadioButton>::from(ffi::wxFileDialogCustomize_AddRadioButton(
                self.as_ptr(),
                label,
            ))
        }
    }
    fn add_choice(&self, n: usize, strings: *const c_void) -> WeakRef<FileDialogChoice> {
        unsafe {
            WeakRef::<FileDialogChoice>::from(ffi::wxFileDialogCustomize_AddChoice(
                self.as_ptr(),
                n,
                strings,
            ))
        }
    }
    fn add_text_ctrl(&self, label: &str) -> WeakRef<FileDialogTextCtrl> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WeakRef::<FileDialogTextCtrl>::from(ffi::wxFileDialogCustomize_AddTextCtrl(
                self.as_ptr(),
                label,
            ))
        }
    }
    fn add_static_text(&self, label: &str) -> WeakRef<FileDialogStaticText> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WeakRef::<FileDialogStaticText>::from(ffi::wxFileDialogCustomize_AddStaticText(
                self.as_ptr(),
                label,
            ))
        }
    }
}

// wxFileDialogCustomizeHook
pub trait FileDialogCustomizeHookMethods: WxRustMethods {
    fn add_custom_controls<F: FileDialogCustomizeMethods>(&self, customizer: &F) {
        unsafe {
            let customizer = customizer.as_ptr();
            ffi::wxFileDialogCustomizeHook_AddCustomControls(self.as_ptr(), customizer)
        }
    }
    fn update_custom_controls(&self) {
        unsafe { ffi::wxFileDialogCustomizeHook_UpdateCustomControls(self.as_ptr()) }
    }
    fn transfer_data_from_custom_controls(&self) {
        unsafe { ffi::wxFileDialogCustomizeHook_TransferDataFromCustomControls(self.as_ptr()) }
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
    fn get_dialog(&self) -> *mut c_void {
        unsafe { ffi::wxFindDialogEvent_GetDialog(self.as_ptr()) }
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
    fn get_native_font_info(&self) -> Option<NativeFontInfoIsOwned<false>> {
        unsafe { NativeFontInfo::option_from(ffi::wxFont_GetNativeFontInfo(self.as_ptr())) }
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
    fn set_native_font_info_nativefontinfo<N: NativeFontInfoMethods>(&self, info: &N) {
        unsafe {
            let info = info.as_ptr();
            ffi::wxFont_SetNativeFontInfo1(self.as_ptr(), info)
        }
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
    fn new_nativefontinfo<N: NativeFontInfoMethods>(native_info: &N) -> Option<FontIsOwned<false>> {
        unsafe {
            let native_info = native_info.as_ptr();
            Font::option_from(ffi::wxFont_New4(native_info))
        }
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
    fn find_or_create_font<F: FontInfoMethods>(&self, font_info: &F) -> Option<FontIsOwned<false>> {
        unsafe {
            let font_info = font_info.as_ptr();
            Font::option_from(ffi::wxFontList_FindOrCreateFont1(self.as_ptr(), font_info))
        }
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
    fn get_status_bar(&self) -> WeakRef<StatusBar> {
        unsafe { WeakRef::<StatusBar>::from(ffi::wxFrame_GetStatusBar(self.as_ptr())) }
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
    fn set_status_bar<S: StatusBarMethods>(&self, status_bar: Option<&S>) {
        unsafe {
            let status_bar = match status_bar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFrame_SetStatusBar(self.as_ptr(), status_bar)
        }
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

// wxGBPosition
pub trait GBPositionMethods: WxRustMethods {
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetCol(self.as_ptr()) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetRow(self.as_ptr()) }
    }
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGBPosition_SetCol(self.as_ptr(), col) }
    }
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGBPosition_SetRow(self.as_ptr(), row) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGBSpan
pub trait GBSpanMethods: WxRustMethods {
    fn get_colspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetColspan(self.as_ptr()) }
    }
    fn get_rowspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetRowspan(self.as_ptr()) }
    }
    fn set_colspan(&self, colspan: c_int) {
        unsafe { ffi::wxGBSpan_SetColspan(self.as_ptr(), colspan) }
    }
    fn set_rowspan(&self, rowspan: c_int) {
        unsafe { ffi::wxGBSpan_SetRowspan(self.as_ptr(), rowspan) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGDIObject
pub trait GDIObjectMethods: ObjectMethods {}

// wxGauge
pub trait GaugeMethods: ControlMethods {
    // DTOR: fn ~wxGauge()
    fn create_int<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        range: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxGauge_Create(
                self.as_ptr(),
                parent,
                id,
                range,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGauge_GetRange(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGauge_GetValue(self.as_ptr()) }
    }
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxGauge_IsVertical(self.as_ptr()) }
    }
    fn pulse(&self) {
        unsafe { ffi::wxGauge_Pulse(self.as_ptr()) }
    }
    fn set_range(&self, range: c_int) {
        unsafe { ffi::wxGauge_SetRange(self.as_ptr(), range) }
    }
    fn set_value(&self, pos: c_int) {
        unsafe { ffi::wxGauge_SetValue(self.as_ptr(), pos) }
    }
}

// wxGenericAboutDialog
pub trait GenericAboutDialogMethods: WxRustMethods {
    fn create<A: AboutDialogInfoMethods, W: WindowMethods>(
        &self,
        info: &A,
        parent: Option<&W>,
    ) -> bool {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGenericAboutDialog_Create(self.as_ptr(), info, parent)
        }
    }
}

// wxGenericAnimationCtrl
pub trait GenericAnimationCtrlMethods: AnimationCtrlMethods {
    fn draw_current_frame<D: DCMethods>(&self, dc: &D) {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxGenericAnimationCtrl_DrawCurrentFrame(self.as_ptr(), dc)
        }
    }
    fn get_backing_store(&self) -> BitmapIsOwned<false> {
        unsafe {
            BitmapIsOwned::from_ptr(ffi::wxGenericAnimationCtrl_GetBackingStore(self.as_ptr()))
        }
    }
    fn play_bool(&self, looped: bool) -> bool {
        unsafe { ffi::wxGenericAnimationCtrl_Play(self.as_ptr(), looped) }
    }
    fn set_use_window_background_colour(&self, use_win_background: bool) {
        unsafe {
            ffi::wxGenericAnimationCtrl_SetUseWindowBackgroundColour(
                self.as_ptr(),
                use_win_background,
            )
        }
    }
    fn is_using_window_background_colour(&self) -> bool {
        unsafe { ffi::wxGenericAnimationCtrl_IsUsingWindowBackgroundColour(self.as_ptr()) }
    }
}

// wxGenericDirCtrl
pub trait GenericDirCtrlMethods: ControlMethods {
    // DTOR: fn ~wxGenericDirCtrl()
    fn collapse_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_CollapsePath(self.as_ptr(), path)
        }
    }
    fn collapse_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_CollapseTree(self.as_ptr()) }
    }
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        dir: &str,
        pos: &P,
        size: &S,
        style: c_long,
        filter: &str,
        default_filter: c_int,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxGenericDirCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                dir,
                pos,
                size,
                style,
                filter,
                default_filter,
                name,
            )
        }
    }
    fn expand_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_ExpandPath(self.as_ptr(), path)
        }
    }
    fn get_default_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetDefaultPath(self.as_ptr())).into() }
    }
    fn get_file_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilePath(self.as_ptr())).into() }
    }
    fn get_file_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetFilePaths(self.as_ptr(), paths)
        }
    }
    fn get_filter(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilter(self.as_ptr())).into() }
    }
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxGenericDirCtrl_GetFilterIndex(self.as_ptr()) }
    }
    fn get_filter_list_ctrl(&self) -> WeakRef<DirFilterListCtrl> {
        unsafe {
            WeakRef::<DirFilterListCtrl>::from(ffi::wxGenericDirCtrl_GetFilterListCtrl(
                self.as_ptr(),
            ))
        }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetPath(self.as_ptr())).into() }
    }
    fn get_path_treeitemid(&self, item_id: ffi::wxTreeItemId) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetPath1(self.as_ptr(), item_id)).into() }
    }
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    fn get_root_id(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxGenericDirCtrl_GetRootId(self.as_ptr())) }
    }
    fn get_tree_ctrl(&self) -> WeakRef<TreeCtrl> {
        unsafe { WeakRef::<TreeCtrl>::from(ffi::wxGenericDirCtrl_GetTreeCtrl(self.as_ptr())) }
    }
    fn init(&self) {
        unsafe { ffi::wxGenericDirCtrl_Init(self.as_ptr()) }
    }
    fn re_create_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_ReCreateTree(self.as_ptr()) }
    }
    fn set_default_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetDefaultPath(self.as_ptr(), path)
        }
    }
    fn set_filter(&self, filter: &str) {
        unsafe {
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            ffi::wxGenericDirCtrl_SetFilter(self.as_ptr(), filter)
        }
    }
    fn set_filter_index(&self, n: c_int) {
        unsafe { ffi::wxGenericDirCtrl_SetFilterIndex(self.as_ptr(), n) }
    }
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetPath(self.as_ptr(), path)
        }
    }
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxGenericDirCtrl_ShowHidden(self.as_ptr(), show) }
    }
    fn select_path(&self, path: &str, select: bool) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SelectPath(self.as_ptr(), path, select)
        }
    }
    fn select_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_SelectPaths(self.as_ptr(), paths)
        }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxGenericDirCtrl_UnselectAll(self.as_ptr()) }
    }
}

// wxGenericValidator
pub trait GenericValidatorMethods: ValidatorMethods {
    // DTOR: fn ~wxGenericValidator()
}

// wxGestureEvent
pub trait GestureEventMethods: EventMethods {
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxGestureEvent_GetPosition(self.as_ptr())) }
    }
    fn is_gesture_start(&self) -> bool {
        unsafe { ffi::wxGestureEvent_IsGestureStart(self.as_ptr()) }
    }
    fn is_gesture_end(&self) -> bool {
        unsafe { ffi::wxGestureEvent_IsGestureEnd(self.as_ptr()) }
    }
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGestureEvent_SetPosition(self.as_ptr(), pos)
        }
    }
    fn set_gesture_start(&self, is_start: bool) {
        unsafe { ffi::wxGestureEvent_SetGestureStart(self.as_ptr(), is_start) }
    }
    fn set_gesture_end(&self, is_end: bool) {
        unsafe { ffi::wxGestureEvent_SetGestureEnd(self.as_ptr(), is_end) }
    }
}

// wxGraphicsBrush
pub trait GraphicsBrushMethods: GraphicsObjectMethods {}

// wxGraphicsFont
pub trait GraphicsFontMethods: GraphicsObjectMethods {}

// wxGraphicsGradientStop
pub trait GraphicsGradientStopMethods: WxRustMethods {
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxGraphicsGradientStop_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxGraphicsGradientStop_SetColour(self.as_ptr(), col)
        }
    }
    // NOT_SUPPORTED: fn GetPosition()
    // NOT_SUPPORTED: fn SetPosition()
}

// wxGraphicsGradientStops
pub trait GraphicsGradientStopsMethods: WxRustMethods {
    fn add<G: GraphicsGradientStopMethods>(&self, stop: &G) {
        unsafe {
            let stop = stop.as_ptr();
            ffi::wxGraphicsGradientStops_Add(self.as_ptr(), stop)
        }
    }
    // NOT_SUPPORTED: fn Add1()
    // NOT_SUPPORTED: fn Item()
    fn get_count(&self) -> usize {
        unsafe { ffi::wxGraphicsGradientStops_GetCount(self.as_ptr()) }
    }
    fn set_start_colour(&self, col: ffi::wxColour) {
        unsafe { ffi::wxGraphicsGradientStops_SetStartColour(self.as_ptr(), col) }
    }
    fn get_start_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetStartColour(self.as_ptr())) }
    }
    fn set_end_colour(&self, col: ffi::wxColour) {
        unsafe { ffi::wxGraphicsGradientStops_SetEndColour(self.as_ptr(), col) }
    }
    fn get_end_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetEndColour(self.as_ptr())) }
    }
}

// wxGraphicsMatrix
pub trait GraphicsMatrixMethods: GraphicsObjectMethods {
    fn concat_graphicsmatrix<G: GraphicsMatrixMethods>(&self, t: Option<&G>) {
        unsafe {
            let t = match t {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGraphicsMatrix_Concat(self.as_ptr(), t)
        }
    }
    fn concat_graphicsmatrix<G: GraphicsMatrixMethods>(&self, t: &G) {
        unsafe {
            let t = t.as_ptr();
            ffi::wxGraphicsMatrix_Concat1(self.as_ptr(), t)
        }
    }
    fn get(
        &self,
        a: *mut c_void,
        b: *mut c_void,
        c: *mut c_void,
        d: *mut c_void,
        tx: *mut c_void,
        ty: *mut c_void,
    ) {
        unsafe { ffi::wxGraphicsMatrix_Get(self.as_ptr(), a, b, c, d, tx, ty) }
    }
    fn get_native_matrix(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsMatrix_GetNativeMatrix(self.as_ptr()) }
    }
    fn invert(&self) {
        unsafe { ffi::wxGraphicsMatrix_Invert(self.as_ptr()) }
    }
    fn is_equal_graphicsmatrix<G: GraphicsMatrixMethods>(&self, t: Option<&G>) -> bool {
        unsafe {
            let t = match t {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGraphicsMatrix_IsEqual(self.as_ptr(), t)
        }
    }
    fn is_equal_graphicsmatrix<G: GraphicsMatrixMethods>(&self, t: &G) -> bool {
        unsafe {
            let t = t.as_ptr();
            ffi::wxGraphicsMatrix_IsEqual1(self.as_ptr(), t)
        }
    }
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxGraphicsMatrix_IsIdentity(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Set()
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformDistance(self.as_ptr(), dx, dy) }
    }
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn Translate()
}

// wxGraphicsObject
pub trait GraphicsObjectMethods: ObjectMethods {
    fn get_renderer(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsObject_GetRenderer(self.as_ptr()) }
    }
    fn is_null(&self) -> bool {
        unsafe { ffi::wxGraphicsObject_IsNull(self.as_ptr()) }
    }
}

// wxGraphicsPath
pub trait GraphicsPathMethods: GraphicsObjectMethods {
    // NOT_SUPPORTED: fn AddArc()
    // NOT_SUPPORTED: fn AddArc1()
    // NOT_SUPPORTED: fn AddArcToPoint()
    // NOT_SUPPORTED: fn AddCircle()
    // NOT_SUPPORTED: fn AddCurveToPoint()
    fn add_curve_to_point<
        P: Point2DDoubleMethods,
        P2: Point2DDoubleMethods,
        P3: Point2DDoubleMethods,
    >(
        &self,
        c1: &P,
        c2: &P2,
        e: &P3,
    ) {
        unsafe {
            let c1 = c1.as_ptr();
            let c2 = c2.as_ptr();
            let e = e.as_ptr();
            ffi::wxGraphicsPath_AddCurveToPoint1(self.as_ptr(), c1, c2, e)
        }
    }
    // NOT_SUPPORTED: fn AddEllipse()
    // NOT_SUPPORTED: fn AddLineToPoint()
    fn add_line_to_point<P: Point2DDoubleMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxGraphicsPath_AddLineToPoint1(self.as_ptr(), p)
        }
    }
    fn add_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsPath_AddPath(self.as_ptr(), path)
        }
    }
    // NOT_SUPPORTED: fn AddQuadCurveToPoint()
    // NOT_SUPPORTED: fn AddRectangle()
    // NOT_SUPPORTED: fn AddRoundedRectangle()
    fn close_subpath(&self) {
        unsafe { ffi::wxGraphicsPath_CloseSubpath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    fn get_box(&self) -> Rect2DDouble {
        unsafe { Rect2DDouble::from_ptr(ffi::wxGraphicsPath_GetBox(self.as_ptr())) }
    }
    fn get_box_double(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetBox1(self.as_ptr(), x, y, w, h) }
    }
    fn get_current_point_double(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetCurrentPoint(self.as_ptr(), x, y) }
    }
    fn get_current_point(&self) -> Point2DDouble {
        unsafe { Point2DDouble::from_ptr(ffi::wxGraphicsPath_GetCurrentPoint1(self.as_ptr())) }
    }
    fn get_native_path(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsPath_GetNativePath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn MoveToPoint()
    fn move_to_point<P: Point2DDoubleMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxGraphicsPath_MoveToPoint1(self.as_ptr(), p)
        }
    }
    fn transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsPath_Transform(self.as_ptr(), matrix)
        }
    }
    fn un_get_native_path(&self, p: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_UnGetNativePath(self.as_ptr(), p) }
    }
}

// wxGraphicsPen
pub trait GraphicsPenMethods: GraphicsObjectMethods {}

// wxGridBagSizer
pub trait GridBagSizerMethods: FlexGridSizerMethods {
    fn add_window_gbposition<
        W: WindowMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        &self,
        window: Option<&W>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_Add(self.as_ptr(), window, pos, span, flag, border, user_data)
        }
    }
    fn add_sizer_gbposition<
        S: SizerMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        &self,
        sizer: Option<&S>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_Add1(self.as_ptr(), sizer, pos, span, flag, border, user_data)
        }
    }
    fn add_gbsizeritem(&self, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxGridBagSizer_Add2(self.as_ptr(), item) }
    }
    fn add_int_gbposition<G: GBPositionMethods, G2: GBSpanMethods, O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_Add3(
                self.as_ptr(),
                width,
                height,
                pos,
                span,
                flag,
                border,
                user_data,
            )
        }
    }
    fn check_for_intersection_gbsizeritem(
        &self,
        item: *mut c_void,
        exclude_item: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxGridBagSizer_CheckForIntersection(self.as_ptr(), item, exclude_item) }
    }
    fn check_for_intersection_gbposition<G: GBPositionMethods, G2: GBSpanMethods>(
        &self,
        pos: &G,
        span: &G2,
        exclude_item: *mut c_void,
    ) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            ffi::wxGridBagSizer_CheckForIntersection1(self.as_ptr(), pos, span, exclude_item)
        }
    }
    fn find_item_window<W: WindowMethods>(&self, window: Option<&W>) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_FindItem(self.as_ptr(), window)
        }
    }
    fn find_item_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_FindItem1(self.as_ptr(), sizer)
        }
    }
    fn find_item_at_point<P: PointMethods>(&self, pt: &P) -> *mut c_void {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxGridBagSizer_FindItemAtPoint(self.as_ptr(), pt)
        }
    }
    fn find_item_at_position<G: GBPositionMethods>(&self, pos: &G) -> *mut c_void {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_FindItemAtPosition(self.as_ptr(), pos)
        }
    }
    fn find_item_with_data<O: ObjectMethods>(&self, user_data: Option<&O>) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_FindItemWithData(self.as_ptr(), user_data)
        }
    }
    fn get_cell_size(&self, row: c_int, col: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetCellSize(self.as_ptr(), row, col)) }
    }
    fn get_empty_cell_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetEmptyCellSize(self.as_ptr())) }
    }
    fn get_item_position_window<W: WindowMethods>(&self, window: Option<&W>) -> GBPosition {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition(self.as_ptr(), window))
        }
    }
    fn get_item_position_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> GBPosition {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition1(self.as_ptr(), sizer))
        }
    }
    fn get_item_position_sz(&self, index: usize) -> GBPosition {
        unsafe { GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition2(self.as_ptr(), index)) }
    }
    fn get_item_span_window<W: WindowMethods>(&self, window: Option<&W>) -> GBSpan {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan(self.as_ptr(), window))
        }
    }
    fn get_item_span_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> GBSpan {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan1(self.as_ptr(), sizer))
        }
    }
    fn get_item_span_sz(&self, index: usize) -> GBSpan {
        unsafe { GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan2(self.as_ptr(), index)) }
    }
    fn set_empty_cell_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxGridBagSizer_SetEmptyCellSize(self.as_ptr(), sz)
        }
    }
    fn set_item_position_window<W: WindowMethods, G: GBPositionMethods>(
        &self,
        window: Option<&W>,
        pos: &G,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition(self.as_ptr(), window, pos)
        }
    }
    fn set_item_position_sizer<S: SizerMethods, G: GBPositionMethods>(
        &self,
        sizer: Option<&S>,
        pos: &G,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition1(self.as_ptr(), sizer, pos)
        }
    }
    fn set_item_position_sz<G: GBPositionMethods>(&self, index: usize, pos: &G) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition2(self.as_ptr(), index, pos)
        }
    }
    fn set_item_span_window<W: WindowMethods, G: GBSpanMethods>(
        &self,
        window: Option<&W>,
        span: &G,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan(self.as_ptr(), window, span)
        }
    }
    fn set_item_span_sizer<S: SizerMethods, G: GBSpanMethods>(
        &self,
        sizer: Option<&S>,
        span: &G,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan1(self.as_ptr(), sizer, span)
        }
    }
    fn set_item_span_sz<G: GBSpanMethods>(&self, index: usize, span: &G) -> bool {
        unsafe {
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan2(self.as_ptr(), index, span)
        }
    }
}

// wxGridEditorCreatedEvent
pub trait GridEditorCreatedEventMethods: CommandEventMethods {
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetCol(self.as_ptr()) }
    }
    fn get_control(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxGridEditorCreatedEvent_GetControl(self.as_ptr())) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetRow(self.as_ptr()) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGridEditorCreatedEvent_GetWindow(self.as_ptr())) }
    }
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetCol(self.as_ptr(), col) }
    }
    fn set_control<C: ControlMethods>(&self, ctrl: Option<&C>) {
        unsafe {
            let ctrl = match ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetControl(self.as_ptr(), ctrl)
        }
    }
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetRow(self.as_ptr(), row) }
    }
    fn set_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetWindow(self.as_ptr(), window)
        }
    }
}

// wxGridEvent
pub trait GridEventMethods: NotifyEventMethods {
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_AltDown(self.as_ptr()) }
    }
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ControlDown(self.as_ptr()) }
    }
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetCol(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridEvent_GetPosition(self.as_ptr())) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetRow(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_MetaDown(self.as_ptr()) }
    }
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridEvent_Selecting(self.as_ptr()) }
    }
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridFitMode
pub trait GridFitModeMethods: WxRustMethods {
    fn is_specified(&self) -> bool {
        unsafe { ffi::wxGridFitMode_IsSpecified(self.as_ptr()) }
    }
    fn is_clip(&self) -> bool {
        unsafe { ffi::wxGridFitMode_IsClip(self.as_ptr()) }
    }
    fn is_overflow(&self) -> bool {
        unsafe { ffi::wxGridFitMode_IsOverflow(self.as_ptr()) }
    }
    fn get_ellipsize_mode(&self) -> c_int {
        unsafe { ffi::wxGridFitMode_GetEllipsizeMode(self.as_ptr()) }
    }
    fn clip() -> GridFitMode {
        unsafe { GridFitMode::from_ptr(ffi::wxGridFitMode_Clip()) }
    }
    fn overflow() -> GridFitMode {
        unsafe { GridFitMode::from_ptr(ffi::wxGridFitMode_Overflow()) }
    }
    fn ellipsize(ellipsize: c_int) -> GridFitMode {
        unsafe { GridFitMode::from_ptr(ffi::wxGridFitMode_Ellipsize(ellipsize)) }
    }
}

// wxGridRangeSelectEvent
pub trait GridRangeSelectEventMethods: NotifyEventMethods {
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_AltDown(self.as_ptr()) }
    }
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ControlDown(self.as_ptr()) }
    }
    fn get_bottom_right_coords(&self) -> GridCellCoords {
        unsafe {
            GridCellCoords::from_ptr(ffi::wxGridRangeSelectEvent_GetBottomRightCoords(
                self.as_ptr(),
            ))
        }
    }
    fn get_bottom_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetBottomRow(self.as_ptr()) }
    }
    fn get_left_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetLeftCol(self.as_ptr()) }
    }
    fn get_right_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetRightCol(self.as_ptr()) }
    }
    fn get_top_left_coords(&self) -> GridCellCoords {
        unsafe {
            GridCellCoords::from_ptr(ffi::wxGridRangeSelectEvent_GetTopLeftCoords(self.as_ptr()))
        }
    }
    fn get_top_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetTopRow(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_MetaDown(self.as_ptr()) }
    }
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_Selecting(self.as_ptr()) }
    }
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizeEvent
pub trait GridSizeEventMethods: NotifyEventMethods {
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_AltDown(self.as_ptr()) }
    }
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ControlDown(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridSizeEvent_GetPosition(self.as_ptr())) }
    }
    fn get_row_or_col(&self) -> c_int {
        unsafe { ffi::wxGridSizeEvent_GetRowOrCol(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_MetaDown(self.as_ptr()) }
    }
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizer
pub trait GridSizerMethods: SizerMethods {
    fn get_cols(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetCols(self.as_ptr()) }
    }
    fn get_rows(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetRows(self.as_ptr()) }
    }
    fn get_effective_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveColsCount(self.as_ptr()) }
    }
    fn get_effective_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveRowsCount(self.as_ptr()) }
    }
    fn get_h_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetHGap(self.as_ptr()) }
    }
    fn get_v_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetVGap(self.as_ptr()) }
    }
    fn set_cols(&self, cols: c_int) {
        unsafe { ffi::wxGridSizer_SetCols(self.as_ptr(), cols) }
    }
    fn set_h_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetHGap(self.as_ptr(), gap) }
    }
    fn set_rows(&self, rows: c_int) {
        unsafe { ffi::wxGridSizer_SetRows(self.as_ptr(), rows) }
    }
    fn set_v_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetVGap(self.as_ptr(), gap) }
    }
}

// wxGridUpdateLocker
pub trait GridUpdateLockerMethods: WxRustMethods {
    // DTOR: fn ~wxGridUpdateLocker()
    fn create(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridUpdateLocker_Create(self.as_ptr(), grid) }
    }
}

// wxHeaderColumn
pub trait HeaderColumnMethods: WxRustMethods {
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHeaderColumn_GetTitle(self.as_ptr())).into() }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxHeaderColumn_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxHeaderColumn_GetBitmapBundle(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetWidth(self.as_ptr()) }
    }
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetMinWidth(self.as_ptr()) }
    }
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetAlignment(self.as_ptr()) }
    }
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetFlags(self.as_ptr()) }
    }
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxHeaderColumn_HasFlag(self.as_ptr(), flag) }
    }
    fn is_resizeable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsResizeable(self.as_ptr()) }
    }
    fn is_sortable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortable(self.as_ptr()) }
    }
    fn is_reorderable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsReorderable(self.as_ptr()) }
    }
    fn is_hidden(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsHidden(self.as_ptr()) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsShown(self.as_ptr()) }
    }
    fn is_sort_key(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortKey(self.as_ptr()) }
    }
    fn is_sort_order_ascending(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortOrderAscending(self.as_ptr()) }
    }
}

// wxHeaderColumnSimple
pub trait HeaderColumnSimpleMethods: SettableHeaderColumnMethods {}

// wxHeaderCtrl
pub trait HeaderCtrlMethods: ControlMethods {
    fn set_column_count(&self, count: c_uint) {
        unsafe { ffi::wxHeaderCtrl_SetColumnCount(self.as_ptr(), count) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_IsEmpty(self.as_ptr()) }
    }
    fn update_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrl_UpdateColumn(self.as_ptr(), idx) }
    }
    fn set_columns_order<A: ArrayIntMethods>(&self, order: &A) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_SetColumnsOrder(self.as_ptr(), order)
        }
    }
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxHeaderCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    fn get_column_at(&self, pos: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnAt(self.as_ptr(), pos) }
    }
    fn get_column_pos(&self, idx: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnPos(self.as_ptr(), idx) }
    }
    fn reset_columns_order(&self) {
        unsafe { ffi::wxHeaderCtrl_ResetColumnsOrder(self.as_ptr()) }
    }
    fn show_columns_menu<P: PointMethods>(&self, pt: &P, title: &str) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxHeaderCtrl_ShowColumnsMenu(self.as_ptr(), pt, title)
        }
    }
    fn add_columns_items<M: MenuMethods>(&self, menu: &M, id_columns_base: c_int) {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxHeaderCtrl_AddColumnsItems(self.as_ptr(), menu, id_columns_base)
        }
    }
    fn show_customize_dialog(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_ShowCustomizeDialog(self.as_ptr()) }
    }
    fn get_column_title_width_headercolumn<H: HeaderColumnMethods>(&self, col: &H) -> c_int {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrl_GetColumnTitleWidth(self.as_ptr(), col)
        }
    }
    fn get_column_title_width_uint(&self, idx: c_uint) -> c_int {
        unsafe { ffi::wxHeaderCtrl_GetColumnTitleWidth1(self.as_ptr(), idx) }
    }
    fn move_column_in_order_array<A: ArrayIntMethods>(order: &A, idx: c_uint, pos: c_uint) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_MoveColumnInOrderArray(order, idx, pos)
        }
    }
}

// wxHeaderCtrlEvent
pub trait HeaderCtrlEventMethods: NotifyEventMethods {
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetColumn(self.as_ptr()) }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetColumn(self.as_ptr(), col) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderCtrlEvent_GetWidth(self.as_ptr()) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxHeaderCtrlEvent_SetWidth(self.as_ptr(), width) }
    }
    fn get_new_order(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrlEvent_GetNewOrder(self.as_ptr()) }
    }
    fn set_new_order(&self, order: c_uint) {
        unsafe { ffi::wxHeaderCtrlEvent_SetNewOrder(self.as_ptr(), order) }
    }
}

// wxHeaderCtrlSimple
pub trait HeaderCtrlSimpleMethods: HeaderCtrlMethods {
    fn insert_column<H: HeaderColumnSimpleMethods>(&self, col: &H, idx: c_uint) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_InsertColumn(self.as_ptr(), col, idx)
        }
    }
    fn append_column<H: HeaderColumnSimpleMethods>(&self, col: &H) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_AppendColumn(self.as_ptr(), col)
        }
    }
    fn delete_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_DeleteColumn(self.as_ptr(), idx) }
    }
    fn show_column(&self, idx: c_uint, show: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowColumn(self.as_ptr(), idx, show) }
    }
    fn hide_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_HideColumn(self.as_ptr(), idx) }
    }
    fn show_sort_indicator(&self, idx: c_uint, sort_order: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowSortIndicator(self.as_ptr(), idx, sort_order) }
    }
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxHeaderCtrlSimple_RemoveSortIndicator(self.as_ptr()) }
    }
}

// wxHelpControllerHelpProvider
pub trait HelpControllerHelpProviderMethods: SimpleHelpProviderMethods {
    fn get_help_controller(&self) -> *mut c_void {
        unsafe { ffi::wxHelpControllerHelpProvider_GetHelpController(self.as_ptr()) }
    }
    fn set_help_controller(&self, hc: *mut c_void) {
        unsafe { ffi::wxHelpControllerHelpProvider_SetHelpController(self.as_ptr(), hc) }
    }
}

// wxHelpEvent
pub trait HelpEventMethods: CommandEventMethods {
    // NOT_SUPPORTED: fn GetOrigin()
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxHelpEvent_GetPosition(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetOrigin()
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxHelpEvent_SetPosition(self.as_ptr(), pt)
        }
    }
}

// wxHelpProvider
pub trait HelpProviderMethods: WxRustMethods {
    // DTOR: fn ~wxHelpProvider()
    fn add_help_window<W: WindowMethods>(&self, window: Option<&W>, text: &str) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxHelpProvider_AddHelp(self.as_ptr(), window, text)
        }
    }
    fn add_help_windowid(&self, id: c_int, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxHelpProvider_AddHelp1(self.as_ptr(), id, text)
        }
    }
    fn get_help<W: WindowMethods>(&self, window: Option<&W>) -> String {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WxString::from_ptr(ffi::wxHelpProvider_GetHelp(self.as_ptr(), window)).into()
        }
    }
    fn remove_help<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxHelpProvider_RemoveHelp(self.as_ptr(), window)
        }
    }
    fn show_help<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxHelpProvider_ShowHelp(self.as_ptr(), window)
        }
    }
    // NOT_SUPPORTED: fn ShowHelpAtPoint()
    fn get() -> Option<HelpProviderIsOwned<false>> {
        unsafe { HelpProvider::option_from(ffi::wxHelpProvider_Get()) }
    }
    fn set<H: HelpProviderMethods>(
        help_provider: Option<&H>,
    ) -> Option<HelpProviderIsOwned<false>> {
        unsafe {
            let help_provider = match help_provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            HelpProvider::option_from(ffi::wxHelpProvider_Set(help_provider))
        }
    }
}

// wxHyperlinkCtrl
pub trait HyperlinkCtrlMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHyperlinkCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                url,
                pos,
                size,
                style,
                name,
            )
        }
    }
    fn get_hover_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetHoverColour(self.as_ptr())) }
    }
    fn get_normal_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetNormalColour(self.as_ptr())) }
    }
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkCtrl_GetURL(self.as_ptr())).into() }
    }
    fn get_visited(&self) -> bool {
        unsafe { ffi::wxHyperlinkCtrl_GetVisited(self.as_ptr()) }
    }
    fn get_visited_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetVisitedColour(self.as_ptr())) }
    }
    fn set_hover_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetHoverColour(self.as_ptr(), colour)
        }
    }
    fn set_normal_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetNormalColour(self.as_ptr(), colour)
        }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkCtrl_SetURL(self.as_ptr(), url)
        }
    }
    fn set_visited(&self, visited: bool) {
        unsafe { ffi::wxHyperlinkCtrl_SetVisited(self.as_ptr(), visited) }
    }
    fn set_visited_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetVisitedColour(self.as_ptr(), colour)
        }
    }
}

// wxHyperlinkEvent
pub trait HyperlinkEventMethods: CommandEventMethods {
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkEvent_GetURL(self.as_ptr())).into() }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkEvent_SetURL(self.as_ptr(), url)
        }
    }
}

// wxIcon
pub trait IconMethods: GDIObjectMethods {
    // DTOR: fn ~wxIcon()
    // NOT_SUPPORTED: fn CreateFromHICON()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    fn copy_from_bitmap<B: BitmapMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxIcon_CopyFromBitmap(self.as_ptr(), bmp)
        }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxIcon_GetDepth(self.as_ptr()) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxIcon_GetHeight(self.as_ptr()) }
    }
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxIcon_GetLogicalHeight(self.as_ptr()) }
    }
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxIcon_GetLogicalSize(self.as_ptr())) }
    }
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxIcon_GetLogicalWidth(self.as_ptr()) }
    }
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxIcon_GetScaleFactor(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxIcon_GetSize(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxIcon_GetWidth(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxIcon_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    fn set_depth(&self, depth: c_int) {
        unsafe { ffi::wxIcon_SetDepth(self.as_ptr(), depth) }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxIcon_SetHeight(self.as_ptr(), height) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxIcon_SetWidth(self.as_ptr(), width) }
    }
    // BLOCKED: fn operator=()
}

// wxIconBundle
pub trait IconBundleMethods: GDIObjectMethods {
    // DTOR: fn ~wxIconBundle()
    // NOT_SUPPORTED: fn AddIcon()
    // NOT_SUPPORTED: fn AddIcon1()
    // NOT_SUPPORTED: fn AddIcon2()
    fn add_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxIconBundle_AddIcon3(self.as_ptr(), icon)
        }
    }
    fn get_icon_size<S: SizeMethods>(&self, size: &S, flags: c_int) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxIconBundle_GetIcon(self.as_ptr(), size, flags))
        }
    }
    fn get_icon_coord(&self, size: c_int, flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxIconBundle_GetIcon1(self.as_ptr(), size, flags)) }
    }
    fn get_icon_of_exact_size<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxIconBundle_GetIconOfExactSize(self.as_ptr(), size))
        }
    }
    fn get_icon_count(&self) -> usize {
        unsafe { ffi::wxIconBundle_GetIconCount(self.as_ptr()) }
    }
    fn get_icon_by_index(&self, n: usize) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxIconBundle_GetIconByIndex(self.as_ptr(), n)) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxIconBundle_IsEmpty(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxIdManager
pub trait IdManagerMethods: WxRustMethods {
    fn reserve_id(count: c_int) -> c_int {
        unsafe { ffi::wxIdManager_ReserveId(count) }
    }
    fn unreserve_id(id: c_int, count: c_int) {
        unsafe { ffi::wxIdManager_UnreserveId(id, count) }
    }
}

// wxImageList
pub trait ImageListMethods: ObjectMethods {
    fn add_bitmap_bitmap<B: BitmapMethods, B2: BitmapMethods>(
        &self,
        bitmap: &B,
        mask: &B2,
    ) -> c_int {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let mask = mask.as_ptr();
            ffi::wxImageList_Add(self.as_ptr(), bitmap, mask)
        }
    }
    fn add_bitmap_colour<B: BitmapMethods, C: ColourMethods>(
        &self,
        bitmap: &B,
        mask_colour: &C,
    ) -> c_int {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let mask_colour = mask_colour.as_ptr();
            ffi::wxImageList_Add1(self.as_ptr(), bitmap, mask_colour)
        }
    }
    fn add_icon<I: IconMethods>(&self, icon: &I) -> c_int {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxImageList_Add2(self.as_ptr(), icon)
        }
    }
    fn create(&self, width: c_int, height: c_int, mask: bool, initial_count: c_int) -> bool {
        unsafe { ffi::wxImageList_Create(self.as_ptr(), width, height, mask, initial_count) }
    }
    fn destroy(&self) {
        unsafe { ffi::wxImageList_Destroy(self.as_ptr()) }
    }
    fn draw<D: DCMethods>(
        &self,
        index: c_int,
        dc: &D,
        x: c_int,
        y: c_int,
        flags: c_int,
        solid_background: bool,
    ) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxImageList_Draw(self.as_ptr(), index, dc, x, y, flags, solid_background)
        }
    }
    fn get_bitmap(&self, index: c_int) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxImageList_GetBitmap(self.as_ptr(), index)) }
    }
    fn get_icon(&self, index: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxImageList_GetIcon(self.as_ptr(), index)) }
    }
    fn get_image_count(&self) -> c_int {
        unsafe { ffi::wxImageList_GetImageCount(self.as_ptr()) }
    }
    fn get_size_int(&self, index: c_int, width: *mut c_void, height: *mut c_void) -> bool {
        unsafe { ffi::wxImageList_GetSize(self.as_ptr(), index, width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxImageList_GetSize1(self.as_ptr())) }
    }
    fn remove(&self, index: c_int) -> bool {
        unsafe { ffi::wxImageList_Remove(self.as_ptr(), index) }
    }
    fn remove_all(&self) -> bool {
        unsafe { ffi::wxImageList_RemoveAll(self.as_ptr()) }
    }
    fn replace_bitmap<B: BitmapMethods, B2: BitmapMethods>(
        &self,
        index: c_int,
        bitmap: &B,
        mask: &B2,
    ) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let mask = mask.as_ptr();
            ffi::wxImageList_Replace(self.as_ptr(), index, bitmap, mask)
        }
    }
    fn replace_icon<I: IconMethods>(&self, index: c_int, icon: &I) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxImageList_Replace1(self.as_ptr(), index, icon)
        }
    }
}

// wxInfoBar
pub trait InfoBarMethods: ControlMethods {
    // NOT_SUPPORTED: fn SetShowHideEffects()
    // NOT_SUPPORTED: fn GetShowEffect()
    // NOT_SUPPORTED: fn GetHideEffect()
    fn set_effect_duration(&self, duration: c_int) {
        unsafe { ffi::wxInfoBar_SetEffectDuration(self.as_ptr(), duration) }
    }
    fn get_effect_duration(&self) -> c_int {
        unsafe { ffi::wxInfoBar_GetEffectDuration(self.as_ptr()) }
    }
    fn create<W: WindowMethods>(&self, parent: Option<&W>, winid: c_int) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxInfoBar_Create(self.as_ptr(), parent, winid)
        }
    }
    fn add_button(&self, btnid: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxInfoBar_AddButton(self.as_ptr(), btnid, label)
        }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxInfoBar_Dismiss(self.as_ptr()) }
    }
    fn remove_button(&self, btnid: c_int) {
        unsafe { ffi::wxInfoBar_RemoveButton(self.as_ptr(), btnid) }
    }
    fn show_message(&self, msg: &str, flags: c_int) {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            ffi::wxInfoBar_ShowMessage(self.as_ptr(), msg, flags)
        }
    }
    fn get_button_count(&self) -> usize {
        unsafe { ffi::wxInfoBar_GetButtonCount(self.as_ptr()) }
    }
    fn get_button_id(&self, idx: usize) -> c_int {
        unsafe { ffi::wxInfoBar_GetButtonId(self.as_ptr(), idx) }
    }
    fn has_button_id(&self, btnid: c_int) -> bool {
        unsafe { ffi::wxInfoBar_HasButtonId(self.as_ptr(), btnid) }
    }
}

// wxInitDialogEvent
pub trait InitDialogEventMethods: EventMethods {}

// wxItemAttr
pub trait ItemAttrMethods: WxRustMethods {
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxItemAttr_GetBackgroundColour(self.as_ptr())) }
    }
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxItemAttr_GetFont(self.as_ptr())) }
    }
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxItemAttr_GetTextColour(self.as_ptr())) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxItemAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn has_colours(&self) -> bool {
        unsafe { ffi::wxItemAttr_HasColours(self.as_ptr()) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxItemAttr_HasFont(self.as_ptr()) }
    }
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxItemAttr_HasTextColour(self.as_ptr()) }
    }
    fn is_default(&self) -> bool {
        unsafe { ffi::wxItemAttr_IsDefault(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxItemAttr_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxItemAttr_SetFont(self.as_ptr(), font)
        }
    }
    fn set_text_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxItemAttr_SetTextColour(self.as_ptr(), colour)
        }
    }
}

// wxItemContainer
pub trait ItemContainerMethods: ItemContainerImmutableMethods {
    fn as_item_container(&self) -> *mut c_void;
    fn append_str(&self, item: &str) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Append(self.as_item_container(), item)
        }
    }
    fn append_str_void(&self, item: &str, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Append1(self.as_item_container(), item, client_data)
        }
    }
    fn append_str_clientdata<C: ClientDataMethods>(
        &self,
        item: &str,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Append2(self.as_item_container(), item, client_data)
        }
    }
    fn append_arraystring<A: ArrayStringMethods>(&self, items: &A) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Append3(self.as_item_container(), items)
        }
    }
    // BLOCKED: fn Append4()
    fn append_arraystring_void<A: ArrayStringMethods>(
        &self,
        items: &A,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Append5(self.as_item_container(), items, client_data)
        }
    }
    fn append_arraystring_clientdata<A: ArrayStringMethods, C: ClientDataMethods>(
        &self,
        items: &A,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let items = items.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Append6(self.as_item_container(), items, client_data)
        }
    }
    fn append_uint(&self, n: c_uint, items: *const c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append7(self.as_item_container(), n, items) }
    }
    fn append_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append8(self.as_item_container(), n, items, client_data) }
    }
    fn append_uint_clientdata<C: ClientDataMethods>(
        &self,
        n: c_uint,
        items: *const c_void,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Append9(self.as_item_container(), n, items, client_data)
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxItemContainer_Clear(self.as_item_container()) }
    }
    fn delete(&self, n: c_uint) {
        unsafe { ffi::wxItemContainer_Delete(self.as_item_container(), n) }
    }
    fn detach_client_object(&self, n: c_uint) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxItemContainer_DetachClientObject(
                self.as_item_container(),
                n,
            ))
        }
    }
    fn has_client_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientData(self.as_item_container()) }
    }
    fn has_client_object_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientObjectData(self.as_item_container()) }
    }
    fn has_client_untyped_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientUntypedData(self.as_item_container()) }
    }
    fn get_client_data(&self, n: c_uint) -> *mut c_void {
        unsafe { ffi::wxItemContainer_GetClientData(self.as_item_container(), n) }
    }
    fn get_client_object_uint(&self, n: c_uint) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxItemContainer_GetClientObject(
                self.as_item_container(),
                n,
            ))
        }
    }
    fn set_client_data(&self, n: c_uint, data: *mut c_void) {
        unsafe { ffi::wxItemContainer_SetClientData(self.as_item_container(), n, data) }
    }
    fn set_client_object_uint<C: ClientDataMethods>(&self, n: c_uint, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_SetClientObject(self.as_item_container(), n, data)
        }
    }
    fn insert_str_uint(&self, item: &str, pos: c_uint) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Insert(self.as_item_container(), item, pos)
        }
    }
    fn insert_str_uint_void(&self, item: &str, pos: c_uint, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Insert1(self.as_item_container(), item, pos, client_data)
        }
    }
    fn insert_str_uint_clientdata<C: ClientDataMethods>(
        &self,
        item: &str,
        pos: c_uint,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Insert2(self.as_item_container(), item, pos, client_data)
        }
    }
    fn insert_arraystring<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Insert3(self.as_item_container(), items, pos)
        }
    }
    // BLOCKED: fn Insert4()
    fn insert_arraystring_void<A: ArrayStringMethods>(
        &self,
        items: &A,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Insert5(self.as_item_container(), items, pos, client_data)
        }
    }
    fn insert_arraystring_clientdata<A: ArrayStringMethods, C: ClientDataMethods>(
        &self,
        items: &A,
        pos: c_uint,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let items = items.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Insert6(self.as_item_container(), items, pos, client_data)
        }
    }
    fn insert_uint(&self, n: c_uint, items: *const c_void, pos: c_uint) -> c_int {
        unsafe { ffi::wxItemContainer_Insert7(self.as_item_container(), n, items, pos) }
    }
    fn insert_uint_void(
        &self,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            ffi::wxItemContainer_Insert8(self.as_item_container(), n, items, pos, client_data)
        }
    }
    fn insert_uint_clientdata<C: ClientDataMethods>(
        &self,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: Option<&C>,
    ) -> c_int {
        unsafe {
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Insert9(self.as_item_container(), n, items, pos, client_data)
        }
    }
    fn set_arraystring<A: ArrayStringMethods>(&self, items: &A) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set(self.as_item_container(), items)
        }
    }
    // BLOCKED: fn Set1()
    fn set_arraystring_void<A: ArrayStringMethods>(&self, items: &A, client_data: *mut c_void) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set2(self.as_item_container(), items, client_data)
        }
    }
    fn set_arraystring_clientdata<A: ArrayStringMethods, C: ClientDataMethods>(
        &self,
        items: &A,
        client_data: Option<&C>,
    ) {
        unsafe {
            let items = items.as_ptr();
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Set3(self.as_item_container(), items, client_data)
        }
    }
    fn set_uint(&self, n: c_uint, items: *const c_void) {
        unsafe { ffi::wxItemContainer_Set4(self.as_item_container(), n, items) }
    }
    fn set_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) {
        unsafe { ffi::wxItemContainer_Set5(self.as_item_container(), n, items, client_data) }
    }
    fn set_uint_clientdata<C: ClientDataMethods>(
        &self,
        n: c_uint,
        items: *const c_void,
        client_data: Option<&C>,
    ) {
        unsafe {
            let client_data = match client_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_Set6(self.as_item_container(), n, items, client_data)
        }
    }
}

// wxItemContainerImmutable
pub trait ItemContainerImmutableMethods: WxRustMethods {
    fn as_item_container_immutable(&self) -> *mut c_void;
    fn set_selection(&self, n: c_int) {
        unsafe { ffi::wxItemContainerImmutable_SetSelection(self.as_item_container_immutable(), n) }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxItemContainerImmutable_GetSelection(self.as_item_container_immutable()) }
    }
    fn set_string_selection(&self, string: &str) -> bool {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxItemContainerImmutable_SetStringSelection(
                self.as_item_container_immutable(),
                string,
            )
        }
    }
    fn get_string_selection(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxItemContainerImmutable_GetStringSelection(
                self.as_item_container_immutable(),
            ))
            .into()
        }
    }
    fn select(&self, n: c_int) {
        unsafe { ffi::wxItemContainerImmutable_Select(self.as_item_container_immutable(), n) }
    }
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxItemContainerImmutable_GetCount(self.as_item_container_immutable()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxItemContainerImmutable_IsEmpty(self.as_item_container_immutable()) }
    }
    fn get_string(&self, n: c_uint) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxItemContainerImmutable_GetString(
                self.as_item_container_immutable(),
                n,
            ))
            .into()
        }
    }
    fn get_strings(&self) -> ArrayString {
        unsafe {
            ArrayString::from_ptr(ffi::wxItemContainerImmutable_GetStrings(
                self.as_item_container_immutable(),
            ))
        }
    }
    fn set_string(&self, n: c_uint, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxItemContainerImmutable_SetString(self.as_item_container_immutable(), n, string)
        }
    }
    fn find_string(&self, string: &str, case_sensitive: bool) -> c_int {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxItemContainerImmutable_FindString(
                self.as_item_container_immutable(),
                string,
                case_sensitive,
            )
        }
    }
}

// wxJoystick
pub trait JoystickMethods: ObjectMethods {
    // DTOR: fn ~wxJoystick()
    fn get_button_state(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetButtonState(self.as_ptr()) }
    }
    fn get_button_state_uint(&self, id: c_uint) -> bool {
        unsafe { ffi::wxJoystick_GetButtonState1(self.as_ptr(), id) }
    }
    fn get_manufacturer_id(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetManufacturerId(self.as_ptr()) }
    }
    fn get_movement_threshold(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetMovementThreshold(self.as_ptr()) }
    }
    fn get_number_axes(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetNumberAxes(self.as_ptr()) }
    }
    fn get_number_buttons(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetNumberButtons(self.as_ptr()) }
    }
    fn get_povcts_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPOVCTSPosition(self.as_ptr()) }
    }
    fn get_pov_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPOVPosition(self.as_ptr()) }
    }
    fn get_polling_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPollingMax(self.as_ptr()) }
    }
    fn get_polling_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetPollingMin(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxJoystick_GetPosition(self.as_ptr())) }
    }
    fn get_position_uint(&self, axis: c_uint) -> c_int {
        unsafe { ffi::wxJoystick_GetPosition1(self.as_ptr(), axis) }
    }
    fn get_product_id(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetProductId(self.as_ptr()) }
    }
    fn get_product_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxJoystick_GetProductName(self.as_ptr())).into() }
    }
    fn get_rudder_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetRudderMax(self.as_ptr()) }
    }
    fn get_rudder_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetRudderMin(self.as_ptr()) }
    }
    fn get_rudder_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetRudderPosition(self.as_ptr()) }
    }
    fn get_u_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetUMax(self.as_ptr()) }
    }
    fn get_u_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetUMin(self.as_ptr()) }
    }
    fn get_u_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetUPosition(self.as_ptr()) }
    }
    fn get_v_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetVMax(self.as_ptr()) }
    }
    fn get_v_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetVMin(self.as_ptr()) }
    }
    fn get_v_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetVPosition(self.as_ptr()) }
    }
    fn get_x_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetXMax(self.as_ptr()) }
    }
    fn get_x_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetXMin(self.as_ptr()) }
    }
    fn get_y_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetYMax(self.as_ptr()) }
    }
    fn get_y_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetYMin(self.as_ptr()) }
    }
    fn get_z_max(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetZMax(self.as_ptr()) }
    }
    fn get_z_min(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetZMin(self.as_ptr()) }
    }
    fn get_z_position(&self) -> c_int {
        unsafe { ffi::wxJoystick_GetZPosition(self.as_ptr()) }
    }
    fn has_pov(&self) -> bool {
        unsafe { ffi::wxJoystick_HasPOV(self.as_ptr()) }
    }
    fn has_po_v4_dir(&self) -> bool {
        unsafe { ffi::wxJoystick_HasPOV4Dir(self.as_ptr()) }
    }
    fn has_povcts(&self) -> bool {
        unsafe { ffi::wxJoystick_HasPOVCTS(self.as_ptr()) }
    }
    fn has_rudder(&self) -> bool {
        unsafe { ffi::wxJoystick_HasRudder(self.as_ptr()) }
    }
    fn has_u(&self) -> bool {
        unsafe { ffi::wxJoystick_HasU(self.as_ptr()) }
    }
    fn has_v(&self) -> bool {
        unsafe { ffi::wxJoystick_HasV(self.as_ptr()) }
    }
    fn has_z(&self) -> bool {
        unsafe { ffi::wxJoystick_HasZ(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxJoystick_IsOk(self.as_ptr()) }
    }
    fn release_capture(&self) -> bool {
        unsafe { ffi::wxJoystick_ReleaseCapture(self.as_ptr()) }
    }
    fn set_capture<W: WindowMethods>(&self, win: Option<&W>, polling_freq: c_int) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxJoystick_SetCapture(self.as_ptr(), win, polling_freq)
        }
    }
    fn set_movement_threshold(&self, threshold: c_int) {
        unsafe { ffi::wxJoystick_SetMovementThreshold(self.as_ptr(), threshold) }
    }
    fn get_number_joysticks() -> c_int {
        unsafe { ffi::wxJoystick_GetNumberJoysticks() }
    }
}

// wxJoystickEvent
pub trait JoystickEventMethods: EventMethods {
    fn button_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonDown(self.as_ptr(), button) }
    }
    fn button_is_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonIsDown(self.as_ptr(), button) }
    }
    fn button_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxJoystickEvent_ButtonUp(self.as_ptr(), button) }
    }
    fn get_button_change(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonChange(self.as_ptr()) }
    }
    fn get_button_ordinal(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonOrdinal(self.as_ptr()) }
    }
    fn get_button_state(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetButtonState(self.as_ptr()) }
    }
    fn get_joystick(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetJoystick(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxJoystickEvent_GetPosition(self.as_ptr())) }
    }
    fn get_z_position(&self) -> c_int {
        unsafe { ffi::wxJoystickEvent_GetZPosition(self.as_ptr()) }
    }
    fn is_button(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsButton(self.as_ptr()) }
    }
    fn is_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsMove(self.as_ptr()) }
    }
    fn is_z_move(&self) -> bool {
        unsafe { ffi::wxJoystickEvent_IsZMove(self.as_ptr()) }
    }
}

// wxKeyEvent
pub trait KeyEventMethods: EventMethods {
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetKeyCode(self.as_ptr()) }
    }
    fn is_key_in_category(&self, category: c_int) -> bool {
        unsafe { ffi::wxKeyEvent_IsKeyInCategory(self.as_ptr(), category) }
    }
    fn is_auto_repeat(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsAutoRepeat(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxKeyEvent_GetPosition(self.as_ptr())) }
    }
    fn get_position_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxKeyEvent_GetPosition1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetRawKeyCode()
    // NOT_SUPPORTED: fn GetRawKeyFlags()
    // NOT_SUPPORTED: fn GetUnicodeKey()
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxKeyEvent_GetY(self.as_ptr()) }
    }
    fn do_allow_next_event(&self) {
        unsafe { ffi::wxKeyEvent_DoAllowNextEvent(self.as_ptr()) }
    }
    fn is_next_event_allowed(&self) -> bool {
        unsafe { ffi::wxKeyEvent_IsNextEventAllowed(self.as_ptr()) }
    }
}

// wxLayoutAlgorithm
pub trait LayoutAlgorithmMethods: ObjectMethods {
    // DTOR: fn ~wxLayoutAlgorithm()
    fn layout_frame<F: FrameMethods, W: WindowMethods>(
        &self,
        frame: Option<&F>,
        main_window: Option<&W>,
    ) -> bool {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_window = match main_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLayoutAlgorithm_LayoutFrame(self.as_ptr(), frame, main_window)
        }
    }
    fn layout_mdi_frame<M: MDIParentFrameMethods, R: RectMethods>(
        &self,
        frame: Option<&M>,
        rect: Option<&R>,
    ) -> bool {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLayoutAlgorithm_LayoutMDIFrame(self.as_ptr(), frame, rect)
        }
    }
    fn layout_window<W: WindowMethods, W2: WindowMethods>(
        &self,
        parent: Option<&W>,
        main_window: Option<&W2>,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_window = match main_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLayoutAlgorithm_LayoutWindow(self.as_ptr(), parent, main_window)
        }
    }
}

// wxListBox
pub trait ListBoxMethods: ControlMethods {
    // DTOR: fn ~wxListBox()
    // NOT_SUPPORTED: fn Create()
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxListBox_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    fn deselect(&self, n: c_int) {
        unsafe { ffi::wxListBox_Deselect(self.as_ptr(), n) }
    }
    fn set_string_selection_bool(&self, s: &str, select: bool) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxListBox_SetStringSelection(self.as_ptr(), s, select)
        }
    }
    fn set_string_selection(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxListBox_SetStringSelection1(self.as_ptr(), s)
        }
    }
    fn get_selections<A: ArrayIntMethods>(&self, selections: &A) -> c_int {
        unsafe {
            let selections = selections.as_ptr();
            ffi::wxListBox_GetSelections(self.as_ptr(), selections)
        }
    }
    fn hit_test_point<P: PointMethods>(&self, point: &P) -> c_int {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListBox_HitTest(self.as_ptr(), point)
        }
    }
    fn hit_test_int(&self, x: c_int, y: c_int) -> c_int {
        unsafe { ffi::wxListBox_HitTest1(self.as_ptr(), x, y) }
    }
    // BLOCKED: fn InsertItems()
    fn insert_items<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxListBox_InsertItems1(self.as_ptr(), items, pos)
        }
    }
    fn is_selected(&self, n: c_int) -> bool {
        unsafe { ffi::wxListBox_IsSelected(self.as_ptr(), n) }
    }
    fn set_first_item_int(&self, n: c_int) {
        unsafe { ffi::wxListBox_SetFirstItem(self.as_ptr(), n) }
    }
    fn set_first_item_str(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxListBox_SetFirstItem1(self.as_ptr(), string)
        }
    }
    fn ensure_visible(&self, n: c_int) {
        unsafe { ffi::wxListBox_EnsureVisible(self.as_ptr(), n) }
    }
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxListBox_IsSorted(self.as_ptr()) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListBox_GetCountPerPage(self.as_ptr()) }
    }
    fn get_top_item(&self) -> c_int {
        unsafe { ffi::wxListBox_GetTopItem(self.as_ptr()) }
    }
    // BLOCKED: fn MSWSetTabStops()
}

// wxListCtrl
pub trait ListCtrlMethods: ControlMethods {
    // DTOR: fn ~wxListCtrl()
    // NOT_SUPPORTED: fn AppendColumn()
    fn arrange(&self, flag: c_int) -> bool {
        unsafe { ffi::wxListCtrl_Arrange(self.as_ptr(), flag) }
    }
    fn assign_image_list<I: ImageListMethods>(&self, image_list: Option<&I>, which: c_int) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxListCtrl_AssignImageList(self.as_ptr(), image_list, which)
        }
    }
    fn clear_all(&self) {
        unsafe { ffi::wxListCtrl_ClearAll(self.as_ptr()) }
    }
    fn delete_all_columns(&self) -> bool {
        unsafe { ffi::wxListCtrl_DeleteAllColumns(self.as_ptr()) }
    }
    fn delete_all_items(&self) -> bool {
        unsafe { ffi::wxListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn delete_column(&self, col: c_int) -> bool {
        unsafe { ffi::wxListCtrl_DeleteColumn(self.as_ptr(), col) }
    }
    fn delete_item(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_DeleteItem(self.as_ptr(), item) }
    }
    fn edit_label<C: ClassInfoMethods>(
        &self,
        item: c_long,
        text_control_class: Option<&C>,
    ) -> WeakRef<TextCtrl> {
        unsafe {
            let text_control_class = match text_control_class {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<TextCtrl>::from(ffi::wxListCtrl_EditLabel(
                self.as_ptr(),
                item,
                text_control_class,
            ))
        }
    }
    fn enable_alternate_row_colours(&self, enable: bool) {
        unsafe { ffi::wxListCtrl_EnableAlternateRowColours(self.as_ptr(), enable) }
    }
    fn enable_bell_on_no_match(&self, on: bool) {
        unsafe { ffi::wxListCtrl_EnableBellOnNoMatch(self.as_ptr(), on) }
    }
    fn end_edit_label(&self, cancel: bool) -> bool {
        unsafe { ffi::wxListCtrl_EndEditLabel(self.as_ptr(), cancel) }
    }
    fn ensure_visible(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_EnsureVisible(self.as_ptr(), item) }
    }
    fn find_item_str(&self, start: c_long, str: &str, partial: bool) -> c_long {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxListCtrl_FindItem(self.as_ptr(), start, str, partial)
        }
    }
    // NOT_SUPPORTED: fn FindItem1()
    fn find_item_point<P: PointMethods>(&self, start: c_long, pt: &P, direction: c_int) -> c_long {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxListCtrl_FindItem2(self.as_ptr(), start, pt, direction)
        }
    }
    fn get_column<L: ListItemMethods>(&self, col: c_int, item: &L) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListCtrl_GetColumn(self.as_ptr(), col, item)
        }
    }
    fn get_column_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn get_column_index_from_order(&self, pos: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnIndexFromOrder(self.as_ptr(), pos) }
    }
    fn get_column_order(&self, col: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnOrder(self.as_ptr(), col) }
    }
    fn get_column_width(&self, col: c_int) -> c_int {
        unsafe { ffi::wxListCtrl_GetColumnWidth(self.as_ptr(), col) }
    }
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxListCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetCountPerPage(self.as_ptr()) }
    }
    fn get_edit_control(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxListCtrl_GetEditControl(self.as_ptr())) }
    }
    fn get_image_list(&self, which: c_int) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxListCtrl_GetImageList(self.as_ptr(), which)) }
    }
    fn get_item<L: ListItemMethods>(&self, info: &L) -> bool {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_GetItem(self.as_ptr(), info)
        }
    }
    fn get_item_background_colour(&self, item: c_long) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetItemBackgroundColour(self.as_ptr(), item)) }
    }
    fn get_item_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    fn get_item_font(&self, item: c_long) -> Font {
        unsafe { Font::from_ptr(ffi::wxListCtrl_GetItemFont(self.as_ptr(), item)) }
    }
    fn get_item_position<P: PointMethods>(&self, item: c_long, pos: &P) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxListCtrl_GetItemPosition(self.as_ptr(), item, pos)
        }
    }
    fn get_item_rect<R: RectMethods>(&self, item: c_long, rect: &R, code: c_int) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxListCtrl_GetItemRect(self.as_ptr(), item, rect, code)
        }
    }
    fn get_item_spacing(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxListCtrl_GetItemSpacing(self.as_ptr())) }
    }
    fn get_item_state(&self, item: c_long, state_mask: c_long) -> c_int {
        unsafe { ffi::wxListCtrl_GetItemState(self.as_ptr(), item, state_mask) }
    }
    fn get_item_text(&self, item: c_long, col: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxListCtrl_GetItemText(self.as_ptr(), item, col)).into() }
    }
    fn get_item_text_colour(&self, item: c_long) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetItemTextColour(self.as_ptr(), item)) }
    }
    fn get_next_item(&self, item: c_long, geometry: c_int, state: c_int) -> c_long {
        unsafe { ffi::wxListCtrl_GetNextItem(self.as_ptr(), item, geometry, state) }
    }
    fn get_selected_item_count(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetSelectedItemCount(self.as_ptr()) }
    }
    fn get_sub_item_rect<R: RectMethods>(
        &self,
        item: c_long,
        sub_item: c_long,
        rect: &R,
        code: c_int,
    ) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxListCtrl_GetSubItemRect(self.as_ptr(), item, sub_item, rect, code)
        }
    }
    fn get_text_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetTextColour(self.as_ptr())) }
    }
    fn get_top_item(&self) -> c_long {
        unsafe { ffi::wxListCtrl_GetTopItem(self.as_ptr()) }
    }
    fn get_view_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxListCtrl_GetViewRect(self.as_ptr())) }
    }
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxListCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    fn get_alternate_row_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListCtrl_GetAlternateRowColour(self.as_ptr())) }
    }
    fn hit_test<P: PointMethods>(
        &self,
        point: &P,
        flags: *mut c_void,
        ptr_sub_item: *mut c_void,
    ) -> c_long {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListCtrl_HitTest(self.as_ptr(), point, flags, ptr_sub_item)
        }
    }
    fn in_report_view(&self) -> bool {
        unsafe { ffi::wxListCtrl_InReportView(self.as_ptr()) }
    }
    fn insert_column_listitem<L: ListItemMethods>(&self, col: c_long, info: &L) -> c_long {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_InsertColumn(self.as_ptr(), col, info)
        }
    }
    fn insert_column_str(&self, col: c_long, heading: &str, format: c_int, width: c_int) -> c_long {
        unsafe {
            let heading = WxString::from(heading);
            let heading = heading.as_ptr();
            ffi::wxListCtrl_InsertColumn1(self.as_ptr(), col, heading, format, width)
        }
    }
    fn insert_item_listitem<L: ListItemMethods>(&self, info: &L) -> c_long {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_InsertItem(self.as_ptr(), info)
        }
    }
    fn insert_item_long_str(&self, index: c_long, label: &str) -> c_long {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_InsertItem1(self.as_ptr(), index, label)
        }
    }
    fn insert_item_long_int(&self, index: c_long, image_index: c_int) -> c_long {
        unsafe { ffi::wxListCtrl_InsertItem2(self.as_ptr(), index, image_index) }
    }
    fn insert_item_long_str_int(&self, index: c_long, label: &str, image_index: c_int) -> c_long {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_InsertItem3(self.as_ptr(), index, label, image_index)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsEmpty(self.as_ptr()) }
    }
    fn is_virtual(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsVirtual(self.as_ptr()) }
    }
    fn refresh_item(&self, item: c_long) {
        unsafe { ffi::wxListCtrl_RefreshItem(self.as_ptr(), item) }
    }
    fn refresh_items(&self, item_from: c_long, item_to: c_long) {
        unsafe { ffi::wxListCtrl_RefreshItems(self.as_ptr(), item_from, item_to) }
    }
    fn scroll_list(&self, dx: c_int, dy: c_int) -> bool {
        unsafe { ffi::wxListCtrl_ScrollList(self.as_ptr(), dx, dy) }
    }
    fn set_column<L: ListItemMethods>(&self, col: c_int, item: &L) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListCtrl_SetColumn(self.as_ptr(), col, item)
        }
    }
    fn set_column_width(&self, col: c_int, width: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetColumnWidth(self.as_ptr(), col, width) }
    }
    fn set_columns_order<A: ArrayIntMethods>(&self, orders: &A) -> bool {
        unsafe {
            let orders = orders.as_ptr();
            ffi::wxListCtrl_SetColumnsOrder(self.as_ptr(), orders)
        }
    }
    fn set_header_attr<I: ItemAttrMethods>(&self, attr: &I) -> bool {
        unsafe {
            let attr = attr.as_ptr();
            ffi::wxListCtrl_SetHeaderAttr(self.as_ptr(), attr)
        }
    }
    fn set_image_list<I: ImageListMethods>(&self, image_list: Option<&I>, which: c_int) {
        unsafe {
            let image_list = match image_list {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxListCtrl_SetImageList(self.as_ptr(), image_list, which)
        }
    }
    fn set_normal_images(&self, images: *const c_void) {
        unsafe { ffi::wxListCtrl_SetNormalImages(self.as_ptr(), images) }
    }
    fn set_small_images(&self, images: *const c_void) {
        unsafe { ffi::wxListCtrl_SetSmallImages(self.as_ptr(), images) }
    }
    fn is_visible(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_IsVisible(self.as_ptr(), item) }
    }
    fn set_item_listitem<L: ListItemMethods>(&self, info: &L) -> bool {
        unsafe {
            let info = info.as_ptr();
            ffi::wxListCtrl_SetItem(self.as_ptr(), info)
        }
    }
    fn set_item_long(&self, index: c_long, column: c_int, label: &str, image_id: c_int) -> bool {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxListCtrl_SetItem1(self.as_ptr(), index, column, label, image_id)
        }
    }
    fn set_item_background_colour<C: ColourMethods>(&self, item: c_long, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetItemBackgroundColour(self.as_ptr(), item, col)
        }
    }
    fn set_item_column_image(&self, item: c_long, column: c_long, image: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetItemColumnImage(self.as_ptr(), item, column, image) }
    }
    fn set_item_count(&self, count: c_long) {
        unsafe { ffi::wxListCtrl_SetItemCount(self.as_ptr(), count) }
    }
    fn set_item_data(&self, item: c_long, data: c_long) -> bool {
        unsafe { ffi::wxListCtrl_SetItemData(self.as_ptr(), item, data) }
    }
    fn set_item_font<F: FontMethods>(&self, item: c_long, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxListCtrl_SetItemFont(self.as_ptr(), item, font)
        }
    }
    fn set_item_image(&self, item: c_long, image: c_int, sel_image: c_int) -> bool {
        unsafe { ffi::wxListCtrl_SetItemImage(self.as_ptr(), item, image, sel_image) }
    }
    fn set_item_position<P: PointMethods>(&self, item: c_long, pos: &P) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxListCtrl_SetItemPosition(self.as_ptr(), item, pos)
        }
    }
    // NOT_SUPPORTED: fn SetItemPtrData()
    fn set_item_state(&self, item: c_long, state: c_long, state_mask: c_long) -> bool {
        unsafe { ffi::wxListCtrl_SetItemState(self.as_ptr(), item, state, state_mask) }
    }
    fn set_item_text(&self, item: c_long, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxListCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
    fn set_item_text_colour<C: ColourMethods>(&self, item: c_long, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetItemTextColour(self.as_ptr(), item, col)
        }
    }
    fn set_single_style(&self, style: c_long, add: bool) {
        unsafe { ffi::wxListCtrl_SetSingleStyle(self.as_ptr(), style, add) }
    }
    fn set_text_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxListCtrl_SetTextColour(self.as_ptr(), col)
        }
    }
    // NOT_SUPPORTED: fn SortItems()
    fn has_check_boxes(&self) -> bool {
        unsafe { ffi::wxListCtrl_HasCheckBoxes(self.as_ptr()) }
    }
    fn enable_check_boxes(&self, enable: bool) -> bool {
        unsafe { ffi::wxListCtrl_EnableCheckBoxes(self.as_ptr(), enable) }
    }
    fn is_item_checked(&self, item: c_long) -> bool {
        unsafe { ffi::wxListCtrl_IsItemChecked(self.as_ptr(), item) }
    }
    fn check_item(&self, item: c_long, check: bool) {
        unsafe { ffi::wxListCtrl_CheckItem(self.as_ptr(), item, check) }
    }
    fn extend_rules_and_alternate_colour(&self, extend: bool) {
        unsafe { ffi::wxListCtrl_ExtendRulesAndAlternateColour(self.as_ptr(), extend) }
    }
    fn show_sort_indicator(&self, col: c_int, ascending: bool) {
        unsafe { ffi::wxListCtrl_ShowSortIndicator(self.as_ptr(), col, ascending) }
    }
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxListCtrl_RemoveSortIndicator(self.as_ptr()) }
    }
    fn get_sort_indicator(&self) -> c_int {
        unsafe { ffi::wxListCtrl_GetSortIndicator(self.as_ptr()) }
    }
    fn get_updated_ascending_sort_indicator(&self, col: c_int) -> bool {
        unsafe { ffi::wxListCtrl_GetUpdatedAscendingSortIndicator(self.as_ptr(), col) }
    }
    fn is_ascending_sort_indicator(&self) -> bool {
        unsafe { ffi::wxListCtrl_IsAscendingSortIndicator(self.as_ptr()) }
    }
}

// wxListEvent
pub trait ListEventMethods: NotifyEventMethods {
    fn get_cache_from(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetCacheFrom(self.as_ptr()) }
    }
    fn get_cache_to(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetCacheTo(self.as_ptr()) }
    }
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetColumn(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetData()
    fn get_image(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetImage(self.as_ptr()) }
    }
    fn get_index(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetIndex(self.as_ptr()) }
    }
    fn get_item(&self) -> ListItemIsOwned<false> {
        unsafe { ListItemIsOwned::from_ptr(ffi::wxListEvent_GetItem(self.as_ptr())) }
    }
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxListEvent_GetKeyCode(self.as_ptr()) }
    }
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListEvent_GetLabel(self.as_ptr())).into() }
    }
    fn get_mask(&self) -> c_long {
        unsafe { ffi::wxListEvent_GetMask(self.as_ptr()) }
    }
    fn get_point(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxListEvent_GetPoint(self.as_ptr())) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListEvent_GetText(self.as_ptr())).into() }
    }
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxListEvent_IsEditCancelled(self.as_ptr()) }
    }
    fn set_key_code(&self, code: c_int) {
        unsafe { ffi::wxListEvent_SetKeyCode(self.as_ptr(), code) }
    }
    fn set_index(&self, index: c_long) {
        unsafe { ffi::wxListEvent_SetIndex(self.as_ptr(), index) }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxListEvent_SetColumn(self.as_ptr(), col) }
    }
    fn set_point<P: PointMethods>(&self, point: &P) {
        unsafe {
            let point = point.as_ptr();
            ffi::wxListEvent_SetPoint(self.as_ptr(), point)
        }
    }
    fn set_item<L: ListItemMethods>(&self, item: &L) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxListEvent_SetItem(self.as_ptr(), item)
        }
    }
    fn set_cache_from(&self, cache_from: c_long) {
        unsafe { ffi::wxListEvent_SetCacheFrom(self.as_ptr(), cache_from) }
    }
    fn set_cache_to(&self, cache_to: c_long) {
        unsafe { ffi::wxListEvent_SetCacheTo(self.as_ptr(), cache_to) }
    }
}

// wxListItem
pub trait ListItemMethods: ObjectMethods {
    fn clear(&self) {
        unsafe { ffi::wxListItem_Clear(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAlign()
    fn get_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListItem_GetBackgroundColour(self.as_ptr())) }
    }
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxListItem_GetColumn(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetData()
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxListItem_GetFont(self.as_ptr())) }
    }
    fn get_id(&self) -> c_long {
        unsafe { ffi::wxListItem_GetId(self.as_ptr()) }
    }
    fn get_image(&self) -> c_int {
        unsafe { ffi::wxListItem_GetImage(self.as_ptr()) }
    }
    fn get_mask(&self) -> c_long {
        unsafe { ffi::wxListItem_GetMask(self.as_ptr()) }
    }
    fn get_state(&self) -> c_long {
        unsafe { ffi::wxListItem_GetState(self.as_ptr()) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxListItem_GetText(self.as_ptr())).into() }
    }
    fn get_text_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxListItem_GetTextColour(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxListItem_GetWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAlign()
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxListItem_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxListItem_SetColumn(self.as_ptr(), col) }
    }
    fn set_data_long(&self, data: c_long) {
        unsafe { ffi::wxListItem_SetData(self.as_ptr(), data) }
    }
    fn set_data_void(&self, data: *mut c_void) {
        unsafe { ffi::wxListItem_SetData1(self.as_ptr(), data) }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxListItem_SetFont(self.as_ptr(), font)
        }
    }
    fn set_id(&self, id: c_long) {
        unsafe { ffi::wxListItem_SetId(self.as_ptr(), id) }
    }
    fn set_image(&self, image: c_int) {
        unsafe { ffi::wxListItem_SetImage(self.as_ptr(), image) }
    }
    fn set_mask(&self, mask: c_long) {
        unsafe { ffi::wxListItem_SetMask(self.as_ptr(), mask) }
    }
    fn set_state(&self, state: c_long) {
        unsafe { ffi::wxListItem_SetState(self.as_ptr(), state) }
    }
    fn set_state_mask(&self, state_mask: c_long) {
        unsafe { ffi::wxListItem_SetStateMask(self.as_ptr(), state_mask) }
    }
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxListItem_SetText(self.as_ptr(), text)
        }
    }
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxListItem_SetTextColour(self.as_ptr(), col_text)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxListItem_SetWidth(self.as_ptr(), width) }
    }
}

// wxListView
pub trait ListViewMethods: ListCtrlMethods {
    // DTOR: fn ~wxListView()
    fn clear_column_image(&self, col: c_int) {
        unsafe { ffi::wxListView_ClearColumnImage(self.as_ptr(), col) }
    }
    fn focus(&self, index: c_long) {
        unsafe { ffi::wxListView_Focus(self.as_ptr(), index) }
    }
    fn get_first_selected(&self) -> c_long {
        unsafe { ffi::wxListView_GetFirstSelected(self.as_ptr()) }
    }
    fn get_focused_item(&self) -> c_long {
        unsafe { ffi::wxListView_GetFocusedItem(self.as_ptr()) }
    }
    fn get_next_selected(&self, item: c_long) -> c_long {
        unsafe { ffi::wxListView_GetNextSelected(self.as_ptr(), item) }
    }
    fn is_selected(&self, index: c_long) -> bool {
        unsafe { ffi::wxListView_IsSelected(self.as_ptr(), index) }
    }
    fn select(&self, n: c_long, on: bool) {
        unsafe { ffi::wxListView_Select(self.as_ptr(), n, on) }
    }
    fn set_column_image(&self, col: c_int, image: c_int) {
        unsafe { ffi::wxListView_SetColumnImage(self.as_ptr(), col, image) }
    }
}

// wxListbook
pub trait ListbookMethods: BookCtrlBaseMethods {
    fn get_list_view(&self) -> WeakRef<ListView> {
        unsafe { WeakRef::<ListView>::from(ffi::wxListbook_GetListView(self.as_ptr())) }
    }
}

// wxLogGui
pub trait LogGuiMethods: LogMethods {}

// wxLogTextCtrl
pub trait LogTextCtrlMethods: LogMethods {}

// wxLogWindow
pub trait LogWindowMethods: LogInterposerMethods {
    fn get_frame(&self) -> WeakRef<Frame> {
        unsafe { WeakRef::<Frame>::from(ffi::wxLogWindow_GetFrame(self.as_ptr())) }
    }
    fn on_frame_close<F: FrameMethods>(&self, frame: Option<&F>) -> bool {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLogWindow_OnFrameClose(self.as_ptr(), frame)
        }
    }
    fn on_frame_delete<F: FrameMethods>(&self, frame: Option<&F>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLogWindow_OnFrameDelete(self.as_ptr(), frame)
        }
    }
    fn show(&self, show: bool) {
        unsafe { ffi::wxLogWindow_Show(self.as_ptr(), show) }
    }
}

// wxLongPressEvent
pub trait LongPressEventMethods: GestureEventMethods {}

// wxMDIChildFrame
pub trait MDIChildFrameMethods: FrameMethods {
    // DTOR: fn ~wxMDIChildFrame()
    fn activate(&self) {
        unsafe { ffi::wxMDIChildFrame_Activate(self.as_ptr()) }
    }
    fn create_mdiparentframe<M: MDIParentFrameMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&M>,
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
            ffi::wxMDIChildFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    fn get_mdi_parent(&self) -> WeakRef<MDIParentFrame> {
        unsafe { WeakRef::<MDIParentFrame>::from(ffi::wxMDIChildFrame_GetMDIParent(self.as_ptr())) }
    }
}

// wxMDIClientWindow
pub trait MDIClientWindowMethods: WindowMethods {
    fn create_client<M: MDIParentFrameMethods>(&self, parent: Option<&M>, style: c_long) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMDIClientWindow_CreateClient(self.as_ptr(), parent, style)
        }
    }
}

// wxMDIParentFrame
pub trait MDIParentFrameMethods: FrameMethods {
    // DTOR: fn ~wxMDIParentFrame()
    fn activate_next(&self) {
        unsafe { ffi::wxMDIParentFrame_ActivateNext(self.as_ptr()) }
    }
    fn activate_previous(&self) {
        unsafe { ffi::wxMDIParentFrame_ActivatePrevious(self.as_ptr()) }
    }
    fn arrange_icons(&self) {
        unsafe { ffi::wxMDIParentFrame_ArrangeIcons(self.as_ptr()) }
    }
    fn cascade(&self) {
        unsafe { ffi::wxMDIParentFrame_Cascade(self.as_ptr()) }
    }
    fn get_active_child(&self) -> WeakRef<MDIChildFrame> {
        unsafe {
            WeakRef::<MDIChildFrame>::from(ffi::wxMDIParentFrame_GetActiveChild(self.as_ptr()))
        }
    }
    fn get_client_window(&self) -> *mut c_void {
        unsafe { ffi::wxMDIParentFrame_GetClientWindow(self.as_ptr()) }
    }
    fn get_window_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMDIParentFrame_GetWindowMenu(self.as_ptr())) }
    }
    fn on_create_client(&self) -> WeakRef<MDIClientWindow> {
        unsafe {
            WeakRef::<MDIClientWindow>::from(ffi::wxMDIParentFrame_OnCreateClient(self.as_ptr()))
        }
    }
    fn set_window_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMDIParentFrame_SetWindowMenu(self.as_ptr(), menu)
        }
    }
    // NOT_SUPPORTED: fn Tile()
    fn is_tdi() -> bool {
        unsafe { ffi::wxMDIParentFrame_IsTDI() }
    }
}

// wxMask
pub trait MaskMethods: ObjectMethods {
    // DTOR: fn ~wxMask()
    fn create_int<B: BitmapMethods>(&self, bitmap: &B, index: c_int) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMask_Create(self.as_ptr(), bitmap, index)
        }
    }
    fn create<B: BitmapMethods>(&self, bitmap: &B) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMask_Create1(self.as_ptr(), bitmap)
        }
    }
    fn create_colour<B: BitmapMethods, C: ColourMethods>(&self, bitmap: &B, colour: &C) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxMask_Create2(self.as_ptr(), bitmap, colour)
        }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMask_GetBitmap(self.as_ptr())) }
    }
}

// wxMaximizeEvent
pub trait MaximizeEventMethods: EventMethods {}

// wxMemoryDC
pub trait MemoryDCMethods: DCMethods {
    fn select_object<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMemoryDC_SelectObject(self.as_ptr(), bitmap)
        }
    }
    fn select_object_as_source<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxMemoryDC_SelectObjectAsSource(self.as_ptr(), bitmap)
        }
    }
    fn get_selected_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxMemoryDC_GetSelectedBitmap(self.as_ptr())) }
    }
    fn get_selected_bitmap(&self) -> BitmapIsOwned<false> {
        unsafe { BitmapIsOwned::from_ptr(ffi::wxMemoryDC_GetSelectedBitmap1(self.as_ptr())) }
    }
}

// wxMenu
pub trait MenuMethods: EvtHandlerMethods {
    // DTOR: fn ~wxMenu()
    fn append_int_str(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Append(
                self.as_ptr(),
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    fn append_int_menu<M: MenuMethods>(
        &self,
        id: c_int,
        item: &str,
        sub_menu: Option<&M>,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let sub_menu = match sub_menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Append1(
                self.as_ptr(),
                id,
                item,
                sub_menu,
                help_string,
            ))
        }
    }
    fn append_menuitem<M: MenuItemMethods>(
        &self,
        menu_item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu_item = match menu_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Append2(self.as_ptr(), menu_item))
        }
    }
    fn append_check_item(
        &self,
        id: c_int,
        item: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_AppendCheckItem(self.as_ptr(), id, item, help))
        }
    }
    fn append_radio_item(
        &self,
        id: c_int,
        item: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_AppendRadioItem(self.as_ptr(), id, item, help))
        }
    }
    fn append_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_AppendSeparator(self.as_ptr())) }
    }
    fn append_sub_menu<M: MenuMethods>(
        &self,
        submenu: Option<&M>,
        text: &str,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let text = WxString::from(text);
            let text = text.as_ptr();
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_AppendSubMenu(
                self.as_ptr(),
                submenu,
                text,
                help,
            ))
        }
    }
    fn break_(&self) {
        unsafe { ffi::wxMenu_Break(self.as_ptr()) }
    }
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenu_Check(self.as_ptr(), id, check) }
    }
    fn delete_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Delete(self.as_ptr(), id) }
    }
    fn delete_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Delete1(self.as_ptr(), item)
        }
    }
    fn destroy_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_Destroy(self.as_ptr(), id) }
    }
    fn destroy_menuitem<M: MenuItemMethods>(&self, item: Option<&M>) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Destroy1(self.as_ptr(), item)
        }
    }
    fn enable(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenu_Enable(self.as_ptr(), id, enable) }
    }
    fn find_child_item(&self, id: c_int, pos: *mut c_void) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindChildItem(self.as_ptr(), id, pos)) }
    }
    fn find_item_str(&self, item_string: &str) -> c_int {
        unsafe {
            let item_string = WxString::from(item_string);
            let item_string = item_string.as_ptr();
            ffi::wxMenu_FindItem(self.as_ptr(), item_string)
        }
    }
    fn find_item_int<M: MenuMethods>(
        &self,
        id: c_int,
        menu: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_FindItem1(self.as_ptr(), id, menu))
        }
    }
    fn find_item_by_position(&self, position: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_FindItemByPosition(self.as_ptr(), position)) }
    }
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetHelpString(self.as_ptr(), id)).into() }
    }
    fn get_label(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetLabel(self.as_ptr(), id)).into() }
    }
    fn get_label_text(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetLabelText(self.as_ptr(), id)).into() }
    }
    fn get_menu_item_count(&self) -> usize {
        unsafe { ffi::wxMenu_GetMenuItemCount(self.as_ptr()) }
    }
    // BLOCKED: fn GetMenuItems()
    // BLOCKED: fn GetMenuItems1()
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenu_GetTitle(self.as_ptr())).into() }
    }
    fn insert_menuitem<M: MenuItemMethods>(
        &self,
        pos: usize,
        menu_item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu_item = match menu_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Insert(self.as_ptr(), pos, menu_item))
        }
    }
    fn insert_int_str(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Insert1(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    fn insert_int_menu<M: MenuMethods>(
        &self,
        pos: usize,
        id: c_int,
        text: &str,
        submenu: Option<&M>,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Insert2(
                self.as_ptr(),
                pos,
                id,
                text,
                submenu,
                help,
            ))
        }
    }
    fn insert_check_item(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_InsertCheckItem(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
            ))
        }
    }
    fn insert_radio_item(
        &self,
        pos: usize,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_InsertRadioItem(
                self.as_ptr(),
                pos,
                id,
                item,
                help_string,
            ))
        }
    }
    fn insert_separator(&self, pos: usize) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_InsertSeparator(self.as_ptr(), pos)) }
    }
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsChecked(self.as_ptr(), id) }
    }
    fn is_enabled(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenu_IsEnabled(self.as_ptr(), id) }
    }
    // NOT_SUPPORTED: fn MSWCommand()
    fn prepend_menuitem<M: MenuItemMethods>(
        &self,
        item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Prepend(self.as_ptr(), item))
        }
    }
    fn prepend_int_str(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
        kind: c_int,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Prepend1(
                self.as_ptr(),
                id,
                item,
                help_string,
                kind,
            ))
        }
    }
    fn prepend_int_menu<M: MenuMethods>(
        &self,
        id: c_int,
        text: &str,
        submenu: Option<&M>,
        help: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let submenu = match submenu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let help = WxString::from(help);
            let help = help.as_ptr();
            MenuItem::option_from(ffi::wxMenu_Prepend2(self.as_ptr(), id, text, submenu, help))
        }
    }
    fn prepend_check_item(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_PrependCheckItem(
                self.as_ptr(),
                id,
                item,
                help_string,
            ))
        }
    }
    fn prepend_radio_item(
        &self,
        id: c_int,
        item: &str,
        help_string: &str,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            MenuItem::option_from(ffi::wxMenu_PrependRadioItem(
                self.as_ptr(),
                id,
                item,
                help_string,
            ))
        }
    }
    fn prepend_separator(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_PrependSeparator(self.as_ptr())) }
    }
    fn remove_int(&self, id: c_int) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxMenu_Remove(self.as_ptr(), id)) }
    }
    fn remove_menuitem<M: MenuItemMethods>(
        &self,
        item: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenu_Remove1(self.as_ptr(), item))
        }
    }
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenu_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    fn set_label(&self, id: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenu_SetLabel(self.as_ptr(), id, label)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenu_SetTitle(self.as_ptr(), title)
        }
    }
    fn update_ui<E: EvtHandlerMethods>(&self, source: Option<&E>) {
        unsafe {
            let source = match source {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_UpdateUI(self.as_ptr(), source)
        }
    }
    fn set_invoking_window<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetInvokingWindow(self.as_ptr(), win)
        }
    }
    fn get_invoking_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetInvokingWindow(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxMenu_GetWindow(self.as_ptr())) }
    }
    fn get_style(&self) -> c_long {
        unsafe { ffi::wxMenu_GetStyle(self.as_ptr()) }
    }
    fn set_parent<M: MenuMethods>(&self, parent: Option<&M>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_SetParent(self.as_ptr(), parent)
        }
    }
    fn get_parent(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenu_GetParent(self.as_ptr())) }
    }
    fn attach<M: MenuBarMethods>(&self, menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenu_Attach(self.as_ptr(), menubar)
        }
    }
    fn detach(&self) {
        unsafe { ffi::wxMenu_Detach(self.as_ptr()) }
    }
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenu_IsAttached(self.as_ptr()) }
    }
}

// wxMenuBar
pub trait MenuBarMethods: WindowMethods {
    // DTOR: fn ~wxMenuBar()
    fn append<M: MenuMethods>(&self, menu: Option<&M>, title: &str) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_Append(self.as_ptr(), menu, title)
        }
    }
    fn check(&self, id: c_int, check: bool) {
        unsafe { ffi::wxMenuBar_Check(self.as_ptr(), id, check) }
    }
    fn enable_int(&self, id: c_int, enable: bool) {
        unsafe { ffi::wxMenuBar_Enable(self.as_ptr(), id, enable) }
    }
    fn is_enabled_top(&self, pos: usize) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabledTop(self.as_ptr(), pos) }
    }
    fn enable_top(&self, pos: usize, enable: bool) {
        unsafe { ffi::wxMenuBar_EnableTop(self.as_ptr(), pos, enable) }
    }
    fn find_item<M: MenuMethods>(
        &self,
        id: c_int,
        menu: Option<&M>,
    ) -> Option<MenuItemIsOwned<false>> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MenuItem::option_from(ffi::wxMenuBar_FindItem(self.as_ptr(), id, menu))
        }
    }
    fn find_menu(&self, title: &str) -> c_int {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_FindMenu(self.as_ptr(), title)
        }
    }
    fn find_menu_item(&self, menu_string: &str, item_string: &str) -> c_int {
        unsafe {
            let menu_string = WxString::from(menu_string);
            let menu_string = menu_string.as_ptr();
            let item_string = WxString::from(item_string);
            let item_string = item_string.as_ptr();
            ffi::wxMenuBar_FindMenuItem(self.as_ptr(), menu_string, item_string)
        }
    }
    fn get_help_string(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetHelpString(self.as_ptr(), id)).into() }
    }
    fn get_label_int(&self, id: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetLabel(self.as_ptr(), id)).into() }
    }
    // BLOCKED: fn GetLabelTop()
    fn get_menu(&self, menu_index: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_GetMenu(self.as_ptr(), menu_index)) }
    }
    fn get_menu_count(&self) -> usize {
        unsafe { ffi::wxMenuBar_GetMenuCount(self.as_ptr()) }
    }
    fn get_menu_label(&self, pos: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetMenuLabel(self.as_ptr(), pos)).into() }
    }
    fn get_menu_label_text(&self, pos: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuBar_GetMenuLabelText(self.as_ptr(), pos)).into() }
    }
    fn insert<M: MenuMethods>(&self, pos: usize, menu: Option<&M>, title: &str) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxMenuBar_Insert(self.as_ptr(), pos, menu, title)
        }
    }
    fn is_checked(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsChecked(self.as_ptr(), id) }
    }
    fn is_enabled_int(&self, id: c_int) -> bool {
        unsafe { ffi::wxMenuBar_IsEnabled(self.as_ptr(), id) }
    }
    fn remove(&self, pos: usize) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_Remove(self.as_ptr(), pos)) }
    }
    fn replace<M: MenuMethods>(&self, pos: usize, menu: Option<&M>, title: &str) -> WeakRef<Menu> {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            WeakRef::<Menu>::from(ffi::wxMenuBar_Replace(self.as_ptr(), pos, menu, title))
        }
    }
    fn set_help_string(&self, id: c_int, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenuBar_SetHelpString(self.as_ptr(), id, help_string)
        }
    }
    fn set_label_int(&self, id: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuBar_SetLabel(self.as_ptr(), id, label)
        }
    }
    // BLOCKED: fn SetLabelTop()
    fn set_menu_label(&self, pos: usize, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuBar_SetMenuLabel(self.as_ptr(), pos, label)
        }
    }
    fn osx_get_apple_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuBar_OSXGetAppleMenu(self.as_ptr())) }
    }
    fn get_frame(&self) -> WeakRef<Frame> {
        unsafe { WeakRef::<Frame>::from(ffi::wxMenuBar_GetFrame(self.as_ptr())) }
    }
    fn is_attached(&self) -> bool {
        unsafe { ffi::wxMenuBar_IsAttached(self.as_ptr()) }
    }
    fn attach<F: FrameMethods>(&self, frame: Option<&F>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_Attach(self.as_ptr(), frame)
        }
    }
    fn detach(&self) {
        unsafe { ffi::wxMenuBar_Detach(self.as_ptr()) }
    }
    fn mac_set_common_menu_bar<M: MenuBarMethods>(menubar: Option<&M>) {
        unsafe {
            let menubar = match menubar {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuBar_MacSetCommonMenuBar(menubar)
        }
    }
    fn mac_get_common_menu_bar() -> WeakRef<MenuBar> {
        unsafe { WeakRef::<MenuBar>::from(ffi::wxMenuBar_MacGetCommonMenuBar()) }
    }
}

// wxMenuEvent
pub trait MenuEventMethods: EventMethods {
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuEvent_GetMenu(self.as_ptr())) }
    }
    fn get_menu_id(&self) -> c_int {
        unsafe { ffi::wxMenuEvent_GetMenuId(self.as_ptr()) }
    }
    fn is_popup(&self) -> bool {
        unsafe { ffi::wxMenuEvent_IsPopup(self.as_ptr()) }
    }
}

// wxMenuItem
pub trait MenuItemMethods: ObjectMethods {
    // BLOCKED: fn GetBackgroundColour()
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_bool(&self, checked: bool) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetBitmap1(self.as_ptr(), checked)) }
    }
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxMenuItem_GetBitmapBundle(self.as_ptr())) }
    }
    fn get_disabled_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxMenuItem_GetDisabledBitmap(self.as_ptr())) }
    }
    // BLOCKED: fn GetFont()
    fn get_help(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetHelp(self.as_ptr())).into() }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetId(self.as_ptr()) }
    }
    fn get_item_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetItemLabel(self.as_ptr())).into() }
    }
    fn get_item_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxMenuItem_GetItemLabelText(self.as_ptr())).into() }
    }
    fn get_kind(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetKind(self.as_ptr()) }
    }
    // BLOCKED: fn GetLabel()
    fn get_margin_width(&self) -> c_int {
        unsafe { ffi::wxMenuItem_GetMarginWidth(self.as_ptr()) }
    }
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetName()
    fn get_sub_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxMenuItem_GetSubMenu(self.as_ptr())) }
    }
    // BLOCKED: fn GetText()
    // BLOCKED: fn GetTextColour()
    fn get_accel(&self) -> Option<AcceleratorEntryIsOwned<false>> {
        unsafe { AcceleratorEntry::option_from(ffi::wxMenuItem_GetAccel(self.as_ptr())) }
    }
    // BLOCKED: fn GetAccelFromString()
    fn is_check(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheck(self.as_ptr()) }
    }
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsCheckable(self.as_ptr()) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsChecked(self.as_ptr()) }
    }
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsEnabled(self.as_ptr()) }
    }
    fn is_radio(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsRadio(self.as_ptr()) }
    }
    fn is_separator(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSeparator(self.as_ptr()) }
    }
    fn is_sub_menu(&self) -> bool {
        unsafe { ffi::wxMenuItem_IsSubMenu(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxMenuItem_SetBitmap(self.as_ptr(), bmp)
        }
    }
    fn set_bitmap_bool<B: BitmapBundleMethods>(&self, bmp: &B, checked: bool) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxMenuItem_SetBitmap1(self.as_ptr(), bmp, checked)
        }
    }
    fn set_bitmaps<B: BitmapBundleMethods, B2: BitmapBundleMethods>(
        &self,
        checked: &B,
        unchecked: &B2,
    ) {
        unsafe {
            let checked = checked.as_ptr();
            let unchecked = unchecked.as_ptr();
            ffi::wxMenuItem_SetBitmaps(self.as_ptr(), checked, unchecked)
        }
    }
    fn set_disabled_bitmap<B: BitmapBundleMethods>(&self, disabled: &B) {
        unsafe {
            let disabled = disabled.as_ptr();
            ffi::wxMenuItem_SetDisabledBitmap(self.as_ptr(), disabled)
        }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxMenuItem_SetFont(self.as_ptr(), font)
        }
    }
    fn set_help(&self, help_string: &str) {
        unsafe {
            let help_string = WxString::from(help_string);
            let help_string = help_string.as_ptr();
            ffi::wxMenuItem_SetHelp(self.as_ptr(), help_string)
        }
    }
    fn set_item_label(&self, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxMenuItem_SetItemLabel(self.as_ptr(), label)
        }
    }
    fn set_margin_width(&self, width: c_int) {
        unsafe { ffi::wxMenuItem_SetMarginWidth(self.as_ptr(), width) }
    }
    fn set_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetMenu(self.as_ptr(), menu)
        }
    }
    fn set_sub_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetSubMenu(self.as_ptr(), menu)
        }
    }
    // BLOCKED: fn SetText()
    fn set_text_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxMenuItem_SetTextColour(self.as_ptr(), colour)
        }
    }
    fn set_accel<A: AcceleratorEntryMethods>(&self, accel: Option<&A>) {
        unsafe {
            let accel = match accel {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMenuItem_SetAccel(self.as_ptr(), accel)
        }
    }
    fn add_extra_accel<A: AcceleratorEntryMethods>(&self, accel: &A) {
        unsafe {
            let accel = accel.as_ptr();
            ffi::wxMenuItem_AddExtraAccel(self.as_ptr(), accel)
        }
    }
    fn clear_extra_accels(&self) {
        unsafe { ffi::wxMenuItem_ClearExtraAccels(self.as_ptr()) }
    }
    // DTOR: fn ~wxMenuItem()
    fn check(&self, check: bool) {
        unsafe { ffi::wxMenuItem_Check(self.as_ptr(), check) }
    }
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxMenuItem_Enable(self.as_ptr(), enable) }
    }
    // BLOCKED: fn GetLabelFromText()
    fn get_label_text(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxMenuItem_GetLabelText(text)).into()
        }
    }
}

// wxMessageOutputMessageBox
pub trait MessageOutputMessageBoxMethods: MessageOutputMethods {}

// wxMetafile
pub trait MetafileMethods: ObjectMethods {
    // DTOR: fn ~wxMetafile()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxMetafile_IsOk(self.as_ptr()) }
    }
    fn play<D: DCMethods>(&self, dc: Option<&D>) -> bool {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxMetafile_Play(self.as_ptr(), dc)
        }
    }
    fn set_clipboard(&self, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxMetafile_SetClipboard(self.as_ptr(), width, height) }
    }
}

// wxMiniFrame
pub trait MiniFrameMethods: FrameMethods {
    // DTOR: fn ~wxMiniFrame()
}

// wxMirrorDC
pub trait MirrorDCMethods: DCMethods {}

// wxMouseCaptureChangedEvent
pub trait MouseCaptureChangedEventMethods: EventMethods {
    fn get_captured_window(&self) -> WeakRef<Window> {
        unsafe {
            WeakRef::<Window>::from(ffi::wxMouseCaptureChangedEvent_GetCapturedWindow(
                self.as_ptr(),
            ))
        }
    }
}

// wxMouseCaptureLostEvent
pub trait MouseCaptureLostEventMethods: EventMethods {}

// wxMouseEvent
pub trait MouseEventMethods: EventMethods {
    fn aux1_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1DClick(self.as_ptr()) }
    }
    fn aux1_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1Down(self.as_ptr()) }
    }
    fn aux1_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux1Up(self.as_ptr()) }
    }
    fn aux2_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2DClick(self.as_ptr()) }
    }
    fn aux2_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2Down(self.as_ptr()) }
    }
    fn aux2_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Aux2Up(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Button()
    // NOT_SUPPORTED: fn ButtonDClick()
    // NOT_SUPPORTED: fn ButtonDown()
    // NOT_SUPPORTED: fn ButtonUp()
    fn dragging(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Dragging(self.as_ptr()) }
    }
    fn entering(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Entering(self.as_ptr()) }
    }
    fn get_button(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetButton(self.as_ptr()) }
    }
    fn get_click_count(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetClickCount(self.as_ptr()) }
    }
    fn get_lines_per_action(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetLinesPerAction(self.as_ptr()) }
    }
    fn get_columns_per_action(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetColumnsPerAction(self.as_ptr()) }
    }
    fn get_logical_position<D: DCMethods>(&self, dc: &D) -> Point {
        unsafe {
            let dc = dc.as_ptr();
            Point::from_ptr(ffi::wxMouseEvent_GetLogicalPosition(self.as_ptr(), dc))
        }
    }
    // NOT_SUPPORTED: fn GetMagnification()
    fn get_wheel_delta(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetWheelDelta(self.as_ptr()) }
    }
    fn is_wheel_inverted(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsWheelInverted(self.as_ptr()) }
    }
    fn get_wheel_rotation(&self) -> c_int {
        unsafe { ffi::wxMouseEvent_GetWheelRotation(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWheelAxis()
    fn is_button(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsButton(self.as_ptr()) }
    }
    fn is_page_scroll(&self) -> bool {
        unsafe { ffi::wxMouseEvent_IsPageScroll(self.as_ptr()) }
    }
    fn leaving(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Leaving(self.as_ptr()) }
    }
    fn left_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftDClick(self.as_ptr()) }
    }
    fn left_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftDown(self.as_ptr()) }
    }
    fn left_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_LeftUp(self.as_ptr()) }
    }
    fn magnify(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Magnify(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MetaDown(self.as_ptr()) }
    }
    fn middle_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleDClick(self.as_ptr()) }
    }
    fn middle_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleDown(self.as_ptr()) }
    }
    fn middle_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_MiddleUp(self.as_ptr()) }
    }
    fn moving(&self) -> bool {
        unsafe { ffi::wxMouseEvent_Moving(self.as_ptr()) }
    }
    fn right_d_click(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightDClick(self.as_ptr()) }
    }
    fn right_down(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightDown(self.as_ptr()) }
    }
    fn right_up(&self) -> bool {
        unsafe { ffi::wxMouseEvent_RightUp(self.as_ptr()) }
    }
}

// wxMoveEvent
pub trait MoveEventMethods: EventMethods {
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxMoveEvent_GetPosition(self.as_ptr())) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxMoveEvent_GetRect(self.as_ptr())) }
    }
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxMoveEvent_SetRect(self.as_ptr(), rect)
        }
    }
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxMoveEvent_SetPosition(self.as_ptr(), pos)
        }
    }
}

// wxNativeFontInfo
pub trait NativeFontInfoMethods: WxRustMethods {
    // DTOR: fn ~wxNativeFontInfo()
    // BLOCKED: fn operator=()
    fn init(&self) {
        unsafe { ffi::wxNativeFontInfo_Init(self.as_ptr()) }
    }
    fn init_from_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxNativeFontInfo_InitFromFont(self.as_ptr(), font)
        }
    }
    fn get_point_size(&self) -> c_int {
        unsafe { ffi::wxNativeFontInfo_GetPointSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFractionalPointSize()
    fn get_pixel_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxNativeFontInfo_GetPixelSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    fn get_numeric_weight(&self) -> c_int {
        unsafe { ffi::wxNativeFontInfo_GetNumericWeight(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetWeight()
    fn get_underlined(&self) -> bool {
        unsafe { ffi::wxNativeFontInfo_GetUnderlined(self.as_ptr()) }
    }
    fn get_face_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_GetFaceName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFamily()
    // NOT_SUPPORTED: fn GetEncoding()
    fn set_point_size(&self, pointsize: c_int) {
        unsafe { ffi::wxNativeFontInfo_SetPointSize(self.as_ptr(), pointsize) }
    }
    // NOT_SUPPORTED: fn SetFractionalPointSize()
    fn set_pixel_size<S: SizeMethods>(&self, pixel_size: &S) {
        unsafe {
            let pixel_size = pixel_size.as_ptr();
            ffi::wxNativeFontInfo_SetPixelSize(self.as_ptr(), pixel_size)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    fn set_numeric_weight(&self, weight: c_int) {
        unsafe { ffi::wxNativeFontInfo_SetNumericWeight(self.as_ptr(), weight) }
    }
    // NOT_SUPPORTED: fn SetWeight()
    fn set_underlined(&self, underlined: bool) {
        unsafe { ffi::wxNativeFontInfo_SetUnderlined(self.as_ptr(), underlined) }
    }
    fn set_face_name_str(&self, facename: &str) -> bool {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            ffi::wxNativeFontInfo_SetFaceName(self.as_ptr(), facename)
        }
    }
    // NOT_SUPPORTED: fn SetFamily()
    // NOT_SUPPORTED: fn SetEncoding()
    fn set_face_name_arraystring<A: ArrayStringMethods>(&self, facenames: &A) {
        unsafe {
            let facenames = facenames.as_ptr();
            ffi::wxNativeFontInfo_SetFaceName1(self.as_ptr(), facenames)
        }
    }
    fn from_string(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxNativeFontInfo_FromString(self.as_ptr(), s)
        }
    }
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_ToString(self.as_ptr())).into() }
    }
    fn from_user_string(&self, s: &str) -> bool {
        unsafe {
            let s = WxString::from(s);
            let s = s.as_ptr();
            ffi::wxNativeFontInfo_FromUserString(self.as_ptr(), s)
        }
    }
    fn to_user_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxNativeFontInfo_ToUserString(self.as_ptr())).into() }
    }
}

// wxNativeWindow
pub trait NativeWindowMethods: WindowMethods {
    // NOT_SUPPORTED: fn Create()
    fn disown(&self) {
        unsafe { ffi::wxNativeWindow_Disown(self.as_ptr()) }
    }
}

// wxNavigationKeyEvent
pub trait NavigationKeyEventMethods: EventMethods {
    fn get_current_focus(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxNavigationKeyEvent_GetCurrentFocus(self.as_ptr())) }
    }
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_GetDirection(self.as_ptr()) }
    }
    fn is_from_tab(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsFromTab(self.as_ptr()) }
    }
    fn is_window_change(&self) -> bool {
        unsafe { ffi::wxNavigationKeyEvent_IsWindowChange(self.as_ptr()) }
    }
    fn set_current_focus<W: WindowMethods>(&self, current_focus: Option<&W>) {
        unsafe {
            let current_focus = match current_focus {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxNavigationKeyEvent_SetCurrentFocus(self.as_ptr(), current_focus)
        }
    }
    fn set_direction(&self, direction: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetDirection(self.as_ptr(), direction) }
    }
    fn set_flags(&self, flags: c_long) {
        unsafe { ffi::wxNavigationKeyEvent_SetFlags(self.as_ptr(), flags) }
    }
    fn set_from_tab(&self, from_tab: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetFromTab(self.as_ptr(), from_tab) }
    }
    fn set_window_change(&self, window_change: bool) {
        unsafe { ffi::wxNavigationKeyEvent_SetWindowChange(self.as_ptr(), window_change) }
    }
}

// wxNonOwnedWindow
pub trait NonOwnedWindowMethods: WindowMethods {
    fn set_shape_region(&self, region: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape(self.as_ptr(), region) }
    }
    fn set_shape_graphicspath<G: GraphicsPathMethods>(&self, path: &G) -> bool {
        unsafe {
            let path = path.as_ptr();
            ffi::wxNonOwnedWindow_SetShape1(self.as_ptr(), path)
        }
    }
}

// wxNotebook
pub trait NotebookMethods: BookCtrlBaseMethods {
    // DTOR: fn ~wxNotebook()
    fn get_row_count(&self) -> c_int {
        unsafe { ffi::wxNotebook_GetRowCount(self.as_ptr()) }
    }
    fn get_theme_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxNotebook_GetThemeBackgroundColour(self.as_ptr())) }
    }
    // BLOCKED: fn OnSelChange()
    fn set_padding<S: SizeMethods>(&self, padding: &S) {
        unsafe {
            let padding = padding.as_ptr();
            ffi::wxNotebook_SetPadding(self.as_ptr(), padding)
        }
    }
}

// wxNotifyEvent
pub trait NotifyEventMethods: CommandEventMethods {
    fn allow(&self) {
        unsafe { ffi::wxNotifyEvent_Allow(self.as_ptr()) }
    }
    fn is_allowed(&self) -> bool {
        unsafe { ffi::wxNotifyEvent_IsAllowed(self.as_ptr()) }
    }
    fn veto(&self) {
        unsafe { ffi::wxNotifyEvent_Veto(self.as_ptr()) }
    }
}

// wxOverlay
pub trait OverlayMethods: WxRustMethods {
    // DTOR: fn ~wxOverlay()
    fn reset(&self) {
        unsafe { ffi::wxOverlay_Reset(self.as_ptr()) }
    }
}

// wxPageSetupDialog
pub trait PageSetupDialogMethods: ObjectMethods {
    // DTOR: fn ~wxPageSetupDialog()
    fn get_page_setup_data(&self) -> PageSetupDialogDataIsOwned<false> {
        unsafe {
            PageSetupDialogDataIsOwned::from_ptr(ffi::wxPageSetupDialog_GetPageSetupData(
                self.as_ptr(),
            ))
        }
    }
    fn show_modal(&self) -> c_int {
        unsafe { ffi::wxPageSetupDialog_ShowModal(self.as_ptr()) }
    }
}

// wxPageSetupDialogData
pub trait PageSetupDialogDataMethods: ObjectMethods {
    // DTOR: fn ~wxPageSetupDialogData()
    fn enable_help(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_EnableHelp(self.as_ptr(), flag) }
    }
    fn enable_margins(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_EnableMargins(self.as_ptr(), flag) }
    }
    fn enable_orientation(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_EnableOrientation(self.as_ptr(), flag) }
    }
    fn enable_paper(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_EnablePaper(self.as_ptr(), flag) }
    }
    fn enable_printer(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_EnablePrinter(self.as_ptr(), flag) }
    }
    fn get_default_info(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetDefaultInfo(self.as_ptr()) }
    }
    fn get_default_min_margins(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetDefaultMinMargins(self.as_ptr()) }
    }
    fn get_enable_help(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetEnableHelp(self.as_ptr()) }
    }
    fn get_enable_margins(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetEnableMargins(self.as_ptr()) }
    }
    fn get_enable_orientation(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetEnableOrientation(self.as_ptr()) }
    }
    fn get_enable_paper(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetEnablePaper(self.as_ptr()) }
    }
    fn get_enable_printer(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_GetEnablePrinter(self.as_ptr()) }
    }
    fn get_margin_bottom_right(&self) -> Point {
        unsafe {
            Point::from_ptr(ffi::wxPageSetupDialogData_GetMarginBottomRight(
                self.as_ptr(),
            ))
        }
    }
    fn get_margin_top_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxPageSetupDialogData_GetMarginTopLeft(self.as_ptr())) }
    }
    fn get_min_margin_bottom_right(&self) -> Point {
        unsafe {
            Point::from_ptr(ffi::wxPageSetupDialogData_GetMinMarginBottomRight(
                self.as_ptr(),
            ))
        }
    }
    fn get_min_margin_top_left(&self) -> Point {
        unsafe {
            Point::from_ptr(ffi::wxPageSetupDialogData_GetMinMarginTopLeft(
                self.as_ptr(),
            ))
        }
    }
    // NOT_SUPPORTED: fn GetPaperId()
    fn get_paper_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxPageSetupDialogData_GetPaperSize(self.as_ptr())) }
    }
    fn get_print_data(&self) -> PrintDataIsOwned<false> {
        unsafe {
            PrintDataIsOwned::from_ptr(ffi::wxPageSetupDialogData_GetPrintData(self.as_ptr()))
        }
    }
    fn get_print_data(&self) -> PrintDataIsOwned<false> {
        unsafe {
            PrintDataIsOwned::from_ptr(ffi::wxPageSetupDialogData_GetPrintData1(self.as_ptr()))
        }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPageSetupDialogData_IsOk(self.as_ptr()) }
    }
    fn set_default_info(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_SetDefaultInfo(self.as_ptr(), flag) }
    }
    fn set_default_min_margins(&self, flag: bool) {
        unsafe { ffi::wxPageSetupDialogData_SetDefaultMinMargins(self.as_ptr(), flag) }
    }
    fn set_margin_bottom_right<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPageSetupDialogData_SetMarginBottomRight(self.as_ptr(), pt)
        }
    }
    fn set_margin_top_left<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPageSetupDialogData_SetMarginTopLeft(self.as_ptr(), pt)
        }
    }
    fn set_min_margin_bottom_right<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPageSetupDialogData_SetMinMarginBottomRight(self.as_ptr(), pt)
        }
    }
    fn set_min_margin_top_left<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPageSetupDialogData_SetMinMarginTopLeft(self.as_ptr(), pt)
        }
    }
    // NOT_SUPPORTED: fn SetPaperId()
    fn set_paper_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxPageSetupDialogData_SetPaperSize(self.as_ptr(), size)
        }
    }
    fn set_print_data<P: PrintDataMethods>(&self, print_data: &P) {
        unsafe {
            let print_data = print_data.as_ptr();
            ffi::wxPageSetupDialogData_SetPrintData(self.as_ptr(), print_data)
        }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator=1()
}

// wxPaintDC
pub trait PaintDCMethods: ClientDCMethods {}

// wxPalette
pub trait PaletteMethods: GDIObjectMethods {
    // DTOR: fn ~wxPalette()
    fn create(
        &self,
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> bool {
        unsafe { ffi::wxPalette_Create(self.as_ptr(), n, red, green, blue) }
    }
    fn get_colours_count(&self) -> c_int {
        unsafe { ffi::wxPalette_GetColoursCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    fn get_rgb(
        &self,
        pixel: c_int,
        red: *mut c_void,
        green: *mut c_void,
        blue: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxPalette_GetRGB(self.as_ptr(), pixel, red, green, blue) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPalette_IsOk(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxPanGestureEvent
pub trait PanGestureEventMethods: GestureEventMethods {
    fn get_delta(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxPanGestureEvent_GetDelta(self.as_ptr())) }
    }
    fn set_delta<P: PointMethods>(&self, delta: &P) {
        unsafe {
            let delta = delta.as_ptr();
            ffi::wxPanGestureEvent_SetDelta(self.as_ptr(), delta)
        }
    }
}

// wxPanel
pub trait PanelMethods: WindowMethods {
    // DTOR: fn ~wxPanel()
    fn on_sys_colour_changed<S: SysColourChangedEventMethods>(&self, event: &S) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPanel_OnSysColourChanged(self.as_ptr(), event)
        }
    }
    fn set_focus_ignoring_children(&self) {
        unsafe { ffi::wxPanel_SetFocusIgnoringChildren(self.as_ptr()) }
    }
}

// wxPen
pub trait PenMethods: GDIObjectMethods {
    // DTOR: fn ~wxPen()
    // NOT_SUPPORTED: fn GetCap()
    // NOT_SUPPORTED: fn GetQuality()
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxPen_GetColour(self.as_ptr())) }
    }
    fn get_dashes(&self, dashes: *mut c_void) -> c_int {
        unsafe { ffi::wxPen_GetDashes(self.as_ptr(), dashes) }
    }
    // NOT_SUPPORTED: fn GetJoin()
    fn get_stipple(&self) -> Option<BitmapIsOwned<false>> {
        unsafe { Bitmap::option_from(ffi::wxPen_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxPen_GetWidth(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPen_IsOk(self.as_ptr()) }
    }
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsNonTransparent(self.as_ptr()) }
    }
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxPen_IsTransparent(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetCap()
    // NOT_SUPPORTED: fn SetQuality()
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxPen_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    fn set_dashes(&self, n: c_int, dash: *const c_void) {
        unsafe { ffi::wxPen_SetDashes(self.as_ptr(), n, dash) }
    }
    // NOT_SUPPORTED: fn SetJoin()
    fn set_stipple<B: BitmapMethods>(&self, stipple: &B) {
        unsafe {
            let stipple = stipple.as_ptr();
            ffi::wxPen_SetStipple(self.as_ptr(), stipple)
        }
    }
    // NOT_SUPPORTED: fn SetStyle()
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxPen_SetWidth(self.as_ptr(), width) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxPenList
pub trait PenListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreatePen()
}

// wxPickerBase
pub trait PickerBaseMethods: ControlMethods {
    // DTOR: fn ~wxPickerBase()
    fn create_base<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        text: &str,
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
            let text = WxString::from(text);
            let text = text.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxPickerBase_CreateBase(
                self.as_ptr(),
                parent,
                id,
                text,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_internal_margin(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetInternalMargin(self.as_ptr()) }
    }
    fn get_picker_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetPickerCtrlProportion(self.as_ptr()) }
    }
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxPickerBase_GetTextCtrl(self.as_ptr())) }
    }
    fn get_picker_ctrl(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxPickerBase_GetPickerCtrl(self.as_ptr())) }
    }
    fn get_text_ctrl_proportion(&self) -> c_int {
        unsafe { ffi::wxPickerBase_GetTextCtrlProportion(self.as_ptr()) }
    }
    fn has_text_ctrl(&self) -> bool {
        unsafe { ffi::wxPickerBase_HasTextCtrl(self.as_ptr()) }
    }
    fn is_picker_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsPickerCtrlGrowable(self.as_ptr()) }
    }
    fn is_text_ctrl_growable(&self) -> bool {
        unsafe { ffi::wxPickerBase_IsTextCtrlGrowable(self.as_ptr()) }
    }
    fn set_internal_margin(&self, margin: c_int) {
        unsafe { ffi::wxPickerBase_SetInternalMargin(self.as_ptr(), margin) }
    }
    fn set_picker_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlGrowable(self.as_ptr(), grow) }
    }
    fn set_picker_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetPickerCtrlProportion(self.as_ptr(), prop) }
    }
    fn set_text_ctrl_growable(&self, grow: bool) {
        unsafe { ffi::wxPickerBase_SetTextCtrlGrowable(self.as_ptr(), grow) }
    }
    fn set_text_ctrl_proportion(&self, prop: c_int) {
        unsafe { ffi::wxPickerBase_SetTextCtrlProportion(self.as_ptr(), prop) }
    }
    fn set_text_ctrl<T: TextCtrlMethods>(&self, text: Option<&T>) {
        unsafe {
            let text = match text {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPickerBase_SetTextCtrl(self.as_ptr(), text)
        }
    }
    fn set_picker_ctrl<C: ControlMethods>(&self, picker: Option<&C>) {
        unsafe {
            let picker = match picker {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPickerBase_SetPickerCtrl(self.as_ptr(), picker)
        }
    }
    fn update_picker_from_text_ctrl(&self) {
        unsafe { ffi::wxPickerBase_UpdatePickerFromTextCtrl(self.as_ptr()) }
    }
    fn update_text_ctrl_from_picker(&self) {
        unsafe { ffi::wxPickerBase_UpdateTextCtrlFromPicker(self.as_ptr()) }
    }
}

// wxPoint
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr()) }
    }
    fn set_defaults<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPoint_SetDefaults(self.as_ptr(), pt)
        }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+2()
    // BLOCKED: fn operator-2()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxPopupTransientWindow
pub trait PopupTransientWindowMethods: PopupWindowMethods {
    fn popup<W: WindowMethods>(&self, focus: Option<&W>) {
        unsafe {
            let focus = match focus {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPopupTransientWindow_Popup(self.as_ptr(), focus)
        }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxPopupTransientWindow_Dismiss(self.as_ptr()) }
    }
    fn process_left_down<M: MouseEventMethods>(&self, event: &M) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPopupTransientWindow_ProcessLeftDown(self.as_ptr(), event)
        }
    }
}

// wxPopupWindow
pub trait PopupWindowMethods: NonOwnedWindowMethods {
    fn create_int<W: WindowMethods>(&self, parent: Option<&W>, flags: c_int) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPopupWindow_Create(self.as_ptr(), parent, flags)
        }
    }
    fn position<P: PointMethods, S: SizeMethods>(&self, pt_origin: &P, size_popup: &S) {
        unsafe {
            let pt_origin = pt_origin.as_ptr();
            let size_popup = size_popup.as_ptr();
            ffi::wxPopupWindow_Position(self.as_ptr(), pt_origin, size_popup)
        }
    }
}

// wxPreferencesEditor
pub trait PreferencesEditorMethods: WxRustMethods {
    // DTOR: fn ~wxPreferencesEditor()
    fn add_page(&self, page: *mut c_void) {
        unsafe { ffi::wxPreferencesEditor_AddPage(self.as_ptr(), page) }
    }
    fn show<W: WindowMethods>(&self, parent: Option<&W>) {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPreferencesEditor_Show(self.as_ptr(), parent)
        }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxPreferencesEditor_Dismiss(self.as_ptr()) }
    }
    fn should_apply_changes_immediately() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShouldApplyChangesImmediately() }
    }
    fn shown_modally() -> bool {
        unsafe { ffi::wxPreferencesEditor_ShownModally() }
    }
}

// wxPressAndTapEvent
pub trait PressAndTapEventMethods: GestureEventMethods {}

// wxPreviewControlBar
pub trait PreviewControlBarMethods: PanelMethods {
    // DTOR: fn ~wxPreviewControlBar()
    fn create_buttons(&self) {
        unsafe { ffi::wxPreviewControlBar_CreateButtons(self.as_ptr()) }
    }
    fn get_print_preview(&self) -> *mut c_void {
        unsafe { ffi::wxPreviewControlBar_GetPrintPreview(self.as_ptr()) }
    }
    fn get_zoom_control(&self) -> c_int {
        unsafe { ffi::wxPreviewControlBar_GetZoomControl(self.as_ptr()) }
    }
    fn set_zoom_control(&self, percent: c_int) {
        unsafe { ffi::wxPreviewControlBar_SetZoomControl(self.as_ptr(), percent) }
    }
}

// wxPreviewFrame
pub trait PreviewFrameMethods: FrameMethods {
    // DTOR: fn ~wxPreviewFrame()
    fn create_canvas(&self) {
        unsafe { ffi::wxPreviewFrame_CreateCanvas(self.as_ptr()) }
    }
    fn create_control_bar(&self) {
        unsafe { ffi::wxPreviewFrame_CreateControlBar(self.as_ptr()) }
    }
    fn initialize(&self) {
        unsafe { ffi::wxPreviewFrame_Initialize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn InitializeWithModality()
    fn on_close_window<C: CloseEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxPreviewFrame_OnCloseWindow(self.as_ptr(), event)
        }
    }
}

// wxPrintData
pub trait PrintDataMethods: ObjectMethods {
    // DTOR: fn ~wxPrintData()
    // NOT_SUPPORTED: fn GetBin()
    fn get_collate(&self) -> bool {
        unsafe { ffi::wxPrintData_GetCollate(self.as_ptr()) }
    }
    fn get_colour(&self) -> bool {
        unsafe { ffi::wxPrintData_GetColour(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDuplex()
    fn get_no_copies(&self) -> c_int {
        unsafe { ffi::wxPrintData_GetNoCopies(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetOrientation()
    // NOT_SUPPORTED: fn GetPaperId()
    fn get_printer_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPrintData_GetPrinterName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetQuality()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPrintData_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetBin()
    fn set_collate(&self, flag: bool) {
        unsafe { ffi::wxPrintData_SetCollate(self.as_ptr(), flag) }
    }
    fn set_colour(&self, flag: bool) {
        unsafe { ffi::wxPrintData_SetColour(self.as_ptr(), flag) }
    }
    // NOT_SUPPORTED: fn SetDuplex()
    fn set_no_copies(&self, n: c_int) {
        unsafe { ffi::wxPrintData_SetNoCopies(self.as_ptr(), n) }
    }
    // NOT_SUPPORTED: fn SetOrientation()
    // NOT_SUPPORTED: fn SetPaperId()
    fn set_paper_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxPrintData_SetPaperSize(self.as_ptr(), size)
        }
    }
    fn set_printer_name(&self, printer_name: &str) {
        unsafe {
            let printer_name = WxString::from(printer_name);
            let printer_name = printer_name.as_ptr();
            ffi::wxPrintData_SetPrinterName(self.as_ptr(), printer_name)
        }
    }
    // NOT_SUPPORTED: fn SetQuality()
    // BLOCKED: fn operator=()
    fn get_filename(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxPrintData_GetFilename(self.as_ptr())).into() }
    }
    fn set_filename(&self, filename: &str) {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxPrintData_SetFilename(self.as_ptr(), filename)
        }
    }
    // NOT_SUPPORTED: fn GetPrintMode()
    // NOT_SUPPORTED: fn SetPrintMode()
}

// wxPrintPreview
pub trait PrintPreviewMethods: ObjectMethods {
    // DTOR: fn ~wxPrintPreview()
    fn get_canvas(&self) -> *mut c_void {
        unsafe { ffi::wxPrintPreview_GetCanvas(self.as_ptr()) }
    }
    fn get_current_page(&self) -> c_int {
        unsafe { ffi::wxPrintPreview_GetCurrentPage(self.as_ptr()) }
    }
    fn get_frame(&self) -> WeakRef<Frame> {
        unsafe { WeakRef::<Frame>::from(ffi::wxPrintPreview_GetFrame(self.as_ptr())) }
    }
    fn get_max_page(&self) -> c_int {
        unsafe { ffi::wxPrintPreview_GetMaxPage(self.as_ptr()) }
    }
    fn get_min_page(&self) -> c_int {
        unsafe { ffi::wxPrintPreview_GetMinPage(self.as_ptr()) }
    }
    fn get_printout(&self) -> *mut c_void {
        unsafe { ffi::wxPrintPreview_GetPrintout(self.as_ptr()) }
    }
    fn get_printout_for_printing(&self) -> *mut c_void {
        unsafe { ffi::wxPrintPreview_GetPrintoutForPrinting(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxPrintPreview_IsOk(self.as_ptr()) }
    }
    fn paint_page<D: DCMethods>(&self, canvas: *mut c_void, dc: &D) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxPrintPreview_PaintPage(self.as_ptr(), canvas, dc)
        }
    }
    fn print(&self, prompt: bool) -> bool {
        unsafe { ffi::wxPrintPreview_Print(self.as_ptr(), prompt) }
    }
    fn render_page(&self, page_num: c_int) -> bool {
        unsafe { ffi::wxPrintPreview_RenderPage(self.as_ptr(), page_num) }
    }
    fn set_canvas(&self, window: *mut c_void) {
        unsafe { ffi::wxPrintPreview_SetCanvas(self.as_ptr(), window) }
    }
    fn set_current_page(&self, page_num: c_int) -> bool {
        unsafe { ffi::wxPrintPreview_SetCurrentPage(self.as_ptr(), page_num) }
    }
    fn set_frame<F: FrameMethods>(&self, frame: Option<&F>) {
        unsafe {
            let frame = match frame {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxPrintPreview_SetFrame(self.as_ptr(), frame)
        }
    }
    fn set_printout(&self, printout: *mut c_void) {
        unsafe { ffi::wxPrintPreview_SetPrintout(self.as_ptr(), printout) }
    }
    fn set_zoom(&self, percent: c_int) {
        unsafe { ffi::wxPrintPreview_SetZoom(self.as_ptr(), percent) }
    }
}

// wxPrinterDC
pub trait PrinterDCMethods: DCMethods {
    fn get_paper_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxPrinterDC_GetPaperRect(self.as_ptr())) }
    }
}

// wxQuantize
pub trait QuantizeMethods: ObjectMethods {
    fn do_quantize(
        w: c_uint,
        h: c_uint,
        in_rows: *mut c_void,
        out_rows: *mut c_void,
        palette: *mut c_void,
        desired_no_colours: c_int,
    ) {
        unsafe { ffi::wxQuantize_DoQuantize(w, h, in_rows, out_rows, palette, desired_no_colours) }
    }
    fn quantize_palette<P: PaletteMethods>(
        src: *const c_void,
        dest: *mut c_void,
        p_palette: Option<&P>,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool {
        unsafe {
            let p_palette = match p_palette {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxQuantize_Quantize(
                src,
                dest,
                p_palette,
                desired_no_colours,
                eight_bit_data,
                flags,
            )
        }
    }
    fn quantize_int(
        src: *const c_void,
        dest: *mut c_void,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool {
        unsafe { ffi::wxQuantize_Quantize1(src, dest, desired_no_colours, eight_bit_data, flags) }
    }
}

// wxQueryLayoutInfoEvent
pub trait QueryLayoutInfoEventMethods: EventMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetOrientation()
    fn get_requested_length(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetRequestedLength(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxQueryLayoutInfoEvent_GetSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SetOrientation()
    fn set_requested_length(&self, length: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetRequestedLength(self.as_ptr(), length) }
    }
    fn set_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxQueryLayoutInfoEvent_SetSize(self.as_ptr(), size)
        }
    }
}

// wxRadioBox
pub trait RadioBoxMethods: ControlMethods {
    // DTOR: fn ~wxRadioBox()
    // NOT_SUPPORTED: fn Create()
    fn create_str<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        choices: &A,
        major_dimension: c_int,
        style: c_long,
        validator: &V,
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
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRadioBox_Create1(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                choices,
                major_dimension,
                style,
                validator,
                name,
            )
        }
    }
    fn enable_uint(&self, n: c_uint, enable: bool) -> bool {
        unsafe { ffi::wxRadioBox_Enable(self.as_ptr(), n, enable) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetColumnCount(self.as_ptr()) }
    }
    fn get_item_from_point<P: PointMethods>(&self, pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRadioBox_GetItemFromPoint(self.as_ptr(), pt)
        }
    }
    fn get_item_help_text(&self, item: c_uint) -> String {
        unsafe { WxString::from_ptr(ffi::wxRadioBox_GetItemHelpText(self.as_ptr(), item)).into() }
    }
    fn get_item_tool_tip(&self, item: c_uint) -> *mut c_void {
        unsafe { ffi::wxRadioBox_GetItemToolTip(self.as_ptr(), item) }
    }
    fn get_row_count(&self) -> c_uint {
        unsafe { ffi::wxRadioBox_GetRowCount(self.as_ptr()) }
    }
    fn is_item_enabled(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemEnabled(self.as_ptr(), n) }
    }
    fn is_item_shown(&self, n: c_uint) -> bool {
        unsafe { ffi::wxRadioBox_IsItemShown(self.as_ptr(), n) }
    }
    fn set_item_help_text(&self, item: c_uint, helptext: &str) {
        unsafe {
            let helptext = WxString::from(helptext);
            let helptext = helptext.as_ptr();
            ffi::wxRadioBox_SetItemHelpText(self.as_ptr(), item, helptext)
        }
    }
    fn set_item_tool_tip(&self, item: c_uint, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxRadioBox_SetItemToolTip(self.as_ptr(), item, text)
        }
    }
    fn show_uint(&self, item: c_uint, show: bool) -> bool {
        unsafe { ffi::wxRadioBox_Show(self.as_ptr(), item, show) }
    }
}

// wxRadioButton
pub trait RadioButtonMethods: ControlMethods {
    // DTOR: fn ~wxRadioButton()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRadioButton_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_value(&self) -> bool {
        unsafe { ffi::wxRadioButton_GetValue(self.as_ptr()) }
    }
    fn set_value(&self, value: bool) {
        unsafe { ffi::wxRadioButton_SetValue(self.as_ptr(), value) }
    }
    fn get_first_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetFirstInGroup(self.as_ptr())) }
    }
    fn get_last_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetLastInGroup(self.as_ptr())) }
    }
    fn get_previous_in_group(&self) -> WeakRef<RadioButton> {
        unsafe {
            WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetPreviousInGroup(self.as_ptr()))
        }
    }
    fn get_next_in_group(&self) -> WeakRef<RadioButton> {
        unsafe { WeakRef::<RadioButton>::from(ffi::wxRadioButton_GetNextInGroup(self.as_ptr())) }
    }
}

// wxRealPoint
pub trait RealPointMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+2()
    // BLOCKED: fn operator-2()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxRearrangeCtrl
pub trait RearrangeCtrlMethods: PanelMethods {
    fn create_arrayint<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let order = order.as_ptr();
            let items = items.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRearrangeCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                order,
                items,
                style,
                validator,
                name,
            )
        }
    }
    fn get_list(&self) -> WeakRef<RearrangeList> {
        unsafe { WeakRef::<RearrangeList>::from(ffi::wxRearrangeCtrl_GetList(self.as_ptr())) }
    }
}

// wxRearrangeList
pub trait RearrangeListMethods: CheckListBoxMethods {
    fn create_arrayint<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayIntMethods,
        A2: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        order: &A,
        items: &A2,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let order = order.as_ptr();
            let items = items.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxRearrangeList_Create(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                order,
                items,
                style,
                validator,
                name,
            )
        }
    }
    fn get_current_order(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxRearrangeList_GetCurrentOrder(self.as_ptr())) }
    }
    fn can_move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentUp(self.as_ptr()) }
    }
    fn can_move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_CanMoveCurrentDown(self.as_ptr()) }
    }
    fn move_current_up(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentUp(self.as_ptr()) }
    }
    fn move_current_down(&self) -> bool {
        unsafe { ffi::wxRearrangeList_MoveCurrentDown(self.as_ptr()) }
    }
}

// wxRect
pub trait RectMethods: WxRustMethods {
    fn centre_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect::from_ptr(ffi::wxRect_CentreIn(self.as_ptr(), r, dir))
        }
    }
    fn center_in<R: RectMethods>(&self, r: &R, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect::from_ptr(ffi::wxRect_CenterIn(self.as_ptr(), r, dir))
        }
    }
    fn contains_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr(), x, y) }
    }
    fn contains_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Contains1(self.as_ptr(), pt)
        }
    }
    fn contains_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Contains2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRect_Deflate3(self.as_ptr(), dx, dy)) }
    }
    fn get_bottom(&self) -> c_int {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr()) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomLeft(self.as_ptr())) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetBottomRight(self.as_ptr())) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr()) }
    }
    fn get_left(&self) -> c_int {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetPosition(self.as_ptr())) }
    }
    fn get_right(&self) -> c_int {
        unsafe { ffi::wxRect_GetRight(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxRect_GetSize(self.as_ptr())) }
    }
    fn get_top(&self) -> c_int {
        unsafe { ffi::wxRect_GetTop(self.as_ptr()) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopLeft(self.as_ptr())) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxRect_GetTopRight(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr()) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRect_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRect_GetY(self.as_ptr()) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRect_Inflate3(self.as_ptr(), dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect::from_ptr(ffi::wxRect_Intersect1(self.as_ptr(), rect))
        }
    }
    fn intersects<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Intersects(self.as_ptr(), rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr()) }
    }
    fn offset_coord(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxRect_Offset(self.as_ptr(), dx, dy) }
    }
    fn offset_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Offset1(self.as_ptr(), pt)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr(), height) }
    }
    fn set_position<P: PointMethods>(&self, pos: &P) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxRect_SetPosition(self.as_ptr(), pos)
        }
    }
    fn set_size<S: SizeMethods>(&self, s: &S) {
        unsafe {
            let s = s.as_ptr();
            ffi::wxRect_SetSize(self.as_ptr(), s)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr(), width) }
    }
    fn set_x(&self, x: c_int) {
        unsafe { ffi::wxRect_SetX(self.as_ptr(), x) }
    }
    fn set_y(&self, y: c_int) {
        unsafe { ffi::wxRect_SetY(self.as_ptr(), y) }
    }
    fn set_left(&self, left: c_int) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr(), left) }
    }
    fn set_right(&self, right: c_int) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr(), right) }
    }
    fn set_top(&self, top: c_int) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr(), top) }
    }
    fn set_bottom(&self, bottom: c_int) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr(), bottom) }
    }
    fn set_top_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopLeft(self.as_ptr(), p)
        }
    }
    fn set_bottom_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomRight(self.as_ptr(), p)
        }
    }
    fn set_top_right<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopRight(self.as_ptr(), p)
        }
    }
    fn set_bottom_left<P: PointMethods>(&self, p: &P) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomLeft(self.as_ptr(), p)
        }
    }
    fn union<R: RectMethods>(&self, rect: &R) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect::from_ptr(ffi::wxRect_Union(self.as_ptr(), rect))
        }
    }
    // BLOCKED: fn Union1()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxRegionIterator
pub trait RegionIteratorMethods: ObjectMethods {
    fn get_h(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetH(self.as_ptr()) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetHeight(self.as_ptr()) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxRegionIterator_GetRect(self.as_ptr())) }
    }
    fn get_w(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetW(self.as_ptr()) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetWidth(self.as_ptr()) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRegionIterator_GetY(self.as_ptr()) }
    }
    fn have_rects(&self) -> bool {
        unsafe { ffi::wxRegionIterator_HaveRects(self.as_ptr()) }
    }
    fn reset(&self) {
        unsafe { ffi::wxRegionIterator_Reset(self.as_ptr()) }
    }
    fn reset_region(&self, region: *const c_void) {
        unsafe { ffi::wxRegionIterator_Reset1(self.as_ptr(), region) }
    }
    // BLOCKED: fn operator++()
    // NOT_SUPPORTED: fn operator bool()
}

// wxRendererNative
pub trait RendererNativeMethods: WxRustMethods {
    // DTOR: fn ~wxRendererNative()
    fn draw_check_box<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawCheckBox(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_combo_box_drop_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawComboBoxDropButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_drop_arrow<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawDropArrow(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_focus_rect<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawFocusRect(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_gauge<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        value: c_int,
        max: c_int,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawGauge(self.as_ptr(), win, dc, rect, value, max, flags)
        }
    }
    // NOT_SUPPORTED: fn DrawHeaderButton()
    // NOT_SUPPORTED: fn DrawHeaderButtonContents()
    fn draw_item_selection_rect<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawItemSelectionRect(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_item_text<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        text: &str,
        rect: &R,
        align: c_int,
        flags: c_int,
        ellipsize_mode: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawItemText(
                self.as_ptr(),
                win,
                dc,
                text,
                rect,
                align,
                flags,
                ellipsize_mode,
            )
        }
    }
    fn draw_push_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawPushButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_collapse_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawCollapseButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn get_collapse_button_size<W: WindowMethods, D: DCMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
    ) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            Size::from_ptr(ffi::wxRendererNative_GetCollapseButtonSize(
                self.as_ptr(),
                win,
                dc,
            ))
        }
    }
    fn draw_splitter_border<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawSplitterBorder(self.as_ptr(), win, dc, rect, flags)
        }
    }
    // NOT_SUPPORTED: fn DrawSplitterSash()
    fn draw_tree_item_button<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawTreeItemButton(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_choice<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawChoice(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_combo_box<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawComboBox(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_text_ctrl<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawTextCtrl(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn draw_radio_bitmap<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawRadioBitmap(self.as_ptr(), win, dc, rect, flags)
        }
    }
    // NOT_SUPPORTED: fn DrawTitleBarBitmap()
    fn draw_check_mark<W: WindowMethods, D: DCMethods, R: RectMethods>(
        &self,
        win: Option<&W>,
        dc: &D,
        rect: &R,
        flags: c_int,
    ) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxRendererNative_DrawCheckMark(self.as_ptr(), win, dc, rect, flags)
        }
    }
    fn get_check_box_size<W: WindowMethods>(&self, win: Option<&W>, flags: c_int) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetCheckBoxSize(
                self.as_ptr(),
                win,
                flags,
            ))
        }
    }
    fn get_check_mark_size<W: WindowMethods>(&self, win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetCheckMarkSize(self.as_ptr(), win))
        }
    }
    fn get_expander_size<W: WindowMethods>(&self, win: Option<&W>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxRendererNative_GetExpanderSize(self.as_ptr(), win))
        }
    }
    fn get_header_button_height<W: WindowMethods>(&self, win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRendererNative_GetHeaderButtonHeight(self.as_ptr(), win)
        }
    }
    fn get_header_button_margin<W: WindowMethods>(&self, win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRendererNative_GetHeaderButtonMargin(self.as_ptr(), win)
        }
    }
    // NOT_SUPPORTED: fn GetSplitterParams()
    // NOT_SUPPORTED: fn GetVersion()
    fn get() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_Get()) }
    }
    fn get_default() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetDefault()) }
    }
    fn get_generic() -> RendererNativeIsOwned<false> {
        unsafe { RendererNativeIsOwned::from_ptr(ffi::wxRendererNative_GetGeneric()) }
    }
    fn load(name: &str) -> Option<RendererNativeIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            RendererNative::option_from(ffi::wxRendererNative_Load(name))
        }
    }
    fn set<R: RendererNativeMethods>(renderer: Option<&R>) -> Option<RendererNativeIsOwned<false>> {
        unsafe {
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            RendererNative::option_from(ffi::wxRendererNative_Set(renderer))
        }
    }
}

// wxRichToolTip
pub trait RichToolTipMethods: WxRustMethods {
    fn set_background_colour<C: ColourMethods, C2: ColourMethods>(&self, col: &C, col_end: &C2) {
        unsafe {
            let col = col.as_ptr();
            let col_end = col_end.as_ptr();
            ffi::wxRichToolTip_SetBackgroundColour(self.as_ptr(), col, col_end)
        }
    }
    fn set_icon_int(&self, icon: c_int) {
        unsafe { ffi::wxRichToolTip_SetIcon(self.as_ptr(), icon) }
    }
    fn set_icon_bitmapbundle<B: BitmapBundleMethods>(&self, icon: &B) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxRichToolTip_SetIcon1(self.as_ptr(), icon)
        }
    }
    // NOT_SUPPORTED: fn SetTimeout()
    // NOT_SUPPORTED: fn SetTipKind()
    fn set_title_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxRichToolTip_SetTitleFont(self.as_ptr(), font)
        }
    }
    fn show_for<W: WindowMethods, R: RectMethods>(&self, win: Option<&W>, rect: Option<&R>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxRichToolTip_ShowFor(self.as_ptr(), win, rect)
        }
    }
    // DTOR: fn ~wxRichToolTip()
}

// wxRotateGestureEvent
pub trait RotateGestureEventMethods: GestureEventMethods {
    fn get_rotation_angle(&self) -> c_double {
        unsafe { ffi::wxRotateGestureEvent_GetRotationAngle(self.as_ptr()) }
    }
    fn set_rotation_angle(&self, rotation_angle: c_double) {
        unsafe { ffi::wxRotateGestureEvent_SetRotationAngle(self.as_ptr(), rotation_angle) }
    }
}

// wxSVGBitmapEmbedHandler
pub trait SVGBitmapEmbedHandlerMethods: SVGBitmapHandlerMethods {}

// wxSVGBitmapFileHandler
pub trait SVGBitmapFileHandlerMethods: SVGBitmapHandlerMethods {}

// wxSVGBitmapHandler
pub trait SVGBitmapHandlerMethods: WxRustMethods {
    fn process_bitmap<B: BitmapMethods>(
        &self,
        bitmap: &B,
        x: c_int,
        y: c_int,
        stream: *mut c_void,
    ) -> bool {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxSVGBitmapHandler_ProcessBitmap(self.as_ptr(), bitmap, x, y, stream)
        }
    }
}

// wxSVGFileDC
pub trait SVGFileDCMethods: DCMethods {
    fn set_bitmap_handler<S: SVGBitmapHandlerMethods>(&self, handler: Option<&S>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSVGFileDC_SetBitmapHandler(self.as_ptr(), handler)
        }
    }
    // NOT_SUPPORTED: fn SetShapeRenderingMode()
}

// wxSashEvent
pub trait SashEventMethods: CommandEventMethods {
    fn get_drag_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxSashEvent_GetDragRect(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetDragStatus()
    // NOT_SUPPORTED: fn GetEdge()
    // NOT_SUPPORTED: fn SetEdge()
    fn set_drag_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxSashEvent_SetDragRect(self.as_ptr(), rect)
        }
    }
    // NOT_SUPPORTED: fn SetDragStatus()
}

// wxSashLayoutWindow
pub trait SashLayoutWindowMethods: SashWindowMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    // NOT_SUPPORTED: fn GetOrientation()
    fn on_calculate_layout<C: CalculateLayoutEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxSashLayoutWindow_OnCalculateLayout(self.as_ptr(), event)
        }
    }
    fn on_query_layout_info<Q: QueryLayoutInfoEventMethods>(&self, event: &Q) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxSashLayoutWindow_OnQueryLayoutInfo(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    fn set_default_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSashLayoutWindow_SetDefaultSize(self.as_ptr(), size)
        }
    }
    // NOT_SUPPORTED: fn SetOrientation()
}

// wxSashWindow
pub trait SashWindowMethods: WindowMethods {
    // DTOR: fn ~wxSashWindow()
    fn get_maximum_size_x(&self) -> c_int {
        unsafe { ffi::wxSashWindow_GetMaximumSizeX(self.as_ptr()) }
    }
    fn get_maximum_size_y(&self) -> c_int {
        unsafe { ffi::wxSashWindow_GetMaximumSizeY(self.as_ptr()) }
    }
    fn get_minimum_size_x(&self) -> c_int {
        unsafe { ffi::wxSashWindow_GetMinimumSizeX(self.as_ptr()) }
    }
    fn get_minimum_size_y(&self) -> c_int {
        unsafe { ffi::wxSashWindow_GetMinimumSizeY(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetSashVisible()
    fn set_maximum_size_x(&self, min: c_int) {
        unsafe { ffi::wxSashWindow_SetMaximumSizeX(self.as_ptr(), min) }
    }
    fn set_maximum_size_y(&self, min: c_int) {
        unsafe { ffi::wxSashWindow_SetMaximumSizeY(self.as_ptr(), min) }
    }
    fn set_minimum_size_x(&self, min: c_int) {
        unsafe { ffi::wxSashWindow_SetMinimumSizeX(self.as_ptr(), min) }
    }
    fn set_minimum_size_y(&self, min: c_int) {
        unsafe { ffi::wxSashWindow_SetMinimumSizeY(self.as_ptr(), min) }
    }
    // NOT_SUPPORTED: fn SetSashVisible()
    // NOT_SUPPORTED: fn GetEdgeMargin()
    fn set_default_border_size(&self, width: c_int) {
        unsafe { ffi::wxSashWindow_SetDefaultBorderSize(self.as_ptr(), width) }
    }
    fn get_default_border_size(&self) -> c_int {
        unsafe { ffi::wxSashWindow_GetDefaultBorderSize(self.as_ptr()) }
    }
    fn set_extra_border_size(&self, width: c_int) {
        unsafe { ffi::wxSashWindow_SetExtraBorderSize(self.as_ptr(), width) }
    }
    fn get_extra_border_size(&self) -> c_int {
        unsafe { ffi::wxSashWindow_GetExtraBorderSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SashHitTest()
    fn size_windows(&self) {
        unsafe { ffi::wxSashWindow_SizeWindows(self.as_ptr()) }
    }
}

// wxScreenDC
pub trait ScreenDCMethods: DCMethods {
    fn end_drawing_on_top() -> bool {
        unsafe { ffi::wxScreenDC_EndDrawingOnTop() }
    }
    fn start_drawing_on_top_window<W: WindowMethods>(window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxScreenDC_StartDrawingOnTop(window)
        }
    }
    fn start_drawing_on_top_rect<R: RectMethods>(rect: Option<&R>) -> bool {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxScreenDC_StartDrawingOnTop1(rect)
        }
    }
}

// wxScrollBar
pub trait ScrollBarMethods: ControlMethods {
    // DTOR: fn ~wxScrollBar()
    fn get_page_size(&self) -> c_int {
        unsafe { ffi::wxScrollBar_GetPageSize(self.as_ptr()) }
    }
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxScrollBar_GetRange(self.as_ptr()) }
    }
    fn get_thumb_position(&self) -> c_int {
        unsafe { ffi::wxScrollBar_GetThumbPosition(self.as_ptr()) }
    }
    fn get_thumb_size(&self) -> c_int {
        unsafe { ffi::wxScrollBar_GetThumbSize(self.as_ptr()) }
    }
    fn set_thumb_position(&self, view_start: c_int) {
        unsafe { ffi::wxScrollBar_SetThumbPosition(self.as_ptr(), view_start) }
    }
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxScrollBar_IsVertical(self.as_ptr()) }
    }
}

// wxScrollEvent
pub trait ScrollEventMethods: CommandEventMethods {
    fn get_orientation(&self) -> c_int {
        unsafe { ffi::wxScrollEvent_GetOrientation(self.as_ptr()) }
    }
    fn get_position(&self) -> c_int {
        unsafe { ffi::wxScrollEvent_GetPosition(self.as_ptr()) }
    }
    fn set_orientation(&self, orient: c_int) {
        unsafe { ffi::wxScrollEvent_SetOrientation(self.as_ptr(), orient) }
    }
    fn set_position(&self, pos: c_int) {
        unsafe { ffi::wxScrollEvent_SetPosition(self.as_ptr(), pos) }
    }
}

// wxScrollWinEvent
pub trait ScrollWinEventMethods: EventMethods {
    fn get_orientation(&self) -> c_int {
        unsafe { ffi::wxScrollWinEvent_GetOrientation(self.as_ptr()) }
    }
    fn get_position(&self) -> c_int {
        unsafe { ffi::wxScrollWinEvent_GetPosition(self.as_ptr()) }
    }
    fn set_orientation(&self, orient: c_int) {
        unsafe { ffi::wxScrollWinEvent_SetOrientation(self.as_ptr(), orient) }
    }
    fn set_position(&self, pos: c_int) {
        unsafe { ffi::wxScrollWinEvent_SetPosition(self.as_ptr(), pos) }
    }
}

// wxSearchCtrl
pub trait SearchCtrlMethods: TextCtrlMethods {
    // DTOR: fn ~wxSearchCtrl()
    fn get_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxSearchCtrl_GetMenu(self.as_ptr())) }
    }
    fn is_search_button_visible(&self) -> bool {
        unsafe { ffi::wxSearchCtrl_IsSearchButtonVisible(self.as_ptr()) }
    }
    fn is_cancel_button_visible(&self) -> bool {
        unsafe { ffi::wxSearchCtrl_IsCancelButtonVisible(self.as_ptr()) }
    }
    fn set_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSearchCtrl_SetMenu(self.as_ptr(), menu)
        }
    }
    fn show_cancel_button(&self, show: bool) {
        unsafe { ffi::wxSearchCtrl_ShowCancelButton(self.as_ptr(), show) }
    }
    fn show_search_button(&self, show: bool) {
        unsafe { ffi::wxSearchCtrl_ShowSearchButton(self.as_ptr(), show) }
    }
    fn set_descriptive_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxSearchCtrl_SetDescriptiveText(self.as_ptr(), text)
        }
    }
    fn get_descriptive_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxSearchCtrl_GetDescriptiveText(self.as_ptr())).into() }
    }
}

// wxSetCursorEvent
pub trait SetCursorEventMethods: EventMethods {
    fn get_cursor(&self) -> CursorIsOwned<false> {
        unsafe { CursorIsOwned::from_ptr(ffi::wxSetCursorEvent_GetCursor(self.as_ptr())) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxSetCursorEvent_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxSetCursorEvent_GetY(self.as_ptr()) }
    }
    fn has_cursor(&self) -> bool {
        unsafe { ffi::wxSetCursorEvent_HasCursor(self.as_ptr()) }
    }
    fn set_cursor<C: CursorMethods>(&self, cursor: &C) {
        unsafe {
            let cursor = cursor.as_ptr();
            ffi::wxSetCursorEvent_SetCursor(self.as_ptr(), cursor)
        }
    }
}

// wxSettableHeaderColumn
pub trait SettableHeaderColumnMethods: HeaderColumnMethods {
    fn set_title(&self, title: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxSettableHeaderColumn_SetTitle(self.as_ptr(), title)
        }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxSettableHeaderColumn_SetBitmap(self.as_ptr(), bitmap)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetWidth(self.as_ptr(), width) }
    }
    fn set_min_width(&self, min_width: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetMinWidth(self.as_ptr(), min_width) }
    }
    fn set_alignment(&self, align: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetAlignment(self.as_ptr(), align) }
    }
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetFlags(self.as_ptr(), flags) }
    }
    fn change_flag(&self, flag: c_int, set: bool) {
        unsafe { ffi::wxSettableHeaderColumn_ChangeFlag(self.as_ptr(), flag, set) }
    }
    fn set_flag(&self, flag: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_SetFlag(self.as_ptr(), flag) }
    }
    fn clear_flag(&self, flag: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_ClearFlag(self.as_ptr(), flag) }
    }
    fn toggle_flag(&self, flag: c_int) {
        unsafe { ffi::wxSettableHeaderColumn_ToggleFlag(self.as_ptr(), flag) }
    }
    fn set_resizeable(&self, resizable: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetResizeable(self.as_ptr(), resizable) }
    }
    fn set_sortable(&self, sortable: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetSortable(self.as_ptr(), sortable) }
    }
    fn set_reorderable(&self, reorderable: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetReorderable(self.as_ptr(), reorderable) }
    }
    fn set_hidden(&self, hidden: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetHidden(self.as_ptr(), hidden) }
    }
    fn unset_as_sort_key(&self) {
        unsafe { ffi::wxSettableHeaderColumn_UnsetAsSortKey(self.as_ptr()) }
    }
    fn set_sort_order(&self, ascending: bool) {
        unsafe { ffi::wxSettableHeaderColumn_SetSortOrder(self.as_ptr(), ascending) }
    }
    fn toggle_sort_order(&self) {
        unsafe { ffi::wxSettableHeaderColumn_ToggleSortOrder(self.as_ptr()) }
    }
}

// wxSimpleHelpProvider
pub trait SimpleHelpProviderMethods: HelpProviderMethods {}

// wxSimplebook
pub trait SimplebookMethods: BookCtrlBaseMethods {
    // NOT_SUPPORTED: fn SetEffects()
    // NOT_SUPPORTED: fn SetEffect()
    // NOT_SUPPORTED: fn SetEffectsTimeouts()
    // NOT_SUPPORTED: fn SetEffectTimeout()
    fn show_new_page<W: WindowMethods>(&self, page: Option<&W>) -> bool {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSimplebook_ShowNewPage(self.as_ptr(), page)
        }
    }
}

// wxSize
pub trait SizeMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator/1()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator*2()
    // BLOCKED: fn operator*3()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator/=1()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator*=1()
    fn dec_by_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_DecBy(self.as_ptr(), pt)
        }
    }
    fn dec_by_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecBy1(self.as_ptr(), size)
        }
    }
    fn dec_by_int_int(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_DecBy2(self.as_ptr(), dx, dy) }
    }
    fn dec_by_int(&self, d: c_int) {
        unsafe { ffi::wxSize_DecBy3(self.as_ptr(), d) }
    }
    fn dec_to<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecTo(self.as_ptr(), size)
        }
    }
    fn dec_to_if_specified<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecToIfSpecified(self.as_ptr(), size)
        }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxSize_GetHeight(self.as_ptr()) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxSize_GetWidth(self.as_ptr()) }
    }
    fn inc_by_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_IncBy(self.as_ptr(), pt)
        }
    }
    fn inc_by_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncBy1(self.as_ptr(), size)
        }
    }
    fn inc_by_int_int(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_IncBy2(self.as_ptr(), dx, dy) }
    }
    fn inc_by_int(&self, d: c_int) {
        unsafe { ffi::wxSize_IncBy3(self.as_ptr(), d) }
    }
    fn inc_to<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncTo(self.as_ptr(), size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(self.as_ptr()) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSize_Set(self.as_ptr(), width, height) }
    }
    fn set_defaults<S: SizeMethods>(&self, size_default: &S) {
        unsafe {
            let size_default = size_default.as_ptr();
            ffi::wxSize_SetDefaults(self.as_ptr(), size_default)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxSize_SetHeight(self.as_ptr(), height) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxSize_SetWidth(self.as_ptr(), width) }
    }
}

// wxSizeEvent
pub trait SizeEventMethods: EventMethods {
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizeEvent_GetSize(self.as_ptr())) }
    }
    fn set_size(&self, size: ffi::wxSize) {
        unsafe { ffi::wxSizeEvent_SetSize(self.as_ptr(), size) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxSizeEvent_GetRect(self.as_ptr())) }
    }
    fn set_rect(&self, rect: ffi::wxRect) {
        unsafe { ffi::wxSizeEvent_SetRect(self.as_ptr(), rect) }
    }
}

// wxSizer
pub trait SizerMethods: ObjectMethods {
    // DTOR: fn ~wxSizer()
    fn add_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        window: Option<&W>,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Add(self.as_ptr(), window, flags)
        }
    }
    fn add_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Add1(self.as_ptr(), window, proportion, flag, border, user_data)
        }
    }
    fn add_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        sizer: Option<&S>,
        flags: &S2,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Add2(self.as_ptr(), sizer, flags)
        }
    }
    fn add_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Add3(self.as_ptr(), sizer, proportion, flag, border, user_data)
        }
    }
    fn add_int_int<O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Add4(
                self.as_ptr(),
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn add_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let flags = flags.as_ptr();
            ffi::wxSizer_Add5(self.as_ptr(), width, height, flags)
        }
    }
    fn add_sizeritem(&self, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxSizer_Add6(self.as_ptr(), item) }
    }
    fn add_spacer(&self, size: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_AddSpacer(self.as_ptr(), size) }
    }
    fn add_stretch_spacer(&self, prop: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_AddStretchSpacer(self.as_ptr(), prop) }
    }
    fn calc_min(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizer_CalcMin(self.as_ptr())) }
    }
    fn clear(&self, delete_windows: bool) {
        unsafe { ffi::wxSizer_Clear(self.as_ptr(), delete_windows) }
    }
    fn compute_fitting_client_size<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxSizer_ComputeFittingClientSize(self.as_ptr(), window))
        }
    }
    fn compute_fitting_window_size<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxSizer_ComputeFittingWindowSize(self.as_ptr(), window))
        }
    }
    fn detach_window<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Detach(self.as_ptr(), window)
        }
    }
    fn detach_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Detach1(self.as_ptr(), sizer)
        }
    }
    fn detach_int(&self, index: c_int) -> bool {
        unsafe { ffi::wxSizer_Detach2(self.as_ptr(), index) }
    }
    fn fit<W: WindowMethods>(&self, window: Option<&W>) -> Size {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxSizer_Fit(self.as_ptr(), window))
        }
    }
    fn fit_inside<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_FitInside(self.as_ptr(), window)
        }
    }
    fn inform_first_direction(
        &self,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool {
        unsafe {
            ffi::wxSizer_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir)
        }
    }
    fn get_children(&self) -> SizerItemListIsOwned<false> {
        unsafe { SizerItemListIsOwned::from_ptr(ffi::wxSizer_GetChildren(self.as_ptr())) }
    }
    // BLOCKED: fn GetChildren1()
    fn get_containing_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxSizer_GetContainingWindow(self.as_ptr())) }
    }
    fn set_containing_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetContainingWindow(self.as_ptr(), window)
        }
    }
    fn get_item_count(&self) -> usize {
        unsafe { ffi::wxSizer_GetItemCount(self.as_ptr()) }
    }
    fn get_item_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
        recursive: bool,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_GetItem(self.as_ptr(), window, recursive)
        }
    }
    fn get_item_sizer<S: SizerMethods>(&self, sizer: Option<&S>, recursive: bool) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_GetItem1(self.as_ptr(), sizer, recursive)
        }
    }
    fn get_item_sz(&self, index: usize) -> *mut c_void {
        unsafe { ffi::wxSizer_GetItem2(self.as_ptr(), index) }
    }
    fn get_item_by_id(&self, id: c_int, recursive: bool) -> *mut c_void {
        unsafe { ffi::wxSizer_GetItemById(self.as_ptr(), id, recursive) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizer_GetMinSize(self.as_ptr())) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxSizer_GetPosition(self.as_ptr())) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxSizer_GetSize(self.as_ptr())) }
    }
    fn hide_window<W: WindowMethods>(&self, window: Option<&W>, recursive: bool) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Hide(self.as_ptr(), window, recursive)
        }
    }
    fn hide_sizer<S: SizerMethods>(&self, sizer: Option<&S>, recursive: bool) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Hide1(self.as_ptr(), sizer, recursive)
        }
    }
    fn hide_sz(&self, index: usize) -> bool {
        unsafe { ffi::wxSizer_Hide2(self.as_ptr(), index) }
    }
    fn insert_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        index: usize,
        window: Option<&W>,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Insert(self.as_ptr(), index, window, flags)
        }
    }
    fn insert_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        index: usize,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Insert1(
                self.as_ptr(),
                index,
                window,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn insert_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        index: usize,
        sizer: Option<&S>,
        flags: &S2,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Insert2(self.as_ptr(), index, sizer, flags)
        }
    }
    fn insert_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        index: usize,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Insert3(
                self.as_ptr(),
                index,
                sizer,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn insert_int_int<O: ObjectMethods>(
        &self,
        index: usize,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Insert4(
                self.as_ptr(),
                index,
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn insert_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        index: usize,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let flags = flags.as_ptr();
            ffi::wxSizer_Insert5(self.as_ptr(), index, width, height, flags)
        }
    }
    fn insert_sizeritem(&self, index: usize, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxSizer_Insert6(self.as_ptr(), index, item) }
    }
    fn insert_spacer(&self, index: usize, size: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_InsertSpacer(self.as_ptr(), index, size) }
    }
    fn insert_stretch_spacer(&self, index: usize, prop: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_InsertStretchSpacer(self.as_ptr(), index, prop) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxSizer_IsEmpty(self.as_ptr()) }
    }
    fn is_shown_window<W: WindowMethods>(&self, window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_IsShown(self.as_ptr(), window)
        }
    }
    fn is_shown_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_IsShown1(self.as_ptr(), sizer)
        }
    }
    fn is_shown_sz(&self, index: usize) -> bool {
        unsafe { ffi::wxSizer_IsShown2(self.as_ptr(), index) }
    }
    fn layout(&self) {
        unsafe { ffi::wxSizer_Layout(self.as_ptr()) }
    }
    fn prepend_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        &self,
        window: Option<&W>,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Prepend(self.as_ptr(), window, flags)
        }
    }
    fn prepend_window_int<W: WindowMethods, O: ObjectMethods>(
        &self,
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Prepend1(self.as_ptr(), window, proportion, flag, border, user_data)
        }
    }
    fn prepend_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        &self,
        sizer: Option<&S>,
        flags: &S2,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            ffi::wxSizer_Prepend2(self.as_ptr(), sizer, flags)
        }
    }
    fn prepend_sizer_int<S: SizerMethods, O: ObjectMethods>(
        &self,
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Prepend3(self.as_ptr(), sizer, proportion, flag, border, user_data)
        }
    }
    fn prepend_int_int<O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> *mut c_void {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Prepend4(
                self.as_ptr(),
                width,
                height,
                proportion,
                flag,
                border,
                user_data,
            )
        }
    }
    fn prepend_int_sizerflags<S: SizerFlagsMethods>(
        &self,
        width: c_int,
        height: c_int,
        flags: &S,
    ) -> *mut c_void {
        unsafe {
            let flags = flags.as_ptr();
            ffi::wxSizer_Prepend5(self.as_ptr(), width, height, flags)
        }
    }
    fn prepend_sizeritem(&self, item: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxSizer_Prepend6(self.as_ptr(), item) }
    }
    fn prepend_spacer(&self, size: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_PrependSpacer(self.as_ptr(), size) }
    }
    fn prepend_stretch_spacer(&self, prop: c_int) -> *mut c_void {
        unsafe { ffi::wxSizer_PrependStretchSpacer(self.as_ptr(), prop) }
    }
    fn reposition_children<S: SizeMethods>(&self, min_size: &S) {
        unsafe {
            let min_size = min_size.as_ptr();
            ffi::wxSizer_RepositionChildren(self.as_ptr(), min_size)
        }
    }
    // BLOCKED: fn Remove()
    fn remove_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Remove1(self.as_ptr(), sizer)
        }
    }
    fn remove_int(&self, index: c_int) -> bool {
        unsafe { ffi::wxSizer_Remove2(self.as_ptr(), index) }
    }
    fn replace_window<W: WindowMethods, W2: WindowMethods>(
        &self,
        oldwin: Option<&W>,
        newwin: Option<&W2>,
        recursive: bool,
    ) -> bool {
        unsafe {
            let oldwin = match oldwin {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let newwin = match newwin {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace(self.as_ptr(), oldwin, newwin, recursive)
        }
    }
    fn replace_sizer<S: SizerMethods, S2: SizerMethods>(
        &self,
        oldsz: Option<&S>,
        newsz: Option<&S2>,
        recursive: bool,
    ) -> bool {
        unsafe {
            let oldsz = match oldsz {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let newsz = match newsz {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Replace1(self.as_ptr(), oldsz, newsz, recursive)
        }
    }
    fn replace_sz(&self, index: usize, newitem: *mut c_void) -> bool {
        unsafe { ffi::wxSizer_Replace2(self.as_ptr(), index, newitem) }
    }
    fn set_dimension_int(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxSizer_SetDimension(self.as_ptr(), x, y, width, height) }
    }
    fn set_dimension_point<P: PointMethods, S: SizeMethods>(&self, pos: &P, size: &S) {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            ffi::wxSizer_SetDimension1(self.as_ptr(), pos, size)
        }
    }
    fn set_item_min_size_window_int<W: WindowMethods>(
        &self,
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetItemMinSize(self.as_ptr(), window, width, height)
        }
    }
    fn set_item_min_size_window_size<W: WindowMethods, S: SizeMethods>(
        &self,
        window: Option<&W>,
        size: &S,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize1(self.as_ptr(), window, size)
        }
    }
    fn set_item_min_size_sizer_int<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetItemMinSize2(self.as_ptr(), sizer, width, height)
        }
    }
    fn set_item_min_size_sizer_size<S: SizerMethods, S2: SizeMethods>(
        &self,
        sizer: Option<&S>,
        size: &S2,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize3(self.as_ptr(), sizer, size)
        }
    }
    fn set_item_min_size_sz_int(&self, index: usize, width: c_int, height: c_int) -> bool {
        unsafe { ffi::wxSizer_SetItemMinSize4(self.as_ptr(), index, width, height) }
    }
    fn set_item_min_size_sz_size<S: SizeMethods>(&self, index: usize, size: &S) -> bool {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizer_SetItemMinSize5(self.as_ptr(), index, size)
        }
    }
    fn set_min_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSizer_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_min_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSizer_SetMinSize1(self.as_ptr(), width, height) }
    }
    fn set_size_hints<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_SetSizeHints(self.as_ptr(), window)
        }
    }
    // BLOCKED: fn SetVirtualSizeHints()
    fn show_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
        show: bool,
        recursive: bool,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Show(self.as_ptr(), window, show, recursive)
        }
    }
    fn show_sizer<S: SizerMethods>(&self, sizer: Option<&S>, show: bool, recursive: bool) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSizer_Show1(self.as_ptr(), sizer, show, recursive)
        }
    }
    fn show_sz(&self, index: usize, show: bool) -> bool {
        unsafe { ffi::wxSizer_Show2(self.as_ptr(), index, show) }
    }
    fn show_items(&self, show: bool) {
        unsafe { ffi::wxSizer_ShowItems(self.as_ptr(), show) }
    }
}

// wxSizerFlags
pub trait SizerFlagsMethods: WxRustMethods {
    fn align(&self, alignment: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Align(self.as_ptr(), alignment);
            &self
        }
    }
    fn border_int(&self, direction: c_int, borderinpixels: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Border(self.as_ptr(), direction, borderinpixels);
            &self
        }
    }
    fn border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Border1(self.as_ptr(), direction);
            &self
        }
    }
    fn bottom(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Bottom(self.as_ptr());
            &self
        }
    }
    fn center(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Center(self.as_ptr());
            &self
        }
    }
    fn centre(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Centre(self.as_ptr());
            &self
        }
    }
    fn center_horizontal(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CenterHorizontal(self.as_ptr());
            &self
        }
    }
    fn center_vertical(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CenterVertical(self.as_ptr());
            &self
        }
    }
    fn centre_horizontal(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CentreHorizontal(self.as_ptr());
            &self
        }
    }
    fn centre_vertical(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_CentreVertical(self.as_ptr());
            &self
        }
    }
    fn double_border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_DoubleBorder(self.as_ptr(), direction);
            &self
        }
    }
    fn double_horz_border(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_DoubleHorzBorder(self.as_ptr());
            &self
        }
    }
    fn expand(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Expand(self.as_ptr());
            &self
        }
    }
    fn fixed_min_size(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_FixedMinSize(self.as_ptr());
            &self
        }
    }
    fn reserve_space_even_if_hidden(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_ReserveSpaceEvenIfHidden(self.as_ptr());
            &self
        }
    }
    fn left(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Left(self.as_ptr());
            &self
        }
    }
    fn proportion(&self, proportion: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Proportion(self.as_ptr(), proportion);
            &self
        }
    }
    fn right(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Right(self.as_ptr());
            &self
        }
    }
    fn shaped(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Shaped(self.as_ptr());
            &self
        }
    }
    fn top(&self) -> &Self {
        unsafe {
            ffi::wxSizerFlags_Top(self.as_ptr());
            &self
        }
    }
    fn triple_border(&self, direction: c_int) -> &Self {
        unsafe {
            ffi::wxSizerFlags_TripleBorder(self.as_ptr(), direction);
            &self
        }
    }
    fn disable_consistency_checks() {
        unsafe { ffi::wxSizerFlags_DisableConsistencyChecks() }
    }
    fn get_default_border() -> c_int {
        unsafe { ffi::wxSizerFlags_GetDefaultBorder() }
    }
    // NOT_SUPPORTED: fn GetDefaultBorderFractional()
}

// wxSlider
pub trait SliderMethods: ControlMethods {
    // DTOR: fn ~wxSlider()
    fn clear_sel(&self) {
        unsafe { ffi::wxSlider_ClearSel(self.as_ptr()) }
    }
    fn clear_ticks(&self) {
        unsafe { ffi::wxSlider_ClearTicks(self.as_ptr()) }
    }
    fn create_int<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        point: &P,
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
            let point = point.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSlider_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                min_value,
                max_value,
                point,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_line_size(&self) -> c_int {
        unsafe { ffi::wxSlider_GetLineSize(self.as_ptr()) }
    }
    fn get_max(&self) -> c_int {
        unsafe { ffi::wxSlider_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_int {
        unsafe { ffi::wxSlider_GetMin(self.as_ptr()) }
    }
    fn get_page_size(&self) -> c_int {
        unsafe { ffi::wxSlider_GetPageSize(self.as_ptr()) }
    }
    fn get_sel_end(&self) -> c_int {
        unsafe { ffi::wxSlider_GetSelEnd(self.as_ptr()) }
    }
    fn get_sel_start(&self) -> c_int {
        unsafe { ffi::wxSlider_GetSelStart(self.as_ptr()) }
    }
    fn get_thumb_length(&self) -> c_int {
        unsafe { ffi::wxSlider_GetThumbLength(self.as_ptr()) }
    }
    fn get_tick_freq(&self) -> c_int {
        unsafe { ffi::wxSlider_GetTickFreq(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxSlider_GetValue(self.as_ptr()) }
    }
    fn set_line_size(&self, line_size: c_int) {
        unsafe { ffi::wxSlider_SetLineSize(self.as_ptr(), line_size) }
    }
    fn set_min(&self, min_value: c_int) {
        unsafe { ffi::wxSlider_SetMin(self.as_ptr(), min_value) }
    }
    fn set_max(&self, max_value: c_int) {
        unsafe { ffi::wxSlider_SetMax(self.as_ptr(), max_value) }
    }
    fn set_page_size(&self, page_size: c_int) {
        unsafe { ffi::wxSlider_SetPageSize(self.as_ptr(), page_size) }
    }
    fn set_range(&self, min_value: c_int, max_value: c_int) {
        unsafe { ffi::wxSlider_SetRange(self.as_ptr(), min_value, max_value) }
    }
    fn set_selection(&self, start_pos: c_int, end_pos: c_int) {
        unsafe { ffi::wxSlider_SetSelection(self.as_ptr(), start_pos, end_pos) }
    }
    fn set_thumb_length(&self, len: c_int) {
        unsafe { ffi::wxSlider_SetThumbLength(self.as_ptr(), len) }
    }
    fn set_tick(&self, tick_pos: c_int) {
        unsafe { ffi::wxSlider_SetTick(self.as_ptr(), tick_pos) }
    }
    fn set_tick_freq(&self, freq: c_int) {
        unsafe { ffi::wxSlider_SetTickFreq(self.as_ptr(), freq) }
    }
    fn set_value(&self, value: c_int) {
        unsafe { ffi::wxSlider_SetValue(self.as_ptr(), value) }
    }
}

// wxSound
pub trait SoundMethods: ObjectMethods {
    // DTOR: fn ~wxSound()
    fn create_str(&self, file_name: &str, is_resource: bool) -> bool {
        unsafe {
            let file_name = WxString::from(file_name);
            let file_name = file_name.as_ptr();
            ffi::wxSound_Create(self.as_ptr(), file_name, is_resource)
        }
    }
    fn create_sz(&self, size: usize, data: *const c_void) -> bool {
        unsafe { ffi::wxSound_Create1(self.as_ptr(), size, data) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxSound_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Play()
    fn is_playing() -> bool {
        unsafe { ffi::wxSound_IsPlaying() }
    }
    // NOT_SUPPORTED: fn Play1()
    fn stop() {
        unsafe { ffi::wxSound_Stop() }
    }
}

// wxSpinButton
pub trait SpinButtonMethods: ControlMethods {
    // DTOR: fn ~wxSpinButton()
    fn get_increment(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetIncrement(self.as_ptr()) }
    }
    fn get_max(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetMin(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxSpinButton_GetValue(self.as_ptr()) }
    }
    fn set_increment(&self, value: c_int) {
        unsafe { ffi::wxSpinButton_SetIncrement(self.as_ptr(), value) }
    }
    fn set_range(&self, min: c_int, max: c_int) {
        unsafe { ffi::wxSpinButton_SetRange(self.as_ptr(), min, max) }
    }
    fn set_value(&self, value: c_int) {
        unsafe { ffi::wxSpinButton_SetValue(self.as_ptr(), value) }
    }
}

// wxSpinCtrl
pub trait SpinCtrlMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSpinCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                min,
                max,
                initial,
                name,
            )
        }
    }
    fn get_base(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetBase(self.as_ptr()) }
    }
    fn get_max(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetMin(self.as_ptr()) }
    }
    fn get_text_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxSpinCtrl_GetTextValue(self.as_ptr())).into() }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetValue(self.as_ptr()) }
    }
    fn get_increment(&self) -> c_int {
        unsafe { ffi::wxSpinCtrl_GetIncrement(self.as_ptr()) }
    }
    fn set_base(&self, base: c_int) -> bool {
        unsafe { ffi::wxSpinCtrl_SetBase(self.as_ptr(), base) }
    }
    fn set_range(&self, min_val: c_int, max_val: c_int) {
        unsafe { ffi::wxSpinCtrl_SetRange(self.as_ptr(), min_val, max_val) }
    }
    fn set_selection(&self, from: c_long, to: c_long) {
        unsafe { ffi::wxSpinCtrl_SetSelection(self.as_ptr(), from, to) }
    }
    fn set_value_str(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxSpinCtrl_SetValue(self.as_ptr(), text)
        }
    }
    fn set_value_int(&self, value: c_int) {
        unsafe { ffi::wxSpinCtrl_SetValue1(self.as_ptr(), value) }
    }
    fn set_increment(&self, value: c_int) {
        unsafe { ffi::wxSpinCtrl_SetIncrement(self.as_ptr(), value) }
    }
}

// wxSpinCtrlDouble
pub trait SpinCtrlDoubleMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
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
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSpinCtrlDouble_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                min,
                max,
                initial,
                inc,
                name,
            )
        }
    }
    fn get_digits(&self) -> c_uint {
        unsafe { ffi::wxSpinCtrlDouble_GetDigits(self.as_ptr()) }
    }
    fn get_increment(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetIncrement(self.as_ptr()) }
    }
    fn get_max(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetMax(self.as_ptr()) }
    }
    fn get_min(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetMin(self.as_ptr()) }
    }
    fn get_text_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxSpinCtrlDouble_GetTextValue(self.as_ptr())).into() }
    }
    fn get_value(&self) -> c_double {
        unsafe { ffi::wxSpinCtrlDouble_GetValue(self.as_ptr()) }
    }
    fn set_digits(&self, digits: c_uint) {
        unsafe { ffi::wxSpinCtrlDouble_SetDigits(self.as_ptr(), digits) }
    }
    fn set_increment(&self, inc: c_double) {
        unsafe { ffi::wxSpinCtrlDouble_SetIncrement(self.as_ptr(), inc) }
    }
    fn set_range(&self, min_val: c_double, max_val: c_double) {
        unsafe { ffi::wxSpinCtrlDouble_SetRange(self.as_ptr(), min_val, max_val) }
    }
    fn set_value_str(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxSpinCtrlDouble_SetValue(self.as_ptr(), text)
        }
    }
    fn set_value_double(&self, value: c_double) {
        unsafe { ffi::wxSpinCtrlDouble_SetValue1(self.as_ptr(), value) }
    }
}

// wxSpinDoubleEvent
pub trait SpinDoubleEventMethods: NotifyEventMethods {
    fn get_value(&self) -> c_double {
        unsafe { ffi::wxSpinDoubleEvent_GetValue(self.as_ptr()) }
    }
    fn set_value(&self, value: c_double) {
        unsafe { ffi::wxSpinDoubleEvent_SetValue(self.as_ptr(), value) }
    }
}

// wxSpinEvent
pub trait SpinEventMethods: NotifyEventMethods {
    fn get_position(&self) -> c_int {
        unsafe { ffi::wxSpinEvent_GetPosition(self.as_ptr()) }
    }
    fn set_position(&self, pos: c_int) {
        unsafe { ffi::wxSpinEvent_SetPosition(self.as_ptr(), pos) }
    }
}

// wxSplashScreen
pub trait SplashScreenMethods: FrameMethods {
    // DTOR: fn ~wxSplashScreen()
    fn get_splash_style(&self) -> c_long {
        unsafe { ffi::wxSplashScreen_GetSplashStyle(self.as_ptr()) }
    }
    fn get_splash_window(&self) -> *mut c_void {
        unsafe { ffi::wxSplashScreen_GetSplashWindow(self.as_ptr()) }
    }
    fn get_timeout(&self) -> c_int {
        unsafe { ffi::wxSplashScreen_GetTimeout(self.as_ptr()) }
    }
    fn on_close_window<C: CloseEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxSplashScreen_OnCloseWindow(self.as_ptr(), event)
        }
    }
}

// wxSplitterEvent
pub trait SplitterEventMethods: NotifyEventMethods {
    fn get_sash_position(&self) -> c_int {
        unsafe { ffi::wxSplitterEvent_GetSashPosition(self.as_ptr()) }
    }
    fn get_window_being_removed(&self) -> WeakRef<Window> {
        unsafe {
            WeakRef::<Window>::from(ffi::wxSplitterEvent_GetWindowBeingRemoved(self.as_ptr()))
        }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxSplitterEvent_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxSplitterEvent_GetY(self.as_ptr()) }
    }
    fn set_sash_position(&self, pos: c_int) {
        unsafe { ffi::wxSplitterEvent_SetSashPosition(self.as_ptr(), pos) }
    }
    fn set_size(&self, old_size: c_int, new_size: c_int) {
        unsafe { ffi::wxSplitterEvent_SetSize(self.as_ptr(), old_size, new_size) }
    }
    fn get_old_size(&self) -> c_int {
        unsafe { ffi::wxSplitterEvent_GetOldSize(self.as_ptr()) }
    }
}

// wxSplitterWindow
pub trait SplitterWindowMethods: WindowMethods {
    // DTOR: fn ~wxSplitterWindow()
    fn get_minimum_pane_size(&self) -> c_int {
        unsafe { ffi::wxSplitterWindow_GetMinimumPaneSize(self.as_ptr()) }
    }
    fn get_sash_gravity(&self) -> c_double {
        unsafe { ffi::wxSplitterWindow_GetSashGravity(self.as_ptr()) }
    }
    fn get_sash_position(&self) -> c_int {
        unsafe { ffi::wxSplitterWindow_GetSashPosition(self.as_ptr()) }
    }
    fn get_sash_size(&self) -> c_int {
        unsafe { ffi::wxSplitterWindow_GetSashSize(self.as_ptr()) }
    }
    fn get_default_sash_size(&self) -> c_int {
        unsafe { ffi::wxSplitterWindow_GetDefaultSashSize(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetSplitMode()
    fn get_window1(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxSplitterWindow_GetWindow1(self.as_ptr())) }
    }
    fn get_window2(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxSplitterWindow_GetWindow2(self.as_ptr())) }
    }
    fn initialize<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSplitterWindow_Initialize(self.as_ptr(), window)
        }
    }
    fn is_sash_invisible(&self) -> bool {
        unsafe { ffi::wxSplitterWindow_IsSashInvisible(self.as_ptr()) }
    }
    fn is_split(&self) -> bool {
        unsafe { ffi::wxSplitterWindow_IsSplit(self.as_ptr()) }
    }
    fn on_double_click_sash(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxSplitterWindow_OnDoubleClickSash(self.as_ptr(), x, y) }
    }
    fn on_sash_position_change(&self, new_sash_position: c_int) -> bool {
        unsafe { ffi::wxSplitterWindow_OnSashPositionChange(self.as_ptr(), new_sash_position) }
    }
    fn on_unsplit<W: WindowMethods>(&self, removed: Option<&W>) {
        unsafe {
            let removed = match removed {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSplitterWindow_OnUnsplit(self.as_ptr(), removed)
        }
    }
    fn replace_window<W: WindowMethods, W2: WindowMethods>(
        &self,
        win_old: Option<&W>,
        win_new: Option<&W2>,
    ) -> bool {
        unsafe {
            let win_old = match win_old {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let win_new = match win_new {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSplitterWindow_ReplaceWindow(self.as_ptr(), win_old, win_new)
        }
    }
    fn set_minimum_pane_size(&self, pane_size: c_int) {
        unsafe { ffi::wxSplitterWindow_SetMinimumPaneSize(self.as_ptr(), pane_size) }
    }
    fn set_sash_gravity(&self, gravity: c_double) {
        unsafe { ffi::wxSplitterWindow_SetSashGravity(self.as_ptr(), gravity) }
    }
    fn set_sash_position(&self, position: c_int, redraw: bool) {
        unsafe { ffi::wxSplitterWindow_SetSashPosition(self.as_ptr(), position, redraw) }
    }
    fn set_split_mode(&self, mode: c_int) {
        unsafe { ffi::wxSplitterWindow_SetSplitMode(self.as_ptr(), mode) }
    }
    fn set_sash_invisible(&self, invisible: bool) {
        unsafe { ffi::wxSplitterWindow_SetSashInvisible(self.as_ptr(), invisible) }
    }
    fn split_horizontally<W: WindowMethods, W2: WindowMethods>(
        &self,
        window1: Option<&W>,
        window2: Option<&W2>,
        sash_position: c_int,
    ) -> bool {
        unsafe {
            let window1 = match window1 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let window2 = match window2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSplitterWindow_SplitHorizontally(self.as_ptr(), window1, window2, sash_position)
        }
    }
    fn split_vertically<W: WindowMethods, W2: WindowMethods>(
        &self,
        window1: Option<&W>,
        window2: Option<&W2>,
        sash_position: c_int,
    ) -> bool {
        unsafe {
            let window1 = match window1 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let window2 = match window2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSplitterWindow_SplitVertically(self.as_ptr(), window1, window2, sash_position)
        }
    }
    fn unsplit<W: WindowMethods>(&self, to_remove: Option<&W>) -> bool {
        unsafe {
            let to_remove = match to_remove {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSplitterWindow_Unsplit(self.as_ptr(), to_remove)
        }
    }
    fn update_size(&self) {
        unsafe { ffi::wxSplitterWindow_UpdateSize(self.as_ptr()) }
    }
}

// wxStaticBitmap
pub trait StaticBitmapMethods: ControlMethods {
    fn create_bitmapbundle<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &B,
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
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticBitmap_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxStaticBitmap_GetBitmap(self.as_ptr())) }
    }
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxStaticBitmap_GetIcon(self.as_ptr())) }
    }
    fn set_bitmap<B: BitmapBundleMethods>(&self, label: &B) {
        unsafe {
            let label = label.as_ptr();
            ffi::wxStaticBitmap_SetBitmap(self.as_ptr(), label)
        }
    }
    fn set_icon<I: IconMethods>(&self, label: &I) {
        unsafe {
            let label = label.as_ptr();
            ffi::wxStaticBitmap_SetIcon(self.as_ptr(), label)
        }
    }
    // NOT_SUPPORTED: fn SetScaleMode()
    // NOT_SUPPORTED: fn GetScaleMode()
}

// wxStaticBox
pub trait StaticBoxMethods: ControlMethods {
    // DTOR: fn ~wxStaticBox()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticBox_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    // BLOCKED: fn Create1()
}

// wxStaticBoxSizer
pub trait StaticBoxSizerMethods: BoxSizerMethods {
    fn get_static_box(&self) -> WeakRef<StaticBox> {
        unsafe { WeakRef::<StaticBox>::from(ffi::wxStaticBoxSizer_GetStaticBox(self.as_ptr())) }
    }
}

// wxStaticLine
pub trait StaticLineMethods: ControlMethods {
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxStaticLine_IsVertical(self.as_ptr()) }
    }
    fn get_default_size() -> c_int {
        unsafe { ffi::wxStaticLine_GetDefaultSize() }
    }
}

// wxStaticText
pub trait StaticTextMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticText_Create(self.as_ptr(), parent, id, label, pos, size, style, name)
        }
    }
    fn is_ellipsized(&self) -> bool {
        unsafe { ffi::wxStaticText_IsEllipsized(self.as_ptr()) }
    }
    fn wrap(&self, width: c_int) {
        unsafe { ffi::wxStaticText_Wrap(self.as_ptr(), width) }
    }
}

// wxStatusBar
pub trait StatusBarMethods: ControlMethods {
    // DTOR: fn ~wxStatusBar()
    fn create_long<W: WindowMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStatusBar_Create(self.as_ptr(), parent, id, style, name)
        }
    }
    fn get_field_rect<R: RectMethods>(&self, i: c_int, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxStatusBar_GetFieldRect(self.as_ptr(), i, rect)
        }
    }
    fn get_fields_count(&self) -> c_int {
        unsafe { ffi::wxStatusBar_GetFieldsCount(self.as_ptr()) }
    }
    fn get_field(&self, n: c_int) -> StatusBarPaneIsOwned<false> {
        unsafe { StatusBarPaneIsOwned::from_ptr(ffi::wxStatusBar_GetField(self.as_ptr(), n)) }
    }
    fn get_borders(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxStatusBar_GetBorders(self.as_ptr())) }
    }
    fn get_status_text(&self, i: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxStatusBar_GetStatusText(self.as_ptr(), i)).into() }
    }
    fn get_status_width(&self, n: c_int) -> c_int {
        unsafe { ffi::wxStatusBar_GetStatusWidth(self.as_ptr(), n) }
    }
    fn get_status_style(&self, n: c_int) -> c_int {
        unsafe { ffi::wxStatusBar_GetStatusStyle(self.as_ptr(), n) }
    }
    fn pop_status_text(&self, field: c_int) {
        unsafe { ffi::wxStatusBar_PopStatusText(self.as_ptr(), field) }
    }
    fn push_status_text(&self, string: &str, field: c_int) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxStatusBar_PushStatusText(self.as_ptr(), string, field)
        }
    }
    fn set_fields_count(&self, number: c_int, widths: *const c_void) {
        unsafe { ffi::wxStatusBar_SetFieldsCount(self.as_ptr(), number, widths) }
    }
    fn set_min_height(&self, height: c_int) {
        unsafe { ffi::wxStatusBar_SetMinHeight(self.as_ptr(), height) }
    }
    fn set_status_styles(&self, n: c_int, styles: *const c_void) {
        unsafe { ffi::wxStatusBar_SetStatusStyles(self.as_ptr(), n, styles) }
    }
    fn set_status_text(&self, text: &str, i: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxStatusBar_SetStatusText(self.as_ptr(), text, i)
        }
    }
    fn set_status_widths(&self, n: c_int, widths_field: *const c_void) {
        unsafe { ffi::wxStatusBar_SetStatusWidths(self.as_ptr(), n, widths_field) }
    }
}

// wxStatusBarPane
pub trait StatusBarPaneMethods: WxRustMethods {
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxStatusBarPane_GetWidth(self.as_ptr()) }
    }
    fn get_style(&self) -> c_int {
        unsafe { ffi::wxStatusBarPane_GetStyle(self.as_ptr()) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStatusBarPane_GetText(self.as_ptr())).into() }
    }
}

// wxStdDialogButtonSizer
pub trait StdDialogButtonSizerMethods: BoxSizerMethods {
    fn add_button<B: ButtonMethods>(&self, button: Option<&B>) {
        unsafe {
            let button = match button {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxStdDialogButtonSizer_AddButton(self.as_ptr(), button)
        }
    }
    fn realize(&self) {
        unsafe { ffi::wxStdDialogButtonSizer_Realize(self.as_ptr()) }
    }
    fn set_affirmative_button<B: ButtonMethods>(&self, button: Option<&B>) {
        unsafe {
            let button = match button {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxStdDialogButtonSizer_SetAffirmativeButton(self.as_ptr(), button)
        }
    }
    fn set_cancel_button<B: ButtonMethods>(&self, button: Option<&B>) {
        unsafe {
            let button = match button {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxStdDialogButtonSizer_SetCancelButton(self.as_ptr(), button)
        }
    }
    fn set_negative_button<B: ButtonMethods>(&self, button: Option<&B>) {
        unsafe {
            let button = match button {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxStdDialogButtonSizer_SetNegativeButton(self.as_ptr(), button)
        }
    }
}

// wxSysColourChangedEvent
pub trait SysColourChangedEventMethods: EventMethods {}

// wxSystemSettings
pub trait SystemSettingsMethods: WxRustMethods {
    // NOT_SUPPORTED: fn GetColour()
    // NOT_SUPPORTED: fn GetFont()
    // NOT_SUPPORTED: fn GetMetric()
    // NOT_SUPPORTED: fn GetScreenType()
    fn get_appearance() -> SystemAppearance {
        unsafe { SystemAppearance::from_ptr(ffi::wxSystemSettings_GetAppearance()) }
    }
    // NOT_SUPPORTED: fn HasFeature()
}

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

// wxTextCompleterSimple
pub trait TextCompleterSimpleMethods: TextCompleterMethods {
    fn get_completions<A: ArrayStringMethods>(&self, prefix: &str, res: &A) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            let res = res.as_ptr();
            ffi::wxTextCompleterSimple_GetCompletions(self.as_ptr(), prefix, res)
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
    fn enable_proof_check<T: TextProofOptionsMethods>(&self, options: &T) -> bool {
        unsafe {
            let options = options.as_ptr();
            ffi::wxTextCtrl_EnableProofCheck(self.as_ptr(), options)
        }
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
    fn get_proof_check_options(&self) -> TextProofOptions {
        unsafe { TextProofOptions::from_ptr(ffi::wxTextCtrl_GetProofCheckOptions(self.as_ptr())) }
    }
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
    fn on_drop_files(&self, event: *mut c_void) {
        unsafe { ffi::wxTextCtrl_OnDropFiles(self.as_ptr(), event) }
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

// wxTextEntry
pub trait TextEntryMethods: WxRustMethods {
    fn as_text_entry(&self) -> *mut c_void;
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
    fn auto_complete_textcompleter<T: TextCompleterMethods>(&self, completer: Option<&T>) -> bool {
        unsafe {
            let completer = match completer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTextEntry_AutoComplete1(self.as_text_entry(), completer)
        }
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
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
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
            ToolBarToolBase::option_from(ffi::wxToolBar_AddCheckTool(
                self.as_ptr(),
                tool_id,
                label,
                bitmap1,
                bmp_disabled,
                short_help,
                long_help,
                client_data,
            ))
        }
    }
    fn add_control<C: ControlMethods>(
        &self,
        control: Option<&C>,
        label: &str,
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            ToolBarToolBase::option_from(ffi::wxToolBar_AddControl(self.as_ptr(), control, label))
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
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
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
            ToolBarToolBase::option_from(ffi::wxToolBar_AddRadioTool(
                self.as_ptr(),
                tool_id,
                label,
                bitmap1,
                bmp_disabled,
                short_help,
                long_help,
                client_data,
            ))
        }
    }
    fn add_separator(&self) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_AddSeparator(self.as_ptr())) }
    }
    fn add_stretchable_space(&self) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_AddStretchableSpace(self.as_ptr())) }
    }
    fn add_tool_toolbartoolbase<T: ToolBarToolBaseMethods>(
        &self,
        tool: Option<&T>,
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            let tool = match tool {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ToolBarToolBase::option_from(ffi::wxToolBar_AddTool(self.as_ptr(), tool))
        }
    }
    fn add_tool_int_str<B: BitmapBundleMethods>(
        &self,
        tool_id: c_int,
        label: &str,
        bitmap: &B,
        short_help: &str,
        kind: c_int,
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let bitmap = bitmap.as_ptr();
            let short_help = WxString::from(short_help);
            let short_help = short_help.as_ptr();
            ToolBarToolBase::option_from(ffi::wxToolBar_AddTool1(
                self.as_ptr(),
                tool_id,
                label,
                bitmap,
                short_help,
                kind,
            ))
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
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
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
            ToolBarToolBase::option_from(ffi::wxToolBar_AddTool2(
                self.as_ptr(),
                tool_id,
                label,
                bitmap,
                bmp_disabled,
                kind,
                short_help,
                long_help,
                client_data,
            ))
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
    fn find_by_id(&self, id: c_int) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_FindById(self.as_ptr(), id)) }
    }
    fn find_control(&self, id: c_int) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxToolBar_FindControl(self.as_ptr(), id)) }
    }
    fn find_tool_for_position(&self, x: c_int, y: c_int) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            ToolBarToolBase::option_from(ffi::wxToolBar_FindToolForPosition(self.as_ptr(), x, y))
        }
    }
    fn get_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetMargins(self.as_ptr())) }
    }
    fn get_tool_bitmap_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxToolBar_GetToolBitmapSize(self.as_ptr())) }
    }
    // BLOCKED: fn GetToolByPos()
    fn get_tool_by_pos(&self, pos: c_int) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_GetToolByPos1(self.as_ptr(), pos)) }
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
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            ToolBarToolBase::option_from(ffi::wxToolBar_InsertControl(
                self.as_ptr(),
                pos,
                control,
                label,
            ))
        }
    }
    fn insert_separator(&self, pos: usize) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_InsertSeparator(self.as_ptr(), pos)) }
    }
    fn insert_stretchable_space(&self, pos: usize) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            ToolBarToolBase::option_from(ffi::wxToolBar_InsertStretchableSpace(self.as_ptr(), pos))
        }
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
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
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
            ToolBarToolBase::option_from(ffi::wxToolBar_InsertTool(
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
            ))
        }
    }
    fn insert_tool_toolbartoolbase<T: ToolBarToolBaseMethods>(
        &self,
        pos: usize,
        tool: Option<&T>,
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            let tool = match tool {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ToolBarToolBase::option_from(ffi::wxToolBar_InsertTool1(self.as_ptr(), pos, tool))
        }
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
    fn remove_tool(&self, id: c_int) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_RemoveTool(self.as_ptr(), id)) }
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
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
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
            ToolBarToolBase::option_from(ffi::wxToolBar_CreateTool(
                self.as_ptr(),
                tool_id,
                label,
                bmp_normal,
                bmp_disabled,
                kind,
                client_data,
                short_help,
                long_help,
            ))
        }
    }
    fn create_tool_control<C: ControlMethods>(
        &self,
        control: Option<&C>,
        label: &str,
    ) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe {
            let control = match control {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            ToolBarToolBase::option_from(ffi::wxToolBar_CreateTool1(self.as_ptr(), control, label))
        }
    }
    fn create_separator(&self) -> Option<ToolBarToolBaseIsOwned<false>> {
        unsafe { ToolBarToolBase::option_from(ffi::wxToolBar_CreateSeparator(self.as_ptr())) }
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
    fn append_item<C: ClientDataMethods>(
        &self,
        parent: ffi::wxTreeListItem,
        text: &str,
        image_closed: c_int,
        image_opened: c_int,
        data: Option<&C>,
    ) -> TreeListItem {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeListItem::from_ptr(ffi::wxTreeListCtrl_AppendItem(
                self.as_ptr(),
                parent,
                text,
                image_closed,
                image_opened,
                data,
            ))
        }
    }
    fn insert_item<C: ClientDataMethods>(
        &self,
        parent: ffi::wxTreeListItem,
        previous: ffi::wxTreeListItem,
        text: &str,
        image_closed: c_int,
        image_opened: c_int,
        data: Option<&C>,
    ) -> TreeListItem {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeListItem::from_ptr(ffi::wxTreeListCtrl_InsertItem(
                self.as_ptr(),
                parent,
                previous,
                text,
                image_closed,
                image_opened,
                data,
            ))
        }
    }
    fn prepend_item<C: ClientDataMethods>(
        &self,
        parent: ffi::wxTreeListItem,
        text: &str,
        image_closed: c_int,
        image_opened: c_int,
        data: Option<&C>,
    ) -> TreeListItem {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TreeListItem::from_ptr(ffi::wxTreeListCtrl_PrependItem(
                self.as_ptr(),
                parent,
                text,
                image_closed,
                image_opened,
                data,
            ))
        }
    }
    fn delete_item(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_DeleteItem(self.as_ptr(), item) }
    }
    fn delete_all_items(&self) {
        unsafe { ffi::wxTreeListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn get_root_item(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetRootItem(self.as_ptr())) }
    }
    fn get_item_parent(&self, item: ffi::wxTreeListItem) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetItemParent(self.as_ptr(), item)) }
    }
    fn get_first_child(&self, item: ffi::wxTreeListItem) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetFirstChild(self.as_ptr(), item)) }
    }
    fn get_next_sibling_treelistitem(&self, item: ffi::wxTreeListItem) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetNextSibling(self.as_ptr(), item)) }
    }
    fn get_first_item(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetFirstItem(self.as_ptr())) }
    }
    fn get_next_item(&self, item: ffi::wxTreeListItem) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetNextItem(self.as_ptr(), item)) }
    }
    // NOT_SUPPORTED: fn GetItemText()
    // NOT_SUPPORTED: fn SetItemText()
    fn set_item_text(&self, item: ffi::wxTreeListItem, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxTreeListCtrl_SetItemText1(self.as_ptr(), item, text)
        }
    }
    fn set_item_image(&self, item: ffi::wxTreeListItem, closed: c_int, opened: c_int) {
        unsafe { ffi::wxTreeListCtrl_SetItemImage(self.as_ptr(), item, closed, opened) }
    }
    fn get_item_data(&self, item: ffi::wxTreeListItem) -> Option<ClientDataIsOwned<false>> {
        unsafe { ClientData::option_from(ffi::wxTreeListCtrl_GetItemData(self.as_ptr(), item)) }
    }
    fn set_item_data<C: ClientDataMethods>(&self, item: ffi::wxTreeListItem, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTreeListCtrl_SetItemData(self.as_ptr(), item, data)
        }
    }
    fn expand(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_Expand(self.as_ptr(), item) }
    }
    fn collapse(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_Collapse(self.as_ptr(), item) }
    }
    fn is_expanded(&self, item: ffi::wxTreeListItem) -> bool {
        unsafe { ffi::wxTreeListCtrl_IsExpanded(self.as_ptr(), item) }
    }
    fn get_selection(&self) -> TreeListItem {
        unsafe { TreeListItem::from_ptr(ffi::wxTreeListCtrl_GetSelection(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetSelections()
    fn select(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_Select(self.as_ptr(), item) }
    }
    fn unselect(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_Unselect(self.as_ptr(), item) }
    }
    fn is_selected(&self, item: ffi::wxTreeListItem) -> bool {
        unsafe { ffi::wxTreeListCtrl_IsSelected(self.as_ptr(), item) }
    }
    fn select_all(&self) {
        unsafe { ffi::wxTreeListCtrl_SelectAll(self.as_ptr()) }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxTreeListCtrl_UnselectAll(self.as_ptr()) }
    }
    fn ensure_visible(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_EnsureVisible(self.as_ptr(), item) }
    }
    fn check_item(&self, item: ffi::wxTreeListItem, state: c_int) {
        unsafe { ffi::wxTreeListCtrl_CheckItem(self.as_ptr(), item, state) }
    }
    fn check_item_recursively(&self, item: ffi::wxTreeListItem, state: c_int) {
        unsafe { ffi::wxTreeListCtrl_CheckItemRecursively(self.as_ptr(), item, state) }
    }
    fn uncheck_item(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_UncheckItem(self.as_ptr(), item) }
    }
    fn update_item_parent_state_recursively(&self, item: ffi::wxTreeListItem) {
        unsafe { ffi::wxTreeListCtrl_UpdateItemParentStateRecursively(self.as_ptr(), item) }
    }
    fn get_checked_state(&self, item: ffi::wxTreeListItem) -> c_int {
        unsafe { ffi::wxTreeListCtrl_GetCheckedState(self.as_ptr(), item) }
    }
    fn are_all_children_in_state(&self, item: ffi::wxTreeListItem, state: c_int) -> bool {
        unsafe { ffi::wxTreeListCtrl_AreAllChildrenInState(self.as_ptr(), item, state) }
    }
    // NOT_SUPPORTED: fn SetSortColumn()
    fn get_sort_column(&self, col: *mut c_void, ascending_order: *mut c_void) -> bool {
        unsafe { ffi::wxTreeListCtrl_GetSortColumn(self.as_ptr(), col, ascending_order) }
    }
    fn set_item_comparator(&self, comparator: *mut c_void) {
        unsafe { ffi::wxTreeListCtrl_SetItemComparator(self.as_ptr(), comparator) }
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

// wxTwoFingerTapEvent
pub trait TwoFingerTapEventMethods: GestureEventMethods {}

// wxUIActionSimulator
pub trait UIActionSimulatorMethods: WxRustMethods {
    fn mouse_move_long(&self, x: c_long, y: c_long) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseMove(self.as_ptr(), x, y) }
    }
    fn mouse_move_point<P: PointMethods>(&self, point: &P) -> bool {
        unsafe {
            let point = point.as_ptr();
            ffi::wxUIActionSimulator_MouseMove1(self.as_ptr(), point)
        }
    }
    fn mouse_down(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDown(self.as_ptr(), button) }
    }
    fn mouse_up(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseUp(self.as_ptr(), button) }
    }
    fn mouse_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseClick(self.as_ptr(), button) }
    }
    fn mouse_dbl_click(&self, button: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDblClick(self.as_ptr(), button) }
    }
    fn mouse_drag_drop(
        &self,
        x1: c_long,
        y1: c_long,
        x2: c_long,
        y2: c_long,
        button: c_int,
    ) -> bool {
        unsafe { ffi::wxUIActionSimulator_MouseDragDrop(self.as_ptr(), x1, y1, x2, y2, button) }
    }
    fn key_down(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyDown(self.as_ptr(), keycode, modifiers) }
    }
    fn key_up(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_KeyUp(self.as_ptr(), keycode, modifiers) }
    }
    fn char(&self, keycode: c_int, modifiers: c_int) -> bool {
        unsafe { ffi::wxUIActionSimulator_Char(self.as_ptr(), keycode, modifiers) }
    }
    fn select(&self, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUIActionSimulator_Select(self.as_ptr(), text)
        }
    }
    fn text(&self, text: *const c_void) -> bool {
        unsafe { ffi::wxUIActionSimulator_Text(self.as_ptr(), text) }
    }
}

// wxUpdateUIEvent
pub trait UpdateUIEventMethods: CommandEventMethods {
    fn check(&self, check: bool) {
        unsafe { ffi::wxUpdateUIEvent_Check(self.as_ptr(), check) }
    }
    fn enable(&self, enable: bool) {
        unsafe { ffi::wxUpdateUIEvent_Enable(self.as_ptr(), enable) }
    }
    fn get_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetChecked(self.as_ptr()) }
    }
    fn get_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetEnabled(self.as_ptr()) }
    }
    fn is_checkable(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_IsCheckable(self.as_ptr()) }
    }
    fn get_set_checked(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetChecked(self.as_ptr()) }
    }
    fn get_set_enabled(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetEnabled(self.as_ptr()) }
    }
    fn get_set_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetShown(self.as_ptr()) }
    }
    fn get_set_text(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetSetText(self.as_ptr()) }
    }
    fn get_shown(&self) -> bool {
        unsafe { ffi::wxUpdateUIEvent_GetShown(self.as_ptr()) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxUpdateUIEvent_GetText(self.as_ptr())).into() }
    }
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxUpdateUIEvent_SetText(self.as_ptr(), text)
        }
    }
    fn show(&self, show: bool) {
        unsafe { ffi::wxUpdateUIEvent_Show(self.as_ptr(), show) }
    }
    fn can_update<W: WindowMethods>(window: Option<&W>) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxUpdateUIEvent_CanUpdate(window)
        }
    }
    // NOT_SUPPORTED: fn GetMode()
    fn get_update_interval() -> c_long {
        unsafe { ffi::wxUpdateUIEvent_GetUpdateInterval() }
    }
    fn reset_update_time() {
        unsafe { ffi::wxUpdateUIEvent_ResetUpdateTime() }
    }
    // NOT_SUPPORTED: fn SetMode()
    fn set_update_interval(update_interval: c_long) {
        unsafe { ffi::wxUpdateUIEvent_SetUpdateInterval(update_interval) }
    }
}

// wxValidator
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    fn clone(&self) -> Object {
        unsafe { Object::from_ptr(ffi::wxValidator_Clone(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxValidator_GetWindow(self.as_ptr())) }
    }
    fn set_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.as_ptr(), window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr()) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr()) }
    }
    fn validate<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.as_ptr(), parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}

// wxWindow
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr()) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr()) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr()) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr()) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr()) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr()) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr()) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr()) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr(), can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr(), enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr()) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr()) }
    }
    fn add_child<W: WindowMethods>(&self, child: Option<&W>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.as_ptr(), child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr()) }
    }
    fn find_window_long(&self, id: c_long) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindWindow(self.as_ptr(), id)) }
    }
    fn find_window_str(&self, name: &str) -> WeakRef<Window> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WeakRef::<Window>::from(ffi::wxWindow_FindWindow1(self.as_ptr(), name))
        }
    }
    // BLOCKED: fn GetChildren()
    fn get_children(&self) -> WindowListIsOwned<false> {
        unsafe { WindowListIsOwned::from_ptr(ffi::wxWindow_GetChildren1(self.as_ptr())) }
    }
    fn remove_child<W: WindowMethods>(&self, child: Option<&W>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.as_ptr(), child)
        }
    }
    fn get_grand_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetGrandParent(self.as_ptr())) }
    }
    fn get_next_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetNextSibling(self.as_ptr())) }
    }
    fn get_parent(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetParent(self.as_ptr())) }
    }
    fn get_prev_sibling(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetPrevSibling(self.as_ptr())) }
    }
    fn is_descendant<W: WindowMethods>(&self, win: Option<&W>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(self.as_ptr(), win)
        }
    }
    fn reparent<W: WindowMethods>(&self, new_parent: Option<&W>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.as_ptr(), new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr(), hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr(), orientation) }
    }
    fn get_scroll_range(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr(), orientation) }
    }
    fn get_scroll_thumb(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr(), orientation) }
    }
    fn can_scroll(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr(), orient) }
    }
    fn has_scrollbar(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr(), orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr(), orient) }
    }
    fn scroll_lines(&self, lines: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr(), lines) }
    }
    fn scroll_pages(&self, pages: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr(), pages) }
    }
    fn scroll_window<R: RectMethods>(&self, dx: c_int, dy: c_int, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ScrollWindow(self.as_ptr(), dx, dy, rect)
        }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr()) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr()) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr()) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr()) }
    }
    fn set_scroll_pos(&self, orientation: c_int, pos: c_int, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr(), orientation, pos, refresh) }
    }
    fn set_scrollbar(
        &self,
        orientation: c_int,
        position: c_int,
        thumb_size: c_int,
        range: c_int,
        refresh: bool,
    ) {
        unsafe {
            ffi::wxWindow_SetScrollbar(
                self.as_ptr(),
                orientation,
                position,
                thumb_size,
                range,
                refresh,
            )
        }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr()) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr()) }
    }
    fn cache_best_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_CacheBestSize(self.as_ptr(), size)
        }
    }
    fn client_to_window_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size::from_ptr(ffi::wxWindow_ClientToWindowSize(self.as_ptr(), size))
        }
    }
    fn window_to_client_size<S: SizeMethods>(&self, size: &S) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size::from_ptr(ffi::wxWindow_WindowToClientSize(self.as_ptr(), size))
        }
    }
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr()) }
    }
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr()) }
    }
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_FromDIP(self.as_ptr(), sz))
        }
    }
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_FromDIP1(self.as_ptr(), pt))
        }
    }
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr(), d) }
    }
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ToDIP(self.as_ptr(), sz))
        }
    }
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ToDIP1(self.as_ptr(), pt))
        }
    }
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr(), d) }
    }
    fn from_phys_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_FromPhys(self.as_ptr(), sz))
        }
    }
    fn from_phys_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_FromPhys1(self.as_ptr(), pt))
        }
    }
    fn from_phys_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromPhys2(self.as_ptr(), d) }
    }
    fn to_phys_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ToPhys(self.as_ptr(), sz))
        }
    }
    fn to_phys_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ToPhys1(self.as_ptr(), pt))
        }
    }
    fn to_phys_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToPhys2(self.as_ptr(), d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetBestSize(self.as_ptr())) }
    }
    fn get_best_height(&self, width: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr(), width) }
    }
    fn get_best_width(&self, height: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr(), height) }
    }
    fn get_client_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr(), width, height) }
    }
    fn get_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetClientSize1(self.as_ptr())) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr())) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMaxClientSize(self.as_ptr())) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMaxSize(self.as_ptr())) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMinClientSize(self.as_ptr())) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetMinSize(self.as_ptr())) }
    }
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr()) }
    }
    fn get_min_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr()) }
    }
    fn get_max_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr()) }
    }
    fn get_max_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr()) }
    }
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetSize1(self.as_ptr())) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetVirtualSize(self.as_ptr())) }
    }
    fn get_virtual_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr(), width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetBestVirtualSize(self.as_ptr())) }
    }
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr()) }
    }
    fn get_dpi_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr()) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetWindowBorderSize(self.as_ptr())) }
    }
    fn inform_first_direction(
        &self,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool {
        unsafe {
            ffi::wxWindow_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir)
        }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr()) }
    }
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr()) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr()) }
    }
    fn send_size_event(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr(), flags) }
    }
    fn send_size_event_to_parent(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr(), flags) }
    }
    fn set_client_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr(), width, height) }
    }
    fn set_client_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetClientSize1(self.as_ptr(), size)
        }
    }
    fn set_client_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetClientSize2(self.as_ptr(), rect)
        }
    }
    fn set_containing_sizer<S: SizerMethods>(&self, sizer: Option<&S>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetContainingSizer(self.as_ptr(), sizer)
        }
    }
    fn set_initial_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetInitialSize(self.as_ptr(), size)
        }
    }
    fn set_max_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxClientSize(self.as_ptr(), size)
        }
    }
    fn set_max_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxSize(self.as_ptr(), size)
        }
    }
    fn set_min_client_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinClientSize(self.as_ptr(), size)
        }
    }
    fn set_min_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_size_int_int(&self, x: c_int, y: c_int, width: c_int, height: c_int, size_flags: c_int) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr(), x, y, width, height, size_flags) }
    }
    fn set_size_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetSize1(self.as_ptr(), rect)
        }
    }
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetSize2(self.as_ptr(), size)
        }
    }
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr(), width, height) }
    }
    fn set_size_hints_size<S: SizeMethods, S2: SizeMethods, S3: SizeMethods>(
        &self,
        min_size: &S,
        max_size: &S2,
        inc_size: &S3,
    ) {
        unsafe {
            let min_size = min_size.as_ptr();
            let max_size = max_size.as_ptr();
            let inc_size = inc_size.as_ptr();
            ffi::wxWindow_SetSizeHints(self.as_ptr(), min_size, max_size, inc_size)
        }
    }
    fn set_size_hints_int(
        &self,
        min_w: c_int,
        min_h: c_int,
        max_w: c_int,
        max_h: c_int,
        inc_w: c_int,
        inc_h: c_int,
    ) {
        unsafe {
            ffi::wxWindow_SetSizeHints1(self.as_ptr(), min_w, min_h, max_w, max_h, inc_w, inc_h)
        }
    }
    fn set_virtual_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr(), width, height) }
    }
    fn set_virtual_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetVirtualSize1(self.as_ptr(), size)
        }
    }
    fn from_dip_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    fn from_phys_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_FromPhys3(sz, w))
        }
    }
    fn from_phys_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_FromPhys4(pt, w))
        }
    }
    fn from_phys_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromPhys5(d, w)
        }
    }
    fn to_phys_size_window<S: SizeMethods, W: WindowMethods>(sz: &S, w: Option<&W>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size::from_ptr(ffi::wxWindow_ToPhys3(sz, w))
        }
    }
    fn to_phys_point_window<P: PointMethods, W: WindowMethods>(pt: &P, w: Option<&W>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point::from_ptr(ffi::wxWindow_ToPhys4(pt, w))
        }
    }
    fn to_phys_int_window<W: WindowMethods>(d: c_int, w: Option<&W>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToPhys5(d, w)
        }
    }
    fn center(&self, dir: c_int) {
        unsafe { ffi::wxWindow_Center(self.as_ptr(), dir) }
    }
    fn center_on_parent(&self, dir: c_int) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr(), dir) }
    }
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr(), direction) }
    }
    fn centre_on_parent(&self, direction: c_int) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr(), direction) }
    }
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr(), x, y) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetPosition1(self.as_ptr())) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetRect(self.as_ptr())) }
    }
    fn get_screen_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr(), x, y) }
    }
    fn get_screen_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetScreenPosition1(self.as_ptr())) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetScreenRect(self.as_ptr())) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr())) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetClientRect(self.as_ptr())) }
    }
    fn move_int(&self, x: c_int, y: c_int, flags: c_int) {
        unsafe { ffi::wxWindow_Move(self.as_ptr(), x, y, flags) }
    }
    fn move_point<P: PointMethods>(&self, pt: &P, flags: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_Move1(self.as_ptr(), pt, flags)
        }
    }
    fn set_position<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_SetPosition(self.as_ptr(), pt)
        }
    }
    fn client_to_screen_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr(), x, y) }
    }
    fn client_to_screen_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ClientToScreen1(self.as_ptr(), pt))
        }
    }
    fn convert_dialog_to_pixels_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr(), pt))
        }
    }
    fn convert_dialog_to_pixels_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr(), sz))
        }
    }
    fn convert_pixels_to_dialog_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr(), pt))
        }
    }
    fn convert_pixels_to_dialog_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr(), sz))
        }
    }
    fn screen_to_client_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr(), x, y) }
    }
    fn screen_to_client_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxWindow_ScreenToClient1(self.as_ptr(), pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr()) }
    }
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr()) }
    }
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr()) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr()) }
    }
    fn get_background_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxWindow_GetBackgroundColour(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr()) }
    }
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxWindow_GetDPI(self.as_ptr())) }
    }
    fn get_font(&self) -> Font {
        unsafe { Font::from_ptr(ffi::wxWindow_GetFont(self.as_ptr())) }
    }
    fn get_foreground_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxWindow_GetForegroundColour(self.as_ptr())) }
    }
    fn get_text_extent_int<F: FontMethods>(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: Option<&F>,
    ) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            let font = match font {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_GetTextExtent(
                self.as_ptr(),
                string,
                w,
                h,
                descent,
                external_leading,
                font,
            )
        }
    }
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxWindow_GetTextExtent1(self.as_ptr(), string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxWindow_GetUpdateClientRect(self.as_ptr())) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr()) }
    }
    fn refresh<R: RectMethods>(&self, erase_background: bool, rect: Option<&R>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Refresh(self.as_ptr(), erase_background, rect)
        }
    }
    fn refresh_rect<R: RectMethods>(&self, rect: &R, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_RefreshRect(self.as_ptr(), rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr(), reason) }
    }
    fn set_font<F: FontMethods>(&self, font: &F) -> bool {
        unsafe {
            let font = font.as_ptr();
            ffi::wxWindow_SetFont(self.as_ptr(), font)
        }
    }
    fn set_foreground_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetForegroundColour(self.as_ptr(), colour)
        }
    }
    fn set_own_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr()) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr()) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr()) }
    }
    fn set_own_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxWindow_SetOwnFont(self.as_ptr(), font)
        }
    }
    fn set_own_foreground_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxWindow_SetOwnForegroundColour(self.as_ptr(), colour)
        }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr()) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr()) }
    }
    fn set_palette<P: PaletteMethods>(&self, pal: &P) {
        unsafe {
            let pal = pal.as_ptr();
            ffi::wxWindow_SetPalette(self.as_ptr(), pal)
        }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr()) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr(), enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr()) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr()) }
    }
    fn set_transparent(&self, alpha: c_uchar) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr(), alpha) }
    }
    fn get_event_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxWindow_GetEventHandler(self.as_ptr())) }
    }
    fn handle_as_navigation_key<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_HandleAsNavigationKey(self.as_ptr(), event)
        }
    }
    fn handle_window_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_HandleWindowEvent(self.as_ptr(), event)
        }
    }
    fn process_window_event<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_ProcessWindowEvent(self.as_ptr(), event)
        }
    }
    fn process_window_event_locally<E: EventMethods>(&self, event: &E) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr(), event)
        }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> WeakRef<EvtHandler> {
        unsafe {
            WeakRef::<EvtHandler>::from(ffi::wxWindow_PopEventHandler(
                self.as_ptr(),
                delete_handler,
            ))
        }
    }
    fn push_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.as_ptr(), handler)
        }
    }
    fn remove_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.as_ptr(), handler)
        }
    }
    fn set_event_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.as_ptr(), handler)
        }
    }
    fn get_extra_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr()) }
    }
    fn get_window_style_flag(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr()) }
    }
    fn get_window_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr()) }
    }
    fn has_extra_style(&self, ex_flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr(), ex_flag) }
    }
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr(), flag) }
    }
    fn set_extra_style(&self, ex_style: c_long) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr(), ex_style) }
    }
    fn set_window_style_flag(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr(), style) }
    }
    fn set_window_style(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr(), style) }
    }
    fn toggle_window_style(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr(), flag) }
    }
    fn move_after_in_tab_order<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.as_ptr(), win)
        }
    }
    fn move_before_in_tab_order<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.as_ptr(), win)
        }
    }
    fn navigate(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr(), flags) }
    }
    fn navigate_in(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr(), flags) }
    }
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr()) }
    }
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr()) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr()) }
    }
    fn is_exposed_int(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr(), x, y) }
    }
    fn is_exposed_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_IsExposed1(self.as_ptr(), pt)
        }
    }
    fn is_exposed_int_int(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr(), x, y, w, h) }
    }
    fn is_exposed_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_IsExposed3(self.as_ptr(), rect)
        }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr()) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr()) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr()) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr(), enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr(), show) }
    }
    // NOT_SUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetHelpText(self.as_ptr())).into() }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = WxString::from(help_text);
            let help_text = help_text.as_ptr();
            ffi::wxWindow_SetHelpText(self.as_ptr(), help_text)
        }
    }
    // NOT_SUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetToolTip(self.as_ptr()) }
    }
    fn get_tool_tip_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetToolTipText(self.as_ptr())).into() }
    }
    fn set_tool_tip_str(&self, tip_string: &str) {
        unsafe {
            let tip_string = WxString::from(tip_string);
            let tip_string = tip_string.as_ptr();
            ffi::wxWindow_SetToolTip(self.as_ptr(), tip_string)
        }
    }
    fn set_tool_tip_tooltip(&self, tip: *mut c_void) {
        unsafe { ffi::wxWindow_SetToolTip1(self.as_ptr(), tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr()) }
    }
    fn get_popup_menu_selection_from_user_point<M: MenuMethods, P: PointMethods>(
        &self,
        menu: &M,
        pos: &P,
    ) -> c_int {
        unsafe {
            let menu = menu.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr(), menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user_int<M: MenuMethods>(
        &self,
        menu: &M,
        x: c_int,
        y: c_int,
    ) -> c_int {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr(), menu, x, y)
        }
    }
    fn popup_menu_point<M: MenuMethods, P: PointMethods>(&self, menu: Option<&M>, pos: &P) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxWindow_PopupMenu(self.as_ptr(), menu, pos)
        }
    }
    fn popup_menu_int<M: MenuMethods>(&self, menu: Option<&M>, x: c_int, y: c_int) -> bool {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PopupMenu1(self.as_ptr(), menu, x, y)
        }
    }
    fn get_validator(&self) -> WeakRef<Validator> {
        unsafe { WeakRef::<Validator>::from(ffi::wxWindow_GetValidator(self.as_ptr())) }
    }
    fn set_validator<V: ValidatorMethods>(&self, validator: &V) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxWindow_SetValidator(self.as_ptr(), validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr()) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr()) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr()) }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxWindow_GetId(self.as_ptr()) }
    }
    fn get_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetLabel(self.as_ptr())).into() }
    }
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxWindow_GetLayoutDirection(self.as_ptr()) }
    }
    fn adjust_for_layout_direction(&self, x: c_int, width: c_int, width_total: c_int) -> c_int {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr(), x, width, width_total) }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxWindow_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: c_int) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr(), winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxWindow_SetLabel(self.as_ptr(), label)
        }
    }
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxWindow_SetLayoutDirection(self.as_ptr(), dir) }
    }
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxWindow_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetAcceleratorTable(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: *const c_void) {
        unsafe { ffi::wxWindow_SetAcceleratorTable(self.as_ptr(), accel) }
    }
    // NOT_SUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr(), force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr()) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr()) }
    }
    fn get_drop_target(&self) -> Option<DropTargetIsOwned<false>> {
        unsafe { DropTarget::option_from(ffi::wxWindow_GetDropTarget(self.as_ptr())) }
    }
    fn set_drop_target<D: DropTargetMethods>(&self, target: Option<&D>) {
        unsafe {
            let target = match target {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetDropTarget(self.as_ptr(), target)
        }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr(), accept) }
    }
    fn get_containing_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetContainingSizer(self.as_ptr())) }
    }
    fn get_sizer(&self) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxWindow_GetSizer(self.as_ptr())) }
    }
    fn set_sizer<S: SizerMethods>(&self, sizer: Option<&S>, delete_old: bool) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetSizer(self.as_ptr(), sizer, delete_old)
        }
    }
    fn set_sizer_and_fit<S: SizerMethods>(&self, sizer: Option<&S>, delete_old: bool) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetSizerAndFit(self.as_ptr(), sizer, delete_old)
        }
    }
    fn get_constraints(&self) -> Option<LayoutConstraintsIsOwned<false>> {
        unsafe { LayoutConstraints::option_from(ffi::wxWindow_GetConstraints(self.as_ptr())) }
    }
    fn set_constraints<L: LayoutConstraintsMethods>(&self, constraints: Option<&L>) {
        unsafe {
            let constraints = match constraints {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetConstraints(self.as_ptr(), constraints)
        }
    }
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr()) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr(), auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr()) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr()) }
    }
    fn get_caret(&self) -> Option<CaretIsOwned<false>> {
        unsafe { Caret::option_from(ffi::wxWindow_GetCaret(self.as_ptr())) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr()) }
    }
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr()) }
    }
    fn set_caret<C: CaretMethods>(&self, caret: Option<&C>) {
        unsafe {
            let caret = match caret {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetCaret(self.as_ptr(), caret)
        }
    }
    fn set_cursor<C: CursorMethods>(&self, cursor: &C) -> bool {
        unsafe {
            let cursor = cursor.as_ptr();
            ffi::wxWindow_SetCursor(self.as_ptr(), cursor)
        }
    }
    fn warp_pointer(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr(), x, y) }
    }
    fn enable_touch_events(&self, events_mask: c_int) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr(), events_mask) }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    // NOT_SUPPORTED: fn GetBorder()
    // NOT_SUPPORTED: fn GetBorder1()
    fn do_update_window_ui<U: UpdateUIEventMethods>(&self, event: &U) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_DoUpdateWindowUI(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr()) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr()) }
    }
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr()) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr()) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr(), on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr()) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr()) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr()) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr()) }
    }
    fn send_idle_events<I: IdleEventMethods>(&self, event: &I) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxWindow_SendIdleEvents(self.as_ptr(), event)
        }
    }
    fn register_hot_key(
        &self,
        hotkey_id: c_int,
        modifiers: c_int,
        virtual_key_code: c_int,
    ) -> bool {
        unsafe {
            ffi::wxWindow_RegisterHotKey(self.as_ptr(), hotkey_id, modifiers, virtual_key_code)
        }
    }
    fn unregister_hot_key(&self, hotkey_id: c_int) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr(), hotkey_id) }
    }
    fn update_window_ui(&self, flags: c_long) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_FindFocus()) }
    }
    fn find_window_by_id<W: WindowMethods>(id: c_long, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowById(id, parent))
        }
    }
    fn find_window_by_label<W: WindowMethods>(label: &str, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowByLabel(label, parent))
        }
    }
    fn find_window_by_name<W: WindowMethods>(name: &str, parent: Option<&W>) -> WeakRef<Window> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WeakRef::<Window>::from(ffi::wxWindow_FindWindowByName(name, parent))
        }
    }
    fn get_capture() -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindow_GetCapture()) }
    }
    fn new_control_id(count: c_int) -> c_int {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    fn unreserve_control_id(id: c_int, count: c_int) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxWindowCreateEvent
pub trait WindowCreateEventMethods: CommandEventMethods {
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindowCreateEvent_GetWindow(self.as_ptr())) }
    }
}

// wxWindowDC
pub trait WindowDCMethods: DCMethods {}

// wxWindowDestroyEvent
pub trait WindowDestroyEventMethods: CommandEventMethods {
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxWindowDestroyEvent_GetWindow(self.as_ptr())) }
    }
}

// wxWindowDisabler
pub trait WindowDisablerMethods: WxRustMethods {
    // DTOR: fn ~wxWindowDisabler()
}

// wxWizardEvent
pub trait WizardEventMethods: NotifyEventMethods {
    fn get_direction(&self) -> bool {
        unsafe { ffi::wxWizardEvent_GetDirection(self.as_ptr()) }
    }
    fn get_page(&self) -> *mut c_void {
        unsafe { ffi::wxWizardEvent_GetPage(self.as_ptr()) }
    }
}

// wxWrapSizer
pub trait WrapSizerMethods: BoxSizerMethods {}

// wxZoomGestureEvent
pub trait ZoomGestureEventMethods: GestureEventMethods {
    fn get_zoom_factor(&self) -> c_double {
        unsafe { ffi::wxZoomGestureEvent_GetZoomFactor(self.as_ptr()) }
    }
    fn set_zoom_factor(&self, zoom_factor: c_double) {
        unsafe { ffi::wxZoomGestureEvent_SetZoomFactor(self.as_ptr(), zoom_factor) }
    }
}
