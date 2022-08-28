use super::*;

// wxFileName
pub trait FileNameMethods: WxRustMethods {
    /// Appends a directory component to the path.
    fn append_dir(&self, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_AppendDir(self.as_ptr(), dir)
        }
    }
    /// Creates the file name from another filename object.
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
    fn assign_cwd(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_AssignCwd(self.as_ptr(), volume)
        }
    }
    // NOT_SUPPORTED: fn AssignDir()
    /// Sets this file name object to the home directory.
    fn assign_home_dir(&self) {
        unsafe { ffi::wxFileName_AssignHomeDir(self.as_ptr()) }
    }
    /// The function calls CreateTempFileName() to create a temporary file and sets this object to the name of the file.
    fn assign_temp_file_name(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName(self.as_ptr(), prefix)
        }
    }
    /// The function calls CreateTempFileName() to create a temporary file name and open fileTemp with it.
    fn assign_temp_file_name_file(&self, prefix: &str, file_temp: *mut c_void) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName1(self.as_ptr(), prefix, file_temp)
        }
    }
    /// The function calls CreateTempFileName() to create a temporary file name and open fileTemp with it.
    fn assign_temp_file_name_ffile(&self, prefix: &str, file_temp: *mut c_void) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName2(self.as_ptr(), prefix, file_temp)
        }
    }
    /// Reset all components to default, uninitialized state.
    fn clear(&self) {
        unsafe { ffi::wxFileName_Clear(self.as_ptr()) }
    }
    /// Removes the extension from the file name resulting in a file name with no trailing dot.
    fn clear_ext(&self) {
        unsafe { ffi::wxFileName_ClearExt(self.as_ptr()) }
    }
    /// Returns true if the directory with this name exists.
    fn dir_exists(&self) -> bool {
        unsafe { ffi::wxFileName_DirExists(self.as_ptr()) }
    }
    /// Turns off symlink dereferencing.
    fn dont_follow_link(&self) {
        unsafe { ffi::wxFileName_DontFollowLink(self.as_ptr()) }
    }
    /// Calls the static overload of this function with the full path of this object.
    fn exists_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Exists(self.as_ptr(), flags) }
    }
    /// Returns true if the file with this name exists.
    fn file_exists(&self) -> bool {
        unsafe { ffi::wxFileName_FileExists(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAbsolutePath()
    /// Returns the number of directories in the file name.
    fn get_dir_count(&self) -> usize {
        unsafe { ffi::wxFileName_GetDirCount(self.as_ptr()) }
    }
    /// Returns the directories in string array form.
    fn get_dirs(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxFileName_GetDirs(self.as_ptr())) }
    }
    /// Returns the file name extension.
    fn get_ext(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetExt(self.as_ptr())).into() }
    }
    /// Returns the full name (including extension but excluding directories).
    fn get_full_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetFullName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFullPath()
    // NOT_SUPPORTED: fn GetHumanReadableSize()
    /// Return the long form of the path (returns identity on non-Windows platforms).
    fn get_long_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetLongPath(self.as_ptr())).into() }
    }
    /// Returns the last time the file was last modified.
    fn get_modification_time(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxFileName_GetModificationTime(self.as_ptr())) }
    }
    /// Returns the name part of the filename (without extension).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetPath()
    // NOT_SUPPORTED: fn GetPathWithSep()
    /// Return the short form of the path (returns identity on non-Windows platforms).
    fn get_short_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetShortPath(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetSize()
    /// Returns the last access, last modification and creation times.
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
    fn get_volume(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetVolume(self.as_ptr())).into() }
    }
    /// Returns true if an extension is present.
    fn has_ext(&self) -> bool {
        unsafe { ffi::wxFileName_HasExt(self.as_ptr()) }
    }
    /// Returns true if a name is present.
    fn has_name(&self) -> bool {
        unsafe { ffi::wxFileName_HasName(self.as_ptr()) }
    }
    /// Returns true if a volume specifier is present.
    fn has_volume(&self) -> bool {
        unsafe { ffi::wxFileName_HasVolume(self.as_ptr()) }
    }
    /// Inserts a directory component before the zero-based position in the directory list.
    fn insert_dir(&self, before: usize, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_InsertDir(self.as_ptr(), before, dir)
        }
    }
    // NOT_SUPPORTED: fn IsAbsolute()
    /// Returns true if this object represents a directory, false otherwise (i.e.
    fn is_dir(&self) -> bool {
        unsafe { ffi::wxFileName_IsDir(self.as_ptr()) }
    }
    /// Returns true if the directory component of this instance is an existing directory and this process has read permissions on it.
    fn is_dir_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirReadable(self.as_ptr()) }
    }
    /// Returns true if the directory component of this instance is an existing directory and this process has write permissions on it.
    fn is_dir_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirWritable(self.as_ptr()) }
    }
    /// Returns true if a file with this name exists and if this process has execute permissions on it.
    fn is_file_executable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileExecutable(self.as_ptr()) }
    }
    /// Returns true if a file with this name exists and if this process has read permissions on it.
    fn is_file_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileReadable(self.as_ptr()) }
    }
    /// Returns true if a file with this name exists and if this process has write permissions on it.
    fn is_file_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileWritable(self.as_ptr()) }
    }
    /// Returns true if the filename is valid, false if it is not initialized yet.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFileName_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsRelative()
    // NOT_SUPPORTED: fn MakeAbsolute()
    // NOT_SUPPORTED: fn MakeRelativeTo()
    /// Creates a directory.
    fn mkdir_int(&self, perm: c_int, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Mkdir(self.as_ptr(), perm, flags) }
    }
    // NOT_SUPPORTED: fn Normalize()
    /// Prepends a directory to the file path.
    fn prepend_dir(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_PrependDir(self.as_ptr(), dir)
        }
    }
    /// Removes the specified directory component from the path.
    fn remove_dir(&self, pos: usize) {
        unsafe { ffi::wxFileName_RemoveDir(self.as_ptr(), pos) }
    }
    /// Removes last directory component from the path.
    fn remove_last_dir(&self) {
        unsafe { ffi::wxFileName_RemoveLastDir(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ReplaceEnvVariable()
    // NOT_SUPPORTED: fn ReplaceHomeDir()
    /// Find the absolute path of the file/directory that is pointed to by this path.
    fn resolve_link(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFileName_ResolveLink(self.as_ptr())) }
    }
    /// Deletes the specified directory from the file system.
    fn rmdir_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Rmdir(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SameAs()
    /// Changes the current working directory.
    fn set_cwd(&self) -> bool {
        unsafe { ffi::wxFileName_SetCwd(self.as_ptr()) }
    }
    /// Sets the extension of the file name to be an empty extension.
    fn set_empty_ext(&self) {
        unsafe { ffi::wxFileName_SetEmptyExt(self.as_ptr()) }
    }
    /// Sets the extension of the file name.
    fn set_ext(&self, ext: &str) {
        unsafe {
            let ext = WxString::from(ext);
            let ext = ext.as_ptr();
            ffi::wxFileName_SetExt(self.as_ptr(), ext)
        }
    }
    /// The full name is the file name and extension (but without the path).
    fn set_full_name(&self, fullname: &str) {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            ffi::wxFileName_SetFullName(self.as_ptr(), fullname)
        }
    }
    /// Sets the name part (without extension).
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFileName_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetPath()
    /// Sets permissions for this file or directory.
    fn set_permissions(&self, permissions: c_int) -> bool {
        unsafe { ffi::wxFileName_SetPermissions(self.as_ptr(), permissions) }
    }
    /// Sets the file creation and last access/modification times (any of the pointers may be NULL).
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
    fn set_volume(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_SetVolume(self.as_ptr(), volume)
        }
    }
    /// Return whether some operations will follow symlink.
    fn should_follow_link(&self) -> bool {
        unsafe { ffi::wxFileName_ShouldFollowLink(self.as_ptr()) }
    }
    /// Sets the access and modification times to the current moment.
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
    fn create_temp_file_name_file(prefix: &str, file_temp: *mut c_void) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName(prefix, file_temp)).into()
        }
    }
    /// This is the same as CreateTempFileName(const wxString &prefix, wxFile *fileTemp) but takes a wxFFile parameter instead of wxFile.
    fn create_temp_file_name_ffile(prefix: &str, file_temp: *mut c_void) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName1(prefix, file_temp)).into()
        }
    }
    /// Returns true if the directory with name dir exists.
    fn dir_exists_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_DirExists1(dir)
        }
    }
    // NOT_SUPPORTED: fn DirName()
    /// Returns true if either a file or a directory or something else with this name exists in the file system.
    fn exists_str(path: &str, flags: c_int) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileName_Exists1(path, flags)
        }
    }
    /// Returns true if the file with name file exists.
    fn file_exists_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_FileExists1(file)
        }
    }
    // NOT_SUPPORTED: fn FileName()
    /// Retrieves the value of the current working directory on the specified volume.
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
    fn get_home_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetHomeDir()).into() }
    }
    // NOT_SUPPORTED: fn GetHumanReadableSize1()
    // NOT_SUPPORTED: fn GetPathSeparator()
    // NOT_SUPPORTED: fn GetPathSeparators()
    // NOT_SUPPORTED: fn GetPathTerminators()
    // NOT_SUPPORTED: fn GetSize1()
    /// Returns the directory used for temporary files, for current user.
    fn get_temp_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetTempDir()).into() }
    }
    // NOT_SUPPORTED: fn GetVolumeSeparator()
    // NOT_SUPPORTED: fn GetVolumeString()
    // NOT_SUPPORTED: fn IsCaseSensitive()
    /// Returns true if the given dir is an existing directory and this process has read permissions on it.
    fn is_dir_readable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirReadable1(dir)
        }
    }
    /// Returns true if the given dir is an existing directory and this process has write permissions on it.
    fn is_dir_writable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirWritable1(dir)
        }
    }
    /// Returns true if a file with this name exists and if this process has execute permissions on it.
    fn is_file_executable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileExecutable1(file)
        }
    }
    /// Returns true if a file with this name exists and if this process has read permissions on it.
    fn is_file_readable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileReadable1(file)
        }
    }
    /// Returns true if a file with this name exists and if this process has write permissions on it.
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
    fn mkdir_str(dir: &str, perm: c_int, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Mkdir1(dir, perm, flags)
        }
    }
    /// Deletes the specified directory from the file system.
    fn rmdir_str(dir: &str, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Rmdir1(dir, flags)
        }
    }
    /// Changes the current working directory.
    fn set_cwd_str(cwd: &str) -> bool {
        unsafe {
            let cwd = WxString::from(cwd);
            let cwd = cwd.as_ptr();
            ffi::wxFileName_SetCwd1(cwd)
        }
    }
    /// Converts URL into a well-formed filename.
    fn url_to_file_name(url: &str) -> FileName {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            FileName::from_ptr(ffi::wxFileName_URLToFileName(url))
        }
    }
    /// Converts wxFileName into an URL.
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
    fn strip_extension(fullname: &str) -> String {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            WxString::from_ptr(ffi::wxFileName_StripExtension(fullname)).into()
        }
    }
}
