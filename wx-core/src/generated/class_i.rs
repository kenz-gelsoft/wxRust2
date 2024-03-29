use super::*;

// wxIcon
wxwidgets! {
    /// An icon is a small rectangular bitmap usually used for denoting a minimized application.
    /// - [`Icon`] represents a C++ `wxIcon` class instance which your code has ownership, [`IconFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Icon`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIcon` class's documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html) for more details.
    #[doc(alias = "wxIcon")]
    #[doc(alias = "Icon")]
    class Icon
        = IconFromCpp<false>(wxIcon) impl
        IconMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> IconFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a1b832f41fcde273eaa4384d2e567aa90).
    pub fn new() -> IconFromCpp<FROM_CPP> {
        unsafe { IconFromCpp(ffi::wxIcon_new()) }
    }
    /// Copy ctor.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a0379f12c09a41e1e18a25f845d1cdafc).
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconFromCpp<FROM_CPP> {
        unsafe {
            let icon = icon.as_ptr();
            IconFromCpp(ffi::wxIcon_new1(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIcon2()
    /// Creates a bitmap from XPM data.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a8923d0c1f69ca83671e57bb439228fe2).
    pub fn new_with_char(bits: *const c_void) -> IconFromCpp<FROM_CPP> {
        unsafe { IconFromCpp(ffi::wxIcon_new3(bits)) }
    }
    // NOT_SUPPORTED: fn wxIcon4()
    /// Loads an icon from the specified location.
    ///
    /// See [C++ `wxIcon::wxIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon.html#a42ebe9eef5b1bc44393af62430ca75b6).
    pub fn new_with_iconlocation(loc: *const c_void) -> IconFromCpp<FROM_CPP> {
        unsafe { IconFromCpp(ffi::wxIcon_new5(loc)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IconFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<IconFromCpp<FROM_CPP>> for GDIObjectFromCpp<FROM_CPP> {
    fn from(o: IconFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<IconFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: IconFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for IconFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxIcon_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for IconFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconBundle
wxwidgets! {
    /// This class contains multiple copies of an icon in different sizes.
    /// - [`IconBundle`] represents a C++ `wxIconBundle` class instance which your code has ownership, [`IconBundleFromCpp`]`<true>` represents one which don't own.
    /// - Use [`IconBundle`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIconBundle` class's documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html) for more details.
    #[doc(alias = "wxIconBundle")]
    #[doc(alias = "IconBundle")]
    class IconBundle
        = IconBundleFromCpp<false>(wxIconBundle) impl
        IconBundleMethods,
        GDIObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> IconBundleFromCpp<FROM_CPP> {
    //  ENUM: @29
    pub const FALLBACK_NONE: c_int = 0;
    pub const FALLBACK_SYSTEM: c_int = 1;
    pub const FALLBACK_NEAREST_LARGER: c_int = 2;

    /// Default ctor.
    ///
    /// See [C++ `wxIconBundle::wxIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a2f65bb12c79fa372019d380ede4cfbb4).
    pub fn new() -> IconBundleFromCpp<FROM_CPP> {
        unsafe { IconBundleFromCpp(ffi::wxIconBundle_new()) }
    }
    // NOT_SUPPORTED: fn wxIconBundle1()
    // NOT_SUPPORTED: fn wxIconBundle2()
    /// Initializes the bundle with a single icon.
    ///
    /// See [C++ `wxIconBundle::wxIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a4ab2055fb57aa5ed44990958e2de2b6d).
    pub fn new_with_icon<I: IconMethods>(icon: &I) -> IconBundleFromCpp<FROM_CPP> {
        unsafe {
            let icon = icon.as_ptr();
            IconBundleFromCpp(ffi::wxIconBundle_new3(icon))
        }
    }
    // NOT_SUPPORTED: fn wxIconBundle4()
    /// Copy constructor.
    ///
    /// See [C++ `wxIconBundle::wxIconBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_icon_bundle.html#a41281e6108842ebfbab4acedfaaaa6de).
    pub fn new_with_iconbundle<I: IconBundleMethods>(ic: &I) -> IconBundleFromCpp<FROM_CPP> {
        unsafe {
            let ic = ic.as_ptr();
            IconBundleFromCpp(ffi::wxIconBundle_new5(ic))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IconBundleFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<IconBundleFromCpp<FROM_CPP>> for GDIObjectFromCpp<FROM_CPP> {
    fn from(o: IconBundleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<IconBundleFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: IconBundleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for IconBundleFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxIconBundle_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for IconBundleFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconizeEvent
wxwidgets! {
    /// An event being sent when the frame is iconized (minimized) or restored.
    /// - [`IconizeEvent`] represents a C++ `wxIconizeEvent` class instance which your code has ownership, [`IconizeEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`IconizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIconizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_iconize_event.html) for more details.
    #[doc(alias = "wxIconizeEvent")]
    #[doc(alias = "IconizeEvent")]
    class IconizeEvent
        = IconizeEventFromCpp<false>(wxIconizeEvent) impl
        IconizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> IconizeEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxIconizeEvent::wxIconizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_iconize_event.html#a16570936576a28a6ae3f979bfe31b128).
    pub fn new(id: c_int, iconized: bool) -> IconizeEventFromCpp<FROM_CPP> {
        unsafe { IconizeEventFromCpp(ffi::wxIconizeEvent_new(id, iconized)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IconizeEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<IconizeEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: IconizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<IconizeEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: IconizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for IconizeEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxIconizeEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for IconizeEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIdManager
wxwidgets! {
    /// wxIdManager is responsible for allocating and releasing window IDs.
    /// - [`IdManager`] represents a C++ `wxIdManager` class instance which your code has ownership, [`IdManagerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`IdManager`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxIdManager` class's documentation](https://docs.wxwidgets.org/3.2/classwx_id_manager.html) for more details.
    #[doc(alias = "wxIdManager")]
    #[doc(alias = "IdManager")]
    class IdManager
        = IdManagerFromCpp<false>(wxIdManager) impl
        IdManagerMethods
}
impl<const FROM_CPP: bool> IdManagerFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for IdManagerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for IdManagerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxIdManager_delete(self.0) }
        }
    }
}

// wxImage
wxwidgets! {
    /// This class encapsulates a platform-independent image.
    /// - [`Image`] represents a C++ `wxImage` class instance which your code has ownership, [`ImageFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Image`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxImage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_image.html) for more details.
    #[doc(alias = "wxImage")]
    #[doc(alias = "Image")]
    class Image
        = ImageFromCpp<false>(wxImage) impl
        ImageMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ImageFromCpp<FROM_CPP> {
    /// Creates an empty wxImage object without an alpha channel.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a0a2febdc997f1d09c98f76fdaf85113d).
    pub fn new() -> ImageFromCpp<FROM_CPP> {
        unsafe { ImageFromCpp(ffi::wxImage_new()) }
    }
    /// Creates an image with the given size and clears it if requested.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a21652c36c8e51bc696756afeaefe2d01).
    pub fn new_with_int_bool(width: c_int, height: c_int, clear: bool) -> ImageFromCpp<FROM_CPP> {
        unsafe { ImageFromCpp(ffi::wxImage_new1(width, height, clear)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae071c8cdd85a48655ba59a70aeced3d4).
    pub fn new_with_size_bool<S: SizeMethods>(sz: &S, clear: bool) -> ImageFromCpp<FROM_CPP> {
        unsafe {
            let sz = sz.as_ptr();
            ImageFromCpp(ffi::wxImage_new2(sz, clear))
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
    ) -> ImageFromCpp<FROM_CPP> {
        unsafe { ImageFromCpp(ffi::wxImage_new3(width, height, data, static_data)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ae4dfc16eddb74fca38a10809f56df264).
    pub fn new_with_size_uchar_bool<S: SizeMethods>(
        sz: &S,
        data: *mut c_void,
        static_data: bool,
    ) -> ImageFromCpp<FROM_CPP> {
        unsafe {
            let sz = sz.as_ptr();
            ImageFromCpp(ffi::wxImage_new4(sz, data, static_data))
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
    ) -> ImageFromCpp<FROM_CPP> {
        unsafe { ImageFromCpp(ffi::wxImage_new5(width, height, data, alpha, static_data)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#ad4d64f98e90c54902bfd9d445d23db29).
    pub fn new_with_size_uchar_uchar<S: SizeMethods>(
        sz: &S,
        data: *mut c_void,
        alpha: *mut c_void,
        static_data: bool,
    ) -> ImageFromCpp<FROM_CPP> {
        unsafe {
            let sz = sz.as_ptr();
            ImageFromCpp(ffi::wxImage_new6(sz, data, alpha, static_data))
        }
    }
    /// Creates an image from XPM data.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a5ab9262fabb41525bc669c245654579b).
    pub fn new_with_char(xpm_data: *const c_void) -> ImageFromCpp<FROM_CPP> {
        unsafe { ImageFromCpp(ffi::wxImage_new7(xpm_data)) }
    }
    // NOT_SUPPORTED: fn wxImage8()
    /// Creates an image from a file using MIME-types to specify the type.
    ///
    /// See [C++ `wxImage::wxImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image.html#a6ef7dc2eb9aaa9bf34437f7c12aad5f2).
    pub fn new_with_str(name: &str, mimetype: &str, index: c_int) -> ImageFromCpp<FROM_CPP> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageFromCpp(ffi::wxImage_new9(name, mimetype, index))
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
    ) -> ImageFromCpp<FROM_CPP> {
        unsafe {
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            ImageFromCpp(ffi::wxImage_new11(stream, mimetype, index))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ImageFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ImageFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ImageFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ImageFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxImage_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ImageFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxImageHandler
wxwidgets! {
    /// This is the base class for implementing image file loading/saving, and image creation from data.
    /// - [`ImageHandler`] represents a C++ `wxImageHandler` class instance which your code has ownership, [`ImageHandlerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ImageHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxImageHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_image_handler.html) for more details.
    #[doc(alias = "wxImageHandler")]
    #[doc(alias = "ImageHandler")]
    class ImageHandler
        = ImageHandlerFromCpp<false>(wxImageHandler) impl
        ImageHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ImageHandlerFromCpp<FROM_CPP> {
    // BLOCKED: fn wxImageHandler()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ImageHandlerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ImageHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ImageHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ImageHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxImageHandler_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ImageHandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxImageList
wxwidgets! {
    /// A wxImageList contains a list of images, which are stored in an unspecified form.
    /// - [`ImageList`] represents a C++ `wxImageList` class instance which your code has ownership, [`ImageListFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ImageList`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxImageList` class's documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html) for more details.
    #[doc(alias = "wxImageList")]
    #[doc(alias = "ImageList")]
    class ImageList
        = ImageListFromCpp<false>(wxImageList) impl
        ImageListMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> ImageListFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxImageList::wxImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a91cdd6895654c2043d51bf31bc370d90).
    pub fn new() -> ImageListFromCpp<FROM_CPP> {
        unsafe { ImageListFromCpp(ffi::wxImageList_new()) }
    }
    /// Constructor specifying the image size, whether image masks should be created, and the initial size of the list.
    ///
    /// See [C++ `wxImageList::wxImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_image_list.html#a7c99503e0d851e8d416204325014901a).
    pub fn new_with_int(
        width: c_int,
        height: c_int,
        mask: bool,
        initial_count: c_int,
    ) -> ImageListFromCpp<FROM_CPP> {
        unsafe { ImageListFromCpp(ffi::wxImageList_new1(width, height, mask, initial_count)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ImageListFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ImageListFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: ImageListFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for ImageListFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxImageList_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ImageListFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxInfoBar
wxwidgets! {
    /// An info bar is a transient window shown at top or bottom of its parent window to display non-critical information to the user.
    /// - [`InfoBar`] represents a C++ `wxInfoBar` class instance which your code has ownership, [`InfoBarFromCpp`]`<true>` represents one which don't own.
    /// - Use [`InfoBar`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxInfoBar` class's documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html) for more details.
    #[doc(alias = "wxInfoBar")]
    #[doc(alias = "InfoBar")]
    class InfoBar
        = InfoBarFromCpp<false>(wxInfoBar) impl
        InfoBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> InfoBarFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxInfoBar::wxInfoBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a802021e4b71dc8286c7fc56a45c73967).
    pub fn new_2step() -> InfoBarFromCpp<FROM_CPP> {
        unsafe { InfoBarFromCpp(ffi::wxInfoBar_new()) }
    }
    /// Constructor creating the info bar window.
    ///
    /// See [C++ `wxInfoBar::wxInfoBar()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_info_bar.html#a4055bcabf87e581864ee9c957989cd5c).
    pub fn new<W: WindowMethods>(parent: Option<&W>, winid: c_int) -> InfoBarFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            InfoBarFromCpp(ffi::wxInfoBar_new1(parent, winid))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for InfoBarFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<InfoBarFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: InfoBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<InfoBarFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: InfoBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<InfoBarFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: InfoBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<InfoBarFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: InfoBarFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for InfoBarFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxInfoBar_CLASSINFO()) }
    }
}

// wxInitDialogEvent
wxwidgets! {
    /// A wxInitDialogEvent is sent as a dialog or panel is being initialised.
    /// - [`InitDialogEvent`] represents a C++ `wxInitDialogEvent` class instance which your code has ownership, [`InitDialogEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`InitDialogEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxInitDialogEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_init_dialog_event.html) for more details.
    #[doc(alias = "wxInitDialogEvent")]
    #[doc(alias = "InitDialogEvent")]
    class InitDialogEvent
        = InitDialogEventFromCpp<false>(wxInitDialogEvent) impl
        InitDialogEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> InitDialogEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxInitDialogEvent::wxInitDialogEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_init_dialog_event.html#a756b195fd7841a718882e999e92aa7fc).
    pub fn new(id: c_int) -> InitDialogEventFromCpp<FROM_CPP> {
        unsafe { InitDialogEventFromCpp(ffi::wxInitDialogEvent_new(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for InitDialogEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<InitDialogEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: InitDialogEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<InitDialogEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: InitDialogEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for InitDialogEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxInitDialogEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for InitDialogEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxItemContainer
wxwidgets! {
    /// This class is an abstract base class for some wxWidgets controls which contain several items such as wxListBox, wxCheckListBox, wxComboBox or wxChoice.
    /// - [`ItemContainer`] represents a C++ `wxItemContainer` class instance which your code has ownership, [`ItemContainerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ItemContainer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxItemContainer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_item_container.html) for more details.
    #[doc(alias = "wxItemContainer")]
    #[doc(alias = "ItemContainer")]
    class ItemContainer
        = ItemContainerFromCpp<false>(wxItemContainer) impl
        ItemContainerMethods,
        ItemContainerImmutableMethods
}
impl<const FROM_CPP: bool> ItemContainerFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ItemContainerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<ItemContainerFromCpp<FROM_CPP>>
    for ItemContainerImmutableFromCpp<FROM_CPP>
{
    fn from(o: ItemContainerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for ItemContainerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxItemContainer_delete(self.0) }
        }
    }
}

// wxItemContainerImmutable
wxwidgets! {
    /// wxItemContainer defines an interface which is implemented by all controls which have string subitems each of which may be selected.
    /// - [`ItemContainerImmutable`] represents a C++ `wxItemContainerImmutable` class instance which your code has ownership, [`ItemContainerImmutableFromCpp`]`<true>` represents one which don't own.
    /// - Use [`ItemContainerImmutable`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxItemContainerImmutable` class's documentation](https://docs.wxwidgets.org/3.2/classwx_item_container_immutable.html) for more details.
    #[doc(alias = "wxItemContainerImmutable")]
    #[doc(alias = "ItemContainerImmutable")]
    class ItemContainerImmutable
        = ItemContainerImmutableFromCpp<false>(wxItemContainerImmutable) impl
        ItemContainerImmutableMethods
}
impl<const FROM_CPP: bool> ItemContainerImmutableFromCpp<FROM_CPP> {
    // BLOCKED: fn wxItemContainerImmutable()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ItemContainerImmutableFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ItemContainerImmutableFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxItemContainerImmutable_delete(self.0) }
        }
    }
}
