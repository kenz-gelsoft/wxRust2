use super::*;

// wxUIActionSimulator
wxwidgets! {
    /// wxUIActionSimulator is a class used to simulate user interface actions such as a mouse click or a key press.
    /// - [`UIActionSimulator`] represents a C++ `wxUIActionSimulator` class instance which your code has ownership, [`UIActionSimulatorInRust`]`<false>` represents one which don't own.
    /// - Use [`UIActionSimulator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxUIActionSimulator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html) for more details.
    #[doc(alias = "wxUIActionSimulator")]
    #[doc(alias = "UIActionSimulator")]
    class UIActionSimulator
        = UIActionSimulatorInRust<true>(wxUIActionSimulator) impl
        UIActionSimulatorMethods
}
impl<const IN_RUST: bool> UIActionSimulatorInRust<IN_RUST> {
    /// Default constructor.
    ///
    /// See [C++ `wxUIActionSimulator::wxUIActionSimulator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aa0f66cea40a642075482e06ccf2b75cb).
    pub fn new() -> UIActionSimulatorInRust<IN_RUST> {
        unsafe { UIActionSimulatorInRust(ffi::wxUIActionSimulator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for UIActionSimulatorInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> Drop for UIActionSimulatorInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxUIActionSimulator_delete(self.0) }
        }
    }
}

// wxURLDataObject
wxwidgets! {
    /// wxURLDataObject is a wxDataObject containing an URL and can be used e.g.
    /// - [`URLDataObject`] represents a C++ `wxURLDataObject` class instance which your code has ownership, [`URLDataObjectInRust`]`<false>` represents one which don't own.
    /// - Use [`URLDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxURLDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html) for more details.
    #[doc(alias = "wxURLDataObject")]
    #[doc(alias = "URLDataObject")]
    class URLDataObject
        = URLDataObjectInRust<true>(wxURLDataObject) impl
        URLDataObjectMethods,
        DataObjectMethods
}
impl<const IN_RUST: bool> URLDataObjectInRust<IN_RUST> {
    /// Constructor, may be used to initialize the URL.
    ///
    /// See [C++ `wxURLDataObject::wxURLDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#a57104fc32eb66f8fa666b1f2799631d5).
    pub fn new(url: &str) -> URLDataObjectInRust<IN_RUST> {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            URLDataObjectInRust(ffi::wxURLDataObject_new(url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for URLDataObjectInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<URLDataObjectInRust<IN_RUST>> for DataObjectInRust<IN_RUST> {
    fn from(o: URLDataObjectInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for URLDataObjectInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxURLDataObject_delete(self.0) }
        }
    }
}

// wxUpdateUIEvent
wxwidgets! {
    /// This class is used for pseudo-events which are called by wxWidgets to give an application the chance to update various user interface elements.
    /// - [`UpdateUIEvent`] represents a C++ `wxUpdateUIEvent` class instance which your code has ownership, [`UpdateUIEventInRust`]`<false>` represents one which don't own.
    /// - Use [`UpdateUIEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxUpdateUIEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html) for more details.
    #[doc(alias = "wxUpdateUIEvent")]
    #[doc(alias = "UpdateUIEvent")]
    class UpdateUIEvent
        = UpdateUIEventInRust<true>(wxUpdateUIEvent) impl
        UpdateUIEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const IN_RUST: bool> UpdateUIEventInRust<IN_RUST> {
    /// Constructor.
    ///
    /// See [C++ `wxUpdateUIEvent::wxUpdateUIEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#aa25df58e7047f819f5dd0520eb2cc8ea).
    pub fn new(command_id: c_int) -> UpdateUIEventInRust<IN_RUST> {
        unsafe { UpdateUIEventInRust(ffi::wxUpdateUIEvent_new(command_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for UpdateUIEventInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<UpdateUIEventInRust<IN_RUST>> for CommandEventInRust<IN_RUST> {
    fn from(o: UpdateUIEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<UpdateUIEventInRust<IN_RUST>> for EventInRust<IN_RUST> {
    fn from(o: UpdateUIEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> From<UpdateUIEventInRust<IN_RUST>> for ObjectInRust<IN_RUST> {
    fn from(o: UpdateUIEventInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> DynamicCast for UpdateUIEventInRust<IN_RUST> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxUpdateUIEvent_CLASSINFO()) }
    }
}
impl<const IN_RUST: bool> Drop for UpdateUIEventInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
