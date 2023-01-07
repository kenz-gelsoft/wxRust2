use super::*;

// wxGBPosition
wxwidgets! {
    /// This class represents the position of an item in a virtual grid of rows and columns managed by a wxGridBagSizer.
    /// - [`GBPosition`] represents a C++ `wxGBPosition` class instance which your code has ownership, [`GBPositionInRust`]`<false>` represents one which don't own.
    /// - Use [`GBPosition`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGBPosition` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html) for more details.
    #[doc(alias = "wxGBPosition")]
    #[doc(alias = "GBPosition")]
    class GBPosition
        = GBPositionInRust<true>(wxGBPosition) impl
        GBPositionMethods
}
impl<const OWNED: bool> GBPositionInRust<OWNED> {
    /// Default constructor, setting the row and column to (0,0).
    ///
    /// See [C++ `wxGBPosition::wxGBPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a88ebe51f95578714548e4a823fcb164e).
    pub fn new() -> GBPositionInRust<OWNED> {
        unsafe { GBPositionInRust(ffi::wxGBPosition_new()) }
    }
    /// Construct a new wxGBPosition, setting the row and column.
    ///
    /// See [C++ `wxGBPosition::wxGBPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a6723ac451dac191b78e2a97aabf39e2b).
    pub fn new_with_int(row: c_int, col: c_int) -> GBPositionInRust<OWNED> {
        unsafe { GBPositionInRust(ffi::wxGBPosition_new1(row, col)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBPositionInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GBPositionInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGBPosition_delete(self.0) }
        }
    }
}

// wxGBSizerItem
wxwidgets! {
    /// The wxGBSizerItem class is used by the wxGridBagSizer for tracking the items in the sizer.
    /// - [`GBSizerItem`] represents a C++ `wxGBSizerItem` class instance which your code has ownership, [`GBSizerItemInRust`]`<false>` represents one which don't own.
    /// - Use [`GBSizerItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGBSizerItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html) for more details.
    #[doc(alias = "wxGBSizerItem")]
    #[doc(alias = "GBSizerItem")]
    class GBSizerItem
        = GBSizerItemInRust<true>(wxGBSizerItem) impl
        GBSizerItemMethods,
        SizerItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> GBSizerItemInRust<OWNED> {
    /// Construct a sizer item for tracking a spacer.
    ///
    /// See [C++ `wxGBSizerItem::wxGBSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a17ecfba64a4dc099f285e45123ec9af9).
    pub fn new_with_int<G: GBPositionMethods, G2: GBSpanMethods, O: ObjectMethods>(
        width: c_int,
        height: c_int,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> GBSizerItemInRust<OWNED> {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemInRust(ffi::wxGBSizerItem_new(
                width, height, pos, span, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a window.
    ///
    /// See [C++ `wxGBSizerItem::wxGBSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#aca1e967cfb791dc102ab1766acb27e4f).
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
    ) -> GBSizerItemInRust<OWNED> {
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
            GBSizerItemInRust(ffi::wxGBSizerItem_new1(
                window, pos, span, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a subsizer.
    ///
    /// See [C++ `wxGBSizerItem::wxGBSizerItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html#a6e02caab6163a761ced57e5b370c72c4).
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
    ) -> GBSizerItemInRust<OWNED> {
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
            GBSizerItemInRust(ffi::wxGBSizerItem_new2(
                sizer, pos, span, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBSizerItemInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GBSizerItemInRust<OWNED>> for SizerItemInRust<OWNED> {
    fn from(o: GBSizerItemInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GBSizerItemInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GBSizerItemInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GBSizerItemInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGBSizerItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GBSizerItemInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGBSpan
wxwidgets! {
    /// This class is used to hold the row and column spanning attributes of items in a wxGridBagSizer.
    /// - [`GBSpan`] represents a C++ `wxGBSpan` class instance which your code has ownership, [`GBSpanInRust`]`<false>` represents one which don't own.
    /// - Use [`GBSpan`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGBSpan` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html) for more details.
    #[doc(alias = "wxGBSpan")]
    #[doc(alias = "GBSpan")]
    class GBSpan
        = GBSpanInRust<true>(wxGBSpan) impl
        GBSpanMethods
}
impl<const OWNED: bool> GBSpanInRust<OWNED> {
    /// Default constructor, setting the rowspan and colspan to (1,1) meaning that the item occupies one cell in each direction.
    ///
    /// See [C++ `wxGBSpan::wxGBSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a8f5790cd566fa20328c97420f5317e93).
    pub fn new() -> GBSpanInRust<OWNED> {
        unsafe { GBSpanInRust(ffi::wxGBSpan_new()) }
    }
    /// Construct a new wxGBSpan, setting the rowspan and colspan.
    ///
    /// See [C++ `wxGBSpan::wxGBSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a51a3cdf14f004d4f5b8e33d20b7ea636).
    pub fn new_with_int(rowspan: c_int, colspan: c_int) -> GBSpanInRust<OWNED> {
        unsafe { GBSpanInRust(ffi::wxGBSpan_new1(rowspan, colspan)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBSpanInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GBSpanInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGBSpan_delete(self.0) }
        }
    }
}

// wxGCDC
wxwidgets! {
    /// wxGCDC is a device context that draws on a wxGraphicsContext.
    /// - [`GCDC`] represents a C++ `wxGCDC` class instance which your code has ownership, [`GCDCInRust`]`<false>` represents one which don't own.
    /// - Use [`GCDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGCDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html) for more details.
    #[doc(alias = "wxGCDC")]
    #[doc(alias = "GCDC")]
    class GCDC
        = GCDCInRust<true>(wxGCDC) impl
        GCDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> GCDCInRust<OWNED> {
    /// Constructs a wxGCDC from a wxWindowDC.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#ab7f82c4acbe2deba41375cce01bcaba3).
    pub fn new_with_windowdc<W: WindowDCMethods>(window_dc: &W) -> GCDCInRust<OWNED> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GCDCInRust(ffi::wxGCDC_new(window_dc))
        }
    }
    /// Constructs a wxGCDC from a wxMemoryDC.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a8bfc3aa5028e38bc5e63b797fef63669).
    pub fn new_with_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> GCDCInRust<OWNED> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GCDCInRust(ffi::wxGCDC_new1(memory_dc))
        }
    }
    // BLOCKED: fn wxGCDC2()
    /// Construct a wxGCDC from an existing graphics context.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a44937b71c3e256a1e2c2187caab904f8).
    pub fn new_with_graphicscontext<G: GraphicsContextMethods>(
        context: Option<&G>,
    ) -> GCDCInRust<OWNED> {
        unsafe {
            let context = match context {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GCDCInRust(ffi::wxGCDC_new3(context))
        }
    }
    /// Constructs a wxGCDC from a wxEnhMetaFileDC.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a96469646f16d67d838fc49500b7af158).
    pub fn new_with_enhmetafiledc(emf_dc: *const c_void) -> GCDCInRust<OWNED> {
        unsafe { GCDCInRust(ffi::wxGCDC_new4(emf_dc)) }
    }
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a655c7b2351ba8ee71cec659030a0fb59).
    pub fn new() -> GCDCInRust<OWNED> {
        unsafe { GCDCInRust(ffi::wxGCDC_new5()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GCDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GCDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: GCDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GCDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GCDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GCDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGCDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GCDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGDIObject
wxwidgets! {
    /// This class allows platforms to implement functionality to optimise GDI objects, such as wxPen, wxBrush and wxFont.
    /// - [`GDIObject`] represents a C++ `wxGDIObject` class instance which your code has ownership, [`GDIObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`GDIObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGDIObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_d_i_object.html) for more details.
    #[doc(alias = "wxGDIObject")]
    #[doc(alias = "GDIObject")]
    class GDIObject
        = GDIObjectInRust<true>(wxGDIObject) impl
        GDIObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GDIObjectInRust<OWNED> {
    // BLOCKED: fn wxGDIObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GDIObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GDIObjectInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GDIObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GDIObjectInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGDIObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GDIObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGIFHandler
wxwidgets! {
    /// This is the image handler for the GIF format.
    /// - [`GIFHandler`] represents a C++ `wxGIFHandler` class instance which your code has ownership, [`GIFHandlerInRust`]`<false>` represents one which don't own.
    /// - Use [`GIFHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGIFHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html) for more details.
    #[doc(alias = "wxGIFHandler")]
    #[doc(alias = "GIFHandler")]
    class GIFHandler
        = GIFHandlerInRust<true>(wxGIFHandler) impl
        GIFHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GIFHandlerInRust<OWNED> {
    /// Default constructor for wxGIFHandler.
    ///
    /// See [C++ `wxGIFHandler::wxGIFHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html#adbd198504747365e2d2ef232880546ce).
    pub fn new() -> GIFHandlerInRust<OWNED> {
        unsafe { GIFHandlerInRust(ffi::wxGIFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GIFHandlerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GIFHandlerInRust<OWNED>> for ImageHandlerInRust<OWNED> {
    fn from(o: GIFHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GIFHandlerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GIFHandlerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GIFHandlerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGIFHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GIFHandlerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGauge
wxwidgets! {
    /// A gauge is a horizontal or vertical bar which shows a quantity (often time).
    /// - [`Gauge`] represents a C++ `wxGauge` class instance which your code has ownership, [`GaugeInRust`]`<false>` represents one which don't own.
    /// - Use [`Gauge`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGauge` class's documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html) for more details.
    #[doc(alias = "wxGauge")]
    #[doc(alias = "Gauge")]
    class Gauge
        = GaugeInRust<true>(wxGauge) impl
        GaugeMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GaugeInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxGauge::wxGauge()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a8bf73ec5e07533060f2579c72b8fc262).
    pub fn new_2step() -> GaugeInRust<OWNED> {
        unsafe { GaugeInRust(ffi::wxGauge_new()) }
    }
    /// Constructor, creating and showing a gauge.
    ///
    /// See [C++ `wxGauge::wxGauge()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a6a258fc160c4b1899b3e8e8f92c6b508).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        range: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> GaugeInRust<OWNED> {
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
            GaugeInRust(ffi::wxGauge_new1(
                parent, id, range, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GaugeInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GaugeInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: GaugeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: GaugeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: GaugeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GaugeInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GaugeInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GaugeInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGauge_CLASSINFO()) }
    }
}

// wxGenericAboutDialog
wxwidgets! {
    /// This class defines a customizable About dialog.
    /// - [`GenericAboutDialog`] represents a C++ `wxGenericAboutDialog` class instance which your code has ownership, [`GenericAboutDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`GenericAboutDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericAboutDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html) for more details.
    #[doc(alias = "wxGenericAboutDialog")]
    #[doc(alias = "GenericAboutDialog")]
    class GenericAboutDialog
        = GenericAboutDialogInRust<true>(wxGenericAboutDialog) impl
        GenericAboutDialogMethods
}
impl<const OWNED: bool> GenericAboutDialogInRust<OWNED> {
    /// Default constructor, Create() must be called later.
    ///
    /// See [C++ `wxGenericAboutDialog::wxGenericAboutDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#ad195f2e454ba722956b73e791e1c6a03).
    pub fn new() -> GenericAboutDialogInRust<OWNED> {
        unsafe { GenericAboutDialogInRust(ffi::wxGenericAboutDialog_new()) }
    }
    /// Creates the dialog and initializes it with the given information.
    ///
    /// See [C++ `wxGenericAboutDialog::wxGenericAboutDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#a219d9040ec0e3ef091d25ed7e865e262).
    pub fn new_with_aboutdialoginfo<A: AboutDialogInfoMethods, W: WindowMethods>(
        info: &A,
        parent: Option<&W>,
    ) -> GenericAboutDialogInRust<OWNED> {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericAboutDialogInRust(ffi::wxGenericAboutDialog_new1(info, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GenericAboutDialogInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GenericAboutDialogInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGenericAboutDialog_delete(self.0) }
        }
    }
}

// wxGenericDirCtrl
wxwidgets! {
    /// This control can be used to place a directory listing (with optional files) on an arbitrary window.
    /// - [`GenericDirCtrl`] represents a C++ `wxGenericDirCtrl` class instance which your code has ownership, [`GenericDirCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`GenericDirCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericDirCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html) for more details.
    #[doc(alias = "wxGenericDirCtrl")]
    #[doc(alias = "GenericDirCtrl")]
    class GenericDirCtrl
        = GenericDirCtrlInRust<true>(wxGenericDirCtrl) impl
        GenericDirCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericDirCtrlInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxGenericDirCtrl::wxGenericDirCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a1072f4e29922e08f7e8a288e573bec5a).
    pub fn new_2step() -> GenericDirCtrlInRust<OWNED> {
        unsafe { GenericDirCtrlInRust(ffi::wxGenericDirCtrl_new()) }
    }
    /// Main constructor.
    ///
    /// See [C++ `wxGenericDirCtrl::wxGenericDirCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#af36fec47ee17f87e517c372981ecf661).
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
    ) -> GenericDirCtrlInRust<OWNED> {
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
            GenericDirCtrlInRust(ffi::wxGenericDirCtrl_new1(
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
impl<const OWNED: bool> Clone for GenericDirCtrlInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GenericDirCtrlInRust<OWNED>> for ControlInRust<OWNED> {
    fn from(o: GenericDirCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: GenericDirCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: GenericDirCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericDirCtrlInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GenericDirCtrlInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericDirCtrlInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGenericDirCtrl_CLASSINFO()) }
    }
}

// wxGenericProgressDialog
wxwidgets! {
    /// This class represents a dialog that shows a short message and a progress bar.
    /// - [`GenericProgressDialog`] represents a C++ `wxGenericProgressDialog` class instance which your code has ownership, [`GenericProgressDialogInRust`]`<false>` represents one which don't own.
    /// - Use [`GenericProgressDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericProgressDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html) for more details.
    #[doc(alias = "wxGenericProgressDialog")]
    #[doc(alias = "GenericProgressDialog")]
    class GenericProgressDialog
        = GenericProgressDialogInRust<true>(wxGenericProgressDialog) impl
        GenericProgressDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericProgressDialogInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxGenericProgressDialog::wxGenericProgressDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#ac015aa72408dcddef95f4f575bb628bc).
    pub fn new<W: WindowMethods>(
        title: &str,
        message: &str,
        maximum: c_int,
        parent: Option<&W>,
        style: c_int,
    ) -> GenericProgressDialogInRust<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericProgressDialogInRust(ffi::wxGenericProgressDialog_new(
                title, message, maximum, parent, style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GenericProgressDialogInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GenericProgressDialogInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: GenericProgressDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: GenericProgressDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: GenericProgressDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: GenericProgressDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: GenericProgressDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericProgressDialogInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GenericProgressDialogInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericProgressDialogInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGenericProgressDialog_CLASSINFO()) }
    }
}

// wxGenericValidator
wxwidgets! {
    /// wxGenericValidator performs data transfer (but not validation or filtering) for many type of controls.
    /// - [`GenericValidator`] represents a C++ `wxGenericValidator` class instance which your code has ownership, [`GenericValidatorInRust`]`<false>` represents one which don't own.
    /// - Use [`GenericValidator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html) for more details.
    #[doc(alias = "wxGenericValidator")]
    #[doc(alias = "GenericValidator")]
    class GenericValidator
        = GenericValidatorInRust<true>(wxGenericValidator) impl
        GenericValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GenericValidatorInRust<OWNED> {
    /// Copy constructor.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a25de71f90148ffe51d947dabdc15b01f).
    pub fn new_with_genericvalidator<G: GenericValidatorMethods>(
        validator: &G,
    ) -> GenericValidatorInRust<OWNED> {
        unsafe {
            let validator = validator.as_ptr();
            GenericValidatorInRust(ffi::wxGenericValidator_new(validator))
        }
    }
    /// Constructor taking a bool pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#af21eecfcd12693a2ba9087cf984a8779).
    pub fn new_with_bool(val_ptr: *mut c_void) -> GenericValidatorInRust<OWNED> {
        unsafe { GenericValidatorInRust(ffi::wxGenericValidator_new1(val_ptr)) }
    }
    /// Constructor taking a wxString pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#ae408126b38460f71f34645fb6a7cba28).
    pub fn new_with_str(val_ptr: *mut c_void) -> GenericValidatorInRust<OWNED> {
        unsafe { GenericValidatorInRust(ffi::wxGenericValidator_new2(val_ptr)) }
    }
    /// Constructor taking an integer pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#abce0743e3f83f4119566661e76652dcc).
    pub fn new_with_int(val_ptr: *mut c_void) -> GenericValidatorInRust<OWNED> {
        unsafe { GenericValidatorInRust(ffi::wxGenericValidator_new3(val_ptr)) }
    }
    /// Constructor taking a wxArrayInt pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a1b08bdcfa0a194d224579914da51d210).
    pub fn new_with_arrayint<A: ArrayIntMethods>(
        val_ptr: Option<&A>,
    ) -> GenericValidatorInRust<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorInRust(ffi::wxGenericValidator_new4(val_ptr))
        }
    }
    /// Constructor taking a wxDateTime pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#ae9716ab95a262e8c02bc6abc64558a13).
    pub fn new_with_datetime<D: DateTimeMethods>(
        val_ptr: Option<&D>,
    ) -> GenericValidatorInRust<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorInRust(ffi::wxGenericValidator_new5(val_ptr))
        }
    }
    /// Constructor taking a wxFileName pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a53edd5f4520573fbb9de25b281d32f2f).
    pub fn new_with_filename<F: FileNameMethods>(
        val_ptr: Option<&F>,
    ) -> GenericValidatorInRust<OWNED> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorInRust(ffi::wxGenericValidator_new6(val_ptr))
        }
    }
    /// Constructor taking a float pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a62ddb0362276e41bf84a547b015554b0).
    pub fn new_with_float(val_ptr: *mut c_void) -> GenericValidatorInRust<OWNED> {
        unsafe { GenericValidatorInRust(ffi::wxGenericValidator_new7(val_ptr)) }
    }
    /// Constructor taking a double pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a66e90c6da5e71e7258a2581370b92795).
    pub fn new_with_double(val_ptr: *mut c_void) -> GenericValidatorInRust<OWNED> {
        unsafe { GenericValidatorInRust(ffi::wxGenericValidator_new8(val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GenericValidatorInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GenericValidatorInRust<OWNED>> for ValidatorInRust<OWNED> {
    fn from(o: GenericValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericValidatorInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: GenericValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GenericValidatorInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GenericValidatorInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GenericValidatorInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGenericValidator_CLASSINFO()) }
    }
}

// wxGraphicsBrush
wxwidgets! {
    /// A wxGraphicsBrush is a native representation of a brush.
    /// - [`GraphicsBrush`] represents a C++ `wxGraphicsBrush` class instance which your code has ownership, [`GraphicsBrushInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsBrush`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsBrush` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_brush.html) for more details.
    #[doc(alias = "wxGraphicsBrush")]
    #[doc(alias = "GraphicsBrush")]
    class GraphicsBrush
        = GraphicsBrushInRust<true>(wxGraphicsBrush) impl
        GraphicsBrushMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsBrushInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsBrushInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsBrushInRust<OWNED>> for GraphicsObjectInRust<OWNED> {
    fn from(o: GraphicsBrushInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsBrushInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsBrushInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsBrushInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsBrush_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsBrushInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsContext
wxwidgets! {
    /// A wxGraphicsContext instance is the object that is drawn upon.
    /// - [`GraphicsContext`] represents a C++ `wxGraphicsContext` class instance which your code has ownership, [`GraphicsContextInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsContext`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsContext` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html) for more details.
    #[doc(alias = "wxGraphicsContext")]
    #[doc(alias = "GraphicsContext")]
    class GraphicsContext
        = GraphicsContextInRust<true>(wxGraphicsContext) impl
        GraphicsContextMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsContextInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsContextInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsContextInRust<OWNED>> for GraphicsObjectInRust<OWNED> {
    fn from(o: GraphicsContextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsContextInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsContextInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsContextInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsContext_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsContextInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsFont
wxwidgets! {
    /// A wxGraphicsFont is a native representation of a font.
    /// - [`GraphicsFont`] represents a C++ `wxGraphicsFont` class instance which your code has ownership, [`GraphicsFontInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsFont`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsFont` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_font.html) for more details.
    #[doc(alias = "wxGraphicsFont")]
    #[doc(alias = "GraphicsFont")]
    class GraphicsFont
        = GraphicsFontInRust<true>(wxGraphicsFont) impl
        GraphicsFontMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsFontInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsFontInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsFontInRust<OWNED>> for GraphicsObjectInRust<OWNED> {
    fn from(o: GraphicsFontInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsFontInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsFontInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsFontInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsFont_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsFontInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStop
wxwidgets! {
    /// Represents a single gradient stop in a collection of gradient stops as represented by wxGraphicsGradientStops.
    /// - [`GraphicsGradientStop`] represents a C++ `wxGraphicsGradientStop` class instance which your code has ownership, [`GraphicsGradientStopInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsGradientStop`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsGradientStop` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stop.html) for more details.
    #[doc(alias = "wxGraphicsGradientStop")]
    #[doc(alias = "GraphicsGradientStop")]
    class GraphicsGradientStop
        = GraphicsGradientStopInRust<true>(wxGraphicsGradientStop) impl
        GraphicsGradientStopMethods
}
impl<const OWNED: bool> GraphicsGradientStopInRust<OWNED> {
    // NOT_SUPPORTED: fn wxGraphicsGradientStop()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsGradientStopInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GraphicsGradientStopInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGraphicsGradientStop_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStops
wxwidgets! {
    /// Represents a collection of wxGraphicGradientStop values for use with CreateLinearGradientBrush and CreateRadialGradientBrush.
    /// - [`GraphicsGradientStops`] represents a C++ `wxGraphicsGradientStops` class instance which your code has ownership, [`GraphicsGradientStopsInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsGradientStops`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsGradientStops` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html) for more details.
    #[doc(alias = "wxGraphicsGradientStops")]
    #[doc(alias = "GraphicsGradientStops")]
    class GraphicsGradientStops
        = GraphicsGradientStopsInRust<true>(wxGraphicsGradientStops) impl
        GraphicsGradientStopsMethods
}
impl<const OWNED: bool> GraphicsGradientStopsInRust<OWNED> {
    // BLOCKED: fn wxGraphicsGradientStops()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsGradientStopsInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GraphicsGradientStopsInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGraphicsGradientStops_delete(self.0) }
        }
    }
}

// wxGraphicsMatrix
wxwidgets! {
    /// A wxGraphicsMatrix is a native representation of an affine matrix.
    /// - [`GraphicsMatrix`] represents a C++ `wxGraphicsMatrix` class instance which your code has ownership, [`GraphicsMatrixInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsMatrix`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsMatrix` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html) for more details.
    #[doc(alias = "wxGraphicsMatrix")]
    #[doc(alias = "GraphicsMatrix")]
    class GraphicsMatrix
        = GraphicsMatrixInRust<true>(wxGraphicsMatrix) impl
        GraphicsMatrixMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsMatrixInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsMatrixInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsMatrixInRust<OWNED>> for GraphicsObjectInRust<OWNED> {
    fn from(o: GraphicsMatrixInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsMatrixInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsMatrixInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsMatrixInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsMatrix_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsMatrixInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsObject
wxwidgets! {
    /// This class is the superclass of native graphics objects like pens etc.
    /// - [`GraphicsObject`] represents a C++ `wxGraphicsObject` class instance which your code has ownership, [`GraphicsObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_object.html) for more details.
    #[doc(alias = "wxGraphicsObject")]
    #[doc(alias = "GraphicsObject")]
    class GraphicsObject
        = GraphicsObjectInRust<true>(wxGraphicsObject) impl
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsObjectInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsObjectInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsObjectInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPath
wxwidgets! {
    /// A wxGraphicsPath is a native representation of a geometric path.
    /// - [`GraphicsPath`] represents a C++ `wxGraphicsPath` class instance which your code has ownership, [`GraphicsPathInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsPath`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsPath` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html) for more details.
    #[doc(alias = "wxGraphicsPath")]
    #[doc(alias = "GraphicsPath")]
    class GraphicsPath
        = GraphicsPathInRust<true>(wxGraphicsPath) impl
        GraphicsPathMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsPathInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsPathInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsPathInRust<OWNED>> for GraphicsObjectInRust<OWNED> {
    fn from(o: GraphicsPathInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsPathInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsPathInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsPathInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsPath_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsPathInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPen
wxwidgets! {
    /// A wxGraphicsPen is a native representation of a pen.
    /// - [`GraphicsPen`] represents a C++ `wxGraphicsPen` class instance which your code has ownership, [`GraphicsPenInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsPen`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsPen` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_pen.html) for more details.
    #[doc(alias = "wxGraphicsPen")]
    #[doc(alias = "GraphicsPen")]
    class GraphicsPen
        = GraphicsPenInRust<true>(wxGraphicsPen) impl
        GraphicsPenMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsPenInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsPenInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsPenInRust<OWNED>> for GraphicsObjectInRust<OWNED> {
    fn from(o: GraphicsPenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GraphicsPenInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsPenInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsPenInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsPen_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsPenInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsRenderer
wxwidgets! {
    /// A wxGraphicsRenderer is the instance corresponding to the rendering engine used.
    /// - [`GraphicsRenderer`] represents a C++ `wxGraphicsRenderer` class instance which your code has ownership, [`GraphicsRendererInRust`]`<false>` represents one which don't own.
    /// - Use [`GraphicsRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html) for more details.
    #[doc(alias = "wxGraphicsRenderer")]
    #[doc(alias = "GraphicsRenderer")]
    class GraphicsRenderer
        = GraphicsRendererInRust<true>(wxGraphicsRenderer) impl
        GraphicsRendererMethods,
        ObjectMethods
}
impl<const OWNED: bool> GraphicsRendererInRust<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsRendererInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GraphicsRendererInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GraphicsRendererInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GraphicsRendererInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGraphicsRenderer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GraphicsRendererInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridBagSizer
wxwidgets! {
    /// A wxSizer that can lay out items in a virtual grid like a wxFlexGridSizer but in this case explicit positioning of the items is allowed using wxGBPosition, and items can optionally span more than one row and/or column using wxGBSpan.
    /// - [`GridBagSizer`] represents a C++ `wxGridBagSizer` class instance which your code has ownership, [`GridBagSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`GridBagSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridBagSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html) for more details.
    #[doc(alias = "wxGridBagSizer")]
    #[doc(alias = "GridBagSizer")]
    class GridBagSizer
        = GridBagSizerInRust<true>(wxGridBagSizer) impl
        GridBagSizerMethods,
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridBagSizerInRust<OWNED> {
    /// Constructor, with optional parameters to specify the gap between the rows and columns.
    ///
    /// See [C++ `wxGridBagSizer::wxGridBagSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a22172ccf78ed632648760f8473bc121f).
    pub fn new(vgap: c_int, hgap: c_int) -> GridBagSizerInRust<OWNED> {
        unsafe { GridBagSizerInRust(ffi::wxGridBagSizer_new(vgap, hgap)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GridBagSizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridBagSizerInRust<OWNED>> for FlexGridSizerInRust<OWNED> {
    fn from(o: GridBagSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerInRust<OWNED>> for GridSizerInRust<OWNED> {
    fn from(o: GridBagSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerInRust<OWNED>> for SizerInRust<OWNED> {
    fn from(o: GridBagSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridBagSizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridBagSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridBagSizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridBagSizer_CLASSINFO()) }
    }
}

// wxGridEditorCreatedEvent
wxwidgets! {
    ///
    /// - [`GridEditorCreatedEvent`] represents a C++ `wxGridEditorCreatedEvent` class instance which your code has ownership, [`GridEditorCreatedEventInRust`]`<false>` represents one which don't own.
    /// - Use [`GridEditorCreatedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridEditorCreatedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html) for more details.
    #[doc(alias = "wxGridEditorCreatedEvent")]
    #[doc(alias = "GridEditorCreatedEvent")]
    class GridEditorCreatedEvent
        = GridEditorCreatedEventInRust<true>(wxGridEditorCreatedEvent) impl
        GridEditorCreatedEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridEditorCreatedEventInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::wxGridEditorCreatedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a311a9e5fe97c903d94a5d6ed84c80c76).
    pub fn new() -> GridEditorCreatedEventInRust<OWNED> {
        unsafe { GridEditorCreatedEventInRust(ffi::wxGridEditorCreatedEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEditorCreatedEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridEditorCreatedEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: GridEditorCreatedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: GridEditorCreatedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEditorCreatedEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridEditorCreatedEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridEditorCreatedEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridEditorCreatedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridEditorCreatedEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridEvent
wxwidgets! {
    /// This event class contains information about various grid events.
    /// - [`GridEvent`] represents a C++ `wxGridEvent` class instance which your code has ownership, [`GridEventInRust`]`<false>` represents one which don't own.
    /// - Use [`GridEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html) for more details.
    #[doc(alias = "wxGridEvent")]
    #[doc(alias = "GridEvent")]
    class GridEvent
        = GridEventInRust<true>(wxGridEvent) impl
        GridEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridEventInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridEvent::wxGridEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a05e8831def820bf32a37693f500bf78d).
    pub fn new() -> GridEventInRust<OWNED> {
        unsafe { GridEventInRust(ffi::wxGridEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: GridEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: GridEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: GridEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridRangeSelectEvent
wxwidgets! {
    /// Events of this class notify about a range of cells being selected.
    /// - [`GridRangeSelectEvent`] represents a C++ `wxGridRangeSelectEvent` class instance which your code has ownership, [`GridRangeSelectEventInRust`]`<false>` represents one which don't own.
    /// - Use [`GridRangeSelectEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridRangeSelectEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html) for more details.
    #[doc(alias = "wxGridRangeSelectEvent")]
    #[doc(alias = "GridRangeSelectEvent")]
    class GridRangeSelectEvent
        = GridRangeSelectEventInRust<true>(wxGridRangeSelectEvent) impl
        GridRangeSelectEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridRangeSelectEventInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridRangeSelectEvent::wxGridRangeSelectEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a6739617b035d38ed793faf2994cf4a40).
    pub fn new() -> GridRangeSelectEventInRust<OWNED> {
        unsafe { GridRangeSelectEventInRust(ffi::wxGridRangeSelectEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridRangeSelectEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridRangeSelectEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: GridRangeSelectEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: GridRangeSelectEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: GridRangeSelectEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridRangeSelectEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridRangeSelectEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridRangeSelectEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridRangeSelectEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridRangeSelectEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizeEvent
wxwidgets! {
    /// This event class contains information about a row/column resize event.
    /// - [`GridSizeEvent`] represents a C++ `wxGridSizeEvent` class instance which your code has ownership, [`GridSizeEventInRust`]`<false>` represents one which don't own.
    /// - Use [`GridSizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridSizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html) for more details.
    #[doc(alias = "wxGridSizeEvent")]
    #[doc(alias = "GridSizeEvent")]
    class GridSizeEvent
        = GridSizeEventInRust<true>(wxGridSizeEvent) impl
        GridSizeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridSizeEventInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridSizeEvent::wxGridSizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#a183ad53c5c01f3c21b7c0b546845731d).
    pub fn new() -> GridSizeEventInRust<OWNED> {
        unsafe { GridSizeEventInRust(ffi::wxGridSizeEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridSizeEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridSizeEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridSizeEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: GridSizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: GridSizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: GridSizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizeEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridSizeEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridSizeEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridSizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridSizeEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizer
wxwidgets! {
    /// A grid sizer is a sizer which lays out its children in a two-dimensional table with all table fields having the same size, i.e.
    /// - [`GridSizer`] represents a C++ `wxGridSizer` class instance which your code has ownership, [`GridSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`GridSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html) for more details.
    #[doc(alias = "wxGridSizer")]
    #[doc(alias = "GridSizer")]
    class GridSizer
        = GridSizerInRust<true>(wxGridSizer) impl
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridSizerInRust<OWNED> {
    /// wxGridSizer constructors.
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a54163c431608c0cbe6c74c9009ef1ca2).
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> GridSizerInRust<OWNED> {
        unsafe { GridSizerInRust(ffi::wxGridSizer_new(cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a6b0493be4197aff5bf59f1e68676d711).
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> GridSizerInRust<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerInRust(ffi::wxGridSizer_new1(cols, gap))
        }
    }
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a1f0a163a3a216ed647452a31951b66bd).
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> GridSizerInRust<OWNED> {
        unsafe { GridSizerInRust(ffi::wxGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#afa96e5d782116e2418ee440724b1a312).
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> GridSizerInRust<OWNED> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerInRust(ffi::wxGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for GridSizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridSizerInRust<OWNED>> for SizerInRust<OWNED> {
    fn from(o: GridSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<GridSizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridSizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridSizer_CLASSINFO()) }
    }
}

// wxGridTableBase
wxwidgets! {
    /// The almost abstract base class for grid tables.
    /// - [`GridTableBase`] represents a C++ `wxGridTableBase` class instance which your code has ownership, [`GridTableBaseInRust`]`<false>` represents one which don't own.
    /// - Use [`GridTableBase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridTableBase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html) for more details.
    #[doc(alias = "wxGridTableBase")]
    #[doc(alias = "GridTableBase")]
    class GridTableBase
        = GridTableBaseInRust<true>(wxGridTableBase) impl
        GridTableBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> GridTableBaseInRust<OWNED> {
    // BLOCKED: fn wxGridTableBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridTableBaseInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<GridTableBaseInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: GridTableBaseInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for GridTableBaseInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxGridTableBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for GridTableBaseInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridUpdateLocker
wxwidgets! {
    /// This small class can be used to prevent wxGrid from redrawing during its lifetime by calling wxGrid::BeginBatch() in its constructor and wxGrid::EndBatch() in its destructor.
    /// - [`GridUpdateLocker`] represents a C++ `wxGridUpdateLocker` class instance which your code has ownership, [`GridUpdateLockerInRust`]`<false>` represents one which don't own.
    /// - Use [`GridUpdateLocker`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridUpdateLocker` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html) for more details.
    #[doc(alias = "wxGridUpdateLocker")]
    #[doc(alias = "GridUpdateLocker")]
    class GridUpdateLocker
        = GridUpdateLockerInRust<true>(wxGridUpdateLocker) impl
        GridUpdateLockerMethods
}
impl<const OWNED: bool> GridUpdateLockerInRust<OWNED> {
    /// Creates an object preventing the updates of the specified grid.
    ///
    /// See [C++ `wxGridUpdateLocker::wxGridUpdateLocker()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html#a13605827243de9ed1c0864fbd055cb8f).
    pub fn new(grid: *mut c_void) -> GridUpdateLockerInRust<OWNED> {
        unsafe { GridUpdateLockerInRust(ffi::wxGridUpdateLocker_new(grid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridUpdateLockerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for GridUpdateLockerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxGridUpdateLocker_delete(self.0) }
        }
    }
}
