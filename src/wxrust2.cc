#include <wx/wx.h>

extern "C" {

// CLASS: wxObject
void *wxObject_new() {
    return new wxObject();
}
void *wxObject_new1(void * other) {
    return new wxObject(*(wxObject *)other);
}
void * wxObject_GetClassInfo(const void * self) {
    return ((wxObject *)self)->GetClassInfo();
}
void * wxObject_GetRefData(const void * self) {
    return ((wxObject *)self)->GetRefData();
}
bool wxObject_IsKindOf(const void * self, void * info) {
    return ((wxObject *)self)->IsKindOf((wxClassInfo *)info);
}
bool wxObject_IsSameAs(const void * self, void * obj) {
    return ((wxObject *)self)->IsSameAs(*(wxObject *)obj);
}
void wxObject_Ref(void * self, void * clone) {
    return ((wxObject *)self)->Ref(*(wxObject *)clone);
}
void wxObject_SetRefData(void * self, void * data) {
    return ((wxObject *)self)->SetRefData((wxObjectRefData *)data);
}
void wxObject_UnRef(void * self) {
    return ((wxObject *)self)->UnRef();
}
void wxObject_UnShare(void * self) {
    return ((wxObject *)self)->UnShare();
}

// CLASS: wxEvtHandler
void wxEvtHandler_QueueEvent(void * self, void * event) {
    return ((wxEvtHandler *)self)->QueueEvent((wxEvent *)event);
}
void wxEvtHandler_AddPendingEvent(void * self, void * event) {
    return ((wxEvtHandler *)self)->AddPendingEvent(*(wxEvent *)event);
}
bool wxEvtHandler_ProcessEvent(void * self, void * event) {
    return ((wxEvtHandler *)self)->ProcessEvent(*(wxEvent *)event);
}
bool wxEvtHandler_ProcessEventLocally(void * self, void * event) {
    return ((wxEvtHandler *)self)->ProcessEventLocally(*(wxEvent *)event);
}
bool wxEvtHandler_SafelyProcessEvent(void * self, void * event) {
    return ((wxEvtHandler *)self)->SafelyProcessEvent(*(wxEvent *)event);
}
void wxEvtHandler_ProcessPendingEvents(void * self) {
    return ((wxEvtHandler *)self)->ProcessPendingEvents();
}
void wxEvtHandler_DeletePendingEvents(void * self) {
    return ((wxEvtHandler *)self)->DeletePendingEvents();
}
void * wxEvtHandler_GetClientObject(const void * self) {
    return ((wxEvtHandler *)self)->GetClientObject();
}
void wxEvtHandler_SetClientObject(void * self, void * data) {
    return ((wxEvtHandler *)self)->SetClientObject((wxClientData *)data);
}
bool wxEvtHandler_GetEvtHandlerEnabled(const void * self) {
    return ((wxEvtHandler *)self)->GetEvtHandlerEnabled();
}
void * wxEvtHandler_GetNextHandler(const void * self) {
    return ((wxEvtHandler *)self)->GetNextHandler();
}
void * wxEvtHandler_GetPreviousHandler(const void * self) {
    return ((wxEvtHandler *)self)->GetPreviousHandler();
}
void wxEvtHandler_SetEvtHandlerEnabled(void * self, bool enabled) {
    return ((wxEvtHandler *)self)->SetEvtHandlerEnabled(enabled);
}
void wxEvtHandler_SetNextHandler(void * self, void * handler) {
    return ((wxEvtHandler *)self)->SetNextHandler((wxEvtHandler *)handler);
}
void wxEvtHandler_SetPreviousHandler(void * self, void * handler) {
    return ((wxEvtHandler *)self)->SetPreviousHandler((wxEvtHandler *)handler);
}
void wxEvtHandler_Unlink(void * self) {
    return ((wxEvtHandler *)self)->Unlink();
}
bool wxEvtHandler_IsUnlinked(const void * self) {
    return ((wxEvtHandler *)self)->IsUnlinked();
}
void wxEvtHandler_AddFilter(void * filter) {
    return wxEvtHandler::AddFilter((wxEventFilter *)filter);
}
void wxEvtHandler_RemoveFilter(void * filter) {
    return wxEvtHandler::RemoveFilter((wxEventFilter *)filter);
}
void *wxEvtHandler_new() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
bool wxWindow_AcceptsFocus(const void * self) {
    return ((wxWindow *)self)->AcceptsFocus();
}
bool wxWindow_AcceptsFocusFromKeyboard(const void * self) {
    return ((wxWindow *)self)->AcceptsFocusFromKeyboard();
}
bool wxWindow_AcceptsFocusRecursively(const void * self) {
    return ((wxWindow *)self)->AcceptsFocusRecursively();
}
void wxWindow_DisableFocusFromKeyboard(void * self) {
    return ((wxWindow *)self)->DisableFocusFromKeyboard();
}
bool wxWindow_IsFocusable(const void * self) {
    return ((wxWindow *)self)->IsFocusable();
}
bool wxWindow_CanAcceptFocus(const void * self) {
    return ((wxWindow *)self)->CanAcceptFocus();
}
bool wxWindow_CanAcceptFocusFromKeyboard(const void * self) {
    return ((wxWindow *)self)->CanAcceptFocusFromKeyboard();
}
bool wxWindow_HasFocus(const void * self) {
    return ((wxWindow *)self)->HasFocus();
}
void wxWindow_SetCanFocus(void * self, bool can_focus) {
    return ((wxWindow *)self)->SetCanFocus(can_focus);
}
void wxWindow_EnableVisibleFocus(void * self, bool enable) {
    return ((wxWindow *)self)->EnableVisibleFocus(enable);
}
void wxWindow_SetFocus(void * self) {
    return ((wxWindow *)self)->SetFocus();
}
void wxWindow_SetFocusFromKbd(void * self) {
    return ((wxWindow *)self)->SetFocusFromKbd();
}
void wxWindow_AddChild(void * self, void * child) {
    return ((wxWindow *)self)->AddChild((wxWindow *)child);
}
bool wxWindow_DestroyChildren(void * self) {
    return ((wxWindow *)self)->DestroyChildren();
}
void * wxWindow_FindWindow(const void * self, int32_t id) {
    return ((wxWindow *)self)->FindWindow(id);
}
void * wxWindow_FindWindow1(const void * self, void * name) {
    return ((wxWindow *)self)->FindWindow(*(wxString *)name);
}
void wxWindow_RemoveChild(void * self, void * child) {
    return ((wxWindow *)self)->RemoveChild((wxWindow *)child);
}
void * wxWindow_GetGrandParent(const void * self) {
    return ((wxWindow *)self)->GetGrandParent();
}
void * wxWindow_GetNextSibling(const void * self) {
    return ((wxWindow *)self)->GetNextSibling();
}
void * wxWindow_GetParent(const void * self) {
    return ((wxWindow *)self)->GetParent();
}
void * wxWindow_GetPrevSibling(const void * self) {
    return ((wxWindow *)self)->GetPrevSibling();
}
bool wxWindow_IsDescendant(const void * self, void * win) {
    return ((wxWindow *)self)->IsDescendant((wxWindow *)win);
}
bool wxWindow_Reparent(void * self, void * new_parent) {
    return ((wxWindow *)self)->Reparent((wxWindow *)new_parent);
}
void wxWindow_AlwaysShowScrollbars(void * self, bool hflag, bool vflag) {
    return ((wxWindow *)self)->AlwaysShowScrollbars(hflag, vflag);
}
int wxWindow_GetScrollPos(const void * self, int orientation) {
    return ((wxWindow *)self)->GetScrollPos(orientation);
}
int wxWindow_GetScrollRange(const void * self, int orientation) {
    return ((wxWindow *)self)->GetScrollRange(orientation);
}
int wxWindow_GetScrollThumb(const void * self, int orientation) {
    return ((wxWindow *)self)->GetScrollThumb(orientation);
}
bool wxWindow_CanScroll(const void * self, int orient) {
    return ((wxWindow *)self)->CanScroll(orient);
}
bool wxWindow_HasScrollbar(const void * self, int orient) {
    return ((wxWindow *)self)->HasScrollbar(orient);
}
bool wxWindow_IsScrollbarAlwaysShown(const void * self, int orient) {
    return ((wxWindow *)self)->IsScrollbarAlwaysShown(orient);
}
bool wxWindow_ScrollLines(void * self, int lines) {
    return ((wxWindow *)self)->ScrollLines(lines);
}
bool wxWindow_ScrollPages(void * self, int pages) {
    return ((wxWindow *)self)->ScrollPages(pages);
}
void wxWindow_ScrollWindow(void * self, int dx, int dy, void * rect) {
    return ((wxWindow *)self)->ScrollWindow(dx, dy, (wxRect *)rect);
}
bool wxWindow_LineUp(void * self) {
    return ((wxWindow *)self)->LineUp();
}
bool wxWindow_LineDown(void * self) {
    return ((wxWindow *)self)->LineDown();
}
bool wxWindow_PageUp(void * self) {
    return ((wxWindow *)self)->PageUp();
}
bool wxWindow_PageDown(void * self) {
    return ((wxWindow *)self)->PageDown();
}
void wxWindow_SetScrollPos(void * self, int orientation, int pos, bool refresh) {
    return ((wxWindow *)self)->SetScrollPos(orientation, pos, refresh);
}
void wxWindow_SetScrollbar(void * self, int orientation, int position, int thumb_size, int range, bool refresh) {
    return ((wxWindow *)self)->SetScrollbar(orientation, position, thumb_size, range, refresh);
}
bool wxWindow_BeginRepositioningChildren(void * self) {
    return ((wxWindow *)self)->BeginRepositioningChildren();
}
void wxWindow_EndRepositioningChildren(void * self) {
    return ((wxWindow *)self)->EndRepositioningChildren();
}
void wxWindow_CacheBestSize(const void * self, void * size) {
    return ((wxWindow *)self)->CacheBestSize(*(wxSize *)size);
}
void *wxWindow_ClientToWindowSize(const void * self, void * size) {
    return new wxSize(((wxWindow *)self)->ClientToWindowSize(*(wxSize *)size));
}
void *wxWindow_WindowToClientSize(const void * self, void * size) {
    return new wxSize(((wxWindow *)self)->WindowToClientSize(*(wxSize *)size));
}
void wxWindow_Fit(void * self) {
    return ((wxWindow *)self)->Fit();
}
void wxWindow_FitInside(void * self) {
    return ((wxWindow *)self)->FitInside();
}
void *wxWindow_FromDIP(const void * self, void * sz) {
    return new wxSize(((wxWindow *)self)->FromDIP(*(wxSize *)sz));
}
void *wxWindow_FromDIP1(const void * self, void * pt) {
    return new wxPoint(((wxWindow *)self)->FromDIP(*(wxPoint *)pt));
}
int wxWindow_FromDIP2(const void * self, int d) {
    return ((wxWindow *)self)->FromDIP(d);
}
void *wxWindow_ToDIP(const void * self, void * sz) {
    return new wxSize(((wxWindow *)self)->ToDIP(*(wxSize *)sz));
}
void *wxWindow_ToDIP1(const void * self, void * pt) {
    return new wxPoint(((wxWindow *)self)->ToDIP(*(wxPoint *)pt));
}
int wxWindow_ToDIP2(const void * self, int d) {
    return ((wxWindow *)self)->ToDIP(d);
}
void *wxWindow_GetBestSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetBestSize());
}
int wxWindow_GetBestHeight(const void * self, int width) {
    return ((wxWindow *)self)->GetBestHeight(width);
}
int wxWindow_GetBestWidth(const void * self, int height) {
    return ((wxWindow *)self)->GetBestWidth(height);
}
void wxWindow_GetClientSize(const void * self, void * width, void * height) {
    return ((wxWindow *)self)->GetClientSize((int *)width, (int *)height);
}
void *wxWindow_GetClientSize1(const void * self) {
    return new wxSize(((wxWindow *)self)->GetClientSize());
}
void *wxWindow_GetEffectiveMinSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetEffectiveMinSize());
}
void *wxWindow_GetMaxClientSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetMaxClientSize());
}
void *wxWindow_GetMaxSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetMaxSize());
}
void *wxWindow_GetMinClientSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetMinClientSize());
}
void *wxWindow_GetMinSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetMinSize());
}
int wxWindow_GetMinWidth(const void * self) {
    return ((wxWindow *)self)->GetMinWidth();
}
int wxWindow_GetMinHeight(const void * self) {
    return ((wxWindow *)self)->GetMinHeight();
}
int wxWindow_GetMaxWidth(const void * self) {
    return ((wxWindow *)self)->GetMaxWidth();
}
int wxWindow_GetMaxHeight(const void * self) {
    return ((wxWindow *)self)->GetMaxHeight();
}
void wxWindow_GetSize(const void * self, void * width, void * height) {
    return ((wxWindow *)self)->GetSize((int *)width, (int *)height);
}
void *wxWindow_GetSize1(const void * self) {
    return new wxSize(((wxWindow *)self)->GetSize());
}
void *wxWindow_GetVirtualSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetVirtualSize());
}
void wxWindow_GetVirtualSize1(const void * self, void * width, void * height) {
    return ((wxWindow *)self)->GetVirtualSize((int *)width, (int *)height);
}
void *wxWindow_GetBestVirtualSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetBestVirtualSize());
}
double wxWindow_GetContentScaleFactor(const void * self) {
    return ((wxWindow *)self)->GetContentScaleFactor();
}
double wxWindow_GetDPIScaleFactor(const void * self) {
    return ((wxWindow *)self)->GetDPIScaleFactor();
}
void *wxWindow_GetWindowBorderSize(const void * self) {
    return new wxSize(((wxWindow *)self)->GetWindowBorderSize());
}
bool wxWindow_InformFirstDirection(void * self, int direction, int size, int available_other_dir) {
    return ((wxWindow *)self)->InformFirstDirection(direction, size, available_other_dir);
}
void wxWindow_InvalidateBestSize(void * self) {
    return ((wxWindow *)self)->InvalidateBestSize();
}
void wxWindow_PostSizeEvent(void * self) {
    return ((wxWindow *)self)->PostSizeEvent();
}
void wxWindow_PostSizeEventToParent(void * self) {
    return ((wxWindow *)self)->PostSizeEventToParent();
}
void wxWindow_SendSizeEvent(void * self, int flags) {
    return ((wxWindow *)self)->SendSizeEvent(flags);
}
void wxWindow_SendSizeEventToParent(void * self, int flags) {
    return ((wxWindow *)self)->SendSizeEventToParent(flags);
}
void wxWindow_SetClientSize(void * self, int width, int height) {
    return ((wxWindow *)self)->SetClientSize(width, height);
}
void wxWindow_SetClientSize1(void * self, void * size) {
    return ((wxWindow *)self)->SetClientSize(*(wxSize *)size);
}
void wxWindow_SetClientSize2(void * self, void * rect) {
    return ((wxWindow *)self)->SetClientSize(*(wxRect *)rect);
}
void wxWindow_SetContainingSizer(void * self, void * sizer) {
    return ((wxWindow *)self)->SetContainingSizer((wxSizer *)sizer);
}
void wxWindow_SetInitialSize(void * self, void * size) {
    return ((wxWindow *)self)->SetInitialSize(*(wxSize *)size);
}
void wxWindow_SetMaxClientSize(void * self, void * size) {
    return ((wxWindow *)self)->SetMaxClientSize(*(wxSize *)size);
}
void wxWindow_SetMaxSize(void * self, void * size) {
    return ((wxWindow *)self)->SetMaxSize(*(wxSize *)size);
}
void wxWindow_SetMinClientSize(void * self, void * size) {
    return ((wxWindow *)self)->SetMinClientSize(*(wxSize *)size);
}
void wxWindow_SetMinSize(void * self, void * size) {
    return ((wxWindow *)self)->SetMinSize(*(wxSize *)size);
}
void wxWindow_SetSize(void * self, int x, int y, int width, int height, int size_flags) {
    return ((wxWindow *)self)->SetSize(x, y, width, height, size_flags);
}
void wxWindow_SetSize1(void * self, void * rect) {
    return ((wxWindow *)self)->SetSize(*(wxRect *)rect);
}
void wxWindow_SetSize2(void * self, void * size) {
    return ((wxWindow *)self)->SetSize(*(wxSize *)size);
}
void wxWindow_SetSize3(void * self, int width, int height) {
    return ((wxWindow *)self)->SetSize(width, height);
}
void wxWindow_SetSizeHints(void * self, void * min_size, void * max_size, void * inc_size) {
    return ((wxWindow *)self)->SetSizeHints(*(wxSize *)min_size, *(wxSize *)max_size, *(wxSize *)inc_size);
}
void wxWindow_SetSizeHints1(void * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return ((wxWindow *)self)->SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
void wxWindow_SetVirtualSize(void * self, int width, int height) {
    return ((wxWindow *)self)->SetVirtualSize(width, height);
}
void wxWindow_SetVirtualSize1(void * self, void * size) {
    return ((wxWindow *)self)->SetVirtualSize(*(wxSize *)size);
}
void *wxWindow_FromDIP3(void * sz, void * w) {
    return new wxSize(wxWindow::FromDIP(*(wxSize *)sz, (wxWindow *)w));
}
void *wxWindow_FromDIP4(void * pt, void * w) {
    return new wxPoint(wxWindow::FromDIP(*(wxPoint *)pt, (wxWindow *)w));
}
int wxWindow_FromDIP5(int d, void * w) {
    return wxWindow::FromDIP(d, (wxWindow *)w);
}
void *wxWindow_ToDIP3(void * sz, void * w) {
    return new wxSize(wxWindow::ToDIP(*(wxSize *)sz, (wxWindow *)w));
}
void *wxWindow_ToDIP4(void * pt, void * w) {
    return new wxPoint(wxWindow::ToDIP(*(wxPoint *)pt, (wxWindow *)w));
}
int wxWindow_ToDIP5(int d, void * w) {
    return wxWindow::ToDIP(d, (wxWindow *)w);
}
void wxWindow_Center(void * self, int dir) {
    return ((wxWindow *)self)->Center(dir);
}
void wxWindow_CenterOnParent(void * self, int dir) {
    return ((wxWindow *)self)->CenterOnParent(dir);
}
void wxWindow_Centre(void * self, int direction) {
    return ((wxWindow *)self)->Centre(direction);
}
void wxWindow_CentreOnParent(void * self, int direction) {
    return ((wxWindow *)self)->CentreOnParent(direction);
}
void wxWindow_GetPosition(const void * self, void * x, void * y) {
    return ((wxWindow *)self)->GetPosition((int *)x, (int *)y);
}
void *wxWindow_GetPosition1(const void * self) {
    return new wxPoint(((wxWindow *)self)->GetPosition());
}
void *wxWindow_GetRect(const void * self) {
    return new wxRect(((wxWindow *)self)->GetRect());
}
void wxWindow_GetScreenPosition(const void * self, void * x, void * y) {
    return ((wxWindow *)self)->GetScreenPosition((int *)x, (int *)y);
}
void *wxWindow_GetScreenPosition1(const void * self) {
    return new wxPoint(((wxWindow *)self)->GetScreenPosition());
}
void *wxWindow_GetScreenRect(const void * self) {
    return new wxRect(((wxWindow *)self)->GetScreenRect());
}
void *wxWindow_GetClientAreaOrigin(const void * self) {
    return new wxPoint(((wxWindow *)self)->GetClientAreaOrigin());
}
void *wxWindow_GetClientRect(const void * self) {
    return new wxRect(((wxWindow *)self)->GetClientRect());
}
void wxWindow_Move(void * self, int x, int y, int flags) {
    return ((wxWindow *)self)->Move(x, y, flags);
}
void wxWindow_Move1(void * self, void * pt, int flags) {
    return ((wxWindow *)self)->Move(*(wxPoint *)pt, flags);
}
void wxWindow_SetPosition(void * self, void * pt) {
    return ((wxWindow *)self)->SetPosition(*(wxPoint *)pt);
}
void wxWindow_ClientToScreen(const void * self, void * x, void * y) {
    return ((wxWindow *)self)->ClientToScreen((int *)x, (int *)y);
}
void *wxWindow_ClientToScreen1(const void * self, void * pt) {
    return new wxPoint(((wxWindow *)self)->ClientToScreen(*(wxPoint *)pt));
}
void *wxWindow_ConvertDialogToPixels(const void * self, void * pt) {
    return new wxPoint(((wxWindow *)self)->ConvertDialogToPixels(*(wxPoint *)pt));
}
void *wxWindow_ConvertDialogToPixels1(const void * self, void * sz) {
    return new wxSize(((wxWindow *)self)->ConvertDialogToPixels(*(wxSize *)sz));
}
void *wxWindow_ConvertPixelsToDialog(const void * self, void * pt) {
    return new wxPoint(((wxWindow *)self)->ConvertPixelsToDialog(*(wxPoint *)pt));
}
void *wxWindow_ConvertPixelsToDialog1(const void * self, void * sz) {
    return new wxSize(((wxWindow *)self)->ConvertPixelsToDialog(*(wxSize *)sz));
}
void wxWindow_ScreenToClient(const void * self, void * x, void * y) {
    return ((wxWindow *)self)->ScreenToClient((int *)x, (int *)y);
}
void *wxWindow_ScreenToClient1(const void * self, void * pt) {
    return new wxPoint(((wxWindow *)self)->ScreenToClient(*(wxPoint *)pt));
}
void wxWindow_ClearBackground(void * self) {
    return ((wxWindow *)self)->ClearBackground();
}
void wxWindow_Freeze(void * self) {
    return ((wxWindow *)self)->Freeze();
}
void wxWindow_Thaw(void * self) {
    return ((wxWindow *)self)->Thaw();
}
bool wxWindow_IsFrozen(const void * self) {
    return ((wxWindow *)self)->IsFrozen();
}
int wxWindow_GetCharHeight(const void * self) {
    return ((wxWindow *)self)->GetCharHeight();
}
int wxWindow_GetCharWidth(const void * self) {
    return ((wxWindow *)self)->GetCharWidth();
}
void *wxWindow_GetDPI(const void * self) {
    return new wxSize(((wxWindow *)self)->GetDPI());
}
void wxWindow_GetTextExtent(const void * self, void * string, void * w, void * h, void * descent, void * external_leading, void * font) {
    return ((wxWindow *)self)->GetTextExtent(*(wxString *)string, (int *)w, (int *)h, (int *)descent, (int *)external_leading, (wxFont *)font);
}
void *wxWindow_GetTextExtent1(const void * self, void * string) {
    return new wxSize(((wxWindow *)self)->GetTextExtent(*(wxString *)string));
}
void *wxWindow_GetUpdateClientRect(const void * self) {
    return new wxRect(((wxWindow *)self)->GetUpdateClientRect());
}
bool wxWindow_HasTransparentBackground(void * self) {
    return ((wxWindow *)self)->HasTransparentBackground();
}
void wxWindow_Refresh(void * self, bool erase_background, void * rect) {
    return ((wxWindow *)self)->Refresh(erase_background, (wxRect *)rect);
}
void wxWindow_RefreshRect(void * self, void * rect, bool erase_background) {
    return ((wxWindow *)self)->RefreshRect(*(wxRect *)rect, erase_background);
}
void wxWindow_Update(void * self) {
    return ((wxWindow *)self)->Update();
}
bool wxWindow_SetBackgroundColour(void * self, void * colour) {
    return ((wxWindow *)self)->SetBackgroundColour(*(wxColour *)colour);
}
bool wxWindow_IsTransparentBackgroundSupported(const void * self, void * reason) {
    return ((wxWindow *)self)->IsTransparentBackgroundSupported((wxString *)reason);
}
bool wxWindow_SetFont(void * self, void * font) {
    return ((wxWindow *)self)->SetFont(*(wxFont *)font);
}
bool wxWindow_SetForegroundColour(void * self, void * colour) {
    return ((wxWindow *)self)->SetForegroundColour(*(wxColour *)colour);
}
void wxWindow_SetOwnBackgroundColour(void * self, void * colour) {
    return ((wxWindow *)self)->SetOwnBackgroundColour(*(wxColour *)colour);
}
bool wxWindow_InheritsBackgroundColour(const void * self) {
    return ((wxWindow *)self)->InheritsBackgroundColour();
}
bool wxWindow_UseBgCol(const void * self) {
    return ((wxWindow *)self)->UseBgCol();
}
bool wxWindow_UseBackgroundColour(const void * self) {
    return ((wxWindow *)self)->UseBackgroundColour();
}
void wxWindow_SetOwnFont(void * self, void * font) {
    return ((wxWindow *)self)->SetOwnFont(*(wxFont *)font);
}
void wxWindow_SetOwnForegroundColour(void * self, void * colour) {
    return ((wxWindow *)self)->SetOwnForegroundColour(*(wxColour *)colour);
}
bool wxWindow_UseForegroundColour(const void * self) {
    return ((wxWindow *)self)->UseForegroundColour();
}
bool wxWindow_InheritsForegroundColour(const void * self) {
    return ((wxWindow *)self)->InheritsForegroundColour();
}
void wxWindow_SetPalette(void * self, void * pal) {
    return ((wxWindow *)self)->SetPalette(*(wxPalette *)pal);
}
bool wxWindow_ShouldInheritColours(const void * self) {
    return ((wxWindow *)self)->ShouldInheritColours();
}
void wxWindow_SetThemeEnabled(void * self, bool enable) {
    return ((wxWindow *)self)->SetThemeEnabled(enable);
}
bool wxWindow_GetThemeEnabled(const void * self) {
    return ((wxWindow *)self)->GetThemeEnabled();
}
bool wxWindow_CanSetTransparent(void * self) {
    return ((wxWindow *)self)->CanSetTransparent();
}
bool wxWindow_SetTransparent(void * self, wxByte alpha) {
    return ((wxWindow *)self)->SetTransparent(alpha);
}
void * wxWindow_GetEventHandler(const void * self) {
    return ((wxWindow *)self)->GetEventHandler();
}
bool wxWindow_HandleAsNavigationKey(void * self, void * event) {
    return ((wxWindow *)self)->HandleAsNavigationKey(*(wxKeyEvent *)event);
}
bool wxWindow_HandleWindowEvent(const void * self, void * event) {
    return ((wxWindow *)self)->HandleWindowEvent(*(wxEvent *)event);
}
bool wxWindow_ProcessWindowEvent(void * self, void * event) {
    return ((wxWindow *)self)->ProcessWindowEvent(*(wxEvent *)event);
}
bool wxWindow_ProcessWindowEventLocally(void * self, void * event) {
    return ((wxWindow *)self)->ProcessWindowEventLocally(*(wxEvent *)event);
}
void * wxWindow_PopEventHandler(void * self, bool delete_handler) {
    return ((wxWindow *)self)->PopEventHandler(delete_handler);
}
void wxWindow_PushEventHandler(void * self, void * handler) {
    return ((wxWindow *)self)->PushEventHandler((wxEvtHandler *)handler);
}
bool wxWindow_RemoveEventHandler(void * self, void * handler) {
    return ((wxWindow *)self)->RemoveEventHandler((wxEvtHandler *)handler);
}
void wxWindow_SetEventHandler(void * self, void * handler) {
    return ((wxWindow *)self)->SetEventHandler((wxEvtHandler *)handler);
}
void wxWindow_SetNextHandler(void * self, void * handler) {
    return ((wxWindow *)self)->SetNextHandler((wxEvtHandler *)handler);
}
void wxWindow_SetPreviousHandler(void * self, void * handler) {
    return ((wxWindow *)self)->SetPreviousHandler((wxEvtHandler *)handler);
}
int32_t wxWindow_GetExtraStyle(const void * self) {
    return ((wxWindow *)self)->GetExtraStyle();
}
int32_t wxWindow_GetWindowStyleFlag(const void * self) {
    return ((wxWindow *)self)->GetWindowStyleFlag();
}
int32_t wxWindow_GetWindowStyle(const void * self) {
    return ((wxWindow *)self)->GetWindowStyle();
}
bool wxWindow_HasExtraStyle(const void * self, int ex_flag) {
    return ((wxWindow *)self)->HasExtraStyle(ex_flag);
}
bool wxWindow_HasFlag(const void * self, int flag) {
    return ((wxWindow *)self)->HasFlag(flag);
}
void wxWindow_SetExtraStyle(void * self, int32_t ex_style) {
    return ((wxWindow *)self)->SetExtraStyle(ex_style);
}
void wxWindow_SetWindowStyleFlag(void * self, int32_t style) {
    return ((wxWindow *)self)->SetWindowStyleFlag(style);
}
void wxWindow_SetWindowStyle(void * self, int32_t style) {
    return ((wxWindow *)self)->SetWindowStyle(style);
}
bool wxWindow_ToggleWindowStyle(void * self, int flag) {
    return ((wxWindow *)self)->ToggleWindowStyle(flag);
}
void wxWindow_MoveAfterInTabOrder(void * self, void * win) {
    return ((wxWindow *)self)->MoveAfterInTabOrder((wxWindow *)win);
}
void wxWindow_MoveBeforeInTabOrder(void * self, void * win) {
    return ((wxWindow *)self)->MoveBeforeInTabOrder((wxWindow *)win);
}
bool wxWindow_Navigate(void * self, int flags) {
    return ((wxWindow *)self)->Navigate(flags);
}
bool wxWindow_NavigateIn(void * self, int flags) {
    return ((wxWindow *)self)->NavigateIn(flags);
}
void wxWindow_Lower(void * self) {
    return ((wxWindow *)self)->Lower();
}
void wxWindow_Raise(void * self) {
    return ((wxWindow *)self)->Raise();
}
bool wxWindow_Hide(void * self) {
    return ((wxWindow *)self)->Hide();
}
bool wxWindow_IsEnabled(const void * self) {
    return ((wxWindow *)self)->IsEnabled();
}
bool wxWindow_IsExposed(const void * self, int x, int y) {
    return ((wxWindow *)self)->IsExposed(x, y);
}
bool wxWindow_IsExposed1(const void * self, void * pt) {
    return ((wxWindow *)self)->IsExposed(*(wxPoint *)pt);
}
bool wxWindow_IsExposed2(const void * self, int x, int y, int w, int h) {
    return ((wxWindow *)self)->IsExposed(x, y, w, h);
}
bool wxWindow_IsExposed3(const void * self, void * rect) {
    return ((wxWindow *)self)->IsExposed(*(wxRect *)rect);
}
bool wxWindow_IsShown(const void * self) {
    return ((wxWindow *)self)->IsShown();
}
bool wxWindow_IsShownOnScreen(const void * self) {
    return ((wxWindow *)self)->IsShownOnScreen();
}
bool wxWindow_Disable(void * self) {
    return ((wxWindow *)self)->Disable();
}
bool wxWindow_Enable(void * self, bool enable) {
    return ((wxWindow *)self)->Enable(enable);
}
bool wxWindow_Show(void * self, bool show) {
    return ((wxWindow *)self)->Show(show);
}
void *wxWindow_GetHelpText(const void * self) {
    return new wxString(((wxWindow *)self)->GetHelpText());
}
void wxWindow_SetHelpText(void * self, void * help_text) {
    return ((wxWindow *)self)->SetHelpText(*(wxString *)help_text);
}
void * wxWindow_GetToolTip(const void * self) {
    return ((wxWindow *)self)->GetToolTip();
}
void *wxWindow_GetToolTipText(const void * self) {
    return new wxString(((wxWindow *)self)->GetToolTipText());
}
void wxWindow_SetToolTip(void * self, void * tip_string) {
    return ((wxWindow *)self)->SetToolTip(*(wxString *)tip_string);
}
void wxWindow_SetToolTip1(void * self, void * tip) {
    return ((wxWindow *)self)->SetToolTip((wxToolTip *)tip);
}
void wxWindow_UnsetToolTip(void * self) {
    return ((wxWindow *)self)->UnsetToolTip();
}
int wxWindow_GetPopupMenuSelectionFromUser(void * self, void * menu, void * pos) {
    return ((wxWindow *)self)->GetPopupMenuSelectionFromUser(*(wxMenu *)menu, *(wxPoint *)pos);
}
int wxWindow_GetPopupMenuSelectionFromUser1(void * self, void * menu, int x, int y) {
    return ((wxWindow *)self)->GetPopupMenuSelectionFromUser(*(wxMenu *)menu, x, y);
}
bool wxWindow_PopupMenu(void * self, void * menu, void * pos) {
    return ((wxWindow *)self)->PopupMenu((wxMenu *)menu, *(wxPoint *)pos);
}
bool wxWindow_PopupMenu1(void * self, void * menu, int x, int y) {
    return ((wxWindow *)self)->PopupMenu((wxMenu *)menu, x, y);
}
void * wxWindow_GetValidator(void * self) {
    return ((wxWindow *)self)->GetValidator();
}
void wxWindow_SetValidator(void * self, void * validator) {
    return ((wxWindow *)self)->SetValidator(*(wxValidator *)validator);
}
bool wxWindow_TransferDataFromWindow(void * self) {
    return ((wxWindow *)self)->TransferDataFromWindow();
}
bool wxWindow_TransferDataToWindow(void * self) {
    return ((wxWindow *)self)->TransferDataToWindow();
}
bool wxWindow_Validate(void * self) {
    return ((wxWindow *)self)->Validate();
}
wxWindowID wxWindow_GetId(const void * self) {
    return ((wxWindow *)self)->GetId();
}
void *wxWindow_GetLabel(const void * self) {
    return new wxString(((wxWindow *)self)->GetLabel());
}
wxCoord wxWindow_AdjustForLayoutDirection(const void * self, wxCoord x, wxCoord width, wxCoord width_total) {
    return ((wxWindow *)self)->AdjustForLayoutDirection(x, width, width_total);
}
void *wxWindow_GetName(const void * self) {
    return new wxString(((wxWindow *)self)->GetName());
}
void wxWindow_SetId(void * self, wxWindowID winid) {
    return ((wxWindow *)self)->SetId(winid);
}
void wxWindow_SetLabel(void * self, void * label) {
    return ((wxWindow *)self)->SetLabel(*(wxString *)label);
}
void wxWindow_SetName(void * self, void * name) {
    return ((wxWindow *)self)->SetName(*(wxString *)name);
}
void * wxWindow_GetAcceleratorTable(void * self) {
    return ((wxWindow *)self)->GetAcceleratorTable();
}
void wxWindow_SetAcceleratorTable(void * self, void * accel) {
    return ((wxWindow *)self)->SetAcceleratorTable(*(wxAcceleratorTable *)accel);
}
bool wxWindow_Close(void * self, bool force) {
    return ((wxWindow *)self)->Close(force);
}
bool wxWindow_Destroy(void * self) {
    return ((wxWindow *)self)->Destroy();
}
bool wxWindow_IsBeingDeleted(const void * self) {
    return ((wxWindow *)self)->IsBeingDeleted();
}
void * wxWindow_GetDropTarget(const void * self) {
    return ((wxWindow *)self)->GetDropTarget();
}
void wxWindow_SetDropTarget(void * self, void * target) {
    return ((wxWindow *)self)->SetDropTarget((wxDropTarget *)target);
}
void wxWindow_DragAcceptFiles(void * self, bool accept) {
    return ((wxWindow *)self)->DragAcceptFiles(accept);
}
void * wxWindow_GetContainingSizer(const void * self) {
    return ((wxWindow *)self)->GetContainingSizer();
}
void * wxWindow_GetSizer(const void * self) {
    return ((wxWindow *)self)->GetSizer();
}
void wxWindow_SetSizer(void * self, void * sizer, bool delete_old) {
    return ((wxWindow *)self)->SetSizer((wxSizer *)sizer, delete_old);
}
void wxWindow_SetSizerAndFit(void * self, void * sizer, bool delete_old) {
    return ((wxWindow *)self)->SetSizerAndFit((wxSizer *)sizer, delete_old);
}
void * wxWindow_GetConstraints(const void * self) {
    return ((wxWindow *)self)->GetConstraints();
}
void wxWindow_SetConstraints(void * self, void * constraints) {
    return ((wxWindow *)self)->SetConstraints((wxLayoutConstraints *)constraints);
}
bool wxWindow_Layout(void * self) {
    return ((wxWindow *)self)->Layout();
}
void wxWindow_SetAutoLayout(void * self, bool auto_layout) {
    return ((wxWindow *)self)->SetAutoLayout(auto_layout);
}
bool wxWindow_GetAutoLayout(const void * self) {
    return ((wxWindow *)self)->GetAutoLayout();
}
void wxWindow_CaptureMouse(void * self) {
    return ((wxWindow *)self)->CaptureMouse();
}
void * wxWindow_GetCaret(const void * self) {
    return ((wxWindow *)self)->GetCaret();
}
bool wxWindow_HasCapture(const void * self) {
    return ((wxWindow *)self)->HasCapture();
}
void wxWindow_ReleaseMouse(void * self) {
    return ((wxWindow *)self)->ReleaseMouse();
}
void wxWindow_SetCaret(void * self, void * caret) {
    return ((wxWindow *)self)->SetCaret((wxCaret *)caret);
}
bool wxWindow_SetCursor(void * self, void * cursor) {
    return ((wxWindow *)self)->SetCursor(*(wxCursor *)cursor);
}
void wxWindow_WarpPointer(void * self, int x, int y) {
    return ((wxWindow *)self)->WarpPointer(x, y);
}
bool wxWindow_EnableTouchEvents(void * self, int events_mask) {
    return ((wxWindow *)self)->EnableTouchEvents(events_mask);
}
void wxWindow_DoUpdateWindowUI(void * self, void * event) {
    return ((wxWindow *)self)->DoUpdateWindowUI(*(wxUpdateUIEvent *)event);
}
bool wxWindow_HasMultiplePages(const void * self) {
    return ((wxWindow *)self)->HasMultiplePages();
}
void wxWindow_InheritAttributes(void * self) {
    return ((wxWindow *)self)->InheritAttributes();
}
void wxWindow_InitDialog(void * self) {
    return ((wxWindow *)self)->InitDialog();
}
bool wxWindow_IsDoubleBuffered(const void * self) {
    return ((wxWindow *)self)->IsDoubleBuffered();
}
void wxWindow_SetDoubleBuffered(void * self, bool on) {
    return ((wxWindow *)self)->SetDoubleBuffered(on);
}
bool wxWindow_IsRetained(const void * self) {
    return ((wxWindow *)self)->IsRetained();
}
bool wxWindow_IsThisEnabled(const void * self) {
    return ((wxWindow *)self)->IsThisEnabled();
}
bool wxWindow_IsTopLevel(const void * self) {
    return ((wxWindow *)self)->IsTopLevel();
}
void wxWindow_OnInternalIdle(void * self) {
    return ((wxWindow *)self)->OnInternalIdle();
}
bool wxWindow_SendIdleEvents(void * self, void * event) {
    return ((wxWindow *)self)->SendIdleEvents(*(wxIdleEvent *)event);
}
bool wxWindow_RegisterHotKey(void * self, int hotkey_id, int modifiers, int virtual_key_code) {
    return ((wxWindow *)self)->RegisterHotKey(hotkey_id, modifiers, virtual_key_code);
}
bool wxWindow_UnregisterHotKey(void * self, int hotkey_id) {
    return ((wxWindow *)self)->UnregisterHotKey(hotkey_id);
}
void wxWindow_UpdateWindowUI(void * self, int32_t flags) {
    return ((wxWindow *)self)->UpdateWindowUI(flags);
}
void * wxWindow_FindFocus() {
    return wxWindow::FindFocus();
}
void * wxWindow_FindWindowById(int32_t id, void * parent) {
    return wxWindow::FindWindowById(id, (wxWindow *)parent);
}
void * wxWindow_FindWindowByLabel(void * label, void * parent) {
    return wxWindow::FindWindowByLabel(*(wxString *)label, (wxWindow *)parent);
}
void * wxWindow_FindWindowByName(void * name, void * parent) {
    return wxWindow::FindWindowByName(*(wxString *)name, (wxWindow *)parent);
}
void * wxWindow_GetCapture() {
    return wxWindow::GetCapture();
}
wxWindowID wxWindow_NewControlId(int count) {
    return wxWindow::NewControlId(count);
}
void wxWindow_UnreserveControlId(wxWindowID id, int count) {
    return wxWindow::UnreserveControlId(id, count);
}
void *wxWindow_new() {
    return new wxWindow();
}
void *wxWindow_new1(void * parent, wxWindowID id, void * pos, void * size, int32_t style, void * name) {
    return new wxWindow((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
bool wxWindow_Create(void * self, void * parent, wxWindowID id, void * pos, void * size, int32_t style, void * name) {
    return ((wxWindow *)self)->Create((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}

// CLASS: wxControl
void *wxControl_new(void * parent, wxWindowID id, void * pos, void * size, int32_t style, void * validator, void * name) {
    return new wxControl((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
void *wxControl_new1() {
    return new wxControl();
}
bool wxControl_Create(void * self, void * parent, wxWindowID id, void * pos, void * size, int32_t style, void * validator, void * name) {
    return ((wxControl *)self)->Create((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
void wxControl_Command(void * self, void * event) {
    return ((wxControl *)self)->Command(*(wxCommandEvent *)event);
}
void *wxControl_GetLabel(const void * self) {
    return new wxString(((wxControl *)self)->GetLabel());
}
void *wxControl_GetLabelText(const void * self) {
    return new wxString(((wxControl *)self)->GetLabelText());
}
void *wxControl_GetSizeFromTextSize(const void * self, int xlen, int ylen) {
    return new wxSize(((wxControl *)self)->GetSizeFromTextSize(xlen, ylen));
}
void *wxControl_GetSizeFromTextSize1(const void * self, void * tsize) {
    return new wxSize(((wxControl *)self)->GetSizeFromTextSize(*(wxSize *)tsize));
}
void *wxControl_GetSizeFromText(const void * self, void * text) {
    return new wxSize(((wxControl *)self)->GetSizeFromText(*(wxString *)text));
}
void wxControl_SetLabel(void * self, void * label) {
    return ((wxControl *)self)->SetLabel(*(wxString *)label);
}
void wxControl_SetLabelText(void * self, void * text) {
    return ((wxControl *)self)->SetLabelText(*(wxString *)text);
}
bool wxControl_SetLabelMarkup(void * self, void * markup) {
    return ((wxControl *)self)->SetLabelMarkup(*(wxString *)markup);
}
void *wxControl_GetLabelText1(void * label) {
    return new wxString(wxControl::GetLabelText(*(wxString *)label));
}
void *wxControl_RemoveMnemonics(void * str) {
    return new wxString(wxControl::RemoveMnemonics(*(wxString *)str));
}
void *wxControl_EscapeMnemonics(void * text) {
    return new wxString(wxControl::EscapeMnemonics(*(wxString *)text));
}

// CLASS: wxAnyButton
void *wxAnyButton_new() {
    return new wxAnyButton();
}
void wxAnyButton_SetBitmapCurrent(void * self, void * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapCurrent(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapDisabled(void * self, void * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapDisabled(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapFocus(void * self, void * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapFocus(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapLabel(void * self, void * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapLabel(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapPressed(void * self, void * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapPressed(*(wxBitmap *)bitmap);
}
void *wxAnyButton_GetBitmapMargins(void * self) {
    return new wxSize(((wxAnyButton *)self)->GetBitmapMargins());
}
void wxAnyButton_SetBitmapMargins(void * self, wxCoord x, wxCoord y) {
    return ((wxAnyButton *)self)->SetBitmapMargins(x, y);
}
void wxAnyButton_SetBitmapMargins1(void * self, void * sz) {
    return ((wxAnyButton *)self)->SetBitmapMargins(*(wxSize *)sz);
}

// CLASS: wxButton
void *wxButton_new() {
    return new wxButton();
}
void *wxButton_new1(void * parent, wxWindowID id, void * label, void * pos, void * size, int32_t style, void * validator, void * name) {
    return new wxButton((wxWindow *)parent, id, *(wxString *)label, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
bool wxButton_Create(void * self, void * parent, wxWindowID id, void * label, void * pos, void * size, int32_t style, void * validator, void * name) {
    return ((wxButton *)self)->Create((wxWindow *)parent, id, *(wxString *)label, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
bool wxButton_GetAuthNeeded(const void * self) {
    return ((wxButton *)self)->GetAuthNeeded();
}
void *wxButton_GetLabel(const void * self) {
    return new wxString(((wxButton *)self)->GetLabel());
}
void wxButton_SetAuthNeeded(void * self, bool needed) {
    return ((wxButton *)self)->SetAuthNeeded(needed);
}
void * wxButton_SetDefault(void * self) {
    return ((wxButton *)self)->SetDefault();
}
void wxButton_SetLabel(void * self, void * label) {
    return ((wxButton *)self)->SetLabel(*(wxString *)label);
}
void *wxButton_GetDefaultSize(void * win) {
    return new wxSize(wxButton::GetDefaultSize((wxWindow *)win));
}

// CLASS: wxNonOwnedWindow
bool wxNonOwnedWindow_SetShape(void * self, void * region) {
    return ((wxNonOwnedWindow *)self)->SetShape(*(wxRegion *)region);
}
bool wxNonOwnedWindow_SetShape1(void * self, void * path) {
    return ((wxNonOwnedWindow *)self)->SetShape(*(wxGraphicsPath *)path);
}

// CLASS: wxTopLevelWindow
void *wxTopLevelWindow_new() {
    return new wxTopLevelWindow();
}
void *wxTopLevelWindow_new1(void * parent, wxWindowID id, void * title, void * pos, void * size, int32_t style, void * name) {
    return new wxTopLevelWindow((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
bool wxTopLevelWindow_Create(void * self, void * parent, wxWindowID id, void * title, void * pos, void * size, int32_t style, void * name) {
    return ((wxTopLevelWindow *)self)->Create((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
bool wxTopLevelWindow_CanSetTransparent(void * self) {
    return ((wxTopLevelWindow *)self)->CanSetTransparent();
}
void wxTopLevelWindow_CenterOnScreen(void * self, int direction) {
    return ((wxTopLevelWindow *)self)->CenterOnScreen(direction);
}
void wxTopLevelWindow_CentreOnScreen(void * self, int direction) {
    return ((wxTopLevelWindow *)self)->CentreOnScreen(direction);
}
bool wxTopLevelWindow_EnableCloseButton(void * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableCloseButton(enable);
}
bool wxTopLevelWindow_EnableMaximizeButton(void * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableMaximizeButton(enable);
}
bool wxTopLevelWindow_EnableMinimizeButton(void * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableMinimizeButton(enable);
}
void * wxTopLevelWindow_GetDefaultItem(const void * self) {
    return ((wxTopLevelWindow *)self)->GetDefaultItem();
}
void *wxTopLevelWindow_GetTitle(const void * self) {
    return new wxString(((wxTopLevelWindow *)self)->GetTitle());
}
void wxTopLevelWindow_Iconize(void * self, bool iconize) {
    return ((wxTopLevelWindow *)self)->Iconize(iconize);
}
bool wxTopLevelWindow_IsActive(void * self) {
    return ((wxTopLevelWindow *)self)->IsActive();
}
bool wxTopLevelWindow_IsAlwaysMaximized(const void * self) {
    return ((wxTopLevelWindow *)self)->IsAlwaysMaximized();
}
bool wxTopLevelWindow_IsFullScreen(const void * self) {
    return ((wxTopLevelWindow *)self)->IsFullScreen();
}
bool wxTopLevelWindow_IsIconized(const void * self) {
    return ((wxTopLevelWindow *)self)->IsIconized();
}
bool wxTopLevelWindow_IsMaximized(const void * self) {
    return ((wxTopLevelWindow *)self)->IsMaximized();
}
bool wxTopLevelWindow_Layout(void * self) {
    return ((wxTopLevelWindow *)self)->Layout();
}
void wxTopLevelWindow_Maximize(void * self, bool maximize) {
    return ((wxTopLevelWindow *)self)->Maximize(maximize);
}
void wxTopLevelWindow_RequestUserAttention(void * self, int flags) {
    return ((wxTopLevelWindow *)self)->RequestUserAttention(flags);
}
void wxTopLevelWindow_Restore(void * self) {
    return ((wxTopLevelWindow *)self)->Restore();
}
void * wxTopLevelWindow_SetDefaultItem(void * self, void * win) {
    return ((wxTopLevelWindow *)self)->SetDefaultItem((wxWindow *)win);
}
void * wxTopLevelWindow_SetTmpDefaultItem(void * self, void * win) {
    return ((wxTopLevelWindow *)self)->SetTmpDefaultItem((wxWindow *)win);
}
void * wxTopLevelWindow_GetTmpDefaultItem(const void * self) {
    return ((wxTopLevelWindow *)self)->GetTmpDefaultItem();
}
void wxTopLevelWindow_SetIcon(void * self, void * icon) {
    return ((wxTopLevelWindow *)self)->SetIcon(*(wxIcon *)icon);
}
void wxTopLevelWindow_SetIcons(void * self, void * icons) {
    return ((wxTopLevelWindow *)self)->SetIcons(*(wxIconBundle *)icons);
}
void wxTopLevelWindow_SetMaxSize(void * self, void * size) {
    return ((wxTopLevelWindow *)self)->SetMaxSize(*(wxSize *)size);
}
void wxTopLevelWindow_SetMinSize(void * self, void * size) {
    return ((wxTopLevelWindow *)self)->SetMinSize(*(wxSize *)size);
}
void wxTopLevelWindow_SetSizeHints(void * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return ((wxTopLevelWindow *)self)->SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
void wxTopLevelWindow_SetSizeHints1(void * self, void * min_size, void * max_size, void * inc_size) {
    return ((wxTopLevelWindow *)self)->SetSizeHints(*(wxSize *)min_size, *(wxSize *)max_size, *(wxSize *)inc_size);
}
void wxTopLevelWindow_SetTitle(void * self, void * title) {
    return ((wxTopLevelWindow *)self)->SetTitle(*(wxString *)title);
}
bool wxTopLevelWindow_SetTransparent(void * self, wxByte alpha) {
    return ((wxTopLevelWindow *)self)->SetTransparent(alpha);
}
bool wxTopLevelWindow_ShouldPreventAppExit(const void * self) {
    return ((wxTopLevelWindow *)self)->ShouldPreventAppExit();
}
void wxTopLevelWindow_OSXSetModified(void * self, bool modified) {
    return ((wxTopLevelWindow *)self)->OSXSetModified(modified);
}
bool wxTopLevelWindow_OSXIsModified(const void * self) {
    return ((wxTopLevelWindow *)self)->OSXIsModified();
}
void wxTopLevelWindow_SetRepresentedFilename(void * self, void * filename) {
    return ((wxTopLevelWindow *)self)->SetRepresentedFilename(*(wxString *)filename);
}
void wxTopLevelWindow_ShowWithoutActivating(void * self) {
    return ((wxTopLevelWindow *)self)->ShowWithoutActivating();
}
bool wxTopLevelWindow_EnableFullScreenView(void * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableFullScreenView(enable);
}
bool wxTopLevelWindow_ShowFullScreen(void * self, bool show, int32_t style) {
    return ((wxTopLevelWindow *)self)->ShowFullScreen(show, style);
}
void *wxTopLevelWindow_GetDefaultSize() {
    return new wxSize(wxTopLevelWindow::GetDefaultSize());
}

// CLASS: wxFrame
void *wxFrame_new() {
    return new wxFrame();
}
void *wxFrame_new1(void * parent, wxWindowID id, void * title, void * pos, void * size, int32_t style, void * name) {
    return new wxFrame((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
void wxFrame_Centre(void * self, int direction) {
    return ((wxFrame *)self)->Centre(direction);
}
bool wxFrame_Create(void * self, void * parent, wxWindowID id, void * title, void * pos, void * size, int32_t style, void * name) {
    return ((wxFrame *)self)->Create((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
void * wxFrame_CreateStatusBar(void * self, int number, int32_t style, wxWindowID id, void * name) {
    return ((wxFrame *)self)->CreateStatusBar(number, style, id, *(wxString *)name);
}
void * wxFrame_CreateToolBar(void * self, int32_t style, wxWindowID id, void * name) {
    return ((wxFrame *)self)->CreateToolBar(style, id, *(wxString *)name);
}
void wxFrame_DoGiveHelp(void * self, void * text, bool show) {
    return ((wxFrame *)self)->DoGiveHelp(*(wxString *)text, show);
}
void *wxFrame_GetClientAreaOrigin(const void * self) {
    return new wxPoint(((wxFrame *)self)->GetClientAreaOrigin());
}
void * wxFrame_GetMenuBar(const void * self) {
    return ((wxFrame *)self)->GetMenuBar();
}
void * wxFrame_GetStatusBar(const void * self) {
    return ((wxFrame *)self)->GetStatusBar();
}
int wxFrame_GetStatusBarPane(const void * self) {
    return ((wxFrame *)self)->GetStatusBarPane();
}
void * wxFrame_GetToolBar(const void * self) {
    return ((wxFrame *)self)->GetToolBar();
}
void * wxFrame_OnCreateStatusBar(void * self, int number, int32_t style, wxWindowID id, void * name) {
    return ((wxFrame *)self)->OnCreateStatusBar(number, style, id, *(wxString *)name);
}
void * wxFrame_OnCreateToolBar(void * self, int32_t style, wxWindowID id, void * name) {
    return ((wxFrame *)self)->OnCreateToolBar(style, id, *(wxString *)name);
}
bool wxFrame_ProcessCommand(void * self, int id) {
    return ((wxFrame *)self)->ProcessCommand(id);
}
void wxFrame_SetMenuBar(void * self, void * menu_bar) {
    return ((wxFrame *)self)->SetMenuBar((wxMenuBar *)menu_bar);
}
void wxFrame_SetStatusBar(void * self, void * status_bar) {
    return ((wxFrame *)self)->SetStatusBar((wxStatusBar *)status_bar);
}
void wxFrame_SetStatusBarPane(void * self, int n) {
    return ((wxFrame *)self)->SetStatusBarPane(n);
}
void wxFrame_SetStatusText(void * self, void * text, int number) {
    return ((wxFrame *)self)->SetStatusText(*(wxString *)text, number);
}
void wxFrame_SetStatusWidths(void * self, int n, void * widths_field) {
    return ((wxFrame *)self)->SetStatusWidths(n, (int *)widths_field);
}
void wxFrame_SetToolBar(void * self, void * tool_bar) {
    return ((wxFrame *)self)->SetToolBar((wxToolBar *)tool_bar);
}
void wxFrame_PushStatusText(void * self, void * text, int number) {
    return ((wxFrame *)self)->PushStatusText(*(wxString *)text, number);
}
void wxFrame_PopStatusText(void * self, int number) {
    return ((wxFrame *)self)->PopStatusText(number);
}

// CLASS: wxPoint
bool wxPoint_IsFullySpecified(const void * self) {
    return ((wxPoint *)self)->IsFullySpecified();
}
void wxPoint_SetDefaults(void * self, void * pt) {
    return ((wxPoint *)self)->SetDefaults(*(wxPoint *)pt);
}
void *wxPoint_new() {
    return new wxPoint();
}
void *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
void *wxPoint_new2(void * pt) {
    return new wxPoint(*(wxRealPoint *)pt);
}

// CLASS: wxRect
void *wxRect_new() {
    return new wxRect();
}
void *wxRect_new1(int x, int y, int width, int height) {
    return new wxRect(x, y, width, height);
}
void *wxRect_new2(void * top_left, void * bottom_right) {
    return new wxRect(*(wxPoint *)top_left, *(wxPoint *)bottom_right);
}
void *wxRect_new3(void * pos, void * size) {
    return new wxRect(*(wxPoint *)pos, *(wxSize *)size);
}
void *wxRect_new4(void * size) {
    return new wxRect(*(wxSize *)size);
}
void *wxRect_CentreIn(const void * self, void * r, int dir) {
    return new wxRect(((wxRect *)self)->CentreIn(*(wxRect *)r, dir));
}
void *wxRect_CenterIn(const void * self, void * r, int dir) {
    return new wxRect(((wxRect *)self)->CenterIn(*(wxRect *)r, dir));
}
bool wxRect_Contains(const void * self, int x, int y) {
    return ((wxRect *)self)->Contains(x, y);
}
bool wxRect_Contains1(const void * self, void * pt) {
    return ((wxRect *)self)->Contains(*(wxPoint *)pt);
}
bool wxRect_Contains2(const void * self, void * rect) {
    return ((wxRect *)self)->Contains(*(wxRect *)rect);
}
void *wxRect_Deflate3(const void * self, wxCoord dx, wxCoord dy) {
    return new wxRect(((wxRect *)self)->Deflate(dx, dy));
}
int wxRect_GetBottom(const void * self) {
    return ((wxRect *)self)->GetBottom();
}
void *wxRect_GetBottomLeft(const void * self) {
    return new wxPoint(((wxRect *)self)->GetBottomLeft());
}
void *wxRect_GetBottomRight(const void * self) {
    return new wxPoint(((wxRect *)self)->GetBottomRight());
}
int wxRect_GetHeight(const void * self) {
    return ((wxRect *)self)->GetHeight();
}
int wxRect_GetLeft(const void * self) {
    return ((wxRect *)self)->GetLeft();
}
void *wxRect_GetPosition(const void * self) {
    return new wxPoint(((wxRect *)self)->GetPosition());
}
int wxRect_GetRight(const void * self) {
    return ((wxRect *)self)->GetRight();
}
void *wxRect_GetSize(const void * self) {
    return new wxSize(((wxRect *)self)->GetSize());
}
int wxRect_GetTop(const void * self) {
    return ((wxRect *)self)->GetTop();
}
void *wxRect_GetTopLeft(const void * self) {
    return new wxPoint(((wxRect *)self)->GetTopLeft());
}
void *wxRect_GetTopRight(const void * self) {
    return new wxPoint(((wxRect *)self)->GetTopRight());
}
int wxRect_GetWidth(const void * self) {
    return ((wxRect *)self)->GetWidth();
}
int wxRect_GetX(const void * self) {
    return ((wxRect *)self)->GetX();
}
int wxRect_GetY(const void * self) {
    return ((wxRect *)self)->GetY();
}
void *wxRect_Inflate3(const void * self, wxCoord dx, wxCoord dy) {
    return new wxRect(((wxRect *)self)->Inflate(dx, dy));
}
void *wxRect_Intersect1(const void * self, void * rect) {
    return new wxRect(((wxRect *)self)->Intersect(*(wxRect *)rect));
}
bool wxRect_Intersects(const void * self, void * rect) {
    return ((wxRect *)self)->Intersects(*(wxRect *)rect);
}
bool wxRect_IsEmpty(const void * self) {
    return ((wxRect *)self)->IsEmpty();
}
void wxRect_Offset(void * self, wxCoord dx, wxCoord dy) {
    return ((wxRect *)self)->Offset(dx, dy);
}
void wxRect_Offset1(void * self, void * pt) {
    return ((wxRect *)self)->Offset(*(wxPoint *)pt);
}
void wxRect_SetHeight(void * self, int height) {
    return ((wxRect *)self)->SetHeight(height);
}
void wxRect_SetPosition(void * self, void * pos) {
    return ((wxRect *)self)->SetPosition(*(wxPoint *)pos);
}
void wxRect_SetSize(void * self, void * s) {
    return ((wxRect *)self)->SetSize(*(wxSize *)s);
}
void wxRect_SetWidth(void * self, int width) {
    return ((wxRect *)self)->SetWidth(width);
}
void wxRect_SetX(void * self, int x) {
    return ((wxRect *)self)->SetX(x);
}
void wxRect_SetY(void * self, int y) {
    return ((wxRect *)self)->SetY(y);
}
void wxRect_SetLeft(void * self, int left) {
    return ((wxRect *)self)->SetLeft(left);
}
void wxRect_SetRight(void * self, int right) {
    return ((wxRect *)self)->SetRight(right);
}
void wxRect_SetTop(void * self, int top) {
    return ((wxRect *)self)->SetTop(top);
}
void wxRect_SetBottom(void * self, int bottom) {
    return ((wxRect *)self)->SetBottom(bottom);
}
void wxRect_SetTopLeft(void * self, void * p) {
    return ((wxRect *)self)->SetTopLeft(*(wxPoint *)p);
}
void wxRect_SetBottomRight(void * self, void * p) {
    return ((wxRect *)self)->SetBottomRight(*(wxPoint *)p);
}
void wxRect_SetTopRight(void * self, void * p) {
    return ((wxRect *)self)->SetTopRight(*(wxPoint *)p);
}
void wxRect_SetBottomLeft(void * self, void * p) {
    return ((wxRect *)self)->SetBottomLeft(*(wxPoint *)p);
}
void *wxRect_Union(const void * self, void * rect) {
    return new wxRect(((wxRect *)self)->Union(*(wxRect *)rect));
}

// CLASS: wxSize
void *wxSize_new() {
    return new wxSize();
}
void *wxSize_new1(int width, int height) {
    return new wxSize(width, height);
}
void wxSize_DecBy(void * self, void * pt) {
    return ((wxSize *)self)->DecBy(*(wxPoint *)pt);
}
void wxSize_DecBy1(void * self, void * size) {
    return ((wxSize *)self)->DecBy(*(wxSize *)size);
}
void wxSize_DecBy2(void * self, int dx, int dy) {
    return ((wxSize *)self)->DecBy(dx, dy);
}
void wxSize_DecBy3(void * self, int d) {
    return ((wxSize *)self)->DecBy(d);
}
void wxSize_DecTo(void * self, void * size) {
    return ((wxSize *)self)->DecTo(*(wxSize *)size);
}
void wxSize_DecToIfSpecified(void * self, void * size) {
    return ((wxSize *)self)->DecToIfSpecified(*(wxSize *)size);
}
int wxSize_GetHeight(const void * self) {
    return ((wxSize *)self)->GetHeight();
}
int wxSize_GetWidth(const void * self) {
    return ((wxSize *)self)->GetWidth();
}
void wxSize_IncBy(void * self, void * pt) {
    return ((wxSize *)self)->IncBy(*(wxPoint *)pt);
}
void wxSize_IncBy1(void * self, void * size) {
    return ((wxSize *)self)->IncBy(*(wxSize *)size);
}
void wxSize_IncBy2(void * self, int dx, int dy) {
    return ((wxSize *)self)->IncBy(dx, dy);
}
void wxSize_IncBy3(void * self, int d) {
    return ((wxSize *)self)->IncBy(d);
}
void wxSize_IncTo(void * self, void * size) {
    return ((wxSize *)self)->IncTo(*(wxSize *)size);
}
bool wxSize_IsFullySpecified(const void * self) {
    return ((wxSize *)self)->IsFullySpecified();
}
void wxSize_Set(void * self, int width, int height) {
    return ((wxSize *)self)->Set(width, height);
}
void wxSize_SetDefaults(void * self, void * size_default) {
    return ((wxSize *)self)->SetDefaults(*(wxSize *)size_default);
}
void wxSize_SetHeight(void * self, int height) {
    return ((wxSize *)self)->SetHeight(height);
}
void wxSize_SetWidth(void * self, int width) {
    return ((wxSize *)self)->SetWidth(width);
}

// CLASS: wxValidator
void *wxValidator_new() {
    return new wxValidator();
}
void * wxValidator_Clone(const void * self) {
    return ((wxValidator *)self)->Clone();
}
void * wxValidator_GetWindow(const void * self) {
    return ((wxValidator *)self)->GetWindow();
}
void wxValidator_SetWindow(void * self, void * window) {
    return ((wxValidator *)self)->SetWindow((wxWindow *)window);
}
bool wxValidator_TransferFromWindow(void * self) {
    return ((wxValidator *)self)->TransferFromWindow();
}
bool wxValidator_TransferToWindow(void * self) {
    return ((wxValidator *)self)->TransferToWindow();
}
bool wxValidator_Validate(void * self, void * parent) {
    return ((wxValidator *)self)->Validate((wxWindow *)parent);
}
void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}

} // extern "C"

