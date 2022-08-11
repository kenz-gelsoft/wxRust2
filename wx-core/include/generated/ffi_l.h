#pragma once
#include <wx/wx.h>
#include <wx/listbox.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxListBox
wxClassInfo *wxListBox_CLASSINFO();
wxListBox *wxListBox_new();
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
void wxListBox_Deselect(wxListBox * self, int n);
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select);
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s);
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections);
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point);
int wxListBox_HitTest1(const wxListBox * self, int x, int y);
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos);
bool wxListBox_IsSelected(const wxListBox * self, int n);
void wxListBox_SetFirstItem(wxListBox * self, int n);
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string);
void wxListBox_EnsureVisible(wxListBox * self, int n);
bool wxListBox_IsSorted(const wxListBox * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self);
int wxListBox_GetTopItem(const wxListBox * self);
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj);

} // extern "C"

