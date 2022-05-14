#include "generated.h"

extern "C" {

// CLASS: wxCommandEvent
void * wxCommandEvent_GetClientData(const wxCommandEvent * self) {
    return self->GetClientData();
}
wxClientData * wxCommandEvent_GetClientObject(const wxCommandEvent * self) {
    return self->GetClientObject();
}
long wxCommandEvent_GetExtraLong(const wxCommandEvent * self) {
    return self->GetExtraLong();
}
int wxCommandEvent_GetInt(const wxCommandEvent * self) {
    return self->GetInt();
}
int wxCommandEvent_GetSelection(const wxCommandEvent * self) {
    return self->GetSelection();
}
wxString *wxCommandEvent_GetString(const wxCommandEvent * self) {
    return new wxString(self->GetString());
}
bool wxCommandEvent_IsChecked(const wxCommandEvent * self) {
    return self->IsChecked();
}
bool wxCommandEvent_IsSelection(const wxCommandEvent * self) {
    return self->IsSelection();
}
void wxCommandEvent_SetClientData(wxCommandEvent * self, void * client_data) {
    return self->SetClientData(client_data);
}
void wxCommandEvent_SetClientObject(wxCommandEvent * self, wxClientData * client_object) {
    return self->SetClientObject(client_object);
}
void wxCommandEvent_SetExtraLong(wxCommandEvent * self, long extra_long) {
    return self->SetExtraLong(extra_long);
}
void wxCommandEvent_SetInt(wxCommandEvent * self, int int_command) {
    return self->SetInt(int_command);
}
void wxCommandEvent_SetString(wxCommandEvent * self, const wxString * string) {
    return self->SetString(*string);
}

// CLASS: wxWindow
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
wxString *wxWindow_GetHelpText(const wxWindow * self) {
    return new wxString(self->GetHelpText());
}
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

// CLASS: wxControl
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxControl(parent, id, *pos, *size, style, *validator, *name);
}
wxControl *wxControl_new1() {
    return new wxControl();
}
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
void wxControl_Command(wxControl * self, wxCommandEvent * event) {
    return self->Command(*event);
}
wxString *wxControl_GetLabelText(const wxControl * self) {
    return new wxString(self->GetLabelText());
}
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen) {
    return new wxSize(self->GetSizeFromTextSize(xlen, ylen));
}
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize) {
    return new wxSize(self->GetSizeFromTextSize(*tsize));
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text) {
    return new wxSize(self->GetSizeFromText(*text));
}
#endif
void wxControl_SetLabelText(wxControl * self, const wxString * text) {
    return self->SetLabelText(*text);
}
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup) {
    return self->SetLabelMarkup(*markup);
}
wxString *wxControl_GetLabelText1(const wxString * label) {
    return new wxString(wxControl::GetLabelText(*label));
}
wxString *wxControl_RemoveMnemonics(const wxString * str) {
    return new wxString(wxControl::RemoveMnemonics(*str));
}
wxString *wxControl_EscapeMnemonics(const wxString * text) {
    return new wxString(wxControl::EscapeMnemonics(*text));
}
wxString *wxControl_Ellipsize(const wxString * label, const wxDC * dc, wxEllipsizeMode mode, int max_width, int flags) {
    return new wxString(wxControl::Ellipsize(*label, *dc, mode, max_width, flags));
}

// CLASS: wxAnyButton
wxAnyButton *wxAnyButton_new() {
    return new wxAnyButton();
}
void wxAnyButton_SetBitmapCurrent(wxAnyButton * self, const wxBitmap * bitmap) {
    return self->SetBitmapCurrent(*bitmap);
}
void wxAnyButton_SetBitmapDisabled(wxAnyButton * self, const wxBitmap * bitmap) {
    return self->SetBitmapDisabled(*bitmap);
}
void wxAnyButton_SetBitmapFocus(wxAnyButton * self, const wxBitmap * bitmap) {
    return self->SetBitmapFocus(*bitmap);
}
void wxAnyButton_SetBitmapLabel(wxAnyButton * self, const wxBitmap * bitmap) {
    return self->SetBitmapLabel(*bitmap);
}
void wxAnyButton_SetBitmapPressed(wxAnyButton * self, const wxBitmap * bitmap) {
    return self->SetBitmapPressed(*bitmap);
}
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton * self) {
    return new wxSize(self->GetBitmapMargins());
}
void wxAnyButton_SetBitmapMargins(wxAnyButton * self, wxCoord x, wxCoord y) {
    return self->SetBitmapMargins(x, y);
}
void wxAnyButton_SetBitmapMargins1(wxAnyButton * self, const wxSize * sz) {
    return self->SetBitmapMargins(*sz);
}

// CLASS: wxButton
wxButton *wxButton_new() {
    return new wxButton();
}
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxButton(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxButton_GetAuthNeeded(const wxButton * self) {
    return self->GetAuthNeeded();
}
void wxButton_SetAuthNeeded(wxButton * self, bool needed) {
    return self->SetAuthNeeded(needed);
}
wxWindow * wxButton_SetDefault(wxButton * self) {
    return self->SetDefault();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxButton_GetDefaultSize(wxWindow * win) {
    return new wxSize(wxButton::GetDefaultSize(win));
}
#endif

// CLASS: wxMenu
wxMenu *wxMenu_new() {
    return new wxMenu();
}
wxMenu *wxMenu_new1(long style) {
    return new wxMenu(style);
}
wxMenu *wxMenu_new2(const wxString * title, long style) {
    return new wxMenu(*title, style);
}
wxMenuItem * wxMenu_Append(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Append(id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Append1(wxMenu * self, int id, const wxString * item, wxMenu * sub_menu, const wxString * help_string) {
    return self->Append(id, *item, sub_menu, *help_string);
}
wxMenuItem * wxMenu_Append2(wxMenu * self, wxMenuItem * menu_item) {
    return self->Append(menu_item);
}
wxMenuItem * wxMenu_AppendCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help) {
    return self->AppendCheckItem(id, *item, *help);
}
wxMenuItem * wxMenu_AppendRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help) {
    return self->AppendRadioItem(id, *item, *help);
}
wxMenuItem * wxMenu_AppendSeparator(wxMenu * self) {
    return self->AppendSeparator();
}
wxMenuItem * wxMenu_AppendSubMenu(wxMenu * self, wxMenu * submenu, const wxString * text, const wxString * help) {
    return self->AppendSubMenu(submenu, *text, *help);
}
void wxMenu_Break(wxMenu * self) {
    return self->Break();
}
void wxMenu_Check(wxMenu * self, int id, bool check) {
    return self->Check(id, check);
}
bool wxMenu_Delete(wxMenu * self, int id) {
    return self->Delete(id);
}
bool wxMenu_Delete1(wxMenu * self, wxMenuItem * item) {
    return self->Delete(item);
}
bool wxMenu_Destroy(wxMenu * self, int id) {
    return self->Destroy(id);
}
bool wxMenu_Destroy1(wxMenu * self, wxMenuItem * item) {
    return self->Destroy(item);
}
void wxMenu_Enable(wxMenu * self, int id, bool enable) {
    return self->Enable(id, enable);
}
wxMenuItem * wxMenu_FindChildItem(const wxMenu * self, int id, size_t * pos) {
    return self->FindChildItem(id, pos);
}
int wxMenu_FindItem(const wxMenu * self, const wxString * item_string) {
    return self->FindItem(*item_string);
}
wxMenuItem * wxMenu_FindItem1(const wxMenu * self, int id, wxMenu ** menu) {
    return self->FindItem(id, menu);
}
wxString *wxMenu_GetHelpString(const wxMenu * self, int id) {
    return new wxString(self->GetHelpString(id));
}
wxString *wxMenu_GetLabel(const wxMenu * self, int id) {
    return new wxString(self->GetLabel(id));
}
wxString *wxMenu_GetLabelText(const wxMenu * self, int id) {
    return new wxString(self->GetLabelText(id));
}
wxString *wxMenu_GetTitle(const wxMenu * self) {
    return new wxString(self->GetTitle());
}
bool wxMenu_IsChecked(const wxMenu * self, int id) {
    return self->IsChecked(id);
}
bool wxMenu_IsEnabled(const wxMenu * self, int id) {
    return self->IsEnabled(id);
}
wxMenuItem * wxMenu_Prepend(wxMenu * self, wxMenuItem * item) {
    return self->Prepend(item);
}
wxMenuItem * wxMenu_Prepend1(wxMenu * self, int id, const wxString * item, const wxString * help_string, wxItemKind kind) {
    return self->Prepend(id, *item, *help_string, kind);
}
wxMenuItem * wxMenu_Prepend2(wxMenu * self, int id, const wxString * text, wxMenu * submenu, const wxString * help) {
    return self->Prepend(id, *text, submenu, *help);
}
wxMenuItem * wxMenu_PrependCheckItem(wxMenu * self, int id, const wxString * item, const wxString * help_string) {
    return self->PrependCheckItem(id, *item, *help_string);
}
wxMenuItem * wxMenu_PrependRadioItem(wxMenu * self, int id, const wxString * item, const wxString * help_string) {
    return self->PrependRadioItem(id, *item, *help_string);
}
wxMenuItem * wxMenu_PrependSeparator(wxMenu * self) {
    return self->PrependSeparator();
}
wxMenuItem * wxMenu_Remove(wxMenu * self, int id) {
    return self->Remove(id);
}
wxMenuItem * wxMenu_Remove1(wxMenu * self, wxMenuItem * item) {
    return self->Remove(item);
}
void wxMenu_SetHelpString(wxMenu * self, int id, const wxString * help_string) {
    return self->SetHelpString(id, *help_string);
}
void wxMenu_SetLabel(wxMenu * self, int id, const wxString * label) {
    return self->SetLabel(id, *label);
}
void wxMenu_SetTitle(wxMenu * self, const wxString * title) {
    return self->SetTitle(*title);
}
void wxMenu_UpdateUI(wxMenu * self, wxEvtHandler * source) {
    return self->UpdateUI(source);
}
void wxMenu_SetInvokingWindow(wxMenu * self, wxWindow * win) {
    return self->SetInvokingWindow(win);
}
wxWindow * wxMenu_GetInvokingWindow(const wxMenu * self) {
    return self->GetInvokingWindow();
}
wxWindow * wxMenu_GetWindow(const wxMenu * self) {
    return self->GetWindow();
}
long wxMenu_GetStyle(const wxMenu * self) {
    return self->GetStyle();
}
void wxMenu_SetParent(wxMenu * self, wxMenu * parent) {
    return self->SetParent(parent);
}
wxMenu * wxMenu_GetParent(const wxMenu * self) {
    return self->GetParent();
}
void wxMenu_Attach(wxMenu * self, wxMenuBar * menubar) {
    return self->Attach(menubar);
}
void wxMenu_Detach(wxMenu * self) {
    return self->Detach();
}
bool wxMenu_IsAttached(const wxMenu * self) {
    return self->IsAttached();
}

// CLASS: wxMenuBar
wxMenuBar *wxMenuBar_new(long style) {
    return new wxMenuBar(style);
}
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title) {
    return self->Append(menu, *title);
}
void wxMenuBar_Check(wxMenuBar * self, int id, bool check) {
    return self->Check(id, check);
}
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable) {
    return self->Enable(id, enable);
}
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu) {
    return self->FindItem(id, menu);
}
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title) {
    return self->FindMenu(*title);
}
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string) {
    return self->FindMenuItem(*menu_string, *item_string);
}
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id) {
    return new wxString(self->GetHelpString(id));
}
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id) {
    return new wxString(self->GetLabel(id));
}
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id) {
    return self->IsChecked(id);
}
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id) {
    return self->IsEnabled(id);
}
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string) {
    return self->SetHelpString(id, *help_string);
}
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label) {
    return self->SetLabel(id, *label);
}
#ifdef __WXOSX__
wxMenu * wxMenuBar_OSXGetAppleMenu(const wxMenuBar * self) {
    return self->OSXGetAppleMenu();
}
#endif
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self) {
    return self->GetFrame();
}
bool wxMenuBar_IsAttached(const wxMenuBar * self) {
    return self->IsAttached();
}
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame) {
    return self->Attach(frame);
}
void wxMenuBar_Detach(wxMenuBar * self) {
    return self->Detach();
}
#ifdef __WXOSX__
void wxMenuBar_MacSetCommonMenuBar(wxMenuBar * menubar) {
    return wxMenuBar::MacSetCommonMenuBar(menubar);
}
wxMenuBar * wxMenuBar_MacGetCommonMenuBar() {
    return wxMenuBar::MacGetCommonMenuBar();
}
#endif

// CLASS: wxNonOwnedWindow
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region) {
    return self->SetShape(*region);
}
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path) {
    return self->SetShape(*path);
}

// CLASS: wxTopLevelWindow
wxTopLevelWindow *wxTopLevelWindow_new() {
    return new wxTopLevelWindow();
}
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxTopLevelWindow(parent, id, *title, *pos, *size, style, *name);
}
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction) {
    return self->CenterOnScreen(direction);
}
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction) {
    return self->CentreOnScreen(direction);
}
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableCloseButton(enable);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableMaximizeButton(enable);
}
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable) {
    return self->EnableMinimizeButton(enable);
}
#endif
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self) {
    return self->GetDefaultItem();
}
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self) {
    return new wxString(self->GetTitle());
}
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize) {
    return self->Iconize(iconize);
}
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self) {
    return self->IsActive();
}
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self) {
    return self->IsAlwaysMaximized();
}
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self) {
    return self->IsFullScreen();
}
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self) {
    return self->IsIconized();
}
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self) {
    return self->IsMaximized();
}
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize) {
    return self->Maximize(maximize);
}
#ifdef __WXMSW__
wxMenu * wxTopLevelWindow_MSWGetSystemMenu(const wxTopLevelWindow * self) {
    return self->MSWGetSystemMenu();
}
#endif
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags) {
    return self->RequestUserAttention(flags);
}
void wxTopLevelWindow_Restore(wxTopLevelWindow * self) {
    return self->Restore();
}
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return self->SetDefaultItem(win);
}
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return self->SetTmpDefaultItem(win);
}
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self) {
    return self->GetTmpDefaultItem();
}
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons) {
    return self->SetIcons(*icons);
}
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title) {
    return self->SetTitle(*title);
}
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self) {
    return self->ShouldPreventAppExit();
}
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified) {
    return self->OSXSetModified(modified);
}
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self) {
    return self->OSXIsModified();
}
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename) {
    return self->SetRepresentedFilename(*filename);
}
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self) {
    return self->ShowWithoutActivating();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable) {
    return self->EnableFullScreenView(enable);
}
#endif
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, long style) {
    return self->ShowFullScreen(show, style);
}
wxSize *wxTopLevelWindow_GetDefaultSize() {
    return new wxSize(wxTopLevelWindow::GetDefaultSize());
}

// CLASS: wxFrame
wxFrame *wxFrame_new() {
    return new wxFrame();
}
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxFrame(parent, id, *title, *pos, *size, style, *name);
}
void wxFrame_Centre(wxFrame * self, int direction) {
    return self->Centre(direction);
}
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name) {
    return self->CreateStatusBar(number, style, id, *name);
}
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name) {
    return self->CreateToolBar(style, id, *name);
}
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show) {
    return self->DoGiveHelp(*text, show);
}
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self) {
    return self->GetMenuBar();
}
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self) {
    return self->GetStatusBar();
}
int wxFrame_GetStatusBarPane(const wxFrame * self) {
    return self->GetStatusBarPane();
}
wxToolBar * wxFrame_GetToolBar(const wxFrame * self) {
    return self->GetToolBar();
}
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name) {
    return self->OnCreateStatusBar(number, style, id, *name);
}
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name) {
    return self->OnCreateToolBar(style, id, *name);
}
bool wxFrame_ProcessCommand(wxFrame * self, int id) {
    return self->ProcessCommand(id);
}
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar) {
    return self->SetMenuBar(menu_bar);
}
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar) {
    return self->SetStatusBar(status_bar);
}
void wxFrame_SetStatusBarPane(wxFrame * self, int n) {
    return self->SetStatusBarPane(n);
}
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number) {
    return self->SetStatusText(*text, number);
}
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field) {
    return self->SetStatusWidths(n, widths_field);
}
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar) {
    return self->SetToolBar(tool_bar);
}
#ifdef __WXMSW__
wxTaskBarButton * wxFrame_MSWGetTaskBarButton(wxFrame * self) {
    return self->MSWGetTaskBarButton();
}
#endif
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number) {
    return self->PushStatusText(*text, number);
}
void wxFrame_PopStatusText(wxFrame * self, int number) {
    return self->PopStatusText(number);
}

// CLASS: wxPanel
wxPanel *wxPanel_new() {
    return new wxPanel();
}
wxPanel *wxPanel_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxPanel(parent, id, *pos, *size, style, *name);
}
bool wxPanel_Create(wxPanel * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
void wxPanel_OnSysColourChanged(wxPanel * self, wxSysColourChangedEvent * event) {
    return self->OnSysColourChanged(*event);
}
void wxPanel_SetFocusIgnoringChildren(wxPanel * self) {
    return self->SetFocusIgnoringChildren();
}

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self) {
    delete self;
}
bool wxPoint_IsFullySpecified(const wxPoint * self) {
    return self->IsFullySpecified();
}
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt) {
    return self->SetDefaults(*pt);
}
wxPoint *wxPoint_new() {
    return new wxPoint();
}
wxPoint *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
wxPoint *wxPoint_new2(const wxRealPoint * pt) {
    return new wxPoint(*pt);
}

// CLASS: wxRect
void wxRect_delete(wxRect *self) {
    delete self;
}
wxRect *wxRect_new() {
    return new wxRect();
}
wxRect *wxRect_new1(int x, int y, int width, int height) {
    return new wxRect(x, y, width, height);
}
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right) {
    return new wxRect(*top_left, *bottom_right);
}
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size) {
    return new wxRect(*pos, *size);
}
wxRect *wxRect_new4(const wxSize * size) {
    return new wxRect(*size);
}
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(self->CentreIn(*r, dir));
}
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(self->CenterIn(*r, dir));
}
bool wxRect_Contains(const wxRect * self, int x, int y) {
    return self->Contains(x, y);
}
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt) {
    return self->Contains(*pt);
}
bool wxRect_Contains2(const wxRect * self, const wxRect * rect) {
    return self->Contains(*rect);
}
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(self->Deflate(dx, dy));
}
int wxRect_GetBottom(const wxRect * self) {
    return self->GetBottom();
}
wxPoint *wxRect_GetBottomLeft(const wxRect * self) {
    return new wxPoint(self->GetBottomLeft());
}
wxPoint *wxRect_GetBottomRight(const wxRect * self) {
    return new wxPoint(self->GetBottomRight());
}
int wxRect_GetHeight(const wxRect * self) {
    return self->GetHeight();
}
int wxRect_GetLeft(const wxRect * self) {
    return self->GetLeft();
}
wxPoint *wxRect_GetPosition(const wxRect * self) {
    return new wxPoint(self->GetPosition());
}
int wxRect_GetRight(const wxRect * self) {
    return self->GetRight();
}
wxSize *wxRect_GetSize(const wxRect * self) {
    return new wxSize(self->GetSize());
}
int wxRect_GetTop(const wxRect * self) {
    return self->GetTop();
}
wxPoint *wxRect_GetTopLeft(const wxRect * self) {
    return new wxPoint(self->GetTopLeft());
}
wxPoint *wxRect_GetTopRight(const wxRect * self) {
    return new wxPoint(self->GetTopRight());
}
int wxRect_GetWidth(const wxRect * self) {
    return self->GetWidth();
}
int wxRect_GetX(const wxRect * self) {
    return self->GetX();
}
int wxRect_GetY(const wxRect * self) {
    return self->GetY();
}
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(self->Inflate(dx, dy));
}
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect) {
    return new wxRect(self->Intersect(*rect));
}
bool wxRect_Intersects(const wxRect * self, const wxRect * rect) {
    return self->Intersects(*rect);
}
bool wxRect_IsEmpty(const wxRect * self) {
    return self->IsEmpty();
}
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy) {
    return self->Offset(dx, dy);
}
void wxRect_Offset1(wxRect * self, const wxPoint * pt) {
    return self->Offset(*pt);
}
void wxRect_SetHeight(wxRect * self, int height) {
    return self->SetHeight(height);
}
void wxRect_SetPosition(wxRect * self, const wxPoint * pos) {
    return self->SetPosition(*pos);
}
void wxRect_SetSize(wxRect * self, const wxSize * s) {
    return self->SetSize(*s);
}
void wxRect_SetWidth(wxRect * self, int width) {
    return self->SetWidth(width);
}
void wxRect_SetX(wxRect * self, int x) {
    return self->SetX(x);
}
void wxRect_SetY(wxRect * self, int y) {
    return self->SetY(y);
}
void wxRect_SetLeft(wxRect * self, int left) {
    return self->SetLeft(left);
}
void wxRect_SetRight(wxRect * self, int right) {
    return self->SetRight(right);
}
void wxRect_SetTop(wxRect * self, int top) {
    return self->SetTop(top);
}
void wxRect_SetBottom(wxRect * self, int bottom) {
    return self->SetBottom(bottom);
}
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p) {
    return self->SetTopLeft(*p);
}
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p) {
    return self->SetBottomRight(*p);
}
void wxRect_SetTopRight(wxRect * self, const wxPoint * p) {
    return self->SetTopRight(*p);
}
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p) {
    return self->SetBottomLeft(*p);
}
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect) {
    return new wxRect(self->Union(*rect));
}

// CLASS: wxSize
void wxSize_delete(wxSize *self) {
    delete self;
}
wxSize *wxSize_new() {
    return new wxSize();
}
wxSize *wxSize_new1(int width, int height) {
    return new wxSize(width, height);
}
void wxSize_DecBy(wxSize * self, const wxPoint * pt) {
    return self->DecBy(*pt);
}
void wxSize_DecBy1(wxSize * self, const wxSize * size) {
    return self->DecBy(*size);
}
void wxSize_DecBy2(wxSize * self, int dx, int dy) {
    return self->DecBy(dx, dy);
}
void wxSize_DecBy3(wxSize * self, int d) {
    return self->DecBy(d);
}
void wxSize_DecTo(wxSize * self, const wxSize * size) {
    return self->DecTo(*size);
}
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size) {
    return self->DecToIfSpecified(*size);
}
int wxSize_GetHeight(const wxSize * self) {
    return self->GetHeight();
}
int wxSize_GetWidth(const wxSize * self) {
    return self->GetWidth();
}
void wxSize_IncBy(wxSize * self, const wxPoint * pt) {
    return self->IncBy(*pt);
}
void wxSize_IncBy1(wxSize * self, const wxSize * size) {
    return self->IncBy(*size);
}
void wxSize_IncBy2(wxSize * self, int dx, int dy) {
    return self->IncBy(dx, dy);
}
void wxSize_IncBy3(wxSize * self, int d) {
    return self->IncBy(d);
}
void wxSize_IncTo(wxSize * self, const wxSize * size) {
    return self->IncTo(*size);
}
bool wxSize_IsFullySpecified(const wxSize * self) {
    return self->IsFullySpecified();
}
void wxSize_Set(wxSize * self, int width, int height) {
    return self->Set(width, height);
}
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default) {
    return self->SetDefaults(*size_default);
}
void wxSize_SetHeight(wxSize * self, int height) {
    return self->SetHeight(height);
}
void wxSize_SetWidth(wxSize * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxSizer
wxSizerItem * wxSizer_Add(wxSizer * self, wxWindow * window, const wxSizerFlags * flags) {
    return self->Add(window, *flags);
}
wxSizerItem * wxSizer_Add1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Add(sizer, *flags);
}
wxSizerItem * wxSizer_Add3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add5(wxSizer * self, int width, int height, const wxSizerFlags * flags) {
    return self->Add(width, height, *flags);
}
wxSizerItem * wxSizer_Add6(wxSizer * self, wxSizerItem * item) {
    return self->Add(item);
}
wxSizerItem * wxSizer_AddSpacer(wxSizer * self, int size) {
    return self->AddSpacer(size);
}
wxSizerItem * wxSizer_AddStretchSpacer(wxSizer * self, int prop) {
    return self->AddStretchSpacer(prop);
}
wxSize *wxSizer_CalcMin(wxSizer * self) {
    return new wxSize(self->CalcMin());
}
void wxSizer_Clear(wxSizer * self, bool delete_windows) {
    return self->Clear(delete_windows);
}
wxSize *wxSizer_ComputeFittingClientSize(wxSizer * self, wxWindow * window) {
    return new wxSize(self->ComputeFittingClientSize(window));
}
wxSize *wxSizer_ComputeFittingWindowSize(wxSizer * self, wxWindow * window) {
    return new wxSize(self->ComputeFittingWindowSize(window));
}
bool wxSizer_Detach(wxSizer * self, wxWindow * window) {
    return self->Detach(window);
}
bool wxSizer_Detach1(wxSizer * self, wxSizer * sizer) {
    return self->Detach(sizer);
}
bool wxSizer_Detach2(wxSizer * self, int index) {
    return self->Detach(index);
}
wxSize *wxSizer_Fit(wxSizer * self, wxWindow * window) {
    return new wxSize(self->Fit(window));
}
void wxSizer_FitInside(wxSizer * self, wxWindow * window) {
    return self->FitInside(window);
}
bool wxSizer_InformFirstDirection(wxSizer * self, int direction, int size, int available_other_dir) {
    return self->InformFirstDirection(direction, size, available_other_dir);
}
wxWindow * wxSizer_GetContainingWindow(const wxSizer * self) {
    return self->GetContainingWindow();
}
void wxSizer_SetContainingWindow(wxSizer * self, wxWindow * window) {
    return self->SetContainingWindow(window);
}
wxSizerItem * wxSizer_GetItem(wxSizer * self, wxWindow * window, bool recursive) {
    return self->GetItem(window, recursive);
}
wxSizerItem * wxSizer_GetItem1(wxSizer * self, wxSizer * sizer, bool recursive) {
    return self->GetItem(sizer, recursive);
}
wxSizerItem * wxSizer_GetItemById(wxSizer * self, int id, bool recursive) {
    return self->GetItemById(id, recursive);
}
wxSize *wxSizer_GetMinSize(wxSizer * self) {
    return new wxSize(self->GetMinSize());
}
wxPoint *wxSizer_GetPosition(const wxSizer * self) {
    return new wxPoint(self->GetPosition());
}
wxSize *wxSizer_GetSize(const wxSizer * self) {
    return new wxSize(self->GetSize());
}
bool wxSizer_Hide(wxSizer * self, wxWindow * window, bool recursive) {
    return self->Hide(window, recursive);
}
bool wxSizer_Hide1(wxSizer * self, wxSizer * sizer, bool recursive) {
    return self->Hide(sizer, recursive);
}
bool wxSizer_IsEmpty(const wxSizer * self) {
    return self->IsEmpty();
}
bool wxSizer_IsShown(const wxSizer * self, wxWindow * window) {
    return self->IsShown(window);
}
bool wxSizer_IsShown1(const wxSizer * self, wxSizer * sizer) {
    return self->IsShown(sizer);
}
void wxSizer_Layout(wxSizer * self) {
    return self->Layout();
}
wxSizerItem * wxSizer_Prepend(wxSizer * self, wxWindow * window, const wxSizerFlags * flags) {
    return self->Prepend(window, *flags);
}
wxSizerItem * wxSizer_Prepend1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Prepend(sizer, *flags);
}
wxSizerItem * wxSizer_Prepend3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend5(wxSizer * self, int width, int height, const wxSizerFlags * flags) {
    return self->Prepend(width, height, *flags);
}
wxSizerItem * wxSizer_Prepend6(wxSizer * self, wxSizerItem * item) {
    return self->Prepend(item);
}
wxSizerItem * wxSizer_PrependSpacer(wxSizer * self, int size) {
    return self->PrependSpacer(size);
}
wxSizerItem * wxSizer_PrependStretchSpacer(wxSizer * self, int prop) {
    return self->PrependStretchSpacer(prop);
}
void wxSizer_RepositionChildren(wxSizer * self, const wxSize * min_size) {
    return self->RepositionChildren(*min_size);
}
bool wxSizer_Remove1(wxSizer * self, wxSizer * sizer) {
    return self->Remove(sizer);
}
bool wxSizer_Remove2(wxSizer * self, int index) {
    return self->Remove(index);
}
bool wxSizer_Replace(wxSizer * self, wxWindow * oldwin, wxWindow * newwin, bool recursive) {
    return self->Replace(oldwin, newwin, recursive);
}
bool wxSizer_Replace1(wxSizer * self, wxSizer * oldsz, wxSizer * newsz, bool recursive) {
    return self->Replace(oldsz, newsz, recursive);
}
void wxSizer_SetDimension(wxSizer * self, int x, int y, int width, int height) {
    return self->SetDimension(x, y, width, height);
}
void wxSizer_SetDimension1(wxSizer * self, const wxPoint * pos, const wxSize * size) {
    return self->SetDimension(*pos, *size);
}
bool wxSizer_SetItemMinSize(wxSizer * self, wxWindow * window, int width, int height) {
    return self->SetItemMinSize(window, width, height);
}
bool wxSizer_SetItemMinSize1(wxSizer * self, wxWindow * window, const wxSize * size) {
    return self->SetItemMinSize(window, *size);
}
bool wxSizer_SetItemMinSize2(wxSizer * self, wxSizer * sizer, int width, int height) {
    return self->SetItemMinSize(sizer, width, height);
}
bool wxSizer_SetItemMinSize3(wxSizer * self, wxSizer * sizer, const wxSize * size) {
    return self->SetItemMinSize(sizer, *size);
}
void wxSizer_SetMinSize(wxSizer * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxSizer_SetMinSize1(wxSizer * self, int width, int height) {
    return self->SetMinSize(width, height);
}
void wxSizer_SetSizeHints(wxSizer * self, wxWindow * window) {
    return self->SetSizeHints(window);
}
bool wxSizer_Show(wxSizer * self, wxWindow * window, bool show, bool recursive) {
    return self->Show(window, show, recursive);
}
bool wxSizer_Show1(wxSizer * self, wxSizer * sizer, bool show, bool recursive) {
    return self->Show(sizer, show, recursive);
}
void wxSizer_ShowItems(wxSizer * self, bool show) {
    return self->ShowItems(show);
}

// CLASS: wxValidator
wxValidator *wxValidator_new() {
    return new wxValidator();
}
wxObject * wxValidator_Clone(const wxValidator * self) {
    return self->Clone();
}
wxWindow * wxValidator_GetWindow(const wxValidator * self) {
    return self->GetWindow();
}
void wxValidator_SetWindow(wxValidator * self, wxWindow * window) {
    return self->SetWindow(window);
}
bool wxValidator_TransferFromWindow(wxValidator * self) {
    return self->TransferFromWindow();
}
bool wxValidator_TransferToWindow(wxValidator * self) {
    return self->TransferToWindow();
}
bool wxValidator_Validate(wxValidator * self, wxWindow * parent) {
    return self->Validate(parent);
}
void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}

// CLASS: wxStaticBitmap
wxStaticBitmap *wxStaticBitmap_new() {
    return new wxStaticBitmap();
}
wxStaticBitmap *wxStaticBitmap_new1(wxWindow * parent, wxWindowID id, const wxBitmap * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticBitmap(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticBitmap_Create(wxStaticBitmap * self, wxWindow * parent, wxWindowID id, const wxBitmap * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
void wxStaticBitmap_SetBitmap(wxStaticBitmap * self, const wxBitmap * label) {
    return self->SetBitmap(*label);
}
void wxStaticBitmap_SetIcon(wxStaticBitmap * self, const wxIcon * label) {
    return self->SetIcon(*label);
}

} // extern "C"

