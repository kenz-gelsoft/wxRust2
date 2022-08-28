use super::*;

// wxClassInfo
wxwidgets! {
    /// This class stores meta-information about classes.
    #[doc(alias = "wxClassInfo")]
    #[doc(alias = "ClassInfo")]
    class ClassInfo
        = ClassInfoIsOwned<true>(wxClassInfo) impl
        ClassInfoMethods
}
impl<const OWNED: bool> ClassInfoIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxClassInfo()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClassInfoIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ClassInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClassInfo_delete(self.0) }
        }
    }
}

// wxClientData
wxwidgets! {
    /// All classes deriving from wxEvtHandler (such as all controls and wxApp) can hold arbitrary data which is here referred to as "client data".
    #[doc(alias = "wxClientData")]
    #[doc(alias = "ClientData")]
    class ClientData
        = ClientDataIsOwned<true>(wxClientData) impl
        ClientDataMethods
}
impl<const OWNED: bool> ClientDataIsOwned<OWNED> {
    /// Constructor.
    pub fn new() -> ClientDataIsOwned<OWNED> {
        unsafe { ClientDataIsOwned(ffi::wxClientData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClientDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ClientDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClientData_delete(self.0) }
        }
    }
}
