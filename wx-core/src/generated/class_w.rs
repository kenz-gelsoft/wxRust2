use super::*;

// wxWindow
wx_class! { Window =
    WindowIsOwned<true>(wxWindow) impl
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowIsOwned<OWNED> {
    pub fn new_2step() -> WindowIsOwned<OWNED> {
        unsafe { WindowIsOwned(ffi::wxWindow_new()) }
    }
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
impl<const OWNED: bool> Trackable<WindowIsOwned<false>> for WindowIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<WindowIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxWindowCreateEvent
wx_class! { WindowCreateEvent =
    WindowCreateEventIsOwned<true>(wxWindowCreateEvent) impl
        WindowCreateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowCreateEventIsOwned<OWNED> {
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
wx_class! { WindowDC =
    WindowDCIsOwned<true>(wxWindowDC) impl
        WindowDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDCIsOwned<OWNED> {
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
wx_class! { WindowDestroyEvent =
    WindowDestroyEventIsOwned<true>(wxWindowDestroyEvent) impl
        WindowDestroyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> WindowDestroyEventIsOwned<OWNED> {
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
wx_class! { WindowDisabler =
    WindowDisablerIsOwned<true>(wxWindowDisabler) impl
        WindowDisablerMethods
}
impl<const OWNED: bool> WindowDisablerIsOwned<OWNED> {
    pub fn new_with_bool(disable: bool) -> WindowDisablerIsOwned<OWNED> {
        unsafe { WindowDisablerIsOwned(ffi::wxWindowDisabler_new(disable)) }
    }
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
impl<const OWNED: bool> Drop for WindowDisablerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowDisabler_delete(self.0) }
        }
    }
}

// wxWizard
wx_class! { Wizard =
    WizardIsOwned<true>(wxWizard) impl
        WizardMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardIsOwned<OWNED> {
    pub fn new_2step() -> WizardIsOwned<OWNED> {
        unsafe { WizardIsOwned(ffi::wxWizard_new()) }
    }
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
impl<const OWNED: bool> Trackable<WizardIsOwned<false>> for WizardIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<WizardIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxWizardEvent
wx_class! { WizardEvent =
    WizardEventIsOwned<true>(wxWizardEvent) impl
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
wx_class! { WizardPage =
    WizardPageIsOwned<true>(wxWizardPage) impl
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
impl<const OWNED: bool> Trackable<WizardPageIsOwned<false>> for WizardPageIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<WizardPageIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxWizardPageSimple
wx_class! { WizardPageSimple =
    WizardPageSimpleIsOwned<true>(wxWizardPageSimple) impl
        WizardPageSimpleMethods,
        WizardPageMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WizardPageSimpleIsOwned<OWNED> {
    pub fn new_2step() -> WizardPageSimpleIsOwned<OWNED> {
        unsafe { WizardPageSimpleIsOwned(ffi::wxWizardPageSimple_new()) }
    }
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
impl<const OWNED: bool> Trackable<WizardPageSimpleIsOwned<false>>
    for WizardPageSimpleIsOwned<OWNED>
{
    fn to_weak_ref(&self) -> WeakRef<WizardPageSimpleIsOwned<false>> {
        unsafe { WeakRef::from(self.as_ptr()) }
    }
}

// wxWrapSizer
wx_class! { WrapSizer =
    WrapSizerIsOwned<true>(wxWrapSizer) impl
        WrapSizerMethods,
        BoxSizerMethods,
        SizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> WrapSizerIsOwned<OWNED> {
    pub fn new(orient: c_int, flags: c_int) -> WrapSizerIsOwned<OWNED> {
        unsafe { WrapSizerIsOwned(ffi::wxWrapSizer_new(orient, flags)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
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
