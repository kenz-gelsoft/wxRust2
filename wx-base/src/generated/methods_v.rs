use super::*;

// wxVariantData
pub trait VariantDataMethods: ObjectRefDataMethods {
    fn clone(&self) -> Option<VariantDataIsOwned<false>> {
        unsafe { VariantData::option_from(ffi::wxVariantData_Clone(self.as_ptr())) }
    }
    fn dec_ref(&self) {
        unsafe { ffi::wxVariantData_DecRef(self.as_ptr()) }
    }
    fn eq<V: VariantDataMethods>(&self, data: &V) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxVariantData_Eq(self.as_ptr(), data)
        }
    }
    // BLOCKED: fn GetAny()
    fn get_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVariantData_GetType(self.as_ptr())).into() }
    }
    fn get_value_class_info(&self) -> Option<ClassInfoIsOwned<false>> {
        unsafe { ClassInfo::option_from(ffi::wxVariantData_GetValueClassInfo(self.as_ptr())) }
    }
    fn inc_ref(&self) {
        unsafe { ffi::wxVariantData_IncRef(self.as_ptr()) }
    }
    // BLOCKED: fn Read()
    fn read(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Read1(self.as_ptr(), string) }
    }
    // BLOCKED: fn Write()
    fn write(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Write1(self.as_ptr(), string) }
    }
}
