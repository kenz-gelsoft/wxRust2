use super::*;

// wxClassInfo
pub trait ClassInfoMethods: WxRustMethods {
    /// Creates an object of the appropriate kind.
    ///
    /// [See `wxClassInfo::CreateObject()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#a99df2fd7d0ec6b0f90790ffaeac872ca)
    fn create_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxClassInfo_CreateObject(self.as_ptr())) }
    }
    /// Returns the name of the first base class (NULL if none).
    ///
    /// [See `wxClassInfo::GetBaseClassName1()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#a18260d1a19bc5213a016dc3b18738ee9)
    fn get_base_class_name1(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetBaseClassName1(self.as_ptr()) }
    }
    /// Returns the name of the second base class (NULL if none).
    ///
    /// [See `wxClassInfo::GetBaseClassName2()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#ab4c6f02564990a01d986cfe770192834)
    fn get_base_class_name2(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetBaseClassName2(self.as_ptr()) }
    }
    /// Returns the string form of the class name.
    ///
    /// [See `wxClassInfo::GetClassName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#a470a32e5eb6f3a804ea7eb4556e3b480)
    fn get_class_name(&self) -> *const c_void {
        unsafe { ffi::wxClassInfo_GetClassName(self.as_ptr()) }
    }
    /// Returns the size of the class.
    ///
    /// [See `wxClassInfo::GetSize()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#aa88c7e3bfebf06787d9d4e3bb457bca6)
    fn get_size(&self) -> c_int {
        unsafe { ffi::wxClassInfo_GetSize(self.as_ptr()) }
    }
    /// Returns true if this class info can create objects of the associated class.
    ///
    /// [See `wxClassInfo::IsDynamic()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#affa753a570e48b53243944974f9014e7)
    fn is_dynamic(&self) -> bool {
        unsafe { ffi::wxClassInfo_IsDynamic(self.as_ptr()) }
    }
    /// Returns true if this class is a kind of (inherits from) the given class.
    ///
    /// [See `wxClassInfo::IsKindOf()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#a13f3f381e570e04b31f574e425c6f446)
    fn is_kind_of<C: ClassInfoMethods>(&self, info: Option<&C>) -> bool {
        unsafe {
            let info = match info {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClassInfo_IsKindOf(self.as_ptr(), info)
        }
    }
    /// Finds the wxClassInfo object for a class with the given name.
    ///
    /// [See `wxClassInfo::FindClass()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_class_info.html#a57553434bc28e74412b58d368702301d)
    fn find_class(class_name: &str) -> Option<ClassInfoIsOwned<false>> {
        unsafe {
            let class_name = WxString::from(class_name);
            let class_name = class_name.as_ptr();
            ClassInfo::option_from(ffi::wxClassInfo_FindClass(class_name))
        }
    }
}

// wxClientData
pub trait ClientDataMethods: WxRustMethods {
    // DTOR: fn ~wxClientData()
}
