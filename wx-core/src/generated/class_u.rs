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
impl<const OWNED: bool> UIActionSimulatorInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxUIActionSimulator::wxUIActionSimulator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aa0f66cea40a642075482e06ccf2b75cb).
    pub fn new() -> UIActionSimulatorInRust<OWNED> {
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
impl<const OWNED: bool> Drop for UIActionSimulatorInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
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
impl<const OWNED: bool> URLDataObjectInRust<OWNED> {
    /// Constructor, may be used to initialize the URL.
    ///
    /// See [C++ `wxURLDataObject::wxURLDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#a57104fc32eb66f8fa666b1f2799631d5).
    pub fn new(url: &str) -> URLDataObjectInRust<OWNED> {
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
impl<const OWNED: bool> From<URLDataObjectInRust<OWNED>> for DataObjectInRust<OWNED> {
    fn from(o: URLDataObjectInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for URLDataObjectInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
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
impl<const OWNED: bool> UpdateUIEventInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxUpdateUIEvent::wxUpdateUIEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#aa25df58e7047f819f5dd0520eb2cc8ea).
    pub fn new(command_id: c_int) -> UpdateUIEventInRust<OWNED> {
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
impl<const OWNED: bool> From<UpdateUIEventInRust<OWNED>> for CommandEventInRust<OWNED> {
    fn from(o: UpdateUIEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<UpdateUIEventInRust<OWNED>> for EventInRust<OWNED> {
    fn from(o: UpdateUIEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<UpdateUIEventInRust<OWNED>> for ObjectInRust<OWNED> {
    fn from(o: UpdateUIEventInRust<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for UpdateUIEventInRust<OWNED> {
    fn class_info() -> ClassInfoInRust<false> {
        unsafe { ClassInfoInRust::from_ptr(ffi::wxUpdateUIEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for UpdateUIEventInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
