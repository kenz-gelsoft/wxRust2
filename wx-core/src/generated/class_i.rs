#![allow(non_upper_case_globals)]

use super::*;

// wxIFFHandler
wx_class! { IFFHandler =
    IFFHandlerIsOwned<true>(wxIFFHandler) impl
        IFFHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> IFFHandlerIsOwned<OWNED> {
    pub fn new() -> IFFHandlerIsOwned<OWNED> {
        unsafe { IFFHandlerIsOwned(ffi::wxIFFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IFFHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: IFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IFFHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IFFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IFFHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIFFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IFFHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIcon
wx_class! { Icon =
    IconIsOwned<true>(wxIcon) impl
        IconMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconIsOwned<OWNED> {
    pub fn new() -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new()) }
    }
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconIsOwned(ffi::wxIcon_new1(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIcon2()
    pub fn new_with_char(bits: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new3(bits)) }
    }
    // NOT_SUPPORTED: fn wxIcon4()
    pub fn new_with_iconlocation(loc: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new5(loc)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: IconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIcon_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconBundle
wx_class! { IconBundle =
    IconBundleIsOwned<true>(wxIconBundle) impl
        IconBundleMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconBundleIsOwned<OWNED> {
    //  ENUM: @29
    pub const FALLBACK_NONE: c_int = 0;
    pub const FALLBACK_SYSTEM: c_int = 1;
    pub const FALLBACK_NEAREST_LARGER: c_int = 2;

    pub fn new() -> IconBundleIsOwned<OWNED> {
        unsafe { IconBundleIsOwned(ffi::wxIconBundle_new()) }
    }
    // NOT_SUPPORTED: fn wxIconBundle1()
    // NOT_SUPPORTED: fn wxIconBundle2()
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconBundleIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconBundleIsOwned(ffi::wxIconBundle_new3(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIconBundle4()
    pub fn new_with_iconbundle<I: IconBundleMethods>(ic: &I) -> IconBundleIsOwned<OWNED> {
        unsafe {
            let ic = ic.as_ptr();
            IconBundleIsOwned(ffi::wxIconBundle_new5(ic))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconBundleIsOwned<OWNED>> for GDIObjectIsOwned<OWNED> {
    fn from(o: IconBundleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconBundleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconBundleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconBundleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIconBundle_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconBundleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconizeEvent
wx_class! { IconizeEvent =
    IconizeEventIsOwned<true>(wxIconizeEvent) impl
        IconizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconizeEventIsOwned<OWNED> {
    pub fn new(id: c_int, iconized: bool) -> IconizeEventIsOwned<OWNED> {
        unsafe { IconizeEventIsOwned(ffi::wxIconizeEvent_new(id, iconized)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IconizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: IconizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IconizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IconizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IconizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIconizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IconizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIdManager
wx_class! { IdManager =
    IdManagerIsOwned<true>(wxIdManager) impl
        IdManagerMethods
}
impl<const OWNED: bool> IdManagerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for IdManagerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxIdManager_delete(self.0) }
        }
    }
}

// wxImage
wx_class! { Image =
    ImageIsOwned<true>(wxImage) impl
        ImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageIsOwned<OWNED> {
    pub fn new() -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new()) }
    }
    pub fn new_with_int_bool(width: c_int, height: c_int, clear: bool) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new1(width, height, clear)) }
    }
    pub fn new_with_size_bool<S: SizeMethods>(sz: &S, clear: bool) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new2(sz, clear))
        }
    }
    pub fn new_with_int_uchar_bool(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new3(width, height, data, static_data)) }
    }
    pub fn new_with_size_uchar_bool<S: SizeMethods>(
        sz: &S,
        data: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new4(sz, data, static_data))
        }
    }
    pub fn new_with_int_uchar_uchar(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new5(width, height, data, alpha, static_data)) }
    }
    pub fn new_with_size_uchar_uchar<S: SizeMethods>(
        sz: &S,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new6(sz, data, alpha, static_data))
        }
    }
    pub fn new_with_char(xpm_data: *const c_void) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new7(xpm_data)) }
    }
    // NOT_SUPPORTED: fn wxImage8()
    pub fn new_with_str(name: &str, mimetype: &str, index: c_int) -> ImageIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageIsOwned(ffi::wxImage_new9(name, mimetype, index))
        }
    }
    // NOT_SUPPORTED: fn wxImage10()
    pub fn new_with_inputstream(
        stream: *mut c_void,
        mimetype: &str,
        index: c_int,
    ) -> ImageIsOwned<OWNED> {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageIsOwned(ffi::wxImage_new11(stream, mimetype, index))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ImageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ImageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxImage_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ImageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxImageHandler
wx_class! { ImageHandler =
    ImageHandlerIsOwned<true>(wxImageHandler) impl
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageHandlerIsOwned<OWNED> {
    // BLOCKED: fn wxImageHandler()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ImageHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ImageHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxImageHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ImageHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxImageList
wx_class! { ImageList =
    ImageListIsOwned<true>(wxImageList) impl
        ImageListMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageListIsOwned<OWNED> {
    pub fn new() -> ImageListIsOwned<OWNED> {
        unsafe { ImageListIsOwned(ffi::wxImageList_new()) }
    }
    pub fn new_with_int(
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> ImageListIsOwned<OWNED> {
        unsafe { ImageListIsOwned(ffi::wxImageList_new1(width, height, mask, initial_count)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ImageListIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ImageListIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ImageListIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxImageList_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ImageListIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxInfoBar
wx_class! { InfoBar =
    InfoBarIsOwned<true>(wxInfoBar) impl
        InfoBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> InfoBarIsOwned<OWNED> {
    pub fn new_2step() -> InfoBarIsOwned<OWNED> {
        unsafe { InfoBarIsOwned(ffi::wxInfoBar_new()) }
    }
    pub fn new<W: WindowMethods>(parent: Option<&W>, winid: c_int) -> InfoBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            InfoBarIsOwned(ffi::wxInfoBar_new1(parent, winid))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InfoBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: InfoBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for InfoBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxInfoBar_CLASSINFO()) }
    }
}

// wxInitDialogEvent
wx_class! { InitDialogEvent =
    InitDialogEventIsOwned<true>(wxInitDialogEvent) impl
        InitDialogEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> InitDialogEventIsOwned<OWNED> {
    pub fn new(id: c_int) -> InitDialogEventIsOwned<OWNED> {
        unsafe { InitDialogEventIsOwned(ffi::wxInitDialogEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<InitDialogEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: InitDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<InitDialogEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: InitDialogEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for InitDialogEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxInitDialogEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for InitDialogEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxItemContainer
wx_class! { ItemContainer =
    ItemContainerIsOwned<true>(wxItemContainer) impl
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> ItemContainerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ItemContainerIsOwned<OWNED>> for ItemContainerImmutableIsOwned<OWNED> {
    fn from(o: ItemContainerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for ItemContainerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxItemContainer_delete(self.0) }
        }
    }
}

// wxItemContainerImmutable
wx_class! { ItemContainerImmutable =
    ItemContainerImmutableIsOwned<true>(wxItemContainerImmutable) impl
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> ItemContainerImmutableIsOwned<OWNED> {
    // BLOCKED: fn wxItemContainerImmutable()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ItemContainerImmutableIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxItemContainerImmutable_delete(self.0) }
        }
    }
}
