use super::*;

// wxClassInfo
wx_class! {
    #[doc(alias = "wxClassInfo")]
    #[doc(alias = "ClassInfo")]
    type ClassInfo = ClassInfoIsOwned<true>(wxClassInfo) impl
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
wx_class! {
    #[doc(alias = "wxClientData")]
    #[doc(alias = "ClientData")]
    type ClientData = ClientDataIsOwned<true>(wxClientData) impl
        ClientDataMethods
}
impl<const OWNED: bool> ClientDataIsOwned<OWNED> {
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
