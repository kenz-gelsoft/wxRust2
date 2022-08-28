use super::*;

// wxVariantData
pub trait VariantDataMethods: ObjectRefDataMethods {
    /// This function can be overridden to clone the data.
    fn clone(&self) -> Option<VariantDataIsOwned<false>> {
        unsafe { VariantData::option_from(ffi::wxVariantData_Clone(self.as_ptr())) }
    }
    /// Decreases reference count.
    fn dec_ref(&self) {
        unsafe { ffi::wxVariantData_DecRef(self.as_ptr()) }
    }
    /// Returns true if this object is equal to data.
    fn eq<V: VariantDataMethods>(&self, data: &V) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxVariantData_Eq(self.as_ptr(), data)
        }
    }
    // BLOCKED: fn GetAny()
    /// Returns the string type of the data.
    fn get_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVariantData_GetType(self.as_ptr())).into() }
    }
    /// If the data is a wxObject returns a pointer to the objects wxClassInfo structure, if the data isn't a wxObject the method returns NULL.
    fn get_value_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxVariantData_GetValueClassInfo(self.as_ptr())) }
    }
    /// Increases reference count.
    fn inc_ref(&self) {
        unsafe { ffi::wxVariantData_IncRef(self.as_ptr()) }
    }
    // BLOCKED: fn Read()
    /// Reads the data from string.
    fn read(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Read1(self.as_ptr(), string) }
    }
    // BLOCKED: fn Write()
    /// Writes the data to string.
    fn write(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Write1(self.as_ptr(), string) }
    }
}
