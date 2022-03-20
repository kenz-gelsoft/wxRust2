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
        // CTOR: unsafe fn wxObject(other: &wxObject) -> Object;
        // DTOR: unsafe fn ~wxObject(self: Pin<&mut wxObject>);
        // GENERATED: unsafe fn GetClassInfo(self: &wxObject) -> *mut wxClassInfo;
        // GENERATED: unsafe fn GetRefData(self: &wxObject) -> *mut wxObjectRefData;
        // GENERATED: unsafe fn IsKindOf(self: &wxObject, info: *const wxClassInfo) -> bool;
        // GENERATED: unsafe fn IsSameAs(self: &wxObject, obj: &wxObject) -> bool;
        // GENERATED: unsafe fn Ref(self: Pin<&mut wxObject>, clone: &wxObject);
        // GENERATED: unsafe fn SetRefData(self: Pin<&mut wxObject>, data: *mut wxObjectRefData);
        // GENERATED: unsafe fn UnRef(self: Pin<&mut wxObject>);
        // GENERATED: unsafe fn UnShare(self: Pin<&mut wxObject>);
        // BLOCKED: unsafe fn operator delete(self: Pin<&mut wxObject>, buf: *mut void);
        // CXX_UNSUPPORTED: unsafe fn operator new(self: Pin<&mut wxObject>, size: size_t, filename: &wxString, line_num: i32) -> *mut void;
        
        // CLASS: wxEvtHandler
        type wxEvtHandler;
        // GENERATED: unsafe fn QueueEvent(self: Pin<&mut wxEvtHandler>, event: *mut wxEvent);
        // GENERATED: unsafe fn AddPendingEvent(self: Pin<&mut wxEvtHandler>, event: &wxEvent);
        // CXX_UNSUPPORTED: unsafe fn CallAfter(self: Pin<&mut wxEvtHandler>, method: *mut void(T::, x1: T1, None: ...);
        // BLOCKED: unsafe fn CallAfter(self: Pin<&mut wxEvtHandler>, functor: &T);
        // GENERATED: unsafe fn ProcessEvent(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: unsafe fn ProcessEventLocally(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: unsafe fn SafelyProcessEvent(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: unsafe fn ProcessPendingEvents(self: Pin<&mut wxEvtHandler>);
        // GENERATED: unsafe fn DeletePendingEvents(self: Pin<&mut wxEvtHandler>);
        // CXX_UNSUPPORTED: unsafe fn Connect(self: Pin<&mut wxEvtHandler>, id: i32, last_id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect(self: Pin<&mut wxEvtHandler>, id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect(self: Pin<&mut wxEvtHandler>, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: Pin<&mut wxEvtHandler>, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: Pin<&mut wxEvtHandler>, id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: Pin<&mut wxEvtHandler>, id: i32, last_id: i32, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut wxObject, event_sink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Bind(self: Pin<&mut wxEvtHandler>, event_type: &EventTag, functor: Functor, id: i32, last_id: i32, user_data: *mut wxObject);
        // BLOCKED: unsafe fn Bind(self: Pin<&mut wxEvtHandler>, event_type: &EventTag, method: *mut void(Class::, handler: *mut EventHandler, id: i32, last_id: i32, user_data: *mut wxObject);
        // CXX_UNSUPPORTED: unsafe fn Unbind(self: Pin<&mut wxEvtHandler>, event_type: &EventTag, functor: Functor, id: i32, last_id: i32, user_data: *mut wxObject) -> bool;
        // BLOCKED: unsafe fn Unbind(self: Pin<&mut wxEvtHandler>, event_type: &EventTag, method: *mut void(Class::, handler: *mut EventHandler, id: i32, last_id: i32, user_data: *mut wxObject) -> bool;
        // BLOCKED: unsafe fn GetClientData(self: &wxEvtHandler) -> *mut void;
        // GENERATED: unsafe fn GetClientObject(self: &wxEvtHandler) -> *mut wxClientData;
        // BLOCKED: unsafe fn SetClientData(self: Pin<&mut wxEvtHandler>, data: *mut void);
        // GENERATED: unsafe fn SetClientObject(self: Pin<&mut wxEvtHandler>, data: *mut wxClientData);
        // GENERATED: unsafe fn GetEvtHandlerEnabled(self: &wxEvtHandler) -> bool;
        // GENERATED: unsafe fn GetNextHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn GetPreviousHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn SetEvtHandlerEnabled(self: Pin<&mut wxEvtHandler>, enabled: bool);
        // GENERATED: unsafe fn SetNextHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetPreviousHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn Unlink(self: Pin<&mut wxEvtHandler>);
        // GENERATED: unsafe fn IsUnlinked(self: &wxEvtHandler) -> bool;
        // GENERATED: unsafe fn AddFilter(filter: *mut wxEventFilter);
        // GENERATED: unsafe fn RemoveFilter(filter: *mut wxEventFilter);
        // CTOR: unsafe fn wxEvtHandler() -> EvtHandler;
        // DTOR: unsafe fn ~wxEvtHandler(self: Pin<&mut wxEvtHandler>);
        
        // CLASS: wxWindow
        type wxWindow;
        // GENERATED: unsafe fn AcceptsFocus(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn AcceptsFocusFromKeyboard(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn AcceptsFocusRecursively(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn DisableFocusFromKeyboard(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn IsFocusable(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn CanAcceptFocus(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn CanAcceptFocusFromKeyboard(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn HasFocus(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn SetCanFocus(self: Pin<&mut wxWindow>, can_focus: bool);
        // GENERATED: unsafe fn EnableVisibleFocus(self: Pin<&mut wxWindow>, enable: bool);
        // GENERATED: unsafe fn SetFocus(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn SetFocusFromKbd(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn AddChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        // GENERATED: unsafe fn DestroyChildren(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn FindWindow(self: &wxWindow, id: i32) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindow(self: &wxWindow, name: &wxString) -> *mut wxWindow;
        // BLOCKED: unsafe fn GetChildren(self: Pin<&mut wxWindow>) -> Pin<&mut wxWindowList>;
        // BLOCKED: unsafe fn GetChildren(self: &wxWindow) -> &wxWindowList;
        // GENERATED: unsafe fn RemoveChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        // GENERATED: unsafe fn GetGrandParent(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetNextSibling(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetParent(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetPrevSibling(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn IsDescendant(self: &wxWindow, win: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn Reparent(self: Pin<&mut wxWindow>, new_parent: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn AlwaysShowScrollbars(self: Pin<&mut wxWindow>, hflag: bool, vflag: bool);
        // GENERATED: unsafe fn GetScrollPos(self: &wxWindow, orientation: i32) -> i32;
        // GENERATED: unsafe fn GetScrollRange(self: &wxWindow, orientation: i32) -> i32;
        // GENERATED: unsafe fn GetScrollThumb(self: &wxWindow, orientation: i32) -> i32;
        // GENERATED: unsafe fn CanScroll(self: &wxWindow, orient: i32) -> bool;
        // GENERATED: unsafe fn HasScrollbar(self: &wxWindow, orient: i32) -> bool;
        // GENERATED: unsafe fn IsScrollbarAlwaysShown(self: &wxWindow, orient: i32) -> bool;
        // GENERATED: unsafe fn ScrollLines(self: Pin<&mut wxWindow>, lines: i32) -> bool;
        // GENERATED: unsafe fn ScrollPages(self: Pin<&mut wxWindow>, pages: i32) -> bool;
        // GENERATED: unsafe fn ScrollWindow(self: Pin<&mut wxWindow>, dx: i32, dy: i32, rect: *const wxRect);
        // GENERATED: unsafe fn LineUp(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn LineDown(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn PageUp(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn PageDown(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn SetScrollPos(self: Pin<&mut wxWindow>, orientation: i32, pos: i32, refresh: bool);
        // GENERATED: unsafe fn SetScrollbar(self: Pin<&mut wxWindow>, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        // GENERATED: unsafe fn BeginRepositioningChildren(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn EndRepositioningChildren(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn CacheBestSize(self: &wxWindow, size: &wxSize);
        // GENERATED: unsafe fn ClientToWindowSize(self: &wxWindow, size: &wxSize) -> wxSize;
        // GENERATED: unsafe fn WindowToClientSize(self: &wxWindow, size: &wxSize) -> wxSize;
        // GENERATED: unsafe fn Fit(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn FitInside(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn FromDIP(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: unsafe fn FromDIP(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: unsafe fn FromDIP(self: &wxWindow, d: i32) -> i32;
        // GENERATED: unsafe fn ToDIP(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: unsafe fn ToDIP(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ToDIP(self: &wxWindow, d: i32) -> i32;
        // GENERATED: unsafe fn GetBestSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetBestHeight(self: &wxWindow, width: i32) -> i32;
        // GENERATED: unsafe fn GetBestWidth(self: &wxWindow, height: i32) -> i32;
        // GENERATED: unsafe fn GetClientSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: unsafe fn GetClientSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetEffectiveMinSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMaxClientSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMaxSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMinClientSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMinSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetMinWidth(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetMinHeight(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetMaxWidth(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetMaxHeight(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: unsafe fn GetSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetVirtualSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetVirtualSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: unsafe fn GetBestVirtualSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetContentScaleFactor(self: &wxWindow) -> f64;
        // GENERATED: unsafe fn GetDPIScaleFactor(self: &wxWindow) -> f64;
        // GENERATED: unsafe fn GetWindowBorderSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn InformFirstDirection(self: Pin<&mut wxWindow>, direction: i32, size: i32, available_other_dir: i32) -> bool;
        // GENERATED: unsafe fn InvalidateBestSize(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn PostSizeEvent(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn PostSizeEventToParent(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn SendSizeEvent(self: Pin<&mut wxWindow>, flags: i32);
        // GENERATED: unsafe fn SendSizeEventToParent(self: Pin<&mut wxWindow>, flags: i32);
        // GENERATED: unsafe fn SetClientSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        // GENERATED: unsafe fn SetClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetClientSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        // GENERATED: unsafe fn SetContainingSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer);
        // GENERATED: unsafe fn SetInitialSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetMaxClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetMaxSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetMinClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetMinSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetSize(self: Pin<&mut wxWindow>, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        // GENERATED: unsafe fn SetSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        // GENERATED: unsafe fn SetSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        // GENERATED: unsafe fn SetSizeHints(self: Pin<&mut wxWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        // GENERATED: unsafe fn SetSizeHints(self: Pin<&mut wxWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: unsafe fn SetVirtualSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        // GENERATED: unsafe fn SetVirtualSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn FromDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn FromDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn FromDIP(d: i32, w: *const wxWindow) -> i32;
        // GENERATED: unsafe fn ToDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn ToDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn ToDIP(d: i32, w: *const wxWindow) -> i32;
        // GENERATED: unsafe fn Center(self: Pin<&mut wxWindow>, dir: i32);
        // GENERATED: unsafe fn CenterOnParent(self: Pin<&mut wxWindow>, dir: i32);
        // GENERATED: unsafe fn Centre(self: Pin<&mut wxWindow>, direction: i32);
        // GENERATED: unsafe fn CentreOnParent(self: Pin<&mut wxWindow>, direction: i32);
        // GENERATED: unsafe fn GetPosition(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn GetPosition(self: &wxWindow) -> wxPoint;
        // GENERATED: unsafe fn GetRect(self: &wxWindow) -> wxRect;
        // GENERATED: unsafe fn GetScreenPosition(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn GetScreenPosition(self: &wxWindow) -> wxPoint;
        // GENERATED: unsafe fn GetScreenRect(self: &wxWindow) -> wxRect;
        // GENERATED: unsafe fn GetClientAreaOrigin(self: &wxWindow) -> wxPoint;
        // GENERATED: unsafe fn GetClientRect(self: &wxWindow) -> wxRect;
        // GENERATED: unsafe fn Move(self: Pin<&mut wxWindow>, x: i32, y: i32, flags: i32);
        // GENERATED: unsafe fn Move(self: Pin<&mut wxWindow>, pt: &wxPoint, flags: i32);
        // GENERATED: unsafe fn SetPosition(self: Pin<&mut wxWindow>, pt: &wxPoint);
        // GENERATED: unsafe fn ClientToScreen(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn ClientToScreen(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ConvertDialogToPixels(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ConvertDialogToPixels(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: unsafe fn ConvertPixelsToDialog(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ConvertPixelsToDialog(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: unsafe fn ScreenToClient(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: unsafe fn ScreenToClient(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: unsafe fn ClearBackground(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn Freeze(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn Thaw(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn IsFrozen(self: &wxWindow) -> bool;
        // CXX_UNSUPPORTED: unsafe fn GetBackgroundColour(self: &wxWindow) -> wxColour;
        // CXX_UNSUPPORTED: unsafe fn GetBackgroundStyle(self: &wxWindow) -> wxBackgroundStyle;
        // GENERATED: unsafe fn GetCharHeight(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetCharWidth(self: &wxWindow) -> i32;
        // CXX_UNSUPPORTED: unsafe fn GetDefaultAttributes(self: &wxWindow) -> wxVisualAttributes;
        // GENERATED: unsafe fn GetDPI(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: unsafe fn GetFont(self: &wxWindow) -> wxFont;
        // CXX_UNSUPPORTED: unsafe fn GetForegroundColour(self: &wxWindow) -> wxColour;
        // GENERATED: unsafe fn GetTextExtent(self: &wxWindow, string: &wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        // GENERATED: unsafe fn GetTextExtent(self: &wxWindow, string: &wxString) -> wxSize;
        // BLOCKED: unsafe fn GetUpdateRegion(self: &wxWindow) -> &wxRegion;
        // GENERATED: unsafe fn GetUpdateClientRect(self: &wxWindow) -> wxRect;
        // GENERATED: unsafe fn HasTransparentBackground(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn Refresh(self: Pin<&mut wxWindow>, erase_background: bool, rect: *const wxRect);
        // GENERATED: unsafe fn RefreshRect(self: Pin<&mut wxWindow>, rect: &wxRect, erase_background: bool);
        // GENERATED: unsafe fn Update(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn SetBackgroundColour(self: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        // CXX_UNSUPPORTED: unsafe fn SetBackgroundStyle(self: Pin<&mut wxWindow>, style: wxBackgroundStyle) -> bool;
        // GENERATED: unsafe fn IsTransparentBackgroundSupported(self: &wxWindow, reason: *mut wxString) -> bool;
        // GENERATED: unsafe fn SetFont(self: Pin<&mut wxWindow>, font: &wxFont) -> bool;
        // GENERATED: unsafe fn SetForegroundColour(self: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        // GENERATED: unsafe fn SetOwnBackgroundColour(self: Pin<&mut wxWindow>, colour: &wxColour);
        // GENERATED: unsafe fn InheritsBackgroundColour(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn UseBgCol(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn UseBackgroundColour(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn SetOwnFont(self: Pin<&mut wxWindow>, font: &wxFont);
        // GENERATED: unsafe fn SetOwnForegroundColour(self: Pin<&mut wxWindow>, colour: &wxColour);
        // GENERATED: unsafe fn UseForegroundColour(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn InheritsForegroundColour(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn SetPalette(self: Pin<&mut wxWindow>, pal: &wxPalette);
        // GENERATED: unsafe fn ShouldInheritColours(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn SetThemeEnabled(self: Pin<&mut wxWindow>, enable: bool);
        // GENERATED: unsafe fn GetThemeEnabled(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn CanSetTransparent(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn SetTransparent(self: Pin<&mut wxWindow>, alpha: u8) -> bool;
        // GENERATED: unsafe fn GetEventHandler(self: &wxWindow) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn HandleAsNavigationKey(self: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        // GENERATED: unsafe fn HandleWindowEvent(self: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: unsafe fn ProcessWindowEvent(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: unsafe fn ProcessWindowEventLocally(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: unsafe fn PopEventHandler(self: Pin<&mut wxWindow>, delete_handler: bool) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn PushEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn RemoveEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler) -> bool;
        // GENERATED: unsafe fn SetEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetNextHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetPreviousHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn GetExtraStyle(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetWindowStyleFlag(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetWindowStyle(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn HasExtraStyle(self: &wxWindow, ex_flag: i32) -> bool;
        // GENERATED: unsafe fn HasFlag(self: &wxWindow, flag: i32) -> bool;
        // GENERATED: unsafe fn SetExtraStyle(self: Pin<&mut wxWindow>, ex_style: i32);
        // GENERATED: unsafe fn SetWindowStyleFlag(self: Pin<&mut wxWindow>, style: i32);
        // GENERATED: unsafe fn SetWindowStyle(self: Pin<&mut wxWindow>, style: i32);
        // GENERATED: unsafe fn ToggleWindowStyle(self: Pin<&mut wxWindow>, flag: i32) -> bool;
        // GENERATED: unsafe fn MoveAfterInTabOrder(self: Pin<&mut wxWindow>, win: *mut wxWindow);
        // GENERATED: unsafe fn MoveBeforeInTabOrder(self: Pin<&mut wxWindow>, win: *mut wxWindow);
        // GENERATED: unsafe fn Navigate(self: Pin<&mut wxWindow>, flags: i32) -> bool;
        // GENERATED: unsafe fn NavigateIn(self: Pin<&mut wxWindow>, flags: i32) -> bool;
        // GENERATED: unsafe fn Lower(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn Raise(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn Hide(self: Pin<&mut wxWindow>) -> bool;
        // CXX_UNSUPPORTED: unsafe fn HideWithEffect(self: Pin<&mut wxWindow>, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: unsafe fn IsEnabled(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn IsExposed(self: &wxWindow, x: i32, y: i32) -> bool;
        // GENERATED: unsafe fn IsExposed(self: &wxWindow, pt: Pin<&mut wxPoint>) -> bool;
        // GENERATED: unsafe fn IsExposed(self: &wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        // GENERATED: unsafe fn IsExposed(self: &wxWindow, rect: Pin<&mut wxRect>) -> bool;
        // GENERATED: unsafe fn IsShown(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn IsShownOnScreen(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn Disable(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn Enable(self: Pin<&mut wxWindow>, enable: bool) -> bool;
        // GENERATED: unsafe fn Show(self: Pin<&mut wxWindow>, show: bool) -> bool;
        // CXX_UNSUPPORTED: unsafe fn ShowWithEffect(self: Pin<&mut wxWindow>, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: unsafe fn GetHelpText(self: &wxWindow) -> wxString;
        // GENERATED: unsafe fn SetHelpText(self: Pin<&mut wxWindow>, help_text: &wxString);
        // CXX_UNSUPPORTED: unsafe fn GetHelpTextAtPoint(self: &wxWindow, point: &wxPoint, origin: wxHelpEvent::Origin) -> wxString;
        // GENERATED: unsafe fn GetToolTip(self: &wxWindow) -> *mut wxToolTip;
        // GENERATED: unsafe fn GetToolTipText(self: &wxWindow) -> wxString;
        // GENERATED: unsafe fn SetToolTip(self: Pin<&mut wxWindow>, tip_string: &wxString);
        // GENERATED: unsafe fn SetToolTip(self: Pin<&mut wxWindow>, tip: *mut wxToolTip);
        // GENERATED: unsafe fn UnsetToolTip(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, pos: &wxPoint) -> i32;
        // GENERATED: unsafe fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, x: i32, y: i32) -> i32;
        // GENERATED: unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, pos: &wxPoint) -> bool;
        // GENERATED: unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        // GENERATED: unsafe fn GetValidator(self: Pin<&mut wxWindow>) -> *mut wxValidator;
        // GENERATED: unsafe fn SetValidator(self: Pin<&mut wxWindow>, validator: &wxValidator);
        // GENERATED: unsafe fn TransferDataFromWindow(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn TransferDataToWindow(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn Validate(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn GetId(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetLabel(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: unsafe fn GetLayoutDirection(self: &wxWindow) -> wxLayoutDirection;
        // GENERATED: unsafe fn AdjustForLayoutDirection(self: &wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        // GENERATED: unsafe fn GetName(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: unsafe fn GetWindowVariant(self: &wxWindow) -> wxWindowVariant;
        // GENERATED: unsafe fn SetId(self: Pin<&mut wxWindow>, winid: i32);
        // GENERATED: unsafe fn SetLabel(self: Pin<&mut wxWindow>, label: &wxString);
        // CXX_UNSUPPORTED: unsafe fn SetLayoutDirection(self: Pin<&mut wxWindow>, dir: wxLayoutDirection);
        // GENERATED: unsafe fn SetName(self: Pin<&mut wxWindow>, name: &wxString);
        // CXX_UNSUPPORTED: unsafe fn SetWindowVariant(self: Pin<&mut wxWindow>, variant: wxWindowVariant);
        // GENERATED: unsafe fn GetAcceleratorTable(self: Pin<&mut wxWindow>) -> *mut wxAcceleratorTable;
        // CXX_UNSUPPORTED: unsafe fn GetAccessible(self: Pin<&mut wxWindow>) -> *mut wxAccessible;
        // GENERATED: unsafe fn SetAcceleratorTable(self: Pin<&mut wxWindow>, accel: &wxAcceleratorTable);
        // CXX_UNSUPPORTED: unsafe fn SetAccessible(self: Pin<&mut wxWindow>, accessible: *mut wxAccessible);
        // GENERATED: unsafe fn Close(self: Pin<&mut wxWindow>, force: bool) -> bool;
        // GENERATED: unsafe fn Destroy(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn IsBeingDeleted(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn GetDropTarget(self: &wxWindow) -> *mut wxDropTarget;
        // GENERATED: unsafe fn SetDropTarget(self: Pin<&mut wxWindow>, target: *mut wxDropTarget);
        // GENERATED: unsafe fn DragAcceptFiles(self: Pin<&mut wxWindow>, accept: bool);
        // GENERATED: unsafe fn GetContainingSizer(self: &wxWindow) -> *mut wxSizer;
        // GENERATED: unsafe fn GetSizer(self: &wxWindow) -> *mut wxSizer;
        // GENERATED: unsafe fn SetSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        // GENERATED: unsafe fn SetSizerAndFit(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        // GENERATED: unsafe fn GetConstraints(self: &wxWindow) -> *mut wxLayoutConstraints;
        // GENERATED: unsafe fn SetConstraints(self: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        // GENERATED: unsafe fn Layout(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn SetAutoLayout(self: Pin<&mut wxWindow>, auto_layout: bool);
        // GENERATED: unsafe fn GetAutoLayout(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn CaptureMouse(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn GetCaret(self: &wxWindow) -> *mut wxCaret;
        // BLOCKED: unsafe fn GetCursor(self: &wxWindow) -> &wxCursor;
        // GENERATED: unsafe fn HasCapture(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn ReleaseMouse(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn SetCaret(self: Pin<&mut wxWindow>, caret: *mut wxCaret);
        // GENERATED: unsafe fn SetCursor(self: Pin<&mut wxWindow>, cursor: &wxCursor) -> bool;
        // GENERATED: unsafe fn WarpPointer(self: Pin<&mut wxWindow>, x: i32, y: i32);
        // GENERATED: unsafe fn EnableTouchEvents(self: Pin<&mut wxWindow>, events_mask: i32) -> bool;
        // CXX_UNSUPPORTED: unsafe fn HitTest(self: &wxWindow, x: i32, y: i32) -> wxHitTest;
        // CXX_UNSUPPORTED: unsafe fn HitTest(self: &wxWindow, pt: &wxPoint) -> wxHitTest;
        // CXX_UNSUPPORTED: unsafe fn GetBorder(self: &wxWindow, flags: i32) -> wxBorder;
        // CXX_UNSUPPORTED: unsafe fn GetBorder(self: &wxWindow) -> wxBorder;
        // GENERATED: unsafe fn DoUpdateWindowUI(self: Pin<&mut wxWindow>, event: Pin<&mut wxUpdateUIEvent>);
        // CXX_UNSUPPORTED: unsafe fn GetHandle(self: &wxWindow) -> WXWidget;
        // GENERATED: unsafe fn HasMultiplePages(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn InheritAttributes(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn InitDialog(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn IsDoubleBuffered(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn SetDoubleBuffered(self: Pin<&mut wxWindow>, on: bool);
        // GENERATED: unsafe fn IsRetained(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn IsThisEnabled(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn IsTopLevel(self: &wxWindow) -> bool;
        // GENERATED: unsafe fn OnInternalIdle(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn SendIdleEvents(self: Pin<&mut wxWindow>, event: Pin<&mut wxIdleEvent>) -> bool;
        // GENERATED: unsafe fn RegisterHotKey(self: Pin<&mut wxWindow>, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        // GENERATED: unsafe fn UnregisterHotKey(self: Pin<&mut wxWindow>, hotkey_id: i32) -> bool;
        // GENERATED: unsafe fn UpdateWindowUI(self: Pin<&mut wxWindow>, flags: i32);
        // CXX_UNSUPPORTED: unsafe fn GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        // GENERATED: unsafe fn FindFocus() -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetCapture() -> *mut wxWindow;
        // GENERATED: unsafe fn NewControlId(count: i32) -> i32;
        // GENERATED: unsafe fn UnreserveControlId(id: i32, count: i32);
        // CTOR: unsafe fn wxWindow() -> Window;
        // CTOR: unsafe fn wxWindow(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Window;
        // DTOR: unsafe fn ~wxWindow(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn Create(self: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        
        // CLASS: wxControl
        type wxControl;
        // CTOR: unsafe fn wxControl(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Control;
        // CTOR: unsafe fn wxControl() -> Control;
        // GENERATED: unsafe fn Create(self: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        // GENERATED: unsafe fn Command(self: Pin<&mut wxControl>, event: Pin<&mut wxCommandEvent>);
        // GENERATED: unsafe fn GetLabel(self: &wxControl) -> wxString;
        // GENERATED: unsafe fn GetLabelText(self: &wxControl) -> wxString;
        // GENERATED: unsafe fn GetSizeFromTextSize(self: &wxControl, xlen: i32, ylen: i32) -> wxSize;
        // GENERATED: unsafe fn GetSizeFromTextSize(self: &wxControl, tsize: &wxSize) -> wxSize;
        // GENERATED: unsafe fn GetSizeFromText(self: &wxControl, text: &wxString) -> wxSize;
        // GENERATED: unsafe fn SetLabel(self: Pin<&mut wxControl>, label: &wxString);
        // GENERATED: unsafe fn SetLabelText(self: Pin<&mut wxControl>, text: &wxString);
        // GENERATED: unsafe fn SetLabelMarkup(self: Pin<&mut wxControl>, markup: &wxString) -> bool;
        // GENERATED: unsafe fn GetLabelText(label: &wxString) -> wxString;
        // GENERATED: unsafe fn RemoveMnemonics(str: &wxString) -> wxString;
        // GENERATED: unsafe fn EscapeMnemonics(text: &wxString) -> wxString;
        // BLOCKED: unsafe fn Ellipsize(label: &wxString, dc: &wxDC, mode: i32, max_width: i32, flags: i32) -> wxString;
        
        // CLASS: wxAnyButton
        type wxAnyButton;
        // CTOR: unsafe fn wxAnyButton() -> AnyButton;
        // DTOR: unsafe fn ~wxAnyButton(self: Pin<&mut wxAnyButton>);
        // CXX_UNSUPPORTED: unsafe fn GetBitmap(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapCurrent(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapDisabled(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapFocus(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapLabel(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn GetBitmapPressed(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: unsafe fn SetBitmap(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap, dir: wxDirection);
        // GENERATED: unsafe fn SetBitmapCurrent(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: unsafe fn SetBitmapDisabled(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: unsafe fn SetBitmapFocus(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: unsafe fn SetBitmapLabel(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: unsafe fn SetBitmapPressed(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: unsafe fn GetBitmapMargins(self: Pin<&mut wxAnyButton>) -> wxSize;
        // GENERATED: unsafe fn SetBitmapMargins(self: Pin<&mut wxAnyButton>, x: i32, y: i32);
        // GENERATED: unsafe fn SetBitmapMargins(self: Pin<&mut wxAnyButton>, sz: &wxSize);
        // CXX_UNSUPPORTED: unsafe fn SetBitmapPosition(self: Pin<&mut wxAnyButton>, dir: wxDirection);
        
        // CLASS: wxButton
        type wxButton;
        // CTOR: unsafe fn wxButton() -> Button;
        // CTOR: unsafe fn wxButton(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Button;
        // GENERATED: unsafe fn Create(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        // GENERATED: unsafe fn GetAuthNeeded(self: &wxButton) -> bool;
        // GENERATED: unsafe fn GetLabel(self: &wxButton) -> wxString;
        // GENERATED: unsafe fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
        // GENERATED: unsafe fn SetDefault(self: Pin<&mut wxButton>) -> *mut wxWindow;
        // GENERATED: unsafe fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
        // GENERATED: unsafe fn GetDefaultSize(win: *mut wxWindow) -> wxSize;
        
        // CLASS: wxNonOwnedWindow
        type wxNonOwnedWindow;
        // GENERATED: unsafe fn SetShape(self: Pin<&mut wxNonOwnedWindow>, region: &wxRegion) -> bool;
        // GENERATED: unsafe fn SetShape(self: Pin<&mut wxNonOwnedWindow>, path: &wxGraphicsPath) -> bool;
        
        // CLASS: wxTopLevelWindow
        type wxTopLevelWindow;
        // CTOR: unsafe fn wxTopLevelWindow() -> TopLevelWindow;
        // CTOR: unsafe fn wxTopLevelWindow(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> TopLevelWindow;
        // DTOR: unsafe fn ~wxTopLevelWindow(self: Pin<&mut wxTopLevelWindow>);
        // GENERATED: unsafe fn Create(self: Pin<&mut wxTopLevelWindow>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // GENERATED: unsafe fn CanSetTransparent(self: Pin<&mut wxTopLevelWindow>) -> bool;
        // GENERATED: unsafe fn CenterOnScreen(self: Pin<&mut wxTopLevelWindow>, direction: i32);
        // GENERATED: unsafe fn CentreOnScreen(self: Pin<&mut wxTopLevelWindow>, direction: i32);
        // GENERATED: unsafe fn EnableCloseButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: unsafe fn EnableMaximizeButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: unsafe fn EnableMinimizeButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: unsafe fn GetDefaultItem(self: &wxTopLevelWindow) -> *mut wxWindow;
        // CXX_UNSUPPORTED: unsafe fn GetIcon(self: &wxTopLevelWindow) -> wxIcon;
        // BLOCKED: unsafe fn GetIcons(self: &wxTopLevelWindow) -> &wxIconBundle;
        // GENERATED: unsafe fn GetTitle(self: &wxTopLevelWindow) -> wxString;
        // GENERATED: unsafe fn Iconize(self: Pin<&mut wxTopLevelWindow>, iconize: bool);
        // GENERATED: unsafe fn IsActive(self: Pin<&mut wxTopLevelWindow>) -> bool;
        // GENERATED: unsafe fn IsAlwaysMaximized(self: &wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsFullScreen(self: &wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsIconized(self: &wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn IsMaximized(self: &wxTopLevelWindow) -> bool;
        // BLOCKED: unsafe fn IsUsingNativeDecorations(self: &wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn Layout(self: Pin<&mut wxTopLevelWindow>) -> bool;
        // GENERATED: unsafe fn Maximize(self: Pin<&mut wxTopLevelWindow>, maximize: bool);
        // BLOCKED: unsafe fn MSWGetSystemMenu(self: &wxTopLevelWindow) -> *mut wxMenu;
        // GENERATED: unsafe fn RequestUserAttention(self: Pin<&mut wxTopLevelWindow>, flags: i32);
        // GENERATED: unsafe fn Restore(self: Pin<&mut wxTopLevelWindow>);
        // BLOCKED: unsafe fn RestoreToGeometry(self: Pin<&mut wxTopLevelWindow>, ser: Pin<&mut GeometrySerializer>) -> bool;
        // BLOCKED: unsafe fn SaveGeometry(self: &wxTopLevelWindow, ser: &GeometrySerializer) -> bool;
        // GENERATED: unsafe fn SetDefaultItem(self: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn SetTmpDefaultItem(self: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn GetTmpDefaultItem(self: &wxTopLevelWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn SetIcon(self: Pin<&mut wxTopLevelWindow>, icon: &wxIcon);
        // GENERATED: unsafe fn SetIcons(self: Pin<&mut wxTopLevelWindow>, icons: &wxIconBundle);
        // GENERATED: unsafe fn SetMaxSize(self: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetMinSize(self: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        // GENERATED: unsafe fn SetSizeHints(self: Pin<&mut wxTopLevelWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: unsafe fn SetSizeHints(self: Pin<&mut wxTopLevelWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        // GENERATED: unsafe fn SetTitle(self: Pin<&mut wxTopLevelWindow>, title: &wxString);
        // GENERATED: unsafe fn SetTransparent(self: Pin<&mut wxTopLevelWindow>, alpha: u8) -> bool;
        // GENERATED: unsafe fn ShouldPreventAppExit(self: &wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn OSXSetModified(self: Pin<&mut wxTopLevelWindow>, modified: bool);
        // GENERATED: unsafe fn OSXIsModified(self: &wxTopLevelWindow) -> bool;
        // GENERATED: unsafe fn SetRepresentedFilename(self: Pin<&mut wxTopLevelWindow>, filename: &wxString);
        // GENERATED: unsafe fn ShowWithoutActivating(self: Pin<&mut wxTopLevelWindow>);
        // GENERATED: unsafe fn EnableFullScreenView(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: unsafe fn ShowFullScreen(self: Pin<&mut wxTopLevelWindow>, show: bool, style: i32) -> bool;
        // BLOCKED: unsafe fn UseNativeDecorations(self: Pin<&mut wxTopLevelWindow>, native: bool);
        // BLOCKED: unsafe fn UseNativeDecorationsByDefault(self: Pin<&mut wxTopLevelWindow>, native: bool);
        // GENERATED: unsafe fn GetDefaultSize() -> wxSize;
        
        // CLASS: wxFrame
        type wxFrame;
        // CTOR: unsafe fn wxFrame() -> Frame;
        // CTOR: unsafe fn wxFrame(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Frame;
        // DTOR: unsafe fn ~wxFrame(self: Pin<&mut wxFrame>);
        // GENERATED: unsafe fn Centre(self: Pin<&mut wxFrame>, direction: i32);
        // GENERATED: unsafe fn Create(self: Pin<&mut wxFrame>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // GENERATED: unsafe fn CreateStatusBar(self: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        // GENERATED: unsafe fn CreateToolBar(self: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        // GENERATED: unsafe fn DoGiveHelp(self: Pin<&mut wxFrame>, text: &wxString, show: bool);
        // GENERATED: unsafe fn GetClientAreaOrigin(self: &wxFrame) -> wxPoint;
        // GENERATED: unsafe fn GetMenuBar(self: &wxFrame) -> *mut wxMenuBar;
        // GENERATED: unsafe fn GetStatusBar(self: &wxFrame) -> *mut wxStatusBar;
        // GENERATED: unsafe fn GetStatusBarPane(self: &wxFrame) -> i32;
        // GENERATED: unsafe fn GetToolBar(self: &wxFrame) -> *mut wxToolBar;
        // GENERATED: unsafe fn OnCreateStatusBar(self: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        // GENERATED: unsafe fn OnCreateToolBar(self: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        // GENERATED: unsafe fn ProcessCommand(self: Pin<&mut wxFrame>, id: i32) -> bool;
        // GENERATED: unsafe fn SetMenuBar(self: Pin<&mut wxFrame>, menu_bar: *mut wxMenuBar);
        // GENERATED: unsafe fn SetStatusBar(self: Pin<&mut wxFrame>, status_bar: *mut wxStatusBar);
        // GENERATED: unsafe fn SetStatusBarPane(self: Pin<&mut wxFrame>, n: i32);
        // GENERATED: unsafe fn SetStatusText(self: Pin<&mut wxFrame>, text: &wxString, number: i32);
        // GENERATED: unsafe fn SetStatusWidths(self: Pin<&mut wxFrame>, n: i32, widths_field: *const i32);
        // GENERATED: unsafe fn SetToolBar(self: Pin<&mut wxFrame>, tool_bar: *mut wxToolBar);
        // BLOCKED: unsafe fn MSWGetTaskBarButton(self: Pin<&mut wxFrame>) -> *mut wxTaskBarButton;
        // GENERATED: unsafe fn PushStatusText(self: Pin<&mut wxFrame>, text: &wxString, number: i32);
        // GENERATED: unsafe fn PopStatusText(self: Pin<&mut wxFrame>, number: i32);
        
        // CLASS: wxPoint
        type wxPoint;
        // GENERATED: unsafe fn IsFullySpecified(self: &wxPoint) -> bool;
        // GENERATED: unsafe fn SetDefaults(self: Pin<&mut wxPoint>, pt: &wxPoint);
        // BLOCKED: unsafe fn operator=(self: Pin<&mut wxPoint>, pt: &wxPoint) -> Pin<&mut wxPoint>;
        // BLOCKED: unsafe fn operator==(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> bool;
        // BLOCKED: unsafe fn operator!=(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> bool;
        // BLOCKED: unsafe fn operator+(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator-(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator+=(self: Pin<&mut wxPoint>, pt: &wxPoint) -> Pin<&mut wxPoint>;
        // BLOCKED: unsafe fn operator-=(self: Pin<&mut wxPoint>, pt: &wxPoint) -> Pin<&mut wxPoint>;
        // BLOCKED: unsafe fn operator+(self: Pin<&mut wxPoint>, pt: &wxPoint, sz: &wxSize) -> wxPoint;
        // BLOCKED: unsafe fn operator-(self: Pin<&mut wxPoint>, pt: &wxPoint, sz: &wxSize) -> wxPoint;
        // BLOCKED: unsafe fn operator+(self: Pin<&mut wxPoint>, sz: &wxSize, pt: &wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator-(self: Pin<&mut wxPoint>, sz: &wxSize, pt: &wxPoint) -> wxPoint;
        // BLOCKED: unsafe fn operator+=(self: Pin<&mut wxPoint>, sz: &wxSize) -> Pin<&mut wxPoint>;
        // BLOCKED: unsafe fn operator-=(self: Pin<&mut wxPoint>, sz: &wxSize) -> Pin<&mut wxPoint>;
        // BLOCKED: unsafe fn operator/(self: Pin<&mut wxPoint>, sz: &wxPoint, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*(self: Pin<&mut wxPoint>, sz: &wxPoint, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*(self: Pin<&mut wxPoint>, factor: i32, sz: &wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator/=(self: Pin<&mut wxPoint>, factor: i32) -> Pin<&mut wxSize>;
        // BLOCKED: unsafe fn operator*=(self: Pin<&mut wxPoint>, factor: i32) -> Pin<&mut wxSize>;
        // CTOR: unsafe fn wxPoint() -> Point;
        // CTOR: unsafe fn wxPoint(x: i32, y: i32) -> Point;
        // CTOR: unsafe fn wxPoint(pt: &wxRealPoint) -> Point;
        
        // CLASS: wxRect
        type wxRect;
        // CTOR: unsafe fn wxRect() -> Rect;
        // CTOR: unsafe fn wxRect(x: i32, y: i32, width: i32, height: i32) -> Rect;
        // CTOR: unsafe fn wxRect(top_left: &wxPoint, bottom_right: &wxPoint) -> Rect;
        // CTOR: unsafe fn wxRect(pos: &wxPoint, size: &wxSize) -> Rect;
        // CTOR: unsafe fn wxRect(size: &wxSize) -> Rect;
        // GENERATED: unsafe fn CentreIn(self: &wxRect, r: &wxRect, dir: i32) -> wxRect;
        // GENERATED: unsafe fn CenterIn(self: &wxRect, r: &wxRect, dir: i32) -> wxRect;
        // GENERATED: unsafe fn Contains(self: &wxRect, x: i32, y: i32) -> bool;
        // GENERATED: unsafe fn Contains(self: &wxRect, pt: &wxPoint) -> bool;
        // GENERATED: unsafe fn Contains(self: &wxRect, rect: &wxRect) -> bool;
        // BLOCKED: unsafe fn Deflate(self: Pin<&mut wxRect>, dx: i32, dy: i32) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn Deflate(self: Pin<&mut wxRect>, diff: &wxSize) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn Deflate(self: Pin<&mut wxRect>, diff: i32) -> Pin<&mut wxRect>;
        // GENERATED: unsafe fn Deflate(self: &wxRect, dx: i32, dy: i32) -> wxRect;
        // GENERATED: unsafe fn GetBottom(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetBottomLeft(self: &wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetBottomRight(self: &wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetHeight(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetLeft(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetPosition(self: &wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetRight(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetSize(self: &wxRect) -> wxSize;
        // GENERATED: unsafe fn GetTop(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetTopLeft(self: &wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetTopRight(self: &wxRect) -> wxPoint;
        // GENERATED: unsafe fn GetWidth(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetX(self: &wxRect) -> i32;
        // GENERATED: unsafe fn GetY(self: &wxRect) -> i32;
        // BLOCKED: unsafe fn Inflate(self: Pin<&mut wxRect>, dx: i32, dy: i32) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn Inflate(self: Pin<&mut wxRect>, diff: &wxSize) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn Inflate(self: Pin<&mut wxRect>, diff: i32) -> Pin<&mut wxRect>;
        // GENERATED: unsafe fn Inflate(self: &wxRect, dx: i32, dy: i32) -> wxRect;
        // BLOCKED: unsafe fn Intersect(self: Pin<&mut wxRect>, rect: &wxRect) -> Pin<&mut wxRect>;
        // GENERATED: unsafe fn Intersect(self: &wxRect, rect: &wxRect) -> wxRect;
        // GENERATED: unsafe fn Intersects(self: &wxRect, rect: &wxRect) -> bool;
        // GENERATED: unsafe fn IsEmpty(self: &wxRect) -> bool;
        // GENERATED: unsafe fn Offset(self: Pin<&mut wxRect>, dx: i32, dy: i32);
        // GENERATED: unsafe fn Offset(self: Pin<&mut wxRect>, pt: &wxPoint);
        // GENERATED: unsafe fn SetHeight(self: Pin<&mut wxRect>, height: i32);
        // GENERATED: unsafe fn SetPosition(self: Pin<&mut wxRect>, pos: &wxPoint);
        // GENERATED: unsafe fn SetSize(self: Pin<&mut wxRect>, s: &wxSize);
        // GENERATED: unsafe fn SetWidth(self: Pin<&mut wxRect>, width: i32);
        // GENERATED: unsafe fn SetX(self: Pin<&mut wxRect>, x: i32);
        // GENERATED: unsafe fn SetY(self: Pin<&mut wxRect>, y: i32);
        // GENERATED: unsafe fn SetLeft(self: Pin<&mut wxRect>, left: i32);
        // GENERATED: unsafe fn SetRight(self: Pin<&mut wxRect>, right: i32);
        // GENERATED: unsafe fn SetTop(self: Pin<&mut wxRect>, top: i32);
        // GENERATED: unsafe fn SetBottom(self: Pin<&mut wxRect>, bottom: i32);
        // GENERATED: unsafe fn SetTopLeft(self: Pin<&mut wxRect>, p: &wxPoint);
        // GENERATED: unsafe fn SetBottomRight(self: Pin<&mut wxRect>, p: &wxPoint);
        // GENERATED: unsafe fn SetTopRight(self: Pin<&mut wxRect>, p: &wxPoint);
        // GENERATED: unsafe fn SetBottomLeft(self: Pin<&mut wxRect>, p: &wxPoint);
        // GENERATED: unsafe fn Union(self: &wxRect, rect: &wxRect) -> wxRect;
        // BLOCKED: unsafe fn Union(self: Pin<&mut wxRect>, rect: &wxRect) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn operator!=(self: Pin<&mut wxRect>, r1: &wxRect, r2: &wxRect) -> bool;
        // BLOCKED: unsafe fn operator+(self: Pin<&mut wxRect>, r1: &wxRect, r2: &wxRect) -> wxRect;
        // BLOCKED: unsafe fn operator+=(self: Pin<&mut wxRect>, r: &wxRect) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn operator*(self: Pin<&mut wxRect>, r1: &wxRect, r2: &wxRect) -> wxRect;
        // BLOCKED: unsafe fn operator*=(self: Pin<&mut wxRect>, r: &wxRect) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn operator=(self: Pin<&mut wxRect>, rect: &wxRect) -> Pin<&mut wxRect>;
        // BLOCKED: unsafe fn operator==(self: Pin<&mut wxRect>, r1: &wxRect, r2: &wxRect) -> bool;
        
        // CLASS: wxSize
        type wxSize;
        // BLOCKED: unsafe fn operator=(self: Pin<&mut wxSize>, sz: &wxSize) -> Pin<&mut wxSize>;
        // BLOCKED: unsafe fn operator==(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> bool;
        // BLOCKED: unsafe fn operator!=(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> bool;
        // BLOCKED: unsafe fn operator+(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator-(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator+=(self: Pin<&mut wxSize>, sz: &wxSize) -> Pin<&mut wxSize>;
        // BLOCKED: unsafe fn operator-=(self: Pin<&mut wxSize>, sz: &wxSize) -> Pin<&mut wxSize>;
        // BLOCKED: unsafe fn operator/(self: Pin<&mut wxSize>, sz: &wxSize, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*(self: Pin<&mut wxSize>, sz: &wxSize, factor: i32) -> wxSize;
        // BLOCKED: unsafe fn operator*(self: Pin<&mut wxSize>, factor: i32, sz: &wxSize) -> wxSize;
        // BLOCKED: unsafe fn operator/=(self: Pin<&mut wxSize>, factor: i32) -> Pin<&mut wxSize>;
        // BLOCKED: unsafe fn operator*=(self: Pin<&mut wxSize>, factor: i32) -> Pin<&mut wxSize>;
        // CTOR: unsafe fn wxSize() -> Size;
        // CTOR: unsafe fn wxSize(width: i32, height: i32) -> Size;
        // GENERATED: unsafe fn DecBy(self: Pin<&mut wxSize>, pt: &wxPoint);
        // GENERATED: unsafe fn DecBy(self: Pin<&mut wxSize>, size: &wxSize);
        // GENERATED: unsafe fn DecBy(self: Pin<&mut wxSize>, dx: i32, dy: i32);
        // GENERATED: unsafe fn DecBy(self: Pin<&mut wxSize>, d: i32);
        // GENERATED: unsafe fn DecTo(self: Pin<&mut wxSize>, size: &wxSize);
        // GENERATED: unsafe fn DecToIfSpecified(self: Pin<&mut wxSize>, size: &wxSize);
        // GENERATED: unsafe fn GetHeight(self: &wxSize) -> i32;
        // GENERATED: unsafe fn GetWidth(self: &wxSize) -> i32;
        // GENERATED: unsafe fn IncBy(self: Pin<&mut wxSize>, pt: &wxPoint);
        // GENERATED: unsafe fn IncBy(self: Pin<&mut wxSize>, size: &wxSize);
        // GENERATED: unsafe fn IncBy(self: Pin<&mut wxSize>, dx: i32, dy: i32);
        // GENERATED: unsafe fn IncBy(self: Pin<&mut wxSize>, d: i32);
        // GENERATED: unsafe fn IncTo(self: Pin<&mut wxSize>, size: &wxSize);
        // GENERATED: unsafe fn IsFullySpecified(self: &wxSize) -> bool;
        // BLOCKED: unsafe fn Scale(self: Pin<&mut wxSize>, xscale: f64, yscale: f64) -> Pin<&mut wxSize>;
        // GENERATED: unsafe fn Set(self: Pin<&mut wxSize>, width: i32, height: i32);
        // GENERATED: unsafe fn SetDefaults(self: Pin<&mut wxSize>, size_default: &wxSize);
        // GENERATED: unsafe fn SetHeight(self: Pin<&mut wxSize>, height: i32);
        // GENERATED: unsafe fn SetWidth(self: Pin<&mut wxSize>, width: i32);
        
        // CLASS: wxValidator
        type wxValidator;
        // CTOR: unsafe fn wxValidator() -> Validator;
        // DTOR: unsafe fn ~wxValidator(self: Pin<&mut wxValidator>);
        // GENERATED: unsafe fn Clone(self: &wxValidator) -> *mut wxObject;
        // GENERATED: unsafe fn GetWindow(self: &wxValidator) -> *mut wxWindow;
        // GENERATED: unsafe fn SetWindow(self: Pin<&mut wxValidator>, window: *mut wxWindow);
        // GENERATED: unsafe fn TransferFromWindow(self: Pin<&mut wxValidator>) -> bool;
        // GENERATED: unsafe fn TransferToWindow(self: Pin<&mut wxValidator>) -> bool;
        // GENERATED: unsafe fn Validate(self: Pin<&mut wxValidator>, parent: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn SuppressBellOnError(suppress: bool);
        // GENERATED: unsafe fn IsSilent() -> bool;
    }
    unsafe extern "C++" {
        // CLASS: wxObject
        unsafe fn wxObject_new() -> *mut wxObject;
        #[rust_name = "wxObject_new1"]
        unsafe fn wxObject_new(other: &wxObject) -> *mut wxObject;
        unsafe fn wxObject_GetClassInfo(self_: &wxObject) -> *mut wxClassInfo;
        unsafe fn wxObject_GetRefData(self_: &wxObject) -> *mut wxObjectRefData;
        unsafe fn wxObject_IsKindOf(self_: &wxObject, info: *const wxClassInfo) -> bool;
        unsafe fn wxObject_IsSameAs(self_: &wxObject, obj: &wxObject) -> bool;
        unsafe fn wxObject_Ref(self_: Pin<&mut wxObject>, clone: &wxObject);
        unsafe fn wxObject_SetRefData(self_: Pin<&mut wxObject>, data: *mut wxObjectRefData);
        unsafe fn wxObject_UnRef(self_: Pin<&mut wxObject>);
        unsafe fn wxObject_UnShare(self_: Pin<&mut wxObject>);
        // CLASS: wxEvtHandler
        unsafe fn wxEvtHandler_QueueEvent(self_: Pin<&mut wxEvtHandler>, event: *mut wxEvent);
        unsafe fn wxEvtHandler_AddPendingEvent(self_: Pin<&mut wxEvtHandler>, event: &wxEvent);
        unsafe fn wxEvtHandler_ProcessEvent(self_: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn wxEvtHandler_ProcessEventLocally(self_: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn wxEvtHandler_SafelyProcessEvent(self_: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn wxEvtHandler_ProcessPendingEvents(self_: Pin<&mut wxEvtHandler>);
        unsafe fn wxEvtHandler_DeletePendingEvents(self_: Pin<&mut wxEvtHandler>);
        unsafe fn wxEvtHandler_GetClientObject(self_: &wxEvtHandler) -> *mut wxClientData;
        unsafe fn wxEvtHandler_SetClientObject(self_: Pin<&mut wxEvtHandler>, data: *mut wxClientData);
        unsafe fn wxEvtHandler_GetEvtHandlerEnabled(self_: &wxEvtHandler) -> bool;
        unsafe fn wxEvtHandler_GetNextHandler(self_: &wxEvtHandler) -> *mut wxEvtHandler;
        unsafe fn wxEvtHandler_GetPreviousHandler(self_: &wxEvtHandler) -> *mut wxEvtHandler;
        unsafe fn wxEvtHandler_SetEvtHandlerEnabled(self_: Pin<&mut wxEvtHandler>, enabled: bool);
        unsafe fn wxEvtHandler_SetNextHandler(self_: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_SetPreviousHandler(self_: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_Unlink(self_: Pin<&mut wxEvtHandler>);
        unsafe fn wxEvtHandler_IsUnlinked(self_: &wxEvtHandler) -> bool;
        unsafe fn wxEvtHandler_AddFilter(filter: *mut wxEventFilter);
        unsafe fn wxEvtHandler_RemoveFilter(filter: *mut wxEventFilter);
        unsafe fn wxEvtHandler_new() -> *mut wxEvtHandler;
        // CLASS: wxWindow
        unsafe fn wxWindow_AcceptsFocus(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_AcceptsFocusFromKeyboard(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_AcceptsFocusRecursively(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_DisableFocusFromKeyboard(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_IsFocusable(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_CanAcceptFocus(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_CanAcceptFocusFromKeyboard(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_HasFocus(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_SetCanFocus(self_: Pin<&mut wxWindow>, can_focus: bool);
        unsafe fn wxWindow_EnableVisibleFocus(self_: Pin<&mut wxWindow>, enable: bool);
        unsafe fn wxWindow_SetFocus(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_SetFocusFromKbd(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_AddChild(self_: Pin<&mut wxWindow>, child: *mut wxWindow);
        unsafe fn wxWindow_DestroyChildren(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_FindWindow(self_: &wxWindow, id: i32) -> *mut wxWindow;
        #[rust_name = "wxWindow_FindWindow1"]
        unsafe fn wxWindow_FindWindow(self_: &wxWindow, name: &wxString) -> *mut wxWindow;
        unsafe fn wxWindow_RemoveChild(self_: Pin<&mut wxWindow>, child: *mut wxWindow);
        unsafe fn wxWindow_GetGrandParent(self_: &wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetNextSibling(self_: &wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetParent(self_: &wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetPrevSibling(self_: &wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_IsDescendant(self_: &wxWindow, win: *mut wxWindow) -> bool;
        unsafe fn wxWindow_Reparent(self_: Pin<&mut wxWindow>, new_parent: *mut wxWindow) -> bool;
        unsafe fn wxWindow_AlwaysShowScrollbars(self_: Pin<&mut wxWindow>, hflag: bool, vflag: bool);
        unsafe fn wxWindow_GetScrollPos(self_: &wxWindow, orientation: i32) -> i32;
        unsafe fn wxWindow_GetScrollRange(self_: &wxWindow, orientation: i32) -> i32;
        unsafe fn wxWindow_GetScrollThumb(self_: &wxWindow, orientation: i32) -> i32;
        unsafe fn wxWindow_CanScroll(self_: &wxWindow, orient: i32) -> bool;
        unsafe fn wxWindow_HasScrollbar(self_: &wxWindow, orient: i32) -> bool;
        unsafe fn wxWindow_IsScrollbarAlwaysShown(self_: &wxWindow, orient: i32) -> bool;
        unsafe fn wxWindow_ScrollLines(self_: Pin<&mut wxWindow>, lines: i32) -> bool;
        unsafe fn wxWindow_ScrollPages(self_: Pin<&mut wxWindow>, pages: i32) -> bool;
        unsafe fn wxWindow_ScrollWindow(self_: Pin<&mut wxWindow>, dx: i32, dy: i32, rect: *const wxRect);
        unsafe fn wxWindow_LineUp(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_LineDown(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_PageUp(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_PageDown(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_SetScrollPos(self_: Pin<&mut wxWindow>, orientation: i32, pos: i32, refresh: bool);
        unsafe fn wxWindow_SetScrollbar(self_: Pin<&mut wxWindow>, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        unsafe fn wxWindow_BeginRepositioningChildren(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_EndRepositioningChildren(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_CacheBestSize(self_: &wxWindow, size: &wxSize);
        unsafe fn wxWindow_ClientToWindowSize(self_: &wxWindow, size: &wxSize) -> *mut wxSize;
        unsafe fn wxWindow_WindowToClientSize(self_: &wxWindow, size: &wxSize) -> *mut wxSize;
        unsafe fn wxWindow_Fit(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_FitInside(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_FromDIP(self_: &wxWindow, sz: &wxSize) -> *mut wxSize;
        #[rust_name = "wxWindow_FromDIP1"]
        unsafe fn wxWindow_FromDIP(self_: &wxWindow, pt: &wxPoint) -> *mut wxPoint;
        #[rust_name = "wxWindow_FromDIP2"]
        unsafe fn wxWindow_FromDIP(self_: &wxWindow, d: i32) -> i32;
        unsafe fn wxWindow_ToDIP(self_: &wxWindow, sz: &wxSize) -> *mut wxSize;
        #[rust_name = "wxWindow_ToDIP1"]
        unsafe fn wxWindow_ToDIP(self_: &wxWindow, pt: &wxPoint) -> *mut wxPoint;
        #[rust_name = "wxWindow_ToDIP2"]
        unsafe fn wxWindow_ToDIP(self_: &wxWindow, d: i32) -> i32;
        unsafe fn wxWindow_GetBestSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetBestHeight(self_: &wxWindow, width: i32) -> i32;
        unsafe fn wxWindow_GetBestWidth(self_: &wxWindow, height: i32) -> i32;
        unsafe fn wxWindow_GetClientSize(self_: &wxWindow, width: *mut i32, height: *mut i32);
        #[rust_name = "wxWindow_GetClientSize1"]
        unsafe fn wxWindow_GetClientSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetEffectiveMinSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMaxClientSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMaxSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMinClientSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMinSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetMinWidth(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetMinHeight(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetMaxWidth(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetMaxHeight(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetSize(self_: &wxWindow, width: *mut i32, height: *mut i32);
        #[rust_name = "wxWindow_GetSize1"]
        unsafe fn wxWindow_GetSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetVirtualSize(self_: &wxWindow) -> *mut wxSize;
        #[rust_name = "wxWindow_GetVirtualSize1"]
        unsafe fn wxWindow_GetVirtualSize(self_: &wxWindow, width: *mut i32, height: *mut i32);
        unsafe fn wxWindow_GetBestVirtualSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetContentScaleFactor(self_: &wxWindow) -> f64;
        unsafe fn wxWindow_GetDPIScaleFactor(self_: &wxWindow) -> f64;
        unsafe fn wxWindow_GetWindowBorderSize(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_InformFirstDirection(self_: Pin<&mut wxWindow>, direction: i32, size: i32, available_other_dir: i32) -> bool;
        unsafe fn wxWindow_InvalidateBestSize(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_PostSizeEvent(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_PostSizeEventToParent(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_SendSizeEvent(self_: Pin<&mut wxWindow>, flags: i32);
        unsafe fn wxWindow_SendSizeEventToParent(self_: Pin<&mut wxWindow>, flags: i32);
        unsafe fn wxWindow_SetClientSize(self_: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "wxWindow_SetClientSize1"]
        unsafe fn wxWindow_SetClientSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "wxWindow_SetClientSize2"]
        unsafe fn wxWindow_SetClientSize(self_: Pin<&mut wxWindow>, rect: &wxRect);
        unsafe fn wxWindow_SetContainingSizer(self_: Pin<&mut wxWindow>, sizer: *mut wxSizer);
        unsafe fn wxWindow_SetInitialSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        unsafe fn wxWindow_SetMaxClientSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        unsafe fn wxWindow_SetMaxSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        unsafe fn wxWindow_SetMinClientSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        unsafe fn wxWindow_SetMinSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        unsafe fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        #[rust_name = "wxWindow_SetSize1"]
        unsafe fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, rect: &wxRect);
        #[rust_name = "wxWindow_SetSize2"]
        unsafe fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "wxWindow_SetSize3"]
        unsafe fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, width: i32, height: i32);
        unsafe fn wxWindow_SetSizeHints(self_: Pin<&mut wxWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        #[rust_name = "wxWindow_SetSizeHints1"]
        unsafe fn wxWindow_SetSizeHints(self_: Pin<&mut wxWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        unsafe fn wxWindow_SetVirtualSize(self_: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "wxWindow_SetVirtualSize1"]
        unsafe fn wxWindow_SetVirtualSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "wxWindow_FromDIP3"]
        unsafe fn wxWindow_FromDIP(sz: &wxSize, w: *const wxWindow) -> *mut wxSize;
        #[rust_name = "wxWindow_FromDIP4"]
        unsafe fn wxWindow_FromDIP(pt: &wxPoint, w: *const wxWindow) -> *mut wxPoint;
        #[rust_name = "wxWindow_FromDIP5"]
        unsafe fn wxWindow_FromDIP(d: i32, w: *const wxWindow) -> i32;
        #[rust_name = "wxWindow_ToDIP3"]
        unsafe fn wxWindow_ToDIP(sz: &wxSize, w: *const wxWindow) -> *mut wxSize;
        #[rust_name = "wxWindow_ToDIP4"]
        unsafe fn wxWindow_ToDIP(pt: &wxPoint, w: *const wxWindow) -> *mut wxPoint;
        #[rust_name = "wxWindow_ToDIP5"]
        unsafe fn wxWindow_ToDIP(d: i32, w: *const wxWindow) -> i32;
        unsafe fn wxWindow_Center(self_: Pin<&mut wxWindow>, dir: i32);
        unsafe fn wxWindow_CenterOnParent(self_: Pin<&mut wxWindow>, dir: i32);
        unsafe fn wxWindow_Centre(self_: Pin<&mut wxWindow>, direction: i32);
        unsafe fn wxWindow_CentreOnParent(self_: Pin<&mut wxWindow>, direction: i32);
        unsafe fn wxWindow_GetPosition(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_GetPosition1"]
        unsafe fn wxWindow_GetPosition(self_: &wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_GetRect(self_: &wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_GetScreenPosition(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_GetScreenPosition1"]
        unsafe fn wxWindow_GetScreenPosition(self_: &wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_GetScreenRect(self_: &wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_GetClientAreaOrigin(self_: &wxWindow) -> *mut wxPoint;
        unsafe fn wxWindow_GetClientRect(self_: &wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_Move(self_: Pin<&mut wxWindow>, x: i32, y: i32, flags: i32);
        #[rust_name = "wxWindow_Move1"]
        unsafe fn wxWindow_Move(self_: Pin<&mut wxWindow>, pt: &wxPoint, flags: i32);
        unsafe fn wxWindow_SetPosition(self_: Pin<&mut wxWindow>, pt: &wxPoint);
        unsafe fn wxWindow_ClientToScreen(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_ClientToScreen1"]
        unsafe fn wxWindow_ClientToScreen(self_: &wxWindow, pt: &wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ConvertDialogToPixels(self_: &wxWindow, pt: &wxPoint) -> *mut wxPoint;
        #[rust_name = "wxWindow_ConvertDialogToPixels1"]
        unsafe fn wxWindow_ConvertDialogToPixels(self_: &wxWindow, sz: &wxSize) -> *mut wxSize;
        unsafe fn wxWindow_ConvertPixelsToDialog(self_: &wxWindow, pt: &wxPoint) -> *mut wxPoint;
        #[rust_name = "wxWindow_ConvertPixelsToDialog1"]
        unsafe fn wxWindow_ConvertPixelsToDialog(self_: &wxWindow, sz: &wxSize) -> *mut wxSize;
        unsafe fn wxWindow_ScreenToClient(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_ScreenToClient1"]
        unsafe fn wxWindow_ScreenToClient(self_: &wxWindow, pt: &wxPoint) -> *mut wxPoint;
        unsafe fn wxWindow_ClearBackground(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_Freeze(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_Thaw(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_IsFrozen(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_GetCharHeight(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetCharWidth(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetDPI(self_: &wxWindow) -> *mut wxSize;
        unsafe fn wxWindow_GetTextExtent(self_: &wxWindow, string: &wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        #[rust_name = "wxWindow_GetTextExtent1"]
        unsafe fn wxWindow_GetTextExtent(self_: &wxWindow, string: &wxString) -> *mut wxSize;
        unsafe fn wxWindow_GetUpdateClientRect(self_: &wxWindow) -> *mut wxRect;
        unsafe fn wxWindow_HasTransparentBackground(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_Refresh(self_: Pin<&mut wxWindow>, erase_background: bool, rect: *const wxRect);
        unsafe fn wxWindow_RefreshRect(self_: Pin<&mut wxWindow>, rect: &wxRect, erase_background: bool);
        unsafe fn wxWindow_Update(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_SetBackgroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        unsafe fn wxWindow_IsTransparentBackgroundSupported(self_: &wxWindow, reason: *mut wxString) -> bool;
        unsafe fn wxWindow_SetFont(self_: Pin<&mut wxWindow>, font: &wxFont) -> bool;
        unsafe fn wxWindow_SetForegroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        unsafe fn wxWindow_SetOwnBackgroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour);
        unsafe fn wxWindow_InheritsBackgroundColour(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_UseBgCol(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_UseBackgroundColour(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_SetOwnFont(self_: Pin<&mut wxWindow>, font: &wxFont);
        unsafe fn wxWindow_SetOwnForegroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour);
        unsafe fn wxWindow_UseForegroundColour(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_InheritsForegroundColour(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_SetPalette(self_: Pin<&mut wxWindow>, pal: &wxPalette);
        unsafe fn wxWindow_ShouldInheritColours(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_SetThemeEnabled(self_: Pin<&mut wxWindow>, enable: bool);
        unsafe fn wxWindow_GetThemeEnabled(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_CanSetTransparent(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_SetTransparent(self_: Pin<&mut wxWindow>, alpha: u8) -> bool;
        unsafe fn wxWindow_GetEventHandler(self_: &wxWindow) -> *mut wxEvtHandler;
        unsafe fn wxWindow_HandleAsNavigationKey(self_: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        unsafe fn wxWindow_HandleWindowEvent(self_: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn wxWindow_ProcessWindowEvent(self_: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn wxWindow_ProcessWindowEventLocally(self_: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn wxWindow_PopEventHandler(self_: Pin<&mut wxWindow>, delete_handler: bool) -> *mut wxEvtHandler;
        unsafe fn wxWindow_PushEventHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_RemoveEventHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler) -> bool;
        unsafe fn wxWindow_SetEventHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_SetNextHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_SetPreviousHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_GetExtraStyle(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetWindowStyleFlag(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetWindowStyle(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_HasExtraStyle(self_: &wxWindow, ex_flag: i32) -> bool;
        unsafe fn wxWindow_HasFlag(self_: &wxWindow, flag: i32) -> bool;
        unsafe fn wxWindow_SetExtraStyle(self_: Pin<&mut wxWindow>, ex_style: i32);
        unsafe fn wxWindow_SetWindowStyleFlag(self_: Pin<&mut wxWindow>, style: i32);
        unsafe fn wxWindow_SetWindowStyle(self_: Pin<&mut wxWindow>, style: i32);
        unsafe fn wxWindow_ToggleWindowStyle(self_: Pin<&mut wxWindow>, flag: i32) -> bool;
        unsafe fn wxWindow_MoveAfterInTabOrder(self_: Pin<&mut wxWindow>, win: *mut wxWindow);
        unsafe fn wxWindow_MoveBeforeInTabOrder(self_: Pin<&mut wxWindow>, win: *mut wxWindow);
        unsafe fn wxWindow_Navigate(self_: Pin<&mut wxWindow>, flags: i32) -> bool;
        unsafe fn wxWindow_NavigateIn(self_: Pin<&mut wxWindow>, flags: i32) -> bool;
        unsafe fn wxWindow_Lower(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_Raise(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_Hide(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_IsEnabled(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_IsExposed(self_: &wxWindow, x: i32, y: i32) -> bool;
        #[rust_name = "wxWindow_IsExposed1"]
        unsafe fn wxWindow_IsExposed(self_: &wxWindow, pt: Pin<&mut wxPoint>) -> bool;
        #[rust_name = "wxWindow_IsExposed2"]
        unsafe fn wxWindow_IsExposed(self_: &wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        #[rust_name = "wxWindow_IsExposed3"]
        unsafe fn wxWindow_IsExposed(self_: &wxWindow, rect: Pin<&mut wxRect>) -> bool;
        unsafe fn wxWindow_IsShown(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_IsShownOnScreen(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_Disable(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_Enable(self_: Pin<&mut wxWindow>, enable: bool) -> bool;
        unsafe fn wxWindow_Show(self_: Pin<&mut wxWindow>, show: bool) -> bool;
        unsafe fn wxWindow_GetHelpText(self_: &wxWindow) -> *mut wxString;
        unsafe fn wxWindow_SetHelpText(self_: Pin<&mut wxWindow>, help_text: &wxString);
        unsafe fn wxWindow_GetToolTip(self_: &wxWindow) -> *mut wxToolTip;
        unsafe fn wxWindow_GetToolTipText(self_: &wxWindow) -> *mut wxString;
        unsafe fn wxWindow_SetToolTip(self_: Pin<&mut wxWindow>, tip_string: &wxString);
        #[rust_name = "wxWindow_SetToolTip1"]
        unsafe fn wxWindow_SetToolTip(self_: Pin<&mut wxWindow>, tip: *mut wxToolTip);
        unsafe fn wxWindow_UnsetToolTip(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_GetPopupMenuSelectionFromUser(self_: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, pos: &wxPoint) -> i32;
        #[rust_name = "wxWindow_GetPopupMenuSelectionFromUser1"]
        unsafe fn wxWindow_GetPopupMenuSelectionFromUser(self_: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, x: i32, y: i32) -> i32;
        unsafe fn wxWindow_PopupMenu(self_: Pin<&mut wxWindow>, menu: *mut wxMenu, pos: &wxPoint) -> bool;
        #[rust_name = "wxWindow_PopupMenu1"]
        unsafe fn wxWindow_PopupMenu(self_: Pin<&mut wxWindow>, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        unsafe fn wxWindow_GetValidator(self_: Pin<&mut wxWindow>) -> *mut wxValidator;
        unsafe fn wxWindow_SetValidator(self_: Pin<&mut wxWindow>, validator: &wxValidator);
        unsafe fn wxWindow_TransferDataFromWindow(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_TransferDataToWindow(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_Validate(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_GetId(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetLabel(self_: &wxWindow) -> *mut wxString;
        unsafe fn wxWindow_AdjustForLayoutDirection(self_: &wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        unsafe fn wxWindow_GetName(self_: &wxWindow) -> *mut wxString;
        unsafe fn wxWindow_SetId(self_: Pin<&mut wxWindow>, winid: i32);
        unsafe fn wxWindow_SetLabel(self_: Pin<&mut wxWindow>, label: &wxString);
        unsafe fn wxWindow_SetName(self_: Pin<&mut wxWindow>, name: &wxString);
        unsafe fn wxWindow_GetAcceleratorTable(self_: Pin<&mut wxWindow>) -> *mut wxAcceleratorTable;
        unsafe fn wxWindow_SetAcceleratorTable(self_: Pin<&mut wxWindow>, accel: &wxAcceleratorTable);
        unsafe fn wxWindow_Close(self_: Pin<&mut wxWindow>, force: bool) -> bool;
        unsafe fn wxWindow_Destroy(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_IsBeingDeleted(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_GetDropTarget(self_: &wxWindow) -> *mut wxDropTarget;
        unsafe fn wxWindow_SetDropTarget(self_: Pin<&mut wxWindow>, target: *mut wxDropTarget);
        unsafe fn wxWindow_DragAcceptFiles(self_: Pin<&mut wxWindow>, accept: bool);
        unsafe fn wxWindow_GetContainingSizer(self_: &wxWindow) -> *mut wxSizer;
        unsafe fn wxWindow_GetSizer(self_: &wxWindow) -> *mut wxSizer;
        unsafe fn wxWindow_SetSizer(self_: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        unsafe fn wxWindow_SetSizerAndFit(self_: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        unsafe fn wxWindow_GetConstraints(self_: &wxWindow) -> *mut wxLayoutConstraints;
        unsafe fn wxWindow_SetConstraints(self_: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        unsafe fn wxWindow_Layout(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_SetAutoLayout(self_: Pin<&mut wxWindow>, auto_layout: bool);
        unsafe fn wxWindow_GetAutoLayout(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_CaptureMouse(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_GetCaret(self_: &wxWindow) -> *mut wxCaret;
        unsafe fn wxWindow_HasCapture(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_ReleaseMouse(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_SetCaret(self_: Pin<&mut wxWindow>, caret: *mut wxCaret);
        unsafe fn wxWindow_SetCursor(self_: Pin<&mut wxWindow>, cursor: &wxCursor) -> bool;
        unsafe fn wxWindow_WarpPointer(self_: Pin<&mut wxWindow>, x: i32, y: i32);
        unsafe fn wxWindow_EnableTouchEvents(self_: Pin<&mut wxWindow>, events_mask: i32) -> bool;
        unsafe fn wxWindow_DoUpdateWindowUI(self_: Pin<&mut wxWindow>, event: Pin<&mut wxUpdateUIEvent>);
        unsafe fn wxWindow_HasMultiplePages(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_InheritAttributes(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_InitDialog(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_IsDoubleBuffered(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_SetDoubleBuffered(self_: Pin<&mut wxWindow>, on: bool);
        unsafe fn wxWindow_IsRetained(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_IsThisEnabled(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_IsTopLevel(self_: &wxWindow) -> bool;
        unsafe fn wxWindow_OnInternalIdle(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_SendIdleEvents(self_: Pin<&mut wxWindow>, event: Pin<&mut wxIdleEvent>) -> bool;
        unsafe fn wxWindow_RegisterHotKey(self_: Pin<&mut wxWindow>, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        unsafe fn wxWindow_UnregisterHotKey(self_: Pin<&mut wxWindow>, hotkey_id: i32) -> bool;
        unsafe fn wxWindow_UpdateWindowUI(self_: Pin<&mut wxWindow>, flags: i32);
        unsafe fn wxWindow_FindFocus() -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_GetCapture() -> *mut wxWindow;
        unsafe fn wxWindow_NewControlId(count: i32) -> i32;
        unsafe fn wxWindow_UnreserveControlId(id: i32, count: i32);
        unsafe fn wxWindow_new() -> *mut wxWindow;
        #[rust_name = "wxWindow_new1"]
        unsafe fn wxWindow_new(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxWindow;
        unsafe fn wxWindow_Create(self_: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // CLASS: wxControl
        unsafe fn wxControl_new(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxControl;
        #[rust_name = "wxControl_new1"]
        unsafe fn wxControl_new() -> *mut wxControl;
        unsafe fn wxControl_Create(self_: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        unsafe fn wxControl_Command(self_: Pin<&mut wxControl>, event: Pin<&mut wxCommandEvent>);
        unsafe fn wxControl_GetLabel(self_: &wxControl) -> *mut wxString;
        unsafe fn wxControl_GetLabelText(self_: &wxControl) -> *mut wxString;
        unsafe fn wxControl_GetSizeFromTextSize(self_: &wxControl, xlen: i32, ylen: i32) -> *mut wxSize;
        #[rust_name = "wxControl_GetSizeFromTextSize1"]
        unsafe fn wxControl_GetSizeFromTextSize(self_: &wxControl, tsize: &wxSize) -> *mut wxSize;
        unsafe fn wxControl_GetSizeFromText(self_: &wxControl, text: &wxString) -> *mut wxSize;
        unsafe fn wxControl_SetLabel(self_: Pin<&mut wxControl>, label: &wxString);
        unsafe fn wxControl_SetLabelText(self_: Pin<&mut wxControl>, text: &wxString);
        unsafe fn wxControl_SetLabelMarkup(self_: Pin<&mut wxControl>, markup: &wxString) -> bool;
        #[rust_name = "wxControl_GetLabelText1"]
        unsafe fn wxControl_GetLabelText(label: &wxString) -> *mut wxString;
        unsafe fn wxControl_RemoveMnemonics(str: &wxString) -> *mut wxString;
        unsafe fn wxControl_EscapeMnemonics(text: &wxString) -> *mut wxString;
        // CLASS: wxAnyButton
        unsafe fn wxAnyButton_new() -> *mut wxAnyButton;
        unsafe fn wxAnyButton_SetBitmapCurrent(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        unsafe fn wxAnyButton_SetBitmapDisabled(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        unsafe fn wxAnyButton_SetBitmapFocus(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        unsafe fn wxAnyButton_SetBitmapLabel(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        unsafe fn wxAnyButton_SetBitmapPressed(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        unsafe fn wxAnyButton_GetBitmapMargins(self_: Pin<&mut wxAnyButton>) -> *mut wxSize;
        unsafe fn wxAnyButton_SetBitmapMargins(self_: Pin<&mut wxAnyButton>, x: i32, y: i32);
        #[rust_name = "wxAnyButton_SetBitmapMargins1"]
        unsafe fn wxAnyButton_SetBitmapMargins(self_: Pin<&mut wxAnyButton>, sz: &wxSize);
        // CLASS: wxButton
        unsafe fn wxButton_new() -> *mut wxButton;
        #[rust_name = "wxButton_new1"]
        unsafe fn wxButton_new(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxButton;
        unsafe fn wxButton_Create(self_: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        unsafe fn wxButton_GetAuthNeeded(self_: &wxButton) -> bool;
        unsafe fn wxButton_GetLabel(self_: &wxButton) -> *mut wxString;
        unsafe fn wxButton_SetAuthNeeded(self_: Pin<&mut wxButton>, needed: bool);
        unsafe fn wxButton_SetDefault(self_: Pin<&mut wxButton>) -> *mut wxWindow;
        unsafe fn wxButton_SetLabel(self_: Pin<&mut wxButton>, label: &wxString);
        unsafe fn wxButton_GetDefaultSize(win: *mut wxWindow) -> *mut wxSize;
        // CLASS: wxNonOwnedWindow
        unsafe fn wxNonOwnedWindow_SetShape(self_: Pin<&mut wxNonOwnedWindow>, region: &wxRegion) -> bool;
        #[rust_name = "wxNonOwnedWindow_SetShape1"]
        unsafe fn wxNonOwnedWindow_SetShape(self_: Pin<&mut wxNonOwnedWindow>, path: &wxGraphicsPath) -> bool;
        // CLASS: wxTopLevelWindow
        unsafe fn wxTopLevelWindow_new() -> *mut wxTopLevelWindow;
        #[rust_name = "wxTopLevelWindow_new1"]
        unsafe fn wxTopLevelWindow_new(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxTopLevelWindow;
        unsafe fn wxTopLevelWindow_Create(self_: Pin<&mut wxTopLevelWindow>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        unsafe fn wxTopLevelWindow_CanSetTransparent(self_: Pin<&mut wxTopLevelWindow>) -> bool;
        unsafe fn wxTopLevelWindow_CenterOnScreen(self_: Pin<&mut wxTopLevelWindow>, direction: i32);
        unsafe fn wxTopLevelWindow_CentreOnScreen(self_: Pin<&mut wxTopLevelWindow>, direction: i32);
        unsafe fn wxTopLevelWindow_EnableCloseButton(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_EnableMaximizeButton(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_EnableMinimizeButton(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_GetDefaultItem(self_: &wxTopLevelWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_GetTitle(self_: &wxTopLevelWindow) -> *mut wxString;
        unsafe fn wxTopLevelWindow_Iconize(self_: Pin<&mut wxTopLevelWindow>, iconize: bool);
        unsafe fn wxTopLevelWindow_IsActive(self_: Pin<&mut wxTopLevelWindow>) -> bool;
        unsafe fn wxTopLevelWindow_IsAlwaysMaximized(self_: &wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsFullScreen(self_: &wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsIconized(self_: &wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_IsMaximized(self_: &wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_Layout(self_: Pin<&mut wxTopLevelWindow>) -> bool;
        unsafe fn wxTopLevelWindow_Maximize(self_: Pin<&mut wxTopLevelWindow>, maximize: bool);
        unsafe fn wxTopLevelWindow_RequestUserAttention(self_: Pin<&mut wxTopLevelWindow>, flags: i32);
        unsafe fn wxTopLevelWindow_Restore(self_: Pin<&mut wxTopLevelWindow>);
        unsafe fn wxTopLevelWindow_SetDefaultItem(self_: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_SetTmpDefaultItem(self_: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_GetTmpDefaultItem(self_: &wxTopLevelWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_SetIcon(self_: Pin<&mut wxTopLevelWindow>, icon: &wxIcon);
        unsafe fn wxTopLevelWindow_SetIcons(self_: Pin<&mut wxTopLevelWindow>, icons: &wxIconBundle);
        unsafe fn wxTopLevelWindow_SetMaxSize(self_: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        unsafe fn wxTopLevelWindow_SetMinSize(self_: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        unsafe fn wxTopLevelWindow_SetSizeHints(self_: Pin<&mut wxTopLevelWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        #[rust_name = "wxTopLevelWindow_SetSizeHints1"]
        unsafe fn wxTopLevelWindow_SetSizeHints(self_: Pin<&mut wxTopLevelWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        unsafe fn wxTopLevelWindow_SetTitle(self_: Pin<&mut wxTopLevelWindow>, title: &wxString);
        unsafe fn wxTopLevelWindow_SetTransparent(self_: Pin<&mut wxTopLevelWindow>, alpha: u8) -> bool;
        unsafe fn wxTopLevelWindow_ShouldPreventAppExit(self_: &wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_OSXSetModified(self_: Pin<&mut wxTopLevelWindow>, modified: bool);
        unsafe fn wxTopLevelWindow_OSXIsModified(self_: &wxTopLevelWindow) -> bool;
        unsafe fn wxTopLevelWindow_SetRepresentedFilename(self_: Pin<&mut wxTopLevelWindow>, filename: &wxString);
        unsafe fn wxTopLevelWindow_ShowWithoutActivating(self_: Pin<&mut wxTopLevelWindow>);
        unsafe fn wxTopLevelWindow_EnableFullScreenView(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        unsafe fn wxTopLevelWindow_ShowFullScreen(self_: Pin<&mut wxTopLevelWindow>, show: bool, style: i32) -> bool;
        unsafe fn wxTopLevelWindow_GetDefaultSize() -> *mut wxSize;
        // CLASS: wxFrame
        unsafe fn wxFrame_new() -> *mut wxFrame;
        #[rust_name = "wxFrame_new1"]
        unsafe fn wxFrame_new(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxFrame;
        unsafe fn wxFrame_Centre(self_: Pin<&mut wxFrame>, direction: i32);
        unsafe fn wxFrame_Create(self_: Pin<&mut wxFrame>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        unsafe fn wxFrame_CreateStatusBar(self_: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        unsafe fn wxFrame_CreateToolBar(self_: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        unsafe fn wxFrame_DoGiveHelp(self_: Pin<&mut wxFrame>, text: &wxString, show: bool);
        unsafe fn wxFrame_GetClientAreaOrigin(self_: &wxFrame) -> *mut wxPoint;
        unsafe fn wxFrame_GetMenuBar(self_: &wxFrame) -> *mut wxMenuBar;
        unsafe fn wxFrame_GetStatusBar(self_: &wxFrame) -> *mut wxStatusBar;
        unsafe fn wxFrame_GetStatusBarPane(self_: &wxFrame) -> i32;
        unsafe fn wxFrame_GetToolBar(self_: &wxFrame) -> *mut wxToolBar;
        unsafe fn wxFrame_OnCreateStatusBar(self_: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        unsafe fn wxFrame_OnCreateToolBar(self_: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        unsafe fn wxFrame_ProcessCommand(self_: Pin<&mut wxFrame>, id: i32) -> bool;
        unsafe fn wxFrame_SetMenuBar(self_: Pin<&mut wxFrame>, menu_bar: *mut wxMenuBar);
        unsafe fn wxFrame_SetStatusBar(self_: Pin<&mut wxFrame>, status_bar: *mut wxStatusBar);
        unsafe fn wxFrame_SetStatusBarPane(self_: Pin<&mut wxFrame>, n: i32);
        unsafe fn wxFrame_SetStatusText(self_: Pin<&mut wxFrame>, text: &wxString, number: i32);
        unsafe fn wxFrame_SetStatusWidths(self_: Pin<&mut wxFrame>, n: i32, widths_field: *const i32);
        unsafe fn wxFrame_SetToolBar(self_: Pin<&mut wxFrame>, tool_bar: *mut wxToolBar);
        unsafe fn wxFrame_PushStatusText(self_: Pin<&mut wxFrame>, text: &wxString, number: i32);
        unsafe fn wxFrame_PopStatusText(self_: Pin<&mut wxFrame>, number: i32);
        // CLASS: wxPoint
        unsafe fn wxPoint_IsFullySpecified(self_: &wxPoint) -> bool;
        unsafe fn wxPoint_SetDefaults(self_: Pin<&mut wxPoint>, pt: &wxPoint);
        unsafe fn wxPoint_new() -> *mut wxPoint;
        #[rust_name = "wxPoint_new1"]
        unsafe fn wxPoint_new(x: i32, y: i32) -> *mut wxPoint;
        #[rust_name = "wxPoint_new2"]
        unsafe fn wxPoint_new(pt: &wxRealPoint) -> *mut wxPoint;
        // CLASS: wxRect
        unsafe fn wxRect_new() -> *mut wxRect;
        #[rust_name = "wxRect_new1"]
        unsafe fn wxRect_new(x: i32, y: i32, width: i32, height: i32) -> *mut wxRect;
        #[rust_name = "wxRect_new2"]
        unsafe fn wxRect_new(top_left: &wxPoint, bottom_right: &wxPoint) -> *mut wxRect;
        #[rust_name = "wxRect_new3"]
        unsafe fn wxRect_new(pos: &wxPoint, size: &wxSize) -> *mut wxRect;
        #[rust_name = "wxRect_new4"]
        unsafe fn wxRect_new(size: &wxSize) -> *mut wxRect;
        unsafe fn wxRect_CentreIn(self_: &wxRect, r: &wxRect, dir: i32) -> *mut wxRect;
        unsafe fn wxRect_CenterIn(self_: &wxRect, r: &wxRect, dir: i32) -> *mut wxRect;
        unsafe fn wxRect_Contains(self_: &wxRect, x: i32, y: i32) -> bool;
        #[rust_name = "wxRect_Contains1"]
        unsafe fn wxRect_Contains(self_: &wxRect, pt: &wxPoint) -> bool;
        #[rust_name = "wxRect_Contains2"]
        unsafe fn wxRect_Contains(self_: &wxRect, rect: &wxRect) -> bool;
        #[rust_name = "wxRect_Deflate3"]
        unsafe fn wxRect_Deflate(self_: &wxRect, dx: i32, dy: i32) -> *mut wxRect;
        unsafe fn wxRect_GetBottom(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetBottomLeft(self_: &wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetBottomRight(self_: &wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetHeight(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetLeft(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetPosition(self_: &wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetRight(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetSize(self_: &wxRect) -> *mut wxSize;
        unsafe fn wxRect_GetTop(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetTopLeft(self_: &wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetTopRight(self_: &wxRect) -> *mut wxPoint;
        unsafe fn wxRect_GetWidth(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetX(self_: &wxRect) -> i32;
        unsafe fn wxRect_GetY(self_: &wxRect) -> i32;
        #[rust_name = "wxRect_Inflate3"]
        unsafe fn wxRect_Inflate(self_: &wxRect, dx: i32, dy: i32) -> *mut wxRect;
        #[rust_name = "wxRect_Intersect1"]
        unsafe fn wxRect_Intersect(self_: &wxRect, rect: &wxRect) -> *mut wxRect;
        unsafe fn wxRect_Intersects(self_: &wxRect, rect: &wxRect) -> bool;
        unsafe fn wxRect_IsEmpty(self_: &wxRect) -> bool;
        unsafe fn wxRect_Offset(self_: Pin<&mut wxRect>, dx: i32, dy: i32);
        #[rust_name = "wxRect_Offset1"]
        unsafe fn wxRect_Offset(self_: Pin<&mut wxRect>, pt: &wxPoint);
        unsafe fn wxRect_SetHeight(self_: Pin<&mut wxRect>, height: i32);
        unsafe fn wxRect_SetPosition(self_: Pin<&mut wxRect>, pos: &wxPoint);
        unsafe fn wxRect_SetSize(self_: Pin<&mut wxRect>, s: &wxSize);
        unsafe fn wxRect_SetWidth(self_: Pin<&mut wxRect>, width: i32);
        unsafe fn wxRect_SetX(self_: Pin<&mut wxRect>, x: i32);
        unsafe fn wxRect_SetY(self_: Pin<&mut wxRect>, y: i32);
        unsafe fn wxRect_SetLeft(self_: Pin<&mut wxRect>, left: i32);
        unsafe fn wxRect_SetRight(self_: Pin<&mut wxRect>, right: i32);
        unsafe fn wxRect_SetTop(self_: Pin<&mut wxRect>, top: i32);
        unsafe fn wxRect_SetBottom(self_: Pin<&mut wxRect>, bottom: i32);
        unsafe fn wxRect_SetTopLeft(self_: Pin<&mut wxRect>, p: &wxPoint);
        unsafe fn wxRect_SetBottomRight(self_: Pin<&mut wxRect>, p: &wxPoint);
        unsafe fn wxRect_SetTopRight(self_: Pin<&mut wxRect>, p: &wxPoint);
        unsafe fn wxRect_SetBottomLeft(self_: Pin<&mut wxRect>, p: &wxPoint);
        unsafe fn wxRect_Union(self_: &wxRect, rect: &wxRect) -> *mut wxRect;
        // CLASS: wxSize
        unsafe fn wxSize_new() -> *mut wxSize;
        #[rust_name = "wxSize_new1"]
        unsafe fn wxSize_new(width: i32, height: i32) -> *mut wxSize;
        unsafe fn wxSize_DecBy(self_: Pin<&mut wxSize>, pt: &wxPoint);
        #[rust_name = "wxSize_DecBy1"]
        unsafe fn wxSize_DecBy(self_: Pin<&mut wxSize>, size: &wxSize);
        #[rust_name = "wxSize_DecBy2"]
        unsafe fn wxSize_DecBy(self_: Pin<&mut wxSize>, dx: i32, dy: i32);
        #[rust_name = "wxSize_DecBy3"]
        unsafe fn wxSize_DecBy(self_: Pin<&mut wxSize>, d: i32);
        unsafe fn wxSize_DecTo(self_: Pin<&mut wxSize>, size: &wxSize);
        unsafe fn wxSize_DecToIfSpecified(self_: Pin<&mut wxSize>, size: &wxSize);
        unsafe fn wxSize_GetHeight(self_: &wxSize) -> i32;
        unsafe fn wxSize_GetWidth(self_: &wxSize) -> i32;
        unsafe fn wxSize_IncBy(self_: Pin<&mut wxSize>, pt: &wxPoint);
        #[rust_name = "wxSize_IncBy1"]
        unsafe fn wxSize_IncBy(self_: Pin<&mut wxSize>, size: &wxSize);
        #[rust_name = "wxSize_IncBy2"]
        unsafe fn wxSize_IncBy(self_: Pin<&mut wxSize>, dx: i32, dy: i32);
        #[rust_name = "wxSize_IncBy3"]
        unsafe fn wxSize_IncBy(self_: Pin<&mut wxSize>, d: i32);
        unsafe fn wxSize_IncTo(self_: Pin<&mut wxSize>, size: &wxSize);
        unsafe fn wxSize_IsFullySpecified(self_: &wxSize) -> bool;
        unsafe fn wxSize_Set(self_: Pin<&mut wxSize>, width: i32, height: i32);
        unsafe fn wxSize_SetDefaults(self_: Pin<&mut wxSize>, size_default: &wxSize);
        unsafe fn wxSize_SetHeight(self_: Pin<&mut wxSize>, height: i32);
        unsafe fn wxSize_SetWidth(self_: Pin<&mut wxSize>, width: i32);
        // CLASS: wxValidator
        unsafe fn wxValidator_new() -> *mut wxValidator;
        unsafe fn wxValidator_Clone(self_: &wxValidator) -> *mut wxObject;
        unsafe fn wxValidator_GetWindow(self_: &wxValidator) -> *mut wxWindow;
        unsafe fn wxValidator_SetWindow(self_: Pin<&mut wxValidator>, window: *mut wxWindow);
        unsafe fn wxValidator_TransferFromWindow(self_: Pin<&mut wxValidator>) -> bool;
        unsafe fn wxValidator_TransferToWindow(self_: Pin<&mut wxValidator>) -> bool;
        unsafe fn wxValidator_Validate(self_: Pin<&mut wxValidator>, parent: *mut wxWindow) -> bool;
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
            let other = &other.pinned::<ffi::wxObject>();
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
        unsafe { ffi::wxObject_GetClassInfo(&self.pinned::<ffi::wxObject>().as_mut()) }
    }
    fn get_ref_data(&self) -> *mut ffi::wxObjectRefData {
        unsafe { ffi::wxObject_GetRefData(&self.pinned::<ffi::wxObject>().as_mut()) }
    }
    fn is_kind_of(&self, info: *const ffi::wxClassInfo) -> bool {
        unsafe { ffi::wxObject_IsKindOf(&self.pinned::<ffi::wxObject>().as_mut(), info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        unsafe {
            let obj = &obj.pinned::<ffi::wxObject>();
            ffi::wxObject_IsSameAs(&self.pinned::<ffi::wxObject>().as_mut(), obj)
        }
    }
    fn ref_(&self, clone: &Object) {
        unsafe {
            let clone = &clone.pinned::<ffi::wxObject>();
            ffi::wxObject_Ref(self.pinned::<ffi::wxObject>().as_mut(), clone)
        }
    }
    fn set_ref_data(&self, data: *mut ffi::wxObjectRefData) {
        unsafe { ffi::wxObject_SetRefData(self.pinned::<ffi::wxObject>().as_mut(), data) }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.pinned::<ffi::wxObject>().as_mut()) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.pinned::<ffi::wxObject>().as_mut()) }
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
        unsafe { ffi::wxEvtHandler_QueueEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event) }
    }
    fn add_pending_event(&self, event: &ffi::wxEvent) {
        unsafe { ffi::wxEvtHandler_AddPendingEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event) }
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event) }
    }
    fn process_event_locally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEventLocally(self.pinned::<ffi::wxEvtHandler>().as_mut(), event) }
    }
    fn safely_process_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        unsafe { ffi::wxEvtHandler_SafelyProcessEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.pinned::<ffi::wxEvtHandler>().as_mut()) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.pinned::<ffi::wxEvtHandler>().as_mut()) }
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
        unsafe { ffi::wxEvtHandler_GetClientObject(&self.pinned::<ffi::wxEvtHandler>().as_mut()) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut ffi::wxClientData) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.pinned::<ffi::wxEvtHandler>().as_mut(), data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(&self.pinned::<ffi::wxEvtHandler>().as_mut()) }
    }
    fn get_next_handler(&self) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxEvtHandler_GetNextHandler(&self.pinned::<ffi::wxEvtHandler>().as_mut()) }
    }
    fn get_previous_handler(&self) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxEvtHandler_GetPreviousHandler(&self.pinned::<ffi::wxEvtHandler>().as_mut()) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.pinned::<ffi::wxEvtHandler>().as_mut(), enabled) }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.pinned::<ffi::wxEvtHandler>().as_mut(), handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.pinned::<ffi::wxEvtHandler>().as_mut(), handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.pinned::<ffi::wxEvtHandler>().as_mut()) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(&self.pinned::<ffi::wxEvtHandler>().as_mut()) }
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let name = &crate::ffi::NewString(name);
            Window(ffi::wxWindow_new1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocus(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusFromKeyboard(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn accepts_focus_recursively(&self) -> bool {
        unsafe { ffi::wxWindow_AcceptsFocusRecursively(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn disable_focus_from_keyboard(&self) {
        unsafe { ffi::wxWindow_DisableFocusFromKeyboard(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_focusable(&self) -> bool {
        unsafe { ffi::wxWindow_IsFocusable(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn can_accept_focus(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocus(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        unsafe { ffi::wxWindow_CanAcceptFocusFromKeyboard(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn has_focus(&self) -> bool {
        unsafe { ffi::wxWindow_HasFocus(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_can_focus(&self, can_focus: bool) {
        unsafe { ffi::wxWindow_SetCanFocus(self.pinned::<ffi::wxWindow>().as_mut(), can_focus) }
    }
    fn enable_visible_focus(&self, enable: bool) {
        unsafe { ffi::wxWindow_EnableVisibleFocus(self.pinned::<ffi::wxWindow>().as_mut(), enable) }
    }
    fn set_focus(&self) {
        unsafe { ffi::wxWindow_SetFocus(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_focus_from_kbd(&self) {
        unsafe { ffi::wxWindow_SetFocusFromKbd(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn add_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_AddChild(self.pinned::<ffi::wxWindow>().as_mut(), child)
        }
    }
    fn destroy_children(&self) -> bool {
        unsafe { ffi::wxWindow_DestroyChildren(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn find_window(&self, id: i32) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_FindWindow(&self.pinned::<ffi::wxWindow>().as_mut(), id) }
    }
    fn find_window1(&self, name: &str) -> *mut ffi::wxWindow {
        unsafe {
            let name = &crate::ffi::NewString(name);
            ffi::wxWindow_FindWindow1(&self.pinned::<ffi::wxWindow>().as_mut(), name)
        }
    }
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren1()
    fn remove_child<T: WindowMethods>(&self, child: Option<&T>) {
        unsafe {
            let child = match child {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveChild(self.pinned::<ffi::wxWindow>().as_mut(), child)
        }
    }
    fn get_grand_parent(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetGrandParent(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_next_sibling(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetNextSibling(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_parent(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetParent(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_prev_sibling(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_GetPrevSibling(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_descendant<T: WindowMethods>(&self, win: Option<&T>) -> bool {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_IsDescendant(&self.pinned::<ffi::wxWindow>().as_mut(), win)
        }
    }
    fn reparent<T: WindowMethods>(&self, new_parent: Option<&T>) -> bool {
        unsafe {
            let new_parent = match new_parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_Reparent(self.pinned::<ffi::wxWindow>().as_mut(), new_parent)
        }
    }
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        unsafe { ffi::wxWindow_AlwaysShowScrollbars(self.pinned::<ffi::wxWindow>().as_mut(), hflag, vflag) }
    }
    fn get_scroll_pos(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollPos(&self.pinned::<ffi::wxWindow>().as_mut(), orientation) }
    }
    fn get_scroll_range(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollRange(&self.pinned::<ffi::wxWindow>().as_mut(), orientation) }
    }
    fn get_scroll_thumb(&self, orientation: i32) -> i32 {
        unsafe { ffi::wxWindow_GetScrollThumb(&self.pinned::<ffi::wxWindow>().as_mut(), orientation) }
    }
    fn can_scroll(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_CanScroll(&self.pinned::<ffi::wxWindow>().as_mut(), orient) }
    }
    fn has_scrollbar(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_HasScrollbar(&self.pinned::<ffi::wxWindow>().as_mut(), orient) }
    }
    fn is_scrollbar_always_shown(&self, orient: i32) -> bool {
        unsafe { ffi::wxWindow_IsScrollbarAlwaysShown(&self.pinned::<ffi::wxWindow>().as_mut(), orient) }
    }
    fn scroll_lines(&self, lines: i32) -> bool {
        unsafe { ffi::wxWindow_ScrollLines(self.pinned::<ffi::wxWindow>().as_mut(), lines) }
    }
    fn scroll_pages(&self, pages: i32) -> bool {
        unsafe { ffi::wxWindow_ScrollPages(self.pinned::<ffi::wxWindow>().as_mut(), pages) }
    }
    fn scroll_window(&self, dx: i32, dy: i32, rect: *const ffi::wxRect) {
        unsafe { ffi::wxWindow_ScrollWindow(self.pinned::<ffi::wxWindow>().as_mut(), dx, dy, rect) }
    }
    fn line_up(&self) -> bool {
        unsafe { ffi::wxWindow_LineUp(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn line_down(&self) -> bool {
        unsafe { ffi::wxWindow_LineDown(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn page_up(&self) -> bool {
        unsafe { ffi::wxWindow_PageUp(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn page_down(&self) -> bool {
        unsafe { ffi::wxWindow_PageDown(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_scroll_pos(&self, orientation: i32, pos: i32, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollPos(self.pinned::<ffi::wxWindow>().as_mut(), orientation, pos, refresh) }
    }
    fn set_scrollbar(&self, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool) {
        unsafe { ffi::wxWindow_SetScrollbar(self.pinned::<ffi::wxWindow>().as_mut(), orientation, position, thumb_size, range, refresh) }
    }
    fn begin_repositioning_children(&self) -> bool {
        unsafe { ffi::wxWindow_BeginRepositioningChildren(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn end_repositioning_children(&self) {
        unsafe { ffi::wxWindow_EndRepositioningChildren(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn cache_best_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_CacheBestSize(&self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn client_to_window_size(&self, size: &Size) -> Size {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            Size(ffi::wxWindow_ClientToWindowSize(&self.pinned::<ffi::wxWindow>().as_mut(), size))
        }
    }
    fn window_to_client_size(&self, size: &Size) -> Size {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            Size(ffi::wxWindow_WindowToClientSize(&self.pinned::<ffi::wxWindow>().as_mut(), size))
        }
    }
    fn fit(&self) {
        unsafe { ffi::wxWindow_Fit(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn fit_inside(&self) {
        unsafe { ffi::wxWindow_FitInside(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn from_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            Size(ffi::wxWindow_FromDIP(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
        }
    }
    fn from_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            Point(ffi::wxWindow_FromDIP1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
        }
    }
    fn from_dip2(&self, d: i32) -> i32 {
        unsafe { ffi::wxWindow_FromDIP2(&self.pinned::<ffi::wxWindow>().as_mut(), d) }
    }
    fn to_dip(&self, sz: &Size) -> Size {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            Size(ffi::wxWindow_ToDIP(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
        }
    }
    fn to_dip1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            Point(ffi::wxWindow_ToDIP1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
        }
    }
    fn to_dip2(&self, d: i32) -> i32 {
        unsafe { ffi::wxWindow_ToDIP2(&self.pinned::<ffi::wxWindow>().as_mut(), d) }
    }
    fn get_best_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_best_height(&self, width: i32) -> i32 {
        unsafe { ffi::wxWindow_GetBestHeight(&self.pinned::<ffi::wxWindow>().as_mut(), width) }
    }
    fn get_best_width(&self, height: i32) -> i32 {
        unsafe { ffi::wxWindow_GetBestWidth(&self.pinned::<ffi::wxWindow>().as_mut(), height) }
    }
    fn get_client_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetClientSize(&self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn get_client_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetClientSize1(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_effective_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetEffectiveMinSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_max_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxClientSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_max_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMaxSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_min_client_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinClientSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_min_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetMinSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_min_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMinWidth(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_min_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMinHeight(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_max_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMaxWidth(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_max_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetMaxHeight(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetSize(&self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn get_size1(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetSize1(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetVirtualSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_virtual_size1(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetVirtualSize1(&self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetBestVirtualSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_content_scale_factor(&self) -> f64 {
        unsafe { ffi::wxWindow_GetContentScaleFactor(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_dpi_scale_factor(&self) -> f64 {
        unsafe { ffi::wxWindow_GetDPIScaleFactor(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_window_border_size(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetWindowBorderSize(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn inform_first_direction(&self, direction: i32, size: i32, available_other_dir: i32) -> bool {
        unsafe { ffi::wxWindow_InformFirstDirection(self.pinned::<ffi::wxWindow>().as_mut(), direction, size, available_other_dir) }
    }
    fn invalidate_best_size(&self) {
        unsafe { ffi::wxWindow_InvalidateBestSize(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn post_size_event(&self) {
        unsafe { ffi::wxWindow_PostSizeEvent(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn post_size_event_to_parent(&self) {
        unsafe { ffi::wxWindow_PostSizeEventToParent(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn send_size_event(&self, flags: i32) {
        unsafe { ffi::wxWindow_SendSizeEvent(self.pinned::<ffi::wxWindow>().as_mut(), flags) }
    }
    fn send_size_event_to_parent(&self, flags: i32) {
        unsafe { ffi::wxWindow_SendSizeEventToParent(self.pinned::<ffi::wxWindow>().as_mut(), flags) }
    }
    fn set_client_size(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetClientSize(self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn set_client_size1(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetClientSize1(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_client_size2(&self, rect: &Rect) {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            ffi::wxWindow_SetClientSize2(self.pinned::<ffi::wxWindow>().as_mut(), rect)
        }
    }
    fn set_containing_sizer(&self, sizer: *mut ffi::wxSizer) {
        unsafe { ffi::wxWindow_SetContainingSizer(self.pinned::<ffi::wxWindow>().as_mut(), sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetInitialSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_max_client_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetMaxClientSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetMaxSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_min_client_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetMinClientSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetMinSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_size(&self, x: i32, y: i32, width: i32, height: i32, size_flags: i32) {
        unsafe { ffi::wxWindow_SetSize(self.pinned::<ffi::wxWindow>().as_mut(), x, y, width, height, size_flags) }
    }
    fn set_size1(&self, rect: &Rect) {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            ffi::wxWindow_SetSize1(self.pinned::<ffi::wxWindow>().as_mut(), rect)
        }
    }
    fn set_size2(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetSize2(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn set_size3(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetSize3(self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = &min_size.pinned::<ffi::wxSize>();
            let max_size = &max_size.pinned::<ffi::wxSize>();
            let inc_size = &inc_size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetSizeHints(self.pinned::<ffi::wxWindow>().as_mut(), min_size, max_size, inc_size)
        }
    }
    fn set_size_hints1(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi::wxWindow_SetSizeHints1(self.pinned::<ffi::wxWindow>().as_mut(), min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_virtual_size(&self, width: i32, height: i32) {
        unsafe { ffi::wxWindow_SetVirtualSize(self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn set_virtual_size1(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxWindow_SetVirtualSize1(self.pinned::<ffi::wxWindow>().as_mut(), size)
        }
    }
    fn from_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_FromDIP4(pt, w))
        }
    }
    fn from_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FromDIP5(d, w)
        }
    }
    fn to_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            Point(ffi::wxWindow_ToDIP4(pt, w))
        }
    }
    fn to_dip5<T: WindowMethods>(d: i32, w: Option<&T>) -> i32 {
        unsafe {
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_ToDIP5(d, w)
        }
    }
    fn center(&self, dir: i32) {
        unsafe { ffi::wxWindow_Center(self.pinned::<ffi::wxWindow>().as_mut(), dir) }
    }
    fn center_on_parent(&self, dir: i32) {
        unsafe { ffi::wxWindow_CenterOnParent(self.pinned::<ffi::wxWindow>().as_mut(), dir) }
    }
    fn centre(&self, direction: i32) {
        unsafe { ffi::wxWindow_Centre(self.pinned::<ffi::wxWindow>().as_mut(), direction) }
    }
    fn centre_on_parent(&self, direction: i32) {
        unsafe { ffi::wxWindow_CentreOnParent(self.pinned::<ffi::wxWindow>().as_mut(), direction) }
    }
    fn get_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_GetPosition(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn get_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetPosition1(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetRect(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_screen_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_GetScreenPosition(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn get_screen_position1(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetScreenPosition1(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_screen_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetScreenRect(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxWindow_GetClientAreaOrigin(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn get_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetClientRect(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn move_(&self, x: i32, y: i32, flags: i32) {
        unsafe { ffi::wxWindow_Move(self.pinned::<ffi::wxWindow>().as_mut(), x, y, flags) }
    }
    fn move1(&self, pt: &Point, flags: i32) {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxWindow_Move1(self.pinned::<ffi::wxWindow>().as_mut(), pt, flags)
        }
    }
    fn set_position(&self, pt: &Point) {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxWindow_SetPosition(self.pinned::<ffi::wxWindow>().as_mut(), pt)
        }
    }
    fn client_to_screen(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_ClientToScreen(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn client_to_screen1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            Point(ffi::wxWindow_ClientToScreen1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
        }
    }
    fn convert_dialog_to_pixels(&self, pt: &Point) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            Point(ffi::wxWindow_ConvertDialogToPixels(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
        }
    }
    fn convert_dialog_to_pixels1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            Size(ffi::wxWindow_ConvertDialogToPixels1(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
        }
    }
    fn convert_pixels_to_dialog(&self, pt: &Point) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            Point(ffi::wxWindow_ConvertPixelsToDialog(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
        }
    }
    fn convert_pixels_to_dialog1(&self, sz: &Size) -> Size {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            Size(ffi::wxWindow_ConvertPixelsToDialog1(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
        }
    }
    fn screen_to_client(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_ScreenToClient(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn screen_to_client1(&self, pt: &Point) -> Point {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            Point(ffi::wxWindow_ScreenToClient1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
        }
    }
    fn clear_background(&self) {
        unsafe { ffi::wxWindow_ClearBackground(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn freeze(&self) {
        unsafe { ffi::wxWindow_Freeze(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn thaw(&self) {
        unsafe { ffi::wxWindow_Thaw(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_frozen(&self) -> bool {
        unsafe { ffi::wxWindow_IsFrozen(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> i32 {
        unsafe { ffi::wxWindow_GetCharHeight(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_char_width(&self) -> i32 {
        unsafe { ffi::wxWindow_GetCharWidth(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        unsafe { Size(ffi::wxWindow_GetDPI(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    // CXX_UNSUPPORTED: fn GetFont()
    // CXX_UNSUPPORTED: fn GetForegroundColour()
    fn get_text_extent(&self, string: &str, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const ffi::wxFont) {
        unsafe {
            let string = &crate::ffi::NewString(string);
            ffi::wxWindow_GetTextExtent(&self.pinned::<ffi::wxWindow>().as_mut(), string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent1(&self, string: &str) -> Size {
        unsafe {
            let string = &crate::ffi::NewString(string);
            Size(ffi::wxWindow_GetTextExtent1(&self.pinned::<ffi::wxWindow>().as_mut(), string))
        }
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        unsafe { Rect(ffi::wxWindow_GetUpdateClientRect(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn has_transparent_background(&self) -> bool {
        unsafe { ffi::wxWindow_HasTransparentBackground(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn refresh(&self, erase_background: bool, rect: *const ffi::wxRect) {
        unsafe { ffi::wxWindow_Refresh(self.pinned::<ffi::wxWindow>().as_mut(), erase_background, rect) }
    }
    fn refresh_rect(&self, rect: &Rect, erase_background: bool) {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            ffi::wxWindow_RefreshRect(self.pinned::<ffi::wxWindow>().as_mut(), rect, erase_background)
        }
    }
    fn update(&self) {
        unsafe { ffi::wxWindow_Update(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_background_colour(&self, colour: &ffi::wxColour) -> bool {
        unsafe { ffi::wxWindow_SetBackgroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour) }
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut ffi::wxString) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(&self.pinned::<ffi::wxWindow>().as_mut(), reason) }
    }
    fn set_font(&self, font: &ffi::wxFont) -> bool {
        unsafe { ffi::wxWindow_SetFont(self.pinned::<ffi::wxWindow>().as_mut(), font) }
    }
    fn set_foreground_colour(&self, colour: &ffi::wxColour) -> bool {
        unsafe { ffi::wxWindow_SetForegroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour) }
    }
    fn set_own_background_colour(&self, colour: &ffi::wxColour) {
        unsafe { ffi::wxWindow_SetOwnBackgroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour) }
    }
    fn inherits_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsBackgroundColour(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn use_bg_col(&self) -> bool {
        unsafe { ffi::wxWindow_UseBgCol(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn use_background_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseBackgroundColour(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_own_font(&self, font: &ffi::wxFont) {
        unsafe { ffi::wxWindow_SetOwnFont(self.pinned::<ffi::wxWindow>().as_mut(), font) }
    }
    fn set_own_foreground_colour(&self, colour: &ffi::wxColour) {
        unsafe { ffi::wxWindow_SetOwnForegroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour) }
    }
    fn use_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_UseForegroundColour(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn inherits_foreground_colour(&self) -> bool {
        unsafe { ffi::wxWindow_InheritsForegroundColour(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_palette(&self, pal: &ffi::wxPalette) {
        unsafe { ffi::wxWindow_SetPalette(self.pinned::<ffi::wxWindow>().as_mut(), pal) }
    }
    fn should_inherit_colours(&self) -> bool {
        unsafe { ffi::wxWindow_ShouldInheritColours(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_theme_enabled(&self, enable: bool) {
        unsafe { ffi::wxWindow_SetThemeEnabled(self.pinned::<ffi::wxWindow>().as_mut(), enable) }
    }
    fn get_theme_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_GetThemeEnabled(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxWindow_CanSetTransparent(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi::wxWindow_SetTransparent(self.pinned::<ffi::wxWindow>().as_mut(), alpha) }
    }
    fn get_event_handler(&self) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxWindow_GetEventHandler(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn handle_as_navigation_key(&self, event: &ffi::wxKeyEvent) -> bool {
        unsafe { ffi::wxWindow_HandleAsNavigationKey(self.pinned::<ffi::wxWindow>().as_mut(), event) }
    }
    fn handle_window_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        unsafe { ffi::wxWindow_HandleWindowEvent(&self.pinned::<ffi::wxWindow>().as_mut(), event) }
    }
    fn process_window_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEvent(self.pinned::<ffi::wxWindow>().as_mut(), event) }
    }
    fn process_window_event_locally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        unsafe { ffi::wxWindow_ProcessWindowEventLocally(self.pinned::<ffi::wxWindow>().as_mut(), event) }
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut ffi::wxEvtHandler {
        unsafe { ffi::wxWindow_PopEventHandler(self.pinned::<ffi::wxWindow>().as_mut(), delete_handler) }
    }
    fn push_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_PushEventHandler(self.pinned::<ffi::wxWindow>().as_mut(), handler)
        }
    }
    fn remove_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) -> bool {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_RemoveEventHandler(self.pinned::<ffi::wxWindow>().as_mut(), handler)
        }
    }
    fn set_event_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetEventHandler(self.pinned::<ffi::wxWindow>().as_mut(), handler)
        }
    }
    fn set_next_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetNextHandler(self.pinned::<ffi::wxWindow>().as_mut(), handler)
        }
    }
    fn set_previous_handler<T: EvtHandlerMethods>(&self, handler: Option<&T>) {
        unsafe {
            let handler = match handler {
                Some(r) => Pin::<&mut ffi::wxEvtHandler>::into_inner_unchecked(r.pinned::<ffi::wxEvtHandler>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_SetPreviousHandler(self.pinned::<ffi::wxWindow>().as_mut(), handler)
        }
    }
    fn get_extra_style(&self) -> i32 {
        unsafe { ffi::wxWindow_GetExtraStyle(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_window_style_flag(&self) -> i32 {
        unsafe { ffi::wxWindow_GetWindowStyleFlag(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_window_style(&self) -> i32 {
        unsafe { ffi::wxWindow_GetWindowStyle(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn has_extra_style(&self, ex_flag: i32) -> bool {
        unsafe { ffi::wxWindow_HasExtraStyle(&self.pinned::<ffi::wxWindow>().as_mut(), ex_flag) }
    }
    fn has_flag(&self, flag: i32) -> bool {
        unsafe { ffi::wxWindow_HasFlag(&self.pinned::<ffi::wxWindow>().as_mut(), flag) }
    }
    fn set_extra_style(&self, ex_style: i32) {
        unsafe { ffi::wxWindow_SetExtraStyle(self.pinned::<ffi::wxWindow>().as_mut(), ex_style) }
    }
    fn set_window_style_flag(&self, style: i32) {
        unsafe { ffi::wxWindow_SetWindowStyleFlag(self.pinned::<ffi::wxWindow>().as_mut(), style) }
    }
    fn set_window_style(&self, style: i32) {
        unsafe { ffi::wxWindow_SetWindowStyle(self.pinned::<ffi::wxWindow>().as_mut(), style) }
    }
    fn toggle_window_style(&self, flag: i32) -> bool {
        unsafe { ffi::wxWindow_ToggleWindowStyle(self.pinned::<ffi::wxWindow>().as_mut(), flag) }
    }
    fn move_after_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveAfterInTabOrder(self.pinned::<ffi::wxWindow>().as_mut(), win)
        }
    }
    fn move_before_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_MoveBeforeInTabOrder(self.pinned::<ffi::wxWindow>().as_mut(), win)
        }
    }
    fn navigate(&self, flags: i32) -> bool {
        unsafe { ffi::wxWindow_Navigate(self.pinned::<ffi::wxWindow>().as_mut(), flags) }
    }
    fn navigate_in(&self, flags: i32) -> bool {
        unsafe { ffi::wxWindow_NavigateIn(self.pinned::<ffi::wxWindow>().as_mut(), flags) }
    }
    fn lower(&self) {
        unsafe { ffi::wxWindow_Lower(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn raise(&self) {
        unsafe { ffi::wxWindow_Raise(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxWindow_Hide(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsEnabled(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_exposed(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_IsExposed(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn is_exposed1(&self, pt: Pin<&mut ffi::wxPoint>) -> bool {
        unsafe { ffi::wxWindow_IsExposed1(&self.pinned::<ffi::wxWindow>().as_mut(), pt) }
    }
    fn is_exposed2(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        unsafe { ffi::wxWindow_IsExposed2(&self.pinned::<ffi::wxWindow>().as_mut(), x, y, w, h) }
    }
    fn is_exposed3(&self, rect: Pin<&mut ffi::wxRect>) -> bool {
        unsafe { ffi::wxWindow_IsExposed3(&self.pinned::<ffi::wxWindow>().as_mut(), rect) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxWindow_IsShown(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_shown_on_screen(&self) -> bool {
        unsafe { ffi::wxWindow_IsShownOnScreen(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn disable(&self) -> bool {
        unsafe { ffi::wxWindow_Disable(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn enable(&self, enable: bool) -> bool {
        unsafe { ffi::wxWindow_Enable(self.pinned::<ffi::wxWindow>().as_mut(), enable) }
    }
    fn show(&self, show: bool) -> bool {
        unsafe { ffi::wxWindow_Show(self.pinned::<ffi::wxWindow>().as_mut(), show) }
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetHelpText(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn set_help_text(&self, help_text: &str) {
        unsafe {
            let help_text = &crate::ffi::NewString(help_text);
            ffi::wxWindow_SetHelpText(self.pinned::<ffi::wxWindow>().as_mut(), help_text)
        }
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut ffi::wxToolTip {
        unsafe { ffi::wxWindow_GetToolTip(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_tool_tip_text(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetToolTipText(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    fn set_tool_tip(&self, tip_string: &str) {
        unsafe {
            let tip_string = &crate::ffi::NewString(tip_string);
            ffi::wxWindow_SetToolTip(self.pinned::<ffi::wxWindow>().as_mut(), tip_string)
        }
    }
    fn set_tool_tip1(&self, tip: *mut ffi::wxToolTip) {
        unsafe { ffi::wxWindow_SetToolTip1(self.pinned::<ffi::wxWindow>().as_mut(), tip) }
    }
    fn unset_tool_tip(&self) {
        unsafe { ffi::wxWindow_UnsetToolTip(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_popup_menu_selection_from_user(&self, menu: Pin<&mut ffi::wxMenu>, pos: &Point) -> i32 {
        unsafe {
            let pos = &pos.pinned::<ffi::wxPoint>();
            ffi::wxWindow_GetPopupMenuSelectionFromUser(self.pinned::<ffi::wxWindow>().as_mut(), menu, pos)
        }
    }
    fn get_popup_menu_selection_from_user1(&self, menu: Pin<&mut ffi::wxMenu>, x: i32, y: i32) -> i32 {
        unsafe { ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.pinned::<ffi::wxWindow>().as_mut(), menu, x, y) }
    }
    fn popup_menu(&self, menu: *mut ffi::wxMenu, pos: &Point) -> bool {
        unsafe {
            let pos = &pos.pinned::<ffi::wxPoint>();
            ffi::wxWindow_PopupMenu(self.pinned::<ffi::wxWindow>().as_mut(), menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut ffi::wxMenu, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_PopupMenu1(self.pinned::<ffi::wxWindow>().as_mut(), menu, x, y) }
    }
    fn get_validator(&self) -> *mut ffi::wxValidator {
        unsafe { ffi::wxWindow_GetValidator(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_validator(&self, validator: &Validator) {
        unsafe {
            let validator = &validator.pinned::<ffi::wxValidator>();
            ffi::wxWindow_SetValidator(self.pinned::<ffi::wxWindow>().as_mut(), validator)
        }
    }
    fn transfer_data_from_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataFromWindow(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn transfer_data_to_window(&self) -> bool {
        unsafe { ffi::wxWindow_TransferDataToWindow(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn validate(&self) -> bool {
        unsafe { ffi::wxWindow_Validate(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_id(&self) -> i32 {
        unsafe { ffi::wxWindow_GetId(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetLabel(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: i32, width: i32, width_total: i32) -> i32 {
        unsafe { ffi::wxWindow_AdjustForLayoutDirection(&self.pinned::<ffi::wxWindow>().as_mut(), x, width, width_total) }
    }
    fn get_name(&self) -> WxString {
        unsafe { WxString(ffi::wxWindow_GetName(&self.pinned::<ffi::wxWindow>().as_mut())) }
    }
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: i32) {
        unsafe { ffi::wxWindow_SetId(self.pinned::<ffi::wxWindow>().as_mut(), winid) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = &crate::ffi::NewString(label);
            ffi::wxWindow_SetLabel(self.pinned::<ffi::wxWindow>().as_mut(), label)
        }
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        unsafe {
            let name = &crate::ffi::NewString(name);
            ffi::wxWindow_SetName(self.pinned::<ffi::wxWindow>().as_mut(), name)
        }
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut ffi::wxAcceleratorTable {
        unsafe { ffi::wxWindow_GetAcceleratorTable(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: &ffi::wxAcceleratorTable) {
        unsafe { ffi::wxWindow_SetAcceleratorTable(self.pinned::<ffi::wxWindow>().as_mut(), accel) }
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        unsafe { ffi::wxWindow_Close(self.pinned::<ffi::wxWindow>().as_mut(), force) }
    }
    fn destroy(&self) -> bool {
        unsafe { ffi::wxWindow_Destroy(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_being_deleted(&self) -> bool {
        unsafe { ffi::wxWindow_IsBeingDeleted(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_drop_target(&self) -> *mut ffi::wxDropTarget {
        unsafe { ffi::wxWindow_GetDropTarget(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_drop_target(&self, target: *mut ffi::wxDropTarget) {
        unsafe { ffi::wxWindow_SetDropTarget(self.pinned::<ffi::wxWindow>().as_mut(), target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        unsafe { ffi::wxWindow_DragAcceptFiles(self.pinned::<ffi::wxWindow>().as_mut(), accept) }
    }
    fn get_containing_sizer(&self) -> *mut ffi::wxSizer {
        unsafe { ffi::wxWindow_GetContainingSizer(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_sizer(&self) -> *mut ffi::wxSizer {
        unsafe { ffi::wxWindow_GetSizer(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_sizer(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizer(self.pinned::<ffi::wxWindow>().as_mut(), sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizerAndFit(self.pinned::<ffi::wxWindow>().as_mut(), sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut ffi::wxLayoutConstraints {
        unsafe { ffi::wxWindow_GetConstraints(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_constraints(&self, constraints: *mut ffi::wxLayoutConstraints) {
        unsafe { ffi::wxWindow_SetConstraints(self.pinned::<ffi::wxWindow>().as_mut(), constraints) }
    }
    fn layout(&self) -> bool {
        unsafe { ffi::wxWindow_Layout(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        unsafe { ffi::wxWindow_SetAutoLayout(self.pinned::<ffi::wxWindow>().as_mut(), auto_layout) }
    }
    fn get_auto_layout(&self) -> bool {
        unsafe { ffi::wxWindow_GetAutoLayout(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn capture_mouse(&self) {
        unsafe { ffi::wxWindow_CaptureMouse(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn get_caret(&self) -> *mut ffi::wxCaret {
        unsafe { ffi::wxWindow_GetCaret(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        unsafe { ffi::wxWindow_HasCapture(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn release_mouse(&self) {
        unsafe { ffi::wxWindow_ReleaseMouse(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_caret(&self, caret: *mut ffi::wxCaret) {
        unsafe { ffi::wxWindow_SetCaret(self.pinned::<ffi::wxWindow>().as_mut(), caret) }
    }
    fn set_cursor(&self, cursor: &ffi::wxCursor) -> bool {
        unsafe { ffi::wxWindow_SetCursor(self.pinned::<ffi::wxWindow>().as_mut(), cursor) }
    }
    fn warp_pointer(&self, x: i32, y: i32) {
        unsafe { ffi::wxWindow_WarpPointer(self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn enable_touch_events(&self, events_mask: i32) -> bool {
        unsafe { ffi::wxWindow_EnableTouchEvents(self.pinned::<ffi::wxWindow>().as_mut(), events_mask) }
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest1()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: Pin<&mut ffi::wxUpdateUIEvent>) {
        unsafe { ffi::wxWindow_DoUpdateWindowUI(self.pinned::<ffi::wxWindow>().as_mut(), event) }
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        unsafe { ffi::wxWindow_HasMultiplePages(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn inherit_attributes(&self) {
        unsafe { ffi::wxWindow_InheritAttributes(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn init_dialog(&self) {
        unsafe { ffi::wxWindow_InitDialog(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_double_buffered(&self) -> bool {
        unsafe { ffi::wxWindow_IsDoubleBuffered(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn set_double_buffered(&self, on: bool) {
        unsafe { ffi::wxWindow_SetDoubleBuffered(self.pinned::<ffi::wxWindow>().as_mut(), on) }
    }
    fn is_retained(&self) -> bool {
        unsafe { ffi::wxWindow_IsRetained(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_this_enabled(&self) -> bool {
        unsafe { ffi::wxWindow_IsThisEnabled(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn is_top_level(&self) -> bool {
        unsafe { ffi::wxWindow_IsTopLevel(&self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn on_internal_idle(&self) {
        unsafe { ffi::wxWindow_OnInternalIdle(self.pinned::<ffi::wxWindow>().as_mut()) }
    }
    fn send_idle_events(&self, event: Pin<&mut ffi::wxIdleEvent>) -> bool {
        unsafe { ffi::wxWindow_SendIdleEvents(self.pinned::<ffi::wxWindow>().as_mut(), event) }
    }
    fn register_hot_key(&self, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool {
        unsafe { ffi::wxWindow_RegisterHotKey(self.pinned::<ffi::wxWindow>().as_mut(), hotkey_id, modifiers, virtual_key_code) }
    }
    fn unregister_hot_key(&self, hotkey_id: i32) -> bool {
        unsafe { ffi::wxWindow_UnregisterHotKey(self.pinned::<ffi::wxWindow>().as_mut(), hotkey_id) }
    }
    fn update_window_ui(&self, flags: i32) {
        unsafe { ffi::wxWindow_UpdateWindowUI(self.pinned::<ffi::wxWindow>().as_mut(), flags) }
    }
    // CXX_UNSUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> *mut ffi::wxWindow {
        unsafe { ffi::wxWindow_FindFocus() }
    }
    fn find_window_by_id<T: WindowMethods>(id: i32, parent: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowById(id, parent)
        }
    }
    fn find_window_by_label<T: WindowMethods>(label: &str, parent: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let label = &crate::ffi::NewString(label);
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxWindow_FindWindowByLabel(label, parent)
        }
    }
    fn find_window_by_name<T: WindowMethods>(name: &str, parent: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let name = &crate::ffi::NewString(name);
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let name = &crate::ffi::NewString(name);
            ffi::wxWindow_Create(self.pinned::<ffi::wxWindow>().as_mut(), parent, id, pos, size, style, name)
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
            ffi::wxControl_Create(self.pinned::<ffi::wxControl>().as_mut(), parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: Pin<&mut ffi::wxCommandEvent>) {
        unsafe { ffi::wxControl_Command(self.pinned::<ffi::wxControl>().as_mut(), event) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxControl_GetLabel(&self.pinned::<ffi::wxControl>().as_mut())) }
    }
    fn get_label_text(&self) -> WxString {
        unsafe { WxString(ffi::wxControl_GetLabelText(&self.pinned::<ffi::wxControl>().as_mut())) }
    }
    fn get_size_from_text_size(&self, xlen: i32, ylen: i32) -> Size {
        unsafe { Size(ffi::wxControl_GetSizeFromTextSize(&self.pinned::<ffi::wxControl>().as_mut(), xlen, ylen)) }
    }
    fn get_size_from_text_size1(&self, tsize: &Size) -> Size {
        unsafe {
            let tsize = &tsize.pinned::<ffi::wxSize>();
            Size(ffi::wxControl_GetSizeFromTextSize1(&self.pinned::<ffi::wxControl>().as_mut(), tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = &crate::ffi::NewString(text);
            Size(ffi::wxControl_GetSizeFromText(&self.pinned::<ffi::wxControl>().as_mut(), text))
        }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = &crate::ffi::NewString(label);
            ffi::wxControl_SetLabel(self.pinned::<ffi::wxControl>().as_mut(), label)
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = &crate::ffi::NewString(text);
            ffi::wxControl_SetLabelText(self.pinned::<ffi::wxControl>().as_mut(), text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = &crate::ffi::NewString(markup);
            ffi::wxControl_SetLabelMarkup(self.pinned::<ffi::wxControl>().as_mut(), markup)
        }
    }
    fn get_label_text1(label: &str) -> WxString {
        unsafe {
            let label = &crate::ffi::NewString(label);
            WxString(ffi::wxControl_GetLabelText1(label))
        }
    }
    fn remove_mnemonics(str: &str) -> WxString {
        unsafe {
            let str = &crate::ffi::NewString(str);
            WxString(ffi::wxControl_RemoveMnemonics(str))
        }
    }
    fn escape_mnemonics(text: &str) -> WxString {
        unsafe {
            let text = &crate::ffi::NewString(text);
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
    fn set_bitmap_current(&self, bitmap: &ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapCurrent(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap) }
    }
    fn set_bitmap_disabled(&self, bitmap: &ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapDisabled(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap) }
    }
    fn set_bitmap_focus(&self, bitmap: &ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapFocus(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap) }
    }
    fn set_bitmap_label(&self, bitmap: &ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapLabel(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap) }
    }
    fn set_bitmap_pressed(&self, bitmap: &ffi::wxBitmap) {
        unsafe { ffi::wxAnyButton_SetBitmapPressed(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap) }
    }
    fn get_bitmap_margins(&self) -> Size {
        unsafe { Size(ffi::wxAnyButton_GetBitmapMargins(self.pinned::<ffi::wxAnyButton>().as_mut())) }
    }
    fn set_bitmap_margins(&self, x: i32, y: i32) {
        unsafe { ffi::wxAnyButton_SetBitmapMargins(self.pinned::<ffi::wxAnyButton>().as_mut(), x, y) }
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        unsafe {
            let sz = &sz.pinned::<ffi::wxSize>();
            ffi::wxAnyButton_SetBitmapMargins1(self.pinned::<ffi::wxAnyButton>().as_mut(), sz)
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let label = &crate::ffi::NewString(label);
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let label = &crate::ffi::NewString(label);
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
            ffi::wxButton_Create(self.pinned::<ffi::wxButton>().as_mut(), parent, id, label, pos, size, style, validator, name)
        }
    }
    fn get_auth_needed(&self) -> bool {
        unsafe { ffi::wxButton_GetAuthNeeded(&self.pinned::<ffi::wxButton>().as_mut()) }
    }
    fn get_label(&self) -> WxString {
        unsafe { WxString(ffi::wxButton_GetLabel(&self.pinned::<ffi::wxButton>().as_mut())) }
    }
    fn set_auth_needed(&self, needed: bool) {
        unsafe { ffi::wxButton_SetAuthNeeded(self.pinned::<ffi::wxButton>().as_mut(), needed) }
    }
    fn set_default(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxButton_SetDefault(self.pinned::<ffi::wxButton>().as_mut()) }
    }
    fn set_label(&self, label: &str) {
        unsafe {
            let label = &crate::ffi::NewString(label);
            ffi::wxButton_SetLabel(self.pinned::<ffi::wxButton>().as_mut(), label)
        }
    }
    fn get_default_size<T: WindowMethods>(win: Option<&T>) -> Size {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
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
    fn set_shape(&self, region: &ffi::wxRegion) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape(self.pinned::<ffi::wxNonOwnedWindow>().as_mut(), region) }
    }
    fn set_shape1(&self, path: &ffi::wxGraphicsPath) -> bool {
        unsafe { ffi::wxNonOwnedWindow_SetShape1(self.pinned::<ffi::wxNonOwnedWindow>().as_mut(), path) }
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let name = &crate::ffi::NewString(name);
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let name = &crate::ffi::NewString(name);
            ffi::wxTopLevelWindow_Create(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), parent, id, title, pos, size, style, name)
        }
    }
    fn can_set_transparent(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_CanSetTransparent(self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn center_on_screen(&self, direction: i32) {
        unsafe { ffi::wxTopLevelWindow_CenterOnScreen(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), direction) }
    }
    fn centre_on_screen(&self, direction: i32) {
        unsafe { ffi::wxTopLevelWindow_CentreOnScreen(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), direction) }
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableCloseButton(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable) }
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMaximizeButton(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable) }
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableMinimizeButton(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable) }
    }
    fn get_default_item(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxTopLevelWindow_GetDefaultItem(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    // CXX_UNSUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> WxString {
        unsafe { WxString(ffi::wxTopLevelWindow_GetTitle(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())) }
    }
    fn iconize(&self, iconize: bool) {
        unsafe { ffi::wxTopLevelWindow_Iconize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), iconize) }
    }
    fn is_active(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsActive(self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn is_always_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsAlwaysMaximized(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn is_full_screen(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsFullScreen(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn is_iconized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsIconized(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn is_maximized(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_IsMaximized(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn layout(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_Layout(self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn maximize(&self, maximize: bool) {
        unsafe { ffi::wxTopLevelWindow_Maximize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), maximize) }
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: i32) {
        unsafe { ffi::wxTopLevelWindow_RequestUserAttention(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), flags) }
    }
    fn restore(&self) {
        unsafe { ffi::wxTopLevelWindow_Restore(self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetDefaultItem(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), win)
        }
    }
    fn set_tmp_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxTopLevelWindow_SetTmpDefaultItem(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), win)
        }
    }
    fn get_tmp_default_item(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxTopLevelWindow_GetTmpDefaultItem(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn set_icon(&self, icon: &ffi::wxIcon) {
        unsafe { ffi::wxTopLevelWindow_SetIcon(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), icon) }
    }
    fn set_icons(&self, icons: &ffi::wxIconBundle) {
        unsafe { ffi::wxTopLevelWindow_SetIcons(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), icons) }
    }
    fn set_max_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxTopLevelWindow_SetMaxSize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), size)
        }
    }
    fn set_min_size(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxTopLevelWindow_SetMinSize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), size)
        }
    }
    fn set_size_hints(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        unsafe { ffi::wxTopLevelWindow_SetSizeHints(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), min_w, min_h, max_w, max_h, inc_w, inc_h) }
    }
    fn set_size_hints1(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        unsafe {
            let min_size = &min_size.pinned::<ffi::wxSize>();
            let max_size = &max_size.pinned::<ffi::wxSize>();
            let inc_size = &inc_size.pinned::<ffi::wxSize>();
            ffi::wxTopLevelWindow_SetSizeHints1(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), min_size, max_size, inc_size)
        }
    }
    fn set_title(&self, title: &str) {
        unsafe {
            let title = &crate::ffi::NewString(title);
            ffi::wxTopLevelWindow_SetTitle(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), title)
        }
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        unsafe { ffi::wxTopLevelWindow_SetTransparent(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), alpha) }
    }
    fn should_prevent_app_exit(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShouldPreventAppExit(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn osx_set_modified(&self, modified: bool) {
        unsafe { ffi::wxTopLevelWindow_OSXSetModified(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), modified) }
    }
    fn osx_is_modified(&self) -> bool {
        unsafe { ffi::wxTopLevelWindow_OSXIsModified(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn set_represented_filename(&self, filename: &str) {
        unsafe {
            let filename = &crate::ffi::NewString(filename);
            ffi::wxTopLevelWindow_SetRepresentedFilename(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), filename)
        }
    }
    fn show_without_activating(&self) {
        unsafe { ffi::wxTopLevelWindow_ShowWithoutActivating(self.pinned::<ffi::wxTopLevelWindow>().as_mut()) }
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        unsafe { ffi::wxTopLevelWindow_EnableFullScreenView(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable) }
    }
    fn show_full_screen(&self, show: bool, style: i32) -> bool {
        unsafe { ffi::wxTopLevelWindow_ShowFullScreen(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), show, style) }
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
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let name = &crate::ffi::NewString(name);
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
        unsafe { ffi::wxFrame_Centre(self.pinned::<ffi::wxFrame>().as_mut(), direction) }
    }
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            let name = &crate::ffi::NewString(name);
            ffi::wxFrame_Create(self.pinned::<ffi::wxFrame>().as_mut(), parent, id, title, pos, size, style, name)
        }
    }
    fn create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut ffi::wxStatusBar {
        unsafe {
            let name = &crate::ffi::NewString(name);
            ffi::wxFrame_CreateStatusBar(self.pinned::<ffi::wxFrame>().as_mut(), number, style, id, name)
        }
    }
    fn create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut ffi::wxToolBar {
        unsafe {
            let name = &crate::ffi::NewString(name);
            ffi::wxFrame_CreateToolBar(self.pinned::<ffi::wxFrame>().as_mut(), style, id, name)
        }
    }
    fn do_give_help(&self, text: &str, show: bool) {
        unsafe {
            let text = &crate::ffi::NewString(text);
            ffi::wxFrame_DoGiveHelp(self.pinned::<ffi::wxFrame>().as_mut(), text, show)
        }
    }
    fn get_client_area_origin(&self) -> Point {
        unsafe { Point(ffi::wxFrame_GetClientAreaOrigin(&self.pinned::<ffi::wxFrame>().as_mut())) }
    }
    fn get_menu_bar(&self) -> *mut ffi::wxMenuBar {
        unsafe { ffi::wxFrame_GetMenuBar(&self.pinned::<ffi::wxFrame>().as_mut()) }
    }
    fn get_status_bar(&self) -> *mut ffi::wxStatusBar {
        unsafe { ffi::wxFrame_GetStatusBar(&self.pinned::<ffi::wxFrame>().as_mut()) }
    }
    fn get_status_bar_pane(&self) -> i32 {
        unsafe { ffi::wxFrame_GetStatusBarPane(&self.pinned::<ffi::wxFrame>().as_mut()) }
    }
    fn get_tool_bar(&self) -> *mut ffi::wxToolBar {
        unsafe { ffi::wxFrame_GetToolBar(&self.pinned::<ffi::wxFrame>().as_mut()) }
    }
    fn on_create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut ffi::wxStatusBar {
        unsafe {
            let name = &crate::ffi::NewString(name);
            ffi::wxFrame_OnCreateStatusBar(self.pinned::<ffi::wxFrame>().as_mut(), number, style, id, name)
        }
    }
    fn on_create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut ffi::wxToolBar {
        unsafe {
            let name = &crate::ffi::NewString(name);
            ffi::wxFrame_OnCreateToolBar(self.pinned::<ffi::wxFrame>().as_mut(), style, id, name)
        }
    }
    fn process_command(&self, id: i32) -> bool {
        unsafe { ffi::wxFrame_ProcessCommand(self.pinned::<ffi::wxFrame>().as_mut(), id) }
    }
    fn set_menu_bar(&self, menu_bar: *mut ffi::wxMenuBar) {
        unsafe { ffi::wxFrame_SetMenuBar(self.pinned::<ffi::wxFrame>().as_mut(), menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut ffi::wxStatusBar) {
        unsafe { ffi::wxFrame_SetStatusBar(self.pinned::<ffi::wxFrame>().as_mut(), status_bar) }
    }
    fn set_status_bar_pane(&self, n: i32) {
        unsafe { ffi::wxFrame_SetStatusBarPane(self.pinned::<ffi::wxFrame>().as_mut(), n) }
    }
    fn set_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = &crate::ffi::NewString(text);
            ffi::wxFrame_SetStatusText(self.pinned::<ffi::wxFrame>().as_mut(), text, number)
        }
    }
    fn set_status_widths(&self, n: i32, widths_field: *const i32) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.pinned::<ffi::wxFrame>().as_mut(), n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut ffi::wxToolBar) {
        unsafe { ffi::wxFrame_SetToolBar(self.pinned::<ffi::wxFrame>().as_mut(), tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: i32) {
        unsafe {
            let text = &crate::ffi::NewString(text);
            ffi::wxFrame_PushStatusText(self.pinned::<ffi::wxFrame>().as_mut(), text, number)
        }
    }
    fn pop_status_text(&self, number: i32) {
        unsafe { ffi::wxFrame_PopStatusText(self.pinned::<ffi::wxFrame>().as_mut(), number) }
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
    pub fn new2(pt: &ffi::wxRealPoint) -> Point {
        unsafe { Point(ffi::wxPoint_new2(pt)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxPoint_IsFullySpecified(&self.pinned::<ffi::wxPoint>().as_mut()) }
    }
    fn set_defaults(&self, pt: &Point) {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxPoint_SetDefaults(self.pinned::<ffi::wxPoint>().as_mut(), pt)
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
            let top_left = &top_left.pinned::<ffi::wxPoint>();
            let bottom_right = &bottom_right.pinned::<ffi::wxPoint>();
            Rect(ffi::wxRect_new2(top_left, bottom_right))
        }
    }
    pub fn new3(pos: &Point, size: &Size) -> Rect {
        unsafe {
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
            Rect(ffi::wxRect_new3(pos, size))
        }
    }
    pub fn new4(size: &Size) -> Rect {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
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
            let r = &r.pinned::<ffi::wxRect>();
            Rect(ffi::wxRect_CentreIn(&self.pinned::<ffi::wxRect>().as_mut(), r, dir))
        }
    }
    fn center_in(&self, r: &Rect, dir: i32) -> Rect {
        unsafe {
            let r = &r.pinned::<ffi::wxRect>();
            Rect(ffi::wxRect_CenterIn(&self.pinned::<ffi::wxRect>().as_mut(), r, dir))
        }
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        unsafe { ffi::wxRect_Contains(&self.pinned::<ffi::wxRect>().as_mut(), x, y) }
    }
    fn contains1(&self, pt: &Point) -> bool {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxRect_Contains1(&self.pinned::<ffi::wxRect>().as_mut(), pt)
        }
    }
    fn contains2(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            ffi::wxRect_Contains2(&self.pinned::<ffi::wxRect>().as_mut(), rect)
        }
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_Deflate3(&self.pinned::<ffi::wxRect>().as_mut(), dx, dy)) }
    }
    fn get_bottom(&self) -> i32 {
        unsafe { ffi::wxRect_GetBottom(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_bottom_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomLeft(&self.pinned::<ffi::wxRect>().as_mut())) }
    }
    fn get_bottom_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetBottomRight(&self.pinned::<ffi::wxRect>().as_mut())) }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi::wxRect_GetHeight(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_left(&self) -> i32 {
        unsafe { ffi::wxRect_GetLeft(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetPosition(&self.pinned::<ffi::wxRect>().as_mut())) }
    }
    fn get_right(&self) -> i32 {
        unsafe { ffi::wxRect_GetRight(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size(ffi::wxRect_GetSize(&self.pinned::<ffi::wxRect>().as_mut())) }
    }
    fn get_top(&self) -> i32 {
        unsafe { ffi::wxRect_GetTop(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_top_left(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopLeft(&self.pinned::<ffi::wxRect>().as_mut())) }
    }
    fn get_top_right(&self) -> Point {
        unsafe { Point(ffi::wxRect_GetTopRight(&self.pinned::<ffi::wxRect>().as_mut())) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi::wxRect_GetWidth(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_x(&self) -> i32 {
        unsafe { ffi::wxRect_GetX(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn get_y(&self) -> i32 {
        unsafe { ffi::wxRect_GetY(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate3(&self, dx: i32, dy: i32) -> Rect {
        unsafe { Rect(ffi::wxRect_Inflate3(&self.pinned::<ffi::wxRect>().as_mut(), dx, dy)) }
    }
    // BLOCKED: fn Intersect()
    fn intersect1(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            Rect(ffi::wxRect_Intersect1(&self.pinned::<ffi::wxRect>().as_mut(), rect))
        }
    }
    fn intersects(&self, rect: &Rect) -> bool {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            ffi::wxRect_Intersects(&self.pinned::<ffi::wxRect>().as_mut(), rect)
        }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxRect_IsEmpty(&self.pinned::<ffi::wxRect>().as_mut()) }
    }
    fn offset(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxRect_Offset(self.pinned::<ffi::wxRect>().as_mut(), dx, dy) }
    }
    fn offset1(&self, pt: &Point) {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxRect_Offset1(self.pinned::<ffi::wxRect>().as_mut(), pt)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi::wxRect_SetHeight(self.pinned::<ffi::wxRect>().as_mut(), height) }
    }
    fn set_position(&self, pos: &Point) {
        unsafe {
            let pos = &pos.pinned::<ffi::wxPoint>();
            ffi::wxRect_SetPosition(self.pinned::<ffi::wxRect>().as_mut(), pos)
        }
    }
    fn set_size(&self, s: &Size) {
        unsafe {
            let s = &s.pinned::<ffi::wxSize>();
            ffi::wxRect_SetSize(self.pinned::<ffi::wxRect>().as_mut(), s)
        }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi::wxRect_SetWidth(self.pinned::<ffi::wxRect>().as_mut(), width) }
    }
    fn set_x(&self, x: i32) {
        unsafe { ffi::wxRect_SetX(self.pinned::<ffi::wxRect>().as_mut(), x) }
    }
    fn set_y(&self, y: i32) {
        unsafe { ffi::wxRect_SetY(self.pinned::<ffi::wxRect>().as_mut(), y) }
    }
    fn set_left(&self, left: i32) {
        unsafe { ffi::wxRect_SetLeft(self.pinned::<ffi::wxRect>().as_mut(), left) }
    }
    fn set_right(&self, right: i32) {
        unsafe { ffi::wxRect_SetRight(self.pinned::<ffi::wxRect>().as_mut(), right) }
    }
    fn set_top(&self, top: i32) {
        unsafe { ffi::wxRect_SetTop(self.pinned::<ffi::wxRect>().as_mut(), top) }
    }
    fn set_bottom(&self, bottom: i32) {
        unsafe { ffi::wxRect_SetBottom(self.pinned::<ffi::wxRect>().as_mut(), bottom) }
    }
    fn set_top_left(&self, p: &Point) {
        unsafe {
            let p = &p.pinned::<ffi::wxPoint>();
            ffi::wxRect_SetTopLeft(self.pinned::<ffi::wxRect>().as_mut(), p)
        }
    }
    fn set_bottom_right(&self, p: &Point) {
        unsafe {
            let p = &p.pinned::<ffi::wxPoint>();
            ffi::wxRect_SetBottomRight(self.pinned::<ffi::wxRect>().as_mut(), p)
        }
    }
    fn set_top_right(&self, p: &Point) {
        unsafe {
            let p = &p.pinned::<ffi::wxPoint>();
            ffi::wxRect_SetTopRight(self.pinned::<ffi::wxRect>().as_mut(), p)
        }
    }
    fn set_bottom_left(&self, p: &Point) {
        unsafe {
            let p = &p.pinned::<ffi::wxPoint>();
            ffi::wxRect_SetBottomLeft(self.pinned::<ffi::wxRect>().as_mut(), p)
        }
    }
    fn union(&self, rect: &Rect) -> Rect {
        unsafe {
            let rect = &rect.pinned::<ffi::wxRect>();
            Rect(ffi::wxRect_Union(&self.pinned::<ffi::wxRect>().as_mut(), rect))
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
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxSize_DecBy(self.pinned::<ffi::wxSize>().as_mut(), pt)
        }
    }
    fn dec_by1(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxSize_DecBy1(self.pinned::<ffi::wxSize>().as_mut(), size)
        }
    }
    fn dec_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxSize_DecBy2(self.pinned::<ffi::wxSize>().as_mut(), dx, dy) }
    }
    fn dec_by3(&self, d: i32) {
        unsafe { ffi::wxSize_DecBy3(self.pinned::<ffi::wxSize>().as_mut(), d) }
    }
    fn dec_to(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxSize_DecTo(self.pinned::<ffi::wxSize>().as_mut(), size)
        }
    }
    fn dec_to_if_specified(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxSize_DecToIfSpecified(self.pinned::<ffi::wxSize>().as_mut(), size)
        }
    }
    fn get_height(&self) -> i32 {
        unsafe { ffi::wxSize_GetHeight(&self.pinned::<ffi::wxSize>().as_mut()) }
    }
    fn get_width(&self) -> i32 {
        unsafe { ffi::wxSize_GetWidth(&self.pinned::<ffi::wxSize>().as_mut()) }
    }
    fn inc_by(&self, pt: &Point) {
        unsafe {
            let pt = &pt.pinned::<ffi::wxPoint>();
            ffi::wxSize_IncBy(self.pinned::<ffi::wxSize>().as_mut(), pt)
        }
    }
    fn inc_by1(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxSize_IncBy1(self.pinned::<ffi::wxSize>().as_mut(), size)
        }
    }
    fn inc_by2(&self, dx: i32, dy: i32) {
        unsafe { ffi::wxSize_IncBy2(self.pinned::<ffi::wxSize>().as_mut(), dx, dy) }
    }
    fn inc_by3(&self, d: i32) {
        unsafe { ffi::wxSize_IncBy3(self.pinned::<ffi::wxSize>().as_mut(), d) }
    }
    fn inc_to(&self, size: &Size) {
        unsafe {
            let size = &size.pinned::<ffi::wxSize>();
            ffi::wxSize_IncTo(self.pinned::<ffi::wxSize>().as_mut(), size)
        }
    }
    fn is_fully_specified(&self) -> bool {
        unsafe { ffi::wxSize_IsFullySpecified(&self.pinned::<ffi::wxSize>().as_mut()) }
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: i32, height: i32) {
        unsafe { ffi::wxSize_Set(self.pinned::<ffi::wxSize>().as_mut(), width, height) }
    }
    fn set_defaults(&self, size_default: &Size) {
        unsafe {
            let size_default = &size_default.pinned::<ffi::wxSize>();
            ffi::wxSize_SetDefaults(self.pinned::<ffi::wxSize>().as_mut(), size_default)
        }
    }
    fn set_height(&self, height: i32) {
        unsafe { ffi::wxSize_SetHeight(self.pinned::<ffi::wxSize>().as_mut(), height) }
    }
    fn set_width(&self, width: i32) {
        unsafe { ffi::wxSize_SetWidth(self.pinned::<ffi::wxSize>().as_mut(), width) }
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
        unsafe { ffi::wxValidator_Clone(&self.pinned::<ffi::wxValidator>().as_mut()) }
    }
    fn get_window(&self) -> *mut ffi::wxWindow {
        unsafe { ffi::wxValidator_GetWindow(&self.pinned::<ffi::wxValidator>().as_mut()) }
    }
    fn set_window<T: WindowMethods>(&self, window: Option<&T>) {
        unsafe {
            let window = match window {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_SetWindow(self.pinned::<ffi::wxValidator>().as_mut(), window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferFromWindow(self.pinned::<ffi::wxValidator>().as_mut()) }
    }
    fn transfer_to_window(&self) -> bool {
        unsafe { ffi::wxValidator_TransferToWindow(self.pinned::<ffi::wxValidator>().as_mut()) }
    }
    fn validate<T: WindowMethods>(&self, parent: Option<&T>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            ffi::wxValidator_Validate(self.pinned::<ffi::wxValidator>().as_mut(), parent)
        }
    }
    fn suppress_bell_on_error(suppress: bool) {
        unsafe { ffi::wxValidator_SuppressBellOnError(suppress) }
    }
    fn is_silent() -> bool {
        unsafe { ffi::wxValidator_IsSilent() }
    }
}

