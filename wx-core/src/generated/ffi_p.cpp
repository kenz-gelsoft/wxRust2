#include "generated.h"

extern "C" {

// CLASS: wxPanel
wxClassInfo *wxPanel_CLASSINFO() {
    return wxCLASSINFO(wxPanel);
}
wxPanel *wxPanel_new() {
    return new wxPanel();
}
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPanel(parent, id, *pos, *size, style, *name);
}
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event) {
    return self->OnSysColourChanged(*event);
}
void wxPanel_SetFocusIgnoringChildren(wxPanel * self) {
    return self->SetFocusIgnoringChildren();
}

// CLASS: wxPickerBase
wxClassInfo *wxPickerBase_CLASSINFO() {
    return wxCLASSINFO(wxPickerBase);
}
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->CreateBase(parent, id, *text, *pos, *size, style, *validator, *name);
}
int wxPickerBase_GetInternalMargin(const wxPickerBase * self) {
    return self->GetInternalMargin();
}
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self) {
    return self->GetPickerCtrlProportion();
}
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self) {
    return self->GetTextCtrl();
}
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self) {
    return self->GetPickerCtrl();
}
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self) {
    return self->GetTextCtrlProportion();
}
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self) {
    return self->HasTextCtrl();
}
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self) {
    return self->IsPickerCtrlGrowable();
}
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self) {
    return self->IsTextCtrlGrowable();
}
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin) {
    return self->SetInternalMargin(margin);
}
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow) {
    return self->SetPickerCtrlGrowable(grow);
}
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop) {
    return self->SetPickerCtrlProportion(prop);
}
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow) {
    return self->SetTextCtrlGrowable(grow);
}
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop) {
    return self->SetTextCtrlProportion(prop);
}
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text) {
    return self->SetTextCtrl(text);
}
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker) {
    return self->SetPickerCtrl(picker);
}
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self) {
    return self->UpdatePickerFromTextCtrl();
}
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self) {
    return self->UpdateTextCtrlFromPicker();
}

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self) {
    delete self;
}
bool wxPoint_IsFullySpecified(const wxPoint * self) {
    return self->IsFullySpecified();
}
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt) {
    return self->SetDefaults(*pt);
}
wxPoint *wxPoint_new() {
    return new wxPoint();
}
wxPoint *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
wxPoint *wxPoint_new2(const wxRealPoint * pt) {
    return new wxPoint(*pt);
}

} // extern "C"

