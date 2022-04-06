#pragma once
#include <wx/wx.h>

extern "C" {

// CLASS: wxCommandEvent
void * wxCommandEvent_GetClientData(const wxCommandEvent * self);
wxClientData * wxCommandEvent_GetClientObject(const wxCommandEvent * self);
long wxCommandEvent_GetExtraLong(const wxCommandEvent * self);
int wxCommandEvent_GetInt(const wxCommandEvent * self);
int wxCommandEvent_GetSelection(const wxCommandEvent * self);
wxString *wxCommandEvent_GetString(const wxCommandEvent * self);
bool wxCommandEvent_IsChecked(const wxCommandEvent * self);
bool wxCommandEvent_IsSelection(const wxCommandEvent * self);
void wxCommandEvent_SetClientData(wxCommandEvent * self, void * client_data);
void wxCommandEvent_SetClientObject(wxCommandEvent * self, wxClientData * client_object);
void wxCommandEvent_SetExtraLong(wxCommandEvent * self, long extra_long);
void wxCommandEvent_SetInt(wxCommandEvent * self, int int_command);
void wxCommandEvent_SetString(wxCommandEvent * self, const wxString * string);

// CLASS: wxWindow
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
int wxWindow_GetCharHeight(const wxWindow * self);
int wxWindow_GetCharWidth(const wxWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxWindow_GetDPI(const wxWindow * self);
#endif
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
wxString *wxWindow_GetHelpText(const wxWindow * self);
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
wxCoord wxWindow_AdjustForLayoutDirection(const wxWindow * self, wxCoord x, wxCoord width, wxCoord width_total);
wxString *wxWindow_GetName(const wxWindow * self);
void wxWindow_SetId(wxWindow * self, wxWindowID winid);
void wxWindow_SetLabel(wxWindow * self, const wxString * label);
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
#if wxCHECK_VERSION(3, 1, 0)
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

// CLASS: wxControl
wxControl *wxControl_new(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxControl *wxControl_new1();
bool wxControl_Create(wxControl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxControl_Command(wxControl * self, wxCommandEvent * event);
wxString *wxControl_GetLabelText(const wxControl * self);
wxSize *wxControl_GetSizeFromTextSize(const wxControl * self, int xlen, int ylen);
wxSize *wxControl_GetSizeFromTextSize1(const wxControl * self, const wxSize * tsize);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxControl_GetSizeFromText(const wxControl * self, const wxString * text);
#endif
void wxControl_SetLabelText(wxControl * self, const wxString * text);
bool wxControl_SetLabelMarkup(wxControl * self, const wxString * markup);
wxString *wxControl_GetLabelText1(const wxString * label);
wxString *wxControl_RemoveMnemonics(const wxString * str);
wxString *wxControl_EscapeMnemonics(const wxString * text);
wxString *wxControl_Ellipsize(const wxString * label, const wxDC * dc, wxEllipsizeMode mode, int max_width, int flags);

// CLASS: wxAnyButton
wxAnyButton *wxAnyButton_new();
void wxAnyButton_SetBitmapCurrent(wxAnyButton * self, const wxBitmap * bitmap);
void wxAnyButton_SetBitmapDisabled(wxAnyButton * self, const wxBitmap * bitmap);
void wxAnyButton_SetBitmapFocus(wxAnyButton * self, const wxBitmap * bitmap);
void wxAnyButton_SetBitmapLabel(wxAnyButton * self, const wxBitmap * bitmap);
void wxAnyButton_SetBitmapPressed(wxAnyButton * self, const wxBitmap * bitmap);
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton * self);
void wxAnyButton_SetBitmapMargins(wxAnyButton * self, wxCoord x, wxCoord y);
void wxAnyButton_SetBitmapMargins1(wxAnyButton * self, const wxSize * sz);

// CLASS: wxButton
wxButton *wxButton_new();
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxButton_GetAuthNeeded(const wxButton * self);
void wxButton_SetAuthNeeded(wxButton * self, bool needed);
wxWindow * wxButton_SetDefault(wxButton * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxButton_GetDefaultSize(wxWindow * win);
#endif

// CLASS: wxMenu
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
wxString *wxMenu_GetHelpString(const wxMenu * self, int id);
wxString *wxMenu_GetLabel(const wxMenu * self, int id);
wxString *wxMenu_GetLabelText(const wxMenu * self, int id);
wxString *wxMenu_GetTitle(const wxMenu * self);
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
wxMenuBar *wxMenuBar_new(long style);
bool wxMenuBar_Append(wxMenuBar * self, wxMenu * menu, const wxString * title);
void wxMenuBar_Check(wxMenuBar * self, int id, bool check);
void wxMenuBar_Enable(wxMenuBar * self, int id, bool enable);
wxMenuItem * wxMenuBar_FindItem(const wxMenuBar * self, int id, wxMenu ** menu);
int wxMenuBar_FindMenu(const wxMenuBar * self, const wxString * title);
int wxMenuBar_FindMenuItem(const wxMenuBar * self, const wxString * menu_string, const wxString * item_string);
wxString *wxMenuBar_GetHelpString(const wxMenuBar * self, int id);
wxString *wxMenuBar_GetLabel(const wxMenuBar * self, int id);
bool wxMenuBar_IsChecked(const wxMenuBar * self, int id);
bool wxMenuBar_IsEnabled(const wxMenuBar * self, int id);
void wxMenuBar_SetHelpString(wxMenuBar * self, int id, const wxString * help_string);
void wxMenuBar_SetLabel(wxMenuBar * self, int id, const wxString * label);
wxFrame * wxMenuBar_GetFrame(const wxMenuBar * self);
bool wxMenuBar_IsAttached(const wxMenuBar * self);
void wxMenuBar_Attach(wxMenuBar * self, wxFrame * frame);
void wxMenuBar_Detach(wxMenuBar * self);

// CLASS: wxNonOwnedWindow
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region);
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path);

// CLASS: wxTopLevelWindow
wxTopLevelWindow *wxTopLevelWindow_new();
wxTopLevelWindow *wxTopLevelWindow_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxTopLevelWindow_Create(wxTopLevelWindow * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxTopLevelWindow_CenterOnScreen(wxTopLevelWindow * self, int direction);
void wxTopLevelWindow_CentreOnScreen(wxTopLevelWindow * self, int direction);
bool wxTopLevelWindow_EnableCloseButton(wxTopLevelWindow * self, bool enable);
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableMaximizeButton(wxTopLevelWindow * self, bool enable);
bool wxTopLevelWindow_EnableMinimizeButton(wxTopLevelWindow * self, bool enable);
#endif
wxWindow * wxTopLevelWindow_GetDefaultItem(const wxTopLevelWindow * self);
wxString *wxTopLevelWindow_GetTitle(const wxTopLevelWindow * self);
void wxTopLevelWindow_Iconize(wxTopLevelWindow * self, bool iconize);
bool wxTopLevelWindow_IsActive(wxTopLevelWindow * self);
bool wxTopLevelWindow_IsAlwaysMaximized(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsFullScreen(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsIconized(const wxTopLevelWindow * self);
bool wxTopLevelWindow_IsMaximized(const wxTopLevelWindow * self);
void wxTopLevelWindow_Maximize(wxTopLevelWindow * self, bool maximize);
void wxTopLevelWindow_RequestUserAttention(wxTopLevelWindow * self, int flags);
void wxTopLevelWindow_Restore(wxTopLevelWindow * self);
wxWindow * wxTopLevelWindow_SetDefaultItem(wxTopLevelWindow * self, wxWindow * win);
wxWindow * wxTopLevelWindow_SetTmpDefaultItem(wxTopLevelWindow * self, wxWindow * win);
wxWindow * wxTopLevelWindow_GetTmpDefaultItem(const wxTopLevelWindow * self);
void wxTopLevelWindow_SetIcon(wxTopLevelWindow * self, const wxIcon * icon);
void wxTopLevelWindow_SetIcons(wxTopLevelWindow * self, const wxIconBundle * icons);
void wxTopLevelWindow_SetTitle(wxTopLevelWindow * self, const wxString * title);
bool wxTopLevelWindow_ShouldPreventAppExit(const wxTopLevelWindow * self);
void wxTopLevelWindow_OSXSetModified(wxTopLevelWindow * self, bool modified);
bool wxTopLevelWindow_OSXIsModified(const wxTopLevelWindow * self);
void wxTopLevelWindow_SetRepresentedFilename(wxTopLevelWindow * self, const wxString * filename);
void wxTopLevelWindow_ShowWithoutActivating(wxTopLevelWindow * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxTopLevelWindow_EnableFullScreenView(wxTopLevelWindow * self, bool enable);
#endif
bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow * self, bool show, long style);
wxSize *wxTopLevelWindow_GetDefaultSize();

// CLASS: wxFrame
wxFrame *wxFrame_new();
wxFrame *wxFrame_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxFrame_Centre(wxFrame * self, int direction);
bool wxFrame_Create(wxFrame * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxStatusBar * wxFrame_CreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name);
wxToolBar * wxFrame_CreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name);
void wxFrame_DoGiveHelp(wxFrame * self, const wxString * text, bool show);
wxMenuBar * wxFrame_GetMenuBar(const wxFrame * self);
wxStatusBar * wxFrame_GetStatusBar(const wxFrame * self);
int wxFrame_GetStatusBarPane(const wxFrame * self);
wxToolBar * wxFrame_GetToolBar(const wxFrame * self);
wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame * self, int number, long style, wxWindowID id, const wxString * name);
wxToolBar * wxFrame_OnCreateToolBar(wxFrame * self, long style, wxWindowID id, const wxString * name);
bool wxFrame_ProcessCommand(wxFrame * self, int id);
void wxFrame_SetMenuBar(wxFrame * self, wxMenuBar * menu_bar);
void wxFrame_SetStatusBar(wxFrame * self, wxStatusBar * status_bar);
void wxFrame_SetStatusBarPane(wxFrame * self, int n);
void wxFrame_SetStatusText(wxFrame * self, const wxString * text, int number);
void wxFrame_SetStatusWidths(wxFrame * self, int n, const int * widths_field);
void wxFrame_SetToolBar(wxFrame * self, wxToolBar * tool_bar);
void wxFrame_PushStatusText(wxFrame * self, const wxString * text, int number);
void wxFrame_PopStatusText(wxFrame * self, int number);

// CLASS: wxPoint
void wxPoint_delete(wxPoint *self);
bool wxPoint_IsFullySpecified(const wxPoint * self);
void wxPoint_SetDefaults(wxPoint * self, const wxPoint * pt);
wxPoint *wxPoint_new();
wxPoint *wxPoint_new1(int x, int y);
wxPoint *wxPoint_new2(const wxRealPoint * pt);

// CLASS: wxRect
void wxRect_delete(wxRect *self);
wxRect *wxRect_new();
wxRect *wxRect_new1(int x, int y, int width, int height);
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right);
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size);
wxRect *wxRect_new4(const wxSize * size);
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir);
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir);
bool wxRect_Contains(const wxRect * self, int x, int y);
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt);
bool wxRect_Contains2(const wxRect * self, const wxRect * rect);
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy);
int wxRect_GetBottom(const wxRect * self);
wxPoint *wxRect_GetBottomLeft(const wxRect * self);
wxPoint *wxRect_GetBottomRight(const wxRect * self);
int wxRect_GetHeight(const wxRect * self);
int wxRect_GetLeft(const wxRect * self);
wxPoint *wxRect_GetPosition(const wxRect * self);
int wxRect_GetRight(const wxRect * self);
wxSize *wxRect_GetSize(const wxRect * self);
int wxRect_GetTop(const wxRect * self);
wxPoint *wxRect_GetTopLeft(const wxRect * self);
wxPoint *wxRect_GetTopRight(const wxRect * self);
int wxRect_GetWidth(const wxRect * self);
int wxRect_GetX(const wxRect * self);
int wxRect_GetY(const wxRect * self);
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy);
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect);
bool wxRect_Intersects(const wxRect * self, const wxRect * rect);
bool wxRect_IsEmpty(const wxRect * self);
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy);
void wxRect_Offset1(wxRect * self, const wxPoint * pt);
void wxRect_SetHeight(wxRect * self, int height);
void wxRect_SetPosition(wxRect * self, const wxPoint * pos);
void wxRect_SetSize(wxRect * self, const wxSize * s);
void wxRect_SetWidth(wxRect * self, int width);
void wxRect_SetX(wxRect * self, int x);
void wxRect_SetY(wxRect * self, int y);
void wxRect_SetLeft(wxRect * self, int left);
void wxRect_SetRight(wxRect * self, int right);
void wxRect_SetTop(wxRect * self, int top);
void wxRect_SetBottom(wxRect * self, int bottom);
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p);
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p);
void wxRect_SetTopRight(wxRect * self, const wxPoint * p);
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p);
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect);

// CLASS: wxSize
void wxSize_delete(wxSize *self);
wxSize *wxSize_new();
wxSize *wxSize_new1(int width, int height);
void wxSize_DecBy(wxSize * self, const wxPoint * pt);
void wxSize_DecBy1(wxSize * self, const wxSize * size);
void wxSize_DecBy2(wxSize * self, int dx, int dy);
void wxSize_DecBy3(wxSize * self, int d);
void wxSize_DecTo(wxSize * self, const wxSize * size);
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size);
int wxSize_GetHeight(const wxSize * self);
int wxSize_GetWidth(const wxSize * self);
void wxSize_IncBy(wxSize * self, const wxPoint * pt);
void wxSize_IncBy1(wxSize * self, const wxSize * size);
void wxSize_IncBy2(wxSize * self, int dx, int dy);
void wxSize_IncBy3(wxSize * self, int d);
void wxSize_IncTo(wxSize * self, const wxSize * size);
bool wxSize_IsFullySpecified(const wxSize * self);
void wxSize_Set(wxSize * self, int width, int height);
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default);
void wxSize_SetHeight(wxSize * self, int height);
void wxSize_SetWidth(wxSize * self, int width);

// CLASS: wxValidator
wxValidator *wxValidator_new();
wxObject * wxValidator_Clone(const wxValidator * self);
wxWindow * wxValidator_GetWindow(const wxValidator * self);
void wxValidator_SetWindow(wxValidator * self, wxWindow * window);
bool wxValidator_TransferFromWindow(wxValidator * self);
bool wxValidator_TransferToWindow(wxValidator * self);
bool wxValidator_Validate(wxValidator * self, wxWindow * parent);
void wxValidator_SuppressBellOnError(bool suppress);
bool wxValidator_IsSilent();

} // extern "C"

