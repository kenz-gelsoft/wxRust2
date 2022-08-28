use super::*;

// wxWindow
wxwidgets! {
    /// wxWindow is the base class for all windows and represents any visible object on screen.
    ///
    /// [See `wxWindow`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html)
    #[doc(alias = "wxWindow")]
    #[doc(alias = "Window")]
    class Window
        = WindowIsOwned<true>(wxWindow) impl
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxWindow::wxWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a695200a4915b934926dcf32afa44544c)
    pub fn new_2step() -> WindowIsOwned<OWNED> {
        unsafe { WindowIsOwned(ffi::wxWindow_new()) }
    }
    /// Constructs a window, which can be a child of a frame, dialog or any other non-control window.
    ///
    /// [See `wxWindow::wxWindow()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window.html#a7799009b10d1765d1bbb6db4994f922e)
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> WindowIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            WindowIsOwned(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindow_CLASSINFO()) }
    }
}

// wxWindowCreateEvent
wxwidgets! {
    /// This event is sent just after the actual window associated with a wxWindow object has been created.
    ///
    /// [See `wxWindowCreateEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_create_event.html)
    #[doc(alias = "wxWindowCreateEvent")]
    #[doc(alias = "WindowCreateEvent")]
    class WindowCreateEvent
        = WindowCreateEventIsOwned<true>(wxWindowCreateEvent) impl
        WindowCreateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowCreateEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxWindowCreateEvent::wxWindowCreateEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_create_event.html#a802f4351900c901d07b9068a8cdfad89)
    pub fn new<W: WindowMethods>(win: Option<&W>) -> WindowCreateEventIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowCreateEventIsOwned(ffi::wxWindowCreateEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowCreateEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowCreateEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: WindowCreateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowCreateEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: WindowCreateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowCreateEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowCreateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowCreateEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindowCreateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowCreateEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDC
wxwidgets! {
    /// A wxWindowDC must be constructed if an application wishes to paint on the whole area of a window (client and decorations).
    ///
    /// [See `wxWindowDC`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_d_c.html)
    #[doc(alias = "wxWindowDC")]
    #[doc(alias = "WindowDC")]
    class WindowDC
        = WindowDCIsOwned<true>(wxWindowDC) impl
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDCIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxWindowDC::wxWindowDC()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_d_c.html#afc21b6628de66136f2b223914b1998fb)
    pub fn new<W: WindowMethods>(window: Option<&W>) -> WindowDCIsOwned<OWNED> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDCIsOwned(ffi::wxWindowDC_new(window))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowDCIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: WindowDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindowDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDestroyEvent
wxwidgets! {
    /// This event is sent as early as possible during the window destruction process.
    ///
    /// [See `wxWindowDestroyEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_destroy_event.html)
    #[doc(alias = "wxWindowDestroyEvent")]
    #[doc(alias = "WindowDestroyEvent")]
    class WindowDestroyEvent
        = WindowDestroyEventIsOwned<true>(wxWindowDestroyEvent) impl
        WindowDestroyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDestroyEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// [See `wxWindowDestroyEvent::wxWindowDestroyEvent()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_destroy_event.html#a4f4750d7c673eaa251e2918665f2288d)
    pub fn new<W: WindowMethods>(win: Option<&W>) -> WindowDestroyEventIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDestroyEventIsOwned(ffi::wxWindowDestroyEvent_new(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowDestroyEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WindowDestroyEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: WindowDestroyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDestroyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: WindowDestroyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WindowDestroyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WindowDestroyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WindowDestroyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWindowDestroyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WindowDestroyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWindowDisabler
wxwidgets! {
    /// This class disables all top level windows of the application (maybe with the exception of one of them) in its constructor and enables them back in its destructor.
    ///
    /// [See `wxWindowDisabler`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html)
    #[doc(alias = "wxWindowDisabler")]
    #[doc(alias = "WindowDisabler")]
    class WindowDisabler
        = WindowDisablerIsOwned<true>(wxWindowDisabler) impl
        WindowDisablerMethods
}
impl<const OWNED: bool> WindowDisablerIsOwned<OWNED> {
    /// Disables all top level windows of the applications.
    ///
    /// [See `wxWindowDisabler::wxWindowDisabler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html#a1f7e8380ee6f7c3b709a54d68770e08b)
    pub fn new_with_bool(disable: bool) -> WindowDisablerIsOwned<OWNED> {
        unsafe { WindowDisablerIsOwned(ffi::wxWindowDisabler_new(disable)) }
    }
    /// Disables all top level windows of the applications with the exception of winToSkip if it is not NULL.
    ///
    /// [See `wxWindowDisabler::wxWindowDisabler()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_window_disabler.html#a5e1fa520ab0c951c858c6f9e9aa34613)
    pub fn new_with_window<W: WindowMethods, W2: WindowMethods>(
        win_to_skip: Option<&W>,
        win_to_skip2: Option<&W2>,
    ) -> WindowDisablerIsOwned<OWNED> {
        unsafe {
            let win_to_skip = match win_to_skip {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let win_to_skip2 = match win_to_skip2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowDisablerIsOwned(ffi::wxWindowDisabler_new1(win_to_skip, win_to_skip2))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WindowDisablerIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for WindowDisablerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowDisabler_delete(self.0) }
        }
    }
}

// wxWizard
wxwidgets! {
    /// wxWizard is the central class for implementing 'wizard-like' dialogs.
    ///
    /// [See `wxWizard`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html)
    #[doc(alias = "wxWizard")]
    #[doc(alias = "Wizard")]
    class Wizard
        = WizardIsOwned<true>(wxWizard) impl
        WizardMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxWizard::wxWizard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#ac871d30f9bcaf00718bbd5aafd2005f6)
    pub fn new_2step() -> WizardIsOwned<OWNED> {
        unsafe { WizardIsOwned(ffi::wxWizard_new()) }
    }
    /// Constructor which really creates the wizard  if you use this constructor, you shouldn't call Create().
    ///
    /// [See `wxWizard::wxWizard()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard.html#a4559db6b1490a941a8432a765462bbec)
    pub fn new<W: WindowMethods, B: BitmapBundleMethods, P: PointMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        bitmap: &B,
        pos: &P,
        style: c_long,
    ) -> WizardIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let bitmap = bitmap.as_ptr();
            let pos = pos.as_ptr();
            WizardIsOwned(ffi::wxWizard_new1(parent, id, title, bitmap, pos, style))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WizardIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizard_CLASSINFO()) }
    }
}

// wxWizardEvent
wxwidgets! {
    /// wxWizardEvent class represents an event generated by the wxWizard: this event is first sent to the page itself and, if not processed there, goes up the window hierarchy as usual.
    ///
    /// [See `wxWizardEvent`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_event.html)
    #[doc(alias = "wxWizardEvent")]
    #[doc(alias = "WizardEvent")]
    class WizardEvent
        = WizardEventIsOwned<true>(wxWizardEvent) impl
        WizardEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxWizardEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for WizardEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizardEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for WizardEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxWizardPage
wxwidgets! {
    /// wxWizardPage is one of the screens in wxWizard: it must know what are the following and preceding pages (which may be NULL for the first/last page).
    ///
    /// [See `wxWizardPage`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page.html)
    #[doc(alias = "wxWizardPage")]
    #[doc(alias = "WizardPage")]
    class WizardPage
        = WizardPageIsOwned<true>(wxWizardPage) impl
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageIsOwned<OWNED> {
    // BLOCKED: fn wxWizardPage()
    // BLOCKED: fn wxWizardPage1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WizardPageIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardPageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardPageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizardPage_CLASSINFO()) }
    }
}

// wxWizardPageSimple
wxwidgets! {
    /// wxWizardPageSimple is the simplest possible wxWizardPage implementation: it just returns the pointers given to its constructor from wxWizardPage::GetNext() and wxWizardPage::GetPrev() functions.
    ///
    /// [See `wxWizardPageSimple`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html)
    #[doc(alias = "wxWizardPageSimple")]
    #[doc(alias = "WizardPageSimple")]
    class WizardPageSimple
        = WizardPageSimpleIsOwned<true>(wxWizardPageSimple) impl
        WizardPageSimpleMethods,
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageSimpleIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxWizardPageSimple::wxWizardPageSimple()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#a244829c284b07c21adc67ea739ec7f9b)
    pub fn new_2step() -> WizardPageSimpleIsOwned<OWNED> {
        unsafe { WizardPageSimpleIsOwned(ffi::wxWizardPageSimple_new()) }
    }
    /// Constructor takes the previous and next pages.
    ///
    /// [See `wxWizardPageSimple::wxWizardPageSimple()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wizard_page_simple.html#abb14e4b94589717dc4ad5e0a81f59e70)
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
    ) -> WizardPageSimpleIsOwned<OWNED> {
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
            WizardPageSimpleIsOwned(ffi::wxWizardPageSimple_new1(parent, prev, next, bitmap))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WizardPageSimpleIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for WizardPageIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WizardPageSimpleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WizardPageSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WizardPageSimpleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWizardPageSimple_CLASSINFO()) }
    }
}

// wxWrapSizer
wxwidgets! {
    /// A wrap sizer lays out its items in a single line, like a box sizer  as long as there is space available in that direction.
    ///
    /// [See `wxWrapSizer`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wrap_sizer.html)
    #[doc(alias = "wxWrapSizer")]
    #[doc(alias = "WrapSizer")]
    class WrapSizer
        = WrapSizerIsOwned<true>(wxWrapSizer) impl
        WrapSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WrapSizerIsOwned<OWNED> {
    /// Constructor for a wxWrapSizer.
    ///
    /// [See `wxWrapSizer::wxWrapSizer()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_wrap_sizer.html#a49f0eff212b41b84b7b4eebd98dac489)
    pub fn new(orient: c_int, flags: c_int) -> WrapSizerIsOwned<OWNED> {
        unsafe { WrapSizerIsOwned(ffi::wxWrapSizer_new(orient, flags)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for WrapSizerIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<WrapSizerIsOwned<OWNED>> for BoxSizerIsOwned<OWNED> {
    fn from(o: WrapSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WrapSizerIsOwned<OWNED>> for SizerIsOwned<OWNED> {
    fn from(o: WrapSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<WrapSizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: WrapSizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for WrapSizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxWrapSizer_CLASSINFO()) }
    }
}
