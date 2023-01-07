use super::*;

// wxBannerWindow
wxwidgets! {
    /// A simple banner window showing either a bitmap or text.
    /// - [`BannerWindow`] represents a C++ `wxBannerWindow` class instance which your code has ownership, [`BannerWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`BannerWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBannerWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html) for more details.
    #[doc(alias = "wxBannerWindow")]
    #[doc(alias = "BannerWindow")]
    class BannerWindow
        = BannerWindowInRust<true>(wxBannerWindow) impl
        BannerWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BannerWindowInRust<OWNED> {
    /// Default constructor, use Create() later.
    ///
    /// See [C++ `wxBannerWindow::wxBannerWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html#a29260797c824b361163f18519994c2f7).
    pub fn new_2step() -> BannerWindowInRust<OWNED> {
        unsafe { BannerWindowInRust(ffi::wxBannerWindow_new()) }
    }
    // BLOCKED: fn wxBannerWindow1()
    /// Full constructor provided for consistency with the other classes only.
    ///
    /// See [C++ `wxBannerWindow::wxBannerWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_banner_window.html#acbff11dfffdae9f0421d782f6adabb63).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        dir: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> BannerWindowInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            BannerWindowInRust(ffi::wxBannerWindow_new2(
                parent, winid, dir, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BannerWindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BannerWindowInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: BannerWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BannerWindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: BannerWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BannerWindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BannerWindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BannerWindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBannerWindow_CLASSINFO()) }
    }
}

// wxBitmap
wxwidgets! {
    /// This class encapsulates the concept of a platform-dependent bitmap, either monochrome or colour or colour with alpha channel support.
    /// - [`Bitmap`] represents a C++ `wxBitmap` class instance which your code has ownership, [`BitmapInRust`]`<false>` represents one which don't own.
    /// - Use [`Bitmap`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmap` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html) for more details.
    #[doc(alias = "wxBitmap")]
    #[doc(alias = "Bitmap")]
    class Bitmap
        = BitmapInRust<true>(wxBitmap) impl
        BitmapMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a5b4b1b408108b78cdbfd325b03c903b7).
    pub fn new() -> BitmapInRust<OWNED> {
        unsafe { BitmapInRust(ffi::wxBitmap_new()) }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#abfaa21ec563a64ea913af918150db900).
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapInRust(ffi::wxBitmap_new1(bitmap))
        }
    }
    // NOT_SUPPORTED: fn wxBitmap2()
    /// Creates a new bitmap.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a3bb19e8d368d6565f52a9c1294d80d7a).
    pub fn new_with_int_int(width: c_int, height: c_int, depth: c_int) -> BitmapInRust<OWNED> {
        unsafe { BitmapInRust(ffi::wxBitmap_new3(width, height, depth)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a2a73d89860df03b474086a7db694527d).
    pub fn new_with_size<S: SizeMethods>(sz: &S, depth: c_int) -> BitmapInRust<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            BitmapInRust(ffi::wxBitmap_new4(sz, depth))
        }
    }
    /// Create a bitmap compatible with the given DC, inheriting its magnification factor.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#adae885a8175635a732e76c575683efd0).
    pub fn new_with_int_dc<D: DCMethods>(
        width: c_int,
        height: c_int,
        dc: &D,
    ) -> BitmapInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            BitmapInRust(ffi::wxBitmap_new5(width, height, dc))
        }
    }
    /// Creates a bitmap from XPM data.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a0b750963aa91e021dfa222138d1678ed).
    pub fn new_with_char(bits: *const c_void) -> BitmapInRust<OWNED> {
        unsafe { BitmapInRust(ffi::wxBitmap_new6(bits)) }
    }
    // NOT_SUPPORTED: fn wxBitmap7()
    /// Creates this bitmap object from the given image.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a9855ffc55043187e4cff075aeefbaaf8).
    pub fn new_with_image_int<I: ImageMethods>(img: &I, depth: c_int) -> BitmapInRust<OWNED> {
        unsafe {
            let img = img.as_ptr();
            BitmapInRust(ffi::wxBitmap_new8(img, depth))
        }
    }
    /// Creates a bitmap compatible with the given DC from the given image.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#ae1200406e701b6d859b14c6bc4c34b31).
    pub fn new_with_image_dc<I: ImageMethods, D: DCMethods>(
        img: &I,
        dc: &D,
    ) -> BitmapInRust<OWNED> {
        unsafe {
            let img = img.as_ptr();
            let dc = dc.as_ptr();
            BitmapInRust(ffi::wxBitmap_new9(img, dc))
        }
    }
    /// Creates bitmap corresponding to the given cursor.
    ///
    /// See [C++ `wxBitmap::wxBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap.html#a6cecda7f133bce6c6fe42394bcfd0f4a).
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> BitmapInRust<OWNED> {
        unsafe {
            let cursor = cursor.as_ptr();
            BitmapInRust(ffi::wxBitmap_new10(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapInRust<OWNED>> for GDIObjectInRust<OWNED> {
    fn from(o: BitmapInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BitmapInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBitmap_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BitmapInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapBundle
wxwidgets! {
    /// Contains representations of the same bitmap in different resolutions.
    /// - [`BitmapBundle`] represents a C++ `wxBitmapBundle` class instance which your code has ownership, [`BitmapBundleInRust`]`<false>` represents one which don't own.
    /// - Use [`BitmapBundle`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmapBundle` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html) for more details.
    #[doc(alias = "wxBitmapBundle")]
    #[doc(alias = "BitmapBundle")]
    class BitmapBundle
        = BitmapBundleInRust<true>(wxBitmapBundle) impl
        BitmapBundleMethods
}
impl<const OWNED: bool> BitmapBundleInRust<OWNED> {
    /// Default constructor constructs an empty bundle.
    ///
    /// See [C++ `wxBitmapBundle::wxBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a147abda71276821e3957be38e9b9baa4).
    pub fn new() -> BitmapBundleInRust<OWNED> {
        unsafe { BitmapBundleInRust(ffi::wxBitmapBundle_new()) }
    }
    /// Conversion constructor from a single bitmap.
    ///
    /// See [C++ `wxBitmapBundle::wxBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a922b10aa1d1127d38a169fc2281a2e03).
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundleInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundleInRust(ffi::wxBitmapBundle_new1(bitmap))
        }
    }
    /// Conversion constructor from a single icon.
    ///
    /// See [C++ `wxBitmapBundle::wxBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a26700eb799253754c332308ee469ad11).
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> BitmapBundleInRust<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            BitmapBundleInRust(ffi::wxBitmapBundle_new2(icon))
        }
    }
    /// Conversion constructor from a single image.
    ///
    /// See [C++ `wxBitmapBundle::wxBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a5e5fa07df8d5e5d60b107f5949f5d12d).
    pub fn new_with_image<I: ImageMethods>(image: &I) -> BitmapBundleInRust<OWNED> {
        unsafe {
            let image = image.as_ptr();
            BitmapBundleInRust(ffi::wxBitmapBundle_new3(image))
        }
    }
    /// Conversion constructor from XPM data.
    ///
    /// See [C++ `wxBitmapBundle::wxBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#aeb9d813e1163b586497c2a86ee7eb2b0).
    pub fn new_with_char(xpm: *const c_void) -> BitmapBundleInRust<OWNED> {
        unsafe { BitmapBundleInRust(ffi::wxBitmapBundle_new4(xpm)) }
    }
    /// Copy constructor creates a copy of another bundle.
    ///
    /// See [C++ `wxBitmapBundle::wxBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_bundle.html#a11d8e7869e602087cc5552b34e4b6230).
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(other: &B) -> BitmapBundleInRust<OWNED> {
        unsafe {
            let other = other.as_ptr();
            BitmapBundleInRust(ffi::wxBitmapBundle_new5(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapBundleInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BitmapBundleInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapBundle_delete(self.0) }
        }
    }
}

// wxBitmapButton
wxwidgets! {
    /// A bitmap button is a control that contains a bitmap.
    /// - [`BitmapButton`] represents a C++ `wxBitmapButton` class instance which your code has ownership, [`BitmapButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`BitmapButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmapButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html) for more details.
    #[doc(alias = "wxBitmapButton")]
    #[doc(alias = "BitmapButton")]
    class BitmapButton
        = BitmapButtonInRust<true>(wxBitmapButton) impl
        BitmapButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapButtonInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxBitmapButton::wxBitmapButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html#af262b3d84985b3724a4e80c46a2b9f5e).
    pub fn new_2step() -> BitmapButtonInRust<OWNED> {
        unsafe { BitmapButtonInRust(ffi::wxBitmapButton_new()) }
    }
    /// Constructor, creating and showing a button.
    ///
    /// See [C++ `wxBitmapButton::wxBitmapButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_button.html#a840e53165d8559eff2263f9908facbfc).
    pub fn new<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        bitmap: &B,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> BitmapButtonInRust<OWNED> {
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
            BitmapButtonInRust(ffi::wxBitmapButton_new1(
                parent, id, bitmap, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BitmapButtonInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapButtonInRust<OWNED>> for ButtonInRust<OWNED> {
    fn from(o: BitmapButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonInRust<OWNED>> for AnyButtonInRust<OWNED> {
    fn from(o: BitmapButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: BitmapButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: BitmapButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: BitmapButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BitmapButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapButtonInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBitmapButton_CLASSINFO()) }
    }
}

// wxBitmapComboBox
wxwidgets! {
    /// A combobox that displays bitmap in front of the list items.
    /// - [`BitmapComboBox`] represents a C++ `wxBitmapComboBox` class instance which your code has ownership, [`BitmapComboBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`BitmapComboBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmapComboBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html) for more details.
    #[doc(alias = "wxBitmapComboBox")]
    #[doc(alias = "BitmapComboBox")]
    class BitmapComboBox
        = BitmapComboBoxInRust<true>(wxBitmapComboBox) impl
        BitmapComboBoxMethods,
        // ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapComboBoxInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxBitmapComboBox::wxBitmapComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#ad1c50828cc279364613f2726a57b524b).
    pub fn new_2step() -> BitmapComboBoxInRust<OWNED> {
        unsafe { BitmapComboBoxInRust(ffi::wxBitmapComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxBitmapComboBox1()
    /// Constructor, creating and showing a combobox.
    ///
    /// See [C++ `wxBitmapComboBox::wxBitmapComboBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a13bdf9ae92e496b4af850ece9e466b33).
    pub fn new<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> BitmapComboBoxInRust<OWNED> {
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
            BitmapComboBoxInRust(ffi::wxBitmapComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BitmapComboBoxInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapComboBoxInRust<OWNED>> for ComboBoxInRust<OWNED> {
    fn from(o: BitmapComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: BitmapComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: BitmapComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: BitmapComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BitmapComboBoxInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapComboBoxInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBitmapComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBitmapComboBox
impl<const OWNED: bool> ItemContainerMethods for BitmapComboBoxInRust<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for BitmapComboBoxInRust<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for BitmapComboBoxInRust<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ComboBoxMethods for BitmapComboBoxInRust<OWNED> {
    // NOT_SUPPORTED: fn Create()
    /// Creates the combobox for two-step construction.
    ///
    /// See [C++ `wxBitmapComboBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_combo_box.html#a4f269b59837fc3e312bb3e929caef6b3).
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
            ffi::wxBitmapComboBox_Create1(
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
}

// wxBitmapDataObject
wxwidgets! {
    /// wxBitmapDataObject is a specialization of wxDataObject for bitmap data.
    /// - [`BitmapDataObject`] represents a C++ `wxBitmapDataObject` class instance which your code has ownership, [`BitmapDataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`BitmapDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmapDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_data_object.html) for more details.
    #[doc(alias = "wxBitmapDataObject")]
    #[doc(alias = "BitmapDataObject")]
    class BitmapDataObject
        = BitmapDataObjectInRust<true>(wxBitmapDataObject) impl
        BitmapDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> BitmapDataObjectInRust<OWNED> {
    /// Constructor, optionally passing a bitmap (otherwise use SetBitmap() later).
    ///
    /// See [C++ `wxBitmapDataObject::wxBitmapDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_data_object.html#a6b4f3a28654382a6ea75d5db261e524b).
    pub fn new<B: BitmapMethods>(bitmap: &B) -> BitmapDataObjectInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapDataObjectInRust(ffi::wxBitmapDataObject_new(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapDataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapDataObjectInRust<OWNED>> for DataObjectSimpleInRust<OWNED> {
    fn from(o: BitmapDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapDataObjectInRust<OWNED>> for DataObjectInRust<OWNED> {
    fn from(o: BitmapDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for BitmapDataObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapDataObject_delete(self.0) }
        }
    }
}

// wxBitmapHandler
wxwidgets! {
    /// This is the base class for implementing bitmap file loading/saving, and bitmap creation from data.
    /// - [`BitmapHandler`] represents a C++ `wxBitmapHandler` class instance which your code has ownership, [`BitmapHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`BitmapHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmapHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html) for more details.
    #[doc(alias = "wxBitmapHandler")]
    #[doc(alias = "BitmapHandler")]
    class BitmapHandler
        = BitmapHandlerInRust<true>(wxBitmapHandler) impl
        BitmapHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapHandlerInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxBitmapHandler::wxBitmapHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_handler.html#ad8a16baff5f93057f4a96fc5fa26dfab).
    pub fn new() -> BitmapHandlerInRust<OWNED> {
        unsafe { BitmapHandlerInRust(ffi::wxBitmapHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapHandlerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BitmapHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapHandlerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBitmapHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BitmapHandlerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapToggleButton
wxwidgets! {
    /// wxBitmapToggleButton is a wxToggleButton that contains a bitmap instead of text.
    /// - [`BitmapToggleButton`] represents a C++ `wxBitmapToggleButton` class instance which your code has ownership, [`BitmapToggleButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`BitmapToggleButton`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBitmapToggleButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_toggle_button.html) for more details.
    #[doc(alias = "wxBitmapToggleButton")]
    #[doc(alias = "BitmapToggleButton")]
    class BitmapToggleButton
        = BitmapToggleButtonInRust<true>(wxBitmapToggleButton) impl
        BitmapToggleButtonMethods,
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapToggleButtonInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxBitmapToggleButton::wxBitmapToggleButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_toggle_button.html#aa5c5981a61bd3458a55266bd39bf5d7a).
    pub fn new_2step() -> BitmapToggleButtonInRust<OWNED> {
        unsafe { BitmapToggleButtonInRust(ffi::wxBitmapToggleButton_new()) }
    }
    /// Constructor, creating and showing a toggle button with the bitmap label.
    ///
    /// See [C++ `wxBitmapToggleButton::wxBitmapToggleButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_bitmap_toggle_button.html#ae243a6a04a7bf5f82b6026e6cfb1f02c).
    pub fn new<
        W: WindowMethods,
        B: BitmapBundleMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        val: &V,
        name: &str,
    ) -> BitmapToggleButtonInRust<OWNED> {
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
            BitmapToggleButtonInRust(ffi::wxBitmapToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BitmapToggleButtonInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonInRust<OWNED>> for ToggleButtonInRust<OWNED> {
    fn from(o: BitmapToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonInRust<OWNED>> for AnyButtonInRust<OWNED> {
    fn from(o: BitmapToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: BitmapToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: BitmapToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: BitmapToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BitmapToggleButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapToggleButtonInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBitmapToggleButton_CLASSINFO()) }
    }
}

// wxBookCtrlBase
wxwidgets! {
    /// A book control is a convenient way of displaying multiple pages of information, displayed one page at a time.
    /// - [`BookCtrlBase`] represents a C++ `wxBookCtrlBase` class instance which your code has ownership, [`BookCtrlBaseInRust`]`<false>` represents one which don't own.
    /// - Use [`BookCtrlBase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBookCtrlBase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html) for more details.
    #[doc(alias = "wxBookCtrlBase")]
    #[doc(alias = "BookCtrlBase")]
    class BookCtrlBase
        = BookCtrlBaseInRust<true>(wxBookCtrlBase) impl
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlBaseInRust<OWNED> {
    //  ENUM: @3
    pub const NO_IMAGE: c_int = -1;

    // BLOCKED: fn wxBookCtrlBase()
    // BLOCKED: fn wxBookCtrlBase1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BookCtrlBaseInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BookCtrlBaseInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: BookCtrlBaseInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: BookCtrlBaseInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: BookCtrlBaseInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BookCtrlBaseInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BookCtrlBaseInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBookCtrlBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for BookCtrlBaseInRust<OWNED> {
    /// Constructs the book control with the given parameters.
    ///
    /// See [C++ `wxBookCtrlBase::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_base.html#ad61f7fc3c1fbccbcb119e1dea3fdc4a6).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxBookCtrlBase_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxBookCtrlEvent
wxwidgets! {
    /// This class represents the events generated by book controls (wxNotebook, wxListbook, wxChoicebook, wxTreebook, wxAuiNotebook).
    /// - [`BookCtrlEvent`] represents a C++ `wxBookCtrlEvent` class instance which your code has ownership, [`BookCtrlEventInRust`]`<false>` represents one which don't own.
    /// - Use [`BookCtrlEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBookCtrlEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_event.html) for more details.
    #[doc(alias = "wxBookCtrlEvent")]
    #[doc(alias = "BookCtrlEvent")]
    class BookCtrlEvent
        = BookCtrlEventInRust<true>(wxBookCtrlEvent) impl
        BookCtrlEventMethods,
        NotifyEventMethods,
        // CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxBookCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BookCtrlEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BookCtrlEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: BookCtrlEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: BookCtrlEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: BookCtrlEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BookCtrlEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BookCtrlEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBookCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BookCtrlEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> CommandEventMethods for BookCtrlEventInRust<OWNED> {
    /// Returns the currently selected page, or wxNOT_FOUND if none was selected.
    ///
    /// See [C++ `wxBookCtrlEvent::GetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_book_ctrl_event.html#acfc78f1292a2e229650cd1a2e2aaf937).
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetSelection(self.as_ptr()) }
    }
}

// wxBoxSizer
wxwidgets! {
    /// The basic idea behind a box sizer is that windows will most often be laid out in rather simple basic geometry, typically in a row or a column or several hierarchies of either.
    /// - [`BoxSizer`] represents a C++ `wxBoxSizer` class instance which your code has ownership, [`BoxSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`BoxSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBoxSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_box_sizer.html) for more details.
    #[doc(alias = "wxBoxSizer")]
    #[doc(alias = "BoxSizer")]
    class BoxSizer
        = BoxSizerInRust<true>(wxBoxSizer) impl
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BoxSizerInRust<OWNED> {
    /// Constructor for a wxBoxSizer.
    ///
    /// See [C++ `wxBoxSizer::wxBoxSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_box_sizer.html#a1d8b1a1bed1af566c1f95e54dbf1d18e).
    pub fn new(orient: c_int) -> BoxSizerInRust<OWNED> {
        unsafe { BoxSizerInRust(ffi::wxBoxSizer_new(orient)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BoxSizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BoxSizerInRust<OWNED>> for SizerInRust<OWNED> {
    fn from(o: BoxSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BoxSizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BoxSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BoxSizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBoxSizer_CLASSINFO()) }
    }
}

// wxBrush
wxwidgets! {
    /// A brush is a drawing tool for filling in areas.
    /// - [`Brush`] represents a C++ `wxBrush` class instance which your code has ownership, [`BrushInRust`]`<false>` represents one which don't own.
    /// - Use [`Brush`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBrush` class's documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html) for more details.
    #[doc(alias = "wxBrush")]
    #[doc(alias = "Brush")]
    class Brush
        = BrushInRust<true>(wxBrush) impl
        BrushMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BrushInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxBrush::wxBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a36d9d097cd95d038427907d2aa0fd2ad).
    pub fn new() -> BrushInRust<OWNED> {
        unsafe { BrushInRust(ffi::wxBrush_new()) }
    }
    // NOT_SUPPORTED: fn wxBrush1()
    /// Constructs a stippled brush using a bitmap.
    ///
    /// See [C++ `wxBrush::wxBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a0a5c0e09b4637f7749435fbe1acd6412).
    pub fn new_with_bitmap<B: BitmapMethods>(stipple_bitmap: &B) -> BrushInRust<OWNED> {
        unsafe {
            let stipple_bitmap = stipple_bitmap.as_ptr();
            BrushInRust(ffi::wxBrush_new2(stipple_bitmap))
        }
    }
    /// Copy constructor, uses reference counting.
    ///
    /// See [C++ `wxBrush::wxBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_brush.html#a6df3fbfc890e8af31fd6bbf2ca38c5e7).
    pub fn new_with_brush<B: BrushMethods>(brush: &B) -> BrushInRust<OWNED> {
        unsafe {
            let brush = brush.as_ptr();
            BrushInRust(ffi::wxBrush_new3(brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BrushInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BrushInRust<OWNED>> for GDIObjectInRust<OWNED> {
    fn from(o: BrushInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BrushInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BrushInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BrushInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBrush_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BrushInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBrushList
wxwidgets! {
    /// A brush list is a list containing all brushes which have been created.
    /// - [`BrushList`] represents a C++ `wxBrushList` class instance which your code has ownership, [`BrushListInRust`]`<false>` represents one which don't own.
    /// - Use [`BrushList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBrushList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_brush_list.html) for more details.
    #[doc(alias = "wxBrushList")]
    #[doc(alias = "BrushList")]
    class BrushList
        = BrushListInRust<true>(wxBrushList) impl
        BrushListMethods
}
impl<const OWNED: bool> BrushListInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BrushListInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BrushListInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBrushList_delete(self.0) }
        }
    }
}

// wxBufferedDC
wxwidgets! {
    /// This class provides a simple way to avoid flicker: when drawing on it, everything is in fact first drawn on an in-memory buffer (a wxBitmap) and then copied to the screen, using the associated wxDC, only once, when this object is destroyed.
    /// - [`BufferedDC`] represents a C++ `wxBufferedDC` class instance which your code has ownership, [`BufferedDCInRust`]`<false>` represents one which don't own.
    /// - Use [`BufferedDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBufferedDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html) for more details.
    #[doc(alias = "wxBufferedDC")]
    #[doc(alias = "BufferedDC")]
    class BufferedDC
        = BufferedDCInRust<true>(wxBufferedDC) impl
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> BufferedDCInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxBufferedDC::wxBufferedDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#adf8f81fe169fb1d3f0e5c9bdbdb1cb2a).
    pub fn new() -> BufferedDCInRust<OWNED> {
        unsafe { BufferedDCInRust(ffi::wxBufferedDC_new()) }
    }
    /// Creates a buffer for the provided dc.
    ///
    /// See [C++ `wxBufferedDC::wxBufferedDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#ae99271390103ee94640a34b19dd4fc05).
    pub fn new_with_dc_size<D: DCMethods, S: SizeMethods>(
        dc: Option<&D>,
        area: &S,
        style: c_int,
    ) -> BufferedDCInRust<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let area = area.as_ptr();
            BufferedDCInRust(ffi::wxBufferedDC_new1(dc, area, style))
        }
    }
    /// Creates a buffer for the provided dc.
    ///
    /// See [C++ `wxBufferedDC::wxBufferedDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_d_c.html#ab48701a98774ef960b750f872fcdbdd1).
    pub fn new_with_dc_bitmap<D: DCMethods, B: BitmapMethods>(
        dc: Option<&D>,
        buffer: &B,
        style: c_int,
    ) -> BufferedDCInRust<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            BufferedDCInRust(ffi::wxBufferedDC_new2(dc, buffer, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BufferedDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BufferedDCInRust<OWNED>> for MemoryDCInRust<OWNED> {
    fn from(o: BufferedDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: BufferedDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BufferedDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BufferedDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBufferedDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BufferedDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBufferedPaintDC
wxwidgets! {
    /// This is a subclass of wxBufferedDC which can be used inside of an EVT_PAINT() event handler to achieve double-buffered drawing.
    /// - [`BufferedPaintDC`] represents a C++ `wxBufferedPaintDC` class instance which your code has ownership, [`BufferedPaintDCInRust`]`<false>` represents one which don't own.
    /// - Use [`BufferedPaintDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBufferedPaintDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_paint_d_c.html) for more details.
    #[doc(alias = "wxBufferedPaintDC")]
    #[doc(alias = "BufferedPaintDC")]
    class BufferedPaintDC
        = BufferedPaintDCInRust<true>(wxBufferedPaintDC) impl
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> BufferedPaintDCInRust<OWNED> {
    /// As with wxBufferedDC, you may either provide the bitmap to be used for buffering or let this object create one internally (in the latter case, the size of the client part of the window is used).
    ///
    /// See [C++ `wxBufferedPaintDC::wxBufferedPaintDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_paint_d_c.html#a176c15663752a760498faec72c370943).
    pub fn new_with_bitmap<W: WindowMethods, B: BitmapMethods>(
        window: Option<&W>,
        buffer: &B,
        style: c_int,
    ) -> BufferedPaintDCInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            BufferedPaintDCInRust(ffi::wxBufferedPaintDC_new(window, buffer, style))
        }
    }
    ///
    /// See [C++ `wxBufferedPaintDC::wxBufferedPaintDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_buffered_paint_d_c.html#a1fb99ae9afce963583629117a9da67b1).
    pub fn new_with_int<W: WindowMethods>(
        window: Option<&W>,
        style: c_int,
    ) -> BufferedPaintDCInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BufferedPaintDCInRust(ffi::wxBufferedPaintDC_new1(window, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BufferedPaintDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BufferedPaintDCInRust<OWNED>> for BufferedDCInRust<OWNED> {
    fn from(o: BufferedPaintDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCInRust<OWNED>> for MemoryDCInRust<OWNED> {
    fn from(o: BufferedPaintDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: BufferedPaintDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: BufferedPaintDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BufferedPaintDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxBufferedPaintDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BufferedPaintDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBusyCursor
wxwidgets! {
    /// This class makes it easy to tell your user that the program is temporarily busy.
    /// - [`BusyCursor`] represents a C++ `wxBusyCursor` class instance which your code has ownership, [`BusyCursorInRust`]`<false>` represents one which don't own.
    /// - Use [`BusyCursor`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBusyCursor` class's documentation](https://docs.wxwidgets.org/3.2/classwx_busy_cursor.html) for more details.
    #[doc(alias = "wxBusyCursor")]
    #[doc(alias = "BusyCursor")]
    class BusyCursor
        = BusyCursorInRust<true>(wxBusyCursor) impl
        BusyCursorMethods
}
impl<const OWNED: bool> BusyCursorInRust<OWNED> {
    /// Constructs a busy cursor object, calling wxBeginBusyCursor().
    ///
    /// See [C++ `wxBusyCursor::wxBusyCursor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_busy_cursor.html#aeaa61938f7322311eb43bfa1ef4ea205).
    pub fn new<C: CursorMethods>(cursor: Option<&C>) -> BusyCursorInRust<OWNED> {
        unsafe {
            let cursor = match cursor {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BusyCursorInRust(ffi::wxBusyCursor_new(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BusyCursorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BusyCursorInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBusyCursor_delete(self.0) }
        }
    }
}

// wxBusyInfo
wxwidgets! {
    /// This class makes it easy to tell your user that the program is temporarily busy.
    /// - [`BusyInfo`] represents a C++ `wxBusyInfo` class instance which your code has ownership, [`BusyInfoInRust`]`<false>` represents one which don't own.
    /// - Use [`BusyInfo`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxBusyInfo` class's documentation](https://docs.wxwidgets.org/3.2/classwx_busy_info.html) for more details.
    #[doc(alias = "wxBusyInfo")]
    #[doc(alias = "BusyInfo")]
    class BusyInfo
        = BusyInfoInRust<true>(wxBusyInfo) impl
        BusyInfoMethods
}
impl<const OWNED: bool> BusyInfoInRust<OWNED> {
    /// General constructor.
    ///
    /// See [C++ `wxBusyInfo::wxBusyInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_busy_info.html#ab5e29cf81518600a855de4ad4619a2a0).
    pub fn new_with_busyinfoflags(flags: *const c_void) -> BusyInfoInRust<OWNED> {
        unsafe { BusyInfoInRust(ffi::wxBusyInfo_new(flags)) }
    }
    /// Simple constructor specifying only the message and the parent.
    ///
    /// See [C++ `wxBusyInfo::wxBusyInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_busy_info.html#aa53bff52ae401bab878169e7d04b86d3).
    pub fn new_with_str<W: WindowMethods>(msg: &str, parent: Option<&W>) -> BusyInfoInRust<OWNED> {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BusyInfoInRust(ffi::wxBusyInfo_new1(msg, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BusyInfoInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BusyInfoInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBusyInfo_delete(self.0) }
        }
    }
}

// wxButton
wxwidgets! {
    /// A button is a control that contains a text string, and is one of the most common elements of a GUI.
    /// - [`Button`] represents a C++ `wxButton` class instance which your code has ownership, [`ButtonInRust`]`<false>` represents one which don't own.
    /// - Use [`Button`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxButton` class's documentation](https://docs.wxwidgets.org/3.2/classwx_button.html) for more details.
    #[doc(alias = "wxButton")]
    #[doc(alias = "Button")]
    class Button
        = ButtonInRust<true>(wxButton) impl
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ButtonInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxButton::wxButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#a61d2be397a8a673ebaf006003eb17b9e).
    pub fn new_2step() -> ButtonInRust<OWNED> {
        unsafe { ButtonInRust(ffi::wxButton_new()) }
    }
    /// Constructor, creating and showing a button.
    ///
    /// See [C++ `wxButton::wxButton()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_button.html#ab30ef360a5f2f48dd3ff537f70808962).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ButtonInRust<OWNED> {
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
            ButtonInRust(ffi::wxButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ButtonInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ButtonInRust<OWNED>> for AnyButtonInRust<OWNED> {
    fn from(o: ButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: ButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: ButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: ButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: ButtonInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ButtonInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxButton_CLASSINFO()) }
    }
}
