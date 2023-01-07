use super::*;

// wxVariantData
wxwidgets! {
    /// The wxVariantData class is used to implement a new type for wxVariant.
    /// - [`VariantData`] represents a C++ `wxVariantData` class instance which your code has ownership, [`VariantDataFromCpp`]`<true>` represents one which don't own.
    /// - Use [`VariantData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxVariantData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html) for more details.
    #[doc(alias = "wxVariantData")]
    #[doc(alias = "VariantData")]
    class VariantData
        = VariantDataFromCpp<false>(wxVariantData) impl
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const FROM_CPP: bool> VariantDataFromCpp<FROM_CPP> {
    // BLOCKED: fn wxVariantData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for VariantDataFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> From<VariantDataFromCpp<FROM_CPP>> for ObjectRefDataFromCpp<FROM_CPP> {
    fn from(o: VariantDataFromCpp<FROM_CPP>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const FROM_CPP: bool> Drop for VariantDataFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxVariantData_delete(self.0) }
        }
    }
}
