#include "generated.h"

extern "C" {

// CLASS: wxFileName
void wxFileName_delete(wxFileName *self) {
    delete self;
}
wxFileName *wxFileName_new() {
    return new wxFileName();
}
wxFileName *wxFileName_new1(const wxFileName * filename) {
    return new wxFileName(*filename);
}
bool wxFileName_AppendDir(wxFileName * self, const wxString * dir) {
    return self->AppendDir(*dir);
}
void wxFileName_Assign(wxFileName * self, const wxFileName * filepath) {
    return self->Assign(*filepath);
}
void wxFileName_AssignCwd(wxFileName * self, const wxString * volume) {
    return self->AssignCwd(*volume);
}
void wxFileName_AssignHomeDir(wxFileName * self) {
    return self->AssignHomeDir();
}
void wxFileName_AssignTempFileName(wxFileName * self, const wxString * prefix) {
    return self->AssignTempFileName(*prefix);
}
void wxFileName_AssignTempFileName1(wxFileName * self, const wxString * prefix, wxFile * file_temp) {
    return self->AssignTempFileName(*prefix, file_temp);
}
void wxFileName_AssignTempFileName2(wxFileName * self, const wxString * prefix, wxFFile * file_temp) {
    return self->AssignTempFileName(*prefix, file_temp);
}
void wxFileName_Clear(wxFileName * self) {
    return self->Clear();
}
void wxFileName_ClearExt(wxFileName * self) {
    return self->ClearExt();
}
bool wxFileName_DirExists(const wxFileName * self) {
    return self->DirExists();
}
void wxFileName_DontFollowLink(wxFileName * self) {
    return self->DontFollowLink();
}
bool wxFileName_Exists(const wxFileName * self, int flags) {
    return self->Exists(flags);
}
bool wxFileName_FileExists(const wxFileName * self) {
    return self->FileExists();
}
size_t wxFileName_GetDirCount(const wxFileName * self) {
    return self->GetDirCount();
}
wxArrayString *wxFileName_GetDirs(const wxFileName * self) {
    return new wxArrayString(self->GetDirs());
}
wxString *wxFileName_GetExt(const wxFileName * self) {
    return new wxString(self->GetExt());
}
wxString *wxFileName_GetFullName(const wxFileName * self) {
    return new wxString(self->GetFullName());
}
wxString *wxFileName_GetLongPath(const wxFileName * self) {
    return new wxString(self->GetLongPath());
}
wxDateTime *wxFileName_GetModificationTime(const wxFileName * self) {
    return new wxDateTime(self->GetModificationTime());
}
wxString *wxFileName_GetName(const wxFileName * self) {
    return new wxString(self->GetName());
}
wxString *wxFileName_GetShortPath(const wxFileName * self) {
    return new wxString(self->GetShortPath());
}
bool wxFileName_GetTimes(const wxFileName * self, wxDateTime * dt_access, wxDateTime * dt_mod, wxDateTime * dt_create) {
    return self->GetTimes(dt_access, dt_mod, dt_create);
}
wxString *wxFileName_GetVolume(const wxFileName * self) {
    return new wxString(self->GetVolume());
}
bool wxFileName_HasExt(const wxFileName * self) {
    return self->HasExt();
}
bool wxFileName_HasName(const wxFileName * self) {
    return self->HasName();
}
bool wxFileName_HasVolume(const wxFileName * self) {
    return self->HasVolume();
}
bool wxFileName_InsertDir(wxFileName * self, size_t before, const wxString * dir) {
    return self->InsertDir(before, *dir);
}
bool wxFileName_IsDir(const wxFileName * self) {
    return self->IsDir();
}
bool wxFileName_IsDirReadable(const wxFileName * self) {
    return self->IsDirReadable();
}
bool wxFileName_IsDirWritable(const wxFileName * self) {
    return self->IsDirWritable();
}
bool wxFileName_IsFileExecutable(const wxFileName * self) {
    return self->IsFileExecutable();
}
bool wxFileName_IsFileReadable(const wxFileName * self) {
    return self->IsFileReadable();
}
bool wxFileName_IsFileWritable(const wxFileName * self) {
    return self->IsFileWritable();
}
bool wxFileName_IsOk(const wxFileName * self) {
    return self->IsOk();
}
bool wxFileName_Mkdir(const wxFileName * self, int perm, int flags) {
    return self->Mkdir(perm, flags);
}
void wxFileName_PrependDir(wxFileName * self, const wxString * dir) {
    return self->PrependDir(*dir);
}
void wxFileName_RemoveDir(wxFileName * self, size_t pos) {
    return self->RemoveDir(pos);
}
void wxFileName_RemoveLastDir(wxFileName * self) {
    return self->RemoveLastDir();
}
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_ResolveLink(wxFileName * self) {
    return new wxFileName(self->ResolveLink());
}
#endif
bool wxFileName_Rmdir(const wxFileName * self, int flags) {
    return self->Rmdir(flags);
}
bool wxFileName_SetCwd(const wxFileName * self) {
    return self->SetCwd();
}
void wxFileName_SetEmptyExt(wxFileName * self) {
    return self->SetEmptyExt();
}
void wxFileName_SetExt(wxFileName * self, const wxString * ext) {
    return self->SetExt(*ext);
}
void wxFileName_SetFullName(wxFileName * self, const wxString * fullname) {
    return self->SetFullName(*fullname);
}
void wxFileName_SetName(wxFileName * self, const wxString * name) {
    return self->SetName(*name);
}
bool wxFileName_SetPermissions(wxFileName * self, int permissions) {
    return self->SetPermissions(permissions);
}
bool wxFileName_SetTimes(const wxFileName * self, const wxDateTime * dt_access, const wxDateTime * dt_mod, const wxDateTime * dt_create) {
    return self->SetTimes(dt_access, dt_mod, dt_create);
}
void wxFileName_SetVolume(wxFileName * self, const wxString * volume) {
    return self->SetVolume(*volume);
}
bool wxFileName_ShouldFollowLink(const wxFileName * self) {
    return self->ShouldFollowLink();
}
bool wxFileName_Touch(const wxFileName * self) {
    return self->Touch();
}
wxString *wxFileName_CreateTempFileName(const wxString * prefix, wxFile * file_temp) {
    return new wxString(wxFileName::CreateTempFileName(*prefix, file_temp));
}
wxString *wxFileName_CreateTempFileName1(const wxString * prefix, wxFFile * file_temp) {
    return new wxString(wxFileName::CreateTempFileName(*prefix, file_temp));
}
bool wxFileName_DirExists1(const wxString * dir) {
    return wxFileName::DirExists(*dir);
}
bool wxFileName_Exists1(const wxString * path, int flags) {
    return wxFileName::Exists(*path, flags);
}
bool wxFileName_FileExists1(const wxString * file) {
    return wxFileName::FileExists(*file);
}
wxString *wxFileName_GetCwd(const wxString * volume) {
    return new wxString(wxFileName::GetCwd(*volume));
}
wxString *wxFileName_GetHomeDir() {
    return new wxString(wxFileName::GetHomeDir());
}
wxString *wxFileName_GetTempDir() {
    return new wxString(wxFileName::GetTempDir());
}
bool wxFileName_IsDirReadable1(const wxString * dir) {
    return wxFileName::IsDirReadable(*dir);
}
bool wxFileName_IsDirWritable1(const wxString * dir) {
    return wxFileName::IsDirWritable(*dir);
}
bool wxFileName_IsFileExecutable1(const wxString * file) {
    return wxFileName::IsFileExecutable(*file);
}
bool wxFileName_IsFileReadable1(const wxString * file) {
    return wxFileName::IsFileReadable(*file);
}
bool wxFileName_IsFileWritable1(const wxString * file) {
    return wxFileName::IsFileWritable(*file);
}
bool wxFileName_Mkdir1(const wxString * dir, int perm, int flags) {
    return wxFileName::Mkdir(*dir, perm, flags);
}
bool wxFileName_Rmdir1(const wxString * dir, int flags) {
    return wxFileName::Rmdir(*dir, flags);
}
bool wxFileName_SetCwd1(const wxString * cwd) {
    return wxFileName::SetCwd(*cwd);
}
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_URLToFileName(const wxString * url) {
    return new wxFileName(wxFileName::URLToFileName(*url));
}
wxString *wxFileName_FileNameToURL(const wxFileName * filename) {
    return new wxString(wxFileName::FileNameToURL(*filename));
}
#endif
wxString *wxFileName_StripExtension(const wxString * fullname) {
    return new wxString(wxFileName::StripExtension(*fullname));
}

} // extern "C"

