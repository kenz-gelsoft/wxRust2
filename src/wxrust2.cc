#include <wx/wx.h>

extern "C" {

// CLASS: wxObject
wxObject *wxObject_new() {
    return new wxObject();
}
wxObject *wxObject_new1(const wxObject * other) {
    return new wxObject(*(wxObject *)other);
}
wxClassInfo * wxObject_GetClassInfo(const wxObject * self) {
    return ((wxObject *)self)->GetClassInfo();
}
wxObjectRefData * wxObject_GetRefData(const wxObject * self) {
    return ((wxObject *)self)->GetRefData();
}
bool wxObject_IsKindOf(const wxObject * self, const wxClassInfo * info) {
    return ((wxObject *)self)->IsKindOf((wxClassInfo *)info);
}
bool wxObject_IsSameAs(const wxObject * self, const wxObject * obj) {
    return ((wxObject *)self)->IsSameAs(*(wxObject *)obj);
}
void wxObject_Ref(wxObject * self, const wxObject * clone) {
    return ((wxObject *)self)->Ref(*(wxObject *)clone);
}
void wxObject_SetRefData(wxObject * self, wxObjectRefData * data) {
    return ((wxObject *)self)->SetRefData((wxObjectRefData *)data);
}
void wxObject_UnRef(wxObject * self) {
    return ((wxObject *)self)->UnRef();
}
void wxObject_UnShare(wxObject * self) {
    return ((wxObject *)self)->UnShare();
}

// CLASS: wxEvtHandler
void wxEvtHandler_QueueEvent(wxEvtHandler * self, wxEvent * event) {
    return ((wxEvtHandler *)self)->QueueEvent((wxEvent *)event);
}
void wxEvtHandler_AddPendingEvent(wxEvtHandler * self, const wxEvent * event) {
    return ((wxEvtHandler *)self)->AddPendingEvent(*(wxEvent *)event);
}
bool wxEvtHandler_ProcessEvent(wxEvtHandler * self, wxEvent * event) {
    return ((wxEvtHandler *)self)->ProcessEvent(*(wxEvent *)event);
}
bool wxEvtHandler_ProcessEventLocally(wxEvtHandler * self, wxEvent * event) {
    return ((wxEvtHandler *)self)->ProcessEventLocally(*(wxEvent *)event);
}
bool wxEvtHandler_SafelyProcessEvent(wxEvtHandler * self, wxEvent * event) {
    return ((wxEvtHandler *)self)->SafelyProcessEvent(*(wxEvent *)event);
}
void wxEvtHandler_ProcessPendingEvents(wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->ProcessPendingEvents();
}
void wxEvtHandler_DeletePendingEvents(wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->DeletePendingEvents();
}
wxClientData * wxEvtHandler_GetClientObject(const wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->GetClientObject();
}
void wxEvtHandler_SetClientObject(wxEvtHandler * self, wxClientData * data) {
    return ((wxEvtHandler *)self)->SetClientObject((wxClientData *)data);
}
bool wxEvtHandler_GetEvtHandlerEnabled(const wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->GetEvtHandlerEnabled();
}
wxEvtHandler * wxEvtHandler_GetNextHandler(const wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->GetNextHandler();
}
wxEvtHandler * wxEvtHandler_GetPreviousHandler(const wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->GetPreviousHandler();
}
void wxEvtHandler_SetEvtHandlerEnabled(wxEvtHandler * self, bool enabled) {
    return ((wxEvtHandler *)self)->SetEvtHandlerEnabled(enabled);
}
void wxEvtHandler_SetNextHandler(wxEvtHandler * self, wxEvtHandler * handler) {
    return ((wxEvtHandler *)self)->SetNextHandler((wxEvtHandler *)handler);
}
void wxEvtHandler_SetPreviousHandler(wxEvtHandler * self, wxEvtHandler * handler) {
    return ((wxEvtHandler *)self)->SetPreviousHandler((wxEvtHandler *)handler);
}
void wxEvtHandler_Unlink(wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->Unlink();
}
bool wxEvtHandler_IsUnlinked(const wxEvtHandler * self) {
    return ((wxEvtHandler *)self)->IsUnlinked();
}
void wxEvtHandler_AddFilter(wxEventFilter * filter) {
    return wxEvtHandler::AddFilter((wxEventFilter *)filter);
}
void wxEvtHandler_RemoveFilter(wxEventFilter * filter) {
    return wxEvtHandler::RemoveFilter((wxEventFilter *)filter);
}
wxEvtHandler *wxEvtHandler_new() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
bool wxWindow_AcceptsFocus(const wxWindow * self) {
    return ((wxWindow *)self)->AcceptsFocus();
}
bool wxWindow_AcceptsFocusFromKeyboard(const wxWindow * self) {
    return ((wxWindow *)self)->AcceptsFocusFromKeyboard();
}
bool wxWindow_AcceptsFocusRecursively(const wxWindow * self) {
    return ((wxWindow *)self)->AcceptsFocusRecursively();
}
void wxWindow_DisableFocusFromKeyboard(wxWindow * self) {
    return ((wxWindow *)self)->DisableFocusFromKeyboard();
}
bool wxWindow_IsFocusable(const wxWindow * self) {
    return ((wxWindow *)self)->IsFocusable();
}
bool wxWindow_CanAcceptFocus(const wxWindow * self) {
    return ((wxWindow *)self)->CanAcceptFocus();
}
bool wxWindow_CanAcceptFocusFromKeyboard(const wxWindow * self) {
    return ((wxWindow *)self)->CanAcceptFocusFromKeyboard();
}
bool wxWindow_HasFocus(const wxWindow * self) {
    return ((wxWindow *)self)->HasFocus();
}
void wxWindow_SetCanFocus(wxWindow * self, bool can_focus) {
    return ((wxWindow *)self)->SetCanFocus(can_focus);
}
void wxWindow_EnableVisibleFocus(wxWindow * self, bool enable) {
    return ((wxWindow *)self)->EnableVisibleFocus(enable);
}
void wxWindow_SetFocus(wxWindow * self) {
    return ((wxWindow *)self)->SetFocus();
}
void wxWindow_SetFocusFromKbd(wxWindow * self) {
    return ((wxWindow *)self)->SetFocusFromKbd();
}
void wxWindow_AddChild(wxWindow * self, wxWindow * child) {
    return ((wxWindow *)self)->AddChild((wxWindow *)child);
}
bool wxWindow_DestroyChildren(wxWindow * self) {
    return ((wxWindow *)self)->DestroyChildren();
}
wxWindow * wxWindow_FindWindow(const wxWindow * self, int32_t id) {
    return ((wxWindow *)self)->FindWindow(id);
}
wxWindow * wxWindow_FindWindow1(const wxWindow * self, const wxString * name) {
    return ((wxWindow *)self)->FindWindow(*(wxString *)name);
}
void wxWindow_RemoveChild(wxWindow * self, wxWindow * child) {
    return ((wxWindow *)self)->RemoveChild((wxWindow *)child);
}
wxWindow * wxWindow_GetGrandParent(const wxWindow * self) {
    return ((wxWindow *)self)->GetGrandParent();
}
wxWindow * wxWindow_GetNextSibling(const wxWindow * self) {
    return ((wxWindow *)self)->GetNextSibling();
}
wxWindow * wxWindow_GetParent(const wxWindow * self) {
    return ((wxWindow *)self)->GetParent();
}
wxWindow * wxWindow_GetPrevSibling(const wxWindow * self) {
    return ((wxWindow *)self)->GetPrevSibling();
}
bool wxWindow_IsDescendant(const wxWindow * self, wxWindow * win) {
    return ((wxWindow *)self)->IsDescendant((wxWindow *)win);
}
bool wxWindow_Reparent(wxWindow * self, wxWindow * new_parent) {
    return ((wxWindow *)self)->Reparent((wxWindow *)new_parent);
}
void wxWindow_AlwaysShowScrollbars(wxWindow * self, bool hflag, bool vflag) {
    return ((wxWindow *)self)->AlwaysShowScrollbars(hflag, vflag);
}
int wxWindow_GetScrollPos(const wxWindow * self, int orientation) {
    return ((wxWindow *)self)->GetScrollPos(orientation);
}
int wxWindow_GetScrollRange(const wxWindow * self, int orientation) {
    return ((wxWindow *)self)->GetScrollRange(orientation);
}
int wxWindow_GetScrollThumb(const wxWindow * self, int orientation) {
    return ((wxWindow *)self)->GetScrollThumb(orientation);
}
bool wxWindow_CanScroll(const wxWindow * self, int orient) {
    return ((wxWindow *)self)->CanScroll(orient);
}
bool wxWindow_HasScrollbar(const wxWindow * self, int orient) {
    return ((wxWindow *)self)->HasScrollbar(orient);
}
bool wxWindow_IsScrollbarAlwaysShown(const wxWindow * self, int orient) {
    return ((wxWindow *)self)->IsScrollbarAlwaysShown(orient);
}
bool wxWindow_ScrollLines(wxWindow * self, int lines) {
    return ((wxWindow *)self)->ScrollLines(lines);
}
bool wxWindow_ScrollPages(wxWindow * self, int pages) {
    return ((wxWindow *)self)->ScrollPages(pages);
}
void wxWindow_ScrollWindow(wxWindow * self, int dx, int dy, const wxRect * rect) {
    return ((wxWindow *)self)->ScrollWindow(dx, dy, (wxRect *)rect);
}
bool wxWindow_LineUp(wxWindow * self) {
    return ((wxWindow *)self)->LineUp();
}
bool wxWindow_LineDown(wxWindow * self) {
    return ((wxWindow *)self)->LineDown();
}
bool wxWindow_PageUp(wxWindow * self) {
    return ((wxWindow *)self)->PageUp();
}
bool wxWindow_PageDown(wxWindow * self) {
    return ((wxWindow *)self)->PageDown();
}
void wxWindow_SetScrollPos(wxWindow * self, int orientation, int pos, bool refresh) {
    return ((wxWindow *)self)->SetScrollPos(orientation, pos, refresh);
}
void wxWindow_SetScrollbar(wxWindow * self, int orientation, int position, int thumb_size, int range, bool refresh) {
    return ((wxWindow *)self)->SetScrollbar(orientation, position, thumb_size, range, refresh);
}
bool wxWindow_BeginRepositioningChildren(wxWindow * self) {
    return ((wxWindow *)self)->BeginRepositioningChildren();
}
void wxWindow_EndRepositioningChildren(wxWindow * self) {
    return ((wxWindow *)self)->EndRepositioningChildren();
}
void wxWindow_CacheBestSize(const wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->CacheBestSize(*(wxSize *)size);
}
wxSize *wxWindow_ClientToWindowSize(const wxWindow * self, const wxSize * size) {
    return new wxSize(((wxWindow *)self)->ClientToWindowSize(*(wxSize *)size));
}
wxSize *wxWindow_WindowToClientSize(const wxWindow * self, const wxSize * size) {
    return new wxSize(((wxWindow *)self)->WindowToClientSize(*(wxSize *)size));
}
void wxWindow_Fit(wxWindow * self) {
    return ((wxWindow *)self)->Fit();
}
void wxWindow_FitInside(wxWindow * self) {
    return ((wxWindow *)self)->FitInside();
}
wxSize *wxWindow_FromDIP(const wxWindow * self, const wxSize * sz) {
    return new wxSize(((wxWindow *)self)->FromDIP(*(wxSize *)sz));
}
wxPoint *wxWindow_FromDIP1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(((wxWindow *)self)->FromDIP(*(wxPoint *)pt));
}
int wxWindow_FromDIP2(const wxWindow * self, int d) {
    return ((wxWindow *)self)->FromDIP(d);
}
wxSize *wxWindow_ToDIP(const wxWindow * self, const wxSize * sz) {
    return new wxSize(((wxWindow *)self)->ToDIP(*(wxSize *)sz));
}
wxPoint *wxWindow_ToDIP1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(((wxWindow *)self)->ToDIP(*(wxPoint *)pt));
}
int wxWindow_ToDIP2(const wxWindow * self, int d) {
    return ((wxWindow *)self)->ToDIP(d);
}
wxSize *wxWindow_GetBestSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetBestSize());
}
int wxWindow_GetBestHeight(const wxWindow * self, int width) {
    return ((wxWindow *)self)->GetBestHeight(width);
}
int wxWindow_GetBestWidth(const wxWindow * self, int height) {
    return ((wxWindow *)self)->GetBestWidth(height);
}
void wxWindow_GetClientSize(const wxWindow * self, int * width, int * height) {
    return ((wxWindow *)self)->GetClientSize((int *)width, (int *)height);
}
wxSize *wxWindow_GetClientSize1(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetClientSize());
}
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetEffectiveMinSize());
}
wxSize *wxWindow_GetMaxClientSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetMaxClientSize());
}
wxSize *wxWindow_GetMaxSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetMaxSize());
}
wxSize *wxWindow_GetMinClientSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetMinClientSize());
}
wxSize *wxWindow_GetMinSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetMinSize());
}
int wxWindow_GetMinWidth(const wxWindow * self) {
    return ((wxWindow *)self)->GetMinWidth();
}
int wxWindow_GetMinHeight(const wxWindow * self) {
    return ((wxWindow *)self)->GetMinHeight();
}
int wxWindow_GetMaxWidth(const wxWindow * self) {
    return ((wxWindow *)self)->GetMaxWidth();
}
int wxWindow_GetMaxHeight(const wxWindow * self) {
    return ((wxWindow *)self)->GetMaxHeight();
}
void wxWindow_GetSize(const wxWindow * self, int * width, int * height) {
    return ((wxWindow *)self)->GetSize((int *)width, (int *)height);
}
wxSize *wxWindow_GetSize1(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetSize());
}
wxSize *wxWindow_GetVirtualSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetVirtualSize());
}
void wxWindow_GetVirtualSize1(const wxWindow * self, int * width, int * height) {
    return ((wxWindow *)self)->GetVirtualSize((int *)width, (int *)height);
}
wxSize *wxWindow_GetBestVirtualSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetBestVirtualSize());
}
double wxWindow_GetContentScaleFactor(const wxWindow * self) {
    return ((wxWindow *)self)->GetContentScaleFactor();
}
double wxWindow_GetDPIScaleFactor(const wxWindow * self) {
    return ((wxWindow *)self)->GetDPIScaleFactor();
}
wxSize *wxWindow_GetWindowBorderSize(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetWindowBorderSize());
}
bool wxWindow_InformFirstDirection(wxWindow * self, int direction, int size, int available_other_dir) {
    return ((wxWindow *)self)->InformFirstDirection(direction, size, available_other_dir);
}
void wxWindow_InvalidateBestSize(wxWindow * self) {
    return ((wxWindow *)self)->InvalidateBestSize();
}
void wxWindow_PostSizeEvent(wxWindow * self) {
    return ((wxWindow *)self)->PostSizeEvent();
}
void wxWindow_PostSizeEventToParent(wxWindow * self) {
    return ((wxWindow *)self)->PostSizeEventToParent();
}
void wxWindow_SendSizeEvent(wxWindow * self, int flags) {
    return ((wxWindow *)self)->SendSizeEvent(flags);
}
void wxWindow_SendSizeEventToParent(wxWindow * self, int flags) {
    return ((wxWindow *)self)->SendSizeEventToParent(flags);
}
void wxWindow_SetClientSize(wxWindow * self, int width, int height) {
    return ((wxWindow *)self)->SetClientSize(width, height);
}
void wxWindow_SetClientSize1(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetClientSize(*(wxSize *)size);
}
void wxWindow_SetClientSize2(wxWindow * self, const wxRect * rect) {
    return ((wxWindow *)self)->SetClientSize(*(wxRect *)rect);
}
void wxWindow_SetContainingSizer(wxWindow * self, wxSizer * sizer) {
    return ((wxWindow *)self)->SetContainingSizer((wxSizer *)sizer);
}
void wxWindow_SetInitialSize(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetInitialSize(*(wxSize *)size);
}
void wxWindow_SetMaxClientSize(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetMaxClientSize(*(wxSize *)size);
}
void wxWindow_SetMaxSize(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetMaxSize(*(wxSize *)size);
}
void wxWindow_SetMinClientSize(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetMinClientSize(*(wxSize *)size);
}
void wxWindow_SetMinSize(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetMinSize(*(wxSize *)size);
}
void wxWindow_SetSize(wxWindow * self, int x, int y, int width, int height, int size_flags) {
    return ((wxWindow *)self)->SetSize(x, y, width, height, size_flags);
}
void wxWindow_SetSize1(wxWindow * self, const wxRect * rect) {
    return ((wxWindow *)self)->SetSize(*(wxRect *)rect);
}
void wxWindow_SetSize2(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetSize(*(wxSize *)size);
}
void wxWindow_SetSize3(wxWindow * self, int width, int height) {
    return ((wxWindow *)self)->SetSize(width, height);
}
void wxWindow_SetSizeHints(wxWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size) {
    return ((wxWindow *)self)->SetSizeHints(*(wxSize *)min_size, *(wxSize *)max_size, *(wxSize *)inc_size);
}
void wxWindow_SetSizeHints1(wxWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return ((wxWindow *)self)->SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
void wxWindow_SetVirtualSize(wxWindow * self, int width, int height) {
    return ((wxWindow *)self)->SetVirtualSize(width, height);
}
void wxWindow_SetVirtualSize1(wxWindow * self, const wxSize * size) {
    return ((wxWindow *)self)->SetVirtualSize(*(wxSize *)size);
}
wxSize *wxWindow_FromDIP3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::FromDIP(*(wxSize *)sz, (wxWindow *)w));
}
wxPoint *wxWindow_FromDIP4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromDIP(*(wxPoint *)pt, (wxWindow *)w));
}
int wxWindow_FromDIP5(int d, const wxWindow * w) {
    return wxWindow::FromDIP(d, (wxWindow *)w);
}
wxSize *wxWindow_ToDIP3(const wxSize * sz, const wxWindow * w) {
    return new wxSize(wxWindow::ToDIP(*(wxSize *)sz, (wxWindow *)w));
}
wxPoint *wxWindow_ToDIP4(const wxPoint * pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToDIP(*(wxPoint *)pt, (wxWindow *)w));
}
int wxWindow_ToDIP5(int d, const wxWindow * w) {
    return wxWindow::ToDIP(d, (wxWindow *)w);
}
void wxWindow_Center(wxWindow * self, int dir) {
    return ((wxWindow *)self)->Center(dir);
}
void wxWindow_CenterOnParent(wxWindow * self, int dir) {
    return ((wxWindow *)self)->CenterOnParent(dir);
}
void wxWindow_Centre(wxWindow * self, int direction) {
    return ((wxWindow *)self)->Centre(direction);
}
void wxWindow_CentreOnParent(wxWindow * self, int direction) {
    return ((wxWindow *)self)->CentreOnParent(direction);
}
void wxWindow_GetPosition(const wxWindow * self, int * x, int * y) {
    return ((wxWindow *)self)->GetPosition((int *)x, (int *)y);
}
wxPoint *wxWindow_GetPosition1(const wxWindow * self) {
    return new wxPoint(((wxWindow *)self)->GetPosition());
}
wxRect *wxWindow_GetRect(const wxWindow * self) {
    return new wxRect(((wxWindow *)self)->GetRect());
}
void wxWindow_GetScreenPosition(const wxWindow * self, int * x, int * y) {
    return ((wxWindow *)self)->GetScreenPosition((int *)x, (int *)y);
}
wxPoint *wxWindow_GetScreenPosition1(const wxWindow * self) {
    return new wxPoint(((wxWindow *)self)->GetScreenPosition());
}
wxRect *wxWindow_GetScreenRect(const wxWindow * self) {
    return new wxRect(((wxWindow *)self)->GetScreenRect());
}
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow * self) {
    return new wxPoint(((wxWindow *)self)->GetClientAreaOrigin());
}
wxRect *wxWindow_GetClientRect(const wxWindow * self) {
    return new wxRect(((wxWindow *)self)->GetClientRect());
}
void wxWindow_Move(wxWindow * self, int x, int y, int flags) {
    return ((wxWindow *)self)->Move(x, y, flags);
}
void wxWindow_Move1(wxWindow * self, const wxPoint * pt, int flags) {
    return ((wxWindow *)self)->Move(*(wxPoint *)pt, flags);
}
void wxWindow_SetPosition(wxWindow * self, const wxPoint * pt) {
    return ((wxWindow *)self)->SetPosition(*(wxPoint *)pt);
}
void wxWindow_ClientToScreen(const wxWindow * self, int * x, int * y) {
    return ((wxWindow *)self)->ClientToScreen((int *)x, (int *)y);
}
wxPoint *wxWindow_ClientToScreen1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(((wxWindow *)self)->ClientToScreen(*(wxPoint *)pt));
}
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(((wxWindow *)self)->ConvertDialogToPixels(*(wxPoint *)pt));
}
wxSize *wxWindow_ConvertDialogToPixels1(const wxWindow * self, const wxSize * sz) {
    return new wxSize(((wxWindow *)self)->ConvertDialogToPixels(*(wxSize *)sz));
}
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(((wxWindow *)self)->ConvertPixelsToDialog(*(wxPoint *)pt));
}
wxSize *wxWindow_ConvertPixelsToDialog1(const wxWindow * self, const wxSize * sz) {
    return new wxSize(((wxWindow *)self)->ConvertPixelsToDialog(*(wxSize *)sz));
}
void wxWindow_ScreenToClient(const wxWindow * self, int * x, int * y) {
    return ((wxWindow *)self)->ScreenToClient((int *)x, (int *)y);
}
wxPoint *wxWindow_ScreenToClient1(const wxWindow * self, const wxPoint * pt) {
    return new wxPoint(((wxWindow *)self)->ScreenToClient(*(wxPoint *)pt));
}
void wxWindow_ClearBackground(wxWindow * self) {
    return ((wxWindow *)self)->ClearBackground();
}
void wxWindow_Freeze(wxWindow * self) {
    return ((wxWindow *)self)->Freeze();
}
void wxWindow_Thaw(wxWindow * self) {
    return ((wxWindow *)self)->Thaw();
}
bool wxWindow_IsFrozen(const wxWindow * self) {
    return ((wxWindow *)self)->IsFrozen();
}
int wxWindow_GetCharHeight(const wxWindow * self) {
    return ((wxWindow *)self)->GetCharHeight();
}
int wxWindow_GetCharWidth(const wxWindow * self) {
    return ((wxWindow *)self)->GetCharWidth();
}
wxSize *wxWindow_GetDPI(const wxWindow * self) {
    return new wxSize(((wxWindow *)self)->GetDPI());
}
void wxWindow_GetTextExtent(const wxWindow * self, const wxString * string, int * w, int * h, int * descent, int * external_leading, const wxFont * font) {
    return ((wxWindow *)self)->GetTextExtent(*(wxString *)string, (int *)w, (int *)h, (int *)descent, (int *)external_leading, (wxFont *)font);
}
wxSize *wxWindow_GetTextExtent1(const wxWindow * self, const wxString * string) {
    return new wxSize(((wxWindow *)self)->GetTextExtent(*(wxString *)string));
}
wxRect *wxWindow_GetUpdateClientRect(const wxWindow * self) {
    return new wxRect(((wxWindow *)self)->GetUpdateClientRect());
}
bool wxWindow_HasTransparentBackground(wxWindow * self) {
    return ((wxWindow *)self)->HasTransparentBackground();
}
void wxWindow_Refresh(wxWindow * self, bool erase_background, const wxRect * rect) {
    return ((wxWindow *)self)->Refresh(erase_background, (wxRect *)rect);
}
void wxWindow_RefreshRect(wxWindow * self, const wxRect * rect, bool erase_background) {
    return ((wxWindow *)self)->RefreshRect(*(wxRect *)rect, erase_background);
}
void wxWindow_Update(wxWindow * self) {
    return ((wxWindow *)self)->Update();
}
bool wxWindow_SetBackgroundColour(wxWindow * self, const wxColour * colour) {
    return ((wxWindow *)self)->SetBackgroundColour(*(wxColour *)colour);
}
bool wxWindow_IsTransparentBackgroundSupported(const wxWindow * self, wxString * reason) {
    return ((wxWindow *)self)->IsTransparentBackgroundSupported((wxString *)reason);
}
bool wxWindow_SetFont(wxWindow * self, const wxFont * font) {
    return ((wxWindow *)self)->SetFont(*(wxFont *)font);
}
bool wxWindow_SetForegroundColour(wxWindow * self, const wxColour * colour) {
    return ((wxWindow *)self)->SetForegroundColour(*(wxColour *)colour);
}
void wxWindow_SetOwnBackgroundColour(wxWindow * self, const wxColour * colour) {
    return ((wxWindow *)self)->SetOwnBackgroundColour(*(wxColour *)colour);
}
bool wxWindow_InheritsBackgroundColour(const wxWindow * self) {
    return ((wxWindow *)self)->InheritsBackgroundColour();
}
bool wxWindow_UseBgCol(const wxWindow * self) {
    return ((wxWindow *)self)->UseBgCol();
}
bool wxWindow_UseBackgroundColour(const wxWindow * self) {
    return ((wxWindow *)self)->UseBackgroundColour();
}
void wxWindow_SetOwnFont(wxWindow * self, const wxFont * font) {
    return ((wxWindow *)self)->SetOwnFont(*(wxFont *)font);
}
void wxWindow_SetOwnForegroundColour(wxWindow * self, const wxColour * colour) {
    return ((wxWindow *)self)->SetOwnForegroundColour(*(wxColour *)colour);
}
bool wxWindow_UseForegroundColour(const wxWindow * self) {
    return ((wxWindow *)self)->UseForegroundColour();
}
bool wxWindow_InheritsForegroundColour(const wxWindow * self) {
    return ((wxWindow *)self)->InheritsForegroundColour();
}
void wxWindow_SetPalette(wxWindow * self, const wxPalette * pal) {
    return ((wxWindow *)self)->SetPalette(*(wxPalette *)pal);
}
bool wxWindow_ShouldInheritColours(const wxWindow * self) {
    return ((wxWindow *)self)->ShouldInheritColours();
}
void wxWindow_SetThemeEnabled(wxWindow * self, bool enable) {
    return ((wxWindow *)self)->SetThemeEnabled(enable);
}
bool wxWindow_GetThemeEnabled(const wxWindow * self) {
    return ((wxWindow *)self)->GetThemeEnabled();
}
bool wxWindow_CanSetTransparent(wxWindow * self) {
    return ((wxWindow *)self)->CanSetTransparent();
}
bool wxWindow_SetTransparent(wxWindow * self, wxByte alpha) {
    return ((wxWindow *)self)->SetTransparent(alpha);
}
wxEvtHandler * wxWindow_GetEventHandler(const wxWindow * self) {
    return ((wxWindow *)self)->GetEventHandler();
}
bool wxWindow_HandleAsNavigationKey(wxWindow * self, const wxKeyEvent * event) {
    return ((wxWindow *)self)->HandleAsNavigationKey(*(wxKeyEvent *)event);
}
bool wxWindow_HandleWindowEvent(const wxWindow * self, wxEvent * event) {
    return ((wxWindow *)self)->HandleWindowEvent(*(wxEvent *)event);
}
bool wxWindow_ProcessWindowEvent(wxWindow * self, wxEvent * event) {
    return ((wxWindow *)self)->ProcessWindowEvent(*(wxEvent *)event);
}
bool wxWindow_ProcessWindowEventLocally(wxWindow * self, wxEvent * event) {
    return ((wxWindow *)self)->ProcessWindowEventLocally(*(wxEvent *)event);
}
wxEvtHandler * wxWindow_PopEventHandler(wxWindow * self, bool delete_handler) {
    return ((wxWindow *)self)->PopEventHandler(delete_handler);
}
void wxWindow_PushEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return ((wxWindow *)self)->PushEventHandler((wxEvtHandler *)handler);
}
bool wxWindow_RemoveEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return ((wxWindow *)self)->RemoveEventHandler((wxEvtHandler *)handler);
}
void wxWindow_SetEventHandler(wxWindow * self, wxEvtHandler * handler) {
    return ((wxWindow *)self)->SetEventHandler((wxEvtHandler *)handler);
}
void wxWindow_SetNextHandler(wxWindow * self, wxEvtHandler * handler) {
    return ((wxWindow *)self)->SetNextHandler((wxEvtHandler *)handler);
}
void wxWindow_SetPreviousHandler(wxWindow * self, wxEvtHandler * handler) {
    return ((wxWindow *)self)->SetPreviousHandler((wxEvtHandler *)handler);
}
int32_t wxWindow_GetExtraStyle(const wxWindow * self) {
    return ((wxWindow *)self)->GetExtraStyle();
}
int32_t wxWindow_GetWindowStyleFlag(const wxWindow * self) {
    return ((wxWindow *)self)->GetWindowStyleFlag();
}
int32_t wxWindow_GetWindowStyle(const wxWindow * self) {
    return ((wxWindow *)self)->GetWindowStyle();
}
bool wxWindow_HasExtraStyle(const wxWindow * self, int ex_flag) {
    return ((wxWindow *)self)->HasExtraStyle(ex_flag);
}
bool wxWindow_HasFlag(const wxWindow * self, int flag) {
    return ((wxWindow *)self)->HasFlag(flag);
}
void wxWindow_SetExtraStyle(wxWindow * self, int32_t ex_style) {
    return ((wxWindow *)self)->SetExtraStyle(ex_style);
}
void wxWindow_SetWindowStyleFlag(wxWindow * self, int32_t style) {
    return ((wxWindow *)self)->SetWindowStyleFlag(style);
}
void wxWindow_SetWindowStyle(wxWindow * self, int32_t style) {
    return ((wxWindow *)self)->SetWindowStyle(style);
}
bool wxWindow_ToggleWindowStyle(wxWindow * self, int flag) {
    return ((wxWindow *)self)->ToggleWindowStyle(flag);
}
void wxWindow_MoveAfterInTabOrder(wxWindow * self, wxWindow * win) {
    return ((wxWindow *)self)->MoveAfterInTabOrder((wxWindow *)win);
}
void wxWindow_MoveBeforeInTabOrder(wxWindow * self, wxWindow * win) {
    return ((wxWindow *)self)->MoveBeforeInTabOrder((wxWindow *)win);
}
bool wxWindow_Navigate(wxWindow * self, int flags) {
    return ((wxWindow *)self)->Navigate(flags);
}
bool wxWindow_NavigateIn(wxWindow * self, int flags) {
    return ((wxWindow *)self)->NavigateIn(flags);
}
void wxWindow_Lower(wxWindow * self) {
    return ((wxWindow *)self)->Lower();
}
void wxWindow_Raise(wxWindow * self) {
    return ((wxWindow *)self)->Raise();
}
bool wxWindow_Hide(wxWindow * self) {
    return ((wxWindow *)self)->Hide();
}
bool wxWindow_IsEnabled(const wxWindow * self) {
    return ((wxWindow *)self)->IsEnabled();
}
bool wxWindow_IsExposed(const wxWindow * self, int x, int y) {
    return ((wxWindow *)self)->IsExposed(x, y);
}
bool wxWindow_IsExposed1(const wxWindow * self, wxPoint * pt) {
    return ((wxWindow *)self)->IsExposed(*(wxPoint *)pt);
}
bool wxWindow_IsExposed2(const wxWindow * self, int x, int y, int w, int h) {
    return ((wxWindow *)self)->IsExposed(x, y, w, h);
}
bool wxWindow_IsExposed3(const wxWindow * self, wxRect * rect) {
    return ((wxWindow *)self)->IsExposed(*(wxRect *)rect);
}
bool wxWindow_IsShown(const wxWindow * self) {
    return ((wxWindow *)self)->IsShown();
}
bool wxWindow_IsShownOnScreen(const wxWindow * self) {
    return ((wxWindow *)self)->IsShownOnScreen();
}
bool wxWindow_Disable(wxWindow * self) {
    return ((wxWindow *)self)->Disable();
}
bool wxWindow_Enable(wxWindow * self, bool enable) {
    return ((wxWindow *)self)->Enable(enable);
}
bool wxWindow_Show(wxWindow * self, bool show) {
    return ((wxWindow *)self)->Show(show);
}
wxString *wxWindow_GetHelpText(const wxWindow * self) {
    return new wxString(((wxWindow *)self)->GetHelpText());
}
void wxWindow_SetHelpText(wxWindow * self, const wxString * help_text) {
    return ((wxWindow *)self)->SetHelpText(*(wxString *)help_text);
}
wxToolTip * wxWindow_GetToolTip(const wxWindow * self) {
    return ((wxWindow *)self)->GetToolTip();
}
wxString *wxWindow_GetToolTipText(const wxWindow * self) {
    return new wxString(((wxWindow *)self)->GetToolTipText());
}
void wxWindow_SetToolTip(wxWindow * self, const wxString * tip_string) {
    return ((wxWindow *)self)->SetToolTip(*(wxString *)tip_string);
}
void wxWindow_SetToolTip1(wxWindow * self, wxToolTip * tip) {
    return ((wxWindow *)self)->SetToolTip((wxToolTip *)tip);
}
void wxWindow_UnsetToolTip(wxWindow * self) {
    return ((wxWindow *)self)->UnsetToolTip();
}
int wxWindow_GetPopupMenuSelectionFromUser(wxWindow * self, wxMenu * menu, const wxPoint * pos) {
    return ((wxWindow *)self)->GetPopupMenuSelectionFromUser(*(wxMenu *)menu, *(wxPoint *)pos);
}
int wxWindow_GetPopupMenuSelectionFromUser1(wxWindow * self, wxMenu * menu, int x, int y) {
    return ((wxWindow *)self)->GetPopupMenuSelectionFromUser(*(wxMenu *)menu, x, y);
}
bool wxWindow_PopupMenu(wxWindow * self, wxMenu * menu, const wxPoint * pos) {
    return ((wxWindow *)self)->PopupMenu((wxMenu *)menu, *(wxPoint *)pos);
}
bool wxWindow_PopupMenu1(wxWindow * self, wxMenu * menu, int x, int y) {
    return ((wxWindow *)self)->PopupMenu((wxMenu *)menu, x, y);
}
wxValidator * wxWindow_GetValidator(wxWindow * self) {
    return ((wxWindow *)self)->GetValidator();
}
void wxWindow_SetValidator(wxWindow * self, const wxValidator * validator) {
    return ((wxWindow *)self)->SetValidator(*(wxValidator *)validator);
}
bool wxWindow_TransferDataFromWindow(wxWindow * self) {
    return ((wxWindow *)self)->TransferDataFromWindow();
}
bool wxWindow_TransferDataToWindow(wxWindow * self) {
    return ((wxWindow *)self)->TransferDataToWindow();
}
bool wxWindow_Validate(wxWindow * self) {
    return ((wxWindow *)self)->Validate();
}
wxWindowID wxWindow_GetId(const wxWindow * self) {
    return ((wxWindow *)self)->GetId();
}
wxString *wxWindow_GetLabel(const wxWindow * self) {
    return new wxString(((wxWindow *)self)->GetLabel());
}
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total) {
    return ((wxWindow *)self)->AdjustForLayoutDirection(x, width, width_total);
}
wxString *wxWindow_GetName(const wxWindow * self) {
    return new wxString(((wxWindow *)self)->GetName());
}
void wxWindow_SetId(wxWindow * self, wxWindowID winid) {
    return ((wxWindow *)self)->SetId(winid);
}
void wxWindow_SetLabel(wxWindow * self, const wxString * label) {
    return ((wxWindow *)self)->SetLabel(*(wxString *)label);
}
void wxWindow_SetName(wxWindow * self, const wxString * name) {
    return ((wxWindow *)self)->SetName(*(wxString *)name);
}
wxAcceleratorTable * wxWindow_GetAcceleratorTable(wxWindow * self) {
    return ((wxWindow *)self)->GetAcceleratorTable();
}
void wxWindow_SetAcceleratorTable(wxWindow * self, const wxAcceleratorTable * accel) {
    return ((wxWindow *)self)->SetAcceleratorTable(*(wxAcceleratorTable *)accel);
}
bool wxWindow_Close(wxWindow * self, bool force) {
    return ((wxWindow *)self)->Close(force);
}
bool wxWindow_Destroy(wxWindow * self) {
    return ((wxWindow *)self)->Destroy();
}
bool wxWindow_IsBeingDeleted(const wxWindow * self) {
    return ((wxWindow *)self)->IsBeingDeleted();
}
wxDropTarget * wxWindow_GetDropTarget(const wxWindow * self) {
    return ((wxWindow *)self)->GetDropTarget();
}
void wxWindow_SetDropTarget(wxWindow * self, wxDropTarget * target) {
    return ((wxWindow *)self)->SetDropTarget((wxDropTarget *)target);
}
void wxWindow_DragAcceptFiles(wxWindow * self, bool accept) {
    return ((wxWindow *)self)->DragAcceptFiles(accept);
}
wxSizer * wxWindow_GetContainingSizer(const wxWindow * self) {
    return ((wxWindow *)self)->GetContainingSizer();
}
wxSizer * wxWindow_GetSizer(const wxWindow * self) {
    return ((wxWindow *)self)->GetSizer();
}
void wxWindow_SetSizer(wxWindow * self, wxSizer * sizer, bool delete_old) {
    return ((wxWindow *)self)->SetSizer((wxSizer *)sizer, delete_old);
}
void wxWindow_SetSizerAndFit(wxWindow * self, wxSizer * sizer, bool delete_old) {
    return ((wxWindow *)self)->SetSizerAndFit((wxSizer *)sizer, delete_old);
}
wxLayoutConstraints * wxWindow_GetConstraints(const wxWindow * self) {
    return ((wxWindow *)self)->GetConstraints();
}
void wxWindow_SetConstraints(wxWindow * self, wxLayoutConstraints * constraints) {
    return ((wxWindow *)self)->SetConstraints((wxLayoutConstraints *)constraints);
}
bool wxWindow_Layout(wxWindow * self) {
    return ((wxWindow *)self)->Layout();
}
void wxWindow_SetAutoLayout(wxWindow * self, bool auto_layout) {
    return ((wxWindow *)self)->SetAutoLayout(auto_layout);
}
bool wxWindow_GetAutoLayout(const wxWindow * self) {
    return ((wxWindow *)self)->GetAutoLayout();
}
void wxWindow_CaptureMouse(wxWindow * self) {
    return ((wxWindow *)self)->CaptureMouse();
}
wxCaret * wxWindow_GetCaret(const wxWindow * self) {
    return ((wxWindow *)self)->GetCaret();
}
bool wxWindow_HasCapture(const wxWindow * self) {
    return ((wxWindow *)self)->HasCapture();
}
void wxWindow_ReleaseMouse(wxWindow * self) {
    return ((wxWindow *)self)->ReleaseMouse();
}
void wxWindow_SetCaret(wxWindow * self, wxCaret * caret) {
    return ((wxWindow *)self)->SetCaret((wxCaret *)caret);
}
bool wxWindow_SetCursor(wxWindow * self, const wxCursor * cursor) {
    return ((wxWindow *)self)->SetCursor(*(wxCursor *)cursor);
}
void wxWindow_WarpPointer(wxWindow * self, int x, int y) {
    return ((wxWindow *)self)->WarpPointer(x, y);
}
bool wxWindow_EnableTouchEvents(wxWindow * self, int events_mask) {
    return ((wxWindow *)self)->EnableTouchEvents(events_mask);
}
void wxWindow_DoUpdateWindowUI(wxWindow * self, wxUpdateUIEvent * event) {
    return ((wxWindow *)self)->DoUpdateWindowUI(*(wxUpdateUIEvent *)event);
}
bool wxWindow_HasMultiplePages(const wxWindow * self) {
    return ((wxWindow *)self)->HasMultiplePages();
}
void wxWindow_InheritAttributes(wxWindow * self) {
    return ((wxWindow *)self)->InheritAttributes();
}
void wxWindow_InitDialog(wxWindow * self) {
    return ((wxWindow *)self)->InitDialog();
}
bool wxWindow_IsDoubleBuffered(const wxWindow * self) {
    return ((wxWindow *)self)->IsDoubleBuffered();
}
void wxWindow_SetDoubleBuffered(wxWindow * self, bool on) {
    return ((wxWindow *)self)->SetDoubleBuffered(on);
}
bool wxWindow_IsRetained(const wxWindow * self) {
    return ((wxWindow *)self)->IsRetained();
}
bool wxWindow_IsThisEnabled(const wxWindow * self) {
    return ((wxWindow *)self)->IsThisEnabled();
}
bool wxWindow_IsTopLevel(const wxWindow * self) {
    return ((wxWindow *)self)->IsTopLevel();
}
void wxWindow_OnInternalIdle(wxWindow * self) {
    return ((wxWindow *)self)->OnInternalIdle();
}
bool wxWindow_SendIdleEvents(wxWindow * self, wxIdleEvent * event) {
    return ((wxWindow *)self)->SendIdleEvents(*(wxIdleEvent *)event);
}
bool wxWindow_RegisterHotKey(wxWindow * self, int hotkey_id, int modifiers, int virtual_key_code) {
    return ((wxWindow *)self)->RegisterHotKey(hotkey_id, modifiers, virtual_key_code);
}
bool wxWindow_UnregisterHotKey(wxWindow * self, int hotkey_id) {
    return ((wxWindow *)self)->UnregisterHotKey(hotkey_id);
}
void wxWindow_UpdateWindowUI(wxWindow * self, int32_t flags) {
    return ((wxWindow *)self)->UpdateWindowUI(flags);
}
wxWindow * wxWindow_FindFocus() {
    return wxWindow::FindFocus();
}
wxWindow * wxWindow_FindWindowById(int32_t id, const wxWindow * parent) {
    return wxWindow::FindWindowById(id, (wxWindow *)parent);
}
wxWindow * wxWindow_FindWindowByLabel(const wxString * label, const wxWindow * parent) {
    return wxWindow::FindWindowByLabel(*(wxString *)label, (wxWindow *)parent);
}
wxWindow * wxWindow_FindWindowByName(const wxString * name, const wxWindow * parent) {
    return wxWindow::FindWindowByName(*(wxString *)name, (wxWindow *)parent);
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
wxWindow *wxWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, int32_t style, const wxString * name) {
    return new wxWindow((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
bool wxWindow_Create(wxWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, int32_t style, const wxString * name) {
    return ((wxWindow *)self)->Create((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}

// CLASS: wxControl
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, int32_t style, const wxValidator * validator, const wxString * name) {
    return new wxControl((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
wxControl *wxControl_new1() {
    return new wxControl();
}
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, int32_t style, const wxValidator * validator, const wxString * name) {
    return ((wxControl *)self)->Create((wxWindow *)parent, id, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
void wxControl_Command(wxControl * self, wxCommandEvent * event) {
    return ((wxControl *)self)->Command(*(wxCommandEvent *)event);
}
wxString *wxControl_GetLabel(const wxControl * self) {
    return new wxString(((wxControl *)self)->GetLabel());
}
wxString *wxControl_GetLabelText(const wxControl * self) {
    return new wxString(((wxControl *)self)->GetLabelText());
}
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen) {
    return new wxSize(((wxControl *)self)->GetSizeFromTextSize(xlen, ylen));
}
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize) {
    return new wxSize(((wxControl *)self)->GetSizeFromTextSize(*(wxSize *)tsize));
}
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text) {
    return new wxSize(((wxControl *)self)->GetSizeFromText(*(wxString *)text));
}
void wxControl_SetLabel(wxControl * self, const wxString * label) {
    return ((wxControl *)self)->SetLabel(*(wxString *)label);
}
void wxControl_SetLabelText(wxControl * self, const wxString * text) {
    return ((wxControl *)self)->SetLabelText(*(wxString *)text);
}
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup) {
    return ((wxControl *)self)->SetLabelMarkup(*(wxString *)markup);
}
wxString *wxControl_GetLabelText1(const wxString * label) {
    return new wxString(wxControl::GetLabelText(*(wxString *)label));
}
wxString *wxControl_RemoveMnemonics(const wxString * str) {
    return new wxString(wxControl::RemoveMnemonics(*(wxString *)str));
}
wxString *wxControl_EscapeMnemonics(const wxString * text) {
    return new wxString(wxControl::EscapeMnemonics(*(wxString *)text));
}

// CLASS: wxAnyButton
wxAnyButton *wxAnyButton_new() {
    return new wxAnyButton();
}
void wxAnyButton_SetBitmapCurrent(wxAnyButton * self, const wxBitmap * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapCurrent(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapDisabled(wxAnyButton * self, const wxBitmap * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapDisabled(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapFocus(wxAnyButton * self, const wxBitmap * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapFocus(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapLabel(wxAnyButton * self, const wxBitmap * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapLabel(*(wxBitmap *)bitmap);
}
void wxAnyButton_SetBitmapPressed(wxAnyButton * self, const wxBitmap * bitmap) {
    return ((wxAnyButton *)self)->SetBitmapPressed(*(wxBitmap *)bitmap);
}
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton * self) {
    return new wxSize(((wxAnyButton *)self)->GetBitmapMargins());
}
void wxAnyButton_SetBitmapMargins(wxAnyButton * self, wxCoord x, wxCoord y) {
    return ((wxAnyButton *)self)->SetBitmapMargins(x, y);
}
void wxAnyButton_SetBitmapMargins1(wxAnyButton * self, const wxSize * sz) {
    return ((wxAnyButton *)self)->SetBitmapMargins(*(wxSize *)sz);
}

// CLASS: wxButton
wxButton *wxButton_new() {
    return new wxButton();
}
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, int32_t style, const wxValidator * validator, const wxString * name) {
    return new wxButton((wxWindow *)parent, id, *(wxString *)label, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, int32_t style, const wxValidator * validator, const wxString * name) {
    return ((wxButton *)self)->Create((wxWindow *)parent, id, *(wxString *)label, *(wxPoint *)pos, *(wxSize *)size, style, *(wxValidator *)validator, *(wxString *)name);
}
bool wxButton_GetAuthNeeded(const wxButton * self) {
    return ((wxButton *)self)->GetAuthNeeded();
}
wxString *wxButton_GetLabel(const wxButton * self) {
    return new wxString(((wxButton *)self)->GetLabel());
}
void wxButton_SetAuthNeeded(wxButton * self, bool needed) {
    return ((wxButton *)self)->SetAuthNeeded(needed);
}
wxWindow * wxButton_SetDefault(wxButton * self) {
    return ((wxButton *)self)->SetDefault();
}
void wxButton_SetLabel(wxButton * self, const wxString * label) {
    return ((wxButton *)self)->SetLabel(*(wxString *)label);
}
wxSize *wxButton_GetDefaultSize(wxWindow * win) {
    return new wxSize(wxButton::GetDefaultSize((wxWindow *)win));
}

// CLASS: wxNonOwnedWindow
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region) {
    return ((wxNonOwnedWindow *)self)->SetShape(*(wxRegion *)region);
}
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path) {
    return ((wxNonOwnedWindow *)self)->SetShape(*(wxGraphicsPath *)path);
}

// CLASS: wxTopLevelWindow
wxTopLevelWindow *wxTopLevelWindow_new() {
    return new wxTopLevelWindow();
}
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, int32_t style, const wxString * name) {
    return new wxTopLevelWindow((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, int32_t style, const wxString * name) {
    return ((wxTopLevelWindow *)self)->Create((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
bool wxTopLevelWindow_CanSetTransparent(wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->CanSetTransparent();
}
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction) {
    return ((wxTopLevelWindow *)self)->CenterOnScreen(direction);
}
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction) {
    return ((wxTopLevelWindow *)self)->CentreOnScreen(direction);
}
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableCloseButton(enable);
}
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableMaximizeButton(enable);
}
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableMinimizeButton(enable);
}
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->GetDefaultItem();
}
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self) {
    return new wxString(((wxTopLevelWindow *)self)->GetTitle());
}
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize) {
    return ((wxTopLevelWindow *)self)->Iconize(iconize);
}
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->IsActive();
}
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->IsAlwaysMaximized();
}
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->IsFullScreen();
}
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->IsIconized();
}
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->IsMaximized();
}
bool wxTopLevelWindow_Layout(wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->Layout();
}
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize) {
    return ((wxTopLevelWindow *)self)->Maximize(maximize);
}
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags) {
    return ((wxTopLevelWindow *)self)->RequestUserAttention(flags);
}
void wxTopLevelWindow_Restore(wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->Restore();
}
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return ((wxTopLevelWindow *)self)->SetDefaultItem((wxWindow *)win);
}
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win) {
    return ((wxTopLevelWindow *)self)->SetTmpDefaultItem((wxWindow *)win);
}
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->GetTmpDefaultItem();
}
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon) {
    return ((wxTopLevelWindow *)self)->SetIcon(*(wxIcon *)icon);
}
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons) {
    return ((wxTopLevelWindow *)self)->SetIcons(*(wxIconBundle *)icons);
}
void wxTopLevelWindow_SetMaxSize(wxTopLevelWindow * self, const wxSize * size) {
    return ((wxTopLevelWindow *)self)->SetMaxSize(*(wxSize *)size);
}
void wxTopLevelWindow_SetMinSize(wxTopLevelWindow * self, const wxSize * size) {
    return ((wxTopLevelWindow *)self)->SetMinSize(*(wxSize *)size);
}
void wxTopLevelWindow_SetSizeHints(wxTopLevelWindow * self, int min_w, int min_h, int max_w, int max_h, int inc_w, int inc_h) {
    return ((wxTopLevelWindow *)self)->SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h);
}
void wxTopLevelWindow_SetSizeHints1(wxTopLevelWindow * self, const wxSize * min_size, const wxSize * max_size, const wxSize * inc_size) {
    return ((wxTopLevelWindow *)self)->SetSizeHints(*(wxSize *)min_size, *(wxSize *)max_size, *(wxSize *)inc_size);
}
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title) {
    return ((wxTopLevelWindow *)self)->SetTitle(*(wxString *)title);
}
bool wxTopLevelWindow_SetTransparent(wxTopLevelWindow * self, wxByte alpha) {
    return ((wxTopLevelWindow *)self)->SetTransparent(alpha);
}
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->ShouldPreventAppExit();
}
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified) {
    return ((wxTopLevelWindow *)self)->OSXSetModified(modified);
}
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->OSXIsModified();
}
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename) {
    return ((wxTopLevelWindow *)self)->SetRepresentedFilename(*(wxString *)filename);
}
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self) {
    return ((wxTopLevelWindow *)self)->ShowWithoutActivating();
}
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable) {
    return ((wxTopLevelWindow *)self)->EnableFullScreenView(enable);
}
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, int32_t style) {
    return ((wxTopLevelWindow *)self)->ShowFullScreen(show, style);
}
wxSize *wxTopLevelWindow_GetDefaultSize() {
    return new wxSize(wxTopLevelWindow::GetDefaultSize());
}

// CLASS: wxFrame
wxFrame *wxFrame_new() {
    return new wxFrame();
}
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, int32_t style, const wxString * name) {
    return new wxFrame((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
void wxFrame_Centre(wxFrame * self, int direction) {
    return ((wxFrame *)self)->Centre(direction);
}
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, int32_t style, const wxString * name) {
    return ((wxFrame *)self)->Create((wxWindow *)parent, id, *(wxString *)title, *(wxPoint *)pos, *(wxSize *)size, style, *(wxString *)name);
}
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, int32_t style, wxWindowID id, const wxString * name) {
    return ((wxFrame *)self)->CreateStatusBar(number, style, id, *(wxString *)name);
}
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, int32_t style, wxWindowID id, const wxString * name) {
    return ((wxFrame *)self)->CreateToolBar(style, id, *(wxString *)name);
}
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show) {
    return ((wxFrame *)self)->DoGiveHelp(*(wxString *)text, show);
}
wxPoint *wxFrame_GetClientAreaOrigin(const wxFrame * self) {
    return new wxPoint(((wxFrame *)self)->GetClientAreaOrigin());
}
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self) {
    return ((wxFrame *)self)->GetMenuBar();
}
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self) {
    return ((wxFrame *)self)->GetStatusBar();
}
int wxFrame_GetStatusBarPane(const wxFrame * self) {
    return ((wxFrame *)self)->GetStatusBarPane();
}
wxToolBar * wxFrame_GetToolBar(const wxFrame * self) {
    return ((wxFrame *)self)->GetToolBar();
}
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, int32_t style, wxWindowID id, const wxString * name) {
    return ((wxFrame *)self)->OnCreateStatusBar(number, style, id, *(wxString *)name);
}
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, int32_t style, wxWindowID id, const wxString * name) {
    return ((wxFrame *)self)->OnCreateToolBar(style, id, *(wxString *)name);
}
bool wxFrame_ProcessCommand(wxFrame * self, int id) {
    return ((wxFrame *)self)->ProcessCommand(id);
}
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar) {
    return ((wxFrame *)self)->SetMenuBar((wxMenuBar *)menu_bar);
}
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar) {
    return ((wxFrame *)self)->SetStatusBar((wxStatusBar *)status_bar);
}
void wxFrame_SetStatusBarPane(wxFrame * self, int n) {
    return ((wxFrame *)self)->SetStatusBarPane(n);
}
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number) {
    return ((wxFrame *)self)->SetStatusText(*(wxString *)text, number);
}
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field) {
    return ((wxFrame *)self)->SetStatusWidths(n, (int *)widths_field);
}
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar) {
    return ((wxFrame *)self)->SetToolBar((wxToolBar *)tool_bar);
}
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number) {
    return ((wxFrame *)self)->PushStatusText(*(wxString *)text, number);
}
void wxFrame_PopStatusText(wxFrame * self, int number) {
    return ((wxFrame *)self)->PopStatusText(number);
}

// CLASS: wxPoint
bool wxPoint_IsFullySpecified(const wxPoint * self) {
    return ((wxPoint *)self)->IsFullySpecified();
}
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt) {
    return ((wxPoint *)self)->SetDefaults(*(wxPoint *)pt);
}
wxPoint *wxPoint_new() {
    return new wxPoint();
}
wxPoint *wxPoint_new1(int x, int y) {
    return new wxPoint(x, y);
}
wxPoint *wxPoint_new2(const wxRealPoint * pt) {
    return new wxPoint(*(wxRealPoint *)pt);
}

// CLASS: wxRect
wxRect *wxRect_new() {
    return new wxRect();
}
wxRect *wxRect_new1(int x, int y, int width, int height) {
    return new wxRect(x, y, width, height);
}
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right) {
    return new wxRect(*(wxPoint *)top_left, *(wxPoint *)bottom_right);
}
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size) {
    return new wxRect(*(wxPoint *)pos, *(wxSize *)size);
}
wxRect *wxRect_new4(const wxSize * size) {
    return new wxRect(*(wxSize *)size);
}
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(((wxRect *)self)->CentreIn(*(wxRect *)r, dir));
}
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(((wxRect *)self)->CenterIn(*(wxRect *)r, dir));
}
bool wxRect_Contains(const wxRect * self, int x, int y) {
    return ((wxRect *)self)->Contains(x, y);
}
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt) {
    return ((wxRect *)self)->Contains(*(wxPoint *)pt);
}
bool wxRect_Contains2(const wxRect * self, const wxRect * rect) {
    return ((wxRect *)self)->Contains(*(wxRect *)rect);
}
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(((wxRect *)self)->Deflate(dx, dy));
}
int wxRect_GetBottom(const wxRect * self) {
    return ((wxRect *)self)->GetBottom();
}
wxPoint *wxRect_GetBottomLeft(const wxRect * self) {
    return new wxPoint(((wxRect *)self)->GetBottomLeft());
}
wxPoint *wxRect_GetBottomRight(const wxRect * self) {
    return new wxPoint(((wxRect *)self)->GetBottomRight());
}
int wxRect_GetHeight(const wxRect * self) {
    return ((wxRect *)self)->GetHeight();
}
int wxRect_GetLeft(const wxRect * self) {
    return ((wxRect *)self)->GetLeft();
}
wxPoint *wxRect_GetPosition(const wxRect * self) {
    return new wxPoint(((wxRect *)self)->GetPosition());
}
int wxRect_GetRight(const wxRect * self) {
    return ((wxRect *)self)->GetRight();
}
wxSize *wxRect_GetSize(const wxRect * self) {
    return new wxSize(((wxRect *)self)->GetSize());
}
int wxRect_GetTop(const wxRect * self) {
    return ((wxRect *)self)->GetTop();
}
wxPoint *wxRect_GetTopLeft(const wxRect * self) {
    return new wxPoint(((wxRect *)self)->GetTopLeft());
}
wxPoint *wxRect_GetTopRight(const wxRect * self) {
    return new wxPoint(((wxRect *)self)->GetTopRight());
}
int wxRect_GetWidth(const wxRect * self) {
    return ((wxRect *)self)->GetWidth();
}
int wxRect_GetX(const wxRect * self) {
    return ((wxRect *)self)->GetX();
}
int wxRect_GetY(const wxRect * self) {
    return ((wxRect *)self)->GetY();
}
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(((wxRect *)self)->Inflate(dx, dy));
}
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect) {
    return new wxRect(((wxRect *)self)->Intersect(*(wxRect *)rect));
}
bool wxRect_Intersects(const wxRect * self, const wxRect * rect) {
    return ((wxRect *)self)->Intersects(*(wxRect *)rect);
}
bool wxRect_IsEmpty(const wxRect * self) {
    return ((wxRect *)self)->IsEmpty();
}
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy) {
    return ((wxRect *)self)->Offset(dx, dy);
}
void wxRect_Offset1(wxRect * self, const wxPoint * pt) {
    return ((wxRect *)self)->Offset(*(wxPoint *)pt);
}
void wxRect_SetHeight(wxRect * self, int height) {
    return ((wxRect *)self)->SetHeight(height);
}
void wxRect_SetPosition(wxRect * self, const wxPoint * pos) {
    return ((wxRect *)self)->SetPosition(*(wxPoint *)pos);
}
void wxRect_SetSize(wxRect * self, const wxSize * s) {
    return ((wxRect *)self)->SetSize(*(wxSize *)s);
}
void wxRect_SetWidth(wxRect * self, int width) {
    return ((wxRect *)self)->SetWidth(width);
}
void wxRect_SetX(wxRect * self, int x) {
    return ((wxRect *)self)->SetX(x);
}
void wxRect_SetY(wxRect * self, int y) {
    return ((wxRect *)self)->SetY(y);
}
void wxRect_SetLeft(wxRect * self, int left) {
    return ((wxRect *)self)->SetLeft(left);
}
void wxRect_SetRight(wxRect * self, int right) {
    return ((wxRect *)self)->SetRight(right);
}
void wxRect_SetTop(wxRect * self, int top) {
    return ((wxRect *)self)->SetTop(top);
}
void wxRect_SetBottom(wxRect * self, int bottom) {
    return ((wxRect *)self)->SetBottom(bottom);
}
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p) {
    return ((wxRect *)self)->SetTopLeft(*(wxPoint *)p);
}
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p) {
    return ((wxRect *)self)->SetBottomRight(*(wxPoint *)p);
}
void wxRect_SetTopRight(wxRect * self, const wxPoint * p) {
    return ((wxRect *)self)->SetTopRight(*(wxPoint *)p);
}
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p) {
    return ((wxRect *)self)->SetBottomLeft(*(wxPoint *)p);
}
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect) {
    return new wxRect(((wxRect *)self)->Union(*(wxRect *)rect));
}

// CLASS: wxSize
wxSize *wxSize_new() {
    return new wxSize();
}
wxSize *wxSize_new1(int width, int height) {
    return new wxSize(width, height);
}
void wxSize_DecBy(wxSize * self, const wxPoint * pt) {
    return ((wxSize *)self)->DecBy(*(wxPoint *)pt);
}
void wxSize_DecBy1(wxSize * self, const wxSize * size) {
    return ((wxSize *)self)->DecBy(*(wxSize *)size);
}
void wxSize_DecBy2(wxSize * self, int dx, int dy) {
    return ((wxSize *)self)->DecBy(dx, dy);
}
void wxSize_DecBy3(wxSize * self, int d) {
    return ((wxSize *)self)->DecBy(d);
}
void wxSize_DecTo(wxSize * self, const wxSize * size) {
    return ((wxSize *)self)->DecTo(*(wxSize *)size);
}
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size) {
    return ((wxSize *)self)->DecToIfSpecified(*(wxSize *)size);
}
int wxSize_GetHeight(const wxSize * self) {
    return ((wxSize *)self)->GetHeight();
}
int wxSize_GetWidth(const wxSize * self) {
    return ((wxSize *)self)->GetWidth();
}
void wxSize_IncBy(wxSize * self, const wxPoint * pt) {
    return ((wxSize *)self)->IncBy(*(wxPoint *)pt);
}
void wxSize_IncBy1(wxSize * self, const wxSize * size) {
    return ((wxSize *)self)->IncBy(*(wxSize *)size);
}
void wxSize_IncBy2(wxSize * self, int dx, int dy) {
    return ((wxSize *)self)->IncBy(dx, dy);
}
void wxSize_IncBy3(wxSize * self, int d) {
    return ((wxSize *)self)->IncBy(d);
}
void wxSize_IncTo(wxSize * self, const wxSize * size) {
    return ((wxSize *)self)->IncTo(*(wxSize *)size);
}
bool wxSize_IsFullySpecified(const wxSize * self) {
    return ((wxSize *)self)->IsFullySpecified();
}
void wxSize_Set(wxSize * self, int width, int height) {
    return ((wxSize *)self)->Set(width, height);
}
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default) {
    return ((wxSize *)self)->SetDefaults(*(wxSize *)size_default);
}
void wxSize_SetHeight(wxSize * self, int height) {
    return ((wxSize *)self)->SetHeight(height);
}
void wxSize_SetWidth(wxSize * self, int width) {
    return ((wxSize *)self)->SetWidth(width);
}

// CLASS: wxValidator
wxValidator *wxValidator_new() {
    return new wxValidator();
}
wxObject * wxValidator_Clone(const wxValidator * self) {
    return ((wxValidator *)self)->Clone();
}
wxWindow * wxValidator_GetWindow(const wxValidator * self) {
    return ((wxValidator *)self)->GetWindow();
}
void wxValidator_SetWindow(wxValidator * self, wxWindow * window) {
    return ((wxValidator *)self)->SetWindow((wxWindow *)window);
}
bool wxValidator_TransferFromWindow(wxValidator * self) {
    return ((wxValidator *)self)->TransferFromWindow();
}
bool wxValidator_TransferToWindow(wxValidator * self) {
    return ((wxValidator *)self)->TransferToWindow();
}
bool wxValidator_Validate(wxValidator * self, wxWindow * parent) {
    return ((wxValidator *)self)->Validate((wxWindow *)parent);
}
void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}

} // extern "C"

