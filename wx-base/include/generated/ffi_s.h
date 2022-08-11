#pragma once
#include <wx/wx.h>
#include <wx/stdpaths.h>

typedef wxDateTime::TimeZone TimeZone;
typedef wxDateTime::Tm       Tm;
typedef wxDateTime::WeekDay  WeekDay;

extern "C" {

// CLASS: wxStandardPaths
void wxStandardPaths_delete(wxStandardPaths *self);
#ifdef __WXMSW__
void wxStandardPaths_DontIgnoreAppSubDir(wxStandardPaths * self);
#endif
wxString *wxStandardPaths_GetAppDocumentsDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetConfigDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetDataDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetDocumentsDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetExecutablePath(const wxStandardPaths * self);
#ifdef __WXGTK__
wxString *wxStandardPaths_GetInstallPrefix(const wxStandardPaths * self);
#endif
wxString *wxStandardPaths_GetLocalDataDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetPluginsDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetResourcesDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetTempDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetUserConfigDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetUserDataDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetUserLocalDataDir(const wxStandardPaths * self);
#ifdef __WXMSW__
void wxStandardPaths_IgnoreAppSubDir(wxStandardPaths * self, const wxString * subdir_pattern);
void wxStandardPaths_IgnoreAppBuildSubDirs(wxStandardPaths * self);
#endif
#ifdef __WXGTK__
void wxStandardPaths_SetInstallPrefix(wxStandardPaths * self, const wxString * prefix);
#endif
void wxStandardPaths_UseAppInfo(wxStandardPaths * self, int info);
wxStandardPaths * wxStandardPaths_Get();
#ifdef __WXMSW__
wxString *wxStandardPaths_MSWGetShellDir(int csidl);
#endif

} // extern "C"

