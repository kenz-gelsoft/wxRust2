use super::*;

// wxDC
wxwidgets! {
    /// A wxDC is a "device context" onto which graphics and text can be drawn.
    /// - [`DC`] represents a C++ `wxDC` class instance which your code has ownership, [`DCFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html) for more details.
    #[doc(alias = "wxDC")]
    #[doc(alias = "DC")]
    class DC
        = DCFromCpp<false>(wxDC) impl
        DCMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DCFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DCFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DCFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDC_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DCFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDCBrushChanger
wxwidgets! {
    /// wxDCBrushChanger is a small helper class for setting a brush on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCBrushChanger`] represents a C++ `wxDCBrushChanger` class instance which your code has ownership, [`DCBrushChangerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DCBrushChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCBrushChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html) for more details.
    #[doc(alias = "wxDCBrushChanger")]
    #[doc(alias = "DCBrushChanger")]
    class DCBrushChanger
        = DCBrushChangerFromCpp<false>(wxDCBrushChanger) impl
        DCBrushChangerMethods
}
impl<const FROM_CPP: bool> DCBrushChangerFromCpp<FROM_CPP> {
    /// Sets brush on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCBrushChanger::wxDCBrushChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html#a047d2af137cb4e6860b3911ab46e4909).
    pub fn new<D: DCMethods, B: BrushMethods>(
        dc: &D,
        brush: &B,
    ) -> DCBrushChangerFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            let brush = brush.as_ptr();
            DCBrushChangerFromCpp(ffi::wxDCBrushChanger_new(dc, brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCBrushChangerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DCBrushChangerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDCBrushChanger_delete(self.0) }
        }
    }
}

// wxDCClipper
wxwidgets! {
    /// wxDCClipper is a helper class for setting a clipping region on a wxDC during its lifetime.
    /// - [`DCClipper`] represents a C++ `wxDCClipper` class instance which your code has ownership, [`DCClipperFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DCClipper`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCClipper` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html) for more details.
    #[doc(alias = "wxDCClipper")]
    #[doc(alias = "DCClipper")]
    class DCClipper
        = DCClipperFromCpp<false>(wxDCClipper) impl
        DCClipperMethods
}
impl<const FROM_CPP: bool> DCClipperFromCpp<FROM_CPP> {
    /// Sets the clipping region to the specified region/coordinates.
    ///
    /// See [C++ `wxDCClipper::wxDCClipper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#aa6548fa7be5cff3a74f6a6f539b00adf).
    pub fn new_with_region<D: DCMethods, R: RegionMethods>(
        dc: &D,
        region: &R,
    ) -> DCClipperFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            let region = region.as_ptr();
            DCClipperFromCpp(ffi::wxDCClipper_new(dc, region))
        }
    }
    ///
    /// See [C++ `wxDCClipper::wxDCClipper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#a995a8e0147459e1ba92cbb965fd963a4).
    pub fn new_with_rect<D: DCMethods, R: RectMethods>(
        dc: &D,
        rect: &R,
    ) -> DCClipperFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            DCClipperFromCpp(ffi::wxDCClipper_new1(dc, rect))
        }
    }
    ///
    /// See [C++ `wxDCClipper::wxDCClipper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#a2096fc82c7ab658fcca0a65650ddeb80).
    pub fn new_with_coord<D: DCMethods>(
        dc: &D,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    ) -> DCClipperFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            DCClipperFromCpp(ffi::wxDCClipper_new2(dc, x, y, w, h))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCClipperFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DCClipperFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDCClipper_delete(self.0) }
        }
    }
}

// wxDCFontChanger
wxwidgets! {
    /// wxDCFontChanger is a small helper class for setting a font on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCFontChanger`] represents a C++ `wxDCFontChanger` class instance which your code has ownership, [`DCFontChangerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DCFontChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCFontChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html) for more details.
    #[doc(alias = "wxDCFontChanger")]
    #[doc(alias = "DCFontChanger")]
    class DCFontChanger
        = DCFontChangerFromCpp<false>(wxDCFontChanger) impl
        DCFontChangerMethods
}
impl<const FROM_CPP: bool> DCFontChangerFromCpp<FROM_CPP> {
    /// Trivial constructor not changing anything.
    ///
    /// See [C++ `wxDCFontChanger::wxDCFontChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#aa4bd96e01e9099f10f9394ef9b69b069).
    pub fn new<D: DCMethods>(dc: &D) -> DCFontChangerFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            DCFontChangerFromCpp(ffi::wxDCFontChanger_new(dc))
        }
    }
    /// Sets font on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCFontChanger::wxDCFontChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#a3c5c5fe626322d365cbd3f90928eeaa2).
    pub fn new_with_font<D: DCMethods, F: FontMethods>(
        dc: &D,
        font: &F,
    ) -> DCFontChangerFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            let font = font.as_ptr();
            DCFontChangerFromCpp(ffi::wxDCFontChanger_new1(dc, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCFontChangerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DCFontChangerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDCFontChanger_delete(self.0) }
        }
    }
}

// wxDCOverlay
wxwidgets! {
    /// Connects an overlay with a drawing DC.
    /// - [`DCOverlay`] represents a C++ `wxDCOverlay` class instance which your code has ownership, [`DCOverlayFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DCOverlay`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCOverlay` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html) for more details.
    #[doc(alias = "wxDCOverlay")]
    #[doc(alias = "DCOverlay")]
    class DCOverlay
        = DCOverlayFromCpp<false>(wxDCOverlay) impl
        DCOverlayMethods
}
impl<const FROM_CPP: bool> DCOverlayFromCpp<FROM_CPP> {
    /// Connects this overlay to the corresponding drawing dc, if the overlay is not initialized yet this call will do so.
    ///
    /// See [C++ `wxDCOverlay::wxDCOverlay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html#aeffcb68537d705a07c00adeb008aa64e).
    pub fn new_with_int<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> DCOverlayFromCpp<FROM_CPP> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayFromCpp(ffi::wxDCOverlay_new(overlay, dc, x, y, width, height))
        }
    }
    /// Convenience wrapper that behaves the same using the entire area of the dc.
    ///
    /// See [C++ `wxDCOverlay::wxDCOverlay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html#a45d692f25022296a11389480c651e13b).
    pub fn new<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
    ) -> DCOverlayFromCpp<FROM_CPP> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayFromCpp(ffi::wxDCOverlay_new1(overlay, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCOverlayFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DCOverlayFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDCOverlay_delete(self.0) }
        }
    }
}

// wxDCPenChanger
wxwidgets! {
    /// wxDCPenChanger is a small helper class for setting a pen on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCPenChanger`] represents a C++ `wxDCPenChanger` class instance which your code has ownership, [`DCPenChangerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DCPenChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCPenChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html) for more details.
    #[doc(alias = "wxDCPenChanger")]
    #[doc(alias = "DCPenChanger")]
    class DCPenChanger
        = DCPenChangerFromCpp<false>(wxDCPenChanger) impl
        DCPenChangerMethods
}
impl<const FROM_CPP: bool> DCPenChangerFromCpp<FROM_CPP> {
    /// Sets pen on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCPenChanger::wxDCPenChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html#abefe06367f53d64e35aeb203537e50e3).
    pub fn new<D: DCMethods, P: PenMethods>(dc: &D, pen: &P) -> DCPenChangerFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            let pen = pen.as_ptr();
            DCPenChangerFromCpp(ffi::wxDCPenChanger_new(dc, pen))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCPenChangerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DCPenChangerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDCPenChanger_delete(self.0) }
        }
    }
}

// wxDCTextColourChanger
wxwidgets! {
    /// wxDCTextColourChanger is a small helper class for setting a foreground text colour on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCTextColourChanger`] represents a C++ `wxDCTextColourChanger` class instance which your code has ownership, [`DCTextColourChangerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DCTextColourChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCTextColourChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html) for more details.
    #[doc(alias = "wxDCTextColourChanger")]
    #[doc(alias = "DCTextColourChanger")]
    class DCTextColourChanger
        = DCTextColourChangerFromCpp<false>(wxDCTextColourChanger) impl
        DCTextColourChangerMethods
}
impl<const FROM_CPP: bool> DCTextColourChangerFromCpp<FROM_CPP> {
    /// Trivial constructor not changing anything.
    ///
    /// See [C++ `wxDCTextColourChanger::wxDCTextColourChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#ae9c21638cef0ad69be36a7359811965d).
    pub fn new<D: DCMethods>(dc: &D) -> DCTextColourChangerFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            DCTextColourChangerFromCpp(ffi::wxDCTextColourChanger_new(dc))
        }
    }
    /// Sets col on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCTextColourChanger::wxDCTextColourChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#a0c0cc00023f4edd806220ac147e40784).
    pub fn new_with_colour<D: DCMethods, C: ColourMethods>(
        dc: &D,
        col: &C,
    ) -> DCTextColourChangerFromCpp<FROM_CPP> {
        unsafe {
            let dc = dc.as_ptr();
            let col = col.as_ptr();
            DCTextColourChangerFromCpp(ffi::wxDCTextColourChanger_new1(dc, col))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCTextColourChangerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DCTextColourChangerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDCTextColourChanger_delete(self.0) }
        }
    }
}

// wxDataFormat
wxwidgets! {
    /// A wxDataFormat is an encapsulation of a platform-specific format handle which is used by the system for the clipboard and drag and drop operations.
    /// - [`DataFormat`] represents a C++ `wxDataFormat` class instance which your code has ownership, [`DataFormatFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataFormat`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataFormat` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_format.html) for more details.
    #[doc(alias = "wxDataFormat")]
    #[doc(alias = "DataFormat")]
    class DataFormat
        = DataFormatFromCpp<false>(wxDataFormat) impl
        DataFormatMethods
}
impl<const FROM_CPP: bool> DataFormatFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataFormat()
    /// Constructs a data format object for a custom format identified by its name format.
    ///
    /// See [C++ `wxDataFormat::wxDataFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_format.html#a6c08911611be5e3a5dd35528b4d091db).
    pub fn new(format: &str) -> DataFormatFromCpp<FROM_CPP> {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            DataFormatFromCpp(ffi::wxDataFormat_new1(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataFormatFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DataFormatFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataFormat_delete(self.0) }
        }
    }
}

// wxDataObject
wxwidgets! {
    /// A wxDataObject represents data that can be copied to or from the clipboard, or dragged and dropped.
    /// - [`DataObject`] represents a C++ `wxDataObject` class instance which your code has ownership, [`DataObjectFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_object.html) for more details.
    #[doc(alias = "wxDataObject")]
    #[doc(alias = "DataObject")]
    class DataObject
        = DataObjectFromCpp<false>(wxDataObject) impl
        DataObjectMethods
}
impl<const FROM_CPP: bool> DataObjectFromCpp<FROM_CPP> {
    //  ENUM: Direction
    pub const Get: c_int = 0x01;
    pub const Set: c_int = 0x02;
    pub const Both: c_int = 0x03;

    // BLOCKED: fn wxDataObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DataObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataObject_delete(self.0) }
        }
    }
}

// wxDataObjectComposite
wxwidgets! {
    /// wxDataObjectComposite is the simplest wxDataObject derivation which may be used to support multiple formats.
    /// - [`DataObjectComposite`] represents a C++ `wxDataObjectComposite` class instance which your code has ownership, [`DataObjectCompositeFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataObjectComposite`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataObjectComposite` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html) for more details.
    #[doc(alias = "wxDataObjectComposite")]
    #[doc(alias = "DataObjectComposite")]
    class DataObjectComposite
        = DataObjectCompositeFromCpp<false>(wxDataObjectComposite) impl
        DataObjectCompositeMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> DataObjectCompositeFromCpp<FROM_CPP> {
    /// The default constructor.
    ///
    /// See [C++ `wxDataObjectComposite::wxDataObjectComposite()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html#a711cfefddb7e091d56f87be3b2d0bcb8).
    pub fn new() -> DataObjectCompositeFromCpp<FROM_CPP> {
        unsafe { DataObjectCompositeFromCpp(ffi::wxDataObjectComposite_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectCompositeFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataObjectCompositeFromCpp<FROM_CPP>>
    for DataObjectFromCpp<FROM_CPP>
{
    fn from(o: DataObjectCompositeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataObjectCompositeFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataObjectComposite_delete(self.0) }
        }
    }
}

// wxDataObjectSimple
wxwidgets! {
    /// This is the simplest possible implementation of the wxDataObject class.
    /// - [`DataObjectSimple`] represents a C++ `wxDataObjectSimple` class instance which your code has ownership, [`DataObjectSimpleFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataObjectSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataObjectSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html) for more details.
    #[doc(alias = "wxDataObjectSimple")]
    #[doc(alias = "DataObjectSimple")]
    class DataObjectSimple
        = DataObjectSimpleFromCpp<false>(wxDataObjectSimple) impl
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> DataObjectSimpleFromCpp<FROM_CPP> {
    /// Constructor accepts the supported format (none by default) which may also be set later with SetFormat().
    ///
    /// See [C++ `wxDataObjectSimple::wxDataObjectSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#ad246b285dd2f414f4b13a4d794bf602d).
    pub fn new<D: DataFormatMethods>(format: &D) -> DataObjectSimpleFromCpp<FROM_CPP> {
        unsafe {
            let format = format.as_ptr();
            DataObjectSimpleFromCpp(ffi::wxDataObjectSimple_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectSimpleFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataObjectSimpleFromCpp<FROM_CPP>> for DataObjectFromCpp<FROM_CPP> {
    fn from(o: DataObjectSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataObjectSimpleFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataObjectSimple_delete(self.0) }
        }
    }
}

// wxDataViewBitmapRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render bitmaps.
    /// - [`DataViewBitmapRenderer`] represents a C++ `wxDataViewBitmapRenderer` class instance which your code has ownership, [`DataViewBitmapRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewBitmapRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewBitmapRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_bitmap_renderer.html) for more details.
    #[doc(alias = "wxDataViewBitmapRenderer")]
    #[doc(alias = "DataViewBitmapRenderer")]
    class DataViewBitmapRenderer
        = DataViewBitmapRendererFromCpp<false>(wxDataViewBitmapRenderer) impl
        DataViewBitmapRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewBitmapRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewBitmapRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewBitmapRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewBitmapRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewBitmapRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewBitmapRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewBitmapRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewBitmapRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewBitmapRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewBitmapRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceByIndexRenderer
wxwidgets! {
    /// A wxDataViewCtrl renderer using wxChoice control and indexes into it.
    /// - [`DataViewChoiceByIndexRenderer`] represents a C++ `wxDataViewChoiceByIndexRenderer` class instance which your code has ownership, [`DataViewChoiceByIndexRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewChoiceByIndexRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewChoiceByIndexRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_by_index_renderer.html) for more details.
    #[doc(alias = "wxDataViewChoiceByIndexRenderer")]
    #[doc(alias = "DataViewChoiceByIndexRenderer")]
    class DataViewChoiceByIndexRenderer
        = DataViewChoiceByIndexRendererFromCpp<false>(wxDataViewChoiceByIndexRenderer) impl
        DataViewChoiceByIndexRendererMethods,
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewChoiceByIndexRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewChoiceByIndexRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewChoiceByIndexRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewChoiceByIndexRendererFromCpp<FROM_CPP>>
    for DataViewChoiceRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewChoiceByIndexRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewChoiceByIndexRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewChoiceByIndexRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewChoiceByIndexRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewChoiceByIndexRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewChoiceByIndexRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewChoiceByIndexRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewChoiceByIndexRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceRenderer
wxwidgets! {
    /// A wxDataViewCtrl renderer using wxChoice control and values of strings in it.
    /// - [`DataViewChoiceRenderer`] represents a C++ `wxDataViewChoiceRenderer` class instance which your code has ownership, [`DataViewChoiceRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewChoiceRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewChoiceRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_renderer.html) for more details.
    #[doc(alias = "wxDataViewChoiceRenderer")]
    #[doc(alias = "DataViewChoiceRenderer")]
    class DataViewChoiceRenderer
        = DataViewChoiceRendererFromCpp<false>(wxDataViewChoiceRenderer) impl
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewChoiceRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewChoiceRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewChoiceRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewChoiceRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewChoiceRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewChoiceRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewChoiceRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewChoiceRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewChoiceRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewChoiceRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewColumn
wxwidgets! {
    /// This class represents a column in a wxDataViewCtrl.
    /// - [`DataViewColumn`] represents a C++ `wxDataViewColumn` class instance which your code has ownership, [`DataViewColumnFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewColumn`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewColumn` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html) for more details.
    #[doc(alias = "wxDataViewColumn")]
    #[doc(alias = "DataViewColumn")]
    class DataViewColumn
        = DataViewColumnFromCpp<false>(wxDataViewColumn) impl
        DataViewColumnMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const FROM_CPP: bool> DataViewColumnFromCpp<FROM_CPP> {
    /// Constructs a text column.
    ///
    /// See [C++ `wxDataViewColumn::wxDataViewColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#aa56f4df8543bf14713a2852af471c768).
    pub fn new_with_str<D: DataViewRendererMethods>(
        title: &str,
        renderer: Option<&D>,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> DataViewColumnFromCpp<FROM_CPP> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnFromCpp(ffi::wxDataViewColumn_new(
                title,
                renderer,
                model_column,
                width,
                align,
                flags,
            ))
        }
    }
    /// Constructs a bitmap column.
    ///
    /// See [C++ `wxDataViewColumn::wxDataViewColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#aa3f5c1c707bd95b39a9c74d281e32f6b).
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods, D: DataViewRendererMethods>(
        bitmap: &B,
        renderer: Option<&D>,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> DataViewColumnFromCpp<FROM_CPP> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnFromCpp(ffi::wxDataViewColumn_new1(
                bitmap,
                renderer,
                model_column,
                width,
                align,
                flags,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewColumnFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewColumnFromCpp<FROM_CPP>>
    for SettableHeaderColumnFromCpp<FROM_CPP>
{
    fn from(o: DataViewColumnFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewColumnFromCpp<FROM_CPP>> for HeaderColumnFromCpp<FROM_CPP> {
    fn from(o: DataViewColumnFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewColumnFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewColumn_delete(self.0) }
        }
    }
}

// wxDataViewCtrl
wxwidgets! {
    /// wxDataViewCtrl is a control to display data either in a tree like fashion or in a tabular form or both.
    /// - [`DataViewCtrl`] represents a C++ `wxDataViewCtrl` class instance which your code has ownership, [`DataViewCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html) for more details.
    #[doc(alias = "wxDataViewCtrl")]
    #[doc(alias = "DataViewCtrl")]
    class DataViewCtrl
        = DataViewCtrlFromCpp<false>(wxDataViewCtrl) impl
        DataViewCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewCtrlFromCpp<FROM_CPP> {
    /// Default Constructor.
    ///
    /// See [C++ `wxDataViewCtrl::wxDataViewCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a3c912686a7c04b53445e2e1985685a3f).
    pub fn new_2step() -> DataViewCtrlFromCpp<FROM_CPP> {
        unsafe { DataViewCtrlFromCpp(ffi::wxDataViewCtrl_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxDataViewCtrl::wxDataViewCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a5cc833d3d27d13f5dae7bd2062a55189).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DataViewCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DataViewCtrlFromCpp(ffi::wxDataViewCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DataViewCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: DataViewCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DataViewCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DataViewCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewCtrl_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> ControlMethods for DataViewCtrlFromCpp<FROM_CPP> {
    /// Create the control.
    ///
    /// See [C++ `wxDataViewCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a1bd86d5869de4d24de791a48e9c6926e).
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDataViewCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}

// wxDataViewCustomRenderer
wxwidgets! {
    /// You need to derive a new class from wxDataViewCustomRenderer in order to write a new renderer.
    /// - [`DataViewCustomRenderer`] represents a C++ `wxDataViewCustomRenderer` class instance which your code has ownership, [`DataViewCustomRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewCustomRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewCustomRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html) for more details.
    #[doc(alias = "wxDataViewCustomRenderer")]
    #[doc(alias = "DataViewCustomRenderer")]
    class DataViewCustomRenderer
        = DataViewCustomRendererFromCpp<false>(wxDataViewCustomRenderer) impl
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewCustomRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewCustomRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewCustomRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewCustomRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewCustomRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewCustomRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewCustomRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewCustomRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewCustomRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewCustomRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewDateRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render calendar controls.
    /// - [`DataViewDateRenderer`] represents a C++ `wxDataViewDateRenderer` class instance which your code has ownership, [`DataViewDateRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewDateRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewDateRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_date_renderer.html) for more details.
    #[doc(alias = "wxDataViewDateRenderer")]
    #[doc(alias = "DataViewDateRenderer")]
    class DataViewDateRenderer
        = DataViewDateRendererFromCpp<false>(wxDataViewDateRenderer) impl
        DataViewDateRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewDateRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewDateRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewDateRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewDateRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewDateRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewDateRendererFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewDateRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewDateRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewDateRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewDateRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewEvent
wxwidgets! {
    /// This is the event class for the wxDataViewCtrl notifications.
    /// - [`DataViewEvent`] represents a C++ `wxDataViewEvent` class instance which your code has ownership, [`DataViewEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html) for more details.
    #[doc(alias = "wxDataViewEvent")]
    #[doc(alias = "DataViewEvent")]
    class DataViewEvent
        = DataViewEventFromCpp<false>(wxDataViewEvent) impl
        DataViewEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewEventFromCpp<FROM_CPP> {
    /// Default ctor, normally shouldn't be used and mostly exists only for backwards compatibility.
    ///
    /// See [C++ `wxDataViewEvent::wxDataViewEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#ae6dde6781192716c6c7ee9f828a2a99d).
    pub fn new() -> DataViewEventFromCpp<FROM_CPP> {
        unsafe { DataViewEventFromCpp(ffi::wxDataViewEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDataViewEvent1()
    // NOT_SUPPORTED: fn wxDataViewEvent2()
    /// Copy constructor.
    ///
    /// See [C++ `wxDataViewEvent::wxDataViewEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a5960c6568e8407e54958e7492859ff68).
    pub fn new_with_dataviewevent<D: DataViewEventMethods>(
        event: &D,
    ) -> DataViewEventFromCpp<FROM_CPP> {
        unsafe {
            let event = event.as_ptr();
            DataViewEventFromCpp(ffi::wxDataViewEvent_new3(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewEventFromCpp<FROM_CPP>> for NotifyEventFromCpp<FROM_CPP> {
    fn from(o: DataViewEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: DataViewEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: DataViewEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconText
wxwidgets! {
    /// wxDataViewIconText is used by wxDataViewIconTextRenderer for data transfer.
    /// - [`DataViewIconText`] represents a C++ `wxDataViewIconText` class instance which your code has ownership, [`DataViewIconTextFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewIconText`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewIconText` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html) for more details.
    #[doc(alias = "wxDataViewIconText")]
    #[doc(alias = "DataViewIconText")]
    class DataViewIconText
        = DataViewIconTextFromCpp<false>(wxDataViewIconText) impl
        DataViewIconTextMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewIconTextFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewIconText::wxDataViewIconText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a1de5295b0774784c21a4d5d694df4725).
    pub fn new_with_str<B: BitmapBundleMethods>(
        text: &str,
        bitmap: &B,
    ) -> DataViewIconTextFromCpp<FROM_CPP> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let bitmap = bitmap.as_ptr();
            DataViewIconTextFromCpp(ffi::wxDataViewIconText_new(text, bitmap))
        }
    }
    ///
    /// See [C++ `wxDataViewIconText::wxDataViewIconText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#aa32e3db38e83550e99367f88965be72c).
    pub fn new_with_dataviewicontext<D: DataViewIconTextMethods>(
        other: &D,
    ) -> DataViewIconTextFromCpp<FROM_CPP> {
        unsafe {
            let other = other.as_ptr();
            DataViewIconTextFromCpp(ffi::wxDataViewIconText_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIconTextFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewIconTextFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewIconTextFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewIconTextFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewIconText_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewIconTextFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconTextRenderer
wxwidgets! {
    /// The wxDataViewIconTextRenderer class is used to display text with a small icon next to it as it is typically done in a file manager.
    /// - [`DataViewIconTextRenderer`] represents a C++ `wxDataViewIconTextRenderer` class instance which your code has ownership, [`DataViewIconTextRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewIconTextRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewIconTextRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text_renderer.html) for more details.
    #[doc(alias = "wxDataViewIconTextRenderer")]
    #[doc(alias = "DataViewIconTextRenderer")]
    class DataViewIconTextRenderer
        = DataViewIconTextRendererFromCpp<false>(wxDataViewIconTextRenderer) impl
        DataViewIconTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewIconTextRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewIconTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIconTextRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewIconTextRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewIconTextRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewIconTextRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewIconTextRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewIconTextRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewIconTextRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewIconTextRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIndexListModel
wxwidgets! {
    /// wxDataViewIndexListModel is a specialized data model which lets you address an item by its position (row) rather than its wxDataViewItem (which you can obtain from this class).
    /// - [`DataViewIndexListModel`] represents a C++ `wxDataViewIndexListModel` class instance which your code has ownership, [`DataViewIndexListModelFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewIndexListModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewIndexListModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html) for more details.
    #[doc(alias = "wxDataViewIndexListModel")]
    #[doc(alias = "DataViewIndexListModel")]
    class DataViewIndexListModel
        = DataViewIndexListModelFromCpp<false>(wxDataViewIndexListModel) impl
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const FROM_CPP: bool> DataViewIndexListModelFromCpp<FROM_CPP> {
    // BLOCKED: fn wxDataViewIndexListModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIndexListModelFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewIndexListModelFromCpp<FROM_CPP>>
    for DataViewListModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewIndexListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewIndexListModelFromCpp<FROM_CPP>>
    for DataViewModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewIndexListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewIndexListModelFromCpp<FROM_CPP>>
    for RefCounterFromCpp<FROM_CPP>
{
    fn from(o: DataViewIndexListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewIndexListModelFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewIndexListModel_delete(self.0) }
        }
    }
}

// wxDataViewItem
wxwidgets! {
    /// wxDataViewItem is a small opaque class that represents an item in a wxDataViewCtrl in a persistent way, i.e.
    /// - [`DataViewItem`] represents a C++ `wxDataViewItem` class instance which your code has ownership, [`DataViewItemFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html) for more details.
    #[doc(alias = "wxDataViewItem")]
    #[doc(alias = "DataViewItem")]
    class DataViewItem
        = DataViewItemFromCpp<false>(wxDataViewItem) impl
        DataViewItemMethods
}
impl<const FROM_CPP: bool> DataViewItemFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewItem::wxDataViewItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a7a8b5a738467b471cd10e96357dc800e).
    pub fn new() -> DataViewItemFromCpp<FROM_CPP> {
        unsafe { DataViewItemFromCpp(ffi::wxDataViewItem_new()) }
    }
    ///
    /// See [C++ `wxDataViewItem::wxDataViewItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a355faeb0fd910141a8621c34b884153c).
    pub fn new_with_dataviewitem<D: DataViewItemMethods>(
        item: &D,
    ) -> DataViewItemFromCpp<FROM_CPP> {
        unsafe {
            let item = item.as_ptr();
            DataViewItemFromCpp(ffi::wxDataViewItem_new1(item))
        }
    }
    ///
    /// See [C++ `wxDataViewItem::wxDataViewItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#aa8c216134b8e17a742c070e39753be59).
    pub fn new_with_void(id: *mut c_void) -> DataViewItemFromCpp<FROM_CPP> {
        unsafe { DataViewItemFromCpp(ffi::wxDataViewItem_new2(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewItemFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DataViewItemFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewItem_delete(self.0) }
        }
    }
}

// wxDataViewItemAttr
wxwidgets! {
    /// This class is used to indicate to a wxDataViewCtrl that a certain item (see wxDataViewItem) has extra font attributes for its renderer.
    /// - [`DataViewItemAttr`] represents a C++ `wxDataViewItemAttr` class instance which your code has ownership, [`DataViewItemAttrFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewItemAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewItemAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html) for more details.
    #[doc(alias = "wxDataViewItemAttr")]
    #[doc(alias = "DataViewItemAttr")]
    class DataViewItemAttr
        = DataViewItemAttrFromCpp<false>(wxDataViewItemAttr) impl
        DataViewItemAttrMethods
}
impl<const FROM_CPP: bool> DataViewItemAttrFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewItemAttr::wxDataViewItemAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a730aee09cf5b3d45db8dcae8ffc48fde).
    pub fn new() -> DataViewItemAttrFromCpp<FROM_CPP> {
        unsafe { DataViewItemAttrFromCpp(ffi::wxDataViewItemAttr_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewItemAttrFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DataViewItemAttrFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewItemAttr_delete(self.0) }
        }
    }
}

// wxDataViewListCtrl
wxwidgets! {
    /// This class is a wxDataViewCtrl which internally uses a wxDataViewListStore and forwards most of its API to that class.
    /// - [`DataViewListCtrl`] represents a C++ `wxDataViewListCtrl` class instance which your code has ownership, [`DataViewListCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html) for more details.
    #[doc(alias = "wxDataViewListCtrl")]
    #[doc(alias = "DataViewListCtrl")]
    class DataViewListCtrl
        = DataViewListCtrlFromCpp<false>(wxDataViewListCtrl) impl
        DataViewListCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewListCtrlFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxDataViewListCtrl::wxDataViewListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#ae4ccb1d19929c49a81d3870a10d11765).
    pub fn new_2step() -> DataViewListCtrlFromCpp<FROM_CPP> {
        unsafe { DataViewListCtrlFromCpp(ffi::wxDataViewListCtrl_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxDataViewListCtrl::wxDataViewListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#af7a2515b02e8b5e4aa27cc1831c3686b).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> DataViewListCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewListCtrlFromCpp(ffi::wxDataViewListCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DataViewListCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewListCtrlFromCpp<FROM_CPP>>
    for DataViewCtrlFromCpp<FROM_CPP>
{
    fn from(o: DataViewListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: DataViewListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DataViewListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DataViewListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewListCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewListCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewListCtrl_CLASSINFO()) }
    }
}

// wxDataViewListModel
wxwidgets! {
    /// Base class with abstract API for wxDataViewIndexListModel and wxDataViewVirtualListModel.
    /// - [`DataViewListModel`] represents a C++ `wxDataViewListModel` class instance which your code has ownership, [`DataViewListModelFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewListModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewListModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html) for more details.
    #[doc(alias = "wxDataViewListModel")]
    #[doc(alias = "DataViewListModel")]
    class DataViewListModel
        = DataViewListModelFromCpp<false>(wxDataViewListModel) impl
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const FROM_CPP: bool> DataViewListModelFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewListModelFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewListModelFromCpp<FROM_CPP>>
    for DataViewModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListModelFromCpp<FROM_CPP>>
    for RefCounterFromCpp<FROM_CPP>
{
    fn from(o: DataViewListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewListModelFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewListModel_delete(self.0) }
        }
    }
}

// wxDataViewListStore
wxwidgets! {
    /// wxDataViewListStore is a specialised wxDataViewModel for storing a simple table of data.
    /// - [`DataViewListStore`] represents a C++ `wxDataViewListStore` class instance which your code has ownership, [`DataViewListStoreFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewListStore`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewListStore` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html) for more details.
    #[doc(alias = "wxDataViewListStore")]
    #[doc(alias = "DataViewListStore")]
    class DataViewListStore
        = DataViewListStoreFromCpp<false>(wxDataViewListStore) impl
        DataViewListStoreMethods,
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const FROM_CPP: bool> DataViewListStoreFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewListStore::wxDataViewListStore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#ad0ea04a252cfa338caca32b9bad11640).
    pub fn new() -> DataViewListStoreFromCpp<FROM_CPP> {
        unsafe { DataViewListStoreFromCpp(ffi::wxDataViewListStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewListStoreFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewListStoreFromCpp<FROM_CPP>>
    for DataViewIndexListModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewListStoreFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListStoreFromCpp<FROM_CPP>>
    for DataViewListModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewListStoreFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListStoreFromCpp<FROM_CPP>>
    for DataViewModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewListStoreFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewListStoreFromCpp<FROM_CPP>>
    for RefCounterFromCpp<FROM_CPP>
{
    fn from(o: DataViewListStoreFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewListStoreFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewListStore_delete(self.0) }
        }
    }
}

// wxDataViewModel
wxwidgets! {
    /// wxDataViewModel is the base class for all data model to be displayed by a wxDataViewCtrl.
    /// - [`DataViewModel`] represents a C++ `wxDataViewModel` class instance which your code has ownership, [`DataViewModelFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html) for more details.
    #[doc(alias = "wxDataViewModel")]
    #[doc(alias = "DataViewModel")]
    class DataViewModel
        = DataViewModelFromCpp<false>(wxDataViewModel) impl
        DataViewModelMethods,
        RefCounterMethods
}
impl<const FROM_CPP: bool> DataViewModelFromCpp<FROM_CPP> {
    // BLOCKED: fn wxDataViewModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewModelFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewModelFromCpp<FROM_CPP>> for RefCounterFromCpp<FROM_CPP> {
    fn from(o: DataViewModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewModelFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewModel_delete(self.0) }
        }
    }
}

// wxDataViewModelNotifier
wxwidgets! {
    /// A wxDataViewModelNotifier instance is owned by a wxDataViewModel and mirrors its notification interface.
    /// - [`DataViewModelNotifier`] represents a C++ `wxDataViewModelNotifier` class instance which your code has ownership, [`DataViewModelNotifierFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewModelNotifier`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewModelNotifier` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html) for more details.
    #[doc(alias = "wxDataViewModelNotifier")]
    #[doc(alias = "DataViewModelNotifier")]
    class DataViewModelNotifier
        = DataViewModelNotifierFromCpp<false>(wxDataViewModelNotifier) impl
        DataViewModelNotifierMethods
}
impl<const FROM_CPP: bool> DataViewModelNotifierFromCpp<FROM_CPP> {
    // BLOCKED: fn wxDataViewModelNotifier()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewModelNotifierFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DataViewModelNotifierFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewModelNotifier_delete(self.0) }
        }
    }
}

// wxDataViewProgressRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render progress bars.
    /// - [`DataViewProgressRenderer`] represents a C++ `wxDataViewProgressRenderer` class instance which your code has ownership, [`DataViewProgressRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewProgressRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewProgressRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_progress_renderer.html) for more details.
    #[doc(alias = "wxDataViewProgressRenderer")]
    #[doc(alias = "DataViewProgressRenderer")]
    class DataViewProgressRenderer
        = DataViewProgressRendererFromCpp<false>(wxDataViewProgressRenderer) impl
        DataViewProgressRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewProgressRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewProgressRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewProgressRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewProgressRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewProgressRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewProgressRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewProgressRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewProgressRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewProgressRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewProgressRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render the individual cells.
    /// - [`DataViewRenderer`] represents a C++ `wxDataViewRenderer` class instance which your code has ownership, [`DataViewRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html) for more details.
    #[doc(alias = "wxDataViewRenderer")]
    #[doc(alias = "DataViewRenderer")]
    class DataViewRenderer
        = DataViewRendererFromCpp<false>(wxDataViewRenderer) impl
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewRendererFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewSpinRenderer
wxwidgets! {
    /// This is a specialized renderer for rendering integer values.
    /// - [`DataViewSpinRenderer`] represents a C++ `wxDataViewSpinRenderer` class instance which your code has ownership, [`DataViewSpinRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewSpinRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewSpinRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_spin_renderer.html) for more details.
    #[doc(alias = "wxDataViewSpinRenderer")]
    #[doc(alias = "DataViewSpinRenderer")]
    class DataViewSpinRenderer
        = DataViewSpinRendererFromCpp<false>(wxDataViewSpinRenderer) impl
        DataViewSpinRendererMethods,
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewSpinRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewSpinRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewSpinRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewSpinRendererFromCpp<FROM_CPP>>
    for DataViewCustomRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewSpinRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewSpinRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewSpinRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewSpinRendererFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewSpinRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewSpinRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewSpinRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewSpinRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTextRenderer
wxwidgets! {
    /// wxDataViewTextRenderer is used for rendering text.
    /// - [`DataViewTextRenderer`] represents a C++ `wxDataViewTextRenderer` class instance which your code has ownership, [`DataViewTextRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewTextRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewTextRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_text_renderer.html) for more details.
    #[doc(alias = "wxDataViewTextRenderer")]
    #[doc(alias = "DataViewTextRenderer")]
    class DataViewTextRenderer
        = DataViewTextRendererFromCpp<false>(wxDataViewTextRenderer) impl
        DataViewTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewTextRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewTextRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewTextRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewTextRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewTextRendererFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewTextRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewTextRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewTextRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewTextRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewToggleRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render toggle controls.
    /// - [`DataViewToggleRenderer`] represents a C++ `wxDataViewToggleRenderer` class instance which your code has ownership, [`DataViewToggleRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewToggleRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewToggleRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_toggle_renderer.html) for more details.
    #[doc(alias = "wxDataViewToggleRenderer")]
    #[doc(alias = "DataViewToggleRenderer")]
    class DataViewToggleRenderer
        = DataViewToggleRendererFromCpp<false>(wxDataViewToggleRenderer) impl
        DataViewToggleRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewToggleRendererFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDataViewToggleRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewToggleRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewToggleRendererFromCpp<FROM_CPP>>
    for DataViewRendererFromCpp<FROM_CPP>
{
    fn from(o: DataViewToggleRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewToggleRendererFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: DataViewToggleRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewToggleRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewToggleRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewToggleRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTreeCtrl
wxwidgets! {
    /// This class is a wxDataViewCtrl which internally uses a wxDataViewTreeStore and forwards most of its API to that class.
    /// - [`DataViewTreeCtrl`] represents a C++ `wxDataViewTreeCtrl` class instance which your code has ownership, [`DataViewTreeCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewTreeCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewTreeCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html) for more details.
    #[doc(alias = "wxDataViewTreeCtrl")]
    #[doc(alias = "DataViewTreeCtrl")]
    class DataViewTreeCtrl
        = DataViewTreeCtrlFromCpp<false>(wxDataViewTreeCtrl) impl
        DataViewTreeCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DataViewTreeCtrlFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxDataViewTreeCtrl::wxDataViewTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a49d8a55c826b9b3bb75b8607a0d94fe8).
    pub fn new_2step() -> DataViewTreeCtrlFromCpp<FROM_CPP> {
        unsafe { DataViewTreeCtrlFromCpp(ffi::wxDataViewTreeCtrl_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxDataViewTreeCtrl::wxDataViewTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a263a8abb605575c5ab9db5eba259cf89).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> DataViewTreeCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewTreeCtrlFromCpp(ffi::wxDataViewTreeCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DataViewTreeCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeCtrlFromCpp<FROM_CPP>>
    for DataViewCtrlFromCpp<FROM_CPP>
{
    fn from(o: DataViewTreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: DataViewTreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DataViewTreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DataViewTreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DataViewTreeCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DataViewTreeCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDataViewTreeCtrl_CLASSINFO()) }
    }
}

// wxDataViewTreeStore
wxwidgets! {
    /// wxDataViewTreeStore is a specialised wxDataViewModel for storing simple trees very much like wxTreeCtrl does and it offers a similar API.
    /// - [`DataViewTreeStore`] represents a C++ `wxDataViewTreeStore` class instance which your code has ownership, [`DataViewTreeStoreFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewTreeStore`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewTreeStore` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html) for more details.
    #[doc(alias = "wxDataViewTreeStore")]
    #[doc(alias = "DataViewTreeStore")]
    class DataViewTreeStore
        = DataViewTreeStoreFromCpp<false>(wxDataViewTreeStore) impl
        DataViewTreeStoreMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const FROM_CPP: bool> DataViewTreeStoreFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewTreeStore::wxDataViewTreeStore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#aad8493a851fbef80f8b7c3f368ca53db).
    pub fn new() -> DataViewTreeStoreFromCpp<FROM_CPP> {
        unsafe { DataViewTreeStoreFromCpp(ffi::wxDataViewTreeStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewTreeStoreFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeStoreFromCpp<FROM_CPP>>
    for DataViewModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewTreeStoreFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewTreeStoreFromCpp<FROM_CPP>>
    for RefCounterFromCpp<FROM_CPP>
{
    fn from(o: DataViewTreeStoreFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewTreeStoreFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewTreeStore_delete(self.0) }
        }
    }
}

// wxDataViewVirtualListModel
wxwidgets! {
    /// wxDataViewVirtualListModel is a specialized data model which lets you address an item by its position (row) rather than its wxDataViewItem and as such offers the exact same interface as wxDataViewIndexListModel.
    /// - [`DataViewVirtualListModel`] represents a C++ `wxDataViewVirtualListModel` class instance which your code has ownership, [`DataViewVirtualListModelFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DataViewVirtualListModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewVirtualListModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html) for more details.
    #[doc(alias = "wxDataViewVirtualListModel")]
    #[doc(alias = "DataViewVirtualListModel")]
    class DataViewVirtualListModel
        = DataViewVirtualListModelFromCpp<false>(wxDataViewVirtualListModel) impl
        DataViewVirtualListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const FROM_CPP: bool> DataViewVirtualListModelFromCpp<FROM_CPP> {
    // BLOCKED: fn wxDataViewVirtualListModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewVirtualListModelFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DataViewVirtualListModelFromCpp<FROM_CPP>>
    for DataViewListModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewVirtualListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewVirtualListModelFromCpp<FROM_CPP>>
    for DataViewModelFromCpp<FROM_CPP>
{
    fn from(o: DataViewVirtualListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DataViewVirtualListModelFromCpp<FROM_CPP>>
    for RefCounterFromCpp<FROM_CPP>
{
    fn from(o: DataViewVirtualListModelFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DataViewVirtualListModelFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDataViewVirtualListModel_delete(self.0) }
        }
    }
}

// wxDateEvent
wxwidgets! {
    /// This event class holds information about a date change and is used together with wxDatePickerCtrl.
    /// - [`DateEvent`] represents a C++ `wxDateEvent` class instance which your code has ownership, [`DateEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DateEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDateEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_date_event.html) for more details.
    #[doc(alias = "wxDateEvent")]
    #[doc(alias = "DateEvent")]
    class DateEvent
        = DateEventFromCpp<false>(wxDateEvent) impl
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DateEventFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxDateEvent::wxDateEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_event.html#a54de582d97abba75950d2aa9b3ba84a2).
    pub fn new() -> DateEventFromCpp<FROM_CPP> {
        unsafe { DateEventFromCpp(ffi::wxDateEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDateEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DateEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DateEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: DateEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DateEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: DateEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DateEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DateEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DateEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDateEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DateEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDatePickerCtrl
wxwidgets! {
    /// This control allows the user to select a date.
    /// - [`DatePickerCtrl`] represents a C++ `wxDatePickerCtrl` class instance which your code has ownership, [`DatePickerCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DatePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDatePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html) for more details.
    #[doc(alias = "wxDatePickerCtrl")]
    #[doc(alias = "DatePickerCtrl")]
    class DatePickerCtrl
        = DatePickerCtrlFromCpp<false>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DatePickerCtrlFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxDatePickerCtrl::wxDatePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#ae0700b0b3a7b522e3053e0bbb4de27ec).
    pub fn new_2step() -> DatePickerCtrlFromCpp<FROM_CPP> {
        unsafe { DatePickerCtrlFromCpp(ffi::wxDatePickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxDatePickerCtrl::wxDatePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a697230c98830fca84021f14a697f156c).
    pub fn new<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        dt: &D,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DatePickerCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DatePickerCtrlFromCpp(ffi::wxDatePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DatePickerCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DatePickerCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: DatePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DatePickerCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DatePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DatePickerCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DatePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DatePickerCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DatePickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DatePickerCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDatePickerCtrl_CLASSINFO()) }
    }
}

// wxDelegateRendererNative
wxwidgets! {
    /// wxDelegateRendererNative allows reuse of renderers code by forwarding all the wxRendererNative methods to the given object and thus allowing you to only modify some of its methods  without having to reimplement all of them.
    /// - [`DelegateRendererNative`] represents a C++ `wxDelegateRendererNative` class instance which your code has ownership, [`DelegateRendererNativeFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DelegateRendererNative`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDelegateRendererNative` class's documentation](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html) for more details.
    #[doc(alias = "wxDelegateRendererNative")]
    #[doc(alias = "DelegateRendererNative")]
    class DelegateRendererNative
        = DelegateRendererNativeFromCpp<false>(wxDelegateRendererNative) impl
        DelegateRendererNativeMethods,
        RendererNativeMethods
}
impl<const FROM_CPP: bool> DelegateRendererNativeFromCpp<FROM_CPP> {
    /// The default constructor does the same thing as the other one except that it uses the generic renderer instead of the user-specified rendererNative.
    ///
    /// See [C++ `wxDelegateRendererNative::wxDelegateRendererNative()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html#a184f122211b4632234a5398575305e2c).
    pub fn new() -> DelegateRendererNativeFromCpp<FROM_CPP> {
        unsafe { DelegateRendererNativeFromCpp(ffi::wxDelegateRendererNative_new()) }
    }
    /// This constructor uses the user-specified rendererNative to set up the delegate renderer object to follow all calls to the specified real renderer.
    ///
    /// See [C++ `wxDelegateRendererNative::wxDelegateRendererNative()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html#a8d9dc80cc471a8162a4c9360790fb4cd).
    pub fn new_with_renderernative<R: RendererNativeMethods>(
        renderer_native: &R,
    ) -> DelegateRendererNativeFromCpp<FROM_CPP> {
        unsafe {
            let renderer_native = renderer_native.as_ptr();
            DelegateRendererNativeFromCpp(ffi::wxDelegateRendererNative_new1(renderer_native))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DelegateRendererNativeFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DelegateRendererNativeFromCpp<FROM_CPP>>
    for RendererNativeFromCpp<FROM_CPP>
{
    fn from(o: DelegateRendererNativeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for DelegateRendererNativeFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDelegateRendererNative_delete(self.0) }
        }
    }
}

// wxDialog
wxwidgets! {
    /// A dialog box is a window with a title bar and sometimes a system menu, which can be moved around the screen.
    /// - [`Dialog`] represents a C++ `wxDialog` class instance which your code has ownership, [`DialogFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Dialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html) for more details.
    #[doc(alias = "wxDialog")]
    #[doc(alias = "Dialog")]
    class Dialog
        = DialogFromCpp<false>(wxDialog) impl
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DialogFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxDialog::wxDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a11f9715b3975218071a4de9b29a4ed03).
    pub fn new_2step() -> DialogFromCpp<FROM_CPP> {
        unsafe { DialogFromCpp(ffi::wxDialog_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxDialog::wxDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a0bbd20a18b306aad59429b9d6783d39b).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DialogFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DialogFromCpp(ffi::wxDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DialogFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: DialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DialogFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: DialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDialog_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> TopLevelWindowMethods for DialogFromCpp<FROM_CPP> {
    /// Used for two-step dialog box construction.
    ///
    /// See [C++ `wxDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a44e42338cb8bd2a1b312ab7a6f1722a3).
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
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
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDialog_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    /// Sets the icon for this dialog.
    ///
    /// See [C++ `wxDialog::SetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a717435f3dd9d977feaa40fb359a6da84).
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDialog_SetIcon(self.as_ptr(), icon)
        }
    }
}
impl<const FROM_CPP: bool> WindowMethods for DialogFromCpp<FROM_CPP> {
    /// Centres the dialog box on the display.
    ///
    /// See [C++ `wxDialog::Centre()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a6af384c4a558965bfee61784f5e0b7fc).
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxDialog_Centre(self.as_ptr(), direction) }
    }
}

// wxDialogLayoutAdapter
wxwidgets! {
    /// This abstract class is the base for classes that help wxWidgets perform run-time layout adaptation of dialogs.
    /// - [`DialogLayoutAdapter`] represents a C++ `wxDialogLayoutAdapter` class instance which your code has ownership, [`DialogLayoutAdapterFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DialogLayoutAdapter`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDialogLayoutAdapter` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dialog_layout_adapter.html) for more details.
    #[doc(alias = "wxDialogLayoutAdapter")]
    #[doc(alias = "DialogLayoutAdapter")]
    class DialogLayoutAdapter
        = DialogLayoutAdapterFromCpp<false>(wxDialogLayoutAdapter) impl
        DialogLayoutAdapterMethods
}
impl<const FROM_CPP: bool> DialogLayoutAdapterFromCpp<FROM_CPP> {
    // BLOCKED: fn wxDialogLayoutAdapter()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DialogLayoutAdapterFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DialogLayoutAdapterFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDialogLayoutAdapter_delete(self.0) }
        }
    }
}

// wxDirDialog
wxwidgets! {
    /// This class represents the directory chooser dialog.
    /// - [`DirDialog`] represents a C++ `wxDirDialog` class instance which your code has ownership, [`DirDialogFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DirDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDirDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html) for more details.
    #[doc(alias = "wxDirDialog")]
    #[doc(alias = "DirDialog")]
    class DirDialog
        = DirDialogFromCpp<false>(wxDirDialog) impl
        DirDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DirDialogFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxDirDialog::wxDirDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#a72322832d7830dd637fb4daa541c267a).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        message: &str,
        default_path: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> DirDialogFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let default_path = WxString::from(default_path);
            let default_path = default_path.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DirDialogFromCpp(ffi::wxDirDialog_new(
                parent,
                message,
                default_path,
                style,
                pos,
                size,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DirDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DirDialogFromCpp<FROM_CPP>> for DialogFromCpp<FROM_CPP> {
    fn from(o: DirDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirDialogFromCpp<FROM_CPP>> for TopLevelWindowFromCpp<FROM_CPP> {
    fn from(o: DirDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirDialogFromCpp<FROM_CPP>> for NonOwnedWindowFromCpp<FROM_CPP> {
    fn from(o: DirDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirDialogFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DirDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirDialogFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DirDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirDialogFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DirDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DirDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDirDialog_CLASSINFO()) }
    }
}

// wxDirPickerCtrl
wxwidgets! {
    /// This control allows the user to select a directory.
    /// - [`DirPickerCtrl`] represents a C++ `wxDirPickerCtrl` class instance which your code has ownership, [`DirPickerCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DirPickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDirPickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html) for more details.
    #[doc(alias = "wxDirPickerCtrl")]
    #[doc(alias = "DirPickerCtrl")]
    class DirPickerCtrl
        = DirPickerCtrlFromCpp<false>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DirPickerCtrlFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxDirPickerCtrl::wxDirPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a7afb789fa0326ec140f4645d49cc735b).
    pub fn new_2step() -> DirPickerCtrlFromCpp<FROM_CPP> {
        unsafe { DirPickerCtrlFromCpp(ffi::wxDirPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// See [C++ `wxDirPickerCtrl::wxDirPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a6a8c66f10082401f7445c5660c3b6d79).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DirPickerCtrlFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DirPickerCtrlFromCpp(ffi::wxDirPickerCtrl_new1(
                parent, id, path, message, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for DirPickerCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DirPickerCtrlFromCpp<FROM_CPP>> for PickerBaseFromCpp<FROM_CPP> {
    fn from(o: DirPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirPickerCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: DirPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirPickerCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: DirPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirPickerCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: DirPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DirPickerCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DirPickerCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DirPickerCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDirPickerCtrl_CLASSINFO()) }
    }
}

// wxDisplay
wxwidgets! {
    /// Determines the sizes and locations of displays connected to the system.
    /// - [`Display`] represents a C++ `wxDisplay` class instance which your code has ownership, [`DisplayFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Display`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDisplay` class's documentation](https://docs.wxwidgets.org/3.2/classwx_display.html) for more details.
    #[doc(alias = "wxDisplay")]
    #[doc(alias = "Display")]
    class Display
        = DisplayFromCpp<false>(wxDisplay) impl
        DisplayMethods
}
impl<const FROM_CPP: bool> DisplayFromCpp<FROM_CPP> {
    /// Default constructor creating wxDisplay object representing the primary display.
    ///
    /// See [C++ `wxDisplay::wxDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a8455f1450af24319a5c434d9a1984437).
    pub fn new() -> DisplayFromCpp<FROM_CPP> {
        unsafe { DisplayFromCpp(ffi::wxDisplay_new()) }
    }
    /// Constructor, setting up a wxDisplay instance with the specified display.
    ///
    /// See [C++ `wxDisplay::wxDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a3ff5e051699a4f9ab6ce984d9a8a8943).
    pub fn new_with_uint(index: c_uint) -> DisplayFromCpp<FROM_CPP> {
        unsafe { DisplayFromCpp(ffi::wxDisplay_new1(index)) }
    }
    /// Constructor creating the display object associated with the given window.
    ///
    /// See [C++ `wxDisplay::wxDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a75dc6e0f101a1126269bb0de990b8599).
    pub fn new_with_window<W: WindowMethods>(window: Option<&W>) -> DisplayFromCpp<FROM_CPP> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DisplayFromCpp(ffi::wxDisplay_new2(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DisplayFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DisplayFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDisplay_delete(self.0) }
        }
    }
}

// wxDisplayChangedEvent
wxwidgets! {
    /// A display changed event is sent to top-level windows when the display resolution has changed.
    /// - [`DisplayChangedEvent`] represents a C++ `wxDisplayChangedEvent` class instance which your code has ownership, [`DisplayChangedEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DisplayChangedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDisplayChangedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html) for more details.
    #[doc(alias = "wxDisplayChangedEvent")]
    #[doc(alias = "DisplayChangedEvent")]
    class DisplayChangedEvent
        = DisplayChangedEventFromCpp<false>(wxDisplayChangedEvent) impl
        DisplayChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DisplayChangedEventFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxDisplayChangedEvent::wxDisplayChangedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html#aa9169eb3e0bbe259a738459f39a6eb1a).
    pub fn new() -> DisplayChangedEventFromCpp<FROM_CPP> {
        unsafe { DisplayChangedEventFromCpp(ffi::wxDisplayChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DisplayChangedEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DisplayChangedEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: DisplayChangedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DisplayChangedEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DisplayChangedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DisplayChangedEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDisplayChangedEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DisplayChangedEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDragImage
wxwidgets! {
    /// This class is used when you wish to drag an object on the screen, and a simple cursor is not enough.
    /// - [`DragImage`] represents a C++ `wxDragImage` class instance which your code has ownership, [`DragImageFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DragImage`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDragImage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html) for more details.
    #[doc(alias = "wxDragImage")]
    #[doc(alias = "DragImage")]
    class DragImage
        = DragImageFromCpp<false>(wxDragImage) impl
        DragImageMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DragImageFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1f9e5d1596679795bedb2aebe4841e6d).
    pub fn new() -> DragImageFromCpp<FROM_CPP> {
        unsafe { DragImageFromCpp(ffi::wxDragImage_new()) }
    }
    /// Constructs a drag image from a bitmap and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1aabc326eb82c6aeda17b699b1d45ecb).
    pub fn new_with_bitmap<B: BitmapMethods, C: CursorMethods>(
        image: &B,
        cursor: &C,
    ) -> DragImageFromCpp<FROM_CPP> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageFromCpp(ffi::wxDragImage_new1(image, cursor))
        }
    }
    /// Constructs a drag image from an icon and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a7d1efd4e364ffe45c22aff65b52d19f3).
    pub fn new_with_icon<I: IconMethods, C: CursorMethods>(
        image: &I,
        cursor: &C,
    ) -> DragImageFromCpp<FROM_CPP> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageFromCpp(ffi::wxDragImage_new2(image, cursor))
        }
    }
    /// Constructs a drag image from a text string and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a594a13372b1c37b9a853d0817c17e9c0).
    pub fn new_with_str<C: CursorMethods>(text: &str, cursor: &C) -> DragImageFromCpp<FROM_CPP> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageFromCpp(ffi::wxDragImage_new3(text, cursor))
        }
    }
    /// Constructs a drag image from the text in the given tree control item, and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#afe1171ec9545ff5fbb6268501ed6ff60).
    pub fn new_with_treectrl<T: TreeCtrlMethods, T2: TreeItemIdMethods>(
        tree_ctrl: &T,
        id: &T2,
    ) -> DragImageFromCpp<FROM_CPP> {
        unsafe {
            let tree_ctrl = tree_ctrl.as_ptr();
            let id = id.as_ptr();
            DragImageFromCpp(ffi::wxDragImage_new4(tree_ctrl, id))
        }
    }
    /// Constructs a drag image from the text in the given list control item, and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a24a75b4679d42593180e1e8b9d29c5a5).
    pub fn new_with_listctrl<L: ListCtrlMethods>(
        list_ctrl: &L,
        id: c_long,
    ) -> DragImageFromCpp<FROM_CPP> {
        unsafe {
            let list_ctrl = list_ctrl.as_ptr();
            DragImageFromCpp(ffi::wxDragImage_new5(list_ctrl, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DragImageFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DragImageFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DragImageFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DragImageFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDragImage_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DragImageFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropFilesEvent
wxwidgets! {
    /// This class is used for drop files events, that is, when files have been dropped onto the window.
    /// - [`DropFilesEvent`] represents a C++ `wxDropFilesEvent` class instance which your code has ownership, [`DropFilesEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DropFilesEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDropFilesEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drop_files_event.html) for more details.
    #[doc(alias = "wxDropFilesEvent")]
    #[doc(alias = "DropFilesEvent")]
    class DropFilesEvent
        = DropFilesEventFromCpp<false>(wxDropFilesEvent) impl
        DropFilesEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> DropFilesEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxDropFilesEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropFilesEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<DropFilesEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: DropFilesEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<DropFilesEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: DropFilesEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for DropFilesEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxDropFilesEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for DropFilesEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropSource
wxwidgets! {
    /// This class represents a source for a drag and drop operation.
    /// - [`DropSource`] represents a C++ `wxDropSource` class instance which your code has ownership, [`DropSourceFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DropSource`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDropSource` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html) for more details.
    #[doc(alias = "wxDropSource")]
    #[doc(alias = "DropSource")]
    class DropSource
        = DropSourceFromCpp<false>(wxDropSource) impl
        DropSourceMethods
}
impl<const FROM_CPP: bool> DropSourceFromCpp<FROM_CPP> {
    /// This constructor requires that you must call SetData() later.
    ///
    /// See [C++ `wxDropSource::wxDropSource()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a0534684be549317b7a14ce27d5afc699).
    pub fn new_with_window_cursor<
        W: WindowMethods,
        C: CursorMethods,
        C2: CursorMethods,
        C3: CursorMethods,
    >(
        win: Option<&W>,
        icon_copy: &C,
        icon_move: &C2,
        icon_none: &C3,
    ) -> DropSourceFromCpp<FROM_CPP> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceFromCpp(ffi::wxDropSource_new(win, icon_copy, icon_move, icon_none))
        }
    }
    /// The constructor taking a wxDataObject.
    ///
    /// See [C++ `wxDropSource::wxDropSource()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#afd4966dacaa526f6d2e5708d91136da7).
    pub fn new_with_dataobject_cursor<
        D: DataObjectMethods,
        W: WindowMethods,
        C: CursorMethods,
        C2: CursorMethods,
        C3: CursorMethods,
    >(
        data: &D,
        win: Option<&W>,
        icon_copy: &C,
        icon_move: &C2,
        icon_none: &C3,
    ) -> DropSourceFromCpp<FROM_CPP> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceFromCpp(ffi::wxDropSource_new1(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    /// This constructor requires that you must call SetData() later.
    ///
    /// See [C++ `wxDropSource::wxDropSource()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a06dbcfe97a0615d59c97d0be7d11f6e5).
    pub fn new_with_window_icon<
        W: WindowMethods,
        I: IconMethods,
        I2: IconMethods,
        I3: IconMethods,
    >(
        win: Option<&W>,
        icon_copy: &I,
        icon_move: &I2,
        icon_none: &I3,
    ) -> DropSourceFromCpp<FROM_CPP> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceFromCpp(ffi::wxDropSource_new2(win, icon_copy, icon_move, icon_none))
        }
    }
    /// The constructor taking a wxDataObject.
    ///
    /// See [C++ `wxDropSource::wxDropSource()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a867fea285027d6625acacfcb799833b1).
    pub fn new_with_dataobject_icon<
        D: DataObjectMethods,
        W: WindowMethods,
        I: IconMethods,
        I2: IconMethods,
        I3: IconMethods,
    >(
        data: &D,
        win: Option<&W>,
        icon_copy: &I,
        icon_move: &I2,
        icon_none: &I3,
    ) -> DropSourceFromCpp<FROM_CPP> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceFromCpp(ffi::wxDropSource_new3(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropSourceFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DropSourceFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDropSource_delete(self.0) }
        }
    }
}

// wxDropTarget
wxwidgets! {
    /// This class represents a target for a drag and drop operation.
    /// - [`DropTarget`] represents a C++ `wxDropTarget` class instance which your code has ownership, [`DropTargetFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html) for more details.
    #[doc(alias = "wxDropTarget")]
    #[doc(alias = "DropTarget")]
    class DropTarget
        = DropTargetFromCpp<false>(wxDropTarget) impl
        DropTargetMethods
}
impl<const FROM_CPP: bool> DropTargetFromCpp<FROM_CPP> {
    // BLOCKED: fn wxDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropTargetFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DropTargetFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDropTarget_delete(self.0) }
        }
    }
}
