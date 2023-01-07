use super::*;

// wxWindow
wxwidgets! {
    /// wxWindow is the base class for all windows and represents any visible object on screen.
    /// - [`Window`] represents a C++ `wxWindow` class instance which your code has ownership, [`WindowInRust`]`<false>` represents one which don't own.
    /// - Use [`Window`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWindow` class's documentation](https://docs.wxwidgets.org/3.2/classwx_window.html) for more details.
    #[doc(alias = "wxWindow")]
    #[doc(alias = "Window")]
    class Window
        = WindowInRust<true>(wxWindow) impl
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxWindow::wxWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window.html#a695200a4915b934926dcf32afa44544c).
    pub fn new_2step() -> WindowInRust<OWNED> {
        unsafe { WindowInRust(ffi::wxWindow_new()) }
    }
    /// Constructs a window, which can be a child of a frame, dialog or any other non-control window.
    ///
    /// See [C++ `wxWindow::wxWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window.html#a7799009b10d1765d1bbb6db4994f922e).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> WindowInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            WindowInRust(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WindowInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: WindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WindowInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWindow_CLASSINFO()) }
    }
}

// wxWindowCreateEvent
wxwidgets! {
    /// This event is sent just after the actual window associated with a wxWindow object has been created.
    /// - [`WindowCreateEvent`] represents a C++ `wxWindowCreateEvent` class instance which your code has ownership, [`WindowCreateEventInRust`]`<false>` represents one which don't own.
    /// - Use [`WindowCreateEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWindowCreateEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_window_create_event.html) for more details.
    #[doc(alias = "wxWindowCreateEvent")]
    #[doc(alias = "WindowCreateEvent")]
    class WindowCreateEvent
        = WindowCreateEventInRust<true>(wxWindowCreateEvent) impl
        WindowCreateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowCreateEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxWindowCreateEvent::wxWindowCreateEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window_create_event.html#a802f4351900c901d07b9068a8cdfad89).
    pub fn new<W: WindowMethods>(win: Option<&W>) -> WindowCreateEventInRust<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowCreateEventInRust(ffi::wxWindowCreateEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowCreateEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowCreateEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: WindowCreateEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowCreateEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: WindowCreateEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowCreateEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WindowCreateEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowCreateEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWindowCreateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowCreateEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDC
wxwidgets! {
    /// A wxWindowDC must be constructed if an application wishes to paint on the whole area of a window (client and decorations).
    /// - [`WindowDC`] represents a C++ `wxWindowDC` class instance which your code has ownership, [`WindowDCInRust`]`<false>` represents one which don't own.
    /// - Use [`WindowDC`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWindowDC` class's documentation](https://docs.wxwidgets.org/3.2/classwx_window_d_c.html) for more details.
    #[doc(alias = "wxWindowDC")]
    #[doc(alias = "WindowDC")]
    class WindowDC
        = WindowDCInRust<true>(wxWindowDC) impl
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDCInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxWindowDC::wxWindowDC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window_d_c.html#afc21b6628de66136f2b223914b1998fb).
    pub fn new<W: WindowMethods>(window: Option<&W>) -> WindowDCInRust<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDCInRust(ffi::wxWindowDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowDCInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowDCInRust<OWNED>> for DCInRust<OWNED> {
    fn from(o: WindowDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDCInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WindowDCInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowDCInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWindowDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowDCInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDestroyEvent
wxwidgets! {
    /// This event is sent as early as possible during the window destruction process.
    /// - [`WindowDestroyEvent`] represents a C++ `wxWindowDestroyEvent` class instance which your code has ownership, [`WindowDestroyEventInRust`]`<false>` represents one which don't own.
    /// - Use [`WindowDestroyEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWindowDestroyEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_window_destroy_event.html) for more details.
    #[doc(alias = "wxWindowDestroyEvent")]
    #[doc(alias = "WindowDestroyEvent")]
    class WindowDestroyEvent
        = WindowDestroyEventInRust<true>(wxWindowDestroyEvent) impl
        WindowDestroyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDestroyEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxWindowDestroyEvent::wxWindowDestroyEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window_destroy_event.html#a4f4750d7c673eaa251e2918665f2288d).
    pub fn new<W: WindowMethods>(win: Option<&W>) -> WindowDestroyEventInRust<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDestroyEventInRust(ffi::wxWindowDestroyEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowDestroyEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowDestroyEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: WindowDestroyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDestroyEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: WindowDestroyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDestroyEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WindowDestroyEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowDestroyEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWindowDestroyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowDestroyEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDisabler
wxwidgets! {
    /// This class disables all top level windows of the application (maybe with the exception of one of them) in its constructor and enables them back in its destructor.
    /// - [`WindowDisabler`] represents a C++ `wxWindowDisabler` class instance which your code has ownership, [`WindowDisablerInRust`]`<false>` represents one which don't own.
    /// - Use [`WindowDisabler`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWindowDisabler` class's documentation](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html) for more details.
    #[doc(alias = "wxWindowDisabler")]
    #[doc(alias = "WindowDisabler")]
    class WindowDisabler
        = WindowDisablerInRust<true>(wxWindowDisabler) impl
        WindowDisablerMethods
}
impl<const OWNED: bool> WindowDisablerInRust<OWNED> {
    /// Disables all top level windows of the applications.
    ///
    /// See [C++ `wxWindowDisabler::wxWindowDisabler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html#a1f7e8380ee6f7c3b709a54d68770e08b).
    pub fn new_with_bool(disable: bool) -> WindowDisablerInRust<OWNED> {
        unsafe { WindowDisablerInRust(ffi::wxWindowDisabler_new(disable)) }
    }
    /// Disables all top level windows of the applications with the exception of winToSkip if it is not NULL.
    ///
    /// See [C++ `wxWindowDisabler::wxWindowDisabler()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html#a5e1fa520ab0c951c858c6f9e9aa34613).
    pub fn new_with_window<W: WindowMethods, W2: WindowMethods>(
        win_to_skip: Option<&W>,
        win_to_skip2: Option<&W2>,
    ) -> WindowDisablerInRust<OWNED> {
        unsafe {
            let win_to_skip = match win_to_skip {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let win_to_skip2 = match win_to_skip2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDisablerInRust(ffi::wxWindowDisabler_new1(win_to_skip, win_to_skip2))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowDisablerInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for WindowDisablerInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowDisabler_delete(self.0) }
        }
    }
}

// wxWizard
wxwidgets! {
    /// wxWizard is the central class for implementing 'wizard-like' dialogs.
    /// - [`Wizard`] represents a C++ `wxWizard` class instance which your code has ownership, [`WizardInRust`]`<false>` represents one which don't own.
    /// - Use [`Wizard`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWizard` class's documentation](https://docs.wxwidgets.org/3.2/classwx_wizard.html) for more details.
    #[doc(alias = "wxWizard")]
    #[doc(alias = "Wizard")]
    class Wizard
        = WizardInRust<true>(wxWizard) impl
        WizardMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxWizard::wxWizard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_wizard.html#ac871d30f9bcaf00718bbd5aafd2005f6).
    pub fn new_2step() -> WizardInRust<OWNED> {
        unsafe { WizardInRust(ffi::wxWizard_new()) }
    }
    /// Constructor which really creates the wizard  if you use this constructor, you shouldn't call Create().
    ///
    /// See [C++ `wxWizard::wxWizard()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a4559db6b1490a941a8432a765462bbec).
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        bitmap: &B,
        pos: &P,
        style: c_long,
    ) -> WizardInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            WizardInRust(ffi::wxWizard_new1(parent, id, title, bitmap, pos, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WizardInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardInRust<OWNED>> for DialogInRust<OWNED> {
    fn from(o: WizardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardInRust<OWNED>> for TopLevelWindowInRust<OWNED> {
    fn from(o: WizardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardInRust<OWNED>> for NonOwnedWindowInRust<OWNED> {
    fn from(o: WizardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: WizardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: WizardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WizardInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWizard_CLASSINFO()) }
    }
}

// wxWizardEvent
wxwidgets! {
    /// wxWizardEvent class represents an event generated by the wxWizard: this event is first sent to the page itself and, if not processed there, goes up the window hierarchy as usual.
    /// - [`WizardEvent`] represents a C++ `wxWizardEvent` class instance which your code has ownership, [`WizardEventInRust`]`<false>` represents one which don't own.
    /// - Use [`WizardEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWizardEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_wizard_event.html) for more details.
    #[doc(alias = "wxWizardEvent")]
    #[doc(alias = "WizardEvent")]
    class WizardEvent
        = WizardEventInRust<true>(wxWizardEvent) impl
        WizardEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardEventInRust<OWNED> {
    // NOT_SUPPORTED: fn wxWizardEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WizardEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardEventInRust<OWNED>> for NotifyEventInRust<OWNED> {
    fn from(o: WizardEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: WizardEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: WizardEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WizardEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWizardEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WizardEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWizardPage
wxwidgets! {
    /// wxWizardPage is one of the screens in wxWizard: it must know what are the following and preceding pages (which may be NULL for the first/last page).
    /// - [`WizardPage`] represents a C++ `wxWizardPage` class instance which your code has ownership, [`WizardPageInRust`]`<false>` represents one which don't own.
    /// - Use [`WizardPage`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWizardPage` class's documentation](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html) for more details.
    #[doc(alias = "wxWizardPage")]
    #[doc(alias = "WizardPage")]
    class WizardPage
        = WizardPageInRust<true>(wxWizardPage) impl
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageInRust<OWNED> {
    // BLOCKED: fn wxWizardPage()
    // BLOCKED: fn wxWizardPage1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WizardPageInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardPageInRust<OWNED>> for PanelInRust<OWNED> {
    fn from(o: WizardPageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: WizardPageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: WizardPageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WizardPageInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardPageInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWizardPage_CLASSINFO()) }
    }
}

// wxWizardPageSimple
wxwidgets! {
    /// wxWizardPageSimple is the simplest possible wxWizardPage implementation: it just returns the pointers given to its constructor from wxWizardPage::GetNext() and wxWizardPage::GetPrev() functions.
    /// - [`WizardPageSimple`] represents a C++ `wxWizardPageSimple` class instance which your code has ownership, [`WizardPageSimpleInRust`]`<false>` represents one which don't own.
    /// - Use [`WizardPageSimple`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWizardPageSimple` class's documentation](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html) for more details.
    #[doc(alias = "wxWizardPageSimple")]
    #[doc(alias = "WizardPageSimple")]
    class WizardPageSimple
        = WizardPageSimpleInRust<true>(wxWizardPageSimple) impl
        WizardPageSimpleMethods,
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageSimpleInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxWizardPageSimple::wxWizardPageSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#a244829c284b07c21adc67ea739ec7f9b).
    pub fn new_2step() -> WizardPageSimpleInRust<OWNED> {
        unsafe { WizardPageSimpleInRust(ffi::wxWizardPageSimple_new()) }
    }
    /// Constructor takes the previous and next pages.
    ///
    /// See [C++ `wxWizardPageSimple::wxWizardPageSimple()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#abb14e4b94589717dc4ad5e0a81f59e70).
    pub fn new<
        W: WizardMethods,
        W2: WizardPageMethods,
        W3: WizardPageMethods,
        B: BitmapBundleMethods,
    >(
        parent: Option<&W>,
        prev: Option<&W2>,
        next: Option<&W3>,
        bitmap: &B,
    ) -> WizardPageSimpleInRust<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let prev = match prev {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let next = match next {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bitmap = bitmap.as_ptr();
            WizardPageSimpleInRust(ffi::wxWizardPageSimple_new1(parent, prev, next, bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WizardPageSimpleInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardPageSimpleInRust<OWNED>> for WizardPageInRust<OWNED> {
    fn from(o: WizardPageSimpleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleInRust<OWNED>> for PanelInRust<OWNED> {
    fn from(o: WizardPageSimpleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleInRust<OWNED>> for WindowInRust<OWNED> {
    fn from(o: WizardPageSimpleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleInRust<OWNED>> for EvtHandlerInRust<OWNED> {
    fn from(o: WizardPageSimpleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WizardPageSimpleInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardPageSimpleInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWizardPageSimple_CLASSINFO()) }
    }
}

// wxWrapSizer
wxwidgets! {
    /// A wrap sizer lays out its items in a single line, like a box sizer  as long as there is space available in that direction.
    /// - [`WrapSizer`] represents a C++ `wxWrapSizer` class instance which your code has ownership, [`WrapSizerInRust`]`<false>` represents one which don't own.
    /// - Use [`WrapSizer`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxWrapSizer` class's documentation](https://docs.wxwidgets.org/3.2/classwx_wrap_sizer.html) for more details.
    #[doc(alias = "wxWrapSizer")]
    #[doc(alias = "WrapSizer")]
    class WrapSizer
        = WrapSizerInRust<true>(wxWrapSizer) impl
        WrapSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WrapSizerInRust<OWNED> {
    /// Constructor for a wxWrapSizer.
    ///
    /// See [C++ `wxWrapSizer::wxWrapSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_wrap_sizer.html#a49f0eff212b41b84b7b4eebd98dac489).
    pub fn new(orient: c_int, flags: c_int) -> WrapSizerInRust<OWNED> {
        unsafe { WrapSizerInRust(ffi::wxWrapSizer_new(orient, flags)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WrapSizerInRust<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WrapSizerInRust<OWNED>> for BoxSizerInRust<OWNED> {
    fn from(o: WrapSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WrapSizerInRust<OWNED>> for SizerInRust<OWNED> {
    fn from(o: WrapSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WrapSizerInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: WrapSizerInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WrapSizerInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxWrapSizer_CLASSINFO()) }
    }
}
