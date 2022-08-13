use super::*;

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

// wxAcceleratorTable
pub trait AcceleratorTableMethods: ObjectMethods {
    // DTOR: fn ~wxAcceleratorTable()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxAcceleratorTable_IsOk(self.as_ptr()) }
    }
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

// wxAddRemoveAdaptor
pub trait AddRemoveAdaptorMethods: WxRustMethods {
    // DTOR: fn ~wxAddRemoveAdaptor()
    fn get_items_ctrl(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxAddRemoveAdaptor_GetItemsCtrl(self.as_ptr())) }
    }
    fn can_add(&self) -> bool {
        unsafe { ffi::wxAddRemoveAdaptor_CanAdd(self.as_ptr()) }
    }
    fn can_remove(&self) -> bool {
        unsafe { ffi::wxAddRemoveAdaptor_CanRemove(self.as_ptr()) }
    }
    fn on_add(&self) {
        unsafe { ffi::wxAddRemoveAdaptor_OnAdd(self.as_ptr()) }
    }
    fn on_remove(&self) {
        unsafe { ffi::wxAddRemoveAdaptor_OnRemove(self.as_ptr()) }
    }
}

// wxAddRemoveCtrl
pub trait AddRemoveCtrlMethods: PanelMethods {
    fn set_adaptor<A: AddRemoveAdaptorMethods>(&self, adaptor: Option<&A>) {
        unsafe {
            let adaptor = match adaptor {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxAddRemoveCtrl_SetAdaptor(self.as_ptr(), adaptor)
        }
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

// wxAffineMatrix2D
pub trait AffineMatrix2DMethods: AffineMatrix2DBaseMethods {
    // BLOCKED: fn IsEqual()
}

// wxAffineMatrix2DBase
pub trait AffineMatrix2DBaseMethods: WxRustMethods {
    // DTOR: fn ~wxAffineMatrix2DBase()
    fn set(&self, mat2_d: *const c_void, tr: *const c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_Set(self.as_ptr(), mat2_d, tr) }
    }
    fn get(&self, mat2_d: *mut c_void, tr: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_Get(self.as_ptr(), mat2_d, tr) }
    }
    fn concat<A: AffineMatrix2DBaseMethods>(&self, t: &A) {
        unsafe {
            let t = t.as_ptr();
            ffi::wxAffineMatrix2DBase_Concat(self.as_ptr(), t)
        }
    }
    fn invert(&self) -> bool {
        unsafe { ffi::wxAffineMatrix2DBase_Invert(self.as_ptr()) }
    }
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxAffineMatrix2DBase_IsIdentity(self.as_ptr()) }
    }
    fn is_equal<A: AffineMatrix2DBaseMethods>(&self, t: &A) -> bool {
        unsafe {
            let t = t.as_ptr();
            ffi::wxAffineMatrix2DBase_IsEqual(self.as_ptr(), t)
        }
    }
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // NOT_SUPPORTED: fn Translate()
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Rotate()
    fn mirror(&self, direction: c_int) {
        unsafe { ffi::wxAffineMatrix2DBase_Mirror(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn TransformPoint()
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_TransformPoint1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn TransformDistance()
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_TransformDistance1(self.as_ptr(), dx, dy) }
    }
}

// wxAnimationCtrl
pub trait AnimationCtrlMethods: ControlMethods {
    fn create_animation<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        anim: *const c_void,
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
            ffi::wxAnimationCtrl_Create(self.as_ptr(), parent, id, anim, pos, size, style, name)
        }
    }
    // NOT_SUPPORTED: fn CreateAnimation()
    // NOT_SUPPORTED: fn GetAnimation()
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
    fn set_animation(&self, anim: *const c_void) {
        unsafe { ffi::wxAnimationCtrl_SetAnimation(self.as_ptr(), anim) }
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
    // NOT_SUPPORTED: fn CreateCompatibleAnimation()
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

// wxAppProgressIndicator
pub trait AppProgressIndicatorMethods: WxRustMethods {
    // DTOR: fn ~wxAppProgressIndicator()
    fn is_available(&self) -> bool {
        unsafe { ffi::wxAppProgressIndicator_IsAvailable(self.as_ptr()) }
    }
    fn set_value(&self, value: c_int) {
        unsafe { ffi::wxAppProgressIndicator_SetValue(self.as_ptr(), value) }
    }
    fn set_range(&self, range: c_int) {
        unsafe { ffi::wxAppProgressIndicator_SetRange(self.as_ptr(), range) }
    }
    // BLOCKED: fn Pulse()
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
    // NOT_SUPPORTED: fn GetIconBundle()
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
