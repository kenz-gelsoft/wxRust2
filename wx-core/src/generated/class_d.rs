use super::*;

// wxDC
wx_class! { DC =
    DCIsOwned<true>(wxDC) impl
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
wx_class! { DCBrushChanger =
    DCBrushChangerIsOwned<true>(wxDCBrushChanger) impl
        DCBrushChangerMethods
}
impl<const OWNED: bool> DCBrushChangerIsOwned<OWNED> {
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
wx_class! { DCClipper =
    DCClipperIsOwned<true>(wxDCClipper) impl
        DCClipperMethods
}
impl<const OWNED: bool> DCClipperIsOwned<OWNED> {
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
wx_class! { DCFontChanger =
    DCFontChangerIsOwned<true>(wxDCFontChanger) impl
        DCFontChangerMethods
}
impl<const OWNED: bool> DCFontChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D) -> DCFontChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCFontChangerIsOwned(ffi::wxDCFontChanger_new(dc))
        }
    }
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
wx_class! { DCOverlay =
    DCOverlayIsOwned<true>(wxDCOverlay) impl
        DCOverlayMethods
}
impl<const OWNED: bool> DCOverlayIsOwned<OWNED> {
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
wx_class! { DCPenChanger =
    DCPenChangerIsOwned<true>(wxDCPenChanger) impl
        DCPenChangerMethods
}
impl<const OWNED: bool> DCPenChangerIsOwned<OWNED> {
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
wx_class! { DCTextColourChanger =
    DCTextColourChangerIsOwned<true>(wxDCTextColourChanger) impl
        DCTextColourChangerMethods
}
impl<const OWNED: bool> DCTextColourChangerIsOwned<OWNED> {
    pub fn new<D: DCMethods>(dc: &D) -> DCTextColourChangerIsOwned<OWNED> {
        unsafe {
            let dc = dc.as_ptr();
            DCTextColourChangerIsOwned(ffi::wxDCTextColourChanger_new(dc))
        }
    }
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
wx_class! { DataFormat =
    DataFormatIsOwned<true>(wxDataFormat) impl
        DataFormatMethods
}
impl<const OWNED: bool> DataFormatIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxDataFormat()
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
wx_class! { DataObject =
    DataObjectIsOwned<true>(wxDataObject) impl
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
wx_class! { DataObjectComposite =
    DataObjectCompositeIsOwned<true>(wxDataObjectComposite) impl
        DataObjectCompositeMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectCompositeIsOwned<OWNED> {
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
wx_class! { DataObjectSimple =
    DataObjectSimpleIsOwned<true>(wxDataObjectSimple) impl
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectSimpleIsOwned<OWNED> {
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
wx_class! { DataViewBitmapRenderer =
    DataViewBitmapRendererIsOwned<true>(wxDataViewBitmapRenderer) impl
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
wx_class! { DataViewChoiceByIndexRenderer =
    DataViewChoiceByIndexRendererIsOwned<true>(wxDataViewChoiceByIndexRenderer) impl
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
wx_class! { DataViewChoiceRenderer =
    DataViewChoiceRendererIsOwned<true>(wxDataViewChoiceRenderer) impl
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
wx_class! { DataViewColumn =
    DataViewColumnIsOwned<true>(wxDataViewColumn) impl
        DataViewColumnMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> DataViewColumnIsOwned<OWNED> {
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
wx_class! { DataViewCtrl =
    DataViewCtrlIsOwned<true>(wxDataViewCtrl) impl
        DataViewCtrlMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DataViewCtrlIsOwned<OWNED> {
        unsafe { DataViewCtrlIsOwned(ffi::wxDataViewCtrl_new()) }
    }
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
impl<const OWNED: bool> Trackable<DataViewCtrlIsOwned<false>> for DataViewCtrlIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<DataViewCtrlIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}
impl<const OWNED: bool> ControlMethods for DataViewCtrlIsOwned<OWNED> {
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
wx_class! { DataViewCustomRenderer =
    DataViewCustomRendererIsOwned<true>(wxDataViewCustomRenderer) impl
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
wx_class! { DataViewDateRenderer =
    DataViewDateRendererIsOwned<true>(wxDataViewDateRenderer) impl
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
wx_class! { DataViewEvent =
    DataViewEventIsOwned<true>(wxDataViewEvent) impl
        DataViewEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewEventIsOwned<OWNED> {
    pub fn new() -> DataViewEventIsOwned<OWNED> {
        unsafe { DataViewEventIsOwned(ffi::wxDataViewEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDataViewEvent1()
    // NOT_SUPPORTED: fn wxDataViewEvent2()
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
wx_class! { DataViewIconText =
    DataViewIconTextIsOwned<true>(wxDataViewIconText) impl
        DataViewIconTextMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewIconTextIsOwned<OWNED> {
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
wx_class! { DataViewIconTextRenderer =
    DataViewIconTextRendererIsOwned<true>(wxDataViewIconTextRenderer) impl
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
wx_class! { DataViewIndexListModel =
    DataViewIndexListModelIsOwned<true>(wxDataViewIndexListModel) impl
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
wx_class! { DataViewItem =
    DataViewItemIsOwned<true>(wxDataViewItem) impl
        DataViewItemMethods
}
impl<const OWNED: bool> DataViewItemIsOwned<OWNED> {
    pub fn new() -> DataViewItemIsOwned<OWNED> {
        unsafe { DataViewItemIsOwned(ffi::wxDataViewItem_new()) }
    }
    pub fn new_with_dataviewitem<D: DataViewItemMethods>(item: &D) -> DataViewItemIsOwned<OWNED> {
        unsafe {
            let item = item.as_ptr();
            DataViewItemIsOwned(ffi::wxDataViewItem_new1(item))
        }
    }
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
wx_class! { DataViewItemAttr =
    DataViewItemAttrIsOwned<true>(wxDataViewItemAttr) impl
        DataViewItemAttrMethods
}
impl<const OWNED: bool> DataViewItemAttrIsOwned<OWNED> {
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
wx_class! { DataViewListCtrl =
    DataViewListCtrlIsOwned<true>(wxDataViewListCtrl) impl
        DataViewListCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewListCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DataViewListCtrlIsOwned<OWNED> {
        unsafe { DataViewListCtrlIsOwned(ffi::wxDataViewListCtrl_new()) }
    }
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
impl<const OWNED: bool> Trackable<DataViewListCtrlIsOwned<false>>
    for DataViewListCtrlIsOwned<OWNED>
{
    fn to_weak_ref(&self) -> WeakRef<DataViewListCtrlIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxDataViewListModel
wx_class! { DataViewListModel =
    DataViewListModelIsOwned<true>(wxDataViewListModel) impl
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
wx_class! { DataViewListStore =
    DataViewListStoreIsOwned<true>(wxDataViewListStore) impl
        DataViewListStoreMethods,
        DataViewIndexListModelMethods,
        DataViewListModelMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewListStoreIsOwned<OWNED> {
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
wx_class! { DataViewModel =
    DataViewModelIsOwned<true>(wxDataViewModel) impl
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
wx_class! { DataViewModelNotifier =
    DataViewModelNotifierIsOwned<true>(wxDataViewModelNotifier) impl
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
wx_class! { DataViewProgressRenderer =
    DataViewProgressRendererIsOwned<true>(wxDataViewProgressRenderer) impl
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
wx_class! { DataViewRenderer =
    DataViewRendererIsOwned<true>(wxDataViewRenderer) impl
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
wx_class! { DataViewSpinRenderer =
    DataViewSpinRendererIsOwned<true>(wxDataViewSpinRenderer) impl
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
wx_class! { DataViewTextRenderer =
    DataViewTextRendererIsOwned<true>(wxDataViewTextRenderer) impl
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
wx_class! { DataViewToggleRenderer =
    DataViewToggleRendererIsOwned<true>(wxDataViewToggleRenderer) impl
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
wx_class! { DataViewTreeCtrl =
    DataViewTreeCtrlIsOwned<true>(wxDataViewTreeCtrl) impl
        DataViewTreeCtrlMethods,
        DataViewCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DataViewTreeCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DataViewTreeCtrlIsOwned<OWNED> {
        unsafe { DataViewTreeCtrlIsOwned(ffi::wxDataViewTreeCtrl_new()) }
    }
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
impl<const OWNED: bool> Trackable<DataViewTreeCtrlIsOwned<false>>
    for DataViewTreeCtrlIsOwned<OWNED>
{
    fn to_weak_ref(&self) -> WeakRef<DataViewTreeCtrlIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxDataViewTreeStore
wx_class! { DataViewTreeStore =
    DataViewTreeStoreIsOwned<true>(wxDataViewTreeStore) impl
        DataViewTreeStoreMethods,
        DataViewModelMethods,
        RefCounterMethods
}
impl<const OWNED: bool> DataViewTreeStoreIsOwned<OWNED> {
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
wx_class! { DataViewVirtualListModel =
    DataViewVirtualListModelIsOwned<true>(wxDataViewVirtualListModel) impl
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
wx_class! { DateEvent =
    DateEventIsOwned<true>(wxDateEvent) impl
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DateEventIsOwned<OWNED> {
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
wx_class! { DatePickerCtrl =
    DatePickerCtrlIsOwned<true>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DatePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DatePickerCtrlIsOwned<OWNED> {
        unsafe { DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new()) }
    }
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
impl<const OWNED: bool> Trackable<DatePickerCtrlIsOwned<false>> for DatePickerCtrlIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<DatePickerCtrlIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxDelegateRendererNative
wx_class! { DelegateRendererNative =
    DelegateRendererNativeIsOwned<true>(wxDelegateRendererNative) impl
        DelegateRendererNativeMethods,
        RendererNativeMethods
}
impl<const OWNED: bool> DelegateRendererNativeIsOwned<OWNED> {
    pub fn new() -> DelegateRendererNativeIsOwned<OWNED> {
        unsafe { DelegateRendererNativeIsOwned(ffi::wxDelegateRendererNative_new()) }
    }
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
wx_class! { Dialog =
    DialogIsOwned<true>(wxDialog) impl
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DialogIsOwned<OWNED> {
    pub fn new_2step() -> DialogIsOwned<OWNED> {
        unsafe { DialogIsOwned(ffi::wxDialog_new()) }
    }
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
impl<const OWNED: bool> Trackable<DialogIsOwned<false>> for DialogIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<DialogIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for DialogIsOwned<OWNED> {
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
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDialog_SetIcon(self.as_ptr(), icon)
        }
    }
}
impl<const OWNED: bool> WindowMethods for DialogIsOwned<OWNED> {
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxDialog_Centre(self.as_ptr(), direction) }
    }
}

// wxDialogLayoutAdapter
wx_class! { DialogLayoutAdapter =
    DialogLayoutAdapterIsOwned<true>(wxDialogLayoutAdapter) impl
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
wx_class! { DirDialog =
    DirDialogIsOwned<true>(wxDirDialog) impl
        DirDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirDialogIsOwned<OWNED> {
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
impl<const OWNED: bool> Trackable<DirDialogIsOwned<false>> for DirDialogIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<DirDialogIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxDirPickerCtrl
wx_class! { DirPickerCtrl =
    DirPickerCtrlIsOwned<true>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DirPickerCtrlIsOwned<OWNED> {
        unsafe { DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new()) }
    }
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
impl<const OWNED: bool> Trackable<DirPickerCtrlIsOwned<false>> for DirPickerCtrlIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<DirPickerCtrlIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxDisplay
wx_class! { Display =
    DisplayIsOwned<true>(wxDisplay) impl
        DisplayMethods
}
impl<const OWNED: bool> DisplayIsOwned<OWNED> {
    pub fn new() -> DisplayIsOwned<OWNED> {
        unsafe { DisplayIsOwned(ffi::wxDisplay_new()) }
    }
    pub fn new_with_uint(index: c_uint) -> DisplayIsOwned<OWNED> {
        unsafe { DisplayIsOwned(ffi::wxDisplay_new1(index)) }
    }
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
wx_class! { DisplayChangedEvent =
    DisplayChangedEventIsOwned<true>(wxDisplayChangedEvent) impl
        DisplayChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DisplayChangedEventIsOwned<OWNED> {
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
wx_class! { DragImage =
    DragImageIsOwned<true>(wxDragImage) impl
        DragImageMethods,
        ObjectMethods
}
impl<const OWNED: bool> DragImageIsOwned<OWNED> {
    pub fn new() -> DragImageIsOwned<OWNED> {
        unsafe { DragImageIsOwned(ffi::wxDragImage_new()) }
    }
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
    pub fn new_with_str<C: CursorMethods>(text: &str, cursor: &C) -> DragImageIsOwned<OWNED> {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let cursor = cursor.as_ptr();
            DragImageIsOwned(ffi::wxDragImage_new3(text, cursor))
        }
    }
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
wx_class! { DropFilesEvent =
    DropFilesEventIsOwned<true>(wxDropFilesEvent) impl
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
wx_class! { DropSource =
    DropSourceIsOwned<true>(wxDropSource) impl
        DropSourceMethods
}
impl<const OWNED: bool> DropSourceIsOwned<OWNED> {
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
wx_class! { DropTarget =
    DropTargetIsOwned<true>(wxDropTarget) impl
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
