use super::*;

// wxFileName
wxwidgets! {
    /// wxFileName encapsulates a file name.
    /// - [`FileName`] represents a C++ `wxFileName` class instance which your code has ownership, [`FileNameInRust`]`<false>` represents one which don't own.
    /// - Use [`FileName`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxFileName` class's documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html) for more details.
    #[doc(alias = "wxFileName")]
    #[doc(alias = "FileName")]
    class FileName
        = FileNameInRust<true>(wxFileName) impl
        FileNameMethods
}
impl<const OWNED: bool> FileNameInRust<OWNED> {
    /// Default constructor.
    ///
    /// See [C++ `wxFileName::wxFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a2e54bc95c94eb773841c7cb1d7732e35).
    pub fn new() -> FileNameInRust<OWNED> {
        unsafe { FileNameInRust(ffi::wxFileName_new()) }
    }
    /// Copy constructor.
    ///
    /// See [C++ `wxFileName::wxFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#abc232361df39f16a802da8391692e2f5).
    pub fn new_with_filename<F: FileNameMethods>(filename: &F) -> FileNameInRust<OWNED> {
        unsafe {
            let filename = filename.as_ptr();
            FileNameInRust(ffi::wxFileName_new1(filename))
        }
    }
    // NOT_SUPPORTED: fn wxFileName2()
    // NOT_SUPPORTED: fn wxFileName3()
    // NOT_SUPPORTED: fn wxFileName4()
    // NOT_SUPPORTED: fn wxFileName5()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for FileNameInRust<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for FileNameInRust<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileName_delete(self.0) }
        }
    }
}
