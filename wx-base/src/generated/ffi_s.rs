use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxStandardPaths
    pub fn wxStandardPaths_delete(self_: *mut c_void);
    pub fn wxStandardPaths_DontIgnoreAppSubDir(self_: *mut c_void);
    pub fn wxStandardPaths_GetAppDocumentsDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetConfigDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetDataDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetDocumentsDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetExecutablePath(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetInstallPrefix(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetLocalDataDir(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxStandardPaths_GetLocalizedResourcesDir(self_: *const c_void, lang: *const c_void, category: ResourceCat) -> *mut c_void;
    pub fn wxStandardPaths_GetPluginsDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetResourcesDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetTempDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetUserConfigDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetUserDataDir(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxStandardPaths_GetUserDir(self_: *const c_void, user_dir: Dir) -> *mut c_void;
    pub fn wxStandardPaths_GetUserLocalDataDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_IgnoreAppSubDir(self_: *mut c_void, subdir_pattern: *const c_void);
    pub fn wxStandardPaths_IgnoreAppBuildSubDirs(self_: *mut c_void);
    pub fn wxStandardPaths_SetInstallPrefix(self_: *mut c_void, prefix: *const c_void);
    pub fn wxStandardPaths_UseAppInfo(self_: *mut c_void, info: c_int);
    // NOT_SUPPORTED: pub fn wxStandardPaths_SetFileLayout(self_: *mut c_void, layout: FileLayout);
    // NOT_SUPPORTED: pub fn wxStandardPaths_GetFileLayout(self_: *const c_void) -> FileLayout;
    // NOT_SUPPORTED: pub fn wxStandardPaths_MakeConfigFileName(self_: *const c_void, basename: *const c_void, conv: ConfigFileConv) -> *mut c_void;
    pub fn wxStandardPaths_Get() -> *mut c_void;
    pub fn wxStandardPaths_MSWGetShellDir(csidl: c_int) -> *mut c_void;

}
