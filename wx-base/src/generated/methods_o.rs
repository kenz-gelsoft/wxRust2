use super::*;

// wxObject
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    /// This virtual function is redefined for every class that requires run-time type information, when using the wxDECLARE_CLASS macro (or similar).
    fn get_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxObject_GetClassInfo(self.as_ptr())) }
    }
    /// Returns the wxObject::m_refData pointer, i.e. the data referenced by this object.
    fn get_ref_data(&self) -> Option<ObjectRefDataIsOwned<false>> {
        unsafe { ObjectRefData::option_from(ffi::wxObject_GetRefData(self.as_ptr())) }
    }
    /// Determines whether this class is a subclass of (or the same class as) the given class.
    fn is_kind_of<C: ClassInfoMethods>(&self, info: Option<&C>) -> bool {
        unsafe {
            let info = match info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxObject_IsKindOf(self.as_ptr(), info)
        }
    }
    /// Returns true if this object has the same data pointer as obj.
    fn is_same_as<O: ObjectMethods>(&self, obj: &O) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    /// Makes this object refer to the data in clone.
    fn ref_<O: ObjectMethods>(&self, clone: &O) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    /// Sets the wxObject::m_refData pointer.
    fn set_ref_data<O: ObjectRefDataMethods>(&self, data: Option<&O>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxObject_SetRefData(self.as_ptr(), data)
        }
    }
    /// Decrements the reference count in the associated data, and if it is zero, deletes the data.
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    /// This is the same of AllocExclusive() but this method is public.
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // BLOCKED: fn operator new()
}

// wxObjectRefData
pub trait ObjectRefDataMethods: WxRustMethods {}
