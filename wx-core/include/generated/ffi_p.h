#pragma once

#include <wx/dcclient.h>
#include <wx/event.h>
#include <wx/gdicmn.h>
#include <wx/imagpcx.h>
#include <wx/imagpng.h>
#include <wx/imagpnm.h>
#include <wx/palette.h>
#include <wx/panel.h>
#include <wx/pen.h>
#include <wx/persist.h>
#include <wx/pickerbase.h>
#include <wx/popupwin.h>
#include <wx/preferences.h>
#include <wx/propdlg.h>
#include <wx/textdlg.h>

extern "C" {

// CLASS: wxPCXHandler
wxClassInfo *wxPCXHandler_CLASSINFO();
wxPCXHandler *wxPCXHandler_new();

// CLASS: wxPNGHandler
wxClassInfo *wxPNGHandler_CLASSINFO();
wxPNGHandler *wxPNGHandler_new();

// CLASS: wxPNMHandler
wxClassInfo *wxPNMHandler_CLASSINFO();
wxPNMHandler *wxPNMHandler_new();

// CLASS: wxPaintDC
wxClassInfo *wxPaintDC_CLASSINFO();
wxPaintDC *wxPaintDC_new(wxWindow * window);

// CLASS: wxPaintEvent
wxClassInfo *wxPaintEvent_CLASSINFO();

// CLASS: wxPalette
wxClassInfo *wxPalette_CLASSINFO();
wxPalette *wxPalette_new();
wxPalette *wxPalette_new1(const wxPalette * palette);
wxPalette *wxPalette_new2(int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue);
bool wxPalette_Create(wxPalette * self, int n, const unsigned char * red, const unsigned char * green, const unsigned char * blue);
int wxPalette_GetColoursCount(const wxPalette * self);
bool wxPalette_GetRGB(const wxPalette * self, int pixel, unsigned char * red, unsigned char * green, unsigned char * blue);
bool wxPalette_IsOk(const wxPalette * self);

// CLASS: wxPanel
wxClassInfo *wxPanel_CLASSINFO();
wxPanel *wxPanel_new();
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event);
void wxPanel_SetFocusIgnoringChildren(wxPanel * self);

// CLASS: wxPasswordEntryDialog
wxClassInfo *wxPasswordEntryDialog_CLASSINFO();
wxPasswordEntryDialog *wxPasswordEntryDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, const wxString * default_value, long style, const wxPoint * pos);

// CLASS: wxPen
wxClassInfo *wxPen_CLASSINFO();
wxPen *wxPen_new();
#if wxCHECK_VERSION(3, 1, 0)
wxPen *wxPen_new1(const wxPenInfo * info);
#endif
#ifndef __WXGTK__
wxPen *wxPen_new3(const wxBitmap * stipple, int width);
#endif
wxPen *wxPen_new4(const wxPen * pen);
wxColour *wxPen_GetColour(const wxPen * self);
int wxPen_GetDashes(const wxPen * self, wxDash ** dashes);
wxBitmap * wxPen_GetStipple(const wxPen * self);
int wxPen_GetWidth(const wxPen * self);
bool wxPen_IsOk(const wxPen * self);
bool wxPen_IsNonTransparent(const wxPen * self);
bool wxPen_IsTransparent(const wxPen * self);
void wxPen_SetColour(wxPen * self, wxColour * colour);
void wxPen_SetDashes(wxPen * self, int n, const wxDash * dash);
void wxPen_SetStipple(wxPen * self, const wxBitmap * stipple);
void wxPen_SetWidth(wxPen * self, int width);

// CLASS: wxPenList
void wxPenList_delete(wxPenList *self);
wxPenList *wxPenList_new();

// CLASS: wxPersistenceManager
void wxPersistenceManager_delete(wxPersistenceManager *self);
void wxPersistenceManager_Set(wxPersistenceManager * manager);
wxPersistenceManager * wxPersistenceManager_Get();
void wxPersistenceManager_DisableSaving(wxPersistenceManager * self);
void wxPersistenceManager_DisableRestoring(wxPersistenceManager * self);
wxPersistentObject * wxPersistenceManager_Register1(wxPersistenceManager * self, void * obj, wxPersistentObject * po);
wxPersistentObject * wxPersistenceManager_Find(const wxPersistenceManager * self, void * obj);
void wxPersistenceManager_Unregister(wxPersistenceManager * self, void * obj);
void wxPersistenceManager_Save(wxPersistenceManager * self, void * obj);
bool wxPersistenceManager_Restore(wxPersistenceManager * self, void * obj);
void wxPersistenceManager_SaveAndUnregister(wxPersistenceManager * self, void * obj);
bool wxPersistenceManager_RegisterAndRestore1(wxPersistenceManager * self, void * obj, wxPersistentObject * po);

// CLASS: wxPickerBase
wxClassInfo *wxPickerBase_CLASSINFO();
bool wxPickerBase_CreateBase(wxPickerBase * self, wxWindow * parent, wxWindowID id, const wxString * text, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxPickerBase_GetInternalMargin(const wxPickerBase * self);
int wxPickerBase_GetPickerCtrlProportion(const wxPickerBase * self);
wxTextCtrl * wxPickerBase_GetTextCtrl(wxPickerBase * self);
wxControl * wxPickerBase_GetPickerCtrl(wxPickerBase * self);
int wxPickerBase_GetTextCtrlProportion(const wxPickerBase * self);
bool wxPickerBase_HasTextCtrl(const wxPickerBase * self);
bool wxPickerBase_IsPickerCtrlGrowable(const wxPickerBase * self);
bool wxPickerBase_IsTextCtrlGrowable(const wxPickerBase * self);
void wxPickerBase_SetInternalMargin(wxPickerBase * self, int margin);
void wxPickerBase_SetPickerCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetPickerCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrlGrowable(wxPickerBase * self, bool grow);
void wxPickerBase_SetTextCtrlProportion(wxPickerBase * self, int prop);
void wxPickerBase_SetTextCtrl(wxPickerBase * self, wxTextCtrl * text);
void wxPickerBase_SetPickerCtrl(wxPickerBase * self, wxControl * picker);
void wxPickerBase_UpdatePickerFromTextCtrl(wxPickerBase * self);
void wxPickerBase_UpdateTextCtrlFromPicker(wxPickerBase * self);

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self);
bool wxPoint_IsFullySpecified(const wxPoint * self);
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt);
wxPoint *wxPoint_new();
wxPoint *wxPoint_new1(int x, int y);
wxPoint *wxPoint_new2(const wxRealPoint * pt);

// CLASS: wxPopupTransientWindow
wxClassInfo *wxPopupTransientWindow_CLASSINFO();
wxPopupTransientWindow *wxPopupTransientWindow_new();
wxPopupTransientWindow *wxPopupTransientWindow_new1(wxWindow * parent, int flags);
void wxPopupTransientWindow_Popup(wxPopupTransientWindow * self, wxWindow * focus);
void wxPopupTransientWindow_Dismiss(wxPopupTransientWindow * self);
bool wxPopupTransientWindow_ProcessLeftDown(wxPopupTransientWindow * self, wxMouseEvent * event);

// CLASS: wxPopupWindow
wxClassInfo *wxPopupWindow_CLASSINFO();
wxPopupWindow *wxPopupWindow_new();
wxPopupWindow *wxPopupWindow_new1(wxWindow * parent, int flags);
bool wxPopupWindow_Create(wxPopupWindow * self, wxWindow * parent, int flags);
void wxPopupWindow_Position(wxPopupWindow * self, const wxPoint * pt_origin, const wxSize * size_popup);

// CLASS: wxPreferencesEditor
void wxPreferencesEditor_delete(wxPreferencesEditor *self);
wxPreferencesEditor *wxPreferencesEditor_new(const wxString * title);
void wxPreferencesEditor_AddPage(wxPreferencesEditor * self, wxPreferencesPage * page);
void wxPreferencesEditor_Show(wxPreferencesEditor * self, wxWindow * parent);
void wxPreferencesEditor_Dismiss(wxPreferencesEditor * self);
bool wxPreferencesEditor_ShouldApplyChangesImmediately();
bool wxPreferencesEditor_ShownModally();

// CLASS: wxPreferencesPage
void wxPreferencesPage_delete(wxPreferencesPage *self);
wxString *wxPreferencesPage_GetName(const wxPreferencesPage * self);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxPreferencesPage_GetIcon(const wxPreferencesPage * self);
#endif
wxBitmap *wxPreferencesPage_GetLargeIcon(const wxPreferencesPage * self);
wxWindow * wxPreferencesPage_CreateWindow(wxPreferencesPage * self, wxWindow * parent);

// CLASS: wxPropertySheetDialog
wxClassInfo *wxPropertySheetDialog_CLASSINFO();
wxPropertySheetDialog *wxPropertySheetDialog_new();
wxPropertySheetDialog *wxPropertySheetDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxPropertySheetDialog_AddBookCtrl(wxPropertySheetDialog * self, wxSizer * sizer);
bool wxPropertySheetDialog_Create(wxPropertySheetDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxBookCtrlBase * wxPropertySheetDialog_CreateBookCtrl(wxPropertySheetDialog * self);
void wxPropertySheetDialog_CreateButtons(wxPropertySheetDialog * self, int flags);
wxBookCtrlBase * wxPropertySheetDialog_GetBookCtrl(const wxPropertySheetDialog * self);
wxSizer * wxPropertySheetDialog_GetInnerSizer(const wxPropertySheetDialog * self);
void wxPropertySheetDialog_SetInnerSizer(wxPropertySheetDialog * self, wxSizer * sizer);
long wxPropertySheetDialog_GetSheetStyle(const wxPropertySheetDialog * self);
void wxPropertySheetDialog_LayoutDialog(wxPropertySheetDialog * self, int centre_flags);
void wxPropertySheetDialog_SetBookCtrl(wxPropertySheetDialog * self, wxBookCtrlBase * book_ctrl);
void wxPropertySheetDialog_SetSheetStyle(wxPropertySheetDialog * self, long style);
void wxPropertySheetDialog_SetSheetOuterBorder(wxPropertySheetDialog * self, int border);
int wxPropertySheetDialog_GetSheetOuterBorder(const wxPropertySheetDialog * self);
void wxPropertySheetDialog_SetSheetInnerBorder(wxPropertySheetDialog * self, int border);
int wxPropertySheetDialog_GetSheetInnerBorder(const wxPropertySheetDialog * self);

} // extern "C"

