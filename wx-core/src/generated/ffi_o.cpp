#include "generated.h"

extern "C" {

// CLASS: wxOverlay
void wxOverlay_delete(wxOverlay *self) {
    delete self;
}
wxOverlay *wxOverlay_new() {
    return new wxOverlay();
}
void wxOverlay_Reset(wxOverlay * self) {
    return self->Reset();
}

// CLASS: wxOwnerDrawnComboBox
wxClassInfo *wxOwnerDrawnComboBox_CLASSINFO() {
    return wxCLASSINFO(wxOwnerDrawnComboBox);
}
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new() {
    return new wxOwnerDrawnComboBox();
}
wxOwnerDrawnComboBox *wxOwnerDrawnComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxOwnerDrawnComboBox(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
bool wxOwnerDrawnComboBox_Create(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxOwnerDrawnComboBox_Create2(wxOwnerDrawnComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxOwnerDrawnComboBox_IsListEmpty(const wxOwnerDrawnComboBox * self) {
    return self->IsListEmpty();
}
bool wxOwnerDrawnComboBox_IsTextEmpty(const wxOwnerDrawnComboBox * self) {
    return self->IsTextEmpty();
}
#endif
int wxOwnerDrawnComboBox_GetWidestItem(wxOwnerDrawnComboBox * self) {
    return self->GetWidestItem();
}
int wxOwnerDrawnComboBox_GetWidestItemWidth(wxOwnerDrawnComboBox * self) {
    return self->GetWidestItemWidth();
}
// Mix-in(s) to wxOwnerDrawnComboBox
wxItemContainer *wxOwnerDrawnComboBox_AsItemContainer(wxOwnerDrawnComboBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTextEntryBase *wxOwnerDrawnComboBox_AsTextEntry(wxOwnerDrawnComboBox* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}

} // extern "C"

