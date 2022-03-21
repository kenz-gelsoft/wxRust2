#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::{c_char, c_void};
use std::pin::Pin;
use std::ptr;

use crate::WxString;
use crate::macros::wx_class;

// any pointer type used on ffi boundary.
// we chose this type as it's handy in cxx.
type UnsafeAnyPtr = *const c_char;

#[cxx::bridge(namespace = "wxrust")]
mod ffi {
    #[namespace = ""]
    unsafe extern "C++" {
        include!("wx/include/wxrust.h");
        include!("wx/include/wxrust2.h");

        type wxString;
        type wxWindowList;
        type wxSizer;
        type wxFont;
        type wxRegion;
        type wxColour;
        type wxPalette;
        type wxKeyEvent;
        type wxEvent;
        type wxToolTip;
        type wxMenu;
        type wxAcceleratorTable;
        type wxDropTarget;
        type wxLayoutConstraints;
        type wxCaret;
        type wxCursor;
        type wxUpdateUIEvent;
        type wxIdleEvent;
        type wxBitmap;
        type wxCommandEvent;
        type wxClientData;
        type wxEventFilter;
        type wxClassInfo;
        type wxObjectRefData;
        type wxMenuBar;
        type wxToolBar;
        type wxStatusBar;
        type wxIconBundle;
        type wxIcon;
        type wxGraphicsPath;
        type wxRealPoint;
        type wxDC;
        
        // CLASS: wxObject
        type wxObject;
        // CTOR: pub fn wxObject() -> *mut c_void;
        // CTOR: pub fn wxObject1(other: *const c_void) -> *mut c_void;
        // DTOR: pub fn ~wxObject(self: *mut c_void);
        // GENERATED: pub fn GetClassInfo(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetRefData(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn IsKindOf(self: *const c_void, info: *const c_void) -> bool;
        // GENERATED: pub fn IsSameAs(self: *const c_void, obj: *const c_void) -> bool;
        // GENERATED: pub fn Ref(self: *mut c_void, clone: *const c_void);
        // GENERATED: pub fn SetRefData(self: *mut c_void, data: *mut c_void);
        // GENERATED: pub fn UnRef(self: *mut c_void);
        // GENERATED: pub fn UnShare(self: *mut c_void);
        // BLOCKED: pub fn operator delete(self: *mut c_void, buf: *mut c_void);
        // CXX_UNSUPPORTED: pub fn operator new(self: *mut c_void, size: size_t, filename: *const c_void, line_num: i32) -> *mut c_void;
        
        // CLASS: wxEvtHandler
        type wxEvtHandler;
        // GENERATED: pub fn QueueEvent(self: *mut c_void, event: *mut c_void);
        // GENERATED: pub fn AddPendingEvent(self: *mut c_void, event: *const c_void);
        // CXX_UNSUPPORTED: pub fn CallAfter(self: *mut c_void, method: *mut c_void, x1: T1, None: ...);
        // BLOCKED: pub fn CallAfter1(self: *mut c_void, functor: *const c_void);
        // GENERATED: pub fn ProcessEvent(self: *mut c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn ProcessEventLocally(self: *mut c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn SafelyProcessEvent(self: *mut c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn ProcessPendingEvents(self: *mut c_void);
        // GENERATED: pub fn DeletePendingEvents(self: *mut c_void);
        // CXX_UNSUPPORTED: pub fn Connect(self: *mut c_void, id: i32, last_id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // CXX_UNSUPPORTED: pub fn Connect1(self: *mut c_void, id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // CXX_UNSUPPORTED: pub fn Connect2(self: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
        // CXX_UNSUPPORTED: pub fn Disconnect(self: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // CXX_UNSUPPORTED: pub fn Disconnect1(self: *mut c_void, id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // CXX_UNSUPPORTED: pub fn Disconnect2(self: *mut c_void, id: i32, last_id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
        // CXX_UNSUPPORTED: pub fn Bind(self: *mut c_void, event_type: *const c_void, functor: Functor, id: i32, last_id: i32, user_data: *mut c_void);
        // BLOCKED: pub fn Bind1(self: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: i32, last_id: i32, user_data: *mut c_void);
        // CXX_UNSUPPORTED: pub fn Unbind(self: *mut c_void, event_type: *const c_void, functor: Functor, id: i32, last_id: i32, user_data: *mut c_void) -> bool;
        // BLOCKED: pub fn Unbind1(self: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: i32, last_id: i32, user_data: *mut c_void) -> bool;
        // BLOCKED: pub fn GetClientData(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetClientObject(self: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn SetClientData(self: *mut c_void, data: *mut c_void);
        // GENERATED: pub fn SetClientObject(self: *mut c_void, data: *mut c_void);
        // GENERATED: pub fn GetEvtHandlerEnabled(self: *const c_void) -> bool;
        // GENERATED: pub fn GetNextHandler(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetPreviousHandler(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn SetEvtHandlerEnabled(self: *mut c_void, enabled: bool);
        // GENERATED: pub fn SetNextHandler(self: *mut c_void, handler: *mut c_void);
        // GENERATED: pub fn SetPreviousHandler(self: *mut c_void, handler: *mut c_void);
        // GENERATED: pub fn Unlink(self: *mut c_void);
        // GENERATED: pub fn IsUnlinked(self: *const c_void) -> bool;
        // GENERATED: pub fn AddFilter(filter: *mut c_void);
        // GENERATED: pub fn RemoveFilter(filter: *mut c_void);
        // CTOR: pub fn wxEvtHandler() -> *mut c_void;
        // DTOR: pub fn ~wxEvtHandler(self: *mut c_void);
        
        // CLASS: wxWindow
        type wxWindow;
        // GENERATED: pub fn AcceptsFocus(self: *const c_void) -> bool;
        // GENERATED: pub fn AcceptsFocusFromKeyboard(self: *const c_void) -> bool;
        // GENERATED: pub fn AcceptsFocusRecursively(self: *const c_void) -> bool;
        // GENERATED: pub fn DisableFocusFromKeyboard(self: *mut c_void);
        // GENERATED: pub fn IsFocusable(self: *const c_void) -> bool;
        // GENERATED: pub fn CanAcceptFocus(self: *const c_void) -> bool;
        // GENERATED: pub fn CanAcceptFocusFromKeyboard(self: *const c_void) -> bool;
        // GENERATED: pub fn HasFocus(self: *const c_void) -> bool;
        // GENERATED: pub fn SetCanFocus(self: *mut c_void, can_focus: bool);
        // GENERATED: pub fn EnableVisibleFocus(self: *mut c_void, enable: bool);
        // GENERATED: pub fn SetFocus(self: *mut c_void);
        // GENERATED: pub fn SetFocusFromKbd(self: *mut c_void);
        // GENERATED: pub fn AddChild(self: *mut c_void, child: *mut c_void);
        // GENERATED: pub fn DestroyChildren(self: *mut c_void) -> bool;
        // GENERATED: pub fn FindWindow(self: *const c_void, id: i32) -> *mut c_void;
        // GENERATED: pub fn FindWindow1(self: *const c_void, name: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn GetChildren(self: *mut c_void) -> *mut c_void;
        // BLOCKED: pub fn GetChildren1(self: *const c_void) -> *const c_void;
        // GENERATED: pub fn RemoveChild(self: *mut c_void, child: *mut c_void);
        // GENERATED: pub fn GetGrandParent(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetNextSibling(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetParent(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetPrevSibling(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn IsDescendant(self: *const c_void, win: *mut c_void) -> bool;
        // GENERATED: pub fn Reparent(self: *mut c_void, new_parent: *mut c_void) -> bool;
        // GENERATED: pub fn AlwaysShowScrollbars(self: *mut c_void, hflag: bool, vflag: bool);
        // GENERATED: pub fn GetScrollPos(self: *const c_void, orientation: i32) -> i32;
        // GENERATED: pub fn GetScrollRange(self: *const c_void, orientation: i32) -> i32;
        // GENERATED: pub fn GetScrollThumb(self: *const c_void, orientation: i32) -> i32;
        // GENERATED: pub fn CanScroll(self: *const c_void, orient: i32) -> bool;
        // GENERATED: pub fn HasScrollbar(self: *const c_void, orient: i32) -> bool;
        // GENERATED: pub fn IsScrollbarAlwaysShown(self: *const c_void, orient: i32) -> bool;
        // GENERATED: pub fn ScrollLines(self: *mut c_void, lines: i32) -> bool;
        // GENERATED: pub fn ScrollPages(self: *mut c_void, pages: i32) -> bool;
        // GENERATED: pub fn ScrollWindow(self: *mut c_void, dx: i32, dy: i32, rect: *const c_void);
        // GENERATED: pub fn LineUp(self: *mut c_void) -> bool;
        // GENERATED: pub fn LineDown(self: *mut c_void) -> bool;
        // GENERATED: pub fn PageUp(self: *mut c_void) -> bool;
        // GENERATED: pub fn PageDown(self: *mut c_void) -> bool;
        // GENERATED: pub fn SetScrollPos(self: *mut c_void, orientation: i32, pos: i32, refresh: bool);
        // GENERATED: pub fn SetScrollbar(self: *mut c_void, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        // GENERATED: pub fn BeginRepositioningChildren(self: *mut c_void) -> bool;
        // GENERATED: pub fn EndRepositioningChildren(self: *mut c_void);
        // GENERATED: pub fn CacheBestSize(self: *const c_void, size: *const c_void);
        // GENERATED: pub fn ClientToWindowSize(self: *const c_void, size: *const c_void) -> wxSize;
        // GENERATED: pub fn WindowToClientSize(self: *const c_void, size: *const c_void) -> wxSize;
        // GENERATED: pub fn Fit(self: *mut c_void);
        // GENERATED: pub fn FitInside(self: *mut c_void);
        // GENERATED: pub fn FromDIP(self: *const c_void, sz: *const c_void) -> wxSize;
        // GENERATED: pub fn FromDIP1(self: *const c_void, pt: *const c_void) -> wxPoint;
        // GENERATED: pub fn FromDIP2(self: *const c_void, d: i32) -> i32;
        // GENERATED: pub fn ToDIP(self: *const c_void, sz: *const c_void) -> wxSize;
        // GENERATED: pub fn ToDIP1(self: *const c_void, pt: *const c_void) -> wxPoint;
        // GENERATED: pub fn ToDIP2(self: *const c_void, d: i32) -> i32;
        // GENERATED: pub fn GetBestSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetBestHeight(self: *const c_void, width: i32) -> i32;
        // GENERATED: pub fn GetBestWidth(self: *const c_void, height: i32) -> i32;
        // GENERATED: pub fn GetClientSize(self: *const c_void, width: *mut c_void, height: *mut c_void);
        // GENERATED: pub fn GetClientSize1(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetEffectiveMinSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetMaxClientSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetMaxSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetMinClientSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetMinSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetMinWidth(self: *const c_void) -> i32;
        // GENERATED: pub fn GetMinHeight(self: *const c_void) -> i32;
        // GENERATED: pub fn GetMaxWidth(self: *const c_void) -> i32;
        // GENERATED: pub fn GetMaxHeight(self: *const c_void) -> i32;
        // GENERATED: pub fn GetSize(self: *const c_void, width: *mut c_void, height: *mut c_void);
        // GENERATED: pub fn GetSize1(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetVirtualSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetVirtualSize1(self: *const c_void, width: *mut c_void, height: *mut c_void);
        // GENERATED: pub fn GetBestVirtualSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetContentScaleFactor(self: *const c_void) -> f64;
        // GENERATED: pub fn GetDPIScaleFactor(self: *const c_void) -> f64;
        // GENERATED: pub fn GetWindowBorderSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn InformFirstDirection(self: *mut c_void, direction: i32, size: i32, available_other_dir: i32) -> bool;
        // GENERATED: pub fn InvalidateBestSize(self: *mut c_void);
        // GENERATED: pub fn PostSizeEvent(self: *mut c_void);
        // GENERATED: pub fn PostSizeEventToParent(self: *mut c_void);
        // GENERATED: pub fn SendSizeEvent(self: *mut c_void, flags: i32);
        // GENERATED: pub fn SendSizeEventToParent(self: *mut c_void, flags: i32);
        // GENERATED: pub fn SetClientSize(self: *mut c_void, width: i32, height: i32);
        // GENERATED: pub fn SetClientSize1(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetClientSize2(self: *mut c_void, rect: *const c_void);
        // GENERATED: pub fn SetContainingSizer(self: *mut c_void, sizer: *mut c_void);
        // GENERATED: pub fn SetInitialSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetMaxClientSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetMaxSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetMinClientSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetMinSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetSize(self: *mut c_void, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        // GENERATED: pub fn SetSize1(self: *mut c_void, rect: *const c_void);
        // GENERATED: pub fn SetSize2(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetSize3(self: *mut c_void, width: i32, height: i32);
        // GENERATED: pub fn SetSizeHints(self: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        // GENERATED: pub fn SetSizeHints1(self: *mut c_void, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: pub fn SetVirtualSize(self: *mut c_void, width: i32, height: i32);
        // GENERATED: pub fn SetVirtualSize1(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn FromDIP3(sz: *const c_void, w: *const c_void) -> wxSize;
        // GENERATED: pub fn FromDIP4(pt: *const c_void, w: *const c_void) -> wxPoint;
        // GENERATED: pub fn FromDIP5(d: i32, w: *const c_void) -> i32;
        // GENERATED: pub fn ToDIP3(sz: *const c_void, w: *const c_void) -> wxSize;
        // GENERATED: pub fn ToDIP4(pt: *const c_void, w: *const c_void) -> wxPoint;
        // GENERATED: pub fn ToDIP5(d: i32, w: *const c_void) -> i32;
        // GENERATED: pub fn Center(self: *mut c_void, dir: i32);
        // GENERATED: pub fn CenterOnParent(self: *mut c_void, dir: i32);
        // GENERATED: pub fn Centre(self: *mut c_void, direction: i32);
        // GENERATED: pub fn CentreOnParent(self: *mut c_void, direction: i32);
        // GENERATED: pub fn GetPosition(self: *const c_void, x: *mut c_void, y: *mut c_void);
        // GENERATED: pub fn GetPosition1(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetRect(self: *const c_void) -> wxRect;
        // GENERATED: pub fn GetScreenPosition(self: *const c_void, x: *mut c_void, y: *mut c_void);
        // GENERATED: pub fn GetScreenPosition1(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetScreenRect(self: *const c_void) -> wxRect;
        // GENERATED: pub fn GetClientAreaOrigin(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetClientRect(self: *const c_void) -> wxRect;
        // GENERATED: pub fn Move(self: *mut c_void, x: i32, y: i32, flags: i32);
        // GENERATED: pub fn Move1(self: *mut c_void, pt: *const c_void, flags: i32);
        // GENERATED: pub fn SetPosition(self: *mut c_void, pt: *const c_void);
        // GENERATED: pub fn ClientToScreen(self: *const c_void, x: *mut c_void, y: *mut c_void);
        // GENERATED: pub fn ClientToScreen1(self: *const c_void, pt: *const c_void) -> wxPoint;
        // GENERATED: pub fn ConvertDialogToPixels(self: *const c_void, pt: *const c_void) -> wxPoint;
        // GENERATED: pub fn ConvertDialogToPixels1(self: *const c_void, sz: *const c_void) -> wxSize;
        // GENERATED: pub fn ConvertPixelsToDialog(self: *const c_void, pt: *const c_void) -> wxPoint;
        // GENERATED: pub fn ConvertPixelsToDialog1(self: *const c_void, sz: *const c_void) -> wxSize;
        // GENERATED: pub fn ScreenToClient(self: *const c_void, x: *mut c_void, y: *mut c_void);
        // GENERATED: pub fn ScreenToClient1(self: *const c_void, pt: *const c_void) -> wxPoint;
        // GENERATED: pub fn ClearBackground(self: *mut c_void);
        // GENERATED: pub fn Freeze(self: *mut c_void);
        // GENERATED: pub fn Thaw(self: *mut c_void);
        // GENERATED: pub fn IsFrozen(self: *const c_void) -> bool;
        // CXX_UNSUPPORTED: pub fn GetBackgroundColour(self: *const c_void) -> wxColour;
        // CXX_UNSUPPORTED: pub fn GetBackgroundStyle(self: *const c_void) -> wxBackgroundStyle;
        // GENERATED: pub fn GetCharHeight(self: *const c_void) -> i32;
        // GENERATED: pub fn GetCharWidth(self: *const c_void) -> i32;
        // CXX_UNSUPPORTED: pub fn GetDefaultAttributes(self: *const c_void) -> wxVisualAttributes;
        // GENERATED: pub fn GetDPI(self: *const c_void) -> wxSize;
        // CXX_UNSUPPORTED: pub fn GetFont(self: *const c_void) -> wxFont;
        // CXX_UNSUPPORTED: pub fn GetForegroundColour(self: *const c_void) -> wxColour;
        // GENERATED: pub fn GetTextExtent(self: *const c_void, string: *const c_void, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void);
        // GENERATED: pub fn GetTextExtent1(self: *const c_void, string: *const c_void) -> wxSize;
        // BLOCKED: pub fn GetUpdateRegion(self: *const c_void) -> *const c_void;
        // GENERATED: pub fn GetUpdateClientRect(self: *const c_void) -> wxRect;
        // GENERATED: pub fn HasTransparentBackground(self: *mut c_void) -> bool;
        // GENERATED: pub fn Refresh(self: *mut c_void, erase_background: bool, rect: *const c_void);
        // GENERATED: pub fn RefreshRect(self: *mut c_void, rect: *const c_void, erase_background: bool);
        // GENERATED: pub fn Update(self: *mut c_void);
        // GENERATED: pub fn SetBackgroundColour(self: *mut c_void, colour: *const c_void) -> bool;
        // CXX_UNSUPPORTED: pub fn SetBackgroundStyle(self: *mut c_void, style: wxBackgroundStyle) -> bool;
        // GENERATED: pub fn IsTransparentBackgroundSupported(self: *const c_void, reason: *mut c_void) -> bool;
        // GENERATED: pub fn SetFont(self: *mut c_void, font: *const c_void) -> bool;
        // GENERATED: pub fn SetForegroundColour(self: *mut c_void, colour: *const c_void) -> bool;
        // GENERATED: pub fn SetOwnBackgroundColour(self: *mut c_void, colour: *const c_void);
        // GENERATED: pub fn InheritsBackgroundColour(self: *const c_void) -> bool;
        // GENERATED: pub fn UseBgCol(self: *const c_void) -> bool;
        // GENERATED: pub fn UseBackgroundColour(self: *const c_void) -> bool;
        // GENERATED: pub fn SetOwnFont(self: *mut c_void, font: *const c_void);
        // GENERATED: pub fn SetOwnForegroundColour(self: *mut c_void, colour: *const c_void);
        // GENERATED: pub fn UseForegroundColour(self: *const c_void) -> bool;
        // GENERATED: pub fn InheritsForegroundColour(self: *const c_void) -> bool;
        // GENERATED: pub fn SetPalette(self: *mut c_void, pal: *const c_void);
        // GENERATED: pub fn ShouldInheritColours(self: *const c_void) -> bool;
        // GENERATED: pub fn SetThemeEnabled(self: *mut c_void, enable: bool);
        // GENERATED: pub fn GetThemeEnabled(self: *const c_void) -> bool;
        // GENERATED: pub fn CanSetTransparent(self: *mut c_void) -> bool;
        // GENERATED: pub fn SetTransparent(self: *mut c_void, alpha: u8) -> bool;
        // GENERATED: pub fn GetEventHandler(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn HandleAsNavigationKey(self: *mut c_void, event: *const c_void) -> bool;
        // GENERATED: pub fn HandleWindowEvent(self: *const c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn ProcessWindowEvent(self: *mut c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn ProcessWindowEventLocally(self: *mut c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn PopEventHandler(self: *mut c_void, delete_handler: bool) -> *mut c_void;
        // GENERATED: pub fn PushEventHandler(self: *mut c_void, handler: *mut c_void);
        // GENERATED: pub fn RemoveEventHandler(self: *mut c_void, handler: *mut c_void) -> bool;
        // GENERATED: pub fn SetEventHandler(self: *mut c_void, handler: *mut c_void);
        // GENERATED: pub fn SetNextHandler(self: *mut c_void, handler: *mut c_void);
        // GENERATED: pub fn SetPreviousHandler(self: *mut c_void, handler: *mut c_void);
        // GENERATED: pub fn GetExtraStyle(self: *const c_void) -> i32;
        // GENERATED: pub fn GetWindowStyleFlag(self: *const c_void) -> i32;
        // GENERATED: pub fn GetWindowStyle(self: *const c_void) -> i32;
        // GENERATED: pub fn HasExtraStyle(self: *const c_void, ex_flag: i32) -> bool;
        // GENERATED: pub fn HasFlag(self: *const c_void, flag: i32) -> bool;
        // GENERATED: pub fn SetExtraStyle(self: *mut c_void, ex_style: i32);
        // GENERATED: pub fn SetWindowStyleFlag(self: *mut c_void, style: i32);
        // GENERATED: pub fn SetWindowStyle(self: *mut c_void, style: i32);
        // GENERATED: pub fn ToggleWindowStyle(self: *mut c_void, flag: i32) -> bool;
        // GENERATED: pub fn MoveAfterInTabOrder(self: *mut c_void, win: *mut c_void);
        // GENERATED: pub fn MoveBeforeInTabOrder(self: *mut c_void, win: *mut c_void);
        // GENERATED: pub fn Navigate(self: *mut c_void, flags: i32) -> bool;
        // GENERATED: pub fn NavigateIn(self: *mut c_void, flags: i32) -> bool;
        // GENERATED: pub fn Lower(self: *mut c_void);
        // GENERATED: pub fn Raise(self: *mut c_void);
        // GENERATED: pub fn Hide(self: *mut c_void) -> bool;
        // CXX_UNSUPPORTED: pub fn HideWithEffect(self: *mut c_void, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: pub fn IsEnabled(self: *const c_void) -> bool;
        // GENERATED: pub fn IsExposed(self: *const c_void, x: i32, y: i32) -> bool;
        // GENERATED: pub fn IsExposed1(self: *const c_void, pt: *mut c_void) -> bool;
        // GENERATED: pub fn IsExposed2(self: *const c_void, x: i32, y: i32, w: i32, h: i32) -> bool;
        // GENERATED: pub fn IsExposed3(self: *const c_void, rect: *mut c_void) -> bool;
        // GENERATED: pub fn IsShown(self: *const c_void) -> bool;
        // GENERATED: pub fn IsShownOnScreen(self: *const c_void) -> bool;
        // GENERATED: pub fn Disable(self: *mut c_void) -> bool;
        // GENERATED: pub fn Enable(self: *mut c_void, enable: bool) -> bool;
        // GENERATED: pub fn Show(self: *mut c_void, show: bool) -> bool;
        // CXX_UNSUPPORTED: pub fn ShowWithEffect(self: *mut c_void, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: pub fn GetHelpText(self: *const c_void) -> wxString;
        // GENERATED: pub fn SetHelpText(self: *mut c_void, help_text: *const c_void);
        // CXX_UNSUPPORTED: pub fn GetHelpTextAtPoint(self: *const c_void, point: *const c_void, origin: wxHelpEvent::Origin) -> wxString;
        // GENERATED: pub fn GetToolTip(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetToolTipText(self: *const c_void) -> wxString;
        // GENERATED: pub fn SetToolTip(self: *mut c_void, tip_string: *const c_void);
        // GENERATED: pub fn SetToolTip1(self: *mut c_void, tip: *mut c_void);
        // GENERATED: pub fn UnsetToolTip(self: *mut c_void);
        // GENERATED: pub fn GetPopupMenuSelectionFromUser(self: *mut c_void, menu: *mut c_void, pos: *const c_void) -> i32;
        // GENERATED: pub fn GetPopupMenuSelectionFromUser1(self: *mut c_void, menu: *mut c_void, x: i32, y: i32) -> i32;
        // GENERATED: pub fn PopupMenu(self: *mut c_void, menu: *mut c_void, pos: *const c_void) -> bool;
        // GENERATED: pub fn PopupMenu1(self: *mut c_void, menu: *mut c_void, x: i32, y: i32) -> bool;
        // GENERATED: pub fn GetValidator(self: *mut c_void) -> *mut c_void;
        // GENERATED: pub fn SetValidator(self: *mut c_void, validator: *const c_void);
        // GENERATED: pub fn TransferDataFromWindow(self: *mut c_void) -> bool;
        // GENERATED: pub fn TransferDataToWindow(self: *mut c_void) -> bool;
        // GENERATED: pub fn Validate(self: *mut c_void) -> bool;
        // GENERATED: pub fn GetId(self: *const c_void) -> i32;
        // GENERATED: pub fn GetLabel(self: *const c_void) -> wxString;
        // CXX_UNSUPPORTED: pub fn GetLayoutDirection(self: *const c_void) -> wxLayoutDirection;
        // GENERATED: pub fn AdjustForLayoutDirection(self: *const c_void, x: i32, width: i32, width_total: i32) -> i32;
        // GENERATED: pub fn GetName(self: *const c_void) -> wxString;
        // CXX_UNSUPPORTED: pub fn GetWindowVariant(self: *const c_void) -> wxWindowVariant;
        // GENERATED: pub fn SetId(self: *mut c_void, winid: i32);
        // GENERATED: pub fn SetLabel(self: *mut c_void, label: *const c_void);
        // CXX_UNSUPPORTED: pub fn SetLayoutDirection(self: *mut c_void, dir: wxLayoutDirection);
        // GENERATED: pub fn SetName(self: *mut c_void, name: *const c_void);
        // CXX_UNSUPPORTED: pub fn SetWindowVariant(self: *mut c_void, variant: wxWindowVariant);
        // GENERATED: pub fn GetAcceleratorTable(self: *mut c_void) -> *mut c_void;
        // CXX_UNSUPPORTED: pub fn GetAccessible(self: *mut c_void) -> *mut c_void;
        // GENERATED: pub fn SetAcceleratorTable(self: *mut c_void, accel: *const c_void);
        // CXX_UNSUPPORTED: pub fn SetAccessible(self: *mut c_void, accessible: *mut c_void);
        // GENERATED: pub fn Close(self: *mut c_void, force: bool) -> bool;
        // GENERATED: pub fn Destroy(self: *mut c_void) -> bool;
        // GENERATED: pub fn IsBeingDeleted(self: *const c_void) -> bool;
        // GENERATED: pub fn GetDropTarget(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn SetDropTarget(self: *mut c_void, target: *mut c_void);
        // GENERATED: pub fn DragAcceptFiles(self: *mut c_void, accept: bool);
        // GENERATED: pub fn GetContainingSizer(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetSizer(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn SetSizer(self: *mut c_void, sizer: *mut c_void, delete_old: bool);
        // GENERATED: pub fn SetSizerAndFit(self: *mut c_void, sizer: *mut c_void, delete_old: bool);
        // GENERATED: pub fn GetConstraints(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn SetConstraints(self: *mut c_void, constraints: *mut c_void);
        // GENERATED: pub fn Layout(self: *mut c_void) -> bool;
        // GENERATED: pub fn SetAutoLayout(self: *mut c_void, auto_layout: bool);
        // GENERATED: pub fn GetAutoLayout(self: *const c_void) -> bool;
        // GENERATED: pub fn CaptureMouse(self: *mut c_void);
        // GENERATED: pub fn GetCaret(self: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn GetCursor(self: *const c_void) -> *const c_void;
        // GENERATED: pub fn HasCapture(self: *const c_void) -> bool;
        // GENERATED: pub fn ReleaseMouse(self: *mut c_void);
        // GENERATED: pub fn SetCaret(self: *mut c_void, caret: *mut c_void);
        // GENERATED: pub fn SetCursor(self: *mut c_void, cursor: *const c_void) -> bool;
        // GENERATED: pub fn WarpPointer(self: *mut c_void, x: i32, y: i32);
        // GENERATED: pub fn EnableTouchEvents(self: *mut c_void, events_mask: i32) -> bool;
        // CXX_UNSUPPORTED: pub fn HitTest(self: *const c_void, x: i32, y: i32) -> wxHitTest;
        // CXX_UNSUPPORTED: pub fn HitTest1(self: *const c_void, pt: *const c_void) -> wxHitTest;
        // CXX_UNSUPPORTED: pub fn GetBorder(self: *const c_void, flags: i32) -> wxBorder;
        // CXX_UNSUPPORTED: pub fn GetBorder1(self: *const c_void) -> wxBorder;
        // GENERATED: pub fn DoUpdateWindowUI(self: *mut c_void, event: *mut c_void);
        // CXX_UNSUPPORTED: pub fn GetHandle(self: *const c_void) -> WXWidget;
        // GENERATED: pub fn HasMultiplePages(self: *const c_void) -> bool;
        // GENERATED: pub fn InheritAttributes(self: *mut c_void);
        // GENERATED: pub fn InitDialog(self: *mut c_void);
        // GENERATED: pub fn IsDoubleBuffered(self: *const c_void) -> bool;
        // GENERATED: pub fn SetDoubleBuffered(self: *mut c_void, on: bool);
        // GENERATED: pub fn IsRetained(self: *const c_void) -> bool;
        // GENERATED: pub fn IsThisEnabled(self: *const c_void) -> bool;
        // GENERATED: pub fn IsTopLevel(self: *const c_void) -> bool;
        // GENERATED: pub fn OnInternalIdle(self: *mut c_void);
        // GENERATED: pub fn SendIdleEvents(self: *mut c_void, event: *mut c_void) -> bool;
        // GENERATED: pub fn RegisterHotKey(self: *mut c_void, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        // GENERATED: pub fn UnregisterHotKey(self: *mut c_void, hotkey_id: i32) -> bool;
        // GENERATED: pub fn UpdateWindowUI(self: *mut c_void, flags: i32);
        // CXX_UNSUPPORTED: pub fn GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        // GENERATED: pub fn FindFocus() -> *mut c_void;
        // GENERATED: pub fn FindWindowById(id: i32, parent: *const c_void) -> *mut c_void;
        // GENERATED: pub fn FindWindowByLabel(label: *const c_void, parent: *const c_void) -> *mut c_void;
        // GENERATED: pub fn FindWindowByName(name: *const c_void, parent: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetCapture() -> *mut c_void;
        // GENERATED: pub fn NewControlId(count: i32) -> i32;
        // GENERATED: pub fn UnreserveControlId(id: i32, count: i32);
        // CTOR: pub fn wxWindow() -> *mut c_void;
        // CTOR: pub fn wxWindow1(parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        // DTOR: pub fn ~wxWindow(self: *mut c_void);
        // GENERATED: pub fn Create(self: *mut c_void, parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        
        // CLASS: wxControl
        type wxControl;
        // CTOR: pub fn wxControl(parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> *mut c_void;
        // CTOR: pub fn wxControl1() -> *mut c_void;
        // GENERATED: pub fn Create(self: *mut c_void, parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> bool;
        // GENERATED: pub fn Command(self: *mut c_void, event: *mut c_void);
        // GENERATED: pub fn GetLabel(self: *const c_void) -> wxString;
        // GENERATED: pub fn GetLabelText(self: *const c_void) -> wxString;
        // GENERATED: pub fn GetSizeFromTextSize(self: *const c_void, xlen: i32, ylen: i32) -> wxSize;
        // GENERATED: pub fn GetSizeFromTextSize1(self: *const c_void, tsize: *const c_void) -> wxSize;
        // GENERATED: pub fn GetSizeFromText(self: *const c_void, text: *const c_void) -> wxSize;
        // GENERATED: pub fn SetLabel(self: *mut c_void, label: *const c_void);
        // GENERATED: pub fn SetLabelText(self: *mut c_void, text: *const c_void);
        // GENERATED: pub fn SetLabelMarkup(self: *mut c_void, markup: *const c_void) -> bool;
        // GENERATED: pub fn GetLabelText1(label: *const c_void) -> wxString;
        // GENERATED: pub fn RemoveMnemonics(str: *const c_void) -> wxString;
        // GENERATED: pub fn EscapeMnemonics(text: *const c_void) -> wxString;
        // BLOCKED: pub fn Ellipsize(label: *const c_void, dc: *const c_void, mode: i32, max_width: i32, flags: i32) -> wxString;
        
        // CLASS: wxAnyButton
        type wxAnyButton;
        // CTOR: pub fn wxAnyButton() -> *mut c_void;
        // DTOR: pub fn ~wxAnyButton(self: *mut c_void);
        // CXX_UNSUPPORTED: pub fn GetBitmap(self: *const c_void) -> wxBitmap;
        // CXX_UNSUPPORTED: pub fn GetBitmapCurrent(self: *const c_void) -> wxBitmap;
        // CXX_UNSUPPORTED: pub fn GetBitmapDisabled(self: *const c_void) -> wxBitmap;
        // CXX_UNSUPPORTED: pub fn GetBitmapFocus(self: *const c_void) -> wxBitmap;
        // CXX_UNSUPPORTED: pub fn GetBitmapLabel(self: *const c_void) -> wxBitmap;
        // CXX_UNSUPPORTED: pub fn GetBitmapPressed(self: *const c_void) -> wxBitmap;
        // CXX_UNSUPPORTED: pub fn SetBitmap(self: *mut c_void, bitmap: *const c_void, dir: wxDirection);
        // GENERATED: pub fn SetBitmapCurrent(self: *mut c_void, bitmap: *const c_void);
        // GENERATED: pub fn SetBitmapDisabled(self: *mut c_void, bitmap: *const c_void);
        // GENERATED: pub fn SetBitmapFocus(self: *mut c_void, bitmap: *const c_void);
        // GENERATED: pub fn SetBitmapLabel(self: *mut c_void, bitmap: *const c_void);
        // GENERATED: pub fn SetBitmapPressed(self: *mut c_void, bitmap: *const c_void);
        // GENERATED: pub fn GetBitmapMargins(self: *mut c_void) -> wxSize;
        // GENERATED: pub fn SetBitmapMargins(self: *mut c_void, x: i32, y: i32);
        // GENERATED: pub fn SetBitmapMargins1(self: *mut c_void, sz: *const c_void);
        // CXX_UNSUPPORTED: pub fn SetBitmapPosition(self: *mut c_void, dir: wxDirection);
        
        // CLASS: wxButton
        type wxButton;
        // CTOR: pub fn wxButton() -> *mut c_void;
        // CTOR: pub fn wxButton1(parent: *mut c_void, id: i32, label: *const c_void, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> *mut c_void;
        // GENERATED: pub fn Create(self: *mut c_void, parent: *mut c_void, id: i32, label: *const c_void, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> bool;
        // GENERATED: pub fn GetAuthNeeded(self: *const c_void) -> bool;
        // GENERATED: pub fn GetLabel(self: *const c_void) -> wxString;
        // GENERATED: pub fn SetAuthNeeded(self: *mut c_void, needed: bool);
        // GENERATED: pub fn SetDefault(self: *mut c_void) -> *mut c_void;
        // GENERATED: pub fn SetLabel(self: *mut c_void, label: *const c_void);
        // GENERATED: pub fn GetDefaultSize(win: *mut c_void) -> wxSize;
        
        // CLASS: wxNonOwnedWindow
        type wxNonOwnedWindow;
        // GENERATED: pub fn SetShape(self: *mut c_void, region: *const c_void) -> bool;
        // GENERATED: pub fn SetShape1(self: *mut c_void, path: *const c_void) -> bool;
        
        // CLASS: wxTopLevelWindow
        type wxTopLevelWindow;
        // CTOR: pub fn wxTopLevelWindow() -> *mut c_void;
        // CTOR: pub fn wxTopLevelWindow1(parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        // DTOR: pub fn ~wxTopLevelWindow(self: *mut c_void);
        // GENERATED: pub fn Create(self: *mut c_void, parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        // GENERATED: pub fn CanSetTransparent(self: *mut c_void) -> bool;
        // GENERATED: pub fn CenterOnScreen(self: *mut c_void, direction: i32);
        // GENERATED: pub fn CentreOnScreen(self: *mut c_void, direction: i32);
        // GENERATED: pub fn EnableCloseButton(self: *mut c_void, enable: bool) -> bool;
        // GENERATED: pub fn EnableMaximizeButton(self: *mut c_void, enable: bool) -> bool;
        // GENERATED: pub fn EnableMinimizeButton(self: *mut c_void, enable: bool) -> bool;
        // GENERATED: pub fn GetDefaultItem(self: *const c_void) -> *mut c_void;
        // CXX_UNSUPPORTED: pub fn GetIcon(self: *const c_void) -> wxIcon;
        // BLOCKED: pub fn GetIcons(self: *const c_void) -> *const c_void;
        // GENERATED: pub fn GetTitle(self: *const c_void) -> wxString;
        // GENERATED: pub fn Iconize(self: *mut c_void, iconize: bool);
        // GENERATED: pub fn IsActive(self: *mut c_void) -> bool;
        // GENERATED: pub fn IsAlwaysMaximized(self: *const c_void) -> bool;
        // GENERATED: pub fn IsFullScreen(self: *const c_void) -> bool;
        // GENERATED: pub fn IsIconized(self: *const c_void) -> bool;
        // GENERATED: pub fn IsMaximized(self: *const c_void) -> bool;
        // BLOCKED: pub fn IsUsingNativeDecorations(self: *const c_void) -> bool;
        // GENERATED: pub fn Layout(self: *mut c_void) -> bool;
        // GENERATED: pub fn Maximize(self: *mut c_void, maximize: bool);
        // BLOCKED: pub fn MSWGetSystemMenu(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn RequestUserAttention(self: *mut c_void, flags: i32);
        // GENERATED: pub fn Restore(self: *mut c_void);
        // BLOCKED: pub fn RestoreToGeometry(self: *mut c_void, ser: *mut c_void) -> bool;
        // BLOCKED: pub fn SaveGeometry(self: *const c_void, ser: *const c_void) -> bool;
        // GENERATED: pub fn SetDefaultItem(self: *mut c_void, win: *mut c_void) -> *mut c_void;
        // GENERATED: pub fn SetTmpDefaultItem(self: *mut c_void, win: *mut c_void) -> *mut c_void;
        // GENERATED: pub fn GetTmpDefaultItem(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn SetIcon(self: *mut c_void, icon: *const c_void);
        // GENERATED: pub fn SetIcons(self: *mut c_void, icons: *const c_void);
        // GENERATED: pub fn SetMaxSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetMinSize(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn SetSizeHints(self: *mut c_void, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: pub fn SetSizeHints1(self: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        // GENERATED: pub fn SetTitle(self: *mut c_void, title: *const c_void);
        // GENERATED: pub fn SetTransparent(self: *mut c_void, alpha: u8) -> bool;
        // GENERATED: pub fn ShouldPreventAppExit(self: *const c_void) -> bool;
        // GENERATED: pub fn OSXSetModified(self: *mut c_void, modified: bool);
        // GENERATED: pub fn OSXIsModified(self: *const c_void) -> bool;
        // GENERATED: pub fn SetRepresentedFilename(self: *mut c_void, filename: *const c_void);
        // GENERATED: pub fn ShowWithoutActivating(self: *mut c_void);
        // GENERATED: pub fn EnableFullScreenView(self: *mut c_void, enable: bool) -> bool;
        // GENERATED: pub fn ShowFullScreen(self: *mut c_void, show: bool, style: i32) -> bool;
        // BLOCKED: pub fn UseNativeDecorations(self: *mut c_void, native: bool);
        // BLOCKED: pub fn UseNativeDecorationsByDefault(self: *mut c_void, native: bool);
        // GENERATED: pub fn GetDefaultSize() -> wxSize;
        
        // CLASS: wxFrame
        type wxFrame;
        // CTOR: pub fn wxFrame() -> *mut c_void;
        // CTOR: pub fn wxFrame1(parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        // DTOR: pub fn ~wxFrame(self: *mut c_void);
        // GENERATED: pub fn Centre(self: *mut c_void, direction: i32);
        // GENERATED: pub fn Create(self: *mut c_void, parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        // GENERATED: pub fn CreateStatusBar(self: *mut c_void, number: i32, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        // GENERATED: pub fn CreateToolBar(self: *mut c_void, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        // GENERATED: pub fn DoGiveHelp(self: *mut c_void, text: *const c_void, show: bool);
        // GENERATED: pub fn GetClientAreaOrigin(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetMenuBar(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetStatusBar(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetStatusBarPane(self: *const c_void) -> i32;
        // GENERATED: pub fn GetToolBar(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn OnCreateStatusBar(self: *mut c_void, number: i32, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        // GENERATED: pub fn OnCreateToolBar(self: *mut c_void, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        // GENERATED: pub fn ProcessCommand(self: *mut c_void, id: i32) -> bool;
        // GENERATED: pub fn SetMenuBar(self: *mut c_void, menu_bar: *mut c_void);
        // GENERATED: pub fn SetStatusBar(self: *mut c_void, status_bar: *mut c_void);
        // GENERATED: pub fn SetStatusBarPane(self: *mut c_void, n: i32);
        // GENERATED: pub fn SetStatusText(self: *mut c_void, text: *const c_void, number: i32);
        // GENERATED: pub fn SetStatusWidths(self: *mut c_void, n: i32, widths_field: *const c_void);
        // GENERATED: pub fn SetToolBar(self: *mut c_void, tool_bar: *mut c_void);
        // BLOCKED: pub fn MSWGetTaskBarButton(self: *mut c_void) -> *mut c_void;
        // GENERATED: pub fn PushStatusText(self: *mut c_void, text: *const c_void, number: i32);
        // GENERATED: pub fn PopStatusText(self: *mut c_void, number: i32);
        
        // CLASS: wxPoint
        type wxPoint;
        // GENERATED: pub fn IsFullySpecified(self: *const c_void) -> bool;
        // GENERATED: pub fn SetDefaults(self: *mut c_void, pt: *const c_void);
        // BLOCKED: pub fn operator=(self: *mut c_void, pt: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator==(self: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
        // BLOCKED: pub fn operator!=(self: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
        // BLOCKED: pub fn operator+(self: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
        // BLOCKED: pub fn operator-(self: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
        // BLOCKED: pub fn operator+=(self: *mut c_void, pt: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator-=(self: *mut c_void, pt: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator+1(self: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
        // BLOCKED: pub fn operator-1(self: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
        // BLOCKED: pub fn operator+2(self: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
        // BLOCKED: pub fn operator-2(self: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
        // BLOCKED: pub fn operator+=1(self: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator-=1(self: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator/(self: *mut c_void, sz: *const c_void, factor: i32) -> wxSize;
        // BLOCKED: pub fn operator*(self: *mut c_void, sz: *const c_void, factor: i32) -> wxSize;
        // BLOCKED: pub fn operator*1(self: *mut c_void, factor: i32, sz: *const c_void) -> wxSize;
        // BLOCKED: pub fn operator/=(self: *mut c_void, factor: i32) -> *mut c_void;
        // BLOCKED: pub fn operator*=(self: *mut c_void, factor: i32) -> *mut c_void;
        // CTOR: pub fn wxPoint() -> *mut c_void;
        // CTOR: pub fn wxPoint1(x: i32, y: i32) -> *mut c_void;
        // CTOR: pub fn wxPoint2(pt: *const c_void) -> *mut c_void;
        
        // CLASS: wxRect
        type wxRect;
        // CTOR: pub fn wxRect() -> *mut c_void;
        // CTOR: pub fn wxRect1(x: i32, y: i32, width: i32, height: i32) -> *mut c_void;
        // CTOR: pub fn wxRect2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
        // CTOR: pub fn wxRect3(pos: *const c_void, size: *const c_void) -> *mut c_void;
        // CTOR: pub fn wxRect4(size: *const c_void) -> *mut c_void;
        // GENERATED: pub fn CentreIn(self: *const c_void, r: *const c_void, dir: i32) -> wxRect;
        // GENERATED: pub fn CenterIn(self: *const c_void, r: *const c_void, dir: i32) -> wxRect;
        // GENERATED: pub fn Contains(self: *const c_void, x: i32, y: i32) -> bool;
        // GENERATED: pub fn Contains1(self: *const c_void, pt: *const c_void) -> bool;
        // GENERATED: pub fn Contains2(self: *const c_void, rect: *const c_void) -> bool;
        // BLOCKED: pub fn Deflate(self: *mut c_void, dx: i32, dy: i32) -> *mut c_void;
        // BLOCKED: pub fn Deflate1(self: *mut c_void, diff: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn Deflate2(self: *mut c_void, diff: i32) -> *mut c_void;
        // GENERATED: pub fn Deflate3(self: *const c_void, dx: i32, dy: i32) -> wxRect;
        // GENERATED: pub fn GetBottom(self: *const c_void) -> i32;
        // GENERATED: pub fn GetBottomLeft(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetBottomRight(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetHeight(self: *const c_void) -> i32;
        // GENERATED: pub fn GetLeft(self: *const c_void) -> i32;
        // GENERATED: pub fn GetPosition(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetRight(self: *const c_void) -> i32;
        // GENERATED: pub fn GetSize(self: *const c_void) -> wxSize;
        // GENERATED: pub fn GetTop(self: *const c_void) -> i32;
        // GENERATED: pub fn GetTopLeft(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetTopRight(self: *const c_void) -> wxPoint;
        // GENERATED: pub fn GetWidth(self: *const c_void) -> i32;
        // GENERATED: pub fn GetX(self: *const c_void) -> i32;
        // GENERATED: pub fn GetY(self: *const c_void) -> i32;
        // BLOCKED: pub fn Inflate(self: *mut c_void, dx: i32, dy: i32) -> *mut c_void;
        // BLOCKED: pub fn Inflate1(self: *mut c_void, diff: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn Inflate2(self: *mut c_void, diff: i32) -> *mut c_void;
        // GENERATED: pub fn Inflate3(self: *const c_void, dx: i32, dy: i32) -> wxRect;
        // BLOCKED: pub fn Intersect(self: *mut c_void, rect: *const c_void) -> *mut c_void;
        // GENERATED: pub fn Intersect1(self: *const c_void, rect: *const c_void) -> wxRect;
        // GENERATED: pub fn Intersects(self: *const c_void, rect: *const c_void) -> bool;
        // GENERATED: pub fn IsEmpty(self: *const c_void) -> bool;
        // GENERATED: pub fn Offset(self: *mut c_void, dx: i32, dy: i32);
        // GENERATED: pub fn Offset1(self: *mut c_void, pt: *const c_void);
        // GENERATED: pub fn SetHeight(self: *mut c_void, height: i32);
        // GENERATED: pub fn SetPosition(self: *mut c_void, pos: *const c_void);
        // GENERATED: pub fn SetSize(self: *mut c_void, s: *const c_void);
        // GENERATED: pub fn SetWidth(self: *mut c_void, width: i32);
        // GENERATED: pub fn SetX(self: *mut c_void, x: i32);
        // GENERATED: pub fn SetY(self: *mut c_void, y: i32);
        // GENERATED: pub fn SetLeft(self: *mut c_void, left: i32);
        // GENERATED: pub fn SetRight(self: *mut c_void, right: i32);
        // GENERATED: pub fn SetTop(self: *mut c_void, top: i32);
        // GENERATED: pub fn SetBottom(self: *mut c_void, bottom: i32);
        // GENERATED: pub fn SetTopLeft(self: *mut c_void, p: *const c_void);
        // GENERATED: pub fn SetBottomRight(self: *mut c_void, p: *const c_void);
        // GENERATED: pub fn SetTopRight(self: *mut c_void, p: *const c_void);
        // GENERATED: pub fn SetBottomLeft(self: *mut c_void, p: *const c_void);
        // GENERATED: pub fn Union(self: *const c_void, rect: *const c_void) -> wxRect;
        // BLOCKED: pub fn Union1(self: *mut c_void, rect: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator!=(self: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;
        // BLOCKED: pub fn operator+(self: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
        // BLOCKED: pub fn operator+=(self: *mut c_void, r: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator*(self: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
        // BLOCKED: pub fn operator*=(self: *mut c_void, r: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator=(self: *mut c_void, rect: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator==(self: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;
        
        // CLASS: wxSize
        type wxSize;
        // BLOCKED: pub fn operator=(self: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator==(self: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
        // BLOCKED: pub fn operator!=(self: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
        // BLOCKED: pub fn operator+(self: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
        // BLOCKED: pub fn operator-(self: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
        // BLOCKED: pub fn operator+=(self: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator-=(self: *mut c_void, sz: *const c_void) -> *mut c_void;
        // BLOCKED: pub fn operator/(self: *mut c_void, sz: *const c_void, factor: i32) -> wxSize;
        // BLOCKED: pub fn operator*(self: *mut c_void, sz: *const c_void, factor: i32) -> wxSize;
        // BLOCKED: pub fn operator*1(self: *mut c_void, factor: i32, sz: *const c_void) -> wxSize;
        // BLOCKED: pub fn operator/=(self: *mut c_void, factor: i32) -> *mut c_void;
        // BLOCKED: pub fn operator*=(self: *mut c_void, factor: i32) -> *mut c_void;
        // CTOR: pub fn wxSize() -> *mut c_void;
        // CTOR: pub fn wxSize1(width: i32, height: i32) -> *mut c_void;
        // GENERATED: pub fn DecBy(self: *mut c_void, pt: *const c_void);
        // GENERATED: pub fn DecBy1(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn DecBy2(self: *mut c_void, dx: i32, dy: i32);
        // GENERATED: pub fn DecBy3(self: *mut c_void, d: i32);
        // GENERATED: pub fn DecTo(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn DecToIfSpecified(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn GetHeight(self: *const c_void) -> i32;
        // GENERATED: pub fn GetWidth(self: *const c_void) -> i32;
        // GENERATED: pub fn IncBy(self: *mut c_void, pt: *const c_void);
        // GENERATED: pub fn IncBy1(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn IncBy2(self: *mut c_void, dx: i32, dy: i32);
        // GENERATED: pub fn IncBy3(self: *mut c_void, d: i32);
        // GENERATED: pub fn IncTo(self: *mut c_void, size: *const c_void);
        // GENERATED: pub fn IsFullySpecified(self: *const c_void) -> bool;
        // BLOCKED: pub fn Scale(self: *mut c_void, xscale: f64, yscale: f64) -> *mut c_void;
        // GENERATED: pub fn Set(self: *mut c_void, width: i32, height: i32);
        // GENERATED: pub fn SetDefaults(self: *mut c_void, size_default: *const c_void);
        // GENERATED: pub fn SetHeight(self: *mut c_void, height: i32);
        // GENERATED: pub fn SetWidth(self: *mut c_void, width: i32);
        
        // CLASS: wxValidator
        type wxValidator;
        // CTOR: pub fn wxValidator() -> *mut c_void;
        // DTOR: pub fn ~wxValidator(self: *mut c_void);
        // GENERATED: pub fn Clone(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn GetWindow(self: *const c_void) -> *mut c_void;
        // GENERATED: pub fn SetWindow(self: *mut c_void, window: *mut c_void);
        // GENERATED: pub fn TransferFromWindow(self: *mut c_void) -> bool;
        // GENERATED: pub fn TransferToWindow(self: *mut c_void) -> bool;
        // GENERATED: pub fn Validate(self: *mut c_void, parent: *mut c_void) -> bool;
        // GENERATED: pub fn SuppressBellOnError(suppress: bool);
        // GENERATED: pub fn IsSilent() -> bool;
    }
}
mod ffi2 {
    use std::os::raw::c_void;
    extern "C" {
        // CLASS: wxObject
        pub fn wxObject_new() -> *mut c_void;
        pub fn wxObject_new1(other: *const c_void) -> *mut c_void;
        pub fn wxObject_GetClassInfo(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_GetRefData(self_: *const c_void) -> *mut c_void;
        pub fn wxObject_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
        pub fn wxObject_IsSameAs(self_: *const c_void, obj: *const c_void) -> bool;
        pub fn wxObject_Ref(self_: *mut c_void, clone: *const c_void);
        pub fn wxObject_SetRefData(self_: *mut c_void, data: *mut c_void);
        pub fn wxObject_UnRef(self_: *mut c_void);
        pub fn wxObject_UnShare(self_: *mut c_void);
        // CLASS: wxEvtHandler
        pub fn wxEvtHandler_QueueEvent(self_: *mut c_void, event: *mut c_void);
        pub fn wxEvtHandler_AddPendingEvent(self_: *mut c_void, event: *const c_void);
        pub fn wxEvtHandler_ProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_SafelyProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxEvtHandler_ProcessPendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_DeletePendingEvents(self_: *mut c_void);
        pub fn wxEvtHandler_GetClientObject(self_: *const c_void) -> *mut c_void;
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
        // CLASS: wxWindow
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
        pub fn wxWindow_FindWindow(self_: *const c_void, id: i32) -> *mut c_void;
        pub fn wxWindow_FindWindow1(self_: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxWindow_RemoveChild(self_: *mut c_void, child: *mut c_void);
        pub fn wxWindow_GetGrandParent(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetNextSibling(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetParent(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetPrevSibling(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_IsDescendant(self_: *const c_void, win: *mut c_void) -> bool;
        pub fn wxWindow_Reparent(self_: *mut c_void, new_parent: *mut c_void) -> bool;
        pub fn wxWindow_AlwaysShowScrollbars(self_: *mut c_void, hflag: bool, vflag: bool);
        pub fn wxWindow_GetScrollPos(self_: *const c_void, orientation: i32) -> i32;
        pub fn wxWindow_GetScrollRange(self_: *const c_void, orientation: i32) -> i32;
        pub fn wxWindow_GetScrollThumb(self_: *const c_void, orientation: i32) -> i32;
        pub fn wxWindow_CanScroll(self_: *const c_void, orient: i32) -> bool;
        pub fn wxWindow_HasScrollbar(self_: *const c_void, orient: i32) -> bool;
        pub fn wxWindow_IsScrollbarAlwaysShown(self_: *const c_void, orient: i32) -> bool;
        pub fn wxWindow_ScrollLines(self_: *mut c_void, lines: i32) -> bool;
        pub fn wxWindow_ScrollPages(self_: *mut c_void, pages: i32) -> bool;
        pub fn wxWindow_ScrollWindow(self_: *mut c_void, dx: i32, dy: i32, rect: *const c_void);
        pub fn wxWindow_LineUp(self_: *mut c_void) -> bool;
        pub fn wxWindow_LineDown(self_: *mut c_void) -> bool;
        pub fn wxWindow_PageUp(self_: *mut c_void) -> bool;
        pub fn wxWindow_PageDown(self_: *mut c_void) -> bool;
        pub fn wxWindow_SetScrollPos(self_: *mut c_void, orientation: i32, pos: i32, refresh: bool);
        pub fn wxWindow_SetScrollbar(self_: *mut c_void, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        pub fn wxWindow_BeginRepositioningChildren(self_: *mut c_void) -> bool;
        pub fn wxWindow_EndRepositioningChildren(self_: *mut c_void);
        pub fn wxWindow_CacheBestSize(self_: *const c_void, size: *const c_void);
        pub fn wxWindow_ClientToWindowSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxWindow_WindowToClientSize(self_: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxWindow_Fit(self_: *mut c_void);
        pub fn wxWindow_FitInside(self_: *mut c_void);
        pub fn wxWindow_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP2(self_: *const c_void, d: i32) -> i32;
        pub fn wxWindow_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP2(self_: *const c_void, d: i32) -> i32;
        pub fn wxWindow_GetBestSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetBestHeight(self_: *const c_void, width: i32) -> i32;
        pub fn wxWindow_GetBestWidth(self_: *const c_void, height: i32) -> i32;
        pub fn wxWindow_GetClientSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetClientSize1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetEffectiveMinSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMaxClientSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMaxSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinClientSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetMinWidth(self_: *const c_void) -> i32;
        pub fn wxWindow_GetMinHeight(self_: *const c_void) -> i32;
        pub fn wxWindow_GetMaxWidth(self_: *const c_void) -> i32;
        pub fn wxWindow_GetMaxHeight(self_: *const c_void) -> i32;
        pub fn wxWindow_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetSize1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetVirtualSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetVirtualSize1(self_: *const c_void, width: *mut c_void, height: *mut c_void);
        pub fn wxWindow_GetBestVirtualSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetContentScaleFactor(self_: *const c_void) -> f64;
        pub fn wxWindow_GetDPIScaleFactor(self_: *const c_void) -> f64;
        pub fn wxWindow_GetWindowBorderSize(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_InformFirstDirection(self_: *mut c_void, direction: i32, size: i32, available_other_dir: i32) -> bool;
        pub fn wxWindow_InvalidateBestSize(self_: *mut c_void);
        pub fn wxWindow_PostSizeEvent(self_: *mut c_void);
        pub fn wxWindow_PostSizeEventToParent(self_: *mut c_void);
        pub fn wxWindow_SendSizeEvent(self_: *mut c_void, flags: i32);
        pub fn wxWindow_SendSizeEventToParent(self_: *mut c_void, flags: i32);
        pub fn wxWindow_SetClientSize(self_: *mut c_void, width: i32, height: i32);
        pub fn wxWindow_SetClientSize1(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetClientSize2(self_: *mut c_void, rect: *const c_void);
        pub fn wxWindow_SetContainingSizer(self_: *mut c_void, sizer: *mut c_void);
        pub fn wxWindow_SetInitialSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMaxClientSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMaxSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMinClientSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetMinSize(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetSize(self_: *mut c_void, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        pub fn wxWindow_SetSize1(self_: *mut c_void, rect: *const c_void);
        pub fn wxWindow_SetSize2(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_SetSize3(self_: *mut c_void, width: i32, height: i32);
        pub fn wxWindow_SetSizeHints(self_: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        pub fn wxWindow_SetSizeHints1(self_: *mut c_void, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        pub fn wxWindow_SetVirtualSize(self_: *mut c_void, width: i32, height: i32);
        pub fn wxWindow_SetVirtualSize1(self_: *mut c_void, size: *const c_void);
        pub fn wxWindow_FromDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_FromDIP5(d: i32, w: *const c_void) -> i32;
        pub fn wxWindow_ToDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
        pub fn wxWindow_ToDIP5(d: i32, w: *const c_void) -> i32;
        pub fn wxWindow_Center(self_: *mut c_void, dir: i32);
        pub fn wxWindow_CenterOnParent(self_: *mut c_void, dir: i32);
        pub fn wxWindow_Centre(self_: *mut c_void, direction: i32);
        pub fn wxWindow_CentreOnParent(self_: *mut c_void, direction: i32);
        pub fn wxWindow_GetPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_GetPosition1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetScreenPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
        pub fn wxWindow_GetScreenPosition1(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetScreenRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetClientAreaOrigin(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetClientRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_Move(self_: *mut c_void, x: i32, y: i32, flags: i32);
        pub fn wxWindow_Move1(self_: *mut c_void, pt: *const c_void, flags: i32);
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
        pub fn wxWindow_GetCharHeight(self_: *const c_void) -> i32;
        pub fn wxWindow_GetCharWidth(self_: *const c_void) -> i32;
        pub fn wxWindow_GetDPI(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetTextExtent(self_: *const c_void, string: *const c_void, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void);
        pub fn wxWindow_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetUpdateClientRect(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HasTransparentBackground(self_: *mut c_void) -> bool;
        pub fn wxWindow_Refresh(self_: *mut c_void, erase_background: bool, rect: *const c_void);
        pub fn wxWindow_RefreshRect(self_: *mut c_void, rect: *const c_void, erase_background: bool);
        pub fn wxWindow_Update(self_: *mut c_void);
        pub fn wxWindow_SetBackgroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
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
        pub fn wxWindow_SetTransparent(self_: *mut c_void, alpha: u8) -> bool;
        pub fn wxWindow_GetEventHandler(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_HandleAsNavigationKey(self_: *mut c_void, event: *const c_void) -> bool;
        pub fn wxWindow_HandleWindowEvent(self_: *const c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_ProcessWindowEvent(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_ProcessWindowEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
        pub fn wxWindow_PopEventHandler(self_: *mut c_void, delete_handler: bool) -> *mut c_void;
        pub fn wxWindow_PushEventHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_RemoveEventHandler(self_: *mut c_void, handler: *mut c_void) -> bool;
        pub fn wxWindow_SetEventHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_SetPreviousHandler(self_: *mut c_void, handler: *mut c_void);
        pub fn wxWindow_GetExtraStyle(self_: *const c_void) -> i32;
        pub fn wxWindow_GetWindowStyleFlag(self_: *const c_void) -> i32;
        pub fn wxWindow_GetWindowStyle(self_: *const c_void) -> i32;
        pub fn wxWindow_HasExtraStyle(self_: *const c_void, ex_flag: i32) -> bool;
        pub fn wxWindow_HasFlag(self_: *const c_void, flag: i32) -> bool;
        pub fn wxWindow_SetExtraStyle(self_: *mut c_void, ex_style: i32);
        pub fn wxWindow_SetWindowStyleFlag(self_: *mut c_void, style: i32);
        pub fn wxWindow_SetWindowStyle(self_: *mut c_void, style: i32);
        pub fn wxWindow_ToggleWindowStyle(self_: *mut c_void, flag: i32) -> bool;
        pub fn wxWindow_MoveAfterInTabOrder(self_: *mut c_void, win: *mut c_void);
        pub fn wxWindow_MoveBeforeInTabOrder(self_: *mut c_void, win: *mut c_void);
        pub fn wxWindow_Navigate(self_: *mut c_void, flags: i32) -> bool;
        pub fn wxWindow_NavigateIn(self_: *mut c_void, flags: i32) -> bool;
        pub fn wxWindow_Lower(self_: *mut c_void);
        pub fn wxWindow_Raise(self_: *mut c_void);
        pub fn wxWindow_Hide(self_: *mut c_void) -> bool;
        pub fn wxWindow_IsEnabled(self_: *const c_void) -> bool;
        pub fn wxWindow_IsExposed(self_: *const c_void, x: i32, y: i32) -> bool;
        pub fn wxWindow_IsExposed1(self_: *const c_void, pt: *mut c_void) -> bool;
        pub fn wxWindow_IsExposed2(self_: *const c_void, x: i32, y: i32, w: i32, h: i32) -> bool;
        pub fn wxWindow_IsExposed3(self_: *const c_void, rect: *mut c_void) -> bool;
        pub fn wxWindow_IsShown(self_: *const c_void) -> bool;
        pub fn wxWindow_IsShownOnScreen(self_: *const c_void) -> bool;
        pub fn wxWindow_Disable(self_: *mut c_void) -> bool;
        pub fn wxWindow_Enable(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxWindow_Show(self_: *mut c_void, show: bool) -> bool;
        pub fn wxWindow_GetHelpText(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetHelpText(self_: *mut c_void, help_text: *const c_void);
        pub fn wxWindow_GetToolTip(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetToolTipText(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetToolTip(self_: *mut c_void, tip_string: *const c_void);
        pub fn wxWindow_SetToolTip1(self_: *mut c_void, tip: *mut c_void);
        pub fn wxWindow_UnsetToolTip(self_: *mut c_void);
        pub fn wxWindow_GetPopupMenuSelectionFromUser(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> i32;
        pub fn wxWindow_GetPopupMenuSelectionFromUser1(self_: *mut c_void, menu: *mut c_void, x: i32, y: i32) -> i32;
        pub fn wxWindow_PopupMenu(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> bool;
        pub fn wxWindow_PopupMenu1(self_: *mut c_void, menu: *mut c_void, x: i32, y: i32) -> bool;
        pub fn wxWindow_GetValidator(self_: *mut c_void) -> *mut c_void;
        pub fn wxWindow_SetValidator(self_: *mut c_void, validator: *const c_void);
        pub fn wxWindow_TransferDataFromWindow(self_: *mut c_void) -> bool;
        pub fn wxWindow_TransferDataToWindow(self_: *mut c_void) -> bool;
        pub fn wxWindow_Validate(self_: *mut c_void) -> bool;
        pub fn wxWindow_GetId(self_: *const c_void) -> i32;
        pub fn wxWindow_GetLabel(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_AdjustForLayoutDirection(self_: *const c_void, x: i32, width: i32, width_total: i32) -> i32;
        pub fn wxWindow_GetName(self_: *const c_void) -> *mut c_void;
        pub fn wxWindow_SetId(self_: *mut c_void, winid: i32);
        pub fn wxWindow_SetLabel(self_: *mut c_void, label: *const c_void);
        pub fn wxWindow_SetName(self_: *mut c_void, name: *const c_void);
        pub fn wxWindow_GetAcceleratorTable(self_: *mut c_void) -> *mut c_void;
        pub fn wxWindow_SetAcceleratorTable(self_: *mut c_void, accel: *const c_void);
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
        pub fn wxWindow_HasCapture(self_: *const c_void) -> bool;
        pub fn wxWindow_ReleaseMouse(self_: *mut c_void);
        pub fn wxWindow_SetCaret(self_: *mut c_void, caret: *mut c_void);
        pub fn wxWindow_SetCursor(self_: *mut c_void, cursor: *const c_void) -> bool;
        pub fn wxWindow_WarpPointer(self_: *mut c_void, x: i32, y: i32);
        pub fn wxWindow_EnableTouchEvents(self_: *mut c_void, events_mask: i32) -> bool;
        pub fn wxWindow_DoUpdateWindowUI(self_: *mut c_void, event: *mut c_void);
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
        pub fn wxWindow_RegisterHotKey(self_: *mut c_void, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        pub fn wxWindow_UnregisterHotKey(self_: *mut c_void, hotkey_id: i32) -> bool;
        pub fn wxWindow_UpdateWindowUI(self_: *mut c_void, flags: i32);
        pub fn wxWindow_FindFocus() -> *mut c_void;
        pub fn wxWindow_FindWindowById(id: i32, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_FindWindowByLabel(label: *const c_void, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_FindWindowByName(name: *const c_void, parent: *const c_void) -> *mut c_void;
        pub fn wxWindow_GetCapture() -> *mut c_void;
        pub fn wxWindow_NewControlId(count: i32) -> i32;
        pub fn wxWindow_UnreserveControlId(id: i32, count: i32);
        pub fn wxWindow_new() -> *mut c_void;
        pub fn wxWindow_new1(parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        pub fn wxWindow_Create(self_: *mut c_void, parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        // CLASS: wxControl
        pub fn wxControl_new(parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxControl_new1() -> *mut c_void;
        pub fn wxControl_Create(self_: *mut c_void, parent: *mut c_void, id: i32, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> bool;
        pub fn wxControl_Command(self_: *mut c_void, event: *mut c_void);
        pub fn wxControl_GetLabel(self_: *const c_void) -> *mut c_void;
        pub fn wxControl_GetLabelText(self_: *const c_void) -> *mut c_void;
        pub fn wxControl_GetSizeFromTextSize(self_: *const c_void, xlen: i32, ylen: i32) -> *mut c_void;
        pub fn wxControl_GetSizeFromTextSize1(self_: *const c_void, tsize: *const c_void) -> *mut c_void;
        pub fn wxControl_GetSizeFromText(self_: *const c_void, text: *const c_void) -> *mut c_void;
        pub fn wxControl_SetLabel(self_: *mut c_void, label: *const c_void);
        pub fn wxControl_SetLabelText(self_: *mut c_void, text: *const c_void);
        pub fn wxControl_SetLabelMarkup(self_: *mut c_void, markup: *const c_void) -> bool;
        pub fn wxControl_GetLabelText1(label: *const c_void) -> *mut c_void;
        pub fn wxControl_RemoveMnemonics(str: *const c_void) -> *mut c_void;
        pub fn wxControl_EscapeMnemonics(text: *const c_void) -> *mut c_void;
        // CLASS: wxAnyButton
        pub fn wxAnyButton_new() -> *mut c_void;
        pub fn wxAnyButton_SetBitmapCurrent(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapDisabled(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapFocus(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapLabel(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_SetBitmapPressed(self_: *mut c_void, bitmap: *const c_void);
        pub fn wxAnyButton_GetBitmapMargins(self_: *mut c_void) -> *mut c_void;
        pub fn wxAnyButton_SetBitmapMargins(self_: *mut c_void, x: i32, y: i32);
        pub fn wxAnyButton_SetBitmapMargins1(self_: *mut c_void, sz: *const c_void);
        // CLASS: wxButton
        pub fn wxButton_new() -> *mut c_void;
        pub fn wxButton_new1(parent: *mut c_void, id: i32, label: *const c_void, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> *mut c_void;
        pub fn wxButton_Create(self_: *mut c_void, parent: *mut c_void, id: i32, label: *const c_void, pos: *const c_void, size: *const c_void, style: i32, validator: *const c_void, name: *const c_void) -> bool;
        pub fn wxButton_GetAuthNeeded(self_: *const c_void) -> bool;
        pub fn wxButton_GetLabel(self_: *const c_void) -> *mut c_void;
        pub fn wxButton_SetAuthNeeded(self_: *mut c_void, needed: bool);
        pub fn wxButton_SetDefault(self_: *mut c_void) -> *mut c_void;
        pub fn wxButton_SetLabel(self_: *mut c_void, label: *const c_void);
        pub fn wxButton_GetDefaultSize(win: *mut c_void) -> *mut c_void;
        // CLASS: wxNonOwnedWindow
        pub fn wxNonOwnedWindow_SetShape(self_: *mut c_void, region: *const c_void) -> bool;
        pub fn wxNonOwnedWindow_SetShape1(self_: *mut c_void, path: *const c_void) -> bool;
        // CLASS: wxTopLevelWindow
        pub fn wxTopLevelWindow_new() -> *mut c_void;
        pub fn wxTopLevelWindow_new1(parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_Create(self_: *mut c_void, parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        pub fn wxTopLevelWindow_CanSetTransparent(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_CenterOnScreen(self_: *mut c_void, direction: i32);
        pub fn wxTopLevelWindow_CentreOnScreen(self_: *mut c_void, direction: i32);
        pub fn wxTopLevelWindow_EnableCloseButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_GetDefaultItem(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_GetTitle(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_Iconize(self_: *mut c_void, iconize: bool);
        pub fn wxTopLevelWindow_IsActive(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsFullScreen(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsIconized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_IsMaximized(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_Layout(self_: *mut c_void) -> bool;
        pub fn wxTopLevelWindow_Maximize(self_: *mut c_void, maximize: bool);
        pub fn wxTopLevelWindow_RequestUserAttention(self_: *mut c_void, flags: i32);
        pub fn wxTopLevelWindow_Restore(self_: *mut c_void);
        pub fn wxTopLevelWindow_SetDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const c_void) -> *mut c_void;
        pub fn wxTopLevelWindow_SetIcon(self_: *mut c_void, icon: *const c_void);
        pub fn wxTopLevelWindow_SetIcons(self_: *mut c_void, icons: *const c_void);
        pub fn wxTopLevelWindow_SetMaxSize(self_: *mut c_void, size: *const c_void);
        pub fn wxTopLevelWindow_SetMinSize(self_: *mut c_void, size: *const c_void);
        pub fn wxTopLevelWindow_SetSizeHints(self_: *mut c_void, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        pub fn wxTopLevelWindow_SetSizeHints1(self_: *mut c_void, min_size: *const c_void, max_size: *const c_void, inc_size: *const c_void);
        pub fn wxTopLevelWindow_SetTitle(self_: *mut c_void, title: *const c_void);
        pub fn wxTopLevelWindow_SetTransparent(self_: *mut c_void, alpha: u8) -> bool;
        pub fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_OSXSetModified(self_: *mut c_void, modified: bool);
        pub fn wxTopLevelWindow_OSXIsModified(self_: *const c_void) -> bool;
        pub fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut c_void, filename: *const c_void);
        pub fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut c_void);
        pub fn wxTopLevelWindow_EnableFullScreenView(self_: *mut c_void, enable: bool) -> bool;
        pub fn wxTopLevelWindow_ShowFullScreen(self_: *mut c_void, show: bool, style: i32) -> bool;
        pub fn wxTopLevelWindow_GetDefaultSize() -> *mut c_void;
        // CLASS: wxFrame
        pub fn wxFrame_new() -> *mut c_void;
        pub fn wxFrame_new1(parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_Centre(self_: *mut c_void, direction: i32);
        pub fn wxFrame_Create(self_: *mut c_void, parent: *mut c_void, id: i32, title: *const c_void, pos: *const c_void, size: *const c_void, style: i32, name: *const c_void) -> bool;
        pub fn wxFrame_CreateStatusBar(self_: *mut c_void, number: i32, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_CreateToolBar(self_: *mut c_void, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_DoGiveHelp(self_: *mut c_void, text: *const c_void, show: bool);
        pub fn wxFrame_GetClientAreaOrigin(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetMenuBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetStatusBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_GetStatusBarPane(self_: *const c_void) -> i32;
        pub fn wxFrame_GetToolBar(self_: *const c_void) -> *mut c_void;
        pub fn wxFrame_OnCreateStatusBar(self_: *mut c_void, number: i32, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_OnCreateToolBar(self_: *mut c_void, style: i32, id: i32, name: *const c_void) -> *mut c_void;
        pub fn wxFrame_ProcessCommand(self_: *mut c_void, id: i32) -> bool;
        pub fn wxFrame_SetMenuBar(self_: *mut c_void, menu_bar: *mut c_void);
        pub fn wxFrame_SetStatusBar(self_: *mut c_void, status_bar: *mut c_void);
        pub fn wxFrame_SetStatusBarPane(self_: *mut c_void, n: i32);
        pub fn wxFrame_SetStatusText(self_: *mut c_void, text: *const c_void, number: i32);
        pub fn wxFrame_SetStatusWidths(self_: *mut c_void, n: i32, widths_field: *const c_void);
        pub fn wxFrame_SetToolBar(self_: *mut c_void, tool_bar: *mut c_void);
        pub fn wxFrame_PushStatusText(self_: *mut c_void, text: *const c_void, number: i32);
        pub fn wxFrame_PopStatusText(self_: *mut c_void, number: i32);
        // CLASS: wxPoint
        pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
        pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
        pub fn wxPoint_new() -> *mut c_void;
        pub fn wxPoint_new1(x: i32, y: i32) -> *mut c_void;
        pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;
        // CLASS: wxRect
        pub fn wxRect_new() -> *mut c_void;
        pub fn wxRect_new1(x: i32, y: i32, width: i32, height: i32) -> *mut c_void;
        pub fn wxRect_new2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
        pub fn wxRect_new3(pos: *const c_void, size: *const c_void) -> *mut c_void;
        pub fn wxRect_new4(size: *const c_void) -> *mut c_void;
        pub fn wxRect_CentreIn(self_: *const c_void, r: *const c_void, dir: i32) -> *mut c_void;
        pub fn wxRect_CenterIn(self_: *const c_void, r: *const c_void, dir: i32) -> *mut c_void;
        pub fn wxRect_Contains(self_: *const c_void, x: i32, y: i32) -> bool;
        pub fn wxRect_Contains1(self_: *const c_void, pt: *const c_void) -> bool;
        pub fn wxRect_Contains2(self_: *const c_void, rect: *const c_void) -> bool;
        pub fn wxRect_Deflate3(self_: *const c_void, dx: i32, dy: i32) -> *mut c_void;
        pub fn wxRect_GetBottom(self_: *const c_void) -> i32;
        pub fn wxRect_GetBottomLeft(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetBottomRight(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetHeight(self_: *const c_void) -> i32;
        pub fn wxRect_GetLeft(self_: *const c_void) -> i32;
        pub fn wxRect_GetPosition(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetRight(self_: *const c_void) -> i32;
        pub fn wxRect_GetSize(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetTop(self_: *const c_void) -> i32;
        pub fn wxRect_GetTopLeft(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetTopRight(self_: *const c_void) -> *mut c_void;
        pub fn wxRect_GetWidth(self_: *const c_void) -> i32;
        pub fn wxRect_GetX(self_: *const c_void) -> i32;
        pub fn wxRect_GetY(self_: *const c_void) -> i32;
        pub fn wxRect_Inflate3(self_: *const c_void, dx: i32, dy: i32) -> *mut c_void;
        pub fn wxRect_Intersect1(self_: *const c_void, rect: *const c_void) -> *mut c_void;
        pub fn wxRect_Intersects(self_: *const c_void, rect: *const c_void) -> bool;
        pub fn wxRect_IsEmpty(self_: *const c_void) -> bool;
        pub fn wxRect_Offset(self_: *mut c_void, dx: i32, dy: i32);
        pub fn wxRect_Offset1(self_: *mut c_void, pt: *const c_void);
        pub fn wxRect_SetHeight(self_: *mut c_void, height: i32);
        pub fn wxRect_SetPosition(self_: *mut c_void, pos: *const c_void);
        pub fn wxRect_SetSize(self_: *mut c_void, s: *const c_void);
        pub fn wxRect_SetWidth(self_: *mut c_void, width: i32);
        pub fn wxRect_SetX(self_: *mut c_void, x: i32);
        pub fn wxRect_SetY(self_: *mut c_void, y: i32);
        pub fn wxRect_SetLeft(self_: *mut c_void, left: i32);
        pub fn wxRect_SetRight(self_: *mut c_void, right: i32);
        pub fn wxRect_SetTop(self_: *mut c_void, top: i32);
        pub fn wxRect_SetBottom(self_: *mut c_void, bottom: i32);
        pub fn wxRect_SetTopLeft(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetBottomRight(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetTopRight(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_SetBottomLeft(self_: *mut c_void, p: *const c_void);
        pub fn wxRect_Union(self_: *const c_void, rect: *const c_void) -> *mut c_void;
        // CLASS: wxSize
        pub fn wxSize_new() -> *mut c_void;
        pub fn wxSize_new1(width: i32, height: i32) -> *mut c_void;
        pub fn wxSize_DecBy(self_: *mut c_void, pt: *const c_void);
        pub fn wxSize_DecBy1(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_DecBy2(self_: *mut c_void, dx: i32, dy: i32);
        pub fn wxSize_DecBy3(self_: *mut c_void, d: i32);
        pub fn wxSize_DecTo(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_DecToIfSpecified(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_GetHeight(self_: *const c_void) -> i32;
        pub fn wxSize_GetWidth(self_: *const c_void) -> i32;
        pub fn wxSize_IncBy(self_: *mut c_void, pt: *const c_void);
        pub fn wxSize_IncBy1(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_IncBy2(self_: *mut c_void, dx: i32, dy: i32);
        pub fn wxSize_IncBy3(self_: *mut c_void, d: i32);
        pub fn wxSize_IncTo(self_: *mut c_void, size: *const c_void);
        pub fn wxSize_IsFullySpecified(self_: *const c_void) -> bool;
        pub fn wxSize_Set(self_: *mut c_void, width: i32, height: i32);
        pub fn wxSize_SetDefaults(self_: *mut c_void, size_default: *const c_void);
        pub fn wxSize_SetHeight(self_: *mut c_void, height: i32);
        pub fn wxSize_SetWidth(self_: *mut c_void, width: i32);
        // CLASS: wxValidator
        pub fn wxValidator_new() -> *mut c_void;
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
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}

// wxObject
wx_class! { Object(wxObject) impl
    ObjectMethods
}
impl Object {
    pub fn new() -> Object {
        unsafe { Object(ffi2::wxObject_new()) }
    }
    pub fn new1(other: &Object) -> Object {
        unsafe {
            let other = other.as_ptr() as *mut c_void;
            Object(ffi2::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut c_void {
        unsafe { ffi2::wxObject_GetClassInfo(self.as_ptr() as *mut c_void) }
    }
    fn get_ref_data(&self) -> *mut c_void {
        unsafe { ffi2::wxObject_GetRefData(self.as_ptr() as *mut c_void) }
    }
    fn is_kind_of(&self, info: *const c_void) -> bool {
        unsafe { ffi2::wxObject_IsKindOf(self.as_ptr() as *mut c_void, info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        unsafe {
            let obj = obj.as_ptr() as *mut c_void;
            ffi2::wxObject_IsSameAs(self.as_ptr() as *mut c_void, obj)
        }
    }
    fn ref_(&self, clone: &Object) {
        unsafe {
            let clone = clone.as_ptr() as *mut c_void;
            ffi2::wxObject_Ref(self.as_ptr() as *mut c_void, clone)
        }
    }
    fn set_ref_data(&self, data: *mut c_void) {
        unsafe { ffi2::wxObject_SetRefData(self.as_ptr() as *mut c_void, data) }
    }
    fn un_ref(&self) {
        unsafe { ffi2::wxObject_UnRef(self.as_ptr() as *mut c_void) }
    }
    fn un_share(&self) {
        unsafe { ffi2::wxObject_UnShare(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn operator delete()
    // CXX_UNSUPPORTED: fn operator new()
}

// wxEvtHandler
wx_class! { EvtHandler(wxEvtHandler) impl
    EvtHandlerMethods,
    ObjectMethods
}
impl EvtHandler {
    pub fn new() -> EvtHandler {
        unsafe { EvtHandler(ffi2::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event(&self, event: *mut c_void) {
        unsafe { ffi2::wxEvtHandler_QueueEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn add_pending_event(&self, event: *const c_void) {
        unsafe { ffi2::wxEvtHandler_AddPendingEvent(self.as_ptr() as *mut c_void, event) }
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxEvtHandler_ProcessEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxEvtHandler_ProcessEventLocally(self.as_ptr() as *mut c_void, event) }
    }
    fn safely_process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxEvtHandler_SafelyProcessEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi2::wxEvtHandler_ProcessPendingEvents(self.as_ptr() as *mut c_void) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi2::wxEvtHandler_DeletePendingEvents(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn Connect()
    // CXX_UNSUPPORTED: fn Connect1()
    // CXX_UNSUPPORTED: fn Connect2()
    // CXX_UNSUPPORTED: fn Disconnect()
    // CXX_UNSUPPORTED: fn Disconnect1()
    // CXX_UNSUPPORTED: fn Disconnect2()
    // CXX_UNSUPPORTED: fn Bind()
    // BLOCKED: fn Bind1()
    // CXX_UNSUPPORTED: fn Unbind()
    // BLOCKED: fn Unbind1()
    // BLOCKED: fn GetClientData()
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi2::wxEvtHandler_GetClientObject(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut c_void) {
        unsafe { ffi2::wxEvtHandler_SetClientObject(self.as_ptr() as *mut c_void, data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi2::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr() as *mut c_void) }
    }
    fn get_next_handler(&self) -> *mut c_void {
        unsafe { ffi2::wxEvtHandler_GetNextHandler(self.as_ptr() as *mut c_void) }
    }
    fn get_previous_handler(&self) -> *mut c_void {
        unsafe { ffi2::wxEvtHandler_GetPreviousHandler(self.as_ptr() as *mut c_void) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi2::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr() as *mut c_void, enabled) }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxEvtHandler_SetNextHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxEvtHandler_SetPreviousHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi2::wxEvtHandler_Unlink(self.as_ptr() as *mut c_void) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi2::wxEvtHandler_IsUnlinked(self.as_ptr() as *mut c_void) }
    }
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi2::wxEvtHandler_AddFilter(filter) }
    }
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi2::wxEvtHandler_RemoveFilter(filter) }
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
        unsafe { Window(ffi2::wxWindow_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> Window {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Window(ffi2::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        unsafe { ffi2::wxWindow_AcceptsFocus(self.as_ptr() as *mut c_void) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi2::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr() as *mut c_void) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi2::wxWindow_AcceptsFocusRecursively(self.as_ptr() as *mut c_void) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi2::wxWindow_DisableFocusFromKeyboard(self.as_ptr() as *mut c_void) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi2::wxWindow_IsFocusable(self.as_ptr() as *mut c_void) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi2::wxWindow_CanAcceptFocus(self.as_ptr() as *mut c_void) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi2::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr() as *mut c_void) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi2::wxWindow_HasFocus(self.as_ptr() as *mut c_void) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi2::wxWindow_SetCanFocus(self.as_ptr() as *mut c_void, can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi2::wxWindow_EnableVisibleFocus(self.as_ptr() as *mut c_void, enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi2::wxWindow_SetFocus(self.as_ptr() as *mut c_void) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi2::wxWindow_SetFocusFromKbd(self.as_ptr() as *mut c_void) }
    }
    fn add_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_AddChild(self.as_ptr() as *mut c_void, child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi2::wxWindow_DestroyChildren(self.as_ptr() as *mut c_void) }
    }
    fn find_window(&self, id: i32) -> *mut c_void {
        unsafe { ffi2::wxWindow_FindWindow(self.as_ptr() as *mut c_void, id) }
    }
    fn find_window1(&self, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi2::wxWindow_FindWindow1(self.as_ptr() as *mut c_void, name)
        }
    }
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren1()
    fn remove_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_RemoveChild(self.as_ptr() as *mut c_void, child)
        }
    }
    fn get_grand_parent(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetGrandParent(self.as_ptr() as *mut c_void) }
    }
    fn get_next_sibling(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetNextSibling(self.as_ptr() as *mut c_void) }
    }
    fn get_parent(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetParent(self.as_ptr() as *mut c_void) }
    }
    fn get_prev_sibling(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetPrevSibling(self.as_ptr() as *mut c_void) }
    }
    fn is_descendant<T: WindowMethods>(&self, win: Option<&T>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_IsDescendant(self.as_ptr() as *mut c_void, win)
        }
    }
    fn reparent<T: WindowMethods>(&self, new_parent: Option<&T>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_Reparent(self.as_ptr() as *mut c_void, new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi2::wxWindow_AlwaysShowScrollbars(self.as_ptr() as *mut c_void, hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: i32) -> i32 {
        unsafe { ffi2::wxWindow_GetScrollPos(self.as_ptr() as *mut c_void, orientation) }
    }
    fn get_scroll_range(&self, orientation: i32) -> i32 {
        unsafe { ffi2::wxWindow_GetScrollRange(self.as_ptr() as *mut c_void, orientation) }
    }
    fn get_scroll_thumb(&self, orientation: i32) -> i32 {
        unsafe { ffi2::wxWindow_GetScrollThumb(self.as_ptr() as *mut c_void, orientation) }
    }
    fn can_scroll(&self, orient: i32) -> bool {
        unsafe { ffi2::wxWindow_CanScroll(self.as_ptr() as *mut c_void, orient) }
    }
    fn has_scrollbar(&self, orient: i32) -> bool {
        unsafe { ffi2::wxWindow_HasScrollbar(self.as_ptr() as *mut c_void, orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: i32) -> bool {
        unsafe { ffi2::wxWindow_IsScrollbarAlwaysShown(self.as_ptr() as *mut c_void, orient) }
    }
    fn scroll_lines(&self, lines: i32) -> bool {
        unsafe { ffi2::wxWindow_ScrollLines(self.as_ptr() as *mut c_void, lines) }
    }
    fn scroll_pages(&self, pages: i32) -> bool {
        unsafe { ffi2::wxWindow_ScrollPages(self.as_ptr() as *mut c_void, pages) }
    }
    fn scroll_window(&self, dx: i32, dy: i32, rect: *const c_void) {
        unsafe { ffi2::wxWindow_ScrollWindow(self.as_ptr() as *mut c_void, dx, dy, rect) }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi2::wxWindow_LineUp(self.as_ptr() as *mut c_void) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi2::wxWindow_LineDown(self.as_ptr() as *mut c_void) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi2::wxWindow_PageUp(self.as_ptr() as *mut c_void) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi2::wxWindow_PageDown(self.as_ptr() as *mut c_void) }
    }
    fn set_scroll_pos(&self, orientation: i32, pos: i32, refresh: bool) {
        unsafe { ffi2::wxWindow_SetScrollPos(self.as_ptr() as *mut c_void, orientation, pos, refresh) }
    }
    fn set_scrollbar(&self, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool) {
        unsafe { ffi2::wxWindow_SetScrollbar(self.as_ptr() as *mut c_void, orientation, position, thumb_size, range, refresh) }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi2::wxWindow_BeginRepositioningChildren(self.as_ptr() as *mut c_void) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi2::wxWindow_EndRepositioningChildren(self.as_ptr() as *mut c_void) }
    }
    fn cache_best_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_CacheBestSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn client_to_window_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            Size(ffi2::wxWindow_ClientToWindowSize(self.as_ptr() as *mut c_void, size))
        }
    }
    fn window_to_client_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            Size(ffi2::wxWindow_WindowToClientSize(self.as_ptr() as *mut c_void, size))
        }
    }
    fn fit(&self) {
        unsafe { ffi2::wxWindow_Fit(self.as_ptr() as *mut c_void) }
    }
    fn fit_inside(&self) {
        unsafe { ffi2::wxWindow_FitInside(self.as_ptr() as *mut c_void) }
    }
    fn from_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi2::wxWindow_FromDIP(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn from_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi2::wxWindow_FromDIP1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn from_dip2(&self, d: i32) -> i32 {
        unsafe { ffi2::wxWindow_FromDIP2(self.as_ptr() as *mut c_void, d) }
    }
    fn to_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi2::wxWindow_ToDIP(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn to_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi2::wxWindow_ToDIP1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn to_dip2(&self, d: i32) -> i32 {
        unsafe { ffi2::wxWindow_ToDIP2(self.as_ptr() as *mut c_void, d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetBestSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_best_height(&self, width: i32) -> i32 {
        unsafe { ffi2::wxWindow_GetBestHeight(self.as_ptr() as *mut c_void, width) }
    }
    fn get_best_width(&self, height: i32) -> i32 {
        unsafe { ffi2::wxWindow_GetBestWidth(self.as_ptr() as *mut c_void, height) }
    }
    fn get_client_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi2::wxWindow_GetClientSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn get_client_size1(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetClientSize1(self.as_ptr() as *mut c_void)) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetEffectiveMinSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetMaxClientSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetMaxSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetMinClientSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetMinSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_min_width(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetMinWidth(self.as_ptr() as *mut c_void) }
    }
    fn get_min_height(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetMinHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_max_width(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetMaxWidth(self.as_ptr() as *mut c_void) }
    }
    fn get_max_height(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetMaxHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi2::wxWindow_GetSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn get_size1(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetSize1(self.as_ptr() as *mut c_void)) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetVirtualSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_virtual_size1(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi2::wxWindow_GetVirtualSize1(self.as_ptr() as *mut c_void, width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetBestVirtualSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_content_scale_factor(&self) -> f64 {
        unsafe { ffi2::wxWindow_GetContentScaleFactor(self.as_ptr() as *mut c_void) }
    }
    fn get_dpi_scale_factor(&self) -> f64 {
        unsafe { ffi2::wxWindow_GetDPIScaleFactor(self.as_ptr() as *mut c_void) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetWindowBorderSize(self.as_ptr() as *mut c_void)) }
    }
    fn inform_first_direction(&self, direction: i32, size: i32, available_other_dir: i32) -> bool {
        unsafe { ffi2::wxWindow_InformFirstDirection(self.as_ptr() as *mut c_void, direction, size, available_other_dir) }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi2::wxWindow_InvalidateBestSize(self.as_ptr() as *mut c_void) }
    }
    fn post_size_event(&self) {
        unsafe { ffi2::wxWindow_PostSizeEvent(self.as_ptr() as *mut c_void) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi2::wxWindow_PostSizeEventToParent(self.as_ptr() as *mut c_void) }
    }
    fn send_size_event(&self, flags: i32) {
        unsafe { ffi2::wxWindow_SendSizeEvent(self.as_ptr() as *mut c_void, flags) }
    }
    fn send_size_event_to_parent(&self, flags: i32) {
        unsafe { ffi2::wxWindow_SendSizeEventToParent(self.as_ptr() as *mut c_void, flags) }
    }
    fn set_client_size(&self, width: i32, height: i32) {
        unsafe { ffi2::wxWindow_SetClientSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_client_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetClientSize1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_client_size2(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetClientSize2(self.as_ptr() as *mut c_void, rect)
        }
    }
    fn set_containing_sizer(&self, sizer: *mut c_void) {
        unsafe { ffi2::wxWindow_SetContainingSizer(self.as_ptr() as *mut c_void, sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetInitialSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_max_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetMaxClientSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetMaxSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_min_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetMinClientSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetMinSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_size(&self, x: i32, y: i32, width: i32, height: i32, size_flags: i32) {
        unsafe { ffi2::wxWindow_SetSize(self.as_ptr() as *mut c_void, x, y, width, height, size_flags) }
    }
    fn set_size1(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetSize1(self.as_ptr() as *mut c_void, rect)
        }
    }
    fn set_size2(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetSize2(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_size3(&self, width: i32, height: i32) {
        unsafe { ffi2::wxWindow_SetSize3(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr() as *mut c_void;
            let max_size = max_size.as_ptr() as *mut c_void;
            let inc_size = inc_size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetSizeHints(self.as_ptr() as *mut c_void, min_size, max_size, inc_size)
        }
    }
    fn set_size_hints1(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi2::wxWindow_SetSizeHints1(self.as_ptr() as *mut c_void, min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_virtual_size(&self, width: i32, height: i32) {
        unsafe { ffi2::wxWindow_SetVirtualSize(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_virtual_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetVirtualSize1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn from_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Size(ffi2::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Point(ffi2::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Size(ffi2::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Point(ffi2::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_ToDIP5(d, w)
        }
    }
    fn center(&self, dir: i32) {
        unsafe { ffi2::wxWindow_Center(self.as_ptr() as *mut c_void, dir) }
    }
    fn center_on_parent(&self, dir: i32) {
        unsafe { ffi2::wxWindow_CenterOnParent(self.as_ptr() as *mut c_void, dir) }
    }
    fn centre(&self, direction: i32) {
        unsafe { ffi2::wxWindow_Centre(self.as_ptr() as *mut c_void, direction) }
    }
    fn centre_on_parent(&self, direction: i32) {
        unsafe { ffi2::wxWindow_CentreOnParent(self.as_ptr() as *mut c_void, direction) }
    }
    fn get_position(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi2::wxWindow_GetPosition(self.as_ptr() as *mut c_void, x, y) }
    }
    fn get_position1(&self) -> Point {
        unsafe { Point(ffi2::wxWindow_GetPosition1(self.as_ptr() as *mut c_void)) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect(ffi2::wxWindow_GetRect(self.as_ptr() as *mut c_void)) }
    }
    fn get_screen_position(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi2::wxWindow_GetScreenPosition(self.as_ptr() as *mut c_void, x, y) }
    }
    fn get_screen_position1(&self) -> Point {
        unsafe { Point(ffi2::wxWindow_GetScreenPosition1(self.as_ptr() as *mut c_void)) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect(ffi2::wxWindow_GetScreenRect(self.as_ptr() as *mut c_void)) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi2::wxWindow_GetClientAreaOrigin(self.as_ptr() as *mut c_void)) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect(ffi2::wxWindow_GetClientRect(self.as_ptr() as *mut c_void)) }
    }
    fn move_(&self, x: i32, y: i32, flags: i32) {
        unsafe { ffi2::wxWindow_Move(self.as_ptr() as *mut c_void, x, y, flags) }
    }
    fn move1(&self, pt: &Point, flags: i32) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxWindow_Move1(self.as_ptr() as *mut c_void, pt, flags)
        }
    }
    fn set_position(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetPosition(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn client_to_screen(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi2::wxWindow_ClientToScreen(self.as_ptr() as *mut c_void, x, y) }
    }
    fn client_to_screen1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi2::wxWindow_ClientToScreen1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn convert_dialog_to_pixels(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi2::wxWindow_ConvertDialogToPixels(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn convert_dialog_to_pixels1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi2::wxWindow_ConvertDialogToPixels1(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn convert_pixels_to_dialog(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi2::wxWindow_ConvertPixelsToDialog(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn convert_pixels_to_dialog1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            Size(ffi2::wxWindow_ConvertPixelsToDialog1(self.as_ptr() as *mut c_void, sz))
        }
    }
    fn screen_to_client(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi2::wxWindow_ScreenToClient(self.as_ptr() as *mut c_void, x, y) }
    }
    fn screen_to_client1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            Point(ffi2::wxWindow_ScreenToClient1(self.as_ptr() as *mut c_void, pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi2::wxWindow_ClearBackground(self.as_ptr() as *mut c_void) }
    }
    fn freeze(&self) {
        unsafe { ffi2::wxWindow_Freeze(self.as_ptr() as *mut c_void) }
    }
    fn thaw(&self) {
        unsafe { ffi2::wxWindow_Thaw(self.as_ptr() as *mut c_void) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi2::wxWindow_IsFrozen(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetCharHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_char_width(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetCharWidth(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { Size(ffi2::wxWindow_GetDPI(self.as_ptr() as *mut c_void)) }
    }
    // CXX_UNSUPPORTED: fn GetFont()
    // CXX_UNSUPPORTED: fn GetForegroundColour()
    fn get_text_extent(&self, string: &str, w: *mut c_void, h: *mut c_void, descent: *mut c_void, external_leading: *mut c_void, font: *const c_void) {
        unsafe {
            let string = crate::wx_string_from(string);
            ffi2::wxWindow_GetTextExtent(self.as_ptr() as *mut c_void, string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent1(&self, string: &str) -> Size {
        unsafe {
            let string = crate::wx_string_from(string);
            Size(ffi2::wxWindow_GetTextExtent1(self.as_ptr() as *mut c_void, string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect(ffi2::wxWindow_GetUpdateClientRect(self.as_ptr() as *mut c_void)) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi2::wxWindow_HasTransparentBackground(self.as_ptr() as *mut c_void) }
    }
    fn refresh(&self, erase_background: bool, rect: *const c_void) {
        unsafe { ffi2::wxWindow_Refresh(self.as_ptr() as *mut c_void, erase_background, rect) }
    }
    fn refresh_rect(&self, rect: &Rect, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi2::wxWindow_RefreshRect(self.as_ptr() as *mut c_void, rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi2::wxWindow_Update(self.as_ptr() as *mut c_void) }
    }
    fn set_background_colour(&self, colour: *const c_void) -> bool {
        unsafe { ffi2::wxWindow_SetBackgroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_IsTransparentBackgroundSupported(self.as_ptr() as *mut c_void, reason) }
    }
    fn set_font(&self, font: *const c_void) -> bool {
        unsafe { ffi2::wxWindow_SetFont(self.as_ptr() as *mut c_void, font) }
    }
    fn set_foreground_colour(&self, colour: *const c_void) -> bool {
        unsafe { ffi2::wxWindow_SetForegroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    fn set_own_background_colour(&self, colour: *const c_void) {
        unsafe { ffi2::wxWindow_SetOwnBackgroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi2::wxWindow_InheritsBackgroundColour(self.as_ptr() as *mut c_void) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi2::wxWindow_UseBgCol(self.as_ptr() as *mut c_void) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi2::wxWindow_UseBackgroundColour(self.as_ptr() as *mut c_void) }
    }
    fn set_own_font(&self, font: *const c_void) {
        unsafe { ffi2::wxWindow_SetOwnFont(self.as_ptr() as *mut c_void, font) }
    }
    fn set_own_foreground_colour(&self, colour: *const c_void) {
        unsafe { ffi2::wxWindow_SetOwnForegroundColour(self.as_ptr() as *mut c_void, colour) }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi2::wxWindow_UseForegroundColour(self.as_ptr() as *mut c_void) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi2::wxWindow_InheritsForegroundColour(self.as_ptr() as *mut c_void) }
    }
    fn set_palette(&self, pal: *const c_void) {
        unsafe { ffi2::wxWindow_SetPalette(self.as_ptr() as *mut c_void, pal) }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi2::wxWindow_ShouldInheritColours(self.as_ptr() as *mut c_void) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi2::wxWindow_SetThemeEnabled(self.as_ptr() as *mut c_void, enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi2::wxWindow_GetThemeEnabled(self.as_ptr() as *mut c_void) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi2::wxWindow_CanSetTransparent(self.as_ptr() as *mut c_void) }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi2::wxWindow_SetTransparent(self.as_ptr() as *mut c_void, alpha) }
    }
    fn get_event_handler(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetEventHandler(self.as_ptr() as *mut c_void) }
    }
    fn handle_as_navigation_key(&self, event: *const c_void) -> bool {
        unsafe { ffi2::wxWindow_HandleAsNavigationKey(self.as_ptr() as *mut c_void, event) }
    }
    fn handle_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_HandleWindowEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_window_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_ProcessWindowEvent(self.as_ptr() as *mut c_void, event) }
    }
    fn process_window_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_ProcessWindowEventLocally(self.as_ptr() as *mut c_void, event) }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut c_void {
        unsafe { ffi2::wxWindow_PopEventHandler(self.as_ptr() as *mut c_void, delete_handler) }
    }
    fn push_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_PushEventHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn remove_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_RemoveEventHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_SetEventHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_SetNextHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_SetPreviousHandler(self.as_ptr() as *mut c_void, handler)
        }
    }
    fn get_extra_style(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetExtraStyle(self.as_ptr() as *mut c_void) }
    }
    fn get_window_style_flag(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetWindowStyleFlag(self.as_ptr() as *mut c_void) }
    }
    fn get_window_style(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetWindowStyle(self.as_ptr() as *mut c_void) }
    }
    fn has_extra_style(&self, ex_flag: i32) -> bool {
        unsafe { ffi2::wxWindow_HasExtraStyle(self.as_ptr() as *mut c_void, ex_flag) }
    }
    fn has_flag(&self, flag: i32) -> bool {
        unsafe { ffi2::wxWindow_HasFlag(self.as_ptr() as *mut c_void, flag) }
    }
    fn set_extra_style(&self, ex_style: i32) {
        unsafe { ffi2::wxWindow_SetExtraStyle(self.as_ptr() as *mut c_void, ex_style) }
    }
    fn set_window_style_flag(&self, style: i32) {
        unsafe { ffi2::wxWindow_SetWindowStyleFlag(self.as_ptr() as *mut c_void, style) }
    }
    fn set_window_style(&self, style: i32) {
        unsafe { ffi2::wxWindow_SetWindowStyle(self.as_ptr() as *mut c_void, style) }
    }
    fn toggle_window_style(&self, flag: i32) -> bool {
        unsafe { ffi2::wxWindow_ToggleWindowStyle(self.as_ptr() as *mut c_void, flag) }
    }
    fn move_after_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_MoveAfterInTabOrder(self.as_ptr() as *mut c_void, win)
        }
    }
    fn move_before_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_MoveBeforeInTabOrder(self.as_ptr() as *mut c_void, win)
        }
    }
    fn navigate(&self, flags: i32) -> bool {
        unsafe { ffi2::wxWindow_Navigate(self.as_ptr() as *mut c_void, flags) }
    }
    fn navigate_in(&self, flags: i32) -> bool {
        unsafe { ffi2::wxWindow_NavigateIn(self.as_ptr() as *mut c_void, flags) }
    }
    fn lower(&self) {
        unsafe { ffi2::wxWindow_Lower(self.as_ptr() as *mut c_void) }
    }
    fn raise(&self) {
        unsafe { ffi2::wxWindow_Raise(self.as_ptr() as *mut c_void) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi2::wxWindow_Hide(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi2::wxWindow_IsEnabled(self.as_ptr() as *mut c_void) }
    }
    fn is_exposed(&self, x: i32, y: i32) -> bool {
        unsafe { ffi2::wxWindow_IsExposed(self.as_ptr() as *mut c_void, x, y) }
    }
    fn is_exposed1(&self, pt: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_IsExposed1(self.as_ptr() as *mut c_void, pt) }
    }
    fn is_exposed2(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        unsafe { ffi2::wxWindow_IsExposed2(self.as_ptr() as *mut c_void, x, y, w, h) }
    }
    fn is_exposed3(&self, rect: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_IsExposed3(self.as_ptr() as *mut c_void, rect) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi2::wxWindow_IsShown(self.as_ptr() as *mut c_void) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi2::wxWindow_IsShownOnScreen(self.as_ptr() as *mut c_void) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi2::wxWindow_Disable(self.as_ptr() as *mut c_void) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi2::wxWindow_Enable(self.as_ptr() as *mut c_void, enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi2::wxWindow_Show(self.as_ptr() as *mut c_void, show) }
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> WxString {
        unsafe { WxString(ffi2::wxWindow_GetHelpText(self.as_ptr() as *mut c_void)) }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = crate::wx_string_from(help_text);
            ffi2::wxWindow_SetHelpText(self.as_ptr() as *mut c_void, help_text)
        }
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetToolTip(self.as_ptr() as *mut c_void) }
    }
    fn get_tool_tip_text(&self) -> WxString {
        unsafe { WxString(ffi2::wxWindow_GetToolTipText(self.as_ptr() as *mut c_void)) }
    }
    fn set_tool_tip(&self, tip_string: &str) {
        unsafe {
            let tip_string = crate::wx_string_from(tip_string);
            ffi2::wxWindow_SetToolTip(self.as_ptr() as *mut c_void, tip_string)
        }
    }
    fn set_tool_tip1(&self, tip: *mut c_void) {
        unsafe { ffi2::wxWindow_SetToolTip1(self.as_ptr() as *mut c_void, tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi2::wxWindow_UnsetToolTip(self.as_ptr() as *mut c_void) }
    }
    fn get_popup_menu_selection_from_user(&self, menu: *mut c_void, pos: &Point) -> i32 {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            ffi2::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr() as *mut c_void, menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user1(&self, menu: *mut c_void, x: i32, y: i32) -> i32 {
        unsafe { ffi2::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr() as *mut c_void, menu, x, y) }
    }
    fn popup_menu(&self, menu: *mut c_void, pos: &Point) -> bool {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            ffi2::wxWindow_PopupMenu(self.as_ptr() as *mut c_void, menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut c_void, x: i32, y: i32) -> bool {
        unsafe { ffi2::wxWindow_PopupMenu1(self.as_ptr() as *mut c_void, menu, x, y) }
    }
    fn get_validator(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetValidator(self.as_ptr() as *mut c_void) }
    }
    fn set_validator(&self, validator: &Validator) {
        unsafe {
            let validator = validator.as_ptr() as *mut c_void;
            ffi2::wxWindow_SetValidator(self.as_ptr() as *mut c_void, validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi2::wxWindow_TransferDataFromWindow(self.as_ptr() as *mut c_void) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi2::wxWindow_TransferDataToWindow(self.as_ptr() as *mut c_void) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi2::wxWindow_Validate(self.as_ptr() as *mut c_void) }
    }
    fn get_id(&self) -> i32 {
        unsafe { ffi2::wxWindow_GetId(self.as_ptr() as *mut c_void) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi2::wxWindow_GetLabel(self.as_ptr() as *mut c_void)) }
    }
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: i32, width: i32, width_total: i32) -> i32 {
        unsafe { ffi2::wxWindow_AdjustForLayoutDirection(self.as_ptr() as *mut c_void, x, width, width_total) }
    }
    fn get_name(&self) -> WxString {
        unsafe { WxString(ffi2::wxWindow_GetName(self.as_ptr() as *mut c_void)) }
    }
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: i32) {
        unsafe { ffi2::wxWindow_SetId(self.as_ptr() as *mut c_void, winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi2::wxWindow_SetLabel(self.as_ptr() as *mut c_void, label)
        }
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi2::wxWindow_SetName(self.as_ptr() as *mut c_void, name)
        }
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetAcceleratorTable(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: *const c_void) {
        unsafe { ffi2::wxWindow_SetAcceleratorTable(self.as_ptr() as *mut c_void, accel) }
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi2::wxWindow_Close(self.as_ptr() as *mut c_void, force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi2::wxWindow_Destroy(self.as_ptr() as *mut c_void) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi2::wxWindow_IsBeingDeleted(self.as_ptr() as *mut c_void) }
    }
    fn get_drop_target(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetDropTarget(self.as_ptr() as *mut c_void) }
    }
    fn set_drop_target(&self, target: *mut c_void) {
        unsafe { ffi2::wxWindow_SetDropTarget(self.as_ptr() as *mut c_void, target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi2::wxWindow_DragAcceptFiles(self.as_ptr() as *mut c_void, accept) }
    }
    fn get_containing_sizer(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetContainingSizer(self.as_ptr() as *mut c_void) }
    }
    fn get_sizer(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetSizer(self.as_ptr() as *mut c_void) }
    }
    fn set_sizer(&self, sizer: *mut c_void, delete_old: bool) {
        unsafe { ffi2::wxWindow_SetSizer(self.as_ptr() as *mut c_void, sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut c_void, delete_old: bool) {
        unsafe { ffi2::wxWindow_SetSizerAndFit(self.as_ptr() as *mut c_void, sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetConstraints(self.as_ptr() as *mut c_void) }
    }
    fn set_constraints(&self, constraints: *mut c_void) {
        unsafe { ffi2::wxWindow_SetConstraints(self.as_ptr() as *mut c_void, constraints) }
    }
    fn layout(&self) -> bool {
        unsafe { ffi2::wxWindow_Layout(self.as_ptr() as *mut c_void) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi2::wxWindow_SetAutoLayout(self.as_ptr() as *mut c_void, auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi2::wxWindow_GetAutoLayout(self.as_ptr() as *mut c_void) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi2::wxWindow_CaptureMouse(self.as_ptr() as *mut c_void) }
    }
    fn get_caret(&self) -> *mut c_void {
        unsafe { ffi2::wxWindow_GetCaret(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi2::wxWindow_HasCapture(self.as_ptr() as *mut c_void) }
    }
    fn release_mouse(&self) {
        unsafe { ffi2::wxWindow_ReleaseMouse(self.as_ptr() as *mut c_void) }
    }
    fn set_caret(&self, caret: *mut c_void) {
        unsafe { ffi2::wxWindow_SetCaret(self.as_ptr() as *mut c_void, caret) }
    }
    fn set_cursor(&self, cursor: *const c_void) -> bool {
        unsafe { ffi2::wxWindow_SetCursor(self.as_ptr() as *mut c_void, cursor) }
    }
    fn warp_pointer(&self, x: i32, y: i32) {
        unsafe { ffi2::wxWindow_WarpPointer(self.as_ptr() as *mut c_void, x, y) }
    }
    fn enable_touch_events(&self, events_mask: i32) -> bool {
        unsafe { ffi2::wxWindow_EnableTouchEvents(self.as_ptr() as *mut c_void, events_mask) }
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest1()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: *mut c_void) {
        unsafe { ffi2::wxWindow_DoUpdateWindowUI(self.as_ptr() as *mut c_void, event) }
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi2::wxWindow_HasMultiplePages(self.as_ptr() as *mut c_void) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi2::wxWindow_InheritAttributes(self.as_ptr() as *mut c_void) }
    }
    fn init_dialog(&self) {
        unsafe { ffi2::wxWindow_InitDialog(self.as_ptr() as *mut c_void) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi2::wxWindow_IsDoubleBuffered(self.as_ptr() as *mut c_void) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi2::wxWindow_SetDoubleBuffered(self.as_ptr() as *mut c_void, on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi2::wxWindow_IsRetained(self.as_ptr() as *mut c_void) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi2::wxWindow_IsThisEnabled(self.as_ptr() as *mut c_void) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi2::wxWindow_IsTopLevel(self.as_ptr() as *mut c_void) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi2::wxWindow_OnInternalIdle(self.as_ptr() as *mut c_void) }
    }
    fn send_idle_events(&self, event: *mut c_void) -> bool {
        unsafe { ffi2::wxWindow_SendIdleEvents(self.as_ptr() as *mut c_void, event) }
    }
    fn register_hot_key(&self, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool {
        unsafe { ffi2::wxWindow_RegisterHotKey(self.as_ptr() as *mut c_void, hotkey_id, modifiers, virtual_key_code) }
    }
    fn unregister_hot_key(&self, hotkey_id: i32) -> bool {
        unsafe { ffi2::wxWindow_UnregisterHotKey(self.as_ptr() as *mut c_void, hotkey_id) }
    }
    fn update_window_ui(&self, flags: i32) {
        unsafe { ffi2::wxWindow_UpdateWindowUI(self.as_ptr() as *mut c_void, flags) }
    }
    // CXX_UNSUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> *mut c_void {
        unsafe { ffi2::wxWindow_FindFocus() }
    }
    fn find_window_by_id<T: WindowMethods>(id: i32, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_FindWindowById(id, parent)
        }
    }
    fn find_window_by_label<T: WindowMethods>(label: &str, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let label = crate::wx_string_from(label);
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_FindWindowByLabel(label, parent)
        }
    }
    fn find_window_by_name<T: WindowMethods>(name: &str, parent: Option<&T>) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxWindow_FindWindowByName(name, parent)
        }
    }
    fn get_capture() -> *mut c_void {
        unsafe { ffi2::wxWindow_GetCapture() }
    }
    fn new_control_id(count: i32) -> i32 {
        unsafe { ffi2::wxWindow_NewControlId(count) }
    }
    fn unreserve_control_id(id: i32, count: i32) {
        unsafe { ffi2::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi2::wxWindow_Create(self.as_ptr() as *mut c_void, parent, id, pos, size, style, name)
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
    pub fn new<T: WindowMethods>(parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> Control {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Control(ffi2::wxControl_new(parent, id, pos, size, style, validator, name))
        }
    }
    pub fn new1() -> Control {
        unsafe { Control(ffi2::wxControl_new1()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ControlMethods: WindowMethods {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi2::wxControl_Create(self.as_ptr() as *mut c_void, parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: *mut c_void) {
        unsafe { ffi2::wxControl_Command(self.as_ptr() as *mut c_void, event) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi2::wxControl_GetLabel(self.as_ptr() as *mut c_void)) }
    }
    fn get_label_text(&self) -> WxString {
        unsafe { WxString(ffi2::wxControl_GetLabelText(self.as_ptr() as *mut c_void)) }
    }
    fn get_size_from_text_size(&self, xlen: i32, ylen: i32) -> Size {
        unsafe { Size(ffi2::wxControl_GetSizeFromTextSize(self.as_ptr() as *mut c_void, xlen, ylen)) }
    }
    fn get_size_from_text_size1(&self, tsize: &Size) -> Size {
        unsafe {
            let tsize = tsize.as_ptr() as *mut c_void;
            Size(ffi2::wxControl_GetSizeFromTextSize1(self.as_ptr() as *mut c_void, tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = crate::wx_string_from(text);
            Size(ffi2::wxControl_GetSizeFromText(self.as_ptr() as *mut c_void, text))
        }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi2::wxControl_SetLabel(self.as_ptr() as *mut c_void, label)
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi2::wxControl_SetLabelText(self.as_ptr() as *mut c_void, text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = crate::wx_string_from(markup);
            ffi2::wxControl_SetLabelMarkup(self.as_ptr() as *mut c_void, markup)
        }
    }
    fn get_label_text1(label: &str) -> WxString {
        unsafe {
            let label = crate::wx_string_from(label);
            WxString(ffi2::wxControl_GetLabelText1(label))
        }
    }
    fn remove_mnemonics(str: &str) -> WxString {
        unsafe {
            let str = crate::wx_string_from(str);
            WxString(ffi2::wxControl_RemoveMnemonics(str))
        }
    }
    fn escape_mnemonics(text: &str) -> WxString {
        unsafe {
            let text = crate::wx_string_from(text);
            WxString(ffi2::wxControl_EscapeMnemonics(text))
        }
    }
    // BLOCKED: fn Ellipsize()
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
        unsafe { AnyButton(ffi2::wxAnyButton_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait AnyButtonMethods: ControlMethods {
    // DTOR: fn ~wxAnyButton()
    // CXX_UNSUPPORTED: fn GetBitmap()
    // CXX_UNSUPPORTED: fn GetBitmapCurrent()
    // CXX_UNSUPPORTED: fn GetBitmapDisabled()
    // CXX_UNSUPPORTED: fn GetBitmapFocus()
    // CXX_UNSUPPORTED: fn GetBitmapLabel()
    // CXX_UNSUPPORTED: fn GetBitmapPressed()
    // CXX_UNSUPPORTED: fn SetBitmap()
    fn set_bitmap_current(&self, bitmap: *const c_void) {
        unsafe { ffi2::wxAnyButton_SetBitmapCurrent(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_disabled(&self, bitmap: *const c_void) {
        unsafe { ffi2::wxAnyButton_SetBitmapDisabled(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_focus(&self, bitmap: *const c_void) {
        unsafe { ffi2::wxAnyButton_SetBitmapFocus(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_label(&self, bitmap: *const c_void) {
        unsafe { ffi2::wxAnyButton_SetBitmapLabel(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn set_bitmap_pressed(&self, bitmap: *const c_void) {
        unsafe { ffi2::wxAnyButton_SetBitmapPressed(self.as_ptr() as *mut c_void, bitmap) }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size(ffi2::wxAnyButton_GetBitmapMargins(self.as_ptr() as *mut c_void)) }
    }
    fn set_bitmap_margins(&self, x: i32, y: i32) {
        unsafe { ffi2::wxAnyButton_SetBitmapMargins(self.as_ptr() as *mut c_void, x, y) }
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        unsafe {
            let sz = sz.as_ptr() as *mut c_void;
            ffi2::wxAnyButton_SetBitmapMargins1(self.as_ptr() as *mut c_void, sz)
        }
    }
    // CXX_UNSUPPORTED: fn SetBitmapPosition()
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
        unsafe { Button(ffi2::wxButton_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, label: &str, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> Button {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Button(ffi2::wxButton_new1(parent, id, label, pos, size, style, validator, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ButtonMethods: AnyButtonMethods {
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, label: &str, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let validator = validator.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi2::wxButton_Create(self.as_ptr() as *mut c_void, parent, id, label, pos, size, style, validator, name)
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi2::wxButton_GetAuthNeeded(self.as_ptr() as *mut c_void) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi2::wxButton_GetLabel(self.as_ptr() as *mut c_void)) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi2::wxButton_SetAuthNeeded(self.as_ptr() as *mut c_void, needed) }
    }
    fn set_default(&self) -> *mut c_void {
        unsafe { ffi2::wxButton_SetDefault(self.as_ptr() as *mut c_void) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi2::wxButton_SetLabel(self.as_ptr() as *mut c_void, label)
        }
    }
    fn get_default_size<T: WindowMethods>(win: Option<&T>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            Size(ffi2::wxButton_GetDefaultSize(win))
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
        unsafe { ffi2::wxNonOwnedWindow_SetShape(self.as_ptr() as *mut c_void, region) }
    }
    fn set_shape1(&self, path: *const c_void) -> bool {
        unsafe { ffi2::wxNonOwnedWindow_SetShape1(self.as_ptr() as *mut c_void, path) }
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
        unsafe { TopLevelWindow(ffi2::wxTopLevelWindow_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> TopLevelWindow {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            TopLevelWindow(ffi2::wxTopLevelWindow_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait TopLevelWindowMethods: NonOwnedWindowMethods {
    // DTOR: fn ~wxTopLevelWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi2::wxTopLevelWindow_Create(self.as_ptr() as *mut c_void, parent, id, title, pos, size, style, name)
        }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_CanSetTransparent(self.as_ptr() as *mut c_void) }
    }
    fn center_on_screen(&self, direction: i32) {
        unsafe { ffi2::wxTopLevelWindow_CenterOnScreen(self.as_ptr() as *mut c_void, direction) }
    }
    fn centre_on_screen(&self, direction: i32) {
        unsafe { ffi2::wxTopLevelWindow_CentreOnScreen(self.as_ptr() as *mut c_void, direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi2::wxTopLevelWindow_EnableCloseButton(self.as_ptr() as *mut c_void, enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi2::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr() as *mut c_void, enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi2::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr() as *mut c_void, enable) }
    }
    fn get_default_item(&self) -> *mut c_void {
        unsafe { ffi2::wxTopLevelWindow_GetDefaultItem(self.as_ptr() as *mut c_void) }
    }
    // CXX_UNSUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> WxString {
        unsafe { WxString(ffi2::wxTopLevelWindow_GetTitle(self.as_ptr() as *mut c_void)) }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi2::wxTopLevelWindow_Iconize(self.as_ptr() as *mut c_void, iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_IsActive(self.as_ptr() as *mut c_void) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr() as *mut c_void) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_IsFullScreen(self.as_ptr() as *mut c_void) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_IsIconized(self.as_ptr() as *mut c_void) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_IsMaximized(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn layout(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_Layout(self.as_ptr() as *mut c_void) }
    }
    fn maximize(&self, maximize: bool) {
        unsafe { ffi2::wxTopLevelWindow_Maximize(self.as_ptr() as *mut c_void, maximize) }
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: i32) {
        unsafe { ffi2::wxTopLevelWindow_RequestUserAttention(self.as_ptr() as *mut c_void, flags) }
    }
    fn restore(&self) {
        unsafe { ffi2::wxTopLevelWindow_Restore(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut c_void {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxTopLevelWindow_SetDefaultItem(self.as_ptr() as *mut c_void, win)
        }
    }
    fn set_tmp_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut c_void {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr() as *mut c_void, win)
        }
    }
    fn get_tmp_default_item(&self) -> *mut c_void {
        unsafe { ffi2::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr() as *mut c_void) }
    }
    fn set_icon(&self, icon: *const c_void) {
        unsafe { ffi2::wxTopLevelWindow_SetIcon(self.as_ptr() as *mut c_void, icon) }
    }
    fn set_icons(&self, icons: *const c_void) {
        unsafe { ffi2::wxTopLevelWindow_SetIcons(self.as_ptr() as *mut c_void, icons) }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxTopLevelWindow_SetMaxSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxTopLevelWindow_SetMinSize(self.as_ptr() as *mut c_void, size)
        }
    }
    fn set_size_hints(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi2::wxTopLevelWindow_SetSizeHints(self.as_ptr() as *mut c_void, min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_size_hints1(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr() as *mut c_void;
            let max_size = max_size.as_ptr() as *mut c_void;
            let inc_size = inc_size.as_ptr() as *mut c_void;
            ffi2::wxTopLevelWindow_SetSizeHints1(self.as_ptr() as *mut c_void, min_size, max_size, inc_size)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = crate::wx_string_from(title);
            ffi2::wxTopLevelWindow_SetTitle(self.as_ptr() as *mut c_void, title)
        }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi2::wxTopLevelWindow_SetTransparent(self.as_ptr() as *mut c_void, alpha) }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr() as *mut c_void) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi2::wxTopLevelWindow_OSXSetModified(self.as_ptr() as *mut c_void, modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi2::wxTopLevelWindow_OSXIsModified(self.as_ptr() as *mut c_void) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = crate::wx_string_from(filename);
            ffi2::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr() as *mut c_void, filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi2::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr() as *mut c_void) }
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        unsafe { ffi2::wxTopLevelWindow_EnableFullScreenView(self.as_ptr() as *mut c_void, enable) }
    }
    fn show_full_screen(&self, show: bool, style: i32) -> bool {
        unsafe { ffi2::wxTopLevelWindow_ShowFullScreen(self.as_ptr() as *mut c_void, show, style) }
    }
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    fn get_default_size() -> Size {
        unsafe { Size(ffi2::wxTopLevelWindow_GetDefaultSize()) }
    }
}

// wxFrame
wx_class! { Frame(wxFrame) impl
    FrameMethods,
    TopLevelWindowMethods,
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Frame {
    pub fn new() -> Frame {
        unsafe { Frame(ffi2::wxFrame_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> Frame {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            Frame(ffi2::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    fn centre(&self, direction: i32) {
        unsafe { ffi2::wxFrame_Centre(self.as_ptr() as *mut c_void, direction) }
    }
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            let name = crate::wx_string_from(name);
            ffi2::wxFrame_Create(self.as_ptr() as *mut c_void, parent, id, title, pos, size, style, name)
        }
    }
    fn create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi2::wxFrame_CreateStatusBar(self.as_ptr() as *mut c_void, number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi2::wxFrame_CreateToolBar(self.as_ptr() as *mut c_void, style, id, name)
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi2::wxFrame_DoGiveHelp(self.as_ptr() as *mut c_void, text, show)
        }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi2::wxFrame_GetClientAreaOrigin(self.as_ptr() as *mut c_void)) }
    }
    fn get_menu_bar(&self) -> *mut c_void {
        unsafe { ffi2::wxFrame_GetMenuBar(self.as_ptr() as *mut c_void) }
    }
    fn get_status_bar(&self) -> *mut c_void {
        unsafe { ffi2::wxFrame_GetStatusBar(self.as_ptr() as *mut c_void) }
    }
    fn get_status_bar_pane(&self) -> i32 {
        unsafe { ffi2::wxFrame_GetStatusBarPane(self.as_ptr() as *mut c_void) }
    }
    fn get_tool_bar(&self) -> *mut c_void {
        unsafe { ffi2::wxFrame_GetToolBar(self.as_ptr() as *mut c_void) }
    }
    fn on_create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi2::wxFrame_OnCreateStatusBar(self.as_ptr() as *mut c_void, number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut c_void {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi2::wxFrame_OnCreateToolBar(self.as_ptr() as *mut c_void, style, id, name)
        }
    }
    fn process_command(&self, id: i32) -> bool {
        unsafe { ffi2::wxFrame_ProcessCommand(self.as_ptr() as *mut c_void, id) }
    }
    fn set_menu_bar(&self, menu_bar: *mut c_void) {
        unsafe { ffi2::wxFrame_SetMenuBar(self.as_ptr() as *mut c_void, menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut c_void) {
        unsafe { ffi2::wxFrame_SetStatusBar(self.as_ptr() as *mut c_void, status_bar) }
    }
    fn set_status_bar_pane(&self, n: i32) {
        unsafe { ffi2::wxFrame_SetStatusBarPane(self.as_ptr() as *mut c_void, n) }
    }
    fn set_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi2::wxFrame_SetStatusText(self.as_ptr() as *mut c_void, text, number)
        }
    }
    fn set_status_widths(&self, n: i32, widths_field: *const c_void) {
        unsafe { ffi2::wxFrame_SetStatusWidths(self.as_ptr() as *mut c_void, n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut c_void) {
        unsafe { ffi2::wxFrame_SetToolBar(self.as_ptr() as *mut c_void, tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi2::wxFrame_PushStatusText(self.as_ptr() as *mut c_void, text, number)
        }
    }
    fn pop_status_text(&self, number: i32) {
        unsafe { ffi2::wxFrame_PopStatusText(self.as_ptr() as *mut c_void, number) }
    }
}

// wxPoint
wx_class! { Point(wxPoint) impl
    PointMethods
}
impl Point {
    pub fn new() -> Point {
        unsafe { Point(ffi2::wxPoint_new()) }
    }
    pub fn new1(x: i32, y: i32) -> Point {
        unsafe { Point(ffi2::wxPoint_new1(x, y)) }
    }
    pub fn new2(pt: *const c_void) -> Point {
        unsafe { Point(ffi2::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi2::wxPoint_IsFullySpecified(self.as_ptr() as *mut c_void) }
    }
    fn set_defaults(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxPoint_SetDefaults(self.as_ptr() as *mut c_void, pt)
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
        unsafe { Rect(ffi2::wxRect_new()) }
    }
    pub fn new1(x: i32, y: i32, width: i32, height: i32) -> Rect {
        unsafe { Rect(ffi2::wxRect_new1(x, y, width, height)) }
    }
    pub fn new2(top_left: &Point, bottom_right: &Point) -> Rect {
        unsafe {
            let top_left = top_left.as_ptr() as *mut c_void;
            let bottom_right = bottom_right.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new3(pos: &Point, size: &Size) -> Rect {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            let size = size.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_new3(pos, size))
        }
    }
    pub fn new4(size: &Size) -> Rect {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait RectMethods: WxRustMethods {
    fn centre_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = r.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_CentreIn(self.as_ptr() as *mut c_void, r, dir))
        }
    }
    fn center_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = r.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_CenterIn(self.as_ptr() as *mut c_void, r, dir))
        }
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        unsafe { ffi2::wxRect_Contains(self.as_ptr() as *mut c_void, x, y) }
    }
    fn contains1(&self, pt: &Point) -> bool {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxRect_Contains1(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn contains2(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi2::wxRect_Contains2(self.as_ptr() as *mut c_void, rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi2::wxRect_Deflate3(self.as_ptr() as *mut c_void, dx, dy)) }
    }
    fn get_bottom(&self) -> i32 {
        unsafe { ffi2::wxRect_GetBottom(self.as_ptr() as *mut c_void) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point(ffi2::wxRect_GetBottomLeft(self.as_ptr() as *mut c_void)) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point(ffi2::wxRect_GetBottomRight(self.as_ptr() as *mut c_void)) }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi2::wxRect_GetHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_left(&self) -> i32 {
        unsafe { ffi2::wxRect_GetLeft(self.as_ptr() as *mut c_void) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point(ffi2::wxRect_GetPosition(self.as_ptr() as *mut c_void)) }
    }
    fn get_right(&self) -> i32 {
        unsafe { ffi2::wxRect_GetRight(self.as_ptr() as *mut c_void) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size(ffi2::wxRect_GetSize(self.as_ptr() as *mut c_void)) }
    }
    fn get_top(&self) -> i32 {
        unsafe { ffi2::wxRect_GetTop(self.as_ptr() as *mut c_void) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point(ffi2::wxRect_GetTopLeft(self.as_ptr() as *mut c_void)) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point(ffi2::wxRect_GetTopRight(self.as_ptr() as *mut c_void)) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi2::wxRect_GetWidth(self.as_ptr() as *mut c_void) }
    }
    fn get_x(&self) -> i32 {
        unsafe { ffi2::wxRect_GetX(self.as_ptr() as *mut c_void) }
    }
    fn get_y(&self) -> i32 {
        unsafe { ffi2::wxRect_GetY(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi2::wxRect_Inflate3(self.as_ptr() as *mut c_void, dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect1(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_Intersect1(self.as_ptr() as *mut c_void, rect))
        }
    }
    fn intersects(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            ffi2::wxRect_Intersects(self.as_ptr() as *mut c_void, rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi2::wxRect_IsEmpty(self.as_ptr() as *mut c_void) }
    }
    fn offset(&self, dx: i32, dy: i32) {
        unsafe { ffi2::wxRect_Offset(self.as_ptr() as *mut c_void, dx, dy) }
    }
    fn offset1(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxRect_Offset1(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi2::wxRect_SetHeight(self.as_ptr() as *mut c_void, height) }
    }
    fn set_position(&self, pos: &Point) {
        unsafe {
            let pos = pos.as_ptr() as *mut c_void;
            ffi2::wxRect_SetPosition(self.as_ptr() as *mut c_void, pos)
        }
    }
    fn set_size(&self, s: &Size) {
        unsafe {
            let s = s.as_ptr() as *mut c_void;
            ffi2::wxRect_SetSize(self.as_ptr() as *mut c_void, s)
        }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi2::wxRect_SetWidth(self.as_ptr() as *mut c_void, width) }
    }
    fn set_x(&self, x: i32) {
        unsafe { ffi2::wxRect_SetX(self.as_ptr() as *mut c_void, x) }
    }
    fn set_y(&self, y: i32) {
        unsafe { ffi2::wxRect_SetY(self.as_ptr() as *mut c_void, y) }
    }
    fn set_left(&self, left: i32) {
        unsafe { ffi2::wxRect_SetLeft(self.as_ptr() as *mut c_void, left) }
    }
    fn set_right(&self, right: i32) {
        unsafe { ffi2::wxRect_SetRight(self.as_ptr() as *mut c_void, right) }
    }
    fn set_top(&self, top: i32) {
        unsafe { ffi2::wxRect_SetTop(self.as_ptr() as *mut c_void, top) }
    }
    fn set_bottom(&self, bottom: i32) {
        unsafe { ffi2::wxRect_SetBottom(self.as_ptr() as *mut c_void, bottom) }
    }
    fn set_top_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi2::wxRect_SetTopLeft(self.as_ptr() as *mut c_void, p)
        }
    }
    fn set_bottom_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi2::wxRect_SetBottomRight(self.as_ptr() as *mut c_void, p)
        }
    }
    fn set_top_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi2::wxRect_SetTopRight(self.as_ptr() as *mut c_void, p)
        }
    }
    fn set_bottom_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut c_void;
            ffi2::wxRect_SetBottomLeft(self.as_ptr() as *mut c_void, p)
        }
    }
    fn union(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr() as *mut c_void;
            Rect(ffi2::wxRect_Union(self.as_ptr() as *mut c_void, rect))
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
        unsafe { Size(ffi2::wxSize_new()) }
    }
    pub fn new1(width: i32, height: i32) -> Size {
        unsafe { Size(ffi2::wxSize_new1(width, height)) }
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
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxSize_DecBy(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn dec_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxSize_DecBy1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn dec_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi2::wxSize_DecBy2(self.as_ptr() as *mut c_void, dx, dy) }
    }
    fn dec_by3(&self, d: i32) {
        unsafe { ffi2::wxSize_DecBy3(self.as_ptr() as *mut c_void, d) }
    }
    fn dec_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxSize_DecTo(self.as_ptr() as *mut c_void, size)
        }
    }
    fn dec_to_if_specified(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxSize_DecToIfSpecified(self.as_ptr() as *mut c_void, size)
        }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi2::wxSize_GetHeight(self.as_ptr() as *mut c_void) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi2::wxSize_GetWidth(self.as_ptr() as *mut c_void) }
    }
    fn inc_by(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut c_void;
            ffi2::wxSize_IncBy(self.as_ptr() as *mut c_void, pt)
        }
    }
    fn inc_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxSize_IncBy1(self.as_ptr() as *mut c_void, size)
        }
    }
    fn inc_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi2::wxSize_IncBy2(self.as_ptr() as *mut c_void, dx, dy) }
    }
    fn inc_by3(&self, d: i32) {
        unsafe { ffi2::wxSize_IncBy3(self.as_ptr() as *mut c_void, d) }
    }
    fn inc_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut c_void;
            ffi2::wxSize_IncTo(self.as_ptr() as *mut c_void, size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi2::wxSize_IsFullySpecified(self.as_ptr() as *mut c_void) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: i32, height: i32) {
        unsafe { ffi2::wxSize_Set(self.as_ptr() as *mut c_void, width, height) }
    }
    fn set_defaults(&self, size_default: &Size) {
        unsafe {
            let size_default = size_default.as_ptr() as *mut c_void;
            ffi2::wxSize_SetDefaults(self.as_ptr() as *mut c_void, size_default)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi2::wxSize_SetHeight(self.as_ptr() as *mut c_void, height) }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi2::wxSize_SetWidth(self.as_ptr() as *mut c_void, width) }
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
        unsafe { Validator(ffi2::wxValidator_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    fn clone(&self) -> *mut c_void {
        unsafe { ffi2::wxValidator_Clone(self.as_ptr() as *mut c_void) }
    }
    fn get_window(&self) -> *mut c_void {
        unsafe { ffi2::wxValidator_GetWindow(self.as_ptr() as *mut c_void) }
    }
    fn set_window<T: WindowMethods>(&self, window: Option<&T>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxValidator_SetWindow(self.as_ptr() as *mut c_void, window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi2::wxValidator_TransferFromWindow(self.as_ptr() as *mut c_void) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi2::wxValidator_TransferToWindow(self.as_ptr() as *mut c_void) }
    }
    fn validate<T: WindowMethods>(&self, parent: Option<&T>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut c_void as *mut c_void,
                None => ptr::null_mut(),
            };
            ffi2::wxValidator_Validate(self.as_ptr() as *mut c_void, parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi2::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi2::wxValidator_IsSilent() }
    }
}

