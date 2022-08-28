use super::*;

// wxAboutDialogInfo
pub trait AboutDialogInfoMethods: WxRustMethods {
    /// Adds an artist name to be shown in the program credits.
    fn add_artist(&self, artist: &str) {
        unsafe {
            let artist = WxString::from(artist);
            let artist = artist.as_ptr();
            ffi::wxAboutDialogInfo_AddArtist(self.as_ptr(), artist)
        }
    }
    /// Adds a developer name to be shown in the program credits.
    fn add_developer(&self, developer: &str) {
        unsafe {
            let developer = WxString::from(developer);
            let developer = developer.as_ptr();
            ffi::wxAboutDialogInfo_AddDeveloper(self.as_ptr(), developer)
        }
    }
    /// Adds a documentation writer name to be shown in the program credits.
    fn add_doc_writer(&self, docwriter: &str) {
        unsafe {
            let docwriter = WxString::from(docwriter);
            let docwriter = docwriter.as_ptr();
            ffi::wxAboutDialogInfo_AddDocWriter(self.as_ptr(), docwriter)
        }
    }
    /// Adds a translator name to be shown in the program credits.
    fn add_translator(&self, translator: &str) {
        unsafe {
            let translator = WxString::from(translator);
            let translator = translator.as_ptr();
            ffi::wxAboutDialogInfo_AddTranslator(self.as_ptr(), translator)
        }
    }
    /// Get the name of the program.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetName(self.as_ptr())).into() }
    }
    /// Returns true if a description string has been specified.
    fn has_description(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDescription(self.as_ptr()) }
    }
    /// Get the description string.
    fn get_description(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetDescription(self.as_ptr())).into() }
    }
    /// Returns true if a copyright string has been specified.
    fn has_copyright(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasCopyright(self.as_ptr()) }
    }
    /// Get the copyright string.
    fn get_copyright(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetCopyright(self.as_ptr())).into() }
    }
    /// Sets the list of artists to be shown in the program credits.
    fn set_artists<A: ArrayStringMethods>(&self, artists: &A) {
        unsafe {
            let artists = artists.as_ptr();
            ffi::wxAboutDialogInfo_SetArtists(self.as_ptr(), artists)
        }
    }
    /// Set the short string containing the program copyright information.
    fn set_copyright(&self, copyright: &str) {
        unsafe {
            let copyright = WxString::from(copyright);
            let copyright = copyright.as_ptr();
            ffi::wxAboutDialogInfo_SetCopyright(self.as_ptr(), copyright)
        }
    }
    /// Set brief, but possibly multiline, description of the program.
    fn set_description(&self, desc: &str) {
        unsafe {
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxAboutDialogInfo_SetDescription(self.as_ptr(), desc)
        }
    }
    /// Set the list of developers of the program.
    fn set_developers<A: ArrayStringMethods>(&self, developers: &A) {
        unsafe {
            let developers = developers.as_ptr();
            ffi::wxAboutDialogInfo_SetDevelopers(self.as_ptr(), developers)
        }
    }
    /// Set the list of documentation writers.
    fn set_doc_writers<A: ArrayStringMethods>(&self, docwriters: &A) {
        unsafe {
            let docwriters = docwriters.as_ptr();
            ffi::wxAboutDialogInfo_SetDocWriters(self.as_ptr(), docwriters)
        }
    }
    /// Returns true if an icon has been set for the about dialog.
    fn has_icon(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasIcon(self.as_ptr()) }
    }
    /// Returns the icon set by SetIcon().
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxAboutDialogInfo_GetIcon(self.as_ptr())) }
    }
    /// Set the icon to be shown in the dialog.
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxAboutDialogInfo_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Returns true if the licence string has been set.
    fn has_licence(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasLicence(self.as_ptr()) }
    }
    /// Returns the licence string.
    fn get_licence(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetLicence(self.as_ptr())).into() }
    }
    /// Set the long, multiline string containing the text of the program licence.
    fn set_licence(&self, licence: &str) {
        unsafe {
            let licence = WxString::from(licence);
            let licence = licence.as_ptr();
            ffi::wxAboutDialogInfo_SetLicence(self.as_ptr(), licence)
        }
    }
    /// This is the same as SetLicence().
    fn set_license(&self, licence: &str) {
        unsafe {
            let licence = WxString::from(licence);
            let licence = licence.as_ptr();
            ffi::wxAboutDialogInfo_SetLicense(self.as_ptr(), licence)
        }
    }
    /// Set the name of the program.
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxAboutDialogInfo_SetName(self.as_ptr(), name)
        }
    }
    /// Set the list of translators.
    fn set_translators<A: ArrayStringMethods>(&self, translators: &A) {
        unsafe {
            let translators = translators.as_ptr();
            ffi::wxAboutDialogInfo_SetTranslators(self.as_ptr(), translators)
        }
    }
    /// Set the version of the program.
    fn set_version(&self, version: &str, long_version: &str) {
        unsafe {
            let version = WxString::from(version);
            let version = version.as_ptr();
            let long_version = WxString::from(long_version);
            let long_version = long_version.as_ptr();
            ffi::wxAboutDialogInfo_SetVersion(self.as_ptr(), version, long_version)
        }
    }
    /// Return the short version string.
    fn get_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetVersion(self.as_ptr())).into() }
    }
    /// Return the long version string if set.
    fn get_long_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetLongVersion(self.as_ptr())).into() }
    }
    /// Returns true if the website info has been set.
    fn has_web_site(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasWebSite(self.as_ptr()) }
    }
    /// Returns the website URL set for the dialog.
    fn get_web_site_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetWebSiteURL(self.as_ptr())).into() }
    }
    /// Returns the description of the website URL set for the dialog.
    fn get_web_site_description(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxAboutDialogInfo_GetWebSiteDescription(self.as_ptr())).into()
        }
    }
    /// Set the web site for the program and its description (which defaults to url itself if empty).
    fn set_web_site(&self, url: &str, desc: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxAboutDialogInfo_SetWebSite(self.as_ptr(), url, desc)
        }
    }
    /// Returns true if developers have been set in the dialog info.
    fn has_developers(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDevelopers(self.as_ptr()) }
    }
    /// Returns an array of the developer strings set in the dialog info.
    fn get_developers(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetDevelopers(self.as_ptr())) }
    }
    /// Returns true if writers have been set in the dialog info.
    fn has_doc_writers(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDocWriters(self.as_ptr()) }
    }
    /// Returns an array of the writer strings set in the dialog info.
    fn get_doc_writers(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetDocWriters(self.as_ptr())) }
    }
    /// Returns true if artists have been set in the dialog info.
    fn has_artists(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasArtists(self.as_ptr()) }
    }
    /// Returns an array of the artist strings set in the dialog info.
    fn get_artists(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetArtists(self.as_ptr())) }
    }
    /// Returns true if translators have been set in the dialog info.
    fn has_translators(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasTranslators(self.as_ptr()) }
    }
    /// Returns an array of the translator strings set in the dialog info.
    fn get_translators(&self) -> ArrayStringIsOwned<false> {
        unsafe {
            ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetTranslators(self.as_ptr()))
        }
    }
}

// wxAcceleratorEntry
pub trait AcceleratorEntryMethods: WxRustMethods {
    /// Returns the command identifier for the accelerator table entry.
    fn get_command(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetCommand(self.as_ptr()) }
    }
    /// Returns the flags for the accelerator table entry.
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetFlags(self.as_ptr()) }
    }
    /// Returns the keycode for the accelerator table entry.
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetKeyCode(self.as_ptr()) }
    }
    /// Returns the menu item associated with this accelerator entry.
    fn get_menu_item(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxAcceleratorEntry_GetMenuItem(self.as_ptr())) }
    }
    /// Sets the accelerator entry parameters.
    fn set<M: MenuItemMethods>(&self, flags: c_int, key_code: c_int, cmd: c_int, item: Option<&M>) {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxAcceleratorEntry_Set(self.as_ptr(), flags, key_code, cmd, item)
        }
    }
    /// Returns true if this object is correctly initialized.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxAcceleratorEntry_IsOk(self.as_ptr()) }
    }
    /// Returns a textual representation of this accelerator.
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAcceleratorEntry_ToString(self.as_ptr())).into() }
    }
    /// Returns a textual representation of this accelerator which is appropriate for saving in configuration files.
    fn to_raw_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAcceleratorEntry_ToRawString(self.as_ptr())).into() }
    }
    /// Parses the given string and sets the accelerator accordingly.
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
    /// Returns true if the accelerator table is valid.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxAcceleratorTable_IsOk(self.as_ptr()) }
    }
}

// wxActivateEvent
pub trait ActivateEventMethods: EventMethods {
    /// Returns true if the application or window is being activated, false otherwise.
    fn get_active(&self) -> bool {
        unsafe { ffi::wxActivateEvent_GetActive(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetActivationReason()
}

// wxAffineMatrix2D
pub trait AffineMatrix2DMethods: AffineMatrix2DBaseMethods {
    // BLOCKED: fn IsEqual()
}

// wxAffineMatrix2DBase
pub trait AffineMatrix2DBaseMethods: WxRustMethods {
    // DTOR: fn ~wxAffineMatrix2DBase()
    /// Set all elements of this matrix.
    fn set(&self, mat2_d: *const c_void, tr: *const c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_Set(self.as_ptr(), mat2_d, tr) }
    }
    /// Get the component values of the matrix.
    fn get(&self, mat2_d: *mut c_void, tr: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_Get(self.as_ptr(), mat2_d, tr) }
    }
    /// Concatenate this matrix with another one.
    fn concat<A: AffineMatrix2DBaseMethods>(&self, t: &A) {
        unsafe {
            let t = t.as_ptr();
            ffi::wxAffineMatrix2DBase_Concat(self.as_ptr(), t)
        }
    }
    /// Invert this matrix.
    fn invert(&self) -> bool {
        unsafe { ffi::wxAffineMatrix2DBase_Invert(self.as_ptr()) }
    }
    /// Check if this is the identity matrix.
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxAffineMatrix2DBase_IsIdentity(self.as_ptr()) }
    }
    /// Check that this matrix is identical with t.
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
    /// Add mirroring to this matrix.
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
    /// Creates the control with the given anim animation.
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
    /// Returns the inactive bitmap shown in this control when the; see SetInactiveBitmap() for more info.
    fn get_inactive_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnimationCtrl_GetInactiveBitmap(self.as_ptr())) }
    }
    /// Returns true if the animation is being played.
    fn is_playing(&self) -> bool {
        unsafe { ffi::wxAnimationCtrl_IsPlaying(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn Load()
    /// Starts playing the animation.
    fn play(&self) -> bool {
        unsafe { ffi::wxAnimationCtrl_Play(self.as_ptr()) }
    }
    /// Sets the animation to play in this control.
    fn set_animation(&self, anim: *const c_void) {
        unsafe { ffi::wxAnimationCtrl_SetAnimation(self.as_ptr(), anim) }
    }
    /// Sets the bitmap to show on the control when it's not playing an animation.
    fn set_inactive_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxAnimationCtrl_SetInactiveBitmap(self.as_ptr(), bmp)
        }
    }
    /// Stops playing the animation.
    fn stop(&self) {
        unsafe { ffi::wxAnimationCtrl_Stop(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn CreateCompatibleAnimation()
}

// wxAnyButton
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    /// Return the bitmap shown by the button.
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmap(self.as_ptr())) }
    }
    /// Returns the bitmap used when the mouse is over the button.
    fn get_bitmap_current(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapCurrent(self.as_ptr())) }
    }
    /// Returns the bitmap used for the disabled state.
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapDisabled(self.as_ptr())) }
    }
    /// Returns the bitmap used for the focused state.
    fn get_bitmap_focus(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapFocus(self.as_ptr())) }
    }
    /// Returns the bitmap for the normal state.
    fn get_bitmap_label(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapLabel(self.as_ptr())) }
    }
    /// Returns the bitmap used when the button is pressed.
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapPressed(self.as_ptr())) }
    }
    /// Sets the bitmap to display in the button.
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B, dir: c_int) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmap(self.as_ptr(), bitmap, dir)
        }
    }
    /// Sets the bitmap to be shown when the mouse is over the button.
    fn set_bitmap_current<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap for the disabled button appearance.
    fn set_bitmap_disabled<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap for the button appearance when it has the keyboard focus.
    fn set_bitmap_focus<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapFocus(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap label for the button.
    fn set_bitmap_label<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapLabel(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap for the selected (depressed) button appearance.
    fn set_bitmap_pressed<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapPressed(self.as_ptr(), bitmap)
        }
    }
    /// Get the margins between the bitmap and the text of the button.
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr())) }
    }
    /// Set the margins between the bitmap and the text of the button.
    fn set_bitmap_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr(), x, y) }
    }
    fn set_bitmap_margins_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr(), sz)
        }
    }
    /// Set the position at which the bitmap is displayed.
    fn set_bitmap_position(&self, dir: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapPosition(self.as_ptr(), dir) }
    }
}

// wxArtProvider
pub trait ArtProviderMethods: ObjectMethods {
    // DTOR: fn ~wxArtProvider()
    /// Delete the given provider.
    fn delete<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Delete(provider)
        }
    }
    /// Query registered providers for bitmap with given ID.
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
    /// Query registered providers for a bundle of bitmaps with given ID.
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
    /// Same as wxArtProvider::GetBitmap, but return a wxIcon object (or wxNullIcon on failure).
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
    /// Returns native icon size for use specified by client hint in DIPs.
    fn get_native_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetNativeDIPSizeHint(client))
        }
    }
    /// Returns native icon size for use specified by client hint.
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
    /// Returns a suitable size hint for the given wxArtClient in DIPs.
    fn get_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetDIPSizeHint(client))
        }
    }
    /// Returns a suitable size hint for the given wxArtClient.
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
    /// Query registered providers for icon bundle with given ID.
    fn get_icon_bundle(id: &str, client: &str) -> IconBundle {
        unsafe {
            let id = WxString::from(id);
            let id = id.as_ptr();
            let client = WxString::from(client);
            let client = client.as_ptr();
            IconBundle::from_ptr(ffi::wxArtProvider_GetIconBundle(id, client))
        }
    }
    /// Returns true if the platform uses native icons provider that should take precedence over any customizations.
    fn has_native_provider() -> bool {
        unsafe { ffi::wxArtProvider_HasNativeProvider() }
    }
    // BLOCKED: fn Insert()
    /// Remove latest added provider and delete it.
    fn pop() -> bool {
        unsafe { ffi::wxArtProvider_Pop() }
    }
    /// Register new art provider and add it to the top of providers stack (i.e.
    fn push<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Push(provider)
        }
    }
    /// Register new art provider and add it to the bottom of providers stack.
    fn push_back<A: ArtProviderMethods>(provider: Option<&A>) {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_PushBack(provider)
        }
    }
    /// Remove a provider from the stack if it is on it.
    fn remove<A: ArtProviderMethods>(provider: Option<&A>) -> bool {
        unsafe {
            let provider = match provider {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxArtProvider_Remove(provider)
        }
    }
    /// Helper used by GetMessageBoxIcon(): return the art id corresponding to the standard wxICON_INFORMATION/WARNING/ERROR/QUESTION flags (only one can be set)
    fn get_message_box_icon_id(flags: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxArtProvider_GetMessageBoxIconId(flags)).into() }
    }
    /// Helper used by several generic classes: return the icon corresponding to the standard wxICON_INFORMATION/WARNING/ERROR/QUESTION flags (only one can be set)
    fn get_message_box_icon(flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxArtProvider_GetMessageBoxIcon(flags)) }
    }
}

// wxAutoBufferedPaintDC
pub trait AutoBufferedPaintDCMethods: BufferedPaintDCMethods {}
