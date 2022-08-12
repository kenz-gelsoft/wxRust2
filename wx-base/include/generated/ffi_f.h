#pragma once

#include <wx/filename.h>

extern "C" {

// CLASS: wxFileName
void wxFileName_delete(wxFileName *self);
wxFileName *wxFileName_new();
wxFileName *wxFileName_new1(const wxFileName * filename);
bool wxFileName_AppendDir(wxFileName * self, const wxString * dir);
void wxFileName_Assign(wxFileName * self, const wxFileName * filepath);
void wxFileName_AssignCwd(wxFileName * self, const wxString * volume);
void wxFileName_AssignHomeDir(wxFileName * self);
void wxFileName_AssignTempFileName(wxFileName * self, const wxString * prefix);
void wxFileName_AssignTempFileName1(wxFileName * self, const wxString * prefix, wxFile * file_temp);
void wxFileName_AssignTempFileName2(wxFileName * self, const wxString * prefix, wxFFile * file_temp);
void wxFileName_Clear(wxFileName * self);
void wxFileName_ClearExt(wxFileName * self);
bool wxFileName_DirExists(const wxFileName * self);
void wxFileName_DontFollowLink(wxFileName * self);
bool wxFileName_Exists(const wxFileName * self, int flags);
bool wxFileName_FileExists(const wxFileName * self);
size_t wxFileName_GetDirCount(const wxFileName * self);
wxArrayString *wxFileName_GetDirs(const wxFileName * self);
wxString *wxFileName_GetExt(const wxFileName * self);
wxString *wxFileName_GetFullName(const wxFileName * self);
wxString *wxFileName_GetLongPath(const wxFileName * self);
wxDateTime *wxFileName_GetModificationTime(const wxFileName * self);
wxString *wxFileName_GetName(const wxFileName * self);
wxString *wxFileName_GetShortPath(const wxFileName * self);
bool wxFileName_GetTimes(const wxFileName * self, wxDateTime * dt_access, wxDateTime * dt_mod, wxDateTime * dt_create);
wxString *wxFileName_GetVolume(const wxFileName * self);
bool wxFileName_HasExt(const wxFileName * self);
bool wxFileName_HasName(const wxFileName * self);
bool wxFileName_HasVolume(const wxFileName * self);
bool wxFileName_InsertDir(wxFileName * self, size_t before, const wxString * dir);
bool wxFileName_IsDir(const wxFileName * self);
bool wxFileName_IsDirReadable(const wxFileName * self);
bool wxFileName_IsDirWritable(const wxFileName * self);
bool wxFileName_IsFileExecutable(const wxFileName * self);
bool wxFileName_IsFileReadable(const wxFileName * self);
bool wxFileName_IsFileWritable(const wxFileName * self);
bool wxFileName_IsOk(const wxFileName * self);
bool wxFileName_Mkdir(const wxFileName * self, int perm, int flags);
void wxFileName_PrependDir(wxFileName * self, const wxString * dir);
void wxFileName_RemoveDir(wxFileName * self, size_t pos);
void wxFileName_RemoveLastDir(wxFileName * self);
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_ResolveLink(wxFileName * self);
#endif
bool wxFileName_Rmdir(const wxFileName * self, int flags);
bool wxFileName_SetCwd(const wxFileName * self);
void wxFileName_SetEmptyExt(wxFileName * self);
void wxFileName_SetExt(wxFileName * self, const wxString * ext);
void wxFileName_SetFullName(wxFileName * self, const wxString * fullname);
void wxFileName_SetName(wxFileName * self, const wxString * name);
bool wxFileName_SetPermissions(wxFileName * self, int permissions);
bool wxFileName_SetTimes(const wxFileName * self, const wxDateTime * dt_access, const wxDateTime * dt_mod, const wxDateTime * dt_create);
void wxFileName_SetVolume(wxFileName * self, const wxString * volume);
bool wxFileName_ShouldFollowLink(const wxFileName * self);
bool wxFileName_Touch(const wxFileName * self);
wxString *wxFileName_CreateTempFileName(const wxString * prefix, wxFile * file_temp);
wxString *wxFileName_CreateTempFileName1(const wxString * prefix, wxFFile * file_temp);
bool wxFileName_DirExists1(const wxString * dir);
bool wxFileName_Exists1(const wxString * path, int flags);
bool wxFileName_FileExists1(const wxString * file);
wxString *wxFileName_GetCwd(const wxString * volume);
wxString *wxFileName_GetHomeDir();
wxString *wxFileName_GetTempDir();
bool wxFileName_IsDirReadable1(const wxString * dir);
bool wxFileName_IsDirWritable1(const wxString * dir);
bool wxFileName_IsFileExecutable1(const wxString * file);
bool wxFileName_IsFileReadable1(const wxString * file);
bool wxFileName_IsFileWritable1(const wxString * file);
bool wxFileName_Mkdir1(const wxString * dir, int perm, int flags);
bool wxFileName_Rmdir1(const wxString * dir, int flags);
bool wxFileName_SetCwd1(const wxString * cwd);
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_URLToFileName(const wxString * url);
wxString *wxFileName_FileNameToURL(const wxFileName * filename);
#endif
wxString *wxFileName_StripExtension(const wxString * fullname);

} // extern "C"

