#pragma once

#include <wx/odcombo.h>
#include <wx/overlay.h>

extern "C" {

// CLASS: wxOverlay
void wxOverlay_delete(wxOverlay *self);
wxOverlay *wxOverlay_new();
void wxOverlay_Reset(wxOverlay * self);

// CLASS: wxOwnerDrawnComboBox
wxClassInfo *wxOwnerDrawnComboBox_CLASSINFO();
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new();
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxOwnerDrawnComboBox_Create(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxOwnerDrawnComboBox_Create2(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxOwnerDrawnComboBox_IsListEmpty(const wxOwnerDrawnComboBox * self);
bool wxOwnerDrawnComboBox_IsTextEmpty(const wxOwnerDrawnComboBox * self);
int wxOwnerDrawnComboBox_GetWidestItem(wxOwnerDrawnComboBox * self);
int wxOwnerDrawnComboBox_GetWidestItemWidth(wxOwnerDrawnComboBox * self);
// Mix-in(s) to wxOwnerDrawnComboBox
wxItemContainer *wxOwnerDrawnComboBox_AsItemContainer(wxOwnerDrawnComboBox* obj);
wxTextEntryBase *wxOwnerDrawnComboBox_AsTextEntry(wxOwnerDrawnComboBox* obj);

} // extern "C"

