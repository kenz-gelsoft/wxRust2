use super::*;

// wxIFFHandler
pub trait IFFHandlerMethods: ImageHandlerMethods {}

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

// wxIconizeEvent
pub trait IconizeEventMethods: EventMethods {
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxIconizeEvent_IsIconized(self.as_ptr()) }
    }
    // BLOCKED: fn Iconized()
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

// wxImage
pub trait ImageMethods: ObjectMethods {
    fn copy(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Copy(self.as_ptr())) }
    }
    fn create_int_bool(&self, width: c_int, height: c_int, clear: bool) -> bool {
        unsafe { ffi::wxImage_Create(self.as_ptr(), width, height, clear) }
    }
    fn create_size_bool<S: SizeMethods>(&self, sz: &S, clear: bool) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxImage_Create1(self.as_ptr(), sz, clear)
        }
    }
    fn create_int_uchar_bool(
        &self,
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> bool {
        unsafe { ffi::wxImage_Create2(self.as_ptr(), width, height, data, static_data) }
    }
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
    fn destroy(&self) {
        unsafe { ffi::wxImage_Destroy(self.as_ptr()) }
    }
    fn init_alpha(&self) {
        unsafe { ffi::wxImage_InitAlpha(self.as_ptr()) }
    }
    fn blur(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Blur(self.as_ptr(), blur_radius)) }
    }
    fn blur_horizontal(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_BlurHorizontal(self.as_ptr(), blur_radius)) }
    }
    fn blur_vertical(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_BlurVertical(self.as_ptr(), blur_radius)) }
    }
    fn mirror(&self, horizontally: bool) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Mirror(self.as_ptr(), horizontally)) }
    }
    // NOT_SUPPORTED: fn Paste()
    // NOT_SUPPORTED: fn Replace()
    // NOT_SUPPORTED: fn Rescale()
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
    fn rotate90(&self, clockwise: bool) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Rotate90(self.as_ptr(), clockwise)) }
    }
    fn rotate180(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Rotate180(self.as_ptr())) }
    }
    fn rotate_hue(&self, angle: c_double) {
        unsafe { ffi::wxImage_RotateHue(self.as_ptr(), angle) }
    }
    fn change_saturation(&self, factor: c_double) {
        unsafe { ffi::wxImage_ChangeSaturation(self.as_ptr(), factor) }
    }
    fn change_brightness(&self, factor: c_double) {
        unsafe { ffi::wxImage_ChangeBrightness(self.as_ptr(), factor) }
    }
    fn change_hsv(&self, angle_h: c_double, factor_s: c_double, factor_v: c_double) {
        unsafe { ffi::wxImage_ChangeHSV(self.as_ptr(), angle_h, factor_s, factor_v) }
    }
    // NOT_SUPPORTED: fn Scale()
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
    fn convert_to_greyscale(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_ConvertToGreyscale1(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn ConvertToMono()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    fn change_lightness(&self, alpha: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_ChangeLightness(self.as_ptr(), alpha)) }
    }
    // NOT_SUPPORTED: fn ComputeHistogram()
    // NOT_SUPPORTED: fn FindFirstUnusedColour()
    // BLOCKED: fn operator=()
    fn get_alpha(&self) -> *mut c_void {
        unsafe { ffi::wxImage_GetAlpha(self.as_ptr()) }
    }
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
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxImage_GetWidth(self.as_ptr()) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxImage_GetHeight(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxImage_GetSize(self.as_ptr())) }
    }
    fn get_option(&self, name: &str) -> String {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WxString::from_ptr(ffi::wxImage_GetOption(self.as_ptr(), name)).into()
        }
    }
    fn get_option_int(&self, name: &str) -> c_int {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_GetOptionInt(self.as_ptr(), name)
        }
    }
    fn get_or_find_mask_colour(&self, r: *mut c_void, g: *mut c_void, b: *mut c_void) -> bool {
        unsafe { ffi::wxImage_GetOrFindMaskColour(self.as_ptr(), r, g, b) }
    }
    // BLOCKED: fn GetPalette()
    fn get_sub_image<R: RectMethods>(&self, rect: &R) -> Image {
        unsafe {
            let rect = rect.as_ptr();
            Image::from_ptr(ffi::wxImage_GetSubImage(self.as_ptr(), rect))
        }
    }
    // NOT_SUPPORTED: fn GetType()
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxImage_HasAlpha(self.as_ptr()) }
    }
    fn has_mask(&self) -> bool {
        unsafe { ffi::wxImage_HasMask(self.as_ptr()) }
    }
    fn has_option(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_HasOption(self.as_ptr(), name)
        }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxImage_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsTransparent()
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn LoadFile1()
    fn load_file_str(&self, name: &str, mimetype: &str, index: c_int) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_LoadFile2(self.as_ptr(), name, mimetype, index)
        }
    }
    fn load_file_inputstream(&self, stream: *mut c_void, mimetype: &str, index: c_int) -> bool {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_LoadFile3(self.as_ptr(), stream, mimetype, index)
        }
    }
    fn save_file_outputstream(&self, stream: *mut c_void, mimetype: &str) -> bool {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_SaveFile(self.as_ptr(), stream, mimetype)
        }
    }
    // NOT_SUPPORTED: fn SaveFile1()
    fn save_file_str_str(&self, name: &str, mimetype: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_SaveFile2(self.as_ptr(), name, mimetype)
        }
    }
    fn save_file_str(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_SaveFile3(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SaveFile4()
    fn set_alpha(&self, alpha: *mut c_void, static_data: bool) {
        unsafe { ffi::wxImage_SetAlpha(self.as_ptr(), alpha, static_data) }
    }
    // NOT_SUPPORTED: fn SetAlpha1()
    fn clear_alpha(&self) {
        unsafe { ffi::wxImage_ClearAlpha(self.as_ptr()) }
    }
    fn set_data_bool(&self, data: *mut c_void, static_data: bool) {
        unsafe { ffi::wxImage_SetData(self.as_ptr(), data, static_data) }
    }
    fn set_data_int(
        &self,
        data: *mut c_void,
        new_width: c_int,
        new_height: c_int,
        static_data: bool,
    ) {
        unsafe { ffi::wxImage_SetData1(self.as_ptr(), data, new_width, new_height, static_data) }
    }
    fn set_load_flags(&self, flags: c_int) {
        unsafe { ffi::wxImage_SetLoadFlags(self.as_ptr(), flags) }
    }
    fn set_mask(&self, has_mask: bool) {
        unsafe { ffi::wxImage_SetMask(self.as_ptr(), has_mask) }
    }
    // NOT_SUPPORTED: fn SetMaskColour()
    // NOT_SUPPORTED: fn SetMaskFromImage()
    fn set_option_str(&self, name: &str, value: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxImage_SetOption(self.as_ptr(), name, value)
        }
    }
    fn set_option_int(&self, name: &str, value: c_int) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_SetOption1(self.as_ptr(), name, value)
        }
    }
    fn set_palette(&self, palette: *const c_void) {
        unsafe { ffi::wxImage_SetPalette(self.as_ptr(), palette) }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGB1()
    // NOT_SUPPORTED: fn SetType()
    fn set_default_load_flags(flags: c_int) {
        unsafe { ffi::wxImage_SetDefaultLoadFlags(flags) }
    }
    fn get_load_flags(&self) -> c_int {
        unsafe { ffi::wxImage_GetLoadFlags(self.as_ptr()) }
    }
    fn add_handler<I: ImageHandlerMethods>(handler: Option<&I>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxImage_AddHandler(handler)
        }
    }
    fn clean_up_handlers() {
        unsafe { ffi::wxImage_CleanUpHandlers() }
    }
    fn find_handler(name: &str) -> Option<ImageHandlerIsOwned<false>> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ImageHandler::option_from(ffi::wxImage_FindHandler(name))
        }
    }
    // NOT_SUPPORTED: fn FindHandler1()
    // NOT_SUPPORTED: fn FindHandler2()
    fn find_handler_mime(mimetype: &str) -> Option<ImageHandlerIsOwned<false>> {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageHandler::option_from(ffi::wxImage_FindHandlerMime(mimetype))
        }
    }
    // BLOCKED: fn GetHandlers()
    fn init_standard_handlers() {
        unsafe { ffi::wxImage_InitStandardHandlers() }
    }
    fn insert_handler<I: ImageHandlerMethods>(handler: Option<&I>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxImage_InsertHandler(handler)
        }
    }
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_RemoveHandler(name)
        }
    }
    fn can_read_str(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxImage_CanRead(filename)
        }
    }
    fn can_read_inputstream(stream: *mut c_void) -> bool {
        unsafe { ffi::wxImage_CanRead1(stream) }
    }
    fn get_default_load_flags() -> c_int {
        unsafe { ffi::wxImage_GetDefaultLoadFlags() }
    }
    // NOT_SUPPORTED: fn GetImageCount()
    // NOT_SUPPORTED: fn GetImageCount1()
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
    fn can_read_inputstream(&self, stream: *mut c_void) -> bool {
        unsafe { ffi::wxImageHandler_CanRead(self.as_ptr(), stream) }
    }
    fn can_read_str(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxImageHandler_CanRead1(self.as_ptr(), filename)
        }
    }
    fn get_extension(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetExtension(self.as_ptr())).into() }
    }
    fn get_alt_extensions(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxImageHandler_GetAltExtensions(self.as_ptr())) }
    }
    fn get_image_count(&self, stream: *mut c_void) -> c_int {
        unsafe { ffi::wxImageHandler_GetImageCount(self.as_ptr(), stream) }
    }
    fn get_mime_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetMimeType(self.as_ptr())).into() }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
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
    fn set_extension(&self, extension: &str) {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            ffi::wxImageHandler_SetExtension(self.as_ptr(), extension)
        }
    }
    fn set_alt_extensions<A: ArrayStringMethods>(&self, extensions: &A) {
        unsafe {
            let extensions = extensions.as_ptr();
            ffi::wxImageHandler_SetAltExtensions(self.as_ptr(), extensions)
        }
    }
    fn set_mime_type(&self, mimetype: &str) {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImageHandler_SetMimeType(self.as_ptr(), mimetype)
        }
    }
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

// wxItemContainer
pub trait ItemContainerMethods: ItemContainerImmutableMethods {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
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
    fn append_str_clientdata(&self, item: &str, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
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
    fn append_arraystring_clientdata<A: ArrayStringMethods>(
        &self,
        items: &A,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Append6(self.as_item_container(), items, client_data)
        }
    }
    fn append_uint(&self, n: c_uint, items: *const c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append7(self.as_item_container(), n, items) }
    }
    fn append_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append8(self.as_item_container(), n, items, client_data) }
    }
    fn append_uint_clientdata(
        &self,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe { ffi::wxItemContainer_Append9(self.as_item_container(), n, items, client_data) }
    }
    fn clear(&self) {
        unsafe { ffi::wxItemContainer_Clear(self.as_item_container()) }
    }
    fn delete(&self, n: c_uint) {
        unsafe { ffi::wxItemContainer_Delete(self.as_item_container(), n) }
    }
    fn detach_client_object(&self, n: c_uint) -> *mut c_void {
        unsafe { ffi::wxItemContainer_DetachClientObject(self.as_item_container(), n) }
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
    fn get_client_object_uint(&self, n: c_uint) -> *mut c_void {
        unsafe { ffi::wxItemContainer_GetClientObject(self.as_item_container(), n) }
    }
    fn set_client_data(&self, n: c_uint, data: *mut c_void) {
        unsafe { ffi::wxItemContainer_SetClientData(self.as_item_container(), n, data) }
    }
    fn set_client_object_uint(&self, n: c_uint, data: *mut c_void) {
        unsafe { ffi::wxItemContainer_SetClientObject(self.as_item_container(), n, data) }
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
    fn insert_str_uint_clientdata(
        &self,
        item: &str,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
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
    fn insert_arraystring_clientdata<A: ArrayStringMethods>(
        &self,
        items: &A,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
            let items = items.as_ptr();
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
    fn insert_uint_clientdata(
        &self,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int {
        unsafe {
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
    fn set_arraystring_clientdata<A: ArrayStringMethods>(
        &self,
        items: &A,
        client_data: *mut c_void,
    ) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set3(self.as_item_container(), items, client_data)
        }
    }
    fn set_uint(&self, n: c_uint, items: *const c_void) {
        unsafe { ffi::wxItemContainer_Set4(self.as_item_container(), n, items) }
    }
    fn set_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) {
        unsafe { ffi::wxItemContainer_Set5(self.as_item_container(), n, items, client_data) }
    }
    fn set_uint_clientdata(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) {
        unsafe { ffi::wxItemContainer_Set6(self.as_item_container(), n, items, client_data) }
    }
}

// wxItemContainerImmutable
pub trait ItemContainerImmutableMethods: WxRustMethods {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
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
