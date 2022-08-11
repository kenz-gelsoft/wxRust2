#![allow(non_upper_case_globals)]

use super::*;

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

// wxNotebook
wx_class! { Notebook =
    NotebookIsOwned<true>(wxNotebook) impl
        NotebookMethods,
        // BookCtrlBaseMethods,
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
impl<const OWNED: bool> BookCtrlBaseMethods for NotebookIsOwned<OWNED> {
    // BLOCKED: fn Create()
}
impl<const OWNED: bool> WindowMethods for NotebookIsOwned<OWNED> {
    // BLOCKED: fn Create()
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
