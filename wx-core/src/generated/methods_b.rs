use super::*;

// wxBannerWindow
pub trait BannerWindowMethods: WindowMethods {
    /// Really create the banner window for the objects created using the default constructor.
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
    fn set_bitmap<B: BitmapBundleMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxBannerWindow_SetBitmap(self.as_ptr(), bmp)
        }
    }
    /// Set the text to display.
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
    /// Creates an image from a platform-dependent bitmap.
    fn convert_to_image(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxBitmap_ConvertToImage(self.as_ptr())) }
    }
    /// Creates the bitmap from an icon.
    fn copy_from_icon<I: IconMethods>(&self, icon: &I) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxBitmap_CopyFromIcon(self.as_ptr(), icon)
        }
    }
    /// Creates a fresh bitmap.
    fn create_int_int(&self, width: c_int, height: c_int, depth: c_int) -> bool {
        unsafe { ffi::wxBitmap_Create(self.as_ptr(), width, height, depth) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn create_size<S: SizeMethods>(&self, sz: &S, depth: c_int) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxBitmap_Create1(self.as_ptr(), sz, depth)
        }
    }
    /// Create a bitmap compatible with the given DC, inheriting its magnification factor.
    fn create_int_dc<D: DCMethods>(&self, width: c_int, height: c_int, dc: &D) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxBitmap_Create2(self.as_ptr(), width, height, dc)
        }
    }
    /// Create a bitmap specifying its size in DPI-independent pixels and the scale factor to use.
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
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetDepth(self.as_ptr()) }
    }
    /// Returns the size of bitmap in DPI-independent units.
    fn get_dip_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetDIPSize(self.as_ptr())) }
    }
    /// Returns the height of the bitmap in physical pixels.
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetHeight(self.as_ptr()) }
    }
    /// Returns the height of the bitmap in logical pixels.
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalHeight(self.as_ptr()) }
    }
    /// Returns the size of the bitmap in logical pixels.
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetLogicalSize(self.as_ptr())) }
    }
    /// Returns the width of the bitmap in logical pixels.
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetLogicalWidth(self.as_ptr()) }
    }
    /// Gets the associated mask (if any) which may have been loaded from a file or set for the bitmap.
    fn get_mask(&self) -> Option<MaskIsOwned<false>> {
        unsafe { Mask::option_from(ffi::wxBitmap_GetMask(self.as_ptr())) }
    }
    /// Gets the associated palette (if any) which may have been loaded from a file or set for the bitmap.
    fn get_palette(&self) -> Option<PaletteIsOwned<false>> {
        unsafe { Palette::option_from(ffi::wxBitmap_GetPalette(self.as_ptr())) }
    }
    /// Returns a sub bitmap of the current one as long as the rect belongs entirely to the bitmap.
    fn get_sub_bitmap<R: RectMethods>(&self, rect: &R) -> Bitmap {
        unsafe {
            let rect = rect.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmap_GetSubBitmap(self.as_ptr(), rect))
        }
    }
    /// Returns the scale factor of this bitmap.
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaleFactor(self.as_ptr()) }
    }
    /// Returns the height of the bitmap in logical pixels.
    fn get_scaled_height(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledHeight(self.as_ptr()) }
    }
    /// Returns the size of the bitmap in logical pixels.
    fn get_scaled_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetScaledSize(self.as_ptr())) }
    }
    /// Returns the width of the bitmap in logical pixels.
    fn get_scaled_width(&self) -> c_double {
        unsafe { ffi::wxBitmap_GetScaledWidth(self.as_ptr()) }
    }
    /// Returns the size of the bitmap in physical pixels.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmap_GetSize(self.as_ptr())) }
    }
    /// Returns the width of the bitmap in physical pixels.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxBitmap_GetWidth(self.as_ptr()) }
    }
    /// Returns true if the bitmap has an alpha channel.
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxBitmap_HasAlpha(self.as_ptr()) }
    }
    /// Returns true if bitmap data is present.
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
    /// Sets the bitmap scale factor.
    fn set_scale_factor(&self, scale: c_double) {
        unsafe { ffi::wxBitmap_SetScaleFactor(self.as_ptr(), scale) }
    }
    /// Sets the mask for this bitmap.
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
    /// Adds a handler to the end of the static list of format handlers.
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
    fn clean_up_handlers() {
        unsafe { ffi::wxBitmap_CleanUpHandlers() }
    }
    /// Finds the handler with the given name.
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
    fn init_standard_handlers() {
        unsafe { ffi::wxBitmap_InitStandardHandlers() }
    }
    /// Adds a handler at the start of the static list of format handlers.
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
    fn new_from_png_data(data: *const c_void, size: usize) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmap_NewFromPNGData(data, size)) }
    }
    /// Finds the handler with the given name, and removes it.
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxBitmap_RemoveHandler(name)
        }
    }
    /// Rescale the given bitmap to the requested size.
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
    /// Clear the existing bundle contents.
    fn clear(&self) {
        unsafe { ffi::wxBitmapBundle_Clear(self.as_ptr()) }
    }
    /// Check if bitmap bundle is non-empty.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBitmapBundle_IsOk(self.as_ptr()) }
    }
    /// Get the size of the bitmap represented by this bundle in default resolution or, equivalently, at 100% scaling.
    fn get_default_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapBundle_GetDefaultSize(self.as_ptr())) }
    }
    /// Get the size that would be best to use for this bundle at the given DPI scaling factor.
    fn get_preferred_bitmap_size_at_scale(&self, scale: c_double) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxBitmapBundle_GetPreferredBitmapSizeAtScale(
                self.as_ptr(),
                scale,
            ))
        }
    }
    /// Get the size that would be best to use for this bundle at the DPI scaling factor used by the given window.
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
    fn get_bitmap<S: SizeMethods>(&self, size: &S) -> Bitmap {
        unsafe {
            let size = size.as_ptr();
            Bitmap::from_ptr(ffi::wxBitmapBundle_GetBitmap(self.as_ptr(), size))
        }
    }
    /// Get bitmap of the size appropriate for the DPI scaling used by the given window.
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
    fn get_icon<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxBitmapBundle_GetIcon(self.as_ptr(), size))
        }
    }
    /// Get icon of the size appropriate for the DPI scaling used by the given window.
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
    fn is_same_as<B: BitmapBundleMethods>(&self, other: &B) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxBitmapBundle_IsSameAs(self.as_ptr(), other)
        }
    }
    // BLOCKED: fn FromBitmaps()
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    fn from_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundle {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromBitmap(bitmap))
        }
    }
    /// Create a bundle from an icon bundle.
    fn from_icon_bundle<I: IconBundleMethods>(icon_bundle: &I) -> BitmapBundle {
        unsafe {
            let icon_bundle = icon_bundle.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromIconBundle(icon_bundle))
        }
    }
    /// Create a bundle from a single image.
    fn from_image<I: ImageMethods>(image: &I) -> BitmapBundle {
        unsafe {
            let image = image.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImage(image))
        }
    }
    /// Create a bundle from a custom bitmap bundle implementation.
    fn from_impl(impl_: *mut c_void) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromImpl(impl_)) }
    }
    /// Create a bundle from the bitmaps in the application resources.
    fn from_resources(name: &str) -> BitmapBundle {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromResources(name))
        }
    }
    /// Create a bundle from bitmaps stored as files.
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
    fn from_files(fullpathname: &str) -> BitmapBundle {
        unsafe {
            let fullpathname = WxString::from(fullpathname);
            let fullpathname = fullpathname.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromFiles1(fullpathname))
        }
    }
    // BLOCKED: fn FromSVG()
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn from_svg<S: SizeMethods>(data: *const c_void, size_def: &S) -> BitmapBundle {
        unsafe {
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVG1(data, size_def))
        }
    }
    /// Create a bundle from the SVG image loaded from the given file.
    fn from_svg_file<S: SizeMethods>(path: &str, size_def: &S) -> BitmapBundle {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            let size_def = size_def.as_ptr();
            BitmapBundle::from_ptr(ffi::wxBitmapBundle_FromSVGFile(path, size_def))
        }
    }
    /// Create a bundle from the SVG image loaded from an application resource.
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
    /// Button creation function for two-step creation.
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
    /// Adds the item to the end of the combo box.
    fn append<B: BitmapMethods>(&self, item: &str, bitmap: &B) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Append(self.as_ptr(), item, bitmap)
        }
    }
    /// Adds the item to the end of the combo box, associating the given untyped, client data pointer clientData with the item.
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
    fn get_bitmap_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxBitmapComboBox_GetBitmapSize(self.as_ptr())) }
    }
    /// Returns the bitmap of the item with the given index.
    fn get_item_bitmap(&self, n: c_uint) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmapComboBox_GetItemBitmap(self.as_ptr(), n)) }
    }
    /// Inserts the item into the list before pos.
    fn insert<B: BitmapMethods>(&self, item: &str, bitmap: &B, pos: c_uint) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_Insert(self.as_ptr(), item, bitmap, pos)
        }
    }
    /// Inserts the item into the list before pos, associating the given untyped, client data pointer with the item.
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
    fn set_item_bitmap<B: BitmapBundleMethods>(&self, n: c_uint, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapComboBox_SetItemBitmap(self.as_ptr(), n, bitmap)
        }
    }
}

// wxBitmapDataObject
pub trait BitmapDataObjectMethods: DataObjectSimpleMethods {
    /// Returns the bitmap associated with the data object.
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxBitmapDataObject_GetBitmap(self.as_ptr())) }
    }
    /// Sets the bitmap associated with the data object.
    fn set_bitmap<B: BitmapMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxBitmapDataObject_SetBitmap(self.as_ptr(), bitmap)
        }
    }
}

// wxBitmapHandler
pub trait BitmapHandlerMethods: ObjectMethods {
    // DTOR: fn ~wxBitmapHandler()
    // NOT_SUPPORTED: fn Create()
    /// Gets the file extension associated with this handler.
    fn get_extension(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxBitmapHandler_GetExtension(self.as_ptr())).into() }
    }
    /// Gets the name of this handler.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxBitmapHandler_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn SaveFile()
    /// Sets the handler extension.
    fn set_extension(&self, extension: &str) {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            ffi::wxBitmapHandler_SetExtension(self.as_ptr(), extension)
        }
    }
    /// Sets the handler name.
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
    /// Create method for two-step construction.
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
    /// Returns the image index for the given page.
    fn get_page_image(&self, n_page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetPageImage(self.as_ptr(), n_page) }
    }
    /// Sets the image index for the given page.
    fn set_page_image(&self, page: usize, image: c_int) -> bool {
        unsafe { ffi::wxBookCtrlBase_SetPageImage(self.as_ptr(), page, image) }
    }
    /// Returns the string for the given page.
    fn get_page_text(&self, n_page: usize) -> String {
        unsafe { WxString::from_ptr(ffi::wxBookCtrlBase_GetPageText(self.as_ptr(), n_page)).into() }
    }
    /// Sets the text for the given page.
    fn set_page_text(&self, page: usize, text: &str) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxBookCtrlBase_SetPageText(self.as_ptr(), page, text)
        }
    }
    /// Returns the currently selected page, or wxNOT_FOUND if none was selected.
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlBase_GetSelection(self.as_ptr()) }
    }
    /// Returns the currently selected page or NULL.
    fn get_current_page(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetCurrentPage(self.as_ptr())) }
    }
    /// Sets the selection to the given page, returning the previous selection.
    fn set_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_SetSelection(self.as_ptr(), page) }
    }
    /// Cycles through the tabs.
    fn advance_selection(&self, forward: bool) {
        unsafe { ffi::wxBookCtrlBase_AdvanceSelection(self.as_ptr(), forward) }
    }
    /// Changes the selection to the given page, returning the previous selection.
    fn change_selection(&self, page: usize) -> c_int {
        unsafe { ffi::wxBookCtrlBase_ChangeSelection(self.as_ptr(), page) }
    }
    /// Returns the index of the specified tab window or wxNOT_FOUND if not found.
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
    fn set_page_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxBookCtrlBase_SetPageSize(self.as_ptr(), size)
        }
    }
    /// Returns the index of the tab at the specified position or wxNOT_FOUND if none.
    fn hit_test<P: PointMethods>(&self, pt: &P, flags: *mut c_void) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxBookCtrlBase_HitTest(self.as_ptr(), pt, flags)
        }
    }
    /// Adds a new page.
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
    fn delete_all_pages(&self) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeleteAllPages(self.as_ptr()) }
    }
    /// Deletes the specified page, and the associated window.
    fn delete_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_DeletePage(self.as_ptr(), page) }
    }
    /// Inserts a new page at the specified position.
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
    fn remove_page(&self, page: usize) -> bool {
        unsafe { ffi::wxBookCtrlBase_RemovePage(self.as_ptr(), page) }
    }
    /// Returns the number of pages in the control.
    fn get_page_count(&self) -> usize {
        unsafe { ffi::wxBookCtrlBase_GetPageCount(self.as_ptr()) }
    }
    /// Returns the window at the given page position.
    fn get_page(&self, page: usize) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxBookCtrlBase_GetPage(self.as_ptr(), page)) }
    }
}

// wxBookCtrlEvent
pub trait BookCtrlEventMethods: NotifyEventMethods {
    /// Returns the page that was selected before the change, wxNOT_FOUND if none was selected.
    fn get_old_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetOldSelection(self.as_ptr()) }
    }
    /// Sets the id of the page selected before the change.
    fn set_old_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetOldSelection(self.as_ptr(), page) }
    }
    /// Sets the selection member variable.
    fn set_selection(&self, page: c_int) {
        unsafe { ffi::wxBookCtrlEvent_SetSelection(self.as_ptr(), page) }
    }
}

// wxBoxSizer
pub trait BoxSizerMethods: SizerMethods {
    /// Returns the orientation of the box sizer, either wxVERTICAL or wxHORIZONTAL.
    fn get_orientation(&self) -> c_int {
        unsafe { ffi::wxBoxSizer_GetOrientation(self.as_ptr()) }
    }
    /// Sets the orientation of the box sizer, either wxVERTICAL or wxHORIZONTAL.
    fn set_orientation(&self, orient: c_int) {
        unsafe { ffi::wxBoxSizer_SetOrientation(self.as_ptr(), orient) }
    }
}

// wxBrush
pub trait BrushMethods: GDIObjectMethods {
    // DTOR: fn ~wxBrush()
    /// Returns a reference to the brush colour.
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxBrush_GetColour(self.as_ptr())) }
    }
    /// Gets a pointer to the stipple bitmap.
    fn get_stipple(&self) -> Option<BitmapIsOwned<false>> {
        unsafe { Bitmap::option_from(ffi::wxBrush_GetStipple(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetStyle()
    /// Returns true if the style of the brush is any of hatched fills.
    fn is_hatch(&self) -> bool {
        unsafe { ffi::wxBrush_IsHatch(self.as_ptr()) }
    }
    /// Returns true if the brush is initialised.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxBrush_IsOk(self.as_ptr()) }
    }
    /// Returns true if the brush is a valid non-transparent brush.
    fn is_non_transparent(&self) -> bool {
        unsafe { ffi::wxBrush_IsNonTransparent(self.as_ptr()) }
    }
    /// Returns true if the brush is transparent.
    fn is_transparent(&self) -> bool {
        unsafe { ffi::wxBrush_IsTransparent(self.as_ptr()) }
    }
    /// Sets the brush colour using red, green and blue values.
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxBrush_SetColour(self.as_ptr(), colour)
        }
    }
    // NOT_SUPPORTED: fn SetColour1()
    /// Sets the stipple bitmap.
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
    /// Initializes the object created using the default constructor.
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
    /// Blits the buffer to the dc, and detaches the dc from the buffer (so it can be effectively used once only).
    fn un_mask(&self) {
        unsafe { ffi::wxBufferedDC_UnMask(self.as_ptr()) }
    }
    /// Set the style.
    fn set_style(&self, style: c_int) {
        unsafe { ffi::wxBufferedDC_SetStyle(self.as_ptr(), style) }
    }
    /// Get the style.
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
    /// Update the information text.
    fn update_text(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxBusyInfo_UpdateText(self.as_ptr(), str)
        }
    }
    /// Same as UpdateText() but doesn't interpret the string as containing markup.
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
    /// Button creation function for two-step creation.
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
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr()) }
    }
    /// Sets whether an authentication needed symbol should be displayed on the button.
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr(), needed) }
    }
    /// This sets the button to be the default item in its top-level window (e.g.
    fn set_default(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxButton_SetDefault(self.as_ptr())) }
    }
    /// Returns the default size for the buttons.
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
