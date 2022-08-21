#![allow(non_upper_case_globals)]

use super::*;

// wxVariantData
wx_class! { VariantData =
    VariantDataIsOwned<true>(wxVariantData) impl
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const OWNED: bool> VariantDataIsOwned<OWNED> {
    // BLOCKED: fn wxVariantData()
    pub fn none() -> Option<&'static Self> {
        None
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
