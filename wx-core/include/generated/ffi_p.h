#pragma once
#include <wx/wx.h>
#include <wx/gdicmn.h>
#include <wx/panel.h>
#include <wx/pickerbase.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxPanel
wxClassInfo *wxPanel_CLASSINFO();
wxPanel *wxPanel_new();
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event);
void wxPanel_SetFocusIgnoringChildren(wxPanel * self);

// CLASS: wxPickerBase
wxClassInfo *wxPickerBase_CLASSINFO();
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxPickerBase_GetInternalMargin(const wxPickerBase * self);
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self);
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self);
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self);
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self);
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self);
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self);
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self);
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin);
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text);
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker);
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self);
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self);

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self);
bool wxPoint_IsFullySpecified(const wxPoint * self);
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt);
wxPoint *wxPoint_new();
wxPoint *wxPoint_new1(int x, int y);
wxPoint *wxPoint_new2(const wxRealPoint * pt);

} // extern "C"

