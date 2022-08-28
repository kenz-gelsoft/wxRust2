use super::*;

// wxSVGFileDC
wxwidgets! {
    /// A wxSVGFileDC is a device context onto which graphics and text can be drawn, and the output produced as a vector file, in SVG format.
    ///
    /// [See `wxSVGFileDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_s_v_g_file_d_c.html)
    #[doc(alias = "wxSVGFileDC")]
    #[doc(alias = "SVGFileDC")]
    class SVGFileDC
        = SVGFileDCIsOwned<true>(wxSVGFileDC) impl
        SVGFileDCMethods,
        // DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> SVGFileDCIsOwned<OWNED> {
    /// Initializes a wxSVGFileDC with the given filename, width and height at dpi resolution, and an optional title.
    pub fn new(
        filename: &str,
        width: c_int,
        height: c_int,
        dpi: c_double,
        title: &str,
    ) -> SVGFileDCIsOwned<OWNED> {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            SVGFileDCIsOwned(ffi::wxSVGFileDC_new(filename, width, height, dpi, title))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SVGFileDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SVGFileDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: SVGFileDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SVGFileDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SVGFileDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SVGFileDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSVGFileDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SVGFileDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> DCMethods for SVGFileDCIsOwned<OWNED> {
    /// Draws a rectangle the size of the SVG using the wxDC::SetBackground() brush.
    fn clear(&self) {
        unsafe { ffi::wxSVGFileDC_Clear(self.as_ptr()) }
    }
    /// Destroys the current clipping region so that none of the DC is clipped.
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxSVGFileDC_DestroyClippingRegion(self.as_ptr()) }
    }
    /// Function not implemented in this DC class.
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxSVGFileDC_CrossHair(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn FloodFill()
    fn get_pixel<C: ColourMethods>(&self, x: c_int, y: c_int, colour: Option<&C>) -> bool {
        unsafe {
            let colour = match colour {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxSVGFileDC_GetPixel(self.as_ptr(), x, y, colour)
        }
    }
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxSVGFileDC_SetPalette(self.as_ptr(), palette)
        }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxSVGFileDC_GetDepth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn GetLogicalFunction()
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxSVGFileDC_StartDoc(self.as_ptr(), message)
        }
    }
    fn end_doc(&self) {
        unsafe { ffi::wxSVGFileDC_EndDoc(self.as_ptr()) }
    }
    fn start_page(&self) {
        unsafe { ffi::wxSVGFileDC_StartPage(self.as_ptr()) }
    }
    fn end_page(&self) {
        unsafe { ffi::wxSVGFileDC_EndPage(self.as_ptr()) }
    }
}

// wxSashEvent
wxwidgets! {
    /// A sash event is sent when the sash of a wxSashWindow has been dragged by the user.
    ///
    /// [See `wxSashEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sash_event.html)
    #[doc(alias = "wxSashEvent")]
    #[doc(alias = "SashEvent")]
    class SashEvent
        = SashEventIsOwned<true>(wxSashEvent) impl
        SashEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSashEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SashEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SashEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SashEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SashEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SashEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSashEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SashEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSashLayoutWindow
wxwidgets! {
    /// wxSashLayoutWindow responds to OnCalculateLayout events generated by wxLayoutAlgorithm.
    ///
    /// [See `wxSashLayoutWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sash_layout_window.html)
    #[doc(alias = "wxSashLayoutWindow")]
    #[doc(alias = "SashLayoutWindow")]
    class SashLayoutWindow
        = SashLayoutWindowIsOwned<true>(wxSashLayoutWindow) impl
        SashLayoutWindowMethods,
        SashWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashLayoutWindowIsOwned<OWNED> {
    /// Default ctor.
    pub fn new_2step() -> SashLayoutWindowIsOwned<OWNED> {
        unsafe { SashLayoutWindowIsOwned(ffi::wxSashLayoutWindow_new()) }
    }
    /// Constructs a sash layout window, which can be a child of a frame, dialog or any other non-control window.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SashLayoutWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SashLayoutWindowIsOwned(ffi::wxSashLayoutWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SashLayoutWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for SashWindowIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashLayoutWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SashLayoutWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashLayoutWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSashLayoutWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SashLayoutWindowIsOwned<OWNED> {
    /// Initializes a sash layout window, which can be a child of a frame, dialog or any other non-control window.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSashLayoutWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSashWindow
wxwidgets! {
    /// wxSashWindow allows any of its edges to have a sash which can be dragged to resize the window.
    ///
    /// [See `wxSashWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sash_window.html)
    #[doc(alias = "wxSashWindow")]
    #[doc(alias = "SashWindow")]
    class SashWindow
        = SashWindowIsOwned<true>(wxSashWindow) impl
        SashWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SashWindowIsOwned<OWNED> {
    /// Default ctor.
    pub fn new_2step() -> SashWindowIsOwned<OWNED> {
        unsafe { SashWindowIsOwned(ffi::wxSashWindow_new()) }
    }
    /// Constructs a sash window, which can be a child of a frame, dialog or any other non-control window.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SashWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SashWindowIsOwned(ffi::wxSashWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SashWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SashWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SashWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SashWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SashWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SashWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SashWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSashWindow_CLASSINFO()) }
    }
}

// wxScreenDC
wxwidgets! {
    /// A wxScreenDC can be used to paint on the screen.
    ///
    /// [See `wxScreenDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_screen_d_c.html)
    #[doc(alias = "wxScreenDC")]
    #[doc(alias = "ScreenDC")]
    class ScreenDC
        = ScreenDCIsOwned<true>(wxScreenDC) impl
        ScreenDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScreenDCIsOwned<OWNED> {
    /// Constructor.
    pub fn new() -> ScreenDCIsOwned<OWNED> {
        unsafe { ScreenDCIsOwned(ffi::wxScreenDC_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ScreenDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScreenDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: ScreenDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScreenDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScreenDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScreenDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScreenDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScreenDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxScrollBar
wxwidgets! {
    /// A wxScrollBar is a control that represents a horizontal or vertical scrollbar.
    ///
    /// [See `wxScrollBar`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_scroll_bar.html)
    #[doc(alias = "wxScrollBar")]
    #[doc(alias = "ScrollBar")]
    class ScrollBar
        = ScrollBarIsOwned<true>(wxScrollBar) impl
        ScrollBarMethods,
        // ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollBarIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> ScrollBarIsOwned<OWNED> {
        unsafe { ScrollBarIsOwned(ffi::wxScrollBar_new()) }
    }
    /// Constructor, creating and showing a scrollbar.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> ScrollBarIsOwned<OWNED> {
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
            ScrollBarIsOwned(ffi::wxScrollBar_new1(
                parent, id, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for ScrollBarIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScrollBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScrollBar_CLASSINFO()) }
    }
}
impl<const OWNED: bool> ControlMethods for ScrollBarIsOwned<OWNED> {
    /// Scrollbar creation function called by the scrollbar constructor.
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
            ffi::wxScrollBar_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
}

// wxScrollEvent
wxwidgets! {
    /// A scroll event holds information about events sent from stand-alone scrollbars (see wxScrollBar) and sliders (see wxSlider).
    ///
    /// [See `wxScrollEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_scroll_event.html)
    #[doc(alias = "wxScrollEvent")]
    #[doc(alias = "ScrollEvent")]
    class ScrollEvent
        = ScrollEventIsOwned<true>(wxScrollEvent) impl
        ScrollEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxScrollEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ScrollEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScrollEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: ScrollEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ScrollEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScrollEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScrollEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScrollEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxScrollWinEvent
wxwidgets! {
    /// A scroll event holds information about events sent from scrolling windows.
    ///
    /// [See `wxScrollWinEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_scroll_win_event.html)
    #[doc(alias = "wxScrollWinEvent")]
    #[doc(alias = "ScrollWinEvent")]
    class ScrollWinEvent
        = ScrollWinEventIsOwned<true>(wxScrollWinEvent) impl
        ScrollWinEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ScrollWinEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxScrollWinEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ScrollWinEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ScrollWinEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ScrollWinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ScrollWinEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ScrollWinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ScrollWinEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxScrollWinEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ScrollWinEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSearchCtrl
wxwidgets! {
    /// A search control is a composite control with a search button, a text control, and a cancel button.
    ///
    /// [See `wxSearchCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_search_ctrl.html)
    #[doc(alias = "wxSearchCtrl")]
    #[doc(alias = "SearchCtrl")]
    class SearchCtrl
        = SearchCtrlIsOwned<true>(wxSearchCtrl) impl
        SearchCtrlMethods,
        // TextCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SearchCtrlIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SearchCtrlIsOwned<OWNED> {
        unsafe { SearchCtrlIsOwned(ffi::wxSearchCtrl_new()) }
    }
    /// Constructor, creating and showing a text control.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> SearchCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SearchCtrlIsOwned(ffi::wxSearchCtrl_new1(
                parent, id, value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SearchCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for TextCtrlIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SearchCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SearchCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SearchCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSearchCtrl_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSearchCtrl
impl<const OWNED: bool> TextEntryMethods for SearchCtrlIsOwned<OWNED> {
    fn as_text_entry(&self) -> *mut c_void {
        unsafe { ffi::wxSearchCtrl_AsTextEntry(self.as_ptr()) }
    }
}
impl<const OWNED: bool> TextCtrlMethods for SearchCtrlIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
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
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSearchCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
}

// wxSetCursorEvent
wxwidgets! {
    /// A wxSetCursorEvent is generated from wxWindow when the mouse cursor is about to be set as a result of mouse motion.
    ///
    /// [See `wxSetCursorEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_set_cursor_event.html)
    #[doc(alias = "wxSetCursorEvent")]
    #[doc(alias = "SetCursorEvent")]
    class SetCursorEvent
        = SetCursorEventIsOwned<true>(wxSetCursorEvent) impl
        SetCursorEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SetCursorEventIsOwned<OWNED> {
    /// Constructor, used by the library itself internally to initialize the event object.
    pub fn new(x: c_int, y: c_int) -> SetCursorEventIsOwned<OWNED> {
        unsafe { SetCursorEventIsOwned(ffi::wxSetCursorEvent_new(x, y)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SetCursorEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SetCursorEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SetCursorEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SetCursorEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SetCursorEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SetCursorEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSetCursorEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SetCursorEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSettableHeaderColumn
wxwidgets! {
    /// Adds methods to set the column attributes to wxHeaderColumn.
    ///
    /// [See `wxSettableHeaderColumn`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_settable_header_column.html)
    #[doc(alias = "wxSettableHeaderColumn")]
    #[doc(alias = "SettableHeaderColumn")]
    class SettableHeaderColumn
        = SettableHeaderColumnIsOwned<true>(wxSettableHeaderColumn) impl
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> SettableHeaderColumnIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SettableHeaderColumnIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SettableHeaderColumnIsOwned<OWNED>> for HeaderColumnIsOwned<OWNED> {
    fn from(o: SettableHeaderColumnIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for SettableHeaderColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSettableHeaderColumn_delete(self.0) }
        }
    }
}

// wxShowEvent
wxwidgets! {
    /// An event being sent when the window is shown or hidden.
    ///
    /// [See `wxShowEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_show_event.html)
    #[doc(alias = "wxShowEvent")]
    #[doc(alias = "ShowEvent")]
    class ShowEvent
        = ShowEventIsOwned<true>(wxShowEvent) impl
        ShowEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ShowEventIsOwned<OWNED> {
    /// Constructor.
    pub fn new(winid: c_int, show: bool) -> ShowEventIsOwned<OWNED> {
        unsafe { ShowEventIsOwned(ffi::wxShowEvent_new(winid, show)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ShowEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<ShowEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ShowEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ShowEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ShowEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ShowEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxShowEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ShowEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSimplebook
wxwidgets! {
    /// wxSimplebook is a control showing exactly one of its several pages.
    ///
    /// [See `wxSimplebook`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_simplebook.html)
    #[doc(alias = "wxSimplebook")]
    #[doc(alias = "Simplebook")]
    class Simplebook
        = SimplebookIsOwned<true>(wxSimplebook) impl
        SimplebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SimplebookIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SimplebookIsOwned<OWNED> {
        unsafe { SimplebookIsOwned(ffi::wxSimplebook_new()) }
    }
    /// Constructs a simple book control.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SimplebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SimplebookIsOwned(ffi::wxSimplebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SimplebookIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SimplebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SimplebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SimplebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSimplebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SimplebookIsOwned<OWNED> {
    /// Really create the window of an object created using default constructor.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSimplebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSize
wxwidgets! {
    /// A wxSize is a useful data structure for graphics operations.
    ///
    /// [See `wxSize`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_size.html)
    #[doc(alias = "wxSize")]
    #[doc(alias = "Size")]
    class Size
        = SizeIsOwned<true>(wxSize) impl
        SizeMethods
}
impl<const OWNED: bool> SizeIsOwned<OWNED> {
    /// Initializes this size object with zero width and height.
    pub fn new() -> SizeIsOwned<OWNED> {
        unsafe { SizeIsOwned(ffi::wxSize_new()) }
    }
    /// Initializes this size object with the given width and height.
    pub fn new_with_int(width: c_int, height: c_int) -> SizeIsOwned<OWNED> {
        unsafe { SizeIsOwned(ffi::wxSize_new1(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizeIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for SizeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSize_delete(self.0) }
        }
    }
}

// wxSizeEvent
wxwidgets! {
    /// A size event holds information about size change events of wxWindow.
    ///
    /// [See `wxSizeEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_size_event.html)
    #[doc(alias = "wxSizeEvent")]
    #[doc(alias = "SizeEvent")]
    class SizeEvent
        = SizeEventIsOwned<true>(wxSizeEvent) impl
        SizeEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizeEventIsOwned<OWNED> {
    /// Constructor.
    pub fn new<S: SizeMethods>(sz: &S, id: c_int) -> SizeEventIsOwned<OWNED> {
        unsafe {
            let sz = sz.as_ptr();
            SizeEventIsOwned(ffi::wxSizeEvent_new(sz, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizeEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SizeEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SizeEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SizeEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizeEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSizeEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SizeEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSizer
wxwidgets! {
    /// wxSizer is the abstract base class used for laying out subwindows in a window.
    ///
    /// [See `wxSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sizer.html)
    #[doc(alias = "wxSizer")]
    #[doc(alias = "Sizer")]
    class Sizer
        = SizerIsOwned<true>(wxSizer) impl
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerIsOwned<OWNED> {
    // BLOCKED: fn wxSizer()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSizer_CLASSINFO()) }
    }
}

// wxSizerFlags
wxwidgets! {
    /// Container for sizer items flags providing readable names for them.
    ///
    /// [See `wxSizerFlags`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sizer_flags.html)
    #[doc(alias = "wxSizerFlags")]
    #[doc(alias = "SizerFlags")]
    class SizerFlags
        = SizerFlagsIsOwned<true>(wxSizerFlags) impl
        SizerFlagsMethods
}
impl<const OWNED: bool> SizerFlagsIsOwned<OWNED> {
    /// Creates the wxSizer with the proportion specified by proportion.
    pub fn new(proportion: c_int) -> SizerFlagsIsOwned<OWNED> {
        unsafe { SizerFlagsIsOwned(ffi::wxSizerFlags_new(proportion)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizerFlagsIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for SizerFlagsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSizerFlags_delete(self.0) }
        }
    }
}

// wxSizerItem
wxwidgets! {
    /// The wxSizerItem class is used to track the position, size and other attributes of each item managed by a wxSizer.
    ///
    /// [See `wxSizerItem`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sizer_item.html)
    #[doc(alias = "wxSizerItem")]
    #[doc(alias = "SizerItem")]
    class SizerItem
        = SizerItemIsOwned<true>(wxSizerItem) impl
        SizerItemMethods,
        ObjectMethods
}
impl<const OWNED: bool> SizerItemIsOwned<OWNED> {
    /// Construct a sizer item for tracking a spacer.
    pub fn new_with_int<O: ObjectMethods>(
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemIsOwned(ffi::wxSizerItem_new(
                width, height, proportion, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a window.
    pub fn new_with_window_sizerflags<W: WindowMethods, S: SizerFlagsMethods>(
        window: Option<&W>,
        flags: &S,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItemIsOwned(ffi::wxSizerItem_new1(window, flags))
        }
    }
    pub fn new_with_window_int<W: WindowMethods, O: ObjectMethods>(
        window: Option<&W>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemIsOwned(ffi::wxSizerItem_new2(
                window, proportion, flag, border, user_data,
            ))
        }
    }
    /// Construct a sizer item for tracking a subsizer.
    pub fn new_with_sizer_sizerflags<S: SizerMethods, S2: SizerFlagsMethods>(
        sizer: Option<&S>,
        flags: &S2,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let flags = flags.as_ptr();
            SizerItemIsOwned(ffi::wxSizerItem_new3(sizer, flags))
        }
    }
    pub fn new_with_sizer_int<S: SizerMethods, O: ObjectMethods>(
        sizer: Option<&S>,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> SizerItemIsOwned<OWNED> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItemIsOwned(ffi::wxSizerItem_new4(
                sizer, proportion, flag, border, user_data,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SizerItemIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SizerItemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SizerItemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SizerItemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSizerItem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SizerItemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSlider
wxwidgets! {
    /// A slider is a control with a handle which can be pulled back and forth to change the value.
    ///
    /// [See `wxSlider`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_slider.html)
    #[doc(alias = "wxSlider")]
    #[doc(alias = "Slider")]
    class Slider
        = SliderIsOwned<true>(wxSlider) impl
        SliderMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SliderIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SliderIsOwned<OWNED> {
        unsafe { SliderIsOwned(ffi::wxSlider_new()) }
    }
    /// Constructor, creating and showing a slider.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> SliderIsOwned<OWNED> {
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
            SliderIsOwned(ffi::wxSlider_new1(
                parent, id, value, min_value, max_value, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SliderIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SliderIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SliderIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SliderIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSlider_CLASSINFO()) }
    }
}

// wxSpinButton
wxwidgets! {
    /// A wxSpinButton has two small up and down (or left and right) arrow buttons.
    ///
    /// [See `wxSpinButton`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_spin_button.html)
    #[doc(alias = "wxSpinButton")]
    #[doc(alias = "SpinButton")]
    class SpinButton
        = SpinButtonIsOwned<true>(wxSpinButton) impl
        SpinButtonMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinButtonIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SpinButtonIsOwned<OWNED> {
        unsafe { SpinButtonIsOwned(ffi::wxSpinButton_new()) }
    }
    /// Constructor, creating and showing a spin button.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SpinButtonIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinButtonIsOwned(ffi::wxSpinButton_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SpinButtonIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinButtonIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinButtonIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinButtonIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinButton_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SpinButtonIsOwned<OWNED> {
    /// Scrollbar creation function called by the spin button constructor.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSpinButton_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxSpinCtrl
wxwidgets! {
    /// wxSpinCtrl combines wxTextCtrl and wxSpinButton in one control.
    ///
    /// [See `wxSpinCtrl`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl.html)
    #[doc(alias = "wxSpinCtrl")]
    #[doc(alias = "SpinCtrl")]
    class SpinCtrl
        = SpinCtrlIsOwned<true>(wxSpinCtrl) impl
        SpinCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinCtrlIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SpinCtrlIsOwned<OWNED> {
        unsafe { SpinCtrlIsOwned(ffi::wxSpinCtrl_new()) }
    }
    /// Constructor, creating and showing a spin control.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: &str,
    ) -> SpinCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinCtrlIsOwned(ffi::wxSpinCtrl_new1(
                parent, id, value, pos, size, style, min, max, initial, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SpinCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinCtrl_CLASSINFO()) }
    }
}

// wxSpinCtrlDouble
wxwidgets! {
    /// wxSpinCtrlDouble combines wxTextCtrl and wxSpinButton in one control and displays a real number.
    ///
    /// [See `wxSpinCtrlDouble`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_spin_ctrl_double.html)
    #[doc(alias = "wxSpinCtrlDouble")]
    #[doc(alias = "SpinCtrlDouble")]
    class SpinCtrlDouble
        = SpinCtrlDoubleIsOwned<true>(wxSpinCtrlDouble) impl
        SpinCtrlDoubleMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinCtrlDoubleIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SpinCtrlDoubleIsOwned<OWNED> {
        unsafe { SpinCtrlDoubleIsOwned(ffi::wxSpinCtrlDouble_new()) }
    }
    /// Constructor, creating and showing a spin control.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: &str,
    ) -> SpinCtrlDoubleIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SpinCtrlDoubleIsOwned(ffi::wxSpinCtrlDouble_new1(
                parent, id, value, pos, size, style, min, max, initial, inc, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SpinCtrlDoubleIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinCtrlDoubleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinCtrlDoubleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinCtrlDoubleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinCtrlDouble_CLASSINFO()) }
    }
}

// wxSpinDoubleEvent
wxwidgets! {
    /// This event class is used for the events generated by wxSpinCtrlDouble.
    ///
    /// [See `wxSpinDoubleEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_spin_double_event.html)
    #[doc(alias = "wxSpinDoubleEvent")]
    #[doc(alias = "SpinDoubleEvent")]
    class SpinDoubleEvent
        = SpinDoubleEventIsOwned<true>(wxSpinDoubleEvent) impl
        SpinDoubleEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinDoubleEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSpinDoubleEvent()
    /// The copy constructor.
    pub fn new<S: SpinDoubleEventMethods>(event: &S) -> SpinDoubleEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            SpinDoubleEventIsOwned(ffi::wxSpinDoubleEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SpinDoubleEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinDoubleEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinDoubleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinDoubleEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinDoubleEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SpinDoubleEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSpinEvent
wxwidgets! {
    /// This event class is used for the events generated by wxSpinButton and wxSpinCtrl.
    ///
    /// [See `wxSpinEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_spin_event.html)
    #[doc(alias = "wxSpinEvent")]
    #[doc(alias = "SpinEvent")]
    class SpinEvent
        = SpinEventIsOwned<true>(wxSpinEvent) impl
        SpinEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SpinEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSpinEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SpinEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SpinEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SpinEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SpinEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSpinEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SpinEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSplashScreen
wxwidgets! {
    /// wxSplashScreen shows a window with a thin border, displaying a bitmap describing your application.
    ///
    /// [See `wxSplashScreen`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_splash_screen.html)
    #[doc(alias = "wxSplashScreen")]
    #[doc(alias = "SplashScreen")]
    class SplashScreen
        = SplashScreenIsOwned<true>(wxSplashScreen) impl
        SplashScreenMethods,
        FrameMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplashScreenIsOwned<OWNED> {
    /// Construct the splash screen passing a bitmap, a style, a timeout, a window id, optional position and size, and a window style.
    pub fn new<B: BitmapMethods, W: WindowMethods, P: PointMethods, S: SizeMethods>(
        bitmap: &B,
        splash_style: c_long,
        milliseconds: c_int,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
    ) -> SplashScreenIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            SplashScreenIsOwned(ffi::wxSplashScreen_new(
                bitmap,
                splash_style,
                milliseconds,
                parent,
                id,
                pos,
                size,
                style,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SplashScreenIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for FrameIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplashScreenIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SplashScreenIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplashScreenIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSplashScreen_CLASSINFO()) }
    }
}

// wxSplitterEvent
wxwidgets! {
    /// This class represents the events generated by a splitter control.
    ///
    /// [See `wxSplitterEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_splitter_event.html)
    #[doc(alias = "wxSplitterEvent")]
    #[doc(alias = "SplitterEvent")]
    class SplitterEvent
        = SplitterEventIsOwned<true>(wxSplitterEvent) impl
        SplitterEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplitterEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxSplitterEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SplitterEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SplitterEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplitterEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSplitterEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SplitterEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSplitterWindow
wxwidgets! {
    /// This class manages up to two subwindows.
    ///
    /// [See `wxSplitterWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_splitter_window.html)
    #[doc(alias = "wxSplitterWindow")]
    #[doc(alias = "SplitterWindow")]
    class SplitterWindow
        = SplitterWindowIsOwned<true>(wxSplitterWindow) impl
        SplitterWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SplitterWindowIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> SplitterWindowIsOwned<OWNED> {
        unsafe { SplitterWindowIsOwned(ffi::wxSplitterWindow_new()) }
    }
    /// Constructor for creating the window.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> SplitterWindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            SplitterWindowIsOwned(ffi::wxSplitterWindow_new1(
                parent, id, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for SplitterWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SplitterWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SplitterWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SplitterWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SplitterWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SplitterWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SplitterWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSplitterWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for SplitterWindowIsOwned<OWNED> {
    /// Creation function, for two-step construction.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        point: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let point = point.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxSplitterWindow_Create(self.as_ptr(), parent, id, point, size, style, name)
        }
    }
}

// wxStaticBitmap
wxwidgets! {
    /// A static bitmap control displays a bitmap.
    ///
    /// [See `wxStaticBitmap`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_static_bitmap.html)
    #[doc(alias = "wxStaticBitmap")]
    #[doc(alias = "StaticBitmap")]
    class StaticBitmap
        = StaticBitmapIsOwned<true>(wxStaticBitmap) impl
        StaticBitmapMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBitmapIsOwned<OWNED> {
    //  ENUM: ScaleMode
    pub const Scale_None: c_int = 0;
    pub const Scale_Fill: c_int = 0 + 1;
    pub const Scale_AspectFit: c_int = 0 + 2;
    pub const Scale_AspectFill: c_int = 0 + 3;

    /// Default constructor.
    pub fn new_2step() -> StaticBitmapIsOwned<OWNED> {
        unsafe { StaticBitmapIsOwned(ffi::wxStaticBitmap_new()) }
    }
    /// Constructor, creating and showing a static bitmap control.
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &B,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBitmapIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBitmapIsOwned(ffi::wxStaticBitmap_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticBitmapIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBitmapIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticBitmapIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBitmapIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticBitmap_CLASSINFO()) }
    }
}

// wxStaticBox
wxwidgets! {
    /// A static box is a rectangle drawn around other windows to denote a logical grouping of items.
    ///
    /// [See `wxStaticBox`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_static_box.html)
    #[doc(alias = "wxStaticBox")]
    #[doc(alias = "StaticBox")]
    class StaticBox
        = StaticBoxIsOwned<true>(wxStaticBox) impl
        StaticBoxMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> StaticBoxIsOwned<OWNED> {
        unsafe { StaticBoxIsOwned(ffi::wxStaticBox_new()) }
    }
    /// Constructor, creating and showing a static box.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticBoxIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticBoxIsOwned(ffi::wxStaticBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    // BLOCKED: fn wxStaticBox2()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticBoxIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticBox_CLASSINFO()) }
    }
}

// wxStaticBoxSizer
wxwidgets! {
    /// wxStaticBoxSizer is a sizer derived from wxBoxSizer but adds a static box around the sizer.
    ///
    /// [See `wxStaticBoxSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_static_box_sizer.html)
    #[doc(alias = "wxStaticBoxSizer")]
    #[doc(alias = "StaticBoxSizer")]
    class StaticBoxSizer
        = StaticBoxSizerIsOwned<true>(wxStaticBoxSizer) impl
        StaticBoxSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticBoxSizerIsOwned<OWNED> {
    /// This constructor uses an already existing static box.
    pub fn new_with_staticbox<S: StaticBoxMethods>(
        box_: Option<&S>,
        orient: c_int,
    ) -> StaticBoxSizerIsOwned<OWNED> {
        unsafe {
            let box_ = match box_ {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            StaticBoxSizerIsOwned(ffi::wxStaticBoxSizer_new(box_, orient))
        }
    }
    /// This constructor creates a new static box with the given label and parent window.
    pub fn new_with_int<W: WindowMethods>(
        orient: c_int,
        parent: Option<&W>,
        label: &str,
    ) -> StaticBoxSizerIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            StaticBoxSizerIsOwned(ffi::wxStaticBoxSizer_new1(orient, parent, label))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticBoxSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticBoxSizerIsOwned<OWNED>> for BoxSizerIsOwned<OWNED> {
    fn from(o: StaticBoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: StaticBoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticBoxSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticBoxSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticBoxSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticBoxSizer_CLASSINFO()) }
    }
}

// wxStaticLine
wxwidgets! {
    /// A static line is just a line which may be used in a dialog to separate the groups of controls.
    ///
    /// [See `wxStaticLine`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_static_line.html)
    #[doc(alias = "wxStaticLine")]
    #[doc(alias = "StaticLine")]
    class StaticLine
        = StaticLineIsOwned<true>(wxStaticLine) impl
        StaticLineMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticLineIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> StaticLineIsOwned<OWNED> {
        unsafe { StaticLineIsOwned(ffi::wxStaticLine_new()) }
    }
    /// Constructor, creating and showing a static line.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticLineIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticLineIsOwned(ffi::wxStaticLine_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticLineIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticLineIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticLineIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticLineIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticLine_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for StaticLineIsOwned<OWNED> {
    /// Creates the static line for two-step construction.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
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
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxStaticLine_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxStaticText
wxwidgets! {
    /// A static text control displays one or more lines of read-only text.
    ///
    /// [See `wxStaticText`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_static_text.html)
    #[doc(alias = "wxStaticText")]
    #[doc(alias = "StaticText")]
    class StaticText
        = StaticTextIsOwned<true>(wxStaticText) impl
        StaticTextMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StaticTextIsOwned<OWNED> {
    /// Default constructor.
    pub fn new_2step() -> StaticTextIsOwned<OWNED> {
        unsafe { StaticTextIsOwned(ffi::wxStaticText_new()) }
    }
    /// Constructor, creating and showing a text control.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> StaticTextIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            StaticTextIsOwned(ffi::wxStaticText_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StaticTextIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StaticTextIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StaticTextIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StaticTextIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStaticText_CLASSINFO()) }
    }
}

// wxStatusBar
wxwidgets! {
    /// A status bar is a narrow window that can be placed along the bottom of a frame to give small amounts of status information.
    ///
    /// [See `wxStatusBar`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_status_bar.html)
    #[doc(alias = "wxStatusBar")]
    #[doc(alias = "StatusBar")]
    class StatusBar
        = StatusBarIsOwned<true>(wxStatusBar) impl
        StatusBarMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StatusBarIsOwned<OWNED> {
    /// Default ctor.
    pub fn new_2step() -> StatusBarIsOwned<OWNED> {
        unsafe { StatusBarIsOwned(ffi::wxStatusBar_new()) }
    }
    /// Constructor, creating the window.
    pub fn new<W: WindowMethods>(
        parent: Option<&W>,
        id: c_int,
        style: c_long,
        name: &str,
    ) -> StatusBarIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let name = WxString::from(name);
            let name = name.as_ptr();
            StatusBarIsOwned(ffi::wxStatusBar_new1(parent, id, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StatusBarIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StatusBarIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StatusBarIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StatusBarIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStatusBar_CLASSINFO()) }
    }
}

// wxStatusBarPane
wxwidgets! {
    /// A status bar pane data container used by wxStatusBar.
    ///
    /// [See `wxStatusBarPane`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_status_bar_pane.html)
    #[doc(alias = "wxStatusBarPane")]
    #[doc(alias = "StatusBarPane")]
    class StatusBarPane
        = StatusBarPaneIsOwned<true>(wxStatusBarPane) impl
        StatusBarPaneMethods
}
impl<const OWNED: bool> StatusBarPaneIsOwned<OWNED> {
    /// Constructs the pane with the given style and width.
    pub fn new(style: c_int, width: c_int) -> StatusBarPaneIsOwned<OWNED> {
        unsafe { StatusBarPaneIsOwned(ffi::wxStatusBarPane_new(style, width)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for StatusBarPaneIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for StatusBarPaneIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStatusBarPane_delete(self.0) }
        }
    }
}

// wxStdDialogButtonSizer
wxwidgets! {
    /// This class creates button layouts which conform to the standard button spacing and ordering defined by the platform or toolkit's user interface guidelines (if such things exist).
    ///
    /// [See `wxStdDialogButtonSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_std_dialog_button_sizer.html)
    #[doc(alias = "wxStdDialogButtonSizer")]
    #[doc(alias = "StdDialogButtonSizer")]
    class StdDialogButtonSizer
        = StdDialogButtonSizerIsOwned<true>(wxStdDialogButtonSizer) impl
        StdDialogButtonSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StdDialogButtonSizerIsOwned<OWNED> {
    /// Constructor for a wxStdDialogButtonSizer.
    pub fn new() -> StdDialogButtonSizerIsOwned<OWNED> {
        unsafe { StdDialogButtonSizerIsOwned(ffi::wxStdDialogButtonSizer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for StdDialogButtonSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerIsOwned<OWNED>> for BoxSizerIsOwned<OWNED> {
    fn from(o: StdDialogButtonSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: StdDialogButtonSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<StdDialogButtonSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StdDialogButtonSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StdDialogButtonSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStdDialogButtonSizer_CLASSINFO()) }
    }
}

// wxStockPreferencesPage
wxwidgets! {
    /// Specialization of wxPreferencesPage useful for certain commonly used preferences page.
    ///
    /// [See `wxStockPreferencesPage`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_stock_preferences_page.html)
    #[doc(alias = "wxStockPreferencesPage")]
    #[doc(alias = "StockPreferencesPage")]
    class StockPreferencesPage
        = StockPreferencesPageIsOwned<true>(wxStockPreferencesPage) impl
        StockPreferencesPageMethods,
        PreferencesPageMethods
}
impl<const OWNED: bool> StockPreferencesPageIsOwned<OWNED> {
    //  ENUM: Kind
    pub const Kind_General: c_int = 0;
    pub const Kind_Advanced: c_int = 0 + 1;

    // NOT_SUPPORTED: fn wxStockPreferencesPage()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for StockPreferencesPageIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<StockPreferencesPageIsOwned<OWNED>> for PreferencesPageIsOwned<OWNED> {
    fn from(o: StockPreferencesPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for StockPreferencesPageIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStockPreferencesPage_delete(self.0) }
        }
    }
}

// wxSysColourChangedEvent
wxwidgets! {
    /// This class is used for system colour change events, which are generated when the user changes the colour settings or when the system theme changes (e.g.
    ///
    /// [See `wxSysColourChangedEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_sys_colour_changed_event.html)
    #[doc(alias = "wxSysColourChangedEvent")]
    #[doc(alias = "SysColourChangedEvent")]
    class SysColourChangedEvent
        = SysColourChangedEventIsOwned<true>(wxSysColourChangedEvent) impl
        SysColourChangedEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> SysColourChangedEventIsOwned<OWNED> {
    /// Constructor.
    pub fn new() -> SysColourChangedEventIsOwned<OWNED> {
        unsafe { SysColourChangedEventIsOwned(ffi::wxSysColourChangedEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SysColourChangedEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<SysColourChangedEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: SysColourChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SysColourChangedEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SysColourChangedEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SysColourChangedEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSysColourChangedEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SysColourChangedEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSystemSettings
wxwidgets! {
    /// wxSystemSettings allows the application to ask for details about the system.
    ///
    /// [See `wxSystemSettings`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_system_settings.html)
    #[doc(alias = "wxSystemSettings")]
    #[doc(alias = "SystemSettings")]
    class SystemSettings
        = SystemSettingsIsOwned<true>(wxSystemSettings) impl
        SystemSettingsMethods
}
impl<const OWNED: bool> SystemSettingsIsOwned<OWNED> {
    /// Default constructor.
    pub fn new() -> SystemSettingsIsOwned<OWNED> {
        unsafe { SystemSettingsIsOwned(ffi::wxSystemSettings_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for SystemSettingsIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for SystemSettingsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSystemSettings_delete(self.0) }
        }
    }
}
