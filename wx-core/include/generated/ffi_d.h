#pragma once

#include <wx/dataobj.h>
#include <wx/datectrl.h>
#include <wx/dateevt.h>
#include <wx/dialog.h>
#include <wx/filepicker.h>

extern "C" {

// CLASS: wxDataObject
void wxDataObject_delete(wxDataObject *self);
bool wxDataObject_GetDataHere(const wxDataObject * self, const wxDataFormat * format, void * buf);
size_t wxDataObject_GetDataSize(const wxDataObject * self, const wxDataFormat * format);
bool wxDataObject_SetData(wxDataObject * self, const wxDataFormat * format, size_t len, const void * buf);

// CLASS: wxDataObjectSimple
void wxDataObjectSimple_delete(wxDataObjectSimple *self);
wxDataObjectSimple *wxDataObjectSimple_new(const wxDataFormat * format);
bool wxDataObjectSimple_GetDataHere(const wxDataObjectSimple * self, void * buf);
size_t wxDataObjectSimple_GetDataSize(const wxDataObjectSimple * self);
bool wxDataObjectSimple_SetData(wxDataObjectSimple * self, size_t len, const void * buf);
void wxDataObjectSimple_SetFormat(wxDataObjectSimple * self, const wxDataFormat * format);

// CLASS: wxDateEvent
wxClassInfo *wxDateEvent_CLASSINFO();
wxDateEvent *wxDateEvent_new();
wxDateTime *wxDateEvent_GetDate(const wxDateEvent * self);
void wxDateEvent_SetDate(wxDateEvent * self, const wxDateTime * date);

// CLASS: wxDatePickerCtrl
wxClassInfo *wxDatePickerCtrl_CLASSINFO();
wxDatePickerCtrl *wxDatePickerCtrl_new();
wxDatePickerCtrl *wxDatePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_Create(wxDatePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_GetRange(const wxDatePickerCtrl * self, wxDateTime * dt1, wxDateTime * dt2);
wxDateTime *wxDatePickerCtrl_GetValue(const wxDatePickerCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxDatePickerCtrl_SetNullText(wxDatePickerCtrl * self, const wxString * text);
#endif
void wxDatePickerCtrl_SetRange(wxDatePickerCtrl * self, const wxDateTime * dt1, const wxDateTime * dt2);
void wxDatePickerCtrl_SetValue(wxDatePickerCtrl * self, const wxDateTime * dt);

// CLASS: wxDialog
wxClassInfo *wxDialog_CLASSINFO();
wxDialog *wxDialog_new();
wxDialog *wxDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxDialog_AddMainButtonId(wxDialog * self, wxWindowID id);
bool wxDialog_CanDoLayoutAdaptation(wxDialog * self);
void wxDialog_Centre(wxDialog * self, int direction);
bool wxDialog_Create(wxDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxSizer * wxDialog_CreateButtonSizer(wxDialog * self, long flags);
wxSizer * wxDialog_CreateSeparatedButtonSizer(wxDialog * self, long flags);
wxSizer * wxDialog_CreateSeparatedSizer(wxDialog * self, wxSizer * sizer);
wxStdDialogButtonSizer * wxDialog_CreateStdDialogButtonSizer(wxDialog * self, long flags);
wxSizer * wxDialog_CreateTextSizer(wxDialog * self, const wxString * message, int width_max);
bool wxDialog_DoLayoutAdaptation(wxDialog * self);
void wxDialog_EndModal(wxDialog * self, int ret_code);
int wxDialog_GetAffirmativeId(const wxDialog * self);
wxWindow * wxDialog_GetContentWindow(const wxDialog * self);
int wxDialog_GetEscapeId(const wxDialog * self);
bool wxDialog_GetLayoutAdaptationDone(const wxDialog * self);
int wxDialog_GetLayoutAdaptationLevel(const wxDialog * self);
wxArrayInt * wxDialog_GetMainButtonIds(wxDialog * self);
int wxDialog_GetReturnCode(const wxDialog * self);
#ifdef __WXMSW__
wxToolBar * wxDialog_GetToolBar(const wxDialog * self);
#endif
bool wxDialog_IsMainButtonId(const wxDialog * self, wxWindowID id);
bool wxDialog_IsModal(const wxDialog * self);
void wxDialog_SetAffirmativeId(wxDialog * self, int id);
void wxDialog_SetEscapeId(wxDialog * self, int id);
void wxDialog_SetIcon(wxDialog * self, const wxIcon * icon);
void wxDialog_SetLayoutAdaptationDone(wxDialog * self, bool done);
void wxDialog_SetLayoutAdaptationLevel(wxDialog * self, int level);
void wxDialog_SetReturnCode(wxDialog * self, int ret_code);
int wxDialog_ShowModal(wxDialog * self);
void wxDialog_ShowWindowModal(wxDialog * self);
void wxDialog_EnableLayoutAdaptation(bool enable);
wxDialogLayoutAdapter * wxDialog_GetLayoutAdapter();
bool wxDialog_IsLayoutAdaptationEnabled();
wxDialogLayoutAdapter * wxDialog_SetLayoutAdapter(wxDialogLayoutAdapter * adapter);

// CLASS: wxDirPickerCtrl
wxClassInfo *wxDirPickerCtrl_CLASSINFO();
wxDirPickerCtrl *wxDirPickerCtrl_new();
wxDirPickerCtrl *wxDirPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDirPickerCtrl_Create(wxDirPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxFileName *wxDirPickerCtrl_GetDirName(const wxDirPickerCtrl * self);
wxString *wxDirPickerCtrl_GetPath(const wxDirPickerCtrl * self);
void wxDirPickerCtrl_SetDirName(wxDirPickerCtrl * self, const wxFileName * dirname);
void wxDirPickerCtrl_SetInitialDirectory(wxDirPickerCtrl * self, const wxString * dir);
void wxDirPickerCtrl_SetPath(wxDirPickerCtrl * self, const wxString * dirname);

} // extern "C"

