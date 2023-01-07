use super::*;

// wxClassInfo
wxwidgets! {
    /// This class stores meta-information about classes.
    /// - [`ClassInfo`] represents a C++ `wxClassInfo` class instance which your code has ownership, [`ClassInfoInRust`]`<false>` represents one which don't own.
    /// - Use [`ClassInfo`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxClassInfo` class's documentation](https://docs.wxwidgets.org/3.2/classwx_class_info.html) for more details.
    #[doc(alias = "wxClassInfo")]
    #[doc(alias = "ClassInfo")]
    class ClassInfo
        = ClassInfoInRust<true>(wxClassInfo) impl
        ClassInfoMethods
}
impl<const OWNED: bool> ClassInfoInRust<OWNED> {
    // NOT_SUPPORTED: fn wxClassInfo()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClassInfoInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ClassInfoInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClassInfo_delete(self.0) }
        }
    }
}

// wxClientData
wxwidgets! {
    /// All classes deriving from wxEvtHandler (such as all controls and wxApp) can hold arbitrary data which is here referred to as "client data".
    /// - [`ClientData`] represents a C++ `wxClientData` class instance which your code has ownership, [`ClientDataInRust`]`<false>` represents one which don't own.
    /// - Use [`ClientData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxClientData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_client_data.html) for more details.
    #[doc(alias = "wxClientData")]
    #[doc(alias = "ClientData")]
    class ClientData
        = ClientDataInRust<true>(wxClientData) impl
        ClientDataMethods
}
impl<const OWNED: bool> ClientDataInRust<OWNED> {
    /// Constructor.
    ///
    /// See [C++ `wxClientData::wxClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_client_data.html#acf0e79134d5fb8abd8a4a343c616e8d7).
    pub fn new() -> ClientDataInRust<OWNED> {
        unsafe { ClientDataInRust(ffi::wxClientData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClientDataInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ClientDataInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClientData_delete(self.0) }
        }
    }
}
