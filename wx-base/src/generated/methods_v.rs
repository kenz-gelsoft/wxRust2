use super::*;

// wxVariantData
/// This trait represents [C++ `wxVariantData` class](https://docs.wxwidgets.org/3.2/classwx_variant_data.html)'s methods and inheritance.
///
/// See [`VariantDataFromCpp`] documentation for the class usage.
pub trait VariantDataMethods: ObjectRefDataMethods {
    /// This function can be overridden to clone the data.
    ///
    /// See [C++ `wxVariantData::Clone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#a532ef084409dc34bab886b59d41fc7d3).
    fn clone(&self) -> Option<VariantDataFromCpp<true>> {
        unsafe { VariantData::option_from(ffi::wxVariantData_Clone(self.as_ptr())) }
    }
    /// Decreases reference count.
    ///
    /// See [C++ `wxVariantData::DecRef()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#a5c90358b8b5d3bc99c0c3410cf565010).
    fn dec_ref(&self) {
        unsafe { ffi::wxVariantData_DecRef(self.as_ptr()) }
    }
    /// Returns true if this object is equal to data.
    ///
    /// See [C++ `wxVariantData::Eq()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#a528c7c86af3dd554e5b72d2e5fdead6f).
    fn eq<V: VariantDataMethods>(&self, data: &V) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxVariantData_Eq(self.as_ptr(), data)
        }
    }
    // BLOCKED: fn GetAny()
    /// Returns the string type of the data.
    ///
    /// See [C++ `wxVariantData::GetType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#a56789ab40ed58d058e655576ec1e81bc).
    fn get_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxVariantData_GetType(self.as_ptr())).into() }
    }
    /// If the data is a wxObject returns a pointer to the objects wxClassInfo structure, if the data isn't a wxObject the method returns NULL.
    ///
    /// See [C++ `wxVariantData::GetValueClassInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#ae5c91c7065e3faadacb4ae2ec7cbf772).
    fn get_value_class_info(&self) -> Option<ClassInfoFromCpp<true>> {
        unsafe { ClassInfo::option_from(ffi::wxVariantData_GetValueClassInfo(self.as_ptr())) }
    }
    /// Increases reference count.
    ///
    /// See [C++ `wxVariantData::IncRef()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#ae25c62e3451463ed287367a7b709e51d).
    fn inc_ref(&self) {
        unsafe { ffi::wxVariantData_IncRef(self.as_ptr()) }
    }
    // BLOCKED: fn Read()
    /// Reads the data from string.
    ///
    /// See [C++ `wxVariantData::Read()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#a478c208420802aade488e57de371d27b).
    fn read(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Read1(self.as_ptr(), string) }
    }
    // BLOCKED: fn Write()
    /// Writes the data to string.
    ///
    /// See [C++ `wxVariantData::Write()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html#a9d208924d7b652fb85db4ff6cb0b5d08).
    fn write(&self, string: *mut c_void) -> bool {
        unsafe { ffi::wxVariantData_Write1(self.as_ptr(), string) }
    }
}
