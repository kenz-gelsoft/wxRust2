use super::*;

// wxObject
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    /// This virtual function is redefined for every class that requires run-time type information, when using the wxDECLARE_CLASS macro (or similar).
    ///
    /// [See `wxObject::GetClassInfo()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#a9fd1bc8bc3a47c6e14e679a80e3cb8f4)
    fn get_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxObject_GetClassInfo(self.as_ptr())) }
    }
    /// Returns the wxObject::m_refData pointer, i.e. the data referenced by this object.
    ///
    /// [See `wxObject::GetRefData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#a0e06d77b52ee4c44a31c7cb62c9a4b68)
    fn get_ref_data(&self) -> Option<ObjectRefDataIsOwned<false>> {
        unsafe { ObjectRefData::option_from(ffi::wxObject_GetRefData(self.as_ptr())) }
    }
    /// Determines whether this class is a subclass of (or the same class as) the given class.
    ///
    /// [See `wxObject::IsKindOf()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#a3c7115ef4132dcee0c4fc34e84d7fced)
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
    ///
    /// [See `wxObject::IsSameAs()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#a059373c494c2369d7db2a556efef8ecf)
    fn is_same_as<O: ObjectMethods>(&self, obj: &O) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    /// Makes this object refer to the data in clone.
    ///
    /// [See `wxObject::Ref()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#a2f6f1aa51fe9fc2b1415ca4211a90e9e)
    fn ref_<O: ObjectMethods>(&self, clone: &O) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    /// Sets the wxObject::m_refData pointer.
    ///
    /// [See `wxObject::SetRefData()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#afab780710f2adc1bb33310e27590140b)
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
    ///
    /// [See `wxObject::UnRef()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#af51efc6b1ae632fc7f0cd7ebbce9fa36)
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    /// This is the same of AllocExclusive() but this method is public.
    ///
    /// [See `wxObject::UnShare()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_object.html#a74b40e42d19a4b9e9bec0b57d62a5725)
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // BLOCKED: fn operator new()
}

// wxObjectRefData
pub trait ObjectRefDataMethods: WxRustMethods {}
