#![allow(non_upper_case_globals)]

use super::*;

// wxEditableListBox
wx_class! { EditableListBox =
    EditableListBoxIsOwned<true>(wxEditableListBox) impl
        EditableListBoxMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EditableListBoxIsOwned<OWNED> {
    pub fn new_2step() -> EditableListBoxIsOwned<OWNED> {
        unsafe { EditableListBoxIsOwned(ffi::wxEditableListBox_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> EditableListBoxIsOwned<OWNED> {
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
            EditableListBoxIsOwned(ffi::wxEditableListBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EditableListBoxIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EditableListBoxIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EditableListBoxIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEditableListBox_CLASSINFO()) }
    }
}

// wxEraseEvent
wx_class! { EraseEvent =
    EraseEventIsOwned<true>(wxEraseEvent) impl
        EraseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> EraseEventIsOwned<OWNED> {
    pub fn new<D: DCMethods>(id: c_int, dc: Option<&D>) -> EraseEventIsOwned<OWNED> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            EraseEventIsOwned(ffi::wxEraseEvent_new(id, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EraseEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: EraseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EraseEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EraseEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EraseEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEraseEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for EraseEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEventBlocker
wx_class! { EventBlocker =
    EventBlockerIsOwned<true>(wxEventBlocker) impl
        EventBlockerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EventBlockerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxEventBlocker()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EventBlockerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: EventBlockerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<EventBlockerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EventBlockerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EventBlockerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEventBlocker_CLASSINFO()) }
    }
}
