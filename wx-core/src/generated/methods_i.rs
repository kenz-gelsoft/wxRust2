use super::*;

// wxIcon
/// This trait represents [C++ `wxIcon` class](https://docs.wxwidgets.org/3.2/classwx_icon.html)'s methods and inheritance.
///
/// See [`IconIsOwned`] documentation for the class usage.
pub trait IconMethods: GDIObjectMethods {
    // DTOR: fn ~wxIcon()
    // NOT_SUPPORTED: fn CreateFromHICON()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    /// Copies bmp bitmap to this icon.
    ///
    /// See [C++ `wxIcon::CopyFromBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#aea0254e1fcd09977999799b5744a016c).
    fn copy_from_bitmap<B: BitmapMethods>(&self, bmp: &B) {
        unsafe {
            let bmp = bmp.as_ptr();
            ffi::wxIcon_CopyFromBitmap(self.as_ptr(), bmp)
        }
    }
    /// Gets the colour depth of the icon.
    ///
    /// See [C++ `wxIcon::GetDepth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a5074750955267dbf807a9fa5ca6cbf67).
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxIcon_GetDepth(self.as_ptr()) }
    }
    /// Gets the height of the icon in physical pixels.
    ///
    /// See [C++ `wxIcon::GetHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a56832e71e9efb882d4cad7231d5f1ac7).
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxIcon_GetHeight(self.as_ptr()) }
    }
    /// Gets the height of the icon in logical pixels.
    ///
    /// See [C++ `wxIcon::GetLogicalHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a44861d2d93bbd6d3014d76dae210b1f1).
    fn get_logical_height(&self) -> c_double {
        unsafe { ffi::wxIcon_GetLogicalHeight(self.as_ptr()) }
    }
    /// Gets the size of the icon in logical pixels.
    ///
    /// See [C++ `wxIcon::GetLogicalSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a115b2786820abc73cf0f577a658777c9).
    fn get_logical_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxIcon_GetLogicalSize(self.as_ptr())) }
    }
    /// Gets the width of the icon in logical pixels.
    ///
    /// See [C++ `wxIcon::GetLogicalWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#adacc07b842e992220eb7afdb7f5b790e).
    fn get_logical_width(&self) -> c_double {
        unsafe { ffi::wxIcon_GetLogicalWidth(self.as_ptr()) }
    }
    /// Gets the scale factor of this icon.
    ///
    /// See [C++ `wxIcon::GetScaleFactor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a592d0df112570fc3f696883c8e210d7b).
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxIcon_GetScaleFactor(self.as_ptr()) }
    }
    /// Gets the size of the icon in physical pixels.
    ///
    /// See [C++ `wxIcon::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a5f07d442a47f9d02ec26b9543ccaea83).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxIcon_GetSize(self.as_ptr())) }
    }
    /// Gets the width of the icon in physical pixels.
    ///
    /// See [C++ `wxIcon::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a84442d3fad645f79650ae53eca139a5f).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxIcon_GetWidth(self.as_ptr()) }
    }
    /// Returns true if icon data is present.
    ///
    /// See [C++ `wxIcon::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a77cb179b5a1ab9985e9c0d540562f1a9).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxIcon_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LoadFile()
    // BLOCKED: fn SetDepth()
    // BLOCKED: fn SetHeight()
    // BLOCKED: fn SetWidth()
    // BLOCKED: fn operator=()
}

// wxIconBundle
/// This trait represents [C++ `wxIconBundle` class](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html)'s methods and inheritance.
///
/// See [`IconBundleIsOwned`] documentation for the class usage.
pub trait IconBundleMethods: GDIObjectMethods {
    // DTOR: fn ~wxIconBundle()
    // NOT_SUPPORTED: fn AddIcon()
    // NOT_SUPPORTED: fn AddIcon1()
    // NOT_SUPPORTED: fn AddIcon2()
    /// Adds the icon to the collection; if the collection already contains an icon with the same width and height, it is replaced by the new one.
    ///
    /// See [C++ `wxIconBundle::AddIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a0a8c33284a082b901079e2c5042aa1d2).
    fn add_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxIconBundle_AddIcon3(self.as_ptr(), icon)
        }
    }
    /// Returns the icon with the given size.
    ///
    /// See [C++ `wxIconBundle::GetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#aa005cd0b46f8dc5fc66573a24ec123e7).
    fn get_icon_size<S: SizeMethods>(&self, size: &S, flags: c_int) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxIconBundle_GetIcon(self.as_ptr(), size, flags))
        }
    }
    /// Same as.
    ///
    /// See [C++ `wxIconBundle::GetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a46b7a0ef6d5bc2a011fbfb073f5b49bc).
    fn get_icon_coord(&self, size: c_int, flags: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxIconBundle_GetIcon1(self.as_ptr(), size, flags)) }
    }
    /// Returns the icon with exactly the given size or wxNullIcon if this size is not available.
    ///
    /// See [C++ `wxIconBundle::GetIconOfExactSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a4869c3dddd6feb1de2754907527b33e2).
    fn get_icon_of_exact_size<S: SizeMethods>(&self, size: &S) -> Icon {
        unsafe {
            let size = size.as_ptr();
            Icon::from_ptr(ffi::wxIconBundle_GetIconOfExactSize(self.as_ptr(), size))
        }
    }
    /// return the number of available icons
    ///
    /// See [C++ `wxIconBundle::GetIconCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a94c13e967121bb2600bf6b4cdf37db5f).
    fn get_icon_count(&self) -> usize {
        unsafe { ffi::wxIconBundle_GetIconCount(self.as_ptr()) }
    }
    /// return the icon at index (must be < GetIconCount())
    ///
    /// See [C++ `wxIconBundle::GetIconByIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#abce02205e483898f574e19be7a3f6813).
    fn get_icon_by_index(&self, n: usize) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxIconBundle_GetIconByIndex(self.as_ptr(), n)) }
    }
    /// Returns true if the bundle doesn't contain any icons, false otherwise (in which case a call to GetIcon() with default parameter should return a valid icon).
    ///
    /// See [C++ `wxIconBundle::IsEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a386f43ef20f51856339ed676efd90273).
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxIconBundle_IsEmpty(self.as_ptr()) }
    }
    // BLOCKED: fn operator=()
}

// wxIconizeEvent
/// This trait represents [C++ `wxIconizeEvent` class](https://docs.wxwidgets.org/3.2/classwx_iconize_event.html)'s methods and inheritance.
///
/// See [`IconizeEventIsOwned`] documentation for the class usage.
pub trait IconizeEventMethods: EventMethods {
    /// Returns true if the frame has been iconized, false if it has been restored.
    ///
    /// See [C++ `wxIconizeEvent::IsIconized()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_iconize_event.html#a0f6c68ec519a3af9fd22cefdd52d31a8).
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxIconizeEvent_IsIconized(self.as_ptr()) }
    }
    // BLOCKED: fn Iconized()
}

// wxIdManager
/// This trait represents [C++ `wxIdManager` class](https://docs.wxwidgets.org/3.2/classwx_id_manager.html)'s methods and inheritance.
///
/// See [`IdManagerIsOwned`] documentation for the class usage.
pub trait IdManagerMethods: WxRustMethods {
    /// Called directly by wxWindow::NewControlId(), this function will create a new ID or range of IDs.
    ///
    /// See [C++ `wxIdManager::ReserveId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_id_manager.html#a3038d645cbbb9ce1f89dcafe830275a6).
    fn reserve_id(count: c_int) -> c_int {
        unsafe { ffi::wxIdManager_ReserveId(count) }
    }
    /// Called directly by wxWindow::UnreserveControlId(), this function will unreserve an ID or range of IDs that is currently reserved.
    ///
    /// See [C++ `wxIdManager::UnreserveId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_id_manager.html#a48e2dcc02bb3998861a73de7959b377d).
    fn unreserve_id(id: c_int, count: c_int) {
        unsafe { ffi::wxIdManager_UnreserveId(id, count) }
    }
}

// wxImage
/// This trait represents [C++ `wxImage` class](https://docs.wxwidgets.org/3.2/classwx_image.html)'s methods and inheritance.
///
/// See [`ImageIsOwned`] documentation for the class usage.
pub trait ImageMethods: ObjectMethods {
    /// Returns an identical copy of this image.
    ///
    /// See [C++ `wxImage::Copy()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aa91b90641c946cbcabd5a0677cf85834).
    fn copy(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Copy(self.as_ptr())) }
    }
    /// Creates a fresh image.
    ///
    /// See [C++ `wxImage::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a07194d90bd6320255610a0a88a030a3e).
    fn create_int_bool(&self, width: c_int, height: c_int, clear: bool) -> bool {
        unsafe { ffi::wxImage_Create(self.as_ptr(), width, height, clear) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a37e70a8f9c684974c9c54f43f5e60a3f).
    fn create_size_bool<S: SizeMethods>(&self, sz: &S, clear: bool) -> bool {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxImage_Create1(self.as_ptr(), sz, clear)
        }
    }
    /// Creates a fresh image.
    ///
    /// See [C++ `wxImage::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a63801a380397cdcf465cab68f313c9a6).
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
    ///
    /// See [C++ `wxImage::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a717414a9d74dc882359857050bb4bdc3).
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
    ///
    /// See [C++ `wxImage::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aa395d661de4f3b98c6e92a767d97bb66).
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
    ///
    /// See [C++ `wxImage::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aae3a6d125108f6fb93188d8c3c4990b9).
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
    ///
    /// See [C++ `wxImage::Destroy()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ac64c3524a054db164b811c380db65f97).
    fn destroy(&self) {
        unsafe { ffi::wxImage_Destroy(self.as_ptr()) }
    }
    /// Initializes the image alpha channel data.
    ///
    /// See [C++ `wxImage::InitAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ac941b6b80262b968b575ab1b215e687b).
    fn init_alpha(&self) {
        unsafe { ffi::wxImage_InitAlpha(self.as_ptr()) }
    }
    /// Blurs the image in both horizontal and vertical directions by the specified pixel blurRadius.
    ///
    /// See [C++ `wxImage::Blur()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a9fd6442bc91dba50db6c29b6c6167fb8).
    fn blur(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Blur(self.as_ptr(), blur_radius)) }
    }
    /// Blurs the image in the horizontal direction only.
    ///
    /// See [C++ `wxImage::BlurHorizontal()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ab30dee077c0134de25964a33c5d242c3).
    fn blur_horizontal(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_BlurHorizontal(self.as_ptr(), blur_radius)) }
    }
    /// Blurs the image in the vertical direction only.
    ///
    /// See [C++ `wxImage::BlurVertical()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ac54487ddc5d8c8a8566a2d8ddd454199).
    fn blur_vertical(&self, blur_radius: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_BlurVertical(self.as_ptr(), blur_radius)) }
    }
    /// Returns a mirrored copy of the image.
    ///
    /// See [C++ `wxImage::Mirror()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ac033f86a6923200696ea941695cd680b).
    fn mirror(&self, horizontally: bool) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Mirror(self.as_ptr(), horizontally)) }
    }
    // NOT_SUPPORTED: fn Paste()
    // NOT_SUPPORTED: fn Replace()
    // NOT_SUPPORTED: fn Rescale()
    /// Changes the size of the image in-place without scaling it by adding either a border with the given colour or cropping as necessary.
    ///
    /// See [C++ `wxImage::Resize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ac2f7069343541eef9ed44d088b3dc9fa).
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
    ///
    /// See [C++ `wxImage::Rotate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a3532210633dae17f21e99d111b1ef0b2).
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
    ///
    /// See [C++ `wxImage::Rotate90()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#abe60eccfbf41e5e4d00de2af8fd9637b).
    fn rotate90(&self, clockwise: bool) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Rotate90(self.as_ptr(), clockwise)) }
    }
    /// Returns a copy of the image rotated by 180 degrees.
    ///
    /// See [C++ `wxImage::Rotate180()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae3fb8e5e4518ed21eecf1174df388827).
    fn rotate180(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_Rotate180(self.as_ptr())) }
    }
    /// Rotates the hue of each pixel in the image by angle, which is a double in the range [-1.0..+1.0], where -1.0 corresponds to -360 degrees and +1.0 corresponds to +360 degrees.
    ///
    /// See [C++ `wxImage::RotateHue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a96a43d7bbb26ed775fa10c979c6e511f).
    fn rotate_hue(&self, angle: c_double) {
        unsafe { ffi::wxImage_RotateHue(self.as_ptr(), angle) }
    }
    /// Changes the saturation of each pixel in the image.
    ///
    /// See [C++ `wxImage::ChangeSaturation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae79b3dee30b7292ae2c69f1930a39bfe).
    fn change_saturation(&self, factor: c_double) {
        unsafe { ffi::wxImage_ChangeSaturation(self.as_ptr(), factor) }
    }
    /// Changes the brightness (value) of each pixel in the image.
    ///
    /// See [C++ `wxImage::ChangeBrightness()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aff38604c9b6cb54139097d0ebd2f666a).
    fn change_brightness(&self, factor: c_double) {
        unsafe { ffi::wxImage_ChangeBrightness(self.as_ptr(), factor) }
    }
    /// Changes the hue, the saturation and the brightness (value) of each pixel in the image.
    ///
    /// See [C++ `wxImage::ChangeHSV()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#add2305734a1e1cda13f3fb0f71ef6312).
    fn change_hsv(&self, angle_h: c_double, factor_s: c_double, factor_v: c_double) {
        unsafe { ffi::wxImage_ChangeHSV(self.as_ptr(), angle_h, factor_s, factor_v) }
    }
    // NOT_SUPPORTED: fn Scale()
    /// Returns a resized version of this image without scaling it by adding either a border with the given colour or cropping as necessary.
    ///
    /// See [C++ `wxImage::Size()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a365621ba73e792f40c486d8a65a0cb9b).
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
    ///
    /// See [C++ `wxImage::ConvertToGreyscale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a7718921e0955a0b98266f9c47546408a).
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
    ///
    /// See [C++ `wxImage::ConvertToGreyscale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#abc9ddbe1370677981685a8416a51a4f8).
    fn convert_to_greyscale(&self) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_ConvertToGreyscale1(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn ConvertToMono()
    // NOT_SUPPORTED: fn ConvertToDisabled()
    /// Returns a changed version of the image based on the given lightness.
    ///
    /// See [C++ `wxImage::ChangeLightness()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a2bee3672842b72fcf295f20cc00b3c16).
    fn change_lightness(&self, alpha: c_int) -> Image {
        unsafe { Image::from_ptr(ffi::wxImage_ChangeLightness(self.as_ptr(), alpha)) }
    }
    // NOT_SUPPORTED: fn ComputeHistogram()
    // NOT_SUPPORTED: fn FindFirstUnusedColour()
    // BLOCKED: fn operator=()
    /// Returns pointer to the array storing the alpha values for this image.
    ///
    /// See [C++ `wxImage::GetAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a6e59c7c1945c15df7e6002605864b028).
    fn get_alpha(&self) -> *mut c_void {
        unsafe { ffi::wxImage_GetAlpha(self.as_ptr()) }
    }
    /// Returns the image data as an array.
    ///
    /// See [C++ `wxImage::GetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a2c215115d47da1e4beb1ecabc4ba53aa).
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
    ///
    /// See [C++ `wxImage::GetWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a038af12899d797dad9385b0b62aa605a).
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxImage_GetWidth(self.as_ptr()) }
    }
    /// Gets the height of the image in pixels.
    ///
    /// See [C++ `wxImage::GetHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a96c60d4c78e742700a458f52661f489c).
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxImage_GetHeight(self.as_ptr()) }
    }
    /// Returns the size of the image in pixels.
    ///
    /// See [C++ `wxImage::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a921baa8d74fff2deb66a9c47b6588f2f).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxImage_GetSize(self.as_ptr())) }
    }
    /// Gets a user-defined string-valued option.
    ///
    /// See [C++ `wxImage::GetOption()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#af223e5d8a96e081bfc80271f4dfa8fc7).
    fn get_option(&self, name: &str) -> String {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            WxString::from_ptr(ffi::wxImage_GetOption(self.as_ptr(), name)).into()
        }
    }
    /// Gets a user-defined integer-valued option.
    ///
    /// See [C++ `wxImage::GetOptionInt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ad2ee17ab0ab8654ea14e0ebb18d55ca9).
    fn get_option_int(&self, name: &str) -> c_int {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_GetOptionInt(self.as_ptr(), name)
        }
    }
    /// Get the current mask colour or find a suitable unused colour that could be used as a mask colour.
    ///
    /// See [C++ `wxImage::GetOrFindMaskColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a6d1d3d87a8d0d9b46d331c19f3ce847f).
    fn get_or_find_mask_colour(&self, r: *mut c_void, g: *mut c_void, b: *mut c_void) -> bool {
        unsafe { ffi::wxImage_GetOrFindMaskColour(self.as_ptr(), r, g, b) }
    }
    // BLOCKED: fn GetPalette()
    /// Returns a sub image of the current one as long as the rect belongs entirely to the image.
    ///
    /// See [C++ `wxImage::GetSubImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a808d987824786d2777ec9962cb522f69).
    fn get_sub_image<R: RectMethods>(&self, rect: &R) -> Image {
        unsafe {
            let rect = rect.as_ptr();
            Image::from_ptr(ffi::wxImage_GetSubImage(self.as_ptr(), rect))
        }
    }
    // NOT_SUPPORTED: fn GetType()
    /// Returns true if this image has alpha channel, false otherwise.
    ///
    /// See [C++ `wxImage::HasAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#accd85932f27036014ca00997897e1040).
    fn has_alpha(&self) -> bool {
        unsafe { ffi::wxImage_HasAlpha(self.as_ptr()) }
    }
    /// Returns true if there is a mask active, false otherwise.
    ///
    /// See [C++ `wxImage::HasMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aabe81dbe168e7cac8815cb632fd6c11e).
    fn has_mask(&self) -> bool {
        unsafe { ffi::wxImage_HasMask(self.as_ptr()) }
    }
    /// Returns true if the given option is present.
    ///
    /// See [C++ `wxImage::HasOption()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ab666653f2de8e8ef7de6f8fb381b0053).
    fn has_option(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_HasOption(self.as_ptr(), name)
        }
    }
    /// Returns true if image data is present.
    ///
    /// See [C++ `wxImage::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ad0467837c346bbdc3e312459e4b1e596).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxImage_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsTransparent()
    // NOT_SUPPORTED: fn LoadFile()
    // NOT_SUPPORTED: fn LoadFile1()
    /// Loads an image from a file.
    ///
    /// See [C++ `wxImage::LoadFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a5261b155bbc93ed1719462d80ce699be).
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
    ///
    /// See [C++ `wxImage::LoadFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a318ae843ecc2e10fdc460865dff49a99).
    fn load_file_inputstream(&self, stream: *mut c_void, mimetype: &str, index: c_int) -> bool {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_LoadFile3(self.as_ptr(), stream, mimetype, index)
        }
    }
    /// Saves an image in the given stream.
    ///
    /// See [C++ `wxImage::SaveFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#adcfbb6157c79bc142811d0faf6a85b2c).
    fn save_file_outputstream(&self, stream: *mut c_void, mimetype: &str) -> bool {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImage_SaveFile(self.as_ptr(), stream, mimetype)
        }
    }
    // NOT_SUPPORTED: fn SaveFile1()
    /// Saves an image in the named file.
    ///
    /// See [C++ `wxImage::SaveFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae11d5b3b139d9817bd2a781d9d114e50).
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
    ///
    /// See [C++ `wxImage::SaveFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a17d2048a0dc4f85a86839059c094afeb).
    fn save_file_str(&self, name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_SaveFile3(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SaveFile4()
    /// This function is similar to SetData() and has similar restrictions.
    ///
    /// See [C++ `wxImage::SetAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ab46f59f69221258da1397fd7c59e8671).
    fn set_alpha(&self, alpha: *mut c_void, static_data: bool) {
        unsafe { ffi::wxImage_SetAlpha(self.as_ptr(), alpha, static_data) }
    }
    // NOT_SUPPORTED: fn SetAlpha1()
    /// Removes the alpha channel from the image.
    ///
    /// See [C++ `wxImage::ClearAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a1b82ae0550626a74c3ee97d9065628b1).
    fn clear_alpha(&self) {
        unsafe { ffi::wxImage_ClearAlpha(self.as_ptr()) }
    }
    /// Sets the image data without performing checks.
    ///
    /// See [C++ `wxImage::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a736a8fc26d32e5eab70682b115bacacf).
    fn set_data_bool(&self, data: *mut c_void, static_data: bool) {
        unsafe { ffi::wxImage_SetData(self.as_ptr(), data, static_data) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#afda57bc7ba823e7c060dbd7698c6ad31).
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
    ///
    /// See [C++ `wxImage::SetLoadFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aa32e5d3507cc0f8c3330135bc0befc6a).
    fn set_load_flags(&self, flags: c_int) {
        unsafe { ffi::wxImage_SetLoadFlags(self.as_ptr(), flags) }
    }
    /// Specifies whether there is a mask or not.
    ///
    /// See [C++ `wxImage::SetMask()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a62645fe559149a8ee6b27ddc07cfc220).
    fn set_mask(&self, has_mask: bool) {
        unsafe { ffi::wxImage_SetMask(self.as_ptr(), has_mask) }
    }
    // NOT_SUPPORTED: fn SetMaskColour()
    // NOT_SUPPORTED: fn SetMaskFromImage()
    /// Sets a user-defined option.
    ///
    /// See [C++ `wxImage::SetOption()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a952ee065b17ee07a8cbee568486e01a4).
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
    ///
    /// See [C++ `wxImage::SetOption()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#acc3d48af8a50bff5ceca51fd649ec981).
    fn set_option_int(&self, name: &str, value: c_int) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_SetOption1(self.as_ptr(), name, value)
        }
    }
    /// Associates a palette with the image.
    ///
    /// See [C++ `wxImage::SetPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#acbf3a8b9f954b7da7d65adbb9cc9e026).
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
    ///
    /// See [C++ `wxImage::SetDefaultLoadFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a31cf23299e54403d538ecbd9ea60fb25).
    fn set_default_load_flags(flags: c_int) {
        unsafe { ffi::wxImage_SetDefaultLoadFlags(flags) }
    }
    /// Returns the file load flags used for this object.
    ///
    /// See [C++ `wxImage::GetLoadFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a28503a506d7354584942764f775694e8).
    fn get_load_flags(&self) -> c_int {
        unsafe { ffi::wxImage_GetLoadFlags(self.as_ptr()) }
    }
    /// Register an image handler.
    ///
    /// See [C++ `wxImage::AddHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ab39fb3747dfb8c2d444eff9fe41fa205).
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
    ///
    /// See [C++ `wxImage::CleanUpHandlers()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aa50b222b999e7cd690f2a5be0e0e9cef).
    fn clean_up_handlers() {
        unsafe { ffi::wxImage_CleanUpHandlers() }
    }
    /// Finds the handler with the given name.
    ///
    /// See [C++ `wxImage::FindHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a474c80c4f2a344d131fc9b26778f8440).
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
    ///
    /// See [C++ `wxImage::FindHandlerMime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a464676dac510648439549482e4ed07bd).
    fn find_handler_mime(mimetype: &str) -> Option<ImageHandlerIsOwned<false>> {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageHandler::option_from(ffi::wxImage_FindHandlerMime(mimetype))
        }
    }
    // BLOCKED: fn GetHandlers()
    /// Internal use only.
    ///
    /// See [C++ `wxImage::InitStandardHandlers()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ab2f032e0b7766dc69d4f092f4a55fd60).
    fn init_standard_handlers() {
        unsafe { ffi::wxImage_InitStandardHandlers() }
    }
    /// Adds a handler at the start of the static list of format handlers.
    ///
    /// See [C++ `wxImage::InsertHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#adcecda55890d6e9e3c827751a9ec42b0).
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
    ///
    /// See [C++ `wxImage::RemoveHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a70ef86ff481bc18c0a32f5467a071e42).
    fn remove_handler(name: &str) -> bool {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxImage_RemoveHandler(name)
        }
    }
    /// Returns true if at least one of the available image handlers can read the file with the given name.
    ///
    /// See [C++ `wxImage::CanRead()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a786fdfa44290867e1b1fd64bcd26aded).
    fn can_read_str(filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxImage_CanRead(filename)
        }
    }
    /// Returns true if at least one of the available image handlers can read the data in the given stream.
    ///
    /// See [C++ `wxImage::CanRead()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aac069cba7821715661e03d9b6eeee98f).
    fn can_read_inputstream(stream: *mut c_void) -> bool {
        unsafe { ffi::wxImage_CanRead1(stream) }
    }
    /// Returns the currently used default file load flags.
    ///
    /// See [C++ `wxImage::GetDefaultLoadFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#aed3c02fb1f572fb030a201c7c1b65f33).
    fn get_default_load_flags() -> c_int {
        unsafe { ffi::wxImage_GetDefaultLoadFlags() }
    }
    // NOT_SUPPORTED: fn GetImageCount()
    // NOT_SUPPORTED: fn GetImageCount1()
    /// Iterates all registered wxImageHandler objects, and returns a string containing file extension masks suitable for passing to file open/save dialog boxes.
    ///
    /// See [C++ `wxImage::GetImageExtWildcard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a0ed7bd5c0eba03553b6533e2f79e0ff1).
    fn get_image_ext_wildcard() -> String {
        unsafe { WxString::from_ptr(ffi::wxImage_GetImageExtWildcard()).into() }
    }
    // NOT_SUPPORTED: fn RGBtoHSV()
    // NOT_SUPPORTED: fn HSVtoRGB()
    // DTOR: fn ~wxImage()
}

// wxImageHandler
/// This trait represents [C++ `wxImageHandler` class](https://docs.wxwidgets.org/3.2/classwx_image_handler.html)'s methods and inheritance.
///
/// See [`ImageHandlerIsOwned`] documentation for the class usage.
pub trait ImageHandlerMethods: ObjectMethods {
    // DTOR: fn ~wxImageHandler()
    /// Returns true if this handler supports the image format contained in the given stream.
    ///
    /// See [C++ `wxImageHandler::CanRead()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a7c886c9f2192699183e480066e386133).
    fn can_read_inputstream(&self, stream: *mut c_void) -> bool {
        unsafe { ffi::wxImageHandler_CanRead(self.as_ptr(), stream) }
    }
    /// Returns true if this handler supports the image format contained in the file with the given name.
    ///
    /// See [C++ `wxImageHandler::CanRead()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a91fadb9ace6a42e8cf3c66061e4bbfc9).
    fn can_read_str(&self, filename: &str) -> bool {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            ffi::wxImageHandler_CanRead1(self.as_ptr(), filename)
        }
    }
    /// Gets the preferred file extension associated with this handler.
    ///
    /// See [C++ `wxImageHandler::GetExtension()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a50abd57b932c049e3f48c064f2060e59).
    fn get_extension(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetExtension(self.as_ptr())).into() }
    }
    /// Returns the other file extensions associated with this handler.
    ///
    /// See [C++ `wxImageHandler::GetAltExtensions()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#ad7507ccdf4d120820bd7be84826e86dd).
    fn get_alt_extensions(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxImageHandler_GetAltExtensions(self.as_ptr())) }
    }
    /// If the image file contains more than one image and the image handler is capable of retrieving these individually, this function will return the number of available images.
    ///
    /// See [C++ `wxImageHandler::GetImageCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a1934418d6f844d7db650926d2a4e5f18).
    fn get_image_count(&self, stream: *mut c_void) -> c_int {
        unsafe { ffi::wxImageHandler_GetImageCount(self.as_ptr(), stream) }
    }
    /// Gets the MIME type associated with this handler.
    ///
    /// See [C++ `wxImageHandler::GetMimeType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#aface63bd833c98f470898f7952dd2062).
    fn get_mime_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetMimeType(self.as_ptr())).into() }
    }
    /// Gets the name of this handler.
    ///
    /// See [C++ `wxImageHandler::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#aaa8dbc9a569147c35ffb44b628d8cac4).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxImageHandler_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    /// Loads an image from a stream, putting the resulting data into image.
    ///
    /// See [C++ `wxImageHandler::LoadFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a39793e7d6c1e0138330b4c6727e26861).
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
    ///
    /// See [C++ `wxImageHandler::SaveFile()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a165394d021a199f207ae2910a3ba72e8).
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
    ///
    /// See [C++ `wxImageHandler::SetExtension()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#abe9bfbb83e7f883f3d053264e3283f61).
    fn set_extension(&self, extension: &str) {
        unsafe {
            let extension = WxString::from(extension);
            let extension = extension.as_ptr();
            ffi::wxImageHandler_SetExtension(self.as_ptr(), extension)
        }
    }
    /// Sets the alternative file extensions associated with this handler.
    ///
    /// See [C++ `wxImageHandler::SetAltExtensions()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a65ba010611387392913569867ff60fc2).
    fn set_alt_extensions<A: ArrayStringMethods>(&self, extensions: &A) {
        unsafe {
            let extensions = extensions.as_ptr();
            ffi::wxImageHandler_SetAltExtensions(self.as_ptr(), extensions)
        }
    }
    /// Sets the handler MIME type.
    ///
    /// See [C++ `wxImageHandler::SetMimeType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#a0bcdad1b9948249f511f461323c8a4c9).
    fn set_mime_type(&self, mimetype: &str) {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ffi::wxImageHandler_SetMimeType(self.as_ptr(), mimetype)
        }
    }
    /// Sets the handler name.
    ///
    /// See [C++ `wxImageHandler::SetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html#aec863a8d58771b3c103551cf7a41eaf2).
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
/// This trait represents [C++ `wxImageList` class](https://docs.wxwidgets.org/3.2/classwx_image_list.html)'s methods and inheritance.
///
/// See [`ImageListIsOwned`] documentation for the class usage.
pub trait ImageListMethods: ObjectMethods {
    /// Adds a new image or images using a bitmap and optional mask bitmap.
    ///
    /// See [C++ `wxImageList::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a697f268509e3a8d45da7506349b7533d).
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
    ///
    /// See [C++ `wxImageList::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a2022beccd2564ee2be79b273246b50a8).
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
    ///
    /// See [C++ `wxImageList::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a5cdf301d2c4ec6d074816a7566d6d7c2).
    fn add_icon<I: IconMethods>(&self, icon: &I) -> c_int {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxImageList_Add2(self.as_ptr(), icon)
        }
    }
    /// Initializes the list.
    ///
    /// See [C++ `wxImageList::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a03b4ba6c8a868eff25f3d20cb3499673).
    fn create(&self, width: c_int, height: c_int, mask: bool, initial_count: c_int) -> bool {
        unsafe { ffi::wxImageList_Create(self.as_ptr(), width, height, mask, initial_count) }
    }
    /// Destroys the current list.
    ///
    /// See [C++ `wxImageList::Destroy()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#ac2dc14b1ee81666798e2a1fa3bef206d).
    fn destroy(&self) {
        unsafe { ffi::wxImageList_Destroy(self.as_ptr()) }
    }
    /// Draws a specified image onto a device context.
    ///
    /// See [C++ `wxImageList::Draw()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#aca7da30ce467d74a5e9037ad99ec3287).
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
    ///
    /// See [C++ `wxImageList::GetBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a0002fd12fde3e0d1498535e2cfb09b43).
    fn get_bitmap(&self, index: c_int) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxImageList_GetBitmap(self.as_ptr(), index)) }
    }
    /// Returns the icon corresponding to the given index.
    ///
    /// See [C++ `wxImageList::GetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a8cee2c05aa962f7e45d3bd1ee38da8a6).
    fn get_icon(&self, index: c_int) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxImageList_GetIcon(self.as_ptr(), index)) }
    }
    /// Returns the number of images in the list.
    ///
    /// See [C++ `wxImageList::GetImageCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a4bc7dc442d6266194e0351e2c9baa56c).
    fn get_image_count(&self) -> c_int {
        unsafe { ffi::wxImageList_GetImageCount(self.as_ptr()) }
    }
    /// Retrieves the size of the images in the list.
    ///
    /// See [C++ `wxImageList::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a98be009d306295c1f24f22737ee2aebc).
    fn get_size_int(&self, index: c_int, width: *mut c_void, height: *mut c_void) -> bool {
        unsafe { ffi::wxImageList_GetSize(self.as_ptr(), index, width, height) }
    }
    /// Retrieves the size of the image list as passed to Create().
    ///
    /// See [C++ `wxImageList::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#aa291d7beda14ea5ba64ddeed20eec8d4).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxImageList_GetSize1(self.as_ptr())) }
    }
    /// Removes the image at the given position.
    ///
    /// See [C++ `wxImageList::Remove()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a70e5bc2c695d3bdd1a3a151c8774b9ee).
    fn remove(&self, index: c_int) -> bool {
        unsafe { ffi::wxImageList_Remove(self.as_ptr(), index) }
    }
    /// Removes all the images in the list.
    ///
    /// See [C++ `wxImageList::RemoveAll()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a9de067947c75fc53798541cad5d6ebce).
    fn remove_all(&self) -> bool {
        unsafe { ffi::wxImageList_RemoveAll(self.as_ptr()) }
    }
    /// Replaces the existing image with the new image.
    ///
    /// See [C++ `wxImageList::Replace()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#ada78532ad344127e0917d363a34783a4).
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
    ///
    /// See [C++ `wxImageList::Replace()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#af003aa28ad738528ef15d873e8e96756).
    fn replace_icon<I: IconMethods>(&self, index: c_int, icon: &I) -> bool {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxImageList_Replace1(self.as_ptr(), index, icon)
        }
    }
}

// wxInfoBar
/// This trait represents [C++ `wxInfoBar` class](https://docs.wxwidgets.org/3.2/classwx_info_bar.html)'s methods and inheritance.
///
/// See [`InfoBarIsOwned`] documentation for the class usage.
pub trait InfoBarMethods: ControlMethods {
    // NOT_SUPPORTED: fn SetShowHideEffects()
    // NOT_SUPPORTED: fn GetShowEffect()
    // NOT_SUPPORTED: fn GetHideEffect()
    /// Set the duration of the animation used when showing or hiding the bar.
    ///
    /// See [C++ `wxInfoBar::SetEffectDuration()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a466a967c34d3aadc85f23de17712f605).
    fn set_effect_duration(&self, duration: c_int) {
        unsafe { ffi::wxInfoBar_SetEffectDuration(self.as_ptr(), duration) }
    }
    /// Return the effect animation duration currently used.
    ///
    /// See [C++ `wxInfoBar::GetEffectDuration()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#ae9a76139fe37053b91d7e4acc62218e8).
    fn get_effect_duration(&self) -> c_int {
        unsafe { ffi::wxInfoBar_GetEffectDuration(self.as_ptr()) }
    }
    /// Create the info bar window.
    ///
    /// See [C++ `wxInfoBar::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a67c612de758bfafba1f00c22efe36dd2).
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
    ///
    /// See [C++ `wxInfoBar::AddButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a6c2e88fe0c76e851948445e20f092ff1).
    fn add_button(&self, btnid: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxInfoBar_AddButton(self.as_ptr(), btnid, label)
        }
    }
    /// Hide the info bar window.
    ///
    /// See [C++ `wxInfoBar::Dismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a3fd73896ffa7263985ad7b99aca3d5c9).
    fn dismiss(&self) {
        unsafe { ffi::wxInfoBar_Dismiss(self.as_ptr()) }
    }
    /// Remove a button previously added by AddButton().
    ///
    /// See [C++ `wxInfoBar::RemoveButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#aef66e90d5b841221c2a2f376d76b47e0).
    fn remove_button(&self, btnid: c_int) {
        unsafe { ffi::wxInfoBar_RemoveButton(self.as_ptr(), btnid) }
    }
    /// Show a message in the bar.
    ///
    /// See [C++ `wxInfoBar::ShowMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#ac8c134e479b19a8a99c1c4c78f266524).
    fn show_message(&self, msg: &str, flags: c_int) {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            ffi::wxInfoBar_ShowMessage(self.as_ptr(), msg, flags)
        }
    }
    /// Returns the number of currently shown buttons.
    ///
    /// See [C++ `wxInfoBar::GetButtonCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a95807182df6b8d3a60780c5363e54b6f).
    fn get_button_count(&self) -> usize {
        unsafe { ffi::wxInfoBar_GetButtonCount(self.as_ptr()) }
    }
    /// Returns the ID of the button at the given position.
    ///
    /// See [C++ `wxInfoBar::GetButtonId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a42f2283c358128e268f61c97ee39a748).
    fn get_button_id(&self, idx: usize) -> c_int {
        unsafe { ffi::wxInfoBar_GetButtonId(self.as_ptr(), idx) }
    }
    /// Returns whether a button with the given ID is currently shown.
    ///
    /// See [C++ `wxInfoBar::HasButtonId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a5087ddf888919508461b278037c23098).
    fn has_button_id(&self, btnid: c_int) -> bool {
        unsafe { ffi::wxInfoBar_HasButtonId(self.as_ptr(), btnid) }
    }
}

// wxInitDialogEvent
/// This trait represents [C++ `wxInitDialogEvent` class](https://docs.wxwidgets.org/3.2/classwx_init_dialog_event.html)'s methods and inheritance.
///
/// See [`InitDialogEventIsOwned`] documentation for the class usage.
pub trait InitDialogEventMethods: EventMethods {}

// wxItemContainer
/// This trait represents [C++ `wxItemContainer` class](https://docs.wxwidgets.org/3.2/classwx_item_container.html)'s methods and inheritance.
///
/// See [`ItemContainerIsOwned`] documentation for the class usage.
pub trait ItemContainerMethods: ItemContainerImmutableMethods {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    /// Appends item into the control.
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a8fdc0090e3eabc762ff0e49e925f8bc4).
    fn append_str(&self, item: &str) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Append(self.as_item_container(), item)
        }
    }
    /// Appends item into the control.
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a97d16e94976e21abf796cc6e0c8c0fd0).
    fn append_str_void(&self, item: &str, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Append1(self.as_item_container(), item, client_data)
        }
    }
    /// Appends item into the control.
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a8cb993082012406873ac3ef1b91774f5).
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
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a4c8560213df6b6e7467437c9cff5cc0e).
    fn append_arraystring<A: ArrayStringMethods>(&self, items: &A) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Append3(self.as_item_container(), items)
        }
    }
    // BLOCKED: fn Append4()
    /// Appends several items at once into the control.
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a4de5b5afbf2aebe2ee29c11d009fbe75).
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
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ada2c60d0621a4a2d62a50345778e13f3).
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
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ab75a452e13a6b21e32caae829a026515).
    fn append_uint(&self, n: c_uint, items: *const c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append7(self.as_item_container(), n, items) }
    }
    /// Appends several items at once into the control.
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a87fe2b791693f23475b77ffb037a7766).
    fn append_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) -> c_int {
        unsafe { ffi::wxItemContainer_Append8(self.as_item_container(), n, items, client_data) }
    }
    /// Appends several items at once into the control.
    ///
    /// See [C++ `wxItemContainer::Append()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ab328ab2886486b545f03daa667308a63).
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
    ///
    /// See [C++ `wxItemContainer::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#aea621d4fdfbc3a06bf24dcc97304e2c1).
    fn clear(&self) {
        unsafe { ffi::wxItemContainer_Clear(self.as_item_container()) }
    }
    /// Deletes an item from the control.
    ///
    /// See [C++ `wxItemContainer::Delete()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a0e8379f41e9d7b912564000828140a19).
    fn delete(&self, n: c_uint) {
        unsafe { ffi::wxItemContainer_Delete(self.as_item_container(), n) }
    }
    /// Returns the client object associated with the given item and transfers its ownership to the caller.
    ///
    /// See [C++ `wxItemContainer::DetachClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a9b4c80ee2cc91ccfe534fb56fc4e8bdf).
    fn detach_client_object(&self, n: c_uint) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxItemContainer_DetachClientObject(
                self.as_item_container(),
                n,
            ))
        }
    }
    /// Returns true, if either untyped data (void*) or object data (wxClientData*) is associated with the items of the control.
    ///
    /// See [C++ `wxItemContainer::HasClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a3d217a4c721a5cf2787672624744cac4).
    fn has_client_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientData(self.as_item_container()) }
    }
    /// Returns true, if object data is associated with the items of the control.
    ///
    /// See [C++ `wxItemContainer::HasClientObjectData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a7510107d6022ebab364a23112e21ae9f).
    fn has_client_object_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientObjectData(self.as_item_container()) }
    }
    /// Returns true, if untyped data (void*) is associated with the items of the control.
    ///
    /// See [C++ `wxItemContainer::HasClientUntypedData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ad1887e59bb72192a535c6b92d88692d2).
    fn has_client_untyped_data(&self) -> bool {
        unsafe { ffi::wxItemContainer_HasClientUntypedData(self.as_item_container()) }
    }
    /// Returns a pointer to the client data associated with the given item (if any).
    ///
    /// See [C++ `wxItemContainer::GetClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ab7ef8c064132041f62180e5764b3669a).
    fn get_client_data(&self, n: c_uint) -> *mut c_void {
        unsafe { ffi::wxItemContainer_GetClientData(self.as_item_container(), n) }
    }
    /// Returns a pointer to the client data associated with the given item (if any).
    ///
    /// See [C++ `wxItemContainer::GetClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a95f1b730f3ddcf8eb2d8d8ec4e09dadd).
    fn get_client_object_uint(&self, n: c_uint) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            ClientData::option_from(ffi::wxItemContainer_GetClientObject(
                self.as_item_container(),
                n,
            ))
        }
    }
    /// Associates the given untyped client data pointer with the given item.
    ///
    /// See [C++ `wxItemContainer::SetClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a4316a72acaf30acf9d2c9c457e16a8d9).
    fn set_client_data(&self, n: c_uint, data: *mut c_void) {
        unsafe { ffi::wxItemContainer_SetClientData(self.as_item_container(), n, data) }
    }
    /// Associates the given typed client data pointer with the given item: the data object will be deleted when the item is deleted (either explicitly by using Delete() or implicitly when the control itself is destroyed).
    ///
    /// See [C++ `wxItemContainer::SetClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ae0a26343071841e8d1139642a0081d9f).
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
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a8844cacec8509fe6e637c6f85eb8b395).
    fn insert_str_uint(&self, item: &str, pos: c_uint) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Insert(self.as_item_container(), item, pos)
        }
    }
    /// Inserts item into the control.
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a6540ebe5868badb8ff7ac4975c054309).
    fn insert_str_uint_void(&self, item: &str, pos: c_uint, client_data: *mut c_void) -> c_int {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxItemContainer_Insert1(self.as_item_container(), item, pos, client_data)
        }
    }
    /// Inserts item into the control.
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a3ecaaa4bc63315056ff7fb4894866b9e).
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
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#abd876bdf2d3b5aaf577808c0520f78e4).
    fn insert_arraystring<A: ArrayStringMethods>(&self, items: &A, pos: c_uint) -> c_int {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Insert3(self.as_item_container(), items, pos)
        }
    }
    // BLOCKED: fn Insert4()
    /// Inserts several items at once into the control.
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#aa5b5f6059ef10221c521da9587269192).
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
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a9b15c918cc24743b9af994a48ee0cad6).
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
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a3faa9e4006d69dffd0ecb71088beb24c).
    fn insert_uint(&self, n: c_uint, items: *const c_void, pos: c_uint) -> c_int {
        unsafe { ffi::wxItemContainer_Insert7(self.as_item_container(), n, items, pos) }
    }
    /// Inserts several items at once into the control.
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a109dfe5f94135d8d16445ee72834d1b9).
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
    ///
    /// See [C++ `wxItemContainer::Insert()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#ad2bc86a398c44f6ab6d3d6f64d946c70).
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
    ///
    /// See [C++ `wxItemContainer::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a0758ff813749b9dfe4d8c4975778f40d).
    fn set_arraystring<A: ArrayStringMethods>(&self, items: &A) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set(self.as_item_container(), items)
        }
    }
    // BLOCKED: fn Set1()
    /// Replaces the current control contents with the given items.
    ///
    /// See [C++ `wxItemContainer::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a913f1901456fc75a45558775f967a2ce).
    fn set_arraystring_void<A: ArrayStringMethods>(&self, items: &A, client_data: *mut c_void) {
        unsafe {
            let items = items.as_ptr();
            ffi::wxItemContainer_Set2(self.as_item_container(), items, client_data)
        }
    }
    /// Replaces the current control contents with the given items.
    ///
    /// See [C++ `wxItemContainer::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#aa49e05430329dc56a15b2f75fc7cd05c).
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
    ///
    /// See [C++ `wxItemContainer::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#af5759464c9fd1396ed9b86a9d1242d30).
    fn set_uint(&self, n: c_uint, items: *const c_void) {
        unsafe { ffi::wxItemContainer_Set4(self.as_item_container(), n, items) }
    }
    /// Replaces the current control contents with the given items.
    ///
    /// See [C++ `wxItemContainer::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#aac979e6201e45b121f226389e29fd68a).
    fn set_uint_void(&self, n: c_uint, items: *const c_void, client_data: *mut c_void) {
        unsafe { ffi::wxItemContainer_Set5(self.as_item_container(), n, items, client_data) }
    }
    /// Replaces the current control contents with the given items.
    ///
    /// See [C++ `wxItemContainer::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html#a3a1dab2066917e84f46c57b27fb4f258).
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
/// This trait represents [C++ `wxItemContainerImmutable` class](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html)'s methods and inheritance.
///
/// See [`ItemContainerImmutableIsOwned`] documentation for the class usage.
pub trait ItemContainerImmutableMethods: WxRustMethods {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { self.as_ptr() }
    }
    /// Sets the selection to the given item n or removes the selection entirely if n == wxNOT_FOUND.
    ///
    /// See [C++ `wxItemContainerImmutable::SetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#aea97d8ee51e7af5e06befd5f8c9d793b).
    fn set_selection(&self, n: c_int) {
        unsafe { ffi::wxItemContainerImmutable_SetSelection(self.as_item_container_immutable(), n) }
    }
    /// Returns the index of the selected item or wxNOT_FOUND if no item is selected.
    ///
    /// See [C++ `wxItemContainerImmutable::GetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a18a7cb1a652772d5cb8adc52be1efea0).
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxItemContainerImmutable_GetSelection(self.as_item_container_immutable()) }
    }
    /// Selects the item with the specified string in the control.
    ///
    /// See [C++ `wxItemContainerImmutable::SetStringSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#afa3800ff87f00a47211c3ce206e7bc39).
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
    ///
    /// See [C++ `wxItemContainerImmutable::GetStringSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a32b63731332d90d7107ebfd949512ae5).
    fn get_string_selection(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxItemContainerImmutable_GetStringSelection(
                self.as_item_container_immutable(),
            ))
            .into()
        }
    }
    /// This is the same as SetSelection() and exists only because it is slightly more natural for controls which support multiple selection.
    ///
    /// See [C++ `wxItemContainerImmutable::Select()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#abcc0d37a2a7f29d5c54cfa5252571d61).
    fn select(&self, n: c_int) {
        unsafe { ffi::wxItemContainerImmutable_Select(self.as_item_container_immutable(), n) }
    }
    /// Returns the number of items in the control.
    ///
    /// See [C++ `wxItemContainerImmutable::GetCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a173bdb9c72977d524f524fcd02521f61).
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxItemContainerImmutable_GetCount(self.as_item_container_immutable()) }
    }
    /// Returns true if the control is empty or false if it has some items.
    ///
    /// See [C++ `wxItemContainerImmutable::IsEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a8f863e568895205a1179c9dc835573d6).
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxItemContainerImmutable_IsEmpty(self.as_item_container_immutable()) }
    }
    /// Returns the label of the item with the given index.
    ///
    /// See [C++ `wxItemContainerImmutable::GetString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a30e9fe62bd51415d9af2a9c6f19ec8f7).
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
    ///
    /// See [C++ `wxItemContainerImmutable::GetStrings()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#ac6792c056bf3748f64419960d307d7c0).
    fn get_strings(&self) -> ArrayString {
        unsafe {
            ArrayString::from_ptr(ffi::wxItemContainerImmutable_GetStrings(
                self.as_item_container_immutable(),
            ))
        }
    }
    /// Sets the label for the given item.
    ///
    /// See [C++ `wxItemContainerImmutable::SetString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a1daf356c330bac2d7a93c5b3de8fbabf).
    fn set_string(&self, n: c_uint, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxItemContainerImmutable_SetString(self.as_item_container_immutable(), n, string)
        }
    }
    /// Finds an item whose label matches the given string.
    ///
    /// See [C++ `wxItemContainerImmutable::FindString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html#a78519e3ea6a5fb47c27b337e21c99989).
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
