use super::*;

// wxIcon
pub trait IconMethods: GDIObjectMethods {
    // DTOR: fn ~wxIcon()
    // NOT_SUPPORTED: fn CreateFromHICON()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    /// Copies bmp bitmap to this icon.
    fn copy_from_bitmap<B: BitmapMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxIcon_CopyFromBitmap(self.as_ptr(), bmp)
        }
    }
    /// Gets the colour depth of the icon.
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxIcon_GetDepth(self.as_ptr()) }
    }
    /// Gets the height of the icon in physical pixels.
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxIcon_GetHeight(self.as_ptr()) }
    }
    /// Gets the height of the icon in logical pixels.
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxIcon_GetLogicalHeight(self.as_ptr()) }
    }
    /// Gets the size of the icon in logical pixels.
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxIcon_GetLogicalSize(self.as_ptr())) }
    }
    /// Gets the width of the icon in logical pixels.
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxIcon_GetLogicalWidth(self.as_ptr()) }
    }
    /// Gets the scale factor of this icon.
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxIcon_GetScaleFactor(self.as_ptr()) }
    }
    /// Gets the size of the icon in physical pixels.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxIcon_GetSize(self.as_ptr())) }
    }
    /// Gets the width of the icon in physical pixels.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxIcon_GetWidth(self.as_ptr()) }
    }
    /// Returns true if icon data is present.
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
    /// Adds the icon to the collection; if the collection already contains an icon with the same width and height, it is replaced by the new one.
    fn add_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxIconBundle_AddIcon3(self.as_ptr(), icon)
        }
    }
    /// Returns the icon with the given size.
    fn get_icon_size<S: SizeMethods>(&self, size: &S, flags: c_int) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxIconBundle_GetIcon(self.as_ptr(), size, flags))
        }
    }
    /// Same as.
    fn get_icon_coord(&self, size: c_int, flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxIconBundle_GetIcon1(self.as_ptr(), size, flags)) }
    }
    /// Returns the icon with exactly the given size or wxNullIcon if this size is not available.
    fn get_icon_of_exact_size<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxIconBundle_GetIconOfExactSize(self.as_ptr(), size))
        }
    }
    /// return the number of available icons
    fn get_icon_count(&self) -> usize {
        unsafe { ffi::wxIconBundle_GetIconCount(self.as_ptr()) }
    }
    /// return the icon at index (must be < GetIconCount())
    fn get_icon_by_index(&self, n: usize) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxIconBundle_GetIconByIndex(self.as_ptr(), n)) }
    }
    /// Returns true if the bundle doesn't contain any icons, false otherwise (in which case a call to GetIcon() with default parameter should return a valid icon).
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxIconBundle_IsEmpty(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxIconizeEvent
pub trait IconizeEventMethods: EventMethods {
    /// Returns true if the frame has been iconized, false if it has been restored.
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxIconizeEvent_IsIconized(self.as_ptr()) }
    }
    // BLOCKED: fn Iconized()
}

// wxIdManager
pub trait IdManagerMethods: WxRustMethods {
    /// Called directly by wxWindow::NewControlId(), this function will create a new ID or range of IDs.
    fn reserve_id(count: c_int) -> c_int {
        unsafe { ffi::wxIdManager_ReserveId(count) }
    }
    /// Called directly by wxWindow::UnreserveControlId(), this function will unreserve an ID or range of IDs that is currently reserved.
    fn unreserve_id(id: c_int, count: c_int) {
        unsafe { ffi::wxIdManager_UnreserveId(id, count) }
    }
}

// wxImage
pub trait ImageMethods: ObjectMethods {
    /// Returns an identical copy of this image.
    fn copy(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Copy(self.as_ptr())) }
    }
    /// Creates a fresh image.
    fn create_int_bool(&self, width: c_int, height: c_int, clear: bool) -> bool {
        unsafe { ffi::wxImage_Create(self.as_ptr(), width, height, clear) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn create_size_bool<S: SizeMethods>(&self, sz: &S, clear: bool) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxImage_Create1(self.as_ptr(), sz, clear)
        }
    }
    /// Creates a fresh image.
    fn create_int_uchar_bool(
        &self,
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> bool {
        unsafe { ffi::wxImage_Create2(self.as_ptr(), width, height, data, static_data) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn create_size_uchar_bool<S: SizeMethods>(
        &self,
        sz: &S,
        data: *mut c_void,
        static_data: bool,
    ) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxImage_Create3(self.as_ptr(), sz, data, static_data)
        }
    }
    /// Creates a fresh image.
    fn create_int_uchar_uchar(
        &self,
        width: c_int,
        height: c_int,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> bool {
        unsafe { ffi::wxImage_Create4(self.as_ptr(), width, height, data, alpha, static_data) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn create_size_uchar_uchar<S: SizeMethods>(
        &self,
        sz: &S,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxImage_Create5(self.as_ptr(), sz, data, alpha, static_data)
        }
    }
    // NOT_SUPPORTED: fn Clear()
    /// Destroys the image data.
    fn destroy(&self) {
        unsafe { ffi::wxImage_Destroy(self.as_ptr()) }
    }
    /// Initializes the image alpha channel data.
    fn init_alpha(&self) {
        unsafe { ffi::wxImage_InitAlpha(self.as_ptr()) }
    }
    /// Blurs the image in both horizontal and vertical directions by the specified pixel blurRadius.
    fn blur(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Blur(self.as_ptr(), blur_radius)) }
    }
    /// Blurs the image in the horizontal direction only.
    fn blur_horizontal(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_BlurHorizontal(self.as_ptr(), blur_radius)) }
    }
    /// Blurs the image in the vertical direction only.
    fn blur_vertical(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_BlurVertical(self.as_ptr(), blur_radius)) }
    }
    /// Returns a mirrored copy of the image.
    fn mirror(&self, horizontally: bool) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Mirror(self.as_ptr(), horizontally)) }
    }
    // NOT_SUPPORTED: fn Paste()
    // NOT_SUPPORTED: fn Replace()
    // NOT_SUPPORTED: fn Rescale()
    /// Changes the size of the image in-place without scaling it by adding either a border with the given colour or cropping as necessary.
    fn resize<S: SizeMethods, P: PointMethods>(
        &self,
        size: &S,
        pos: &P,
        red: c_int,
        green: c_int,
        blue: c_int,
    ) -> &Self {
        unsafe {
            let size = size.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxImage_Resize(self.as_ptr(), size, pos, red, green, blue);
            &self
        }
    }
    /// Rotates the image about the given point, by angle radians.
    fn rotate<P: PointMethods, P2: PointMethods>(
        &self,
        angle: c_double,
        rotation_centre: &P,
        interpolating: bool,
        offset_after_rotation: Option<&P2>,
    ) -> Image {
        unsafe {
            let rotation_centre = rotation_centre.as_ptr();
            let offset_after_rotation = match offset_after_rotation {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Image::from_ptr(ffi::wxImage_Rotate(
                self.as_ptr(),
                angle,
                rotation_centre,
                interpolating,
                offset_after_rotation,
            ))
        }
    }
    /// Returns a copy of the image rotated 90 degrees in the direction indicated by clockwise.
    fn rotate90(&self, clockwise: bool) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Rotate90(self.as_ptr(), clockwise)) }
    }
    /// Returns a copy of the image rotated by 180 degrees.
    fn rotate180(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Rotate180(self.as_ptr())) }
    }
    /// Rotates the hue of each pixel in the image by angle, which is a double in the range [-1.0..+1.0], where -1.0 corresponds to -360 degrees and +1.0 corresponds to +360 degrees.
    fn rotate_hue(&self, angle: c_double) {
        unsafe { ffi::wxImage_RotateHue(self.as_ptr(), angle) }
    }
    /// Changes the saturation of each pixel in the image.
    fn change_saturation(&self, factor: c_double) {
        unsafe { ffi::wxImage_ChangeSaturation(self.as_ptr(), factor) }
    }
    /// Changes the brightness (value) of each pixel in the image.
    fn change_brightness(&self, factor: c_double) {
        unsafe { ffi::wxImage_ChangeBrightness(self.as_ptr(), factor) }
    }
    /// Changes the hue, the saturation and the brightness (value) of each pixel in the image.
    fn change_hsv(&self, angle_h: c_double, factor_s: c_double, factor_v: c_double) {
        unsafe { ffi::wxImage_ChangeHSV(self.as_ptr(), angle_h, factor_s, factor_v) }
    }
    // NOT_SUPPORTED: fn Scale()
    /// Returns a resized version of this image without scaling it by adding either a border with the given colour or cropping as necessary.
    fn size<S: SizeMethods, P: PointMethods>(
        &self,
        size: &S,
        pos: &P,
        red: c_int,
        green: c_int,
        blue: c_int,
    ) -> Image {
        unsafe {
            let size = size.as_ptr();
            let pos = pos.as_ptr();
            Image::from_ptr(ffi::wxImage_Size(
                self.as_ptr(),
                size,
                pos,
                red,
                green,
                blue,
            ))
        }
    }
    // NOT_SUPPORTED: fn ConvertAlphaToMask()
    // NOT_SUPPORTED: fn ConvertAlphaToMask1()
    /// Returns a greyscale version of the image.
    fn convert_to_greyscale_double(
        &self,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    ) -> Image {
        unsafe {
            Image::from_ptr(ffi::wxImage_ConvertToGreyscale(
                self.as_ptr(),
                weight_r,
                weight_g,
                weight_b,
            ))
        }
    }
    /// Returns a greyscale version of the image.
    fn convert_to_greyscale(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_ConvertToGreyscale1(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn ConvertToMono()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    /// Returns a changed version of the image based on the given lightness.
    fn change_lightness(&self, alpha: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_ChangeLightness(self.as_ptr(), alpha)) }
    }
    // NOT_SUPPORTED: fn ComputeHistogram()
    // NOT_SUPPORTED: fn FindFirstUnusedColour()
    // BLOCKED: fn operator=()
    /// Returns pointer to the array storing the alpha values for this image.
    fn get_alpha(&self) -> *mut c_void {
        unsafe { ffi::wxImage_GetAlpha(self.as_ptr()) }
    }
    /// Returns the image data as an array.
    fn get_data(&self) -> *mut c_void {
        unsafe { ffi::wxImage_GetData(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAlpha1()
    // NOT_SUPPORTED: fn GetRed()
    // NOT_SUPPORTED: fn GetGreen()
    // NOT_SUPPORTED: fn GetBlue()
    // NOT_SUPPORTED: fn GetMaskRed()
    // NOT_SUPPORTED: fn GetMaskGreen()
    // NOT_SUPPORTED: fn GetMaskBlue()
    /// Gets the width of the image in pixels.
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxImage_GetWidth(self.as_ptr()) }
    }
    /// Gets the height of the image in pixels.
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxImage_GetHeight(self.as_ptr()) }
    }
    /// Returns the size of the image in pixels.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxImage_GetSize(self.as_ptr())) }
    }
    /// Gets a user-defined string-valued option.
    fn get_option(&self, name: &str) -> String {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WxString::from_ptr(ffi::wxImage_GetOption(self.as_ptr(), name)).into()
        }
    }
    /// Gets a user-defined integer-valued option.
    fn get_option_int(&self, name: &str) -> c_int {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_GetOptionInt(self.as_ptr(), name)
        }
    }
    /// Get the current mask colour or find a suitable unused colour that could be used as a mask colour.
    fn get_or_find_mask_colour(&self, r: *mut c_void, g: *mut c_void, b: *mut c_void) -> bool {
        unsafe { ffi::wxImage_GetOrFindMaskColour(self.as_ptr(), r, g, b) }
    }
    // BLOCKED: fn GetPalette()
    /// Returns a sub image of the current one as long as the rect belongs entirely to the image.
    fn get_sub_image<R: RectMethods>(&self, rect: &R) -> Image {
        unsafe {
            let rect = rect.as_ptr();
            Image::from_ptr(ffi::wxImage_GetSubImage(self.as_ptr(), rect))
        }
    }
    // NOT_SUPPORTED: fn GetType()
    /// Returns true if this image has alpha channel, false otherwise.
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxImage_HasAlpha(self.as_ptr()) }
    }
    /// Returns true if there is a mask active, false otherwise.
    fn has_mask(&self) -> bool {
        unsafe { ffi::wxImage_HasMask(self.as_ptr()) }
    }
    /// Returns true if the given option is present.
    fn has_option(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_HasOption(self.as_ptr(), name)
        }
    }
    /// Returns true if image data is present.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxImage_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsTransparent()
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn LoadFile1()
    /// Loads an image from a file.
    fn load_file_str(&self, name: &str, mimetype: &str, index: c_int) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_LoadFile2(self.as_ptr(), name, mimetype, index)
        }
    }
    /// Loads an image from an input stream.
    fn load_file_inputstream(&self, stream: *mut c_void, mimetype: &str, index: c_int) -> bool {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_LoadFile3(self.as_ptr(), stream, mimetype, index)
        }
    }
    /// Saves an image in the given stream.
    fn save_file_outputstream(&self, stream: *mut c_void, mimetype: &str) -> bool {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_SaveFile(self.as_ptr(), stream, mimetype)
        }
    }
    // NOT_SUPPORTED: fn SaveFile1()
    /// Saves an image in the named file.
    fn save_file_str_str(&self, name: &str, mimetype: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_SaveFile2(self.as_ptr(), name, mimetype)
        }
    }
    /// Saves an image in the named file.
    fn save_file_str(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_SaveFile3(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SaveFile4()
    /// This function is similar to SetData() and has similar restrictions.
    fn set_alpha(&self, alpha: *mut c_void, static_data: bool) {
        unsafe { ffi::wxImage_SetAlpha(self.as_ptr(), alpha, static_data) }
    }
    // NOT_SUPPORTED: fn SetAlpha1()
    /// Removes the alpha channel from the image.
    fn clear_alpha(&self) {
        unsafe { ffi::wxImage_ClearAlpha(self.as_ptr()) }
    }
    /// Sets the image data without performing checks.
    fn set_data_bool(&self, data: *mut c_void, static_data: bool) {
        unsafe { ffi::wxImage_SetData(self.as_ptr(), data, static_data) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_data_int(
        &self,
        data: *mut c_void,
        new_width: c_int,
        new_height: c_int,
        static_data: bool,
    ) {
        unsafe { ffi::wxImage_SetData1(self.as_ptr(), data, new_width, new_height, static_data) }
    }
    /// Sets the flags used for loading image files by this object.
    fn set_load_flags(&self, flags: c_int) {
        unsafe { ffi::wxImage_SetLoadFlags(self.as_ptr(), flags) }
    }
    /// Specifies whether there is a mask or not.
    fn set_mask(&self, has_mask: bool) {
        unsafe { ffi::wxImage_SetMask(self.as_ptr(), has_mask) }
    }
    // NOT_SUPPORTED: fn SetMaskColour()
    // NOT_SUPPORTED: fn SetMaskFromImage()
    /// Sets a user-defined option.
    fn set_option_str(&self, name: &str, value: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxImage_SetOption(self.as_ptr(), name, value)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_option_int(&self, name: &str, value: c_int) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_SetOption1(self.as_ptr(), name, value)
        }
    }
    /// Associates a palette with the image.
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxImage_SetPalette(self.as_ptr(), palette)
        }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGB1()
    // NOT_SUPPORTED: fn SetType()
    /// Sets the default value for the flags used for loading image files.
    fn set_default_load_flags(flags: c_int) {
        unsafe { ffi::wxImage_SetDefaultLoadFlags(flags) }
    }
    /// Returns the file load flags used for this object.
    fn get_load_flags(&self) -> c_int {
        unsafe { ffi::wxImage_GetLoadFlags(self.as_ptr()) }
    }
    /// Register an image handler.
    fn add_handler<I: ImageHandlerMethods>(handler: Option<&I>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxImage_AddHandler(handler)
        }
    }
    /// Deletes all image handlers.
    fn clean_up_handlers() {
        unsafe { ffi::wxImage_CleanUpHandlers() }
    }
    /// Finds the handler with the given name.
    fn find_handler(name: &str) -> Option<ImageHandlerIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ImageHandler::option_from(ffi::wxImage_FindHandler(name))
        }
    }
    // NOT_SUPPORTED: fn FindHandler1()
    // NOT_SUPPORTED: fn FindHandler2()
    /// Finds the handler associated with the given MIME type.
    fn find_handler_mime(mimetype: &str) -> Option<ImageHandlerIsOwned<false>> {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageHandler::option_from(ffi::wxImage_FindHandlerMime(mimetype))
        }
    }
    // BLOCKED: fn GetHandlers()
    /// Internal use only.
    fn init_standard_handlers() {
        unsafe { ffi::wxImage_InitStandardHandlers() }
    }
    /// Adds a handler at the start of the static list of format handlers.
    fn insert_handler<I: ImageHandlerMethods>(handler: Option<&I>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxImage_InsertHandler(handler)
        }
    }
    /// Finds the handler with the given name, and removes it.
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_RemoveHandler(name)
        }
    }
    /// Returns true if at least one of the available image handlers can read the file with the given name.
    fn can_read_str(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxImage_CanRead(filename)
        }
    }
    /// Returns true if at least one of the available image handlers can read the data in the given stream.
    fn can_read_inputstream(stream: *mut c_void) -> bool {
        unsafe { ffi::wxImage_CanRead1(stream) }
    }
    /// Returns the currently used default file load flags.
    fn get_default_load_flags() -> c_int {
        unsafe { ffi::wxImage_GetDefaultLoadFlags() }
    }
    // NOT_SUPPORTED: fn GetImageCount()
    // NOT_SUPPORTED: fn GetImageCount1()
    /// Iterates all registered wxImageHandler objects, and returns a string containing file extension masks suitable for passing to file open/save dialog boxes.
    fn get_image_ext_wildcard() -> String {
        unsafe { WxString::from_ptr(ffi::wxImage_GetImageExtWildcard()).into() }
    }
    // NOT_SUPPORTED: fn RGBtoHSV()
    // NOT_SUPPORTED: fn HSVtoRGB()
    // DTOR: fn ~wxImage()
}

// wxImageHandler
pub trait ImageHandlerMethods: ObjectMethods {
    // DTOR: fn ~wxImageHandler()
    /// Returns true if this handler supports the image format contained in the given stream.
    fn can_read_inputstream(&self, stream: *mut c_void) -> bool {
        unsafe { ffi::wxImageHandler_CanRead(self.as_ptr(), stream) }
    }
    /// Returns true if this handler supports the image format contained in the file with the given name.
    fn can_read_str(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxImageHandler_CanRead1(self.as_ptr(), filename)
        }
    }
    /// Gets the preferred file extension associated with this handler.
    fn get_extension(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetExtension(self.as_ptr())).into() }
    }
    /// Returns the other file extensions associated with this handler.
    fn get_alt_extensions(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxImageHandler_GetAltExtensions(self.as_ptr())) }
    }
    /// If the image file contains more than one image and the image handler is capable of retrieving these individually, this function will return the number of available images.
    fn get_image_count(&self, stream: *mut c_void) -> c_int {
        unsafe { ffi::wxImageHandler_GetImageCount(self.as_ptr(), stream) }
    }
    /// Gets the MIME type associated with this handler.
    fn get_mime_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetMimeType(self.as_ptr())).into() }
    }
    /// Gets the name of this handler.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    /// Loads an image from a stream, putting the resulting data into image.
    fn load_file<I: ImageMethods>(
        &self,
        image: Option<&I>,
        stream: *mut c_void,
        verbose: bool,
        index: c_int,
    ) -> bool {
        unsafe {
            let image = match image {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxImageHandler_LoadFile(self.as_ptr(), image, stream, verbose, index)
        }
    }
    /// Saves an image in the output stream.
    fn save_file<I: ImageMethods>(
        &self,
        image: Option<&I>,
        stream: *mut c_void,
        verbose: bool,
    ) -> bool {
        unsafe {
            let image = match image {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxImageHandler_SaveFile(self.as_ptr(), image, stream, verbose)
        }
    }
    /// Sets the preferred file extension associated with this handler.
    fn set_extension(&self, extension: &str) {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            ffi::wxImageHandler_SetExtension(self.as_ptr(), extension)
        }
    }
    /// Sets the alternative file extensions associated with this handler.
    fn set_alt_extensions<A: ArrayStringMethods>(&self, extensions: &A) {
        unsafe {
            let extensions = extensions.as_ptr();
            ffi::wxImageHandler_SetAltExtensions(self.as_ptr(), extensions)
        }
    }
    /// Sets the handler MIME type.
    fn set_mime_type(&self, mimetype: &str) {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImageHandler_SetMimeType(self.as_ptr(), mimetype)
        }
    }
    /// Sets the handler name.
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImageHandler_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetType()
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxImageList
pub trait ImageListMethods: ObjectMethods {
    /// Adds a new image or images using a bitmap and optional mask bitmap.
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
    /// Adds a new image or images using a bitmap and mask colour.
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
    /// Adds a new image using an icon.
    fn add_icon<I: IconMethods>(&self, icon: &I) -> c_int {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxImageList_Add2(self.as_ptr(), icon)
        }
    }
    /// Initializes the list.
    fn create(&self, width: c_int, height: c_int, mask: bool, initial_count: c_int) -> bool {
        unsafe { ffi::wxImageList_Create(self.as_ptr(), width, height, mask, initial_count) }
    }
    /// Destroys the current list.
    fn destroy(&self) {
        unsafe { ffi::wxImageList_Destroy(self.as_ptr()) }
    }
    /// Draws a specified image onto a device context.
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
    /// Returns the bitmap corresponding to the given index.
    fn get_bitmap(&self, index: c_int) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxImageList_GetBitmap(self.as_ptr(), index)) }
    }
    /// Returns the icon corresponding to the given index.
    fn get_icon(&self, index: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxImageList_GetIcon(self.as_ptr(), index)) }
    }
    /// Returns the number of images in the list.
    fn get_image_count(&self) -> c_int {
        unsafe { ffi::wxImageList_GetImageCount(self.as_ptr()) }
    }
    /// Retrieves the size of the images in the list.
    fn get_size_int(&self, index: c_int, width: *mut c_void, height: *mut c_void) -> bool {
        unsafe { ffi::wxImageList_GetSize(self.as_ptr(), index, width, height) }
    }
    /// Retrieves the size of the image list as passed to Create().
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxImageList_GetSize1(self.as_ptr())) }
    }
    /// Removes the image at the given position.
    fn remove(&self, index: c_int) -> bool {
        unsafe { ffi::wxImageList_Remove(self.as_ptr(), index) }
    }
    /// Removes all the images in the list.
    fn remove_all(&self) -> bool {
        unsafe { ffi::wxImageList_RemoveAll(self.as_ptr()) }
    }
    /// Replaces the existing image with the new image.
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
    /// Replaces the existing image with the new image.
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
    /// Set the duration of the animation used when showing or hiding the bar.
    fn set_effect_duration(&self, duration: c_int) {
        unsafe { ffi::wxInfoBar_SetEffectDuration(self.as_ptr(), duration) }
    }
    /// Return the effect animation duration currently used.
    fn get_effect_duration(&self) -> c_int {
        unsafe { ffi::wxInfoBar_GetEffectDuration(self.as_ptr()) }
    }
    /// Create the info bar window.
    fn create<W: WindowMethods>(&self, parent: Option<&W>, winid: c_int) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxInfoBar_Create(self.as_ptr(), parent, winid)
        }
    }
    /// Add a button to be shown in the info bar.
    fn add_button(&self, btnid: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxInfoBar_AddButton(self.as_ptr(), btnid, label)
        }
    }
    /// Hide the info bar window.
    fn dismiss(&self) {
        unsafe { ffi::wxInfoBar_Dismiss(self.as_ptr()) }
    }
    /// Remove a button previously added by AddButton().
    fn remove_button(&self, btnid: c_int) {
        unsafe { ffi::wxInfoBar_RemoveButton(self.as_ptr(), btnid) }
    }
    /// Show a message in the bar.
    fn show_message(&self, msg: &str, flags: c_int) {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            ffi::wxInfoBar_ShowMessage(self.as_ptr(), msg, flags)
        }
    }
    /// Returns the number of currently shown buttons.
    fn get_button_count(&self) -> usize {
        unsafe { ffi::wxInfoBar_GetButtonCount(self.as_ptr()) }
    }
    /// Returns the ID of the button at the given position.
    fn get_button_id(&self, idx: usize) -> c_int {
        unsafe { ffi::wxInfoBar_GetButtonId(self.as_ptr(), idx) }
    }
    /// Returns whether a button with the given ID is currently shown.
    fn has_button_id(&self, btnid: c_int) -> bool {
        unsafe { ffi::wxInfoBar_HasButtonId(self.as_ptr(), btnid) }
    }
}

// wxInitDialogEvent
pub trait InitDialogEventMethods: EventMethods {}

// wxItemContainer
pub trait ItemContainerMethods: ItemContainerImmutableMethods {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    /// Appends item into the control.
    fn append_str(&self, item: &str) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Append(self.as_item_container(), item)
        }
    }
    /// Appends item into the control.
    fn append_str_void(&self, item: &str, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Append1(self.as_item_container(), item, client_data)
        }
    }
    /// Appends item into the control.
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
    /// Appends several items at once into the control.
    fn append_arraystring<A: ArrayStringMethods>(&self, items: &A) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Append3(self.as_item_container(), items)
        }
    }
    // BLOCKED: fn Append4()
    /// Appends several items at once into the control.
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
    /// Appends several items at once into the control.
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
    /// Appends several items at once into the control.
    fn append_uint(&self, n: c_uint, items: *const c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append7(self.as_item_container(), n, items) }
    }
    /// Appends several items at once into the control.
    fn append_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append8(self.as_item_container(), n, items, client_data) }
    }
    /// Appends several items at once into the control.
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
    /// Removes all items from the control.
    fn clear(&self) {
        unsafe { ffi::wxItemContainer_Clear(self.as_item_container()) }
    }
    /// Deletes an item from the control.
    fn delete(&self, n: c_uint) {
        unsafe { ffi::wxItemContainer_Delete(self.as_item_container(), n) }
    }
    /// Returns the client object associated with the given item and transfers its ownership to the caller.
    fn detach_client_object(&self, n: c_uint) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxItemContainer_DetachClientObject(
                self.as_item_container(),
                n,
            ))
        }
    }
    /// Returns true, if either untyped data (void*) or object data (wxClientData*) is associated with the items of the control.
    fn has_client_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientData(self.as_item_container()) }
    }
    /// Returns true, if object data is associated with the items of the control.
    fn has_client_object_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientObjectData(self.as_item_container()) }
    }
    /// Returns true, if untyped data (void*) is associated with the items of the control.
    fn has_client_untyped_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientUntypedData(self.as_item_container()) }
    }
    /// Returns a pointer to the client data associated with the given item (if any).
    fn get_client_data(&self, n: c_uint) -> *mut c_void {
        unsafe { ffi::wxItemContainer_GetClientData(self.as_item_container(), n) }
    }
    /// Returns a pointer to the client data associated with the given item (if any).
    fn get_client_object_uint(&self, n: c_uint) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxItemContainer_GetClientObject(
                self.as_item_container(),
                n,
            ))
        }
    }
    /// Associates the given untyped client data pointer with the given item.
    fn set_client_data(&self, n: c_uint, data: *mut c_void) {
        unsafe { ffi::wxItemContainer_SetClientData(self.as_item_container(), n, data) }
    }
    /// Associates the given typed client data pointer with the given item: the data object will be deleted when the item is deleted (either explicitly by using Delete() or implicitly when the control itself is destroyed).
    fn set_client_object_uint<C: ClientDataMethods>(&self, n: c_uint, data: Option<&C>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxItemContainer_SetClientObject(self.as_item_container(), n, data)
        }
    }
    /// Inserts item into the control.
    fn insert_str_uint(&self, item: &str, pos: c_uint) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Insert(self.as_item_container(), item, pos)
        }
    }
    /// Inserts item into the control.
    fn insert_str_uint_void(&self, item: &str, pos: c_uint, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Insert1(self.as_item_container(), item, pos, client_data)
        }
    }
    /// Inserts item into the control.
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
    /// Inserts several items at once into the control.
    fn insert_arraystring<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Insert3(self.as_item_container(), items, pos)
        }
    }
    // BLOCKED: fn Insert4()
    /// Inserts several items at once into the control.
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
    /// Inserts several items at once into the control.
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
    /// Inserts several items at once into the control.
    fn insert_uint(&self, n: c_uint, items: *const c_void, pos: c_uint) -> c_int {
        unsafe { ffi::wxItemContainer_Insert7(self.as_item_container(), n, items, pos) }
    }
    /// Inserts several items at once into the control.
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
    /// Inserts several items at once into the control.
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
    /// Replaces the current control contents with the given items.
    fn set_arraystring<A: ArrayStringMethods>(&self, items: &A) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set(self.as_item_container(), items)
        }
    }
    // BLOCKED: fn Set1()
    /// Replaces the current control contents with the given items.
    fn set_arraystring_void<A: ArrayStringMethods>(&self, items: &A, client_data: *mut c_void) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set2(self.as_item_container(), items, client_data)
        }
    }
    /// Replaces the current control contents with the given items.
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
    /// Replaces the current control contents with the given items.
    fn set_uint(&self, n: c_uint, items: *const c_void) {
        unsafe { ffi::wxItemContainer_Set4(self.as_item_container(), n, items) }
    }
    /// Replaces the current control contents with the given items.
    fn set_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) {
        unsafe { ffi::wxItemContainer_Set5(self.as_item_container(), n, items, client_data) }
    }
    /// Replaces the current control contents with the given items.
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
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    /// Sets the selection to the given item n or removes the selection entirely if n == wxNOT_FOUND.
    fn set_selection(&self, n: c_int) {
        unsafe { ffi::wxItemContainerImmutable_SetSelection(self.as_item_container_immutable(), n) }
    }
    /// Returns the index of the selected item or wxNOT_FOUND if no item is selected.
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxItemContainerImmutable_GetSelection(self.as_item_container_immutable()) }
    }
    /// Selects the item with the specified string in the control.
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
    /// Returns the label of the selected item or an empty string if no item is selected.
    fn get_string_selection(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxItemContainerImmutable_GetStringSelection(
                self.as_item_container_immutable(),
            ))
            .into()
        }
    }
    /// This is the same as SetSelection() and exists only because it is slightly more natural for controls which support multiple selection.
    fn select(&self, n: c_int) {
        unsafe { ffi::wxItemContainerImmutable_Select(self.as_item_container_immutable(), n) }
    }
    /// Returns the number of items in the control.
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxItemContainerImmutable_GetCount(self.as_item_container_immutable()) }
    }
    /// Returns true if the control is empty or false if it has some items.
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxItemContainerImmutable_IsEmpty(self.as_item_container_immutable()) }
    }
    /// Returns the label of the item with the given index.
    fn get_string(&self, n: c_uint) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxItemContainerImmutable_GetString(
                self.as_item_container_immutable(),
                n,
            ))
            .into()
        }
    }
    /// Returns the array of the labels of all items in the control.
    fn get_strings(&self) -> ArrayString {
        unsafe {
            ArrayString::from_ptr(ffi::wxItemContainerImmutable_GetStrings(
                self.as_item_container_immutable(),
            ))
        }
    }
    /// Sets the label for the given item.
    fn set_string(&self, n: c_uint, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxItemContainerImmutable_SetString(self.as_item_container_immutable(), n, string)
        }
    }
    /// Finds an item whose label matches the given string.
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
