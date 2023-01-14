use super::*;

// wxGBPosition
wxwidgets! {
    /// This class represents the position of an item in a virtual grid of rows and columns managed by a wxGridBagSizer.
    /// - [`GBPosition`] represents a C++ `wxGBPosition` class instance which your code has ownership, [`GBPositionFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GBPosition`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGBPosition` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html) for more details.
    #[doc(alias = "wxGBPosition")]
    #[doc(alias = "GBPosition")]
    class GBPosition
        = GBPositionFromCpp<false>(wxGBPosition) impl
        GBPositionMethods
}
impl<const FROM_CPP: bool> GBPositionFromCpp<FROM_CPP> {
    /// Default constructor, setting the row and column to (0,0).
    ///
    /// See [C++ `wxGBPosition::wxGBPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a88ebe51f95578714548e4a823fcb164e).
    pub fn new() -> GBPositionFromCpp<FROM_CPP> {
        unsafe { GBPositionFromCpp(ffi::wxGBPosition_new()) }
    }
    /// Construct a new wxGBPosition, setting the row and column.
    ///
    /// See [C++ `wxGBPosition::wxGBPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_position.html#a6723ac451dac191b78e2a97aabf39e2b).
    pub fn new_with_int(row: c_int, col: c_int) -> GBPositionFromCpp<FROM_CPP> {
        unsafe { GBPositionFromCpp(ffi::wxGBPosition_new1(row, col)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBPositionFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for GBPositionFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxGBPosition_delete(self.0) }
        }
    }
}

// wxGBSizerItem
wxwidgets! {
    /// The wxGBSizerItem class is used by the wxGridBagSizer for tracking the items in the sizer.
    /// - [`GBSizerItem`] represents a C++ `wxGBSizerItem` class instance which your code has ownership, [`GBSizerItemFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GBSizerItem`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGBSizerItem` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_sizer_item.html) for more details.
    #[doc(alias = "wxGBSizerItem")]
    #[doc(alias = "GBSizerItem")]
    class GBSizerItem
        = GBSizerItemFromCpp<false>(wxGBSizerItem) impl
        GBSizerItemMethods,
        SizerItemMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GBSizerItemFromCpp<FROM_CPP> {
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
    ) -> GBSizerItemFromCpp<FROM_CPP> {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItemFromCpp(ffi::wxGBSizerItem_new(
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
    ) -> GBSizerItemFromCpp<FROM_CPP> {
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
            GBSizerItemFromCpp(ffi::wxGBSizerItem_new1(
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
    ) -> GBSizerItemFromCpp<FROM_CPP> {
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
            GBSizerItemFromCpp(ffi::wxGBSizerItem_new2(
                sizer, pos, span, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBSizerItemFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GBSizerItemFromCpp<FROM_CPP>> for SizerItemFromCpp<FROM_CPP> {
    fn from(o: GBSizerItemFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GBSizerItemFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GBSizerItemFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GBSizerItemFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGBSizerItem_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GBSizerItemFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGBSpan
wxwidgets! {
    /// This class is used to hold the row and column spanning attributes of items in a wxGridBagSizer.
    /// - [`GBSpan`] represents a C++ `wxGBSpan` class instance which your code has ownership, [`GBSpanFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GBSpan`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGBSpan` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html) for more details.
    #[doc(alias = "wxGBSpan")]
    #[doc(alias = "GBSpan")]
    class GBSpan
        = GBSpanFromCpp<false>(wxGBSpan) impl
        GBSpanMethods
}
impl<const FROM_CPP: bool> GBSpanFromCpp<FROM_CPP> {
    /// Default constructor, setting the rowspan and colspan to (1,1) meaning that the item occupies one cell in each direction.
    ///
    /// See [C++ `wxGBSpan::wxGBSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a8f5790cd566fa20328c97420f5317e93).
    pub fn new() -> GBSpanFromCpp<FROM_CPP> {
        unsafe { GBSpanFromCpp(ffi::wxGBSpan_new()) }
    }
    /// Construct a new wxGBSpan, setting the rowspan and colspan.
    ///
    /// See [C++ `wxGBSpan::wxGBSpan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_b_span.html#a51a3cdf14f004d4f5b8e33d20b7ea636).
    pub fn new_with_int(rowspan: c_int, colspan: c_int) -> GBSpanFromCpp<FROM_CPP> {
        unsafe { GBSpanFromCpp(ffi::wxGBSpan_new1(rowspan, colspan)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GBSpanFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for GBSpanFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxGBSpan_delete(self.0) }
        }
    }
}

// wxGCDC
wxwidgets! {
    /// wxGCDC is a device context that draws on a wxGraphicsContext.
    /// - [`GCDC`] represents a C++ `wxGCDC` class instance which your code has ownership, [`GCDCFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GCDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGCDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html) for more details.
    #[doc(alias = "wxGCDC")]
    #[doc(alias = "GCDC")]
    class GCDC
        = GCDCFromCpp<false>(wxGCDC) impl
        GCDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GCDCFromCpp<FROM_CPP> {
    /// Constructs a wxGCDC from a wxWindowDC.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#ab7f82c4acbe2deba41375cce01bcaba3).
    pub fn new_with_windowdc<W: WindowDCMethods>(window_dc: &W) -> GCDCFromCpp<FROM_CPP> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GCDCFromCpp(ffi::wxGCDC_new(window_dc))
        }
    }
    /// Constructs a wxGCDC from a wxMemoryDC.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a8bfc3aa5028e38bc5e63b797fef63669).
    pub fn new_with_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> GCDCFromCpp<FROM_CPP> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GCDCFromCpp(ffi::wxGCDC_new1(memory_dc))
        }
    }
    // BLOCKED: fn wxGCDC2()
    /// Construct a wxGCDC from an existing graphics context.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a44937b71c3e256a1e2c2187caab904f8).
    pub fn new_with_graphicscontext<G: GraphicsContextMethods>(
        context: Option<&G>,
    ) -> GCDCFromCpp<FROM_CPP> {
        unsafe {
            let context = match context {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GCDCFromCpp(ffi::wxGCDC_new3(context))
        }
    }
    /// Constructs a wxGCDC from a wxEnhMetaFileDC.
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a96469646f16d67d838fc49500b7af158).
    pub fn new_with_enhmetafiledc(emf_dc: *const c_void) -> GCDCFromCpp<FROM_CPP> {
        unsafe { GCDCFromCpp(ffi::wxGCDC_new4(emf_dc)) }
    }
    ///
    /// See [C++ `wxGCDC::wxGCDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_c_d_c.html#a655c7b2351ba8ee71cec659030a0fb59).
    pub fn new() -> GCDCFromCpp<FROM_CPP> {
        unsafe { GCDCFromCpp(ffi::wxGCDC_new5()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GCDCFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GCDCFromCpp<FROM_CPP>> for DCFromCpp<FROM_CPP> {
    fn from(o: GCDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GCDCFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GCDCFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GCDCFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGCDC_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GCDCFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGDIObject
wxwidgets! {
    /// This class allows platforms to implement functionality to optimise GDI objects, such as wxPen, wxBrush and wxFont.
    /// - [`GDIObject`] represents a C++ `wxGDIObject` class instance which your code has ownership, [`GDIObjectFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GDIObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGDIObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_d_i_object.html) for more details.
    #[doc(alias = "wxGDIObject")]
    #[doc(alias = "GDIObject")]
    class GDIObject
        = GDIObjectFromCpp<false>(wxGDIObject) impl
        GDIObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GDIObjectFromCpp<FROM_CPP> {
    // BLOCKED: fn wxGDIObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GDIObjectFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GDIObjectFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GDIObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GDIObjectFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGDIObject_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GDIObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGIFHandler
wxwidgets! {
    /// This is the image handler for the GIF format.
    /// - [`GIFHandler`] represents a C++ `wxGIFHandler` class instance which your code has ownership, [`GIFHandlerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GIFHandler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGIFHandler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html) for more details.
    #[doc(alias = "wxGIFHandler")]
    #[doc(alias = "GIFHandler")]
    class GIFHandler
        = GIFHandlerFromCpp<false>(wxGIFHandler) impl
        GIFHandlerMethods,
        ImageHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GIFHandlerFromCpp<FROM_CPP> {
    /// Default constructor for wxGIFHandler.
    ///
    /// See [C++ `wxGIFHandler::wxGIFHandler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_g_i_f_handler.html#adbd198504747365e2d2ef232880546ce).
    pub fn new() -> GIFHandlerFromCpp<FROM_CPP> {
        unsafe { GIFHandlerFromCpp(ffi::wxGIFHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GIFHandlerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GIFHandlerFromCpp<FROM_CPP>> for ImageHandlerFromCpp<FROM_CPP> {
    fn from(o: GIFHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GIFHandlerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GIFHandlerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GIFHandlerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGIFHandler_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GIFHandlerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGauge
wxwidgets! {
    /// A gauge is a horizontal or vertical bar which shows a quantity (often time).
    /// - [`Gauge`] represents a C++ `wxGauge` class instance which your code has ownership, [`GaugeFromCpp`]`<true>` represents one which don't own.
    /// - Use [`Gauge`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGauge` class's documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html) for more details.
    #[doc(alias = "wxGauge")]
    #[doc(alias = "Gauge")]
    class Gauge
        = GaugeFromCpp<false>(wxGauge) impl
        GaugeMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GaugeFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxGauge::wxGauge()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_gauge.html#a8bf73ec5e07533060f2579c72b8fc262).
    pub fn new_2step() -> GaugeFromCpp<FROM_CPP> {
        unsafe { GaugeFromCpp(ffi::wxGauge_new()) }
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
    ) -> GaugeFromCpp<FROM_CPP> {
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
            GaugeFromCpp(ffi::wxGauge_new1(
                parent, id, range, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for GaugeFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GaugeFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: GaugeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GaugeFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: GaugeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GaugeFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: GaugeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GaugeFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GaugeFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GaugeFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGauge_CLASSINFO()) }
    }
}

// wxGenericAboutDialog
wxwidgets! {
    /// This class defines a customizable About dialog.
    /// - [`GenericAboutDialog`] represents a C++ `wxGenericAboutDialog` class instance which your code has ownership, [`GenericAboutDialogFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GenericAboutDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericAboutDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html) for more details.
    #[doc(alias = "wxGenericAboutDialog")]
    #[doc(alias = "GenericAboutDialog")]
    class GenericAboutDialog
        = GenericAboutDialogFromCpp<false>(wxGenericAboutDialog) impl
        GenericAboutDialogMethods
}
impl<const FROM_CPP: bool> GenericAboutDialogFromCpp<FROM_CPP> {
    /// Default constructor, Create() must be called later.
    ///
    /// See [C++ `wxGenericAboutDialog::wxGenericAboutDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#ad195f2e454ba722956b73e791e1c6a03).
    pub fn new() -> GenericAboutDialogFromCpp<FROM_CPP> {
        unsafe { GenericAboutDialogFromCpp(ffi::wxGenericAboutDialog_new()) }
    }
    /// Creates the dialog and initializes it with the given information.
    ///
    /// See [C++ `wxGenericAboutDialog::wxGenericAboutDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_about_dialog.html#a219d9040ec0e3ef091d25ed7e865e262).
    pub fn new_with_aboutdialoginfo<A: AboutDialogInfoMethods, W: WindowMethods>(
        info: &A,
        parent: Option<&W>,
    ) -> GenericAboutDialogFromCpp<FROM_CPP> {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericAboutDialogFromCpp(ffi::wxGenericAboutDialog_new1(info, parent))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GenericAboutDialogFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for GenericAboutDialogFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxGenericAboutDialog_delete(self.0) }
        }
    }
}

// wxGenericDirCtrl
wxwidgets! {
    /// This control can be used to place a directory listing (with optional files) on an arbitrary window.
    /// - [`GenericDirCtrl`] represents a C++ `wxGenericDirCtrl` class instance which your code has ownership, [`GenericDirCtrlFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GenericDirCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericDirCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html) for more details.
    #[doc(alias = "wxGenericDirCtrl")]
    #[doc(alias = "GenericDirCtrl")]
    class GenericDirCtrl
        = GenericDirCtrlFromCpp<false>(wxGenericDirCtrl) impl
        GenericDirCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GenericDirCtrlFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxGenericDirCtrl::wxGenericDirCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_dir_ctrl.html#a1072f4e29922e08f7e8a288e573bec5a).
    pub fn new_2step() -> GenericDirCtrlFromCpp<FROM_CPP> {
        unsafe { GenericDirCtrlFromCpp(ffi::wxGenericDirCtrl_new()) }
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
    ) -> GenericDirCtrlFromCpp<FROM_CPP> {
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
            GenericDirCtrlFromCpp(ffi::wxGenericDirCtrl_new1(
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
impl<const FROM_CPP: bool> Clone for GenericDirCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GenericDirCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: GenericDirCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericDirCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: GenericDirCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericDirCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: GenericDirCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericDirCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GenericDirCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GenericDirCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGenericDirCtrl_CLASSINFO()) }
    }
}

// wxGenericProgressDialog
wxwidgets! {
    /// This class represents a dialog that shows a short message and a progress bar.
    /// - [`GenericProgressDialog`] represents a C++ `wxGenericProgressDialog` class instance which your code has ownership, [`GenericProgressDialogFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GenericProgressDialog`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericProgressDialog` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html) for more details.
    #[doc(alias = "wxGenericProgressDialog")]
    #[doc(alias = "GenericProgressDialog")]
    class GenericProgressDialog
        = GenericProgressDialogFromCpp<false>(wxGenericProgressDialog) impl
        GenericProgressDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GenericProgressDialogFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxGenericProgressDialog::wxGenericProgressDialog()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_progress_dialog.html#ac015aa72408dcddef95f4f575bb628bc).
    pub fn new<W: WindowMethods>(
        title: &str,
        message: &str,
        maximum: c_int,
        parent: Option<&W>,
        style: c_int,
    ) -> GenericProgressDialogFromCpp<FROM_CPP> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericProgressDialogFromCpp(ffi::wxGenericProgressDialog_new(
                title, message, maximum, parent, style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for GenericProgressDialogFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GenericProgressDialogFromCpp<FROM_CPP>>
    for DialogFromCpp<FROM_CPP>
{
    fn from(o: GenericProgressDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericProgressDialogFromCpp<FROM_CPP>>
    for TopLevelWindowFromCpp<FROM_CPP>
{
    fn from(o: GenericProgressDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericProgressDialogFromCpp<FROM_CPP>>
    for NonOwnedWindowFromCpp<FROM_CPP>
{
    fn from(o: GenericProgressDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericProgressDialogFromCpp<FROM_CPP>>
    for WindowFromCpp<FROM_CPP>
{
    fn from(o: GenericProgressDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericProgressDialogFromCpp<FROM_CPP>>
    for EvtHandlerFromCpp<FROM_CPP>
{
    fn from(o: GenericProgressDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericProgressDialogFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: GenericProgressDialogFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GenericProgressDialogFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGenericProgressDialog_CLASSINFO()) }
    }
}

// wxGenericValidator
wxwidgets! {
    /// wxGenericValidator performs data transfer (but not validation or filtering) for many type of controls.
    /// - [`GenericValidator`] represents a C++ `wxGenericValidator` class instance which your code has ownership, [`GenericValidatorFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GenericValidator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGenericValidator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html) for more details.
    #[doc(alias = "wxGenericValidator")]
    #[doc(alias = "GenericValidator")]
    class GenericValidator
        = GenericValidatorFromCpp<false>(wxGenericValidator) impl
        GenericValidatorMethods,
        ValidatorMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GenericValidatorFromCpp<FROM_CPP> {
    /// Copy constructor.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a25de71f90148ffe51d947dabdc15b01f).
    pub fn new_with_genericvalidator<G: GenericValidatorMethods>(
        validator: &G,
    ) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe {
            let validator = validator.as_ptr();
            GenericValidatorFromCpp(ffi::wxGenericValidator_new(validator))
        }
    }
    /// Constructor taking a bool pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#af21eecfcd12693a2ba9087cf984a8779).
    pub fn new_with_bool(val_ptr: *mut c_void) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe { GenericValidatorFromCpp(ffi::wxGenericValidator_new1(val_ptr)) }
    }
    /// Constructor taking a wxString pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#ae408126b38460f71f34645fb6a7cba28).
    pub fn new_with_str(val_ptr: *mut c_void) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe { GenericValidatorFromCpp(ffi::wxGenericValidator_new2(val_ptr)) }
    }
    /// Constructor taking an integer pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#abce0743e3f83f4119566661e76652dcc).
    pub fn new_with_int(val_ptr: *mut c_void) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe { GenericValidatorFromCpp(ffi::wxGenericValidator_new3(val_ptr)) }
    }
    /// Constructor taking a wxArrayInt pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a1b08bdcfa0a194d224579914da51d210).
    pub fn new_with_arrayint<A: ArrayIntMethods>(
        val_ptr: Option<&A>,
    ) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorFromCpp(ffi::wxGenericValidator_new4(val_ptr))
        }
    }
    /// Constructor taking a wxDateTime pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#ae9716ab95a262e8c02bc6abc64558a13).
    pub fn new_with_datetime<D: DateTimeMethods>(
        val_ptr: Option<&D>,
    ) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorFromCpp(ffi::wxGenericValidator_new5(val_ptr))
        }
    }
    /// Constructor taking a wxFileName pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a53edd5f4520573fbb9de25b281d32f2f).
    pub fn new_with_filename<F: FileNameMethods>(
        val_ptr: Option<&F>,
    ) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe {
            let val_ptr = match val_ptr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GenericValidatorFromCpp(ffi::wxGenericValidator_new6(val_ptr))
        }
    }
    /// Constructor taking a float pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a62ddb0362276e41bf84a547b015554b0).
    pub fn new_with_float(val_ptr: *mut c_void) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe { GenericValidatorFromCpp(ffi::wxGenericValidator_new7(val_ptr)) }
    }
    /// Constructor taking a double pointer.
    ///
    /// See [C++ `wxGenericValidator::wxGenericValidator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_generic_validator.html#a66e90c6da5e71e7258a2581370b92795).
    pub fn new_with_double(val_ptr: *mut c_void) -> GenericValidatorFromCpp<FROM_CPP> {
        unsafe { GenericValidatorFromCpp(ffi::wxGenericValidator_new8(val_ptr)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for GenericValidatorFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GenericValidatorFromCpp<FROM_CPP>> for ValidatorFromCpp<FROM_CPP> {
    fn from(o: GenericValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericValidatorFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: GenericValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GenericValidatorFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GenericValidatorFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GenericValidatorFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGenericValidator_CLASSINFO()) }
    }
}

// wxGraphicsBrush
wxwidgets! {
    /// A wxGraphicsBrush is a native representation of a brush.
    /// - [`GraphicsBrush`] represents a C++ `wxGraphicsBrush` class instance which your code has ownership, [`GraphicsBrushFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsBrush`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsBrush` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_brush.html) for more details.
    #[doc(alias = "wxGraphicsBrush")]
    #[doc(alias = "GraphicsBrush")]
    class GraphicsBrush
        = GraphicsBrushFromCpp<false>(wxGraphicsBrush) impl
        GraphicsBrushMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsBrushFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsBrushFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsBrushFromCpp<FROM_CPP>>
    for GraphicsObjectFromCpp<FROM_CPP>
{
    fn from(o: GraphicsBrushFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GraphicsBrushFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsBrushFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsBrushFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsBrush_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsBrushFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsContext
wxwidgets! {
    /// A wxGraphicsContext instance is the object that is drawn upon.
    /// - [`GraphicsContext`] represents a C++ `wxGraphicsContext` class instance which your code has ownership, [`GraphicsContextFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsContext`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsContext` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_context.html) for more details.
    #[doc(alias = "wxGraphicsContext")]
    #[doc(alias = "GraphicsContext")]
    class GraphicsContext
        = GraphicsContextFromCpp<false>(wxGraphicsContext) impl
        GraphicsContextMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsContextFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsContextFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsContextFromCpp<FROM_CPP>>
    for GraphicsObjectFromCpp<FROM_CPP>
{
    fn from(o: GraphicsContextFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GraphicsContextFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsContextFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsContextFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsContext_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsContextFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsFont
wxwidgets! {
    /// A wxGraphicsFont is a native representation of a font.
    /// - [`GraphicsFont`] represents a C++ `wxGraphicsFont` class instance which your code has ownership, [`GraphicsFontFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsFont`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsFont` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_font.html) for more details.
    #[doc(alias = "wxGraphicsFont")]
    #[doc(alias = "GraphicsFont")]
    class GraphicsFont
        = GraphicsFontFromCpp<false>(wxGraphicsFont) impl
        GraphicsFontMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsFontFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsFontFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsFontFromCpp<FROM_CPP>> for GraphicsObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsFontFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GraphicsFontFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsFontFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsFontFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsFont_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsFontFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStop
wxwidgets! {
    /// Represents a single gradient stop in a collection of gradient stops as represented by wxGraphicsGradientStops.
    /// - [`GraphicsGradientStop`] represents a C++ `wxGraphicsGradientStop` class instance which your code has ownership, [`GraphicsGradientStopFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsGradientStop`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsGradientStop` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stop.html) for more details.
    #[doc(alias = "wxGraphicsGradientStop")]
    #[doc(alias = "GraphicsGradientStop")]
    class GraphicsGradientStop
        = GraphicsGradientStopFromCpp<false>(wxGraphicsGradientStop) impl
        GraphicsGradientStopMethods
}
impl<const FROM_CPP: bool> GraphicsGradientStopFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxGraphicsGradientStop()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsGradientStopFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsGradientStopFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxGraphicsGradientStop_delete(self.0) }
        }
    }
}

// wxGraphicsGradientStops
wxwidgets! {
    /// Represents a collection of wxGraphicGradientStop values for use with CreateLinearGradientBrush and CreateRadialGradientBrush.
    /// - [`GraphicsGradientStops`] represents a C++ `wxGraphicsGradientStops` class instance which your code has ownership, [`GraphicsGradientStopsFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsGradientStops`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsGradientStops` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_gradient_stops.html) for more details.
    #[doc(alias = "wxGraphicsGradientStops")]
    #[doc(alias = "GraphicsGradientStops")]
    class GraphicsGradientStops
        = GraphicsGradientStopsFromCpp<false>(wxGraphicsGradientStops) impl
        GraphicsGradientStopsMethods
}
impl<const FROM_CPP: bool> GraphicsGradientStopsFromCpp<FROM_CPP> {
    // BLOCKED: fn wxGraphicsGradientStops()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsGradientStopsFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsGradientStopsFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxGraphicsGradientStops_delete(self.0) }
        }
    }
}

// wxGraphicsMatrix
wxwidgets! {
    /// A wxGraphicsMatrix is a native representation of an affine matrix.
    /// - [`GraphicsMatrix`] represents a C++ `wxGraphicsMatrix` class instance which your code has ownership, [`GraphicsMatrixFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsMatrix`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsMatrix` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_matrix.html) for more details.
    #[doc(alias = "wxGraphicsMatrix")]
    #[doc(alias = "GraphicsMatrix")]
    class GraphicsMatrix
        = GraphicsMatrixFromCpp<false>(wxGraphicsMatrix) impl
        GraphicsMatrixMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsMatrixFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsMatrixFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsMatrixFromCpp<FROM_CPP>>
    for GraphicsObjectFromCpp<FROM_CPP>
{
    fn from(o: GraphicsMatrixFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GraphicsMatrixFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsMatrixFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsMatrixFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsMatrix_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsMatrixFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsObject
wxwidgets! {
    /// This class is the superclass of native graphics objects like pens etc.
    /// - [`GraphicsObject`] represents a C++ `wxGraphicsObject` class instance which your code has ownership, [`GraphicsObjectFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_object.html) for more details.
    #[doc(alias = "wxGraphicsObject")]
    #[doc(alias = "GraphicsObject")]
    class GraphicsObject
        = GraphicsObjectFromCpp<false>(wxGraphicsObject) impl
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsObjectFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsObjectFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsObjectFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsObjectFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsObject_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPath
wxwidgets! {
    /// A wxGraphicsPath is a native representation of a geometric path.
    /// - [`GraphicsPath`] represents a C++ `wxGraphicsPath` class instance which your code has ownership, [`GraphicsPathFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsPath`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsPath` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_path.html) for more details.
    #[doc(alias = "wxGraphicsPath")]
    #[doc(alias = "GraphicsPath")]
    class GraphicsPath
        = GraphicsPathFromCpp<false>(wxGraphicsPath) impl
        GraphicsPathMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsPathFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsPathFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsPathFromCpp<FROM_CPP>> for GraphicsObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsPathFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GraphicsPathFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsPathFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsPathFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsPath_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsPathFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsPen
wxwidgets! {
    /// A wxGraphicsPen is a native representation of a pen.
    /// - [`GraphicsPen`] represents a C++ `wxGraphicsPen` class instance which your code has ownership, [`GraphicsPenFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsPen`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsPen` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_pen.html) for more details.
    #[doc(alias = "wxGraphicsPen")]
    #[doc(alias = "GraphicsPen")]
    class GraphicsPen
        = GraphicsPenFromCpp<false>(wxGraphicsPen) impl
        GraphicsPenMethods,
        GraphicsObjectMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsPenFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsPenFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsPenFromCpp<FROM_CPP>> for GraphicsObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsPenFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GraphicsPenFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsPenFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsPenFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsPen_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsPenFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGraphicsRenderer
wxwidgets! {
    /// A wxGraphicsRenderer is the instance corresponding to the rendering engine used.
    /// - [`GraphicsRenderer`] represents a C++ `wxGraphicsRenderer` class instance which your code has ownership, [`GraphicsRendererFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GraphicsRenderer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGraphicsRenderer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_graphics_renderer.html) for more details.
    #[doc(alias = "wxGraphicsRenderer")]
    #[doc(alias = "GraphicsRenderer")]
    class GraphicsRenderer
        = GraphicsRendererFromCpp<false>(wxGraphicsRenderer) impl
        GraphicsRendererMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GraphicsRendererFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GraphicsRendererFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GraphicsRendererFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GraphicsRendererFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GraphicsRendererFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGraphicsRenderer_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GraphicsRendererFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridBagSizer
wxwidgets! {
    /// A wxSizer that can lay out items in a virtual grid like a wxFlexGridSizer but in this case explicit positioning of the items is allowed using wxGBPosition, and items can optionally span more than one row and/or column using wxGBSpan.
    /// - [`GridBagSizer`] represents a C++ `wxGridBagSizer` class instance which your code has ownership, [`GridBagSizerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridBagSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridBagSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html) for more details.
    #[doc(alias = "wxGridBagSizer")]
    #[doc(alias = "GridBagSizer")]
    class GridBagSizer
        = GridBagSizerFromCpp<false>(wxGridBagSizer) impl
        GridBagSizerMethods,
        FlexGridSizerMethods,
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridBagSizerFromCpp<FROM_CPP> {
    /// Constructor, with optional parameters to specify the gap between the rows and columns.
    ///
    /// See [C++ `wxGridBagSizer::wxGridBagSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_bag_sizer.html#a22172ccf78ed632648760f8473bc121f).
    pub fn new(vgap: c_int, hgap: c_int) -> GridBagSizerFromCpp<FROM_CPP> {
        unsafe { GridBagSizerFromCpp(ffi::wxGridBagSizer_new(vgap, hgap)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for GridBagSizerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridBagSizerFromCpp<FROM_CPP>> for FlexGridSizerFromCpp<FROM_CPP> {
    fn from(o: GridBagSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridBagSizerFromCpp<FROM_CPP>> for GridSizerFromCpp<FROM_CPP> {
    fn from(o: GridBagSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridBagSizerFromCpp<FROM_CPP>> for SizerFromCpp<FROM_CPP> {
    fn from(o: GridBagSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridBagSizerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GridBagSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridBagSizerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridBagSizer_CLASSINFO()) }
    }
}

// wxGridEditorCreatedEvent
wxwidgets! {
    ///
    /// - [`GridEditorCreatedEvent`] represents a C++ `wxGridEditorCreatedEvent` class instance which your code has ownership, [`GridEditorCreatedEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridEditorCreatedEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridEditorCreatedEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html) for more details.
    #[doc(alias = "wxGridEditorCreatedEvent")]
    #[doc(alias = "GridEditorCreatedEvent")]
    class GridEditorCreatedEvent
        = GridEditorCreatedEventFromCpp<false>(wxGridEditorCreatedEvent) impl
        GridEditorCreatedEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridEditorCreatedEventFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridEditorCreatedEvent::wxGridEditorCreatedEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_editor_created_event.html#a311a9e5fe97c903d94a5d6ed84c80c76).
    pub fn new() -> GridEditorCreatedEventFromCpp<FROM_CPP> {
        unsafe { GridEditorCreatedEventFromCpp(ffi::wxGridEditorCreatedEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEditorCreatedEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridEditorCreatedEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridEditorCreatedEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: GridEditorCreatedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridEditorCreatedEventFromCpp<FROM_CPP>>
    for EventFromCpp<FROM_CPP>
{
    fn from(o: GridEditorCreatedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridEditorCreatedEventFromCpp<FROM_CPP>>
    for ObjectFromCpp<FROM_CPP>
{
    fn from(o: GridEditorCreatedEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridEditorCreatedEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridEditorCreatedEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GridEditorCreatedEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridEvent
wxwidgets! {
    /// This event class contains information about various grid events.
    /// - [`GridEvent`] represents a C++ `wxGridEvent` class instance which your code has ownership, [`GridEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html) for more details.
    #[doc(alias = "wxGridEvent")]
    #[doc(alias = "GridEvent")]
    class GridEvent
        = GridEventFromCpp<false>(wxGridEvent) impl
        GridEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridEventFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridEvent::wxGridEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_event.html#a05e8831def820bf32a37693f500bf78d).
    pub fn new() -> GridEventFromCpp<FROM_CPP> {
        unsafe { GridEventFromCpp(ffi::wxGridEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridEventFromCpp<FROM_CPP>> for NotifyEventFromCpp<FROM_CPP> {
    fn from(o: GridEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: GridEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: GridEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GridEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GridEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridRangeSelectEvent
wxwidgets! {
    /// Events of this class notify about a range of cells being selected.
    /// - [`GridRangeSelectEvent`] represents a C++ `wxGridRangeSelectEvent` class instance which your code has ownership, [`GridRangeSelectEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridRangeSelectEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridRangeSelectEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html) for more details.
    #[doc(alias = "wxGridRangeSelectEvent")]
    #[doc(alias = "GridRangeSelectEvent")]
    class GridRangeSelectEvent
        = GridRangeSelectEventFromCpp<false>(wxGridRangeSelectEvent) impl
        GridRangeSelectEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridRangeSelectEventFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridRangeSelectEvent::wxGridRangeSelectEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_range_select_event.html#a6739617b035d38ed793faf2994cf4a40).
    pub fn new() -> GridRangeSelectEventFromCpp<FROM_CPP> {
        unsafe { GridRangeSelectEventFromCpp(ffi::wxGridRangeSelectEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridRangeSelectEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridRangeSelectEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridRangeSelectEventFromCpp<FROM_CPP>>
    for NotifyEventFromCpp<FROM_CPP>
{
    fn from(o: GridRangeSelectEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridRangeSelectEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: GridRangeSelectEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridRangeSelectEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: GridRangeSelectEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridRangeSelectEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GridRangeSelectEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridRangeSelectEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridRangeSelectEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GridRangeSelectEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizeEvent
wxwidgets! {
    /// This event class contains information about a row/column resize event.
    /// - [`GridSizeEvent`] represents a C++ `wxGridSizeEvent` class instance which your code has ownership, [`GridSizeEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridSizeEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridSizeEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html) for more details.
    #[doc(alias = "wxGridSizeEvent")]
    #[doc(alias = "GridSizeEvent")]
    class GridSizeEvent
        = GridSizeEventFromCpp<false>(wxGridSizeEvent) impl
        GridSizeEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridSizeEventFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxGridSizeEvent::wxGridSizeEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_size_event.html#a183ad53c5c01f3c21b7c0b546845731d).
    pub fn new() -> GridSizeEventFromCpp<FROM_CPP> {
        unsafe { GridSizeEventFromCpp(ffi::wxGridSizeEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxGridSizeEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridSizeEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridSizeEventFromCpp<FROM_CPP>> for NotifyEventFromCpp<FROM_CPP> {
    fn from(o: GridSizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridSizeEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: GridSizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridSizeEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: GridSizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridSizeEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GridSizeEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridSizeEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridSizeEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GridSizeEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridSizer
wxwidgets! {
    /// A grid sizer is a sizer which lays out its children in a two-dimensional table with all table fields having the same size, i.e.
    /// - [`GridSizer`] represents a C++ `wxGridSizer` class instance which your code has ownership, [`GridSizerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html) for more details.
    #[doc(alias = "wxGridSizer")]
    #[doc(alias = "GridSizer")]
    class GridSizer
        = GridSizerFromCpp<false>(wxGridSizer) impl
        GridSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridSizerFromCpp<FROM_CPP> {
    /// wxGridSizer constructors.
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a54163c431608c0cbe6c74c9009ef1ca2).
    pub fn new_with_int_int(cols: c_int, vgap: c_int, hgap: c_int) -> GridSizerFromCpp<FROM_CPP> {
        unsafe { GridSizerFromCpp(ffi::wxGridSizer_new(cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a6b0493be4197aff5bf59f1e68676d711).
    pub fn new_with_size<S: SizeMethods>(cols: c_int, gap: &S) -> GridSizerFromCpp<FROM_CPP> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerFromCpp(ffi::wxGridSizer_new1(cols, gap))
        }
    }
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#a1f0a163a3a216ed647452a31951b66bd).
    pub fn new_with_int_int_int(
        rows: c_int,
        cols: c_int,
        vgap: c_int,
        hgap: c_int,
    ) -> GridSizerFromCpp<FROM_CPP> {
        unsafe { GridSizerFromCpp(ffi::wxGridSizer_new2(rows, cols, vgap, hgap)) }
    }
    ///
    /// See [C++ `wxGridSizer::wxGridSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_sizer.html#afa96e5d782116e2418ee440724b1a312).
    pub fn new_with_int_size<S: SizeMethods>(
        rows: c_int,
        cols: c_int,
        gap: &S,
    ) -> GridSizerFromCpp<FROM_CPP> {
        unsafe {
            let gap = gap.as_ptr();
            GridSizerFromCpp(ffi::wxGridSizer_new3(rows, cols, gap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for GridSizerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridSizerFromCpp<FROM_CPP>> for SizerFromCpp<FROM_CPP> {
    fn from(o: GridSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<GridSizerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GridSizerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridSizerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridSizer_CLASSINFO()) }
    }
}

// wxGridTableBase
wxwidgets! {
    /// The almost abstract base class for grid tables.
    /// - [`GridTableBase`] represents a C++ `wxGridTableBase` class instance which your code has ownership, [`GridTableBaseFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridTableBase`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridTableBase` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_table_base.html) for more details.
    #[doc(alias = "wxGridTableBase")]
    #[doc(alias = "GridTableBase")]
    class GridTableBase
        = GridTableBaseFromCpp<false>(wxGridTableBase) impl
        GridTableBaseMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> GridTableBaseFromCpp<FROM_CPP> {
    // BLOCKED: fn wxGridTableBase()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridTableBaseFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<GridTableBaseFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: GridTableBaseFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for GridTableBaseFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxGridTableBase_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for GridTableBaseFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxGridUpdateLocker
wxwidgets! {
    /// This small class can be used to prevent wxGrid from redrawing during its lifetime by calling wxGrid::BeginBatch() in its constructor and wxGrid::EndBatch() in its destructor.
    /// - [`GridUpdateLocker`] represents a C++ `wxGridUpdateLocker` class instance which your code has ownership, [`GridUpdateLockerFromCpp`]`<true>` represents one which don't own.
    /// - Use [`GridUpdateLocker`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxGridUpdateLocker` class's documentation](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html) for more details.
    #[doc(alias = "wxGridUpdateLocker")]
    #[doc(alias = "GridUpdateLocker")]
    class GridUpdateLocker
        = GridUpdateLockerFromCpp<false>(wxGridUpdateLocker) impl
        GridUpdateLockerMethods
}
impl<const FROM_CPP: bool> GridUpdateLockerFromCpp<FROM_CPP> {
    /// Creates an object preventing the updates of the specified grid.
    ///
    /// See [C++ `wxGridUpdateLocker::wxGridUpdateLocker()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_grid_update_locker.html#a13605827243de9ed1c0864fbd055cb8f).
    pub fn new(grid: *mut c_void) -> GridUpdateLockerFromCpp<FROM_CPP> {
        unsafe { GridUpdateLockerFromCpp(ffi::wxGridUpdateLocker_new(grid)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for GridUpdateLockerFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for GridUpdateLockerFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxGridUpdateLocker_delete(self.0) }
        }
    }
}
