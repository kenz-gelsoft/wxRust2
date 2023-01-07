use super::*;

// wxStandardPaths
wxwidgets! {
    /// wxStandardPaths returns the standard locations in the file system and should be used by applications to find their data files in a portable way.
    /// - [`StandardPaths`] represents a C++ `wxStandardPaths` class instance which your code has ownership, [`StandardPathsFromCpp`]`<false>` represents one which don't own.
    /// - Use [`StandardPaths`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxStandardPaths` class's documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html) for more details.
    #[doc(alias = "wxStandardPaths")]
    #[doc(alias = "StandardPaths")]
    class StandardPaths
        = StandardPathsFromCpp<true>(wxStandardPaths) impl
        StandardPathsMethods
}
impl<const FROM_CPP: bool> StandardPathsFromCpp<FROM_CPP> {
    //  ENUM: ResourceCat
    pub const ResourceCat_None: c_int = 0;
    pub const ResourceCat_Messages: c_int = 0 + 1;

    //  ENUM: Dir
    pub const Dir_Cache: c_int = 0;
    pub const Dir_Documents: c_int = 0 + 1;
    pub const Dir_Desktop: c_int = 0 + 2;
    pub const Dir_Downloads: c_int = 0 + 3;
    pub const Dir_Music: c_int = 0 + 4;
    pub const Dir_Pictures: c_int = 0 + 5;
    pub const Dir_Videos: c_int = 0 + 6;

    //  ENUM: FileLayout
    pub const FileLayout_Classic: c_int = 0;
    pub const FileLayout_XDG: c_int = 0 + 1;

    //  ENUM: ConfigFileConv
    pub const ConfigFileConv_Dot: c_int = 0;
    pub const ConfigFileConv_Ext: c_int = 0 + 1;

    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for StandardPathsFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for StandardPathsFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxStandardPaths_delete(self.0) }
        }
    }
}
