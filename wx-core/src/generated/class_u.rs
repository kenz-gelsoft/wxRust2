use super::*;

// wxUIActionSimulator
wx_class! {
    #[doc(alias = "wxUIActionSimulator")]
    #[doc(alias = "UIActionSimulator")]
    type UIActionSimulator = UIActionSimulatorIsOwned<true>(wxUIActionSimulator) impl
        UIActionSimulatorMethods
}
impl<const OWNED: bool> UIActionSimulatorIsOwned<OWNED> {
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
wx_class! {
    #[doc(alias = "wxURLDataObject")]
    #[doc(alias = "URLDataObject")]
    type URLDataObject = URLDataObjectIsOwned<true>(wxURLDataObject) impl
        URLDataObjectMethods,
        DataObjectMethods
}
impl<const OWNED: bool> URLDataObjectIsOwned<OWNED> {
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
wx_class! {
    #[doc(alias = "wxUpdateUIEvent")]
    #[doc(alias = "UpdateUIEvent")]
    type UpdateUIEvent = UpdateUIEventIsOwned<true>(wxUpdateUIEvent) impl
        UpdateUIEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> UpdateUIEventIsOwned<OWNED> {
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
