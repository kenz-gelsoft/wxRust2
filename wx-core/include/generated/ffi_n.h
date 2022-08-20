#pragma once

#include <wx/event.h>
#include <wx/fontutil.h>
#include <wx/nativewin.h>
#include <wx/nonownedwnd.h>
#include <wx/notebook.h>
#include <wx/notifmsg.h>
#include <wx/numdlg.h>

extern "C" {

// CLASS: wxNativeFontInfo
void wxNativeFontInfo_delete(wxNativeFontInfo *self);
wxNativeFontInfo *wxNativeFontInfo_new();
wxNativeFontInfo *wxNativeFontInfo_new1(const wxNativeFontInfo * info);
void wxNativeFontInfo_Init(wxNativeFontInfo * self);
void wxNativeFontInfo_InitFromFont(wxNativeFontInfo * self, const wxFont * font);
int wxNativeFontInfo_GetPointSize(const wxNativeFontInfo * self);
wxSize *wxNativeFontInfo_GetPixelSize(const wxNativeFontInfo * self);
int wxNativeFontInfo_GetNumericWeight(const wxNativeFontInfo * self);
bool wxNativeFontInfo_GetUnderlined(const wxNativeFontInfo * self);
wxString *wxNativeFontInfo_GetFaceName(const wxNativeFontInfo * self);
void wxNativeFontInfo_SetPointSize(wxNativeFontInfo * self, int pointsize);
void wxNativeFontInfo_SetPixelSize(wxNativeFontInfo * self, const wxSize * pixel_size);
void wxNativeFontInfo_SetNumericWeight(wxNativeFontInfo * self, int weight);
void wxNativeFontInfo_SetUnderlined(wxNativeFontInfo * self, bool underlined);
bool wxNativeFontInfo_SetFaceName(wxNativeFontInfo * self, const wxString * facename);
void wxNativeFontInfo_SetFaceName1(wxNativeFontInfo * self, const wxArrayString * facenames);
bool wxNativeFontInfo_FromString(wxNativeFontInfo * self, const wxString * s);
wxString *wxNativeFontInfo_ToString(const wxNativeFontInfo * self);
bool wxNativeFontInfo_FromUserString(wxNativeFontInfo * self, const wxString * s);
wxString *wxNativeFontInfo_ToUserString(const wxNativeFontInfo * self);

// CLASS: wxNativeWindow
wxClassInfo *wxNativeWindow_CLASSINFO();
wxNativeWindow *wxNativeWindow_new();
void wxNativeWindow_Disown(wxNativeWindow * self);

// CLASS: wxNavigationKeyEvent
wxClassInfo *wxNavigationKeyEvent_CLASSINFO();
wxNavigationKeyEvent *wxNavigationKeyEvent_new();
wxNavigationKeyEvent *wxNavigationKeyEvent_new1(const wxNavigationKeyEvent * event);
wxWindow * wxNavigationKeyEvent_GetCurrentFocus(const wxNavigationKeyEvent * self);
bool wxNavigationKeyEvent_GetDirection(const wxNavigationKeyEvent * self);
bool wxNavigationKeyEvent_IsFromTab(const wxNavigationKeyEvent * self);
bool wxNavigationKeyEvent_IsWindowChange(const wxNavigationKeyEvent * self);
void wxNavigationKeyEvent_SetCurrentFocus(wxNavigationKeyEvent * self, wxWindow * current_focus);
void wxNavigationKeyEvent_SetDirection(wxNavigationKeyEvent * self, bool direction);
void wxNavigationKeyEvent_SetFlags(wxNavigationKeyEvent * self, long flags);
void wxNavigationKeyEvent_SetFromTab(wxNavigationKeyEvent * self, bool from_tab);
void wxNavigationKeyEvent_SetWindowChange(wxNavigationKeyEvent * self, bool window_change);

// CLASS: wxNonOwnedWindow
wxClassInfo *wxNonOwnedWindow_CLASSINFO();
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region);
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path);

// CLASS: wxNotebook
wxClassInfo *wxNotebook_CLASSINFO();
wxNotebook *wxNotebook_new();
wxNotebook *wxNotebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxNotebook_Create(wxNotebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
int wxNotebook_GetRowCount(const wxNotebook * self);
wxColour *wxNotebook_GetThemeBackgroundColour(const wxNotebook * self);
void wxNotebook_SetPadding(wxNotebook * self, const wxSize * padding);

// CLASS: wxNotificationMessage
wxClassInfo *wxNotificationMessage_CLASSINFO();
wxNotificationMessage *wxNotificationMessage_new();
wxNotificationMessage *wxNotificationMessage_new1(const wxString * title, const wxString * message, wxWindow * parent, int flags);
bool wxNotificationMessage_AddAction(wxNotificationMessage * self, wxWindowID actionid, const wxString * label);
bool wxNotificationMessage_Close(wxNotificationMessage * self);
void wxNotificationMessage_SetFlags(wxNotificationMessage * self, int flags);
void wxNotificationMessage_SetIcon(wxNotificationMessage * self, const wxIcon * icon);
void wxNotificationMessage_SetMessage(wxNotificationMessage * self, const wxString * message);
void wxNotificationMessage_SetParent(wxNotificationMessage * self, wxWindow * parent);
void wxNotificationMessage_SetTitle(wxNotificationMessage * self, const wxString * title);
bool wxNotificationMessage_Show(wxNotificationMessage * self, int timeout);
#ifdef __WXMSW__
wxTaskBarIcon * wxNotificationMessage_UseTaskBarIcon(wxTaskBarIcon * icon);
bool wxNotificationMessage_MSWUseToasts(const wxString * shortcut_path, const wxString * app_id);
#endif

// CLASS: wxNotifyEvent
wxClassInfo *wxNotifyEvent_CLASSINFO();
void wxNotifyEvent_Allow(wxNotifyEvent * self);
bool wxNotifyEvent_IsAllowed(const wxNotifyEvent * self);
void wxNotifyEvent_Veto(wxNotifyEvent * self);

// CLASS: wxNumberEntryDialog
wxClassInfo *wxNumberEntryDialog_CLASSINFO();
wxNumberEntryDialog *wxNumberEntryDialog_new();
wxNumberEntryDialog *wxNumberEntryDialog_new1(wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos);
bool wxNumberEntryDialog_Create(wxNumberEntryDialog * self, wxWindow * parent, const wxString * message, const wxString * prompt, const wxString * caption, long value, long min, long max, const wxPoint * pos);
long wxNumberEntryDialog_GetValue(const wxNumberEntryDialog * self);

} // extern "C"

