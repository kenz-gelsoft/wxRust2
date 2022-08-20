#include "generated.h"

extern "C" {

// CLASS: wxNativeFontInfo
void wxNativeFontInfo_delete(wxNativeFontInfo *self) {
    delete self;
}
wxNativeFontInfo *wxNativeFontInfo_new() {
    return new wxNativeFontInfo();
}
wxNativeFontInfo *wxNativeFontInfo_new1(const wxNativeFontInfo * info) {
    return new wxNativeFontInfo(*info);
}
void wxNativeFontInfo_Init(wxNativeFontInfo * self) {
    return self->Init();
}
void wxNativeFontInfo_InitFromFont(wxNativeFontInfo * self, const wxFont * font) {
    return self->InitFromFont(*font);
}
int wxNativeFontInfo_GetPointSize(const wxNativeFontInfo * self) {
    return self->GetPointSize();
}
wxSize *wxNativeFontInfo_GetPixelSize(const wxNativeFontInfo * self) {
    return new wxSize(self->GetPixelSize());
}
int wxNativeFontInfo_GetNumericWeight(const wxNativeFontInfo * self) {
    return self->GetNumericWeight();
}
bool wxNativeFontInfo_GetUnderlined(const wxNativeFontInfo * self) {
    return self->GetUnderlined();
}
wxString *wxNativeFontInfo_GetFaceName(const wxNativeFontInfo * self) {
    return new wxString(self->GetFaceName());
}
void wxNativeFontInfo_SetPointSize(wxNativeFontInfo * self, int pointsize) {
    return self->SetPointSize(pointsize);
}
void wxNativeFontInfo_SetPixelSize(wxNativeFontInfo * self, const wxSize * pixel_size) {
    return self->SetPixelSize(*pixel_size);
}
void wxNativeFontInfo_SetNumericWeight(wxNativeFontInfo * self, int weight) {
    return self->SetNumericWeight(weight);
}
void wxNativeFontInfo_SetUnderlined(wxNativeFontInfo * self, bool underlined) {
    return self->SetUnderlined(underlined);
}
bool wxNativeFontInfo_SetFaceName(wxNativeFontInfo * self, const wxString * facename) {
    return self->SetFaceName(*facename);
}
void wxNativeFontInfo_SetFaceName1(wxNativeFontInfo * self, const wxArrayString * facenames) {
    return self->SetFaceName(*facenames);
}
bool wxNativeFontInfo_FromString(wxNativeFontInfo * self, const wxString * s) {
    return self->FromString(*s);
}
wxString *wxNativeFontInfo_ToString(const wxNativeFontInfo * self) {
    return new wxString(self->ToString());
}
bool wxNativeFontInfo_FromUserString(wxNativeFontInfo * self, const wxString * s) {
    return self->FromUserString(*s);
}
wxString *wxNativeFontInfo_ToUserString(const wxNativeFontInfo * self) {
    return new wxString(self->ToUserString());
}

// CLASS: wxNativeWindow
wxClassInfo *wxNativeWindow_CLASSINFO() {
    return wxCLASSINFO(wxNativeWindow);
}
wxNativeWindow *wxNativeWindow_new() {
    return new wxNativeWindow();
}
void wxNativeWindow_Disown(wxNativeWindow * self) {
    return self->Disown();
}

// CLASS: wxNavigationKeyEvent
wxClassInfo *wxNavigationKeyEvent_CLASSINFO() {
    return wxCLASSINFO(wxNavigationKeyEvent);
}
wxNavigationKeyEvent *wxNavigationKeyEvent_new() {
    return new wxNavigationKeyEvent();
}
wxNavigationKeyEvent *wxNavigationKeyEvent_new1(const wxNavigationKeyEvent * event) {
    return new wxNavigationKeyEvent(*event);
}
wxWindow * wxNavigationKeyEvent_GetCurrentFocus(const wxNavigationKeyEvent * self) {
    return self->GetCurrentFocus();
}
bool wxNavigationKeyEvent_GetDirection(const wxNavigationKeyEvent * self) {
    return self->GetDirection();
}
bool wxNavigationKeyEvent_IsFromTab(const wxNavigationKeyEvent * self) {
    return self->IsFromTab();
}
bool wxNavigationKeyEvent_IsWindowChange(const wxNavigationKeyEvent * self) {
    return self->IsWindowChange();
}
void wxNavigationKeyEvent_SetCurrentFocus(wxNavigationKeyEvent * self, wxWindow * current_focus) {
    return self->SetCurrentFocus(current_focus);
}
void wxNavigationKeyEvent_SetDirection(wxNavigationKeyEvent * self, bool direction) {
    return self->SetDirection(direction);
}
void wxNavigationKeyEvent_SetFlags(wxNavigationKeyEvent * self, long flags) {
    return self->SetFlags(flags);
}
void wxNavigationKeyEvent_SetFromTab(wxNavigationKeyEvent * self, bool from_tab) {
    return self->SetFromTab(from_tab);
}
void wxNavigationKeyEvent_SetWindowChange(wxNavigationKeyEvent * self, bool window_change) {
    return self->SetWindowChange(window_change);
}

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

// CLASS: wxNotificationMessage
wxClassInfo *wxNotificationMessage_CLASSINFO() {
    return wxCLASSINFO(wxNotificationMessage);
}
wxNotificationMessage *wxNotificationMessage_new() {
    return new wxNotificationMessage();
}
wxNotificationMessage *wxNotificationMessage_new1(const wxString * title, const wxString * message, wxWindow * parent, int flags) {
    return new wxNotificationMessage(*title, *message, parent, flags);
}
bool wxNotificationMessage_AddAction(wxNotificationMessage * self, wxWindowID actionid, const wxString * label) {
    return self->AddAction(actionid, *label);
}
bool wxNotificationMessage_Close(wxNotificationMessage * self) {
    return self->Close();
}
void wxNotificationMessage_SetFlags(wxNotificationMessage * self, int flags) {
    return self->SetFlags(flags);
}
void wxNotificationMessage_SetIcon(wxNotificationMessage * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxNotificationMessage_SetMessage(wxNotificationMessage * self, const wxString * message) {
    return self->SetMessage(*message);
}
void wxNotificationMessage_SetParent(wxNotificationMessage * self, wxWindow * parent) {
    return self->SetParent(parent);
}
void wxNotificationMessage_SetTitle(wxNotificationMessage * self, const wxString * title) {
    return self->SetTitle(*title);
}
bool wxNotificationMessage_Show(wxNotificationMessage * self, int timeout) {
    return self->Show(timeout);
}
#ifdef __WXMSW__
wxTaskBarIcon * wxNotificationMessage_UseTaskBarIcon(wxTaskBarIcon * icon) {
    return wxNotificationMessage::UseTaskBarIcon(icon);
}
bool wxNotificationMessage_MSWUseToasts(const wxString * shortcut_path, const wxString * app_id) {
    return wxNotificationMessage::MSWUseToasts(*shortcut_path, *app_id);
}
#endif

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

// CLASS: wxNumberEntryDialog
wxClassInfo *wxNumberEntryDialog_CLASSINFO() {
    return wxCLASSINFO(wxNumberEntryDialog);
}
wxNumberEntryDialog *wxNumberEntryDialog_new() {
    return new wxNumberEntryDialog();
}
wxNumberEntryDialog *wxNumberEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos) {
    return new wxNumberEntryDialog(parent, *message, *prompt, *caption, value, min, max, *pos);
}
bool wxNumberEntryDialog_Create(wxNumberEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos) {
    return self->Create(parent, *message, *prompt, *caption, value, min, max, *pos);
}
long wxNumberEntryDialog_GetValue(const wxNumberEntryDialog * self) {
    return self->GetValue();
}

} // extern "C"

