use super::*;

// wxUIActionSimulator
wxwidgets! {
    /// wxUIActionSimulator is a class used to simulate user interface actions such as a mouse click or a key press.
    /// - [`UIActionSimulator`] represents a C++ `wxUIActionSimulator` class instance which your code has ownership, [`UIActionSimulatorIsOwned`]`<false>` represents one which don't own.
    /// - Use [`UIActionSimulator`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxUIActionSimulator` class's documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html) for more details.
    #[doc(alias = "wxUIActionSimulator")]
    #[doc(alias = "UIActionSimulator")]
    class UIActionSimulator
        = UIActionSimulatorIsOwned<true>(wxUIActionSimulator) impl
        UIActionSimulatorMethods
}
impl<const OWNED: bool> UIActionSimulatorIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxUIActionSimulator::wxUIActionSimulator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_i_action_simulator.html#aa0f66cea40a642075482e06ccf2b75cb).
    pub fn new() -> UIActionSimulatorIsOwned<OWNED> {
        unsafe { UIActionSimulatorIsOwned(ffi::wxUIActionSimulator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for UIActionSimulatorIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for UIActionSimulatorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxUIActionSimulator_delete(self.0) }
        }
    }
}

// wxURLDataObject
wxwidgets! {
    /// wxURLDataObject is a wxDataObject containing an URL and can be used e.g.
    /// - [`URLDataObject`] represents a C++ `wxURLDataObject` class instance which your code has ownership, [`URLDataObjectIsOwned`]`<false>` represents one which don't own.
    /// - Use [`URLDataObject`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxURLDataObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html) for more details.
    #[doc(alias = "wxURLDataObject")]
    #[doc(alias = "URLDataObject")]
    class URLDataObject
        = URLDataObjectIsOwned<true>(wxURLDataObject) impl
        URLDataObjectMethods,
        DataObjectMethods
}
impl<const OWNED: bool> URLDataObjectIsOwned<OWNED> {
    /// Constructor, may be used to initialize the URL.
    ///
    /// See [C++ `wxURLDataObject::wxURLDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_u_r_l_data_object.html#a57104fc32eb66f8fa666b1f2799631d5).
    pub fn new(url: &str) -> URLDataObjectIsOwned<OWNED> {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            URLDataObjectIsOwned(ffi::wxURLDataObject_new(url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for URLDataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<URLDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: URLDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for URLDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxURLDataObject_delete(self.0) }
        }
    }
}

// wxUpdateUIEvent
wxwidgets! {
    /// This class is used for pseudo-events which are called by wxWidgets to give an application the chance to update various user interface elements.
    /// - [`UpdateUIEvent`] represents a C++ `wxUpdateUIEvent` class instance which your code has ownership, [`UpdateUIEventIsOwned`]`<false>` represents one which don't own.
    /// - Use [`UpdateUIEvent`]'s `new()` or [`Buildable::builder()`] (if available) to create an instance of this class.
    /// - See [C++ `wxUpdateUIEvent` class's documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html) for more details.
    #[doc(alias = "wxUpdateUIEvent")]
    #[doc(alias = "UpdateUIEvent")]
    class UpdateUIEvent
        = UpdateUIEventIsOwned<true>(wxUpdateUIEvent) impl
        UpdateUIEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> UpdateUIEventIsOwned<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxUpdateUIEvent::wxUpdateUIEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_update_u_i_event.html#aa25df58e7047f819f5dd0520eb2cc8ea).
    pub fn new(command_id: c_int) -> UpdateUIEventIsOwned<OWNED> {
        unsafe { UpdateUIEventIsOwned(ffi::wxUpdateUIEvent_new(command_id)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for UpdateUIEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<UpdateUIEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: UpdateUIEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<UpdateUIEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: UpdateUIEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<UpdateUIEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: UpdateUIEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for UpdateUIEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxUpdateUIEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for UpdateUIEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
