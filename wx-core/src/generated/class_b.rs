use super::*;

// wxBannerWindow
wxwidgets! {
    #[doc(alias = "wxBannerWindow")]
    #[doc(alias = "BannerWindow")]
    type BannerWindow = BannerWindowIsOwned<true>(wxBannerWindow) impl
        BannerWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BannerWindowIsOwned<OWNED> {
    pub fn new_2step() -> BannerWindowIsOwned<OWNED> {
        unsafe { BannerWindowIsOwned(ffi::wxBannerWindow_new()) }
    }
    // BLOCKED: fn wxBannerWindow1()
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        dir: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> BannerWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            BannerWindowIsOwned(ffi::wxBannerWindow_new2(
                parent, winid, dir, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BannerWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BannerWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BannerWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BannerWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BannerWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BannerWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BannerWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BannerWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBannerWindow_CLASSINFO()) }
    }
}

// wxBitmap
wxwidgets! {
    #[doc(alias = "wxBitmap")]
    #[doc(alias = "Bitmap")]
    type Bitmap = BitmapIsOwned<true>(wxBitmap) impl
        BitmapMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapIsOwned<OWNED> {
    pub fn new() -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new1(bitmap))
        }
    }
    // NOT_SUPPORTED: fn wxBitmap2()
    pub fn new_with_int_int(width: c_int, height: c_int, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new3(width, height, depth)) }
    }
    pub fn new_with_size<S: SizeMethods>(sz: &S, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new4(sz, depth))
        }
    }
    pub fn new_with_int_dc<D: DCMethods>(
        width: c_int,
        height: c_int,
        dc: &D,
    ) -> BitmapIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new5(width, height, dc))
        }
    }
    pub fn new_with_char(bits: *const c_void) -> BitmapIsOwned<OWNED> {
        unsafe { BitmapIsOwned(ffi::wxBitmap_new6(bits)) }
    }
    // NOT_SUPPORTED: fn wxBitmap7()
    pub fn new_with_image_int<I: ImageMethods>(img: &I, depth: c_int) -> BitmapIsOwned<OWNED> {
        unsafe {
            let img = img.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new8(img, depth))
        }
    }
    pub fn new_with_image_dc<I: ImageMethods, D: DCMethods>(
        img: &I,
        dc: &D,
    ) -> BitmapIsOwned<OWNED> {
        unsafe {
            let img = img.as_ptr();
            let dc = dc.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new9(img, dc))
        }
    }
    pub fn new_with_cursor<C: CursorMethods>(cursor: &C) -> BitmapIsOwned<OWNED> {
        unsafe {
            let cursor = cursor.as_ptr();
            BitmapIsOwned(ffi::wxBitmap_new10(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: BitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmap_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BitmapIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapBundle
wxwidgets! {
    #[doc(alias = "wxBitmapBundle")]
    #[doc(alias = "BitmapBundle")]
    type BitmapBundle = BitmapBundleIsOwned<true>(wxBitmapBundle) impl
        BitmapBundleMethods
}
impl<const OWNED: bool> BitmapBundleIsOwned<OWNED> {
    pub fn new() -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new()) }
    }
    pub fn new_with_bitmap<B: BitmapMethods>(bitmap: &B) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new1(bitmap))
        }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new2(icon))
        }
    }
    pub fn new_with_image<I: ImageMethods>(image: &I) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new3(image))
        }
    }
    pub fn new_with_char(xpm: *const c_void) -> BitmapBundleIsOwned<OWNED> {
        unsafe { BitmapBundleIsOwned(ffi::wxBitmapBundle_new4(xpm)) }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(other: &B) -> BitmapBundleIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            BitmapBundleIsOwned(ffi::wxBitmapBundle_new5(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapBundleIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BitmapBundleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapBundle_delete(self.0) }
        }
    }
}

// wxBitmapButton
wxwidgets! {
    #[doc(alias = "wxBitmapButton")]
    #[doc(alias = "BitmapButton")]
    type BitmapButton = BitmapButtonIsOwned<true>(wxBitmapButton) impl
        BitmapButtonMethods,
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapButtonIsOwned<OWNED> {
    pub fn new_2step() -> BitmapButtonIsOwned<OWNED> {
        unsafe { BitmapButtonIsOwned(ffi::wxBitmapButton_new()) }
    }
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
    ) -> BitmapButtonIsOwned<OWNED> {
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
            BitmapButtonIsOwned(ffi::wxBitmapButton_new1(
                parent, id, bitmap, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BitmapButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for ButtonIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapButton_CLASSINFO()) }
    }
}

// wxBitmapComboBox
wxwidgets! {
    #[doc(alias = "wxBitmapComboBox")]
    #[doc(alias = "BitmapComboBox")]
    type BitmapComboBox = BitmapComboBoxIsOwned<true>(wxBitmapComboBox) impl
        BitmapComboBoxMethods,
        // ComboBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapComboBoxIsOwned<OWNED> {
    pub fn new_2step() -> BitmapComboBoxIsOwned<OWNED> {
        unsafe { BitmapComboBoxIsOwned(ffi::wxBitmapComboBox_new()) }
    }
    // NOT_SUPPORTED: fn wxBitmapComboBox1()
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
    ) -> BitmapComboBoxIsOwned<OWNED> {
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
            BitmapComboBoxIsOwned(ffi::wxBitmapComboBox_new2(
                parent, id, value, pos, size, choices, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BitmapComboBoxIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for ComboBoxIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapComboBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapComboBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapComboBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapComboBox_CLASSINFO()) }
    }
}
// Mix-in(s) to wxBitmapComboBox
impl<const OWNED: bool> ItemContainerMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_item_container(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ItemContainerImmutableMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_item_container_immutable(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsItemContainer(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextEntryMethods for BitmapComboBoxIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxBitmapComboBox_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ComboBoxMethods for BitmapComboBoxIsOwned<OWNED> {
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
    #[doc(alias = "wxBitmapDataObject")]
    #[doc(alias = "BitmapDataObject")]
    type BitmapDataObject = BitmapDataObjectIsOwned<true>(wxBitmapDataObject) impl
        BitmapDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> BitmapDataObjectIsOwned<OWNED> {
    pub fn new<B: BitmapMethods>(bitmap: &B) -> BitmapDataObjectIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            BitmapDataObjectIsOwned(ffi::wxBitmapDataObject_new(bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapDataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: BitmapDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: BitmapDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for BitmapDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBitmapDataObject_delete(self.0) }
        }
    }
}

// wxBitmapHandler
wxwidgets! {
    #[doc(alias = "wxBitmapHandler")]
    #[doc(alias = "BitmapHandler")]
    type BitmapHandler = BitmapHandlerIsOwned<true>(wxBitmapHandler) impl
        BitmapHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapHandlerIsOwned<OWNED> {
    pub fn new() -> BitmapHandlerIsOwned<OWNED> {
        unsafe { BitmapHandlerIsOwned(ffi::wxBitmapHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BitmapHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BitmapHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBitmapToggleButton
wxwidgets! {
    #[doc(alias = "wxBitmapToggleButton")]
    #[doc(alias = "BitmapToggleButton")]
    type BitmapToggleButton = BitmapToggleButtonIsOwned<true>(wxBitmapToggleButton) impl
        BitmapToggleButtonMethods,
        ToggleButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BitmapToggleButtonIsOwned<OWNED> {
    pub fn new_2step() -> BitmapToggleButtonIsOwned<OWNED> {
        unsafe { BitmapToggleButtonIsOwned(ffi::wxBitmapToggleButton_new()) }
    }
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
    ) -> BitmapToggleButtonIsOwned<OWNED> {
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
            BitmapToggleButtonIsOwned(ffi::wxBitmapToggleButton_new1(
                parent, id, label, pos, size, style, val, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BitmapToggleButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for ToggleButtonIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BitmapToggleButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BitmapToggleButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BitmapToggleButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBitmapToggleButton_CLASSINFO()) }
    }
}

// wxBookCtrlBase
wxwidgets! {
    #[doc(alias = "wxBookCtrlBase")]
    #[doc(alias = "BookCtrlBase")]
    type BookCtrlBase = BookCtrlBaseIsOwned<true>(wxBookCtrlBase) impl
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlBaseIsOwned<OWNED> {
    //  ENUM: @3
    pub const NO_IMAGE: c_int = -1;

    // BLOCKED: fn wxBookCtrlBase()
    // BLOCKED: fn wxBookCtrlBase1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BookCtrlBaseIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BookCtrlBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BookCtrlBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBookCtrlBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for BookCtrlBaseIsOwned<OWNED> {
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
    #[doc(alias = "wxBookCtrlEvent")]
    #[doc(alias = "BookCtrlEvent")]
    type BookCtrlEvent = BookCtrlEventIsOwned<true>(wxBookCtrlEvent) impl
        BookCtrlEventMethods,
        NotifyEventMethods,
        // CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> BookCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxBookCtrlEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BookCtrlEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BookCtrlEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BookCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BookCtrlEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBookCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BookCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> CommandEventMethods for BookCtrlEventIsOwned<OWNED> {
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxBookCtrlEvent_GetSelection(self.as_ptr()) }
    }
}

// wxBoxSizer
wxwidgets! {
    #[doc(alias = "wxBoxSizer")]
    #[doc(alias = "BoxSizer")]
    type BoxSizer = BoxSizerIsOwned<true>(wxBoxSizer) impl
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> BoxSizerIsOwned<OWNED> {
    pub fn new(orient: c_int) -> BoxSizerIsOwned<OWNED> {
        unsafe { BoxSizerIsOwned(ffi::wxBoxSizer_new(orient)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for BoxSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BoxSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: BoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BoxSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BoxSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBoxSizer_CLASSINFO()) }
    }
}

// wxBrush
wxwidgets! {
    #[doc(alias = "wxBrush")]
    #[doc(alias = "Brush")]
    type Brush = BrushIsOwned<true>(wxBrush) impl
        BrushMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> BrushIsOwned<OWNED> {
    pub fn new() -> BrushIsOwned<OWNED> {
        unsafe { BrushIsOwned(ffi::wxBrush_new()) }
    }
    // NOT_SUPPORTED: fn wxBrush1()
    pub fn new_with_bitmap<B: BitmapMethods>(stipple_bitmap: &B) -> BrushIsOwned<OWNED> {
        unsafe {
            let stipple_bitmap = stipple_bitmap.as_ptr();
            BrushIsOwned(ffi::wxBrush_new2(stipple_bitmap))
        }
    }
    pub fn new_with_brush<B: BrushMethods>(brush: &B) -> BrushIsOwned<OWNED> {
        unsafe {
            let brush = brush.as_ptr();
            BrushIsOwned(ffi::wxBrush_new3(brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BrushIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BrushIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: BrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BrushIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BrushIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBrush_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BrushIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBrushList
wxwidgets! {
    #[doc(alias = "wxBrushList")]
    #[doc(alias = "BrushList")]
    type BrushList = BrushListIsOwned<true>(wxBrushList) impl
        BrushListMethods
}
impl<const OWNED: bool> BrushListIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BrushListIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BrushListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBrushList_delete(self.0) }
        }
    }
}

// wxBufferedDC
wxwidgets! {
    #[doc(alias = "wxBufferedDC")]
    #[doc(alias = "BufferedDC")]
    type BufferedDC = BufferedDCIsOwned<true>(wxBufferedDC) impl
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> BufferedDCIsOwned<OWNED> {
    pub fn new() -> BufferedDCIsOwned<OWNED> {
        unsafe { BufferedDCIsOwned(ffi::wxBufferedDC_new()) }
    }
    pub fn new_with_dc_size<D: DCMethods, S: SizeMethods>(
        dc: Option<&D>,
        area: &S,
        style: c_int,
    ) -> BufferedDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let area = area.as_ptr();
            BufferedDCIsOwned(ffi::wxBufferedDC_new1(dc, area, style))
        }
    }
    pub fn new_with_dc_bitmap<D: DCMethods, B: BitmapMethods>(
        dc: Option<&D>,
        buffer: &B,
        style: c_int,
    ) -> BufferedDCIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            BufferedDCIsOwned(ffi::wxBufferedDC_new2(dc, buffer, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BufferedDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BufferedDCIsOwned<OWNED>> for MemoryDCIsOwned<OWNED> {
    fn from(o: BufferedDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: BufferedDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BufferedDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BufferedDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBufferedDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BufferedDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBufferedPaintDC
wxwidgets! {
    #[doc(alias = "wxBufferedPaintDC")]
    #[doc(alias = "BufferedPaintDC")]
    type BufferedPaintDC = BufferedPaintDCIsOwned<true>(wxBufferedPaintDC) impl
        BufferedPaintDCMethods,
        BufferedDCMethods,
        MemoryDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> BufferedPaintDCIsOwned<OWNED> {
    pub fn new_with_bitmap<W: WindowMethods, B: BitmapMethods>(
        window: Option<&W>,
        buffer: &B,
        style: c_int,
    ) -> BufferedPaintDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let buffer = buffer.as_ptr();
            BufferedPaintDCIsOwned(ffi::wxBufferedPaintDC_new(window, buffer, style))
        }
    }
    pub fn new_with_int<W: WindowMethods>(
        window: Option<&W>,
        style: c_int,
    ) -> BufferedPaintDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BufferedPaintDCIsOwned(ffi::wxBufferedPaintDC_new1(window, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BufferedPaintDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for BufferedDCIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for MemoryDCIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<BufferedPaintDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: BufferedPaintDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for BufferedPaintDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxBufferedPaintDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for BufferedPaintDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxBusyCursor
wxwidgets! {
    #[doc(alias = "wxBusyCursor")]
    #[doc(alias = "BusyCursor")]
    type BusyCursor = BusyCursorIsOwned<true>(wxBusyCursor) impl
        BusyCursorMethods
}
impl<const OWNED: bool> BusyCursorIsOwned<OWNED> {
    pub fn new<C: CursorMethods>(cursor: Option<&C>) -> BusyCursorIsOwned<OWNED> {
        unsafe {
            let cursor = match cursor {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BusyCursorIsOwned(ffi::wxBusyCursor_new(cursor))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BusyCursorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BusyCursorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBusyCursor_delete(self.0) }
        }
    }
}

// wxBusyInfo
wxwidgets! {
    #[doc(alias = "wxBusyInfo")]
    #[doc(alias = "BusyInfo")]
    type BusyInfo = BusyInfoIsOwned<true>(wxBusyInfo) impl
        BusyInfoMethods
}
impl<const OWNED: bool> BusyInfoIsOwned<OWNED> {
    pub fn new_with_busyinfoflags(flags: *const c_void) -> BusyInfoIsOwned<OWNED> {
        unsafe { BusyInfoIsOwned(ffi::wxBusyInfo_new(flags)) }
    }
    pub fn new_with_str<W: WindowMethods>(msg: &str, parent: Option<&W>) -> BusyInfoIsOwned<OWNED> {
        unsafe {
            let msg = WxString::from(msg);
            let msg = msg.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            BusyInfoIsOwned(ffi::wxBusyInfo_new1(msg, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for BusyInfoIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for BusyInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxBusyInfo_delete(self.0) }
        }
    }
}

// wxButton
wxwidgets! {
    #[doc(alias = "wxButton")]
    #[doc(alias = "Button")]
    type Button = ButtonIsOwned<true>(wxButton) impl
        ButtonMethods,
        AnyButtonMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ButtonIsOwned<OWNED> {
    pub fn new_2step() -> ButtonIsOwned<OWNED> {
        unsafe { ButtonIsOwned(ffi::wxButton_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ButtonIsOwned<OWNED> {
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
            ButtonIsOwned(ffi::wxButton_new1(
                parent, id, label, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for AnyButtonIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxButton_CLASSINFO()) }
    }
}
