#include "generated.h"

extern "C" {

// CLASS: wxSharedClientDataContainer
void * wxSharedClientDataContainer_GetClientData(const wxSharedClientDataContainer * self) {
    return self->GetClientData();
}
wxClientData * wxSharedClientDataContainer_GetClientObject(const wxSharedClientDataContainer * self) {
    return self->GetClientObject();
}
void wxSharedClientDataContainer_SetClientData(wxSharedClientDataContainer * self, void * data) {
    return self->SetClientData(data);
}
void wxSharedClientDataContainer_SetClientObject(wxSharedClientDataContainer * self, wxClientData * data) {
    return self->SetClientObject(data);
}

// CLASS: wxStandardPaths
void wxStandardPaths_delete(wxStandardPaths *self) {
    delete self;
}
#ifdef __WXMSW__
void wxStandardPaths_DontIgnoreAppSubDir(wxStandardPaths * self) {
    return self->DontIgnoreAppSubDir();
}
#endif
wxString *wxStandardPaths_GetAppDocumentsDir(const wxStandardPaths * self) {
    return new wxString(self->GetAppDocumentsDir());
}
wxString *wxStandardPaths_GetConfigDir(const wxStandardPaths * self) {
    return new wxString(self->GetConfigDir());
}
wxString *wxStandardPaths_GetDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetDataDir());
}
wxString *wxStandardPaths_GetDocumentsDir(const wxStandardPaths * self) {
    return new wxString(self->GetDocumentsDir());
}
wxString *wxStandardPaths_GetExecutablePath(const wxStandardPaths * self) {
    return new wxString(self->GetExecutablePath());
}
#ifdef __WXGTK__
wxString *wxStandardPaths_GetInstallPrefix(const wxStandardPaths * self) {
    return new wxString(self->GetInstallPrefix());
}
#endif
wxString *wxStandardPaths_GetLocalDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetLocalDataDir());
}
wxString *wxStandardPaths_GetPluginsDir(const wxStandardPaths * self) {
    return new wxString(self->GetPluginsDir());
}
wxString *wxStandardPaths_GetResourcesDir(const wxStandardPaths * self) {
    return new wxString(self->GetResourcesDir());
}
wxString *wxStandardPaths_GetTempDir(const wxStandardPaths * self) {
    return new wxString(self->GetTempDir());
}
wxString *wxStandardPaths_GetUserConfigDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserConfigDir());
}
wxString *wxStandardPaths_GetUserDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserDataDir());
}
wxString *wxStandardPaths_GetUserLocalDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserLocalDataDir());
}
#ifdef __WXMSW__
void wxStandardPaths_IgnoreAppSubDir(wxStandardPaths * self, const wxString * subdir_pattern) {
    return self->IgnoreAppSubDir(*subdir_pattern);
}
void wxStandardPaths_IgnoreAppBuildSubDirs(wxStandardPaths * self) {
    return self->IgnoreAppBuildSubDirs();
}
#endif
#ifdef __WXGTK__
void wxStandardPaths_SetInstallPrefix(wxStandardPaths * self, const wxString * prefix) {
    return self->SetInstallPrefix(*prefix);
}
#endif
void wxStandardPaths_UseAppInfo(wxStandardPaths * self, int info) {
    return self->UseAppInfo(info);
}
wxStandardPaths * wxStandardPaths_Get() {
    return &(wxStandardPaths::Get());
}
#ifdef __WXMSW__
wxString *wxStandardPaths_MSWGetShellDir(int csidl) {
    return new wxString(wxStandardPaths::MSWGetShellDir(csidl));
}
#endif

} // extern "C"

