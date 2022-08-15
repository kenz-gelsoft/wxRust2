#include "generated.h"

extern "C" {

// CLASS: wxEditableListBox
wxClassInfo *wxEditableListBox_CLASSINFO() {
    return wxCLASSINFO(wxEditableListBox);
}
wxEditableListBox *wxEditableListBox_new() {
    return new wxEditableListBox();
}
wxEditableListBox *wxEditableListBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxEditableListBox(parent, id, *label, *pos, *size, style, *name);
}
bool wxEditableListBox_Create(wxEditableListBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
void wxEditableListBox_SetStrings(wxEditableListBox * self, const wxArrayString * strings) {
    return self->SetStrings(*strings);
}
void wxEditableListBox_GetStrings(const wxEditableListBox * self, wxArrayString * strings) {
    return self->GetStrings(*strings);
}

// CLASS: wxEraseEvent
wxClassInfo *wxEraseEvent_CLASSINFO() {
    return wxCLASSINFO(wxEraseEvent);
}
wxEraseEvent *wxEraseEvent_new(int id, wxDC * dc) {
    return new wxEraseEvent(id, dc);
}
wxDC * wxEraseEvent_GetDC(const wxEraseEvent * self) {
    return self->GetDC();
}

// CLASS: wxEventBlocker
wxClassInfo *wxEventBlocker_CLASSINFO() {
    return wxCLASSINFO(wxEventBlocker);
}

// CLASS: wxExtHelpController
wxClassInfo *wxExtHelpController_CLASSINFO() {
    return wxCLASSINFO(wxExtHelpController);
}
wxExtHelpController *wxExtHelpController_new(wxWindow * parent_window) {
    return new wxExtHelpController(parent_window);
}
bool wxExtHelpController_DisplayHelp(wxExtHelpController * self, const wxString * relative_url) {
    return self->DisplayHelp(*relative_url);
}

} // extern "C"

