use super::*;

// wxVariantData
wxwidgets! {
    /// The wxVariantData class is used to implement a new type for wxVariant.
    /// - [`VariantData`] represents a C++ `wxVariantData` class instance which your code has ownership, [`VariantDataInRust`]`<false>` represents one which don't own.
    /// - Use [`VariantData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxVariantData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html) for more details.
    #[doc(alias = "wxVariantData")]
    #[doc(alias = "VariantData")]
    class VariantData
        = VariantDataInRust<true>(wxVariantData) impl
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const IN_RUST: bool> VariantDataInRust<IN_RUST> {
    // BLOCKED: fn wxVariantData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for VariantDataInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const IN_RUST: bool> From<VariantDataInRust<IN_RUST>> for ObjectRefDataInRust<IN_RUST> {
    fn from(o: VariantDataInRust<IN_RUST>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const IN_RUST: bool> Drop for VariantDataInRust<IN_RUST> {
    fn drop(&mut self) {
        if IN_RUST {
            unsafe { ffi::wxVariantData_delete(self.0) }
        }
    }
}
