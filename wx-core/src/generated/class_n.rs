use super::*;

// wxNativeFontInfo
wx_class! { NativeFontInfo =
    NativeFontInfoIsOwned<true>(wxNativeFontInfo) impl
        NativeFontInfoMethods
}
impl<const OWNED: bool> NativeFontInfoIsOwned<OWNED> {
    pub fn new() -> NativeFontInfoIsOwned<OWNED> {
        unsafe { NativeFontInfoIsOwned(ffi::wxNativeFontInfo_new()) }
    }
    pub fn new_with_nativefontinfo<N: NativeFontInfoMethods>(
        info: &N,
    ) -> NativeFontInfoIsOwned<OWNED> {
        unsafe {
            let info = info.as_ptr();
            NativeFontInfoIsOwned(ffi::wxNativeFontInfo_new1(info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for NativeFontInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxNativeFontInfo_delete(self.0) }
        }
    }
}

// wxNavigationKeyEvent
wx_class! { NavigationKeyEvent =
    NavigationKeyEventIsOwned<true>(wxNavigationKeyEvent) impl
        NavigationKeyEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NavigationKeyEventIsOwned<OWNED> {
    //  ENUM: wxNavigationKeyEventFlags
    pub const IsBackward: c_int = 0x0000;
    pub const IsForward: c_int = 0x0001;
    pub const WinChange: c_int = 0x0002;
    pub const FromTab: c_int = 0x0004;

    pub fn new() -> NavigationKeyEventIsOwned<OWNED> {
        unsafe { NavigationKeyEventIsOwned(ffi::wxNavigationKeyEvent_new()) }
    }
    pub fn new_with_navigationkeyevent<N: NavigationKeyEventMethods>(
        event: &N,
    ) -> NavigationKeyEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            NavigationKeyEventIsOwned(ffi::wxNavigationKeyEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NavigationKeyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: NavigationKeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NavigationKeyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NavigationKeyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NavigationKeyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNavigationKeyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for NavigationKeyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNonOwnedWindow
wx_class! { NonOwnedWindow =
    NonOwnedWindowIsOwned<true>(wxNonOwnedWindow) impl
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NonOwnedWindowIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NonOwnedWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NonOwnedWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NonOwnedWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NonOwnedWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NonOwnedWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NonOwnedWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NonOwnedWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNonOwnedWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Trackable<NonOwnedWindowIsOwned<false>> for NonOwnedWindowIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<NonOwnedWindowIsOwned<false>> {
        unsafe { WeakRef::from_ptr(self.as_ptr()) }
    }
}

// wxNotebook
wx_class! { Notebook =
    NotebookIsOwned<true>(wxNotebook) impl
        NotebookMethods,
        BookCtrlBaseMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotebookIsOwned<OWNED> {
    pub fn new_2step() -> NotebookIsOwned<OWNED> {
        unsafe { NotebookIsOwned(ffi::wxNotebook_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> NotebookIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            NotebookIsOwned(ffi::wxNotebook_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for BookCtrlBaseIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotebookIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NotebookIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotebookIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNotebook_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Trackable<NotebookIsOwned<false>> for NotebookIsOwned<OWNED> {
    fn to_weak_ref(&self) -> WeakRef<NotebookIsOwned<false>> {
        unsafe { WeakRef::from_ptr(self.as_ptr()) }
    }
}
impl<const OWNED: bool> WindowMethods for NotebookIsOwned<OWNED> {
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
            ffi::wxNotebook_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxNotificationMessage
wx_class! { NotificationMessage =
    NotificationMessageIsOwned<true>(wxNotificationMessage) impl
        NotificationMessageMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotificationMessageIsOwned<OWNED> {
    //  ENUM: @38
    pub const Timeout_Auto: c_int = -1;
    pub const Timeout_Never: c_int = 0;

    pub fn new() -> NotificationMessageIsOwned<OWNED> {
        unsafe { NotificationMessageIsOwned(ffi::wxNotificationMessage_new()) }
    }
    pub fn new_with_str<W: WindowMethods>(
        title: &str,
        message: &str,
        parent: Option<&W>,
        flags: c_int,
    ) -> NotificationMessageIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            NotificationMessageIsOwned(ffi::wxNotificationMessage_new1(
                title, message, parent, flags,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NotificationMessageIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NotificationMessageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotificationMessageIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NotificationMessageIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotificationMessageIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNotificationMessage_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Trackable<NotificationMessageIsOwned<false>>
    for NotificationMessageIsOwned<OWNED>
{
    fn to_weak_ref(&self) -> WeakRef<NotificationMessageIsOwned<false>> {
        unsafe { WeakRef::from_ptr(self.as_ptr()) }
    }
}

// wxNotifyEvent
wx_class! { NotifyEvent =
    NotifyEventIsOwned<true>(wxNotifyEvent) impl
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> NotifyEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxNotifyEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NotifyEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: NotifyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotifyEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: NotifyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NotifyEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NotifyEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NotifyEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNotifyEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for NotifyEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxNumberEntryDialog
wx_class! { NumberEntryDialog =
    NumberEntryDialogIsOwned<true>(wxNumberEntryDialog) impl
        NumberEntryDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> NumberEntryDialogIsOwned<OWNED> {
    pub fn new_2step() -> NumberEntryDialogIsOwned<OWNED> {
        unsafe { NumberEntryDialogIsOwned(ffi::wxNumberEntryDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        prompt: &str,
        caption: &str,
        value: c_long,
        min: c_long,
        max: c_long,
        pos: &P,
    ) -> NumberEntryDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let prompt = WxString::from(prompt);
            let prompt = prompt.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            NumberEntryDialogIsOwned(ffi::wxNumberEntryDialog_new1(
                parent, message, prompt, caption, value, min, max, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<NumberEntryDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: NumberEntryDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for NumberEntryDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxNumberEntryDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Trackable<NumberEntryDialogIsOwned<false>>
    for NumberEntryDialogIsOwned<OWNED>
{
    fn to_weak_ref(&self) -> WeakRef<NumberEntryDialogIsOwned<false>> {
        unsafe { WeakRef::from_ptr(self.as_ptr()) }
    }
}
