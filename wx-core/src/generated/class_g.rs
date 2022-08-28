use super::*;

// wxGBPosition
wxwidgets! {
    /// This class represents the position of an item in a virtual grid of rows and columns managed by a wxGridBagSizer.
    ///
    /// [See `wxGBPosition`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html)
    #[doc(alias = "wxGBPosition")]
    #[doc(alias = "GBPosition")]
    class GBPosition
        = GBPositionIsOwned<true>(wxGBPosition) impl
        GBPositionMethods
}
impl<const OWNED: bool> GBPositionIsOwned<OWNED> {
    /// Default constructor, setting the row and column to (0,0).
    ///
    /// [See `wxGBPosition::wxGBPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a88ebe51f95578714548e4a823fcb164e)
    pub fn new() -> GBPositionIsOwned<OWNED> {
        unsafe { GBPositionIsOwned(ffi::wxGBPosition_new()) }
    }
    /// Construct a new wxGBPosition, setting the row and column.
    ///
    /// [See `wxGBPosition::wxGBPosition()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a6723ac451dac191b78e2a97aabf39e2b)
    pub fn new_with_int(row: c_int, col: c_int) -> GBPositionIsOwned<OWNED> {
        unsafe { GBPositionIsOwned(ffi::wxGBPosition_new1(row, col)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBPositionIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GBPositionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGBPosition_delete(self.0) }
        }
    }
}

// wxGBSizerItem
wxwidgets! {
    /// The wxGBSizerItem class is used by the wxGridBagSizer for tracking the items in the sizer.
    ///
    /// [See `wxGBSizerItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html)
    #[doc(alias = "wxGBSizerItem")]
    #[doc(alias = "GBSizerItem")]
    class GBSizerItem
        = GBSizerItemIsOwned<true>(wxGBSizerItem) impl
        GBSizerItemMethods,
        SizerItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> GBSizerItemIsOwned<OWNED> {
    /// Construct a sizer item for tracking a spacer.
    ///
    /// [See `wxGBSizerItem::wxGBSizerItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a17ecfba64a4dc099f285e45123ec9af9)
    pub fn new_with_int<G: GBPositionMethods, G2: GBSpanMethods, O: ObjectMethods>(
        width: c_int,
        height: c_int,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemIsOwned<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemIsOwned(ffi::wxGBSizerItem_new(
                width, height, pos, span, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a window.
    ///
    /// [See `wxGBSizerItem::wxGBSizerItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#aca1e967cfb791dc102ab1766acb27e4f)
    pub fn new_with_window<
        W: WindowMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        window: Option<&W>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemIsOwned(ffi::wxGBSizerItem_new1(
                window, pos, span, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a subsizer.
    ///
    /// [See `wxGBSizerItem::wxGBSizerItem()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a6e02caab6163a761ced57e5b370c72c4)
    pub fn new_with_sizer<
        S: SizerMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        sizer: Option<&S>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemIsOwned<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemIsOwned(ffi::wxGBSizerItem_new2(
                sizer, pos, span, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBSizerItemIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GBSizerItemIsOwned<OWNED>> for SizerItemIsOwned<OWNED> {
    fn from(o: GBSizerItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GBSizerItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GBSizerItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GBSizerItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGBSizerItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GBSizerItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGBSpan
wxwidgets! {
    /// This class is used to hold the row and column spanning attributes of items in a wxGridBagSizer.
    ///
    /// [See `wxGBSpan`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html)
    #[doc(alias = "wxGBSpan")]
    #[doc(alias = "GBSpan")]
    class GBSpan
        = GBSpanIsOwned<true>(wxGBSpan) impl
        GBSpanMethods
}
impl<const OWNED: bool> GBSpanIsOwned<OWNED> {
    /// Default constructor, setting the rowspan and colspan to (1,1) meaning that the item occupies one cell in each direction.
    ///
    /// [See `wxGBSpan::wxGBSpan()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a8f5790cd566fa20328c97420f5317e93)
    pub fn new() -> GBSpanIsOwned<OWNED> {
        unsafe { GBSpanIsOwned(ffi::wxGBSpan_new()) }
    }
    /// Construct a new wxGBSpan, setting the rowspan and colspan.
    ///
    /// [See `wxGBSpan::wxGBSpan()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a51a3cdf14f004d4f5b8e33d20b7ea636)
    pub fn new_with_int(rowspan: c_int, colspan: c_int) -> GBSpanIsOwned<OWNED> {
        unsafe { GBSpanIsOwned(ffi::wxGBSpan_new1(rowspan, colspan)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBSpanIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GBSpanIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGBSpan_delete(self.0) }
        }
    }
}

// wxGCDC
wxwidgets! {
    /// wxGCDC is a device context that draws on a wxGraphicsContext.
    ///
    /// [See `wxGCDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html)
    #[doc(alias = "wxGCDC")]
    #[doc(alias = "GCDC")]
    class GCDC
        = GCDCIsOwned<true>(wxGCDC) impl
        GCDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> GCDCIsOwned<OWNED> {
    /// Constructs a wxGCDC from a wxWindowDC.
    ///
    /// [See `wxGCDC::wxGCDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#ab7f82c4acbe2deba41375cce01bcaba3)
    pub fn new_with_windowdc<W: WindowDCMethods>(window_dc: &W) -> GCDCIsOwned<OWNED> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GCDCIsOwned(ffi::wxGCDC_new(window_dc))
        }
    }
    /// Constructs a wxGCDC from a wxMemoryDC.
    ///
    /// [See `wxGCDC::wxGCDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a8bfc3aa5028e38bc5e63b797fef63669)
    pub fn new_with_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> GCDCIsOwned<OWNED> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GCDCIsOwned(ffi::wxGCDC_new1(memory_dc))
        }
    }
    // BLOCKED: fn wxGCDC2()
    /// Construct a wxGCDC from an existing graphics context.
    ///
    /// [See `wxGCDC::wxGCDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a44937b71c3e256a1e2c2187caab904f8)
    pub fn new_with_graphicscontext<G: GraphicsContextMethods>(
        context: Option<&G>,
    ) -> GCDCIsOwned<OWNED> {
        unsafe {
            let context = match context {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GCDCIsOwned(ffi::wxGCDC_new3(context))
        }
    }
    /// Constructs a wxGCDC from a wxEnhMetaFileDC.
    ///
    /// [See `wxGCDC::wxGCDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a96469646f16d67d838fc49500b7af158)
    pub fn new_with_enhmetafiledc(emf_dc: *const c_void) -> GCDCIsOwned<OWNED> {
        unsafe { GCDCIsOwned(ffi::wxGCDC_new4(emf_dc)) }
    }
    ///
    /// [See `wxGCDC::wxGCDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a655c7b2351ba8ee71cec659030a0fb59)
    pub fn new() -> GCDCIsOwned<OWNED> {
        unsafe { GCDCIsOwned(ffi::wxGCDC_new5()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GCDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GCDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: GCDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GCDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GCDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GCDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGCDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GCDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGDIObject
wxwidgets! {
    /// This class allows platforms to implement functionality to optimise GDI objects, such as wxPen, wxBrush and wxFont.
    ///
    /// [See `wxGDIObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_d_i_object.html)
    #[doc(alias = "wxGDIObject")]
    #[doc(alias = "GDIObject")]
    class GDIObject
        = GDIObjectIsOwned<true>(wxGDIObject) impl
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GDIObjectIsOwned<OWNED> {
    // BLOCKED: fn wxGDIObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GDIObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GDIObjectIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GDIObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GDIObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGDIObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GDIObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGIFHandler
wxwidgets! {
    /// This is the image handler for the GIF format.
    ///
    /// [See `wxGIFHandler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html)
    #[doc(alias = "wxGIFHandler")]
    #[doc(alias = "GIFHandler")]
    class GIFHandler
        = GIFHandlerIsOwned<true>(wxGIFHandler) impl
        GIFHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GIFHandlerIsOwned<OWNED> {
    /// Default constructor for wxGIFHandler.
    ///
    /// [See `wxGIFHandler::wxGIFHandler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html#adbd198504747365e2d2ef232880546ce)
    pub fn new() -> GIFHandlerIsOwned<OWNED> {
        unsafe { GIFHandlerIsOwned(ffi::wxGIFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GIFHandlerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GIFHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: GIFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GIFHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GIFHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GIFHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGIFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GIFHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGauge
wxwidgets! {
    /// A gauge is a horizontal or vertical bar which shows a quantity (often time).
    ///
    /// [See `wxGauge`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_gauge.html)
    #[doc(alias = "wxGauge")]
    #[doc(alias = "Gauge")]
    class Gauge
        = GaugeIsOwned<true>(wxGauge) impl
        GaugeMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GaugeIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxGauge::wxGauge()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a8bf73ec5e07533060f2579c72b8fc262)
    pub fn new_2step() -> GaugeIsOwned<OWNED> {
        unsafe { GaugeIsOwned(ffi::wxGauge_new()) }
    }
    /// Constructor, creating and showing a gauge.
    ///
    /// [See `wxGauge::wxGauge()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a6a258fc160c4b1899b3e8e8f92c6b508)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        range: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> GaugeIsOwned<OWNED> {
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
            GaugeIsOwned(ffi::wxGauge_new1(
                parent, id, range, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GaugeIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GaugeIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GaugeIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGauge_CLASSINFO()) }
    }
}

// wxGenericAboutDialog
wxwidgets! {
    /// This class defines a customizable About dialog.
    ///
    /// [See `wxGenericAboutDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html)
    #[doc(alias = "wxGenericAboutDialog")]
    #[doc(alias = "GenericAboutDialog")]
    class GenericAboutDialog
        = GenericAboutDialogIsOwned<true>(wxGenericAboutDialog) impl
        GenericAboutDialogMethods
}
impl<const OWNED: bool> GenericAboutDialogIsOwned<OWNED> {
    /// Default constructor, Create() must be called later.
    ///
    /// [See `wxGenericAboutDialog::wxGenericAboutDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#ad195f2e454ba722956b73e791e1c6a03)
    pub fn new() -> GenericAboutDialogIsOwned<OWNED> {
        unsafe { GenericAboutDialogIsOwned(ffi::wxGenericAboutDialog_new()) }
    }
    /// Creates the dialog and initializes it with the given information.
    ///
    /// [See `wxGenericAboutDialog::wxGenericAboutDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#a219d9040ec0e3ef091d25ed7e865e262)
    pub fn new_with_aboutdialoginfo<A: AboutDialogInfoMethods, W: WindowMethods>(
        info: &A,
        parent: Option<&W>,
    ) -> GenericAboutDialogIsOwned<OWNED> {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericAboutDialogIsOwned(ffi::wxGenericAboutDialog_new1(info, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GenericAboutDialogIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GenericAboutDialogIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGenericAboutDialog_delete(self.0) }
        }
    }
}

// wxGenericDirCtrl
wxwidgets! {
    /// This control can be used to place a directory listing (with optional files) on an arbitrary window.
    ///
    /// [See `wxGenericDirCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html)
    #[doc(alias = "wxGenericDirCtrl")]
    #[doc(alias = "GenericDirCtrl")]
    class GenericDirCtrl
        = GenericDirCtrlIsOwned<true>(wxGenericDirCtrl) impl
        GenericDirCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericDirCtrlIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxGenericDirCtrl::wxGenericDirCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a1072f4e29922e08f7e8a288e573bec5a)
    pub fn new_2step() -> GenericDirCtrlIsOwned<OWNED> {
        unsafe { GenericDirCtrlIsOwned(ffi::wxGenericDirCtrl_new()) }
    }
    /// Main constructor.
    ///
    /// [See `wxGenericDirCtrl::wxGenericDirCtrl()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#af36fec47ee17f87e517c372981ecf661)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        dir: &str,
        pos: &P,
        size: &S,
        style: c_long,
        filter: &str,
        default_filter: c_int,
        name: &str,
    ) -> GenericDirCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            GenericDirCtrlIsOwned(ffi::wxGenericDirCtrl_new1(
                parent,
                id,
                dir,
                pos,
                size,
                style,
                filter,
                default_filter,
                name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GenericDirCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericDirCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericDirCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericDirCtrl_CLASSINFO()) }
    }
}

// wxGenericProgressDialog
wxwidgets! {
    /// This class represents a dialog that shows a short message and a progress bar.
    ///
    /// [See `wxGenericProgressDialog`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html)
    #[doc(alias = "wxGenericProgressDialog")]
    #[doc(alias = "GenericProgressDialog")]
    class GenericProgressDialog
        = GenericProgressDialogIsOwned<true>(wxGenericProgressDialog) impl
        GenericProgressDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericProgressDialogIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxGenericProgressDialog::wxGenericProgressDialog()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#ac015aa72408dcddef95f4f575bb628bc)
    pub fn new<W: WindowMethods>(
        title: &str,
        message: &str,
        maximum: c_int,
        parent: Option<&W>,
        style: c_int,
    ) -> GenericProgressDialogIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericProgressDialogIsOwned(ffi::wxGenericProgressDialog_new(
                title, message, maximum, parent, style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GenericProgressDialogIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericProgressDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericProgressDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericProgressDialog_CLASSINFO()) }
    }
}

// wxGenericValidator
wxwidgets! {
    /// wxGenericValidator performs data transfer (but not validation or filtering) for many type of controls.
    ///
    /// [See `wxGenericValidator`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html)
    #[doc(alias = "wxGenericValidator")]
    #[doc(alias = "GenericValidator")]
    class GenericValidator
        = GenericValidatorIsOwned<true>(wxGenericValidator) impl
        GenericValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericValidatorIsOwned<OWNED> {
    /// Copy constructor.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a25de71f90148ffe51d947dabdc15b01f)
    pub fn new_with_genericvalidator<G: GenericValidatorMethods>(
        validator: &G,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            GenericValidatorIsOwned(ffi::wxGenericValidator_new(validator))
        }
    }
    /// Constructor taking a bool pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#af21eecfcd12693a2ba9087cf984a8779)
    pub fn new_with_bool(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new1(val_ptr)) }
    }
    /// Constructor taking a wxString pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#ae408126b38460f71f34645fb6a7cba28)
    pub fn new_with_str(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new2(val_ptr)) }
    }
    /// Constructor taking an integer pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#abce0743e3f83f4119566661e76652dcc)
    pub fn new_with_int(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new3(val_ptr)) }
    }
    /// Constructor taking a wxArrayInt pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a1b08bdcfa0a194d224579914da51d210)
    pub fn new_with_arrayint<A: ArrayIntMethods>(
        val_ptr: Option<&A>,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorIsOwned(ffi::wxGenericValidator_new4(val_ptr))
        }
    }
    /// Constructor taking a wxDateTime pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#ae9716ab95a262e8c02bc6abc64558a13)
    pub fn new_with_datetime<D: DateTimeMethods>(
        val_ptr: Option<&D>,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorIsOwned(ffi::wxGenericValidator_new5(val_ptr))
        }
    }
    /// Constructor taking a wxFileName pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a53edd5f4520573fbb9de25b281d32f2f)
    pub fn new_with_filename<F: FileNameMethods>(
        val_ptr: Option<&F>,
    ) -> GenericValidatorIsOwned<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorIsOwned(ffi::wxGenericValidator_new6(val_ptr))
        }
    }
    /// Constructor taking a float pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a62ddb0362276e41bf84a547b015554b0)
    pub fn new_with_float(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new7(val_ptr)) }
    }
    /// Constructor taking a double pointer.
    ///
    /// [See `wxGenericValidator::wxGenericValidator()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a66e90c6da5e71e7258a2581370b92795)
    pub fn new_with_double(val_ptr: *mut c_void) -> GenericValidatorIsOwned<OWNED> {
        unsafe { GenericValidatorIsOwned(ffi::wxGenericValidator_new8(val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GenericValidatorIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GenericValidatorIsOwned<OWNED>> for ValidatorIsOwned<OWNED> {
    fn from(o: GenericValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericValidatorIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: GenericValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericValidatorIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GenericValidatorIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericValidatorIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGenericValidator_CLASSINFO()) }
    }
}

// wxGraphicsBrush
wxwidgets! {
    /// A wxGraphicsBrush is a native representation of a brush.
    ///
    /// [See `wxGraphicsBrush`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_brush.html)
    #[doc(alias = "wxGraphicsBrush")]
    #[doc(alias = "GraphicsBrush")]
    class GraphicsBrush
        = GraphicsBrushIsOwned<true>(wxGraphicsBrush) impl
        GraphicsBrushMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsBrushIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsBrushIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsBrushIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsBrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsBrushIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsBrushIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsBrushIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsBrush_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsBrushIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsContext
wxwidgets! {
    /// A wxGraphicsContext instance is the object that is drawn upon.
    ///
    /// [See `wxGraphicsContext`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html)
    #[doc(alias = "wxGraphicsContext")]
    #[doc(alias = "GraphicsContext")]
    class GraphicsContext
        = GraphicsContextIsOwned<true>(wxGraphicsContext) impl
        GraphicsContextMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsContextIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsContextIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsContextIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsContextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsContextIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsContextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsContextIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsContext_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsContextIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsFont
wxwidgets! {
    /// A wxGraphicsFont is a native representation of a font.
    ///
    /// [See `wxGraphicsFont`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_font.html)
    #[doc(alias = "wxGraphicsFont")]
    #[doc(alias = "GraphicsFont")]
    class GraphicsFont
        = GraphicsFontIsOwned<true>(wxGraphicsFont) impl
        GraphicsFontMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsFontIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsFontIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsFontIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsFontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsFontIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsFontIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsFontIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsFont_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsFontIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStop
wxwidgets! {
    /// Represents a single gradient stop in a collection of gradient stops as represented by wxGraphicsGradientStops.
    ///
    /// [See `wxGraphicsGradientStop`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stop.html)
    #[doc(alias = "wxGraphicsGradientStop")]
    #[doc(alias = "GraphicsGradientStop")]
    class GraphicsGradientStop
        = GraphicsGradientStopIsOwned<true>(wxGraphicsGradientStop) impl
        GraphicsGradientStopMethods
}
impl<const OWNED: bool> GraphicsGradientStopIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxGraphicsGradientStop()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsGradientStopIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GraphicsGradientStopIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGraphicsGradientStop_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStops
wxwidgets! {
    /// Represents a collection of wxGraphicGradientStop values for use with CreateLinearGradientBrush and CreateRadialGradientBrush.
    ///
    /// [See `wxGraphicsGradientStops`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html)
    #[doc(alias = "wxGraphicsGradientStops")]
    #[doc(alias = "GraphicsGradientStops")]
    class GraphicsGradientStops
        = GraphicsGradientStopsIsOwned<true>(wxGraphicsGradientStops) impl
        GraphicsGradientStopsMethods
}
impl<const OWNED: bool> GraphicsGradientStopsIsOwned<OWNED> {
    // BLOCKED: fn wxGraphicsGradientStops()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsGradientStopsIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GraphicsGradientStopsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGraphicsGradientStops_delete(self.0) }
        }
    }
}

// wxGraphicsMatrix
wxwidgets! {
    /// A wxGraphicsMatrix is a native representation of an affine matrix.
    ///
    /// [See `wxGraphicsMatrix`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html)
    #[doc(alias = "wxGraphicsMatrix")]
    #[doc(alias = "GraphicsMatrix")]
    class GraphicsMatrix
        = GraphicsMatrixIsOwned<true>(wxGraphicsMatrix) impl
        GraphicsMatrixMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsMatrixIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsMatrixIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsMatrixIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsMatrixIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsMatrixIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsMatrixIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsMatrixIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsMatrix_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsMatrixIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsObject
wxwidgets! {
    /// This class is the superclass of native graphics objects like pens etc.
    ///
    /// [See `wxGraphicsObject`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_object.html)
    #[doc(alias = "wxGraphicsObject")]
    #[doc(alias = "GraphicsObject")]
    class GraphicsObject
        = GraphicsObjectIsOwned<true>(wxGraphicsObject) impl
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsObjectIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsObjectIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPath
wxwidgets! {
    /// A wxGraphicsPath is a native representation of a geometric path.
    ///
    /// [See `wxGraphicsPath`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html)
    #[doc(alias = "wxGraphicsPath")]
    #[doc(alias = "GraphicsPath")]
    class GraphicsPath
        = GraphicsPathIsOwned<true>(wxGraphicsPath) impl
        GraphicsPathMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsPathIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsPathIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsPathIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsPathIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsPathIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsPathIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsPathIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsPath_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsPathIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPen
wxwidgets! {
    /// A wxGraphicsPen is a native representation of a pen.
    ///
    /// [See `wxGraphicsPen`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_pen.html)
    #[doc(alias = "wxGraphicsPen")]
    #[doc(alias = "GraphicsPen")]
    class GraphicsPen
        = GraphicsPenIsOwned<true>(wxGraphicsPen) impl
        GraphicsPenMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsPenIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsPenIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsPenIsOwned<OWNED>> for GraphicsObjectIsOwned<OWNED> {
    fn from(o: GraphicsPenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsPenIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsPenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsPenIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsPen_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsPenIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsRenderer
wxwidgets! {
    /// A wxGraphicsRenderer is the instance corresponding to the rendering engine used.
    ///
    /// [See `wxGraphicsRenderer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html)
    #[doc(alias = "wxGraphicsRenderer")]
    #[doc(alias = "GraphicsRenderer")]
    class GraphicsRenderer
        = GraphicsRendererIsOwned<true>(wxGraphicsRenderer) impl
        GraphicsRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsRendererIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsRendererIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsRendererIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GraphicsRendererIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsRendererIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGraphicsRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsRendererIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridBagSizer
wxwidgets! {
    /// A wxSizer that can lay out items in a virtual grid like a wxFlexGridSizer but in this case explicit positioning of the items is allowed using wxGBPosition, and items can optionally span more than one row and/or column using wxGBSpan.
    ///
    /// [See `wxGridBagSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html)
    #[doc(alias = "wxGridBagSizer")]
    #[doc(alias = "GridBagSizer")]
    class GridBagSizer
        = GridBagSizerIsOwned<true>(wxGridBagSizer) impl
        GridBagSizerMethods,
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridBagSizerIsOwned<OWNED> {
    /// Constructor, with optional parameters to specify the gap between the rows and columns.
    ///
    /// [See `wxGridBagSizer::wxGridBagSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a22172ccf78ed632648760f8473bc121f)
    pub fn new(vgap: c_int, hgap: c_int) -> GridBagSizerIsOwned<OWNED> {
        unsafe { GridBagSizerIsOwned(ffi::wxGridBagSizer_new(vgap, hgap)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GridBagSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for FlexGridSizerIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for GridSizerIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridBagSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridBagSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridBagSizer_CLASSINFO()) }
    }
}

// wxGridEditorCreatedEvent
wxwidgets! {
    ///
    ///
    /// [See `wxGridEditorCreatedEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html)
    #[doc(alias = "wxGridEditorCreatedEvent")]
    #[doc(alias = "GridEditorCreatedEvent")]
    class GridEditorCreatedEvent
        = GridEditorCreatedEventIsOwned<true>(wxGridEditorCreatedEvent) impl
        GridEditorCreatedEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridEditorCreatedEventIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxGridEditorCreatedEvent::wxGridEditorCreatedEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a311a9e5fe97c903d94a5d6ed84c80c76)
    pub fn new() -> GridEditorCreatedEventIsOwned<OWNED> {
        unsafe { GridEditorCreatedEventIsOwned(ffi::wxGridEditorCreatedEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEditorCreatedEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridEditorCreatedEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridEditorCreatedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridEditorCreatedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridEditorCreatedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridEditorCreatedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridEditorCreatedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridEditorCreatedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridEvent
wxwidgets! {
    /// This event class contains information about various grid events.
    ///
    /// [See `wxGridEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_event.html)
    #[doc(alias = "wxGridEvent")]
    #[doc(alias = "GridEvent")]
    class GridEvent
        = GridEventIsOwned<true>(wxGridEvent) impl
        GridEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridEventIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxGridEvent::wxGridEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a05e8831def820bf32a37693f500bf78d)
    pub fn new() -> GridEventIsOwned<OWNED> {
        unsafe { GridEventIsOwned(ffi::wxGridEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridRangeSelectEvent
wxwidgets! {
    /// Events of this class notify about a range of cells being selected.
    ///
    /// [See `wxGridRangeSelectEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html)
    #[doc(alias = "wxGridRangeSelectEvent")]
    #[doc(alias = "GridRangeSelectEvent")]
    class GridRangeSelectEvent
        = GridRangeSelectEventIsOwned<true>(wxGridRangeSelectEvent) impl
        GridRangeSelectEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridRangeSelectEventIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxGridRangeSelectEvent::wxGridRangeSelectEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a6739617b035d38ed793faf2994cf4a40)
    pub fn new() -> GridRangeSelectEventIsOwned<OWNED> {
        unsafe { GridRangeSelectEventIsOwned(ffi::wxGridRangeSelectEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridRangeSelectEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridRangeSelectEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridRangeSelectEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridRangeSelectEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridRangeSelectEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridRangeSelectEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizeEvent
wxwidgets! {
    /// This event class contains information about a row/column resize event.
    ///
    /// [See `wxGridSizeEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html)
    #[doc(alias = "wxGridSizeEvent")]
    #[doc(alias = "GridSizeEvent")]
    class GridSizeEvent
        = GridSizeEventIsOwned<true>(wxGridSizeEvent) impl
        GridSizeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridSizeEventIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxGridSizeEvent::wxGridSizeEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#a183ad53c5c01f3c21b7c0b546845731d)
    pub fn new() -> GridSizeEventIsOwned<OWNED> {
        unsafe { GridSizeEventIsOwned(ffi::wxGridSizeEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridSizeEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridSizeEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridSizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridSizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridSizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridSizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizer
wxwidgets! {
    /// A grid sizer is a sizer which lays out its children in a two-dimensional table with all table fields having the same size, i.e.
    ///
    /// [See `wxGridSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html)
    #[doc(alias = "wxGridSizer")]
    #[doc(alias = "GridSizer")]
    class GridSizer
        = GridSizerIsOwned<true>(wxGridSizer) impl
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridSizerIsOwned<OWNED> {
    /// wxGridSizer constructors.
    ///
    /// [See `wxGridSizer::wxGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a54163c431608c0cbe6c74c9009ef1ca2)
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> GridSizerIsOwned<OWNED> {
        unsafe { GridSizerIsOwned(ffi::wxGridSizer_new(cols, vgap, hgap)) }
    }
    ///
    /// [See `wxGridSizer::wxGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a6b0493be4197aff5bf59f1e68676d711)
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> GridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerIsOwned(ffi::wxGridSizer_new1(cols, gap))
        }
    }
    ///
    /// [See `wxGridSizer::wxGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a1f0a163a3a216ed647452a31951b66bd)
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> GridSizerIsOwned<OWNED> {
        unsafe { GridSizerIsOwned(ffi::wxGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    ///
    /// [See `wxGridSizer::wxGridSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#afa96e5d782116e2418ee440724b1a312)
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> GridSizerIsOwned<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerIsOwned(ffi::wxGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GridSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: GridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridSizer_CLASSINFO()) }
    }
}

// wxGridTableBase
wxwidgets! {
    /// The almost abstract base class for grid tables.
    ///
    /// [See `wxGridTableBase`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html)
    #[doc(alias = "wxGridTableBase")]
    #[doc(alias = "GridTableBase")]
    class GridTableBase
        = GridTableBaseIsOwned<true>(wxGridTableBase) impl
        GridTableBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridTableBaseIsOwned<OWNED> {
    // BLOCKED: fn wxGridTableBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridTableBaseIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridTableBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: GridTableBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridTableBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxGridTableBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridTableBaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridUpdateLocker
wxwidgets! {
    /// This small class can be used to prevent wxGrid from redrawing during its lifetime by calling wxGrid::BeginBatch() in its constructor and wxGrid::EndBatch() in its destructor.
    ///
    /// [See `wxGridUpdateLocker`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html)
    #[doc(alias = "wxGridUpdateLocker")]
    #[doc(alias = "GridUpdateLocker")]
    class GridUpdateLocker
        = GridUpdateLockerIsOwned<true>(wxGridUpdateLocker) impl
        GridUpdateLockerMethods
}
impl<const OWNED: bool> GridUpdateLockerIsOwned<OWNED> {
    /// Creates an object preventing the updates of the specified grid.
    ///
    /// [See `wxGridUpdateLocker::wxGridUpdateLocker()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html#a13605827243de9ed1c0864fbd055cb8f)
    pub fn new(grid: *mut c_void) -> GridUpdateLockerIsOwned<OWNED> {
        unsafe { GridUpdateLockerIsOwned(ffi::wxGridUpdateLocker_new(grid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridUpdateLockerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GridUpdateLockerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridUpdateLocker_delete(self.0) }
        }
    }
}
