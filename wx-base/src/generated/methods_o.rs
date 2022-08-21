use super::*;

// wxObject
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxObject_GetClassInfo(self.as_ptr())) }
    }
    fn get_ref_data(&self) -> Option<ObjectRefDataIsOwned<false>> {
        unsafe { ObjectRefData::option_from(ffi::wxObject_GetRefData(self.as_ptr())) }
    }
    fn is_kind_of<C: ClassInfoMethods>(&self, info: Option<&C>) -> bool {
        unsafe {
            let info = match info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxObject_IsKindOf(self.as_ptr(), info)
        }
    }
    fn is_same_as<O: ObjectMethods>(&self, obj: &O) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    fn ref_<O: ObjectMethods>(&self, clone: &O) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    fn set_ref_data<O: ObjectRefDataMethods>(&self, data: Option<&O>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxObject_SetRefData(self.as_ptr(), data)
        }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // BLOCKED: fn operator new()
}

// wxObjectRefData
pub trait ObjectRefDataMethods: WxRustMethods {}
