#pragma once

#include <wx/bitmap.h>
#include <wx/dcmemory.h>
#include <wx/dcmirror.h>
#include <wx/event.h>
#include <wx/menu.h>
#include <wx/menuitem.h>
#include <wx/minifram.h>
#include <wx/mousemanager.h>
#include <wx/msgdlg.h>
#include <wx/msgout.h>

extern "C" {

// CLASS: wxMask
wxClassInfo *wxMask_CLASSINFO();
wxMask *wxMask_new();
wxMask *wxMask_new1(const wxBitmap * bitmap, int index);
wxMask *wxMask_new2(const wxBitmap * bitmap);
wxMask *wxMask_new3(const wxBitmap * bitmap, const wxColour * colour);
bool wxMask_Create(wxMask * self, const wxBitmap * bitmap, int index);
bool wxMask_Create1(wxMask * self, const wxBitmap * bitmap);
bool wxMask_Create2(wxMask * self, const wxBitmap * bitmap, const wxColour * colour);
wxBitmap *wxMask_GetBitmap(const wxMask * self);

// CLASS: wxMaximizeEvent
wxClassInfo *wxMaximizeEvent_CLASSINFO();
wxMaximizeEvent *wxMaximizeEvent_new(int id);

// CLASS: wxMemoryDC
wxClassInfo *wxMemoryDC_CLASSINFO();
wxMemoryDC *wxMemoryDC_new();
wxMemoryDC *wxMemoryDC_new1(wxDC * dc);
wxMemoryDC *wxMemoryDC_new2(wxBitmap * bitmap);
void wxMemoryDC_SelectObject(wxMemoryDC * self, wxBitmap * bitmap);
void wxMemoryDC_SelectObjectAsSource(wxMemoryDC * self, const wxBitmap * bitmap);
wxBitmap *wxMemoryDC_GetSelectedBitmap(const wxMemoryDC * self);

// CLASS: wxMenu
wxClassInfo *wxMenu_CLASSINFO();
wxMenu *wxMenu_new();
wxMenu *wxMenu_new1(long style);
wxMenu *wxMenu_new2(const wxString * title, long style);
wxMenuItem * wxMenu_Append(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Append1(wxMenu * self, int id, const wxString * item, wxMenu * sub_menu, const wxString * help_string);
wxMenuItem * wxMenu_Append2(wxMenu * self, wxMenuItem * menu_item);
wxMenuItem * wxMenu_AppendCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help);
wxMenuItem * wxMenu_AppendRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help);
wxMenuItem * wxMenu_AppendSeparator(wxMenu * self);
wxMenuItem * wxMenu_AppendSubMenu(wxMenu * self, wxMenu * submenu, const wxString * text, const wxString * help);
void wxMenu_Break(wxMenu * self);
void wxMenu_Check(wxMenu * self, int id, bool check);
bool wxMenu_Delete(wxMenu * self, int id);
bool wxMenu_Delete1(wxMenu * self, wxMenuItem * item);
bool wxMenu_Destroy(wxMenu * self, int id);
bool wxMenu_Destroy1(wxMenu * self, wxMenuItem * item);
void wxMenu_Enable(wxMenu * self, int id, bool enable);
wxMenuItem * wxMenu_FindChildItem(const wxMenu * self, int id, size_t * pos);
int wxMenu_FindItem(const wxMenu * self, const wxString * item_string);
wxMenuItem * wxMenu_FindItem1(const wxMenu * self, int id, wxMenu ** menu);
wxMenuItem * wxMenu_FindItemByPosition(const wxMenu * self, size_t position);
wxString *wxMenu_GetHelpString(const wxMenu * self, int id);
wxString *wxMenu_GetLabel(const wxMenu * self, int id);
wxString *wxMenu_GetLabelText(const wxMenu * self, int id);
size_t wxMenu_GetMenuItemCount(const wxMenu * self);
wxString *wxMenu_GetTitle(const wxMenu * self);
wxMenuItem * wxMenu_Insert(wxMenu * self, size_t pos, wxMenuItem * menu_item);
wxMenuItem * wxMenu_Insert1(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Insert2(wxMenu * self, size_t pos, int id, const wxString * text, wxMenu * submenu, const wxString * help);
wxMenuItem * wxMenu_InsertCheckItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_InsertRadioItem(wxMenu * self, size_t pos, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_InsertSeparator(wxMenu * self, size_t pos);
bool wxMenu_IsChecked(const wxMenu * self, int id);
bool wxMenu_IsEnabled(const wxMenu * self, int id);
wxMenuItem * wxMenu_Prepend(wxMenu * self, wxMenuItem * item);
wxMenuItem * wxMenu_Prepend1(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind);
wxMenuItem * wxMenu_Prepend2(wxMenu * self, int id, const wxString * text, wxMenu * submenu, const wxString * help);
wxMenuItem * wxMenu_PrependCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_PrependRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help_string);
wxMenuItem * wxMenu_PrependSeparator(wxMenu * self);
wxMenuItem * wxMenu_Remove(wxMenu * self, int id);
wxMenuItem * wxMenu_Remove1(wxMenu * self, wxMenuItem * item);
void wxMenu_SetHelpString(wxMenu * self, int id, const wxString * help_string);
void wxMenu_SetLabel(wxMenu * self, int id, const wxString * label);
void wxMenu_SetTitle(wxMenu * self, const wxString * title);
void wxMenu_UpdateUI(wxMenu * self, wxEvtHandler * source);
void wxMenu_SetInvokingWindow(wxMenu * self, wxWindow * win);
wxWindow * wxMenu_GetInvokingWindow(const wxMenu * self);
wxWindow * wxMenu_GetWindow(const wxMenu * self);
long wxMenu_GetStyle(const wxMenu * self);
void wxMenu_SetParent(wxMenu * self, wxMenu * parent);
wxMenu * wxMenu_GetParent(const wxMenu * self);
void wxMenu_Attach(wxMenu * self, wxMenuBar * menubar);
void wxMenu_Detach(wxMenu * self);
bool wxMenu_IsAttached(const wxMenu * self);

// CLASS: wxMenuBar
wxClassInfo *wxMenuBar_CLASSINFO();
wxMenuBar *wxMenuBar_new(long style);
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title);
void wxMenuBar_Check(wxMenuBar * self, int id, bool check);
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable);
bool wxMenuBar_IsEnabledTop(const wxMenuBar * self, size_t pos);
void wxMenuBar_EnableTop(wxMenuBar * self, size_t pos, bool enable);
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu);
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title);
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string);
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id);
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id);
wxMenu * wxMenuBar_GetMenu(const wxMenuBar * self, size_t menu_index);
size_t wxMenuBar_GetMenuCount(const wxMenuBar * self);
wxString *wxMenuBar_GetMenuLabel(const wxMenuBar * self, size_t pos);
wxString *wxMenuBar_GetMenuLabelText(const wxMenuBar * self, size_t pos);
bool wxMenuBar_Insert(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title);
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id);
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id);
wxMenu * wxMenuBar_Remove(wxMenuBar * self, size_t pos);
wxMenu * wxMenuBar_Replace(wxMenuBar * self, size_t pos, wxMenu * menu, const wxString * title);
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string);
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label);
void wxMenuBar_SetMenuLabel(wxMenuBar * self, size_t pos, const wxString * label);
#ifdef __WXOSX__
wxMenu * wxMenuBar_OSXGetAppleMenu(const wxMenuBar * self);
#endif
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self);
bool wxMenuBar_IsAttached(const wxMenuBar * self);
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame);
void wxMenuBar_Detach(wxMenuBar * self);
#ifdef __WXOSX__
void wxMenuBar_MacSetCommonMenuBar(wxMenuBar * menubar);
wxMenuBar * wxMenuBar_MacGetCommonMenuBar();
#endif

// CLASS: wxMenuEvent
wxClassInfo *wxMenuEvent_CLASSINFO();
wxMenu * wxMenuEvent_GetMenu(const wxMenuEvent * self);
int wxMenuEvent_GetMenuId(const wxMenuEvent * self);
bool wxMenuEvent_IsPopup(const wxMenuEvent * self);

// CLASS: wxMenuItem
wxClassInfo *wxMenuItem_CLASSINFO();
wxBitmap *wxMenuItem_GetBitmap(const wxMenuItem * self);
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetBitmap1(const wxMenuItem * self, bool checked);
#endif
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxMenuItem_GetBitmapBundle(const wxMenuItem * self);
#endif
#ifdef __WXMSW__
wxBitmap *wxMenuItem_GetDisabledBitmap(const wxMenuItem * self);
#endif
wxString *wxMenuItem_GetHelp(const wxMenuItem * self);
int wxMenuItem_GetId(const wxMenuItem * self);
wxString *wxMenuItem_GetItemLabel(const wxMenuItem * self);
wxString *wxMenuItem_GetItemLabelText(const wxMenuItem * self);
wxItemKind wxMenuItem_GetKind(const wxMenuItem * self);
#ifdef __WXMSW__
int wxMenuItem_GetMarginWidth(const wxMenuItem * self);
#endif
wxMenu * wxMenuItem_GetMenu(const wxMenuItem * self);
wxMenu * wxMenuItem_GetSubMenu(const wxMenuItem * self);
wxAcceleratorEntry * wxMenuItem_GetAccel(const wxMenuItem * self);
bool wxMenuItem_IsCheck(const wxMenuItem * self);
bool wxMenuItem_IsCheckable(const wxMenuItem * self);
bool wxMenuItem_IsChecked(const wxMenuItem * self);
bool wxMenuItem_IsEnabled(const wxMenuItem * self);
bool wxMenuItem_IsRadio(const wxMenuItem * self);
bool wxMenuItem_IsSeparator(const wxMenuItem * self);
bool wxMenuItem_IsSubMenu(const wxMenuItem * self);
#ifdef __WXMSW__
void wxMenuItem_SetBackgroundColour(wxMenuItem * self, const wxColour * colour);
#endif
void wxMenuItem_SetBitmap(wxMenuItem * self, const wxBitmapBundle * bmp);
#ifdef __WXMSW__
void wxMenuItem_SetBitmap1(wxMenuItem * self, const wxBitmapBundle * bmp, bool checked);
void wxMenuItem_SetBitmaps(wxMenuItem * self, const wxBitmapBundle * checked, const wxBitmapBundle * unchecked);
void wxMenuItem_SetDisabledBitmap(wxMenuItem * self, const wxBitmapBundle * disabled);
void wxMenuItem_SetFont(wxMenuItem * self, const wxFont * font);
#endif
void wxMenuItem_SetHelp(wxMenuItem * self, const wxString * help_string);
void wxMenuItem_SetItemLabel(wxMenuItem * self, const wxString * label);
#ifdef __WXMSW__
void wxMenuItem_SetMarginWidth(wxMenuItem * self, int width);
#endif
void wxMenuItem_SetMenu(wxMenuItem * self, wxMenu * menu);
void wxMenuItem_SetSubMenu(wxMenuItem * self, wxMenu * menu);
#ifdef __WXMSW__
void wxMenuItem_SetTextColour(wxMenuItem * self, const wxColour * colour);
#endif
void wxMenuItem_SetAccel(wxMenuItem * self, wxAcceleratorEntry * accel);
#if wxCHECK_VERSION(3, 1, 0)
void wxMenuItem_AddExtraAccel(wxMenuItem * self, const wxAcceleratorEntry * accel);
void wxMenuItem_ClearExtraAccels(wxMenuItem * self);
#endif
wxMenuItem *wxMenuItem_new(wxMenu * parent_menu, int id, const wxString * text, const wxString * help_string, wxItemKind kind, wxMenu * sub_menu);
void wxMenuItem_Check(wxMenuItem * self, bool check);
void wxMenuItem_Enable(wxMenuItem * self, bool enable);
wxString *wxMenuItem_GetLabelText(const wxString * text);

// CLASS: wxMessageDialog
wxClassInfo *wxMessageDialog_CLASSINFO();
wxMessageDialog *wxMessageDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, long style, const wxPoint * pos);
void wxMessageDialog_SetExtendedMessage(wxMessageDialog * self, const wxString * extended_message);
bool wxMessageDialog_SetHelpLabel(wxMessageDialog * self, const ButtonLabel * help);
void wxMessageDialog_SetMessage(wxMessageDialog * self, const wxString * message);
bool wxMessageDialog_SetOKCancelLabels(wxMessageDialog * self, const ButtonLabel * ok, const ButtonLabel * cancel);
bool wxMessageDialog_SetOKLabel(wxMessageDialog * self, const ButtonLabel * ok);
bool wxMessageDialog_SetYesNoCancelLabels(wxMessageDialog * self, const ButtonLabel * yes, const ButtonLabel * no, const ButtonLabel * cancel);
bool wxMessageDialog_SetYesNoLabels(wxMessageDialog * self, const ButtonLabel * yes, const ButtonLabel * no);
wxString *wxMessageDialog_GetCaption(const wxMessageDialog * self);
wxString *wxMessageDialog_GetMessage(const wxMessageDialog * self);
wxString *wxMessageDialog_GetExtendedMessage(const wxMessageDialog * self);
long wxMessageDialog_GetMessageDialogStyle(const wxMessageDialog * self);
bool wxMessageDialog_HasCustomLabels(const wxMessageDialog * self);
wxString *wxMessageDialog_GetYesLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetNoLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetOKLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetCancelLabel(const wxMessageDialog * self);
wxString *wxMessageDialog_GetHelpLabel(const wxMessageDialog * self);
long wxMessageDialog_GetEffectiveIcon(const wxMessageDialog * self);

// CLASS: wxMessageOutputMessageBox
void wxMessageOutputMessageBox_delete(wxMessageOutputMessageBox *self);
wxMessageOutputMessageBox *wxMessageOutputMessageBox_new();

// CLASS: wxMiniFrame
wxClassInfo *wxMiniFrame_CLASSINFO();
wxMiniFrame *wxMiniFrame_new();
wxMiniFrame *wxMiniFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxMiniFrame_Create(wxMiniFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxMirrorDC
wxClassInfo *wxMirrorDC_CLASSINFO();
wxMirrorDC *wxMirrorDC_new(wxDC * dc, bool mirror);

// CLASS: wxMouseCaptureChangedEvent
wxClassInfo *wxMouseCaptureChangedEvent_CLASSINFO();
wxMouseCaptureChangedEvent *wxMouseCaptureChangedEvent_new(wxWindowID window_id, wxWindow * gained_capture);
wxWindow * wxMouseCaptureChangedEvent_GetCapturedWindow(const wxMouseCaptureChangedEvent * self);

// CLASS: wxMouseCaptureLostEvent
wxClassInfo *wxMouseCaptureLostEvent_CLASSINFO();
wxMouseCaptureLostEvent *wxMouseCaptureLostEvent_new(wxWindowID window_id);

// CLASS: wxMouseEvent
wxClassInfo *wxMouseEvent_CLASSINFO();
bool wxMouseEvent_Aux1DClick(const wxMouseEvent * self);
bool wxMouseEvent_Aux1Down(const wxMouseEvent * self);
bool wxMouseEvent_Aux1Up(const wxMouseEvent * self);
bool wxMouseEvent_Aux2DClick(const wxMouseEvent * self);
bool wxMouseEvent_Aux2Down(const wxMouseEvent * self);
bool wxMouseEvent_Aux2Up(const wxMouseEvent * self);
bool wxMouseEvent_Dragging(const wxMouseEvent * self);
bool wxMouseEvent_Entering(const wxMouseEvent * self);
int wxMouseEvent_GetButton(const wxMouseEvent * self);
int wxMouseEvent_GetClickCount(const wxMouseEvent * self);
int wxMouseEvent_GetLinesPerAction(const wxMouseEvent * self);
int wxMouseEvent_GetColumnsPerAction(const wxMouseEvent * self);
wxPoint *wxMouseEvent_GetLogicalPosition(const wxMouseEvent * self, const wxDC * dc);
int wxMouseEvent_GetWheelDelta(const wxMouseEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxMouseEvent_IsWheelInverted(const wxMouseEvent * self);
#endif
int wxMouseEvent_GetWheelRotation(const wxMouseEvent * self);
bool wxMouseEvent_IsButton(const wxMouseEvent * self);
bool wxMouseEvent_IsPageScroll(const wxMouseEvent * self);
bool wxMouseEvent_Leaving(const wxMouseEvent * self);
bool wxMouseEvent_LeftDClick(const wxMouseEvent * self);
bool wxMouseEvent_LeftDown(const wxMouseEvent * self);
bool wxMouseEvent_LeftUp(const wxMouseEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxMouseEvent_Magnify(const wxMouseEvent * self);
#endif
bool wxMouseEvent_MetaDown(const wxMouseEvent * self);
bool wxMouseEvent_MiddleDClick(const wxMouseEvent * self);
bool wxMouseEvent_MiddleDown(const wxMouseEvent * self);
bool wxMouseEvent_MiddleUp(const wxMouseEvent * self);
bool wxMouseEvent_Moving(const wxMouseEvent * self);
bool wxMouseEvent_RightDClick(const wxMouseEvent * self);
bool wxMouseEvent_RightDown(const wxMouseEvent * self);
bool wxMouseEvent_RightUp(const wxMouseEvent * self);

// CLASS: wxMouseEventsManager
wxClassInfo *wxMouseEventsManager_CLASSINFO();
bool wxMouseEventsManager_Create(wxMouseEventsManager * self, wxWindow * win);

// CLASS: wxMoveEvent
wxClassInfo *wxMoveEvent_CLASSINFO();
wxMoveEvent *wxMoveEvent_new(const wxPoint * pt, int id);
wxPoint *wxMoveEvent_GetPosition(const wxMoveEvent * self);
wxRect *wxMoveEvent_GetRect(const wxMoveEvent * self);
void wxMoveEvent_SetRect(wxMoveEvent * self, const wxRect * rect);
void wxMoveEvent_SetPosition(wxMoveEvent * self, const wxPoint * pos);

} // extern "C"

