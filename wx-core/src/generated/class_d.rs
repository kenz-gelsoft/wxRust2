use super::*;

// wxDC
wxwidgets! {
    /// A wxDC is a "device context" onto which graphics and text can be drawn.
    /// - [`DC`] represents a C++ `wxDC` class instance which your code has ownership, [`DCInRust`]`<false>` represents one which don't own.
    /// - Use [`DC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html) for more details.
    #[doc(alias = "wxDC")]
    #[doc(alias = "DC")]
    class DC
        = DCInRust<true>(wxDC) impl
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> DCInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDCBrushChanger
wxwidgets! {
    /// wxDCBrushChanger is a small helper class for setting a brush on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCBrushChanger`] represents a C++ `wxDCBrushChanger` class instance which your code has ownership, [`DCBrushChangerInRust`]`<false>` represents one which don't own.
    /// - Use [`DCBrushChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCBrushChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html) for more details.
    #[doc(alias = "wxDCBrushChanger")]
    #[doc(alias = "DCBrushChanger")]
    class DCBrushChanger
        = DCBrushChangerInRust<true>(wxDCBrushChanger) impl
        DCBrushChangerMethods
}
impl<const OWNED: bool> DCBrushChangerInRust<OWNED> {
    /// Sets brush on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCBrushChanger::wxDCBrushChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html#a047d2af137cb4e6860b3911ab46e4909).
    pub fn new<D: DCMethods, B: BrushMethods>(dc: &D, brush: &B) -> DCBrushChangerInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let brush = brush.as_ptr();
            DCBrushChangerInRust(ffi::wxDCBrushChanger_new(dc, brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCBrushChangerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCBrushChangerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCBrushChanger_delete(self.0) }
        }
    }
}

// wxDCClipper
wxwidgets! {
    /// wxDCClipper is a helper class for setting a clipping region on a wxDC during its lifetime.
    /// - [`DCClipper`] represents a C++ `wxDCClipper` class instance which your code has ownership, [`DCClipperInRust`]`<false>` represents one which don't own.
    /// - Use [`DCClipper`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCClipper` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html) for more details.
    #[doc(alias = "wxDCClipper")]
    #[doc(alias = "DCClipper")]
    class DCClipper
        = DCClipperInRust<true>(wxDCClipper) impl
        DCClipperMethods
}
impl<const OWNED: bool> DCClipperInRust<OWNED> {
    /// Sets the clipping region to the specified region/coordinates.
    ///
    /// See [C++ `wxDCClipper::wxDCClipper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#aa6548fa7be5cff3a74f6a6f539b00adf).
    pub fn new_with_region<D: DCMethods, R: RegionMethods>(
        dc: &D,
        region: &R,
    ) -> DCClipperInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let region = region.as_ptr();
            DCClipperInRust(ffi::wxDCClipper_new(dc, region))
        }
    }
    ///
    /// See [C++ `wxDCClipper::wxDCClipper()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#a995a8e0147459e1ba92cbb965fd963a4).
    pub fn new_with_rect<D: DCMethods, R: RectMethods>(dc: &D, rect: &R) -> DCClipperInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            DCClipperInRust(ffi::wxDCClipper_new1(dc, rect))
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
    ) -> DCClipperInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCClipperInRust(ffi::wxDCClipper_new2(dc, x, y, w, h))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCClipperInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCClipperInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCClipper_delete(self.0) }
        }
    }
}

// wxDCFontChanger
wxwidgets! {
    /// wxDCFontChanger is a small helper class for setting a font on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCFontChanger`] represents a C++ `wxDCFontChanger` class instance which your code has ownership, [`DCFontChangerInRust`]`<false>` represents one which don't own.
    /// - Use [`DCFontChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCFontChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html) for more details.
    #[doc(alias = "wxDCFontChanger")]
    #[doc(alias = "DCFontChanger")]
    class DCFontChanger
        = DCFontChangerInRust<true>(wxDCFontChanger) impl
        DCFontChangerMethods
}
impl<const OWNED: bool> DCFontChangerInRust<OWNED> {
    /// Trivial constructor not changing anything.
    ///
    /// See [C++ `wxDCFontChanger::wxDCFontChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#aa4bd96e01e9099f10f9394ef9b69b069).
    pub fn new<D: DCMethods>(dc: &D) -> DCFontChangerInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCFontChangerInRust(ffi::wxDCFontChanger_new(dc))
        }
    }
    /// Sets font on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCFontChanger::wxDCFontChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#a3c5c5fe626322d365cbd3f90928eeaa2).
    pub fn new_with_font<D: DCMethods, F: FontMethods>(
        dc: &D,
        font: &F,
    ) -> DCFontChangerInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let font = font.as_ptr();
            DCFontChangerInRust(ffi::wxDCFontChanger_new1(dc, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCFontChangerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCFontChangerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCFontChanger_delete(self.0) }
        }
    }
}

// wxDCOverlay
wxwidgets! {
    /// Connects an overlay with a drawing DC.
    /// - [`DCOverlay`] represents a C++ `wxDCOverlay` class instance which your code has ownership, [`DCOverlayInRust`]`<false>` represents one which don't own.
    /// - Use [`DCOverlay`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCOverlay` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html) for more details.
    #[doc(alias = "wxDCOverlay")]
    #[doc(alias = "DCOverlay")]
    class DCOverlay
        = DCOverlayInRust<true>(wxDCOverlay) impl
        DCOverlayMethods
}
impl<const OWNED: bool> DCOverlayInRust<OWNED> {
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
    ) -> DCOverlayInRust<OWNED> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayInRust(ffi::wxDCOverlay_new(overlay, dc, x, y, width, height))
        }
    }
    /// Convenience wrapper that behaves the same using the entire area of the dc.
    ///
    /// See [C++ `wxDCOverlay::wxDCOverlay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html#a45d692f25022296a11389480c651e13b).
    pub fn new<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
    ) -> DCOverlayInRust<OWNED> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayInRust(ffi::wxDCOverlay_new1(overlay, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCOverlayInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCOverlayInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCOverlay_delete(self.0) }
        }
    }
}

// wxDCPenChanger
wxwidgets! {
    /// wxDCPenChanger is a small helper class for setting a pen on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCPenChanger`] represents a C++ `wxDCPenChanger` class instance which your code has ownership, [`DCPenChangerInRust`]`<false>` represents one which don't own.
    /// - Use [`DCPenChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCPenChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html) for more details.
    #[doc(alias = "wxDCPenChanger")]
    #[doc(alias = "DCPenChanger")]
    class DCPenChanger
        = DCPenChangerInRust<true>(wxDCPenChanger) impl
        DCPenChangerMethods
}
impl<const OWNED: bool> DCPenChangerInRust<OWNED> {
    /// Sets pen on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCPenChanger::wxDCPenChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html#abefe06367f53d64e35aeb203537e50e3).
    pub fn new<D: DCMethods, P: PenMethods>(dc: &D, pen: &P) -> DCPenChangerInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let pen = pen.as_ptr();
            DCPenChangerInRust(ffi::wxDCPenChanger_new(dc, pen))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCPenChangerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCPenChangerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCPenChanger_delete(self.0) }
        }
    }
}

// wxDCTextColourChanger
wxwidgets! {
    /// wxDCTextColourChanger is a small helper class for setting a foreground text colour on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    /// - [`DCTextColourChanger`] represents a C++ `wxDCTextColourChanger` class instance which your code has ownership, [`DCTextColourChangerInRust`]`<false>` represents one which don't own.
    /// - Use [`DCTextColourChanger`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDCTextColourChanger` class's documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html) for more details.
    #[doc(alias = "wxDCTextColourChanger")]
    #[doc(alias = "DCTextColourChanger")]
    class DCTextColourChanger
        = DCTextColourChangerInRust<true>(wxDCTextColourChanger) impl
        DCTextColourChangerMethods
}
impl<const OWNED: bool> DCTextColourChangerInRust<OWNED> {
    /// Trivial constructor not changing anything.
    ///
    /// See [C++ `wxDCTextColourChanger::wxDCTextColourChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#ae9c21638cef0ad69be36a7359811965d).
    pub fn new<D: DCMethods>(dc: &D) -> DCTextColourChangerInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCTextColourChangerInRust(ffi::wxDCTextColourChanger_new(dc))
        }
    }
    /// Sets col on the given dc, storing the old one.
    ///
    /// See [C++ `wxDCTextColourChanger::wxDCTextColourChanger()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#a0c0cc00023f4edd806220ac147e40784).
    pub fn new_with_colour<D: DCMethods, C: ColourMethods>(
        dc: &D,
        col: &C,
    ) -> DCTextColourChangerInRust<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let col = col.as_ptr();
            DCTextColourChangerInRust(ffi::wxDCTextColourChanger_new1(dc, col))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCTextColourChangerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCTextColourChangerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCTextColourChanger_delete(self.0) }
        }
    }
}

// wxDataFormat
wxwidgets! {
    /// A wxDataFormat is an encapsulation of a platform-specific format handle which is used by the system for the clipboard and drag and drop operations.
    /// - [`DataFormat`] represents a C++ `wxDataFormat` class instance which your code has ownership, [`DataFormatInRust`]`<false>` represents one which don't own.
    /// - Use [`DataFormat`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataFormat` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_format.html) for more details.
    #[doc(alias = "wxDataFormat")]
    #[doc(alias = "DataFormat")]
    class DataFormat
        = DataFormatInRust<true>(wxDataFormat) impl
        DataFormatMethods
}
impl<const OWNED: bool> DataFormatInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataFormat()
    /// Constructs a data format object for a custom format identified by its name format.
    ///
    /// See [C++ `wxDataFormat::wxDataFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_format.html#a6c08911611be5e3a5dd35528b4d091db).
    pub fn new(format: &str) -> DataFormatInRust<OWNED> {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            DataFormatInRust(ffi::wxDataFormat_new1(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataFormatInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataFormatInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataFormat_delete(self.0) }
        }
    }
}

// wxDataObject
wxwidgets! {
    /// A wxDataObject represents data that can be copied to or from the clipboard, or dragged and dropped.
    /// - [`DataObject`] represents a C++ `wxDataObject` class instance which your code has ownership, [`DataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`DataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_object.html) for more details.
    #[doc(alias = "wxDataObject")]
    #[doc(alias = "DataObject")]
    class DataObject
        = DataObjectInRust<true>(wxDataObject) impl
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectInRust<OWNED> {
    //  ENUM: Direction
    pub const Get: c_int = 0x01;
    pub const Set: c_int = 0x02;
    pub const Both: c_int = 0x03;

    // BLOCKED: fn wxDataObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObject_delete(self.0) }
        }
    }
}

// wxDataObjectComposite
wxwidgets! {
    /// wxDataObjectComposite is the simplest wxDataObject derivation which may be used to support multiple formats.
    /// - [`DataObjectComposite`] represents a C++ `wxDataObjectComposite` class instance which your code has ownership, [`DataObjectCompositeInRust`]`<false>` represents one which don't own.
    /// - Use [`DataObjectComposite`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataObjectComposite` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html) for more details.
    #[doc(alias = "wxDataObjectComposite")]
    #[doc(alias = "DataObjectComposite")]
    class DataObjectComposite
        = DataObjectCompositeInRust<true>(wxDataObjectComposite) impl
        DataObjectCompositeMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectCompositeInRust<OWNED> {
    /// The default constructor.
    ///
    /// See [C++ `wxDataObjectComposite::wxDataObjectComposite()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html#a711cfefddb7e091d56f87be3b2d0bcb8).
    pub fn new() -> DataObjectCompositeInRust<OWNED> {
        unsafe { DataObjectCompositeInRust(ffi::wxDataObjectComposite_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectCompositeInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataObjectCompositeInRust<OWNED>> for DataObjectInRust<OWNED> {
    fn from(o: DataObjectCompositeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectCompositeInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectComposite_delete(self.0) }
        }
    }
}

// wxDataObjectSimple
wxwidgets! {
    /// This is the simplest possible implementation of the wxDataObject class.
    /// - [`DataObjectSimple`] represents a C++ `wxDataObjectSimple` class instance which your code has ownership, [`DataObjectSimpleInRust`]`<false>` represents one which don't own.
    /// - Use [`DataObjectSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataObjectSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html) for more details.
    #[doc(alias = "wxDataObjectSimple")]
    #[doc(alias = "DataObjectSimple")]
    class DataObjectSimple
        = DataObjectSimpleInRust<true>(wxDataObjectSimple) impl
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectSimpleInRust<OWNED> {
    /// Constructor accepts the supported format (none by default) which may also be set later with SetFormat().
    ///
    /// See [C++ `wxDataObjectSimple::wxDataObjectSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#ad246b285dd2f414f4b13a4d794bf602d).
    pub fn new<D: DataFormatMethods>(format: &D) -> DataObjectSimpleInRust<OWNED> {
        unsafe {
            let format = format.as_ptr();
            DataObjectSimpleInRust(ffi::wxDataObjectSimple_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectSimpleInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataObjectSimpleInRust<OWNED>> for DataObjectInRust<OWNED> {
    fn from(o: DataObjectSimpleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectSimpleInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectSimple_delete(self.0) }
        }
    }
}

// wxDataViewBitmapRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render bitmaps.
    /// - [`DataViewBitmapRenderer`] represents a C++ `wxDataViewBitmapRenderer` class instance which your code has ownership, [`DataViewBitmapRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewBitmapRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewBitmapRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_bitmap_renderer.html) for more details.
    #[doc(alias = "wxDataViewBitmapRenderer")]
    #[doc(alias = "DataViewBitmapRenderer")]
    class DataViewBitmapRenderer
        = DataViewBitmapRendererInRust<true>(wxDataViewBitmapRenderer) impl
        DataViewBitmapRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewBitmapRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewBitmapRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewBitmapRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewBitmapRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewBitmapRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewBitmapRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewBitmapRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewBitmapRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewBitmapRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewBitmapRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceByIndexRenderer
wxwidgets! {
    /// A wxDataViewCtrl renderer using wxChoice control and indexes into it.
    /// - [`DataViewChoiceByIndexRenderer`] represents a C++ `wxDataViewChoiceByIndexRenderer` class instance which your code has ownership, [`DataViewChoiceByIndexRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewChoiceByIndexRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewChoiceByIndexRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_by_index_renderer.html) for more details.
    #[doc(alias = "wxDataViewChoiceByIndexRenderer")]
    #[doc(alias = "DataViewChoiceByIndexRenderer")]
    class DataViewChoiceByIndexRenderer
        = DataViewChoiceByIndexRendererInRust<true>(wxDataViewChoiceByIndexRenderer) impl
        DataViewChoiceByIndexRendererMethods,
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewChoiceByIndexRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewChoiceByIndexRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewChoiceByIndexRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererInRust<OWNED>>
    for DataViewChoiceRendererInRust<OWNED>
{
    fn from(o: DataViewChoiceByIndexRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewChoiceByIndexRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewChoiceByIndexRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewChoiceByIndexRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewChoiceByIndexRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewChoiceByIndexRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceRenderer
wxwidgets! {
    /// A wxDataViewCtrl renderer using wxChoice control and values of strings in it.
    /// - [`DataViewChoiceRenderer`] represents a C++ `wxDataViewChoiceRenderer` class instance which your code has ownership, [`DataViewChoiceRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewChoiceRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewChoiceRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_renderer.html) for more details.
    #[doc(alias = "wxDataViewChoiceRenderer")]
    #[doc(alias = "DataViewChoiceRenderer")]
    class DataViewChoiceRenderer
        = DataViewChoiceRendererInRust<true>(wxDataViewChoiceRenderer) impl
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewChoiceRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewChoiceRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewChoiceRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewChoiceRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewChoiceRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewChoiceRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewChoiceRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewChoiceRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewChoiceRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewColumn
wxwidgets! {
    /// This class represents a column in a wxDataViewCtrl.
    /// - [`DataViewColumn`] represents a C++ `wxDataViewColumn` class instance which your code has ownership, [`DataViewColumnInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewColumn`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewColumn` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html) for more details.
    #[doc(alias = "wxDataViewColumn")]
    #[doc(alias = "DataViewColumn")]
    class DataViewColumn
        = DataViewColumnInRust<true>(wxDataViewColumn) impl
        DataViewColumnMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> DataViewColumnInRust<OWNED> {
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
    ) -> DataViewColumnInRust<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnInRust(ffi::wxDataViewColumn_new(
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
    ) -> DataViewColumnInRust<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnInRust(ffi::wxDataViewColumn_new1(
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
impl Clone for DataViewColumnInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewColumnInRust<OWNED>> for SettableHeaderColumnInRust<OWNED> {
    fn from(o: DataViewColumnInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewColumnInRust<OWNED>> for HeaderColumnInRust<OWNED> {
    fn from(o: DataViewColumnInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewColumnInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewColumn_delete(self.0) }
        }
    }
}

// wxDataViewCtrl
wxwidgets! {
    /// wxDataViewCtrl is a control to display data either in a tree like fashion or in a tabular form or both.
    /// - [`DataViewCtrl`] represents a C++ `wxDataViewCtrl` class instance which your code has ownership, [`DataViewCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html) for more details.
    #[doc(alias = "wxDataViewCtrl")]
    #[doc(alias = "DataViewCtrl")]
    class DataViewCtrl
        = DataViewCtrlInRust<true>(wxDataViewCtrl) impl
        DataViewCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCtrlInRust<OWNED> {
    /// Default Constructor.
    ///
    /// See [C++ `wxDataViewCtrl::wxDataViewCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a3c912686a7c04b53445e2e1985685a3f).
    pub fn new_2step() -> DataViewCtrlInRust<OWNED> {
        unsafe { DataViewCtrlInRust(ffi::wxDataViewCtrl_new()) }
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
    ) -> DataViewCtrlInRust<OWNED> {
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
            DataViewCtrlInRust(ffi::wxDataViewCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DataViewCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: DataViewCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DataViewCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DataViewCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for DataViewCtrlInRust<OWNED> {
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
    /// - [`DataViewCustomRenderer`] represents a C++ `wxDataViewCustomRenderer` class instance which your code has ownership, [`DataViewCustomRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewCustomRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewCustomRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html) for more details.
    #[doc(alias = "wxDataViewCustomRenderer")]
    #[doc(alias = "DataViewCustomRenderer")]
    class DataViewCustomRenderer
        = DataViewCustomRendererInRust<true>(wxDataViewCustomRenderer) impl
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCustomRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewCustomRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewCustomRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewCustomRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewCustomRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCustomRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewCustomRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCustomRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewCustomRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewCustomRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewDateRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render calendar controls.
    /// - [`DataViewDateRenderer`] represents a C++ `wxDataViewDateRenderer` class instance which your code has ownership, [`DataViewDateRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewDateRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewDateRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_date_renderer.html) for more details.
    #[doc(alias = "wxDataViewDateRenderer")]
    #[doc(alias = "DataViewDateRenderer")]
    class DataViewDateRenderer
        = DataViewDateRendererInRust<true>(wxDataViewDateRenderer) impl
        DataViewDateRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewDateRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewDateRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewDateRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewDateRendererInRust<OWNED>> for DataViewRendererInRust<OWNED> {
    fn from(o: DataViewDateRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewDateRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewDateRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewDateRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewDateRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewDateRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewEvent
wxwidgets! {
    /// This is the event class for the wxDataViewCtrl notifications.
    /// - [`DataViewEvent`] represents a C++ `wxDataViewEvent` class instance which your code has ownership, [`DataViewEventInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html) for more details.
    #[doc(alias = "wxDataViewEvent")]
    #[doc(alias = "DataViewEvent")]
    class DataViewEvent
        = DataViewEventInRust<true>(wxDataViewEvent) impl
        DataViewEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewEventInRust<OWNED> {
    /// Default ctor, normally shouldn't be used and mostly exists only for backwards compatibility.
    ///
    /// See [C++ `wxDataViewEvent::wxDataViewEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#ae6dde6781192716c6c7ee9f828a2a99d).
    pub fn new() -> DataViewEventInRust<OWNED> {
        unsafe { DataViewEventInRust(ffi::wxDataViewEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDataViewEvent1()
    // NOT_SUPPORTED: fn wxDataViewEvent2()
    /// Copy constructor.
    ///
    /// See [C++ `wxDataViewEvent::wxDataViewEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a5960c6568e8407e54958e7492859ff68).
    pub fn new_with_dataviewevent<D: DataViewEventMethods>(
        event: &D,
    ) -> DataViewEventInRust<OWNED> {
        unsafe {
            let event = event.as_ptr();
            DataViewEventInRust(ffi::wxDataViewEvent_new3(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: DataViewEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: DataViewEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: DataViewEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconText
wxwidgets! {
    /// wxDataViewIconText is used by wxDataViewIconTextRenderer for data transfer.
    /// - [`DataViewIconText`] represents a C++ `wxDataViewIconText` class instance which your code has ownership, [`DataViewIconTextInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewIconText`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewIconText` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html) for more details.
    #[doc(alias = "wxDataViewIconText")]
    #[doc(alias = "DataViewIconText")]
    class DataViewIconText
        = DataViewIconTextInRust<true>(wxDataViewIconText) impl
        DataViewIconTextMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewIconText::wxDataViewIconText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a1de5295b0774784c21a4d5d694df4725).
    pub fn new_with_str<B: BitmapBundleMethods>(
        text: &str,
        bitmap: &B,
    ) -> DataViewIconTextInRust<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let bitmap = bitmap.as_ptr();
            DataViewIconTextInRust(ffi::wxDataViewIconText_new(text, bitmap))
        }
    }
    ///
    /// See [C++ `wxDataViewIconText::wxDataViewIconText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#aa32e3db38e83550e99367f88965be72c).
    pub fn new_with_dataviewicontext<D: DataViewIconTextMethods>(
        other: &D,
    ) -> DataViewIconTextInRust<OWNED> {
        unsafe {
            let other = other.as_ptr();
            DataViewIconTextInRust(ffi::wxDataViewIconText_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIconTextInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewIconTextInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewIconTextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewIconTextInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewIconText_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIconTextInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconTextRenderer
wxwidgets! {
    /// The wxDataViewIconTextRenderer class is used to display text with a small icon next to it as it is typically done in a file manager.
    /// - [`DataViewIconTextRenderer`] represents a C++ `wxDataViewIconTextRenderer` class instance which your code has ownership, [`DataViewIconTextRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewIconTextRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewIconTextRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text_renderer.html) for more details.
    #[doc(alias = "wxDataViewIconTextRenderer")]
    #[doc(alias = "DataViewIconTextRenderer")]
    class DataViewIconTextRenderer
        = DataViewIconTextRendererInRust<true>(wxDataViewIconTextRenderer) impl
        DataViewIconTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewIconTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIconTextRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewIconTextRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewIconTextRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIconTextRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewIconTextRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewIconTextRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewIconTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIconTextRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIndexListModel
wxwidgets! {
    /// wxDataViewIndexListModel is a specialized data model which lets you address an item by its position (row) rather than its wxDataViewItem (which you can obtain from this class).
    /// - [`DataViewIndexListModel`] represents a C++ `wxDataViewIndexListModel` class instance which your code has ownership, [`DataViewIndexListModelInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewIndexListModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewIndexListModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html) for more details.
    #[doc(alias = "wxDataViewIndexListModel")]
    #[doc(alias = "DataViewIndexListModel")]
    class DataViewIndexListModel
        = DataViewIndexListModelInRust<true>(wxDataViewIndexListModel) impl
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewIndexListModelInRust<OWNED> {
    // BLOCKED: fn wxDataViewIndexListModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIndexListModelInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelInRust<OWNED>>
    for DataViewListModelInRust<OWNED>
{
    fn from(o: DataViewIndexListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelInRust<OWNED>> for DataViewModelInRust<OWNED> {
    fn from(o: DataViewIndexListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelInRust<OWNED>> for RefCounterInRust<OWNED> {
    fn from(o: DataViewIndexListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIndexListModelInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewIndexListModel_delete(self.0) }
        }
    }
}

// wxDataViewItem
wxwidgets! {
    /// wxDataViewItem is a small opaque class that represents an item in a wxDataViewCtrl in a persistent way, i.e.
    /// - [`DataViewItem`] represents a C++ `wxDataViewItem` class instance which your code has ownership, [`DataViewItemInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html) for more details.
    #[doc(alias = "wxDataViewItem")]
    #[doc(alias = "DataViewItem")]
    class DataViewItem
        = DataViewItemInRust<true>(wxDataViewItem) impl
        DataViewItemMethods
}
impl<const OWNED: bool> DataViewItemInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewItem::wxDataViewItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a7a8b5a738467b471cd10e96357dc800e).
    pub fn new() -> DataViewItemInRust<OWNED> {
        unsafe { DataViewItemInRust(ffi::wxDataViewItem_new()) }
    }
    ///
    /// See [C++ `wxDataViewItem::wxDataViewItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a355faeb0fd910141a8621c34b884153c).
    pub fn new_with_dataviewitem<D: DataViewItemMethods>(item: &D) -> DataViewItemInRust<OWNED> {
        unsafe {
            let item = item.as_ptr();
            DataViewItemInRust(ffi::wxDataViewItem_new1(item))
        }
    }
    ///
    /// See [C++ `wxDataViewItem::wxDataViewItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#aa8c216134b8e17a742c070e39753be59).
    pub fn new_with_void(id: *mut c_void) -> DataViewItemInRust<OWNED> {
        unsafe { DataViewItemInRust(ffi::wxDataViewItem_new2(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewItemInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataViewItemInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewItem_delete(self.0) }
        }
    }
}

// wxDataViewItemAttr
wxwidgets! {
    /// This class is used to indicate to a wxDataViewCtrl that a certain item (see wxDataViewItem) has extra font attributes for its renderer.
    /// - [`DataViewItemAttr`] represents a C++ `wxDataViewItemAttr` class instance which your code has ownership, [`DataViewItemAttrInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewItemAttr`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewItemAttr` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html) for more details.
    #[doc(alias = "wxDataViewItemAttr")]
    #[doc(alias = "DataViewItemAttr")]
    class DataViewItemAttr
        = DataViewItemAttrInRust<true>(wxDataViewItemAttr) impl
        DataViewItemAttrMethods
}
impl<const OWNED: bool> DataViewItemAttrInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewItemAttr::wxDataViewItemAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a730aee09cf5b3d45db8dcae8ffc48fde).
    pub fn new() -> DataViewItemAttrInRust<OWNED> {
        unsafe { DataViewItemAttrInRust(ffi::wxDataViewItemAttr_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewItemAttrInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataViewItemAttrInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewItemAttr_delete(self.0) }
        }
    }
}

// wxDataViewListCtrl
wxwidgets! {
    /// This class is a wxDataViewCtrl which internally uses a wxDataViewListStore and forwards most of its API to that class.
    /// - [`DataViewListCtrl`] represents a C++ `wxDataViewListCtrl` class instance which your code has ownership, [`DataViewListCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewListCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewListCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html) for more details.
    #[doc(alias = "wxDataViewListCtrl")]
    #[doc(alias = "DataViewListCtrl")]
    class DataViewListCtrl
        = DataViewListCtrlInRust<true>(wxDataViewListCtrl) impl
        DataViewListCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewListCtrlInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxDataViewListCtrl::wxDataViewListCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#ae4ccb1d19929c49a81d3870a10d11765).
    pub fn new_2step() -> DataViewListCtrlInRust<OWNED> {
        unsafe { DataViewListCtrlInRust(ffi::wxDataViewListCtrl_new()) }
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
    ) -> DataViewListCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewListCtrlInRust(ffi::wxDataViewListCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DataViewListCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewListCtrlInRust<OWNED>> for DataViewCtrlInRust<OWNED> {
    fn from(o: DataViewListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: DataViewListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DataViewListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DataViewListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewListCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewListCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewListCtrl_CLASSINFO()) }
    }
}

// wxDataViewListModel
wxwidgets! {
    /// Base class with abstract API for wxDataViewIndexListModel and wxDataViewVirtualListModel.
    /// - [`DataViewListModel`] represents a C++ `wxDataViewListModel` class instance which your code has ownership, [`DataViewListModelInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewListModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewListModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html) for more details.
    #[doc(alias = "wxDataViewListModel")]
    #[doc(alias = "DataViewListModel")]
    class DataViewListModel
        = DataViewListModelInRust<true>(wxDataViewListModel) impl
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListModelInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewListModelInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewListModelInRust<OWNED>> for DataViewModelInRust<OWNED> {
    fn from(o: DataViewListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListModelInRust<OWNED>> for RefCounterInRust<OWNED> {
    fn from(o: DataViewListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewListModelInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewListModel_delete(self.0) }
        }
    }
}

// wxDataViewListStore
wxwidgets! {
    /// wxDataViewListStore is a specialised wxDataViewModel for storing a simple table of data.
    /// - [`DataViewListStore`] represents a C++ `wxDataViewListStore` class instance which your code has ownership, [`DataViewListStoreInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewListStore`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewListStore` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html) for more details.
    #[doc(alias = "wxDataViewListStore")]
    #[doc(alias = "DataViewListStore")]
    class DataViewListStore
        = DataViewListStoreInRust<true>(wxDataViewListStore) impl
        DataViewListStoreMethods,
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListStoreInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewListStore::wxDataViewListStore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#ad0ea04a252cfa338caca32b9bad11640).
    pub fn new() -> DataViewListStoreInRust<OWNED> {
        unsafe { DataViewListStoreInRust(ffi::wxDataViewListStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewListStoreInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewListStoreInRust<OWNED>>
    for DataViewIndexListModelInRust<OWNED>
{
    fn from(o: DataViewListStoreInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreInRust<OWNED>> for DataViewListModelInRust<OWNED> {
    fn from(o: DataViewListStoreInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreInRust<OWNED>> for DataViewModelInRust<OWNED> {
    fn from(o: DataViewListStoreInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreInRust<OWNED>> for RefCounterInRust<OWNED> {
    fn from(o: DataViewListStoreInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewListStoreInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewListStore_delete(self.0) }
        }
    }
}

// wxDataViewModel
wxwidgets! {
    /// wxDataViewModel is the base class for all data model to be displayed by a wxDataViewCtrl.
    /// - [`DataViewModel`] represents a C++ `wxDataViewModel` class instance which your code has ownership, [`DataViewModelInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html) for more details.
    #[doc(alias = "wxDataViewModel")]
    #[doc(alias = "DataViewModel")]
    class DataViewModel
        = DataViewModelInRust<true>(wxDataViewModel) impl
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewModelInRust<OWNED> {
    // BLOCKED: fn wxDataViewModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewModelInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewModelInRust<OWNED>> for RefCounterInRust<OWNED> {
    fn from(o: DataViewModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewModelInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewModel_delete(self.0) }
        }
    }
}

// wxDataViewModelNotifier
wxwidgets! {
    /// A wxDataViewModelNotifier instance is owned by a wxDataViewModel and mirrors its notification interface.
    /// - [`DataViewModelNotifier`] represents a C++ `wxDataViewModelNotifier` class instance which your code has ownership, [`DataViewModelNotifierInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewModelNotifier`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewModelNotifier` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html) for more details.
    #[doc(alias = "wxDataViewModelNotifier")]
    #[doc(alias = "DataViewModelNotifier")]
    class DataViewModelNotifier
        = DataViewModelNotifierInRust<true>(wxDataViewModelNotifier) impl
        DataViewModelNotifierMethods
}
impl<const OWNED: bool> DataViewModelNotifierInRust<OWNED> {
    // BLOCKED: fn wxDataViewModelNotifier()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewModelNotifierInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataViewModelNotifierInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewModelNotifier_delete(self.0) }
        }
    }
}

// wxDataViewProgressRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render progress bars.
    /// - [`DataViewProgressRenderer`] represents a C++ `wxDataViewProgressRenderer` class instance which your code has ownership, [`DataViewProgressRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewProgressRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewProgressRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_progress_renderer.html) for more details.
    #[doc(alias = "wxDataViewProgressRenderer")]
    #[doc(alias = "DataViewProgressRenderer")]
    class DataViewProgressRenderer
        = DataViewProgressRendererInRust<true>(wxDataViewProgressRenderer) impl
        DataViewProgressRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewProgressRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewProgressRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewProgressRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewProgressRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewProgressRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewProgressRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewProgressRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewProgressRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewProgressRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewProgressRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render the individual cells.
    /// - [`DataViewRenderer`] represents a C++ `wxDataViewRenderer` class instance which your code has ownership, [`DataViewRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html) for more details.
    #[doc(alias = "wxDataViewRenderer")]
    #[doc(alias = "DataViewRenderer")]
    class DataViewRenderer
        = DataViewRendererInRust<true>(wxDataViewRenderer) impl
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewSpinRenderer
wxwidgets! {
    /// This is a specialized renderer for rendering integer values.
    /// - [`DataViewSpinRenderer`] represents a C++ `wxDataViewSpinRenderer` class instance which your code has ownership, [`DataViewSpinRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewSpinRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewSpinRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_spin_renderer.html) for more details.
    #[doc(alias = "wxDataViewSpinRenderer")]
    #[doc(alias = "DataViewSpinRenderer")]
    class DataViewSpinRenderer
        = DataViewSpinRendererInRust<true>(wxDataViewSpinRenderer) impl
        DataViewSpinRendererMethods,
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewSpinRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewSpinRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewSpinRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererInRust<OWNED>>
    for DataViewCustomRendererInRust<OWNED>
{
    fn from(o: DataViewSpinRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererInRust<OWNED>> for DataViewRendererInRust<OWNED> {
    fn from(o: DataViewSpinRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewSpinRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewSpinRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewSpinRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewSpinRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTextRenderer
wxwidgets! {
    /// wxDataViewTextRenderer is used for rendering text.
    /// - [`DataViewTextRenderer`] represents a C++ `wxDataViewTextRenderer` class instance which your code has ownership, [`DataViewTextRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewTextRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewTextRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_text_renderer.html) for more details.
    #[doc(alias = "wxDataViewTextRenderer")]
    #[doc(alias = "DataViewTextRenderer")]
    class DataViewTextRenderer
        = DataViewTextRendererInRust<true>(wxDataViewTextRenderer) impl
        DataViewTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTextRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewTextRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewTextRendererInRust<OWNED>> for DataViewRendererInRust<OWNED> {
    fn from(o: DataViewTextRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTextRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewTextRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewTextRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewTextRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewToggleRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render toggle controls.
    /// - [`DataViewToggleRenderer`] represents a C++ `wxDataViewToggleRenderer` class instance which your code has ownership, [`DataViewToggleRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewToggleRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewToggleRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_toggle_renderer.html) for more details.
    #[doc(alias = "wxDataViewToggleRenderer")]
    #[doc(alias = "DataViewToggleRenderer")]
    class DataViewToggleRenderer
        = DataViewToggleRendererInRust<true>(wxDataViewToggleRenderer) impl
        DataViewToggleRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewToggleRendererInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewToggleRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewToggleRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewToggleRendererInRust<OWNED>>
    for DataViewRendererInRust<OWNED>
{
    fn from(o: DataViewToggleRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewToggleRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewToggleRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewToggleRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewToggleRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewToggleRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTreeCtrl
wxwidgets! {
    /// This class is a wxDataViewCtrl which internally uses a wxDataViewTreeStore and forwards most of its API to that class.
    /// - [`DataViewTreeCtrl`] represents a C++ `wxDataViewTreeCtrl` class instance which your code has ownership, [`DataViewTreeCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewTreeCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewTreeCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html) for more details.
    #[doc(alias = "wxDataViewTreeCtrl")]
    #[doc(alias = "DataViewTreeCtrl")]
    class DataViewTreeCtrl
        = DataViewTreeCtrlInRust<true>(wxDataViewTreeCtrl) impl
        DataViewTreeCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTreeCtrlInRust<OWNED> {
    /// Default ctor.
    ///
    /// See [C++ `wxDataViewTreeCtrl::wxDataViewTreeCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a49d8a55c826b9b3bb75b8607a0d94fe8).
    pub fn new_2step() -> DataViewTreeCtrlInRust<OWNED> {
        unsafe { DataViewTreeCtrlInRust(ffi::wxDataViewTreeCtrl_new()) }
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
    ) -> DataViewTreeCtrlInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewTreeCtrlInRust(ffi::wxDataViewTreeCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DataViewTreeCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlInRust<OWNED>> for DataViewCtrlInRust<OWNED> {
    fn from(o: DataViewTreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: DataViewTreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DataViewTreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DataViewTreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DataViewTreeCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewTreeCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDataViewTreeCtrl_CLASSINFO()) }
    }
}

// wxDataViewTreeStore
wxwidgets! {
    /// wxDataViewTreeStore is a specialised wxDataViewModel for storing simple trees very much like wxTreeCtrl does and it offers a similar API.
    /// - [`DataViewTreeStore`] represents a C++ `wxDataViewTreeStore` class instance which your code has ownership, [`DataViewTreeStoreInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewTreeStore`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewTreeStore` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html) for more details.
    #[doc(alias = "wxDataViewTreeStore")]
    #[doc(alias = "DataViewTreeStore")]
    class DataViewTreeStore
        = DataViewTreeStoreInRust<true>(wxDataViewTreeStore) impl
        DataViewTreeStoreMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewTreeStoreInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxDataViewTreeStore::wxDataViewTreeStore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#aad8493a851fbef80f8b7c3f368ca53db).
    pub fn new() -> DataViewTreeStoreInRust<OWNED> {
        unsafe { DataViewTreeStoreInRust(ffi::wxDataViewTreeStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewTreeStoreInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewTreeStoreInRust<OWNED>> for DataViewModelInRust<OWNED> {
    fn from(o: DataViewTreeStoreInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeStoreInRust<OWNED>> for RefCounterInRust<OWNED> {
    fn from(o: DataViewTreeStoreInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewTreeStoreInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewTreeStore_delete(self.0) }
        }
    }
}

// wxDataViewVirtualListModel
wxwidgets! {
    /// wxDataViewVirtualListModel is a specialized data model which lets you address an item by its position (row) rather than its wxDataViewItem and as such offers the exact same interface as wxDataViewIndexListModel.
    /// - [`DataViewVirtualListModel`] represents a C++ `wxDataViewVirtualListModel` class instance which your code has ownership, [`DataViewVirtualListModelInRust`]`<false>` represents one which don't own.
    /// - Use [`DataViewVirtualListModel`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDataViewVirtualListModel` class's documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html) for more details.
    #[doc(alias = "wxDataViewVirtualListModel")]
    #[doc(alias = "DataViewVirtualListModel")]
    class DataViewVirtualListModel
        = DataViewVirtualListModelInRust<true>(wxDataViewVirtualListModel) impl
        DataViewVirtualListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewVirtualListModelInRust<OWNED> {
    // BLOCKED: fn wxDataViewVirtualListModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewVirtualListModelInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelInRust<OWNED>>
    for DataViewListModelInRust<OWNED>
{
    fn from(o: DataViewVirtualListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelInRust<OWNED>> for DataViewModelInRust<OWNED> {
    fn from(o: DataViewVirtualListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelInRust<OWNED>> for RefCounterInRust<OWNED> {
    fn from(o: DataViewVirtualListModelInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewVirtualListModelInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewVirtualListModel_delete(self.0) }
        }
    }
}

// wxDateEvent
wxwidgets! {
    /// This event class holds information about a date change and is used together with wxDatePickerCtrl.
    /// - [`DateEvent`] represents a C++ `wxDateEvent` class instance which your code has ownership, [`DateEventInRust`]`<false>` represents one which don't own.
    /// - Use [`DateEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDateEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_date_event.html) for more details.
    #[doc(alias = "wxDateEvent")]
    #[doc(alias = "DateEvent")]
    class DateEvent
        = DateEventInRust<true>(wxDateEvent) impl
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DateEventInRust<OWNED> {
    ///
    /// See [C++ `wxDateEvent::wxDateEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_event.html#a54de582d97abba75950d2aa9b3ba84a2).
    pub fn new() -> DateEventInRust<OWNED> {
        unsafe { DateEventInRust(ffi::wxDateEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDateEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DateEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DateEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: DateEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: DateEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DateEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DateEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DateEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDatePickerCtrl
wxwidgets! {
    /// This control allows the user to select a date.
    /// - [`DatePickerCtrl`] represents a C++ `wxDatePickerCtrl` class instance which your code has ownership, [`DatePickerCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`DatePickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDatePickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html) for more details.
    #[doc(alias = "wxDatePickerCtrl")]
    #[doc(alias = "DatePickerCtrl")]
    class DatePickerCtrl
        = DatePickerCtrlInRust<true>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DatePickerCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxDatePickerCtrl::wxDatePickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#ae0700b0b3a7b522e3053e0bbb4de27ec).
    pub fn new_2step() -> DatePickerCtrlInRust<OWNED> {
        unsafe { DatePickerCtrlInRust(ffi::wxDatePickerCtrl_new()) }
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
    ) -> DatePickerCtrlInRust<OWNED> {
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
            DatePickerCtrlInRust(ffi::wxDatePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DatePickerCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DatePickerCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: DatePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DatePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DatePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DatePickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DatePickerCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDatePickerCtrl_CLASSINFO()) }
    }
}

// wxDelegateRendererNative
wxwidgets! {
    /// wxDelegateRendererNative allows reuse of renderers code by forwarding all the wxRendererNative methods to the given object and thus allowing you to only modify some of its methods  without having to reimplement all of them.
    /// - [`DelegateRendererNative`] represents a C++ `wxDelegateRendererNative` class instance which your code has ownership, [`DelegateRendererNativeInRust`]`<false>` represents one which don't own.
    /// - Use [`DelegateRendererNative`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDelegateRendererNative` class's documentation](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html) for more details.
    #[doc(alias = "wxDelegateRendererNative")]
    #[doc(alias = "DelegateRendererNative")]
    class DelegateRendererNative
        = DelegateRendererNativeInRust<true>(wxDelegateRendererNative) impl
        DelegateRendererNativeMethods,
        RendererNativeMethods
}
impl<const OWNED: bool> DelegateRendererNativeInRust<OWNED> {
    /// The default constructor does the same thing as the other one except that it uses the generic renderer instead of the user-specified rendererNative.
    ///
    /// See [C++ `wxDelegateRendererNative::wxDelegateRendererNative()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html#a184f122211b4632234a5398575305e2c).
    pub fn new() -> DelegateRendererNativeInRust<OWNED> {
        unsafe { DelegateRendererNativeInRust(ffi::wxDelegateRendererNative_new()) }
    }
    /// This constructor uses the user-specified rendererNative to set up the delegate renderer object to follow all calls to the specified real renderer.
    ///
    /// See [C++ `wxDelegateRendererNative::wxDelegateRendererNative()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html#a8d9dc80cc471a8162a4c9360790fb4cd).
    pub fn new_with_renderernative<R: RendererNativeMethods>(
        renderer_native: &R,
    ) -> DelegateRendererNativeInRust<OWNED> {
        unsafe {
            let renderer_native = renderer_native.as_ptr();
            DelegateRendererNativeInRust(ffi::wxDelegateRendererNative_new1(renderer_native))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DelegateRendererNativeInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DelegateRendererNativeInRust<OWNED>> for RendererNativeInRust<OWNED> {
    fn from(o: DelegateRendererNativeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DelegateRendererNativeInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDelegateRendererNative_delete(self.0) }
        }
    }
}

// wxDialog
wxwidgets! {
    /// A dialog box is a window with a title bar and sometimes a system menu, which can be moved around the screen.
    /// - [`Dialog`] represents a C++ `wxDialog` class instance which your code has ownership, [`DialogInRust`]`<false>` represents one which don't own.
    /// - Use [`Dialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html) for more details.
    #[doc(alias = "wxDialog")]
    #[doc(alias = "Dialog")]
    class Dialog
        = DialogInRust<true>(wxDialog) impl
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DialogInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxDialog::wxDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a11f9715b3975218071a4de9b29a4ed03).
    pub fn new_2step() -> DialogInRust<OWNED> {
        unsafe { DialogInRust(ffi::wxDialog_new()) }
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
    ) -> DialogInRust<OWNED> {
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
            DialogInRust(ffi::wxDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: DialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: DialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for DialogInRust<OWNED> {
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
impl<const OWNED: bool> WindowMethods for DialogInRust<OWNED> {
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
    /// - [`DialogLayoutAdapter`] represents a C++ `wxDialogLayoutAdapter` class instance which your code has ownership, [`DialogLayoutAdapterInRust`]`<false>` represents one which don't own.
    /// - Use [`DialogLayoutAdapter`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDialogLayoutAdapter` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dialog_layout_adapter.html) for more details.
    #[doc(alias = "wxDialogLayoutAdapter")]
    #[doc(alias = "DialogLayoutAdapter")]
    class DialogLayoutAdapter
        = DialogLayoutAdapterInRust<true>(wxDialogLayoutAdapter) impl
        DialogLayoutAdapterMethods
}
impl<const OWNED: bool> DialogLayoutAdapterInRust<OWNED> {
    // BLOCKED: fn wxDialogLayoutAdapter()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DialogLayoutAdapterInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DialogLayoutAdapterInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDialogLayoutAdapter_delete(self.0) }
        }
    }
}

// wxDirDialog
wxwidgets! {
    /// This class represents the directory chooser dialog.
    /// - [`DirDialog`] represents a C++ `wxDirDialog` class instance which your code has ownership, [`DirDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`DirDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDirDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html) for more details.
    #[doc(alias = "wxDirDialog")]
    #[doc(alias = "DirDialog")]
    class DirDialog
        = DirDialogInRust<true>(wxDirDialog) impl
        DirDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirDialogInRust<OWNED> {
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
    ) -> DirDialogInRust<OWNED> {
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
            DirDialogInRust(ffi::wxDirDialog_new(
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
impl<const OWNED: bool> Clone for DirDialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DirDialogInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: DirDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: DirDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: DirDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DirDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DirDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DirDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirDialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDirDialog_CLASSINFO()) }
    }
}

// wxDirPickerCtrl
wxwidgets! {
    /// This control allows the user to select a directory.
    /// - [`DirPickerCtrl`] represents a C++ `wxDirPickerCtrl` class instance which your code has ownership, [`DirPickerCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`DirPickerCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDirPickerCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html) for more details.
    #[doc(alias = "wxDirPickerCtrl")]
    #[doc(alias = "DirPickerCtrl")]
    class DirPickerCtrl
        = DirPickerCtrlInRust<true>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirPickerCtrlInRust<OWNED> {
    ///
    /// See [C++ `wxDirPickerCtrl::wxDirPickerCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a7afb789fa0326ec140f4645d49cc735b).
    pub fn new_2step() -> DirPickerCtrlInRust<OWNED> {
        unsafe { DirPickerCtrlInRust(ffi::wxDirPickerCtrl_new()) }
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
    ) -> DirPickerCtrlInRust<OWNED> {
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
            DirPickerCtrlInRust(ffi::wxDirPickerCtrl_new1(
                parent, id, path, message, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DirPickerCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DirPickerCtrlInRust<OWNED>> for PickerBaseInRust<OWNED> {
    fn from(o: DirPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: DirPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: DirPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: DirPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DirPickerCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirPickerCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDirPickerCtrl_CLASSINFO()) }
    }
}

// wxDisplay
wxwidgets! {
    /// Determines the sizes and locations of displays connected to the system.
    /// - [`Display`] represents a C++ `wxDisplay` class instance which your code has ownership, [`DisplayInRust`]`<false>` represents one which don't own.
    /// - Use [`Display`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDisplay` class's documentation](https://docs.wxwidgets.org/3.2/classwx_display.html) for more details.
    #[doc(alias = "wxDisplay")]
    #[doc(alias = "Display")]
    class Display
        = DisplayInRust<true>(wxDisplay) impl
        DisplayMethods
}
impl<const OWNED: bool> DisplayInRust<OWNED> {
    /// Default constructor creating wxDisplay object representing the primary display.
    ///
    /// See [C++ `wxDisplay::wxDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a8455f1450af24319a5c434d9a1984437).
    pub fn new() -> DisplayInRust<OWNED> {
        unsafe { DisplayInRust(ffi::wxDisplay_new()) }
    }
    /// Constructor, setting up a wxDisplay instance with the specified display.
    ///
    /// See [C++ `wxDisplay::wxDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a3ff5e051699a4f9ab6ce984d9a8a8943).
    pub fn new_with_uint(index: c_uint) -> DisplayInRust<OWNED> {
        unsafe { DisplayInRust(ffi::wxDisplay_new1(index)) }
    }
    /// Constructor creating the display object associated with the given window.
    ///
    /// See [C++ `wxDisplay::wxDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a75dc6e0f101a1126269bb0de990b8599).
    pub fn new_with_window<W: WindowMethods>(window: Option<&W>) -> DisplayInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DisplayInRust(ffi::wxDisplay_new2(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DisplayInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DisplayInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDisplay_delete(self.0) }
        }
    }
}

// wxDisplayChangedEvent
wxwidgets! {
    /// A display changed event is sent to top-level windows when the display resolution has changed.
    /// - [`DisplayChangedEvent`] represents a C++ `wxDisplayChangedEvent` class instance which your code has ownership, [`DisplayChangedEventInRust`]`<false>` represents one which don't own.
    /// - Use [`DisplayChangedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDisplayChangedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html) for more details.
    #[doc(alias = "wxDisplayChangedEvent")]
    #[doc(alias = "DisplayChangedEvent")]
    class DisplayChangedEvent
        = DisplayChangedEventInRust<true>(wxDisplayChangedEvent) impl
        DisplayChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DisplayChangedEventInRust<OWNED> {
    ///
    /// See [C++ `wxDisplayChangedEvent::wxDisplayChangedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html#aa9169eb3e0bbe259a738459f39a6eb1a).
    pub fn new() -> DisplayChangedEventInRust<OWNED> {
        unsafe { DisplayChangedEventInRust(ffi::wxDisplayChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DisplayChangedEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DisplayChangedEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: DisplayChangedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DisplayChangedEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DisplayChangedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DisplayChangedEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDisplayChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DisplayChangedEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDragImage
wxwidgets! {
    /// This class is used when you wish to drag an object on the screen, and a simple cursor is not enough.
    /// - [`DragImage`] represents a C++ `wxDragImage` class instance which your code has ownership, [`DragImageInRust`]`<false>` represents one which don't own.
    /// - Use [`DragImage`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDragImage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html) for more details.
    #[doc(alias = "wxDragImage")]
    #[doc(alias = "DragImage")]
    class DragImage
        = DragImageInRust<true>(wxDragImage) impl
        DragImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> DragImageInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1f9e5d1596679795bedb2aebe4841e6d).
    pub fn new() -> DragImageInRust<OWNED> {
        unsafe { DragImageInRust(ffi::wxDragImage_new()) }
    }
    /// Constructs a drag image from a bitmap and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1aabc326eb82c6aeda17b699b1d45ecb).
    pub fn new_with_bitmap<B: BitmapMethods, C: CursorMethods>(
        image: &B,
        cursor: &C,
    ) -> DragImageInRust<OWNED> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageInRust(ffi::wxDragImage_new1(image, cursor))
        }
    }
    /// Constructs a drag image from an icon and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a7d1efd4e364ffe45c22aff65b52d19f3).
    pub fn new_with_icon<I: IconMethods, C: CursorMethods>(
        image: &I,
        cursor: &C,
    ) -> DragImageInRust<OWNED> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageInRust(ffi::wxDragImage_new2(image, cursor))
        }
    }
    /// Constructs a drag image from a text string and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a594a13372b1c37b9a853d0817c17e9c0).
    pub fn new_with_str<C: CursorMethods>(text: &str, cursor: &C) -> DragImageInRust<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageInRust(ffi::wxDragImage_new3(text, cursor))
        }
    }
    /// Constructs a drag image from the text in the given tree control item, and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#afe1171ec9545ff5fbb6268501ed6ff60).
    pub fn new_with_treectrl<T: TreeCtrlMethods, T2: TreeItemIdMethods>(
        tree_ctrl: &T,
        id: &T2,
    ) -> DragImageInRust<OWNED> {
        unsafe {
            let tree_ctrl = tree_ctrl.as_ptr();
            let id = id.as_ptr();
            DragImageInRust(ffi::wxDragImage_new4(tree_ctrl, id))
        }
    }
    /// Constructs a drag image from the text in the given list control item, and optional cursor.
    ///
    /// See [C++ `wxDragImage::wxDragImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a24a75b4679d42593180e1e8b9d29c5a5).
    pub fn new_with_listctrl<L: ListCtrlMethods>(
        list_ctrl: &L,
        id: c_long,
    ) -> DragImageInRust<OWNED> {
        unsafe {
            let list_ctrl = list_ctrl.as_ptr();
            DragImageInRust(ffi::wxDragImage_new5(list_ctrl, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DragImageInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DragImageInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DragImageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DragImageInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDragImage_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DragImageInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropFilesEvent
wxwidgets! {
    /// This class is used for drop files events, that is, when files have been dropped onto the window.
    /// - [`DropFilesEvent`] represents a C++ `wxDropFilesEvent` class instance which your code has ownership, [`DropFilesEventInRust`]`<false>` represents one which don't own.
    /// - Use [`DropFilesEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDropFilesEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drop_files_event.html) for more details.
    #[doc(alias = "wxDropFilesEvent")]
    #[doc(alias = "DropFilesEvent")]
    class DropFilesEvent
        = DropFilesEventInRust<true>(wxDropFilesEvent) impl
        DropFilesEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DropFilesEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxDropFilesEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropFilesEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DropFilesEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: DropFilesEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DropFilesEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: DropFilesEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DropFilesEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxDropFilesEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DropFilesEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropSource
wxwidgets! {
    /// This class represents a source for a drag and drop operation.
    /// - [`DropSource`] represents a C++ `wxDropSource` class instance which your code has ownership, [`DropSourceInRust`]`<false>` represents one which don't own.
    /// - Use [`DropSource`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDropSource` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html) for more details.
    #[doc(alias = "wxDropSource")]
    #[doc(alias = "DropSource")]
    class DropSource
        = DropSourceInRust<true>(wxDropSource) impl
        DropSourceMethods
}
impl<const OWNED: bool> DropSourceInRust<OWNED> {
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
    ) -> DropSourceInRust<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceInRust(ffi::wxDropSource_new(win, icon_copy, icon_move, icon_none))
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
    ) -> DropSourceInRust<OWNED> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceInRust(ffi::wxDropSource_new1(
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
    ) -> DropSourceInRust<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceInRust(ffi::wxDropSource_new2(win, icon_copy, icon_move, icon_none))
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
    ) -> DropSourceInRust<OWNED> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceInRust(ffi::wxDropSource_new3(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropSourceInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DropSourceInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDropSource_delete(self.0) }
        }
    }
}

// wxDropTarget
wxwidgets! {
    /// This class represents a target for a drag and drop operation.
    /// - [`DropTarget`] represents a C++ `wxDropTarget` class instance which your code has ownership, [`DropTargetInRust`]`<false>` represents one which don't own.
    /// - Use [`DropTarget`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxDropTarget` class's documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html) for more details.
    #[doc(alias = "wxDropTarget")]
    #[doc(alias = "DropTarget")]
    class DropTarget
        = DropTargetInRust<true>(wxDropTarget) impl
        DropTargetMethods
}
impl<const OWNED: bool> DropTargetInRust<OWNED> {
    // BLOCKED: fn wxDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropTargetInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DropTargetInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDropTarget_delete(self.0) }
        }
    }
}
