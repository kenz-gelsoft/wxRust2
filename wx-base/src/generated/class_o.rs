use super::*;

// wxObject
wxwidgets! {
    /// This is the root class of many of the wxWidgets classes.
    /// - [`Object`] represents a C++ `wxObject` class instance which your code has ownership, [`ObjectFromCpp`]`<false>` represents one which don't own.
    /// - Use [`Object`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_object.html) for more details.
    #[doc(alias = "wxObject")]
    #[doc(alias = "Object")]
    class Object
        = ObjectFromCpp<true>(wxObject) impl
        ObjectMethods
}
impl<const FROM_CPP: bool> ObjectFromCpp<FROM_CPP> {
    /// Default ctor; initializes to NULL the internal reference data.
    ///
    /// See [C++ `wxObject::wxObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_object.html#acaa378363a28af421ab56ad7b46eadf0).
    pub fn new() -> ObjectFromCpp<FROM_CPP> {
        unsafe { ObjectFromCpp(ffi::wxObject_new()) }
    }
    /// Copy ctor.
    ///
    /// See [C++ `wxObject::wxObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_object.html#a4721b4dc9b7aff0f30904ba2ea3954cf).
    pub fn new_with_object<O: ObjectMethods>(other: &O) -> ObjectFromCpp<FROM_CPP> {
        unsafe {
            let other = other.as_ptr();
            ObjectFromCpp(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ObjectFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> DynamicCast for ObjectFromCpp<FROM_CPP> {
    fn class_info() -> ClassInfoFromCpp<false> {
        unsafe { ClassInfoFromCpp::from_ptr(ffi::wxObject_CLASSINFO()) }
    }
}
impl<const FROM_CPP: bool> Drop for ObjectFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxObjectRefData
wxwidgets! {
    /// This class is just a typedef to wxRefCounter and is used by wxObject.
    /// - [`ObjectRefData`] represents a C++ `wxObjectRefData` class instance which your code has ownership, [`ObjectRefDataFromCpp`]`<false>` represents one which don't own.
    /// - Use [`ObjectRefData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxObjectRefData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_object_ref_data.html) for more details.
    #[doc(alias = "wxObjectRefData")]
    #[doc(alias = "ObjectRefData")]
    class ObjectRefData
        = ObjectRefDataFromCpp<true>(wxObjectRefData) impl
        ObjectRefDataMethods
}
impl<const FROM_CPP: bool> ObjectRefDataFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ObjectRefDataFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for ObjectRefDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxObjectRefData_delete(self.0) }
        }
    }
}
