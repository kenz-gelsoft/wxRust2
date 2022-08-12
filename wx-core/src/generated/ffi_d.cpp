#include "generated.h"

extern "C" {

// CLASS: wxDatePickerCtrl
wxClassInfo *wxDatePickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDatePickerCtrl);
}
wxDatePickerCtrl *wxDatePickerCtrl_new() {
    return new wxDatePickerCtrl();
}
wxDatePickerCtrl *wxDatePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxDatePickerCtrl(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxDatePickerCtrl_Create(wxDatePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxDatePickerCtrl_GetRange(const wxDatePickerCtrl * self, wxDateTime * dt1, wxDateTime * dt2) {
    return self->GetRange(dt1, dt2);
}
wxDateTime *wxDatePickerCtrl_GetValue(const wxDatePickerCtrl * self) {
    return new wxDateTime(self->GetValue());
}
#if wxCHECK_VERSION(3, 1, 0)
void wxDatePickerCtrl_SetNullText(wxDatePickerCtrl * self, const wxString * text) {
    return self->SetNullText(*text);
}
#endif
void wxDatePickerCtrl_SetRange(wxDatePickerCtrl * self, const wxDateTime * dt1, const wxDateTime * dt2) {
    return self->SetRange(*dt1, *dt2);
}
void wxDatePickerCtrl_SetValue(wxDatePickerCtrl * self, const wxDateTime * dt) {
    return self->SetValue(*dt);
}

// CLASS: wxDirPickerCtrl
wxClassInfo *wxDirPickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDirPickerCtrl);
}
wxDirPickerCtrl *wxDirPickerCtrl_new() {
    return new wxDirPickerCtrl();
}
wxDirPickerCtrl *wxDirPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxDirPickerCtrl(parent, id, *path, *message, *pos, *size, style, *validator, *name);
}
bool wxDirPickerCtrl_Create(wxDirPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *path, *message, *pos, *size, style, *validator, *name);
}
wxFileName *wxDirPickerCtrl_GetDirName(const wxDirPickerCtrl * self) {
    return new wxFileName(self->GetDirName());
}
wxString *wxDirPickerCtrl_GetPath(const wxDirPickerCtrl * self) {
    return new wxString(self->GetPath());
}
void wxDirPickerCtrl_SetDirName(wxDirPickerCtrl * self, const wxFileName * dirname) {
    return self->SetDirName(*dirname);
}
void wxDirPickerCtrl_SetInitialDirectory(wxDirPickerCtrl * self, const wxString * dir) {
    return self->SetInitialDirectory(*dir);
}
void wxDirPickerCtrl_SetPath(wxDirPickerCtrl * self, const wxString * dirname) {
    return self->SetPath(*dirname);
}

} // extern "C"

