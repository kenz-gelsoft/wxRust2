use super::*;

// wxFileName
/// This trait represents [C++ `wxFileName` class](https://docs.wxwidgets.org/3.2/classwx_file_name.html)'s methods and inheritance.
///
/// See [`FileNameFromCpp`] documentation for the class usage.
pub trait FileNameMethods: WxRustMethods {
    /// Appends a directory component to the path.
    ///
    /// See [C++ `wxFileName::AppendDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#acd4448c31bed17a4d1886fc01bac6daa).
    fn append_dir(&self, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_AppendDir(self.as_ptr(), dir)
        }
    }
    /// Creates the file name from another filename object.
    ///
    /// See [C++ `wxFileName::Assign()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ab75d82f4024c6f15cc4247c8835da557).
    fn assign<F: FileNameMethods>(&self, filepath: &F) {
        unsafe {
            let filepath = filepath.as_ptr();
            ffi::wxFileName_Assign(self.as_ptr(), filepath)
        }
    }
    // NOT_SUPPORTED: fn Assign1()
    // NOT_SUPPORTED: fn Assign2()
    // NOT_SUPPORTED: fn Assign3()
    // NOT_SUPPORTED: fn Assign4()
    // NOT_SUPPORTED: fn Assign5()
    /// Makes this object refer to the current working directory on the specified volume (or current volume if volume is empty).
    ///
    /// See [C++ `wxFileName::AssignCwd()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a1cf7a6e3a6ac0f34260ea79f48663cc8).
    fn assign_cwd(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_AssignCwd(self.as_ptr(), volume)
        }
    }
    // NOT_SUPPORTED: fn AssignDir()
    /// Sets this file name object to the home directory.
    ///
    /// See [C++ `wxFileName::AssignHomeDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a021419b87e867a31ff4af5b6e84911cd).
    fn assign_home_dir(&self) {
        unsafe { ffi::wxFileName_AssignHomeDir(self.as_ptr()) }
    }
    /// The function calls CreateTempFileName() to create a temporary file and sets this object to the name of the file.
    ///
    /// See [C++ `wxFileName::AssignTempFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a6c56af101aea8c3d98c693811a44db69).
    fn assign_temp_file_name(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName(self.as_ptr(), prefix)
        }
    }
    /// The function calls CreateTempFileName() to create a temporary file name and open fileTemp with it.
    ///
    /// See [C++ `wxFileName::AssignTempFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#acf47b12b1de8c47fc5cbc2ec7b5fe628).
    fn assign_temp_file_name_file(&self, prefix: &str, file_temp: *mut c_void) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName1(self.as_ptr(), prefix, file_temp)
        }
    }
    /// The function calls CreateTempFileName() to create a temporary file name and open fileTemp with it.
    ///
    /// See [C++ `wxFileName::AssignTempFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a0807cf78dcddc42d2467ee45cc108d08).
    fn assign_temp_file_name_ffile(&self, prefix: &str, file_temp: *mut c_void) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName2(self.as_ptr(), prefix, file_temp)
        }
    }
    /// Reset all components to default, uninitialized state.
    ///
    /// See [C++ `wxFileName::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#aa4fbb922dd41ee6012f15c6d162fb0b3).
    fn clear(&self) {
        unsafe { ffi::wxFileName_Clear(self.as_ptr()) }
    }
    /// Removes the extension from the file name resulting in a file name with no trailing dot.
    ///
    /// See [C++ `wxFileName::ClearExt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a03a2f73191dbf2392b9f1c8e850b407a).
    fn clear_ext(&self) {
        unsafe { ffi::wxFileName_ClearExt(self.as_ptr()) }
    }
    /// Returns true if the directory with this name exists.
    ///
    /// See [C++ `wxFileName::DirExists()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a787c72b7b71d1fa391c5ffd5170e1b17).
    fn dir_exists(&self) -> bool {
        unsafe { ffi::wxFileName_DirExists(self.as_ptr()) }
    }
    /// Turns off symlink dereferencing.
    ///
    /// See [C++ `wxFileName::DontFollowLink()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#af1430dafaf1f522710b52f0a0bf0f060).
    fn dont_follow_link(&self) {
        unsafe { ffi::wxFileName_DontFollowLink(self.as_ptr()) }
    }
    /// Calls the static overload of this function with the full path of this object.
    ///
    /// See [C++ `wxFileName::Exists()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a3a67a151411ee2e7eabbcd6087691ee4).
    fn exists_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Exists(self.as_ptr(), flags) }
    }
    /// Returns true if the file with this name exists.
    ///
    /// See [C++ `wxFileName::FileExists()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a52cb500b34e5338fbd8d3db5867ace9e).
    fn file_exists(&self) -> bool {
        unsafe { ffi::wxFileName_FileExists(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAbsolutePath()
    /// Returns the number of directories in the file name.
    ///
    /// See [C++ `wxFileName::GetDirCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a3706feaa2a3d3576aa62eeac5094bdd4).
    fn get_dir_count(&self) -> usize {
        unsafe { ffi::wxFileName_GetDirCount(self.as_ptr()) }
    }
    /// Returns the directories in string array form.
    ///
    /// See [C++ `wxFileName::GetDirs()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ae7a4a2abba5a95a547dc35e1ed43a67b).
    fn get_dirs(&self) -> ArrayStringFromCpp<false> {
        unsafe { ArrayStringFromCpp::from_ptr(ffi::wxFileName_GetDirs(self.as_ptr())) }
    }
    /// Returns the file name extension.
    ///
    /// See [C++ `wxFileName::GetExt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ae5e2d1095dceb2ff315dcfd57e824945).
    fn get_ext(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetExt(self.as_ptr())).into() }
    }
    /// Returns the full name (including extension but excluding directories).
    ///
    /// See [C++ `wxFileName::GetFullName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#abc779377cd917cc2487b76a5e0ff7949).
    fn get_full_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetFullName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFullPath()
    // NOT_SUPPORTED: fn GetHumanReadableSize()
    /// Return the long form of the path (returns identity on non-Windows platforms).
    ///
    /// See [C++ `wxFileName::GetLongPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ac9866d9a346e6baaa70371ce49d1e29c).
    fn get_long_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetLongPath(self.as_ptr())).into() }
    }
    /// Returns the last time the file was last modified.
    ///
    /// See [C++ `wxFileName::GetModificationTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a81fb52063e04c58128f996c8292ce81f).
    fn get_modification_time(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxFileName_GetModificationTime(self.as_ptr())) }
    }
    /// Returns the name part of the filename (without extension).
    ///
    /// See [C++ `wxFileName::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ad8f3f744b2c926eee9567bb30f9868a9).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetPath()
    // NOT_SUPPORTED: fn GetPathWithSep()
    /// Return the short form of the path (returns identity on non-Windows platforms).
    ///
    /// See [C++ `wxFileName::GetShortPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a522f1e656caef976efd87f82b34b6a85).
    fn get_short_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetShortPath(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetSize()
    /// Returns the last access, last modification and creation times.
    ///
    /// See [C++ `wxFileName::GetTimes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a57f5777485c1409be723755b54a92a12).
    fn get_times<D: DateTimeMethods, D2: DateTimeMethods, D3: DateTimeMethods>(
        &self,
        dt_access: Option<&D>,
        dt_mod: Option<&D2>,
        dt_create: Option<&D3>,
    ) -> bool {
        unsafe {
            let dt_access = match dt_access {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_mod = match dt_mod {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_create = match dt_create {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_GetTimes(self.as_ptr(), dt_access, dt_mod, dt_create)
        }
    }
    /// Returns the string containing the volume for this file name.
    ///
    /// See [C++ `wxFileName::GetVolume()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a51afe14188bbfa6442598730db8a9881).
    fn get_volume(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetVolume(self.as_ptr())).into() }
    }
    /// Returns true if an extension is present.
    ///
    /// See [C++ `wxFileName::HasExt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#afc1569201ca209ef97457f60cf272bfc).
    fn has_ext(&self) -> bool {
        unsafe { ffi::wxFileName_HasExt(self.as_ptr()) }
    }
    /// Returns true if a name is present.
    ///
    /// See [C++ `wxFileName::HasName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a98f33bc4f026e7e396afdb0e1802f1cf).
    fn has_name(&self) -> bool {
        unsafe { ffi::wxFileName_HasName(self.as_ptr()) }
    }
    /// Returns true if a volume specifier is present.
    ///
    /// See [C++ `wxFileName::HasVolume()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a80820a2958875e8e3a4a4d6e367ae538).
    fn has_volume(&self) -> bool {
        unsafe { ffi::wxFileName_HasVolume(self.as_ptr()) }
    }
    /// Inserts a directory component before the zero-based position in the directory list.
    ///
    /// See [C++ `wxFileName::InsertDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ae722c59b18f226312565cccb1c70a83f).
    fn insert_dir(&self, before: usize, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_InsertDir(self.as_ptr(), before, dir)
        }
    }
    // NOT_SUPPORTED: fn IsAbsolute()
    /// Returns true if this object represents a directory, false otherwise (i.e.
    ///
    /// See [C++ `wxFileName::IsDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#abd3d69deb2de515f51e76065f35e75cf).
    fn is_dir(&self) -> bool {
        unsafe { ffi::wxFileName_IsDir(self.as_ptr()) }
    }
    /// Returns true if the directory component of this instance is an existing directory and this process has read permissions on it.
    ///
    /// See [C++ `wxFileName::IsDirReadable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a6d3e8730fe07ccc288a9712f5272ea52).
    fn is_dir_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirReadable(self.as_ptr()) }
    }
    /// Returns true if the directory component of this instance is an existing directory and this process has write permissions on it.
    ///
    /// See [C++ `wxFileName::IsDirWritable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a56d601e5a5a79b75b550da0e1a35e0da).
    fn is_dir_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirWritable(self.as_ptr()) }
    }
    /// Returns true if a file with this name exists and if this process has execute permissions on it.
    ///
    /// See [C++ `wxFileName::IsFileExecutable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#afd7126bf8ce3f1d5ffe675179f5e4ca0).
    fn is_file_executable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileExecutable(self.as_ptr()) }
    }
    /// Returns true if a file with this name exists and if this process has read permissions on it.
    ///
    /// See [C++ `wxFileName::IsFileReadable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ae1d433eda553c7db2ed533eef8ce3766).
    fn is_file_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileReadable(self.as_ptr()) }
    }
    /// Returns true if a file with this name exists and if this process has write permissions on it.
    ///
    /// See [C++ `wxFileName::IsFileWritable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a097707017ebc8d068e6fa4d38d9efb56).
    fn is_file_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileWritable(self.as_ptr()) }
    }
    /// Returns true if the filename is valid, false if it is not initialized yet.
    ///
    /// See [C++ `wxFileName::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a1abe6ced5f0a8f69761cc91711b2a69f).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFileName_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsRelative()
    // NOT_SUPPORTED: fn MakeAbsolute()
    // NOT_SUPPORTED: fn MakeRelativeTo()
    /// Creates a directory.
    ///
    /// See [C++ `wxFileName::Mkdir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#aa6cc6ee8389c6ce4f94fffa6bdf94b17).
    fn mkdir_int(&self, perm: c_int, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Mkdir(self.as_ptr(), perm, flags) }
    }
    // NOT_SUPPORTED: fn Normalize()
    /// Prepends a directory to the file path.
    ///
    /// See [C++ `wxFileName::PrependDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a577475123161b94cb8f8a597cefe64d5).
    fn prepend_dir(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_PrependDir(self.as_ptr(), dir)
        }
    }
    /// Removes the specified directory component from the path.
    ///
    /// See [C++ `wxFileName::RemoveDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ad2b8491684c5bf97395ab9aa2f3d398b).
    fn remove_dir(&self, pos: usize) {
        unsafe { ffi::wxFileName_RemoveDir(self.as_ptr(), pos) }
    }
    /// Removes last directory component from the path.
    ///
    /// See [C++ `wxFileName::RemoveLastDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ab1336b56add13e0f2830197a3680da94).
    fn remove_last_dir(&self) {
        unsafe { ffi::wxFileName_RemoveLastDir(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ReplaceEnvVariable()
    // NOT_SUPPORTED: fn ReplaceHomeDir()
    /// Find the absolute path of the file/directory that is pointed to by this path.
    ///
    /// See [C++ `wxFileName::ResolveLink()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a9c4d1743f75827d99951d033b00719ba).
    fn resolve_link(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFileName_ResolveLink(self.as_ptr())) }
    }
    /// Deletes the specified directory from the file system.
    ///
    /// See [C++ `wxFileName::Rmdir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a55d5f2f9af1a6430a74dfd75ec538028).
    fn rmdir_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Rmdir(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SameAs()
    /// Changes the current working directory.
    ///
    /// See [C++ `wxFileName::SetCwd()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#aa35ca9aa25e0db539031de039c63fd16).
    fn set_cwd(&self) -> bool {
        unsafe { ffi::wxFileName_SetCwd(self.as_ptr()) }
    }
    /// Sets the extension of the file name to be an empty extension.
    ///
    /// See [C++ `wxFileName::SetEmptyExt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a2d5df1c71c011b0f8897dc51fe8d91ce).
    fn set_empty_ext(&self) {
        unsafe { ffi::wxFileName_SetEmptyExt(self.as_ptr()) }
    }
    /// Sets the extension of the file name.
    ///
    /// See [C++ `wxFileName::SetExt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a65c37fa5017b400d41009bbab56f2774).
    fn set_ext(&self, ext: &str) {
        unsafe {
            let ext = WxString::from(ext);
            let ext = ext.as_ptr();
            ffi::wxFileName_SetExt(self.as_ptr(), ext)
        }
    }
    /// The full name is the file name and extension (but without the path).
    ///
    /// See [C++ `wxFileName::SetFullName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a2490c8f2cc92fb88196adc33809a36c0).
    fn set_full_name(&self, fullname: &str) {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            ffi::wxFileName_SetFullName(self.as_ptr(), fullname)
        }
    }
    /// Sets the name part (without extension).
    ///
    /// See [C++ `wxFileName::SetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a1bf12dfe55c3a2a56982a7a731fa1230).
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFileName_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetPath()
    /// Sets permissions for this file or directory.
    ///
    /// See [C++ `wxFileName::SetPermissions()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ae1b6847990f5b41f5b53fb46a5e5fb79).
    fn set_permissions(&self, permissions: c_int) -> bool {
        unsafe { ffi::wxFileName_SetPermissions(self.as_ptr(), permissions) }
    }
    /// Sets the file creation and last access/modification times (any of the pointers may be NULL).
    ///
    /// See [C++ `wxFileName::SetTimes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a38d689624c41dd79170fc94fc6be47cf).
    fn set_times<D: DateTimeMethods, D2: DateTimeMethods, D3: DateTimeMethods>(
        &self,
        dt_access: Option<&D>,
        dt_mod: Option<&D2>,
        dt_create: Option<&D3>,
    ) -> bool {
        unsafe {
            let dt_access = match dt_access {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_mod = match dt_mod {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_create = match dt_create {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_SetTimes(self.as_ptr(), dt_access, dt_mod, dt_create)
        }
    }
    /// Sets the volume specifier.
    ///
    /// See [C++ `wxFileName::SetVolume()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a0c8e256bd978ab9d0cc3c640758b2e2f).
    fn set_volume(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_SetVolume(self.as_ptr(), volume)
        }
    }
    /// Return whether some operations will follow symlink.
    ///
    /// See [C++ `wxFileName::ShouldFollowLink()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a70a36037ef773e2a00eddb7532e8a3f7).
    fn should_follow_link(&self) -> bool {
        unsafe { ffi::wxFileName_ShouldFollowLink(self.as_ptr()) }
    }
    /// Sets the access and modification times to the current moment.
    ///
    /// See [C++ `wxFileName::Touch()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#aaf2926a7cb0ad2dd79f9c9e14d4c38c1).
    fn touch(&self) -> bool {
        unsafe { ffi::wxFileName_Touch(self.as_ptr()) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator!=1()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator==1()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator=1()
    /// Returns a temporary file name starting with the given prefix.
    ///
    /// See [C++ `wxFileName::CreateTempFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a416af84fddd624a3b457dbffbe174317).
    fn create_temp_file_name_file(prefix: &str, file_temp: *mut c_void) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName(prefix, file_temp)).into()
        }
    }
    /// This is the same as CreateTempFileName(const wxString &prefix, wxFile *fileTemp) but takes a wxFFile parameter instead of wxFile.
    ///
    /// See [C++ `wxFileName::CreateTempFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a5a03fe160df4807b4de1c3bd0696d2ad).
    fn create_temp_file_name_ffile(prefix: &str, file_temp: *mut c_void) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName1(prefix, file_temp)).into()
        }
    }
    /// Returns true if the directory with name dir exists.
    ///
    /// See [C++ `wxFileName::DirExists()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a706cd439af5469ff913bb022dff1ed71).
    fn dir_exists_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_DirExists1(dir)
        }
    }
    // NOT_SUPPORTED: fn DirName()
    /// Returns true if either a file or a directory or something else with this name exists in the file system.
    ///
    /// See [C++ `wxFileName::Exists()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a1229d995f7ba67e0b66f5fb556debfb1).
    fn exists_str(path: &str, flags: c_int) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileName_Exists1(path, flags)
        }
    }
    /// Returns true if the file with name file exists.
    ///
    /// See [C++ `wxFileName::FileExists()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ab8a8e5bfeaf12b6479ffc295a68137b9).
    fn file_exists_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_FileExists1(file)
        }
    }
    // NOT_SUPPORTED: fn FileName()
    /// Retrieves the value of the current working directory on the specified volume.
    ///
    /// See [C++ `wxFileName::GetCwd()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#af697f1c0f7864fc35ac0e8198eacc84d).
    fn get_cwd(volume: &str) -> String {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            WxString::from_ptr(ffi::wxFileName_GetCwd(volume)).into()
        }
    }
    // NOT_SUPPORTED: fn GetForbiddenChars()
    // NOT_SUPPORTED: fn GetFormat()
    /// Returns the home directory.
    ///
    /// See [C++ `wxFileName::GetHomeDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a8699fafd7b38069c2af0b4f0de89c180).
    fn get_home_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetHomeDir()).into() }
    }
    // NOT_SUPPORTED: fn GetHumanReadableSize1()
    // NOT_SUPPORTED: fn GetPathSeparator()
    // NOT_SUPPORTED: fn GetPathSeparators()
    // NOT_SUPPORTED: fn GetPathTerminators()
    // NOT_SUPPORTED: fn GetSize1()
    /// Returns the directory used for temporary files, for current user.
    ///
    /// See [C++ `wxFileName::GetTempDir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ada549d0b26eb20fc03f42f2182b05fc9).
    fn get_temp_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetTempDir()).into() }
    }
    // NOT_SUPPORTED: fn GetVolumeSeparator()
    // NOT_SUPPORTED: fn GetVolumeString()
    // NOT_SUPPORTED: fn IsCaseSensitive()
    /// Returns true if the given dir is an existing directory and this process has read permissions on it.
    ///
    /// See [C++ `wxFileName::IsDirReadable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a37759678b0b0e4f01076c4358a2e63b9).
    fn is_dir_readable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirReadable1(dir)
        }
    }
    /// Returns true if the given dir is an existing directory and this process has write permissions on it.
    ///
    /// See [C++ `wxFileName::IsDirWritable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a1af163f5c99159a34be2f905eb1ec03b).
    fn is_dir_writable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirWritable1(dir)
        }
    }
    /// Returns true if a file with this name exists and if this process has execute permissions on it.
    ///
    /// See [C++ `wxFileName::IsFileExecutable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a65973db654e5b7b03eac0079b34f4694).
    fn is_file_executable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileExecutable1(file)
        }
    }
    /// Returns true if a file with this name exists and if this process has read permissions on it.
    ///
    /// See [C++ `wxFileName::IsFileReadable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#aaf33a5de7f79587a58f9e20ae22d89b4).
    fn is_file_readable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileReadable1(file)
        }
    }
    /// Returns true if a file with this name exists and if this process has write permissions on it.
    ///
    /// See [C++ `wxFileName::IsFileWritable()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ac430b77411e87a46f7a354bcf384cf65).
    fn is_file_writable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileWritable1(file)
        }
    }
    // NOT_SUPPORTED: fn IsPathSeparator()
    // NOT_SUPPORTED: fn IsMSWUniqueVolumeNamePath()
    /// Creates a directory.
    ///
    /// See [C++ `wxFileName::Mkdir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#ac2006b2fb30c0c0f11ef44560e1a62c7).
    fn mkdir_str(dir: &str, perm: c_int, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Mkdir1(dir, perm, flags)
        }
    }
    /// Deletes the specified directory from the file system.
    ///
    /// See [C++ `wxFileName::Rmdir()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#aa0aa5becff9a6d91ac5108bea7f12932).
    fn rmdir_str(dir: &str, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Rmdir1(dir, flags)
        }
    }
    /// Changes the current working directory.
    ///
    /// See [C++ `wxFileName::SetCwd()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a5c1d005ca788d42ca5d1c7bf8ff3d6b6).
    fn set_cwd_str(cwd: &str) -> bool {
        unsafe {
            let cwd = WxString::from(cwd);
            let cwd = cwd.as_ptr();
            ffi::wxFileName_SetCwd1(cwd)
        }
    }
    /// Converts URL into a well-formed filename.
    ///
    /// See [C++ `wxFileName::URLToFileName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a2741a6fa2caf95bb539a562dd4cbadf9).
    fn url_to_file_name(url: &str) -> FileName {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            FileName::from_ptr(ffi::wxFileName_URLToFileName(url))
        }
    }
    /// Converts wxFileName into an URL.
    ///
    /// See [C++ `wxFileName::FileNameToURL()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a0eed399043b21d4e397e539c46e557e8).
    fn file_name_to_url<F: FileNameMethods>(filename: &F) -> String {
        unsafe {
            let filename = filename.as_ptr();
            WxString::from_ptr(ffi::wxFileName_FileNameToURL(filename)).into()
        }
    }
    // NOT_SUPPORTED: fn SplitPath()
    // NOT_SUPPORTED: fn SplitPath1()
    // NOT_SUPPORTED: fn SplitPath2()
    // NOT_SUPPORTED: fn SplitVolume()
    /// Strip the file extension.
    ///
    /// See [C++ `wxFileName::StripExtension()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_file_name.html#a1cc6ef65bdf702fc220893d1e1db1141).
    fn strip_extension(fullname: &str) -> String {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            WxString::from_ptr(ffi::wxFileName_StripExtension(fullname)).into()
        }
    }
}
