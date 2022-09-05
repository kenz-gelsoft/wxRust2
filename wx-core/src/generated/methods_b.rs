use super::*;

// wxBannerWindow
/// This trait represents [C++ `wxBannerWindow` class](https://docs.wxwidgets.org/3.2/classwx_banner_window.html)'s methods and inheritance.
///
/// See [`BannerWindowIsOwned`] documentation for the class usage.
pub trait BannerWindowMethods: WindowMethods {
    /// Really create the banner window for the objects created using the default constructor.
    ///
    /// See [C++ `wxBannerWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html#abdf2347d2493376ed7366f964ce3ad4f).
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
    /// Provide the bitmap to use as background.
    ///
    /// See [C++ `wxBannerWindow::SetBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html#a741a8c932163acebcf3070de391d46df).
    fn set_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxBannerWindow_SetBitmap(self.as_ptr(), bmp)
        }
    }
    /// Set the text to display.
    ///
    /// See [C++ `wxBannerWindow::SetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html#aa2dcae1c87dbf15486f34042cae575f6).
    fn set_text(&self, title: &str, message: &str) {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxBannerWindow_SetText(self.as_ptr(), title, message)
        }
    }
    /// Set the colours between which the gradient runs.
    ///
    /// See [C++ `wxBannerWindow::SetGradient()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html#a48920916010216f51a748f4cf026520d).
    fn set_gradient<C: ColourMethods, C2: ColourMethods>(&self, start: &C, end: &C2) {
        unsafe {
            let start = start.as_ptr();
            let end = end.as_ptr();
            ffi::wxBannerWindow_SetGradient(self.as_ptr(), start, end)
        }
    }
}

// wxBitmap
/// This trait represents [C++ `wxBitmap` class](https://docs.wxwidgets.org/3.2/classwx_bitmap.html)'s methods and inheritance.
///
/// See [`BitmapIsOwned`] documentation for the class usage.
pub trait BitmapMethods: GDIObjectMethods {
    // DTOR: fn ~wxBitmap()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    /// Creates an image from a platform-dependent bitmap.
    ///
    /// See [C++ `wxBitmap::ConvertToImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#aad1c403e6e9148835907e3970a08e9bf).
    fn convert_to_image(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxBitmap_ConvertToImage(self.as_ptr())) }
    }
    /// Creates the bitmap from an icon.
    ///
    /// See [C++ `wxBitmap::CopyFromIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#afd23ea63dc5fb72cc806652ba4d6cf9b).
    fn copy_from_icon<I: IconMethods>(&self, icon: &I) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxBitmap_CopyFromIcon(self.as_ptr(), icon)
        }
    }
    /// Creates a fresh bitmap.
    ///
    /// See [C++ `wxBitmap::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#aa793ed387091a03f30060dd6e876bea5).
    fn create_int_int(&self, width: c_int, height: c_int, depth: c_int) -> bool {
        unsafe { ffi::wxBitmap_Create(self.as_ptr(), width, height, depth) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxBitmap::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a5b9cc5283dba021db3de367701497a73).
    fn create_size<S: SizeMethods>(&self, sz: &S, depth: c_int) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxBitmap_Create1(self.as_ptr(), sz, depth)
        }
    }
    /// Create a bitmap compatible with the given DC, inheriting its magnification factor.
    ///
    /// See [C++ `wxBitmap::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a4e9756e55431e4a87e316311f5c3e1fa).
    fn create_int_dc<D: DCMethods>(&self, width: c_int, height: c_int, dc: &D) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxBitmap_Create2(self.as_ptr(), width, height, dc)
        }
    }
    /// Create a bitmap specifying its size in DPI-independent pixels and the scale factor to use.
    ///
    /// See [C++ `wxBitmap::CreateWithDIPSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a975c964214c3b77f1d639c1c85ebc4a5).
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxBitmap::CreateWithDIPSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#ab17211bab9bc8b280edac7060807c335).
    fn create_with_dip_size_int(
        &self,
        width: c_int,
        height: c_int,
        scale: c_double,
        depth: c_int,
    ) -> bool {
        unsafe { ffi::wxBitmap_CreateWithDIPSize1(self.as_ptr(), width, height, scale, depth) }
    }
    /// Create a bitmap with a scale factor.
    ///
    /// See [C++ `wxBitmap::CreateScaled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#af43920a63f540c71021bceda4a036864).
    fn create_scaled(
        &self,
        width: c_int,
        height: c_int,
        depth: c_int,
        logical_scale: c_double,
    ) -> bool {
        unsafe { ffi::wxBitmap_CreateScaled(self.as_ptr(), width, height, depth, logical_scale) }
    }
    /// Gets the colour depth of the bitmap.
    ///
    /// See [C++ `wxBitmap::GetDepth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#af2c006d2c9236efcfe0eeb6768f8c779).
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetDepth(self.as_ptr()) }
    }
    /// Returns the size of bitmap in DPI-independent units.
    ///
    /// See [C++ `wxBitmap::GetDIPSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a2392bcd1b9994f57a3a8da583c3a9fec).
    fn get_dip_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetDIPSize(self.as_ptr())) }
    }
    /// Returns the height of the bitmap in physical pixels.
    ///
    /// See [C++ `wxBitmap::GetHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a61b416cd608708cf823c267386d2176f).
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetHeight(self.as_ptr()) }
    }
    /// Returns the height of the bitmap in logical pixels.
    ///
    /// See [C++ `wxBitmap::GetLogicalHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a95b2ea14a76a39beae31a35b0829a620).
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalHeight(self.as_ptr()) }
    }
    /// Returns the size of the bitmap in logical pixels.
    ///
    /// See [C++ `wxBitmap::GetLogicalSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#ab8db6da1c0bf12acc77975f4b7783e99).
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetLogicalSize(self.as_ptr())) }
    }
    /// Returns the width of the bitmap in logical pixels.
    ///
    /// See [C++ `wxBitmap::GetLogicalWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a181afefc33f5c25e809a4b52225f0fae).
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalWidth(self.as_ptr()) }
    }
    /// Gets the associated mask (if any) which may have been loaded from a file or set for the bitmap.
    ///
    /// See [C++ `wxBitmap::GetMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#aa978a534193f3e21f933b1dba73fbfec).
    fn get_mask(&self) -> Option<MaskIsOwned<false>> {
        unsafe { Mask::option_from(ffi::wxBitmap_GetMask(self.as_ptr())) }
    }
    /// Gets the associated palette (if any) which may have been loaded from a file or set for the bitmap.
    ///
    /// See [C++ `wxBitmap::GetPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a3ed8209f53fe2f843ad8b00c679a8704).
    fn get_palette(&self) -> Option<PaletteIsOwned<false>> {
        unsafe { Palette::option_from(ffi::wxBitmap_GetPalette(self.as_ptr())) }
    }
    /// Returns a sub bitmap of the current one as long as the rect belongs entirely to the bitmap.
    ///
    /// See [C++ `wxBitmap::GetSubBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#aa76912661aa447b74a6ccd7a2e6f99fa).
    fn get_sub_bitmap<R: RectMethods>(&self, rect: &R) -> Bitmap {
        unsafe {
            let rect = rect.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmap_GetSubBitmap(self.as_ptr(), rect))
        }
    }
    /// Returns the scale factor of this bitmap.
    ///
    /// See [C++ `wxBitmap::GetScaleFactor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a51c6d2d3493d5c98a8582d729968d791).
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaleFactor(self.as_ptr()) }
    }
    /// Returns the height of the bitmap in logical pixels.
    ///
    /// See [C++ `wxBitmap::GetScaledHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#ae80e93af1127e4289d0e365d868e9f5d).
    fn get_scaled_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledHeight(self.as_ptr()) }
    }
    /// Returns the size of the bitmap in logical pixels.
    ///
    /// See [C++ `wxBitmap::GetScaledSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a933bb01a01a5cf5d72cf14c3d29c59b3).
    fn get_scaled_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetScaledSize(self.as_ptr())) }
    }
    /// Returns the width of the bitmap in logical pixels.
    ///
    /// See [C++ `wxBitmap::GetScaledWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#ae4a776d78521c18598cbedf36ea6e80c).
    fn get_scaled_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledWidth(self.as_ptr()) }
    }
    /// Returns the size of the bitmap in physical pixels.
    ///
    /// See [C++ `wxBitmap::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#aef5992d9e39232769ad17cdf768a50e0).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetSize(self.as_ptr())) }
    }
    /// Returns the width of the bitmap in physical pixels.
    ///
    /// See [C++ `wxBitmap::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a4b65e32ca794aa762052d0704f81e18d).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetWidth(self.as_ptr()) }
    }
    /// Returns true if the bitmap has an alpha channel.
    ///
    /// See [C++ `wxBitmap::HasAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#aa6ab24d3d5a5de12842c1e9dedaff2fd).
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxBitmap_HasAlpha(self.as_ptr()) }
    }
    /// Returns true if bitmap data is present.
    ///
    /// See [C++ `wxBitmap::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a2ed1a3f550e679decfcd14b717e08ed7).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmap_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // BLOCKED: fn ResetAlpha()
    // NOT_SUPPORTED: fn SaveFile()
    // BLOCKED: fn SetDepth()
    // BLOCKED: fn SetHeight()
    /// Sets the bitmap scale factor.
    ///
    /// See [C++ `wxBitmap::SetScaleFactor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a096efa16cbdecd6ad9ebb65ce3b65407).
    fn set_scale_factor(&self, scale: c_double) {
        unsafe { ffi::wxBitmap_SetScaleFactor(self.as_ptr(), scale) }
    }
    /// Sets the mask for this bitmap.
    ///
    /// See [C++ `wxBitmap::SetMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a93ca840fdf8cd761e7ad419a90626ac4).
    fn set_mask<M: MaskMethods>(&self, mask: Option<&M>) {
        unsafe {
            let mask = match mask {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmap_SetMask(self.as_ptr(), mask)
        }
    }
    /// Sets the associated palette.
    ///
    /// See [C++ `wxBitmap::SetPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a57d5e5389c7603ecb3edf822bc5eaf9c).
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxBitmap_SetPalette(self.as_ptr(), palette)
        }
    }
    // BLOCKED: fn SetWidth()
    // BLOCKED: fn UseAlpha()
    /// Adds a handler to the end of the static list of format handlers.
    ///
    /// See [C++ `wxBitmap::AddHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a6418825ad15574229188e0c5f97a4f3a).
    fn add_handler<B: BitmapHandlerMethods>(handler: Option<&B>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmap_AddHandler(handler)
        }
    }
    /// Deletes all bitmap handlers.
    ///
    /// See [C++ `wxBitmap::CleanUpHandlers()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a62381d78867e552f713ae712fd23bc81).
    fn clean_up_handlers() {
        unsafe { ffi::wxBitmap_CleanUpHandlers() }
    }
    /// Finds the handler with the given name.
    ///
    /// See [C++ `wxBitmap::FindHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a7351c37e4e13cc1ebab1d788cafe0139).
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
    /// Adds the standard bitmap format handlers, which, depending on wxWidgets configuration, can be handlers for Windows bitmap, Windows bitmap resource, and XPM.
    ///
    /// See [C++ `wxBitmap::InitStandardHandlers()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#ac2626d4a4ddca5fa54756d7f942c96a1).
    fn init_standard_handlers() {
        unsafe { ffi::wxBitmap_InitStandardHandlers() }
    }
    /// Adds a handler at the start of the static list of format handlers.
    ///
    /// See [C++ `wxBitmap::InsertHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a228a76708d605d22ed82c889f15859a4).
    fn insert_handler<B: BitmapHandlerMethods>(handler: Option<&B>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBitmap_InsertHandler(handler)
        }
    }
    /// Loads a bitmap from the memory containing image data in PNG format.
    ///
    /// See [C++ `wxBitmap::NewFromPNGData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a2048897970d5e845040fcfc0259e1be1).
    fn new_from_png_data(data: *const c_void, size: usize) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmap_NewFromPNGData(data, size)) }
    }
    /// Finds the handler with the given name, and removes it.
    ///
    /// See [C++ `wxBitmap::RemoveHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a0316baef61f8f9cc3ba05bd8e52f1670).
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmap_RemoveHandler(name)
        }
    }
    /// Rescale the given bitmap to the requested size.
    ///
    /// See [C++ `wxBitmap::Rescale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a763538fd40a30a6237c795bdb486f14b).
    fn rescale<B: BitmapMethods, S: SizeMethods>(bmp: &B, size_needed: &S) {
        unsafe {
            let bmp = bmp.as_ptr();
            let size_needed = size_needed.as_ptr();
            ffi::wxBitmap_Rescale(bmp, size_needed)
        }
    }
}

// wxBitmapBundle
/// This trait represents [C++ `wxBitmapBundle` class](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html)'s methods and inheritance.
///
/// See [`BitmapBundleIsOwned`] documentation for the class usage.
pub trait BitmapBundleMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    /// Clear the existing bundle contents.
    ///
    /// See [C++ `wxBitmapBundle::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#abf7f9c0159c67f3efea55a964b4fe5d5).
    fn clear(&self) {
        unsafe { ffi::wxBitmapBundle_Clear(self.as_ptr()) }
    }
    /// Check if bitmap bundle is non-empty.
    ///
    /// See [C++ `wxBitmapBundle::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a4055896cbaba793a70ef23668b7250bd).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmapBundle_IsOk(self.as_ptr()) }
    }
    /// Get the size of the bitmap represented by this bundle in default resolution or, equivalently, at 100% scaling.
    ///
    /// See [C++ `wxBitmapBundle::GetDefaultSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a577a349a985b8c4b477288673072bcdc).
    fn get_default_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapBundle_GetDefaultSize(self.as_ptr())) }
    }
    /// Get the size that would be best to use for this bundle at the given DPI scaling factor.
    ///
    /// See [C++ `wxBitmapBundle::GetPreferredBitmapSizeAtScale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#ab1b530e0df1b57afe08b2615a2d41ab4).
    fn get_preferred_bitmap_size_at_scale(&self, scale: c_double) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredBitmapSizeAtScale(
                self.as_ptr(),
                scale,
            ))
        }
    }
    /// Get the size that would be best to use for this bundle at the DPI scaling factor used by the given window.
    ///
    /// See [C++ `wxBitmapBundle::GetPreferredBitmapSizeFor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a718bffe698fa7093570968817e9bcf90).
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
    /// Get the size that would be best to use for this bundle at the DPI scaling factor used by the given window in logical size.
    ///
    /// See [C++ `wxBitmapBundle::GetPreferredLogicalSizeFor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a7334c0d48ad8b53a21d315d6eaed75b8).
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
    /// Get bitmap of the specified size, creating a new bitmap from the closest available size by rescaling it if necessary.
    ///
    /// See [C++ `wxBitmapBundle::GetBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a4b05aed3535a217093cff3f604bbe918).
    fn get_bitmap<S: SizeMethods>(&self, size: &S) -> Bitmap {
        unsafe {
            let size = size.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmap(self.as_ptr(), size))
        }
    }
    /// Get bitmap of the size appropriate for the DPI scaling used by the given window.
    ///
    /// See [C++ `wxBitmapBundle::GetBitmapFor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a2b427e6ad729074b042acdd805c91e46).
    fn get_bitmap_for<W: WindowMethods>(&self, window: Option<&W>) -> Bitmap {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmapFor(self.as_ptr(), window))
        }
    }
    /// Get icon of the specified size.
    ///
    /// See [C++ `wxBitmapBundle::GetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a8da4786c0260adb65d5a4caedd7ef944).
    fn get_icon<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxBitmapBundle_GetIcon(self.as_ptr(), size))
        }
    }
    /// Get icon of the size appropriate for the DPI scaling used by the given window.
    ///
    /// See [C++ `wxBitmapBundle::GetIconFor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a8306317d34c3abc57254834998224498).
    fn get_icon_for<W: WindowMethods>(&self, window: Option<&W>) -> Icon {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Icon::from_ptr(ffi::wxBitmapBundle_GetIconFor(self.as_ptr(), window))
        }
    }
    /// Check if the two bundles refer to the same object.
    ///
    /// See [C++ `wxBitmapBundle::IsSameAs()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a307c73ddb5c04414303418146b54e40e).
    fn is_same_as<B: BitmapBundleMethods>(&self, other: &B) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxBitmapBundle_IsSameAs(self.as_ptr(), other)
        }
    }
    // BLOCKED: fn FromBitmaps()
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxBitmapBundle::FromBitmaps()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a2ac52084fc3b501fbabd3c952f264d30).
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
    /// Create a bundle from a single bitmap.
    ///
    /// See [C++ `wxBitmapBundle::FromBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a17e765ddbd5b3ffa3946be04b7ea5137).
    fn from_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundle {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromBitmap(bitmap))
        }
    }
    /// Create a bundle from an icon bundle.
    ///
    /// See [C++ `wxBitmapBundle::FromIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a1208f457855de2a641cf5c523cf287d1).
    fn from_icon_bundle<I: IconBundleMethods>(icon_bundle: &I) -> BitmapBundle {
        unsafe {
            let icon_bundle = icon_bundle.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromIconBundle(icon_bundle))
        }
    }
    /// Create a bundle from a single image.
    ///
    /// See [C++ `wxBitmapBundle::FromImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a01b30c8f19d1b36addf9553a3e33833a).
    fn from_image<I: ImageMethods>(image: &I) -> BitmapBundle {
        unsafe {
            let image = image.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImage(image))
        }
    }
    /// Create a bundle from a custom bitmap bundle implementation.
    ///
    /// See [C++ `wxBitmapBundle::FromImpl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#ab1d0babc46f3176bd4b63ce2750c89d6).
    fn from_impl(impl_: *mut c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImpl(impl_)) }
    }
    /// Create a bundle from the bitmaps in the application resources.
    ///
    /// See [C++ `wxBitmapBundle::FromResources()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#af5521b4db71cbca163f54ecb1d2ad69d).
    fn from_resources(name: &str) -> BitmapBundle {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromResources(name))
        }
    }
    /// Create a bundle from bitmaps stored as files.
    ///
    /// See [C++ `wxBitmapBundle::FromFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#aef5f4778d6fd9463128d68d9e280c67a).
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxBitmapBundle::FromFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#af5f2d82a22c4a5903dc95569f8556e3e).
    fn from_files(fullpathname: &str) -> BitmapBundle {
        unsafe {
            let fullpathname = WxString::from(fullpathname);
            let fullpathname = fullpathname.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromFiles1(fullpathname))
        }
    }
    // BLOCKED: fn FromSVG()
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxBitmapBundle::FromSVG()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#ad7add757fd27af9ad000750cef9c61f9).
    fn from_svg<S: SizeMethods>(data: *const c_void, size_def: &S) -> BitmapBundle {
        unsafe {
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVG1(data, size_def))
        }
    }
    /// Create a bundle from the SVG image loaded from the given file.
    ///
    /// See [C++ `wxBitmapBundle::FromSVGFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#ac0b99263576f8c11815ab60cfbb573ab).
    fn from_svg_file<S: SizeMethods>(path: &str, size_def: &S) -> BitmapBundle {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVGFile(path, size_def))
        }
    }
    /// Create a bundle from the SVG image loaded from an application resource.
    ///
    /// See [C++ `wxBitmapBundle::FromSVGResource()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#ac0b9721c5265240ec7a9728ba6be0fe1).
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
/// This trait represents [C++ `wxBitmapButton` class](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html)'s methods and inheritance.
///
/// See [`BitmapButtonIsOwned`] documentation for the class usage.
pub trait BitmapButtonMethods: ButtonMethods {
    /// Button creation function for two-step creation.
    ///
    /// See [C++ `wxBitmapButton::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html#aefa9135ab196a0d2694436692bb32972).
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
    /// Creation function for two-step creation of "Close" button.
    ///
    /// See [C++ `wxBitmapButton::CreateCloseButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html#a558bf8e66279a784260950d9e585baf7).
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
    /// Helper function creating a standard-looking "Close" button.
    ///
    /// See [C++ `wxBitmapButton::NewCloseButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html#a1807c52c68f2dbeeea42cdd8302356a6).
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
/// This trait represents [C++ `wxBitmapComboBox` class](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html)'s methods and inheritance.
///
/// See [`BitmapComboBoxIsOwned`] documentation for the class usage.
pub trait BitmapComboBoxMethods: ComboBoxMethods {
    // DTOR: fn ~wxBitmapComboBox()
    /// Adds the item to the end of the combo box.
    ///
    /// See [C++ `wxBitmapComboBox::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a79af57f3911325b5a61cb31ea183ff08).
    fn append<B: BitmapMethods>(&self, item: &str, bitmap: &B) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Append(self.as_ptr(), item, bitmap)
        }
    }
    /// Adds the item to the end of the combo box, associating the given untyped, client data pointer clientData with the item.
    ///
    /// See [C++ `wxBitmapComboBox::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a16742fef1682808010c547978c4a4ebb).
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
    /// Adds the item to the end of the combo box, associating the given typed client data pointer clientData with the item.
    ///
    /// See [C++ `wxBitmapComboBox::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a27b43b346a164dbfcf689aaa3e2a5104).
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
    /// Returns the size of the bitmaps used in the combo box.
    ///
    /// See [C++ `wxBitmapComboBox::GetBitmapSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a31b0aa450c5753153b3e10976c49ca80).
    fn get_bitmap_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapComboBox_GetBitmapSize(self.as_ptr())) }
    }
    /// Returns the bitmap of the item with the given index.
    ///
    /// See [C++ `wxBitmapComboBox::GetItemBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a15da37175af6d51c32ce7c2b94802942).
    fn get_item_bitmap(&self, n: c_uint) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmapComboBox_GetItemBitmap(self.as_ptr(), n)) }
    }
    /// Inserts the item into the list before pos.
    ///
    /// See [C++ `wxBitmapComboBox::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#ae5d7c2193eb93aef156b901a3f78be7d).
    fn insert<B: BitmapMethods>(&self, item: &str, bitmap: &B, pos: c_uint) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Insert(self.as_ptr(), item, bitmap, pos)
        }
    }
    /// Inserts the item into the list before pos, associating the given untyped, client data pointer with the item.
    ///
    /// See [C++ `wxBitmapComboBox::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a28fb428bb8a950f82d365ec7be303913).
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
    /// Inserts the item into the list before pos, associating the given typed client data pointer with the item.
    ///
    /// See [C++ `wxBitmapComboBox::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#aaf177238a6a7ef6beb56cde669042736).
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
    /// Sets the bitmap for the given item.
    ///
    /// See [C++ `wxBitmapComboBox::SetItemBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a7a08117b20faeb0293b42cd0b01b10b5).
    fn set_item_bitmap<B: BitmapBundleMethods>(&self, n: c_uint, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_SetItemBitmap(self.as_ptr(), n, bitmap)
        }
    }
}

// wxBitmapDataObject
/// This trait represents [C++ `wxBitmapDataObject` class](https://docs.wxwidgets.org/3.2/classwx_bitmap_data_object.html)'s methods and inheritance.
///
/// See [`BitmapDataObjectIsOwned`] documentation for the class usage.
pub trait BitmapDataObjectMethods: DataObjectSimpleMethods {
    /// Returns the bitmap associated with the data object.
    ///
    /// See [C++ `wxBitmapDataObject::GetBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_data_object.html#a7629dc0b45e537ebe84f18d8f27237f6).
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmapDataObject_GetBitmap(self.as_ptr())) }
    }
    /// Sets the bitmap associated with the data object.
    ///
    /// See [C++ `wxBitmapDataObject::SetBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_data_object.html#ab08fe86cb5428c632aff7854dcc99630).
    fn set_bitmap<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapDataObject_SetBitmap(self.as_ptr(), bitmap)
        }
    }
}

// wxBitmapHandler
/// This trait represents [C++ `wxBitmapHandler` class](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html)'s methods and inheritance.
///
/// See [`BitmapHandlerIsOwned`] documentation for the class usage.
pub trait BitmapHandlerMethods: ObjectMethods {
    // DTOR: fn ~wxBitmapHandler()
    // NOT_SUPPORTED: fn Create()
    /// Gets the file extension associated with this handler.
    ///
    /// See [C++ `wxBitmapHandler::GetExtension()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html#ad69951c7ac366cca99bde4705d872a0c).
    fn get_extension(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxBitmapHandler_GetExtension(self.as_ptr())).into() }
    }
    /// Gets the name of this handler.
    ///
    /// See [C++ `wxBitmapHandler::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html#a3ceadf665f2d4d31dfacad0bcd49834a).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxBitmapHandler_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn SaveFile()
    /// Sets the handler extension.
    ///
    /// See [C++ `wxBitmapHandler::SetExtension()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html#a0a416eea17524f4153a1ac5bb0294218).
    fn set_extension(&self, extension: &str) {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            ffi::wxBitmapHandler_SetExtension(self.as_ptr(), extension)
        }
    }
    /// Sets the handler name.
    ///
    /// See [C++ `wxBitmapHandler::SetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html#ae7b2cf15809b3940cbdd339b31dec25e).
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
/// This trait represents [C++ `wxBitmapToggleButton` class](https://docs.wxwidgets.org/3.2/classwx_bitmap_toggle_button.html)'s methods and inheritance.
///
/// See [`BitmapToggleButtonIsOwned`] documentation for the class usage.
pub trait BitmapToggleButtonMethods: ToggleButtonMethods {
    /// Create method for two-step construction.
    ///
    /// See [C++ `wxBitmapToggleButton::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_toggle_button.html#a976086d51b606af39432c5acdafa45ed).
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
/// This trait represents [C++ `wxBookCtrlBase` class](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html)'s methods and inheritance.
///
/// See [`BookCtrlBaseIsOwned`] documentation for the class usage.
pub trait BookCtrlBaseMethods: ControlMethods {
    /// Returns the image index for the given page.
    ///
    /// See [C++ `wxBookCtrlBase::GetPageImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a87f9cf7b1a5abc5cca716511277b39f8).
    fn get_page_image(&self, n_page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetPageImage(self.as_ptr(), n_page) }
    }
    /// Sets the image index for the given page.
    ///
    /// See [C++ `wxBookCtrlBase::SetPageImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a990cc596ea313b3c057314398085d0b3).
    fn set_page_image(&self, page: usize, image: c_int) -> bool {
        unsafe { ffi::wxBookCtrlBase_SetPageImage(self.as_ptr(), page, image) }
    }
    /// Returns the string for the given page.
    ///
    /// See [C++ `wxBookCtrlBase::GetPageText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a795e4f831ade0580a391ae876f69fb28).
    fn get_page_text(&self, n_page: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxBookCtrlBase_GetPageText(self.as_ptr(), n_page)).into() }
    }
    /// Sets the text for the given page.
    ///
    /// See [C++ `wxBookCtrlBase::SetPageText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#aacc118888433ae50194cde356dbf2d50).
    fn set_page_text(&self, page: usize, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxBookCtrlBase_SetPageText(self.as_ptr(), page, text)
        }
    }
    /// Returns the currently selected page, or wxNOT_FOUND if none was selected.
    ///
    /// See [C++ `wxBookCtrlBase::GetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a42b7d557ec288a1819b024613adf6315).
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetSelection(self.as_ptr()) }
    }
    /// Returns the currently selected page or NULL.
    ///
    /// See [C++ `wxBookCtrlBase::GetCurrentPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#aa1467ecccb3fb6d6cd5155f9680d52eb).
    fn get_current_page(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetCurrentPage(self.as_ptr())) }
    }
    /// Sets the selection to the given page, returning the previous selection.
    ///
    /// See [C++ `wxBookCtrlBase::SetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a522ca472343a07a262de92348e3f2d2c).
    fn set_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_SetSelection(self.as_ptr(), page) }
    }
    /// Cycles through the tabs.
    ///
    /// See [C++ `wxBookCtrlBase::AdvanceSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a8b5b18fbf0281e99d227e574b5ffc1ac).
    fn advance_selection(&self, forward: bool) {
        unsafe { ffi::wxBookCtrlBase_AdvanceSelection(self.as_ptr(), forward) }
    }
    /// Changes the selection to the given page, returning the previous selection.
    ///
    /// See [C++ `wxBookCtrlBase::ChangeSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#ab938512b16be50849c1c4888ffa6aa29).
    fn change_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_ChangeSelection(self.as_ptr(), page) }
    }
    /// Returns the index of the specified tab window or wxNOT_FOUND if not found.
    ///
    /// See [C++ `wxBookCtrlBase::FindPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a35050097fe659c923bc2f1c9d1b03942).
    fn find_page<W: WindowMethods>(&self, page: Option<&W>) -> c_int {
        unsafe {
            let page = match page {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxBookCtrlBase_FindPage(self.as_ptr(), page)
        }
    }
    /// Sets the width and height of the pages.
    ///
    /// See [C++ `wxBookCtrlBase::SetPageSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a7ab6ba69644452e74bf914b3c30a9e58).
    fn set_page_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxBookCtrlBase_SetPageSize(self.as_ptr(), size)
        }
    }
    /// Returns the index of the tab at the specified position or wxNOT_FOUND if none.
    ///
    /// See [C++ `wxBookCtrlBase::HitTest()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#acd103967eb5392e62447774cfc84faa2).
    fn hit_test<P: PointMethods>(&self, pt: &P, flags: *mut c_void) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxBookCtrlBase_HitTest(self.as_ptr(), pt, flags)
        }
    }
    /// Adds a new page.
    ///
    /// See [C++ `wxBookCtrlBase::AddPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#ab47f8935c3705a452fce7d292d8181dd).
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
    /// Deletes all pages.
    ///
    /// See [C++ `wxBookCtrlBase::DeleteAllPages()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a91c1d122585b48ae1b5eab04104210da).
    fn delete_all_pages(&self) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeleteAllPages(self.as_ptr()) }
    }
    /// Deletes the specified page, and the associated window.
    ///
    /// See [C++ `wxBookCtrlBase::DeletePage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a7cbc19f60daf15af7019de20561b29f4).
    fn delete_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeletePage(self.as_ptr(), page) }
    }
    /// Inserts a new page at the specified position.
    ///
    /// See [C++ `wxBookCtrlBase::InsertPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a2dcfe645ba865a3f44ebaf660a9ab0ed).
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
    /// Deletes the specified page, without deleting the associated window.
    ///
    /// See [C++ `wxBookCtrlBase::RemovePage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a3297ad25597b3e993aa2b7a310c5159c).
    fn remove_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_RemovePage(self.as_ptr(), page) }
    }
    /// Returns the number of pages in the control.
    ///
    /// See [C++ `wxBookCtrlBase::GetPageCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#aaaa9c1c87a2559a4a79ce81d79f0c5fc).
    fn get_page_count(&self) -> usize {
        unsafe { ffi::wxBookCtrlBase_GetPageCount(self.as_ptr()) }
    }
    /// Returns the window at the given page position.
    ///
    /// See [C++ `wxBookCtrlBase::GetPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#a5918b9f7cf6785657ffea939f9e99e90).
    fn get_page(&self, page: usize) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetPage(self.as_ptr(), page)) }
    }
}

// wxBookCtrlEvent
/// This trait represents [C++ `wxBookCtrlEvent` class](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_event.html)'s methods and inheritance.
///
/// See [`BookCtrlEventIsOwned`] documentation for the class usage.
pub trait BookCtrlEventMethods: NotifyEventMethods {
    /// Returns the page that was selected before the change, wxNOT_FOUND if none was selected.
    ///
    /// See [C++ `wxBookCtrlEvent::GetOldSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_event.html#a39976c808cca06bc5693d69055615dcd).
    fn get_old_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetOldSelection(self.as_ptr()) }
    }
    /// Sets the id of the page selected before the change.
    ///
    /// See [C++ `wxBookCtrlEvent::SetOldSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_event.html#abf4ac02fe4445dce418536b94fbebc84).
    fn set_old_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetOldSelection(self.as_ptr(), page) }
    }
    /// Sets the selection member variable.
    ///
    /// See [C++ `wxBookCtrlEvent::SetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_event.html#a7afa201992c65fe827b905d8941b1b66).
    fn set_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetSelection(self.as_ptr(), page) }
    }
}

// wxBoxSizer
/// This trait represents [C++ `wxBoxSizer` class](https://docs.wxwidgets.org/3.2/classwx_box_sizer.html)'s methods and inheritance.
///
/// See [`BoxSizerIsOwned`] documentation for the class usage.
pub trait BoxSizerMethods: SizerMethods {
    /// Returns the orientation of the box sizer, either wxVERTICAL or wxHORIZONTAL.
    ///
    /// See [C++ `wxBoxSizer::GetOrientation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_box_sizer.html#a991ac9c5319f34e0a60ac29c70ae77af).
    fn get_orientation(&self) -> c_int {
        unsafe { ffi::wxBoxSizer_GetOrientation(self.as_ptr()) }
    }
    /// Sets the orientation of the box sizer, either wxVERTICAL or wxHORIZONTAL.
    ///
    /// See [C++ `wxBoxSizer::SetOrientation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_box_sizer.html#a62b873c88ed118bbf9497d53fd442ad9).
    fn set_orientation(&self, orient: c_int) {
        unsafe { ffi::wxBoxSizer_SetOrientation(self.as_ptr(), orient) }
    }
}

// wxBrush
/// This trait represents [C++ `wxBrush` class](https://docs.wxwidgets.org/3.2/classwx_brush.html)'s methods and inheritance.
///
/// See [`BrushIsOwned`] documentation for the class usage.
pub trait BrushMethods: GDIObjectMethods {
    // DTOR: fn ~wxBrush()
    /// Returns a reference to the brush colour.
    ///
    /// See [C++ `wxBrush::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#ad544080909640917f771f81254cf5865).
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxBrush_GetColour(self.as_ptr())) }
    }
    /// Gets a pointer to the stipple bitmap.
    ///
    /// See [C++ `wxBrush::GetStipple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a9f6d3e31079107f82ce28b40ea3fd04f).
    fn get_stipple(&self) -> Option<BitmapIsOwned<false>> {
        unsafe { Bitmap::option_from(ffi::wxBrush_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    /// Returns true if the style of the brush is any of hatched fills.
    ///
    /// See [C++ `wxBrush::IsHatch()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a52b6d37c94fd2490f271a89bd24e2532).
    fn is_hatch(&self) -> bool {
        unsafe { ffi::wxBrush_IsHatch(self.as_ptr()) }
    }
    /// Returns true if the brush is initialised.
    ///
    /// See [C++ `wxBrush::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a6844b1c23fccb5963aeb497367518bd7).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBrush_IsOk(self.as_ptr()) }
    }
    /// Returns true if the brush is a valid non-transparent brush.
    ///
    /// See [C++ `wxBrush::IsNonTransparent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a1d6f82269bc0ce604251fec8f4ed847d).
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxBrush_IsNonTransparent(self.as_ptr()) }
    }
    /// Returns true if the brush is transparent.
    ///
    /// See [C++ `wxBrush::IsTransparent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a4a7c2f442e2e7d0d928f64cf126a6850).
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxBrush_IsTransparent(self.as_ptr()) }
    }
    /// Sets the brush colour using red, green and blue values.
    ///
    /// See [C++ `wxBrush::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#ab1b04a52bb694853168a88c4deab53b1).
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxBrush_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    /// Sets the stipple bitmap.
    ///
    /// See [C++ `wxBrush::SetStipple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a7da8162eda06d522fcde31ae51b5952c).
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
/// This trait represents [C++ `wxBrushList` class](https://docs.wxwidgets.org/3.2/classwx_brush_list.html)'s methods and inheritance.
///
/// See [`BrushListIsOwned`] documentation for the class usage.
pub trait BrushListMethods: WxRustMethods {
    // NOT_SUPPORTED: fn FindOrCreateBrush()
}

// wxBufferedDC
/// This trait represents [C++ `wxBufferedDC` class](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html)'s methods and inheritance.
///
/// See [`BufferedDCIsOwned`] documentation for the class usage.
pub trait BufferedDCMethods: MemoryDCMethods {
    // DTOR: fn ~wxBufferedDC()
    /// Initializes the object created using the default constructor.
    ///
    /// See [C++ `wxBufferedDC::Init()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#a9a77fe1f6a944e2140c69f1dbee57ebb).
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
    ///
    /// See [C++ `wxBufferedDC::Init()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#a4023e4947950aa856d84b8d6d99025fa).
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
    /// Blits the buffer to the dc, and detaches the dc from the buffer (so it can be effectively used once only).
    ///
    /// See [C++ `wxBufferedDC::UnMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#a6af93506345c418732e857c5551dc30b).
    fn un_mask(&self) {
        unsafe { ffi::wxBufferedDC_UnMask(self.as_ptr()) }
    }
    /// Set the style.
    ///
    /// See [C++ `wxBufferedDC::SetStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#ae64d3425a6dd712da0f22f8ac2a4e8f7).
    fn set_style(&self, style: c_int) {
        unsafe { ffi::wxBufferedDC_SetStyle(self.as_ptr(), style) }
    }
    /// Get the style.
    ///
    /// See [C++ `wxBufferedDC::GetStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#a10034cd62c408a0fc082b7138dd5de74).
    fn get_style(&self) -> c_int {
        unsafe { ffi::wxBufferedDC_GetStyle(self.as_ptr()) }
    }
}

// wxBufferedPaintDC
/// This trait represents [C++ `wxBufferedPaintDC` class](https://docs.wxwidgets.org/3.2/classwx_buffered_paint_d_c.html)'s methods and inheritance.
///
/// See [`BufferedPaintDCIsOwned`] documentation for the class usage.
pub trait BufferedPaintDCMethods: BufferedDCMethods {
    // DTOR: fn ~wxBufferedPaintDC()
}

// wxBusyCursor
/// This trait represents [C++ `wxBusyCursor` class](https://docs.wxwidgets.org/3.2/classwx_busy_cursor.html)'s methods and inheritance.
///
/// See [`BusyCursorIsOwned`] documentation for the class usage.
pub trait BusyCursorMethods: WxRustMethods {
    // DTOR: fn ~wxBusyCursor()
}

// wxBusyInfo
/// This trait represents [C++ `wxBusyInfo` class](https://docs.wxwidgets.org/3.2/classwx_busy_info.html)'s methods and inheritance.
///
/// See [`BusyInfoIsOwned`] documentation for the class usage.
pub trait BusyInfoMethods: WxRustMethods {
    /// Update the information text.
    ///
    /// See [C++ `wxBusyInfo::UpdateText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_busy_info.html#ad88b7c7da6843fc904640d3d3d80a4f3).
    fn update_text(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxBusyInfo_UpdateText(self.as_ptr(), str)
        }
    }
    /// Same as UpdateText() but doesn't interpret the string as containing markup.
    ///
    /// See [C++ `wxBusyInfo::UpdateLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_busy_info.html#a24b955b0479ee37f0eb462e54871c577).
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
/// This trait represents [C++ `wxButton` class](https://docs.wxwidgets.org/3.2/classwx_button.html)'s methods and inheritance.
///
/// See [`ButtonIsOwned`] documentation for the class usage.
pub trait ButtonMethods: AnyButtonMethods {
    /// Button creation function for two-step creation.
    ///
    /// See [C++ `wxButton::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#af7f5f8316423b6ae17fbd06b3371610b).
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
    /// Returns true if an authentication needed symbol is displayed on the button.
    ///
    /// See [C++ `wxButton::GetAuthNeeded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#a7421e6fc4c646b21898e06305397606e).
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr()) }
    }
    /// Sets whether an authentication needed symbol should be displayed on the button.
    ///
    /// See [C++ `wxButton::SetAuthNeeded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#a2e9c19ca18bda893653deb023e08c037).
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr(), needed) }
    }
    /// This sets the button to be the default item in its top-level window (e.g.
    ///
    /// See [C++ `wxButton::SetDefault()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#a2f15eef9a7c15debd17f50f3337d7c4e).
    fn set_default(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxButton_SetDefault(self.as_ptr())) }
    }
    /// Returns the default size for the buttons.
    ///
    /// See [C++ `wxButton::GetDefaultSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#aa14f01c4c176b0fa875b45af65870cc9).
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
