use super::*;

// wxDC
wxwidgets! {
    /// A wxDC is a "device context" onto which graphics and text can be drawn.
    ///
    /// [See `wxDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c.html)
    #[doc(alias = "wxDC")]
    #[doc(alias = "DC")]
    class DC
        = DCIsOwned<true>(wxDC) impl
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> DCIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDCBrushChanger
wxwidgets! {
    /// wxDCBrushChanger is a small helper class for setting a brush on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    ///
    /// [See `wxDCBrushChanger`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html)
    #[doc(alias = "wxDCBrushChanger")]
    #[doc(alias = "DCBrushChanger")]
    class DCBrushChanger
        = DCBrushChangerIsOwned<true>(wxDCBrushChanger) impl
        DCBrushChangerMethods
}
impl<const OWNED: bool> DCBrushChangerIsOwned<OWNED> {
    /// Sets brush on the given dc, storing the old one.
    ///
    /// [See `wxDCBrushChanger::wxDCBrushChanger()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html#a047d2af137cb4e6860b3911ab46e4909)
    pub fn new<D: DCMethods, B: BrushMethods>(dc: &D, brush: &B) -> DCBrushChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let brush = brush.as_ptr();
            DCBrushChangerIsOwned(ffi::wxDCBrushChanger_new(dc, brush))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCBrushChangerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCBrushChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCBrushChanger_delete(self.0) }
        }
    }
}

// wxDCClipper
wxwidgets! {
    /// wxDCClipper is a helper class for setting a clipping region on a wxDC during its lifetime.
    ///
    /// [See `wxDCClipper`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html)
    #[doc(alias = "wxDCClipper")]
    #[doc(alias = "DCClipper")]
    class DCClipper
        = DCClipperIsOwned<true>(wxDCClipper) impl
        DCClipperMethods
}
impl<const OWNED: bool> DCClipperIsOwned<OWNED> {
    /// Sets the clipping region to the specified region/coordinates.
    ///
    /// [See `wxDCClipper::wxDCClipper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#aa6548fa7be5cff3a74f6a6f539b00adf)
    pub fn new_with_region<D: DCMethods, R: RegionMethods>(
        dc: &D,
        region: &R,
    ) -> DCClipperIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let region = region.as_ptr();
            DCClipperIsOwned(ffi::wxDCClipper_new(dc, region))
        }
    }
    ///
    /// [See `wxDCClipper::wxDCClipper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#a995a8e0147459e1ba92cbb965fd963a4)
    pub fn new_with_rect<D: DCMethods, R: RectMethods>(
        dc: &D,
        rect: &R,
    ) -> DCClipperIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            DCClipperIsOwned(ffi::wxDCClipper_new1(dc, rect))
        }
    }
    ///
    /// [See `wxDCClipper::wxDCClipper()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html#a2096fc82c7ab658fcca0a65650ddeb80)
    pub fn new_with_coord<D: DCMethods>(
        dc: &D,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    ) -> DCClipperIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCClipperIsOwned(ffi::wxDCClipper_new2(dc, x, y, w, h))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCClipperIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCClipperIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCClipper_delete(self.0) }
        }
    }
}

// wxDCFontChanger
wxwidgets! {
    /// wxDCFontChanger is a small helper class for setting a font on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    ///
    /// [See `wxDCFontChanger`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html)
    #[doc(alias = "wxDCFontChanger")]
    #[doc(alias = "DCFontChanger")]
    class DCFontChanger
        = DCFontChangerIsOwned<true>(wxDCFontChanger) impl
        DCFontChangerMethods
}
impl<const OWNED: bool> DCFontChangerIsOwned<OWNED> {
    /// Trivial constructor not changing anything.
    ///
    /// [See `wxDCFontChanger::wxDCFontChanger()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#aa4bd96e01e9099f10f9394ef9b69b069)
    pub fn new<D: DCMethods>(dc: &D) -> DCFontChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCFontChangerIsOwned(ffi::wxDCFontChanger_new(dc))
        }
    }
    /// Sets font on the given dc, storing the old one.
    ///
    /// [See `wxDCFontChanger::wxDCFontChanger()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#a3c5c5fe626322d365cbd3f90928eeaa2)
    pub fn new_with_font<D: DCMethods, F: FontMethods>(
        dc: &D,
        font: &F,
    ) -> DCFontChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let font = font.as_ptr();
            DCFontChangerIsOwned(ffi::wxDCFontChanger_new1(dc, font))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCFontChangerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCFontChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCFontChanger_delete(self.0) }
        }
    }
}

// wxDCOverlay
wxwidgets! {
    /// Connects an overlay with a drawing DC.
    ///
    /// [See `wxDCOverlay`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html)
    #[doc(alias = "wxDCOverlay")]
    #[doc(alias = "DCOverlay")]
    class DCOverlay
        = DCOverlayIsOwned<true>(wxDCOverlay) impl
        DCOverlayMethods
}
impl<const OWNED: bool> DCOverlayIsOwned<OWNED> {
    /// Connects this overlay to the corresponding drawing dc, if the overlay is not initialized yet this call will do so.
    ///
    /// [See `wxDCOverlay::wxDCOverlay()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html#aeffcb68537d705a07c00adeb008aa64e)
    pub fn new_with_int<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> DCOverlayIsOwned<OWNED> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayIsOwned(ffi::wxDCOverlay_new(overlay, dc, x, y, width, height))
        }
    }
    /// Convenience wrapper that behaves the same using the entire area of the dc.
    ///
    /// [See `wxDCOverlay::wxDCOverlay()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html#a45d692f25022296a11389480c651e13b)
    pub fn new<O: OverlayMethods, D: DCMethods>(
        overlay: &O,
        dc: Option<&D>,
    ) -> DCOverlayIsOwned<OWNED> {
        unsafe {
            let overlay = overlay.as_ptr();
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DCOverlayIsOwned(ffi::wxDCOverlay_new1(overlay, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCOverlayIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCOverlayIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCOverlay_delete(self.0) }
        }
    }
}

// wxDCPenChanger
wxwidgets! {
    /// wxDCPenChanger is a small helper class for setting a pen on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    ///
    /// [See `wxDCPenChanger`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html)
    #[doc(alias = "wxDCPenChanger")]
    #[doc(alias = "DCPenChanger")]
    class DCPenChanger
        = DCPenChangerIsOwned<true>(wxDCPenChanger) impl
        DCPenChangerMethods
}
impl<const OWNED: bool> DCPenChangerIsOwned<OWNED> {
    /// Sets pen on the given dc, storing the old one.
    ///
    /// [See `wxDCPenChanger::wxDCPenChanger()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html#abefe06367f53d64e35aeb203537e50e3)
    pub fn new<D: DCMethods, P: PenMethods>(dc: &D, pen: &P) -> DCPenChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let pen = pen.as_ptr();
            DCPenChangerIsOwned(ffi::wxDCPenChanger_new(dc, pen))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCPenChangerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCPenChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCPenChanger_delete(self.0) }
        }
    }
}

// wxDCTextColourChanger
wxwidgets! {
    /// wxDCTextColourChanger is a small helper class for setting a foreground text colour on a wxDC and unsetting it automatically in the destructor, restoring the previous one.
    ///
    /// [See `wxDCTextColourChanger`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html)
    #[doc(alias = "wxDCTextColourChanger")]
    #[doc(alias = "DCTextColourChanger")]
    class DCTextColourChanger
        = DCTextColourChangerIsOwned<true>(wxDCTextColourChanger) impl
        DCTextColourChangerMethods
}
impl<const OWNED: bool> DCTextColourChangerIsOwned<OWNED> {
    /// Trivial constructor not changing anything.
    ///
    /// [See `wxDCTextColourChanger::wxDCTextColourChanger()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#ae9c21638cef0ad69be36a7359811965d)
    pub fn new<D: DCMethods>(dc: &D) -> DCTextColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCTextColourChangerIsOwned(ffi::wxDCTextColourChanger_new(dc))
        }
    }
    /// Sets col on the given dc, storing the old one.
    ///
    /// [See `wxDCTextColourChanger::wxDCTextColourChanger()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#a0c0cc00023f4edd806220ac147e40784)
    pub fn new_with_colour<D: DCMethods, C: ColourMethods>(
        dc: &D,
        col: &C,
    ) -> DCTextColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            let col = col.as_ptr();
            DCTextColourChangerIsOwned(ffi::wxDCTextColourChanger_new1(dc, col))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DCTextColourChangerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DCTextColourChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDCTextColourChanger_delete(self.0) }
        }
    }
}

// wxDataFormat
wxwidgets! {
    /// A wxDataFormat is an encapsulation of a platform-specific format handle which is used by the system for the clipboard and drag and drop operations.
    ///
    /// [See `wxDataFormat`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_format.html)
    #[doc(alias = "wxDataFormat")]
    #[doc(alias = "DataFormat")]
    class DataFormat
        = DataFormatIsOwned<true>(wxDataFormat) impl
        DataFormatMethods
}
impl<const OWNED: bool> DataFormatIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataFormat()
    /// Constructs a data format object for a custom format identified by its name format.
    ///
    /// [See `wxDataFormat::wxDataFormat()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_format.html#a6c08911611be5e3a5dd35528b4d091db)
    pub fn new(format: &str) -> DataFormatIsOwned<OWNED> {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            DataFormatIsOwned(ffi::wxDataFormat_new1(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataFormatIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataFormatIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataFormat_delete(self.0) }
        }
    }
}

// wxDataObject
wxwidgets! {
    /// A wxDataObject represents data that can be copied to or from the clipboard, or dragged and dropped.
    ///
    /// [See `wxDataObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_object.html)
    #[doc(alias = "wxDataObject")]
    #[doc(alias = "DataObject")]
    class DataObject
        = DataObjectIsOwned<true>(wxDataObject) impl
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectIsOwned<OWNED> {
    //  ENUM: Direction
    pub const Get: c_int = 0x01;
    pub const Set: c_int = 0x02;
    pub const Both: c_int = 0x03;

    // BLOCKED: fn wxDataObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObject_delete(self.0) }
        }
    }
}

// wxDataObjectComposite
wxwidgets! {
    /// wxDataObjectComposite is the simplest wxDataObject derivation which may be used to support multiple formats.
    ///
    /// [See `wxDataObjectComposite`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html)
    #[doc(alias = "wxDataObjectComposite")]
    #[doc(alias = "DataObjectComposite")]
    class DataObjectComposite
        = DataObjectCompositeIsOwned<true>(wxDataObjectComposite) impl
        DataObjectCompositeMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectCompositeIsOwned<OWNED> {
    /// The default constructor.
    ///
    /// [See `wxDataObjectComposite::wxDataObjectComposite()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html#a711cfefddb7e091d56f87be3b2d0bcb8)
    pub fn new() -> DataObjectCompositeIsOwned<OWNED> {
        unsafe { DataObjectCompositeIsOwned(ffi::wxDataObjectComposite_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectCompositeIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataObjectCompositeIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: DataObjectCompositeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectCompositeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectComposite_delete(self.0) }
        }
    }
}

// wxDataObjectSimple
wxwidgets! {
    /// This is the simplest possible implementation of the wxDataObject class.
    ///
    /// [See `wxDataObjectSimple`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html)
    #[doc(alias = "wxDataObjectSimple")]
    #[doc(alias = "DataObjectSimple")]
    class DataObjectSimple
        = DataObjectSimpleIsOwned<true>(wxDataObjectSimple) impl
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectSimpleIsOwned<OWNED> {
    /// Constructor accepts the supported format (none by default) which may also be set later with SetFormat().
    ///
    /// [See `wxDataObjectSimple::wxDataObjectSimple()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#ad246b285dd2f414f4b13a4d794bf602d)
    pub fn new<D: DataFormatMethods>(format: &D) -> DataObjectSimpleIsOwned<OWNED> {
        unsafe {
            let format = format.as_ptr();
            DataObjectSimpleIsOwned(ffi::wxDataObjectSimple_new(format))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataObjectSimpleIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataObjectSimpleIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: DataObjectSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectSimple_delete(self.0) }
        }
    }
}

// wxDataViewBitmapRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render bitmaps.
    ///
    /// [See `wxDataViewBitmapRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_bitmap_renderer.html)
    #[doc(alias = "wxDataViewBitmapRenderer")]
    #[doc(alias = "DataViewBitmapRenderer")]
    class DataViewBitmapRenderer
        = DataViewBitmapRendererIsOwned<true>(wxDataViewBitmapRenderer) impl
        DataViewBitmapRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewBitmapRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewBitmapRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewBitmapRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewBitmapRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewBitmapRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewBitmapRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewBitmapRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewBitmapRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewBitmapRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewBitmapRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceByIndexRenderer
wxwidgets! {
    /// A wxDataViewCtrl renderer using wxChoice control and indexes into it.
    ///
    /// [See `wxDataViewChoiceByIndexRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_by_index_renderer.html)
    #[doc(alias = "wxDataViewChoiceByIndexRenderer")]
    #[doc(alias = "DataViewChoiceByIndexRenderer")]
    class DataViewChoiceByIndexRenderer
        = DataViewChoiceByIndexRendererIsOwned<true>(wxDataViewChoiceByIndexRenderer) impl
        DataViewChoiceByIndexRendererMethods,
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewChoiceByIndexRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewChoiceByIndexRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewChoiceByIndexRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererIsOwned<OWNED>>
    for DataViewChoiceRendererIsOwned<OWNED>
{
    fn from(o: DataViewChoiceByIndexRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewChoiceByIndexRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceByIndexRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewChoiceByIndexRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewChoiceByIndexRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewChoiceByIndexRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewChoiceByIndexRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewChoiceRenderer
wxwidgets! {
    /// A wxDataViewCtrl renderer using wxChoice control and values of strings in it.
    ///
    /// [See `wxDataViewChoiceRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_renderer.html)
    #[doc(alias = "wxDataViewChoiceRenderer")]
    #[doc(alias = "DataViewChoiceRenderer")]
    class DataViewChoiceRenderer
        = DataViewChoiceRendererIsOwned<true>(wxDataViewChoiceRenderer) impl
        DataViewChoiceRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewChoiceRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewChoiceRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewChoiceRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewChoiceRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewChoiceRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewChoiceRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewChoiceRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewChoiceRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewChoiceRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewChoiceRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewColumn
wxwidgets! {
    /// This class represents a column in a wxDataViewCtrl.
    ///
    /// [See `wxDataViewColumn`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html)
    #[doc(alias = "wxDataViewColumn")]
    #[doc(alias = "DataViewColumn")]
    class DataViewColumn
        = DataViewColumnIsOwned<true>(wxDataViewColumn) impl
        DataViewColumnMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> DataViewColumnIsOwned<OWNED> {
    /// Constructs a text column.
    ///
    /// [See `wxDataViewColumn::wxDataViewColumn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#aa56f4df8543bf14713a2852af471c768)
    pub fn new_with_str<D: DataViewRendererMethods>(
        title: &str,
        renderer: Option<&D>,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> DataViewColumnIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnIsOwned(ffi::wxDataViewColumn_new(
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
    /// [See `wxDataViewColumn::wxDataViewColumn()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#aa3f5c1c707bd95b39a9c74d281e32f6b)
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods, D: DataViewRendererMethods>(
        bitmap: &B,
        renderer: Option<&D>,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> DataViewColumnIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let renderer = match renderer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewColumnIsOwned(ffi::wxDataViewColumn_new1(
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
impl Clone for DataViewColumnIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewColumnIsOwned<OWNED>> for SettableHeaderColumnIsOwned<OWNED> {
    fn from(o: DataViewColumnIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewColumnIsOwned<OWNED>> for HeaderColumnIsOwned<OWNED> {
    fn from(o: DataViewColumnIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewColumn_delete(self.0) }
        }
    }
}

// wxDataViewCtrl
wxwidgets! {
    /// wxDataViewCtrl is a control to display data either in a tree like fashion or in a tabular form or both.
    ///
    /// [See `wxDataViewCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html)
    #[doc(alias = "wxDataViewCtrl")]
    #[doc(alias = "DataViewCtrl")]
    class DataViewCtrl
        = DataViewCtrlIsOwned<true>(wxDataViewCtrl) impl
        DataViewCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCtrlIsOwned<OWNED> {
    /// Default Constructor.
    ///
    /// [See `wxDataViewCtrl::wxDataViewCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a3c912686a7c04b53445e2e1985685a3f)
    pub fn new_2step() -> DataViewCtrlIsOwned<OWNED> {
        unsafe { DataViewCtrlIsOwned(ffi::wxDataViewCtrl_new()) }
    }
    /// Constructor.
    ///
    /// [See `wxDataViewCtrl::wxDataViewCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a5cc833d3d27d13f5dae7bd2062a55189)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DataViewCtrlIsOwned<OWNED> {
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
            DataViewCtrlIsOwned(ffi::wxDataViewCtrl_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DataViewCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for DataViewCtrlIsOwned<OWNED> {
    /// Create the control.
    ///
    /// [See `wxDataViewCtrl::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a1bd86d5869de4d24de791a48e9c6926e)
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
    ///
    /// [See `wxDataViewCustomRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html)
    #[doc(alias = "wxDataViewCustomRenderer")]
    #[doc(alias = "DataViewCustomRenderer")]
    class DataViewCustomRenderer
        = DataViewCustomRendererIsOwned<true>(wxDataViewCustomRenderer) impl
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCustomRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewCustomRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewCustomRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewCustomRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewCustomRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewCustomRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewCustomRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewCustomRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewCustomRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewCustomRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewDateRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render calendar controls.
    ///
    /// [See `wxDataViewDateRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_date_renderer.html)
    #[doc(alias = "wxDataViewDateRenderer")]
    #[doc(alias = "DataViewDateRenderer")]
    class DataViewDateRenderer
        = DataViewDateRendererIsOwned<true>(wxDataViewDateRenderer) impl
        DataViewDateRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewDateRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewDateRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewDateRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewDateRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewDateRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewDateRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewDateRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewDateRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewDateRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewEvent
wxwidgets! {
    /// This is the event class for the wxDataViewCtrl notifications.
    ///
    /// [See `wxDataViewEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html)
    #[doc(alias = "wxDataViewEvent")]
    #[doc(alias = "DataViewEvent")]
    class DataViewEvent
        = DataViewEventIsOwned<true>(wxDataViewEvent) impl
        DataViewEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewEventIsOwned<OWNED> {
    /// Default ctor, normally shouldn't be used and mostly exists only for backwards compatibility.
    ///
    /// [See `wxDataViewEvent::wxDataViewEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#ae6dde6781192716c6c7ee9f828a2a99d)
    pub fn new() -> DataViewEventIsOwned<OWNED> {
        unsafe { DataViewEventIsOwned(ffi::wxDataViewEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDataViewEvent1()
    // NOT_SUPPORTED: fn wxDataViewEvent2()
    /// Copy constructor.
    ///
    /// [See `wxDataViewEvent::wxDataViewEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a5960c6568e8407e54958e7492859ff68)
    pub fn new_with_dataviewevent<D: DataViewEventMethods>(
        event: &D,
    ) -> DataViewEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            DataViewEventIsOwned(ffi::wxDataViewEvent_new3(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconText
wxwidgets! {
    /// wxDataViewIconText is used by wxDataViewIconTextRenderer for data transfer.
    ///
    /// [See `wxDataViewIconText`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html)
    #[doc(alias = "wxDataViewIconText")]
    #[doc(alias = "DataViewIconText")]
    class DataViewIconText
        = DataViewIconTextIsOwned<true>(wxDataViewIconText) impl
        DataViewIconTextMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxDataViewIconText::wxDataViewIconText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a1de5295b0774784c21a4d5d694df4725)
    pub fn new_with_str<B: BitmapBundleMethods>(
        text: &str,
        bitmap: &B,
    ) -> DataViewIconTextIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let bitmap = bitmap.as_ptr();
            DataViewIconTextIsOwned(ffi::wxDataViewIconText_new(text, bitmap))
        }
    }
    ///
    /// [See `wxDataViewIconText::wxDataViewIconText()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#aa32e3db38e83550e99367f88965be72c)
    pub fn new_with_dataviewicontext<D: DataViewIconTextMethods>(
        other: &D,
    ) -> DataViewIconTextIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            DataViewIconTextIsOwned(ffi::wxDataViewIconText_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIconTextIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewIconTextIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewIconTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewIconTextIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewIconText_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIconTextIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIconTextRenderer
wxwidgets! {
    /// The wxDataViewIconTextRenderer class is used to display text with a small icon next to it as it is typically done in a file manager.
    ///
    /// [See `wxDataViewIconTextRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text_renderer.html)
    #[doc(alias = "wxDataViewIconTextRenderer")]
    #[doc(alias = "DataViewIconTextRenderer")]
    class DataViewIconTextRenderer
        = DataViewIconTextRendererIsOwned<true>(wxDataViewIconTextRenderer) impl
        DataViewIconTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewIconTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIconTextRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewIconTextRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewIconTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIconTextRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewIconTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewIconTextRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewIconTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIconTextRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewIndexListModel
wxwidgets! {
    /// wxDataViewIndexListModel is a specialized data model which lets you address an item by its position (row) rather than its wxDataViewItem (which you can obtain from this class).
    ///
    /// [See `wxDataViewIndexListModel`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html)
    #[doc(alias = "wxDataViewIndexListModel")]
    #[doc(alias = "DataViewIndexListModel")]
    class DataViewIndexListModel
        = DataViewIndexListModelIsOwned<true>(wxDataViewIndexListModel) impl
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewIndexListModelIsOwned<OWNED> {
    // BLOCKED: fn wxDataViewIndexListModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewIndexListModelIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelIsOwned<OWNED>>
    for DataViewListModelIsOwned<OWNED>
{
    fn from(o: DataViewIndexListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewIndexListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewIndexListModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewIndexListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewIndexListModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewIndexListModel_delete(self.0) }
        }
    }
}

// wxDataViewItem
wxwidgets! {
    /// wxDataViewItem is a small opaque class that represents an item in a wxDataViewCtrl in a persistent way, i.e.
    ///
    /// [See `wxDataViewItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html)
    #[doc(alias = "wxDataViewItem")]
    #[doc(alias = "DataViewItem")]
    class DataViewItem
        = DataViewItemIsOwned<true>(wxDataViewItem) impl
        DataViewItemMethods
}
impl<const OWNED: bool> DataViewItemIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxDataViewItem::wxDataViewItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a7a8b5a738467b471cd10e96357dc800e)
    pub fn new() -> DataViewItemIsOwned<OWNED> {
        unsafe { DataViewItemIsOwned(ffi::wxDataViewItem_new()) }
    }
    ///
    /// [See `wxDataViewItem::wxDataViewItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a355faeb0fd910141a8621c34b884153c)
    pub fn new_with_dataviewitem<D: DataViewItemMethods>(item: &D) -> DataViewItemIsOwned<OWNED> {
        unsafe {
            let item = item.as_ptr();
            DataViewItemIsOwned(ffi::wxDataViewItem_new1(item))
        }
    }
    ///
    /// [See `wxDataViewItem::wxDataViewItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#aa8c216134b8e17a742c070e39753be59)
    pub fn new_with_void(id: *mut c_void) -> DataViewItemIsOwned<OWNED> {
        unsafe { DataViewItemIsOwned(ffi::wxDataViewItem_new2(id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewItemIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataViewItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewItem_delete(self.0) }
        }
    }
}

// wxDataViewItemAttr
wxwidgets! {
    /// This class is used to indicate to a wxDataViewCtrl that a certain item (see wxDataViewItem) has extra font attributes for its renderer.
    ///
    /// [See `wxDataViewItemAttr`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html)
    #[doc(alias = "wxDataViewItemAttr")]
    #[doc(alias = "DataViewItemAttr")]
    class DataViewItemAttr
        = DataViewItemAttrIsOwned<true>(wxDataViewItemAttr) impl
        DataViewItemAttrMethods
}
impl<const OWNED: bool> DataViewItemAttrIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxDataViewItemAttr::wxDataViewItemAttr()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a730aee09cf5b3d45db8dcae8ffc48fde)
    pub fn new() -> DataViewItemAttrIsOwned<OWNED> {
        unsafe { DataViewItemAttrIsOwned(ffi::wxDataViewItemAttr_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewItemAttrIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataViewItemAttrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewItemAttr_delete(self.0) }
        }
    }
}

// wxDataViewListCtrl
wxwidgets! {
    /// This class is a wxDataViewCtrl which internally uses a wxDataViewListStore and forwards most of its API to that class.
    ///
    /// [See `wxDataViewListCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html)
    #[doc(alias = "wxDataViewListCtrl")]
    #[doc(alias = "DataViewListCtrl")]
    class DataViewListCtrl
        = DataViewListCtrlIsOwned<true>(wxDataViewListCtrl) impl
        DataViewListCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewListCtrlIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// [See `wxDataViewListCtrl::wxDataViewListCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#ae4ccb1d19929c49a81d3870a10d11765)
    pub fn new_2step() -> DataViewListCtrlIsOwned<OWNED> {
        unsafe { DataViewListCtrlIsOwned(ffi::wxDataViewListCtrl_new()) }
    }
    /// Constructor.
    ///
    /// [See `wxDataViewListCtrl::wxDataViewListCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#af7a2515b02e8b5e4aa27cc1831c3686b)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> DataViewListCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewListCtrlIsOwned(ffi::wxDataViewListCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DataViewListCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for DataViewCtrlIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewListCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewListCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewListCtrl_CLASSINFO()) }
    }
}

// wxDataViewListModel
wxwidgets! {
    /// Base class with abstract API for wxDataViewIndexListModel and wxDataViewVirtualListModel.
    ///
    /// [See `wxDataViewListModel`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html)
    #[doc(alias = "wxDataViewListModel")]
    #[doc(alias = "DataViewListModel")]
    class DataViewListModel
        = DataViewListModelIsOwned<true>(wxDataViewListModel) impl
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListModelIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewListModelIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewListModelIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewListModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewListModel_delete(self.0) }
        }
    }
}

// wxDataViewListStore
wxwidgets! {
    /// wxDataViewListStore is a specialised wxDataViewModel for storing a simple table of data.
    ///
    /// [See `wxDataViewListStore`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html)
    #[doc(alias = "wxDataViewListStore")]
    #[doc(alias = "DataViewListStore")]
    class DataViewListStore
        = DataViewListStoreIsOwned<true>(wxDataViewListStore) impl
        DataViewListStoreMethods,
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListStoreIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxDataViewListStore::wxDataViewListStore()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#ad0ea04a252cfa338caca32b9bad11640)
    pub fn new() -> DataViewListStoreIsOwned<OWNED> {
        unsafe { DataViewListStoreIsOwned(ffi::wxDataViewListStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewListStoreIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>>
    for DataViewIndexListModelIsOwned<OWNED>
{
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>> for DataViewListModelIsOwned<OWNED> {
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewListStoreIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewListStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewListStoreIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewListStore_delete(self.0) }
        }
    }
}

// wxDataViewModel
wxwidgets! {
    /// wxDataViewModel is the base class for all data model to be displayed by a wxDataViewCtrl.
    ///
    /// [See `wxDataViewModel`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html)
    #[doc(alias = "wxDataViewModel")]
    #[doc(alias = "DataViewModel")]
    class DataViewModel
        = DataViewModelIsOwned<true>(wxDataViewModel) impl
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewModelIsOwned<OWNED> {
    // BLOCKED: fn wxDataViewModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewModelIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewModel_delete(self.0) }
        }
    }
}

// wxDataViewModelNotifier
wxwidgets! {
    /// A wxDataViewModelNotifier instance is owned by a wxDataViewModel and mirrors its notification interface.
    ///
    /// [See `wxDataViewModelNotifier`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html)
    #[doc(alias = "wxDataViewModelNotifier")]
    #[doc(alias = "DataViewModelNotifier")]
    class DataViewModelNotifier
        = DataViewModelNotifierIsOwned<true>(wxDataViewModelNotifier) impl
        DataViewModelNotifierMethods
}
impl<const OWNED: bool> DataViewModelNotifierIsOwned<OWNED> {
    // BLOCKED: fn wxDataViewModelNotifier()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewModelNotifierIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DataViewModelNotifierIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewModelNotifier_delete(self.0) }
        }
    }
}

// wxDataViewProgressRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render progress bars.
    ///
    /// [See `wxDataViewProgressRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_progress_renderer.html)
    #[doc(alias = "wxDataViewProgressRenderer")]
    #[doc(alias = "DataViewProgressRenderer")]
    class DataViewProgressRenderer
        = DataViewProgressRendererIsOwned<true>(wxDataViewProgressRenderer) impl
        DataViewProgressRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewProgressRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewProgressRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewProgressRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewProgressRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewProgressRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewProgressRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewProgressRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewProgressRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewProgressRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewProgressRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render the individual cells.
    ///
    /// [See `wxDataViewRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html)
    #[doc(alias = "wxDataViewRenderer")]
    #[doc(alias = "DataViewRenderer")]
    class DataViewRenderer
        = DataViewRendererIsOwned<true>(wxDataViewRenderer) impl
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewSpinRenderer
wxwidgets! {
    /// This is a specialized renderer for rendering integer values.
    ///
    /// [See `wxDataViewSpinRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_spin_renderer.html)
    #[doc(alias = "wxDataViewSpinRenderer")]
    #[doc(alias = "DataViewSpinRenderer")]
    class DataViewSpinRenderer
        = DataViewSpinRendererIsOwned<true>(wxDataViewSpinRenderer) impl
        DataViewSpinRendererMethods,
        DataViewCustomRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewSpinRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewSpinRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewSpinRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererIsOwned<OWNED>>
    for DataViewCustomRendererIsOwned<OWNED>
{
    fn from(o: DataViewSpinRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewSpinRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewSpinRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewSpinRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewSpinRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewSpinRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewSpinRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTextRenderer
wxwidgets! {
    /// wxDataViewTextRenderer is used for rendering text.
    ///
    /// [See `wxDataViewTextRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_text_renderer.html)
    #[doc(alias = "wxDataViewTextRenderer")]
    #[doc(alias = "DataViewTextRenderer")]
    class DataViewTextRenderer
        = DataViewTextRendererIsOwned<true>(wxDataViewTextRenderer) impl
        DataViewTextRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTextRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewTextRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewTextRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewTextRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTextRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewTextRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewTextRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewTextRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewTextRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewToggleRenderer
wxwidgets! {
    /// This class is used by wxDataViewCtrl to render toggle controls.
    ///
    /// [See `wxDataViewToggleRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_toggle_renderer.html)
    #[doc(alias = "wxDataViewToggleRenderer")]
    #[doc(alias = "DataViewToggleRenderer")]
    class DataViewToggleRenderer
        = DataViewToggleRendererIsOwned<true>(wxDataViewToggleRenderer) impl
        DataViewToggleRendererMethods,
        DataViewRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewToggleRendererIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataViewToggleRenderer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewToggleRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewToggleRendererIsOwned<OWNED>>
    for DataViewRendererIsOwned<OWNED>
{
    fn from(o: DataViewToggleRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewToggleRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewToggleRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewToggleRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewToggleRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DataViewToggleRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDataViewTreeCtrl
wxwidgets! {
    /// This class is a wxDataViewCtrl which internally uses a wxDataViewTreeStore and forwards most of its API to that class.
    ///
    /// [See `wxDataViewTreeCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html)
    #[doc(alias = "wxDataViewTreeCtrl")]
    #[doc(alias = "DataViewTreeCtrl")]
    class DataViewTreeCtrl
        = DataViewTreeCtrlIsOwned<true>(wxDataViewTreeCtrl) impl
        DataViewTreeCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTreeCtrlIsOwned<OWNED> {
    /// Default ctor.
    ///
    /// [See `wxDataViewTreeCtrl::wxDataViewTreeCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a49d8a55c826b9b3bb75b8607a0d94fe8)
    pub fn new_2step() -> DataViewTreeCtrlIsOwned<OWNED> {
        unsafe { DataViewTreeCtrlIsOwned(ffi::wxDataViewTreeCtrl_new()) }
    }
    /// Constructor.
    ///
    /// [See `wxDataViewTreeCtrl::wxDataViewTreeCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a263a8abb605575c5ab9db5eba259cf89)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> DataViewTreeCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            DataViewTreeCtrlIsOwned(ffi::wxDataViewTreeCtrl_new1(
                parent, id, pos, size, style, validator,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DataViewTreeCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for DataViewCtrlIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DataViewTreeCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DataViewTreeCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDataViewTreeCtrl_CLASSINFO()) }
    }
}

// wxDataViewTreeStore
wxwidgets! {
    /// wxDataViewTreeStore is a specialised wxDataViewModel for storing simple trees very much like wxTreeCtrl does and it offers a similar API.
    ///
    /// [See `wxDataViewTreeStore`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html)
    #[doc(alias = "wxDataViewTreeStore")]
    #[doc(alias = "DataViewTreeStore")]
    class DataViewTreeStore
        = DataViewTreeStoreIsOwned<true>(wxDataViewTreeStore) impl
        DataViewTreeStoreMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewTreeStoreIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxDataViewTreeStore::wxDataViewTreeStore()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#aad8493a851fbef80f8b7c3f368ca53db)
    pub fn new() -> DataViewTreeStoreIsOwned<OWNED> {
        unsafe { DataViewTreeStoreIsOwned(ffi::wxDataViewTreeStore_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewTreeStoreIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewTreeStoreIsOwned<OWNED>> for DataViewModelIsOwned<OWNED> {
    fn from(o: DataViewTreeStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewTreeStoreIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewTreeStoreIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewTreeStoreIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewTreeStore_delete(self.0) }
        }
    }
}

// wxDataViewVirtualListModel
wxwidgets! {
    /// wxDataViewVirtualListModel is a specialized data model which lets you address an item by its position (row) rather than its wxDataViewItem and as such offers the exact same interface as wxDataViewIndexListModel.
    ///
    /// [See `wxDataViewVirtualListModel`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html)
    #[doc(alias = "wxDataViewVirtualListModel")]
    #[doc(alias = "DataViewVirtualListModel")]
    class DataViewVirtualListModel
        = DataViewVirtualListModelIsOwned<true>(wxDataViewVirtualListModel) impl
        DataViewVirtualListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewVirtualListModelIsOwned<OWNED> {
    // BLOCKED: fn wxDataViewVirtualListModel()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DataViewVirtualListModelIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelIsOwned<OWNED>>
    for DataViewListModelIsOwned<OWNED>
{
    fn from(o: DataViewVirtualListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelIsOwned<OWNED>>
    for DataViewModelIsOwned<OWNED>
{
    fn from(o: DataViewVirtualListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DataViewVirtualListModelIsOwned<OWNED>> for RefCounterIsOwned<OWNED> {
    fn from(o: DataViewVirtualListModelIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataViewVirtualListModelIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataViewVirtualListModel_delete(self.0) }
        }
    }
}

// wxDateEvent
wxwidgets! {
    /// This event class holds information about a date change and is used together with wxDatePickerCtrl.
    ///
    /// [See `wxDateEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_date_event.html)
    #[doc(alias = "wxDateEvent")]
    #[doc(alias = "DateEvent")]
    class DateEvent
        = DateEventIsOwned<true>(wxDateEvent) impl
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DateEventIsOwned<OWNED> {
    ///
    /// [See `wxDateEvent::wxDateEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_date_event.html#a54de582d97abba75950d2aa9b3ba84a2)
    pub fn new() -> DateEventIsOwned<OWNED> {
        unsafe { DateEventIsOwned(ffi::wxDateEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDateEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DateEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DateEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DateEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDatePickerCtrl
wxwidgets! {
    /// This control allows the user to select a date.
    ///
    /// [See `wxDatePickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html)
    #[doc(alias = "wxDatePickerCtrl")]
    #[doc(alias = "DatePickerCtrl")]
    class DatePickerCtrl
        = DatePickerCtrlIsOwned<true>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DatePickerCtrlIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxDatePickerCtrl::wxDatePickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#ae0700b0b3a7b522e3053e0bbb4de27ec)
    pub fn new_2step() -> DatePickerCtrlIsOwned<OWNED> {
        unsafe { DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// [See `wxDatePickerCtrl::wxDatePickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a697230c98830fca84021f14a697f156c)
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
    ) -> DatePickerCtrlIsOwned<OWNED> {
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
            DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DatePickerCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DatePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDatePickerCtrl_CLASSINFO()) }
    }
}

// wxDelegateRendererNative
wxwidgets! {
    /// wxDelegateRendererNative allows reuse of renderers code by forwarding all the wxRendererNative methods to the given object and thus allowing you to only modify some of its methods  without having to reimplement all of them.
    ///
    /// [See `wxDelegateRendererNative`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html)
    #[doc(alias = "wxDelegateRendererNative")]
    #[doc(alias = "DelegateRendererNative")]
    class DelegateRendererNative
        = DelegateRendererNativeIsOwned<true>(wxDelegateRendererNative) impl
        DelegateRendererNativeMethods,
        RendererNativeMethods
}
impl<const OWNED: bool> DelegateRendererNativeIsOwned<OWNED> {
    /// The default constructor does the same thing as the other one except that it uses the generic renderer instead of the user-specified rendererNative.
    ///
    /// [See `wxDelegateRendererNative::wxDelegateRendererNative()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html#a184f122211b4632234a5398575305e2c)
    pub fn new() -> DelegateRendererNativeIsOwned<OWNED> {
        unsafe { DelegateRendererNativeIsOwned(ffi::wxDelegateRendererNative_new()) }
    }
    /// This constructor uses the user-specified rendererNative to set up the delegate renderer object to follow all calls to the specified real renderer.
    ///
    /// [See `wxDelegateRendererNative::wxDelegateRendererNative()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html#a8d9dc80cc471a8162a4c9360790fb4cd)
    pub fn new_with_renderernative<R: RendererNativeMethods>(
        renderer_native: &R,
    ) -> DelegateRendererNativeIsOwned<OWNED> {
        unsafe {
            let renderer_native = renderer_native.as_ptr();
            DelegateRendererNativeIsOwned(ffi::wxDelegateRendererNative_new1(renderer_native))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DelegateRendererNativeIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DelegateRendererNativeIsOwned<OWNED>>
    for RendererNativeIsOwned<OWNED>
{
    fn from(o: DelegateRendererNativeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DelegateRendererNativeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDelegateRendererNative_delete(self.0) }
        }
    }
}

// wxDialog
wxwidgets! {
    /// A dialog box is a window with a title bar and sometimes a system menu, which can be moved around the screen.
    ///
    /// [See `wxDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog.html)
    #[doc(alias = "wxDialog")]
    #[doc(alias = "Dialog")]
    class Dialog
        = DialogIsOwned<true>(wxDialog) impl
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DialogIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxDialog::wxDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a11f9715b3975218071a4de9b29a4ed03)
    pub fn new_2step() -> DialogIsOwned<OWNED> {
        unsafe { DialogIsOwned(ffi::wxDialog_new()) }
    }
    /// Constructor.
    ///
    /// [See `wxDialog::wxDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a0bbd20a18b306aad59429b9d6783d39b)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DialogIsOwned<OWNED> {
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
            DialogIsOwned(ffi::wxDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for DialogIsOwned<OWNED> {
    /// Used for two-step dialog box construction.
    ///
    /// [See `wxDialog::Create()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a44e42338cb8bd2a1b312ab7a6f1722a3)
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
    /// [See `wxDialog::SetIcon()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a717435f3dd9d977feaa40fb359a6da84)
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDialog_SetIcon(self.as_ptr(), icon)
        }
    }
}
impl<const OWNED: bool> WindowMethods for DialogIsOwned<OWNED> {
    /// Centres the dialog box on the display.
    ///
    /// [See `wxDialog::Centre()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a6af384c4a558965bfee61784f5e0b7fc)
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxDialog_Centre(self.as_ptr(), direction) }
    }
}

// wxDialogLayoutAdapter
wxwidgets! {
    /// This abstract class is the base for classes that help wxWidgets perform run-time layout adaptation of dialogs.
    ///
    /// [See `wxDialogLayoutAdapter`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dialog_layout_adapter.html)
    #[doc(alias = "wxDialogLayoutAdapter")]
    #[doc(alias = "DialogLayoutAdapter")]
    class DialogLayoutAdapter
        = DialogLayoutAdapterIsOwned<true>(wxDialogLayoutAdapter) impl
        DialogLayoutAdapterMethods
}
impl<const OWNED: bool> DialogLayoutAdapterIsOwned<OWNED> {
    // BLOCKED: fn wxDialogLayoutAdapter()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DialogLayoutAdapterIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DialogLayoutAdapterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDialogLayoutAdapter_delete(self.0) }
        }
    }
}

// wxDirDialog
wxwidgets! {
    /// This class represents the directory chooser dialog.
    ///
    /// [See `wxDirDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html)
    #[doc(alias = "wxDirDialog")]
    #[doc(alias = "DirDialog")]
    class DirDialog
        = DirDialogIsOwned<true>(wxDirDialog) impl
        DirDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirDialogIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxDirDialog::wxDirDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#a72322832d7830dd637fb4daa541c267a)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        message: &str,
        default_path: &str,
        style: c_long,
        pos: &P,
        size: &S,
        name: &str,
    ) -> DirDialogIsOwned<OWNED> {
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
            DirDialogIsOwned(ffi::wxDirDialog_new(
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
impl<const OWNED: bool> Clone for DirDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DirDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDirDialog_CLASSINFO()) }
    }
}

// wxDirPickerCtrl
wxwidgets! {
    /// This control allows the user to select a directory.
    ///
    /// [See `wxDirPickerCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html)
    #[doc(alias = "wxDirPickerCtrl")]
    #[doc(alias = "DirPickerCtrl")]
    class DirPickerCtrl
        = DirPickerCtrlIsOwned<true>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirPickerCtrlIsOwned<OWNED> {
    ///
    /// [See `wxDirPickerCtrl::wxDirPickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a7afb789fa0326ec140f4645d49cc735b)
    pub fn new_2step() -> DirPickerCtrlIsOwned<OWNED> {
        unsafe { DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new()) }
    }
    /// Initializes the object and calls Create() with all the parameters.
    ///
    /// [See `wxDirPickerCtrl::wxDirPickerCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a6a8c66f10082401f7445c5660c3b6d79)
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
    ) -> DirPickerCtrlIsOwned<OWNED> {
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
            DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new1(
                parent, id, path, message, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for DirPickerCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDirPickerCtrl_CLASSINFO()) }
    }
}

// wxDisplay
wxwidgets! {
    /// Determines the sizes and locations of displays connected to the system.
    ///
    /// [See `wxDisplay`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_display.html)
    #[doc(alias = "wxDisplay")]
    #[doc(alias = "Display")]
    class Display
        = DisplayIsOwned<true>(wxDisplay) impl
        DisplayMethods
}
impl<const OWNED: bool> DisplayIsOwned<OWNED> {
    /// Default constructor creating wxDisplay object representing the primary display.
    ///
    /// [See `wxDisplay::wxDisplay()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_display.html#a8455f1450af24319a5c434d9a1984437)
    pub fn new() -> DisplayIsOwned<OWNED> {
        unsafe { DisplayIsOwned(ffi::wxDisplay_new()) }
    }
    /// Constructor, setting up a wxDisplay instance with the specified display.
    ///
    /// [See `wxDisplay::wxDisplay()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_display.html#a3ff5e051699a4f9ab6ce984d9a8a8943)
    pub fn new_with_uint(index: c_uint) -> DisplayIsOwned<OWNED> {
        unsafe { DisplayIsOwned(ffi::wxDisplay_new1(index)) }
    }
    /// Constructor creating the display object associated with the given window.
    ///
    /// [See `wxDisplay::wxDisplay()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_display.html#a75dc6e0f101a1126269bb0de990b8599)
    pub fn new_with_window<W: WindowMethods>(window: Option<&W>) -> DisplayIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DisplayIsOwned(ffi::wxDisplay_new2(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DisplayIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DisplayIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDisplay_delete(self.0) }
        }
    }
}

// wxDisplayChangedEvent
wxwidgets! {
    /// A display changed event is sent to top-level windows when the display resolution has changed.
    ///
    /// [See `wxDisplayChangedEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html)
    #[doc(alias = "wxDisplayChangedEvent")]
    #[doc(alias = "DisplayChangedEvent")]
    class DisplayChangedEvent
        = DisplayChangedEventIsOwned<true>(wxDisplayChangedEvent) impl
        DisplayChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DisplayChangedEventIsOwned<OWNED> {
    ///
    /// [See `wxDisplayChangedEvent::wxDisplayChangedEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html#aa9169eb3e0bbe259a738459f39a6eb1a)
    pub fn new() -> DisplayChangedEventIsOwned<OWNED> {
        unsafe { DisplayChangedEventIsOwned(ffi::wxDisplayChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DisplayChangedEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DisplayChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DisplayChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DisplayChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DisplayChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DisplayChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDisplayChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DisplayChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDragImage
wxwidgets! {
    /// This class is used when you wish to drag an object on the screen, and a simple cursor is not enough.
    ///
    /// [See `wxDragImage`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html)
    #[doc(alias = "wxDragImage")]
    #[doc(alias = "DragImage")]
    class DragImage
        = DragImageIsOwned<true>(wxDragImage) impl
        DragImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> DragImageIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxDragImage::wxDragImage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1f9e5d1596679795bedb2aebe4841e6d)
    pub fn new() -> DragImageIsOwned<OWNED> {
        unsafe { DragImageIsOwned(ffi::wxDragImage_new()) }
    }
    /// Constructs a drag image from a bitmap and optional cursor.
    ///
    /// [See `wxDragImage::wxDragImage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1aabc326eb82c6aeda17b699b1d45ecb)
    pub fn new_with_bitmap<B: BitmapMethods, C: CursorMethods>(
        image: &B,
        cursor: &C,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new1(image, cursor))
        }
    }
    /// Constructs a drag image from an icon and optional cursor.
    ///
    /// [See `wxDragImage::wxDragImage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a7d1efd4e364ffe45c22aff65b52d19f3)
    pub fn new_with_icon<I: IconMethods, C: CursorMethods>(
        image: &I,
        cursor: &C,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let image = image.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new2(image, cursor))
        }
    }
    /// Constructs a drag image from a text string and optional cursor.
    ///
    /// [See `wxDragImage::wxDragImage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a594a13372b1c37b9a853d0817c17e9c0)
    pub fn new_with_str<C: CursorMethods>(text: &str, cursor: &C) -> DragImageIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new3(text, cursor))
        }
    }
    /// Constructs a drag image from the text in the given tree control item, and optional cursor.
    ///
    /// [See `wxDragImage::wxDragImage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#afe1171ec9545ff5fbb6268501ed6ff60)
    pub fn new_with_treectrl<T: TreeCtrlMethods, T2: TreeItemIdMethods>(
        tree_ctrl: &T,
        id: &T2,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let tree_ctrl = tree_ctrl.as_ptr();
            let id = id.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new4(tree_ctrl, id))
        }
    }
    /// Constructs a drag image from the text in the given list control item, and optional cursor.
    ///
    /// [See `wxDragImage::wxDragImage()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a24a75b4679d42593180e1e8b9d29c5a5)
    pub fn new_with_listctrl<L: ListCtrlMethods>(
        list_ctrl: &L,
        id: c_long,
    ) -> DragImageIsOwned<OWNED> {
        unsafe {
            let list_ctrl = list_ctrl.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new5(list_ctrl, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DragImageIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DragImageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DragImageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DragImageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDragImage_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DragImageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropFilesEvent
wxwidgets! {
    /// This class is used for drop files events, that is, when files have been dropped onto the window.
    ///
    /// [See `wxDropFilesEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_files_event.html)
    #[doc(alias = "wxDropFilesEvent")]
    #[doc(alias = "DropFilesEvent")]
    class DropFilesEvent
        = DropFilesEventIsOwned<true>(wxDropFilesEvent) impl
        DropFilesEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DropFilesEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDropFilesEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropFilesEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<DropFilesEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DropFilesEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DropFilesEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DropFilesEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DropFilesEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDropFilesEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DropFilesEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDropSource
wxwidgets! {
    /// This class represents a source for a drag and drop operation.
    ///
    /// [See `wxDropSource`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_source.html)
    #[doc(alias = "wxDropSource")]
    #[doc(alias = "DropSource")]
    class DropSource
        = DropSourceIsOwned<true>(wxDropSource) impl
        DropSourceMethods
}
impl<const OWNED: bool> DropSourceIsOwned<OWNED> {
    /// This constructor requires that you must call SetData() later.
    ///
    /// [See `wxDropSource::wxDropSource()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a0534684be549317b7a14ce27d5afc699)
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
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new(win, icon_copy, icon_move, icon_none))
        }
    }
    /// The constructor taking a wxDataObject.
    ///
    /// [See `wxDropSource::wxDropSource()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#afd4966dacaa526f6d2e5708d91136da7)
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
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new1(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    /// This constructor requires that you must call SetData() later.
    ///
    /// [See `wxDropSource::wxDropSource()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a06dbcfe97a0615d59c97d0be7d11f6e5)
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
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new2(win, icon_copy, icon_move, icon_none))
        }
    }
    /// The constructor taking a wxDataObject.
    ///
    /// [See `wxDropSource::wxDropSource()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a867fea285027d6625acacfcb799833b1)
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
    ) -> DropSourceIsOwned<OWNED> {
        unsafe {
            let data = data.as_ptr();
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let icon_copy = icon_copy.as_ptr();
            let icon_move = icon_move.as_ptr();
            let icon_none = icon_none.as_ptr();
            DropSourceIsOwned(ffi::wxDropSource_new3(
                data, win, icon_copy, icon_move, icon_none,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropSourceIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DropSourceIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDropSource_delete(self.0) }
        }
    }
}

// wxDropTarget
wxwidgets! {
    /// This class represents a target for a drag and drop operation.
    ///
    /// [See `wxDropTarget`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_drop_target.html)
    #[doc(alias = "wxDropTarget")]
    #[doc(alias = "DropTarget")]
    class DropTarget
        = DropTargetIsOwned<true>(wxDropTarget) impl
        DropTargetMethods
}
impl<const OWNED: bool> DropTargetIsOwned<OWNED> {
    // BLOCKED: fn wxDropTarget()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DropTargetIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for DropTargetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDropTarget_delete(self.0) }
        }
    }
}
