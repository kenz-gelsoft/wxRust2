#include "generated.h"

extern "C" {

// CLASS: wxWindow
wxClassInfo *wxWindow_CLASSINFO() {
    return wxCLASSINFO(wxWindow);
}
bool wxWindow_AcceptsFocus(const wxWindow * self) {
    return self->AcceptsFocus();
}
bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow * self) {
    return self->AcceptsFocusFromKeyboard();
}
bool wxWindow_AcceptsFocusRecursively(const wxWindow * self) {
    return self->AcceptsFocusRecursively();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_DisableFocusFromKeyboard(wxWindow * self) {
    return self->DisableFocusFromKeyboard();
}
#endif
bool wxWindow_IsFocusable(const wxWindow * self) {
    return self->IsFocusable();
}
bool wxWindow_CanAcceptFocus(const wxWindow * self) {
    return self->CanAcceptFocus();
}
bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow * self) {
    return self->CanAcceptFocusFromKeyboard();
}
bool wxWindow_HasFocus(const wxWindow * self) {
    return self->HasFocus();
}
void wxWindow_SetCanFocus(wxWindow * self, bool can_focus) {
    return self->SetCanFocus(can_focus);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxWindow_EnableVisibleFocus(wxWindow * self, bool enable) {
    return self->EnableVisibleFocus(enable);
}
#endif
void wxWindow_SetFocus(wxWindow * self) {
    return self->SetFocus();
}
void wxWindow_SetFocusFromKbd(wxWindow * self) {
    return self->SetFocusFromKbd();
}
void wxWindow_AddChild(wxWindow * self, wxWindow * child) {
    return self->AddChild(child);
}
bool wxWindow_DestroyChildren(wxWindow * self) {
    return self->DestroyChildren();
}
wxWindow * wxWindow_FindWindow(const wxWindow * self, long id) {
    return self->FindWindow(id);
}
wxWindow * wxWindow_FindWindow1(const wxWindow * self, const wxString * name) {
    return self->FindWindow(*name);
}
wxWindowList *wxWindow_GetChildren1(const wxWindow * self) {
    return new wxWindowList(self->GetChildren());
}
void wxWindow_RemoveChild(wxWindow * self, wxWindow * child) {
    return self->RemoveChild(child);
}
wxWindow * wxWindow_GetGrandParent(const wxWindow * self) {
    return self->GetGrandParent();
}
wxWindow * wxWindow_GetNextSibling(const wxWindow * self) {
    return self->GetNextSibling();
}
wxWindow * wxWindow_GetParent(const wxWindow * self) {
    return self->GetParent();
}
wxWindow * wxWindow_GetPrevSibling(const wxWindow * self) {
    return self->GetPrevSibling();
}
bool wxWindow_IsDescendant(const wxWindow * self, wxWindow * win) {
    return self->IsDescendant(win);
}
bool wxWindow_Reparent(wxWindow * self, wxWindow * new_parent) {
    return self->Reparent(new_parent);
}
void wxWindow_AlwaysShowScrollbars(wxWindow * self, bool hflag, bool vflag) {
    return self->AlwaysShowScrollbars(hflag, vflag);
}
int wxWindow_GetScrollPos(const wxWindow * self, int orientation) {
    return self->GetScrollPos(orientation);
}
int wxWindow_GetScrollRange(const wxWindow * self, int orientation) {
    return self->GetScrollRange(orientation);
}
int wxWindow_GetScrollThumb(const wxWindow * self, int orientation) {
    return self->GetScrollThumb(orientation);
}
bool wxWindow_CanScroll(const wxWindow * self, int orient) {
    return self->CanScroll(orient);
}
bool wxWindow_HasScrollbar(const wxWindow * self, int orient) {
    return self->HasScrollbar(orient);
}
bool wxWindow_IsScrollbarAlwaysShown(const wxWindow * self, int orient) {
    return self->IsScrollbarAlwaysShown(orient);
}
bool wxWindow_ScrollLines(wxWindow * self, int lines) {
    return self->ScrollLines(lines);
}
bool wxWindow_ScrollPages(wxWindow * self, int pages) {
    return self->ScrollPages(pages);
}
void wxWindow_ScrollWindow(wxWindow * self, int dx, int dy, const wxRect * rect) {
    return self->ScrollWindow(dx, dy, rect);
}
bool wxWindow_LineUp(wxWindow * self) {
    return self->LineUp();
}
bool wxWindow_LineDown(wxWindow * self) {
    return self->LineDown();
}
bool wxWindow_PageUp(wxWindow * self) {
    return self->PageUp();
}
bool wxWindow_PageDown(wxWindow * self) {
    return self->PageDown();
}
void wxWindow_SetScrollPos(wxWindow * self, int orientation, int pos, bool refresh) {
    return self->SetScrollPos(orientation, pos, refresh);
}
void wxWindow_SetScrollbar(wxWindow * self, int orientation, int position, int thumb_size, int range, bool refresh) {
    return self->SetScrollbar(orientation, position, thumb_size, range, refresh);
}
bool wxWindow_BeginRepositioningChildren(wxWindow * self) {
    return self->BeginRepositioningChildren();
}
void wxWindow_EndRepositioningChildren(wxWindow * self) {
    return self->EndRepositioningChildren();
}
void wxWindow_CacheBestSize(const wxWindow * self, const wxSize * size) {
    return self->CacheBestSize(*size);
}
wxSize *wxWindow_ClientToWindowSize(const wxWindow * self, const wxSize * size) {
    return new wxSize(self->ClientToWindowSize(*size));
}
wxSize *wxWindow_WindowToClientSize(const wxWindow * self, const wxSize * size) {
    return new wxSize(self->WindowToClientSize(*size));
}
void wxWindow_Fit(wxWindow * self) {
    return self->Fit();
}
void wxWindow_FitInside(wxWindow * self) {
    return self->FitInside();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->FromDIP(*sz));
}
wxPoint *wxWindow_FromDIP1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->FromDIP(*pt));
}
int wxWindow_FromDIP2(const wxWindow * self, int d) {
    return self->FromDIP(d);
}
wxSize *wxWindow_ToDIP(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ToDIP(*sz));
}
wxPoint *wxWindow_ToDIP1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ToDIP(*pt));
}
int wxWindow_ToDIP2(const wxWindow * self, int d) {
    return self->ToDIP(d);
}
wxSize *wxWindow_FromPhys(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->FromPhys(*sz));
}
wxPoint *wxWindow_FromPhys1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->FromPhys(*pt));
}
int wxWindow_FromPhys2(const wxWindow * self, int d) {
    return self->FromPhys(d);
}
wxSize *wxWindow_ToPhys(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ToPhys(*sz));
}
wxPoint *wxWindow_ToPhys1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ToPhys(*pt));
}
int wxWindow_ToPhys2(const wxWindow * self, int d) {
    return self->ToPhys(d);
}
#endif
wxSize *wxWindow_GetBestSize(const wxWindow * self) {
    return new wxSize(self->GetBestSize());
}
int wxWindow_GetBestHeight(const wxWindow * self, int width) {
    return self->GetBestHeight(width);
}
int wxWindow_GetBestWidth(const wxWindow * self, int height) {
    return self->GetBestWidth(height);
}
void wxWindow_GetClientSize(const wxWindow * self, int * width, int * height) {
    return self->GetClientSize(width, height);
}
wxSize *wxWindow_GetClientSize1(const wxWindow * self) {
    return new wxSize(self->GetClientSize());
}
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow * self) {
    return new wxSize(self->GetEffectiveMinSize());
}
wxSize *wxWindow_GetMaxClientSize(const wxWindow * self) {
    return new wxSize(self->GetMaxClientSize());
}
wxSize *wxWindow_GetMaxSize(const wxWindow * self) {
    return new wxSize(self->GetMaxSize());
}
wxSize *wxWindow_GetMinClientSize(const wxWindow * self) {
    return new wxSize(self->GetMinClientSize());
}
wxSize *wxWindow_GetMinSize(const wxWindow * self) {
    return new wxSize(self->GetMinSize());
}
int wxWindow_GetMinWidth(const wxWindow * self) {
    return self->GetMinWidth();
}
int wxWindow_GetMinHeight(const wxWindow * self) {
    return self->GetMinHeight();
}
int wxWindow_GetMaxWidth(const wxWindow * self) {
    return self->GetMaxWidth();
}
int wxWindow_GetMaxHeight(const wxWindow * self) {
    return self->GetMaxHeight();
}
void wxWindow_GetSize(const wxWindow * self, int * width, int * height) {
    return self->GetSize(width, height);
}
wxSize *wxWindow_GetSize1(const wxWindow * self) {
    return new wxSize(self->GetSize());
}
wxSize *wxWindow_GetVirtualSize(const wxWindow * self) {
    return new wxSize(self->GetVirtualSize());
}
void wxWindow_GetVirtualSize1(const wxWindow * self, int * width, int * height) {
    return self->GetVirtualSize(width, height);
}
wxSize *wxWindow_GetBestVirtualSize(const wxWindow * self) {
    return new wxSize(self->GetBestVirtualSize());
}
double wxWindow_GetContentScaleFactor(const wxWindow * self) {
    return self->GetContentScaleFactor();
}
#if wxCHECK_VERSION(3, 1, 0)
double wxWindow_GetDPIScaleFactor(const wxWindow * self) {
    return self->GetDPIScaleFactor();
}
#endif
wxSize *wxWindow_GetWindowBorderSize(const wxWindow * self) {
    return new wxSize(self->GetWindowBorderSize());
}
bool wxWindow_InformFirstDirection(wxWindow * self, int direction, int size, int available_other_dir) {
    return self->InformFirstDirection(direction, size, available_other_dir);
}
void wxWindow_InvalidateBestSize(wxWindow * self) {
    return self->InvalidateBestSize();
}
void wxWindow_PostSizeEvent(wxWindow * self) {
    return self->PostSizeEvent();
}
void wxWindow_PostSizeEventToParent(wxWindow * self) {
    return self->PostSizeEventToParent();
}
void wxWindow_SendSizeEvent(wxWindow * self, int flags) {
    return self->SendSizeEvent(flags);
}
void wxWindow_SendSizeEventToParent(wxWindow * self, int flags) {
    return self->SendSizeEventToParent(flags);
}
void wxWindow_SetClientSize(wxWindow * self, int width, int height) {
    return self->SetClientSize(width, height);
}
void wxWindow_SetClientSize1(wxWindow * self, const wxSize * size) {
    return self->SetClientSize(*size);
}
void wxWindow_SetClientSize2(wxWindow * self, const wxRect * rect) {
    return self->SetClientSize(*rect);
}
void wxWindow_SetContainingSizer(wxWindow * self, wxSizer * sizer) {
    return self->SetContainingSizer(sizer);
}
void wxWindow_SetInitialSize(wxWindow * self, const wxSize * size) {
    return self->SetInitialSize(*size);
}
void wxWindow_SetMaxClientSize(wxWindow * self, const wxSize * size) {
    return self->SetMaxClientSize(*size);
}
void wxWindow_SetMaxSize(wxWindow * self, const wxSize * size) {
    return self->SetMaxSize(*size);
}
void wxWindow_SetMinClientSize(wxWindow * self, const wxSize * size) {
    return self->SetMinClientSize(*size);
}
void wxWindow_SetMinSize(wxWindow * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxWindow_SetSize(wxWindow * self, int x, int y, int width, int height, int size_flags) {
    return self->SetSize(x, y, width, height, size_flags);
}
void wxWindow_SetSize1(wxWindow * self, const wxRect * rect) {
    return self->SetSize(*rect);
}
void wxWindow_SetSize2(wxWindow * self, const wxSize * size) {
    return self->SetSize(*size);
}
void wxWindow_SetSize3(wxWindow * self, int width, int height) {
    return self->SetSize(width, height);
}
void wxWindow_SetSizeHints(wxWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size) {
    return self->SetSizeHints(*min_size, *max_size, *inc_size);
}
void wxWindow_SetSizeHints1(wxWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return self->SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
void wxWindow_SetVirtualSize(wxWindow * self, int width, int height) {
    return self->SetVirtualSize(width, height);
}
void wxWindow_SetVirtualSize1(wxWindow * self, const wxSize * size) {
    return self->SetVirtualSize(*size);
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_FromDIP3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::FromDIP(*sz, w));
}
wxPoint *wxWindow_FromDIP4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromDIP(*pt, w));
}
int wxWindow_FromDIP5(int d, const wxWindow * w) {
    return wxWindow::FromDIP(d, w);
}
wxSize *wxWindow_ToDIP3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::ToDIP(*sz, w));
}
wxPoint *wxWindow_ToDIP4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToDIP(*pt, w));
}
int wxWindow_ToDIP5(int d, const wxWindow * w) {
    return wxWindow::ToDIP(d, w);
}
wxSize *wxWindow_FromPhys3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::FromPhys(*sz, w));
}
wxPoint *wxWindow_FromPhys4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromPhys(*pt, w));
}
int wxWindow_FromPhys5(int d, const wxWindow * w) {
    return wxWindow::FromPhys(d, w);
}
wxSize *wxWindow_ToPhys3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::ToPhys(*sz, w));
}
wxPoint *wxWindow_ToPhys4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToPhys(*pt, w));
}
int wxWindow_ToPhys5(int d, const wxWindow * w) {
    return wxWindow::ToPhys(d, w);
}
#endif
void wxWindow_Center(wxWindow * self, int dir) {
    return self->Center(dir);
}
void wxWindow_CenterOnParent(wxWindow * self, int dir) {
    return self->CenterOnParent(dir);
}
void wxWindow_Centre(wxWindow * self, int direction) {
    return self->Centre(direction);
}
void wxWindow_CentreOnParent(wxWindow * self, int direction) {
    return self->CentreOnParent(direction);
}
void wxWindow_GetPosition(const wxWindow * self, int * x, int * y) {
    return self->GetPosition(x, y);
}
wxPoint *wxWindow_GetPosition1(const wxWindow * self) {
    return new wxPoint(self->GetPosition());
}
wxRect *wxWindow_GetRect(const wxWindow * self) {
    return new wxRect(self->GetRect());
}
void wxWindow_GetScreenPosition(const wxWindow * self, int * x, int * y) {
    return self->GetScreenPosition(x, y);
}
wxPoint *wxWindow_GetScreenPosition1(const wxWindow * self) {
    return new wxPoint(self->GetScreenPosition());
}
wxRect *wxWindow_GetScreenRect(const wxWindow * self) {
    return new wxRect(self->GetScreenRect());
}
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow * self) {
    return new wxPoint(self->GetClientAreaOrigin());
}
wxRect *wxWindow_GetClientRect(const wxWindow * self) {
    return new wxRect(self->GetClientRect());
}
void wxWindow_Move(wxWindow * self, int x, int y, int flags) {
    return self->Move(x, y, flags);
}
void wxWindow_Move1(wxWindow * self, const wxPoint * pt, int flags) {
    return self->Move(*pt, flags);
}
void wxWindow_SetPosition(wxWindow * self, const wxPoint * pt) {
    return self->SetPosition(*pt);
}
void wxWindow_ClientToScreen(const wxWindow * self, int * x, int * y) {
    return self->ClientToScreen(x, y);
}
wxPoint *wxWindow_ClientToScreen1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ClientToScreen(*pt));
}
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ConvertDialogToPixels(*pt));
}
wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ConvertDialogToPixels(*sz));
}
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ConvertPixelsToDialog(*pt));
}
wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow * self, const wxSize * sz) {
    return new wxSize(self->ConvertPixelsToDialog(*sz));
}
void wxWindow_ScreenToClient(const wxWindow * self, int * x, int * y) {
    return self->ScreenToClient(x, y);
}
wxPoint *wxWindow_ScreenToClient1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(self->ScreenToClient(*pt));
}
void wxWindow_ClearBackground(wxWindow * self) {
    return self->ClearBackground();
}
void wxWindow_Freeze(wxWindow * self) {
    return self->Freeze();
}
void wxWindow_Thaw(wxWindow * self) {
    return self->Thaw();
}
bool wxWindow_IsFrozen(const wxWindow * self) {
    return self->IsFrozen();
}
wxColour *wxWindow_GetBackgroundColour(const wxWindow * self) {
    return new wxColour(self->GetBackgroundColour());
}
int wxWindow_GetCharHeight(const wxWindow * self) {
    return self->GetCharHeight();
}
int wxWindow_GetCharWidth(const wxWindow * self) {
    return self->GetCharWidth();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_GetDPI(const wxWindow * self) {
    return new wxSize(self->GetDPI());
}
#endif
wxFont *wxWindow_GetFont(const wxWindow * self) {
    return new wxFont(self->GetFont());
}
wxColour *wxWindow_GetForegroundColour(const wxWindow * self) {
    return new wxColour(self->GetForegroundColour());
}
void wxWindow_GetTextExtent(const wxWindow * self, const wxString * string, int * w, int * h, int * descent, int * external_leading, const wxFont * font) {
    return self->GetTextExtent(*string, w, h, descent, external_leading, font);
}
wxSize *wxWindow_GetTextExtent1(const wxWindow * self, const wxString * string) {
    return new wxSize(self->GetTextExtent(*string));
}
wxRect *wxWindow_GetUpdateClientRect(const wxWindow * self) {
    return new wxRect(self->GetUpdateClientRect());
}
bool wxWindow_HasTransparentBackground(wxWindow * self) {
    return self->HasTransparentBackground();
}
void wxWindow_Refresh(wxWindow * self, bool erase_background, const wxRect * rect) {
    return self->Refresh(erase_background, rect);
}
void wxWindow_RefreshRect(wxWindow * self, const wxRect * rect, bool erase_background) {
    return self->RefreshRect(*rect, erase_background);
}
void wxWindow_Update(wxWindow * self) {
    return self->Update();
}
bool wxWindow_SetBackgroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetBackgroundColour(*colour);
}
bool wxWindow_IsTransparentBackgroundSupported(const wxWindow * self, wxString * reason) {
    return self->IsTransparentBackgroundSupported(reason);
}
bool wxWindow_SetFont(wxWindow * self, const wxFont * font) {
    return self->SetFont(*font);
}
bool wxWindow_SetForegroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetForegroundColour(*colour);
}
void wxWindow_SetOwnBackgroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetOwnBackgroundColour(*colour);
}
bool wxWindow_InheritsBackgroundColour(const wxWindow * self) {
    return self->InheritsBackgroundColour();
}
bool wxWindow_UseBgCol(const wxWindow * self) {
    return self->UseBgCol();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseBackgroundColour(const wxWindow * self) {
    return self->UseBackgroundColour();
}
#endif
void wxWindow_SetOwnFont(wxWindow * self, const wxFont * font) {
    return self->SetOwnFont(*font);
}
void wxWindow_SetOwnForegroundColour(wxWindow * self, const wxColour * colour) {
    return self->SetOwnForegroundColour(*colour);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_UseForegroundColour(const wxWindow * self) {
    return self->UseForegroundColour();
}
bool wxWindow_InheritsForegroundColour(const wxWindow * self) {
    return self->InheritsForegroundColour();
}
#endif
void wxWindow_SetPalette(wxWindow * self, const wxPalette * pal) {
    return self->SetPalette(*pal);
}
bool wxWindow_ShouldInheritColours(const wxWindow * self) {
    return self->ShouldInheritColours();
}
void wxWindow_SetThemeEnabled(wxWindow * self, bool enable) {
    return self->SetThemeEnabled(enable);
}
bool wxWindow_GetThemeEnabled(const wxWindow * self) {
    return self->GetThemeEnabled();
}
bool wxWindow_CanSetTransparent(wxWindow * self) {
    return self->CanSetTransparent();
}
bool wxWindow_SetTransparent(wxWindow * self, wxByte alpha) {
    return self->SetTransparent(alpha);
}
wxEvtHandler * wxWindow_GetEventHandler(const wxWindow * self) {
    return self->GetEventHandler();
}
bool wxWindow_HandleAsNavigationKey(wxWindow * self, const wxKeyEvent * event) {
    return self->HandleAsNavigationKey(*event);
}
bool wxWindow_HandleWindowEvent(const wxWindow * self, wxEvent * event) {
    return self->HandleWindowEvent(*event);
}
bool wxWindow_ProcessWindowEvent(wxWindow * self, wxEvent * event) {
    return self->ProcessWindowEvent(*event);
}
bool wxWindow_ProcessWindowEventLocally(wxWindow * self, wxEvent * event) {
    return self->ProcessWindowEventLocally(*event);
}
wxEvtHandler * wxWindow_PopEventHandler(wxWindow * self, bool delete_handler) {
    return self->PopEventHandler(delete_handler);
}
void wxWindow_PushEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return self->PushEventHandler(handler);
}
bool wxWindow_RemoveEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return self->RemoveEventHandler(handler);
}
void wxWindow_SetEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return self->SetEventHandler(handler);
}
long wxWindow_GetExtraStyle(const wxWindow * self) {
    return self->GetExtraStyle();
}
long wxWindow_GetWindowStyleFlag(const wxWindow * self) {
    return self->GetWindowStyleFlag();
}
long wxWindow_GetWindowStyle(const wxWindow * self) {
    return self->GetWindowStyle();
}
bool wxWindow_HasExtraStyle(const wxWindow * self, int ex_flag) {
    return self->HasExtraStyle(ex_flag);
}
bool wxWindow_HasFlag(const wxWindow * self, int flag) {
    return self->HasFlag(flag);
}
void wxWindow_SetExtraStyle(wxWindow * self, long ex_style) {
    return self->SetExtraStyle(ex_style);
}
void wxWindow_SetWindowStyleFlag(wxWindow * self, long style) {
    return self->SetWindowStyleFlag(style);
}
void wxWindow_SetWindowStyle(wxWindow * self, long style) {
    return self->SetWindowStyle(style);
}
bool wxWindow_ToggleWindowStyle(wxWindow * self, int flag) {
    return self->ToggleWindowStyle(flag);
}
void wxWindow_MoveAfterInTabOrder(wxWindow * self, wxWindow * win) {
    return self->MoveAfterInTabOrder(win);
}
void wxWindow_MoveBeforeInTabOrder(wxWindow * self, wxWindow * win) {
    return self->MoveBeforeInTabOrder(win);
}
bool wxWindow_Navigate(wxWindow * self, int flags) {
    return self->Navigate(flags);
}
bool wxWindow_NavigateIn(wxWindow * self, int flags) {
    return self->NavigateIn(flags);
}
void wxWindow_Lower(wxWindow * self) {
    return self->Lower();
}
void wxWindow_Raise(wxWindow * self) {
    return self->Raise();
}
bool wxWindow_Hide(wxWindow * self) {
    return self->Hide();
}
bool wxWindow_IsEnabled(const wxWindow * self) {
    return self->IsEnabled();
}
bool wxWindow_IsExposed(const wxWindow * self, int x, int y) {
    return self->IsExposed(x, y);
}
bool wxWindow_IsExposed1(const wxWindow * self, wxPoint * pt) {
    return self->IsExposed(*pt);
}
bool wxWindow_IsExposed2(const wxWindow * self, int x, int y, int w, int h) {
    return self->IsExposed(x, y, w, h);
}
bool wxWindow_IsExposed3(const wxWindow * self, wxRect * rect) {
    return self->IsExposed(*rect);
}
bool wxWindow_IsShown(const wxWindow * self) {
    return self->IsShown();
}
bool wxWindow_IsShownOnScreen(const wxWindow * self) {
    return self->IsShownOnScreen();
}
bool wxWindow_Disable(wxWindow * self) {
    return self->Disable();
}
bool wxWindow_Enable(wxWindow * self, bool enable) {
    return self->Enable(enable);
}
bool wxWindow_Show(wxWindow * self, bool show) {
    return self->Show(show);
}
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxWindow_GetHelpText(const wxWindow * self) {
    return new wxString(self->GetHelpText());
}
#endif
void wxWindow_SetHelpText(wxWindow * self, const wxString * help_text) {
    return self->SetHelpText(*help_text);
}
wxToolTip * wxWindow_GetToolTip(const wxWindow * self) {
    return self->GetToolTip();
}
wxString *wxWindow_GetToolTipText(const wxWindow * self) {
    return new wxString(self->GetToolTipText());
}
void wxWindow_SetToolTip(wxWindow * self, const wxString * tip_string) {
    return self->SetToolTip(*tip_string);
}
void wxWindow_SetToolTip1(wxWindow * self, wxToolTip * tip) {
    return self->SetToolTip(tip);
}
void wxWindow_UnsetToolTip(wxWindow * self) {
    return self->UnsetToolTip();
}
int wxWindow_GetPopupMenuSelectionFromUser(wxWindow * self, wxMenu * menu, const wxPoint * pos) {
    return self->GetPopupMenuSelectionFromUser(*menu, *pos);
}
int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow * self, wxMenu * menu, int x, int y) {
    return self->GetPopupMenuSelectionFromUser(*menu, x, y);
}
bool wxWindow_PopupMenu(wxWindow * self, wxMenu * menu, const wxPoint * pos) {
    return self->PopupMenu(menu, *pos);
}
bool wxWindow_PopupMenu1(wxWindow * self, wxMenu * menu, int x, int y) {
    return self->PopupMenu(menu, x, y);
}
wxValidator * wxWindow_GetValidator(wxWindow * self) {
    return self->GetValidator();
}
void wxWindow_SetValidator(wxWindow * self, const wxValidator * validator) {
    return self->SetValidator(*validator);
}
bool wxWindow_TransferDataFromWindow(wxWindow * self) {
    return self->TransferDataFromWindow();
}
bool wxWindow_TransferDataToWindow(wxWindow * self) {
    return self->TransferDataToWindow();
}
bool wxWindow_Validate(wxWindow * self) {
    return self->Validate();
}
wxWindowID wxWindow_GetId(const wxWindow * self) {
    return self->GetId();
}
wxString *wxWindow_GetLabel(const wxWindow * self) {
    return new wxString(self->GetLabel());
}
wxLayoutDirection wxWindow_GetLayoutDirection(const wxWindow * self) {
    return self->GetLayoutDirection();
}
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total) {
    return self->AdjustForLayoutDirection(x, width, width_total);
}
wxString *wxWindow_GetName(const wxWindow * self) {
    return new wxString(self->GetName());
}
void wxWindow_SetId(wxWindow * self, wxWindowID winid) {
    return self->SetId(winid);
}
void wxWindow_SetLabel(wxWindow * self, const wxString * label) {
    return self->SetLabel(*label);
}
void wxWindow_SetLayoutDirection(wxWindow * self, wxLayoutDirection dir) {
    return self->SetLayoutDirection(dir);
}
void wxWindow_SetName(wxWindow * self, const wxString * name) {
    return self->SetName(*name);
}
wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow * self) {
    return self->GetAcceleratorTable();
}
void wxWindow_SetAcceleratorTable(wxWindow * self, const wxAcceleratorTable * accel) {
    return self->SetAcceleratorTable(*accel);
}
bool wxWindow_Close(wxWindow * self, bool force) {
    return self->Close(force);
}
bool wxWindow_Destroy(wxWindow * self) {
    return self->Destroy();
}
bool wxWindow_IsBeingDeleted(const wxWindow * self) {
    return self->IsBeingDeleted();
}
wxDropTarget * wxWindow_GetDropTarget(const wxWindow * self) {
    return self->GetDropTarget();
}
void wxWindow_SetDropTarget(wxWindow * self, wxDropTarget * target) {
    return self->SetDropTarget(target);
}
void wxWindow_DragAcceptFiles(wxWindow * self, bool accept) {
    return self->DragAcceptFiles(accept);
}
wxSizer * wxWindow_GetContainingSizer(const wxWindow * self) {
    return self->GetContainingSizer();
}
wxSizer * wxWindow_GetSizer(const wxWindow * self) {
    return self->GetSizer();
}
void wxWindow_SetSizer(wxWindow * self, wxSizer * sizer, bool delete_old) {
    return self->SetSizer(sizer, delete_old);
}
void wxWindow_SetSizerAndFit(wxWindow * self, wxSizer * sizer, bool delete_old) {
    return self->SetSizerAndFit(sizer, delete_old);
}
wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow * self) {
    return self->GetConstraints();
}
void wxWindow_SetConstraints(wxWindow * self, wxLayoutConstraints * constraints) {
    return self->SetConstraints(constraints);
}
bool wxWindow_Layout(wxWindow * self) {
    return self->Layout();
}
void wxWindow_SetAutoLayout(wxWindow * self, bool auto_layout) {
    return self->SetAutoLayout(auto_layout);
}
bool wxWindow_GetAutoLayout(const wxWindow * self) {
    return self->GetAutoLayout();
}
void wxWindow_CaptureMouse(wxWindow * self) {
    return self->CaptureMouse();
}
wxCaret * wxWindow_GetCaret(const wxWindow * self) {
    return self->GetCaret();
}
bool wxWindow_HasCapture(const wxWindow * self) {
    return self->HasCapture();
}
void wxWindow_ReleaseMouse(wxWindow * self) {
    return self->ReleaseMouse();
}
void wxWindow_SetCaret(wxWindow * self, wxCaret * caret) {
    return self->SetCaret(caret);
}
bool wxWindow_SetCursor(wxWindow * self, const wxCursor * cursor) {
    return self->SetCursor(*cursor);
}
void wxWindow_WarpPointer(wxWindow * self, int x, int y) {
    return self->WarpPointer(x, y);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxWindow_EnableTouchEvents(wxWindow * self, int events_mask) {
    return self->EnableTouchEvents(events_mask);
}
#endif
void wxWindow_DoUpdateWindowUI(wxWindow * self, wxUpdateUIEvent * event) {
    return self->DoUpdateWindowUI(*event);
}
bool wxWindow_HasMultiplePages(const wxWindow * self) {
    return self->HasMultiplePages();
}
void wxWindow_InheritAttributes(wxWindow * self) {
    return self->InheritAttributes();
}
void wxWindow_InitDialog(wxWindow * self) {
    return self->InitDialog();
}
bool wxWindow_IsDoubleBuffered(const wxWindow * self) {
    return self->IsDoubleBuffered();
}
void wxWindow_SetDoubleBuffered(wxWindow * self, bool on) {
    return self->SetDoubleBuffered(on);
}
bool wxWindow_IsRetained(const wxWindow * self) {
    return self->IsRetained();
}
bool wxWindow_IsThisEnabled(const wxWindow * self) {
    return self->IsThisEnabled();
}
bool wxWindow_IsTopLevel(const wxWindow * self) {
    return self->IsTopLevel();
}
void wxWindow_OnInternalIdle(wxWindow * self) {
    return self->OnInternalIdle();
}
bool wxWindow_SendIdleEvents(wxWindow * self, wxIdleEvent * event) {
    return self->SendIdleEvents(*event);
}
#ifndef __WXGTK__
bool wxWindow_RegisterHotKey(wxWindow * self, int hotkey_id, int modifiers, int virtual_key_code) {
    return self->RegisterHotKey(hotkey_id, modifiers, virtual_key_code);
}
bool wxWindow_UnregisterHotKey(wxWindow * self, int hotkey_id) {
    return self->UnregisterHotKey(hotkey_id);
}
#endif
void wxWindow_UpdateWindowUI(wxWindow * self, long flags) {
    return self->UpdateWindowUI(flags);
}
wxWindow * wxWindow_FindFocus() {
    return wxWindow::FindFocus();
}
wxWindow * wxWindow_FindWindowById(long id, const wxWindow * parent) {
    return wxWindow::FindWindowById(id, parent);
}
wxWindow * wxWindow_FindWindowByLabel(const wxString * label, const wxWindow * parent) {
    return wxWindow::FindWindowByLabel(*label, parent);
}
wxWindow * wxWindow_FindWindowByName(const wxString * name, const wxWindow * parent) {
    return wxWindow::FindWindowByName(*name, parent);
}
wxWindow * wxWindow_GetCapture() {
    return wxWindow::GetCapture();
}
wxWindowID wxWindow_NewControlId(int count) {
    return wxWindow::NewControlId(count);
}
void wxWindow_UnreserveControlId(wxWindowID id, int count) {
    return wxWindow::UnreserveControlId(id, count);
}
wxWindow *wxWindow_new() {
    return new wxWindow();
}
wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxWindow(parent, id, *pos, *size, style, *name);
}
bool wxWindow_Create(wxWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}

// CLASS: wxWrapSizer
wxClassInfo *wxWrapSizer_CLASSINFO() {
    return wxCLASSINFO(wxWrapSizer);
}
wxWrapSizer *wxWrapSizer_new(int orient, int flags) {
    return new wxWrapSizer(orient, flags);
}

} // extern "C"

