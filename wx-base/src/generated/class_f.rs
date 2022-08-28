use super::*;

// wxFileName
wxwidgets! {
    /// wxFileName encapsulates a file name.
    ///
    /// [See `wxFileName`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_name.html)
    #[doc(alias = "wxFileName")]
    #[doc(alias = "FileName")]
    class FileName
        = FileNameIsOwned<true>(wxFileName) impl
        FileNameMethods
}
impl<const OWNED: bool> FileNameIsOwned<OWNED> {
    /// Default constructor.
    ///
    /// [See `wxFileName::wxFileName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a2e54bc95c94eb773841c7cb1d7732e35)
    pub fn new() -> FileNameIsOwned<OWNED> {
        unsafe { FileNameIsOwned(ffi::wxFileName_new()) }
    }
    /// Copy constructor.
    ///
    /// [See `wxFileName::wxFileName()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_file_name.html#abc232361df39f16a802da8391692e2f5)
    pub fn new_with_filename<F: FileNameMethods>(filename: &F) -> FileNameIsOwned<OWNED> {
        unsafe {
            let filename = filename.as_ptr();
            FileNameIsOwned(ffi::wxFileName_new1(filename))
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
impl Clone for FileNameIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for FileNameIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileName_delete(self.0) }
        }
    }
}
