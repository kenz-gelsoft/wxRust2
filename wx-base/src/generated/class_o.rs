use super::*;

// wxObject
wxwidgets! {
    /// This is the root class of many of the wxWidgets classes.
    /// - [`Object`] represents a C++ `wxObject` class instance which your code has ownership, [`ObjectIsOwned`]`<false>` represents one which don't own.
    /// - Use [`Object`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxObject` class's documentation](https://docs.wxwidgets.org/3.2/classwx_object.html) for more details.
    #[doc(alias = "wxObject")]
    #[doc(alias = "Object")]
    class Object
        = ObjectIsOwned<true>(wxObject) impl
        ObjectMethods
}
impl<const OWNED: bool> ObjectIsOwned<OWNED> {
    /// Default ctor; initializes to NULL the internal reference data.
    ///
    /// See [C++ `wxObject::wxObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_object.html#acaa378363a28af421ab56ad7b46eadf0).
    pub fn new() -> ObjectIsOwned<OWNED> {
        unsafe { ObjectIsOwned(ffi::wxObject_new()) }
    }
    /// Copy ctor.
    ///
    /// See [C++ `wxObject::wxObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_object.html#a4721b4dc9b7aff0f30904ba2ea3954cf).
    pub fn new_with_object<O: ObjectMethods>(other: &O) -> ObjectIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            ObjectIsOwned(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> DynamicCast for ObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxObjectRefData
wxwidgets! {
    /// This class is just a typedef to wxRefCounter and is used by wxObject.
    /// - [`ObjectRefData`] represents a C++ `wxObjectRefData` class instance which your code has ownership, [`ObjectRefDataIsOwned`]`<false>` represents one which don't own.
    /// - Use [`ObjectRefData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxObjectRefData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_object_ref_data.html) for more details.
    #[doc(alias = "wxObjectRefData")]
    #[doc(alias = "ObjectRefData")]
    class ObjectRefData
        = ObjectRefDataIsOwned<true>(wxObjectRefData) impl
        ObjectRefDataMethods
}
impl<const OWNED: bool> ObjectRefDataIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for ObjectRefDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for ObjectRefDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObjectRefData_delete(self.0) }
        }
    }
}
