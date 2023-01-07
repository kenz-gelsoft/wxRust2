use super::*;

// wxClassInfo
wxwidgets! {
    /// This class stores meta-information about classes.
    /// - [`ClassInfo`] represents a C++ `wxClassInfo` class instance which your code has ownership, [`ClassInfoFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ClassInfo`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxClassInfo` class's documentation](https://docs.wxwidgets.org/3.2/classwx_class_info.html) for more details.
    #[doc(alias = "wxClassInfo")]
    #[doc(alias = "ClassInfo")]
    class ClassInfo
        = ClassInfoFromCpp<true>(wxClassInfo) impl
        ClassInfoMethods
}
impl<const FROM_CPP: bool> ClassInfoFromCpp<FROM_CPP> {
    // NOT_SUPPORTED: fn wxClassInfo()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClassInfoFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ClassInfoFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxClassInfo_delete(self.0) }
        }
    }
}

// wxClientData
wxwidgets! {
    /// All classes deriving from wxEvtHandler (such as all controls and wxApp) can hold arbitrary data which is here referred to as "client data".
    /// - [`ClientData`] represents a C++ `wxClientData` class instance which your code has ownership, [`ClientDataFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ClientData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxClientData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_client_data.html) for more details.
    #[doc(alias = "wxClientData")]
    #[doc(alias = "ClientData")]
    class ClientData
        = ClientDataFromCpp<true>(wxClientData) impl
        ClientDataMethods
}
impl<const FROM_CPP: bool> ClientDataFromCpp<FROM_CPP> {
    /// Constructor.
    ///
    /// See [C++ `wxClientData::wxClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_client_data.html#acf0e79134d5fb8abd8a4a343c616e8d7).
    pub fn new() -> ClientDataFromCpp<FROM_CPP> {
        unsafe { ClientDataFromCpp(ffi::wxClientData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ClientDataFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ClientDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxClientData_delete(self.0) }
        }
    }
}
