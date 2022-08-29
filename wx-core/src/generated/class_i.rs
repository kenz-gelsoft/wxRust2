use super::*;

// wxIcon
wxwidgets! {
    /// An icon is a small rectangular bitmap usually used for denoting a minimized application.
    /// - [`Icon`] represents a C++ `wxIcon` class instance which your code has ownership, [`IconIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Icon`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIcon` class's documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html) for more details.
    #[doc(alias = "wxIcon")]
    #[doc(alias = "Icon")]
    class Icon
        = IconIsOwned<true>(wxIcon) impl
        IconMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a1b832f41fcde273eaa4384d2e567aa90).
    pub fn new() -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new()) }
    }
    /// Copy ctor.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a0379f12c09a41e1e18a25f845d1cdafc).
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconIsOwned(ffi::wxIcon_new1(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIcon2()
    /// Creates a bitmap from XPM data.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a8923d0c1f69ca83671e57bb439228fe2).
    pub fn new_with_char(bits: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new3(bits)) }
    }
    // NOT_SUPPORTED: fn wxIcon4()
    /// Loads an icon from the specified location.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a42ebe9eef5b1bc44393af62430ca75b6).
    pub fn new_with_iconlocation(loc: *const c_void) -> IconIsOwned<OWNED> {
        unsafe { IconIsOwned(ffi::wxIcon_new5(loc)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IconIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This class contains multiple copies of an icon in different sizes.
    /// - [`IconBundle`] represents a C++ `wxIconBundle` class instance which your code has ownership, [`IconBundleIsOwned`]`<false>` represents one which don't own.
    /// - Use [`IconBundle`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIconBundle` class's documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html) for more details.
    #[doc(alias = "wxIconBundle")]
    #[doc(alias = "IconBundle")]
    class IconBundle
        = IconBundleIsOwned<true>(wxIconBundle) impl
        IconBundleMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconBundleIsOwned<OWNED> {
    //  ENUM: @29
    pub const FALLBACK_NONE: c_int = 0;
    pub const FALLBACK_SYSTEM: c_int = 1;
    pub const FALLBACK_NEAREST_LARGER: c_int = 2;

    /// Default ctor.
    ///
    /// See [C++ `wxIconBundle::wxIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a2f65bb12c79fa372019d380ede4cfbb4).
    pub fn new() -> IconBundleIsOwned<OWNED> {
        unsafe { IconBundleIsOwned(ffi::wxIconBundle_new()) }
    }
    // NOT_SUPPORTED: fn wxIconBundle1()
    // NOT_SUPPORTED: fn wxIconBundle2()
    /// Initializes the bundle with a single icon.
    ///
    /// See [C++ `wxIconBundle::wxIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a4ab2055fb57aa5ed44990958e2de2b6d).
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconBundleIsOwned<OWNED> {
        unsafe {
            let icon = icon.as_ptr();
            IconBundleIsOwned(ffi::wxIconBundle_new3(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIconBundle4()
    /// Copy constructor.
    ///
    /// See [C++ `wxIconBundle::wxIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a41281e6108842ebfbab4acedfaaaa6de).
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
impl Clone for IconBundleIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// An event being sent when the frame is iconized (minimized) or restored.
    /// - [`IconizeEvent`] represents a C++ `wxIconizeEvent` class instance which your code has ownership, [`IconizeEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`IconizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIconizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_iconize_event.html) for more details.
    #[doc(alias = "wxIconizeEvent")]
    #[doc(alias = "IconizeEvent")]
    class IconizeEvent
        = IconizeEventIsOwned<true>(wxIconizeEvent) impl
        IconizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> IconizeEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxIconizeEvent::wxIconizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_iconize_event.html#a16570936576a28a6ae3f979bfe31b128).
    pub fn new(id: c_int, iconized: bool) -> IconizeEventIsOwned<OWNED> {
        unsafe { IconizeEventIsOwned(ffi::wxIconizeEvent_new(id, iconized)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IconizeEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// wxIdManager is responsible for allocating and releasing window IDs.
    /// - [`IdManager`] represents a C++ `wxIdManager` class instance which your code has ownership, [`IdManagerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`IdManager`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIdManager` class's documentation](https://docs.wxwidgets.org/3.2/classwx_id_manager.html) for more details.
    #[doc(alias = "wxIdManager")]
    #[doc(alias = "IdManager")]
    class IdManager
        = IdManagerIsOwned<true>(wxIdManager) impl
        IdManagerMethods
}
impl<const OWNED: bool> IdManagerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IdManagerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This class encapsulates a platform-independent image.
    /// - [`Image`] represents a C++ `wxImage` class instance which your code has ownership, [`ImageIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Image`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxImage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_image.html) for more details.
    #[doc(alias = "wxImage")]
    #[doc(alias = "Image")]
    class Image
        = ImageIsOwned<true>(wxImage) impl
        ImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageIsOwned<OWNED> {
    /// Creates an empty wxImage object without an alpha channel.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a0a2febdc997f1d09c98f76fdaf85113d).
    pub fn new() -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new()) }
    }
    /// Creates an image with the given size and clears it if requested.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a21652c36c8e51bc696756afeaefe2d01).
    pub fn new_with_int_bool(width: c_int, height: c_int, clear: bool) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new1(width, height, clear)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae071c8cdd85a48655ba59a70aeced3d4).
    pub fn new_with_size_bool<S: SizeMethods>(sz: &S, clear: bool) -> ImageIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            ImageIsOwned(ffi::wxImage_new2(sz, clear))
        }
    }
    /// Creates an image from data in memory.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a2c97634b43bdd143f34418fb1f98a690).
    pub fn new_with_int_uchar_bool(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new3(width, height, data, static_data)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae4dfc16eddb74fca38a10809f56df264).
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
    /// Creates an image from data in memory.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#abdb7e8a7ca45e63935cda55b45869a7a).
    pub fn new_with_int_uchar_uchar(
        width: c_int,
        height: c_int,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new5(width, height, data, alpha, static_data)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ad4d64f98e90c54902bfd9d445d23db29).
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
    /// Creates an image from XPM data.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a5ab9262fabb41525bc669c245654579b).
    pub fn new_with_char(xpm_data: *const c_void) -> ImageIsOwned<OWNED> {
        unsafe { ImageIsOwned(ffi::wxImage_new7(xpm_data)) }
    }
    // NOT_SUPPORTED: fn wxImage8()
    /// Creates an image from a file using MIME-types to specify the type.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a6ef7dc2eb9aaa9bf34437f7c12aad5f2).
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
    /// Creates an image from a stream using MIME-types to specify the type.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#af8df8e96a278f30954592c452b3c0806).
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
impl Clone for ImageIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This is the base class for implementing image file loading/saving, and image creation from data.
    /// - [`ImageHandler`] represents a C++ `wxImageHandler` class instance which your code has ownership, [`ImageHandlerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ImageHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxImageHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html) for more details.
    #[doc(alias = "wxImageHandler")]
    #[doc(alias = "ImageHandler")]
    class ImageHandler
        = ImageHandlerIsOwned<true>(wxImageHandler) impl
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageHandlerIsOwned<OWNED> {
    // BLOCKED: fn wxImageHandler()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ImageHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A wxImageList contains a list of images, which are stored in an unspecified form.
    /// - [`ImageList`] represents a C++ `wxImageList` class instance which your code has ownership, [`ImageListIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ImageList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxImageList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html) for more details.
    #[doc(alias = "wxImageList")]
    #[doc(alias = "ImageList")]
    class ImageList
        = ImageListIsOwned<true>(wxImageList) impl
        ImageListMethods,
        ObjectMethods
}
impl<const OWNED: bool> ImageListIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxImageList::wxImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a91cdd6895654c2043d51bf31bc370d90).
    pub fn new() -> ImageListIsOwned<OWNED> {
        unsafe { ImageListIsOwned(ffi::wxImageList_new()) }
    }
    /// Constructor specifying the image size, whether image masks should be created, and the initial size of the list.
    ///
    /// See [C++ `wxImageList::wxImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a7c99503e0d851e8d416204325014901a).
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
impl Clone for ImageListIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// An info bar is a transient window shown at top or bottom of its parent window to display non-critical information to the user.
    /// - [`InfoBar`] represents a C++ `wxInfoBar` class instance which your code has ownership, [`InfoBarIsOwned`]`<false>` represents one which don't own.
    /// - Use [`InfoBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxInfoBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html) for more details.
    #[doc(alias = "wxInfoBar")]
    #[doc(alias = "InfoBar")]
    class InfoBar
        = InfoBarIsOwned<true>(wxInfoBar) impl
        InfoBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> InfoBarIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxInfoBar::wxInfoBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a802021e4b71dc8286c7fc56a45c73967).
    pub fn new_2step() -> InfoBarIsOwned<OWNED> {
        unsafe { InfoBarIsOwned(ffi::wxInfoBar_new()) }
    }
    /// Constructor creating the info bar window.
    ///
    /// See [C++ `wxInfoBar::wxInfoBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a4055bcabf87e581864ee9c957989cd5c).
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
impl<const OWNED: bool> Clone for InfoBarIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// A wxInitDialogEvent is sent as a dialog or panel is being initialised.
    /// - [`InitDialogEvent`] represents a C++ `wxInitDialogEvent` class instance which your code has ownership, [`InitDialogEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`InitDialogEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxInitDialogEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_init_dialog_event.html) for more details.
    #[doc(alias = "wxInitDialogEvent")]
    #[doc(alias = "InitDialogEvent")]
    class InitDialogEvent
        = InitDialogEventIsOwned<true>(wxInitDialogEvent) impl
        InitDialogEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> InitDialogEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxInitDialogEvent::wxInitDialogEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_init_dialog_event.html#a756b195fd7841a718882e999e92aa7fc).
    pub fn new(id: c_int) -> InitDialogEventIsOwned<OWNED> {
        unsafe { InitDialogEventIsOwned(ffi::wxInitDialogEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for InitDialogEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// This class is an abstract base class for some wxWidgets controls which contain several items such as wxListBox, wxCheckListBox, wxComboBox or wxChoice.
    /// - [`ItemContainer`] represents a C++ `wxItemContainer` class instance which your code has ownership, [`ItemContainerIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ItemContainer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxItemContainer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html) for more details.
    #[doc(alias = "wxItemContainer")]
    #[doc(alias = "ItemContainer")]
    class ItemContainer
        = ItemContainerIsOwned<true>(wxItemContainer) impl
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> ItemContainerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ItemContainerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
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
wxwidgets! {
    /// wxItemContainer defines an interface which is implemented by all controls which have string subitems each of which may be selected.
    /// - [`ItemContainerImmutable`] represents a C++ `wxItemContainerImmutable` class instance which your code has ownership, [`ItemContainerImmutableIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ItemContainerImmutable`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxItemContainerImmutable` class's documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html) for more details.
    #[doc(alias = "wxItemContainerImmutable")]
    #[doc(alias = "ItemContainerImmutable")]
    class ItemContainerImmutable
        = ItemContainerImmutableIsOwned<true>(wxItemContainerImmutable) impl
        ItemContainerImmutableMethods
}
impl<const OWNED: bool> ItemContainerImmutableIsOwned<OWNED> {
    // BLOCKED: fn wxItemContainerImmutable()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ItemContainerImmutableIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ItemContainerImmutableIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxItemContainerImmutable_delete(self.0) }
        }
    }
}
