use super::*;

// wxHScrolledWindow
wxwidgets! {
    /// In the name of this class, "H" stands for "horizontal" because it can be used for scrolling columns of variable widths.
    /// - [`HScrolledWindow`] represents a C++ `wxHScrolledWindow` class instance which your code has ownership, [`HScrolledWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`HScrolledWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHScrolledWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_h_scrolled_window.html) for more details.
    #[doc(alias = "wxHScrolledWindow")]
    #[doc(alias = "HScrolledWindow")]
    class HScrolledWindow
        = HScrolledWindowInRust<true>(wxHScrolledWindow) impl
        HScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HScrolledWindowInRust<IN_RUST> {
    // BLOCKED: fn wxHScrolledWindow()
    // BLOCKED: fn wxHScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for HScrolledWindowInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HScrolledWindowInRust<IN_RUST>> for PanelInRust<IN_RUST> {
    fn from(o: HScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HScrolledWindowInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: HScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HScrolledWindowInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: HScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HScrolledWindowInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HScrolledWindowInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHScrolledWindow_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> WindowMethods for HScrolledWindowInRust<IN_RUST> {
    /// Same as the non-default constructor, but returns a status code: true if ok, false if the window couldn't be created.
    ///
    /// See [C++ `wxHScrolledWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_h_scrolled_window.html#a5982d08464fabd3b47543d298b710224).
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
            ffi::wxHScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxHTMLDataObject
wxwidgets! {
    /// wxHTMLDataObject is used for working with HTML-formatted text.
    /// - [`HTMLDataObject`] represents a C++ `wxHTMLDataObject` class instance which your code has ownership, [`HTMLDataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`HTMLDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHTMLDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html) for more details.
    #[doc(alias = "wxHTMLDataObject")]
    #[doc(alias = "HTMLDataObject")]
    class HTMLDataObject
        = HTMLDataObjectInRust<true>(wxHTMLDataObject) impl
        HTMLDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const IN_RUST: bool> HTMLDataObjectInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxHTMLDataObject::wxHTMLDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html#a3e09f114be84aef40c05809ddfe83333).
    pub fn new(html: &str) -> HTMLDataObjectInRust<IN_RUST> {
        unsafe {
            let html = WxString::from(html);
            let html = html.as_ptr();
            HTMLDataObjectInRust(ffi::wxHTMLDataObject_new(html))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HTMLDataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HTMLDataObjectInRust<IN_RUST>> for DataObjectSimpleInRust<IN_RUST> {
    fn from(o: HTMLDataObjectInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HTMLDataObjectInRust<IN_RUST>> for DataObjectInRust<IN_RUST> {
    fn from(o: HTMLDataObjectInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for HTMLDataObjectInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxHTMLDataObject_delete(self.0) }
        }
    }
}

// wxHVScrolledWindow
wxwidgets! {
    /// This window inherits all functionality of both vertical and horizontal, variable scrolled windows.
    /// - [`HVScrolledWindow`] represents a C++ `wxHVScrolledWindow` class instance which your code has ownership, [`HVScrolledWindowInRust`]`<false>` represents one which don't own.
    /// - Use [`HVScrolledWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHVScrolledWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_h_v_scrolled_window.html) for more details.
    #[doc(alias = "wxHVScrolledWindow")]
    #[doc(alias = "HVScrolledWindow")]
    class HVScrolledWindow
        = HVScrolledWindowInRust<true>(wxHVScrolledWindow) impl
        HVScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HVScrolledWindowInRust<IN_RUST> {
    // BLOCKED: fn wxHVScrolledWindow()
    // BLOCKED: fn wxHVScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for HVScrolledWindowInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HVScrolledWindowInRust<IN_RUST>> for PanelInRust<IN_RUST> {
    fn from(o: HVScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HVScrolledWindowInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: HVScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HVScrolledWindowInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: HVScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HVScrolledWindowInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HVScrolledWindowInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HVScrolledWindowInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHVScrolledWindow_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> WindowMethods for HVScrolledWindowInRust<IN_RUST> {
    /// Same as the non-default constructor, but returns a status code: true if ok, false if the window couldn't be created.
    ///
    /// See [C++ `wxHVScrolledWindow::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_h_v_scrolled_window.html#ae14453b61a87766376e8c64653c90ed6).
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
            ffi::wxHVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxHeaderColumn
wxwidgets! {
    /// Represents a column header in controls displaying tabular data such as wxDataViewCtrl or wxGrid.
    /// - [`HeaderColumn`] represents a C++ `wxHeaderColumn` class instance which your code has ownership, [`HeaderColumnInRust`]`<false>` represents one which don't own.
    /// - Use [`HeaderColumn`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderColumn` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html) for more details.
    #[doc(alias = "wxHeaderColumn")]
    #[doc(alias = "HeaderColumn")]
    class HeaderColumn
        = HeaderColumnInRust<true>(wxHeaderColumn) impl
        HeaderColumnMethods
}
impl<const IN_RUST: bool> HeaderColumnInRust<IN_RUST> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderColumnInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for HeaderColumnInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxHeaderColumn_delete(self.0) }
        }
    }
}

// wxHeaderColumnSimple
wxwidgets! {
    /// Simple container for the information about the column.
    /// - [`HeaderColumnSimple`] represents a C++ `wxHeaderColumnSimple` class instance which your code has ownership, [`HeaderColumnSimpleInRust`]`<false>` represents one which don't own.
    /// - Use [`HeaderColumnSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderColumnSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html) for more details.
    #[doc(alias = "wxHeaderColumnSimple")]
    #[doc(alias = "HeaderColumnSimple")]
    class HeaderColumnSimple
        = HeaderColumnSimpleInRust<true>(wxHeaderColumnSimple) impl
        HeaderColumnSimpleMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const IN_RUST: bool> HeaderColumnSimpleInRust<IN_RUST> {
    /// Constructor for a column header.
    ///
    /// See [C++ `wxHeaderColumnSimple::wxHeaderColumnSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html#a219431d463c61334f8de7ac45ebb16f0).
    pub fn new_with_str(
        title: &str,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleInRust<IN_RUST> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            HeaderColumnSimpleInRust(ffi::wxHeaderColumnSimple_new(title, width, align, flags))
        }
    }
    ///
    /// See [C++ `wxHeaderColumnSimple::wxHeaderColumnSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html#af5ed3aa43db1341f91b268a045e8f556).
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(
        bitmap: &B,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleInRust<IN_RUST> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            HeaderColumnSimpleInRust(ffi::wxHeaderColumnSimple_new1(bitmap, width, align, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderColumnSimpleInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HeaderColumnSimpleInRust<IN_RUST>>
    for SettableHeaderColumnInRust<IN_RUST>
{
    fn from(o: HeaderColumnSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderColumnSimpleInRust<IN_RUST>> for HeaderColumnInRust<IN_RUST> {
    fn from(o: HeaderColumnSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for HeaderColumnSimpleInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxHeaderColumnSimple_delete(self.0) }
        }
    }
}

// wxHeaderCtrl
wxwidgets! {
    /// wxHeaderCtrl is the control containing the column headings which is usually used for display of tabular data.
    /// - [`HeaderCtrl`] represents a C++ `wxHeaderCtrl` class instance which your code has ownership, [`HeaderCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`HeaderCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html) for more details.
    #[doc(alias = "wxHeaderCtrl")]
    #[doc(alias = "HeaderCtrl")]
    class HeaderCtrl
        = HeaderCtrlInRust<true>(wxHeaderCtrl) impl
        HeaderCtrlMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HeaderCtrlInRust<IN_RUST> {
    // BLOCKED: fn wxHeaderCtrl()
    // BLOCKED: fn wxHeaderCtrl1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for HeaderCtrlInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: HeaderCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: HeaderCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: HeaderCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HeaderCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HeaderCtrlInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHeaderCtrl_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> WindowMethods for HeaderCtrlInRust<IN_RUST> {
    /// Create the header control window.
    ///
    /// See [C++ `wxHeaderCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html#a1f8ff3be92e0fb2bd59e25642447c2d5).
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
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
            ffi::wxHeaderCtrl_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxHeaderCtrlEvent
wxwidgets! {
    /// Event class representing the events generated by wxHeaderCtrl.
    /// - [`HeaderCtrlEvent`] represents a C++ `wxHeaderCtrlEvent` class instance which your code has ownership, [`HeaderCtrlEventInRust`]`<false>` represents one which don't own.
    /// - Use [`HeaderCtrlEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderCtrlEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html) for more details.
    #[doc(alias = "wxHeaderCtrlEvent")]
    #[doc(alias = "HeaderCtrlEvent")]
    class HeaderCtrlEvent
        = HeaderCtrlEventInRust<true>(wxHeaderCtrlEvent) impl
        HeaderCtrlEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HeaderCtrlEventInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxHeaderCtrlEvent()
    ///
    /// See [C++ `wxHeaderCtrlEvent::wxHeaderCtrlEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#aa9c9fdd75b48ea8dfcb976ed16dbfcd1).
    pub fn new<H: HeaderCtrlEventMethods>(event: &H) -> HeaderCtrlEventInRust<IN_RUST> {
        unsafe {
            let event = event.as_ptr();
            HeaderCtrlEventInRust(ffi::wxHeaderCtrlEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderCtrlEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlEventInRust<IN_RUST>> for NotifyEventInRust<IN_RUST> {
    fn from(o: HeaderCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: HeaderCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: HeaderCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HeaderCtrlEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HeaderCtrlEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHeaderCtrlEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for HeaderCtrlEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHeaderCtrlSimple
wxwidgets! {
    /// wxHeaderCtrlSimple is a concrete header control which can be used directly, without inheriting from it as you need to do when using wxHeaderCtrl itself.
    /// - [`HeaderCtrlSimple`] represents a C++ `wxHeaderCtrlSimple` class instance which your code has ownership, [`HeaderCtrlSimpleInRust`]`<false>` represents one which don't own.
    /// - Use [`HeaderCtrlSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderCtrlSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html) for more details.
    #[doc(alias = "wxHeaderCtrlSimple")]
    #[doc(alias = "HeaderCtrlSimple")]
    class HeaderCtrlSimple
        = HeaderCtrlSimpleInRust<true>(wxHeaderCtrlSimple) impl
        HeaderCtrlSimpleMethods,
        HeaderCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HeaderCtrlSimpleInRust<IN_RUST> {
    /// Default constructor not creating the underlying window.
    ///
    /// See [C++ `wxHeaderCtrlSimple::wxHeaderCtrlSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a9a0288a46b35b0ebc7e8704a082dbd58).
    pub fn new_2step() -> HeaderCtrlSimpleInRust<IN_RUST> {
        unsafe { HeaderCtrlSimpleInRust(ffi::wxHeaderCtrlSimple_new()) }
    }
    /// Constructor creating the window.
    ///
    /// See [C++ `wxHeaderCtrlSimple::wxHeaderCtrlSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#aab2ae72cdb8374216da503d7747c65fa).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HeaderCtrlSimpleInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HeaderCtrlSimpleInRust(ffi::wxHeaderCtrlSimple_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for HeaderCtrlSimpleInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlSimpleInRust<IN_RUST>> for HeaderCtrlInRust<IN_RUST> {
    fn from(o: HeaderCtrlSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlSimpleInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: HeaderCtrlSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlSimpleInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: HeaderCtrlSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlSimpleInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: HeaderCtrlSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HeaderCtrlSimpleInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HeaderCtrlSimpleInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HeaderCtrlSimpleInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHeaderCtrlSimple_CLASSINFO()) }
    }
}

// wxHelpEvent
wxwidgets! {
    /// A help event is sent when the user has requested context-sensitive help.
    /// - [`HelpEvent`] represents a C++ `wxHelpEvent` class instance which your code has ownership, [`HelpEventInRust`]`<false>` represents one which don't own.
    /// - Use [`HelpEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHelpEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_help_event.html) for more details.
    #[doc(alias = "wxHelpEvent")]
    #[doc(alias = "HelpEvent")]
    class HelpEvent
        = HelpEventInRust<true>(wxHelpEvent) impl
        HelpEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HelpEventInRust<IN_RUST> {
    //  ENUM: Origin
    pub const Origin_Unknown: c_int = 0;
    pub const Origin_Keyboard: c_int = 0 + 1;
    pub const Origin_HelpButton: c_int = 0 + 2;

    // NOT_SUPPORTED: fn wxHelpEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HelpEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HelpEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: HelpEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HelpEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: HelpEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HelpEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HelpEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HelpEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHelpEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for HelpEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHyperlinkCtrl
wxwidgets! {
    /// This class shows a static text element which links to an URL.
    /// - [`HyperlinkCtrl`] represents a C++ `wxHyperlinkCtrl` class instance which your code has ownership, [`HyperlinkCtrlInRust`]`<false>` represents one which don't own.
    /// - Use [`HyperlinkCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHyperlinkCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html) for more details.
    #[doc(alias = "wxHyperlinkCtrl")]
    #[doc(alias = "HyperlinkCtrl")]
    class HyperlinkCtrl
        = HyperlinkCtrlInRust<true>(wxHyperlinkCtrl) impl
        HyperlinkCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HyperlinkCtrlInRust<IN_RUST> {
    ///
    /// See [C++ `wxHyperlinkCtrl::wxHyperlinkCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a9b0d333830376d2d437f35f31656405f).
    pub fn new_2step() -> HyperlinkCtrlInRust<IN_RUST> {
        unsafe { HyperlinkCtrlInRust(ffi::wxHyperlinkCtrl_new()) }
    }
    /// Constructor.
    ///
    /// See [C++ `wxHyperlinkCtrl::wxHyperlinkCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a0070265321d1df4d2a25cfd032c5df58).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HyperlinkCtrlInRust<IN_RUST> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HyperlinkCtrlInRust(ffi::wxHyperlinkCtrl_new1(
                parent, id, label, url, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for HyperlinkCtrlInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HyperlinkCtrlInRust<IN_RUST>> for ControlInRust<IN_RUST> {
    fn from(o: HyperlinkCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HyperlinkCtrlInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: HyperlinkCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HyperlinkCtrlInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: HyperlinkCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HyperlinkCtrlInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HyperlinkCtrlInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HyperlinkCtrlInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHyperlinkCtrl_CLASSINFO()) }
    }
}

// wxHyperlinkEvent
wxwidgets! {
    /// This event class is used for the events generated by wxHyperlinkCtrl.
    /// - [`HyperlinkEvent`] represents a C++ `wxHyperlinkEvent` class instance which your code has ownership, [`HyperlinkEventInRust`]`<false>` represents one which don't own.
    /// - Use [`HyperlinkEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHyperlinkEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html) for more details.
    #[doc(alias = "wxHyperlinkEvent")]
    #[doc(alias = "HyperlinkEvent")]
    class HyperlinkEvent
        = HyperlinkEventInRust<true>(wxHyperlinkEvent) impl
        HyperlinkEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> HyperlinkEventInRust<IN_RUST> {
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxHyperlinkEvent::wxHyperlinkEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html#aa80a85f87e14160f23feca68f8819740).
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        url: &str,
    ) -> HyperlinkEventInRust<IN_RUST> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let url = WxString::from(url);
            let url = url.as_ptr();
            HyperlinkEventInRust(ffi::wxHyperlinkEvent_new(generator, id, url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HyperlinkEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<HyperlinkEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: HyperlinkEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HyperlinkEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: HyperlinkEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<HyperlinkEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: HyperlinkEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for HyperlinkEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxHyperlinkEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for HyperlinkEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
