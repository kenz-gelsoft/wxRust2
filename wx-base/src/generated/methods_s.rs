use super::*;

// wxStandardPaths
pub trait StandardPathsMethods: WxRustMethods {
    fn dont_ignore_app_sub_dir(&self) {
        unsafe { ffi::wxStandardPaths_DontIgnoreAppSubDir(self.as_ptr()) }
    }
    fn get_app_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetAppDocumentsDir(self.as_ptr())).into() }
    }
    fn get_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetConfigDir(self.as_ptr())).into() }
    }
    fn get_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDataDir(self.as_ptr())).into() }
    }
    fn get_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDocumentsDir(self.as_ptr())).into() }
    }
    fn get_executable_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetExecutablePath(self.as_ptr())).into() }
    }
    fn get_install_prefix(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetInstallPrefix(self.as_ptr())).into() }
    }
    fn get_local_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetLocalDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetLocalizedResourcesDir()
    fn get_plugins_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetPluginsDir(self.as_ptr())).into() }
    }
    fn get_resources_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetResourcesDir(self.as_ptr())).into() }
    }
    fn get_temp_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetTempDir(self.as_ptr())).into() }
    }
    fn get_user_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserConfigDir(self.as_ptr())).into() }
    }
    fn get_user_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetUserDir()
    fn get_user_local_data_dir(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxStandardPaths_GetUserLocalDataDir(self.as_ptr())).into()
        }
    }
    fn ignore_app_sub_dir(&self, subdir_pattern: &str) {
        unsafe {
            let subdir_pattern = WxString::from(subdir_pattern);
            let subdir_pattern = subdir_pattern.as_ptr();
            ffi::wxStandardPaths_IgnoreAppSubDir(self.as_ptr(), subdir_pattern)
        }
    }
    fn ignore_app_build_sub_dirs(&self) {
        unsafe { ffi::wxStandardPaths_IgnoreAppBuildSubDirs(self.as_ptr()) }
    }
    fn set_install_prefix(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxStandardPaths_SetInstallPrefix(self.as_ptr(), prefix)
        }
    }
    fn use_app_info(&self, info: c_int) {
        unsafe { ffi::wxStandardPaths_UseAppInfo(self.as_ptr(), info) }
    }
    // NOT_SUPPORTED: fn SetFileLayout()
    // NOT_SUPPORTED: fn GetFileLayout()
    // NOT_SUPPORTED: fn MakeConfigFileName()
    fn get() -> StandardPathsIsOwned<false> {
        unsafe { StandardPathsIsOwned::from_ptr(ffi::wxStandardPaths_Get()) }
    }
    fn msw_get_shell_dir(csidl: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_MSWGetShellDir(csidl)).into() }
    }
}
