use super::*;

// wxEditableListBox
wxwidgets! {
    /// An editable listbox is composite control that lets the user easily enter, delete and reorder a list of strings.
    /// - [`EditableListBox`] represents a C++ `wxEditableListBox` class instance which your code has ownership, [`EditableListBoxFromCpp`]`<false>` represents one which don't own.
    /// - Use [`EditableListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxEditableListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html) for more details.
    #[doc(alias = "wxEditableListBox")]
    #[doc(alias = "EditableListBox")]
    class EditableListBox
        = EditableListBoxFromCpp<true>(wxEditableListBox) impl
        EditableListBoxMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> EditableListBoxFromCpp<FROM_CPP> {
    /// Default ctor.
    ///
    /// See [C++ `wxEditableListBox::wxEditableListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html#ab42098c0b81e1f027e307fad8a9ecc8a).
    pub fn new_2step() -> EditableListBoxFromCpp<FROM_CPP> {
        unsafe { EditableListBoxFromCpp(ffi::wxEditableListBox_new()) }
    }
    /// Constructor, creating and showing a list box.
    ///
    /// See [C++ `wxEditableListBox::wxEditableListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html#aa00cc37cddb598967111e4ee8a35e812).
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> EditableListBoxFromCpp<FROM_CPP> {
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
            EditableListBoxFromCpp(ffi::wxEditableListBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for EditableListBoxFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<EditableListBoxFromCpp<FROM_CPP>> for PanelFromCpp<FROM_CPP> {
    fn from(o: EditableListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<EditableListBoxFromCpp<FROM_CPP>> for WindowFromCpp<FROM_CPP> {
    fn from(o: EditableListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<EditableListBoxFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: EditableListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<EditableListBoxFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: EditableListBoxFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for EditableListBoxFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxEditableListBox_CLASSINFO()) }
    }
}

// wxEraseEvent
wxwidgets! {
    /// An erase event is sent when a window's background needs to be repainted.
    /// - [`EraseEvent`] represents a C++ `wxEraseEvent` class instance which your code has ownership, [`EraseEventFromCpp`]`<false>` represents one which don't own.
    /// - Use [`EraseEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxEraseEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_erase_event.html) for more details.
    #[doc(alias = "wxEraseEvent")]
    #[doc(alias = "EraseEvent")]
    class EraseEvent
        = EraseEventFromCpp<true>(wxEraseEvent) impl
        EraseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> EraseEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxEraseEvent::wxEraseEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_erase_event.html#ae6d16ac169480be125ea4e4138fc29ef).
    pub fn new<D: DCMethods>(id: c_int, dc: Option<&D>) -> EraseEventFromCpp<FROM_CPP> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            EraseEventFromCpp(ffi::wxEraseEvent_new(id, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for EraseEventFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<EraseEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: EraseEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<EraseEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: EraseEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for EraseEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxEraseEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for EraseEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEventBlocker
wxwidgets! {
    /// This class is a special event handler which allows discarding any event (or a set of event types) directed to a specific window.
    /// - [`EventBlocker`] represents a C++ `wxEventBlocker` class instance which your code has ownership, [`EventBlockerFromCpp`]`<false>` represents one which don't own.
    /// - Use [`EventBlocker`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxEventBlocker` class's documentation](https://docs.wxwidgets.org/3.2/classwx_event_blocker.html) for more details.
    #[doc(alias = "wxEventBlocker")]
    #[doc(alias = "EventBlocker")]
    class EventBlocker
        = EventBlockerFromCpp<true>(wxEventBlocker) impl
        EventBlockerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> EventBlockerFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxEventBlocker()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const FROM_CPP: bool> Clone for EventBlockerFromCpp<FROM_CPP> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<EventBlockerFromCpp<FROM_CPP>> for EvtHandlerFromCpp<FROM_CPP> {
    fn from(o: EventBlockerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<EventBlockerFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: EventBlockerFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for EventBlockerFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxEventBlocker_CLASSINFO()) }
    }
}
