#include "generated.h"

extern "C" {

// CLASS: wxUIActionSimulator
void wxUIActionSimulator_delete(wxUIActionSimulator *self) {
    delete self;
}
wxUIActionSimulator *wxUIActionSimulator_new() {
    return new wxUIActionSimulator();
}
bool wxUIActionSimulator_MouseMove(wxUIActionSimulator * self, long x, long y) {
    return self->MouseMove(x, y);
}
bool wxUIActionSimulator_MouseMove1(wxUIActionSimulator * self, const wxPoint * point) {
    return self->MouseMove(*point);
}
bool wxUIActionSimulator_MouseDown(wxUIActionSimulator * self, int button) {
    return self->MouseDown(button);
}
bool wxUIActionSimulator_MouseUp(wxUIActionSimulator * self, int button) {
    return self->MouseUp(button);
}
bool wxUIActionSimulator_MouseClick(wxUIActionSimulator * self, int button) {
    return self->MouseClick(button);
}
bool wxUIActionSimulator_MouseDblClick(wxUIActionSimulator * self, int button) {
    return self->MouseDblClick(button);
}
bool wxUIActionSimulator_MouseDragDrop(wxUIActionSimulator * self, long x1, long y1, long x2, long y2, int button) {
    return self->MouseDragDrop(x1, y1, x2, y2, button);
}
bool wxUIActionSimulator_KeyDown(wxUIActionSimulator * self, int keycode, int modifiers) {
    return self->KeyDown(keycode, modifiers);
}
bool wxUIActionSimulator_KeyUp(wxUIActionSimulator * self, int keycode, int modifiers) {
    return self->KeyUp(keycode, modifiers);
}
bool wxUIActionSimulator_Char(wxUIActionSimulator * self, int keycode, int modifiers) {
    return self->Char(keycode, modifiers);
}
bool wxUIActionSimulator_Select(wxUIActionSimulator * self, const wxString * text) {
    return self->Select(*text);
}
bool wxUIActionSimulator_Text(wxUIActionSimulator * self, const char * text) {
    return self->Text(text);
}

// CLASS: wxURLDataObject
void wxURLDataObject_delete(wxURLDataObject *self) {
    delete self;
}
wxURLDataObject *wxURLDataObject_new(const wxString * url) {
    return new wxURLDataObject(*url);
}
wxString *wxURLDataObject_GetURL(const wxURLDataObject * self) {
    return new wxString(self->GetURL());
}
void wxURLDataObject_SetURL(wxURLDataObject * self, const wxString * url) {
    return self->SetURL(*url);
}

// CLASS: wxUpdateUIEvent
wxClassInfo *wxUpdateUIEvent_CLASSINFO() {
    return wxCLASSINFO(wxUpdateUIEvent);
}
wxUpdateUIEvent *wxUpdateUIEvent_new(wxWindowID command_id) {
    return new wxUpdateUIEvent(command_id);
}
void wxUpdateUIEvent_Check(wxUpdateUIEvent * self, bool check) {
    return self->Check(check);
}
void wxUpdateUIEvent_Enable(wxUpdateUIEvent * self, bool enable) {
    return self->Enable(enable);
}
bool wxUpdateUIEvent_GetChecked(const wxUpdateUIEvent * self) {
    return self->GetChecked();
}
bool wxUpdateUIEvent_GetEnabled(const wxUpdateUIEvent * self) {
    return self->GetEnabled();
}
bool wxUpdateUIEvent_IsCheckable(const wxUpdateUIEvent * self) {
    return self->IsCheckable();
}
bool wxUpdateUIEvent_GetSetChecked(const wxUpdateUIEvent * self) {
    return self->GetSetChecked();
}
bool wxUpdateUIEvent_GetSetEnabled(const wxUpdateUIEvent * self) {
    return self->GetSetEnabled();
}
bool wxUpdateUIEvent_GetSetShown(const wxUpdateUIEvent * self) {
    return self->GetSetShown();
}
bool wxUpdateUIEvent_GetSetText(const wxUpdateUIEvent * self) {
    return self->GetSetText();
}
bool wxUpdateUIEvent_GetShown(const wxUpdateUIEvent * self) {
    return self->GetShown();
}
wxString *wxUpdateUIEvent_GetText(const wxUpdateUIEvent * self) {
    return new wxString(self->GetText());
}
void wxUpdateUIEvent_SetText(wxUpdateUIEvent * self, const wxString * text) {
    return self->SetText(*text);
}
void wxUpdateUIEvent_Show(wxUpdateUIEvent * self, bool show) {
    return self->Show(show);
}
bool wxUpdateUIEvent_CanUpdate(wxWindow * window) {
    return wxUpdateUIEvent::CanUpdate(window);
}
long wxUpdateUIEvent_GetUpdateInterval() {
    return wxUpdateUIEvent::GetUpdateInterval();
}
void wxUpdateUIEvent_ResetUpdateTime() {
    return wxUpdateUIEvent::ResetUpdateTime();
}
void wxUpdateUIEvent_SetUpdateInterval(long update_interval) {
    return wxUpdateUIEvent::SetUpdateInterval(update_interval);
}

} // extern "C"

