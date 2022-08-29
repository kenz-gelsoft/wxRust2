use super::*;

// wxAboutDialogInfo
/// This trait represents C++ [`wxAboutDialogInfo`](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html) class's methods and inheritance.
///
/// See [`AboutDialogInfoIsOwned`] documentation for the class usage.
pub trait AboutDialogInfoMethods: WxRustMethods {
    /// Adds an artist name to be shown in the program credits.
    ///
    /// [See `wxAboutDialogInfo::AddArtist()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a48d616165e8a0614f7a063f898c69f8b)
    fn add_artist(&self, artist: &str) {
        unsafe {
            let artist = WxString::from(artist);
            let artist = artist.as_ptr();
            ffi::wxAboutDialogInfo_AddArtist(self.as_ptr(), artist)
        }
    }
    /// Adds a developer name to be shown in the program credits.
    ///
    /// [See `wxAboutDialogInfo::AddDeveloper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a26dba3b09941ae3761c452b78d37165f)
    fn add_developer(&self, developer: &str) {
        unsafe {
            let developer = WxString::from(developer);
            let developer = developer.as_ptr();
            ffi::wxAboutDialogInfo_AddDeveloper(self.as_ptr(), developer)
        }
    }
    /// Adds a documentation writer name to be shown in the program credits.
    ///
    /// [See `wxAboutDialogInfo::AddDocWriter()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ab4d85602b53de48169bbdb4d0cf9c67a)
    fn add_doc_writer(&self, docwriter: &str) {
        unsafe {
            let docwriter = WxString::from(docwriter);
            let docwriter = docwriter.as_ptr();
            ffi::wxAboutDialogInfo_AddDocWriter(self.as_ptr(), docwriter)
        }
    }
    /// Adds a translator name to be shown in the program credits.
    ///
    /// [See `wxAboutDialogInfo::AddTranslator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#aeaa05b49f9c1df2b1ee6304d64426843)
    fn add_translator(&self, translator: &str) {
        unsafe {
            let translator = WxString::from(translator);
            let translator = translator.as_ptr();
            ffi::wxAboutDialogInfo_AddTranslator(self.as_ptr(), translator)
        }
    }
    /// Get the name of the program.
    ///
    /// [See `wxAboutDialogInfo::GetName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a3bd339e4b28c4cb49a40a17765bb741d)
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetName(self.as_ptr())).into() }
    }
    /// Returns true if a description string has been specified.
    ///
    /// [See `wxAboutDialogInfo::HasDescription()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#abf020f899343483c2a66b3cbe5c48eb2)
    fn has_description(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDescription(self.as_ptr()) }
    }
    /// Get the description string.
    ///
    /// [See `wxAboutDialogInfo::GetDescription()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#abc72b224c9a0317dcab1dfc3d31d019d)
    fn get_description(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetDescription(self.as_ptr())).into() }
    }
    /// Returns true if a copyright string has been specified.
    ///
    /// [See `wxAboutDialogInfo::HasCopyright()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a9ae57445d46b6e827c710fb7cfed980b)
    fn has_copyright(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasCopyright(self.as_ptr()) }
    }
    /// Get the copyright string.
    ///
    /// [See `wxAboutDialogInfo::GetCopyright()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a6215ac2165c0396c87e48a460073b794)
    fn get_copyright(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetCopyright(self.as_ptr())).into() }
    }
    /// Sets the list of artists to be shown in the program credits.
    ///
    /// [See `wxAboutDialogInfo::SetArtists()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ab0bf3125a82ea0acef226d37aa54879d)
    fn set_artists<A: ArrayStringMethods>(&self, artists: &A) {
        unsafe {
            let artists = artists.as_ptr();
            ffi::wxAboutDialogInfo_SetArtists(self.as_ptr(), artists)
        }
    }
    /// Set the short string containing the program copyright information.
    ///
    /// [See `wxAboutDialogInfo::SetCopyright()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a10f5d9b8ca9ed69754e54fd2e03c4538)
    fn set_copyright(&self, copyright: &str) {
        unsafe {
            let copyright = WxString::from(copyright);
            let copyright = copyright.as_ptr();
            ffi::wxAboutDialogInfo_SetCopyright(self.as_ptr(), copyright)
        }
    }
    /// Set brief, but possibly multiline, description of the program.
    ///
    /// [See `wxAboutDialogInfo::SetDescription()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a47bd84aa96af70cb304a76c6cf3428e3)
    fn set_description(&self, desc: &str) {
        unsafe {
            let desc = WxString::from(desc);
            let desc = desc.as_ptr();
            ffi::wxAboutDialogInfo_SetDescription(self.as_ptr(), desc)
        }
    }
    /// Set the list of developers of the program.
    ///
    /// [See `wxAboutDialogInfo::SetDevelopers()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a2bde3b28eb55f653e8229ea534775c24)
    fn set_developers<A: ArrayStringMethods>(&self, developers: &A) {
        unsafe {
            let developers = developers.as_ptr();
            ffi::wxAboutDialogInfo_SetDevelopers(self.as_ptr(), developers)
        }
    }
    /// Set the list of documentation writers.
    ///
    /// [See `wxAboutDialogInfo::SetDocWriters()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ad268df6e3fd75d9e9c02a463c87bdab6)
    fn set_doc_writers<A: ArrayStringMethods>(&self, docwriters: &A) {
        unsafe {
            let docwriters = docwriters.as_ptr();
            ffi::wxAboutDialogInfo_SetDocWriters(self.as_ptr(), docwriters)
        }
    }
    /// Returns true if an icon has been set for the about dialog.
    ///
    /// [See `wxAboutDialogInfo::HasIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a8189b1afc9d836050c0296e4de7ba32b)
    fn has_icon(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasIcon(self.as_ptr()) }
    }
    /// Returns the icon set by SetIcon().
    ///
    /// [See `wxAboutDialogInfo::GetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#aa6c5a14ab0ed2dfd2bf7d3e62703796e)
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxAboutDialogInfo_GetIcon(self.as_ptr())) }
    }
    /// Set the icon to be shown in the dialog.
    ///
    /// [See `wxAboutDialogInfo::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a949bb4edf09f05b9aa36ea6d1828f5b5)
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxAboutDialogInfo_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Returns true if the licence string has been set.
    ///
    /// [See `wxAboutDialogInfo::HasLicence()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a4eb70085ef9895583493ceb95682c465)
    fn has_licence(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasLicence(self.as_ptr()) }
    }
    /// Returns the licence string.
    ///
    /// [See `wxAboutDialogInfo::GetLicence()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ae5b9d2aedd2a0e0709be6ad25b0bc2c2)
    fn get_licence(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetLicence(self.as_ptr())).into() }
    }
    /// Set the long, multiline string containing the text of the program licence.
    ///
    /// [See `wxAboutDialogInfo::SetLicence()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ae0e88ab74655d5cdd9eef68e019309c1)
    fn set_licence(&self, licence: &str) {
        unsafe {
            let licence = WxString::from(licence);
            let licence = licence.as_ptr();
            ffi::wxAboutDialogInfo_SetLicence(self.as_ptr(), licence)
        }
    }
    /// This is the same as SetLicence().
    ///
    /// [See `wxAboutDialogInfo::SetLicense()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a02df2d9f14171914609f8401ca3f1ea2)
    fn set_license(&self, licence: &str) {
        unsafe {
            let licence = WxString::from(licence);
            let licence = licence.as_ptr();
            ffi::wxAboutDialogInfo_SetLicense(self.as_ptr(), licence)
        }
    }
    /// Set the name of the program.
    ///
    /// [See `wxAboutDialogInfo::SetName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a5fef05c5e61acb59d9554bda8de26eef)
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxAboutDialogInfo_SetName(self.as_ptr(), name)
        }
    }
    /// Set the list of translators.
    ///
    /// [See `wxAboutDialogInfo::SetTranslators()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a1deb2e1d6a68f87857aa9cd10d666221)
    fn set_translators<A: ArrayStringMethods>(&self, translators: &A) {
        unsafe {
            let translators = translators.as_ptr();
            ffi::wxAboutDialogInfo_SetTranslators(self.as_ptr(), translators)
        }
    }
    /// Set the version of the program.
    ///
    /// [See `wxAboutDialogInfo::SetVersion()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#aac5029cab6b04b34dbf8cd6b66fff251)
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
    ///
    /// [See `wxAboutDialogInfo::GetVersion()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#afeed1bb45e289a76e481d7140c7d3bc6)
    fn get_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetVersion(self.as_ptr())).into() }
    }
    /// Return the long version string if set.
    ///
    /// [See `wxAboutDialogInfo::GetLongVersion()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a81b4d196aec462b3d0fcecf9c7a49baf)
    fn get_long_version(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetLongVersion(self.as_ptr())).into() }
    }
    /// Returns true if the website info has been set.
    ///
    /// [See `wxAboutDialogInfo::HasWebSite()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a35e4680c9b1c76af71daf57ce881e813)
    fn has_web_site(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasWebSite(self.as_ptr()) }
    }
    /// Returns the website URL set for the dialog.
    ///
    /// [See `wxAboutDialogInfo::GetWebSiteURL()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ac606840130d6dc061c95e0ab00a7fe2f)
    fn get_web_site_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAboutDialogInfo_GetWebSiteURL(self.as_ptr())).into() }
    }
    /// Returns the description of the website URL set for the dialog.
    ///
    /// [See `wxAboutDialogInfo::GetWebSiteDescription()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a546b89f1a7a0eefba129178138d585b0)
    fn get_web_site_description(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxAboutDialogInfo_GetWebSiteDescription(self.as_ptr())).into()
        }
    }
    /// Set the web site for the program and its description (which defaults to url itself if empty).
    ///
    /// [See `wxAboutDialogInfo::SetWebSite()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ab6104e724f64b936ee2ff0727e8e797a)
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
    ///
    /// [See `wxAboutDialogInfo::HasDevelopers()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#abc58092538dc37f506d10eeeb3113fe7)
    fn has_developers(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDevelopers(self.as_ptr()) }
    }
    /// Returns an array of the developer strings set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::GetDevelopers()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a17e0b6d92fd9f882660453e7f4fb245a)
    fn get_developers(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetDevelopers(self.as_ptr())) }
    }
    /// Returns true if writers have been set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::HasDocWriters()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ad25618c17a096fb947cb859de49917c9)
    fn has_doc_writers(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasDocWriters(self.as_ptr()) }
    }
    /// Returns an array of the writer strings set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::GetDocWriters()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#aa033004a6a61b6b916ea8c1c7fa9391f)
    fn get_doc_writers(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetDocWriters(self.as_ptr())) }
    }
    /// Returns true if artists have been set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::HasArtists()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#aadc4e032c6fa9d1b2cc7e067da0f8c41)
    fn has_artists(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasArtists(self.as_ptr()) }
    }
    /// Returns an array of the artist strings set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::GetArtists()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a2cd77196f049420669de335f70bc99fc)
    fn get_artists(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetArtists(self.as_ptr())) }
    }
    /// Returns true if translators have been set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::HasTranslators()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#a486aff07a28b8e73675df128333a3fdb)
    fn has_translators(&self) -> bool {
        unsafe { ffi::wxAboutDialogInfo_HasTranslators(self.as_ptr()) }
    }
    /// Returns an array of the translator strings set in the dialog info.
    ///
    /// [See `wxAboutDialogInfo::GetTranslators()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_about_dialog_info.html#ab1533f81dfc5b8768993970f543fc093)
    fn get_translators(&self) -> ArrayStringIsOwned<false> {
        unsafe {
            ArrayStringIsOwned::from_ptr(ffi::wxAboutDialogInfo_GetTranslators(self.as_ptr()))
        }
    }
}

// wxAcceleratorEntry
/// This trait represents C++ [`wxAcceleratorEntry`](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html) class's methods and inheritance.
///
/// See [`AcceleratorEntryIsOwned`] documentation for the class usage.
pub trait AcceleratorEntryMethods: WxRustMethods {
    /// Returns the command identifier for the accelerator table entry.
    ///
    /// [See `wxAcceleratorEntry::GetCommand()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#ac04ae5109857d6a432057375d1f72174)
    fn get_command(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetCommand(self.as_ptr()) }
    }
    /// Returns the flags for the accelerator table entry.
    ///
    /// [See `wxAcceleratorEntry::GetFlags()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#aeb825a84804091ee0495bf40924d6610)
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetFlags(self.as_ptr()) }
    }
    /// Returns the keycode for the accelerator table entry.
    ///
    /// [See `wxAcceleratorEntry::GetKeyCode()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a9281b3cd75b6f2a9e3eabae6444eea18)
    fn get_key_code(&self) -> c_int {
        unsafe { ffi::wxAcceleratorEntry_GetKeyCode(self.as_ptr()) }
    }
    /// Returns the menu item associated with this accelerator entry.
    ///
    /// [See `wxAcceleratorEntry::GetMenuItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a4bf52758919312c09be2a7194edcb37d)
    fn get_menu_item(&self) -> Option<MenuItemIsOwned<false>> {
        unsafe { MenuItem::option_from(ffi::wxAcceleratorEntry_GetMenuItem(self.as_ptr())) }
    }
    /// Sets the accelerator entry parameters.
    ///
    /// [See `wxAcceleratorEntry::Set()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#abaac10bba35d3e3c32f05761e0d19d1a)
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
    ///
    /// [See `wxAcceleratorEntry::IsOk()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#ab7cbeb55611e7102f59f804814921368)
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxAcceleratorEntry_IsOk(self.as_ptr()) }
    }
    /// Returns a textual representation of this accelerator.
    ///
    /// [See `wxAcceleratorEntry::ToString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a79fcc359fddfaee93f68d0ee1f5c871d)
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAcceleratorEntry_ToString(self.as_ptr())).into() }
    }
    /// Returns a textual representation of this accelerator which is appropriate for saving in configuration files.
    ///
    /// [See `wxAcceleratorEntry::ToRawString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#ae877a331da1de8c705ae13132524dd52)
    fn to_raw_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxAcceleratorEntry_ToRawString(self.as_ptr())).into() }
    }
    /// Parses the given string and sets the accelerator accordingly.
    ///
    /// [See `wxAcceleratorEntry::FromString()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_entry.html#a201b717e9069367c60bd8765eda60371)
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
/// This trait represents C++ [`wxAcceleratorTable`](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html) class's methods and inheritance.
///
/// See [`AcceleratorTableIsOwned`] documentation for the class usage.
pub trait AcceleratorTableMethods: ObjectMethods {
    // DTOR: fn ~wxAcceleratorTable()
    /// Returns true if the accelerator table is valid.
    ///
    /// [See `wxAcceleratorTable::IsOk()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_accelerator_table.html#a9c28ef355e343ed963b3281f3897f622)
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxAcceleratorTable_IsOk(self.as_ptr()) }
    }
}

// wxActivateEvent
/// This trait represents C++ [`wxActivateEvent`](https://docs.wxwidgets.org/3.2/classwx_activate_event.html) class's methods and inheritance.
///
/// See [`ActivateEventIsOwned`] documentation for the class usage.
pub trait ActivateEventMethods: EventMethods {
    /// Returns true if the application or window is being activated, false otherwise.
    ///
    /// [See `wxActivateEvent::GetActive()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_activate_event.html#a27c26e49ca21fca595e075343425b209)
    fn get_active(&self) -> bool {
        unsafe { ffi::wxActivateEvent_GetActive(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetActivationReason()
}

// wxAffineMatrix2D
/// This trait represents C++ [`wxAffineMatrix2D`](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d.html) class's methods and inheritance.
///
/// See [`AffineMatrix2DIsOwned`] documentation for the class usage.
pub trait AffineMatrix2DMethods: AffineMatrix2DBaseMethods {
    // BLOCKED: fn IsEqual()
}

// wxAffineMatrix2DBase
/// This trait represents C++ [`wxAffineMatrix2DBase`](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html) class's methods and inheritance.
///
/// See [`AffineMatrix2DBaseIsOwned`] documentation for the class usage.
pub trait AffineMatrix2DBaseMethods: WxRustMethods {
    // DTOR: fn ~wxAffineMatrix2DBase()
    /// Set all elements of this matrix.
    ///
    /// [See `wxAffineMatrix2DBase::Set()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#a9c9d218ff4e13a07028ba322c6f392ed)
    fn set(&self, mat2_d: *const c_void, tr: *const c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_Set(self.as_ptr(), mat2_d, tr) }
    }
    /// Get the component values of the matrix.
    ///
    /// [See `wxAffineMatrix2DBase::Get()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#a92c55f3082b18e441222e0c33c9ea48f)
    fn get(&self, mat2_d: *mut c_void, tr: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_Get(self.as_ptr(), mat2_d, tr) }
    }
    /// Concatenate this matrix with another one.
    ///
    /// [See `wxAffineMatrix2DBase::Concat()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#a7bc767e9e7abeb95464e6d3399400bce)
    fn concat<A: AffineMatrix2DBaseMethods>(&self, t: &A) {
        unsafe {
            let t = t.as_ptr();
            ffi::wxAffineMatrix2DBase_Concat(self.as_ptr(), t)
        }
    }
    /// Invert this matrix.
    ///
    /// [See `wxAffineMatrix2DBase::Invert()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#a2e2d2352476c2aac28152a3d24c6c32e)
    fn invert(&self) -> bool {
        unsafe { ffi::wxAffineMatrix2DBase_Invert(self.as_ptr()) }
    }
    /// Check if this is the identity matrix.
    ///
    /// [See `wxAffineMatrix2DBase::IsIdentity()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#aaad31059aa13923b36524a4bda86ff62)
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxAffineMatrix2DBase_IsIdentity(self.as_ptr()) }
    }
    /// Check that this matrix is identical with t.
    ///
    /// [See `wxAffineMatrix2DBase::IsEqual()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#a6dd44b7e4efc2263b9579a3d0baa0a3a)
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
    ///
    /// [See `wxAffineMatrix2DBase::Mirror()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#ab6c40a633a3bd3ad7919aaf5080b94d7)
    fn mirror(&self, direction: c_int) {
        unsafe { ffi::wxAffineMatrix2DBase_Mirror(self.as_ptr(), direction) }
    }
    // NOT_SUPPORTED: fn TransformPoint()
    ///
    /// [See `wxAffineMatrix2DBase::TransformPoint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#acb3cac84badbc457f10ef3b694d9cc78)
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_TransformPoint1(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn TransformDistance()
    ///
    /// [See `wxAffineMatrix2DBase::TransformDistance()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_affine_matrix2_d_base.html#afaf09f8d64572e4c48fec940788f0403)
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxAffineMatrix2DBase_TransformDistance1(self.as_ptr(), dx, dy) }
    }
}

// wxAnimationCtrl
/// This trait represents C++ [`wxAnimationCtrl`](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html) class's methods and inheritance.
///
/// See [`AnimationCtrlIsOwned`] documentation for the class usage.
pub trait AnimationCtrlMethods: ControlMethods {
    /// Creates the control with the given anim animation.
    ///
    /// [See `wxAnimationCtrl::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#a29e4df79f64e431a83c9e17f2acaa9ac)
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
    ///
    /// [See `wxAnimationCtrl::GetInactiveBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#ae643280581fe80f256e3a9d10ddbabe5)
    fn get_inactive_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnimationCtrl_GetInactiveBitmap(self.as_ptr())) }
    }
    /// Returns true if the animation is being played.
    ///
    /// [See `wxAnimationCtrl::IsPlaying()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#a8a1eec70dfad9cc45988c9c6403e08af)
    fn is_playing(&self) -> bool {
        unsafe { ffi::wxAnimationCtrl_IsPlaying(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn Load()
    /// Starts playing the animation.
    ///
    /// [See `wxAnimationCtrl::Play()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#ace0ae1bf6e30458cc5da6b4696c9b2fa)
    fn play(&self) -> bool {
        unsafe { ffi::wxAnimationCtrl_Play(self.as_ptr()) }
    }
    /// Sets the animation to play in this control.
    ///
    /// [See `wxAnimationCtrl::SetAnimation()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#a19f6444bb0fb00d7a3bee9745010770a)
    fn set_animation(&self, anim: *const c_void) {
        unsafe { ffi::wxAnimationCtrl_SetAnimation(self.as_ptr(), anim) }
    }
    /// Sets the bitmap to show on the control when it's not playing an animation.
    ///
    /// [See `wxAnimationCtrl::SetInactiveBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#ad91f77e32a65d9194c0757d9d4c29c27)
    fn set_inactive_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxAnimationCtrl_SetInactiveBitmap(self.as_ptr(), bmp)
        }
    }
    /// Stops playing the animation.
    ///
    /// [See `wxAnimationCtrl::Stop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_animation_ctrl.html#aa7849c5941a6d8b6143c6c5dec6118a8)
    fn stop(&self) {
        unsafe { ffi::wxAnimationCtrl_Stop(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn CreateCompatibleAnimation()
}

// wxAnyButton
/// This trait represents C++ [`wxAnyButton`](https://docs.wxwidgets.org/3.2/classwx_any_button.html) class's methods and inheritance.
///
/// See [`AnyButtonIsOwned`] documentation for the class usage.
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    /// Return the bitmap shown by the button.
    ///
    /// [See `wxAnyButton::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a0c16ae4c69fec5dc3b4ba0aec9f9f4fa)
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmap(self.as_ptr())) }
    }
    /// Returns the bitmap used when the mouse is over the button.
    ///
    /// [See `wxAnyButton::GetBitmapCurrent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a11a021444ed0f69157cd541d6b1bd319)
    fn get_bitmap_current(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapCurrent(self.as_ptr())) }
    }
    /// Returns the bitmap used for the disabled state.
    ///
    /// [See `wxAnyButton::GetBitmapDisabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a0fcd803db5e971f84df526f092404afa)
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapDisabled(self.as_ptr())) }
    }
    /// Returns the bitmap used for the focused state.
    ///
    /// [See `wxAnyButton::GetBitmapFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a2a622be1e5e410d920aa2fafee528d5f)
    fn get_bitmap_focus(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapFocus(self.as_ptr())) }
    }
    /// Returns the bitmap for the normal state.
    ///
    /// [See `wxAnyButton::GetBitmapLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a4b0a3f9802c2705fd2712302e5cf04f5)
    fn get_bitmap_label(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapLabel(self.as_ptr())) }
    }
    /// Returns the bitmap used when the button is pressed.
    ///
    /// [See `wxAnyButton::GetBitmapPressed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#aff568c746ab7ddd9d25165a290e01391)
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxAnyButton_GetBitmapPressed(self.as_ptr())) }
    }
    /// Sets the bitmap to display in the button.
    ///
    /// [See `wxAnyButton::SetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#acdf75b25b6d948dd5ec4e04292283a70)
    fn set_bitmap<B: BitmapBundleMethods>(&self, bitmap: &B, dir: c_int) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmap(self.as_ptr(), bitmap, dir)
        }
    }
    /// Sets the bitmap to be shown when the mouse is over the button.
    ///
    /// [See `wxAnyButton::SetBitmapCurrent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a1763182bb87be55f21bf4099d7a168f3)
    fn set_bitmap_current<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap for the disabled button appearance.
    ///
    /// [See `wxAnyButton::SetBitmapDisabled()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#abe5111aaa070d4ce9603511210fc39e3)
    fn set_bitmap_disabled<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap for the button appearance when it has the keyboard focus.
    ///
    /// [See `wxAnyButton::SetBitmapFocus()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#aa2f48cf306eb0375c5ff5a94a9a83970)
    fn set_bitmap_focus<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapFocus(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap label for the button.
    ///
    /// [See `wxAnyButton::SetBitmapLabel()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#affe257f3d018f448ad1e4c2d33ae5e98)
    fn set_bitmap_label<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapLabel(self.as_ptr(), bitmap)
        }
    }
    /// Sets the bitmap for the selected (depressed) button appearance.
    ///
    /// [See `wxAnyButton::SetBitmapPressed()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a92164bbc6a2659f8ac751b146056a6c6)
    fn set_bitmap_pressed<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxAnyButton_SetBitmapPressed(self.as_ptr(), bitmap)
        }
    }
    /// Get the margins between the bitmap and the text of the button.
    ///
    /// [See `wxAnyButton::GetBitmapMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#ad45ae096a0a0ab0ba89442a34e76b30b)
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr())) }
    }
    /// Set the margins between the bitmap and the text of the button.
    ///
    /// [See `wxAnyButton::SetBitmapMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a6cd7a739c0795b85c82077934f6f8ba6)
    fn set_bitmap_margins_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr(), x, y) }
    }
    ///
    /// [See `wxAnyButton::SetBitmapMargins()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#ab12a46889cfb85eaa96f1a1dc3a21b8f)
    fn set_bitmap_margins_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr(), sz)
        }
    }
    /// Set the position at which the bitmap is displayed.
    ///
    /// [See `wxAnyButton::SetBitmapPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_any_button.html#a841c7747f78ea37110338b0ce1aa97dd)
    fn set_bitmap_position(&self, dir: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapPosition(self.as_ptr(), dir) }
    }
}

// wxArtProvider
/// This trait represents C++ [`wxArtProvider`](https://docs.wxwidgets.org/3.2/classwx_art_provider.html) class's methods and inheritance.
///
/// See [`ArtProviderIsOwned`] documentation for the class usage.
pub trait ArtProviderMethods: ObjectMethods {
    // DTOR: fn ~wxArtProvider()
    /// Delete the given provider.
    ///
    /// [See `wxArtProvider::Delete()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a2ea46956a7acce89f91be6ce422283eb)
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
    ///
    /// [See `wxArtProvider::GetBitmap()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a405ecf7cdead6dbfb092376a51a856c4)
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
    ///
    /// [See `wxArtProvider::GetBitmapBundle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a68224db0503920e8279c7fe5c46663c1)
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
    ///
    /// [See `wxArtProvider::GetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#ac9bd0ba166e5ef9515c68542bb96da09)
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
    ///
    /// [See `wxArtProvider::GetNativeDIPSizeHint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a389dc846b771923ed7e75b366dd12144)
    fn get_native_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetNativeDIPSizeHint(client))
        }
    }
    /// Returns native icon size for use specified by client hint.
    ///
    /// [See `wxArtProvider::GetNativeSizeHint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#ae60e56c8a8150e9d8961c763afba330a)
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
    ///
    /// [See `wxArtProvider::GetDIPSizeHint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a417520952155d602d8cc29a444f440d5)
    fn get_dip_size_hint(client: &str) -> Size {
        unsafe {
            let client = WxString::from(client);
            let client = client.as_ptr();
            Size::from_ptr(ffi::wxArtProvider_GetDIPSizeHint(client))
        }
    }
    /// Returns a suitable size hint for the given wxArtClient.
    ///
    /// [See `wxArtProvider::GetSizeHint()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#aa3e2dd0a936b876c32056bc37afff222)
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
    ///
    /// [See `wxArtProvider::GetIconBundle()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a0849828481ed689d716e600672fbaec0)
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
    ///
    /// [See `wxArtProvider::HasNativeProvider()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#ae64f10b3bc37c12ca7b18c2f43c42611)
    fn has_native_provider() -> bool {
        unsafe { ffi::wxArtProvider_HasNativeProvider() }
    }
    // BLOCKED: fn Insert()
    /// Remove latest added provider and delete it.
    ///
    /// [See `wxArtProvider::Pop()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a0be076c2270930e8a698dff5bee94b59)
    fn pop() -> bool {
        unsafe { ffi::wxArtProvider_Pop() }
    }
    /// Register new art provider and add it to the top of providers stack (i.e.
    ///
    /// [See `wxArtProvider::Push()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a6af5625b194423247ff3845e1f0f61d6)
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
    ///
    /// [See `wxArtProvider::PushBack()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a55fe50769f0aeae2cf49860ef0b168d2)
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
    ///
    /// [See `wxArtProvider::Remove()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a990035bf586ca89c93c509b704687cbc)
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
    ///
    /// [See `wxArtProvider::GetMessageBoxIconId()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#acf98fdabb4a521464d3a1cfdc6c8cb1e)
    fn get_message_box_icon_id(flags: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxArtProvider_GetMessageBoxIconId(flags)).into() }
    }
    /// Helper used by several generic classes: return the icon corresponding to the standard wxICON_INFORMATION/WARNING/ERROR/QUESTION flags (only one can be set)
    ///
    /// [See `wxArtProvider::GetMessageBoxIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_art_provider.html#a9660e47ce153346000bbcf80032f5ac4)
    fn get_message_box_icon(flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxArtProvider_GetMessageBoxIcon(flags)) }
    }
}

// wxAutoBufferedPaintDC
/// This trait represents C++ [`wxAutoBufferedPaintDC`](https://docs.wxwidgets.org/3.2/classwx_auto_buffered_paint_d_c.html) class's methods and inheritance.
///
/// See [`AutoBufferedPaintDCIsOwned`] documentation for the class usage.
pub trait AutoBufferedPaintDCMethods: BufferedPaintDCMethods {}
