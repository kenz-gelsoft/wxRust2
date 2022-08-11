#pragma once
#include <wx/wx.h>
#include <wx/editlbox.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxEditableListBox
wxClassInfo *wxEditableListBox_CLASSINFO();
wxEditableListBox *wxEditableListBox_new();
wxEditableListBox *wxEditableListBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxEditableListBox_Create(wxEditableListBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxEditableListBox_SetStrings(wxEditableListBox * self, const wxArrayString * strings);
void wxEditableListBox_GetStrings(const wxEditableListBox * self, wxArrayString * strings);

} // extern "C"

