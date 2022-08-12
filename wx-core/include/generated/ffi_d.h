#pragma once

#include <wx/datectrl.h>
#include <wx/filepicker.h>

extern "C" {

// CLASS: wxDatePickerCtrl
wxClassInfo *wxDatePickerCtrl_CLASSINFO();
wxDatePickerCtrl *wxDatePickerCtrl_new();
wxDatePickerCtrl *wxDatePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_Create(wxDatePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_GetRange(const wxDatePickerCtrl * self, wxDateTime * dt1, wxDateTime * dt2);
wxDateTime *wxDatePickerCtrl_GetValue(const wxDatePickerCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxDatePickerCtrl_SetNullText(wxDatePickerCtrl * self, const wxString * text);
#endif
void wxDatePickerCtrl_SetRange(wxDatePickerCtrl * self, const wxDateTime * dt1, const wxDateTime * dt2);
void wxDatePickerCtrl_SetValue(wxDatePickerCtrl * self, const wxDateTime * dt);

// CLASS: wxDirPickerCtrl
wxClassInfo *wxDirPickerCtrl_CLASSINFO();
wxDirPickerCtrl *wxDirPickerCtrl_new();
wxDirPickerCtrl *wxDirPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDirPickerCtrl_Create(wxDirPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxFileName *wxDirPickerCtrl_GetDirName(const wxDirPickerCtrl * self);
wxString *wxDirPickerCtrl_GetPath(const wxDirPickerCtrl * self);
void wxDirPickerCtrl_SetDirName(wxDirPickerCtrl * self, const wxFileName * dirname);
void wxDirPickerCtrl_SetInitialDirectory(wxDirPickerCtrl * self, const wxString * dir);
void wxDirPickerCtrl_SetPath(wxDirPickerCtrl * self, const wxString * dirname);

} // extern "C"

