use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxFileName
    pub fn wxFileName_delete(self_: *mut c_void);
    pub fn wxFileName_new() -> *mut c_void;
    pub fn wxFileName_new1(filename: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new2(fullpath: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new3(path: *const c_void, name: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new4(path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new5(volume: *const c_void, path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_AppendDir(self_: *mut c_void, dir: *const c_void) -> bool;
    pub fn wxFileName_Assign(self_: *mut c_void, filepath: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileName_Assign1(self_: *mut c_void, fullpath: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign2(self_: *mut c_void, volume: *const c_void, path: *const c_void, name: *const c_void, ext: *const c_void, has_ext: bool, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign3(self_: *mut c_void, volume: *const c_void, path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign4(self_: *mut c_void, path: *const c_void, name: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign5(self_: *mut c_void, path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat);
    pub fn wxFileName_AssignCwd(self_: *mut c_void, volume: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileName_AssignDir(self_: *mut c_void, dir: *const c_void, format: wxPathFormat);
    pub fn wxFileName_AssignHomeDir(self_: *mut c_void);
    pub fn wxFileName_AssignTempFileName(self_: *mut c_void, prefix: *const c_void);
    pub fn wxFileName_AssignTempFileName1(
        self_: *mut c_void,
        prefix: *const c_void,
        file_temp: *mut c_void,
    );
    pub fn wxFileName_AssignTempFileName2(
        self_: *mut c_void,
        prefix: *const c_void,
        file_temp: *mut c_void,
    );
    pub fn wxFileName_Clear(self_: *mut c_void);
    pub fn wxFileName_ClearExt(self_: *mut c_void);
    pub fn wxFileName_DirExists(self_: *const c_void) -> bool;
    pub fn wxFileName_DontFollowLink(self_: *mut c_void);
    pub fn wxFileName_Exists(self_: *const c_void, flags: c_int) -> bool;
    pub fn wxFileName_FileExists(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_GetAbsolutePath(self_: *const c_void, cwd: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_GetDirCount(self_: *const c_void) -> usize;
    pub fn wxFileName_GetDirs(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetExt(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetFullName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetFullPath(self_: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetHumanReadableSize(self_: *const c_void, failmsg: *const c_void, precision: c_int, conv: wxSizeConvention) -> *mut c_void;
    pub fn wxFileName_GetLongPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetModificationTime(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPath(self_: *const c_void, flags: c_int, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathWithSep(self_: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_GetShortPath(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetSize(self_: *const c_void) -> wxULongLong;
    pub fn wxFileName_GetTimes(
        self_: *const c_void,
        dt_access: *mut c_void,
        dt_mod: *mut c_void,
        dt_create: *mut c_void,
    ) -> bool;
    pub fn wxFileName_GetVolume(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_HasExt(self_: *const c_void) -> bool;
    pub fn wxFileName_HasName(self_: *const c_void) -> bool;
    pub fn wxFileName_HasVolume(self_: *const c_void) -> bool;
    pub fn wxFileName_InsertDir(self_: *mut c_void, before: usize, dir: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsAbsolute(self_: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_IsDir(self_: *const c_void) -> bool;
    pub fn wxFileName_IsDirReadable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsDirWritable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsFileExecutable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsFileReadable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsFileWritable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsRelative(self_: *const c_void, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_MakeAbsolute(self_: *mut c_void, cwd: *const c_void, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_MakeRelativeTo(self_: *mut c_void, path_base: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_Mkdir(self_: *const c_void, perm: c_int, flags: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_Normalize(self_: *mut c_void, flags: c_int, cwd: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_PrependDir(self_: *mut c_void, dir: *const c_void);
    pub fn wxFileName_RemoveDir(self_: *mut c_void, pos: usize);
    pub fn wxFileName_RemoveLastDir(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxFileName_ReplaceEnvVariable(self_: *mut c_void, envname: *const c_void, replacement_fmt_string: *const c_void, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_ReplaceHomeDir(self_: *mut c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_ResolveLink(self_: *mut c_void) -> *mut c_void;
    pub fn wxFileName_Rmdir(self_: *const c_void, flags: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_SameAs(self_: *const c_void, filepath: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_SetCwd(self_: *const c_void) -> bool;
    pub fn wxFileName_SetEmptyExt(self_: *mut c_void);
    pub fn wxFileName_SetExt(self_: *mut c_void, ext: *const c_void);
    pub fn wxFileName_SetFullName(self_: *mut c_void, fullname: *const c_void);
    pub fn wxFileName_SetName(self_: *mut c_void, name: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileName_SetPath(self_: *mut c_void, path: *const c_void, format: wxPathFormat);
    pub fn wxFileName_SetPermissions(self_: *mut c_void, permissions: c_int) -> bool;
    pub fn wxFileName_SetTimes(
        self_: *const c_void,
        dt_access: *const c_void,
        dt_mod: *const c_void,
        dt_create: *const c_void,
    ) -> bool;
    pub fn wxFileName_SetVolume(self_: *mut c_void, volume: *const c_void);
    pub fn wxFileName_ShouldFollowLink(self_: *const c_void) -> bool;
    pub fn wxFileName_Touch(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator!=(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator!=1(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator==(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator==1(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator=(self_: *mut c_void, filename: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxFileName_operator=1(self_: *mut c_void, filename: *const c_void) -> *mut c_void;
    pub fn wxFileName_CreateTempFileName(
        prefix: *const c_void,
        file_temp: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFileName_CreateTempFileName1(
        prefix: *const c_void,
        file_temp: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFileName_DirExists1(dir: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_DirName(dir: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_Exists1(path: *const c_void, flags: c_int) -> bool;
    pub fn wxFileName_FileExists1(file: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_FileName(file: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_GetCwd(volume: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetForbiddenChars(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetFormat(format: wxPathFormat) -> wxPathFormat;
    pub fn wxFileName_GetHomeDir() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetHumanReadableSize1(bytes: *const c_void, nullsize: *const c_void, precision: c_int, conv: wxSizeConvention) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathSeparator(format: wxPathFormat) -> wxUniChar;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathSeparators(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathTerminators(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetSize1(filename: *const c_void) -> wxULongLong;
    pub fn wxFileName_GetTempDir() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetVolumeSeparator(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetVolumeString(drive: char, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_IsCaseSensitive(format: wxPathFormat) -> bool;
    pub fn wxFileName_IsDirReadable1(dir: *const c_void) -> bool;
    pub fn wxFileName_IsDirWritable1(dir: *const c_void) -> bool;
    pub fn wxFileName_IsFileExecutable1(file: *const c_void) -> bool;
    pub fn wxFileName_IsFileReadable1(file: *const c_void) -> bool;
    pub fn wxFileName_IsFileWritable1(file: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsPathSeparator(ch: wxChar, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsMSWUniqueVolumeNamePath(path: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_Mkdir1(dir: *const c_void, perm: c_int, flags: c_int) -> bool;
    pub fn wxFileName_Rmdir1(dir: *const c_void, flags: c_int) -> bool;
    pub fn wxFileName_SetCwd1(cwd: *const c_void) -> bool;
    pub fn wxFileName_URLToFileName(url: *const c_void) -> *mut c_void;
    pub fn wxFileName_FileNameToURL(filename: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_SplitPath(fullpath: *const c_void, volume: *mut c_void, path: *mut c_void, name: *mut c_void, ext: *mut c_void, has_ext: *mut c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_SplitPath1(fullpath: *const c_void, volume: *mut c_void, path: *mut c_void, name: *mut c_void, ext: *mut c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_SplitPath2(fullpath: *const c_void, path: *mut c_void, name: *mut c_void, ext: *mut c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_SplitVolume(fullpath: *const c_void, volume: *mut c_void, path: *mut c_void, format: wxPathFormat);
    pub fn wxFileName_StripExtension(fullname: *const c_void) -> *mut c_void;

}
