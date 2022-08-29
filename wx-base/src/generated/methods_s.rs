use super::*;

// wxStandardPaths
/// This trait represents [C++ `wxStandardPaths` class](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html)'s methods and inheritance.
///
/// See [`StandardPathsIsOwned`] documentation for the class usage.
pub trait StandardPathsMethods: WxRustMethods {
    /// MSW-specific function undoing the effect of IgnoreAppSubDir() calls.
    ///
    /// See [C++ `wxStandardPaths::DontIgnoreAppSubDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a3348987c98140398e567ef76509400b6).
    fn dont_ignore_app_sub_dir(&self) {
        unsafe { ffi::wxStandardPaths_DontIgnoreAppSubDir(self.as_ptr()) }
    }
    /// Return the directory for the document files used by this application.
    ///
    /// See [C++ `wxStandardPaths::GetAppDocumentsDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#ae3a7994f9a565189daadb1544c5791e2).
    fn get_app_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetAppDocumentsDir(self.as_ptr())).into() }
    }
    /// Return the directory containing the system config files.
    ///
    /// See [C++ `wxStandardPaths::GetConfigDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#aa89751fadd8ca041c4e0316d5008d3ca).
    fn get_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetConfigDir(self.as_ptr())).into() }
    }
    /// Return the location of the applications global, i.e. not user-specific, data files.
    ///
    /// See [C++ `wxStandardPaths::GetDataDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a07037fbc6c9d4afec0258d504a5e073e).
    fn get_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDataDir(self.as_ptr())).into() }
    }
    /// Same as calling GetUserDir() with Dir_Documents parameter.
    ///
    /// See [C++ `wxStandardPaths::GetDocumentsDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a9f2ff127aabdba5a9ce474bd1105ad9a).
    fn get_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDocumentsDir(self.as_ptr())).into() }
    }
    /// Return the directory and the filename for the current executable.
    ///
    /// See [C++ `wxStandardPaths::GetExecutablePath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#aff7a2321cd3389ae9d7c93dded616de6).
    fn get_executable_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetExecutablePath(self.as_ptr())).into() }
    }
    /// Return the program installation prefix, e.g. /usr, /opt or /home/zeitlin.
    ///
    /// See [C++ `wxStandardPaths::GetInstallPrefix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#ac22879eca20ea4fceca9b375f105d29d).
    fn get_install_prefix(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetInstallPrefix(self.as_ptr())).into() }
    }
    /// Return the location for application data files which are host-specific and can't, or shouldn't, be shared with the other machines.
    ///
    /// See [C++ `wxStandardPaths::GetLocalDataDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a2a0ce6d61da8c28df8cd213493698697).
    fn get_local_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetLocalDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetLocalizedResourcesDir()
    /// Return the directory where the loadable modules (plugins) live.
    ///
    /// See [C++ `wxStandardPaths::GetPluginsDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#af876b25e3fb9552e0c22b2331adfef36).
    fn get_plugins_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetPluginsDir(self.as_ptr())).into() }
    }
    /// Return the directory where the application resource files are located.
    ///
    /// See [C++ `wxStandardPaths::GetResourcesDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a4faa3ebe2c42f101601ead08afd561b9).
    fn get_resources_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetResourcesDir(self.as_ptr())).into() }
    }
    /// Return the directory for storing temporary files, for the current user.
    ///
    /// See [C++ `wxStandardPaths::GetTempDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a67e70dee83ed715db981eaad74cdf427).
    fn get_temp_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetTempDir(self.as_ptr())).into() }
    }
    /// Return the directory for the user config files.
    ///
    /// See [C++ `wxStandardPaths::GetUserConfigDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a0561d70f7da648e10b25bf9679309da3).
    fn get_user_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserConfigDir(self.as_ptr())).into() }
    }
    /// Return the directory for the user-dependent application data files:
    ///
    /// See [C++ `wxStandardPaths::GetUserDataDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a5b9d1b1addc3e4ce30ccb3817cbbe19b).
    fn get_user_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetUserDir()
    /// Return the directory for user data files which shouldn't be shared with the other machines.
    ///
    /// See [C++ `wxStandardPaths::GetUserLocalDataDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a7425a6e35c7fb12a0f47792b079ab05b).
    fn get_user_local_data_dir(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxStandardPaths_GetUserLocalDataDir(self.as_ptr())).into()
        }
    }
    /// MSW-specific function to customize application directory detection.
    ///
    /// See [C++ `wxStandardPaths::IgnoreAppSubDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#ab7534e9987d802dada6c02ab70fbaa96).
    fn ignore_app_sub_dir(&self, subdir_pattern: &str) {
        unsafe {
            let subdir_pattern = WxString::from(subdir_pattern);
            let subdir_pattern = subdir_pattern.as_ptr();
            ffi::wxStandardPaths_IgnoreAppSubDir(self.as_ptr(), subdir_pattern)
        }
    }
    /// MSW-specific function to ignore all common build directories.
    ///
    /// See [C++ `wxStandardPaths::IgnoreAppBuildSubDirs()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a36a497e73df7226240f8ac0f0dfa52bc).
    fn ignore_app_build_sub_dirs(&self) {
        unsafe { ffi::wxStandardPaths_IgnoreAppBuildSubDirs(self.as_ptr()) }
    }
    /// Lets wxStandardPaths know about the real program installation prefix on a Unix system.
    ///
    /// See [C++ `wxStandardPaths::SetInstallPrefix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a80af124a7df05bf1a1f7bee7406d278f).
    fn set_install_prefix(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxStandardPaths_SetInstallPrefix(self.as_ptr(), prefix)
        }
    }
    /// Controls what application information is used when constructing paths that should be unique to this program, such as the application data directory, the plugins directory on Unix, etc.
    ///
    /// See [C++ `wxStandardPaths::UseAppInfo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#ada72c782dcab502f17c348c0e5d3874c).
    fn use_app_info(&self, info: c_int) {
        unsafe { ffi::wxStandardPaths_UseAppInfo(self.as_ptr(), info) }
    }
    // NOT_SUPPORTED: fn SetFileLayout()
    // NOT_SUPPORTED: fn GetFileLayout()
    // NOT_SUPPORTED: fn MakeConfigFileName()
    /// Returns reference to the unique global standard paths object.
    ///
    /// See [C++ `wxStandardPaths::Get()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#adcc47d33eccc3d432e40f6952d405c23).
    fn get() -> StandardPathsIsOwned<false> {
        unsafe { StandardPathsIsOwned::from_ptr(ffi::wxStandardPaths_Get()) }
    }
    /// Returns location of Windows shell special folder.
    ///
    /// See [C++ `wxStandardPaths::MSWGetShellDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_standard_paths.html#a5a523db5b6aca46154ac76d264d2bb57).
    fn msw_get_shell_dir(csidl: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_MSWGetShellDir(csidl)).into() }
    }
}
