#include "generated.h"

extern "C" {

// CLASS: wxNonOwnedWindow
wxClassInfo *wxNonOwnedWindow_CLASSINFO() {
    return wxCLASSINFO(wxNonOwnedWindow);
}
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region) {
    return self->SetShape(*region);
}
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path) {
    return self->SetShape(*path);
}

// CLASS: wxNotebook
wxClassInfo *wxNotebook_CLASSINFO() {
    return wxCLASSINFO(wxNotebook);
}
wxNotebook *wxNotebook_new() {
    return new wxNotebook();
}
wxNotebook *wxNotebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxNotebook(parent, id, *pos, *size, style, *name);
}
bool wxNotebook_Create(wxNotebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
int wxNotebook_GetRowCount(const wxNotebook * self) {
    return self->GetRowCount();
}
wxColour *wxNotebook_GetThemeBackgroundColour(const wxNotebook * self) {
    return new wxColour(self->GetThemeBackgroundColour());
}
void wxNotebook_SetPadding(wxNotebook * self, const wxSize * padding) {
    return self->SetPadding(*padding);
}

// CLASS: wxNotifyEvent
wxClassInfo *wxNotifyEvent_CLASSINFO() {
    return wxCLASSINFO(wxNotifyEvent);
}
void wxNotifyEvent_Allow(wxNotifyEvent * self) {
    return self->Allow();
}
bool wxNotifyEvent_IsAllowed(const wxNotifyEvent * self) {
    return self->IsAllowed();
}
void wxNotifyEvent_Veto(wxNotifyEvent * self) {
    return self->Veto();
}

} // extern "C"

