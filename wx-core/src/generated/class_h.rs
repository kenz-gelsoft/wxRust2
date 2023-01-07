use super::*;

// wxHScrolledWindow
wxwidgets! {
    /// In the name of this class, "H" stands for "horizontal" because it can be used for scrolling columns of variable widths.
    /// - [`HScrolledWindow`] represents a C++ `wxHScrolledWindow` class instance which your code has ownership, [`HScrolledWindowFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HScrolledWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHScrolledWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_h_scrolled_window.html) for more details.
    #[doc(alias = "wxHScrolledWindow")]
    #[doc(alias = "HScrolledWindow")]
    class HScrolledWindow
        = HScrolledWindowFromCpp<true>(wxHScrolledWindow) impl
        HScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HScrolledWindowFromCpp<FROM_CPP> {
    // BLOCKED: fn wxHScrolledWindow()
    // BLOCKED: fn wxHScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for HScrolledWindowFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HScrolledWindowFromCpp<FROM_CPP>> for PanelFromCpp<FROM_CPP> {
    fn from(o: HScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HScrolledWindowFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: HScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HScrolledWindowFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: HScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HScrolledWindowFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HScrolledWindowFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHScrolledWindow_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for HScrolledWindowFromCpp<FROM_CPP> {
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
    /// - [`HTMLDataObject`] represents a C++ `wxHTMLDataObject` class instance which your code has ownership, [`HTMLDataObjectFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HTMLDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHTMLDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html) for more details.
    #[doc(alias = "wxHTMLDataObject")]
    #[doc(alias = "HTMLDataObject")]
    class HTMLDataObject
        = HTMLDataObjectFromCpp<true>(wxHTMLDataObject) impl
        HTMLDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> HTMLDataObjectFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxHTMLDataObject::wxHTMLDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_h_t_m_l_data_object.html#a3e09f114be84aef40c05809ddfe83333).
    pub fn new(html: &str) -> HTMLDataObjectFromCpp<FROM_CPP> {
        unsafe {
            let html = WxString::from(html);
            let html = html.as_ptr();
            HTMLDataObjectFromCpp(ffi::wxHTMLDataObject_new(html))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HTMLDataObjectFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HTMLDataObjectFromCpp<FROM_CPP>>
    for DataObjectSimpleFromCpp<FROM_CPP>
{
    fn from(o: HTMLDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HTMLDataObjectFromCpp<FROM_CPP>> for DataObjectFromCpp<FROM_CPP> {
    fn from(o: HTMLDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for HTMLDataObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxHTMLDataObject_delete(self.0) }
        }
    }
}

// wxHVScrolledWindow
wxwidgets! {
    /// This window inherits all functionality of both vertical and horizontal, variable scrolled windows.
    /// - [`HVScrolledWindow`] represents a C++ `wxHVScrolledWindow` class instance which your code has ownership, [`HVScrolledWindowFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HVScrolledWindow`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHVScrolledWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_h_v_scrolled_window.html) for more details.
    #[doc(alias = "wxHVScrolledWindow")]
    #[doc(alias = "HVScrolledWindow")]
    class HVScrolledWindow
        = HVScrolledWindowFromCpp<true>(wxHVScrolledWindow) impl
        HVScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HVScrolledWindowFromCpp<FROM_CPP> {
    // BLOCKED: fn wxHVScrolledWindow()
    // BLOCKED: fn wxHVScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for HVScrolledWindowFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HVScrolledWindowFromCpp<FROM_CPP>> for PanelFromCpp<FROM_CPP> {
    fn from(o: HVScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HVScrolledWindowFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: HVScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HVScrolledWindowFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: HVScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HVScrolledWindowFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HVScrolledWindowFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HVScrolledWindowFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHVScrolledWindow_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for HVScrolledWindowFromCpp<FROM_CPP> {
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
    /// - [`HeaderColumn`] represents a C++ `wxHeaderColumn` class instance which your code has ownership, [`HeaderColumnFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HeaderColumn`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderColumn` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_column.html) for more details.
    #[doc(alias = "wxHeaderColumn")]
    #[doc(alias = "HeaderColumn")]
    class HeaderColumn
        = HeaderColumnFromCpp<true>(wxHeaderColumn) impl
        HeaderColumnMethods
}
impl<const FROM_CPP: bool> HeaderColumnFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderColumnFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for HeaderColumnFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxHeaderColumn_delete(self.0) }
        }
    }
}

// wxHeaderColumnSimple
wxwidgets! {
    /// Simple container for the information about the column.
    /// - [`HeaderColumnSimple`] represents a C++ `wxHeaderColumnSimple` class instance which your code has ownership, [`HeaderColumnSimpleFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HeaderColumnSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderColumnSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html) for more details.
    #[doc(alias = "wxHeaderColumnSimple")]
    #[doc(alias = "HeaderColumnSimple")]
    class HeaderColumnSimple
        = HeaderColumnSimpleFromCpp<true>(wxHeaderColumnSimple) impl
        HeaderColumnSimpleMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const FROM_CPP: bool> HeaderColumnSimpleFromCpp<FROM_CPP> {
    /// Constructor for a column header.
    ///
    /// See [C++ `wxHeaderColumnSimple::wxHeaderColumnSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html#a219431d463c61334f8de7ac45ebb16f0).
    pub fn new_with_str(
        title: &str,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleFromCpp<FROM_CPP> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            HeaderColumnSimpleFromCpp(ffi::wxHeaderColumnSimple_new(title, width, align, flags))
        }
    }
    ///
    /// See [C++ `wxHeaderColumnSimple::wxHeaderColumnSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_column_simple.html#af5ed3aa43db1341f91b268a045e8f556).
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(
        bitmap: &B,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleFromCpp<FROM_CPP> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            HeaderColumnSimpleFromCpp(ffi::wxHeaderColumnSimple_new1(bitmap, width, align, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderColumnSimpleFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HeaderColumnSimpleFromCpp<FROM_CPP>>
    for SettableHeaderColumnFromCpp<FROM_CPP>
{
    fn from(o: HeaderColumnSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderColumnSimpleFromCpp<FROM_CPP>>
    for HeaderColumnFromCpp<FROM_CPP>
{
    fn from(o: HeaderColumnSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for HeaderColumnSimpleFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxHeaderColumnSimple_delete(self.0) }
        }
    }
}

// wxHeaderCtrl
wxwidgets! {
    /// wxHeaderCtrl is the control containing the column headings which is usually used for display of tabular data.
    /// - [`HeaderCtrl`] represents a C++ `wxHeaderCtrl` class instance which your code has ownership, [`HeaderCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HeaderCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl.html) for more details.
    #[doc(alias = "wxHeaderCtrl")]
    #[doc(alias = "HeaderCtrl")]
    class HeaderCtrl
        = HeaderCtrlFromCpp<true>(wxHeaderCtrl) impl
        HeaderCtrlMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HeaderCtrlFromCpp<FROM_CPP> {
    // BLOCKED: fn wxHeaderCtrl()
    // BLOCKED: fn wxHeaderCtrl1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for HeaderCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HeaderCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHeaderCtrl_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> WindowMethods for HeaderCtrlFromCpp<FROM_CPP> {
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
    /// - [`HeaderCtrlEvent`] represents a C++ `wxHeaderCtrlEvent` class instance which your code has ownership, [`HeaderCtrlEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HeaderCtrlEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderCtrlEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html) for more details.
    #[doc(alias = "wxHeaderCtrlEvent")]
    #[doc(alias = "HeaderCtrlEvent")]
    class HeaderCtrlEvent
        = HeaderCtrlEventFromCpp<true>(wxHeaderCtrlEvent) impl
        HeaderCtrlEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HeaderCtrlEventFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxHeaderCtrlEvent()
    ///
    /// See [C++ `wxHeaderCtrlEvent::wxHeaderCtrlEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_event.html#aa9c9fdd75b48ea8dfcb976ed16dbfcd1).
    pub fn new<H: HeaderCtrlEventMethods>(event: &H) -> HeaderCtrlEventFromCpp<FROM_CPP> {
        unsafe {
            let event = event.as_ptr();
            HeaderCtrlEventFromCpp(ffi::wxHeaderCtrlEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderCtrlEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlEventFromCpp<FROM_CPP>> for NotifyEventFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlEventFromCpp<FROM_CPP>>
    for CommandEventFromCpp<FROM_CPP>
{
    fn from(o: HeaderCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HeaderCtrlEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHeaderCtrlEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for HeaderCtrlEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHeaderCtrlSimple
wxwidgets! {
    /// wxHeaderCtrlSimple is a concrete header control which can be used directly, without inheriting from it as you need to do when using wxHeaderCtrl itself.
    /// - [`HeaderCtrlSimple`] represents a C++ `wxHeaderCtrlSimple` class instance which your code has ownership, [`HeaderCtrlSimpleFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HeaderCtrlSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHeaderCtrlSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html) for more details.
    #[doc(alias = "wxHeaderCtrlSimple")]
    #[doc(alias = "HeaderCtrlSimple")]
    class HeaderCtrlSimple
        = HeaderCtrlSimpleFromCpp<true>(wxHeaderCtrlSimple) impl
        HeaderCtrlSimpleMethods,
        HeaderCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HeaderCtrlSimpleFromCpp<FROM_CPP> {
    /// Default constructor not creating the underlying window.
    ///
    /// See [C++ `wxHeaderCtrlSimple::wxHeaderCtrlSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_header_ctrl_simple.html#a9a0288a46b35b0ebc7e8704a082dbd58).
    pub fn new_2step() -> HeaderCtrlSimpleFromCpp<FROM_CPP> {
        unsafe { HeaderCtrlSimpleFromCpp(ffi::wxHeaderCtrlSimple_new()) }
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
    ) -> HeaderCtrlSimpleFromCpp<FROM_CPP> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HeaderCtrlSimpleFromCpp(ffi::wxHeaderCtrlSimple_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for HeaderCtrlSimpleFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlSimpleFromCpp<FROM_CPP>> for HeaderCtrlFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlSimpleFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlSimpleFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlSimpleFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HeaderCtrlSimpleFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HeaderCtrlSimpleFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HeaderCtrlSimpleFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHeaderCtrlSimple_CLASSINFO()) }
    }
}

// wxHelpEvent
wxwidgets! {
    /// A help event is sent when the user has requested context-sensitive help.
    /// - [`HelpEvent`] represents a C++ `wxHelpEvent` class instance which your code has ownership, [`HelpEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HelpEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHelpEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_help_event.html) for more details.
    #[doc(alias = "wxHelpEvent")]
    #[doc(alias = "HelpEvent")]
    class HelpEvent
        = HelpEventFromCpp<true>(wxHelpEvent) impl
        HelpEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HelpEventFromCpp<FROM_CPP> {
    //  ENUM: Origin
    pub const Origin_Unknown: c_int = 0;
    pub const Origin_Keyboard: c_int = 0 + 1;
    pub const Origin_HelpButton: c_int = 0 + 2;

    // NOT_SUPPORTED: fn wxHelpEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HelpEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HelpEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: HelpEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HelpEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: HelpEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HelpEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HelpEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HelpEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHelpEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for HelpEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHyperlinkCtrl
wxwidgets! {
    /// This class shows a static text element which links to an URL.
    /// - [`HyperlinkCtrl`] represents a C++ `wxHyperlinkCtrl` class instance which your code has ownership, [`HyperlinkCtrlFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HyperlinkCtrl`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHyperlinkCtrl` class's documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html) for more details.
    #[doc(alias = "wxHyperlinkCtrl")]
    #[doc(alias = "HyperlinkCtrl")]
    class HyperlinkCtrl
        = HyperlinkCtrlFromCpp<true>(wxHyperlinkCtrl) impl
        HyperlinkCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HyperlinkCtrlFromCpp<FROM_CPP> {
    ///
    /// See [C++ `wxHyperlinkCtrl::wxHyperlinkCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_ctrl.html#a9b0d333830376d2d437f35f31656405f).
    pub fn new_2step() -> HyperlinkCtrlFromCpp<FROM_CPP> {
        unsafe { HyperlinkCtrlFromCpp(ffi::wxHyperlinkCtrl_new()) }
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
    ) -> HyperlinkCtrlFromCpp<FROM_CPP> {
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
            HyperlinkCtrlFromCpp(ffi::wxHyperlinkCtrl_new1(
                parent, id, label, url, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for HyperlinkCtrlFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HyperlinkCtrlFromCpp<FROM_CPP>> for ControlFromCpp<FROM_CPP> {
    fn from(o: HyperlinkCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HyperlinkCtrlFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: HyperlinkCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HyperlinkCtrlFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: HyperlinkCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HyperlinkCtrlFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HyperlinkCtrlFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HyperlinkCtrlFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHyperlinkCtrl_CLASSINFO()) }
    }
}

// wxHyperlinkEvent
wxwidgets! {
    /// This event class is used for the events generated by wxHyperlinkCtrl.
    /// - [`HyperlinkEvent`] represents a C++ `wxHyperlinkEvent` class instance which your code has ownership, [`HyperlinkEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`HyperlinkEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxHyperlinkEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html) for more details.
    #[doc(alias = "wxHyperlinkEvent")]
    #[doc(alias = "HyperlinkEvent")]
    class HyperlinkEvent
        = HyperlinkEventFromCpp<true>(wxHyperlinkEvent) impl
        HyperlinkEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> HyperlinkEventFromCpp<FROM_CPP> {
    /// The constructor is not normally used by the user code.
    ///
    /// See [C++ `wxHyperlinkEvent::wxHyperlinkEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_hyperlink_event.html#aa80a85f87e14160f23feca68f8819740).
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        url: &str,
    ) -> HyperlinkEventFromCpp<FROM_CPP> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let url = WxString::from(url);
            let url = url.as_ptr();
            HyperlinkEventFromCpp(ffi::wxHyperlinkEvent_new(generator, id, url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HyperlinkEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<HyperlinkEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: HyperlinkEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HyperlinkEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: HyperlinkEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<HyperlinkEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: HyperlinkEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for HyperlinkEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxHyperlinkEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for HyperlinkEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
