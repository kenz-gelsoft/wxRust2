use super::*;

// wxVariantData
wxwidgets! {
    /// The wxVariantData class is used to implement a new type for wxVariant.
    /// - [`VariantData`] represents a C++ `wxVariantData` class instance which your code has ownership, [`VariantDataIsOwned`]`<false>` represents one which don't own.
    /// - Use [`VariantData`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxVariantData` class's documentation](https://docs.wxwidgets.org/3.2/classwx_variant_data.html) for more details.
    #[doc(alias = "wxVariantData")]
    #[doc(alias = "VariantData")]
    class VariantData
        = VariantDataIsOwned<true>(wxVariantData) impl
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const OWNED: bool> VariantDataIsOwned<OWNED> {
    // BLOCKED: fn wxVariantData()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for VariantDataIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<VariantDataIsOwned<OWNED>> for ObjectRefDataIsOwned<OWNED> {
    fn from(o: VariantDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for VariantDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVariantData_delete(self.0) }
        }
    }
}
