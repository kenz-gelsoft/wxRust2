use super::*;

// wxStandardPaths
pub trait StandardPathsMethods: WxRustMethods {
    /// MSW-specific function undoing the effect of IgnoreAppSubDir() calls.
    fn dont_ignore_app_sub_dir(&self) {
        unsafe { ffi::wxStandardPaths_DontIgnoreAppSubDir(self.as_ptr()) }
    }
    /// Return the directory for the document files used by this application.
    fn get_app_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetAppDocumentsDir(self.as_ptr())).into() }
    }
    /// Return the directory containing the system config files.
    fn get_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetConfigDir(self.as_ptr())).into() }
    }
    /// Return the location of the applications global, i.e. not user-specific, data files.
    fn get_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDataDir(self.as_ptr())).into() }
    }
    /// Same as calling GetUserDir() with Dir_Documents parameter.
    fn get_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDocumentsDir(self.as_ptr())).into() }
    }
    /// Return the directory and the filename for the current executable.
    fn get_executable_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetExecutablePath(self.as_ptr())).into() }
    }
    /// Return the program installation prefix, e.g. /usr, /opt or /home/zeitlin.
    fn get_install_prefix(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetInstallPrefix(self.as_ptr())).into() }
    }
    /// Return the location for application data files which are host-specific and can't, or shouldn't, be shared with the other machines.
    fn get_local_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetLocalDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetLocalizedResourcesDir()
    /// Return the directory where the loadable modules (plugins) live.
    fn get_plugins_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetPluginsDir(self.as_ptr())).into() }
    }
    /// Return the directory where the application resource files are located.
    fn get_resources_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetResourcesDir(self.as_ptr())).into() }
    }
    /// Return the directory for storing temporary files, for the current user.
    fn get_temp_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetTempDir(self.as_ptr())).into() }
    }
    /// Return the directory for the user config files.
    fn get_user_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserConfigDir(self.as_ptr())).into() }
    }
    /// Return the directory for the user-dependent application data files:
    fn get_user_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetUserDir()
    /// Return the directory for user data files which shouldn't be shared with the other machines.
    fn get_user_local_data_dir(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxStandardPaths_GetUserLocalDataDir(self.as_ptr())).into()
        }
    }
    /// MSW-specific function to customize application directory detection.
    fn ignore_app_sub_dir(&self, subdir_pattern: &str) {
        unsafe {
            let subdir_pattern = WxString::from(subdir_pattern);
            let subdir_pattern = subdir_pattern.as_ptr();
            ffi::wxStandardPaths_IgnoreAppSubDir(self.as_ptr(), subdir_pattern)
        }
    }
    /// MSW-specific function to ignore all common build directories.
    fn ignore_app_build_sub_dirs(&self) {
        unsafe { ffi::wxStandardPaths_IgnoreAppBuildSubDirs(self.as_ptr()) }
    }
    /// Lets wxStandardPaths know about the real program installation prefix on a Unix system.
    fn set_install_prefix(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxStandardPaths_SetInstallPrefix(self.as_ptr(), prefix)
        }
    }
    /// Controls what application information is used when constructing paths that should be unique to this program, such as the application data directory, the plugins directory on Unix, etc.
    fn use_app_info(&self, info: c_int) {
        unsafe { ffi::wxStandardPaths_UseAppInfo(self.as_ptr(), info) }
    }
    // NOT_SUPPORTED: fn SetFileLayout()
    // NOT_SUPPORTED: fn GetFileLayout()
    // NOT_SUPPORTED: fn MakeConfigFileName()
    /// Returns reference to the unique global standard paths object.
    fn get() -> StandardPathsIsOwned<false> {
        unsafe { StandardPathsIsOwned::from_ptr(ffi::wxStandardPaths_Get()) }
    }
    /// Returns location of Windows shell special folder.
    fn msw_get_shell_dir(csidl: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_MSWGetShellDir(csidl)).into() }
    }
}
