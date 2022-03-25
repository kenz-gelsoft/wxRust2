#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use crate::macros::wx_class;

mod ffi {
    use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
    extern "C" {

        // wxObject
        pub fn wxObject_new() -> *mut c_void;
        pub fn wxObject_new1(other: *const c_void) -> *mut c_void;
        // DTOR: pub fn wxObject_~wxObject(self_: *mut c_void);
        pub fn wxObject_GetClassInfo(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_GetRefData(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
        pub fn wxObject_IsSameAs(self_: *const c_void, obj: *const c_void) -> bool;
        pub fn wxObject_Ref(self_: *mut c_void, clone: *const c_void);
        pub fn wxObject_SetRefData(self_: *mut c_void, data: *mut c_void);
        pub fn wxObject_UnRef(self_: *mut c_void);
        pub fn wxObject_UnShare(self_: *mut c_void);
        // BLOCKED: pub fn wxObject_operator delete(self_: *mut c_void, buf: *mut c_void);
        // NOT_SUPPORTED: pub fn wxObject_operator new(self_: *mut c_void, size: size_t, filename: *const c_void, line_num: c_int) -> *mut c_void;
        
        // wxEvtHandler
        pub fn wxEvtHandler_QueueEvent(self_: *mut c_void, event: *mut c_void);
        pub fn wxEvtHandler_AddPendingEvent(self_: *mut c_void, event: *const c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_CallAfter(self_: *mut c_void, method: *mut c_void, x1: T1, None: ...);
        // BLOCKED: pub fn wxEvtHandler_CallAfter1(self_: *mut c_void, functor: *const c_void);
        pub fn wxEvtHandler_ProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_SafelyProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessPendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_DeletePendingEvents(self_: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Connect(self_: *mut c_void, id: c_int, last_id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Connect1(self_: *mut c_void, id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Connect2(self_: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect(self_: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect1(self_: *mut c_void, id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect2(self_: *mut c_void, id: c_int, last_id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxEvtHandler_Bind(self_: *mut c_void, event_type: *const c_void, functor: Functor, id: c_int, last_id: c_int, user_data: *mut c_void);
        // BLOCKED: pub fn wxEvtHandler_Bind1(self_: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: c_int, last_id: c_int, user_data: *mut c_void);
        // NOT_SUPPORTED: pub fn wxEvtHandler_Unbind(self_: *mut c_void, event_type: *const c_void, functor: Functor, id: c_int, last_id: c_int, user_data: *mut c_void) -> bool;
        // BLOCKED: pub fn wxEvtHandler_Unbind1(self_: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: c_int, last_id: c_int, user_data: *mut c_void) -> bool;
        // BLOCKED: pub fn wxEvtHandler_GetClientData(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_GetClientObject(self_: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxEvtHandler_SetClientData(self_: *mut c_void, data: *mut c_void);
        pub fn wxEvtHandler_SetClientObject(self_: *mut c_void, data: *mut c_void);
        pub fn wxEvtHandler_GetEvtHandlerEnabled(self_: *const c_void) -> bool;
        pub fn wxEvtHandler_GetNextHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_GetPreviousHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxEvtHandler_SetEvtHandlerEnabled(self_: *mut c_void, enabled: bool);
        pub fn wxEvtHandler_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxEvtHandler_SetPreviousHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxEvtHandler_Unlink(self_: *mut c_void);
        pub fn wxEvtHandler_IsUnlinked(self_: *const c_void) -> bool;
        pub fn wxEvtHandler_AddFilter(filter: *mut c_void);
        pub fn wxEvtHandler_RemoveFilter(filter: *mut c_void);
        pub fn wxEvtHandler_new() -> *mut c_void;
        // DTOR: pub fn wxEvtHandler_~wxEvtHandler(self_: *mut c_void);
        
        // wxWindow
        pub fn wxWindow_AcceptsFocus(self_: *const c_void) -> bool;
        pub fn wxWindow_AcceptsFocusFromKeyboard(self_: *const c_void) -> bool;
        pub fn wxWindow_AcceptsFocusRecursively(self_: *const c_void) -> bool;
        pub fn wxWindow_DisableFocusFromKeyboard(self_: *mut c_void);
        pub fn wxWindow_IsFocusable(self_: *const c_void) -> bool;
        pub fn wxWindow_CanAcceptFocus(self_: *const c_void) -> bool;
        pub fn wxWindow_CanAcceptFocusFromKeyboard(self_: *const c_void) -> bool;
        pub fn wxWindow_HasFocus(self_: *const c_void) -> bool;
        pub fn wxWindow_SetCanFocus(self_: *mut c_void, can_focus: bool);
        pub fn wxWindow_EnableVisibleFocus(self_: *mut c_void, enable: bool);
        pub fn wxWindow_SetFocus(self_: *mut c_void);
        pub fn wxWindow_SetFocusFromKbd(self_: *mut c_void);
        pub fn wxWindow_AddChild(self_: *mut c_void, child: *mut c_void);
        pub fn wxWindow_DestroyChildren(self_: *mut c_void) -> bool;
        pub fn wxWindow_FindWindow(self_: *const c_void, id: c_long) -> *mut c_void;
        pub fn wxWindow_FindWindow1(self_: *const c_void, name: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxWindow_GetChildren(self_: *mut c_void) -> *mut c_void;
        // BLOCKED: pub fn wxWindow_GetChildren1(self_: *const c_void) -> *const c_void;
        pub fn wxWindow_RemoveChild(self_: *mut c_void, child: *mut c_void);
        pub fn wxWindow_GetGrandParent(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetNextSibling(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetParent(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetPrevSibling(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_IsDescendant(self_: *const c_void, win: *mut c_void) -> bool;
        pub fn wxWindow_Reparent(self_: *mut c_void, new_parent: *mut c_void) -> bool;
        pub fn wxWindow_AlwaysShowScrollbars(self_: *mut c_void, hflag: bool, vflag: bool);
        pub fn wxWindow_GetScrollPos(self_: *const c_void, orientation: c_int) -> c_int;
        pub fn wxWindow_GetScrollRange(self_: *const c_void, orientation: c_int) -> c_int;
        pub fn wxWindow_GetScrollThumb(self_: *const c_void, orientation: c_int) -> c_int;
        pub fn wxWindow_CanScroll(self_: *const c_void, orient: c_int) -> bool;
        pub fn wxWindow_HasScrollbar(self_: *const c_void, orient: c_int) -> bool;
        pub fn wxWindow_IsScrollbarAlwaysShown(self_: *const c_void, orient: c_int) -> bool;
        pub fn wxWindow_ScrollLines(self_: *mut c_void, lines: c_int) -> bool;
        pub fn wxWindow_ScrollPages(self_: *mut c_void, pages: c_int) -> bool;
        pub fn wxWindow_ScrollWindow(self_: *mut c_void, dx: c_int, dy: c_int, rect: *const c_void);
        pub fn wxWindow_LineUp(self_: *mut c_void) -> bool;
        pub fn wxWindow_LineDown(self_: *mut c_void) -> bool;
        pub fn wxWindow_PageUp(self_: *mut c_void) -> bool;
        pub fn wxWindow_PageDown(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetScrollPos(self_: *mut c_void, orientation: c_int, pos: c_int, refresh: bool);
        pub fn wxWindow_SetScrollbar(self_: *mut c_void, orientation: c_int, position: c_int, thumb_size: c_int, range: c_int, refresh: bool);
        pub fn wxWindow_BeginRepositioningChildren(self_: *mut c_void) -> bool;
        pub fn wxWindow_EndRepositioningChildren(self_: *mut c_void);
        pub fn wxWindow_CacheBestSize(self_: *const c_void, size: *const c_void);
        pub fn wxWindow_ClientToWindowSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxWindow_WindowToClientSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxWindow_Fit(self_: *mut c_void);
        pub fn wxWindow_FitInside(self_: *mut c_void);
        pub fn wxWindow_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP2(self_: *const c_void, d: c_int) -> c_int;
        pub fn wxWindow_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP2(self_: *const c_void, d: c_int) -> c_int;
        pub fn wxWindow_GetBestSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetBestHeight(self_: *const c_void, width: c_int) -> c_int;
        pub fn wxWindow_GetBestWidth(self_: *const c_void, height: c_int) -> c_int;
        pub fn wxWindow_GetClientSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetClientSize1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetEffectiveMinSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMaxClientSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMaxSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinClientSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinWidth(self_: *const c_void) -> c_int;
        pub fn wxWindow_GetMinHeight(self_: *const c_void) -> c_int;
        pub fn wxWindow_GetMaxWidth(self_: *const c_void) -> c_int;
        pub fn wxWindow_GetMaxHeight(self_: *const c_void) -> c_int;
        pub fn wxWindow_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetSize1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetVirtualSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetVirtualSize1(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetBestVirtualSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetContentScaleFactor(self_: *const c_void) -> c_double;
        pub fn wxWindow_GetDPIScaleFactor(self_: *const c_void) -> c_double;
        pub fn wxWindow_GetWindowBorderSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_InformFirstDirection(self_: *mut c_void, direction: c_int, size: c_int, available_other_dir: c_int) -> bool;
        pub fn wxWindow_InvalidateBestSize(self_: *mut c_void);
        pub fn wxWindow_PostSizeEvent(self_: *mut c_void);
        pub fn wxWindow_PostSizeEventToParent(self_: *mut c_void);
        pub fn wxWindow_SendSizeEvent(self_: *mut c_void, flags: c_int);
        pub fn wxWindow_SendSizeEventToParent(self_: *mut c_void, flags: c_int);
        pub fn wxWindow_SetClientSize(self_: *mut c_void, width: c_int, height: c_int);
        pub fn wxWindow_SetClientSize1(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetClientSize2(self_: *mut c_void, rect: *const c_void);
        pub fn wxWindow_SetContainingSizer(self_: *mut c_void, sizer: *mut c_void);
        pub fn wxWindow_SetInitialSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMaxClientSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMaxSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMinClientSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMinSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetSize(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int, size_flags: c_int);
        pub fn wxWindow_SetSize1(self_: *mut c_void, rect: *const c_void);
        pub fn wxWindow_SetSize2(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetSize3(self_: *mut c_void, width: c_int, height: c_int);
        pub fn wxWindow_SetSizeHints(self_: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        pub fn wxWindow_SetSizeHints1(self_: *mut c_void, min_w: c_int, min_h: c_int, max_w: c_int, max_h: c_int, inc_w: c_int, inc_h: c_int);
        pub fn wxWindow_SetVirtualSize(self_: *mut c_void, width: c_int, height: c_int);
        pub fn wxWindow_SetVirtualSize1(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_FromDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP5(d: c_int, w: *const c_void) -> c_int;
        pub fn wxWindow_ToDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP5(d: c_int, w: *const c_void) -> c_int;
        pub fn wxWindow_Center(self_: *mut c_void, dir: c_int);
        pub fn wxWindow_CenterOnParent(self_: *mut c_void, dir: c_int);
        pub fn wxWindow_Centre(self_: *mut c_void, direction: c_int);
        pub fn wxWindow_CentreOnParent(self_: *mut c_void, direction: c_int);
        pub fn wxWindow_GetPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_GetPosition1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetScreenPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_GetScreenPosition1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetScreenRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetClientAreaOrigin(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetClientRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_Move(self_: *mut c_void, x: c_int, y: c_int, flags: c_int);
        pub fn wxWindow_Move1(self_: *mut c_void, pt: *const c_void, flags: c_int);
        pub fn wxWindow_SetPosition(self_: *mut c_void, pt: *const c_void);
        pub fn wxWindow_ClientToScreen(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_ClientToScreen1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertDialogToPixels(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertDialogToPixels1(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertPixelsToDialog(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ConvertPixelsToDialog1(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ScreenToClient(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_ScreenToClient1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ClearBackground(self_: *mut c_void);
        pub fn wxWindow_Freeze(self_: *mut c_void);
        pub fn wxWindow_Thaw(self_: *mut c_void);
        pub fn wxWindow_IsFrozen(self_: *const c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxWindow_GetBackgroundColour(self_: *const c_void) -> wxColour;
        // NOT_SUPPORTED: pub fn wxWindow_GetBackgroundStyle(self_: *const c_void) -> wxBackgroundStyle;
        pub fn wxWindow_GetCharHeight(self_: *const c_void) -> c_int;
        pub fn wxWindow_GetCharWidth(self_: *const c_void) -> c_int;
        // NOT_SUPPORTED: pub fn wxWindow_GetDefaultAttributes(self_: *const c_void) -> wxVisualAttributes;
        pub fn wxWindow_GetDPI(self_: *const c_void) -> *mut c_void;
        // NOT_SUPPORTED: pub fn wxWindow_GetFont(self_: *const c_void) -> wxFont;
        // NOT_SUPPORTED: pub fn wxWindow_GetForegroundColour(self_: *const c_void) -> wxColour;
        pub fn wxWindow_GetTextExtent(self_: *const c_void, string: *const c_void, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void);
        pub fn wxWindow_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxWindow_GetUpdateRegion(self_: *const c_void) -> *const c_void;
        pub fn wxWindow_GetUpdateClientRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HasTransparentBackground(self_: *mut c_void) -> bool;
        pub fn wxWindow_Refresh(self_: *mut c_void, erase_background: bool, rect: *const c_void);
        pub fn wxWindow_RefreshRect(self_: *mut c_void, rect: *const c_void, erase_background: bool);
        pub fn wxWindow_Update(self_: *mut c_void);
        pub fn wxWindow_SetBackgroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxWindow_SetBackgroundStyle(self_: *mut c_void, style: wxBackgroundStyle) -> bool;
        pub fn wxWindow_IsTransparentBackgroundSupported(self_: *const c_void, reason: *mut c_void) -> bool;
        pub fn wxWindow_SetFont(self_: *mut c_void, font: *const c_void) -> bool;
        pub fn wxWindow_SetForegroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
        pub fn wxWindow_SetOwnBackgroundColour(self_: *mut c_void, colour: *const c_void);
        pub fn wxWindow_InheritsBackgroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_UseBgCol(self_: *const c_void) -> bool;
        pub fn wxWindow_UseBackgroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_SetOwnFont(self_: *mut c_void, font: *const c_void);
        pub fn wxWindow_SetOwnForegroundColour(self_: *mut c_void, colour: *const c_void);
        pub fn wxWindow_UseForegroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_InheritsForegroundColour(self_: *const c_void) -> bool;
        pub fn wxWindow_SetPalette(self_: *mut c_void, pal: *const c_void);
        pub fn wxWindow_ShouldInheritColours(self_: *const c_void) -> bool;
        pub fn wxWindow_SetThemeEnabled(self_: *mut c_void, enable: bool);
        pub fn wxWindow_GetThemeEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_CanSetTransparent(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetTransparent(self_: *mut c_void, alpha: c_uchar) -> bool;
        pub fn wxWindow_GetEventHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HandleAsNavigationKey(self_: *mut c_void, event: *const c_void) -> bool;
        pub fn wxWindow_HandleWindowEvent(self_: *const c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_ProcessWindowEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_ProcessWindowEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_PopEventHandler(self_: *mut c_void, delete_handler: bool) -> *mut c_void;
        pub fn wxWindow_PushEventHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_RemoveEventHandler(self_: *mut c_void, handler: *mut c_void) -> bool;
        pub fn wxWindow_SetEventHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_GetExtraStyle(self_: *const c_void) -> c_long;
        pub fn wxWindow_GetWindowStyleFlag(self_: *const c_void) -> c_long;
        pub fn wxWindow_GetWindowStyle(self_: *const c_void) -> c_long;
        pub fn wxWindow_HasExtraStyle(self_: *const c_void, ex_flag: c_int) -> bool;
        pub fn wxWindow_HasFlag(self_: *const c_void, flag: c_int) -> bool;
        pub fn wxWindow_SetExtraStyle(self_: *mut c_void, ex_style: c_long);
        pub fn wxWindow_SetWindowStyleFlag(self_: *mut c_void, style: c_long);
        pub fn wxWindow_SetWindowStyle(self_: *mut c_void, style: c_long);
        pub fn wxWindow_ToggleWindowStyle(self_: *mut c_void, flag: c_int) -> bool;
        pub fn wxWindow_MoveAfterInTabOrder(self_: *mut c_void, win: *mut c_void);
        pub fn wxWindow_MoveBeforeInTabOrder(self_: *mut c_void, win: *mut c_void);
        pub fn wxWindow_Navigate(self_: *mut c_void, flags: c_int) -> bool;
        pub fn wxWindow_NavigateIn(self_: *mut c_void, flags: c_int) -> bool;
        pub fn wxWindow_Lower(self_: *mut c_void);
        pub fn wxWindow_Raise(self_: *mut c_void);
        pub fn wxWindow_Hide(self_: *mut c_void) -> bool;
        // NOT_SUPPORTED: pub fn wxWindow_HideWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: unsigned int) -> bool;
        pub fn wxWindow_IsEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_IsExposed(self_: *const c_void, x: c_int, y: c_int) -> bool;
        pub fn wxWindow_IsExposed1(self_: *const c_void, pt: *mut c_void) -> bool;
        pub fn wxWindow_IsExposed2(self_: *const c_void, x: c_int, y: c_int, w: c_int, h: c_int) -> bool;
        pub fn wxWindow_IsExposed3(self_: *const c_void, rect: *mut c_void) -> bool;
        pub fn wxWindow_IsShown(self_: *const c_void) -> bool;
        pub fn wxWindow_IsShownOnScreen(self_: *const c_void) -> bool;
        pub fn wxWindow_Disable(self_: *mut c_void) -> bool;
        pub fn wxWindow_Enable(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxWindow_Show(self_: *mut c_void, show: bool) -> bool;
        // NOT_SUPPORTED: pub fn wxWindow_ShowWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: unsigned int) -> bool;
        pub fn wxWindow_GetHelpText(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetHelpText(self_: *mut c_void, help_text: *const c_void);
        // NOT_SUPPORTED: pub fn wxWindow_GetHelpTextAtPoint(self_: *const c_void, point: *const c_void, origin: wxHelpEvent::Origin) -> *mut c_void;
        pub fn wxWindow_GetToolTip(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetToolTipText(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetToolTip(self_: *mut c_void, tip_string: *const c_void);
        pub fn wxWindow_SetToolTip1(self_: *mut c_void, tip: *mut c_void);
        pub fn wxWindow_UnsetToolTip(self_: *mut c_void);
        pub fn wxWindow_GetPopupMenuSelectionFromUser(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> c_int;
        pub fn wxWindow_GetPopupMenuSelectionFromUser1(self_: *mut c_void, menu: *mut c_void, x: c_int, y: c_int) -> c_int;
        pub fn wxWindow_PopupMenu(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> bool;
        pub fn wxWindow_PopupMenu1(self_: *mut c_void, menu: *mut c_void, x: c_int, y: c_int) -> bool;
        pub fn wxWindow_GetValidator(self_: *mut c_void) -> *mut c_void;
        pub fn wxWindow_SetValidator(self_: *mut c_void, validator: *const c_void);
        pub fn wxWindow_TransferDataFromWindow(self_: *mut c_void) -> bool;
        pub fn wxWindow_TransferDataToWindow(self_: *mut c_void) -> bool;
        pub fn wxWindow_Validate(self_: *mut c_void) -> bool;
        pub fn wxWindow_GetId(self_: *const c_void) -> c_int;
        pub fn wxWindow_GetLabel(self_: *const c_void) -> *mut c_void;
        // NOT_SUPPORTED: pub fn wxWindow_GetLayoutDirection(self_: *const c_void) -> wxLayoutDirection;
        pub fn wxWindow_AdjustForLayoutDirection(self_: *const c_void, x: c_int, width: c_int, width_total: c_int) -> c_int;
        pub fn wxWindow_GetName(self_: *const c_void) -> *mut c_void;
        // NOT_SUPPORTED: pub fn wxWindow_GetWindowVariant(self_: *const c_void) -> wxWindowVariant;
        pub fn wxWindow_SetId(self_: *mut c_void, winid: c_int);
        pub fn wxWindow_SetLabel(self_: *mut c_void, label: *const c_void);
        // NOT_SUPPORTED: pub fn wxWindow_SetLayoutDirection(self_: *mut c_void, dir: wxLayoutDirection);
        pub fn wxWindow_SetName(self_: *mut c_void, name: *const c_void);
        // NOT_SUPPORTED: pub fn wxWindow_SetWindowVariant(self_: *mut c_void, variant: wxWindowVariant);
        pub fn wxWindow_GetAcceleratorTable(self_: *mut c_void) -> *mut c_void;
        // NOT_SUPPORTED: pub fn wxWindow_GetAccessible(self_: *mut c_void) -> *mut c_void;
        pub fn wxWindow_SetAcceleratorTable(self_: *mut c_void, accel: *const c_void);
        // NOT_SUPPORTED: pub fn wxWindow_SetAccessible(self_: *mut c_void, accessible: *mut c_void);
        pub fn wxWindow_Close(self_: *mut c_void, force: bool) -> bool;
        pub fn wxWindow_Destroy(self_: *mut c_void) -> bool;
        pub fn wxWindow_IsBeingDeleted(self_: *const c_void) -> bool;
        pub fn wxWindow_GetDropTarget(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetDropTarget(self_: *mut c_void, target: *mut c_void);
        pub fn wxWindow_DragAcceptFiles(self_: *mut c_void, accept: bool);
        pub fn wxWindow_GetContainingSizer(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetSizer(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetSizer(self_: *mut c_void, sizer: *mut c_void, delete_old: bool);
        pub fn wxWindow_SetSizerAndFit(self_: *mut c_void, sizer: *mut c_void, delete_old: bool);
        pub fn wxWindow_GetConstraints(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetConstraints(self_: *mut c_void, constraints: *mut c_void);
        pub fn wxWindow_Layout(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetAutoLayout(self_: *mut c_void, auto_layout: bool);
        pub fn wxWindow_GetAutoLayout(self_: *const c_void) -> bool;
        pub fn wxWindow_CaptureMouse(self_: *mut c_void);
        pub fn wxWindow_GetCaret(self_: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxWindow_GetCursor(self_: *const c_void) -> *const c_void;
        pub fn wxWindow_HasCapture(self_: *const c_void) -> bool;
        pub fn wxWindow_ReleaseMouse(self_: *mut c_void);
        pub fn wxWindow_SetCaret(self_: *mut c_void, caret: *mut c_void);
        pub fn wxWindow_SetCursor(self_: *mut c_void, cursor: *const c_void) -> bool;
        pub fn wxWindow_WarpPointer(self_: *mut c_void, x: c_int, y: c_int);
        pub fn wxWindow_EnableTouchEvents(self_: *mut c_void, events_mask: c_int) -> bool;
        // NOT_SUPPORTED: pub fn wxWindow_HitTest(self_: *const c_void, x: c_int, y: c_int) -> wxHitTest;
        // NOT_SUPPORTED: pub fn wxWindow_HitTest1(self_: *const c_void, pt: *const c_void) -> wxHitTest;
        // NOT_SUPPORTED: pub fn wxWindow_GetBorder(self_: *const c_void, flags: c_long) -> wxBorder;
        // NOT_SUPPORTED: pub fn wxWindow_GetBorder1(self_: *const c_void) -> wxBorder;
        pub fn wxWindow_DoUpdateWindowUI(self_: *mut c_void, event: *mut c_void);
        // NOT_SUPPORTED: pub fn wxWindow_GetHandle(self_: *const c_void) -> WXWidget;
        pub fn wxWindow_HasMultiplePages(self_: *const c_void) -> bool;
        pub fn wxWindow_InheritAttributes(self_: *mut c_void);
        pub fn wxWindow_InitDialog(self_: *mut c_void);
        pub fn wxWindow_IsDoubleBuffered(self_: *const c_void) -> bool;
        pub fn wxWindow_SetDoubleBuffered(self_: *mut c_void, on: bool);
        pub fn wxWindow_IsRetained(self_: *const c_void) -> bool;
        pub fn wxWindow_IsThisEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_IsTopLevel(self_: *const c_void) -> bool;
        pub fn wxWindow_OnInternalIdle(self_: *mut c_void);
        pub fn wxWindow_SendIdleEvents(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_RegisterHotKey(self_: *mut c_void, hotkey_id: c_int, modifiers: c_int, virtual_key_code: c_int) -> bool;
        pub fn wxWindow_UnregisterHotKey(self_: *mut c_void, hotkey_id: c_int) -> bool;
        pub fn wxWindow_UpdateWindowUI(self_: *mut c_void, flags: c_long);
        // NOT_SUPPORTED: pub fn wxWindow_GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        pub fn wxWindow_FindFocus() -> *mut c_void;
        pub fn wxWindow_FindWindowById(id: c_long, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_FindWindowByLabel(label: *const c_void, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_FindWindowByName(name: *const c_void, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetCapture() -> *mut c_void;
        pub fn wxWindow_NewControlId(count: c_int) -> c_int;
        pub fn wxWindow_UnreserveControlId(id: c_int, count: c_int);
        pub fn wxWindow_new() -> *mut c_void;
        pub fn wxWindow_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
        // DTOR: pub fn wxWindow_~wxWindow(self_: *mut c_void);
        pub fn wxWindow_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;
        
        // wxControl
        pub fn wxControl_new(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxControl_new1() -> *mut c_void;
        pub fn wxControl_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
        pub fn wxControl_Command(self_: *mut c_void, event: *mut c_void);
        pub fn wxControl_GetLabelText(self_: *const c_void) -> *mut c_void;
        pub fn wxControl_GetSizeFromTextSize(self_: *const c_void, xlen: c_int, ylen: c_int) -> *mut c_void;
        pub fn wxControl_GetSizeFromTextSize1(self_: *const c_void, tsize: *const c_void) -> *mut c_void;
        pub fn wxControl_GetSizeFromText(self_: *const c_void, text: *const c_void) -> *mut c_void;
        pub fn wxControl_SetLabelText(self_: *mut c_void, text: *const c_void);
        pub fn wxControl_SetLabelMarkup(self_: *mut c_void, markup: *const c_void) -> bool;
        pub fn wxControl_GetLabelText1(label: *const c_void) -> *mut c_void;
        pub fn wxControl_RemoveMnemonics(str: *const c_void) -> *mut c_void;
        pub fn wxControl_EscapeMnemonics(text: *const c_void) -> *mut c_void;
        pub fn wxControl_Ellipsize(label: *const c_void, dc: *const c_void, mode: c_int, max_width: c_int, flags: c_int) -> *mut c_void;
        
        // wxAnyButton
        pub fn wxAnyButton_new() -> *mut c_void;
        // DTOR: pub fn wxAnyButton_~wxAnyButton(self_: *mut c_void);
        // NOT_SUPPORTED: pub fn wxAnyButton_GetBitmap(self_: *const c_void) -> wxBitmap;
        // NOT_SUPPORTED: pub fn wxAnyButton_GetBitmapCurrent(self_: *const c_void) -> wxBitmap;
        // NOT_SUPPORTED: pub fn wxAnyButton_GetBitmapDisabled(self_: *const c_void) -> wxBitmap;
        // NOT_SUPPORTED: pub fn wxAnyButton_GetBitmapFocus(self_: *const c_void) -> wxBitmap;
        // NOT_SUPPORTED: pub fn wxAnyButton_GetBitmapLabel(self_: *const c_void) -> wxBitmap;
        // NOT_SUPPORTED: pub fn wxAnyButton_GetBitmapPressed(self_: *const c_void) -> wxBitmap;
        // NOT_SUPPORTED: pub fn wxAnyButton_SetBitmap(self_: *mut c_void, bitmap: *const c_void, dir: wxDirection);
        pub fn wxAnyButton_SetBitmapCurrent(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapDisabled(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapFocus(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapLabel(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapPressed(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_GetBitmapMargins(self_: *mut c_void) -> *mut c_void;
        pub fn wxAnyButton_SetBitmapMargins(self_: *mut c_void, x: c_int, y: c_int);
        pub fn wxAnyButton_SetBitmapMargins1(self_: *mut c_void, sz: *const c_void);
        // NOT_SUPPORTED: pub fn wxAnyButton_SetBitmapPosition(self_: *mut c_void, dir: wxDirection);
        
        // wxButton
        pub fn wxButton_new() -> *mut c_void;
        pub fn wxButton_new1(parent: *mut c_void, id: c_int, label: *const c_void, pos: *const c_void, size: *const c_void, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxButton_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, label: *const c_void, pos: *const c_void, size: *const c_void, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
        pub fn wxButton_GetAuthNeeded(self_: *const c_void) -> bool;
        pub fn wxButton_SetAuthNeeded(self_: *mut c_void, needed: bool);
        pub fn wxButton_SetDefault(self_: *mut c_void) -> *mut c_void;
        pub fn wxButton_GetDefaultSize(win: *mut c_void) -> *mut c_void;
        
        // wxNonOwnedWindow
        pub fn wxNonOwnedWindow_SetShape(self_: *mut c_void, region: *const c_void) -> bool;
        pub fn wxNonOwnedWindow_SetShape1(self_: *mut c_void, path: *const c_void) -> bool;
        
        // wxTopLevelWindow
        pub fn wxTopLevelWindow_new() -> *mut c_void;
        pub fn wxTopLevelWindow_new1(parent: *mut c_void, id: c_int, title: *const c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
        // DTOR: pub fn wxTopLevelWindow_~wxTopLevelWindow(self_: *mut c_void);
        pub fn wxTopLevelWindow_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, title: *const c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;
        pub fn wxTopLevelWindow_CenterOnScreen(self_: *mut c_void, direction: c_int);
        pub fn wxTopLevelWindow_CentreOnScreen(self_: *mut c_void, direction: c_int);
        pub fn wxTopLevelWindow_EnableCloseButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_GetDefaultItem(self_: *const c_void) -> *mut c_void;
        // NOT_SUPPORTED: pub fn wxTopLevelWindow_GetIcon(self_: *const c_void) -> wxIcon;
        // BLOCKED: pub fn wxTopLevelWindow_GetIcons(self_: *const c_void) -> *const c_void;
        pub fn wxTopLevelWindow_GetTitle(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_Iconize(self_: *mut c_void, iconize: bool);
        pub fn wxTopLevelWindow_IsActive(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsFullScreen(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsIconized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsMaximized(self_: *const c_void) -> bool;
        // BLOCKED: pub fn wxTopLevelWindow_IsUsingNativeDecorations(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_Maximize(self_: *mut c_void, maximize: bool);
        // BLOCKED: pub fn wxTopLevelWindow_MSWGetSystemMenu(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_RequestUserAttention(self_: *mut c_void, flags: c_int);
        pub fn wxTopLevelWindow_Restore(self_: *mut c_void);
        // BLOCKED: pub fn wxTopLevelWindow_RestoreToGeometry(self_: *mut c_void, ser: *mut c_void) -> bool;
        // BLOCKED: pub fn wxTopLevelWindow_SaveGeometry(self_: *const c_void, ser: *const c_void) -> bool;
        pub fn wxTopLevelWindow_SetDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_SetIcon(self_: *mut c_void, icon: *const c_void);
        pub fn wxTopLevelWindow_SetIcons(self_: *mut c_void, icons: *const c_void);
        pub fn wxTopLevelWindow_SetTitle(self_: *mut c_void, title: *const c_void);
        pub fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_OSXSetModified(self_: *mut c_void, modified: bool);
        pub fn wxTopLevelWindow_OSXIsModified(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut c_void, filename: *const c_void);
        pub fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut c_void);
        pub fn wxTopLevelWindow_EnableFullScreenView(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_ShowFullScreen(self_: *mut c_void, show: bool, style: c_long) -> bool;
        // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorations(self_: *mut c_void, native: bool);
        // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorationsByDefault(self_: *mut c_void, native: bool);
        pub fn wxTopLevelWindow_GetDefaultSize() -> *mut c_void;
        
        // wxFrame
        pub fn wxFrame_new() -> *mut c_void;
        pub fn wxFrame_new1(parent: *mut c_void, id: c_int, title: *const c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
        // DTOR: pub fn wxFrame_~wxFrame(self_: *mut c_void);
        pub fn wxFrame_Centre(self_: *mut c_void, direction: c_int);
        pub fn wxFrame_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, title: *const c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;
        pub fn wxFrame_CreateStatusBar(self_: *mut c_void, number: c_int, style: c_long, id: c_int, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_CreateToolBar(self_: *mut c_void, style: c_long, id: c_int, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_DoGiveHelp(self_: *mut c_void, text: *const c_void, show: bool);
        pub fn wxFrame_GetMenuBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetStatusBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetStatusBarPane(self_: *const c_void) -> c_int;
        pub fn wxFrame_GetToolBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_OnCreateStatusBar(self_: *mut c_void, number: c_int, style: c_long, id: c_int, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_OnCreateToolBar(self_: *mut c_void, style: c_long, id: c_int, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_ProcessCommand(self_: *mut c_void, id: c_int) -> bool;
        pub fn wxFrame_SetMenuBar(self_: *mut c_void, menu_bar: *mut c_void);
        pub fn wxFrame_SetStatusBar(self_: *mut c_void, status_bar: *mut c_void);
        pub fn wxFrame_SetStatusBarPane(self_: *mut c_void, n: c_int);
        pub fn wxFrame_SetStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
        pub fn wxFrame_SetStatusWidths(self_: *mut c_void, n: c_int, widths_field: *const c_void);
        pub fn wxFrame_SetToolBar(self_: *mut c_void, tool_bar: *mut c_void);
        // BLOCKED: pub fn wxFrame_MSWGetTaskBarButton(self_: *mut c_void) -> *mut c_void;
        pub fn wxFrame_PushStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
        pub fn wxFrame_PopStatusText(self_: *mut c_void, number: c_int);
        
        // wxPoint
        pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
        pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
        // BLOCKED: pub fn wxPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
        // BLOCKED: pub fn wxPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
        // BLOCKED: pub fn wxPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
        // BLOCKED: pub fn wxPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
        // BLOCKED: pub fn wxPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
        // BLOCKED: pub fn wxPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
        // BLOCKED: pub fn wxPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
        // BLOCKED: pub fn wxPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
        // BLOCKED: pub fn wxPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
        // BLOCKED: pub fn wxPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
        // BLOCKED: pub fn wxPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
        // BLOCKED: pub fn wxPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
        // BLOCKED: pub fn wxPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
        pub fn wxPoint_new() -> *mut c_void;
        pub fn wxPoint_new1(x: c_int, y: c_int) -> *mut c_void;
        pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;
        
        // wxRect
        pub fn wxRect_new() -> *mut c_void;
        pub fn wxRect_new1(x: c_int, y: c_int, width: c_int, height: c_int) -> *mut c_void;
        pub fn wxRect_new2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
        pub fn wxRect_new3(pos: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxRect_new4(size: *const c_void) -> *mut c_void;
        pub fn wxRect_CentreIn(self_: *const c_void, r: *const c_void, dir: c_int) -> *mut c_void;
        pub fn wxRect_CenterIn(self_: *const c_void, r: *const c_void, dir: c_int) -> *mut c_void;
        pub fn wxRect_Contains(self_: *const c_void, x: c_int, y: c_int) -> bool;
        pub fn wxRect_Contains1(self_: *const c_void, pt: *const c_void) -> bool;
        pub fn wxRect_Contains2(self_: *const c_void, rect: *const c_void) -> bool;
        // BLOCKED: pub fn wxRect_Deflate(self_: *mut c_void, dx: c_int, dy: c_int) -> *mut c_void;
        // BLOCKED: pub fn wxRect_Deflate1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_Deflate2(self_: *mut c_void, diff: c_int) -> *mut c_void;
        pub fn wxRect_Deflate3(self_: *const c_void, dx: c_int, dy: c_int) -> *mut c_void;
        pub fn wxRect_GetBottom(self_: *const c_void) -> c_int;
        pub fn wxRect_GetBottomLeft(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetBottomRight(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetHeight(self_: *const c_void) -> c_int;
        pub fn wxRect_GetLeft(self_: *const c_void) -> c_int;
        pub fn wxRect_GetPosition(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetRight(self_: *const c_void) -> c_int;
        pub fn wxRect_GetSize(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetTop(self_: *const c_void) -> c_int;
        pub fn wxRect_GetTopLeft(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetTopRight(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetWidth(self_: *const c_void) -> c_int;
        pub fn wxRect_GetX(self_: *const c_void) -> c_int;
        pub fn wxRect_GetY(self_: *const c_void) -> c_int;
        // BLOCKED: pub fn wxRect_Inflate(self_: *mut c_void, dx: c_int, dy: c_int) -> *mut c_void;
        // BLOCKED: pub fn wxRect_Inflate1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_Inflate2(self_: *mut c_void, diff: c_int) -> *mut c_void;
        pub fn wxRect_Inflate3(self_: *const c_void, dx: c_int, dy: c_int) -> *mut c_void;
        // BLOCKED: pub fn wxRect_Intersect(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
        pub fn wxRect_Intersect1(self_: *const c_void, rect: *const c_void) -> *mut c_void;
        pub fn wxRect_Intersects(self_: *const c_void, rect: *const c_void) -> bool;
        pub fn wxRect_IsEmpty(self_: *const c_void) -> bool;
        pub fn wxRect_Offset(self_: *mut c_void, dx: c_int, dy: c_int);
        pub fn wxRect_Offset1(self_: *mut c_void, pt: *const c_void);
        pub fn wxRect_SetHeight(self_: *mut c_void, height: c_int);
        pub fn wxRect_SetPosition(self_: *mut c_void, pos: *const c_void);
        pub fn wxRect_SetSize(self_: *mut c_void, s: *const c_void);
        pub fn wxRect_SetWidth(self_: *mut c_void, width: c_int);
        pub fn wxRect_SetX(self_: *mut c_void, x: c_int);
        pub fn wxRect_SetY(self_: *mut c_void, y: c_int);
        pub fn wxRect_SetLeft(self_: *mut c_void, left: c_int);
        pub fn wxRect_SetRight(self_: *mut c_void, right: c_int);
        pub fn wxRect_SetTop(self_: *mut c_void, top: c_int);
        pub fn wxRect_SetBottom(self_: *mut c_void, bottom: c_int);
        pub fn wxRect_SetTopLeft(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetBottomRight(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetTopRight(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetBottomLeft(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_Union(self_: *const c_void, rect: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_Union1(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_operator!=(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;
        // BLOCKED: pub fn wxRect_operator+(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
        // BLOCKED: pub fn wxRect_operator+=(self_: *mut c_void, r: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_operator*(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
        // BLOCKED: pub fn wxRect_operator*=(self_: *mut c_void, r: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_operator=(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxRect_operator==(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;
        
        // wxSize
        // BLOCKED: pub fn wxSize_operator=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxSize_operator==(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
        // BLOCKED: pub fn wxSize_operator!=(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
        // BLOCKED: pub fn wxSize_operator+(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
        // BLOCKED: pub fn wxSize_operator-(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
        // BLOCKED: pub fn wxSize_operator+=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxSize_operator-=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn wxSize_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
        // BLOCKED: pub fn wxSize_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
        // BLOCKED: pub fn wxSize_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
        // BLOCKED: pub fn wxSize_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
        // BLOCKED: pub fn wxSize_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
        pub fn wxSize_new() -> *mut c_void;
        pub fn wxSize_new1(width: c_int, height: c_int) -> *mut c_void;
        pub fn wxSize_DecBy(self_: *mut c_void, pt: *const c_void);
        pub fn wxSize_DecBy1(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_DecBy2(self_: *mut c_void, dx: c_int, dy: c_int);
        pub fn wxSize_DecBy3(self_: *mut c_void, d: c_int);
        pub fn wxSize_DecTo(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_DecToIfSpecified(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_GetHeight(self_: *const c_void) -> c_int;
        pub fn wxSize_GetWidth(self_: *const c_void) -> c_int;
        pub fn wxSize_IncBy(self_: *mut c_void, pt: *const c_void);
        pub fn wxSize_IncBy1(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_IncBy2(self_: *mut c_void, dx: c_int, dy: c_int);
        pub fn wxSize_IncBy3(self_: *mut c_void, d: c_int);
        pub fn wxSize_IncTo(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_IsFullySpecified(self_: *const c_void) -> bool;
        // BLOCKED: pub fn wxSize_Scale(self_: *mut c_void, xscale: c_double, yscale: c_double) -> *mut c_void;
        pub fn wxSize_Set(self_: *mut c_void, width: c_int, height: c_int);
        pub fn wxSize_SetDefaults(self_: *mut c_void, size_default: *const c_void);
        pub fn wxSize_SetHeight(self_: *mut c_void, height: c_int);
        pub fn wxSize_SetWidth(self_: *mut c_void, width: c_int);
        
        // wxValidator
        pub fn wxValidator_new() -> *mut c_void;
        // DTOR: pub fn wxValidator_~wxValidator(self_: *mut c_void);
        pub fn wxValidator_Clone(self_: *const c_void) -> *mut c_void;
        pub fn wxValidator_GetWindow(self_: *const c_void) -> *mut c_void;
        pub fn wxValidator_SetWindow(self_: *mut c_void, window: *mut c_void);
        pub fn wxValidator_TransferFromWindow(self_: *mut c_void) -> bool;
        pub fn wxValidator_TransferToWindow(self_: *mut c_void) -> bool;
        pub fn wxValidator_Validate(self_: *mut c_void, parent: *mut c_void) -> bool;
        pub fn wxValidator_SuppressBellOnError(suppress: bool);
        pub fn wxValidator_IsSilent() -> bool;
        
    }
}

pub trait WxRustMethods {
    unsafe fn as_ptr(&self) -> *mut c_void;
}

// wxObject
wx_class! { Object(wxObject) impl
    ObjectMethods
}
impl Object {
    pub fn new() -> Object {
        unsafe { Object(ffi::wxObject_new()) }
    }
    pub fn new1(other: &Object) -> Object {
        unsafe {
            let other = other.as_ptr();
            Object(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetClassInfo(self.as_ptr()) }
    }
    fn get_ref_data(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetRefData(self.as_ptr()) }
    }
    fn is_kind_of(&self, info: *const c_void) -> bool {
        unsafe { ffi::wxObject_IsKindOf(self.as_ptr(), info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    fn ref_(&self, clone: &Object) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    fn set_ref_data(&self, data: *mut c_void) {
        unsafe { ffi::wxObject_SetRefData(self.as_ptr(), data) }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // NOT_SUPPORTED: fn operator new()
}

// wxEvtHandler
wx_class! { EvtHandler(wxEvtHandler) impl
    EvtHandlerMethods,
    ObjectMethods
}
impl EvtHandler {
    pub fn new() -> EvtHandler {
        unsafe { EvtHandler(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event(&self, event: *mut c_void) {
        unsafe { ffi::wxEvtHandler_QueueEvent(self.as_ptr(), event) }
    }
    fn add_pending_event(&self, event: *const c_void) {
        unsafe { ffi::wxEvtHandler_AddPendingEvent(self.as_ptr(), event) }
    }
    // NOT_SUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEvent(self.as_ptr(), event) }
    }
    fn process_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr(), event) }
    }
    fn safely_process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr(), event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr()) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Connect()
    // NOT_SUPPORTED: fn Connect1()
    // NOT_SUPPORTED: fn Connect2()
    // NOT_SUPPORTED: fn Disconnect()
    // NOT_SUPPORTED: fn Disconnect1()
    // NOT_SUPPORTED: fn Disconnect2()
    // NOT_SUPPORTED: fn Bind()
    // BLOCKED: fn Bind1()
    // NOT_SUPPORTED: fn Unbind()
    // BLOCKED: fn Unbind1()
    // BLOCKED: fn GetClientData()
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetClientObject(self.as_ptr()) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut c_void) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.as_ptr(), data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr()) }
    }
    fn get_next_handler(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetNextHandler(self.as_ptr()) }
    }
    fn get_previous_handler(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr()) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr(), enabled) }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr(), handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.as_ptr(), handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr()) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr()) }
    }
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_RemoveFilter(filter) }
    }
    // DTOR: fn ~wxEvtHandler()
}

// wxWindow
wx_class! { Window(wxWindow) impl
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Window {
    pub fn new() -> Window {
        unsafe { Window(ffi::wxWindow_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: c_int, pos: &Point, size: &Size, style: c_long, name: &str) -> Window {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = crate::wx_string_from(name);
            Window(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr()) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr()) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr()) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr()) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr()) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr()) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr()) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr()) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr(), can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr(), enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr()) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr()) }
    }
    fn add_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.as_ptr(), child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr()) }
    }
    fn find_window(&self, id: c_long) -> *mut c_void {
        unsafe { ffi::wxWindow_FindWindow(self.as_ptr(), id) }
    }
    fn find_window1(&self, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxWindow_FindWindow1(self.as_ptr(), name)
        }
    }
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren1()
    fn remove_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.as_ptr(), child)
        }
    }
    fn get_grand_parent(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetGrandParent(self.as_ptr()) }
    }
    fn get_next_sibling(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetNextSibling(self.as_ptr()) }
    }
    fn get_parent(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetParent(self.as_ptr()) }
    }
    fn get_prev_sibling(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetPrevSibling(self.as_ptr()) }
    }
    fn is_descendant<T: WindowMethods>(&self, win: Option<&T>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(self.as_ptr(), win)
        }
    }
    fn reparent<T: WindowMethods>(&self, new_parent: Option<&T>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.as_ptr(), new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr(), hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr(), orientation) }
    }
    fn get_scroll_range(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr(), orientation) }
    }
    fn get_scroll_thumb(&self, orientation: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr(), orientation) }
    }
    fn can_scroll(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr(), orient) }
    }
    fn has_scrollbar(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr(), orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: c_int) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr(), orient) }
    }
    fn scroll_lines(&self, lines: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr(), lines) }
    }
    fn scroll_pages(&self, pages: c_int) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr(), pages) }
    }
    fn scroll_window<T: RectMethods>(&self, dx: c_int, dy: c_int, rect: Option<&T>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ScrollWindow(self.as_ptr(), dx, dy, rect)
        }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr()) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr()) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr()) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr()) }
    }
    fn set_scroll_pos(&self, orientation: c_int, pos: c_int, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr(), orientation, pos, refresh) }
    }
    fn set_scrollbar(&self, orientation: c_int, position: c_int, thumb_size: c_int, range: c_int, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollbar(self.as_ptr(), orientation, position, thumb_size, range, refresh) }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr()) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr()) }
    }
    fn cache_best_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_CacheBestSize(self.as_ptr(), size)
        }
    }
    fn client_to_window_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size(ffi::wxWindow_ClientToWindowSize(self.as_ptr(), size))
        }
    }
    fn window_to_client_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr();
            Size(ffi::wxWindow_WindowToClientSize(self.as_ptr(), size))
        }
    }
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr()) }
    }
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr()) }
    }
    fn from_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size(ffi::wxWindow_FromDIP(self.as_ptr(), sz))
        }
    }
    fn from_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point(ffi::wxWindow_FromDIP1(self.as_ptr(), pt))
        }
    }
    fn from_dip2(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr(), d) }
    }
    fn to_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size(ffi::wxWindow_ToDIP(self.as_ptr(), sz))
        }
    }
    fn to_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point(ffi::wxWindow_ToDIP1(self.as_ptr(), pt))
        }
    }
    fn to_dip2(&self, d: c_int) -> c_int {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr(), d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestSize(self.as_ptr())) }
    }
    fn get_best_height(&self, width: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr(), width) }
    }
    fn get_best_width(&self, height: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr(), height) }
    }
    fn get_client_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr(), width, height) }
    }
    fn get_client_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetClientSize1(self.as_ptr())) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr())) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxClientSize(self.as_ptr())) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxSize(self.as_ptr())) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinClientSize(self.as_ptr())) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinSize(self.as_ptr())) }
    }
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr()) }
    }
    fn get_min_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr()) }
    }
    fn get_max_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr()) }
    }
    fn get_max_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr()) }
    }
    fn get_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetSize1(self.as_ptr())) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetVirtualSize(self.as_ptr())) }
    }
    fn get_virtual_size1(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr(), width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestVirtualSize(self.as_ptr())) }
    }
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr()) }
    }
    fn get_dpi_scale_factor(&self) -> c_double {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr()) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetWindowBorderSize(self.as_ptr())) }
    }
    fn inform_first_direction(&self, direction: c_int, size: c_int, available_other_dir: c_int) -> bool {
        unsafe { ffi::wxWindow_InformFirstDirection(self.as_ptr(), direction, size, available_other_dir) }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr()) }
    }
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr()) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr()) }
    }
    fn send_size_event(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr(), flags) }
    }
    fn send_size_event_to_parent(&self, flags: c_int) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr(), flags) }
    }
    fn set_client_size(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr(), width, height) }
    }
    fn set_client_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetClientSize1(self.as_ptr(), size)
        }
    }
    fn set_client_size2(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetClientSize2(self.as_ptr(), rect)
        }
    }
    fn set_containing_sizer(&self, sizer: *mut c_void) {
        unsafe { ffi::wxWindow_SetContainingSizer(self.as_ptr(), sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetInitialSize(self.as_ptr(), size)
        }
    }
    fn set_max_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxClientSize(self.as_ptr(), size)
        }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMaxSize(self.as_ptr(), size)
        }
    }
    fn set_min_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinClientSize(self.as_ptr(), size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetMinSize(self.as_ptr(), size)
        }
    }
    fn set_size(&self, x: c_int, y: c_int, width: c_int, height: c_int, size_flags: c_int) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr(), x, y, width, height, size_flags) }
    }
    fn set_size1(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_SetSize1(self.as_ptr(), rect)
        }
    }
    fn set_size2(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetSize2(self.as_ptr(), size)
        }
    }
    fn set_size3(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr(), width, height) }
    }
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr();
            let max_size = max_size.as_ptr();
            let inc_size = inc_size.as_ptr();
            ffi::wxWindow_SetSizeHints(self.as_ptr(), min_size, max_size, inc_size)
        }
    }
    fn set_size_hints1(&self, min_w: c_int, min_h: c_int, max_w: c_int, max_h: c_int, inc_w: c_int, inc_h: c_int) {
        unsafe { ffi::wxWindow_SetSizeHints1(self.as_ptr(), min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_virtual_size(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr(), width, height) }
    }
    fn set_virtual_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxWindow_SetVirtualSize1(self.as_ptr(), size)
        }
    }
    fn from_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip5<T: WindowMethods>(d: c_int, w: Option<&T>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip5<T: WindowMethods>(d: c_int, w: Option<&T>) -> c_int {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    fn center(&self, dir: c_int) {
        unsafe { ffi::wxWindow_Center(self.as_ptr(), dir) }
    }
    fn center_on_parent(&self, dir: c_int) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr(), dir) }
    }
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr(), direction) }
    }
    fn centre_on_parent(&self, direction: c_int) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr(), direction) }
    }
    fn get_position(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr(), x, y) }
    }
    fn get_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetPosition1(self.as_ptr())) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetRect(self.as_ptr())) }
    }
    fn get_screen_position(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr(), x, y) }
    }
    fn get_screen_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetScreenPosition1(self.as_ptr())) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetScreenRect(self.as_ptr())) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr())) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetClientRect(self.as_ptr())) }
    }
    fn move_(&self, x: c_int, y: c_int, flags: c_int) {
        unsafe { ffi::wxWindow_Move(self.as_ptr(), x, y, flags) }
    }
    fn move1(&self, pt: &Point, flags: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_Move1(self.as_ptr(), pt, flags)
        }
    }
    fn set_position(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxWindow_SetPosition(self.as_ptr(), pt)
        }
    }
    fn client_to_screen(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr(), x, y) }
    }
    fn client_to_screen1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point(ffi::wxWindow_ClientToScreen1(self.as_ptr(), pt))
        }
    }
    fn convert_dialog_to_pixels(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr(), pt))
        }
    }
    fn convert_dialog_to_pixels1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr(), sz))
        }
    }
    fn convert_pixels_to_dialog(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr(), pt))
        }
    }
    fn convert_pixels_to_dialog1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr(), sz))
        }
    }
    fn screen_to_client(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr(), x, y) }
    }
    fn screen_to_client1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point(ffi::wxWindow_ScreenToClient1(self.as_ptr(), pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr()) }
    }
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr()) }
    }
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr()) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetBackgroundColour()
    // NOT_SUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr()) }
    }
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetDPI(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetFont()
    // NOT_SUPPORTED: fn GetForegroundColour()
    fn get_text_extent(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void) {
        unsafe {
            let string = crate::wx_string_from(string);
            ffi::wxWindow_GetTextExtent(self.as_ptr(), string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent1(&self, string: &str) -> Size {
        unsafe {
            let string = crate::wx_string_from(string);
            Size(ffi::wxWindow_GetTextExtent1(self.as_ptr(), string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetUpdateClientRect(self.as_ptr())) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr()) }
    }
    fn refresh<T: RectMethods>(&self, erase_background: bool, rect: Option<&T>) {
        unsafe {
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Refresh(self.as_ptr(), erase_background, rect)
        }
    }
    fn refresh_rect(&self, rect: &Rect, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxWindow_RefreshRect(self.as_ptr(), rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr()) }
    }
    fn set_background_colour(&self, colour: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetBackgroundColour(self.as_ptr(), colour) }
    }
    // NOT_SUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr(), reason) }
    }
    fn set_font(&self, font: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetFont(self.as_ptr(), font) }
    }
    fn set_foreground_colour(&self, colour: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetForegroundColour(self.as_ptr(), colour) }
    }
    fn set_own_background_colour(&self, colour: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr(), colour) }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr()) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr()) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr()) }
    }
    fn set_own_font(&self, font: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnFont(self.as_ptr(), font) }
    }
    fn set_own_foreground_colour(&self, colour: *const c_void) {
        unsafe { ffi::wxWindow_SetOwnForegroundColour(self.as_ptr(), colour) }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr()) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr()) }
    }
    fn set_palette(&self, pal: *const c_void) {
        unsafe { ffi::wxWindow_SetPalette(self.as_ptr(), pal) }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr()) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr(), enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr()) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr()) }
    }
    fn set_transparent(&self, alpha: c_uchar) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr(), alpha) }
    }
    fn get_event_handler(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetEventHandler(self.as_ptr()) }
    }
    fn handle_as_navigation_key(&self, event: *const c_void) -> bool {
        unsafe { ffi::wxWindow_HandleAsNavigationKey(self.as_ptr(), event) }
    }
    fn handle_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_HandleWindowEvent(self.as_ptr(), event) }
    }
    fn process_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEvent(self.as_ptr(), event) }
    }
    fn process_window_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr(), event) }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut c_void {
        unsafe { ffi::wxWindow_PopEventHandler(self.as_ptr(), delete_handler) }
    }
    fn push_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.as_ptr(), handler)
        }
    }
    fn remove_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.as_ptr(), handler)
        }
    }
    fn set_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.as_ptr(), handler)
        }
    }
    fn get_extra_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr()) }
    }
    fn get_window_style_flag(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr()) }
    }
    fn get_window_style(&self) -> c_long {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr()) }
    }
    fn has_extra_style(&self, ex_flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr(), ex_flag) }
    }
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr(), flag) }
    }
    fn set_extra_style(&self, ex_style: c_long) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr(), ex_style) }
    }
    fn set_window_style_flag(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr(), style) }
    }
    fn set_window_style(&self, style: c_long) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr(), style) }
    }
    fn toggle_window_style(&self, flag: c_int) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr(), flag) }
    }
    fn move_after_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.as_ptr(), win)
        }
    }
    fn move_before_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.as_ptr(), win)
        }
    }
    fn navigate(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr(), flags) }
    }
    fn navigate_in(&self, flags: c_int) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr(), flags) }
    }
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr()) }
    }
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr()) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr()) }
    }
    fn is_exposed(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr(), x, y) }
    }
    fn is_exposed1(&self, pt: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsExposed1(self.as_ptr(), pt) }
    }
    fn is_exposed2(&self, x: c_int, y: c_int, w: c_int, h: c_int) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr(), x, y, w, h) }
    }
    fn is_exposed3(&self, rect: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_IsExposed3(self.as_ptr(), rect) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr()) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr()) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr()) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr(), enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr(), show) }
    }
    // NOT_SUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> String {
        unsafe { crate::from_wx_string(ffi::wxWindow_GetHelpText(self.as_ptr())) }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = crate::wx_string_from(help_text);
            ffi::wxWindow_SetHelpText(self.as_ptr(), help_text)
        }
    }
    // NOT_SUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetToolTip(self.as_ptr()) }
    }
    fn get_tool_tip_text(&self) -> String {
        unsafe { crate::from_wx_string(ffi::wxWindow_GetToolTipText(self.as_ptr())) }
    }
    fn set_tool_tip(&self, tip_string: &str) {
        unsafe {
            let tip_string = crate::wx_string_from(tip_string);
            ffi::wxWindow_SetToolTip(self.as_ptr(), tip_string)
        }
    }
    fn set_tool_tip1(&self, tip: *mut c_void) {
        unsafe { ffi::wxWindow_SetToolTip1(self.as_ptr(), tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr()) }
    }
    fn get_popup_menu_selection_from_user(&self, menu: *mut c_void, pos: &Point) -> c_int {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr(), menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user1(&self, menu: *mut c_void, x: c_int, y: c_int) -> c_int {
        unsafe { ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr(), menu, x, y) }
    }
    fn popup_menu(&self, menu: *mut c_void, pos: &Point) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxWindow_PopupMenu(self.as_ptr(), menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut c_void, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxWindow_PopupMenu1(self.as_ptr(), menu, x, y) }
    }
    fn get_validator(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetValidator(self.as_ptr()) }
    }
    fn set_validator(&self, validator: &Validator) {
        unsafe {
            let validator = validator.as_ptr();
            ffi::wxWindow_SetValidator(self.as_ptr(), validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr()) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr()) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr()) }
    }
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxWindow_GetId(self.as_ptr()) }
    }
    fn get_label(&self) -> String {
        unsafe { crate::from_wx_string(ffi::wxWindow_GetLabel(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: c_int, width: c_int, width_total: c_int) -> c_int {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr(), x, width, width_total) }
    }
    fn get_name(&self) -> String {
        unsafe { crate::from_wx_string(ffi::wxWindow_GetName(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: c_int) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr(), winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxWindow_SetLabel(self.as_ptr(), label)
        }
    }
    // NOT_SUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxWindow_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetAcceleratorTable(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: *const c_void) {
        unsafe { ffi::wxWindow_SetAcceleratorTable(self.as_ptr(), accel) }
    }
    // NOT_SUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr(), force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr()) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr()) }
    }
    fn get_drop_target(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetDropTarget(self.as_ptr()) }
    }
    fn set_drop_target(&self, target: *mut c_void) {
        unsafe { ffi::wxWindow_SetDropTarget(self.as_ptr(), target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr(), accept) }
    }
    fn get_containing_sizer(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetContainingSizer(self.as_ptr()) }
    }
    fn get_sizer(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetSizer(self.as_ptr()) }
    }
    fn set_sizer(&self, sizer: *mut c_void, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizer(self.as_ptr(), sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut c_void, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizerAndFit(self.as_ptr(), sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetConstraints(self.as_ptr()) }
    }
    fn set_constraints(&self, constraints: *mut c_void) {
        unsafe { ffi::wxWindow_SetConstraints(self.as_ptr(), constraints) }
    }
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr()) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr(), auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr()) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr()) }
    }
    fn get_caret(&self) -> *mut c_void {
        unsafe { ffi::wxWindow_GetCaret(self.as_ptr()) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr()) }
    }
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr()) }
    }
    fn set_caret(&self, caret: *mut c_void) {
        unsafe { ffi::wxWindow_SetCaret(self.as_ptr(), caret) }
    }
    fn set_cursor(&self, cursor: *const c_void) -> bool {
        unsafe { ffi::wxWindow_SetCursor(self.as_ptr(), cursor) }
    }
    fn warp_pointer(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr(), x, y) }
    }
    fn enable_touch_events(&self, events_mask: c_int) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr(), events_mask) }
    }
    // NOT_SUPPORTED: fn HitTest()
    // NOT_SUPPORTED: fn HitTest1()
    // NOT_SUPPORTED: fn GetBorder()
    // NOT_SUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: *mut c_void) {
        unsafe { ffi::wxWindow_DoUpdateWindowUI(self.as_ptr(), event) }
    }
    // NOT_SUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr()) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr()) }
    }
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr()) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr()) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr(), on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr()) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr()) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr()) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr()) }
    }
    fn send_idle_events(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.as_ptr(), event) }
    }
    fn register_hot_key(&self, hotkey_id: c_int, modifiers: c_int, virtual_key_code: c_int) -> bool {
        unsafe { ffi::wxWindow_RegisterHotKey(self.as_ptr(), hotkey_id, modifiers, virtual_key_code) }
    }
    fn unregister_hot_key(&self, hotkey_id: c_int) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr(), hotkey_id) }
    }
    fn update_window_ui(&self, flags: c_long) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> *mut c_void {
        unsafe { ffi::wxWindow_FindFocus() }
    }
    fn find_window_by_id<T: WindowMethods>(id: c_long, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowById(id, parent)
        }
    }
    fn find_window_by_label<T: WindowMethods>(label: &str, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let label = crate::wx_string_from(label);
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByLabel(label, parent)
        }
    }
    fn find_window_by_name<T: WindowMethods>(name: &str, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByName(name, parent)
        }
    }
    fn get_capture() -> *mut c_void {
        unsafe { ffi::wxWindow_GetCapture() }
    }
    fn new_control_id(count: c_int) -> c_int {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    fn unreserve_control_id(id: c_int, count: c_int) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: c_int, pos: &Point, size: &Size, style: c_long, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = crate::wx_string_from(name);
            ffi::wxWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxControl
wx_class! { Control(wxControl) impl
    ControlMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Control {
    pub fn new<T: WindowMethods>(parent: Option<&T>, id: c_int, pos: &Point, size: &Size, style: c_long, validator: &Validator, name: &str) -> Control {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = crate::wx_string_from(name);
            Control(ffi::wxControl_new(parent, id, pos, size, style, validator, name))
        }
    }
    pub fn new1() -> Control {
        unsafe { Control(ffi::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ControlMethods: WindowMethods {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: c_int, pos: &Point, size: &Size, style: c_long, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = crate::wx_string_from(name);
            ffi::wxControl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: *mut c_void) {
        unsafe { ffi::wxControl_Command(self.as_ptr(), event) }
    }
    fn get_label_text(&self) -> String {
        unsafe { crate::from_wx_string(ffi::wxControl_GetLabelText(self.as_ptr())) }
    }
    fn get_size_from_text_size(&self, xlen: c_int, ylen: c_int) -> Size {
        unsafe { Size(ffi::wxControl_GetSizeFromTextSize(self.as_ptr(), xlen, ylen)) }
    }
    fn get_size_from_text_size1(&self, tsize: &Size) -> Size {
        unsafe {
            let tsize = tsize.as_ptr();
            Size(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr(), tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = crate::wx_string_from(text);
            Size(ffi::wxControl_GetSizeFromText(self.as_ptr(), text))
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxControl_SetLabelText(self.as_ptr(), text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = crate::wx_string_from(markup);
            ffi::wxControl_SetLabelMarkup(self.as_ptr(), markup)
        }
    }
    fn get_label_text1(label: &str) -> String {
        unsafe {
            let label = crate::wx_string_from(label);
            crate::from_wx_string(ffi::wxControl_GetLabelText1(label))
        }
    }
    fn remove_mnemonics(str: &str) -> String {
        unsafe {
            let str = crate::wx_string_from(str);
            crate::from_wx_string(ffi::wxControl_RemoveMnemonics(str))
        }
    }
    fn escape_mnemonics(text: &str) -> String {
        unsafe {
            let text = crate::wx_string_from(text);
            crate::from_wx_string(ffi::wxControl_EscapeMnemonics(text))
        }
    }
    fn ellipsize(label: &str, dc: *const c_void, mode: c_int, max_width: c_int, flags: c_int) -> String {
        unsafe {
            let label = crate::wx_string_from(label);
            crate::from_wx_string(ffi::wxControl_Ellipsize(label, dc, mode, max_width, flags))
        }
    }
}

// wxAnyButton
wx_class! { AnyButton(wxAnyButton) impl
    AnyButtonMethods,
    ControlMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl AnyButton {
    pub fn new() -> AnyButton {
        unsafe { AnyButton(ffi::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    // NOT_SUPPORTED: fn GetBitmap()
    // NOT_SUPPORTED: fn GetBitmapCurrent()
    // NOT_SUPPORTED: fn GetBitmapDisabled()
    // NOT_SUPPORTED: fn GetBitmapFocus()
    // NOT_SUPPORTED: fn GetBitmapLabel()
    // NOT_SUPPORTED: fn GetBitmapPressed()
    // NOT_SUPPORTED: fn SetBitmap()
    fn set_bitmap_current(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr(), bitmap) }
    }
    fn set_bitmap_disabled(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr(), bitmap) }
    }
    fn set_bitmap_focus(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapFocus(self.as_ptr(), bitmap) }
    }
    fn set_bitmap_label(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapLabel(self.as_ptr(), bitmap) }
    }
    fn set_bitmap_pressed(&self, bitmap: *const c_void) {
        unsafe { ffi::wxAnyButton_SetBitmapPressed(self.as_ptr(), bitmap) }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr())) }
    }
    fn set_bitmap_margins(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr(), x, y) }
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr(), sz)
        }
    }
    // NOT_SUPPORTED: fn SetBitmapPosition()
}

// wxButton
wx_class! { Button(wxButton) impl
    ButtonMethods,
    AnyButtonMethods,
    ControlMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Button {
    pub fn new() -> Button {
        unsafe { Button(ffi::wxButton_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: c_int, label: &str, pos: &Point, size: &Size, style: c_long, validator: &Validator, name: &str) -> Button {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = crate::wx_string_from(name);
            Button(ffi::wxButton_new1(parent, id, label, pos, size, style, validator, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ButtonMethods: AnyButtonMethods {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: c_int, label: &str, pos: &Point, size: &Size, style: c_long, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = crate::wx_string_from(name);
            ffi::wxButton_Create(self.as_ptr(), parent, id, label, pos, size, style, validator, name)
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr()) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr(), needed) }
    }
    fn set_default(&self) -> *mut c_void {
        unsafe { ffi::wxButton_SetDefault(self.as_ptr()) }
    }
    fn get_default_size<T: WindowMethods>(win: Option<&T>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Size(ffi::wxButton_GetDefaultSize(win))
        }
    }
}

// wxNonOwnedWindow
wx_class! { NonOwnedWindow(wxNonOwnedWindow) impl
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl NonOwnedWindow {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait NonOwnedWindowMethods: WindowMethods {
    fn set_shape(&self, region: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape(self.as_ptr(), region) }
    }
    fn set_shape1(&self, path: *const c_void) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape1(self.as_ptr(), path) }
    }
}

// wxTopLevelWindow
wx_class! { TopLevelWindow(wxTopLevelWindow) impl
    TopLevelWindowMethods,
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl TopLevelWindow {
    pub fn new() -> TopLevelWindow {
        unsafe { TopLevelWindow(ffi::wxTopLevelWindow_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: c_int, title: &str, pos: &Point, size: &Size, style: c_long, name: &str) -> TopLevelWindow {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = crate::wx_string_from(name);
            TopLevelWindow(ffi::wxTopLevelWindow_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait TopLevelWindowMethods: NonOwnedWindowMethods {
    // DTOR: fn ~wxTopLevelWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: c_int, title: &str, pos: &Point, size: &Size, style: c_long, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = crate::wx_string_from(name);
            ffi::wxTopLevelWindow_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    fn center_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.as_ptr(), direction) }
    }
    fn centre_on_screen(&self, direction: c_int) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.as_ptr(), direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.as_ptr(), enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr(), enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr(), enable) }
    }
    fn get_default_item(&self) -> *mut c_void {
        unsafe { ffi::wxTopLevelWindow_GetDefaultItem(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> String {
        unsafe { crate::from_wx_string(ffi::wxTopLevelWindow_GetTitle(self.as_ptr())) }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.as_ptr(), iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.as_ptr()) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr()) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(self.as_ptr()) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(self.as_ptr()) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(self.as_ptr()) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.as_ptr(), maximize) }
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: c_int) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.as_ptr(), flags) }
    }
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.as_ptr()) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut c_void {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetDefaultItem(self.as_ptr(), win)
        }
    }
    fn set_tmp_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut c_void {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr(), win)
        }
    }
    fn get_tmp_default_item(&self) -> *mut c_void {
        unsafe { ffi::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr()) }
    }
    fn set_icon(&self, icon: *const c_void) {
        unsafe { ffi::wxTopLevelWindow_SetIcon(self.as_ptr(), icon) }
    }
    fn set_icons(&self, icons: *const c_void) {
        unsafe { ffi::wxTopLevelWindow_SetIcons(self.as_ptr(), icons) }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = crate::wx_string_from(title);
            ffi::wxTopLevelWindow_SetTitle(self.as_ptr(), title)
        }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr()) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.as_ptr(), modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(self.as_ptr()) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = crate::wx_string_from(filename);
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr(), filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr()) }
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.as_ptr(), enable) }
    }
    fn show_full_screen(&self, show: bool, style: c_long) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.as_ptr(), show, style) }
    }
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    fn get_default_size() -> Size {
        unsafe { Size(ffi::wxTopLevelWindow_GetDefaultSize()) }
    }
}

// wxFrame
wx_class! { Frame(wxFrame) impl
    FrameMethods,
    // TopLevelWindowMethods,
    NonOwnedWindowMethods,
    // WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Frame {
    pub fn new() -> Frame {
        unsafe { Frame(ffi::wxFrame_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: c_int, title: &str, pos: &Point, size: &Size, style: c_long, name: &str) -> Frame {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = crate::wx_string_from(name);
            Frame(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl TopLevelWindowMethods for Frame {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: c_int, title: &str, pos: &Point, size: &Size, style: c_long, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = crate::wx_string_from(name);
            ffi::wxFrame_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
}
impl WindowMethods for Frame {
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr(), direction) }
    }
}
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    fn create_status_bar(&self, number: c_int, style: c_long, id: c_int, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_CreateStatusBar(self.as_ptr(), number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_CreateToolBar(self.as_ptr(), style, id, name)
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_DoGiveHelp(self.as_ptr(), text, show)
        }
    }
    fn get_menu_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetMenuBar(self.as_ptr()) }
    }
    fn get_status_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetStatusBar(self.as_ptr()) }
    }
    fn get_status_bar_pane(&self) -> c_int {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr()) }
    }
    fn get_tool_bar(&self) -> *mut c_void {
        unsafe { ffi::wxFrame_GetToolBar(self.as_ptr()) }
    }
    fn on_create_status_bar(&self, number: c_int, style: c_long, id: c_int, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_OnCreateStatusBar(self.as_ptr(), number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: c_long, id: c_int, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_OnCreateToolBar(self.as_ptr(), style, id, name)
        }
    }
    fn process_command(&self, id: c_int) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr(), id) }
    }
    fn set_menu_bar(&self, menu_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetMenuBar(self.as_ptr(), menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetStatusBar(self.as_ptr(), status_bar) }
    }
    fn set_status_bar_pane(&self, n: c_int) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr(), n) }
    }
    fn set_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_SetStatusText(self.as_ptr(), text, number)
        }
    }
    fn set_status_widths(&self, n: c_int, widths_field: *const c_void) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr(), n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut c_void) {
        unsafe { ffi::wxFrame_SetToolBar(self.as_ptr(), tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: c_int) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_PushStatusText(self.as_ptr(), text, number)
        }
    }
    fn pop_status_text(&self, number: c_int) {
        unsafe { ffi::wxFrame_PopStatusText(self.as_ptr(), number) }
    }
}

// wxPoint
wx_class! { Point(wxPoint) impl
    PointMethods
}
impl Point {
    pub fn new() -> Point {
        unsafe { Point(ffi::wxPoint_new()) }
    }
    pub fn new1(x: c_int, y: c_int) -> Point {
        unsafe { Point(ffi::wxPoint_new1(x, y)) }
    }
    pub fn new2(pt: *const c_void) -> Point {
        unsafe { Point(ffi::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr()) }
    }
    fn set_defaults(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxPoint_SetDefaults(self.as_ptr(), pt)
        }
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-1()
    // BLOCKED: fn operator+2()
    // BLOCKED: fn operator-2()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxRect
wx_class! { Rect(wxRect) impl
    RectMethods
}
impl Rect {
    pub fn new() -> Rect {
        unsafe { Rect(ffi::wxRect_new()) }
    }
    pub fn new1(x: c_int, y: c_int, width: c_int, height: c_int) -> Rect {
        unsafe { Rect(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new2(top_left: &Point, bottom_right: &Point) -> Rect {
        unsafe {
            let top_left = top_left.as_ptr();
            let bottom_right = bottom_right.as_ptr();
            Rect(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new3(pos: &Point, size: &Size) -> Rect {
        unsafe {
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            Rect(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new4(size: &Size) -> Rect {
        unsafe {
            let size = size.as_ptr();
            Rect(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait RectMethods: WxRustMethods {
    fn centre_in(&self, r: &Rect, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect(ffi::wxRect_CentreIn(self.as_ptr(), r, dir))
        }
    }
    fn center_in(&self, r: &Rect, dir: c_int) -> Rect {
        unsafe {
            let r = r.as_ptr();
            Rect(ffi::wxRect_CenterIn(self.as_ptr(), r, dir))
        }
    }
    fn contains(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr(), x, y) }
    }
    fn contains1(&self, pt: &Point) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Contains1(self.as_ptr(), pt)
        }
    }
    fn contains2(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Contains2(self.as_ptr(), rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate3(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect(ffi::wxRect_Deflate3(self.as_ptr(), dx, dy)) }
    }
    fn get_bottom(&self) -> c_int {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr()) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomLeft(self.as_ptr())) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomRight(self.as_ptr())) }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr()) }
    }
    fn get_left(&self) -> c_int {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetPosition(self.as_ptr())) }
    }
    fn get_right(&self) -> c_int {
        unsafe { ffi::wxRect_GetRight(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size(ffi::wxRect_GetSize(self.as_ptr())) }
    }
    fn get_top(&self) -> c_int {
        unsafe { ffi::wxRect_GetTop(self.as_ptr()) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopLeft(self.as_ptr())) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopRight(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr()) }
    }
    fn get_x(&self) -> c_int {
        unsafe { ffi::wxRect_GetX(self.as_ptr()) }
    }
    fn get_y(&self) -> c_int {
        unsafe { ffi::wxRect_GetY(self.as_ptr()) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate3(&self, dx: c_int, dy: c_int) -> Rect {
        unsafe { Rect(ffi::wxRect_Inflate3(self.as_ptr(), dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect1(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect(ffi::wxRect_Intersect1(self.as_ptr(), rect))
        }
    }
    fn intersects(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxRect_Intersects(self.as_ptr(), rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr()) }
    }
    fn offset(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxRect_Offset(self.as_ptr(), dx, dy) }
    }
    fn offset1(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxRect_Offset1(self.as_ptr(), pt)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr(), height) }
    }
    fn set_position(&self, pos: &Point) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxRect_SetPosition(self.as_ptr(), pos)
        }
    }
    fn set_size(&self, s: &Size) {
        unsafe {
            let s = s.as_ptr();
            ffi::wxRect_SetSize(self.as_ptr(), s)
        }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr(), width) }
    }
    fn set_x(&self, x: c_int) {
        unsafe { ffi::wxRect_SetX(self.as_ptr(), x) }
    }
    fn set_y(&self, y: c_int) {
        unsafe { ffi::wxRect_SetY(self.as_ptr(), y) }
    }
    fn set_left(&self, left: c_int) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr(), left) }
    }
    fn set_right(&self, right: c_int) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr(), right) }
    }
    fn set_top(&self, top: c_int) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr(), top) }
    }
    fn set_bottom(&self, bottom: c_int) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr(), bottom) }
    }
    fn set_top_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopLeft(self.as_ptr(), p)
        }
    }
    fn set_bottom_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomRight(self.as_ptr(), p)
        }
    }
    fn set_top_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetTopRight(self.as_ptr(), p)
        }
    }
    fn set_bottom_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr();
            ffi::wxRect_SetBottomLeft(self.as_ptr(), p)
        }
    }
    fn union(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr();
            Rect(ffi::wxRect_Union(self.as_ptr(), rect))
        }
    }
    // BLOCKED: fn Union1()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
}

// wxSize
wx_class! { Size(wxSize) impl
    SizeMethods
}
impl Size {
    pub fn new() -> Size {
        unsafe { Size(ffi::wxSize_new()) }
    }
    pub fn new1(width: c_int, height: c_int) -> Size {
        unsafe { Size(ffi::wxSize_new1(width, height)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait SizeMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator/()
    // BLOCKED: fn operator*()
    // BLOCKED: fn operator*1()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
    fn dec_by(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_DecBy(self.as_ptr(), pt)
        }
    }
    fn dec_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecBy1(self.as_ptr(), size)
        }
    }
    fn dec_by2(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_DecBy2(self.as_ptr(), dx, dy) }
    }
    fn dec_by3(&self, d: c_int) {
        unsafe { ffi::wxSize_DecBy3(self.as_ptr(), d) }
    }
    fn dec_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecTo(self.as_ptr(), size)
        }
    }
    fn dec_to_if_specified(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_DecToIfSpecified(self.as_ptr(), size)
        }
    }
    fn get_height(&self) -> c_int {
        unsafe { ffi::wxSize_GetHeight(self.as_ptr()) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxSize_GetWidth(self.as_ptr()) }
    }
    fn inc_by(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxSize_IncBy(self.as_ptr(), pt)
        }
    }
    fn inc_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncBy1(self.as_ptr(), size)
        }
    }
    fn inc_by2(&self, dx: c_int, dy: c_int) {
        unsafe { ffi::wxSize_IncBy2(self.as_ptr(), dx, dy) }
    }
    fn inc_by3(&self, d: c_int) {
        unsafe { ffi::wxSize_IncBy3(self.as_ptr(), d) }
    }
    fn inc_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxSize_IncTo(self.as_ptr(), size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(self.as_ptr()) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxSize_Set(self.as_ptr(), width, height) }
    }
    fn set_defaults(&self, size_default: &Size) {
        unsafe {
            let size_default = size_default.as_ptr();
            ffi::wxSize_SetDefaults(self.as_ptr(), size_default)
        }
    }
    fn set_height(&self, height: c_int) {
        unsafe { ffi::wxSize_SetHeight(self.as_ptr(), height) }
    }
    fn set_width(&self, width: c_int) {
        unsafe { ffi::wxSize_SetWidth(self.as_ptr(), width) }
    }
}

// wxValidator
wx_class! { Validator(wxValidator) impl
    ValidatorMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Validator {
    pub fn new() -> Validator {
        unsafe { Validator(ffi::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    fn clone(&self) -> *mut c_void {
        unsafe { ffi::wxValidator_Clone(self.as_ptr()) }
    }
    fn get_window(&self) -> *mut c_void {
        unsafe { ffi::wxValidator_GetWindow(self.as_ptr()) }
    }
    fn set_window<T: WindowMethods>(&self, window: Option<&T>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.as_ptr(), window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr()) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr()) }
    }
    fn validate<T: WindowMethods>(&self, parent: Option<&T>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.as_ptr(), parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}

