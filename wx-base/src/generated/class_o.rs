use super::*;

// wxObject
wxwidgets! {
    /// This is the root class of many of the wxWidgets classes.
    #[doc(alias = "wxObject")]
    #[doc(alias = "Object")]
    class Object
        = ObjectIsOwned<true>(wxObject) impl
        ObjectMethods
}
impl<const OWNED: bool> ObjectIsOwned<OWNED> {
    /// Default ctor; initializes to NULL the internal reference data.
    pub fn new() -> ObjectIsOwned<OWNED> {
        unsafe { ObjectIsOwned(ffi::wxObject_new()) }
    }
    /// Copy ctor.
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
