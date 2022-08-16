#![allow(non_upper_case_globals)]

use super::*;

// wxSharedClientDataContainer
wx_class! { SharedClientDataContainer =
    SharedClientDataContainerIsOwned<true>(wxSharedClientDataContainer) impl
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> SharedClientDataContainerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SharedClientDataContainerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSharedClientDataContainer_delete(self.0) }
        }
    }
}

// wxStandardPaths
wx_class! { StandardPaths =
    StandardPathsIsOwned<true>(wxStandardPaths) impl
        StandardPathsMethods
}
impl<const OWNED: bool> StandardPathsIsOwned<OWNED> {
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
impl<const OWNED: bool> Drop for StandardPathsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStandardPaths_delete(self.0) }
        }
    }
}
