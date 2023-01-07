use super::*;

// wxUIActionSimulator
wxwidgets! {
    /// wxUIActionSimulator is a class used to simulate user interface actions such as a mouse click or a key press.
    /// - [`UIActionSimulator`] represents a C++ `wxUIActionSimulator` class instance which your code has ownership, [`UIActionSimulatorFromCpp`]`<true>` represents one which don't own.
    /// - Use [`UIActionSimulator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxUIActionSimulator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html) for more details.
    #[doc(alias = "wxUIActionSimulator")]
    #[doc(alias = "UIActionSimulator")]
    class UIActionSimulator
        = UIActionSimulatorFromCpp<false>(wxUIActionSimulator) impl
        UIActionSimulatorMethods
}
impl<const FROM_CPP: bool> UIActionSimulatorFromCpp<FROM_CPP> {
    /// Default constructor.
    ///
    /// See [C++ `wxUIActionSimulator::wxUIActionSimulator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aa0f66cea40a642075482e06ccf2b75cb).
    pub fn new() -> UIActionSimulatorFromCpp<FROM_CPP> {
        unsafe { UIActionSimulatorFromCpp(ffi::wxUIActionSimulator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for UIActionSimulatorFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for UIActionSimulatorFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxUIActionSimulator_delete(self.0) }
        }
    }
}

// wxURLDataObject
wxwidgets! {
    /// wxURLDataObject is a wxDataObject containing an URL and can be used e.g.
    /// - [`URLDataObject`] represents a C++ `wxURLDataObject` class instance which your code has ownership, [`URLDataObjectFromCpp`]`<true>` represents one which don't own.
    /// - Use [`URLDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxURLDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html) for more details.
    #[doc(alias = "wxURLDataObject")]
    #[doc(alias = "URLDataObject")]
    class URLDataObject
        = URLDataObjectFromCpp<false>(wxURLDataObject) impl
        URLDataObjectMethods,
        DataObjectMethods
}
impl<const FROM_CPP: bool> URLDataObjectFromCpp<FROM_CPP> {
    /// Constructor, may be used to initialize the URL.
    ///
    /// See [C++ `wxURLDataObject::wxURLDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#a57104fc32eb66f8fa666b1f2799631d5).
    pub fn new(url: &str) -> URLDataObjectFromCpp<FROM_CPP> {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            URLDataObjectFromCpp(ffi::wxURLDataObject_new(url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for URLDataObjectFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<URLDataObjectFromCpp<FROM_CPP>> for DataObjectFromCpp<FROM_CPP> {
    fn from(o: URLDataObjectFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for URLDataObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxURLDataObject_delete(self.0) }
        }
    }
}

// wxUpdateUIEvent
wxwidgets! {
    /// This class is used for pseudo-events which are called by wxWidgets to give an application the chance to update various user interface elements.
    /// - [`UpdateUIEvent`] represents a C++ `wxUpdateUIEvent` class instance which your code has ownership, [`UpdateUIEventFromCpp`]`<true>` represents one which don't own.
    /// - Use [`UpdateUIEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxUpdateUIEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html) for more details.
    #[doc(alias = "wxUpdateUIEvent")]
    #[doc(alias = "UpdateUIEvent")]
    class UpdateUIEvent
        = UpdateUIEventFromCpp<false>(wxUpdateUIEvent) impl
        UpdateUIEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const FROM_CPP: bool> UpdateUIEventFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxUpdateUIEvent::wxUpdateUIEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#aa25df58e7047f819f5dd0520eb2cc8ea).
    pub fn new(command_id: c_int) -> UpdateUIEventFromCpp<FROM_CPP> {
        unsafe { UpdateUIEventFromCpp(ffi::wxUpdateUIEvent_new(command_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for UpdateUIEventFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<UpdateUIEventFromCpp<FROM_CPP>> for CommandEventFromCpp<FROM_CPP> {
    fn from(o: UpdateUIEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<UpdateUIEventFromCpp<FROM_CPP>> for EventFromCpp<FROM_CPP> {
    fn from(o: UpdateUIEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> From<UpdateUIEventFromCpp<FROM_CPP>> for ObjectFromCpp<FROM_CPP> {
    fn from(o: UpdateUIEventFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> DynamicCast for UpdateUIEventFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<true> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxUpdateUIEvent_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for UpdateUIEventFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
