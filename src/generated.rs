#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::c_char;
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

        type wxString = crate::ffi::wxString;
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
        // CTOR: unsafe fn wxObject() -> Object;
        // CTOR: unsafe fn wxObject1(other: *const wxObject) -> Object;
        // DTOR: unsafe fn ~wxObject(self: *mut wxObject);
        // GENERATED: unsafe fn GetClassInfo(self: *const wxObject) -> *mut wxClassInfo;
        // GENERATED: unsafe fn GetRefData(self: *const wxObject) -> *mut wxObjectRefData;
        // GENERATED: unsafe fn IsKindOf(self: *const wxObject, info: *const wxClassInfo) -> bool;
        // GENERATED: unsafe fn IsSameAs(self: *const wxObject, obj: *const wxObject) -> bool;
        // GENERATED: unsafe fn Ref(self: *mut wxObject, clone: *const wxObject);
        // GENERATED: unsafe fn SetRefData(self: *mut wxObject, data: *mut wxObjectRefData);
        // GENERATED: unsafe fn UnRef(self: *mut wxObject);
        // GENERATED: unsafe fn UnShare(self: *mut wxObject);
        // BLOCKED: unsafe fn operator delete(self: *mut wxObject, buf: *mut void);
        // CXX_UNSUPPORTED: unsafe fn operator new(self: *mut wxObject, size: size_t, filename: *const wxString, line_num: i32) -> *mut void;
        
        // CLASS: wxEvtHandler
        type wxEvtHandler;
        // GENERATED: unsafe fn QueueEvent(self: *mut wxEvtHandler, event: *mut wxEvent);
        // GENERATED: unsafe fn AddPendingEvent(self: *mut wxEvtHandler, event: *const wxEvent);
        // CXX_UNSUPPORTED: unsafe fn CallAfter(self: *mut wxEvtHandler, method: *mut void(T::, x1: T1, None: ...);
        // BLOCKED: unsafe fn CallAfter1(self: *mut wxEvtHandler, functor: *const T);
        // GENERATED: unsafe fn ProcessEvent(self: *mut wxEvtHandler, event: *mut wxEvent) -> bool;
        // GENERATED: unsafe fn ProcessEventLocally(self: *mut wxEvtHandler, event: *mut wxEvent) -> bool;
        // GENERATED: unsafe fn SafelyProcessEvent(self: *mut wxEvtHandler, event: *mut wxEvent) -> bool;
        // GENERATED: unsafe fn ProcessPendingEvents(self: *mut wxEvtHandler);
        // GENERATED: unsafe fn DeletePendingEvents(self: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect(self: *mut wxEvtHandler, id: i32, last_id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect1(self: *mut wxEvtHandler, id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect2(self: *mut wxEvtHandler, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: *mut wxEvtHandler, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Disconnect1(self: *mut wxEvtHandler, id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Disconnect2(self: *mut wxEvtHandler, id: i32, last_id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Bind(self: *mut wxEvtHandler, event_type: *const EventTag, functor: Functor, id: i32, last_id: i32, user_data: *mut wxObject);
        // BLOCKED: unsafe fn Bind1(self: *mut wxEvtHandler, event_type: *const EventTag, method: *mut void(Class::, handler: *mut EventHandler, id: i32, last_id: i32, user_data: *mut wxObject);
        // CXX_UNSUPPORTED: unsafe fn Unbind(self: *mut wxEvtHandler, event_type: *const EventTag, functor: Functor, id: i32, last_id: i32, user_data: *mut wxObject) -> bool;
        // BLOCKED: unsafe fn Unbind1(self: *mut wxEvtHandler, event_type: *const EventTag, method: *mut void(Class::, handler: *mut EventHandler, id: i32, last_id: i32, user_data: *mut wxObject) -> bool;
        // BLOCKED: unsafe fn GetClientData(self: *const wxEvtHandler) -> *mut void;
        // GENERATED: unsafe fn GetClientObject(self: *const wxEvtHandler) -> *mut wxClientData;
        // BLOCKED: unsafe fn SetClientData(self: *mut wxEvtHandler, data: *mut void);
        // GENERATED: unsafe fn SetClientObject(self: *mut wxEvtHandler, data: *mut wxClientData);
        // GENERATED: unsafe fn GetEvtHandlerEnabled(self: *const wxEvtHandler) -> bool;
        // GENERATED: unsafe fn GetNextHandler(self: *const wxEvtHandler) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn GetPreviousHandler(self: *const wxEvtHandler) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn SetEvtHandlerEnabled(self: *mut wxEvtHandler, enabled: bool);
        // GENERATED: unsafe fn SetNextHandler(self: *mut wxEvtHandler, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetPreviousHandler(self: *mut wxEvtHandler, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn Unlink(self: *mut wxEvtHandler);
        // GENERATED: unsafe fn IsUnlinked(self: *const wxEvtHandler) -> bool;
        // GENERATED: unsafe fn AddFilter(filter: *mut wxEventFilter);
        // GENERATED: unsafe fn RemoveFilter(filter: *mut wxEventFilter);
        // CTOR: unsafe fn wxEvtHandler() -> EvtHandler;
        // DTOR: unsafe fn ~wxEvtHandler(self: *mut wxEvtHandler);
        
        // CLASS: wxWindow
        type wxWindow;
        // GENERATED: unsafe fn AcceptsFocus(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn AcceptsFocusFromKeyboard(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn AcceptsFocusRecursively(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn DisableFocusFromKeyboard(self: *mut wxWindow);
        // GENERATED: unsafe fn IsFocusable(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn CanAcceptFocus(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn CanAcceptFocusFromKeyboard(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn HasFocus(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn SetCanFocus(self: *mut wxWindow, can_focus: bool);
        // GENERATED: unsafe fn EnableVisibleFocus(self: *mut wxWindow, enable: bool);
        // GENERATED: unsafe fn SetFocus(self: *mut wxWindow);
        // GENERATED: unsafe fn SetFocusFromKbd(self: *mut wxWindow);
        // GENERATED: unsafe fn AddChild(self: *mut wxWindow, child: *mut wxWindow);
        // GENERATED: unsafe fn DestroyChildren(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn FindWindow(self: *const wxWindow, id: i32) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindow1(self: *const wxWindow, name: *const wxString) -> *mut wxWindow;
        // BLOCKED: unsafe fn GetChildren(self: *mut wxWindow) -> *mut wxWindowList;
        // BLOCKED: unsafe fn GetChildren1(self: *const wxWindow) -> *const wxWindowList;
        // GENERATED: unsafe fn RemoveChild(self: *mut wxWindow, child: *mut wxWindow);
        // GENERATED: unsafe fn GetGrandParent(self: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetNextSibling(self: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetParent(self: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetPrevSibling(self: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn IsDescendant(self: *const wxWindow, win: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn Reparent(self: *mut wxWindow, new_parent: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn AlwaysShowScrollbars(self: *mut wxWindow, hflag: bool, vflag: bool);
        // GENERATED: unsafe fn GetScrollPos(self: *const wxWindow, orientation: i32) -> i32;
        // GENERATED: unsafe fn GetScrollRange(self: *const wxWindow, orientation: i32) -> i32;
        // GENERATED: unsafe fn GetScrollThumb(self: *const wxWindow, orientation: i32) -> i32;
        // GENERATED: unsafe fn CanScroll(self: *const wxWindow, orient: i32) -> bool;
        // GENERATED: unsafe fn HasScrollbar(self: *const wxWindow, orient: i32) -> bool;
        // GENERATED: unsafe fn IsScrollbarAlwaysShown(self: *const wxWindow, orient: i32) -> bool;
        // GENERATED: unsafe fn ScrollLines(self: *mut wxWindow, lines: i32) -> bool;
        // GENERATED: unsafe fn ScrollPages(self: *mut wxWindow, pages: i32) -> bool;
        // GENERATED: unsafe fn ScrollWindow(self: *mut wxWindow, dx: i32, dy: i32, rect: *const wxRect);
        // GENERATED: unsafe fn LineUp(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn LineDown(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn PageUp(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn PageDown(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn SetScrollPos(self: *mut wxWindow, orientation: i32, pos: i32, refresh: bool);
        // GENERATED: unsafe fn SetScrollbar(self: *mut wxWindow, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        // GENERATED: unsafe fn BeginRepositioningChildren(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn EndRepositioningChildren(self: *mut wxWindow);
        // GENERATED: unsafe fn CacheBestSize(self: *const wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn ClientToWindowSize(self: *const wxWindow, size: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn WindowToClientSize(self: *const wxWindow, size: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn Fit(self: *mut wxWindow);
        // GENERATED: unsafe fn FitInside(self: *mut wxWindow);
        // GENERATED: unsafe fn FromDIP(self: *const wxWindow, sz: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn FromDIP1(self: *const wxWindow, pt: *const wxPoint) -> wxPoint;
        // GENERATED: unsafe fn FromDIP2(self: *const wxWindow, d: i32) -> i32;
        // GENERATED: unsafe fn ToDIP(self: *const wxWindow, sz: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn ToDIP1(self: *const wxWindow, pt: *const wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ToDIP2(self: *const wxWindow, d: i32) -> i32;
        // GENERATED: unsafe fn GetBestSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetBestHeight(self: *const wxWindow, width: i32) -> i32;
        // GENERATED: unsafe fn GetBestWidth(self: *const wxWindow, height: i32) -> i32;
        // GENERATED: unsafe fn GetClientSize(self: *const wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: unsafe fn GetClientSize1(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetEffectiveMinSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMaxClientSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMaxSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMinClientSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMinSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMinWidth(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetMinHeight(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetMaxWidth(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetMaxHeight(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetSize(self: *const wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: unsafe fn GetSize1(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetVirtualSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetVirtualSize1(self: *const wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: unsafe fn GetBestVirtualSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetContentScaleFactor(self: *const wxWindow) -> f64;
        // GENERATED: unsafe fn GetDPIScaleFactor(self: *const wxWindow) -> f64;
        // GENERATED: unsafe fn GetWindowBorderSize(self: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn InformFirstDirection(self: *mut wxWindow, direction: i32, size: i32, available_other_dir: i32) -> bool;
        // GENERATED: unsafe fn InvalidateBestSize(self: *mut wxWindow);
        // GENERATED: unsafe fn PostSizeEvent(self: *mut wxWindow);
        // GENERATED: unsafe fn PostSizeEventToParent(self: *mut wxWindow);
        // GENERATED: unsafe fn SendSizeEvent(self: *mut wxWindow, flags: i32);
        // GENERATED: unsafe fn SendSizeEventToParent(self: *mut wxWindow, flags: i32);
        // GENERATED: unsafe fn SetClientSize(self: *mut wxWindow, width: i32, height: i32);
        // GENERATED: unsafe fn SetClientSize1(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetClientSize2(self: *mut wxWindow, rect: *const wxRect);
        // GENERATED: unsafe fn SetContainingSizer(self: *mut wxWindow, sizer: *mut wxSizer);
        // GENERATED: unsafe fn SetInitialSize(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetMaxClientSize(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetMaxSize(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetMinClientSize(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetMinSize(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetSize(self: *mut wxWindow, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        // GENERATED: unsafe fn SetSize1(self: *mut wxWindow, rect: *const wxRect);
        // GENERATED: unsafe fn SetSize2(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetSize3(self: *mut wxWindow, width: i32, height: i32);
        // GENERATED: unsafe fn SetSizeHints(self: *mut wxWindow, min_size: *const wxSize, max_size: *const wxSize, inc_size: *const wxSize);
        // GENERATED: unsafe fn SetSizeHints1(self: *mut wxWindow, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: unsafe fn SetVirtualSize(self: *mut wxWindow, width: i32, height: i32);
        // GENERATED: unsafe fn SetVirtualSize1(self: *mut wxWindow, size: *const wxSize);
        // GENERATED: unsafe fn FromDIP3(sz: *const wxSize, w: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn FromDIP4(pt: *const wxPoint, w: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn FromDIP5(d: i32, w: *const wxWindow) -> i32;
        // GENERATED: unsafe fn ToDIP3(sz: *const wxSize, w: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn ToDIP4(pt: *const wxPoint, w: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn ToDIP5(d: i32, w: *const wxWindow) -> i32;
        // GENERATED: unsafe fn Center(self: *mut wxWindow, dir: i32);
        // GENERATED: unsafe fn CenterOnParent(self: *mut wxWindow, dir: i32);
        // GENERATED: unsafe fn Centre(self: *mut wxWindow, direction: i32);
        // GENERATED: unsafe fn CentreOnParent(self: *mut wxWindow, direction: i32);
        // GENERATED: unsafe fn GetPosition(self: *const wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn GetPosition1(self: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn GetRect(self: *const wxWindow) -> wxRect;
        // GENERATED: unsafe fn GetScreenPosition(self: *const wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn GetScreenPosition1(self: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn GetScreenRect(self: *const wxWindow) -> wxRect;
        // GENERATED: unsafe fn GetClientAreaOrigin(self: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn GetClientRect(self: *const wxWindow) -> wxRect;
        // GENERATED: unsafe fn Move(self: *mut wxWindow, x: i32, y: i32, flags: i32);
        // GENERATED: unsafe fn Move1(self: *mut wxWindow, pt: *const wxPoint, flags: i32);
        // GENERATED: unsafe fn SetPosition(self: *mut wxWindow, pt: *const wxPoint);
        // GENERATED: unsafe fn ClientToScreen(self: *const wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn ClientToScreen1(self: *const wxWindow, pt: *const wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ConvertDialogToPixels(self: *const wxWindow, pt: *const wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ConvertDialogToPixels1(self: *const wxWindow, sz: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn ConvertPixelsToDialog(self: *const wxWindow, pt: *const wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ConvertPixelsToDialog1(self: *const wxWindow, sz: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn ScreenToClient(self: *const wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn ScreenToClient1(self: *const wxWindow, pt: *const wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ClearBackground(self: *mut wxWindow);
        // GENERATED: unsafe fn Freeze(self: *mut wxWindow);
        // GENERATED: unsafe fn Thaw(self: *mut wxWindow);
        // GENERATED: unsafe fn IsFrozen(self: *const wxWindow) -> bool;
        // CXX_UNSUPPORTED: unsafe fn GetBackgroundColour(self: *const wxWindow) -> wxColour;
        // CXX_UNSUPPORTED: unsafe fn GetBackgroundStyle(self: *const wxWindow) -> wxBackgroundStyle;
        // GENERATED: unsafe fn GetCharHeight(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetCharWidth(self: *const wxWindow) -> i32;
        // CXX_UNSUPPORTED: unsafe fn GetDefaultAttributes(self: *const wxWindow) -> wxVisualAttributes;
        // GENERATED: unsafe fn GetDPI(self: *const wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: unsafe fn GetFont(self: *const wxWindow) -> wxFont;
        // CXX_UNSUPPORTED: unsafe fn GetForegroundColour(self: *const wxWindow) -> wxColour;
        // GENERATED: unsafe fn GetTextExtent(self: *const wxWindow, string: *const wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        // GENERATED: unsafe fn GetTextExtent1(self: *const wxWindow, string: *const wxString) -> wxSize;
        // BLOCKED: unsafe fn GetUpdateRegion(self: *const wxWindow) -> *const wxRegion;
        // GENERATED: unsafe fn GetUpdateClientRect(self: *const wxWindow) -> wxRect;
        // GENERATED: unsafe fn HasTransparentBackground(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn Refresh(self: *mut wxWindow, erase_background: bool, rect: *const wxRect);
        // GENERATED: unsafe fn RefreshRect(self: *mut wxWindow, rect: *const wxRect, erase_background: bool);
        // GENERATED: unsafe fn Update(self: *mut wxWindow);
        // GENERATED: unsafe fn SetBackgroundColour(self: *mut wxWindow, colour: *const wxColour) -> bool;
        // CXX_UNSUPPORTED: unsafe fn SetBackgroundStyle(self: *mut wxWindow, style: wxBackgroundStyle) -> bool;
        // GENERATED: unsafe fn IsTransparentBackgroundSupported(self: *const wxWindow, reason: *mut wxString) -> bool;
        // GENERATED: unsafe fn SetFont(self: *mut wxWindow, font: *const wxFont) -> bool;
        // GENERATED: unsafe fn SetForegroundColour(self: *mut wxWindow, colour: *const wxColour) -> bool;
        // GENERATED: unsafe fn SetOwnBackgroundColour(self: *mut wxWindow, colour: *const wxColour);
        // GENERATED: unsafe fn InheritsBackgroundColour(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn UseBgCol(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn UseBackgroundColour(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn SetOwnFont(self: *mut wxWindow, font: *const wxFont);
        // GENERATED: unsafe fn SetOwnForegroundColour(self: *mut wxWindow, colour: *const wxColour);
        // GENERATED: unsafe fn UseForegroundColour(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn InheritsForegroundColour(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn SetPalette(self: *mut wxWindow, pal: *const wxPalette);
        // GENERATED: unsafe fn ShouldInheritColours(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn SetThemeEnabled(self: *mut wxWindow, enable: bool);
        // GENERATED: unsafe fn GetThemeEnabled(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn CanSetTransparent(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn SetTransparent(self: *mut wxWindow, alpha: u8) -> bool;
        // GENERATED: unsafe fn GetEventHandler(self: *const wxWindow) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn HandleAsNavigationKey(self: *mut wxWindow, event: *const wxKeyEvent) -> bool;
        // GENERATED: unsafe fn HandleWindowEvent(self: *const wxWindow, event: *mut wxEvent) -> bool;
        // GENERATED: unsafe fn ProcessWindowEvent(self: *mut wxWindow, event: *mut wxEvent) -> bool;
        // GENERATED: unsafe fn ProcessWindowEventLocally(self: *mut wxWindow, event: *mut wxEvent) -> bool;
        // GENERATED: unsafe fn PopEventHandler(self: *mut wxWindow, delete_handler: bool) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn PushEventHandler(self: *mut wxWindow, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn RemoveEventHandler(self: *mut wxWindow, handler: *mut wxEvtHandler) -> bool;
        // GENERATED: unsafe fn SetEventHandler(self: *mut wxWindow, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetNextHandler(self: *mut wxWindow, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetPreviousHandler(self: *mut wxWindow, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn GetExtraStyle(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetWindowStyleFlag(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetWindowStyle(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn HasExtraStyle(self: *const wxWindow, ex_flag: i32) -> bool;
        // GENERATED: unsafe fn HasFlag(self: *const wxWindow, flag: i32) -> bool;
        // GENERATED: unsafe fn SetExtraStyle(self: *mut wxWindow, ex_style: i32);
        // GENERATED: unsafe fn SetWindowStyleFlag(self: *mut wxWindow, style: i32);
        // GENERATED: unsafe fn SetWindowStyle(self: *mut wxWindow, style: i32);
        // GENERATED: unsafe fn ToggleWindowStyle(self: *mut wxWindow, flag: i32) -> bool;
        // GENERATED: unsafe fn MoveAfterInTabOrder(self: *mut wxWindow, win: *mut wxWindow);
        // GENERATED: unsafe fn MoveBeforeInTabOrder(self: *mut wxWindow, win: *mut wxWindow);
        // GENERATED: unsafe fn Navigate(self: *mut wxWindow, flags: i32) -> bool;
        // GENERATED: unsafe fn NavigateIn(self: *mut wxWindow, flags: i32) -> bool;
        // GENERATED: unsafe fn Lower(self: *mut wxWindow);
        // GENERATED: unsafe fn Raise(self: *mut wxWindow);
        // GENERATED: unsafe fn Hide(self: *mut wxWindow) -> bool;
        // CXX_UNSUPPORTED: unsafe fn HideWithEffect(self: *mut wxWindow, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: unsafe fn IsEnabled(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn IsExposed(self: *const wxWindow, x: i32, y: i32) -> bool;
        // GENERATED: unsafe fn IsExposed1(self: *const wxWindow, pt: *mut wxPoint) -> bool;
        // GENERATED: unsafe fn IsExposed2(self: *const wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        // GENERATED: unsafe fn IsExposed3(self: *const wxWindow, rect: *mut wxRect) -> bool;
        // GENERATED: unsafe fn IsShown(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn IsShownOnScreen(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn Disable(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn Enable(self: *mut wxWindow, enable: bool) -> bool;
        // GENERATED: unsafe fn Show(self: *mut wxWindow, show: bool) -> bool;
        // CXX_UNSUPPORTED: unsafe fn ShowWithEffect(self: *mut wxWindow, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: unsafe fn GetHelpText(self: *const wxWindow) -> wxString;
        // GENERATED: unsafe fn SetHelpText(self: *mut wxWindow, help_text: *const wxString);
        // CXX_UNSUPPORTED: unsafe fn GetHelpTextAtPoint(self: *const wxWindow, point: *const wxPoint, origin: wxHelpEvent::Origin) -> wxString;
        // GENERATED: unsafe fn GetToolTip(self: *const wxWindow) -> *mut wxToolTip;
        // GENERATED: unsafe fn GetToolTipText(self: *const wxWindow) -> wxString;
        // GENERATED: unsafe fn SetToolTip(self: *mut wxWindow, tip_string: *const wxString);
        // GENERATED: unsafe fn SetToolTip1(self: *mut wxWindow, tip: *mut wxToolTip);
        // GENERATED: unsafe fn UnsetToolTip(self: *mut wxWindow);
        // GENERATED: unsafe fn GetPopupMenuSelectionFromUser(self: *mut wxWindow, menu: *mut wxMenu, pos: *const wxPoint) -> i32;
        // GENERATED: unsafe fn GetPopupMenuSelectionFromUser1(self: *mut wxWindow, menu: *mut wxMenu, x: i32, y: i32) -> i32;
        // GENERATED: unsafe fn PopupMenu(self: *mut wxWindow, menu: *mut wxMenu, pos: *const wxPoint) -> bool;
        // GENERATED: unsafe fn PopupMenu1(self: *mut wxWindow, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        // GENERATED: unsafe fn GetValidator(self: *mut wxWindow) -> *mut wxValidator;
        // GENERATED: unsafe fn SetValidator(self: *mut wxWindow, validator: *const wxValidator);
        // GENERATED: unsafe fn TransferDataFromWindow(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn TransferDataToWindow(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn Validate(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn GetId(self: *const wxWindow) -> i32;
        // GENERATED: unsafe fn GetLabel(self: *const wxWindow) -> wxString;
        // CXX_UNSUPPORTED: unsafe fn GetLayoutDirection(self: *const wxWindow) -> wxLayoutDirection;
        // GENERATED: unsafe fn AdjustForLayoutDirection(self: *const wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        // GENERATED: unsafe fn GetName(self: *const wxWindow) -> wxString;
        // CXX_UNSUPPORTED: unsafe fn GetWindowVariant(self: *const wxWindow) -> wxWindowVariant;
        // GENERATED: unsafe fn SetId(self: *mut wxWindow, winid: i32);
        // GENERATED: unsafe fn SetLabel(self: *mut wxWindow, label: *const wxString);
        // CXX_UNSUPPORTED: unsafe fn SetLayoutDirection(self: *mut wxWindow, dir: wxLayoutDirection);
        // GENERATED: unsafe fn SetName(self: *mut wxWindow, name: *const wxString);
        // CXX_UNSUPPORTED: unsafe fn SetWindowVariant(self: *mut wxWindow, variant: wxWindowVariant);
        // GENERATED: unsafe fn GetAcceleratorTable(self: *mut wxWindow) -> *mut wxAcceleratorTable;
        // CXX_UNSUPPORTED: unsafe fn GetAccessible(self: *mut wxWindow) -> *mut wxAccessible;
        // GENERATED: unsafe fn SetAcceleratorTable(self: *mut wxWindow, accel: *const wxAcceleratorTable);
        // CXX_UNSUPPORTED: unsafe fn SetAccessible(self: *mut wxWindow, accessible: *mut wxAccessible);
        // GENERATED: unsafe fn Close(self: *mut wxWindow, force: bool) -> bool;
        // GENERATED: unsafe fn Destroy(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn IsBeingDeleted(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn GetDropTarget(self: *const wxWindow) -> *mut wxDropTarget;
        // GENERATED: unsafe fn SetDropTarget(self: *mut wxWindow, target: *mut wxDropTarget);
        // GENERATED: unsafe fn DragAcceptFiles(self: *mut wxWindow, accept: bool);
        // GENERATED: unsafe fn GetContainingSizer(self: *const wxWindow) -> *mut wxSizer;
        // GENERATED: unsafe fn GetSizer(self: *const wxWindow) -> *mut wxSizer;
        // GENERATED: unsafe fn SetSizer(self: *mut wxWindow, sizer: *mut wxSizer, delete_old: bool);
        // GENERATED: unsafe fn SetSizerAndFit(self: *mut wxWindow, sizer: *mut wxSizer, delete_old: bool);
        // GENERATED: unsafe fn GetConstraints(self: *const wxWindow) -> *mut wxLayoutConstraints;
        // GENERATED: unsafe fn SetConstraints(self: *mut wxWindow, constraints: *mut wxLayoutConstraints);
        // GENERATED: unsafe fn Layout(self: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn SetAutoLayout(self: *mut wxWindow, auto_layout: bool);
        // GENERATED: unsafe fn GetAutoLayout(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn CaptureMouse(self: *mut wxWindow);
        // GENERATED: unsafe fn GetCaret(self: *const wxWindow) -> *mut wxCaret;
        // BLOCKED: unsafe fn GetCursor(self: *const wxWindow) -> *const wxCursor;
        // GENERATED: unsafe fn HasCapture(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn ReleaseMouse(self: *mut wxWindow);
        // GENERATED: unsafe fn SetCaret(self: *mut wxWindow, caret: *mut wxCaret);
        // GENERATED: unsafe fn SetCursor(self: *mut wxWindow, cursor: *const wxCursor) -> bool;
        // GENERATED: unsafe fn WarpPointer(self: *mut wxWindow, x: i32, y: i32);
        // GENERATED: unsafe fn EnableTouchEvents(self: *mut wxWindow, events_mask: i32) -> bool;
        // CXX_UNSUPPORTED: unsafe fn HitTest(self: *const wxWindow, x: i32, y: i32) -> wxHitTest;
        // CXX_UNSUPPORTED: unsafe fn HitTest1(self: *const wxWindow, pt: *const wxPoint) -> wxHitTest;
        // CXX_UNSUPPORTED: unsafe fn GetBorder(self: *const wxWindow, flags: i32) -> wxBorder;
        // CXX_UNSUPPORTED: unsafe fn GetBorder1(self: *const wxWindow) -> wxBorder;
        // GENERATED: unsafe fn DoUpdateWindowUI(self: *mut wxWindow, event: *mut wxUpdateUIEvent);
        // CXX_UNSUPPORTED: unsafe fn GetHandle(self: *const wxWindow) -> WXWidget;
        // GENERATED: unsafe fn HasMultiplePages(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn InheritAttributes(self: *mut wxWindow);
        // GENERATED: unsafe fn InitDialog(self: *mut wxWindow);
        // GENERATED: unsafe fn IsDoubleBuffered(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn SetDoubleBuffered(self: *mut wxWindow, on: bool);
        // GENERATED: unsafe fn IsRetained(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn IsThisEnabled(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn IsTopLevel(self: *const wxWindow) -> bool;
        // GENERATED: unsafe fn OnInternalIdle(self: *mut wxWindow);
        // GENERATED: unsafe fn SendIdleEvents(self: *mut wxWindow, event: *mut wxIdleEvent) -> bool;
        // GENERATED: unsafe fn RegisterHotKey(self: *mut wxWindow, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        // GENERATED: unsafe fn UnregisterHotKey(self: *mut wxWindow, hotkey_id: i32) -> bool;
        // GENERATED: unsafe fn UpdateWindowUI(self: *mut wxWindow, flags: i32);
        // CXX_UNSUPPORTED: unsafe fn GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        // GENERATED: unsafe fn FindFocus() -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowByLabel(label: *const wxString, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowByName(name: *const wxString, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetCapture() -> *mut wxWindow;
        // GENERATED: unsafe fn NewControlId(count: i32) -> i32;
        // GENERATED: unsafe fn UnreserveControlId(id: i32, count: i32);
        // CTOR: unsafe fn wxWindow() -> Window;
        // CTOR: unsafe fn wxWindow1(parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> Window;
        // DTOR: unsafe fn ~wxWindow(self: *mut wxWindow);
        // GENERATED: unsafe fn Create(self: *mut wxWindow, parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> bool;
        
        // CLASS: wxControl
        type wxControl;
        // CTOR: unsafe fn wxControl(parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> Control;
        // CTOR: unsafe fn wxControl1() -> Control;
        // GENERATED: unsafe fn Create(self: *mut wxControl, parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> bool;
        // GENERATED: unsafe fn Command(self: *mut wxControl, event: *mut wxCommandEvent);
        // GENERATED: unsafe fn GetLabel(self: *const wxControl) -> wxString;
        // GENERATED: unsafe fn GetLabelText(self: *const wxControl) -> wxString;
        // GENERATED: unsafe fn GetSizeFromTextSize(self: *const wxControl, xlen: i32, ylen: i32) -> wxSize;
        // GENERATED: unsafe fn GetSizeFromTextSize1(self: *const wxControl, tsize: *const wxSize) -> wxSize;
        // GENERATED: unsafe fn GetSizeFromText(self: *const wxControl, text: *const wxString) -> wxSize;
        // GENERATED: unsafe fn SetLabel(self: *mut wxControl, label: *const wxString);
        // GENERATED: unsafe fn SetLabelText(self: *mut wxControl, text: *const wxString);
        // GENERATED: unsafe fn SetLabelMarkup(self: *mut wxControl, markup: *const wxString) -> bool;
        // GENERATED: unsafe fn GetLabelText1(label: *const wxString) -> wxString;
        // GENERATED: unsafe fn RemoveMnemonics(str: *const wxString) -> wxString;
        // GENERATED: unsafe fn EscapeMnemonics(text: *const wxString) -> wxString;
        // BLOCKED: unsafe fn Ellipsize(label: *const wxString, dc: *const wxDC, mode: i32, max_width: i32, flags: i32) -> wxString;
        
        // CLASS: wxAnyButton
        type wxAnyButton;
        // CTOR: unsafe fn wxAnyButton() -> AnyButton;
        // DTOR: unsafe fn ~wxAnyButton(self: *mut wxAnyButton);
        // CXX_UNSUPPORTED: unsafe fn GetBitmap(self: *const wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapCurrent(self: *const wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapDisabled(self: *const wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapFocus(self: *const wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapLabel(self: *const wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapPressed(self: *const wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn SetBitmap(self: *mut wxAnyButton, bitmap: *const wxBitmap, dir: wxDirection);
        // GENERATED: unsafe fn SetBitmapCurrent(self: *mut wxAnyButton, bitmap: *const wxBitmap);
        // GENERATED: unsafe fn SetBitmapDisabled(self: *mut wxAnyButton, bitmap: *const wxBitmap);
        // GENERATED: unsafe fn SetBitmapFocus(self: *mut wxAnyButton, bitmap: *const wxBitmap);
        // GENERATED: unsafe fn SetBitmapLabel(self: *mut wxAnyButton, bitmap: *const wxBitmap);
        // GENERATED: unsafe fn SetBitmapPressed(self: *mut wxAnyButton, bitmap: *const wxBitmap);
        // GENERATED: unsafe fn GetBitmapMargins(self: *mut wxAnyButton) -> wxSize;
        // GENERATED: unsafe fn SetBitmapMargins(self: *mut wxAnyButton, x: i32, y: i32);
        // GENERATED: unsafe fn SetBitmapMargins1(self: *mut wxAnyButton, sz: *const wxSize);
        // CXX_UNSUPPORTED: unsafe fn SetBitmapPosition(self: *mut wxAnyButton, dir: wxDirection);
        
        // CLASS: wxButton
        type wxButton;
        // CTOR: unsafe fn wxButton() -> Button;
        // CTOR: unsafe fn wxButton1(parent: *mut wxWindow, id: i32, label: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> Button;
        // GENERATED: unsafe fn Create(self: *mut wxButton, parent: *mut wxWindow, id: i32, label: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> bool;
        // GENERATED: unsafe fn GetAuthNeeded(self: *const wxButton) -> bool;
        // GENERATED: unsafe fn GetLabel(self: *const wxButton) -> wxString;
        // GENERATED: unsafe fn SetAuthNeeded(self: *mut wxButton, needed: bool);
        // GENERATED: unsafe fn SetDefault(self: *mut wxButton) -> *mut wxWindow;
        // GENERATED: unsafe fn SetLabel(self: *mut wxButton, label: *const wxString);
        // GENERATED: unsafe fn GetDefaultSize(win: *mut wxWindow) -> wxSize;
        
        // CLASS: wxNonOwnedWindow
        type wxNonOwnedWindow;
        // GENERATED: unsafe fn SetShape(self: *mut wxNonOwnedWindow, region: *const wxRegion) -> bool;
        // GENERATED: unsafe fn SetShape1(self: *mut wxNonOwnedWindow, path: *const wxGraphicsPath) -> bool;
        
        // CLASS: wxTopLevelWindow
        type wxTopLevelWindow;
        // CTOR: unsafe fn wxTopLevelWindow() -> TopLevelWindow;
        // CTOR: unsafe fn wxTopLevelWindow1(parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> TopLevelWindow;
        // DTOR: unsafe fn ~wxTopLevelWindow(self: *mut wxTopLevelWindow);
        // GENERATED: unsafe fn Create(self: *mut wxTopLevelWindow, parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> bool;
        // GENERATED: unsafe fn CanSetTransparent(self: *mut wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn CenterOnScreen(self: *mut wxTopLevelWindow, direction: i32);
        // GENERATED: unsafe fn CentreOnScreen(self: *mut wxTopLevelWindow, direction: i32);
        // GENERATED: unsafe fn EnableCloseButton(self: *mut wxTopLevelWindow, enable: bool) -> bool;
        // GENERATED: unsafe fn EnableMaximizeButton(self: *mut wxTopLevelWindow, enable: bool) -> bool;
        // GENERATED: unsafe fn EnableMinimizeButton(self: *mut wxTopLevelWindow, enable: bool) -> bool;
        // GENERATED: unsafe fn GetDefaultItem(self: *const wxTopLevelWindow) -> *mut wxWindow;
        // CXX_UNSUPPORTED: unsafe fn GetIcon(self: *const wxTopLevelWindow) -> wxIcon;
        // BLOCKED: unsafe fn GetIcons(self: *const wxTopLevelWindow) -> *const wxIconBundle;
        // GENERATED: unsafe fn GetTitle(self: *const wxTopLevelWindow) -> wxString;
        // GENERATED: unsafe fn Iconize(self: *mut wxTopLevelWindow, iconize: bool);
        // GENERATED: unsafe fn IsActive(self: *mut wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsAlwaysMaximized(self: *const wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsFullScreen(self: *const wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsIconized(self: *const wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsMaximized(self: *const wxTopLevelWindow) -> bool;
        // BLOCKED: unsafe fn IsUsingNativeDecorations(self: *const wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn Layout(self: *mut wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn Maximize(self: *mut wxTopLevelWindow, maximize: bool);
        // BLOCKED: unsafe fn MSWGetSystemMenu(self: *const wxTopLevelWindow) -> *mut wxMenu;
        // GENERATED: unsafe fn RequestUserAttention(self: *mut wxTopLevelWindow, flags: i32);
        // GENERATED: unsafe fn Restore(self: *mut wxTopLevelWindow);
        // BLOCKED: unsafe fn RestoreToGeometry(self: *mut wxTopLevelWindow, ser: *mut GeometrySerializer) -> bool;
        // BLOCKED: unsafe fn SaveGeometry(self: *const wxTopLevelWindow, ser: *const GeometrySerializer) -> bool;
        // GENERATED: unsafe fn SetDefaultItem(self: *mut wxTopLevelWindow, win: *mut wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn SetTmpDefaultItem(self: *mut wxTopLevelWindow, win: *mut wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetTmpDefaultItem(self: *const wxTopLevelWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn SetIcon(self: *mut wxTopLevelWindow, icon: *const wxIcon);
        // GENERATED: unsafe fn SetIcons(self: *mut wxTopLevelWindow, icons: *const wxIconBundle);
        // GENERATED: unsafe fn SetMaxSize(self: *mut wxTopLevelWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetMinSize(self: *mut wxTopLevelWindow, size: *const wxSize);
        // GENERATED: unsafe fn SetSizeHints(self: *mut wxTopLevelWindow, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: unsafe fn SetSizeHints1(self: *mut wxTopLevelWindow, min_size: *const wxSize, max_size: *const wxSize, inc_size: *const wxSize);
        // GENERATED: unsafe fn SetTitle(self: *mut wxTopLevelWindow, title: *const wxString);
        // GENERATED: unsafe fn SetTransparent(self: *mut wxTopLevelWindow, alpha: u8) -> bool;
        // GENERATED: unsafe fn ShouldPreventAppExit(self: *const wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn OSXSetModified(self: *mut wxTopLevelWindow, modified: bool);
        // GENERATED: unsafe fn OSXIsModified(self: *const wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn SetRepresentedFilename(self: *mut wxTopLevelWindow, filename: *const wxString);
        // GENERATED: unsafe fn ShowWithoutActivating(self: *mut wxTopLevelWindow);
        // GENERATED: unsafe fn EnableFullScreenView(self: *mut wxTopLevelWindow, enable: bool) -> bool;
        // GENERATED: unsafe fn ShowFullScreen(self: *mut wxTopLevelWindow, show: bool, style: i32) -> bool;
        // BLOCKED: unsafe fn UseNativeDecorations(self: *mut wxTopLevelWindow, native: bool);
        // BLOCKED: unsafe fn UseNativeDecorationsByDefault(self: *mut wxTopLevelWindow, native: bool);
        // GENERATED: unsafe fn GetDefaultSize() -> wxSize;
        
        // CLASS: wxFrame
        type wxFrame;
        // CTOR: unsafe fn wxFrame() -> Frame;
        // CTOR: unsafe fn wxFrame1(parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> Frame;
        // DTOR: unsafe fn ~wxFrame(self: *mut wxFrame);
        // GENERATED: unsafe fn Centre(self: *mut wxFrame, direction: i32);
        // GENERATED: unsafe fn Create(self: *mut wxFrame, parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> bool;
        // GENERATED: unsafe fn CreateStatusBar(self: *mut wxFrame, number: i32, style: i32, id: i32, name: *const wxString) -> *mut wxStatusBar;
        // GENERATED: unsafe fn CreateToolBar(self: *mut wxFrame, style: i32, id: i32, name: *const wxString) -> *mut wxToolBar;
        // GENERATED: unsafe fn DoGiveHelp(self: *mut wxFrame, text: *const wxString, show: bool);
        // GENERATED: unsafe fn GetClientAreaOrigin(self: *const wxFrame) -> wxPoint;
        // GENERATED: unsafe fn GetMenuBar(self: *const wxFrame) -> *mut wxMenuBar;
        // GENERATED: unsafe fn GetStatusBar(self: *const wxFrame) -> *mut wxStatusBar;
        // GENERATED: unsafe fn GetStatusBarPane(self: *const wxFrame) -> i32;
        // GENERATED: unsafe fn GetToolBar(self: *const wxFrame) -> *mut wxToolBar;
        // GENERATED: unsafe fn OnCreateStatusBar(self: *mut wxFrame, number: i32, style: i32, id: i32, name: *const wxString) -> *mut wxStatusBar;
        // GENERATED: unsafe fn OnCreateToolBar(self: *mut wxFrame, style: i32, id: i32, name: *const wxString) -> *mut wxToolBar;
        // GENERATED: unsafe fn ProcessCommand(self: *mut wxFrame, id: i32) -> bool;
        // GENERATED: unsafe fn SetMenuBar(self: *mut wxFrame, menu_bar: *mut wxMenuBar);
        // GENERATED: unsafe fn SetStatusBar(self: *mut wxFrame, status_bar: *mut wxStatusBar);
        // GENERATED: unsafe fn SetStatusBarPane(self: *mut wxFrame, n: i32);
        // GENERATED: unsafe fn SetStatusText(self: *mut wxFrame, text: *const wxString, number: i32);
        // GENERATED: unsafe fn SetStatusWidths(self: *mut wxFrame, n: i32, widths_field: *const i32);
        // GENERATED: unsafe fn SetToolBar(self: *mut wxFrame, tool_bar: *mut wxToolBar);
        // BLOCKED: unsafe fn MSWGetTaskBarButton(self: *mut wxFrame) -> *mut wxTaskBarButton;
        // GENERATED: unsafe fn PushStatusText(self: *mut wxFrame, text: *const wxString, number: i32);
        // GENERATED: unsafe fn PopStatusText(self: *mut wxFrame, number: i32);
        
        // CLASS: wxPoint
        type wxPoint;
        // GENERATED: unsafe fn IsFullySpecified(self: *const wxPoint) -> bool;
        // GENERATED: unsafe fn SetDefaults(self: *mut wxPoint, pt: *const wxPoint);
        // BLOCKED: unsafe fn operator=(self: *mut wxPoint, pt: *const wxPoint) -> *mut wxPoint;
        // BLOCKED: unsafe fn operator==(self: *mut wxPoint, p1: *const wxPoint, p2: *const wxPoint) -> bool;
        // BLOCKED: unsafe fn operator!=(self: *mut wxPoint, p1: *const wxPoint, p2: *const wxPoint) -> bool;
        // BLOCKED: unsafe fn operator+(self: *mut wxPoint, p1: *const wxPoint, p2: *const wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator-(self: *mut wxPoint, p1: *const wxPoint, p2: *const wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator+=(self: *mut wxPoint, pt: *const wxPoint) -> *mut wxPoint;
        // BLOCKED: unsafe fn operator-=(self: *mut wxPoint, pt: *const wxPoint) -> *mut wxPoint;
        // BLOCKED: unsafe fn operator+1(self: *mut wxPoint, pt: *const wxPoint, sz: *const wxSize) -> wxPoint;
        // BLOCKED: unsafe fn operator-1(self: *mut wxPoint, pt: *const wxPoint, sz: *const wxSize) -> wxPoint;
        // BLOCKED: unsafe fn operator+2(self: *mut wxPoint, sz: *const wxSize, pt: *const wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator-2(self: *mut wxPoint, sz: *const wxSize, pt: *const wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator+=1(self: *mut wxPoint, sz: *const wxSize) -> *mut wxPoint;
        // BLOCKED: unsafe fn operator-=1(self: *mut wxPoint, sz: *const wxSize) -> *mut wxPoint;
        // BLOCKED: unsafe fn operator/(self: *mut wxPoint, sz: *const wxPoint, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*(self: *mut wxPoint, sz: *const wxPoint, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*1(self: *mut wxPoint, factor: i32, sz: *const wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator/=(self: *mut wxPoint, factor: i32) -> *mut wxSize;
        // BLOCKED: unsafe fn operator*=(self: *mut wxPoint, factor: i32) -> *mut wxSize;
        // CTOR: unsafe fn wxPoint() -> Point;
        // CTOR: unsafe fn wxPoint1(x: i32, y: i32) -> Point;
        // CTOR: unsafe fn wxPoint2(pt: *const wxRealPoint) -> Point;
        
        // CLASS: wxRect
        type wxRect;
        // CTOR: unsafe fn wxRect() -> Rect;
        // CTOR: unsafe fn wxRect1(x: i32, y: i32, width: i32, height: i32) -> Rect;
        // CTOR: unsafe fn wxRect2(top_left: *const wxPoint, bottom_right: *const wxPoint) -> Rect;
        // CTOR: unsafe fn wxRect3(pos: *const wxPoint, size: *const wxSize) -> Rect;
        // CTOR: unsafe fn wxRect4(size: *const wxSize) -> Rect;
        // GENERATED: unsafe fn CentreIn(self: *const wxRect, r: *const wxRect, dir: i32) -> wxRect;
        // GENERATED: unsafe fn CenterIn(self: *const wxRect, r: *const wxRect, dir: i32) -> wxRect;
        // GENERATED: unsafe fn Contains(self: *const wxRect, x: i32, y: i32) -> bool;
        // GENERATED: unsafe fn Contains1(self: *const wxRect, pt: *const wxPoint) -> bool;
        // GENERATED: unsafe fn Contains2(self: *const wxRect, rect: *const wxRect) -> bool;
        // BLOCKED: unsafe fn Deflate(self: *mut wxRect, dx: i32, dy: i32) -> *mut wxRect;
        // BLOCKED: unsafe fn Deflate1(self: *mut wxRect, diff: *const wxSize) -> *mut wxRect;
        // BLOCKED: unsafe fn Deflate2(self: *mut wxRect, diff: i32) -> *mut wxRect;
        // GENERATED: unsafe fn Deflate3(self: *const wxRect, dx: i32, dy: i32) -> wxRect;
        // GENERATED: unsafe fn GetBottom(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetBottomLeft(self: *const wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetBottomRight(self: *const wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetHeight(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetLeft(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetPosition(self: *const wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetRight(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetSize(self: *const wxRect) -> wxSize;
        // GENERATED: unsafe fn GetTop(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetTopLeft(self: *const wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetTopRight(self: *const wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetWidth(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetX(self: *const wxRect) -> i32;
        // GENERATED: unsafe fn GetY(self: *const wxRect) -> i32;
        // BLOCKED: unsafe fn Inflate(self: *mut wxRect, dx: i32, dy: i32) -> *mut wxRect;
        // BLOCKED: unsafe fn Inflate1(self: *mut wxRect, diff: *const wxSize) -> *mut wxRect;
        // BLOCKED: unsafe fn Inflate2(self: *mut wxRect, diff: i32) -> *mut wxRect;
        // GENERATED: unsafe fn Inflate3(self: *const wxRect, dx: i32, dy: i32) -> wxRect;
        // BLOCKED: unsafe fn Intersect(self: *mut wxRect, rect: *const wxRect) -> *mut wxRect;
        // GENERATED: unsafe fn Intersect1(self: *const wxRect, rect: *const wxRect) -> wxRect;
        // GENERATED: unsafe fn Intersects(self: *const wxRect, rect: *const wxRect) -> bool;
        // GENERATED: unsafe fn IsEmpty(self: *const wxRect) -> bool;
        // GENERATED: unsafe fn Offset(self: *mut wxRect, dx: i32, dy: i32);
        // GENERATED: unsafe fn Offset1(self: *mut wxRect, pt: *const wxPoint);
        // GENERATED: unsafe fn SetHeight(self: *mut wxRect, height: i32);
        // GENERATED: unsafe fn SetPosition(self: *mut wxRect, pos: *const wxPoint);
        // GENERATED: unsafe fn SetSize(self: *mut wxRect, s: *const wxSize);
        // GENERATED: unsafe fn SetWidth(self: *mut wxRect, width: i32);
        // GENERATED: unsafe fn SetX(self: *mut wxRect, x: i32);
        // GENERATED: unsafe fn SetY(self: *mut wxRect, y: i32);
        // GENERATED: unsafe fn SetLeft(self: *mut wxRect, left: i32);
        // GENERATED: unsafe fn SetRight(self: *mut wxRect, right: i32);
        // GENERATED: unsafe fn SetTop(self: *mut wxRect, top: i32);
        // GENERATED: unsafe fn SetBottom(self: *mut wxRect, bottom: i32);
        // GENERATED: unsafe fn SetTopLeft(self: *mut wxRect, p: *const wxPoint);
        // GENERATED: unsafe fn SetBottomRight(self: *mut wxRect, p: *const wxPoint);
        // GENERATED: unsafe fn SetTopRight(self: *mut wxRect, p: *const wxPoint);
        // GENERATED: unsafe fn SetBottomLeft(self: *mut wxRect, p: *const wxPoint);
        // GENERATED: unsafe fn Union(self: *const wxRect, rect: *const wxRect) -> wxRect;
        // BLOCKED: unsafe fn Union1(self: *mut wxRect, rect: *const wxRect) -> *mut wxRect;
        // BLOCKED: unsafe fn operator!=(self: *mut wxRect, r1: *const wxRect, r2: *const wxRect) -> bool;
        // BLOCKED: unsafe fn operator+(self: *mut wxRect, r1: *const wxRect, r2: *const wxRect) -> wxRect;
        // BLOCKED: unsafe fn operator+=(self: *mut wxRect, r: *const wxRect) -> *mut wxRect;
        // BLOCKED: unsafe fn operator*(self: *mut wxRect, r1: *const wxRect, r2: *const wxRect) -> wxRect;
        // BLOCKED: unsafe fn operator*=(self: *mut wxRect, r: *const wxRect) -> *mut wxRect;
        // BLOCKED: unsafe fn operator=(self: *mut wxRect, rect: *const wxRect) -> *mut wxRect;
        // BLOCKED: unsafe fn operator==(self: *mut wxRect, r1: *const wxRect, r2: *const wxRect) -> bool;
        
        // CLASS: wxSize
        type wxSize;
        // BLOCKED: unsafe fn operator=(self: *mut wxSize, sz: *const wxSize) -> *mut wxSize;
        // BLOCKED: unsafe fn operator==(self: *mut wxSize, s1: *const wxSize, s2: *const wxSize) -> bool;
        // BLOCKED: unsafe fn operator!=(self: *mut wxSize, s1: *const wxSize, s2: *const wxSize) -> bool;
        // BLOCKED: unsafe fn operator+(self: *mut wxSize, s1: *const wxSize, s2: *const wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator-(self: *mut wxSize, s1: *const wxSize, s2: *const wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator+=(self: *mut wxSize, sz: *const wxSize) -> *mut wxSize;
        // BLOCKED: unsafe fn operator-=(self: *mut wxSize, sz: *const wxSize) -> *mut wxSize;
        // BLOCKED: unsafe fn operator/(self: *mut wxSize, sz: *const wxSize, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*(self: *mut wxSize, sz: *const wxSize, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*1(self: *mut wxSize, factor: i32, sz: *const wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator/=(self: *mut wxSize, factor: i32) -> *mut wxSize;
        // BLOCKED: unsafe fn operator*=(self: *mut wxSize, factor: i32) -> *mut wxSize;
        // CTOR: unsafe fn wxSize() -> Size;
        // CTOR: unsafe fn wxSize1(width: i32, height: i32) -> Size;
        // GENERATED: unsafe fn DecBy(self: *mut wxSize, pt: *const wxPoint);
        // GENERATED: unsafe fn DecBy1(self: *mut wxSize, size: *const wxSize);
        // GENERATED: unsafe fn DecBy2(self: *mut wxSize, dx: i32, dy: i32);
        // GENERATED: unsafe fn DecBy3(self: *mut wxSize, d: i32);
        // GENERATED: unsafe fn DecTo(self: *mut wxSize, size: *const wxSize);
        // GENERATED: unsafe fn DecToIfSpecified(self: *mut wxSize, size: *const wxSize);
        // GENERATED: unsafe fn GetHeight(self: *const wxSize) -> i32;
        // GENERATED: unsafe fn GetWidth(self: *const wxSize) -> i32;
        // GENERATED: unsafe fn IncBy(self: *mut wxSize, pt: *const wxPoint);
        // GENERATED: unsafe fn IncBy1(self: *mut wxSize, size: *const wxSize);
        // GENERATED: unsafe fn IncBy2(self: *mut wxSize, dx: i32, dy: i32);
        // GENERATED: unsafe fn IncBy3(self: *mut wxSize, d: i32);
        // GENERATED: unsafe fn IncTo(self: *mut wxSize, size: *const wxSize);
        // GENERATED: unsafe fn IsFullySpecified(self: *const wxSize) -> bool;
        // BLOCKED: unsafe fn Scale(self: *mut wxSize, xscale: f64, yscale: f64) -> *mut wxSize;
        // GENERATED: unsafe fn Set(self: *mut wxSize, width: i32, height: i32);
        // GENERATED: unsafe fn SetDefaults(self: *mut wxSize, size_default: *const wxSize);
        // GENERATED: unsafe fn SetHeight(self: *mut wxSize, height: i32);
        // GENERATED: unsafe fn SetWidth(self: *mut wxSize, width: i32);
        
        // CLASS: wxValidator
        type wxValidator;
        // CTOR: unsafe fn wxValidator() -> Validator;
        // DTOR: unsafe fn ~wxValidator(self: *mut wxValidator);
        // GENERATED: unsafe fn Clone(self: *const wxValidator) -> *mut wxObject;
        // GENERATED: unsafe fn GetWindow(self: *const wxValidator) -> *mut wxWindow;
        // GENERATED: unsafe fn SetWindow(self: *mut wxValidator, window: *mut wxWindow);
        // GENERATED: unsafe fn TransferFromWindow(self: *mut wxValidator) -> bool;
        // GENERATED: unsafe fn TransferToWindow(self: *mut wxValidator) -> bool;
        // GENERATED: unsafe fn Validate(self: *mut wxValidator, parent: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn SuppressBellOnError(suppress: bool);
        // GENERATED: unsafe fn IsSilent() -> bool;
    }
    unsafe extern "C++" {
        // CLASS: wxObject
        unsafe fn wxObject_new() -> *mut wxObject;
        unsafe fn wxObject_new1(other: *const wxObject) -> *mut wxObject;
        unsafe fn wxObject_GetClassInfo(self_: *const wxObject) -> *mut wxClassInfo;
        unsafe fn wxObject_GetRefData(self_: *const wxObject) -> *mut wxObjectRefData;
        unsafe fn wxObject_IsKindOf(self_: *const wxObject, info: *const wxClassInfo) -> bool;
        unsafe fn wxObject_IsSameAs(self_: *const wxObject, obj: *const wxObject) -> bool;
        unsafe fn wxObject_Ref(self_: *mut wxObject, clone: *const wxObject);
        unsafe fn wxObject_SetRefData(self_: *mut wxObject, data: *mut wxObjectRefData);
        unsafe fn wxObject_UnRef(self_: *mut wxObject);
        unsafe fn wxObject_UnShare(self_: *mut wxObject);
        // CLASS: wxEvtHandler
        unsafe fn wxEvtHandler_QueueEvent(self_: *mut wxEvtHandler, event: *mut wxEvent);
        unsafe fn wxEvtHandler_AddPendingEvent(self_: *mut wxEvtHandler, event: *const wxEvent);
        unsafe fn wxEvtHandler_ProcessEvent(self_: *mut wxEvtHandler, event: *mut wxEvent) -> bool;
        unsafe fn wxEvtHandler_ProcessEventLocally(self_: *mut wxEvtHandler, event: *mut wxEvent) -> bool;
        unsafe fn wxEvtHandler_SafelyProcessEvent(self_: *mut wxEvtHandler, event: *mut wxEvent) -> bool;
        unsafe fn wxEvtHandler_ProcessPendingEvents(self_: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_DeletePendingEvents(self_: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_GetClientObject(self_: *const wxEvtHandler) -> *mut wxClientData;
        unsafe fn wxEvtHandler_SetClientObject(self_: *mut wxEvtHandler, data: *mut wxClientData);
        unsafe fn wxEvtHandler_GetEvtHandlerEnabled(self_: *const wxEvtHandler) -> bool;
        unsafe fn wxEvtHandler_GetNextHandler(self_: *const wxEvtHandler) -> *mut wxEvtHandler;
        unsafe fn wxEvtHandler_GetPreviousHandler(self_: *const wxEvtHandler) -> *mut wxEvtHandler;
        unsafe fn wxEvtHandler_SetEvtHandlerEnabled(self_: *mut wxEvtHandler, enabled: bool);
        unsafe fn wxEvtHandler_SetNextHandler(self_: *mut wxEvtHandler, handler: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_SetPreviousHandler(self_: *mut wxEvtHandler, handler: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_Unlink(self_: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_IsUnlinked(self_: *const wxEvtHandler) -> bool;
        unsafe fn wxEvtHandler_AddFilter(filter: *mut wxEventFilter);
        unsafe fn wxEvtHandler_RemoveFilter(filter: *mut wxEventFilter);
        unsafe fn wxEvtHandler_new() -> *mut wxEvtHandler;
        // CLASS: wxWindow
        unsafe fn wxWindow_AcceptsFocus(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_AcceptsFocusFromKeyboard(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_AcceptsFocusRecursively(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_DisableFocusFromKeyboard(self_: *mut wxWindow);
        unsafe fn wxWindow_IsFocusable(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_CanAcceptFocus(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_CanAcceptFocusFromKeyboard(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_HasFocus(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_SetCanFocus(self_: *mut wxWindow, can_focus: bool);
        unsafe fn wxWindow_EnableVisibleFocus(self_: *mut wxWindow, enable: bool);
        unsafe fn wxWindow_SetFocus(self_: *mut wxWindow);
        unsafe fn wxWindow_SetFocusFromKbd(self_: *mut wxWindow);
        unsafe fn wxWindow_AddChild(self_: *mut wxWindow, child: *mut wxWindow);
        unsafe fn wxWindow_DestroyChildren(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_FindWindow(self_: *const wxWindow, id: i32) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindow1(self_: *const wxWindow, name: *const wxString) -> *mut wxWindow;
        unsafe fn wxWindow_RemoveChild(self_: *mut wxWindow, child: *mut wxWindow);
        unsafe fn wxWindow_GetGrandParent(self_: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetNextSibling(self_: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetParent(self_: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetPrevSibling(self_: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_IsDescendant(self_: *const wxWindow, win: *mut wxWindow) -> bool;
        unsafe fn wxWindow_Reparent(self_: *mut wxWindow, new_parent: *mut wxWindow) -> bool;
        unsafe fn wxWindow_AlwaysShowScrollbars(self_: *mut wxWindow, hflag: bool, vflag: bool);
        unsafe fn wxWindow_GetScrollPos(self_: *const wxWindow, orientation: i32) -> i32;
        unsafe fn wxWindow_GetScrollRange(self_: *const wxWindow, orientation: i32) -> i32;
        unsafe fn wxWindow_GetScrollThumb(self_: *const wxWindow, orientation: i32) -> i32;
        unsafe fn wxWindow_CanScroll(self_: *const wxWindow, orient: i32) -> bool;
        unsafe fn wxWindow_HasScrollbar(self_: *const wxWindow, orient: i32) -> bool;
        unsafe fn wxWindow_IsScrollbarAlwaysShown(self_: *const wxWindow, orient: i32) -> bool;
        unsafe fn wxWindow_ScrollLines(self_: *mut wxWindow, lines: i32) -> bool;
        unsafe fn wxWindow_ScrollPages(self_: *mut wxWindow, pages: i32) -> bool;
        unsafe fn wxWindow_ScrollWindow(self_: *mut wxWindow, dx: i32, dy: i32, rect: *const wxRect);
        unsafe fn wxWindow_LineUp(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_LineDown(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_PageUp(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_PageDown(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_SetScrollPos(self_: *mut wxWindow, orientation: i32, pos: i32, refresh: bool);
        unsafe fn wxWindow_SetScrollbar(self_: *mut wxWindow, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        unsafe fn wxWindow_BeginRepositioningChildren(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_EndRepositioningChildren(self_: *mut wxWindow);
        unsafe fn wxWindow_CacheBestSize(self_: *const wxWindow, size: *const wxSize);
        unsafe fn wxWindow_ClientToWindowSize(self_: *const wxWindow, size: *const wxSize) -> *mut wxSize;
        unsafe fn wxWindow_WindowToClientSize(self_: *const wxWindow, size: *const wxSize) -> *mut wxSize;
        unsafe fn wxWindow_Fit(self_: *mut wxWindow);
        unsafe fn wxWindow_FitInside(self_: *mut wxWindow);
        unsafe fn wxWindow_FromDIP(self_: *const wxWindow, sz: *const wxSize) -> *mut wxSize;
        unsafe fn wxWindow_FromDIP1(self_: *const wxWindow, pt: *const wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_FromDIP2(self_: *const wxWindow, d: i32) -> i32;
        unsafe fn wxWindow_ToDIP(self_: *const wxWindow, sz: *const wxSize) -> *mut wxSize;
        unsafe fn wxWindow_ToDIP1(self_: *const wxWindow, pt: *const wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ToDIP2(self_: *const wxWindow, d: i32) -> i32;
        unsafe fn wxWindow_GetBestSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetBestHeight(self_: *const wxWindow, width: i32) -> i32;
        unsafe fn wxWindow_GetBestWidth(self_: *const wxWindow, height: i32) -> i32;
        unsafe fn wxWindow_GetClientSize(self_: *const wxWindow, width: *mut i32, height: *mut i32);
        unsafe fn wxWindow_GetClientSize1(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetEffectiveMinSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMaxClientSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMaxSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMinClientSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMinSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMinWidth(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetMinHeight(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetMaxWidth(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetMaxHeight(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetSize(self_: *const wxWindow, width: *mut i32, height: *mut i32);
        unsafe fn wxWindow_GetSize1(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetVirtualSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetVirtualSize1(self_: *const wxWindow, width: *mut i32, height: *mut i32);
        unsafe fn wxWindow_GetBestVirtualSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetContentScaleFactor(self_: *const wxWindow) -> f64;
        unsafe fn wxWindow_GetDPIScaleFactor(self_: *const wxWindow) -> f64;
        unsafe fn wxWindow_GetWindowBorderSize(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_InformFirstDirection(self_: *mut wxWindow, direction: i32, size: i32, available_other_dir: i32) -> bool;
        unsafe fn wxWindow_InvalidateBestSize(self_: *mut wxWindow);
        unsafe fn wxWindow_PostSizeEvent(self_: *mut wxWindow);
        unsafe fn wxWindow_PostSizeEventToParent(self_: *mut wxWindow);
        unsafe fn wxWindow_SendSizeEvent(self_: *mut wxWindow, flags: i32);
        unsafe fn wxWindow_SendSizeEventToParent(self_: *mut wxWindow, flags: i32);
        unsafe fn wxWindow_SetClientSize(self_: *mut wxWindow, width: i32, height: i32);
        unsafe fn wxWindow_SetClientSize1(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetClientSize2(self_: *mut wxWindow, rect: *const wxRect);
        unsafe fn wxWindow_SetContainingSizer(self_: *mut wxWindow, sizer: *mut wxSizer);
        unsafe fn wxWindow_SetInitialSize(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetMaxClientSize(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetMaxSize(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetMinClientSize(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetMinSize(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetSize(self_: *mut wxWindow, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        unsafe fn wxWindow_SetSize1(self_: *mut wxWindow, rect: *const wxRect);
        unsafe fn wxWindow_SetSize2(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_SetSize3(self_: *mut wxWindow, width: i32, height: i32);
        unsafe fn wxWindow_SetSizeHints(self_: *mut wxWindow, min_size: *const wxSize, max_size: *const wxSize, inc_size: *const wxSize);
        unsafe fn wxWindow_SetSizeHints1(self_: *mut wxWindow, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        unsafe fn wxWindow_SetVirtualSize(self_: *mut wxWindow, width: i32, height: i32);
        unsafe fn wxWindow_SetVirtualSize1(self_: *mut wxWindow, size: *const wxSize);
        unsafe fn wxWindow_FromDIP3(sz: *const wxSize, w: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_FromDIP4(pt: *const wxPoint, w: *const wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_FromDIP5(d: i32, w: *const wxWindow) -> i32;
        unsafe fn wxWindow_ToDIP3(sz: *const wxSize, w: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_ToDIP4(pt: *const wxPoint, w: *const wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_ToDIP5(d: i32, w: *const wxWindow) -> i32;
        unsafe fn wxWindow_Center(self_: *mut wxWindow, dir: i32);
        unsafe fn wxWindow_CenterOnParent(self_: *mut wxWindow, dir: i32);
        unsafe fn wxWindow_Centre(self_: *mut wxWindow, direction: i32);
        unsafe fn wxWindow_CentreOnParent(self_: *mut wxWindow, direction: i32);
        unsafe fn wxWindow_GetPosition(self_: *const wxWindow, x: *mut i32, y: *mut i32);
        unsafe fn wxWindow_GetPosition1(self_: *const wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_GetRect(self_: *const wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_GetScreenPosition(self_: *const wxWindow, x: *mut i32, y: *mut i32);
        unsafe fn wxWindow_GetScreenPosition1(self_: *const wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_GetScreenRect(self_: *const wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_GetClientAreaOrigin(self_: *const wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_GetClientRect(self_: *const wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_Move(self_: *mut wxWindow, x: i32, y: i32, flags: i32);
        unsafe fn wxWindow_Move1(self_: *mut wxWindow, pt: *const wxPoint, flags: i32);
        unsafe fn wxWindow_SetPosition(self_: *mut wxWindow, pt: *const wxPoint);
        unsafe fn wxWindow_ClientToScreen(self_: *const wxWindow, x: *mut i32, y: *mut i32);
        unsafe fn wxWindow_ClientToScreen1(self_: *const wxWindow, pt: *const wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ConvertDialogToPixels(self_: *const wxWindow, pt: *const wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ConvertDialogToPixels1(self_: *const wxWindow, sz: *const wxSize) -> *mut wxSize;
        unsafe fn wxWindow_ConvertPixelsToDialog(self_: *const wxWindow, pt: *const wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ConvertPixelsToDialog1(self_: *const wxWindow, sz: *const wxSize) -> *mut wxSize;
        unsafe fn wxWindow_ScreenToClient(self_: *const wxWindow, x: *mut i32, y: *mut i32);
        unsafe fn wxWindow_ScreenToClient1(self_: *const wxWindow, pt: *const wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ClearBackground(self_: *mut wxWindow);
        unsafe fn wxWindow_Freeze(self_: *mut wxWindow);
        unsafe fn wxWindow_Thaw(self_: *mut wxWindow);
        unsafe fn wxWindow_IsFrozen(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_GetCharHeight(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetCharWidth(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetDPI(self_: *const wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetTextExtent(self_: *const wxWindow, string: *const wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        unsafe fn wxWindow_GetTextExtent1(self_: *const wxWindow, string: *const wxString) -> *mut wxSize;
        unsafe fn wxWindow_GetUpdateClientRect(self_: *const wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_HasTransparentBackground(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_Refresh(self_: *mut wxWindow, erase_background: bool, rect: *const wxRect);
        unsafe fn wxWindow_RefreshRect(self_: *mut wxWindow, rect: *const wxRect, erase_background: bool);
        unsafe fn wxWindow_Update(self_: *mut wxWindow);
        unsafe fn wxWindow_SetBackgroundColour(self_: *mut wxWindow, colour: *const wxColour) -> bool;
        unsafe fn wxWindow_IsTransparentBackgroundSupported(self_: *const wxWindow, reason: *mut wxString) -> bool;
        unsafe fn wxWindow_SetFont(self_: *mut wxWindow, font: *const wxFont) -> bool;
        unsafe fn wxWindow_SetForegroundColour(self_: *mut wxWindow, colour: *const wxColour) -> bool;
        unsafe fn wxWindow_SetOwnBackgroundColour(self_: *mut wxWindow, colour: *const wxColour);
        unsafe fn wxWindow_InheritsBackgroundColour(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_UseBgCol(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_UseBackgroundColour(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_SetOwnFont(self_: *mut wxWindow, font: *const wxFont);
        unsafe fn wxWindow_SetOwnForegroundColour(self_: *mut wxWindow, colour: *const wxColour);
        unsafe fn wxWindow_UseForegroundColour(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_InheritsForegroundColour(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_SetPalette(self_: *mut wxWindow, pal: *const wxPalette);
        unsafe fn wxWindow_ShouldInheritColours(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_SetThemeEnabled(self_: *mut wxWindow, enable: bool);
        unsafe fn wxWindow_GetThemeEnabled(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_CanSetTransparent(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_SetTransparent(self_: *mut wxWindow, alpha: u8) -> bool;
        unsafe fn wxWindow_GetEventHandler(self_: *const wxWindow) -> *mut wxEvtHandler;
        unsafe fn wxWindow_HandleAsNavigationKey(self_: *mut wxWindow, event: *const wxKeyEvent) -> bool;
        unsafe fn wxWindow_HandleWindowEvent(self_: *const wxWindow, event: *mut wxEvent) -> bool;
        unsafe fn wxWindow_ProcessWindowEvent(self_: *mut wxWindow, event: *mut wxEvent) -> bool;
        unsafe fn wxWindow_ProcessWindowEventLocally(self_: *mut wxWindow, event: *mut wxEvent) -> bool;
        unsafe fn wxWindow_PopEventHandler(self_: *mut wxWindow, delete_handler: bool) -> *mut wxEvtHandler;
        unsafe fn wxWindow_PushEventHandler(self_: *mut wxWindow, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_RemoveEventHandler(self_: *mut wxWindow, handler: *mut wxEvtHandler) -> bool;
        unsafe fn wxWindow_SetEventHandler(self_: *mut wxWindow, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_SetNextHandler(self_: *mut wxWindow, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_SetPreviousHandler(self_: *mut wxWindow, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_GetExtraStyle(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetWindowStyleFlag(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetWindowStyle(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_HasExtraStyle(self_: *const wxWindow, ex_flag: i32) -> bool;
        unsafe fn wxWindow_HasFlag(self_: *const wxWindow, flag: i32) -> bool;
        unsafe fn wxWindow_SetExtraStyle(self_: *mut wxWindow, ex_style: i32);
        unsafe fn wxWindow_SetWindowStyleFlag(self_: *mut wxWindow, style: i32);
        unsafe fn wxWindow_SetWindowStyle(self_: *mut wxWindow, style: i32);
        unsafe fn wxWindow_ToggleWindowStyle(self_: *mut wxWindow, flag: i32) -> bool;
        unsafe fn wxWindow_MoveAfterInTabOrder(self_: *mut wxWindow, win: *mut wxWindow);
        unsafe fn wxWindow_MoveBeforeInTabOrder(self_: *mut wxWindow, win: *mut wxWindow);
        unsafe fn wxWindow_Navigate(self_: *mut wxWindow, flags: i32) -> bool;
        unsafe fn wxWindow_NavigateIn(self_: *mut wxWindow, flags: i32) -> bool;
        unsafe fn wxWindow_Lower(self_: *mut wxWindow);
        unsafe fn wxWindow_Raise(self_: *mut wxWindow);
        unsafe fn wxWindow_Hide(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_IsEnabled(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_IsExposed(self_: *const wxWindow, x: i32, y: i32) -> bool;
        unsafe fn wxWindow_IsExposed1(self_: *const wxWindow, pt: *mut wxPoint) -> bool;
        unsafe fn wxWindow_IsExposed2(self_: *const wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        unsafe fn wxWindow_IsExposed3(self_: *const wxWindow, rect: *mut wxRect) -> bool;
        unsafe fn wxWindow_IsShown(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_IsShownOnScreen(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_Disable(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_Enable(self_: *mut wxWindow, enable: bool) -> bool;
        unsafe fn wxWindow_Show(self_: *mut wxWindow, show: bool) -> bool;
        unsafe fn wxWindow_GetHelpText(self_: *const wxWindow) -> *mut wxString;
        unsafe fn wxWindow_SetHelpText(self_: *mut wxWindow, help_text: *const wxString);
        unsafe fn wxWindow_GetToolTip(self_: *const wxWindow) -> *mut wxToolTip;
        unsafe fn wxWindow_GetToolTipText(self_: *const wxWindow) -> *mut wxString;
        unsafe fn wxWindow_SetToolTip(self_: *mut wxWindow, tip_string: *const wxString);
        unsafe fn wxWindow_SetToolTip1(self_: *mut wxWindow, tip: *mut wxToolTip);
        unsafe fn wxWindow_UnsetToolTip(self_: *mut wxWindow);
        unsafe fn wxWindow_GetPopupMenuSelectionFromUser(self_: *mut wxWindow, menu: *mut wxMenu, pos: *const wxPoint) -> i32;
        unsafe fn wxWindow_GetPopupMenuSelectionFromUser1(self_: *mut wxWindow, menu: *mut wxMenu, x: i32, y: i32) -> i32;
        unsafe fn wxWindow_PopupMenu(self_: *mut wxWindow, menu: *mut wxMenu, pos: *const wxPoint) -> bool;
        unsafe fn wxWindow_PopupMenu1(self_: *mut wxWindow, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        unsafe fn wxWindow_GetValidator(self_: *mut wxWindow) -> *mut wxValidator;
        unsafe fn wxWindow_SetValidator(self_: *mut wxWindow, validator: *const wxValidator);
        unsafe fn wxWindow_TransferDataFromWindow(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_TransferDataToWindow(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_Validate(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_GetId(self_: *const wxWindow) -> i32;
        unsafe fn wxWindow_GetLabel(self_: *const wxWindow) -> *mut wxString;
        unsafe fn wxWindow_AdjustForLayoutDirection(self_: *const wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        unsafe fn wxWindow_GetName(self_: *const wxWindow) -> *mut wxString;
        unsafe fn wxWindow_SetId(self_: *mut wxWindow, winid: i32);
        unsafe fn wxWindow_SetLabel(self_: *mut wxWindow, label: *const wxString);
        unsafe fn wxWindow_SetName(self_: *mut wxWindow, name: *const wxString);
        unsafe fn wxWindow_GetAcceleratorTable(self_: *mut wxWindow) -> *mut wxAcceleratorTable;
        unsafe fn wxWindow_SetAcceleratorTable(self_: *mut wxWindow, accel: *const wxAcceleratorTable);
        unsafe fn wxWindow_Close(self_: *mut wxWindow, force: bool) -> bool;
        unsafe fn wxWindow_Destroy(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_IsBeingDeleted(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_GetDropTarget(self_: *const wxWindow) -> *mut wxDropTarget;
        unsafe fn wxWindow_SetDropTarget(self_: *mut wxWindow, target: *mut wxDropTarget);
        unsafe fn wxWindow_DragAcceptFiles(self_: *mut wxWindow, accept: bool);
        unsafe fn wxWindow_GetContainingSizer(self_: *const wxWindow) -> *mut wxSizer;
        unsafe fn wxWindow_GetSizer(self_: *const wxWindow) -> *mut wxSizer;
        unsafe fn wxWindow_SetSizer(self_: *mut wxWindow, sizer: *mut wxSizer, delete_old: bool);
        unsafe fn wxWindow_SetSizerAndFit(self_: *mut wxWindow, sizer: *mut wxSizer, delete_old: bool);
        unsafe fn wxWindow_GetConstraints(self_: *const wxWindow) -> *mut wxLayoutConstraints;
        unsafe fn wxWindow_SetConstraints(self_: *mut wxWindow, constraints: *mut wxLayoutConstraints);
        unsafe fn wxWindow_Layout(self_: *mut wxWindow) -> bool;
        unsafe fn wxWindow_SetAutoLayout(self_: *mut wxWindow, auto_layout: bool);
        unsafe fn wxWindow_GetAutoLayout(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_CaptureMouse(self_: *mut wxWindow);
        unsafe fn wxWindow_GetCaret(self_: *const wxWindow) -> *mut wxCaret;
        unsafe fn wxWindow_HasCapture(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_ReleaseMouse(self_: *mut wxWindow);
        unsafe fn wxWindow_SetCaret(self_: *mut wxWindow, caret: *mut wxCaret);
        unsafe fn wxWindow_SetCursor(self_: *mut wxWindow, cursor: *const wxCursor) -> bool;
        unsafe fn wxWindow_WarpPointer(self_: *mut wxWindow, x: i32, y: i32);
        unsafe fn wxWindow_EnableTouchEvents(self_: *mut wxWindow, events_mask: i32) -> bool;
        unsafe fn wxWindow_DoUpdateWindowUI(self_: *mut wxWindow, event: *mut wxUpdateUIEvent);
        unsafe fn wxWindow_HasMultiplePages(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_InheritAttributes(self_: *mut wxWindow);
        unsafe fn wxWindow_InitDialog(self_: *mut wxWindow);
        unsafe fn wxWindow_IsDoubleBuffered(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_SetDoubleBuffered(self_: *mut wxWindow, on: bool);
        unsafe fn wxWindow_IsRetained(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_IsThisEnabled(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_IsTopLevel(self_: *const wxWindow) -> bool;
        unsafe fn wxWindow_OnInternalIdle(self_: *mut wxWindow);
        unsafe fn wxWindow_SendIdleEvents(self_: *mut wxWindow, event: *mut wxIdleEvent) -> bool;
        unsafe fn wxWindow_RegisterHotKey(self_: *mut wxWindow, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        unsafe fn wxWindow_UnregisterHotKey(self_: *mut wxWindow, hotkey_id: i32) -> bool;
        unsafe fn wxWindow_UpdateWindowUI(self_: *mut wxWindow, flags: i32);
        unsafe fn wxWindow_FindFocus() -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowByLabel(label: *const wxString, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowByName(name: *const wxString, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetCapture() -> *mut wxWindow;
        unsafe fn wxWindow_NewControlId(count: i32) -> i32;
        unsafe fn wxWindow_UnreserveControlId(id: i32, count: i32);
        unsafe fn wxWindow_new() -> *mut wxWindow;
        unsafe fn wxWindow_new1(parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> *mut wxWindow;
        unsafe fn wxWindow_Create(self_: *mut wxWindow, parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> bool;
        // CLASS: wxControl
        unsafe fn wxControl_new(parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> *mut wxControl;
        unsafe fn wxControl_new1() -> *mut wxControl;
        unsafe fn wxControl_Create(self_: *mut wxControl, parent: *mut wxWindow, id: i32, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> bool;
        unsafe fn wxControl_Command(self_: *mut wxControl, event: *mut wxCommandEvent);
        unsafe fn wxControl_GetLabel(self_: *const wxControl) -> *mut wxString;
        unsafe fn wxControl_GetLabelText(self_: *const wxControl) -> *mut wxString;
        unsafe fn wxControl_GetSizeFromTextSize(self_: *const wxControl, xlen: i32, ylen: i32) -> *mut wxSize;
        unsafe fn wxControl_GetSizeFromTextSize1(self_: *const wxControl, tsize: *const wxSize) -> *mut wxSize;
        unsafe fn wxControl_GetSizeFromText(self_: *const wxControl, text: *const wxString) -> *mut wxSize;
        unsafe fn wxControl_SetLabel(self_: *mut wxControl, label: *const wxString);
        unsafe fn wxControl_SetLabelText(self_: *mut wxControl, text: *const wxString);
        unsafe fn wxControl_SetLabelMarkup(self_: *mut wxControl, markup: *const wxString) -> bool;
        unsafe fn wxControl_GetLabelText1(label: *const wxString) -> *mut wxString;
        unsafe fn wxControl_RemoveMnemonics(str: *const wxString) -> *mut wxString;
        unsafe fn wxControl_EscapeMnemonics(text: *const wxString) -> *mut wxString;
        // CLASS: wxAnyButton
        unsafe fn wxAnyButton_new() -> *mut wxAnyButton;
        unsafe fn wxAnyButton_SetBitmapCurrent(self_: *mut wxAnyButton, bitmap: *const wxBitmap);
        unsafe fn wxAnyButton_SetBitmapDisabled(self_: *mut wxAnyButton, bitmap: *const wxBitmap);
        unsafe fn wxAnyButton_SetBitmapFocus(self_: *mut wxAnyButton, bitmap: *const wxBitmap);
        unsafe fn wxAnyButton_SetBitmapLabel(self_: *mut wxAnyButton, bitmap: *const wxBitmap);
        unsafe fn wxAnyButton_SetBitmapPressed(self_: *mut wxAnyButton, bitmap: *const wxBitmap);
        unsafe fn wxAnyButton_GetBitmapMargins(self_: *mut wxAnyButton) -> *mut wxSize;
        unsafe fn wxAnyButton_SetBitmapMargins(self_: *mut wxAnyButton, x: i32, y: i32);
        unsafe fn wxAnyButton_SetBitmapMargins1(self_: *mut wxAnyButton, sz: *const wxSize);
        // CLASS: wxButton
        unsafe fn wxButton_new() -> *mut wxButton;
        unsafe fn wxButton_new1(parent: *mut wxWindow, id: i32, label: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> *mut wxButton;
        unsafe fn wxButton_Create(self_: *mut wxButton, parent: *mut wxWindow, id: i32, label: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, validator: *const wxValidator, name: *const wxString) -> bool;
        unsafe fn wxButton_GetAuthNeeded(self_: *const wxButton) -> bool;
        unsafe fn wxButton_GetLabel(self_: *const wxButton) -> *mut wxString;
        unsafe fn wxButton_SetAuthNeeded(self_: *mut wxButton, needed: bool);
        unsafe fn wxButton_SetDefault(self_: *mut wxButton) -> *mut wxWindow;
        unsafe fn wxButton_SetLabel(self_: *mut wxButton, label: *const wxString);
        unsafe fn wxButton_GetDefaultSize(win: *mut wxWindow) -> *mut wxSize;
        // CLASS: wxNonOwnedWindow
        unsafe fn wxNonOwnedWindow_SetShape(self_: *mut wxNonOwnedWindow, region: *const wxRegion) -> bool;
        unsafe fn wxNonOwnedWindow_SetShape1(self_: *mut wxNonOwnedWindow, path: *const wxGraphicsPath) -> bool;
        // CLASS: wxTopLevelWindow
        unsafe fn wxTopLevelWindow_new() -> *mut wxTopLevelWindow;
        unsafe fn wxTopLevelWindow_new1(parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> *mut wxTopLevelWindow;
        unsafe fn wxTopLevelWindow_Create(self_: *mut wxTopLevelWindow, parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> bool;
        unsafe fn wxTopLevelWindow_CanSetTransparent(self_: *mut wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_CenterOnScreen(self_: *mut wxTopLevelWindow, direction: i32);
        unsafe fn wxTopLevelWindow_CentreOnScreen(self_: *mut wxTopLevelWindow, direction: i32);
        unsafe fn wxTopLevelWindow_EnableCloseButton(self_: *mut wxTopLevelWindow, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut wxTopLevelWindow, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut wxTopLevelWindow, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_GetDefaultItem(self_: *const wxTopLevelWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_GetTitle(self_: *const wxTopLevelWindow) -> *mut wxString;
        unsafe fn wxTopLevelWindow_Iconize(self_: *mut wxTopLevelWindow, iconize: bool);
        unsafe fn wxTopLevelWindow_IsActive(self_: *mut wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsFullScreen(self_: *const wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsIconized(self_: *const wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsMaximized(self_: *const wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_Layout(self_: *mut wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_Maximize(self_: *mut wxTopLevelWindow, maximize: bool);
        unsafe fn wxTopLevelWindow_RequestUserAttention(self_: *mut wxTopLevelWindow, flags: i32);
        unsafe fn wxTopLevelWindow_Restore(self_: *mut wxTopLevelWindow);
        unsafe fn wxTopLevelWindow_SetDefaultItem(self_: *mut wxTopLevelWindow, win: *mut wxWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut wxTopLevelWindow, win: *mut wxWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const wxTopLevelWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_SetIcon(self_: *mut wxTopLevelWindow, icon: *const wxIcon);
        unsafe fn wxTopLevelWindow_SetIcons(self_: *mut wxTopLevelWindow, icons: *const wxIconBundle);
        unsafe fn wxTopLevelWindow_SetMaxSize(self_: *mut wxTopLevelWindow, size: *const wxSize);
        unsafe fn wxTopLevelWindow_SetMinSize(self_: *mut wxTopLevelWindow, size: *const wxSize);
        unsafe fn wxTopLevelWindow_SetSizeHints(self_: *mut wxTopLevelWindow, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        unsafe fn wxTopLevelWindow_SetSizeHints1(self_: *mut wxTopLevelWindow, min_size: *const wxSize, max_size: *const wxSize, inc_size: *const wxSize);
        unsafe fn wxTopLevelWindow_SetTitle(self_: *mut wxTopLevelWindow, title: *const wxString);
        unsafe fn wxTopLevelWindow_SetTransparent(self_: *mut wxTopLevelWindow, alpha: u8) -> bool;
        unsafe fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_OSXSetModified(self_: *mut wxTopLevelWindow, modified: bool);
        unsafe fn wxTopLevelWindow_OSXIsModified(self_: *const wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut wxTopLevelWindow, filename: *const wxString);
        unsafe fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut wxTopLevelWindow);
        unsafe fn wxTopLevelWindow_EnableFullScreenView(self_: *mut wxTopLevelWindow, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_ShowFullScreen(self_: *mut wxTopLevelWindow, show: bool, style: i32) -> bool;
        unsafe fn wxTopLevelWindow_GetDefaultSize() -> *mut wxSize;
        // CLASS: wxFrame
        unsafe fn wxFrame_new() -> *mut wxFrame;
        unsafe fn wxFrame_new1(parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> *mut wxFrame;
        unsafe fn wxFrame_Centre(self_: *mut wxFrame, direction: i32);
        unsafe fn wxFrame_Create(self_: *mut wxFrame, parent: *mut wxWindow, id: i32, title: *const wxString, pos: *const wxPoint, size: *const wxSize, style: i32, name: *const wxString) -> bool;
        unsafe fn wxFrame_CreateStatusBar(self_: *mut wxFrame, number: i32, style: i32, id: i32, name: *const wxString) -> *mut wxStatusBar;
        unsafe fn wxFrame_CreateToolBar(self_: *mut wxFrame, style: i32, id: i32, name: *const wxString) -> *mut wxToolBar;
        unsafe fn wxFrame_DoGiveHelp(self_: *mut wxFrame, text: *const wxString, show: bool);
        unsafe fn wxFrame_GetClientAreaOrigin(self_: *const wxFrame) -> *mut wxPoint;
        unsafe fn wxFrame_GetMenuBar(self_: *const wxFrame) -> *mut wxMenuBar;
        unsafe fn wxFrame_GetStatusBar(self_: *const wxFrame) -> *mut wxStatusBar;
        unsafe fn wxFrame_GetStatusBarPane(self_: *const wxFrame) -> i32;
        unsafe fn wxFrame_GetToolBar(self_: *const wxFrame) -> *mut wxToolBar;
        unsafe fn wxFrame_OnCreateStatusBar(self_: *mut wxFrame, number: i32, style: i32, id: i32, name: *const wxString) -> *mut wxStatusBar;
        unsafe fn wxFrame_OnCreateToolBar(self_: *mut wxFrame, style: i32, id: i32, name: *const wxString) -> *mut wxToolBar;
        unsafe fn wxFrame_ProcessCommand(self_: *mut wxFrame, id: i32) -> bool;
        unsafe fn wxFrame_SetMenuBar(self_: *mut wxFrame, menu_bar: *mut wxMenuBar);
        unsafe fn wxFrame_SetStatusBar(self_: *mut wxFrame, status_bar: *mut wxStatusBar);
        unsafe fn wxFrame_SetStatusBarPane(self_: *mut wxFrame, n: i32);
        unsafe fn wxFrame_SetStatusText(self_: *mut wxFrame, text: *const wxString, number: i32);
        unsafe fn wxFrame_SetStatusWidths(self_: *mut wxFrame, n: i32, widths_field: *const i32);
        unsafe fn wxFrame_SetToolBar(self_: *mut wxFrame, tool_bar: *mut wxToolBar);
        unsafe fn wxFrame_PushStatusText(self_: *mut wxFrame, text: *const wxString, number: i32);
        unsafe fn wxFrame_PopStatusText(self_: *mut wxFrame, number: i32);
        // CLASS: wxPoint
        unsafe fn wxPoint_IsFullySpecified(self_: *const wxPoint) -> bool;
        unsafe fn wxPoint_SetDefaults(self_: *mut wxPoint, pt: *const wxPoint);
        unsafe fn wxPoint_new() -> *mut wxPoint;
        unsafe fn wxPoint_new1(x: i32, y: i32) -> *mut wxPoint;
        unsafe fn wxPoint_new2(pt: *const wxRealPoint) -> *mut wxPoint;
        // CLASS: wxRect
        unsafe fn wxRect_new() -> *mut wxRect;
        unsafe fn wxRect_new1(x: i32, y: i32, width: i32, height: i32) -> *mut wxRect;
        unsafe fn wxRect_new2(top_left: *const wxPoint, bottom_right: *const wxPoint) -> *mut wxRect;
        unsafe fn wxRect_new3(pos: *const wxPoint, size: *const wxSize) -> *mut wxRect;
        unsafe fn wxRect_new4(size: *const wxSize) -> *mut wxRect;
        unsafe fn wxRect_CentreIn(self_: *const wxRect, r: *const wxRect, dir: i32) -> *mut wxRect;
        unsafe fn wxRect_CenterIn(self_: *const wxRect, r: *const wxRect, dir: i32) -> *mut wxRect;
        unsafe fn wxRect_Contains(self_: *const wxRect, x: i32, y: i32) -> bool;
        unsafe fn wxRect_Contains1(self_: *const wxRect, pt: *const wxPoint) -> bool;
        unsafe fn wxRect_Contains2(self_: *const wxRect, rect: *const wxRect) -> bool;
        unsafe fn wxRect_Deflate3(self_: *const wxRect, dx: i32, dy: i32) -> *mut wxRect;
        unsafe fn wxRect_GetBottom(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetBottomLeft(self_: *const wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetBottomRight(self_: *const wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetHeight(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetLeft(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetPosition(self_: *const wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetRight(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetSize(self_: *const wxRect) -> *mut wxSize;
        unsafe fn wxRect_GetTop(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetTopLeft(self_: *const wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetTopRight(self_: *const wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetWidth(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetX(self_: *const wxRect) -> i32;
        unsafe fn wxRect_GetY(self_: *const wxRect) -> i32;
        unsafe fn wxRect_Inflate3(self_: *const wxRect, dx: i32, dy: i32) -> *mut wxRect;
        unsafe fn wxRect_Intersect1(self_: *const wxRect, rect: *const wxRect) -> *mut wxRect;
        unsafe fn wxRect_Intersects(self_: *const wxRect, rect: *const wxRect) -> bool;
        unsafe fn wxRect_IsEmpty(self_: *const wxRect) -> bool;
        unsafe fn wxRect_Offset(self_: *mut wxRect, dx: i32, dy: i32);
        unsafe fn wxRect_Offset1(self_: *mut wxRect, pt: *const wxPoint);
        unsafe fn wxRect_SetHeight(self_: *mut wxRect, height: i32);
        unsafe fn wxRect_SetPosition(self_: *mut wxRect, pos: *const wxPoint);
        unsafe fn wxRect_SetSize(self_: *mut wxRect, s: *const wxSize);
        unsafe fn wxRect_SetWidth(self_: *mut wxRect, width: i32);
        unsafe fn wxRect_SetX(self_: *mut wxRect, x: i32);
        unsafe fn wxRect_SetY(self_: *mut wxRect, y: i32);
        unsafe fn wxRect_SetLeft(self_: *mut wxRect, left: i32);
        unsafe fn wxRect_SetRight(self_: *mut wxRect, right: i32);
        unsafe fn wxRect_SetTop(self_: *mut wxRect, top: i32);
        unsafe fn wxRect_SetBottom(self_: *mut wxRect, bottom: i32);
        unsafe fn wxRect_SetTopLeft(self_: *mut wxRect, p: *const wxPoint);
        unsafe fn wxRect_SetBottomRight(self_: *mut wxRect, p: *const wxPoint);
        unsafe fn wxRect_SetTopRight(self_: *mut wxRect, p: *const wxPoint);
        unsafe fn wxRect_SetBottomLeft(self_: *mut wxRect, p: *const wxPoint);
        unsafe fn wxRect_Union(self_: *const wxRect, rect: *const wxRect) -> *mut wxRect;
        // CLASS: wxSize
        unsafe fn wxSize_new() -> *mut wxSize;
        unsafe fn wxSize_new1(width: i32, height: i32) -> *mut wxSize;
        unsafe fn wxSize_DecBy(self_: *mut wxSize, pt: *const wxPoint);
        unsafe fn wxSize_DecBy1(self_: *mut wxSize, size: *const wxSize);
        unsafe fn wxSize_DecBy2(self_: *mut wxSize, dx: i32, dy: i32);
        unsafe fn wxSize_DecBy3(self_: *mut wxSize, d: i32);
        unsafe fn wxSize_DecTo(self_: *mut wxSize, size: *const wxSize);
        unsafe fn wxSize_DecToIfSpecified(self_: *mut wxSize, size: *const wxSize);
        unsafe fn wxSize_GetHeight(self_: *const wxSize) -> i32;
        unsafe fn wxSize_GetWidth(self_: *const wxSize) -> i32;
        unsafe fn wxSize_IncBy(self_: *mut wxSize, pt: *const wxPoint);
        unsafe fn wxSize_IncBy1(self_: *mut wxSize, size: *const wxSize);
        unsafe fn wxSize_IncBy2(self_: *mut wxSize, dx: i32, dy: i32);
        unsafe fn wxSize_IncBy3(self_: *mut wxSize, d: i32);
        unsafe fn wxSize_IncTo(self_: *mut wxSize, size: *const wxSize);
        unsafe fn wxSize_IsFullySpecified(self_: *const wxSize) -> bool;
        unsafe fn wxSize_Set(self_: *mut wxSize, width: i32, height: i32);
        unsafe fn wxSize_SetDefaults(self_: *mut wxSize, size_default: *const wxSize);
        unsafe fn wxSize_SetHeight(self_: *mut wxSize, height: i32);
        unsafe fn wxSize_SetWidth(self_: *mut wxSize, width: i32);
        // CLASS: wxValidator
        unsafe fn wxValidator_new() -> *mut wxValidator;
        unsafe fn wxValidator_Clone(self_: *const wxValidator) -> *mut wxObject;
        unsafe fn wxValidator_GetWindow(self_: *const wxValidator) -> *mut wxWindow;
        unsafe fn wxValidator_SetWindow(self_: *mut wxValidator, window: *mut wxWindow);
        unsafe fn wxValidator_TransferFromWindow(self_: *mut wxValidator) -> bool;
        unsafe fn wxValidator_TransferToWindow(self_: *mut wxValidator) -> bool;
        unsafe fn wxValidator_Validate(self_: *mut wxValidator, parent: *mut wxWindow) -> bool;
        unsafe fn wxValidator_SuppressBellOnError(suppress: bool);
        unsafe fn wxValidator_IsSilent() -> bool;
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
        unsafe { Object(ffi::wxObject_new()) }
    }
    pub fn new1(other: &Object) -> Object {
        unsafe {
            let other = other.as_ptr() as *mut ffi::wxObject;
            Object(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut ffi::wxClassInfo {
        unsafe { ffi::wxObject_GetClassInfo(self.as_ptr() as *mut ffi::wxObject) }
    }
    fn get_ref_data(&self) -> *mut ffi::wxObjectRefData {
        unsafe { ffi::wxObject_GetRefData(self.as_ptr() as *mut ffi::wxObject) }
    }
    fn is_kind_of(&self, info: *const ffi::wxClassInfo) -> bool {
        unsafe { ffi::wxObject_IsKindOf(self.as_ptr() as *mut ffi::wxObject, info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        unsafe {
            let obj = obj.as_ptr() as *mut ffi::wxObject;
            ffi::wxObject_IsSameAs(self.as_ptr() as *mut ffi::wxObject, obj)
        }
    }
    fn ref_(&self, clone: &Object) {
        unsafe {
            let clone = clone.as_ptr() as *mut ffi::wxObject;
            ffi::wxObject_Ref(self.as_ptr() as *mut ffi::wxObject, clone)
        }
    }
    fn set_ref_data(&self, data: *mut ffi::wxObjectRefData) {
        unsafe { ffi::wxObject_SetRefData(self.as_ptr() as *mut ffi::wxObject, data) }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr() as *mut ffi::wxObject) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr() as *mut ffi::wxObject) }
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
        unsafe { EvtHandler(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event(&self, event: *mut ffi::wxEvent) {
        unsafe { ffi::wxEvtHandler_QueueEvent(self.as_ptr() as *mut ffi::wxEvtHandler, event) }
    }
    fn add_pending_event(&self, event: *const ffi::wxEvent) {
        unsafe { ffi::wxEvtHandler_AddPendingEvent(self.as_ptr() as *mut ffi::wxEvtHandler, event) }
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: *mut ffi::wxEvent) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEvent(self.as_ptr() as *mut ffi::wxEvtHandler, event) }
    }
    fn process_event_locally(&self, event: *mut ffi::wxEvent) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr() as *mut ffi::wxEvtHandler, event) }
    }
    fn safely_process_event(&self, event: *mut ffi::wxEvent) -> bool {
        unsafe { ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr() as *mut ffi::wxEvtHandler, event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.as_ptr() as *mut ffi::wxEvtHandler) }
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
    fn get_client_object(&self) -> *mut ffi::wxClientData {
        unsafe { ffi::wxEvtHandler_GetClientObject(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut ffi::wxClientData) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.as_ptr() as *mut ffi::wxEvtHandler, data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    fn get_next_handler(&self) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxEvtHandler_GetNextHandler(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    fn get_previous_handler(&self) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr() as *mut ffi::wxEvtHandler, enabled) }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr() as *mut ffi::wxEvtHandler, handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.as_ptr() as *mut ffi::wxEvtHandler, handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr() as *mut ffi::wxEvtHandler) }
    }
    fn add_filter(filter: *mut ffi::wxEventFilter) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    fn remove_filter(filter: *mut ffi::wxEventFilter) {
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
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> Window {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
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
        unsafe { ffi::wxWindow_AcceptsFocus(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.as_ptr() as *mut ffi::wxWindow, can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.as_ptr() as *mut ffi::wxWindow, enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn add_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.as_ptr() as *mut ffi::wxWindow, child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn find_window(&self, id: i32) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_FindWindow(self.as_ptr() as *mut ffi::wxWindow, id) }
    }
    fn find_window1(&self, name: &str) -> *mut ffi::wxWindow {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxWindow_FindWindow1(self.as_ptr() as *mut ffi::wxWindow, name)
        }
    }
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren1()
    fn remove_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.as_ptr() as *mut ffi::wxWindow, child)
        }
    }
    fn get_grand_parent(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetGrandParent(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_next_sibling(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetNextSibling(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_parent(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetParent(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_prev_sibling(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetPrevSibling(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_descendant<T: WindowMethods>(&self, win: Option<&T>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(self.as_ptr() as *mut ffi::wxWindow, win)
        }
    }
    fn reparent<T: WindowMethods>(&self, new_parent: Option<&T>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.as_ptr() as *mut ffi::wxWindow, new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.as_ptr() as *mut ffi::wxWindow, hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollPos(self.as_ptr() as *mut ffi::wxWindow, orientation) }
    }
    fn get_scroll_range(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollRange(self.as_ptr() as *mut ffi::wxWindow, orientation) }
    }
    fn get_scroll_thumb(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollThumb(self.as_ptr() as *mut ffi::wxWindow, orientation) }
    }
    fn can_scroll(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_CanScroll(self.as_ptr() as *mut ffi::wxWindow, orient) }
    }
    fn has_scrollbar(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(self.as_ptr() as *mut ffi::wxWindow, orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(self.as_ptr() as *mut ffi::wxWindow, orient) }
    }
    fn scroll_lines(&self, lines: i32) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.as_ptr() as *mut ffi::wxWindow, lines) }
    }
    fn scroll_pages(&self, pages: i32) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.as_ptr() as *mut ffi::wxWindow, pages) }
    }
    fn scroll_window(&self, dx: i32, dy: i32, rect: *const ffi::wxRect) {
        unsafe { ffi::wxWindow_ScrollWindow(self.as_ptr() as *mut ffi::wxWindow, dx, dy, rect) }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_scroll_pos(&self, orientation: i32, pos: i32, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.as_ptr() as *mut ffi::wxWindow, orientation, pos, refresh) }
    }
    fn set_scrollbar(&self, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollbar(self.as_ptr() as *mut ffi::wxWindow, orientation, position, thumb_size, range, refresh) }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn cache_best_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_CacheBestSize(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn client_to_window_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxWindow_ClientToWindowSize(self.as_ptr() as *mut ffi::wxWindow, size))
        }
    }
    fn window_to_client_size(&self, size: &Size) -> Size {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxWindow_WindowToClientSize(self.as_ptr() as *mut ffi::wxWindow, size))
        }
    }
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn from_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxWindow_FromDIP(self.as_ptr() as *mut ffi::wxWindow, sz))
        }
    }
    fn from_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            Point(ffi::wxWindow_FromDIP1(self.as_ptr() as *mut ffi::wxWindow, pt))
        }
    }
    fn from_dip2(&self, d: i32) -> i32 {
        unsafe { ffi::wxWindow_FromDIP2(self.as_ptr() as *mut ffi::wxWindow, d) }
    }
    fn to_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxWindow_ToDIP(self.as_ptr() as *mut ffi::wxWindow, sz))
        }
    }
    fn to_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            Point(ffi::wxWindow_ToDIP1(self.as_ptr() as *mut ffi::wxWindow, pt))
        }
    }
    fn to_dip2(&self, d: i32) -> i32 {
        unsafe { ffi::wxWindow_ToDIP2(self.as_ptr() as *mut ffi::wxWindow, d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_best_height(&self, width: i32) -> i32 {
        unsafe { ffi::wxWindow_GetBestHeight(self.as_ptr() as *mut ffi::wxWindow, width) }
    }
    fn get_best_width(&self, height: i32) -> i32 {
        unsafe { ffi::wxWindow_GetBestWidth(self.as_ptr() as *mut ffi::wxWindow, height) }
    }
    fn get_client_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetClientSize(self.as_ptr() as *mut ffi::wxWindow, width, height) }
    }
    fn get_client_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetClientSize1(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetEffectiveMinSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxClientSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinClientSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_min_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMinWidth(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_min_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMinHeight(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_max_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMaxWidth(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_max_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMaxHeight(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetSize(self.as_ptr() as *mut ffi::wxWindow, width, height) }
    }
    fn get_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetSize1(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetVirtualSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_virtual_size1(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetVirtualSize1(self.as_ptr() as *mut ffi::wxWindow, width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestVirtualSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_content_scale_factor(&self) -> f64 {
        unsafe { ffi::wxWindow_GetContentScaleFactor(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_dpi_scale_factor(&self) -> f64 {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetWindowBorderSize(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn inform_first_direction(&self, direction: i32, size: i32, available_other_dir: i32) -> bool {
        unsafe { ffi::wxWindow_InformFirstDirection(self.as_ptr() as *mut ffi::wxWindow, direction, size, available_other_dir) }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn send_size_event(&self, flags: i32) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.as_ptr() as *mut ffi::wxWindow, flags) }
    }
    fn send_size_event_to_parent(&self, flags: i32) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.as_ptr() as *mut ffi::wxWindow, flags) }
    }
    fn set_client_size(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetClientSize(self.as_ptr() as *mut ffi::wxWindow, width, height) }
    }
    fn set_client_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetClientSize1(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_client_size2(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            ffi::wxWindow_SetClientSize2(self.as_ptr() as *mut ffi::wxWindow, rect)
        }
    }
    fn set_containing_sizer(&self, sizer: *mut ffi::wxSizer) {
        unsafe { ffi::wxWindow_SetContainingSizer(self.as_ptr() as *mut ffi::wxWindow, sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetInitialSize(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_max_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetMaxClientSize(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetMaxSize(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_min_client_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetMinClientSize(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetMinSize(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_size(&self, x: i32, y: i32, width: i32, height: i32, size_flags: i32) {
        unsafe { ffi::wxWindow_SetSize(self.as_ptr() as *mut ffi::wxWindow, x, y, width, height, size_flags) }
    }
    fn set_size1(&self, rect: &Rect) {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            ffi::wxWindow_SetSize1(self.as_ptr() as *mut ffi::wxWindow, rect)
        }
    }
    fn set_size2(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetSize2(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn set_size3(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetSize3(self.as_ptr() as *mut ffi::wxWindow, width, height) }
    }
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr() as *mut ffi::wxSize;
            let max_size = max_size.as_ptr() as *mut ffi::wxSize;
            let inc_size = inc_size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetSizeHints(self.as_ptr() as *mut ffi::wxWindow, min_size, max_size, inc_size)
        }
    }
    fn set_size_hints1(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi::wxWindow_SetSizeHints1(self.as_ptr() as *mut ffi::wxWindow, min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_virtual_size(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.as_ptr() as *mut ffi::wxWindow, width, height) }
    }
    fn set_virtual_size1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxWindow_SetVirtualSize1(self.as_ptr() as *mut ffi::wxWindow, size)
        }
    }
    fn from_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            let w = match w {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            let w = match w {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            let w = match w {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            let w = match w {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    fn center(&self, dir: i32) {
        unsafe { ffi::wxWindow_Center(self.as_ptr() as *mut ffi::wxWindow, dir) }
    }
    fn center_on_parent(&self, dir: i32) {
        unsafe { ffi::wxWindow_CenterOnParent(self.as_ptr() as *mut ffi::wxWindow, dir) }
    }
    fn centre(&self, direction: i32) {
        unsafe { ffi::wxWindow_Centre(self.as_ptr() as *mut ffi::wxWindow, direction) }
    }
    fn centre_on_parent(&self, direction: i32) {
        unsafe { ffi::wxWindow_CentreOnParent(self.as_ptr() as *mut ffi::wxWindow, direction) }
    }
    fn get_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_GetPosition(self.as_ptr() as *mut ffi::wxWindow, x, y) }
    }
    fn get_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetPosition1(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetRect(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_screen_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_GetScreenPosition(self.as_ptr() as *mut ffi::wxWindow, x, y) }
    }
    fn get_screen_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetScreenPosition1(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetScreenRect(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetClientAreaOrigin(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetClientRect(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn move_(&self, x: i32, y: i32, flags: i32) {
        unsafe { ffi::wxWindow_Move(self.as_ptr() as *mut ffi::wxWindow, x, y, flags) }
    }
    fn move1(&self, pt: &Point, flags: i32) {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxWindow_Move1(self.as_ptr() as *mut ffi::wxWindow, pt, flags)
        }
    }
    fn set_position(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxWindow_SetPosition(self.as_ptr() as *mut ffi::wxWindow, pt)
        }
    }
    fn client_to_screen(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_ClientToScreen(self.as_ptr() as *mut ffi::wxWindow, x, y) }
    }
    fn client_to_screen1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            Point(ffi::wxWindow_ClientToScreen1(self.as_ptr() as *mut ffi::wxWindow, pt))
        }
    }
    fn convert_dialog_to_pixels(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            Point(ffi::wxWindow_ConvertDialogToPixels(self.as_ptr() as *mut ffi::wxWindow, pt))
        }
    }
    fn convert_dialog_to_pixels1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxWindow_ConvertDialogToPixels1(self.as_ptr() as *mut ffi::wxWindow, sz))
        }
    }
    fn convert_pixels_to_dialog(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            Point(ffi::wxWindow_ConvertPixelsToDialog(self.as_ptr() as *mut ffi::wxWindow, pt))
        }
    }
    fn convert_pixels_to_dialog1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxWindow_ConvertPixelsToDialog1(self.as_ptr() as *mut ffi::wxWindow, sz))
        }
    }
    fn screen_to_client(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_ScreenToClient(self.as_ptr() as *mut ffi::wxWindow, x, y) }
    }
    fn screen_to_client1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            Point(ffi::wxWindow_ScreenToClient1(self.as_ptr() as *mut ffi::wxWindow, pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(self.as_ptr() as *mut ffi::wxWindow) }
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetCharHeight(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_char_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetCharWidth(self.as_ptr() as *mut ffi::wxWindow) }
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetDPI(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    // CXX_UNSUPPORTED: fn GetFont()
    // CXX_UNSUPPORTED: fn GetForegroundColour()
    fn get_text_extent(&self, string: &str, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const ffi::wxFont) {
        unsafe {
            let string = crate::wx_string_from(string);
            ffi::wxWindow_GetTextExtent(self.as_ptr() as *mut ffi::wxWindow, string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent1(&self, string: &str) -> Size {
        unsafe {
            let string = crate::wx_string_from(string);
            Size(ffi::wxWindow_GetTextExtent1(self.as_ptr() as *mut ffi::wxWindow, string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetUpdateClientRect(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn refresh(&self, erase_background: bool, rect: *const ffi::wxRect) {
        unsafe { ffi::wxWindow_Refresh(self.as_ptr() as *mut ffi::wxWindow, erase_background, rect) }
    }
    fn refresh_rect(&self, rect: &Rect, erase_background: bool) {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            ffi::wxWindow_RefreshRect(self.as_ptr() as *mut ffi::wxWindow, rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_background_colour(&self, colour: *const ffi::wxColour) -> bool {
        unsafe { ffi::wxWindow_SetBackgroundColour(self.as_ptr() as *mut ffi::wxWindow, colour) }
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut ffi::wxString) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(self.as_ptr() as *mut ffi::wxWindow, reason) }
    }
    fn set_font(&self, font: *const ffi::wxFont) -> bool {
        unsafe { ffi::wxWindow_SetFont(self.as_ptr() as *mut ffi::wxWindow, font) }
    }
    fn set_foreground_colour(&self, colour: *const ffi::wxColour) -> bool {
        unsafe { ffi::wxWindow_SetForegroundColour(self.as_ptr() as *mut ffi::wxWindow, colour) }
    }
    fn set_own_background_colour(&self, colour: *const ffi::wxColour) {
        unsafe { ffi::wxWindow_SetOwnBackgroundColour(self.as_ptr() as *mut ffi::wxWindow, colour) }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_own_font(&self, font: *const ffi::wxFont) {
        unsafe { ffi::wxWindow_SetOwnFont(self.as_ptr() as *mut ffi::wxWindow, font) }
    }
    fn set_own_foreground_colour(&self, colour: *const ffi::wxColour) {
        unsafe { ffi::wxWindow_SetOwnForegroundColour(self.as_ptr() as *mut ffi::wxWindow, colour) }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_palette(&self, pal: *const ffi::wxPalette) {
        unsafe { ffi::wxWindow_SetPalette(self.as_ptr() as *mut ffi::wxWindow, pal) }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.as_ptr() as *mut ffi::wxWindow, enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.as_ptr() as *mut ffi::wxWindow, alpha) }
    }
    fn get_event_handler(&self) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxWindow_GetEventHandler(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn handle_as_navigation_key(&self, event: *const ffi::wxKeyEvent) -> bool {
        unsafe { ffi::wxWindow_HandleAsNavigationKey(self.as_ptr() as *mut ffi::wxWindow, event) }
    }
    fn handle_window_event(&self, event: *mut ffi::wxEvent) -> bool {
        unsafe { ffi::wxWindow_HandleWindowEvent(self.as_ptr() as *mut ffi::wxWindow, event) }
    }
    fn process_window_event(&self, event: *mut ffi::wxEvent) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEvent(self.as_ptr() as *mut ffi::wxWindow, event) }
    }
    fn process_window_event_locally(&self, event: *mut ffi::wxEvent) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEventLocally(self.as_ptr() as *mut ffi::wxWindow, event) }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxWindow_PopEventHandler(self.as_ptr() as *mut ffi::wxWindow, delete_handler) }
    }
    fn push_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.as_ptr() as *mut ffi::wxWindow, handler)
        }
    }
    fn remove_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.as_ptr() as *mut ffi::wxWindow, handler)
        }
    }
    fn set_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.as_ptr() as *mut ffi::wxWindow, handler)
        }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetNextHandler(self.as_ptr() as *mut ffi::wxWindow, handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr() as *mut ffi::wxEvtHandler as *mut ffi::wxEvtHandler,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetPreviousHandler(self.as_ptr() as *mut ffi::wxWindow, handler)
        }
    }
    fn get_extra_style(&self) -> i32 {
        unsafe { ffi::wxWindow_GetExtraStyle(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_window_style_flag(&self) -> i32 {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_window_style(&self) -> i32 {
        unsafe { ffi::wxWindow_GetWindowStyle(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn has_extra_style(&self, ex_flag: i32) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(self.as_ptr() as *mut ffi::wxWindow, ex_flag) }
    }
    fn has_flag(&self, flag: i32) -> bool {
        unsafe { ffi::wxWindow_HasFlag(self.as_ptr() as *mut ffi::wxWindow, flag) }
    }
    fn set_extra_style(&self, ex_style: i32) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.as_ptr() as *mut ffi::wxWindow, ex_style) }
    }
    fn set_window_style_flag(&self, style: i32) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.as_ptr() as *mut ffi::wxWindow, style) }
    }
    fn set_window_style(&self, style: i32) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.as_ptr() as *mut ffi::wxWindow, style) }
    }
    fn toggle_window_style(&self, flag: i32) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.as_ptr() as *mut ffi::wxWindow, flag) }
    }
    fn move_after_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.as_ptr() as *mut ffi::wxWindow, win)
        }
    }
    fn move_before_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.as_ptr() as *mut ffi::wxWindow, win)
        }
    }
    fn navigate(&self, flags: i32) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.as_ptr() as *mut ffi::wxWindow, flags) }
    }
    fn navigate_in(&self, flags: i32) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.as_ptr() as *mut ffi::wxWindow, flags) }
    }
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.as_ptr() as *mut ffi::wxWindow) }
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_exposed(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_IsExposed(self.as_ptr() as *mut ffi::wxWindow, x, y) }
    }
    fn is_exposed1(&self, pt: *mut ffi::wxPoint) -> bool {
        unsafe { ffi::wxWindow_IsExposed1(self.as_ptr() as *mut ffi::wxWindow, pt) }
    }
    fn is_exposed2(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(self.as_ptr() as *mut ffi::wxWindow, x, y, w, h) }
    }
    fn is_exposed3(&self, rect: *mut ffi::wxRect) -> bool {
        unsafe { ffi::wxWindow_IsExposed3(self.as_ptr() as *mut ffi::wxWindow, rect) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.as_ptr() as *mut ffi::wxWindow, enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.as_ptr() as *mut ffi::wxWindow, show) }
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetHelpText(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = crate::wx_string_from(help_text);
            ffi::wxWindow_SetHelpText(self.as_ptr() as *mut ffi::wxWindow, help_text)
        }
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut ffi::wxToolTip {
        unsafe { ffi::wxWindow_GetToolTip(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_tool_tip_text(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetToolTipText(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    fn set_tool_tip(&self, tip_string: &str) {
        unsafe {
            let tip_string = crate::wx_string_from(tip_string);
            ffi::wxWindow_SetToolTip(self.as_ptr() as *mut ffi::wxWindow, tip_string)
        }
    }
    fn set_tool_tip1(&self, tip: *mut ffi::wxToolTip) {
        unsafe { ffi::wxWindow_SetToolTip1(self.as_ptr() as *mut ffi::wxWindow, tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_popup_menu_selection_from_user(&self, menu: *mut ffi::wxMenu, pos: &Point) -> i32 {
        unsafe {
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.as_ptr() as *mut ffi::wxWindow, menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user1(&self, menu: *mut ffi::wxMenu, x: i32, y: i32) -> i32 {
        unsafe { ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.as_ptr() as *mut ffi::wxWindow, menu, x, y) }
    }
    fn popup_menu(&self, menu: *mut ffi::wxMenu, pos: &Point) -> bool {
        unsafe {
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            ffi::wxWindow_PopupMenu(self.as_ptr() as *mut ffi::wxWindow, menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut ffi::wxMenu, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_PopupMenu1(self.as_ptr() as *mut ffi::wxWindow, menu, x, y) }
    }
    fn get_validator(&self) -> *mut ffi::wxValidator {
        unsafe { ffi::wxWindow_GetValidator(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_validator(&self, validator: &Validator) {
        unsafe {
            let validator = validator.as_ptr() as *mut ffi::wxValidator;
            ffi::wxWindow_SetValidator(self.as_ptr() as *mut ffi::wxWindow, validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_id(&self) -> i32 {
        unsafe { ffi::wxWindow_GetId(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetLabel(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: i32, width: i32, width_total: i32) -> i32 {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(self.as_ptr() as *mut ffi::wxWindow, x, width, width_total) }
    }
    fn get_name(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetName(self.as_ptr() as *mut ffi::wxWindow)) }
    }
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: i32) {
        unsafe { ffi::wxWindow_SetId(self.as_ptr() as *mut ffi::wxWindow, winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxWindow_SetLabel(self.as_ptr() as *mut ffi::wxWindow, label)
        }
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxWindow_SetName(self.as_ptr() as *mut ffi::wxWindow, name)
        }
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut ffi::wxAcceleratorTable {
        unsafe { ffi::wxWindow_GetAcceleratorTable(self.as_ptr() as *mut ffi::wxWindow) }
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: *const ffi::wxAcceleratorTable) {
        unsafe { ffi::wxWindow_SetAcceleratorTable(self.as_ptr() as *mut ffi::wxWindow, accel) }
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.as_ptr() as *mut ffi::wxWindow, force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_drop_target(&self) -> *mut ffi::wxDropTarget {
        unsafe { ffi::wxWindow_GetDropTarget(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_drop_target(&self, target: *mut ffi::wxDropTarget) {
        unsafe { ffi::wxWindow_SetDropTarget(self.as_ptr() as *mut ffi::wxWindow, target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.as_ptr() as *mut ffi::wxWindow, accept) }
    }
    fn get_containing_sizer(&self) -> *mut ffi::wxSizer {
        unsafe { ffi::wxWindow_GetContainingSizer(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_sizer(&self) -> *mut ffi::wxSizer {
        unsafe { ffi::wxWindow_GetSizer(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_sizer(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizer(self.as_ptr() as *mut ffi::wxWindow, sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizerAndFit(self.as_ptr() as *mut ffi::wxWindow, sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut ffi::wxLayoutConstraints {
        unsafe { ffi::wxWindow_GetConstraints(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_constraints(&self, constraints: *mut ffi::wxLayoutConstraints) {
        unsafe { ffi::wxWindow_SetConstraints(self.as_ptr() as *mut ffi::wxWindow, constraints) }
    }
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.as_ptr() as *mut ffi::wxWindow, auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn get_caret(&self) -> *mut ffi::wxCaret {
        unsafe { ffi::wxWindow_GetCaret(self.as_ptr() as *mut ffi::wxWindow) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_caret(&self, caret: *mut ffi::wxCaret) {
        unsafe { ffi::wxWindow_SetCaret(self.as_ptr() as *mut ffi::wxWindow, caret) }
    }
    fn set_cursor(&self, cursor: *const ffi::wxCursor) -> bool {
        unsafe { ffi::wxWindow_SetCursor(self.as_ptr() as *mut ffi::wxWindow, cursor) }
    }
    fn warp_pointer(&self, x: i32, y: i32) {
        unsafe { ffi::wxWindow_WarpPointer(self.as_ptr() as *mut ffi::wxWindow, x, y) }
    }
    fn enable_touch_events(&self, events_mask: i32) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.as_ptr() as *mut ffi::wxWindow, events_mask) }
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest1()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: *mut ffi::wxUpdateUIEvent) {
        unsafe { ffi::wxWindow_DoUpdateWindowUI(self.as_ptr() as *mut ffi::wxWindow, event) }
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.as_ptr() as *mut ffi::wxWindow, on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.as_ptr() as *mut ffi::wxWindow) }
    }
    fn send_idle_events(&self, event: *mut ffi::wxIdleEvent) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.as_ptr() as *mut ffi::wxWindow, event) }
    }
    fn register_hot_key(&self, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool {
        unsafe { ffi::wxWindow_RegisterHotKey(self.as_ptr() as *mut ffi::wxWindow, hotkey_id, modifiers, virtual_key_code) }
    }
    fn unregister_hot_key(&self, hotkey_id: i32) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.as_ptr() as *mut ffi::wxWindow, hotkey_id) }
    }
    fn update_window_ui(&self, flags: i32) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.as_ptr() as *mut ffi::wxWindow, flags) }
    }
    // CXX_UNSUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_FindFocus() }
    }
    fn find_window_by_id<T: WindowMethods>(id: i32, parent: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowById(id, parent)
        }
    }
    fn find_window_by_label<T: WindowMethods>(label: &str, parent: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let label = crate::wx_string_from(label);
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByLabel(label, parent)
        }
    }
    fn find_window_by_name<T: WindowMethods>(name: &str, parent: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let name = crate::wx_string_from(name);
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByName(name, parent)
        }
    }
    fn get_capture() -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetCapture() }
    }
    fn new_control_id(count: i32) -> i32 {
        unsafe { ffi::wxWindow_NewControlId(count) }
    }
    fn unreserve_control_id(id: i32, count: i32) {
        unsafe { ffi::wxWindow_UnreserveControlId(id, count) }
    }
    // DTOR: fn ~wxWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let name = crate::wx_string_from(name);
            ffi::wxWindow_Create(self.as_ptr() as *mut ffi::wxWindow, parent, id, pos, size, style, name)
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
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let validator = validator.as_ptr() as *mut ffi::wxValidator;
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
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let validator = validator.as_ptr() as *mut ffi::wxValidator;
            let name = crate::wx_string_from(name);
            ffi::wxControl_Create(self.as_ptr() as *mut ffi::wxControl, parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: *mut ffi::wxCommandEvent) {
        unsafe { ffi::wxControl_Command(self.as_ptr() as *mut ffi::wxControl, event) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxControl_GetLabel(self.as_ptr() as *mut ffi::wxControl)) }
    }
    fn get_label_text(&self) -> WxString {
        unsafe { WxString(ffi::wxControl_GetLabelText(self.as_ptr() as *mut ffi::wxControl)) }
    }
    fn get_size_from_text_size(&self, xlen: i32, ylen: i32) -> Size {
        unsafe { Size(ffi::wxControl_GetSizeFromTextSize(self.as_ptr() as *mut ffi::wxControl, xlen, ylen)) }
    }
    fn get_size_from_text_size1(&self, tsize: &Size) -> Size {
        unsafe {
            let tsize = tsize.as_ptr() as *mut ffi::wxSize;
            Size(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr() as *mut ffi::wxControl, tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = crate::wx_string_from(text);
            Size(ffi::wxControl_GetSizeFromText(self.as_ptr() as *mut ffi::wxControl, text))
        }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxControl_SetLabel(self.as_ptr() as *mut ffi::wxControl, label)
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxControl_SetLabelText(self.as_ptr() as *mut ffi::wxControl, text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = crate::wx_string_from(markup);
            ffi::wxControl_SetLabelMarkup(self.as_ptr() as *mut ffi::wxControl, markup)
        }
    }
    fn get_label_text1(label: &str) -> WxString {
        unsafe {
            let label = crate::wx_string_from(label);
            WxString(ffi::wxControl_GetLabelText1(label))
        }
    }
    fn remove_mnemonics(str: &str) -> WxString {
        unsafe {
            let str = crate::wx_string_from(str);
            WxString(ffi::wxControl_RemoveMnemonics(str))
        }
    }
    fn escape_mnemonics(text: &str) -> WxString {
        unsafe {
            let text = crate::wx_string_from(text);
            WxString(ffi::wxControl_EscapeMnemonics(text))
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
        unsafe { AnyButton(ffi::wxAnyButton_new()) }
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
    fn set_bitmap_current(&self, bitmap: *const ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapCurrent(self.as_ptr() as *mut ffi::wxAnyButton, bitmap) }
    }
    fn set_bitmap_disabled(&self, bitmap: *const ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapDisabled(self.as_ptr() as *mut ffi::wxAnyButton, bitmap) }
    }
    fn set_bitmap_focus(&self, bitmap: *const ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapFocus(self.as_ptr() as *mut ffi::wxAnyButton, bitmap) }
    }
    fn set_bitmap_label(&self, bitmap: *const ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapLabel(self.as_ptr() as *mut ffi::wxAnyButton, bitmap) }
    }
    fn set_bitmap_pressed(&self, bitmap: *const ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapPressed(self.as_ptr() as *mut ffi::wxAnyButton, bitmap) }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size(ffi::wxAnyButton_GetBitmapMargins(self.as_ptr() as *mut ffi::wxAnyButton)) }
    }
    fn set_bitmap_margins(&self, x: i32, y: i32) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.as_ptr() as *mut ffi::wxAnyButton, x, y) }
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        unsafe {
            let sz = sz.as_ptr() as *mut ffi::wxSize;
            ffi::wxAnyButton_SetBitmapMargins1(self.as_ptr() as *mut ffi::wxAnyButton, sz)
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
        unsafe { Button(ffi::wxButton_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, label: &str, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> Button {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let validator = validator.as_ptr() as *mut ffi::wxValidator;
            let name = crate::wx_string_from(name);
            Button(ffi::wxButton_new1(parent, id, label, pos, size, style, validator, name))
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
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let label = crate::wx_string_from(label);
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let validator = validator.as_ptr() as *mut ffi::wxValidator;
            let name = crate::wx_string_from(name);
            ffi::wxButton_Create(self.as_ptr() as *mut ffi::wxButton, parent, id, label, pos, size, style, validator, name)
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(self.as_ptr() as *mut ffi::wxButton) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxButton_GetLabel(self.as_ptr() as *mut ffi::wxButton)) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.as_ptr() as *mut ffi::wxButton, needed) }
    }
    fn set_default(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxButton_SetDefault(self.as_ptr() as *mut ffi::wxButton) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = crate::wx_string_from(label);
            ffi::wxButton_SetLabel(self.as_ptr() as *mut ffi::wxButton, label)
        }
    }
    fn get_default_size<T: WindowMethods>(win: Option<&T>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
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
    fn set_shape(&self, region: *const ffi::wxRegion) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape(self.as_ptr() as *mut ffi::wxNonOwnedWindow, region) }
    }
    fn set_shape1(&self, path: *const ffi::wxGraphicsPath) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape1(self.as_ptr() as *mut ffi::wxNonOwnedWindow, path) }
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
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> TopLevelWindow {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
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
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let name = crate::wx_string_from(name);
            ffi::wxTopLevelWindow_Create(self.as_ptr() as *mut ffi::wxTopLevelWindow, parent, id, title, pos, size, style, name)
        }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_CanSetTransparent(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn center_on_screen(&self, direction: i32) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.as_ptr() as *mut ffi::wxTopLevelWindow, direction) }
    }
    fn centre_on_screen(&self, direction: i32) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.as_ptr() as *mut ffi::wxTopLevelWindow, direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.as_ptr() as *mut ffi::wxTopLevelWindow, enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.as_ptr() as *mut ffi::wxTopLevelWindow, enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.as_ptr() as *mut ffi::wxTopLevelWindow, enable) }
    }
    fn get_default_item(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxTopLevelWindow_GetDefaultItem(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    // CXX_UNSUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> WxString {
        unsafe { WxString(ffi::wxTopLevelWindow_GetTitle(self.as_ptr() as *mut ffi::wxTopLevelWindow)) }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.as_ptr() as *mut ffi::wxTopLevelWindow, iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn layout(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_Layout(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.as_ptr() as *mut ffi::wxTopLevelWindow, maximize) }
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: i32) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.as_ptr() as *mut ffi::wxTopLevelWindow, flags) }
    }
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetDefaultItem(self.as_ptr() as *mut ffi::wxTopLevelWindow, win)
        }
    }
    fn set_tmp_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetTmpDefaultItem(self.as_ptr() as *mut ffi::wxTopLevelWindow, win)
        }
    }
    fn get_tmp_default_item(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxTopLevelWindow_GetTmpDefaultItem(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn set_icon(&self, icon: *const ffi::wxIcon) {
        unsafe { ffi::wxTopLevelWindow_SetIcon(self.as_ptr() as *mut ffi::wxTopLevelWindow, icon) }
    }
    fn set_icons(&self, icons: *const ffi::wxIconBundle) {
        unsafe { ffi::wxTopLevelWindow_SetIcons(self.as_ptr() as *mut ffi::wxTopLevelWindow, icons) }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxTopLevelWindow_SetMaxSize(self.as_ptr() as *mut ffi::wxTopLevelWindow, size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxTopLevelWindow_SetMinSize(self.as_ptr() as *mut ffi::wxTopLevelWindow, size)
        }
    }
    fn set_size_hints(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi::wxTopLevelWindow_SetSizeHints(self.as_ptr() as *mut ffi::wxTopLevelWindow, min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_size_hints1(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = min_size.as_ptr() as *mut ffi::wxSize;
            let max_size = max_size.as_ptr() as *mut ffi::wxSize;
            let inc_size = inc_size.as_ptr() as *mut ffi::wxSize;
            ffi::wxTopLevelWindow_SetSizeHints1(self.as_ptr() as *mut ffi::wxTopLevelWindow, min_size, max_size, inc_size)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = crate::wx_string_from(title);
            ffi::wxTopLevelWindow_SetTitle(self.as_ptr() as *mut ffi::wxTopLevelWindow, title)
        }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi::wxTopLevelWindow_SetTransparent(self.as_ptr() as *mut ffi::wxTopLevelWindow, alpha) }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.as_ptr() as *mut ffi::wxTopLevelWindow, modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = crate::wx_string_from(filename);
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.as_ptr() as *mut ffi::wxTopLevelWindow, filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.as_ptr() as *mut ffi::wxTopLevelWindow) }
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.as_ptr() as *mut ffi::wxTopLevelWindow, enable) }
    }
    fn show_full_screen(&self, show: bool, style: i32) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.as_ptr() as *mut ffi::wxTopLevelWindow, show, style) }
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
    TopLevelWindowMethods,
    NonOwnedWindowMethods,
    WindowMethods,
    EvtHandlerMethods,
    ObjectMethods
}
impl Frame {
    pub fn new() -> Frame {
        unsafe { Frame(ffi::wxFrame_new()) }
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> Frame {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let name = crate::wx_string_from(name);
            Frame(ffi::wxFrame_new1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    fn centre(&self, direction: i32) {
        unsafe { ffi::wxFrame_Centre(self.as_ptr() as *mut ffi::wxFrame, direction) }
    }
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            let title = crate::wx_string_from(title);
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            let name = crate::wx_string_from(name);
            ffi::wxFrame_Create(self.as_ptr() as *mut ffi::wxFrame, parent, id, title, pos, size, style, name)
        }
    }
    fn create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut ffi::wxStatusBar {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_CreateStatusBar(self.as_ptr() as *mut ffi::wxFrame, number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut ffi::wxToolBar {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_CreateToolBar(self.as_ptr() as *mut ffi::wxFrame, style, id, name)
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_DoGiveHelp(self.as_ptr() as *mut ffi::wxFrame, text, show)
        }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxFrame_GetClientAreaOrigin(self.as_ptr() as *mut ffi::wxFrame)) }
    }
    fn get_menu_bar(&self) -> *mut ffi::wxMenuBar {
        unsafe { ffi::wxFrame_GetMenuBar(self.as_ptr() as *mut ffi::wxFrame) }
    }
    fn get_status_bar(&self) -> *mut ffi::wxStatusBar {
        unsafe { ffi::wxFrame_GetStatusBar(self.as_ptr() as *mut ffi::wxFrame) }
    }
    fn get_status_bar_pane(&self) -> i32 {
        unsafe { ffi::wxFrame_GetStatusBarPane(self.as_ptr() as *mut ffi::wxFrame) }
    }
    fn get_tool_bar(&self) -> *mut ffi::wxToolBar {
        unsafe { ffi::wxFrame_GetToolBar(self.as_ptr() as *mut ffi::wxFrame) }
    }
    fn on_create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut ffi::wxStatusBar {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_OnCreateStatusBar(self.as_ptr() as *mut ffi::wxFrame, number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut ffi::wxToolBar {
        unsafe {
            let name = crate::wx_string_from(name);
            ffi::wxFrame_OnCreateToolBar(self.as_ptr() as *mut ffi::wxFrame, style, id, name)
        }
    }
    fn process_command(&self, id: i32) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.as_ptr() as *mut ffi::wxFrame, id) }
    }
    fn set_menu_bar(&self, menu_bar: *mut ffi::wxMenuBar) {
        unsafe { ffi::wxFrame_SetMenuBar(self.as_ptr() as *mut ffi::wxFrame, menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut ffi::wxStatusBar) {
        unsafe { ffi::wxFrame_SetStatusBar(self.as_ptr() as *mut ffi::wxFrame, status_bar) }
    }
    fn set_status_bar_pane(&self, n: i32) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.as_ptr() as *mut ffi::wxFrame, n) }
    }
    fn set_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_SetStatusText(self.as_ptr() as *mut ffi::wxFrame, text, number)
        }
    }
    fn set_status_widths(&self, n: i32, widths_field: *const i32) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.as_ptr() as *mut ffi::wxFrame, n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut ffi::wxToolBar) {
        unsafe { ffi::wxFrame_SetToolBar(self.as_ptr() as *mut ffi::wxFrame, tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = crate::wx_string_from(text);
            ffi::wxFrame_PushStatusText(self.as_ptr() as *mut ffi::wxFrame, text, number)
        }
    }
    fn pop_status_text(&self, number: i32) {
        unsafe { ffi::wxFrame_PopStatusText(self.as_ptr() as *mut ffi::wxFrame, number) }
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
    pub fn new1(x: i32, y: i32) -> Point {
        unsafe { Point(ffi::wxPoint_new1(x, y)) }
    }
    pub fn new2(pt: *const ffi::wxRealPoint) -> Point {
        unsafe { Point(ffi::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(self.as_ptr() as *mut ffi::wxPoint) }
    }
    fn set_defaults(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxPoint_SetDefaults(self.as_ptr() as *mut ffi::wxPoint, pt)
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
    pub fn new1(x: i32, y: i32, width: i32, height: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_new1(x, y, width, height)) }
    }
    pub fn new2(top_left: &Point, bottom_right: &Point) -> Rect {
        unsafe {
            let top_left = top_left.as_ptr() as *mut ffi::wxPoint;
            let bottom_right = bottom_right.as_ptr() as *mut ffi::wxPoint;
            Rect(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new3(pos: &Point, size: &Size) -> Rect {
        unsafe {
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            let size = size.as_ptr() as *mut ffi::wxSize;
            Rect(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new4(size: &Size) -> Rect {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            Rect(ffi::wxRect_new4(size))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait RectMethods: WxRustMethods {
    fn centre_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = r.as_ptr() as *mut ffi::wxRect;
            Rect(ffi::wxRect_CentreIn(self.as_ptr() as *mut ffi::wxRect, r, dir))
        }
    }
    fn center_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = r.as_ptr() as *mut ffi::wxRect;
            Rect(ffi::wxRect_CenterIn(self.as_ptr() as *mut ffi::wxRect, r, dir))
        }
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::wxRect_Contains(self.as_ptr() as *mut ffi::wxRect, x, y) }
    }
    fn contains1(&self, pt: &Point) -> bool {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_Contains1(self.as_ptr() as *mut ffi::wxRect, pt)
        }
    }
    fn contains2(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            ffi::wxRect_Contains2(self.as_ptr() as *mut ffi::wxRect, rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_Deflate3(self.as_ptr() as *mut ffi::wxRect, dx, dy)) }
    }
    fn get_bottom(&self) -> i32 {
        unsafe { ffi::wxRect_GetBottom(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomLeft(self.as_ptr() as *mut ffi::wxRect)) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomRight(self.as_ptr() as *mut ffi::wxRect)) }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi::wxRect_GetHeight(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_left(&self) -> i32 {
        unsafe { ffi::wxRect_GetLeft(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetPosition(self.as_ptr() as *mut ffi::wxRect)) }
    }
    fn get_right(&self) -> i32 {
        unsafe { ffi::wxRect_GetRight(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size(ffi::wxRect_GetSize(self.as_ptr() as *mut ffi::wxRect)) }
    }
    fn get_top(&self) -> i32 {
        unsafe { ffi::wxRect_GetTop(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopLeft(self.as_ptr() as *mut ffi::wxRect)) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopRight(self.as_ptr() as *mut ffi::wxRect)) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi::wxRect_GetWidth(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_x(&self) -> i32 {
        unsafe { ffi::wxRect_GetX(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn get_y(&self) -> i32 {
        unsafe { ffi::wxRect_GetY(self.as_ptr() as *mut ffi::wxRect) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_Inflate3(self.as_ptr() as *mut ffi::wxRect, dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect1(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            Rect(ffi::wxRect_Intersect1(self.as_ptr() as *mut ffi::wxRect, rect))
        }
    }
    fn intersects(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            ffi::wxRect_Intersects(self.as_ptr() as *mut ffi::wxRect, rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(self.as_ptr() as *mut ffi::wxRect) }
    }
    fn offset(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxRect_Offset(self.as_ptr() as *mut ffi::wxRect, dx, dy) }
    }
    fn offset1(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_Offset1(self.as_ptr() as *mut ffi::wxRect, pt)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi::wxRect_SetHeight(self.as_ptr() as *mut ffi::wxRect, height) }
    }
    fn set_position(&self, pos: &Point) {
        unsafe {
            let pos = pos.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_SetPosition(self.as_ptr() as *mut ffi::wxRect, pos)
        }
    }
    fn set_size(&self, s: &Size) {
        unsafe {
            let s = s.as_ptr() as *mut ffi::wxSize;
            ffi::wxRect_SetSize(self.as_ptr() as *mut ffi::wxRect, s)
        }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi::wxRect_SetWidth(self.as_ptr() as *mut ffi::wxRect, width) }
    }
    fn set_x(&self, x: i32) {
        unsafe { ffi::wxRect_SetX(self.as_ptr() as *mut ffi::wxRect, x) }
    }
    fn set_y(&self, y: i32) {
        unsafe { ffi::wxRect_SetY(self.as_ptr() as *mut ffi::wxRect, y) }
    }
    fn set_left(&self, left: i32) {
        unsafe { ffi::wxRect_SetLeft(self.as_ptr() as *mut ffi::wxRect, left) }
    }
    fn set_right(&self, right: i32) {
        unsafe { ffi::wxRect_SetRight(self.as_ptr() as *mut ffi::wxRect, right) }
    }
    fn set_top(&self, top: i32) {
        unsafe { ffi::wxRect_SetTop(self.as_ptr() as *mut ffi::wxRect, top) }
    }
    fn set_bottom(&self, bottom: i32) {
        unsafe { ffi::wxRect_SetBottom(self.as_ptr() as *mut ffi::wxRect, bottom) }
    }
    fn set_top_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_SetTopLeft(self.as_ptr() as *mut ffi::wxRect, p)
        }
    }
    fn set_bottom_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_SetBottomRight(self.as_ptr() as *mut ffi::wxRect, p)
        }
    }
    fn set_top_right(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_SetTopRight(self.as_ptr() as *mut ffi::wxRect, p)
        }
    }
    fn set_bottom_left(&self, p: &Point) {
        unsafe {
            let p = p.as_ptr() as *mut ffi::wxPoint;
            ffi::wxRect_SetBottomLeft(self.as_ptr() as *mut ffi::wxRect, p)
        }
    }
    fn union(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = rect.as_ptr() as *mut ffi::wxRect;
            Rect(ffi::wxRect_Union(self.as_ptr() as *mut ffi::wxRect, rect))
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
    pub fn new1(width: i32, height: i32) -> Size {
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
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxSize_DecBy(self.as_ptr() as *mut ffi::wxSize, pt)
        }
    }
    fn dec_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxSize_DecBy1(self.as_ptr() as *mut ffi::wxSize, size)
        }
    }
    fn dec_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxSize_DecBy2(self.as_ptr() as *mut ffi::wxSize, dx, dy) }
    }
    fn dec_by3(&self, d: i32) {
        unsafe { ffi::wxSize_DecBy3(self.as_ptr() as *mut ffi::wxSize, d) }
    }
    fn dec_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxSize_DecTo(self.as_ptr() as *mut ffi::wxSize, size)
        }
    }
    fn dec_to_if_specified(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxSize_DecToIfSpecified(self.as_ptr() as *mut ffi::wxSize, size)
        }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi::wxSize_GetHeight(self.as_ptr() as *mut ffi::wxSize) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi::wxSize_GetWidth(self.as_ptr() as *mut ffi::wxSize) }
    }
    fn inc_by(&self, pt: &Point) {
        unsafe {
            let pt = pt.as_ptr() as *mut ffi::wxPoint;
            ffi::wxSize_IncBy(self.as_ptr() as *mut ffi::wxSize, pt)
        }
    }
    fn inc_by1(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxSize_IncBy1(self.as_ptr() as *mut ffi::wxSize, size)
        }
    }
    fn inc_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxSize_IncBy2(self.as_ptr() as *mut ffi::wxSize, dx, dy) }
    }
    fn inc_by3(&self, d: i32) {
        unsafe { ffi::wxSize_IncBy3(self.as_ptr() as *mut ffi::wxSize, d) }
    }
    fn inc_to(&self, size: &Size) {
        unsafe {
            let size = size.as_ptr() as *mut ffi::wxSize;
            ffi::wxSize_IncTo(self.as_ptr() as *mut ffi::wxSize, size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(self.as_ptr() as *mut ffi::wxSize) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: i32, height: i32) {
        unsafe { ffi::wxSize_Set(self.as_ptr() as *mut ffi::wxSize, width, height) }
    }
    fn set_defaults(&self, size_default: &Size) {
        unsafe {
            let size_default = size_default.as_ptr() as *mut ffi::wxSize;
            ffi::wxSize_SetDefaults(self.as_ptr() as *mut ffi::wxSize, size_default)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi::wxSize_SetHeight(self.as_ptr() as *mut ffi::wxSize, height) }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi::wxSize_SetWidth(self.as_ptr() as *mut ffi::wxSize, width) }
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
    fn clone(&self) -> *mut ffi::wxObject {
        unsafe { ffi::wxValidator_Clone(self.as_ptr() as *mut ffi::wxValidator) }
    }
    fn get_window(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxValidator_GetWindow(self.as_ptr() as *mut ffi::wxValidator) }
    }
    fn set_window<T: WindowMethods>(&self, window: Option<&T>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.as_ptr() as *mut ffi::wxValidator, window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.as_ptr() as *mut ffi::wxValidator) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.as_ptr() as *mut ffi::wxValidator) }
    }
    fn validate<T: WindowMethods>(&self, parent: Option<&T>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr() as *mut ffi::wxWindow as *mut ffi::wxWindow,
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.as_ptr() as *mut ffi::wxValidator, parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}

