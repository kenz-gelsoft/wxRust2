use super::*;

// wxEditableListBox
wxwidgets! {
    /// An editable listbox is composite control that lets the user easily enter, delete and reorder a list of strings.
    /// - [`EditableListBox`] represents a C++ `wxEditableListBox` class instance which your code has ownership, [`EditableListBoxInRust`]`<false>` represents one which don't own.
    /// - Use [`EditableListBox`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxEditableListBox` class's documentation](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html) for more details.
    #[doc(alias = "wxEditableListBox")]
    #[doc(alias = "EditableListBox")]
    class EditableListBox
        = EditableListBoxInRust<true>(wxEditableListBox) impl
        EditableListBoxMethods,
        PanelMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> EditableListBoxInRust<IN_RUST> {
    /// Default ctor.
    ///
    /// See [C++ `wxEditableListBox::wxEditableListBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_editable_list_box.html#ab42098c0b81e1f027e307fad8a9ecc8a).
    pub fn new_2step() -> EditableListBoxInRust<IN_RUST> {
        unsafe { EditableListBoxInRust(ffi::wxEditableListBox_new()) }
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
    ) -> EditableListBoxInRust<IN_RUST> {
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
            EditableListBoxInRust(ffi::wxEditableListBox_new1(
                parent, id, label, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for EditableListBoxInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<EditableListBoxInRust<IN_RUST>> for PanelInRust<IN_RUST> {
    fn from(o: EditableListBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<EditableListBoxInRust<IN_RUST>> for WindowInRust<IN_RUST> {
    fn from(o: EditableListBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<EditableListBoxInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: EditableListBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<EditableListBoxInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: EditableListBoxInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for EditableListBoxInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxEditableListBox_CLASSINFO()) }
    }
}

// wxEraseEvent
wxwidgets! {
    /// An erase event is sent when a window's background needs to be repainted.
    /// - [`EraseEvent`] represents a C++ `wxEraseEvent` class instance which your code has ownership, [`EraseEventInRust`]`<false>` represents one which don't own.
    /// - Use [`EraseEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxEraseEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_erase_event.html) for more details.
    #[doc(alias = "wxEraseEvent")]
    #[doc(alias = "EraseEvent")]
    class EraseEvent
        = EraseEventInRust<true>(wxEraseEvent) impl
        EraseEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> EraseEventInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxEraseEvent::wxEraseEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_erase_event.html#ae6d16ac169480be125ea4e4138fc29ef).
    pub fn new<D: DCMethods>(id: c_int, dc: Option<&D>) -> EraseEventInRust<IN_RUST> {
        unsafe {
            let dc = match dc {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            EraseEventInRust(ffi::wxEraseEvent_new(id, dc))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for EraseEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<EraseEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: EraseEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<EraseEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: EraseEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for EraseEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxEraseEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for EraseEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEventBlocker
wxwidgets! {
    /// This class is a special event handler which allows discarding any event (or a set of event types) directed to a specific window.
    /// - [`EventBlocker`] represents a C++ `wxEventBlocker` class instance which your code has ownership, [`EventBlockerInRust`]`<false>` represents one which don't own.
    /// - Use [`EventBlocker`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxEventBlocker` class's documentation](https://docs.wxwidgets.org/3.2/classwx_event_blocker.html) for more details.
    #[doc(alias = "wxEventBlocker")]
    #[doc(alias = "EventBlocker")]
    class EventBlocker
        = EventBlockerInRust<true>(wxEventBlocker) impl
        EventBlockerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> EventBlockerInRust<IN_RUST> {
    // NOT_SUPPORTED: fn wxEventBlocker()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const IN_RUST: bool> Clone for EventBlockerInRust<IN_RUST> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<EventBlockerInRust<IN_RUST>> for EvtHandlerInRust<IN_RUST> {
    fn from(o: EventBlockerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<EventBlockerInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: EventBlockerInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for EventBlockerInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxEventBlocker_CLASSINFO()) }
    }
}
