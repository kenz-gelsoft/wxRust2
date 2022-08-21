#pragma once

#include <wx/dcclient.h>
#include <wx/event.h>
#include <wx/utils.h>
#include <wx/window.h>
#include <wx/wizard.h>
#include <wx/wrapsizer.h>

extern "C" {

// CLASS: wxWindow
wxClassInfo *wxWindow_CLASSINFO();
bool wxWindow_AcceptsFocus(const wxWindow * self);
bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow * self);
bool wxWindow_AcceptsFocusRecursively(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_DisableFocusFromKeyboard(wxWindow * self);
#endif
bool wxWindow_IsFocusable(const wxWindow * self);
bool wxWindow_CanAcceptFocus(const wxWindow * self);
bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow * self);
bool wxWindow_HasFocus(const wxWindow * self);
void wxWindow_SetCanFocus(wxWindow * self, bool can_focus);
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_EnableVisibleFocus(wxWindow * self, bool enable);
#endif
void wxWindow_SetFocus(wxWindow * self);
void wxWindow_SetFocusFromKbd(wxWindow * self);
void wxWindow_AddChild(wxWindow * self, wxWindow * child);
bool wxWindow_DestroyChildren(wxWindow * self);
wxWindow * wxWindow_FindWindow(const wxWindow * self, long id);
wxWindow * wxWindow_FindWindow1(const wxWindow * self, const wxString * name);
wxWindowList *wxWindow_GetChildren1(const wxWindow * self);
void wxWindow_RemoveChild(wxWindow * self, wxWindow * child);
wxWindow * wxWindow_GetGrandParent(const wxWindow * self);
wxWindow * wxWindow_GetNextSibling(const wxWindow * self);
wxWindow * wxWindow_GetParent(const wxWindow * self);
wxWindow * wxWindow_GetPrevSibling(const wxWindow * self);
bool wxWindow_IsDescendant(const wxWindow * self, wxWindow * win);
bool wxWindow_Reparent(wxWindow * self, wxWindow * new_parent);
void wxWindow_AlwaysShowScrollbars(wxWindow * self, bool hflag, bool vflag);
int wxWindow_GetScrollPos(const wxWindow * self, int orientation);
int wxWindow_GetScrollRange(const wxWindow * self, int orientation);
int wxWindow_GetScrollThumb(const wxWindow * self, int orientation);
bool wxWindow_CanScroll(const wxWindow * self, int orient);
bool wxWindow_HasScrollbar(const wxWindow * self, int orient);
bool wxWindow_IsScrollbarAlwaysShown(const wxWindow * self, int orient);
bool wxWindow_ScrollLines(wxWindow * self, int lines);
bool wxWindow_ScrollPages(wxWindow * self, int pages);
void wxWindow_ScrollWindow(wxWindow * self, int dx, int dy, const wxRect * rect);
bool wxWindow_LineUp(wxWindow * self);
bool wxWindow_LineDown(wxWindow * self);
bool wxWindow_PageUp(wxWindow * self);
bool wxWindow_PageDown(wxWindow * self);
void wxWindow_SetScrollPos(wxWindow * self, int orientation, int pos, bool refresh);
void wxWindow_SetScrollbar(wxWindow * self, int orientation, int position, int thumb_size, int range, bool refresh);
bool wxWindow_BeginRepositioningChildren(wxWindow * self);
void wxWindow_EndRepositioningChildren(wxWindow * self);
void wxWindow_CacheBestSize(const wxWindow * self, const wxSize * size);
wxSize *wxWindow_ClientToWindowSize(const wxWindow * self, const wxSize * size);
wxSize *wxWindow_WindowToClientSize(const wxWindow * self, const wxSize * size);
void wxWindow_Fit(wxWindow * self);
void wxWindow_FitInside(wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_FromDIP1(const wxWindow * self, const wxPoint * pt);
int wxWindow_FromDIP2(const wxWindow * self, int d);
wxSize *wxWindow_ToDIP(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ToDIP1(const wxWindow * self, const wxPoint * pt);
int wxWindow_ToDIP2(const wxWindow * self, int d);
wxSize *wxWindow_FromPhys(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_FromPhys1(const wxWindow * self, const wxPoint * pt);
int wxWindow_FromPhys2(const wxWindow * self, int d);
wxSize *wxWindow_ToPhys(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ToPhys1(const wxWindow * self, const wxPoint * pt);
int wxWindow_ToPhys2(const wxWindow * self, int d);
#endif
wxSize *wxWindow_GetBestSize(const wxWindow * self);
int wxWindow_GetBestHeight(const wxWindow * self, int width);
int wxWindow_GetBestWidth(const wxWindow * self, int height);
void wxWindow_GetClientSize(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetClientSize1(const wxWindow * self);
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow * self);
wxSize *wxWindow_GetMaxClientSize(const wxWindow * self);
wxSize *wxWindow_GetMaxSize(const wxWindow * self);
wxSize *wxWindow_GetMinClientSize(const wxWindow * self);
wxSize *wxWindow_GetMinSize(const wxWindow * self);
int wxWindow_GetMinWidth(const wxWindow * self);
int wxWindow_GetMinHeight(const wxWindow * self);
int wxWindow_GetMaxWidth(const wxWindow * self);
int wxWindow_GetMaxHeight(const wxWindow * self);
void wxWindow_GetSize(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetSize1(const wxWindow * self);
wxSize *wxWindow_GetVirtualSize(const wxWindow * self);
void wxWindow_GetVirtualSize1(const wxWindow * self, int * width, int * height);
wxSize *wxWindow_GetBestVirtualSize(const wxWindow * self);
double wxWindow_GetContentScaleFactor(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
double wxWindow_GetDPIScaleFactor(const wxWindow * self);
#endif
wxSize *wxWindow_GetWindowBorderSize(const wxWindow * self);
bool wxWindow_InformFirstDirection(wxWindow * self, int direction, int size, int available_other_dir);
void wxWindow_InvalidateBestSize(wxWindow * self);
void wxWindow_PostSizeEvent(wxWindow * self);
void wxWindow_PostSizeEventToParent(wxWindow * self);
void wxWindow_SendSizeEvent(wxWindow * self, int flags);
void wxWindow_SendSizeEventToParent(wxWindow * self, int flags);
void wxWindow_SetClientSize(wxWindow * self, int width, int height);
void wxWindow_SetClientSize1(wxWindow * self, const wxSize * size);
void wxWindow_SetClientSize2(wxWindow * self, const wxRect * rect);
void wxWindow_SetContainingSizer(wxWindow * self, wxSizer * sizer);
void wxWindow_SetInitialSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMaxClientSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMaxSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMinClientSize(wxWindow * self, const wxSize * size);
void wxWindow_SetMinSize(wxWindow * self, const wxSize * size);
void wxWindow_SetSize(wxWindow * self, int x, int y, int width, int height, int size_flags);
void wxWindow_SetSize1(wxWindow * self, const wxRect * rect);
void wxWindow_SetSize2(wxWindow * self, const wxSize * size);
void wxWindow_SetSize3(wxWindow * self, int width, int height);
void wxWindow_SetSizeHints(wxWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size);
void wxWindow_SetSizeHints1(wxWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h);
void wxWindow_SetVirtualSize(wxWindow * self, int width, int height);
void wxWindow_SetVirtualSize1(wxWindow * self, const wxSize * size);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_FromDIP4(const wxPoint * pt, const wxWindow * w);
int wxWindow_FromDIP5(int d, const wxWindow * w);
wxSize *wxWindow_ToDIP3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_ToDIP4(const wxPoint * pt, const wxWindow * w);
int wxWindow_ToDIP5(int d, const wxWindow * w);
wxSize *wxWindow_FromPhys3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_FromPhys4(const wxPoint * pt, const wxWindow * w);
int wxWindow_FromPhys5(int d, const wxWindow * w);
wxSize *wxWindow_ToPhys3(const wxSize * sz, const wxWindow * w);
wxPoint *wxWindow_ToPhys4(const wxPoint * pt, const wxWindow * w);
int wxWindow_ToPhys5(int d, const wxWindow * w);
#endif
void wxWindow_Center(wxWindow * self, int dir);
void wxWindow_CenterOnParent(wxWindow * self, int dir);
void wxWindow_Centre(wxWindow * self, int direction);
void wxWindow_CentreOnParent(wxWindow * self, int direction);
void wxWindow_GetPosition(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_GetPosition1(const wxWindow * self);
wxRect *wxWindow_GetRect(const wxWindow * self);
void wxWindow_GetScreenPosition(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_GetScreenPosition1(const wxWindow * self);
wxRect *wxWindow_GetScreenRect(const wxWindow * self);
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow * self);
wxRect *wxWindow_GetClientRect(const wxWindow * self);
void wxWindow_Move(wxWindow * self, int x, int y, int flags);
void wxWindow_Move1(wxWindow * self, const wxPoint * pt, int flags);
void wxWindow_SetPosition(wxWindow * self, const wxPoint * pt);
void wxWindow_ClientToScreen(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_ClientToScreen1(const wxWindow * self, const wxPoint * pt);
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow * self, const wxPoint * pt);
wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow * self, const wxSize * sz);
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow * self, const wxPoint * pt);
wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow * self, const wxSize * sz);
void wxWindow_ScreenToClient(const wxWindow * self, int * x, int * y);
wxPoint *wxWindow_ScreenToClient1(const wxWindow * self, const wxPoint * pt);
void wxWindow_ClearBackground(wxWindow * self);
void wxWindow_Freeze(wxWindow * self);
void wxWindow_Thaw(wxWindow * self);
bool wxWindow_IsFrozen(const wxWindow * self);
wxColour *wxWindow_GetBackgroundColour(const wxWindow * self);
int wxWindow_GetCharHeight(const wxWindow * self);
int wxWindow_GetCharWidth(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_GetDPI(const wxWindow * self);
#endif
wxFont *wxWindow_GetFont(const wxWindow * self);
wxColour *wxWindow_GetForegroundColour(const wxWindow * self);
void wxWindow_GetTextExtent(const wxWindow * self, const wxString * string, int * w, int * h, int * descent, int * external_leading, const wxFont * font);
wxSize *wxWindow_GetTextExtent1(const wxWindow * self, const wxString * string);
wxRect *wxWindow_GetUpdateClientRect(const wxWindow * self);
bool wxWindow_HasTransparentBackground(wxWindow * self);
void wxWindow_Refresh(wxWindow * self, bool erase_background, const wxRect * rect);
void wxWindow_RefreshRect(wxWindow * self, const wxRect * rect, bool erase_background);
void wxWindow_Update(wxWindow * self);
bool wxWindow_SetBackgroundColour(wxWindow * self, const wxColour * colour);
bool wxWindow_IsTransparentBackgroundSupported(const wxWindow * self, wxString * reason);
bool wxWindow_SetFont(wxWindow * self, const wxFont * font);
bool wxWindow_SetForegroundColour(wxWindow * self, const wxColour * colour);
void wxWindow_SetOwnBackgroundColour(wxWindow * self, const wxColour * colour);
bool wxWindow_InheritsBackgroundColour(const wxWindow * self);
bool wxWindow_UseBgCol(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseBackgroundColour(const wxWindow * self);
#endif
void wxWindow_SetOwnFont(wxWindow * self, const wxFont * font);
void wxWindow_SetOwnForegroundColour(wxWindow * self, const wxColour * colour);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseForegroundColour(const wxWindow * self);
bool wxWindow_InheritsForegroundColour(const wxWindow * self);
#endif
void wxWindow_SetPalette(wxWindow * self, const wxPalette * pal);
bool wxWindow_ShouldInheritColours(const wxWindow * self);
void wxWindow_SetThemeEnabled(wxWindow * self, bool enable);
bool wxWindow_GetThemeEnabled(const wxWindow * self);
bool wxWindow_CanSetTransparent(wxWindow * self);
bool wxWindow_SetTransparent(wxWindow * self, wxByte alpha);
wxEvtHandler * wxWindow_GetEventHandler(const wxWindow * self);
bool wxWindow_HandleAsNavigationKey(wxWindow * self, const wxKeyEvent * event);
bool wxWindow_HandleWindowEvent(const wxWindow * self, wxEvent * event);
bool wxWindow_ProcessWindowEvent(wxWindow * self, wxEvent * event);
bool wxWindow_ProcessWindowEventLocally(wxWindow * self, wxEvent * event);
wxEvtHandler * wxWindow_PopEventHandler(wxWindow * self, bool delete_handler);
void wxWindow_PushEventHandler(wxWindow * self, wxEvtHandler * handler);
bool wxWindow_RemoveEventHandler(wxWindow * self, wxEvtHandler * handler);
void wxWindow_SetEventHandler(wxWindow * self, wxEvtHandler * handler);
long wxWindow_GetExtraStyle(const wxWindow * self);
long wxWindow_GetWindowStyleFlag(const wxWindow * self);
long wxWindow_GetWindowStyle(const wxWindow * self);
bool wxWindow_HasExtraStyle(const wxWindow * self, int ex_flag);
bool wxWindow_HasFlag(const wxWindow * self, int flag);
void wxWindow_SetExtraStyle(wxWindow * self, long ex_style);
void wxWindow_SetWindowStyleFlag(wxWindow * self, long style);
void wxWindow_SetWindowStyle(wxWindow * self, long style);
bool wxWindow_ToggleWindowStyle(wxWindow * self, int flag);
void wxWindow_MoveAfterInTabOrder(wxWindow * self, wxWindow * win);
void wxWindow_MoveBeforeInTabOrder(wxWindow * self, wxWindow * win);
bool wxWindow_Navigate(wxWindow * self, int flags);
bool wxWindow_NavigateIn(wxWindow * self, int flags);
void wxWindow_Lower(wxWindow * self);
void wxWindow_Raise(wxWindow * self);
bool wxWindow_Hide(wxWindow * self);
bool wxWindow_IsEnabled(const wxWindow * self);
bool wxWindow_IsExposed(const wxWindow * self, int x, int y);
bool wxWindow_IsExposed1(const wxWindow * self, wxPoint * pt);
bool wxWindow_IsExposed2(const wxWindow * self, int x, int y, int w, int h);
bool wxWindow_IsExposed3(const wxWindow * self, wxRect * rect);
bool wxWindow_IsShown(const wxWindow * self);
bool wxWindow_IsShownOnScreen(const wxWindow * self);
bool wxWindow_Disable(wxWindow * self);
bool wxWindow_Enable(wxWindow * self, bool enable);
bool wxWindow_Show(wxWindow * self, bool show);
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxWindow_GetHelpText(const wxWindow * self);
#endif
void wxWindow_SetHelpText(wxWindow * self, const wxString * help_text);
wxToolTip * wxWindow_GetToolTip(const wxWindow * self);
wxString *wxWindow_GetToolTipText(const wxWindow * self);
void wxWindow_SetToolTip(wxWindow * self, const wxString * tip_string);
void wxWindow_SetToolTip1(wxWindow * self, wxToolTip * tip);
void wxWindow_UnsetToolTip(wxWindow * self);
int wxWindow_GetPopupMenuSelectionFromUser(wxWindow * self, wxMenu * menu, const wxPoint * pos);
int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow * self, wxMenu * menu, int x, int y);
bool wxWindow_PopupMenu(wxWindow * self, wxMenu * menu, const wxPoint * pos);
bool wxWindow_PopupMenu1(wxWindow * self, wxMenu * menu, int x, int y);
wxValidator * wxWindow_GetValidator(wxWindow * self);
void wxWindow_SetValidator(wxWindow * self, const wxValidator * validator);
bool wxWindow_TransferDataFromWindow(wxWindow * self);
bool wxWindow_TransferDataToWindow(wxWindow * self);
bool wxWindow_Validate(wxWindow * self);
wxWindowID wxWindow_GetId(const wxWindow * self);
wxString *wxWindow_GetLabel(const wxWindow * self);
wxLayoutDirection wxWindow_GetLayoutDirection(const wxWindow * self);
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total);
wxString *wxWindow_GetName(const wxWindow * self);
void wxWindow_SetId(wxWindow * self, wxWindowID winid);
void wxWindow_SetLabel(wxWindow * self, const wxString * label);
void wxWindow_SetLayoutDirection(wxWindow * self, wxLayoutDirection dir);
void wxWindow_SetName(wxWindow * self, const wxString * name);
wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow * self);
void wxWindow_SetAcceleratorTable(wxWindow * self, const wxAcceleratorTable * accel);
bool wxWindow_Close(wxWindow * self, bool force);
bool wxWindow_Destroy(wxWindow * self);
bool wxWindow_IsBeingDeleted(const wxWindow * self);
wxDropTarget * wxWindow_GetDropTarget(const wxWindow * self);
void wxWindow_SetDropTarget(wxWindow * self, wxDropTarget * target);
void wxWindow_DragAcceptFiles(wxWindow * self, bool accept);
wxSizer * wxWindow_GetContainingSizer(const wxWindow * self);
wxSizer * wxWindow_GetSizer(const wxWindow * self);
void wxWindow_SetSizer(wxWindow * self, wxSizer * sizer, bool delete_old);
void wxWindow_SetSizerAndFit(wxWindow * self, wxSizer * sizer, bool delete_old);
wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow * self);
void wxWindow_SetConstraints(wxWindow * self, wxLayoutConstraints * constraints);
bool wxWindow_Layout(wxWindow * self);
void wxWindow_SetAutoLayout(wxWindow * self, bool auto_layout);
bool wxWindow_GetAutoLayout(const wxWindow * self);
void wxWindow_CaptureMouse(wxWindow * self);
wxCaret * wxWindow_GetCaret(const wxWindow * self);
bool wxWindow_HasCapture(const wxWindow * self);
void wxWindow_ReleaseMouse(wxWindow * self);
void wxWindow_SetCaret(wxWindow * self, wxCaret * caret);
bool wxWindow_SetCursor(wxWindow * self, const wxCursor * cursor);
void wxWindow_WarpPointer(wxWindow * self, int x, int y);
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_EnableTouchEvents(wxWindow * self, int events_mask);
#endif
void wxWindow_DoUpdateWindowUI(wxWindow * self, wxUpdateUIEvent * event);
bool wxWindow_HasMultiplePages(const wxWindow * self);
void wxWindow_InheritAttributes(wxWindow * self);
void wxWindow_InitDialog(wxWindow * self);
bool wxWindow_IsDoubleBuffered(const wxWindow * self);
void wxWindow_SetDoubleBuffered(wxWindow * self, bool on);
bool wxWindow_IsRetained(const wxWindow * self);
bool wxWindow_IsThisEnabled(const wxWindow * self);
bool wxWindow_IsTopLevel(const wxWindow * self);
void wxWindow_OnInternalIdle(wxWindow * self);
bool wxWindow_SendIdleEvents(wxWindow * self, wxIdleEvent * event);
#ifndef __WXGTK__
bool wxWindow_RegisterHotKey(wxWindow * self, int hotkey_id, int modifiers, int virtual_key_code);
bool wxWindow_UnregisterHotKey(wxWindow * self, int hotkey_id);
#endif
void wxWindow_UpdateWindowUI(wxWindow * self, long flags);
wxWindow * wxWindow_FindFocus();
wxWindow * wxWindow_FindWindowById(long id, const wxWindow * parent);
wxWindow * wxWindow_FindWindowByLabel(const wxString * label, const wxWindow * parent);
wxWindow * wxWindow_FindWindowByName(const wxString * name, const wxWindow * parent);
wxWindow * wxWindow_GetCapture();
wxWindowID wxWindow_NewControlId(int count);
void wxWindow_UnreserveControlId(wxWindowID id, int count);
wxWindow *wxWindow_new();
wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxWindow_Create(wxWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxWindowCreateEvent
wxClassInfo *wxWindowCreateEvent_CLASSINFO();
wxWindowCreateEvent *wxWindowCreateEvent_new(wxWindow * win);
wxWindow * wxWindowCreateEvent_GetWindow(const wxWindowCreateEvent * self);

// CLASS: wxWindowDC
wxClassInfo *wxWindowDC_CLASSINFO();
wxWindowDC *wxWindowDC_new(wxWindow * window);

// CLASS: wxWindowDestroyEvent
wxClassInfo *wxWindowDestroyEvent_CLASSINFO();
wxWindowDestroyEvent *wxWindowDestroyEvent_new(wxWindow * win);
wxWindow * wxWindowDestroyEvent_GetWindow(const wxWindowDestroyEvent * self);

// CLASS: wxWindowDisabler
void wxWindowDisabler_delete(wxWindowDisabler *self);
wxWindowDisabler *wxWindowDisabler_new(bool disable);
#if wxCHECK_VERSION(3, 1, 7)
wxWindowDisabler *wxWindowDisabler_new1(wxWindow * win_to_skip, wxWindow * win_to_skip2);
#endif

// CLASS: wxWizard
wxClassInfo *wxWizard_CLASSINFO();
wxWizard *wxWizard_new();
wxWizard *wxWizard_new1(wxWindow * parent, int id, const wxString * title, const wxBitmapBundle * bitmap, const wxPoint * pos, long style);
bool wxWizard_Create(wxWizard * self, wxWindow * parent, int id, const wxString * title, const wxBitmapBundle * bitmap, const wxPoint * pos, long style);
void wxWizard_FitToPage(wxWizard * self, const wxWizardPage * first_page);
wxBitmap *wxWizard_GetBitmap(const wxWizard * self);
wxColour *wxWizard_GetBitmapBackgroundColour(const wxWizard * self);
int wxWizard_GetBitmapPlacement(const wxWizard * self);
wxWizardPage * wxWizard_GetCurrentPage(const wxWizard * self);
int wxWizard_GetMinimumBitmapWidth(const wxWizard * self);
wxSizer * wxWizard_GetPageAreaSizer(const wxWizard * self);
wxSize *wxWizard_GetPageSize(const wxWizard * self);
bool wxWizard_HasNextPage(wxWizard * self, wxWizardPage * page);
bool wxWizard_HasPrevPage(wxWizard * self, wxWizardPage * page);
bool wxWizard_RunWizard(wxWizard * self, wxWizardPage * first_page);
void wxWizard_SetBitmap(wxWizard * self, const wxBitmapBundle * bitmap);
void wxWizard_SetBitmapBackgroundColour(wxWizard * self, const wxColour * colour);
void wxWizard_SetBitmapPlacement(wxWizard * self, int placement);
void wxWizard_SetBorder(wxWizard * self, int border);
void wxWizard_SetMinimumBitmapWidth(wxWizard * self, int width);
void wxWizard_SetPageSize(wxWizard * self, const wxSize * size_page);

// CLASS: wxWizardEvent
wxClassInfo *wxWizardEvent_CLASSINFO();
bool wxWizardEvent_GetDirection(const wxWizardEvent * self);
wxWizardPage * wxWizardEvent_GetPage(const wxWizardEvent * self);

// CLASS: wxWizardPage
wxClassInfo *wxWizardPage_CLASSINFO();
bool wxWizardPage_Create(wxWizardPage * self, wxWizard * parent, const wxBitmapBundle * bitmap);
wxBitmap *wxWizardPage_GetBitmap(const wxWizardPage * self);
wxWizardPage * wxWizardPage_GetNext(const wxWizardPage * self);
wxWizardPage * wxWizardPage_GetPrev(const wxWizardPage * self);

// CLASS: wxWizardPageSimple
wxClassInfo *wxWizardPageSimple_CLASSINFO();
wxWizardPageSimple *wxWizardPageSimple_new();
wxWizardPageSimple *wxWizardPageSimple_new1(wxWizard * parent, wxWizardPage * prev, wxWizardPage * next, const wxBitmapBundle * bitmap);
bool wxWizardPageSimple_Create(wxWizardPageSimple * self, wxWizard * parent, wxWizardPage * prev, wxWizardPage * next, const wxBitmapBundle * bitmap);
wxWizardPageSimple * wxWizardPageSimple_Chain(wxWizardPageSimple * self, wxWizardPageSimple * next);
void wxWizardPageSimple_SetNext(wxWizardPageSimple * self, wxWizardPage * next);
void wxWizardPageSimple_SetPrev(wxWizardPageSimple * self, wxWizardPage * prev);
void wxWizardPageSimple_Chain1(wxWizardPageSimple * first, wxWizardPageSimple * second);

// CLASS: wxWrapSizer
wxClassInfo *wxWrapSizer_CLASSINFO();
wxWrapSizer *wxWrapSizer_new(int orient, int flags);

} // extern "C"

