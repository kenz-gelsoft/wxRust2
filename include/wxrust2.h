#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxObject
inline wxObject *wxObject_new() {
    return new wxObject();
}
inline wxObject *wxObject_new1(const wxObject & other) {
    return new wxObject(other);
}
inline wxClassInfo * wxObject_GetClassInfo(const wxObject & self) {
    return self.GetClassInfo();
}
inline wxObjectRefData * wxObject_GetRefData(const wxObject & self) {
    return self.GetRefData();
}
inline bool wxObject_IsKindOf(const wxObject & self, const wxClassInfo * info) {
    return self.IsKindOf(info);
}
inline bool wxObject_IsSameAs(const wxObject & self, const wxObject * obj) {
    return self.IsSameAs(*obj);
}
inline void wxObject_Ref(wxObject & self, const wxObject & clone) {
    return self.Ref(clone);
}
inline void wxObject_SetRefData(wxObject & self, wxObjectRefData * data) {
    return self.SetRefData(data);
}
inline void wxObject_UnRef(wxObject & self) {
    return self.UnRef();
}
inline void wxObject_UnShare(wxObject & self) {
    return self.UnShare();
}

// CLASS: wxEvtHandler
inline void wxEvtHandler_QueueEvent(wxEvtHandler & self, wxEvent * event) {
    return self.QueueEvent(event);
}
inline void wxEvtHandler_AddPendingEvent(wxEvtHandler & self, const wxEvent & event) {
    return self.AddPendingEvent(event);
}
inline bool wxEvtHandler_ProcessEvent(wxEvtHandler & self, wxEvent & event) {
    return self.ProcessEvent(event);
}
inline bool wxEvtHandler_ProcessEventLocally(wxEvtHandler & self, wxEvent & event) {
    return self.ProcessEventLocally(event);
}
inline bool wxEvtHandler_SafelyProcessEvent(wxEvtHandler & self, wxEvent & event) {
    return self.SafelyProcessEvent(event);
}
inline void wxEvtHandler_ProcessPendingEvents(wxEvtHandler & self) {
    return self.ProcessPendingEvents();
}
inline void wxEvtHandler_DeletePendingEvents(wxEvtHandler & self) {
    return self.DeletePendingEvents();
}
inline wxClientData * wxEvtHandler_GetClientObject(const wxEvtHandler & self) {
    return self.GetClientObject();
}
inline void wxEvtHandler_SetClientObject(wxEvtHandler & self, wxClientData * data) {
    return self.SetClientObject(data);
}
inline bool wxEvtHandler_GetEvtHandlerEnabled(const wxEvtHandler & self) {
    return self.GetEvtHandlerEnabled();
}
inline wxEvtHandler * wxEvtHandler_GetNextHandler(const wxEvtHandler & self) {
    return self.GetNextHandler();
}
inline wxEvtHandler * wxEvtHandler_GetPreviousHandler(const wxEvtHandler & self) {
    return self.GetPreviousHandler();
}
inline void wxEvtHandler_SetEvtHandlerEnabled(wxEvtHandler & self, bool enabled) {
    return self.SetEvtHandlerEnabled(enabled);
}
inline void wxEvtHandler_SetNextHandler(wxEvtHandler & self, wxEvtHandler * handler) {
    return self.SetNextHandler(handler);
}
inline void wxEvtHandler_SetPreviousHandler(wxEvtHandler & self, wxEvtHandler * handler) {
    return self.SetPreviousHandler(handler);
}
inline void wxEvtHandler_Unlink(wxEvtHandler & self) {
    return self.Unlink();
}
inline bool wxEvtHandler_IsUnlinked(const wxEvtHandler & self) {
    return self.IsUnlinked();
}
inline void wxEvtHandler_AddFilter(wxEventFilter * filter) {
    return wxEvtHandler::AddFilter(filter);
}
inline void wxEvtHandler_RemoveFilter(wxEventFilter * filter) {
    return wxEvtHandler::RemoveFilter(filter);
}
inline wxEvtHandler *wxEvtHandler_new() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
inline bool wxWindow_AcceptsFocus(const wxWindow & self) {
    return self.AcceptsFocus();
}
inline bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow & self) {
    return self.AcceptsFocusFromKeyboard();
}
inline bool wxWindow_AcceptsFocusRecursively(const wxWindow & self) {
    return self.AcceptsFocusRecursively();
}
inline void wxWindow_DisableFocusFromKeyboard(wxWindow & self) {
    return self.DisableFocusFromKeyboard();
}
inline bool wxWindow_IsFocusable(const wxWindow & self) {
    return self.IsFocusable();
}
inline bool wxWindow_CanAcceptFocus(const wxWindow & self) {
    return self.CanAcceptFocus();
}
inline bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow & self) {
    return self.CanAcceptFocusFromKeyboard();
}
inline bool wxWindow_HasFocus(const wxWindow & self) {
    return self.HasFocus();
}
inline void wxWindow_SetCanFocus(wxWindow & self, bool can_focus) {
    return self.SetCanFocus(can_focus);
}
inline void wxWindow_EnableVisibleFocus(wxWindow & self, bool enable) {
    return self.EnableVisibleFocus(enable);
}
inline void wxWindow_SetFocus(wxWindow & self) {
    return self.SetFocus();
}
inline void wxWindow_SetFocusFromKbd(wxWindow & self) {
    return self.SetFocusFromKbd();
}
inline void wxWindow_AddChild(wxWindow & self, wxWindow * child) {
    return self.AddChild(child);
}
inline bool wxWindow_DestroyChildren(wxWindow & self) {
    return self.DestroyChildren();
}
inline wxWindow * wxWindow_FindWindow(const wxWindow & self, int32_t id) {
    return self.FindWindow(id);
}
inline wxWindow * wxWindow_FindWindow1(const wxWindow & self, const wxString & name) {
    return self.FindWindow(name);
}
inline void wxWindow_RemoveChild(wxWindow & self, wxWindow * child) {
    return self.RemoveChild(child);
}
inline wxWindow * wxWindow_GetGrandParent(const wxWindow & self) {
    return self.GetGrandParent();
}
inline wxWindow * wxWindow_GetNextSibling(const wxWindow & self) {
    return self.GetNextSibling();
}
inline wxWindow * wxWindow_GetParent(const wxWindow & self) {
    return self.GetParent();
}
inline wxWindow * wxWindow_GetPrevSibling(const wxWindow & self) {
    return self.GetPrevSibling();
}
inline bool wxWindow_IsDescendant(const wxWindow & self, wxWindow * win) {
    return self.IsDescendant(win);
}
inline bool wxWindow_Reparent(wxWindow & self, wxWindow * new_parent) {
    return self.Reparent(new_parent);
}
inline void wxWindow_AlwaysShowScrollbars(wxWindow & self, bool hflag, bool vflag) {
    return self.AlwaysShowScrollbars(hflag, vflag);
}
inline int wxWindow_GetScrollPos(const wxWindow & self, int orientation) {
    return self.GetScrollPos(orientation);
}
inline int wxWindow_GetScrollRange(const wxWindow & self, int orientation) {
    return self.GetScrollRange(orientation);
}
inline int wxWindow_GetScrollThumb(const wxWindow & self, int orientation) {
    return self.GetScrollThumb(orientation);
}
inline bool wxWindow_CanScroll(const wxWindow & self, int orient) {
    return self.CanScroll(orient);
}
inline bool wxWindow_HasScrollbar(const wxWindow & self, int orient) {
    return self.HasScrollbar(orient);
}
inline bool wxWindow_IsScrollbarAlwaysShown(const wxWindow & self, int orient) {
    return self.IsScrollbarAlwaysShown(orient);
}
inline bool wxWindow_ScrollLines(wxWindow & self, int lines) {
    return self.ScrollLines(lines);
}
inline bool wxWindow_ScrollPages(wxWindow & self, int pages) {
    return self.ScrollPages(pages);
}
inline void wxWindow_ScrollWindow(wxWindow & self, int dx, int dy, const wxRect * rect) {
    return self.ScrollWindow(dx, dy, rect);
}
inline bool wxWindow_LineUp(wxWindow & self) {
    return self.LineUp();
}
inline bool wxWindow_LineDown(wxWindow & self) {
    return self.LineDown();
}
inline bool wxWindow_PageUp(wxWindow & self) {
    return self.PageUp();
}
inline bool wxWindow_PageDown(wxWindow & self) {
    return self.PageDown();
}
inline void wxWindow_SetScrollPos(wxWindow & self, int orientation, int pos, bool refresh) {
    return self.SetScrollPos(orientation, pos, refresh);
}
inline void wxWindow_SetScrollbar(wxWindow & self, int orientation, int position, int thumb_size, int range, bool refresh) {
    return self.SetScrollbar(orientation, position, thumb_size, range, refresh);
}
inline bool wxWindow_BeginRepositioningChildren(wxWindow & self) {
    return self.BeginRepositioningChildren();
}
inline void wxWindow_EndRepositioningChildren(wxWindow & self) {
    return self.EndRepositioningChildren();
}
inline void wxWindow_CacheBestSize(const wxWindow & self, const wxSize & size) {
    return self.CacheBestSize(size);
}
inline wxSize *wxWindow_ClientToWindowSize(const wxWindow & self, const wxSize & size) {
    return new wxSize(self.ClientToWindowSize(size));
}
inline wxSize *wxWindow_WindowToClientSize(const wxWindow & self, const wxSize & size) {
    return new wxSize(self.WindowToClientSize(size));
}
inline void wxWindow_Fit(wxWindow & self) {
    return self.Fit();
}
inline void wxWindow_FitInside(wxWindow & self) {
    return self.FitInside();
}
inline wxSize *wxWindow_FromDIP(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.FromDIP(sz));
}
inline wxPoint *wxWindow_FromDIP1(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.FromDIP(pt));
}
inline int wxWindow_FromDIP2(const wxWindow & self, int d) {
    return self.FromDIP(d);
}
inline wxSize *wxWindow_ToDIP(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ToDIP(sz));
}
inline wxPoint *wxWindow_ToDIP1(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ToDIP(pt));
}
inline int wxWindow_ToDIP2(const wxWindow & self, int d) {
    return self.ToDIP(d);
}
inline wxSize *wxWindow_GetBestSize(const wxWindow & self) {
    return new wxSize(self.GetBestSize());
}
inline int wxWindow_GetBestHeight(const wxWindow & self, int width) {
    return self.GetBestHeight(width);
}
inline int wxWindow_GetBestWidth(const wxWindow & self, int height) {
    return self.GetBestWidth(height);
}
inline void wxWindow_GetClientSize(const wxWindow & self, int * width, int * height) {
    return self.GetClientSize(width, height);
}
inline wxSize *wxWindow_GetClientSize1(const wxWindow & self) {
    return new wxSize(self.GetClientSize());
}
inline wxSize *wxWindow_GetEffectiveMinSize(const wxWindow & self) {
    return new wxSize(self.GetEffectiveMinSize());
}
inline wxSize *wxWindow_GetMaxClientSize(const wxWindow & self) {
    return new wxSize(self.GetMaxClientSize());
}
inline wxSize *wxWindow_GetMaxSize(const wxWindow & self) {
    return new wxSize(self.GetMaxSize());
}
inline wxSize *wxWindow_GetMinClientSize(const wxWindow & self) {
    return new wxSize(self.GetMinClientSize());
}
inline wxSize *wxWindow_GetMinSize(const wxWindow & self) {
    return new wxSize(self.GetMinSize());
}
inline int wxWindow_GetMinWidth(const wxWindow & self) {
    return self.GetMinWidth();
}
inline int wxWindow_GetMinHeight(const wxWindow & self) {
    return self.GetMinHeight();
}
inline int wxWindow_GetMaxWidth(const wxWindow & self) {
    return self.GetMaxWidth();
}
inline int wxWindow_GetMaxHeight(const wxWindow & self) {
    return self.GetMaxHeight();
}
inline void wxWindow_GetSize(const wxWindow & self, int * width, int * height) {
    return self.GetSize(width, height);
}
inline wxSize *wxWindow_GetSize1(const wxWindow & self) {
    return new wxSize(self.GetSize());
}
inline wxSize *wxWindow_GetVirtualSize(const wxWindow & self) {
    return new wxSize(self.GetVirtualSize());
}
inline void wxWindow_GetVirtualSize1(const wxWindow & self, int * width, int * height) {
    return self.GetVirtualSize(width, height);
}
inline wxSize *wxWindow_GetBestVirtualSize(const wxWindow & self) {
    return new wxSize(self.GetBestVirtualSize());
}
inline double wxWindow_GetContentScaleFactor(const wxWindow & self) {
    return self.GetContentScaleFactor();
}
inline double wxWindow_GetDPIScaleFactor(const wxWindow & self) {
    return self.GetDPIScaleFactor();
}
inline wxSize *wxWindow_GetWindowBorderSize(const wxWindow & self) {
    return new wxSize(self.GetWindowBorderSize());
}
inline bool wxWindow_InformFirstDirection(wxWindow & self, int direction, int size, int available_other_dir) {
    return self.InformFirstDirection(direction, size, available_other_dir);
}
inline void wxWindow_InvalidateBestSize(wxWindow & self) {
    return self.InvalidateBestSize();
}
inline void wxWindow_PostSizeEvent(wxWindow & self) {
    return self.PostSizeEvent();
}
inline void wxWindow_PostSizeEventToParent(wxWindow & self) {
    return self.PostSizeEventToParent();
}
inline void wxWindow_SendSizeEvent(wxWindow & self, int flags) {
    return self.SendSizeEvent(flags);
}
inline void wxWindow_SendSizeEventToParent(wxWindow & self, int flags) {
    return self.SendSizeEventToParent(flags);
}
inline void wxWindow_SetClientSize(wxWindow & self, int width, int height) {
    return self.SetClientSize(width, height);
}
inline void wxWindow_SetClientSize1(wxWindow & self, const wxSize & size) {
    return self.SetClientSize(size);
}
inline void wxWindow_SetClientSize2(wxWindow & self, const wxRect & rect) {
    return self.SetClientSize(rect);
}
inline void wxWindow_SetContainingSizer(wxWindow & self, wxSizer * sizer) {
    return self.SetContainingSizer(sizer);
}
inline void wxWindow_SetInitialSize(wxWindow & self, const wxSize & size) {
    return self.SetInitialSize(size);
}
inline void wxWindow_SetMaxClientSize(wxWindow & self, const wxSize & size) {
    return self.SetMaxClientSize(size);
}
inline void wxWindow_SetMaxSize(wxWindow & self, const wxSize & size) {
    return self.SetMaxSize(size);
}
inline void wxWindow_SetMinClientSize(wxWindow & self, const wxSize & size) {
    return self.SetMinClientSize(size);
}
inline void wxWindow_SetMinSize(wxWindow & self, const wxSize & size) {
    return self.SetMinSize(size);
}
inline void wxWindow_SetSize(wxWindow & self, int x, int y, int width, int height, int size_flags) {
    return self.SetSize(x, y, width, height, size_flags);
}
inline void wxWindow_SetSize1(wxWindow & self, const wxRect & rect) {
    return self.SetSize(rect);
}
inline void wxWindow_SetSize2(wxWindow & self, const wxSize & size) {
    return self.SetSize(size);
}
inline void wxWindow_SetSize3(wxWindow & self, int width, int height) {
    return self.SetSize(width, height);
}
inline void wxWindow_SetSizeHints(wxWindow & self, const wxSize & min_size, const wxSize & max_size, const wxSize & inc_size) {
    return self.SetSizeHints(min_size, max_size, inc_size);
}
inline void wxWindow_SetSizeHints1(wxWindow & self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return self.SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
inline void wxWindow_SetVirtualSize(wxWindow & self, int width, int height) {
    return self.SetVirtualSize(width, height);
}
inline void wxWindow_SetVirtualSize1(wxWindow & self, const wxSize & size) {
    return self.SetVirtualSize(size);
}
inline wxSize *wxWindow_FromDIP3(const wxSize & sz, const wxWindow * w) {
    return new wxSize(wxWindow::FromDIP(sz, w));
}
inline wxPoint *wxWindow_FromDIP4(const wxPoint & pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromDIP(pt, w));
}
inline int wxWindow_FromDIP5(int d, const wxWindow * w) {
    return wxWindow::FromDIP(d, w);
}
inline wxSize *wxWindow_ToDIP3(const wxSize & sz, const wxWindow * w) {
    return new wxSize(wxWindow::ToDIP(sz, w));
}
inline wxPoint *wxWindow_ToDIP4(const wxPoint & pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToDIP(pt, w));
}
inline int wxWindow_ToDIP5(int d, const wxWindow * w) {
    return wxWindow::ToDIP(d, w);
}
inline void wxWindow_Center(wxWindow & self, int dir) {
    return self.Center(dir);
}
inline void wxWindow_CenterOnParent(wxWindow & self, int dir) {
    return self.CenterOnParent(dir);
}
inline void wxWindow_Centre(wxWindow & self, int direction) {
    return self.Centre(direction);
}
inline void wxWindow_CentreOnParent(wxWindow & self, int direction) {
    return self.CentreOnParent(direction);
}
inline void wxWindow_GetPosition(const wxWindow & self, int * x, int * y) {
    return self.GetPosition(x, y);
}
inline wxPoint *wxWindow_GetPosition1(const wxWindow & self) {
    return new wxPoint(self.GetPosition());
}
inline wxRect *wxWindow_GetRect(const wxWindow & self) {
    return new wxRect(self.GetRect());
}
inline void wxWindow_GetScreenPosition(const wxWindow & self, int * x, int * y) {
    return self.GetScreenPosition(x, y);
}
inline wxPoint *wxWindow_GetScreenPosition1(const wxWindow & self) {
    return new wxPoint(self.GetScreenPosition());
}
inline wxRect *wxWindow_GetScreenRect(const wxWindow & self) {
    return new wxRect(self.GetScreenRect());
}
inline wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow & self) {
    return new wxPoint(self.GetClientAreaOrigin());
}
inline wxRect *wxWindow_GetClientRect(const wxWindow & self) {
    return new wxRect(self.GetClientRect());
}
inline void wxWindow_Move(wxWindow & self, int x, int y, int flags) {
    return self.Move(x, y, flags);
}
inline void wxWindow_Move1(wxWindow & self, const wxPoint & pt, int flags) {
    return self.Move(pt, flags);
}
inline void wxWindow_SetPosition(wxWindow & self, const wxPoint & pt) {
    return self.SetPosition(pt);
}
inline void wxWindow_ClientToScreen(const wxWindow & self, int * x, int * y) {
    return self.ClientToScreen(x, y);
}
inline wxPoint *wxWindow_ClientToScreen1(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ClientToScreen(pt));
}
inline wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertDialogToPixels(pt));
}
inline wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ConvertDialogToPixels(sz));
}
inline wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertPixelsToDialog(pt));
}
inline wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ConvertPixelsToDialog(sz));
}
inline void wxWindow_ScreenToClient(const wxWindow & self, int * x, int * y) {
    return self.ScreenToClient(x, y);
}
inline wxPoint *wxWindow_ScreenToClient1(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ScreenToClient(pt));
}
inline void wxWindow_ClearBackground(wxWindow & self) {
    return self.ClearBackground();
}
inline void wxWindow_Freeze(wxWindow & self) {
    return self.Freeze();
}
inline void wxWindow_Thaw(wxWindow & self) {
    return self.Thaw();
}
inline bool wxWindow_IsFrozen(const wxWindow & self) {
    return self.IsFrozen();
}
inline int wxWindow_GetCharHeight(const wxWindow & self) {
    return self.GetCharHeight();
}
inline int wxWindow_GetCharWidth(const wxWindow & self) {
    return self.GetCharWidth();
}
inline wxSize *wxWindow_GetDPI(const wxWindow & self) {
    return new wxSize(self.GetDPI());
}
inline void wxWindow_GetTextExtent(const wxWindow & self, const wxString & string, int * w, int * h, int * descent, int * external_leading, const wxFont * font) {
    return self.GetTextExtent(string, w, h, descent, external_leading, font);
}
inline wxSize *wxWindow_GetTextExtent1(const wxWindow & self, const wxString & string) {
    return new wxSize(self.GetTextExtent(string));
}
inline wxRect *wxWindow_GetUpdateClientRect(const wxWindow & self) {
    return new wxRect(self.GetUpdateClientRect());
}
inline bool wxWindow_HasTransparentBackground(wxWindow & self) {
    return self.HasTransparentBackground();
}
inline void wxWindow_Refresh(wxWindow & self, bool erase_background, const wxRect * rect) {
    return self.Refresh(erase_background, rect);
}
inline void wxWindow_RefreshRect(wxWindow & self, const wxRect & rect, bool erase_background) {
    return self.RefreshRect(rect, erase_background);
}
inline void wxWindow_Update(wxWindow & self) {
    return self.Update();
}
inline bool wxWindow_SetBackgroundColour(wxWindow & self, const wxColour & colour) {
    return self.SetBackgroundColour(colour);
}
inline bool wxWindow_IsTransparentBackgroundSupported(const wxWindow & self, wxString * reason) {
    return self.IsTransparentBackgroundSupported(reason);
}
inline bool wxWindow_SetFont(wxWindow & self, const wxFont & font) {
    return self.SetFont(font);
}
inline bool wxWindow_SetForegroundColour(wxWindow & self, const wxColour & colour) {
    return self.SetForegroundColour(colour);
}
inline void wxWindow_SetOwnBackgroundColour(wxWindow & self, const wxColour & colour) {
    return self.SetOwnBackgroundColour(colour);
}
inline bool wxWindow_InheritsBackgroundColour(const wxWindow & self) {
    return self.InheritsBackgroundColour();
}
inline bool wxWindow_UseBgCol(const wxWindow & self) {
    return self.UseBgCol();
}
inline bool wxWindow_UseBackgroundColour(const wxWindow & self) {
    return self.UseBackgroundColour();
}
inline void wxWindow_SetOwnFont(wxWindow & self, const wxFont & font) {
    return self.SetOwnFont(font);
}
inline void wxWindow_SetOwnForegroundColour(wxWindow & self, const wxColour & colour) {
    return self.SetOwnForegroundColour(colour);
}
inline bool wxWindow_UseForegroundColour(const wxWindow & self) {
    return self.UseForegroundColour();
}
inline bool wxWindow_InheritsForegroundColour(const wxWindow & self) {
    return self.InheritsForegroundColour();
}
inline void wxWindow_SetPalette(wxWindow & self, const wxPalette & pal) {
    return self.SetPalette(pal);
}
inline bool wxWindow_ShouldInheritColours(const wxWindow & self) {
    return self.ShouldInheritColours();
}
inline void wxWindow_SetThemeEnabled(wxWindow & self, bool enable) {
    return self.SetThemeEnabled(enable);
}
inline bool wxWindow_GetThemeEnabled(const wxWindow & self) {
    return self.GetThemeEnabled();
}
inline bool wxWindow_CanSetTransparent(wxWindow & self) {
    return self.CanSetTransparent();
}
inline bool wxWindow_SetTransparent(wxWindow & self, wxByte alpha) {
    return self.SetTransparent(alpha);
}
inline wxEvtHandler * wxWindow_GetEventHandler(const wxWindow & self) {
    return self.GetEventHandler();
}
inline bool wxWindow_HandleAsNavigationKey(wxWindow & self, const wxKeyEvent & event) {
    return self.HandleAsNavigationKey(event);
}
inline bool wxWindow_HandleWindowEvent(const wxWindow & self, wxEvent & event) {
    return self.HandleWindowEvent(event);
}
inline bool wxWindow_ProcessWindowEvent(wxWindow & self, wxEvent & event) {
    return self.ProcessWindowEvent(event);
}
inline bool wxWindow_ProcessWindowEventLocally(wxWindow & self, wxEvent & event) {
    return self.ProcessWindowEventLocally(event);
}
inline wxEvtHandler * wxWindow_PopEventHandler(wxWindow & self, bool delete_handler) {
    return self.PopEventHandler(delete_handler);
}
inline void wxWindow_PushEventHandler(wxWindow & self, wxEvtHandler * handler) {
    return self.PushEventHandler(handler);
}
inline bool wxWindow_RemoveEventHandler(wxWindow & self, wxEvtHandler * handler) {
    return self.RemoveEventHandler(handler);
}
inline void wxWindow_SetEventHandler(wxWindow & self, wxEvtHandler * handler) {
    return self.SetEventHandler(handler);
}
inline void wxWindow_SetNextHandler(wxWindow & self, wxEvtHandler * handler) {
    return self.SetNextHandler(handler);
}
inline void wxWindow_SetPreviousHandler(wxWindow & self, wxEvtHandler * handler) {
    return self.SetPreviousHandler(handler);
}
inline int32_t wxWindow_GetExtraStyle(const wxWindow & self) {
    return self.GetExtraStyle();
}
inline int32_t wxWindow_GetWindowStyleFlag(const wxWindow & self) {
    return self.GetWindowStyleFlag();
}
inline int32_t wxWindow_GetWindowStyle(const wxWindow & self) {
    return self.GetWindowStyle();
}
inline bool wxWindow_HasExtraStyle(const wxWindow & self, int ex_flag) {
    return self.HasExtraStyle(ex_flag);
}
inline bool wxWindow_HasFlag(const wxWindow & self, int flag) {
    return self.HasFlag(flag);
}
inline void wxWindow_SetExtraStyle(wxWindow & self, int32_t ex_style) {
    return self.SetExtraStyle(ex_style);
}
inline void wxWindow_SetWindowStyleFlag(wxWindow & self, int32_t style) {
    return self.SetWindowStyleFlag(style);
}
inline void wxWindow_SetWindowStyle(wxWindow & self, int32_t style) {
    return self.SetWindowStyle(style);
}
inline bool wxWindow_ToggleWindowStyle(wxWindow & self, int flag) {
    return self.ToggleWindowStyle(flag);
}
inline void wxWindow_MoveAfterInTabOrder(wxWindow & self, wxWindow * win) {
    return self.MoveAfterInTabOrder(win);
}
inline void wxWindow_MoveBeforeInTabOrder(wxWindow & self, wxWindow * win) {
    return self.MoveBeforeInTabOrder(win);
}
inline bool wxWindow_Navigate(wxWindow & self, int flags) {
    return self.Navigate(flags);
}
inline bool wxWindow_NavigateIn(wxWindow & self, int flags) {
    return self.NavigateIn(flags);
}
inline void wxWindow_Lower(wxWindow & self) {
    return self.Lower();
}
inline void wxWindow_Raise(wxWindow & self) {
    return self.Raise();
}
inline bool wxWindow_Hide(wxWindow & self) {
    return self.Hide();
}
inline bool wxWindow_IsEnabled(const wxWindow & self) {
    return self.IsEnabled();
}
inline bool wxWindow_IsExposed(const wxWindow & self, int x, int y) {
    return self.IsExposed(x, y);
}
inline bool wxWindow_IsExposed1(const wxWindow & self, wxPoint & pt) {
    return self.IsExposed(pt);
}
inline bool wxWindow_IsExposed2(const wxWindow & self, int x, int y, int w, int h) {
    return self.IsExposed(x, y, w, h);
}
inline bool wxWindow_IsExposed3(const wxWindow & self, wxRect & rect) {
    return self.IsExposed(rect);
}
inline bool wxWindow_IsShown(const wxWindow & self) {
    return self.IsShown();
}
inline bool wxWindow_IsShownOnScreen(const wxWindow & self) {
    return self.IsShownOnScreen();
}
inline bool wxWindow_Disable(wxWindow & self) {
    return self.Disable();
}
inline bool wxWindow_Enable(wxWindow & self, bool enable) {
    return self.Enable(enable);
}
inline bool wxWindow_Show(wxWindow & self, bool show) {
    return self.Show(show);
}
inline wxString *wxWindow_GetHelpText(const wxWindow & self) {
    return new wxString(self.GetHelpText());
}
inline void wxWindow_SetHelpText(wxWindow & self, const wxString & help_text) {
    return self.SetHelpText(help_text);
}
inline wxToolTip * wxWindow_GetToolTip(const wxWindow & self) {
    return self.GetToolTip();
}
inline wxString *wxWindow_GetToolTipText(const wxWindow & self) {
    return new wxString(self.GetToolTipText());
}
inline void wxWindow_SetToolTip(wxWindow & self, const wxString & tip_string) {
    return self.SetToolTip(tip_string);
}
inline void wxWindow_SetToolTip1(wxWindow & self, wxToolTip * tip) {
    return self.SetToolTip(tip);
}
inline void wxWindow_UnsetToolTip(wxWindow & self) {
    return self.UnsetToolTip();
}
inline int wxWindow_GetPopupMenuSelectionFromUser(wxWindow & self, wxMenu & menu, const wxPoint & pos) {
    return self.GetPopupMenuSelectionFromUser(menu, pos);
}
inline int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow & self, wxMenu & menu, int x, int y) {
    return self.GetPopupMenuSelectionFromUser(menu, x, y);
}
inline bool wxWindow_PopupMenu(wxWindow & self, wxMenu * menu, const wxPoint & pos) {
    return self.PopupMenu(menu, pos);
}
inline bool wxWindow_PopupMenu1(wxWindow & self, wxMenu * menu, int x, int y) {
    return self.PopupMenu(menu, x, y);
}
inline wxValidator * wxWindow_GetValidator(wxWindow & self) {
    return self.GetValidator();
}
inline void wxWindow_SetValidator(wxWindow & self, const wxValidator & validator) {
    return self.SetValidator(validator);
}
inline bool wxWindow_TransferDataFromWindow(wxWindow & self) {
    return self.TransferDataFromWindow();
}
inline bool wxWindow_TransferDataToWindow(wxWindow & self) {
    return self.TransferDataToWindow();
}
inline bool wxWindow_Validate(wxWindow & self) {
    return self.Validate();
}
inline wxWindowID wxWindow_GetId(const wxWindow & self) {
    return self.GetId();
}
inline wxString *wxWindow_GetLabel(const wxWindow & self) {
    return new wxString(self.GetLabel());
}
inline wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow & self, wxCoord x, wxCoord width, wxCoord width_total) {
    return self.AdjustForLayoutDirection(x, width, width_total);
}
inline wxString *wxWindow_GetName(const wxWindow & self) {
    return new wxString(self.GetName());
}
inline void wxWindow_SetId(wxWindow & self, wxWindowID winid) {
    return self.SetId(winid);
}
inline void wxWindow_SetLabel(wxWindow & self, const wxString & label) {
    return self.SetLabel(label);
}
inline void wxWindow_SetName(wxWindow & self, const wxString & name) {
    return self.SetName(name);
}
inline wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow & self) {
    return self.GetAcceleratorTable();
}
inline void wxWindow_SetAcceleratorTable(wxWindow & self, const wxAcceleratorTable & accel) {
    return self.SetAcceleratorTable(accel);
}
inline bool wxWindow_Close(wxWindow & self, bool force) {
    return self.Close(force);
}
inline bool wxWindow_Destroy(wxWindow & self) {
    return self.Destroy();
}
inline bool wxWindow_IsBeingDeleted(const wxWindow & self) {
    return self.IsBeingDeleted();
}
inline wxDropTarget * wxWindow_GetDropTarget(const wxWindow & self) {
    return self.GetDropTarget();
}
inline void wxWindow_SetDropTarget(wxWindow & self, wxDropTarget * target) {
    return self.SetDropTarget(target);
}
inline void wxWindow_DragAcceptFiles(wxWindow & self, bool accept) {
    return self.DragAcceptFiles(accept);
}
inline wxSizer * wxWindow_GetContainingSizer(const wxWindow & self) {
    return self.GetContainingSizer();
}
inline wxSizer * wxWindow_GetSizer(const wxWindow & self) {
    return self.GetSizer();
}
inline void wxWindow_SetSizer(wxWindow & self, wxSizer * sizer, bool delete_old) {
    return self.SetSizer(sizer, delete_old);
}
inline void wxWindow_SetSizerAndFit(wxWindow & self, wxSizer * sizer, bool delete_old) {
    return self.SetSizerAndFit(sizer, delete_old);
}
inline wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow & self) {
    return self.GetConstraints();
}
inline void wxWindow_SetConstraints(wxWindow & self, wxLayoutConstraints * constraints) {
    return self.SetConstraints(constraints);
}
inline bool wxWindow_Layout(wxWindow & self) {
    return self.Layout();
}
inline void wxWindow_SetAutoLayout(wxWindow & self, bool auto_layout) {
    return self.SetAutoLayout(auto_layout);
}
inline bool wxWindow_GetAutoLayout(const wxWindow & self) {
    return self.GetAutoLayout();
}
inline void wxWindow_CaptureMouse(wxWindow & self) {
    return self.CaptureMouse();
}
inline wxCaret * wxWindow_GetCaret(const wxWindow & self) {
    return self.GetCaret();
}
inline bool wxWindow_HasCapture(const wxWindow & self) {
    return self.HasCapture();
}
inline void wxWindow_ReleaseMouse(wxWindow & self) {
    return self.ReleaseMouse();
}
inline void wxWindow_SetCaret(wxWindow & self, wxCaret * caret) {
    return self.SetCaret(caret);
}
inline bool wxWindow_SetCursor(wxWindow & self, const wxCursor & cursor) {
    return self.SetCursor(cursor);
}
inline void wxWindow_WarpPointer(wxWindow & self, int x, int y) {
    return self.WarpPointer(x, y);
}
inline bool wxWindow_EnableTouchEvents(wxWindow & self, int events_mask) {
    return self.EnableTouchEvents(events_mask);
}
inline void wxWindow_DoUpdateWindowUI(wxWindow & self, wxUpdateUIEvent & event) {
    return self.DoUpdateWindowUI(event);
}
inline bool wxWindow_HasMultiplePages(const wxWindow & self) {
    return self.HasMultiplePages();
}
inline void wxWindow_InheritAttributes(wxWindow & self) {
    return self.InheritAttributes();
}
inline void wxWindow_InitDialog(wxWindow & self) {
    return self.InitDialog();
}
inline bool wxWindow_IsDoubleBuffered(const wxWindow & self) {
    return self.IsDoubleBuffered();
}
inline void wxWindow_SetDoubleBuffered(wxWindow & self, bool on) {
    return self.SetDoubleBuffered(on);
}
inline bool wxWindow_IsRetained(const wxWindow & self) {
    return self.IsRetained();
}
inline bool wxWindow_IsThisEnabled(const wxWindow & self) {
    return self.IsThisEnabled();
}
inline bool wxWindow_IsTopLevel(const wxWindow & self) {
    return self.IsTopLevel();
}
inline void wxWindow_OnInternalIdle(wxWindow & self) {
    return self.OnInternalIdle();
}
inline bool wxWindow_SendIdleEvents(wxWindow & self, wxIdleEvent & event) {
    return self.SendIdleEvents(event);
}
inline bool wxWindow_RegisterHotKey(wxWindow & self, int hotkey_id, int modifiers, int virtual_key_code) {
    return self.RegisterHotKey(hotkey_id, modifiers, virtual_key_code);
}
inline bool wxWindow_UnregisterHotKey(wxWindow & self, int hotkey_id) {
    return self.UnregisterHotKey(hotkey_id);
}
inline void wxWindow_UpdateWindowUI(wxWindow & self, int32_t flags) {
    return self.UpdateWindowUI(flags);
}
inline wxWindow * wxWindow_FindFocus() {
    return wxWindow::FindFocus();
}
inline wxWindow * wxWindow_FindWindowById(int32_t id, const wxWindow * parent) {
    return wxWindow::FindWindowById(id, parent);
}
inline wxWindow * wxWindow_FindWindowByLabel(const wxString & label, const wxWindow * parent) {
    return wxWindow::FindWindowByLabel(label, parent);
}
inline wxWindow * wxWindow_FindWindowByName(const wxString & name, const wxWindow * parent) {
    return wxWindow::FindWindowByName(name, parent);
}
inline wxWindow * wxWindow_GetCapture() {
    return wxWindow::GetCapture();
}
inline wxWindowID wxWindow_NewControlId(int count) {
    return wxWindow::NewControlId(count);
}
inline void wxWindow_UnreserveControlId(wxWindowID id, int count) {
    return wxWindow::UnreserveControlId(id, count);
}
inline wxWindow *wxWindow_new() {
    return new wxWindow();
}
inline wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxWindow(parent, id, pos, size, style, name);
}
inline bool wxWindow_Create(wxWindow & self, wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return self.Create(parent, id, pos, size, style, name);
}

// CLASS: wxControl
inline wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxControl(parent, id, pos, size, style, validator, name);
}
inline wxControl *wxControl_new1() {
    return new wxControl();
}
inline bool wxControl_Create(wxControl & self, wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return self.Create(parent, id, pos, size, style, validator, name);
}
inline void wxControl_Command(wxControl & self, wxCommandEvent & event) {
    return self.Command(event);
}
inline wxString *wxControl_GetLabel(const wxControl & self) {
    return new wxString(self.GetLabel());
}
inline wxString *wxControl_GetLabelText(const wxControl & self) {
    return new wxString(self.GetLabelText());
}
inline wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, int xlen, int ylen) {
    return new wxSize(self.GetSizeFromTextSize(xlen, ylen));
}
inline wxSize *wxControl_GetSizeFromTextSize1(const wxControl & self, const wxSize & tsize) {
    return new wxSize(self.GetSizeFromTextSize(tsize));
}
inline wxSize *wxControl_GetSizeFromText(const wxControl & self, const wxString & text) {
    return new wxSize(self.GetSizeFromText(text));
}
inline void wxControl_SetLabel(wxControl & self, const wxString & label) {
    return self.SetLabel(label);
}
inline void wxControl_SetLabelText(wxControl & self, const wxString & text) {
    return self.SetLabelText(text);
}
inline bool wxControl_SetLabelMarkup(wxControl & self, const wxString & markup) {
    return self.SetLabelMarkup(markup);
}
inline wxString *wxControl_GetLabelText1(const wxString & label) {
    return new wxString(wxControl::GetLabelText(label));
}
inline wxString *wxControl_RemoveMnemonics(const wxString & str) {
    return new wxString(wxControl::RemoveMnemonics(str));
}
inline wxString *wxControl_EscapeMnemonics(const wxString & text) {
    return new wxString(wxControl::EscapeMnemonics(text));
}

// CLASS: wxAnyButton
inline wxAnyButton *wxAnyButton_new() {
    return new wxAnyButton();
}
inline void wxAnyButton_SetBitmapCurrent(wxAnyButton & self, const wxBitmap & bitmap) {
    return self.SetBitmapCurrent(bitmap);
}
inline void wxAnyButton_SetBitmapDisabled(wxAnyButton & self, const wxBitmap & bitmap) {
    return self.SetBitmapDisabled(bitmap);
}
inline void wxAnyButton_SetBitmapFocus(wxAnyButton & self, const wxBitmap & bitmap) {
    return self.SetBitmapFocus(bitmap);
}
inline void wxAnyButton_SetBitmapLabel(wxAnyButton & self, const wxBitmap & bitmap) {
    return self.SetBitmapLabel(bitmap);
}
inline void wxAnyButton_SetBitmapPressed(wxAnyButton & self, const wxBitmap & bitmap) {
    return self.SetBitmapPressed(bitmap);
}
inline wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton & self) {
    return new wxSize(self.GetBitmapMargins());
}
inline void wxAnyButton_SetBitmapMargins(wxAnyButton & self, wxCoord x, wxCoord y) {
    return self.SetBitmapMargins(x, y);
}
inline void wxAnyButton_SetBitmapMargins1(wxAnyButton & self, const wxSize & sz) {
    return self.SetBitmapMargins(sz);
}

// CLASS: wxButton
inline wxButton *wxButton_new() {
    return new wxButton();
}
inline wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}
inline bool wxButton_Create(wxButton & self, wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return self.Create(parent, id, label, pos, size, style, validator, name);
}
inline bool wxButton_GetAuthNeeded(const wxButton & self) {
    return self.GetAuthNeeded();
}
inline wxString *wxButton_GetLabel(const wxButton & self) {
    return new wxString(self.GetLabel());
}
inline void wxButton_SetAuthNeeded(wxButton & self, bool needed) {
    return self.SetAuthNeeded(needed);
}
inline wxWindow * wxButton_SetDefault(wxButton & self) {
    return self.SetDefault();
}
inline void wxButton_SetLabel(wxButton & self, const wxString & label) {
    return self.SetLabel(label);
}
inline wxSize *wxButton_GetDefaultSize(wxWindow * win) {
    return new wxSize(wxButton::GetDefaultSize(win));
}

// CLASS: wxNonOwnedWindow
inline bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow & self, const wxRegion & region) {
    return self.SetShape(region);
}
inline bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow & self, const wxGraphicsPath & path) {
    return self.SetShape(path);
}

// CLASS: wxTopLevelWindow
inline wxTopLevelWindow *wxTopLevelWindow_new() {
    return new wxTopLevelWindow();
}
inline wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxTopLevelWindow(parent, id, title, pos, size, style, name);
}
inline bool wxTopLevelWindow_Create(wxTopLevelWindow & self, wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return self.Create(parent, id, title, pos, size, style, name);
}
inline bool wxTopLevelWindow_CanSetTransparent(wxTopLevelWindow & self) {
    return self.CanSetTransparent();
}
inline void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow & self, int direction) {
    return self.CenterOnScreen(direction);
}
inline void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow & self, int direction) {
    return self.CentreOnScreen(direction);
}
inline bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow & self, bool enable) {
    return self.EnableCloseButton(enable);
}
inline bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow & self, bool enable) {
    return self.EnableMaximizeButton(enable);
}
inline bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow & self, bool enable) {
    return self.EnableMinimizeButton(enable);
}
inline wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow & self) {
    return self.GetDefaultItem();
}
inline wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow & self) {
    return new wxString(self.GetTitle());
}
inline void wxTopLevelWindow_Iconize(wxTopLevelWindow & self, bool iconize) {
    return self.Iconize(iconize);
}
inline bool wxTopLevelWindow_IsActive(wxTopLevelWindow & self) {
    return self.IsActive();
}
inline bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow & self) {
    return self.IsAlwaysMaximized();
}
inline bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow & self) {
    return self.IsFullScreen();
}
inline bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow & self) {
    return self.IsIconized();
}
inline bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow & self) {
    return self.IsMaximized();
}
inline bool wxTopLevelWindow_Layout(wxTopLevelWindow & self) {
    return self.Layout();
}
inline void wxTopLevelWindow_Maximize(wxTopLevelWindow & self, bool maximize) {
    return self.Maximize(maximize);
}
inline void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow & self, int flags) {
    return self.RequestUserAttention(flags);
}
inline void wxTopLevelWindow_Restore(wxTopLevelWindow & self) {
    return self.Restore();
}
inline wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow & self, wxWindow * win) {
    return self.SetDefaultItem(win);
}
inline wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow & self, wxWindow * win) {
    return self.SetTmpDefaultItem(win);
}
inline wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow & self) {
    return self.GetTmpDefaultItem();
}
inline void wxTopLevelWindow_SetIcon(wxTopLevelWindow & self, const wxIcon & icon) {
    return self.SetIcon(icon);
}
inline void wxTopLevelWindow_SetIcons(wxTopLevelWindow & self, const wxIconBundle & icons) {
    return self.SetIcons(icons);
}
inline void wxTopLevelWindow_SetMaxSize(wxTopLevelWindow & self, const wxSize & size) {
    return self.SetMaxSize(size);
}
inline void wxTopLevelWindow_SetMinSize(wxTopLevelWindow & self, const wxSize & size) {
    return self.SetMinSize(size);
}
inline void wxTopLevelWindow_SetSizeHints(wxTopLevelWindow & self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return self.SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
inline void wxTopLevelWindow_SetSizeHints1(wxTopLevelWindow & self, const wxSize & min_size, const wxSize & max_size, const wxSize & inc_size) {
    return self.SetSizeHints(min_size, max_size, inc_size);
}
inline void wxTopLevelWindow_SetTitle(wxTopLevelWindow & self, const wxString & title) {
    return self.SetTitle(title);
}
inline bool wxTopLevelWindow_SetTransparent(wxTopLevelWindow & self, wxByte alpha) {
    return self.SetTransparent(alpha);
}
inline bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow & self) {
    return self.ShouldPreventAppExit();
}
inline void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow & self, bool modified) {
    return self.OSXSetModified(modified);
}
inline bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow & self) {
    return self.OSXIsModified();
}
inline void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow & self, const wxString & filename) {
    return self.SetRepresentedFilename(filename);
}
inline void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow & self) {
    return self.ShowWithoutActivating();
}
inline bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow & self, bool enable) {
    return self.EnableFullScreenView(enable);
}
inline bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow & self, bool show, int32_t style) {
    return self.ShowFullScreen(show, style);
}
inline wxSize *wxTopLevelWindow_GetDefaultSize() {
    return new wxSize(wxTopLevelWindow::GetDefaultSize());
}

// CLASS: wxFrame
inline wxFrame *wxFrame_new() {
    return new wxFrame();
}
inline wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxFrame(parent, id, title, pos, size, style, name);
}
inline void wxFrame_Centre(wxFrame & self, int direction) {
    return self.Centre(direction);
}
inline bool wxFrame_Create(wxFrame & self, wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return self.Create(parent, id, title, pos, size, style, name);
}
inline wxStatusBar * wxFrame_CreateStatusBar(wxFrame & self, int number, int32_t style, wxWindowID id, const wxString & name) {
    return self.CreateStatusBar(number, style, id, name);
}
inline wxToolBar * wxFrame_CreateToolBar(wxFrame & self, int32_t style, wxWindowID id, const wxString & name) {
    return self.CreateToolBar(style, id, name);
}
inline void wxFrame_DoGiveHelp(wxFrame & self, const wxString & text, bool show) {
    return self.DoGiveHelp(text, show);
}
inline wxPoint *wxFrame_GetClientAreaOrigin(const wxFrame & self) {
    return new wxPoint(self.GetClientAreaOrigin());
}
inline wxMenuBar * wxFrame_GetMenuBar(const wxFrame & self) {
    return self.GetMenuBar();
}
inline wxStatusBar * wxFrame_GetStatusBar(const wxFrame & self) {
    return self.GetStatusBar();
}
inline int wxFrame_GetStatusBarPane(const wxFrame & self) {
    return self.GetStatusBarPane();
}
inline wxToolBar * wxFrame_GetToolBar(const wxFrame & self) {
    return self.GetToolBar();
}
inline wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame & self, int number, int32_t style, wxWindowID id, const wxString & name) {
    return self.OnCreateStatusBar(number, style, id, name);
}
inline wxToolBar * wxFrame_OnCreateToolBar(wxFrame & self, int32_t style, wxWindowID id, const wxString & name) {
    return self.OnCreateToolBar(style, id, name);
}
inline bool wxFrame_ProcessCommand(wxFrame & self, int id) {
    return self.ProcessCommand(id);
}
inline void wxFrame_SetMenuBar(wxFrame & self, wxMenuBar * menu_bar) {
    return self.SetMenuBar(menu_bar);
}
inline void wxFrame_SetStatusBar(wxFrame & self, wxStatusBar * status_bar) {
    return self.SetStatusBar(status_bar);
}
inline void wxFrame_SetStatusBarPane(wxFrame & self, int n) {
    return self.SetStatusBarPane(n);
}
inline void wxFrame_SetStatusText(wxFrame & self, const wxString & text, int number) {
    return self.SetStatusText(text, number);
}
inline void wxFrame_SetStatusWidths(wxFrame & self, int n, const int * widths_field) {
    return self.SetStatusWidths(n, widths_field);
}
inline void wxFrame_SetToolBar(wxFrame & self, wxToolBar * tool_bar) {
    return self.SetToolBar(tool_bar);
}
inline void wxFrame_PushStatusText(wxFrame & self, const wxString & text, int number) {
    return self.PushStatusText(text, number);
}
inline void wxFrame_PopStatusText(wxFrame & self, int number) {
    return self.PopStatusText(number);
}

// CLASS: wxPoint
inline bool wxPoint_IsFullySpecified(const wxPoint & self) {
    return self.IsFullySpecified();
}
inline void wxPoint_SetDefaults(wxPoint & self, const wxPoint & pt) {
    return self.SetDefaults(pt);
}
inline wxPoint *wxPoint_new() {
    return new wxPoint();
}
inline wxPoint *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
inline wxPoint *wxPoint_new2(const wxRealPoint & pt) {
    return new wxPoint(pt);
}

// CLASS: wxRect
inline wxRect *wxRect_new() {
    return new wxRect();
}
inline wxRect *wxRect_new1(int x, int y, int width, int height) {
    return new wxRect(x, y, width, height);
}
inline wxRect *wxRect_new2(const wxPoint & top_left, const wxPoint & bottom_right) {
    return new wxRect(top_left, bottom_right);
}
inline wxRect *wxRect_new3(const wxPoint & pos, const wxSize & size) {
    return new wxRect(pos, size);
}
inline wxRect *wxRect_new4(const wxSize & size) {
    return new wxRect(size);
}
inline wxRect *wxRect_CentreIn(const wxRect & self, const wxRect & r, int dir) {
    return new wxRect(self.CentreIn(r, dir));
}
inline wxRect *wxRect_CenterIn(const wxRect & self, const wxRect & r, int dir) {
    return new wxRect(self.CenterIn(r, dir));
}
inline bool wxRect_Contains(const wxRect & self, int x, int y) {
    return self.Contains(x, y);
}
inline bool wxRect_Contains1(const wxRect & self, const wxPoint & pt) {
    return self.Contains(pt);
}
inline bool wxRect_Contains2(const wxRect & self, const wxRect & rect) {
    return self.Contains(rect);
}
inline wxRect *wxRect_Deflate3(const wxRect & self, wxCoord dx, wxCoord dy) {
    return new wxRect(self.Deflate(dx, dy));
}
inline int wxRect_GetBottom(const wxRect & self) {
    return self.GetBottom();
}
inline wxPoint *wxRect_GetBottomLeft(const wxRect & self) {
    return new wxPoint(self.GetBottomLeft());
}
inline wxPoint *wxRect_GetBottomRight(const wxRect & self) {
    return new wxPoint(self.GetBottomRight());
}
inline int wxRect_GetHeight(const wxRect & self) {
    return self.GetHeight();
}
inline int wxRect_GetLeft(const wxRect & self) {
    return self.GetLeft();
}
inline wxPoint *wxRect_GetPosition(const wxRect & self) {
    return new wxPoint(self.GetPosition());
}
inline int wxRect_GetRight(const wxRect & self) {
    return self.GetRight();
}
inline wxSize *wxRect_GetSize(const wxRect & self) {
    return new wxSize(self.GetSize());
}
inline int wxRect_GetTop(const wxRect & self) {
    return self.GetTop();
}
inline wxPoint *wxRect_GetTopLeft(const wxRect & self) {
    return new wxPoint(self.GetTopLeft());
}
inline wxPoint *wxRect_GetTopRight(const wxRect & self) {
    return new wxPoint(self.GetTopRight());
}
inline int wxRect_GetWidth(const wxRect & self) {
    return self.GetWidth();
}
inline int wxRect_GetX(const wxRect & self) {
    return self.GetX();
}
inline int wxRect_GetY(const wxRect & self) {
    return self.GetY();
}
inline wxRect *wxRect_Inflate3(const wxRect & self, wxCoord dx, wxCoord dy) {
    return new wxRect(self.Inflate(dx, dy));
}
inline wxRect *wxRect_Intersect1(const wxRect & self, const wxRect & rect) {
    return new wxRect(self.Intersect(rect));
}
inline bool wxRect_Intersects(const wxRect & self, const wxRect & rect) {
    return self.Intersects(rect);
}
inline bool wxRect_IsEmpty(const wxRect & self) {
    return self.IsEmpty();
}
inline void wxRect_Offset(wxRect & self, wxCoord dx, wxCoord dy) {
    return self.Offset(dx, dy);
}
inline void wxRect_Offset1(wxRect & self, const wxPoint & pt) {
    return self.Offset(pt);
}
inline void wxRect_SetHeight(wxRect & self, int height) {
    return self.SetHeight(height);
}
inline void wxRect_SetPosition(wxRect & self, const wxPoint & pos) {
    return self.SetPosition(pos);
}
inline void wxRect_SetSize(wxRect & self, const wxSize & s) {
    return self.SetSize(s);
}
inline void wxRect_SetWidth(wxRect & self, int width) {
    return self.SetWidth(width);
}
inline void wxRect_SetX(wxRect & self, int x) {
    return self.SetX(x);
}
inline void wxRect_SetY(wxRect & self, int y) {
    return self.SetY(y);
}
inline void wxRect_SetLeft(wxRect & self, int left) {
    return self.SetLeft(left);
}
inline void wxRect_SetRight(wxRect & self, int right) {
    return self.SetRight(right);
}
inline void wxRect_SetTop(wxRect & self, int top) {
    return self.SetTop(top);
}
inline void wxRect_SetBottom(wxRect & self, int bottom) {
    return self.SetBottom(bottom);
}
inline void wxRect_SetTopLeft(wxRect & self, const wxPoint & p) {
    return self.SetTopLeft(p);
}
inline void wxRect_SetBottomRight(wxRect & self, const wxPoint & p) {
    return self.SetBottomRight(p);
}
inline void wxRect_SetTopRight(wxRect & self, const wxPoint & p) {
    return self.SetTopRight(p);
}
inline void wxRect_SetBottomLeft(wxRect & self, const wxPoint & p) {
    return self.SetBottomLeft(p);
}
inline wxRect *wxRect_Union(const wxRect & self, const wxRect & rect) {
    return new wxRect(self.Union(rect));
}

// CLASS: wxSize
inline wxSize *wxSize_new() {
    return new wxSize();
}
inline wxSize *wxSize_new1(int width, int height) {
    return new wxSize(width, height);
}
inline void wxSize_DecBy(wxSize & self, const wxPoint & pt) {
    return self.DecBy(pt);
}
inline void wxSize_DecBy1(wxSize & self, const wxSize & size) {
    return self.DecBy(size);
}
inline void wxSize_DecBy2(wxSize & self, int dx, int dy) {
    return self.DecBy(dx, dy);
}
inline void wxSize_DecBy3(wxSize & self, int d) {
    return self.DecBy(d);
}
inline void wxSize_DecTo(wxSize & self, const wxSize & size) {
    return self.DecTo(size);
}
inline void wxSize_DecToIfSpecified(wxSize & self, const wxSize & size) {
    return self.DecToIfSpecified(size);
}
inline int wxSize_GetHeight(const wxSize & self) {
    return self.GetHeight();
}
inline int wxSize_GetWidth(const wxSize & self) {
    return self.GetWidth();
}
inline void wxSize_IncBy(wxSize & self, const wxPoint & pt) {
    return self.IncBy(pt);
}
inline void wxSize_IncBy1(wxSize & self, const wxSize & size) {
    return self.IncBy(size);
}
inline void wxSize_IncBy2(wxSize & self, int dx, int dy) {
    return self.IncBy(dx, dy);
}
inline void wxSize_IncBy3(wxSize & self, int d) {
    return self.IncBy(d);
}
inline void wxSize_IncTo(wxSize & self, const wxSize & size) {
    return self.IncTo(size);
}
inline bool wxSize_IsFullySpecified(const wxSize & self) {
    return self.IsFullySpecified();
}
inline void wxSize_Set(wxSize & self, int width, int height) {
    return self.Set(width, height);
}
inline void wxSize_SetDefaults(wxSize & self, const wxSize & size_default) {
    return self.SetDefaults(size_default);
}
inline void wxSize_SetHeight(wxSize & self, int height) {
    return self.SetHeight(height);
}
inline void wxSize_SetWidth(wxSize & self, int width) {
    return self.SetWidth(width);
}

// CLASS: wxValidator
inline wxValidator *wxValidator_new() {
    return new wxValidator();
}
inline wxObject * wxValidator_Clone(const wxValidator & self) {
    return self.Clone();
}
inline wxWindow * wxValidator_GetWindow(const wxValidator & self) {
    return self.GetWindow();
}
inline void wxValidator_SetWindow(wxValidator & self, wxWindow * window) {
    return self.SetWindow(window);
}
inline bool wxValidator_TransferFromWindow(wxValidator & self) {
    return self.TransferFromWindow();
}
inline bool wxValidator_TransferToWindow(wxValidator & self) {
    return self.TransferToWindow();
}
inline bool wxValidator_Validate(wxValidator & self, wxWindow * parent) {
    return self.Validate(parent);
}
inline void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
inline bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}

} // namespace wxrust

