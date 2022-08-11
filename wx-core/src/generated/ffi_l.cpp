#include "generated.h"

extern "C" {

// CLASS: wxListBox
wxClassInfo *wxListBox_CLASSINFO() {
    return wxCLASSINFO(wxListBox);
}
wxListBox *wxListBox_new() {
    return new wxListBox();
}
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxListBox(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
void wxListBox_Deselect(wxListBox * self, int n) {
    return self->Deselect(n);
}
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select) {
    return self->SetStringSelection(*s, select);
}
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s) {
    return self->SetStringSelection(*s);
}
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections) {
    return self->GetSelections(*selections);
}
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point) {
    return self->HitTest(*point);
}
int wxListBox_HitTest1(const wxListBox * self, int x, int y) {
    return self->HitTest(x, y);
}
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos) {
    return self->InsertItems(*items, pos);
}
bool wxListBox_IsSelected(const wxListBox * self, int n) {
    return self->IsSelected(n);
}
void wxListBox_SetFirstItem(wxListBox * self, int n) {
    return self->SetFirstItem(n);
}
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string) {
    return self->SetFirstItem(*string);
}
void wxListBox_EnsureVisible(wxListBox * self, int n) {
    return self->EnsureVisible(n);
}
bool wxListBox_IsSorted(const wxListBox * self) {
    return self->IsSorted();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self) {
    return self->GetCountPerPage();
}
int wxListBox_GetTopItem(const wxListBox * self) {
    return self->GetTopItem();
}
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}

} // extern "C"

