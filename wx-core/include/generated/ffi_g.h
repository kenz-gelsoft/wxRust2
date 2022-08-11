#pragma once
#include <wx/wx.h>
#include <wx/dirctrl.h>
#include <wx/gauge.h>
#include <wx/gdiobj.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxGDIObject
wxClassInfo *wxGDIObject_CLASSINFO();

// CLASS: wxGauge
wxClassInfo *wxGauge_CLASSINFO();
wxGauge *wxGauge_new();
wxGauge *wxGauge_new1(wxWindow * parent, wxWindowID id, int range, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxGauge_Create(wxGauge * self, wxWindow * parent, wxWindowID id, int range, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxGauge_GetRange(const wxGauge * self);
int wxGauge_GetValue(const wxGauge * self);
bool wxGauge_IsVertical(const wxGauge * self);
void wxGauge_Pulse(wxGauge * self);
void wxGauge_SetRange(wxGauge * self, int range);
void wxGauge_SetValue(wxGauge * self, int pos);

// CLASS: wxGenericDirCtrl
wxClassInfo *wxGenericDirCtrl_CLASSINFO();
wxGenericDirCtrl *wxGenericDirCtrl_new();
wxGenericDirCtrl *wxGenericDirCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * dir, const wxPoint * pos, const wxSize * size, long style, const wxString * filter, int default_filter, const wxString * name);
bool wxGenericDirCtrl_CollapsePath(wxGenericDirCtrl * self, const wxString * path);
void wxGenericDirCtrl_CollapseTree(wxGenericDirCtrl * self);
bool wxGenericDirCtrl_Create(wxGenericDirCtrl * self, wxWindow * parent, wxWindowID id, const wxString * dir, const wxPoint * pos, const wxSize * size, long style, const wxString * filter, int default_filter, const wxString * name);
bool wxGenericDirCtrl_ExpandPath(wxGenericDirCtrl * self, const wxString * path);
wxString *wxGenericDirCtrl_GetDefaultPath(const wxGenericDirCtrl * self);
wxString *wxGenericDirCtrl_GetFilePath(const wxGenericDirCtrl * self);
void wxGenericDirCtrl_GetFilePaths(const wxGenericDirCtrl * self, wxArrayString * paths);
wxString *wxGenericDirCtrl_GetFilter(const wxGenericDirCtrl * self);
int wxGenericDirCtrl_GetFilterIndex(const wxGenericDirCtrl * self);
wxDirFilterListCtrl * wxGenericDirCtrl_GetFilterListCtrl(const wxGenericDirCtrl * self);
wxString *wxGenericDirCtrl_GetPath(const wxGenericDirCtrl * self);
void wxGenericDirCtrl_GetPaths(const wxGenericDirCtrl * self, wxArrayString * paths);
wxTreeCtrl * wxGenericDirCtrl_GetTreeCtrl(const wxGenericDirCtrl * self);
void wxGenericDirCtrl_Init(wxGenericDirCtrl * self);
void wxGenericDirCtrl_ReCreateTree(wxGenericDirCtrl * self);
void wxGenericDirCtrl_SetDefaultPath(wxGenericDirCtrl * self, const wxString * path);
void wxGenericDirCtrl_SetFilter(wxGenericDirCtrl * self, const wxString * filter);
void wxGenericDirCtrl_SetFilterIndex(wxGenericDirCtrl * self, int n);
void wxGenericDirCtrl_SetPath(wxGenericDirCtrl * self, const wxString * path);
void wxGenericDirCtrl_ShowHidden(wxGenericDirCtrl * self, bool show);
void wxGenericDirCtrl_SelectPath(wxGenericDirCtrl * self, const wxString * path, bool select);
void wxGenericDirCtrl_SelectPaths(wxGenericDirCtrl * self, const wxArrayString * paths);
void wxGenericDirCtrl_UnselectAll(wxGenericDirCtrl * self);

} // extern "C"

