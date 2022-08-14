#include "generated.h"

extern "C" {

// CLASS: wxDataObject
void wxDataObject_delete(wxDataObject *self) {
    delete self;
}
bool wxDataObject_GetDataHere(const wxDataObject * self, const wxDataFormat * format, void * buf) {
    return self->GetDataHere(*format, buf);
}
size_t wxDataObject_GetDataSize(const wxDataObject * self, const wxDataFormat * format) {
    return self->GetDataSize(*format);
}
bool wxDataObject_SetData(wxDataObject * self, const wxDataFormat * format, size_t len, const void * buf) {
    return self->SetData(*format, len, buf);
}

// CLASS: wxDataObjectSimple
void wxDataObjectSimple_delete(wxDataObjectSimple *self) {
    delete self;
}
wxDataObjectSimple *wxDataObjectSimple_new(const wxDataFormat * format) {
    return new wxDataObjectSimple(*format);
}
bool wxDataObjectSimple_GetDataHere(const wxDataObjectSimple * self, void * buf) {
    return self->GetDataHere(buf);
}
size_t wxDataObjectSimple_GetDataSize(const wxDataObjectSimple * self) {
    return self->GetDataSize();
}
bool wxDataObjectSimple_SetData(wxDataObjectSimple * self, size_t len, const void * buf) {
    return self->SetData(len, buf);
}
void wxDataObjectSimple_SetFormat(wxDataObjectSimple * self, const wxDataFormat * format) {
    return self->SetFormat(*format);
}

// CLASS: wxDateEvent
wxClassInfo *wxDateEvent_CLASSINFO() {
    return wxCLASSINFO(wxDateEvent);
}
wxDateEvent *wxDateEvent_new() {
    return new wxDateEvent();
}
wxDateTime *wxDateEvent_GetDate(const wxDateEvent * self) {
    return new wxDateTime(self->GetDate());
}
void wxDateEvent_SetDate(wxDateEvent * self, const wxDateTime * date) {
    return self->SetDate(*date);
}

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

// CLASS: wxDialog
wxClassInfo *wxDialog_CLASSINFO() {
    return wxCLASSINFO(wxDialog);
}
wxDialog *wxDialog_new() {
    return new wxDialog();
}
wxDialog *wxDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxDialog(parent, id, *title, *pos, *size, style, *name);
}
void wxDialog_AddMainButtonId(wxDialog * self, wxWindowID id) {
    return self->AddMainButtonId(id);
}
bool wxDialog_CanDoLayoutAdaptation(wxDialog * self) {
    return self->CanDoLayoutAdaptation();
}
void wxDialog_Centre(wxDialog * self, int direction) {
    return self->Centre(direction);
}
bool wxDialog_Create(wxDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxSizer * wxDialog_CreateButtonSizer(wxDialog * self, long flags) {
    return self->CreateButtonSizer(flags);
}
wxSizer * wxDialog_CreateSeparatedButtonSizer(wxDialog * self, long flags) {
    return self->CreateSeparatedButtonSizer(flags);
}
wxSizer * wxDialog_CreateSeparatedSizer(wxDialog * self, wxSizer * sizer) {
    return self->CreateSeparatedSizer(sizer);
}
wxStdDialogButtonSizer * wxDialog_CreateStdDialogButtonSizer(wxDialog * self, long flags) {
    return self->CreateStdDialogButtonSizer(flags);
}
wxSizer * wxDialog_CreateTextSizer(wxDialog * self, const wxString * message, int width_max) {
    return self->CreateTextSizer(*message, width_max);
}
bool wxDialog_DoLayoutAdaptation(wxDialog * self) {
    return self->DoLayoutAdaptation();
}
void wxDialog_EndModal(wxDialog * self, int ret_code) {
    return self->EndModal(ret_code);
}
int wxDialog_GetAffirmativeId(const wxDialog * self) {
    return self->GetAffirmativeId();
}
wxWindow * wxDialog_GetContentWindow(const wxDialog * self) {
    return self->GetContentWindow();
}
int wxDialog_GetEscapeId(const wxDialog * self) {
    return self->GetEscapeId();
}
bool wxDialog_GetLayoutAdaptationDone(const wxDialog * self) {
    return self->GetLayoutAdaptationDone();
}
int wxDialog_GetLayoutAdaptationLevel(const wxDialog * self) {
    return self->GetLayoutAdaptationLevel();
}
wxArrayInt * wxDialog_GetMainButtonIds(wxDialog * self) {
    return &(self->GetMainButtonIds());
}
int wxDialog_GetReturnCode(const wxDialog * self) {
    return self->GetReturnCode();
}
#ifdef __WXMSW__
wxToolBar * wxDialog_GetToolBar(const wxDialog * self) {
    return self->GetToolBar();
}
#endif
bool wxDialog_IsMainButtonId(const wxDialog * self, wxWindowID id) {
    return self->IsMainButtonId(id);
}
bool wxDialog_IsModal(const wxDialog * self) {
    return self->IsModal();
}
void wxDialog_SetAffirmativeId(wxDialog * self, int id) {
    return self->SetAffirmativeId(id);
}
void wxDialog_SetEscapeId(wxDialog * self, int id) {
    return self->SetEscapeId(id);
}
void wxDialog_SetIcon(wxDialog * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxDialog_SetLayoutAdaptationDone(wxDialog * self, bool done) {
    return self->SetLayoutAdaptationDone(done);
}
void wxDialog_SetLayoutAdaptationLevel(wxDialog * self, int level) {
    return self->SetLayoutAdaptationLevel(level);
}
void wxDialog_SetReturnCode(wxDialog * self, int ret_code) {
    return self->SetReturnCode(ret_code);
}
int wxDialog_ShowModal(wxDialog * self) {
    return self->ShowModal();
}
void wxDialog_ShowWindowModal(wxDialog * self) {
    return self->ShowWindowModal();
}
void wxDialog_EnableLayoutAdaptation(bool enable) {
    return wxDialog::EnableLayoutAdaptation(enable);
}
wxDialogLayoutAdapter * wxDialog_GetLayoutAdapter() {
    return wxDialog::GetLayoutAdapter();
}
bool wxDialog_IsLayoutAdaptationEnabled() {
    return wxDialog::IsLayoutAdaptationEnabled();
}
wxDialogLayoutAdapter * wxDialog_SetLayoutAdapter(wxDialogLayoutAdapter * adapter) {
    return wxDialog::SetLayoutAdapter(adapter);
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

