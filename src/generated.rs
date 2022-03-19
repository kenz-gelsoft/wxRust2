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
        // CTOR: fn wxObject() -> Object;
        // CTOR: fn wxObject(other: &wxObject) -> Object;
        // DTOR: fn ~wxObject(self: Pin<&mut wxObject>);
        // GENERATED: fn GetClassInfo(self: &wxObject) -> *mut wxClassInfo;
        // GENERATED: fn GetRefData(self: &wxObject) -> *mut wxObjectRefData;
        // GENERATED: unsafe fn IsKindOf(self: &wxObject, info: *const wxClassInfo) -> bool;
        // GENERATED: fn IsSameAs(self: &wxObject, obj: &wxObject) -> bool;
        // GENERATED: fn Ref(self: Pin<&mut wxObject>, clone: &wxObject);
        // GENERATED: unsafe fn SetRefData(self: Pin<&mut wxObject>, data: *mut wxObjectRefData);
        // GENERATED: fn UnRef(self: Pin<&mut wxObject>);
        // GENERATED: fn UnShare(self: Pin<&mut wxObject>);
        // BLOCKED: unsafe fn operator delete(self: Pin<&mut wxObject>, buf: *mut void);
        // CXX_UNSUPPORTED: fn operator new(self: Pin<&mut wxObject>, size: size_t, filename: &wxString, line_num: i32) -> *mut void;
        
        // CLASS: wxEvtHandler
        type wxEvtHandler;
        // GENERATED: unsafe fn QueueEvent(self: Pin<&mut wxEvtHandler>, event: *mut wxEvent);
        // GENERATED: fn AddPendingEvent(self: Pin<&mut wxEvtHandler>, event: &wxEvent);
        // CXX_UNSUPPORTED: unsafe fn CallAfter(self: Pin<&mut wxEvtHandler>, method: *mut void(T::, x1: T1, None: ...);
        // BLOCKED: fn CallAfter(self: Pin<&mut wxEvtHandler>, functor: &T);
        // GENERATED: fn ProcessEvent(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: fn ProcessEventLocally(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: fn SafelyProcessEvent(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: fn ProcessPendingEvents(self: Pin<&mut wxEvtHandler>);
        // GENERATED: fn DeletePendingEvents(self: Pin<&mut wxEvtHandler>);
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
        // BLOCKED: fn GetClientData(self: &wxEvtHandler) -> *mut void;
        // GENERATED: fn GetClientObject(self: &wxEvtHandler) -> *mut wxClientData;
        // BLOCKED: unsafe fn SetClientData(self: Pin<&mut wxEvtHandler>, data: *mut void);
        // GENERATED: unsafe fn SetClientObject(self: Pin<&mut wxEvtHandler>, data: *mut wxClientData);
        // GENERATED: fn GetEvtHandlerEnabled(self: &wxEvtHandler) -> bool;
        // GENERATED: fn GetNextHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        // GENERATED: fn GetPreviousHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        // GENERATED: fn SetEvtHandlerEnabled(self: Pin<&mut wxEvtHandler>, enabled: bool);
        // GENERATED: unsafe fn SetNextHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetPreviousHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        // GENERATED: fn Unlink(self: Pin<&mut wxEvtHandler>);
        // GENERATED: fn IsUnlinked(self: &wxEvtHandler) -> bool;
        // GENERATED: unsafe fn AddFilter(filter: *mut wxEventFilter);
        // GENERATED: unsafe fn RemoveFilter(filter: *mut wxEventFilter);
        // CTOR: fn wxEvtHandler() -> EvtHandler;
        // DTOR: fn ~wxEvtHandler(self: Pin<&mut wxEvtHandler>);
        
        // CLASS: wxWindow
        type wxWindow;
        // GENERATED: fn AcceptsFocus(self: &wxWindow) -> bool;
        // GENERATED: fn AcceptsFocusFromKeyboard(self: &wxWindow) -> bool;
        // GENERATED: fn AcceptsFocusRecursively(self: &wxWindow) -> bool;
        // GENERATED: fn DisableFocusFromKeyboard(self: Pin<&mut wxWindow>);
        // GENERATED: fn IsFocusable(self: &wxWindow) -> bool;
        // GENERATED: fn CanAcceptFocus(self: &wxWindow) -> bool;
        // GENERATED: fn CanAcceptFocusFromKeyboard(self: &wxWindow) -> bool;
        // GENERATED: fn HasFocus(self: &wxWindow) -> bool;
        // GENERATED: fn SetCanFocus(self: Pin<&mut wxWindow>, can_focus: bool);
        // GENERATED: fn EnableVisibleFocus(self: Pin<&mut wxWindow>, enable: bool);
        // GENERATED: fn SetFocus(self: Pin<&mut wxWindow>);
        // GENERATED: fn SetFocusFromKbd(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn AddChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        // GENERATED: fn DestroyChildren(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn FindWindow(self: &wxWindow, id: i32) -> *mut wxWindow;
        // GENERATED: fn FindWindow(self: &wxWindow, name: &wxString) -> *mut wxWindow;
        // BLOCKED: fn GetChildren(self: Pin<&mut wxWindow>) -> Pin<&mut wxWindowList>;
        // BLOCKED: fn GetChildren(self: &wxWindow) -> &wxWindowList;
        // GENERATED: unsafe fn RemoveChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        // GENERATED: fn GetGrandParent(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: fn GetNextSibling(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: fn GetParent(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: fn GetPrevSibling(self: &wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn IsDescendant(self: &wxWindow, win: *mut wxWindow) -> bool;
        // GENERATED: unsafe fn Reparent(self: Pin<&mut wxWindow>, new_parent: *mut wxWindow) -> bool;
        // GENERATED: fn AlwaysShowScrollbars(self: Pin<&mut wxWindow>, hflag: bool, vflag: bool);
        // GENERATED: fn GetScrollPos(self: &wxWindow, orientation: i32) -> i32;
        // GENERATED: fn GetScrollRange(self: &wxWindow, orientation: i32) -> i32;
        // GENERATED: fn GetScrollThumb(self: &wxWindow, orientation: i32) -> i32;
        // GENERATED: fn CanScroll(self: &wxWindow, orient: i32) -> bool;
        // GENERATED: fn HasScrollbar(self: &wxWindow, orient: i32) -> bool;
        // GENERATED: fn IsScrollbarAlwaysShown(self: &wxWindow, orient: i32) -> bool;
        // GENERATED: fn ScrollLines(self: Pin<&mut wxWindow>, lines: i32) -> bool;
        // GENERATED: fn ScrollPages(self: Pin<&mut wxWindow>, pages: i32) -> bool;
        // GENERATED: unsafe fn ScrollWindow(self: Pin<&mut wxWindow>, dx: i32, dy: i32, rect: *const wxRect);
        // GENERATED: fn LineUp(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn LineDown(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn PageUp(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn PageDown(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn SetScrollPos(self: Pin<&mut wxWindow>, orientation: i32, pos: i32, refresh: bool);
        // GENERATED: fn SetScrollbar(self: Pin<&mut wxWindow>, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        // GENERATED: fn BeginRepositioningChildren(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn EndRepositioningChildren(self: Pin<&mut wxWindow>);
        // GENERATED: fn CacheBestSize(self: &wxWindow, size: &wxSize);
        // GENERATED: fn ClientToWindowSize(self: &wxWindow, size: &wxSize) -> wxSize;
        // GENERATED: fn WindowToClientSize(self: &wxWindow, size: &wxSize) -> wxSize;
        // GENERATED: fn Fit(self: Pin<&mut wxWindow>);
        // GENERATED: fn FitInside(self: Pin<&mut wxWindow>);
        // GENERATED: fn FromDIP(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: fn FromDIP(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: fn FromDIP(self: &wxWindow, d: i32) -> i32;
        // GENERATED: fn ToDIP(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: fn ToDIP(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: fn ToDIP(self: &wxWindow, d: i32) -> i32;
        // GENERATED: fn GetBestSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetBestHeight(self: &wxWindow, width: i32) -> i32;
        // GENERATED: fn GetBestWidth(self: &wxWindow, height: i32) -> i32;
        // GENERATED: unsafe fn GetClientSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: fn GetClientSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetEffectiveMinSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetMaxClientSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetMaxSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetMinClientSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetMinSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetMinWidth(self: &wxWindow) -> i32;
        // GENERATED: fn GetMinHeight(self: &wxWindow) -> i32;
        // GENERATED: fn GetMaxWidth(self: &wxWindow) -> i32;
        // GENERATED: fn GetMaxHeight(self: &wxWindow) -> i32;
        // GENERATED: unsafe fn GetSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: fn GetSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetVirtualSize(self: &wxWindow) -> wxSize;
        // GENERATED: unsafe fn GetVirtualSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // GENERATED: fn GetBestVirtualSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn GetContentScaleFactor(self: &wxWindow) -> f64;
        // GENERATED: fn GetDPIScaleFactor(self: &wxWindow) -> f64;
        // GENERATED: fn GetWindowBorderSize(self: &wxWindow) -> wxSize;
        // GENERATED: fn InformFirstDirection(self: Pin<&mut wxWindow>, direction: i32, size: i32, available_other_dir: i32) -> bool;
        // GENERATED: fn InvalidateBestSize(self: Pin<&mut wxWindow>);
        // GENERATED: fn PostSizeEvent(self: Pin<&mut wxWindow>);
        // GENERATED: fn PostSizeEventToParent(self: Pin<&mut wxWindow>);
        // GENERATED: fn SendSizeEvent(self: Pin<&mut wxWindow>, flags: i32);
        // GENERATED: fn SendSizeEventToParent(self: Pin<&mut wxWindow>, flags: i32);
        // GENERATED: fn SetClientSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        // GENERATED: fn SetClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetClientSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        // GENERATED: unsafe fn SetContainingSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer);
        // GENERATED: fn SetInitialSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetMaxClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetMaxSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetMinClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetMinSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetSize(self: Pin<&mut wxWindow>, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        // GENERATED: fn SetSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        // GENERATED: fn SetSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: fn SetSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        // GENERATED: fn SetSizeHints(self: Pin<&mut wxWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        // GENERATED: fn SetSizeHints(self: Pin<&mut wxWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: fn SetVirtualSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        // GENERATED: fn SetVirtualSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // GENERATED: unsafe fn FromDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn FromDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn FromDIP(d: i32, w: *const wxWindow) -> i32;
        // GENERATED: unsafe fn ToDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // GENERATED: unsafe fn ToDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // GENERATED: unsafe fn ToDIP(d: i32, w: *const wxWindow) -> i32;
        // GENERATED: fn Center(self: Pin<&mut wxWindow>, dir: i32);
        // GENERATED: fn CenterOnParent(self: Pin<&mut wxWindow>, dir: i32);
        // GENERATED: fn Centre(self: Pin<&mut wxWindow>, direction: i32);
        // GENERATED: fn CentreOnParent(self: Pin<&mut wxWindow>, direction: i32);
        // GENERATED: unsafe fn GetPosition(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: fn GetPosition(self: &wxWindow) -> wxPoint;
        // GENERATED: fn GetRect(self: &wxWindow) -> wxRect;
        // GENERATED: unsafe fn GetScreenPosition(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: fn GetScreenPosition(self: &wxWindow) -> wxPoint;
        // GENERATED: fn GetScreenRect(self: &wxWindow) -> wxRect;
        // GENERATED: fn GetClientAreaOrigin(self: &wxWindow) -> wxPoint;
        // GENERATED: fn GetClientRect(self: &wxWindow) -> wxRect;
        // GENERATED: fn Move(self: Pin<&mut wxWindow>, x: i32, y: i32, flags: i32);
        // GENERATED: fn Move(self: Pin<&mut wxWindow>, pt: &wxPoint, flags: i32);
        // GENERATED: fn SetPosition(self: Pin<&mut wxWindow>, pt: &wxPoint);
        // GENERATED: unsafe fn ClientToScreen(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: fn ClientToScreen(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: fn ConvertDialogToPixels(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: fn ConvertDialogToPixels(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: fn ConvertPixelsToDialog(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: fn ConvertPixelsToDialog(self: &wxWindow, sz: &wxSize) -> wxSize;
        // GENERATED: unsafe fn ScreenToClient(self: &wxWindow, x: *mut i32, y: *mut i32);
        // GENERATED: fn ScreenToClient(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // GENERATED: fn ClearBackground(self: Pin<&mut wxWindow>);
        // GENERATED: fn Freeze(self: Pin<&mut wxWindow>);
        // GENERATED: fn Thaw(self: Pin<&mut wxWindow>);
        // GENERATED: fn IsFrozen(self: &wxWindow) -> bool;
        // CXX_UNSUPPORTED: fn GetBackgroundColour(self: &wxWindow) -> wxColour;
        // CXX_UNSUPPORTED: fn GetBackgroundStyle(self: &wxWindow) -> wxBackgroundStyle;
        // GENERATED: fn GetCharHeight(self: &wxWindow) -> i32;
        // GENERATED: fn GetCharWidth(self: &wxWindow) -> i32;
        // CXX_UNSUPPORTED: fn GetDefaultAttributes(self: &wxWindow) -> wxVisualAttributes;
        // GENERATED: fn GetDPI(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetFont(self: &wxWindow) -> wxFont;
        // CXX_UNSUPPORTED: fn GetForegroundColour(self: &wxWindow) -> wxColour;
        // GENERATED: unsafe fn GetTextExtent(self: &wxWindow, string: &wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        // GENERATED: fn GetTextExtent(self: &wxWindow, string: &wxString) -> wxSize;
        // BLOCKED: fn GetUpdateRegion(self: &wxWindow) -> &wxRegion;
        // GENERATED: fn GetUpdateClientRect(self: &wxWindow) -> wxRect;
        // GENERATED: fn HasTransparentBackground(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: unsafe fn Refresh(self: Pin<&mut wxWindow>, erase_background: bool, rect: *const wxRect);
        // GENERATED: fn RefreshRect(self: Pin<&mut wxWindow>, rect: &wxRect, erase_background: bool);
        // GENERATED: fn Update(self: Pin<&mut wxWindow>);
        // GENERATED: fn SetBackgroundColour(self: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        // CXX_UNSUPPORTED: fn SetBackgroundStyle(self: Pin<&mut wxWindow>, style: wxBackgroundStyle) -> bool;
        // GENERATED: unsafe fn IsTransparentBackgroundSupported(self: &wxWindow, reason: *mut wxString) -> bool;
        // GENERATED: fn SetFont(self: Pin<&mut wxWindow>, font: &wxFont) -> bool;
        // GENERATED: fn SetForegroundColour(self: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        // GENERATED: fn SetOwnBackgroundColour(self: Pin<&mut wxWindow>, colour: &wxColour);
        // GENERATED: fn InheritsBackgroundColour(self: &wxWindow) -> bool;
        // GENERATED: fn UseBgCol(self: &wxWindow) -> bool;
        // GENERATED: fn UseBackgroundColour(self: &wxWindow) -> bool;
        // GENERATED: fn SetOwnFont(self: Pin<&mut wxWindow>, font: &wxFont);
        // GENERATED: fn SetOwnForegroundColour(self: Pin<&mut wxWindow>, colour: &wxColour);
        // GENERATED: fn UseForegroundColour(self: &wxWindow) -> bool;
        // GENERATED: fn InheritsForegroundColour(self: &wxWindow) -> bool;
        // GENERATED: fn SetPalette(self: Pin<&mut wxWindow>, pal: &wxPalette);
        // GENERATED: fn ShouldInheritColours(self: &wxWindow) -> bool;
        // GENERATED: fn SetThemeEnabled(self: Pin<&mut wxWindow>, enable: bool);
        // GENERATED: fn GetThemeEnabled(self: &wxWindow) -> bool;
        // GENERATED: fn CanSetTransparent(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn SetTransparent(self: Pin<&mut wxWindow>, alpha: u8) -> bool;
        // GENERATED: fn GetEventHandler(self: &wxWindow) -> *mut wxEvtHandler;
        // GENERATED: fn HandleAsNavigationKey(self: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        // GENERATED: fn HandleWindowEvent(self: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: fn ProcessWindowEvent(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: fn ProcessWindowEventLocally(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        // GENERATED: fn PopEventHandler(self: Pin<&mut wxWindow>, delete_handler: bool) -> *mut wxEvtHandler;
        // GENERATED: unsafe fn PushEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn RemoveEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler) -> bool;
        // GENERATED: unsafe fn SetEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetNextHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: unsafe fn SetPreviousHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // GENERATED: fn GetExtraStyle(self: &wxWindow) -> i32;
        // GENERATED: fn GetWindowStyleFlag(self: &wxWindow) -> i32;
        // GENERATED: fn GetWindowStyle(self: &wxWindow) -> i32;
        // GENERATED: fn HasExtraStyle(self: &wxWindow, ex_flag: i32) -> bool;
        // GENERATED: fn HasFlag(self: &wxWindow, flag: i32) -> bool;
        // GENERATED: fn SetExtraStyle(self: Pin<&mut wxWindow>, ex_style: i32);
        // GENERATED: fn SetWindowStyleFlag(self: Pin<&mut wxWindow>, style: i32);
        // GENERATED: fn SetWindowStyle(self: Pin<&mut wxWindow>, style: i32);
        // GENERATED: fn ToggleWindowStyle(self: Pin<&mut wxWindow>, flag: i32) -> bool;
        // GENERATED: unsafe fn MoveAfterInTabOrder(self: Pin<&mut wxWindow>, win: *mut wxWindow);
        // GENERATED: unsafe fn MoveBeforeInTabOrder(self: Pin<&mut wxWindow>, win: *mut wxWindow);
        // GENERATED: fn Navigate(self: Pin<&mut wxWindow>, flags: i32) -> bool;
        // GENERATED: fn NavigateIn(self: Pin<&mut wxWindow>, flags: i32) -> bool;
        // GENERATED: fn Lower(self: Pin<&mut wxWindow>);
        // GENERATED: fn Raise(self: Pin<&mut wxWindow>);
        // GENERATED: fn Hide(self: Pin<&mut wxWindow>) -> bool;
        // CXX_UNSUPPORTED: fn HideWithEffect(self: Pin<&mut wxWindow>, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: fn IsEnabled(self: &wxWindow) -> bool;
        // GENERATED: fn IsExposed(self: &wxWindow, x: i32, y: i32) -> bool;
        // GENERATED: fn IsExposed(self: &wxWindow, pt: &mut wxPoint) -> bool;
        // GENERATED: fn IsExposed(self: &wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        // GENERATED: fn IsExposed(self: &wxWindow, rect: &mut wxRect) -> bool;
        // GENERATED: fn IsShown(self: &wxWindow) -> bool;
        // GENERATED: fn IsShownOnScreen(self: &wxWindow) -> bool;
        // GENERATED: fn Disable(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn Enable(self: Pin<&mut wxWindow>, enable: bool) -> bool;
        // GENERATED: fn Show(self: Pin<&mut wxWindow>, show: bool) -> bool;
        // CXX_UNSUPPORTED: fn ShowWithEffect(self: Pin<&mut wxWindow>, effect: wxShowEffect, timeout: u32) -> bool;
        // GENERATED: fn GetHelpText(self: &wxWindow) -> wxString;
        // GENERATED: fn SetHelpText(self: Pin<&mut wxWindow>, help_text: &wxString);
        // CXX_UNSUPPORTED: fn GetHelpTextAtPoint(self: &wxWindow, point: &wxPoint, origin: wxHelpEvent::Origin) -> wxString;
        // GENERATED: fn GetToolTip(self: &wxWindow) -> *mut wxToolTip;
        // GENERATED: fn GetToolTipText(self: &wxWindow) -> wxString;
        // GENERATED: fn SetToolTip(self: Pin<&mut wxWindow>, tip_string: &wxString);
        // GENERATED: unsafe fn SetToolTip(self: Pin<&mut wxWindow>, tip: *mut wxToolTip);
        // GENERATED: fn UnsetToolTip(self: Pin<&mut wxWindow>);
        // GENERATED: fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, pos: &wxPoint) -> i32;
        // GENERATED: fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, x: i32, y: i32) -> i32;
        // GENERATED: unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, pos: &wxPoint) -> bool;
        // GENERATED: unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        // GENERATED: fn GetValidator(self: Pin<&mut wxWindow>) -> *mut wxValidator;
        // GENERATED: fn SetValidator(self: Pin<&mut wxWindow>, validator: &wxValidator);
        // GENERATED: fn TransferDataFromWindow(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn TransferDataToWindow(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn Validate(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn GetId(self: &wxWindow) -> i32;
        // GENERATED: fn GetLabel(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: fn GetLayoutDirection(self: &wxWindow) -> wxLayoutDirection;
        // GENERATED: fn AdjustForLayoutDirection(self: &wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        // GENERATED: fn GetName(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: fn GetWindowVariant(self: &wxWindow) -> wxWindowVariant;
        // GENERATED: fn SetId(self: Pin<&mut wxWindow>, winid: i32);
        // GENERATED: fn SetLabel(self: Pin<&mut wxWindow>, label: &wxString);
        // CXX_UNSUPPORTED: fn SetLayoutDirection(self: Pin<&mut wxWindow>, dir: wxLayoutDirection);
        // GENERATED: fn SetName(self: Pin<&mut wxWindow>, name: &wxString);
        // CXX_UNSUPPORTED: fn SetWindowVariant(self: Pin<&mut wxWindow>, variant: wxWindowVariant);
        // GENERATED: fn GetAcceleratorTable(self: Pin<&mut wxWindow>) -> *mut wxAcceleratorTable;
        // CXX_UNSUPPORTED: fn GetAccessible(self: Pin<&mut wxWindow>) -> *mut wxAccessible;
        // GENERATED: fn SetAcceleratorTable(self: Pin<&mut wxWindow>, accel: &wxAcceleratorTable);
        // CXX_UNSUPPORTED: unsafe fn SetAccessible(self: Pin<&mut wxWindow>, accessible: *mut wxAccessible);
        // GENERATED: fn Close(self: Pin<&mut wxWindow>, force: bool) -> bool;
        // GENERATED: fn Destroy(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn IsBeingDeleted(self: &wxWindow) -> bool;
        // GENERATED: fn GetDropTarget(self: &wxWindow) -> *mut wxDropTarget;
        // GENERATED: unsafe fn SetDropTarget(self: Pin<&mut wxWindow>, target: *mut wxDropTarget);
        // GENERATED: fn DragAcceptFiles(self: Pin<&mut wxWindow>, accept: bool);
        // GENERATED: fn GetContainingSizer(self: &wxWindow) -> *mut wxSizer;
        // GENERATED: fn GetSizer(self: &wxWindow) -> *mut wxSizer;
        // GENERATED: unsafe fn SetSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        // GENERATED: unsafe fn SetSizerAndFit(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        // GENERATED: fn GetConstraints(self: &wxWindow) -> *mut wxLayoutConstraints;
        // GENERATED: unsafe fn SetConstraints(self: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        // GENERATED: fn Layout(self: Pin<&mut wxWindow>) -> bool;
        // GENERATED: fn SetAutoLayout(self: Pin<&mut wxWindow>, auto_layout: bool);
        // GENERATED: fn GetAutoLayout(self: &wxWindow) -> bool;
        // GENERATED: fn CaptureMouse(self: Pin<&mut wxWindow>);
        // GENERATED: fn GetCaret(self: &wxWindow) -> *mut wxCaret;
        // BLOCKED: fn GetCursor(self: &wxWindow) -> &wxCursor;
        // GENERATED: fn HasCapture(self: &wxWindow) -> bool;
        // GENERATED: fn ReleaseMouse(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn SetCaret(self: Pin<&mut wxWindow>, caret: *mut wxCaret);
        // GENERATED: fn SetCursor(self: Pin<&mut wxWindow>, cursor: &wxCursor) -> bool;
        // GENERATED: fn WarpPointer(self: Pin<&mut wxWindow>, x: i32, y: i32);
        // GENERATED: fn EnableTouchEvents(self: Pin<&mut wxWindow>, events_mask: i32) -> bool;
        // CXX_UNSUPPORTED: fn HitTest(self: &wxWindow, x: i32, y: i32) -> wxHitTest;
        // CXX_UNSUPPORTED: fn HitTest(self: &wxWindow, pt: &wxPoint) -> wxHitTest;
        // CXX_UNSUPPORTED: fn GetBorder(self: &wxWindow, flags: i32) -> wxBorder;
        // CXX_UNSUPPORTED: fn GetBorder(self: &wxWindow) -> wxBorder;
        // GENERATED: fn DoUpdateWindowUI(self: Pin<&mut wxWindow>, event: Pin<&mut wxUpdateUIEvent>);
        // CXX_UNSUPPORTED: fn GetHandle(self: &wxWindow) -> WXWidget;
        // GENERATED: fn HasMultiplePages(self: &wxWindow) -> bool;
        // GENERATED: fn InheritAttributes(self: Pin<&mut wxWindow>);
        // GENERATED: fn InitDialog(self: Pin<&mut wxWindow>);
        // GENERATED: fn IsDoubleBuffered(self: &wxWindow) -> bool;
        // GENERATED: fn SetDoubleBuffered(self: Pin<&mut wxWindow>, on: bool);
        // GENERATED: fn IsRetained(self: &wxWindow) -> bool;
        // GENERATED: fn IsThisEnabled(self: &wxWindow) -> bool;
        // GENERATED: fn IsTopLevel(self: &wxWindow) -> bool;
        // GENERATED: fn OnInternalIdle(self: Pin<&mut wxWindow>);
        // GENERATED: fn SendIdleEvents(self: Pin<&mut wxWindow>, event: Pin<&mut wxIdleEvent>) -> bool;
        // GENERATED: fn RegisterHotKey(self: Pin<&mut wxWindow>, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        // GENERATED: fn UnregisterHotKey(self: Pin<&mut wxWindow>, hotkey_id: i32) -> bool;
        // GENERATED: fn UpdateWindowUI(self: Pin<&mut wxWindow>, flags: i32);
        // CXX_UNSUPPORTED: fn GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        // GENERATED: fn FindFocus() -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // GENERATED: fn GetCapture() -> *mut wxWindow;
        // GENERATED: fn NewControlId(count: i32) -> i32;
        // GENERATED: fn UnreserveControlId(id: i32, count: i32);
        // CTOR: fn wxWindow() -> Window;
        // CTOR: unsafe fn wxWindow(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Window;
        // DTOR: fn ~wxWindow(self: Pin<&mut wxWindow>);
        // GENERATED: unsafe fn Create(self: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        
        // CLASS: wxControl
        type wxControl;
        // CTOR: unsafe fn wxControl(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Control;
        // CTOR: fn wxControl() -> Control;
        // GENERATED: unsafe fn Create(self: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        // GENERATED: fn Command(self: Pin<&mut wxControl>, event: Pin<&mut wxCommandEvent>);
        // GENERATED: fn GetLabel(self: &wxControl) -> wxString;
        // GENERATED: fn GetLabelText(self: &wxControl) -> wxString;
        // GENERATED: fn GetSizeFromTextSize(self: &wxControl, xlen: i32, ylen: i32) -> wxSize;
        // GENERATED: fn GetSizeFromTextSize(self: &wxControl, tsize: &wxSize) -> wxSize;
        // GENERATED: fn GetSizeFromText(self: &wxControl, text: &wxString) -> wxSize;
        // GENERATED: fn SetLabel(self: Pin<&mut wxControl>, label: &wxString);
        // GENERATED: fn SetLabelText(self: Pin<&mut wxControl>, text: &wxString);
        // GENERATED: fn SetLabelMarkup(self: Pin<&mut wxControl>, markup: &wxString) -> bool;
        // GENERATED: fn GetLabelText(label: &wxString) -> wxString;
        // GENERATED: fn RemoveMnemonics(str: &wxString) -> wxString;
        // GENERATED: fn EscapeMnemonics(text: &wxString) -> wxString;
        // BLOCKED: fn Ellipsize(label: &wxString, dc: &wxDC, mode: i32, max_width: i32, flags: i32) -> wxString;
        
        // CLASS: wxAnyButton
        type wxAnyButton;
        // CTOR: fn wxAnyButton() -> AnyButton;
        // DTOR: fn ~wxAnyButton(self: Pin<&mut wxAnyButton>);
        // CXX_UNSUPPORTED: fn GetBitmap(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: fn GetBitmapCurrent(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: fn GetBitmapDisabled(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: fn GetBitmapFocus(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: fn GetBitmapLabel(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: fn GetBitmapPressed(self: &wxAnyButton) -> wxBitmap;
        // CXX_UNSUPPORTED: fn SetBitmap(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap, dir: wxDirection);
        // GENERATED: fn SetBitmapCurrent(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: fn SetBitmapDisabled(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: fn SetBitmapFocus(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: fn SetBitmapLabel(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: fn SetBitmapPressed(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // GENERATED: fn GetBitmapMargins(self: Pin<&mut wxAnyButton>) -> wxSize;
        // GENERATED: fn SetBitmapMargins(self: Pin<&mut wxAnyButton>, x: i32, y: i32);
        // GENERATED: fn SetBitmapMargins(self: Pin<&mut wxAnyButton>, sz: &wxSize);
        // CXX_UNSUPPORTED: fn SetBitmapPosition(self: Pin<&mut wxAnyButton>, dir: wxDirection);
        
        // CLASS: wxButton
        type wxButton;
        // CTOR: fn wxButton() -> Button;
        // CTOR: unsafe fn wxButton(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Button;
        // GENERATED: unsafe fn Create(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        // GENERATED: fn GetAuthNeeded(self: &wxButton) -> bool;
        // GENERATED: fn GetLabel(self: &wxButton) -> wxString;
        // GENERATED: fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
        // GENERATED: fn SetDefault(self: Pin<&mut wxButton>) -> *mut wxWindow;
        // GENERATED: fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
        // GENERATED: unsafe fn GetDefaultSize(win: *mut wxWindow) -> wxSize;
        
        // CLASS: wxNonOwnedWindow
        type wxNonOwnedWindow;
        // GENERATED: fn SetShape(self: Pin<&mut wxNonOwnedWindow>, region: &wxRegion) -> bool;
        // GENERATED: fn SetShape(self: Pin<&mut wxNonOwnedWindow>, path: &wxGraphicsPath) -> bool;
        
        // CLASS: wxTopLevelWindow
        type wxTopLevelWindow;
        // CTOR: fn wxTopLevelWindow() -> TopLevelWindow;
        // CTOR: unsafe fn wxTopLevelWindow(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> TopLevelWindow;
        // DTOR: fn ~wxTopLevelWindow(self: Pin<&mut wxTopLevelWindow>);
        // GENERATED: unsafe fn Create(self: Pin<&mut wxTopLevelWindow>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // GENERATED: fn CanSetTransparent(self: Pin<&mut wxTopLevelWindow>) -> bool;
        // GENERATED: fn CenterOnScreen(self: Pin<&mut wxTopLevelWindow>, direction: i32);
        // GENERATED: fn CentreOnScreen(self: Pin<&mut wxTopLevelWindow>, direction: i32);
        // GENERATED: fn EnableCloseButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: fn EnableMaximizeButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: fn EnableMinimizeButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: fn GetDefaultItem(self: &wxTopLevelWindow) -> *mut wxWindow;
        // CXX_UNSUPPORTED: fn GetIcon(self: &wxTopLevelWindow) -> wxIcon;
        // BLOCKED: fn GetIcons(self: &wxTopLevelWindow) -> &wxIconBundle;
        // GENERATED: fn GetTitle(self: &wxTopLevelWindow) -> wxString;
        // GENERATED: fn Iconize(self: Pin<&mut wxTopLevelWindow>, iconize: bool);
        // GENERATED: fn IsActive(self: Pin<&mut wxTopLevelWindow>) -> bool;
        // GENERATED: fn IsAlwaysMaximized(self: &wxTopLevelWindow) -> bool;
        // GENERATED: fn IsFullScreen(self: &wxTopLevelWindow) -> bool;
        // GENERATED: fn IsIconized(self: &wxTopLevelWindow) -> bool;
        // GENERATED: fn IsMaximized(self: &wxTopLevelWindow) -> bool;
        // BLOCKED: fn IsUsingNativeDecorations(self: &wxTopLevelWindow) -> bool;
        // GENERATED: fn Layout(self: Pin<&mut wxTopLevelWindow>) -> bool;
        // GENERATED: fn Maximize(self: Pin<&mut wxTopLevelWindow>, maximize: bool);
        // BLOCKED: fn MSWGetSystemMenu(self: &wxTopLevelWindow) -> *mut wxMenu;
        // GENERATED: fn RequestUserAttention(self: Pin<&mut wxTopLevelWindow>, flags: i32);
        // GENERATED: fn Restore(self: Pin<&mut wxTopLevelWindow>);
        // BLOCKED: fn RestoreToGeometry(self: Pin<&mut wxTopLevelWindow>, ser: Pin<&mut GeometrySerializer>) -> bool;
        // BLOCKED: fn SaveGeometry(self: &wxTopLevelWindow, ser: &GeometrySerializer) -> bool;
        // GENERATED: unsafe fn SetDefaultItem(self: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        // GENERATED: unsafe fn SetTmpDefaultItem(self: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        // GENERATED: fn GetTmpDefaultItem(self: &wxTopLevelWindow) -> *mut wxWindow;
        // GENERATED: fn SetIcon(self: Pin<&mut wxTopLevelWindow>, icon: &wxIcon);
        // GENERATED: fn SetIcons(self: Pin<&mut wxTopLevelWindow>, icons: &wxIconBundle);
        // GENERATED: fn SetMaxSize(self: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        // GENERATED: fn SetMinSize(self: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        // GENERATED: fn SetSizeHints(self: Pin<&mut wxTopLevelWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        // GENERATED: fn SetSizeHints(self: Pin<&mut wxTopLevelWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        // GENERATED: fn SetTitle(self: Pin<&mut wxTopLevelWindow>, title: &wxString);
        // GENERATED: fn SetTransparent(self: Pin<&mut wxTopLevelWindow>, alpha: u8) -> bool;
        // GENERATED: fn ShouldPreventAppExit(self: &wxTopLevelWindow) -> bool;
        // GENERATED: fn OSXSetModified(self: Pin<&mut wxTopLevelWindow>, modified: bool);
        // GENERATED: fn OSXIsModified(self: &wxTopLevelWindow) -> bool;
        // GENERATED: fn SetRepresentedFilename(self: Pin<&mut wxTopLevelWindow>, filename: &wxString);
        // GENERATED: fn ShowWithoutActivating(self: Pin<&mut wxTopLevelWindow>);
        // GENERATED: fn EnableFullScreenView(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // GENERATED: fn ShowFullScreen(self: Pin<&mut wxTopLevelWindow>, show: bool, style: i32) -> bool;
        // BLOCKED: fn UseNativeDecorations(self: Pin<&mut wxTopLevelWindow>, native: bool);
        // BLOCKED: fn UseNativeDecorationsByDefault(self: Pin<&mut wxTopLevelWindow>, native: bool);
        // GENERATED: fn GetDefaultSize() -> wxSize;
        
        // CLASS: wxFrame
        type wxFrame;
        // CTOR: fn wxFrame() -> Frame;
        // CTOR: unsafe fn wxFrame(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Frame;
        // DTOR: fn ~wxFrame(self: Pin<&mut wxFrame>);
        // GENERATED: fn Centre(self: Pin<&mut wxFrame>, direction: i32);
        // GENERATED: unsafe fn Create(self: Pin<&mut wxFrame>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // GENERATED: fn CreateStatusBar(self: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        // GENERATED: fn CreateToolBar(self: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        // GENERATED: fn DoGiveHelp(self: Pin<&mut wxFrame>, text: &wxString, show: bool);
        // GENERATED: fn GetClientAreaOrigin(self: &wxFrame) -> wxPoint;
        // GENERATED: fn GetMenuBar(self: &wxFrame) -> *mut wxMenuBar;
        // GENERATED: fn GetStatusBar(self: &wxFrame) -> *mut wxStatusBar;
        // GENERATED: fn GetStatusBarPane(self: &wxFrame) -> i32;
        // GENERATED: fn GetToolBar(self: &wxFrame) -> *mut wxToolBar;
        // GENERATED: fn OnCreateStatusBar(self: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        // GENERATED: fn OnCreateToolBar(self: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        // GENERATED: fn ProcessCommand(self: Pin<&mut wxFrame>, id: i32) -> bool;
        // GENERATED: unsafe fn SetMenuBar(self: Pin<&mut wxFrame>, menu_bar: *mut wxMenuBar);
        // GENERATED: unsafe fn SetStatusBar(self: Pin<&mut wxFrame>, status_bar: *mut wxStatusBar);
        // GENERATED: fn SetStatusBarPane(self: Pin<&mut wxFrame>, n: i32);
        // GENERATED: fn SetStatusText(self: Pin<&mut wxFrame>, text: &wxString, number: i32);
        // GENERATED: unsafe fn SetStatusWidths(self: Pin<&mut wxFrame>, n: i32, widths_field: *const i32);
        // GENERATED: unsafe fn SetToolBar(self: Pin<&mut wxFrame>, tool_bar: *mut wxToolBar);
        // BLOCKED: fn MSWGetTaskBarButton(self: Pin<&mut wxFrame>) -> *mut wxTaskBarButton;
        // GENERATED: fn PushStatusText(self: Pin<&mut wxFrame>, text: &wxString, number: i32);
        // GENERATED: fn PopStatusText(self: Pin<&mut wxFrame>, number: i32);
        
        // CLASS: wxPoint
        type wxPoint = crate::wxPoint;
        // GENERATED: fn IsFullySpecified(self: &wxPoint) -> bool;
        // GENERATED: fn SetDefaults(self: &mut wxPoint, pt: &wxPoint);
        // BLOCKED: fn operator=(self: &mut wxPoint, pt: &wxPoint) -> &mut wxPoint;
        // BLOCKED: fn operator==(self: &mut wxPoint, p1: &wxPoint, p2: &wxPoint) -> bool;
        // BLOCKED: fn operator!=(self: &mut wxPoint, p1: &wxPoint, p2: &wxPoint) -> bool;
        // BLOCKED: fn operator+(self: &mut wxPoint, p1: &wxPoint, p2: &wxPoint) -> wxPoint;
        // BLOCKED: fn operator-(self: &mut wxPoint, p1: &wxPoint, p2: &wxPoint) -> wxPoint;
        // BLOCKED: fn operator+=(self: &mut wxPoint, pt: &wxPoint) -> &mut wxPoint;
        // BLOCKED: fn operator-=(self: &mut wxPoint, pt: &wxPoint) -> &mut wxPoint;
        // BLOCKED: fn operator+(self: &mut wxPoint, pt: &wxPoint, sz: &wxSize) -> wxPoint;
        // BLOCKED: fn operator-(self: &mut wxPoint, pt: &wxPoint, sz: &wxSize) -> wxPoint;
        // BLOCKED: fn operator+(self: &mut wxPoint, sz: &wxSize, pt: &wxPoint) -> wxPoint;
        // BLOCKED: fn operator-(self: &mut wxPoint, sz: &wxSize, pt: &wxPoint) -> wxPoint;
        // BLOCKED: fn operator+=(self: &mut wxPoint, sz: &wxSize) -> &mut wxPoint;
        // BLOCKED: fn operator-=(self: &mut wxPoint, sz: &wxSize) -> &mut wxPoint;
        // BLOCKED: fn operator/(self: &mut wxPoint, sz: &wxPoint, factor: i32) -> wxSize;
        // BLOCKED: fn operator*(self: &mut wxPoint, sz: &wxPoint, factor: i32) -> wxSize;
        // BLOCKED: fn operator*(self: &mut wxPoint, factor: i32, sz: &wxSize) -> wxSize;
        // BLOCKED: fn operator/=(self: &mut wxPoint, factor: i32) -> &mut wxSize;
        // BLOCKED: fn operator*=(self: &mut wxPoint, factor: i32) -> &mut wxSize;
        // CTOR: fn wxPoint() -> Point;
        // CTOR: fn wxPoint(x: i32, y: i32) -> Point;
        // CTOR: fn wxPoint(pt: &wxRealPoint) -> Point;
        
        // CLASS: wxRect
        type wxRect = crate::wxRect;
        // CTOR: fn wxRect() -> Rect;
        // CTOR: fn wxRect(x: i32, y: i32, width: i32, height: i32) -> Rect;
        // CTOR: fn wxRect(top_left: &wxPoint, bottom_right: &wxPoint) -> Rect;
        // CTOR: fn wxRect(pos: &wxPoint, size: &wxSize) -> Rect;
        // CTOR: fn wxRect(size: &wxSize) -> Rect;
        // GENERATED: fn CentreIn(self: &wxRect, r: &wxRect, dir: i32) -> wxRect;
        // GENERATED: fn CenterIn(self: &wxRect, r: &wxRect, dir: i32) -> wxRect;
        // GENERATED: fn Contains(self: &wxRect, x: i32, y: i32) -> bool;
        // GENERATED: fn Contains(self: &wxRect, pt: &wxPoint) -> bool;
        // GENERATED: fn Contains(self: &wxRect, rect: &wxRect) -> bool;
        // BLOCKED: fn Deflate(self: &mut wxRect, dx: i32, dy: i32) -> &mut wxRect;
        // BLOCKED: fn Deflate(self: &mut wxRect, diff: &wxSize) -> &mut wxRect;
        // BLOCKED: fn Deflate(self: &mut wxRect, diff: i32) -> &mut wxRect;
        // GENERATED: fn Deflate(self: &wxRect, dx: i32, dy: i32) -> wxRect;
        // GENERATED: fn GetBottom(self: &wxRect) -> i32;
        // GENERATED: fn GetBottomLeft(self: &wxRect) -> wxPoint;
        // GENERATED: fn GetBottomRight(self: &wxRect) -> wxPoint;
        // GENERATED: fn GetHeight(self: &wxRect) -> i32;
        // GENERATED: fn GetLeft(self: &wxRect) -> i32;
        // GENERATED: fn GetPosition(self: &wxRect) -> wxPoint;
        // GENERATED: fn GetRight(self: &wxRect) -> i32;
        // GENERATED: fn GetSize(self: &wxRect) -> wxSize;
        // GENERATED: fn GetTop(self: &wxRect) -> i32;
        // GENERATED: fn GetTopLeft(self: &wxRect) -> wxPoint;
        // GENERATED: fn GetTopRight(self: &wxRect) -> wxPoint;
        // GENERATED: fn GetWidth(self: &wxRect) -> i32;
        // GENERATED: fn GetX(self: &wxRect) -> i32;
        // GENERATED: fn GetY(self: &wxRect) -> i32;
        // BLOCKED: fn Inflate(self: &mut wxRect, dx: i32, dy: i32) -> &mut wxRect;
        // BLOCKED: fn Inflate(self: &mut wxRect, diff: &wxSize) -> &mut wxRect;
        // BLOCKED: fn Inflate(self: &mut wxRect, diff: i32) -> &mut wxRect;
        // GENERATED: fn Inflate(self: &wxRect, dx: i32, dy: i32) -> wxRect;
        // BLOCKED: fn Intersect(self: &mut wxRect, rect: &wxRect) -> &mut wxRect;
        // GENERATED: fn Intersect(self: &wxRect, rect: &wxRect) -> wxRect;
        // GENERATED: fn Intersects(self: &wxRect, rect: &wxRect) -> bool;
        // GENERATED: fn IsEmpty(self: &wxRect) -> bool;
        // GENERATED: fn Offset(self: &mut wxRect, dx: i32, dy: i32);
        // GENERATED: fn Offset(self: &mut wxRect, pt: &wxPoint);
        // GENERATED: fn SetHeight(self: &mut wxRect, height: i32);
        // GENERATED: fn SetPosition(self: &mut wxRect, pos: &wxPoint);
        // GENERATED: fn SetSize(self: &mut wxRect, s: &wxSize);
        // GENERATED: fn SetWidth(self: &mut wxRect, width: i32);
        // GENERATED: fn SetX(self: &mut wxRect, x: i32);
        // GENERATED: fn SetY(self: &mut wxRect, y: i32);
        // GENERATED: fn SetLeft(self: &mut wxRect, left: i32);
        // GENERATED: fn SetRight(self: &mut wxRect, right: i32);
        // GENERATED: fn SetTop(self: &mut wxRect, top: i32);
        // GENERATED: fn SetBottom(self: &mut wxRect, bottom: i32);
        // GENERATED: fn SetTopLeft(self: &mut wxRect, p: &wxPoint);
        // GENERATED: fn SetBottomRight(self: &mut wxRect, p: &wxPoint);
        // GENERATED: fn SetTopRight(self: &mut wxRect, p: &wxPoint);
        // GENERATED: fn SetBottomLeft(self: &mut wxRect, p: &wxPoint);
        // GENERATED: fn Union(self: &wxRect, rect: &wxRect) -> wxRect;
        // BLOCKED: fn Union(self: &mut wxRect, rect: &wxRect) -> &mut wxRect;
        // BLOCKED: fn operator!=(self: &mut wxRect, r1: &wxRect, r2: &wxRect) -> bool;
        // BLOCKED: fn operator+(self: &mut wxRect, r1: &wxRect, r2: &wxRect) -> wxRect;
        // BLOCKED: fn operator+=(self: &mut wxRect, r: &wxRect) -> &mut wxRect;
        // BLOCKED: fn operator*(self: &mut wxRect, r1: &wxRect, r2: &wxRect) -> wxRect;
        // BLOCKED: fn operator*=(self: &mut wxRect, r: &wxRect) -> &mut wxRect;
        // BLOCKED: fn operator=(self: &mut wxRect, rect: &wxRect) -> &mut wxRect;
        // BLOCKED: fn operator==(self: &mut wxRect, r1: &wxRect, r2: &wxRect) -> bool;
        
        // CLASS: wxSize
        type wxSize = crate::wxSize;
        // BLOCKED: fn operator=(self: &mut wxSize, sz: &wxSize) -> &mut wxSize;
        // BLOCKED: fn operator==(self: &mut wxSize, s1: &wxSize, s2: &wxSize) -> bool;
        // BLOCKED: fn operator!=(self: &mut wxSize, s1: &wxSize, s2: &wxSize) -> bool;
        // BLOCKED: fn operator+(self: &mut wxSize, s1: &wxSize, s2: &wxSize) -> wxSize;
        // BLOCKED: fn operator-(self: &mut wxSize, s1: &wxSize, s2: &wxSize) -> wxSize;
        // BLOCKED: fn operator+=(self: &mut wxSize, sz: &wxSize) -> &mut wxSize;
        // BLOCKED: fn operator-=(self: &mut wxSize, sz: &wxSize) -> &mut wxSize;
        // BLOCKED: fn operator/(self: &mut wxSize, sz: &wxSize, factor: i32) -> wxSize;
        // BLOCKED: fn operator*(self: &mut wxSize, sz: &wxSize, factor: i32) -> wxSize;
        // BLOCKED: fn operator*(self: &mut wxSize, factor: i32, sz: &wxSize) -> wxSize;
        // BLOCKED: fn operator/=(self: &mut wxSize, factor: i32) -> &mut wxSize;
        // BLOCKED: fn operator*=(self: &mut wxSize, factor: i32) -> &mut wxSize;
        // CTOR: fn wxSize() -> Size;
        // CTOR: fn wxSize(width: i32, height: i32) -> Size;
        // GENERATED: fn DecBy(self: &mut wxSize, pt: &wxPoint);
        // GENERATED: fn DecBy(self: &mut wxSize, size: &wxSize);
        // GENERATED: fn DecBy(self: &mut wxSize, dx: i32, dy: i32);
        // GENERATED: fn DecBy(self: &mut wxSize, d: i32);
        // GENERATED: fn DecTo(self: &mut wxSize, size: &wxSize);
        // GENERATED: fn DecToIfSpecified(self: &mut wxSize, size: &wxSize);
        // GENERATED: fn GetHeight(self: &wxSize) -> i32;
        // GENERATED: fn GetWidth(self: &wxSize) -> i32;
        // GENERATED: fn IncBy(self: &mut wxSize, pt: &wxPoint);
        // GENERATED: fn IncBy(self: &mut wxSize, size: &wxSize);
        // GENERATED: fn IncBy(self: &mut wxSize, dx: i32, dy: i32);
        // GENERATED: fn IncBy(self: &mut wxSize, d: i32);
        // GENERATED: fn IncTo(self: &mut wxSize, size: &wxSize);
        // GENERATED: fn IsFullySpecified(self: &wxSize) -> bool;
        // BLOCKED: fn Scale(self: &mut wxSize, xscale: f64, yscale: f64) -> &mut wxSize;
        // GENERATED: fn Set(self: &mut wxSize, width: i32, height: i32);
        // GENERATED: fn SetDefaults(self: &mut wxSize, size_default: &wxSize);
        // GENERATED: fn SetHeight(self: &mut wxSize, height: i32);
        // GENERATED: fn SetWidth(self: &mut wxSize, width: i32);
        
        // CLASS: wxValidator
        type wxValidator;
        // CTOR: fn wxValidator() -> Validator;
        // DTOR: fn ~wxValidator(self: Pin<&mut wxValidator>);
        // GENERATED: fn Clone(self: &wxValidator) -> *mut wxObject;
        // GENERATED: fn GetWindow(self: &wxValidator) -> *mut wxWindow;
        // GENERATED: unsafe fn SetWindow(self: Pin<&mut wxValidator>, window: *mut wxWindow);
        // GENERATED: fn TransferFromWindow(self: Pin<&mut wxValidator>) -> bool;
        // GENERATED: fn TransferToWindow(self: Pin<&mut wxValidator>) -> bool;
        // GENERATED: unsafe fn Validate(self: Pin<&mut wxValidator>, parent: *mut wxWindow) -> bool;
        // GENERATED: fn SuppressBellOnError(suppress: bool);
        // GENERATED: fn IsSilent() -> bool;
    }
    unsafe extern "C++" {
        // CLASS: wxObject
        fn NewObject() -> *mut wxObject;
        #[rust_name = "NewObject1"]
        fn NewObject(other: &wxObject) -> *mut wxObject;
        fn wxObject_GetClassInfo(self_: &wxObject) -> *mut wxClassInfo;
        fn wxObject_GetRefData(self_: &wxObject) -> *mut wxObjectRefData;
        unsafe fn wxObject_IsKindOf(self_: &wxObject, info: *const wxClassInfo) -> bool;
        fn wxObject_IsSameAs(self_: &wxObject, obj: &wxObject) -> bool;
        fn wxObject_Ref(self_: Pin<&mut wxObject>, clone: &wxObject);
        unsafe fn wxObject_SetRefData(self_: Pin<&mut wxObject>, data: *mut wxObjectRefData);
        fn wxObject_UnRef(self_: Pin<&mut wxObject>);
        fn wxObject_UnShare(self_: Pin<&mut wxObject>);
        // CLASS: wxEvtHandler
        unsafe fn wxEvtHandler_QueueEvent(self_: Pin<&mut wxEvtHandler>, event: *mut wxEvent);
        fn wxEvtHandler_AddPendingEvent(self_: Pin<&mut wxEvtHandler>, event: &wxEvent);
        fn wxEvtHandler_ProcessEvent(self_: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        fn wxEvtHandler_ProcessEventLocally(self_: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        fn wxEvtHandler_SafelyProcessEvent(self_: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        fn wxEvtHandler_ProcessPendingEvents(self_: Pin<&mut wxEvtHandler>);
        fn wxEvtHandler_DeletePendingEvents(self_: Pin<&mut wxEvtHandler>);
        fn wxEvtHandler_GetClientObject(self_: &wxEvtHandler) -> *mut wxClientData;
        unsafe fn wxEvtHandler_SetClientObject(self_: Pin<&mut wxEvtHandler>, data: *mut wxClientData);
        fn wxEvtHandler_GetEvtHandlerEnabled(self_: &wxEvtHandler) -> bool;
        fn wxEvtHandler_GetNextHandler(self_: &wxEvtHandler) -> *mut wxEvtHandler;
        fn wxEvtHandler_GetPreviousHandler(self_: &wxEvtHandler) -> *mut wxEvtHandler;
        fn wxEvtHandler_SetEvtHandlerEnabled(self_: Pin<&mut wxEvtHandler>, enabled: bool);
        unsafe fn wxEvtHandler_SetNextHandler(self_: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        unsafe fn wxEvtHandler_SetPreviousHandler(self_: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        fn wxEvtHandler_Unlink(self_: Pin<&mut wxEvtHandler>);
        fn wxEvtHandler_IsUnlinked(self_: &wxEvtHandler) -> bool;
        unsafe fn wxEvtHandler_AddFilter(filter: *mut wxEventFilter);
        unsafe fn wxEvtHandler_RemoveFilter(filter: *mut wxEventFilter);
        fn NewEvtHandler() -> *mut wxEvtHandler;
        // CLASS: wxWindow
        fn wxWindow_AcceptsFocus(self_: &wxWindow) -> bool;
        fn wxWindow_AcceptsFocusFromKeyboard(self_: &wxWindow) -> bool;
        fn wxWindow_AcceptsFocusRecursively(self_: &wxWindow) -> bool;
        fn wxWindow_DisableFocusFromKeyboard(self_: Pin<&mut wxWindow>);
        fn wxWindow_IsFocusable(self_: &wxWindow) -> bool;
        fn wxWindow_CanAcceptFocus(self_: &wxWindow) -> bool;
        fn wxWindow_CanAcceptFocusFromKeyboard(self_: &wxWindow) -> bool;
        fn wxWindow_HasFocus(self_: &wxWindow) -> bool;
        fn wxWindow_SetCanFocus(self_: Pin<&mut wxWindow>, can_focus: bool);
        fn wxWindow_EnableVisibleFocus(self_: Pin<&mut wxWindow>, enable: bool);
        fn wxWindow_SetFocus(self_: Pin<&mut wxWindow>);
        fn wxWindow_SetFocusFromKbd(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_AddChild(self_: Pin<&mut wxWindow>, child: *mut wxWindow);
        fn wxWindow_DestroyChildren(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_FindWindow(self_: &wxWindow, id: i32) -> *mut wxWindow;
        #[rust_name = "wxWindow_FindWindow1"]
        fn wxWindow_FindWindow(self_: &wxWindow, name: &wxString) -> *mut wxWindow;
        unsafe fn wxWindow_RemoveChild(self_: Pin<&mut wxWindow>, child: *mut wxWindow);
        fn wxWindow_GetGrandParent(self_: &wxWindow) -> *mut wxWindow;
        fn wxWindow_GetNextSibling(self_: &wxWindow) -> *mut wxWindow;
        fn wxWindow_GetParent(self_: &wxWindow) -> *mut wxWindow;
        fn wxWindow_GetPrevSibling(self_: &wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_IsDescendant(self_: &wxWindow, win: *mut wxWindow) -> bool;
        unsafe fn wxWindow_Reparent(self_: Pin<&mut wxWindow>, new_parent: *mut wxWindow) -> bool;
        fn wxWindow_AlwaysShowScrollbars(self_: Pin<&mut wxWindow>, hflag: bool, vflag: bool);
        fn wxWindow_GetScrollPos(self_: &wxWindow, orientation: i32) -> i32;
        fn wxWindow_GetScrollRange(self_: &wxWindow, orientation: i32) -> i32;
        fn wxWindow_GetScrollThumb(self_: &wxWindow, orientation: i32) -> i32;
        fn wxWindow_CanScroll(self_: &wxWindow, orient: i32) -> bool;
        fn wxWindow_HasScrollbar(self_: &wxWindow, orient: i32) -> bool;
        fn wxWindow_IsScrollbarAlwaysShown(self_: &wxWindow, orient: i32) -> bool;
        fn wxWindow_ScrollLines(self_: Pin<&mut wxWindow>, lines: i32) -> bool;
        fn wxWindow_ScrollPages(self_: Pin<&mut wxWindow>, pages: i32) -> bool;
        unsafe fn wxWindow_ScrollWindow(self_: Pin<&mut wxWindow>, dx: i32, dy: i32, rect: *const wxRect);
        fn wxWindow_LineUp(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_LineDown(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_PageUp(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_PageDown(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_SetScrollPos(self_: Pin<&mut wxWindow>, orientation: i32, pos: i32, refresh: bool);
        fn wxWindow_SetScrollbar(self_: Pin<&mut wxWindow>, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        fn wxWindow_BeginRepositioningChildren(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_EndRepositioningChildren(self_: Pin<&mut wxWindow>);
        fn wxWindow_CacheBestSize(self_: &wxWindow, size: &wxSize);
        fn wxWindow_ClientToWindowSize(self_: &wxWindow, size: &wxSize) -> wxSize;
        fn wxWindow_WindowToClientSize(self_: &wxWindow, size: &wxSize) -> wxSize;
        fn wxWindow_Fit(self_: Pin<&mut wxWindow>);
        fn wxWindow_FitInside(self_: Pin<&mut wxWindow>);
        fn wxWindow_FromDIP(self_: &wxWindow, sz: &wxSize) -> wxSize;
        #[rust_name = "wxWindow_FromDIP1"]
        fn wxWindow_FromDIP(self_: &wxWindow, pt: &wxPoint) -> wxPoint;
        #[rust_name = "wxWindow_FromDIP2"]
        fn wxWindow_FromDIP(self_: &wxWindow, d: i32) -> i32;
        fn wxWindow_ToDIP(self_: &wxWindow, sz: &wxSize) -> wxSize;
        #[rust_name = "wxWindow_ToDIP1"]
        fn wxWindow_ToDIP(self_: &wxWindow, pt: &wxPoint) -> wxPoint;
        #[rust_name = "wxWindow_ToDIP2"]
        fn wxWindow_ToDIP(self_: &wxWindow, d: i32) -> i32;
        fn wxWindow_GetBestSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetBestHeight(self_: &wxWindow, width: i32) -> i32;
        fn wxWindow_GetBestWidth(self_: &wxWindow, height: i32) -> i32;
        unsafe fn wxWindow_GetClientSize(self_: &wxWindow, width: *mut i32, height: *mut i32);
        #[rust_name = "wxWindow_GetClientSize1"]
        fn wxWindow_GetClientSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetEffectiveMinSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetMaxClientSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetMaxSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetMinClientSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetMinSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetMinWidth(self_: &wxWindow) -> i32;
        fn wxWindow_GetMinHeight(self_: &wxWindow) -> i32;
        fn wxWindow_GetMaxWidth(self_: &wxWindow) -> i32;
        fn wxWindow_GetMaxHeight(self_: &wxWindow) -> i32;
        unsafe fn wxWindow_GetSize(self_: &wxWindow, width: *mut i32, height: *mut i32);
        #[rust_name = "wxWindow_GetSize1"]
        fn wxWindow_GetSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetVirtualSize(self_: &wxWindow) -> wxSize;
        #[rust_name = "wxWindow_GetVirtualSize1"]
        unsafe fn wxWindow_GetVirtualSize(self_: &wxWindow, width: *mut i32, height: *mut i32);
        fn wxWindow_GetBestVirtualSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_GetContentScaleFactor(self_: &wxWindow) -> f64;
        fn wxWindow_GetDPIScaleFactor(self_: &wxWindow) -> f64;
        fn wxWindow_GetWindowBorderSize(self_: &wxWindow) -> wxSize;
        fn wxWindow_InformFirstDirection(self_: Pin<&mut wxWindow>, direction: i32, size: i32, available_other_dir: i32) -> bool;
        fn wxWindow_InvalidateBestSize(self_: Pin<&mut wxWindow>);
        fn wxWindow_PostSizeEvent(self_: Pin<&mut wxWindow>);
        fn wxWindow_PostSizeEventToParent(self_: Pin<&mut wxWindow>);
        fn wxWindow_SendSizeEvent(self_: Pin<&mut wxWindow>, flags: i32);
        fn wxWindow_SendSizeEventToParent(self_: Pin<&mut wxWindow>, flags: i32);
        fn wxWindow_SetClientSize(self_: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "wxWindow_SetClientSize1"]
        fn wxWindow_SetClientSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "wxWindow_SetClientSize2"]
        fn wxWindow_SetClientSize(self_: Pin<&mut wxWindow>, rect: &wxRect);
        unsafe fn wxWindow_SetContainingSizer(self_: Pin<&mut wxWindow>, sizer: *mut wxSizer);
        fn wxWindow_SetInitialSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        fn wxWindow_SetMaxClientSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        fn wxWindow_SetMaxSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        fn wxWindow_SetMinClientSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        fn wxWindow_SetMinSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        #[rust_name = "wxWindow_SetSize1"]
        fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, rect: &wxRect);
        #[rust_name = "wxWindow_SetSize2"]
        fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "wxWindow_SetSize3"]
        fn wxWindow_SetSize(self_: Pin<&mut wxWindow>, width: i32, height: i32);
        fn wxWindow_SetSizeHints(self_: Pin<&mut wxWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        #[rust_name = "wxWindow_SetSizeHints1"]
        fn wxWindow_SetSizeHints(self_: Pin<&mut wxWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        fn wxWindow_SetVirtualSize(self_: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "wxWindow_SetVirtualSize1"]
        fn wxWindow_SetVirtualSize(self_: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "wxWindow_FromDIP3"]
        unsafe fn wxWindow_FromDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        #[rust_name = "wxWindow_FromDIP4"]
        unsafe fn wxWindow_FromDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        #[rust_name = "wxWindow_FromDIP5"]
        unsafe fn wxWindow_FromDIP(d: i32, w: *const wxWindow) -> i32;
        #[rust_name = "wxWindow_ToDIP3"]
        unsafe fn wxWindow_ToDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        #[rust_name = "wxWindow_ToDIP4"]
        unsafe fn wxWindow_ToDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        #[rust_name = "wxWindow_ToDIP5"]
        unsafe fn wxWindow_ToDIP(d: i32, w: *const wxWindow) -> i32;
        fn wxWindow_Center(self_: Pin<&mut wxWindow>, dir: i32);
        fn wxWindow_CenterOnParent(self_: Pin<&mut wxWindow>, dir: i32);
        fn wxWindow_Centre(self_: Pin<&mut wxWindow>, direction: i32);
        fn wxWindow_CentreOnParent(self_: Pin<&mut wxWindow>, direction: i32);
        unsafe fn wxWindow_GetPosition(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_GetPosition1"]
        fn wxWindow_GetPosition(self_: &wxWindow) -> wxPoint;
        fn wxWindow_GetRect(self_: &wxWindow) -> wxRect;
        unsafe fn wxWindow_GetScreenPosition(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_GetScreenPosition1"]
        fn wxWindow_GetScreenPosition(self_: &wxWindow) -> wxPoint;
        fn wxWindow_GetScreenRect(self_: &wxWindow) -> wxRect;
        fn wxWindow_GetClientAreaOrigin(self_: &wxWindow) -> wxPoint;
        fn wxWindow_GetClientRect(self_: &wxWindow) -> wxRect;
        fn wxWindow_Move(self_: Pin<&mut wxWindow>, x: i32, y: i32, flags: i32);
        #[rust_name = "wxWindow_Move1"]
        fn wxWindow_Move(self_: Pin<&mut wxWindow>, pt: &wxPoint, flags: i32);
        fn wxWindow_SetPosition(self_: Pin<&mut wxWindow>, pt: &wxPoint);
        unsafe fn wxWindow_ClientToScreen(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_ClientToScreen1"]
        fn wxWindow_ClientToScreen(self_: &wxWindow, pt: &wxPoint) -> wxPoint;
        fn wxWindow_ConvertDialogToPixels(self_: &wxWindow, pt: &wxPoint) -> wxPoint;
        #[rust_name = "wxWindow_ConvertDialogToPixels1"]
        fn wxWindow_ConvertDialogToPixels(self_: &wxWindow, sz: &wxSize) -> wxSize;
        fn wxWindow_ConvertPixelsToDialog(self_: &wxWindow, pt: &wxPoint) -> wxPoint;
        #[rust_name = "wxWindow_ConvertPixelsToDialog1"]
        fn wxWindow_ConvertPixelsToDialog(self_: &wxWindow, sz: &wxSize) -> wxSize;
        unsafe fn wxWindow_ScreenToClient(self_: &wxWindow, x: *mut i32, y: *mut i32);
        #[rust_name = "wxWindow_ScreenToClient1"]
        fn wxWindow_ScreenToClient(self_: &wxWindow, pt: &wxPoint) -> wxPoint;
        fn wxWindow_ClearBackground(self_: Pin<&mut wxWindow>);
        fn wxWindow_Freeze(self_: Pin<&mut wxWindow>);
        fn wxWindow_Thaw(self_: Pin<&mut wxWindow>);
        fn wxWindow_IsFrozen(self_: &wxWindow) -> bool;
        fn wxWindow_GetCharHeight(self_: &wxWindow) -> i32;
        fn wxWindow_GetCharWidth(self_: &wxWindow) -> i32;
        fn wxWindow_GetDPI(self_: &wxWindow) -> wxSize;
        unsafe fn wxWindow_GetTextExtent(self_: &wxWindow, string: &wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        #[rust_name = "wxWindow_GetTextExtent1"]
        fn wxWindow_GetTextExtent(self_: &wxWindow, string: &wxString) -> wxSize;
        fn wxWindow_GetUpdateClientRect(self_: &wxWindow) -> wxRect;
        fn wxWindow_HasTransparentBackground(self_: Pin<&mut wxWindow>) -> bool;
        unsafe fn wxWindow_Refresh(self_: Pin<&mut wxWindow>, erase_background: bool, rect: *const wxRect);
        fn wxWindow_RefreshRect(self_: Pin<&mut wxWindow>, rect: &wxRect, erase_background: bool);
        fn wxWindow_Update(self_: Pin<&mut wxWindow>);
        fn wxWindow_SetBackgroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        unsafe fn wxWindow_IsTransparentBackgroundSupported(self_: &wxWindow, reason: *mut wxString) -> bool;
        fn wxWindow_SetFont(self_: Pin<&mut wxWindow>, font: &wxFont) -> bool;
        fn wxWindow_SetForegroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        fn wxWindow_SetOwnBackgroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour);
        fn wxWindow_InheritsBackgroundColour(self_: &wxWindow) -> bool;
        fn wxWindow_UseBgCol(self_: &wxWindow) -> bool;
        fn wxWindow_UseBackgroundColour(self_: &wxWindow) -> bool;
        fn wxWindow_SetOwnFont(self_: Pin<&mut wxWindow>, font: &wxFont);
        fn wxWindow_SetOwnForegroundColour(self_: Pin<&mut wxWindow>, colour: &wxColour);
        fn wxWindow_UseForegroundColour(self_: &wxWindow) -> bool;
        fn wxWindow_InheritsForegroundColour(self_: &wxWindow) -> bool;
        fn wxWindow_SetPalette(self_: Pin<&mut wxWindow>, pal: &wxPalette);
        fn wxWindow_ShouldInheritColours(self_: &wxWindow) -> bool;
        fn wxWindow_SetThemeEnabled(self_: Pin<&mut wxWindow>, enable: bool);
        fn wxWindow_GetThemeEnabled(self_: &wxWindow) -> bool;
        fn wxWindow_CanSetTransparent(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_SetTransparent(self_: Pin<&mut wxWindow>, alpha: u8) -> bool;
        fn wxWindow_GetEventHandler(self_: &wxWindow) -> *mut wxEvtHandler;
        fn wxWindow_HandleAsNavigationKey(self_: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        fn wxWindow_HandleWindowEvent(self_: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        fn wxWindow_ProcessWindowEvent(self_: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn wxWindow_ProcessWindowEventLocally(self_: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn wxWindow_PopEventHandler(self_: Pin<&mut wxWindow>, delete_handler: bool) -> *mut wxEvtHandler;
        unsafe fn wxWindow_PushEventHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_RemoveEventHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler) -> bool;
        unsafe fn wxWindow_SetEventHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_SetNextHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn wxWindow_SetPreviousHandler(self_: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        fn wxWindow_GetExtraStyle(self_: &wxWindow) -> i32;
        fn wxWindow_GetWindowStyleFlag(self_: &wxWindow) -> i32;
        fn wxWindow_GetWindowStyle(self_: &wxWindow) -> i32;
        fn wxWindow_HasExtraStyle(self_: &wxWindow, ex_flag: i32) -> bool;
        fn wxWindow_HasFlag(self_: &wxWindow, flag: i32) -> bool;
        fn wxWindow_SetExtraStyle(self_: Pin<&mut wxWindow>, ex_style: i32);
        fn wxWindow_SetWindowStyleFlag(self_: Pin<&mut wxWindow>, style: i32);
        fn wxWindow_SetWindowStyle(self_: Pin<&mut wxWindow>, style: i32);
        fn wxWindow_ToggleWindowStyle(self_: Pin<&mut wxWindow>, flag: i32) -> bool;
        unsafe fn wxWindow_MoveAfterInTabOrder(self_: Pin<&mut wxWindow>, win: *mut wxWindow);
        unsafe fn wxWindow_MoveBeforeInTabOrder(self_: Pin<&mut wxWindow>, win: *mut wxWindow);
        fn wxWindow_Navigate(self_: Pin<&mut wxWindow>, flags: i32) -> bool;
        fn wxWindow_NavigateIn(self_: Pin<&mut wxWindow>, flags: i32) -> bool;
        fn wxWindow_Lower(self_: Pin<&mut wxWindow>);
        fn wxWindow_Raise(self_: Pin<&mut wxWindow>);
        fn wxWindow_Hide(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_IsEnabled(self_: &wxWindow) -> bool;
        fn wxWindow_IsExposed(self_: &wxWindow, x: i32, y: i32) -> bool;
        #[rust_name = "wxWindow_IsExposed1"]
        fn wxWindow_IsExposed(self_: &wxWindow, pt: &mut wxPoint) -> bool;
        #[rust_name = "wxWindow_IsExposed2"]
        fn wxWindow_IsExposed(self_: &wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        #[rust_name = "wxWindow_IsExposed3"]
        fn wxWindow_IsExposed(self_: &wxWindow, rect: &mut wxRect) -> bool;
        fn wxWindow_IsShown(self_: &wxWindow) -> bool;
        fn wxWindow_IsShownOnScreen(self_: &wxWindow) -> bool;
        fn wxWindow_Disable(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_Enable(self_: Pin<&mut wxWindow>, enable: bool) -> bool;
        fn wxWindow_Show(self_: Pin<&mut wxWindow>, show: bool) -> bool;
        fn wxWindow_GetHelpText(self_: &wxWindow) -> *mut wxString;
        fn wxWindow_SetHelpText(self_: Pin<&mut wxWindow>, help_text: &wxString);
        fn wxWindow_GetToolTip(self_: &wxWindow) -> *mut wxToolTip;
        fn wxWindow_GetToolTipText(self_: &wxWindow) -> *mut wxString;
        fn wxWindow_SetToolTip(self_: Pin<&mut wxWindow>, tip_string: &wxString);
        #[rust_name = "wxWindow_SetToolTip1"]
        unsafe fn wxWindow_SetToolTip(self_: Pin<&mut wxWindow>, tip: *mut wxToolTip);
        fn wxWindow_UnsetToolTip(self_: Pin<&mut wxWindow>);
        fn wxWindow_GetPopupMenuSelectionFromUser(self_: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, pos: &wxPoint) -> i32;
        #[rust_name = "wxWindow_GetPopupMenuSelectionFromUser1"]
        fn wxWindow_GetPopupMenuSelectionFromUser(self_: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, x: i32, y: i32) -> i32;
        unsafe fn wxWindow_PopupMenu(self_: Pin<&mut wxWindow>, menu: *mut wxMenu, pos: &wxPoint) -> bool;
        #[rust_name = "wxWindow_PopupMenu1"]
        unsafe fn wxWindow_PopupMenu(self_: Pin<&mut wxWindow>, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        fn wxWindow_GetValidator(self_: Pin<&mut wxWindow>) -> *mut wxValidator;
        fn wxWindow_SetValidator(self_: Pin<&mut wxWindow>, validator: &wxValidator);
        fn wxWindow_TransferDataFromWindow(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_TransferDataToWindow(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_Validate(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_GetId(self_: &wxWindow) -> i32;
        fn wxWindow_GetLabel(self_: &wxWindow) -> *mut wxString;
        fn wxWindow_AdjustForLayoutDirection(self_: &wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        fn wxWindow_GetName(self_: &wxWindow) -> *mut wxString;
        fn wxWindow_SetId(self_: Pin<&mut wxWindow>, winid: i32);
        fn wxWindow_SetLabel(self_: Pin<&mut wxWindow>, label: &wxString);
        fn wxWindow_SetName(self_: Pin<&mut wxWindow>, name: &wxString);
        fn wxWindow_GetAcceleratorTable(self_: Pin<&mut wxWindow>) -> *mut wxAcceleratorTable;
        fn wxWindow_SetAcceleratorTable(self_: Pin<&mut wxWindow>, accel: &wxAcceleratorTable);
        fn wxWindow_Close(self_: Pin<&mut wxWindow>, force: bool) -> bool;
        fn wxWindow_Destroy(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_IsBeingDeleted(self_: &wxWindow) -> bool;
        fn wxWindow_GetDropTarget(self_: &wxWindow) -> *mut wxDropTarget;
        unsafe fn wxWindow_SetDropTarget(self_: Pin<&mut wxWindow>, target: *mut wxDropTarget);
        fn wxWindow_DragAcceptFiles(self_: Pin<&mut wxWindow>, accept: bool);
        fn wxWindow_GetContainingSizer(self_: &wxWindow) -> *mut wxSizer;
        fn wxWindow_GetSizer(self_: &wxWindow) -> *mut wxSizer;
        unsafe fn wxWindow_SetSizer(self_: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        unsafe fn wxWindow_SetSizerAndFit(self_: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        fn wxWindow_GetConstraints(self_: &wxWindow) -> *mut wxLayoutConstraints;
        unsafe fn wxWindow_SetConstraints(self_: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        fn wxWindow_Layout(self_: Pin<&mut wxWindow>) -> bool;
        fn wxWindow_SetAutoLayout(self_: Pin<&mut wxWindow>, auto_layout: bool);
        fn wxWindow_GetAutoLayout(self_: &wxWindow) -> bool;
        fn wxWindow_CaptureMouse(self_: Pin<&mut wxWindow>);
        fn wxWindow_GetCaret(self_: &wxWindow) -> *mut wxCaret;
        fn wxWindow_HasCapture(self_: &wxWindow) -> bool;
        fn wxWindow_ReleaseMouse(self_: Pin<&mut wxWindow>);
        unsafe fn wxWindow_SetCaret(self_: Pin<&mut wxWindow>, caret: *mut wxCaret);
        fn wxWindow_SetCursor(self_: Pin<&mut wxWindow>, cursor: &wxCursor) -> bool;
        fn wxWindow_WarpPointer(self_: Pin<&mut wxWindow>, x: i32, y: i32);
        fn wxWindow_EnableTouchEvents(self_: Pin<&mut wxWindow>, events_mask: i32) -> bool;
        fn wxWindow_DoUpdateWindowUI(self_: Pin<&mut wxWindow>, event: Pin<&mut wxUpdateUIEvent>);
        fn wxWindow_HasMultiplePages(self_: &wxWindow) -> bool;
        fn wxWindow_InheritAttributes(self_: Pin<&mut wxWindow>);
        fn wxWindow_InitDialog(self_: Pin<&mut wxWindow>);
        fn wxWindow_IsDoubleBuffered(self_: &wxWindow) -> bool;
        fn wxWindow_SetDoubleBuffered(self_: Pin<&mut wxWindow>, on: bool);
        fn wxWindow_IsRetained(self_: &wxWindow) -> bool;
        fn wxWindow_IsThisEnabled(self_: &wxWindow) -> bool;
        fn wxWindow_IsTopLevel(self_: &wxWindow) -> bool;
        fn wxWindow_OnInternalIdle(self_: Pin<&mut wxWindow>);
        fn wxWindow_SendIdleEvents(self_: Pin<&mut wxWindow>, event: Pin<&mut wxIdleEvent>) -> bool;
        fn wxWindow_RegisterHotKey(self_: Pin<&mut wxWindow>, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        fn wxWindow_UnregisterHotKey(self_: Pin<&mut wxWindow>, hotkey_id: i32) -> bool;
        fn wxWindow_UpdateWindowUI(self_: Pin<&mut wxWindow>, flags: i32);
        fn wxWindow_FindFocus() -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        unsafe fn wxWindow_FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        fn wxWindow_GetCapture() -> *mut wxWindow;
        fn wxWindow_NewControlId(count: i32) -> i32;
        fn wxWindow_UnreserveControlId(id: i32, count: i32);
        fn NewWindow() -> *mut wxWindow;
        #[rust_name = "NewWindow1"]
        unsafe fn NewWindow(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxWindow;
        unsafe fn wxWindow_Create(self_: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // CLASS: wxControl
        unsafe fn NewControl(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxControl;
        #[rust_name = "NewControl1"]
        fn NewControl() -> *mut wxControl;
        unsafe fn wxControl_Create(self_: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        fn wxControl_Command(self_: Pin<&mut wxControl>, event: Pin<&mut wxCommandEvent>);
        fn wxControl_GetLabel(self_: &wxControl) -> *mut wxString;
        fn wxControl_GetLabelText(self_: &wxControl) -> *mut wxString;
        fn wxControl_GetSizeFromTextSize(self_: &wxControl, xlen: i32, ylen: i32) -> wxSize;
        #[rust_name = "wxControl_GetSizeFromTextSize1"]
        fn wxControl_GetSizeFromTextSize(self_: &wxControl, tsize: &wxSize) -> wxSize;
        fn wxControl_GetSizeFromText(self_: &wxControl, text: &wxString) -> wxSize;
        fn wxControl_SetLabel(self_: Pin<&mut wxControl>, label: &wxString);
        fn wxControl_SetLabelText(self_: Pin<&mut wxControl>, text: &wxString);
        fn wxControl_SetLabelMarkup(self_: Pin<&mut wxControl>, markup: &wxString) -> bool;
        #[rust_name = "wxControl_GetLabelText1"]
        fn wxControl_GetLabelText(label: &wxString) -> *mut wxString;
        fn wxControl_RemoveMnemonics(str: &wxString) -> *mut wxString;
        fn wxControl_EscapeMnemonics(text: &wxString) -> *mut wxString;
        // CLASS: wxAnyButton
        fn NewAnyButton() -> *mut wxAnyButton;
        fn wxAnyButton_SetBitmapCurrent(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn wxAnyButton_SetBitmapDisabled(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn wxAnyButton_SetBitmapFocus(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn wxAnyButton_SetBitmapLabel(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn wxAnyButton_SetBitmapPressed(self_: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn wxAnyButton_GetBitmapMargins(self_: Pin<&mut wxAnyButton>) -> wxSize;
        fn wxAnyButton_SetBitmapMargins(self_: Pin<&mut wxAnyButton>, x: i32, y: i32);
        #[rust_name = "wxAnyButton_SetBitmapMargins1"]
        fn wxAnyButton_SetBitmapMargins(self_: Pin<&mut wxAnyButton>, sz: &wxSize);
        // CLASS: wxButton
        fn NewButton() -> *mut wxButton;
        #[rust_name = "NewButton1"]
        unsafe fn NewButton(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxButton;
        unsafe fn wxButton_Create(self_: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        fn wxButton_GetAuthNeeded(self_: &wxButton) -> bool;
        fn wxButton_GetLabel(self_: &wxButton) -> *mut wxString;
        fn wxButton_SetAuthNeeded(self_: Pin<&mut wxButton>, needed: bool);
        fn wxButton_SetDefault(self_: Pin<&mut wxButton>) -> *mut wxWindow;
        fn wxButton_SetLabel(self_: Pin<&mut wxButton>, label: &wxString);
        unsafe fn wxButton_GetDefaultSize(win: *mut wxWindow) -> wxSize;
        // CLASS: wxNonOwnedWindow
        fn wxNonOwnedWindow_SetShape(self_: Pin<&mut wxNonOwnedWindow>, region: &wxRegion) -> bool;
        #[rust_name = "wxNonOwnedWindow_SetShape1"]
        fn wxNonOwnedWindow_SetShape(self_: Pin<&mut wxNonOwnedWindow>, path: &wxGraphicsPath) -> bool;
        // CLASS: wxTopLevelWindow
        fn NewTopLevelWindow() -> *mut wxTopLevelWindow;
        #[rust_name = "NewTopLevelWindow1"]
        unsafe fn NewTopLevelWindow(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxTopLevelWindow;
        unsafe fn wxTopLevelWindow_Create(self_: Pin<&mut wxTopLevelWindow>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        fn wxTopLevelWindow_CanSetTransparent(self_: Pin<&mut wxTopLevelWindow>) -> bool;
        fn wxTopLevelWindow_CenterOnScreen(self_: Pin<&mut wxTopLevelWindow>, direction: i32);
        fn wxTopLevelWindow_CentreOnScreen(self_: Pin<&mut wxTopLevelWindow>, direction: i32);
        fn wxTopLevelWindow_EnableCloseButton(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn wxTopLevelWindow_EnableMaximizeButton(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn wxTopLevelWindow_EnableMinimizeButton(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn wxTopLevelWindow_GetDefaultItem(self_: &wxTopLevelWindow) -> *mut wxWindow;
        fn wxTopLevelWindow_GetTitle(self_: &wxTopLevelWindow) -> *mut wxString;
        fn wxTopLevelWindow_Iconize(self_: Pin<&mut wxTopLevelWindow>, iconize: bool);
        fn wxTopLevelWindow_IsActive(self_: Pin<&mut wxTopLevelWindow>) -> bool;
        fn wxTopLevelWindow_IsAlwaysMaximized(self_: &wxTopLevelWindow) -> bool;
        fn wxTopLevelWindow_IsFullScreen(self_: &wxTopLevelWindow) -> bool;
        fn wxTopLevelWindow_IsIconized(self_: &wxTopLevelWindow) -> bool;
        fn wxTopLevelWindow_IsMaximized(self_: &wxTopLevelWindow) -> bool;
        fn wxTopLevelWindow_Layout(self_: Pin<&mut wxTopLevelWindow>) -> bool;
        fn wxTopLevelWindow_Maximize(self_: Pin<&mut wxTopLevelWindow>, maximize: bool);
        fn wxTopLevelWindow_RequestUserAttention(self_: Pin<&mut wxTopLevelWindow>, flags: i32);
        fn wxTopLevelWindow_Restore(self_: Pin<&mut wxTopLevelWindow>);
        unsafe fn wxTopLevelWindow_SetDefaultItem(self_: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        unsafe fn wxTopLevelWindow_SetTmpDefaultItem(self_: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        fn wxTopLevelWindow_GetTmpDefaultItem(self_: &wxTopLevelWindow) -> *mut wxWindow;
        fn wxTopLevelWindow_SetIcon(self_: Pin<&mut wxTopLevelWindow>, icon: &wxIcon);
        fn wxTopLevelWindow_SetIcons(self_: Pin<&mut wxTopLevelWindow>, icons: &wxIconBundle);
        fn wxTopLevelWindow_SetMaxSize(self_: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        fn wxTopLevelWindow_SetMinSize(self_: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        fn wxTopLevelWindow_SetSizeHints(self_: Pin<&mut wxTopLevelWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        #[rust_name = "wxTopLevelWindow_SetSizeHints1"]
        fn wxTopLevelWindow_SetSizeHints(self_: Pin<&mut wxTopLevelWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        fn wxTopLevelWindow_SetTitle(self_: Pin<&mut wxTopLevelWindow>, title: &wxString);
        fn wxTopLevelWindow_SetTransparent(self_: Pin<&mut wxTopLevelWindow>, alpha: u8) -> bool;
        fn wxTopLevelWindow_ShouldPreventAppExit(self_: &wxTopLevelWindow) -> bool;
        fn wxTopLevelWindow_OSXSetModified(self_: Pin<&mut wxTopLevelWindow>, modified: bool);
        fn wxTopLevelWindow_OSXIsModified(self_: &wxTopLevelWindow) -> bool;
        fn wxTopLevelWindow_SetRepresentedFilename(self_: Pin<&mut wxTopLevelWindow>, filename: &wxString);
        fn wxTopLevelWindow_ShowWithoutActivating(self_: Pin<&mut wxTopLevelWindow>);
        fn wxTopLevelWindow_EnableFullScreenView(self_: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn wxTopLevelWindow_ShowFullScreen(self_: Pin<&mut wxTopLevelWindow>, show: bool, style: i32) -> bool;
        fn wxTopLevelWindow_GetDefaultSize() -> wxSize;
        // CLASS: wxFrame
        fn NewFrame() -> *mut wxFrame;
        #[rust_name = "NewFrame1"]
        unsafe fn NewFrame(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxFrame;
        fn wxFrame_Centre(self_: Pin<&mut wxFrame>, direction: i32);
        unsafe fn wxFrame_Create(self_: Pin<&mut wxFrame>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        fn wxFrame_CreateStatusBar(self_: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        fn wxFrame_CreateToolBar(self_: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        fn wxFrame_DoGiveHelp(self_: Pin<&mut wxFrame>, text: &wxString, show: bool);
        fn wxFrame_GetClientAreaOrigin(self_: &wxFrame) -> wxPoint;
        fn wxFrame_GetMenuBar(self_: &wxFrame) -> *mut wxMenuBar;
        fn wxFrame_GetStatusBar(self_: &wxFrame) -> *mut wxStatusBar;
        fn wxFrame_GetStatusBarPane(self_: &wxFrame) -> i32;
        fn wxFrame_GetToolBar(self_: &wxFrame) -> *mut wxToolBar;
        fn wxFrame_OnCreateStatusBar(self_: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        fn wxFrame_OnCreateToolBar(self_: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        fn wxFrame_ProcessCommand(self_: Pin<&mut wxFrame>, id: i32) -> bool;
        unsafe fn wxFrame_SetMenuBar(self_: Pin<&mut wxFrame>, menu_bar: *mut wxMenuBar);
        unsafe fn wxFrame_SetStatusBar(self_: Pin<&mut wxFrame>, status_bar: *mut wxStatusBar);
        fn wxFrame_SetStatusBarPane(self_: Pin<&mut wxFrame>, n: i32);
        fn wxFrame_SetStatusText(self_: Pin<&mut wxFrame>, text: &wxString, number: i32);
        unsafe fn wxFrame_SetStatusWidths(self_: Pin<&mut wxFrame>, n: i32, widths_field: *const i32);
        unsafe fn wxFrame_SetToolBar(self_: Pin<&mut wxFrame>, tool_bar: *mut wxToolBar);
        fn wxFrame_PushStatusText(self_: Pin<&mut wxFrame>, text: &wxString, number: i32);
        fn wxFrame_PopStatusText(self_: Pin<&mut wxFrame>, number: i32);
        // CLASS: wxPoint
        fn wxPoint_IsFullySpecified(self_: &wxPoint) -> bool;
        fn wxPoint_SetDefaults(self_: &mut wxPoint, pt: &wxPoint);
        fn NewPoint() -> wxPoint;
        #[rust_name = "NewPoint1"]
        fn NewPoint(x: i32, y: i32) -> wxPoint;
        #[rust_name = "NewPoint2"]
        fn NewPoint(pt: &wxRealPoint) -> wxPoint;
        // CLASS: wxRect
        fn NewRect() -> wxRect;
        #[rust_name = "NewRect1"]
        fn NewRect(x: i32, y: i32, width: i32, height: i32) -> wxRect;
        #[rust_name = "NewRect2"]
        fn NewRect(top_left: &wxPoint, bottom_right: &wxPoint) -> wxRect;
        #[rust_name = "NewRect3"]
        fn NewRect(pos: &wxPoint, size: &wxSize) -> wxRect;
        #[rust_name = "NewRect4"]
        fn NewRect(size: &wxSize) -> wxRect;
        fn wxRect_CentreIn(self_: &wxRect, r: &wxRect, dir: i32) -> wxRect;
        fn wxRect_CenterIn(self_: &wxRect, r: &wxRect, dir: i32) -> wxRect;
        fn wxRect_Contains(self_: &wxRect, x: i32, y: i32) -> bool;
        #[rust_name = "wxRect_Contains1"]
        fn wxRect_Contains(self_: &wxRect, pt: &wxPoint) -> bool;
        #[rust_name = "wxRect_Contains2"]
        fn wxRect_Contains(self_: &wxRect, rect: &wxRect) -> bool;
        #[rust_name = "wxRect_Deflate3"]
        fn wxRect_Deflate(self_: &wxRect, dx: i32, dy: i32) -> wxRect;
        fn wxRect_GetBottom(self_: &wxRect) -> i32;
        fn wxRect_GetBottomLeft(self_: &wxRect) -> wxPoint;
        fn wxRect_GetBottomRight(self_: &wxRect) -> wxPoint;
        fn wxRect_GetHeight(self_: &wxRect) -> i32;
        fn wxRect_GetLeft(self_: &wxRect) -> i32;
        fn wxRect_GetPosition(self_: &wxRect) -> wxPoint;
        fn wxRect_GetRight(self_: &wxRect) -> i32;
        fn wxRect_GetSize(self_: &wxRect) -> wxSize;
        fn wxRect_GetTop(self_: &wxRect) -> i32;
        fn wxRect_GetTopLeft(self_: &wxRect) -> wxPoint;
        fn wxRect_GetTopRight(self_: &wxRect) -> wxPoint;
        fn wxRect_GetWidth(self_: &wxRect) -> i32;
        fn wxRect_GetX(self_: &wxRect) -> i32;
        fn wxRect_GetY(self_: &wxRect) -> i32;
        #[rust_name = "wxRect_Inflate3"]
        fn wxRect_Inflate(self_: &wxRect, dx: i32, dy: i32) -> wxRect;
        #[rust_name = "wxRect_Intersect1"]
        fn wxRect_Intersect(self_: &wxRect, rect: &wxRect) -> wxRect;
        fn wxRect_Intersects(self_: &wxRect, rect: &wxRect) -> bool;
        fn wxRect_IsEmpty(self_: &wxRect) -> bool;
        fn wxRect_Offset(self_: &mut wxRect, dx: i32, dy: i32);
        #[rust_name = "wxRect_Offset1"]
        fn wxRect_Offset(self_: &mut wxRect, pt: &wxPoint);
        fn wxRect_SetHeight(self_: &mut wxRect, height: i32);
        fn wxRect_SetPosition(self_: &mut wxRect, pos: &wxPoint);
        fn wxRect_SetSize(self_: &mut wxRect, s: &wxSize);
        fn wxRect_SetWidth(self_: &mut wxRect, width: i32);
        fn wxRect_SetX(self_: &mut wxRect, x: i32);
        fn wxRect_SetY(self_: &mut wxRect, y: i32);
        fn wxRect_SetLeft(self_: &mut wxRect, left: i32);
        fn wxRect_SetRight(self_: &mut wxRect, right: i32);
        fn wxRect_SetTop(self_: &mut wxRect, top: i32);
        fn wxRect_SetBottom(self_: &mut wxRect, bottom: i32);
        fn wxRect_SetTopLeft(self_: &mut wxRect, p: &wxPoint);
        fn wxRect_SetBottomRight(self_: &mut wxRect, p: &wxPoint);
        fn wxRect_SetTopRight(self_: &mut wxRect, p: &wxPoint);
        fn wxRect_SetBottomLeft(self_: &mut wxRect, p: &wxPoint);
        fn wxRect_Union(self_: &wxRect, rect: &wxRect) -> wxRect;
        // CLASS: wxSize
        fn NewSize() -> wxSize;
        #[rust_name = "NewSize1"]
        fn NewSize(width: i32, height: i32) -> wxSize;
        fn wxSize_DecBy(self_: &mut wxSize, pt: &wxPoint);
        #[rust_name = "wxSize_DecBy1"]
        fn wxSize_DecBy(self_: &mut wxSize, size: &wxSize);
        #[rust_name = "wxSize_DecBy2"]
        fn wxSize_DecBy(self_: &mut wxSize, dx: i32, dy: i32);
        #[rust_name = "wxSize_DecBy3"]
        fn wxSize_DecBy(self_: &mut wxSize, d: i32);
        fn wxSize_DecTo(self_: &mut wxSize, size: &wxSize);
        fn wxSize_DecToIfSpecified(self_: &mut wxSize, size: &wxSize);
        fn wxSize_GetHeight(self_: &wxSize) -> i32;
        fn wxSize_GetWidth(self_: &wxSize) -> i32;
        fn wxSize_IncBy(self_: &mut wxSize, pt: &wxPoint);
        #[rust_name = "wxSize_IncBy1"]
        fn wxSize_IncBy(self_: &mut wxSize, size: &wxSize);
        #[rust_name = "wxSize_IncBy2"]
        fn wxSize_IncBy(self_: &mut wxSize, dx: i32, dy: i32);
        #[rust_name = "wxSize_IncBy3"]
        fn wxSize_IncBy(self_: &mut wxSize, d: i32);
        fn wxSize_IncTo(self_: &mut wxSize, size: &wxSize);
        fn wxSize_IsFullySpecified(self_: &wxSize) -> bool;
        fn wxSize_Set(self_: &mut wxSize, width: i32, height: i32);
        fn wxSize_SetDefaults(self_: &mut wxSize, size_default: &wxSize);
        fn wxSize_SetHeight(self_: &mut wxSize, height: i32);
        fn wxSize_SetWidth(self_: &mut wxSize, width: i32);
        // CLASS: wxValidator
        fn NewValidator() -> *mut wxValidator;
        fn wxValidator_Clone(self_: &wxValidator) -> *mut wxObject;
        fn wxValidator_GetWindow(self_: &wxValidator) -> *mut wxWindow;
        unsafe fn wxValidator_SetWindow(self_: Pin<&mut wxValidator>, window: *mut wxWindow);
        fn wxValidator_TransferFromWindow(self_: Pin<&mut wxValidator>) -> bool;
        fn wxValidator_TransferToWindow(self_: Pin<&mut wxValidator>) -> bool;
        unsafe fn wxValidator_Validate(self_: Pin<&mut wxValidator>, parent: *mut wxWindow) -> bool;
        fn wxValidator_SuppressBellOnError(suppress: bool);
        fn wxValidator_IsSilent() -> bool;
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
        Object(ffi::NewObject())
    }
    pub fn new1(other: &Object) -> Object {
        let other = &other.pinned::<ffi::wxObject>();
        Object(ffi::NewObject1(other))
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut ffi::wxClassInfo {
        ffi::wxObject_GetClassInfo(&self.pinned::<ffi::wxObject>().as_mut())
    }
    fn get_ref_data(&self) -> *mut ffi::wxObjectRefData {
        ffi::wxObject_GetRefData(&self.pinned::<ffi::wxObject>().as_mut())
    }
    fn is_kind_of(&self, info: *const ffi::wxClassInfo) -> bool {
        unsafe { ffi::wxObject_IsKindOf(&self.pinned::<ffi::wxObject>().as_mut(), info) }
    }
    fn is_same_as(&self, obj: &Object) -> bool {
        let obj = &obj.pinned::<ffi::wxObject>();
        ffi::wxObject_IsSameAs(&self.pinned::<ffi::wxObject>().as_mut(), obj)
    }
    fn ref_(&self, clone: &Object) {
        let clone = &clone.pinned::<ffi::wxObject>();
        ffi::wxObject_Ref(self.pinned::<ffi::wxObject>().as_mut(), clone)
    }
    fn set_ref_data(&self, data: *mut ffi::wxObjectRefData) {
        unsafe { ffi::wxObject_SetRefData(self.pinned::<ffi::wxObject>().as_mut(), data) }
    }
    fn un_ref(&self) {
        ffi::wxObject_UnRef(self.pinned::<ffi::wxObject>().as_mut())
    }
    fn un_share(&self) {
        ffi::wxObject_UnShare(self.pinned::<ffi::wxObject>().as_mut())
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
        EvtHandler(ffi::NewEvtHandler())
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
        ffi::wxEvtHandler_AddPendingEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event)
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        ffi::wxEvtHandler_ProcessEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event)
    }
    fn process_event_locally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        ffi::wxEvtHandler_ProcessEventLocally(self.pinned::<ffi::wxEvtHandler>().as_mut(), event)
    }
    fn safely_process_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        ffi::wxEvtHandler_SafelyProcessEvent(self.pinned::<ffi::wxEvtHandler>().as_mut(), event)
    }
    fn process_pending_events(&self) {
        ffi::wxEvtHandler_ProcessPendingEvents(self.pinned::<ffi::wxEvtHandler>().as_mut())
    }
    fn delete_pending_events(&self) {
        ffi::wxEvtHandler_DeletePendingEvents(self.pinned::<ffi::wxEvtHandler>().as_mut())
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
        ffi::wxEvtHandler_GetClientObject(&self.pinned::<ffi::wxEvtHandler>().as_mut())
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut ffi::wxClientData) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.pinned::<ffi::wxEvtHandler>().as_mut(), data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        ffi::wxEvtHandler_GetEvtHandlerEnabled(&self.pinned::<ffi::wxEvtHandler>().as_mut())
    }
    fn get_next_handler(&self) -> *mut ffi::wxEvtHandler {
        ffi::wxEvtHandler_GetNextHandler(&self.pinned::<ffi::wxEvtHandler>().as_mut())
    }
    fn get_previous_handler(&self) -> *mut ffi::wxEvtHandler {
        ffi::wxEvtHandler_GetPreviousHandler(&self.pinned::<ffi::wxEvtHandler>().as_mut())
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        ffi::wxEvtHandler_SetEvtHandlerEnabled(self.pinned::<ffi::wxEvtHandler>().as_mut(), enabled)
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
        ffi::wxEvtHandler_Unlink(self.pinned::<ffi::wxEvtHandler>().as_mut())
    }
    fn is_unlinked(&self) -> bool {
        ffi::wxEvtHandler_IsUnlinked(&self.pinned::<ffi::wxEvtHandler>().as_mut())
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
        Window(ffi::NewWindow())
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> Window {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let pos = &pos.0;
            let size = &size.0;
            let name = &crate::ffi::NewString(name);
            Window(ffi::NewWindow1(parent, id, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait WindowMethods: EvtHandlerMethods {
    fn accepts_focus(&self) -> bool {
        ffi::wxWindow_AcceptsFocus(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        ffi::wxWindow_AcceptsFocusFromKeyboard(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn accepts_focus_recursively(&self) -> bool {
        ffi::wxWindow_AcceptsFocusRecursively(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn disable_focus_from_keyboard(&self) {
        ffi::wxWindow_DisableFocusFromKeyboard(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_focusable(&self) -> bool {
        ffi::wxWindow_IsFocusable(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn can_accept_focus(&self) -> bool {
        ffi::wxWindow_CanAcceptFocus(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        ffi::wxWindow_CanAcceptFocusFromKeyboard(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn has_focus(&self) -> bool {
        ffi::wxWindow_HasFocus(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_can_focus(&self, can_focus: bool) {
        ffi::wxWindow_SetCanFocus(self.pinned::<ffi::wxWindow>().as_mut(), can_focus)
    }
    fn enable_visible_focus(&self, enable: bool) {
        ffi::wxWindow_EnableVisibleFocus(self.pinned::<ffi::wxWindow>().as_mut(), enable)
    }
    fn set_focus(&self) {
        ffi::wxWindow_SetFocus(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_focus_from_kbd(&self) {
        ffi::wxWindow_SetFocusFromKbd(self.pinned::<ffi::wxWindow>().as_mut())
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
        ffi::wxWindow_DestroyChildren(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn find_window(&self, id: i32) -> *mut ffi::wxWindow {
        ffi::wxWindow_FindWindow(&self.pinned::<ffi::wxWindow>().as_mut(), id)
    }
    fn find_window1(&self, name: &str) -> *mut ffi::wxWindow {
        let name = &crate::ffi::NewString(name);
        ffi::wxWindow_FindWindow1(&self.pinned::<ffi::wxWindow>().as_mut(), name)
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
        ffi::wxWindow_GetGrandParent(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_next_sibling(&self) -> *mut ffi::wxWindow {
        ffi::wxWindow_GetNextSibling(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_parent(&self) -> *mut ffi::wxWindow {
        ffi::wxWindow_GetParent(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_prev_sibling(&self) -> *mut ffi::wxWindow {
        ffi::wxWindow_GetPrevSibling(&self.pinned::<ffi::wxWindow>().as_mut())
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
        ffi::wxWindow_AlwaysShowScrollbars(self.pinned::<ffi::wxWindow>().as_mut(), hflag, vflag)
    }
    fn get_scroll_pos(&self, orientation: i32) -> i32 {
        ffi::wxWindow_GetScrollPos(&self.pinned::<ffi::wxWindow>().as_mut(), orientation)
    }
    fn get_scroll_range(&self, orientation: i32) -> i32 {
        ffi::wxWindow_GetScrollRange(&self.pinned::<ffi::wxWindow>().as_mut(), orientation)
    }
    fn get_scroll_thumb(&self, orientation: i32) -> i32 {
        ffi::wxWindow_GetScrollThumb(&self.pinned::<ffi::wxWindow>().as_mut(), orientation)
    }
    fn can_scroll(&self, orient: i32) -> bool {
        ffi::wxWindow_CanScroll(&self.pinned::<ffi::wxWindow>().as_mut(), orient)
    }
    fn has_scrollbar(&self, orient: i32) -> bool {
        ffi::wxWindow_HasScrollbar(&self.pinned::<ffi::wxWindow>().as_mut(), orient)
    }
    fn is_scrollbar_always_shown(&self, orient: i32) -> bool {
        ffi::wxWindow_IsScrollbarAlwaysShown(&self.pinned::<ffi::wxWindow>().as_mut(), orient)
    }
    fn scroll_lines(&self, lines: i32) -> bool {
        ffi::wxWindow_ScrollLines(self.pinned::<ffi::wxWindow>().as_mut(), lines)
    }
    fn scroll_pages(&self, pages: i32) -> bool {
        ffi::wxWindow_ScrollPages(self.pinned::<ffi::wxWindow>().as_mut(), pages)
    }
    fn scroll_window(&self, dx: i32, dy: i32, rect: *const ffi::wxRect) {
        unsafe { ffi::wxWindow_ScrollWindow(self.pinned::<ffi::wxWindow>().as_mut(), dx, dy, rect) }
    }
    fn line_up(&self) -> bool {
        ffi::wxWindow_LineUp(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn line_down(&self) -> bool {
        ffi::wxWindow_LineDown(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn page_up(&self) -> bool {
        ffi::wxWindow_PageUp(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn page_down(&self) -> bool {
        ffi::wxWindow_PageDown(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_scroll_pos(&self, orientation: i32, pos: i32, refresh: bool) {
        ffi::wxWindow_SetScrollPos(self.pinned::<ffi::wxWindow>().as_mut(), orientation, pos, refresh)
    }
    fn set_scrollbar(&self, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool) {
        ffi::wxWindow_SetScrollbar(self.pinned::<ffi::wxWindow>().as_mut(), orientation, position, thumb_size, range, refresh)
    }
    fn begin_repositioning_children(&self) -> bool {
        ffi::wxWindow_BeginRepositioningChildren(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn end_repositioning_children(&self) {
        ffi::wxWindow_EndRepositioningChildren(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn cache_best_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_CacheBestSize(&self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn client_to_window_size(&self, size: &Size) -> Size {
        let size = &size.0;
        Size(ffi::wxWindow_ClientToWindowSize(&self.pinned::<ffi::wxWindow>().as_mut(), size))
    }
    fn window_to_client_size(&self, size: &Size) -> Size {
        let size = &size.0;
        Size(ffi::wxWindow_WindowToClientSize(&self.pinned::<ffi::wxWindow>().as_mut(), size))
    }
    fn fit(&self) {
        ffi::wxWindow_Fit(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn fit_inside(&self) {
        ffi::wxWindow_FitInside(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn from_dip(&self, sz: &Size) -> Size {
        let sz = &sz.0;
        Size(ffi::wxWindow_FromDIP(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
    }
    fn from_dip1(&self, pt: &Point) -> Point {
        let pt = &pt.0;
        Point(ffi::wxWindow_FromDIP1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
    }
    fn from_dip2(&self, d: i32) -> i32 {
        ffi::wxWindow_FromDIP2(&self.pinned::<ffi::wxWindow>().as_mut(), d)
    }
    fn to_dip(&self, sz: &Size) -> Size {
        let sz = &sz.0;
        Size(ffi::wxWindow_ToDIP(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
    }
    fn to_dip1(&self, pt: &Point) -> Point {
        let pt = &pt.0;
        Point(ffi::wxWindow_ToDIP1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
    }
    fn to_dip2(&self, d: i32) -> i32 {
        ffi::wxWindow_ToDIP2(&self.pinned::<ffi::wxWindow>().as_mut(), d)
    }
    fn get_best_size(&self) -> Size {
        Size(ffi::wxWindow_GetBestSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_best_height(&self, width: i32) -> i32 {
        ffi::wxWindow_GetBestHeight(&self.pinned::<ffi::wxWindow>().as_mut(), width)
    }
    fn get_best_width(&self, height: i32) -> i32 {
        ffi::wxWindow_GetBestWidth(&self.pinned::<ffi::wxWindow>().as_mut(), height)
    }
    fn get_client_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetClientSize(&self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn get_client_size1(&self) -> Size {
        Size(ffi::wxWindow_GetClientSize1(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_effective_min_size(&self) -> Size {
        Size(ffi::wxWindow_GetEffectiveMinSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_max_client_size(&self) -> Size {
        Size(ffi::wxWindow_GetMaxClientSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_max_size(&self) -> Size {
        Size(ffi::wxWindow_GetMaxSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_min_client_size(&self) -> Size {
        Size(ffi::wxWindow_GetMinClientSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_min_size(&self) -> Size {
        Size(ffi::wxWindow_GetMinSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_min_width(&self) -> i32 {
        ffi::wxWindow_GetMinWidth(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_min_height(&self) -> i32 {
        ffi::wxWindow_GetMinHeight(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_max_width(&self) -> i32 {
        ffi::wxWindow_GetMaxWidth(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_max_height(&self) -> i32 {
        ffi::wxWindow_GetMaxHeight(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetSize(&self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn get_size1(&self) -> Size {
        Size(ffi::wxWindow_GetSize1(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_virtual_size(&self) -> Size {
        Size(ffi::wxWindow_GetVirtualSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_virtual_size1(&self, width: *mut i32, height: *mut i32) {
        unsafe { ffi::wxWindow_GetVirtualSize1(&self.pinned::<ffi::wxWindow>().as_mut(), width, height) }
    }
    fn get_best_virtual_size(&self) -> Size {
        Size(ffi::wxWindow_GetBestVirtualSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_content_scale_factor(&self) -> f64 {
        ffi::wxWindow_GetContentScaleFactor(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_dpi_scale_factor(&self) -> f64 {
        ffi::wxWindow_GetDPIScaleFactor(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_window_border_size(&self) -> Size {
        Size(ffi::wxWindow_GetWindowBorderSize(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn inform_first_direction(&self, direction: i32, size: i32, available_other_dir: i32) -> bool {
        ffi::wxWindow_InformFirstDirection(self.pinned::<ffi::wxWindow>().as_mut(), direction, size, available_other_dir)
    }
    fn invalidate_best_size(&self) {
        ffi::wxWindow_InvalidateBestSize(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn post_size_event(&self) {
        ffi::wxWindow_PostSizeEvent(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn post_size_event_to_parent(&self) {
        ffi::wxWindow_PostSizeEventToParent(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn send_size_event(&self, flags: i32) {
        ffi::wxWindow_SendSizeEvent(self.pinned::<ffi::wxWindow>().as_mut(), flags)
    }
    fn send_size_event_to_parent(&self, flags: i32) {
        ffi::wxWindow_SendSizeEventToParent(self.pinned::<ffi::wxWindow>().as_mut(), flags)
    }
    fn set_client_size(&self, width: i32, height: i32) {
        ffi::wxWindow_SetClientSize(self.pinned::<ffi::wxWindow>().as_mut(), width, height)
    }
    fn set_client_size1(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetClientSize1(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_client_size2(&self, rect: &Rect) {
        let rect = &rect.0;
        ffi::wxWindow_SetClientSize2(self.pinned::<ffi::wxWindow>().as_mut(), rect)
    }
    fn set_containing_sizer(&self, sizer: *mut ffi::wxSizer) {
        unsafe { ffi::wxWindow_SetContainingSizer(self.pinned::<ffi::wxWindow>().as_mut(), sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetInitialSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_max_client_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetMaxClientSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_max_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetMaxSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_min_client_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetMinClientSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_min_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetMinSize(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_size(&self, x: i32, y: i32, width: i32, height: i32, size_flags: i32) {
        ffi::wxWindow_SetSize(self.pinned::<ffi::wxWindow>().as_mut(), x, y, width, height, size_flags)
    }
    fn set_size1(&self, rect: &Rect) {
        let rect = &rect.0;
        ffi::wxWindow_SetSize1(self.pinned::<ffi::wxWindow>().as_mut(), rect)
    }
    fn set_size2(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetSize2(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn set_size3(&self, width: i32, height: i32) {
        ffi::wxWindow_SetSize3(self.pinned::<ffi::wxWindow>().as_mut(), width, height)
    }
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        let min_size = &min_size.0;
        let max_size = &max_size.0;
        let inc_size = &inc_size.0;
        ffi::wxWindow_SetSizeHints(self.pinned::<ffi::wxWindow>().as_mut(), min_size, max_size, inc_size)
    }
    fn set_size_hints1(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        ffi::wxWindow_SetSizeHints1(self.pinned::<ffi::wxWindow>().as_mut(), min_w, min_h, max_w, max_h, inc_w, inc_h)
    }
    fn set_virtual_size(&self, width: i32, height: i32) {
        ffi::wxWindow_SetVirtualSize(self.pinned::<ffi::wxWindow>().as_mut(), width, height)
    }
    fn set_virtual_size1(&self, size: &Size) {
        let size = &size.0;
        ffi::wxWindow_SetVirtualSize1(self.pinned::<ffi::wxWindow>().as_mut(), size)
    }
    fn from_dip3<T: WindowMethods>(sz: &Size, w: Option<&T>) -> Size {
        unsafe {
            let sz = &sz.0;
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_FromDIP3(sz, w))
        }
    }
    fn from_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = &pt.0;
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
            let sz = &sz.0;
            let w = match w {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            Size(ffi::wxWindow_ToDIP3(sz, w))
        }
    }
    fn to_dip4<T: WindowMethods>(pt: &Point, w: Option<&T>) -> Point {
        unsafe {
            let pt = &pt.0;
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
        ffi::wxWindow_Center(self.pinned::<ffi::wxWindow>().as_mut(), dir)
    }
    fn center_on_parent(&self, dir: i32) {
        ffi::wxWindow_CenterOnParent(self.pinned::<ffi::wxWindow>().as_mut(), dir)
    }
    fn centre(&self, direction: i32) {
        ffi::wxWindow_Centre(self.pinned::<ffi::wxWindow>().as_mut(), direction)
    }
    fn centre_on_parent(&self, direction: i32) {
        ffi::wxWindow_CentreOnParent(self.pinned::<ffi::wxWindow>().as_mut(), direction)
    }
    fn get_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_GetPosition(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn get_position1(&self) -> Point {
        Point(ffi::wxWindow_GetPosition1(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_rect(&self) -> Rect {
        Rect(ffi::wxWindow_GetRect(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_screen_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_GetScreenPosition(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn get_screen_position1(&self) -> Point {
        Point(ffi::wxWindow_GetScreenPosition1(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_screen_rect(&self) -> Rect {
        Rect(ffi::wxWindow_GetScreenRect(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_client_area_origin(&self) -> Point {
        Point(ffi::wxWindow_GetClientAreaOrigin(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn get_client_rect(&self) -> Rect {
        Rect(ffi::wxWindow_GetClientRect(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn move_(&self, x: i32, y: i32, flags: i32) {
        ffi::wxWindow_Move(self.pinned::<ffi::wxWindow>().as_mut(), x, y, flags)
    }
    fn move1(&self, pt: &Point, flags: i32) {
        let pt = &pt.0;
        ffi::wxWindow_Move1(self.pinned::<ffi::wxWindow>().as_mut(), pt, flags)
    }
    fn set_position(&self, pt: &Point) {
        let pt = &pt.0;
        ffi::wxWindow_SetPosition(self.pinned::<ffi::wxWindow>().as_mut(), pt)
    }
    fn client_to_screen(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_ClientToScreen(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn client_to_screen1(&self, pt: &Point) -> Point {
        let pt = &pt.0;
        Point(ffi::wxWindow_ClientToScreen1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
    }
    fn convert_dialog_to_pixels(&self, pt: &Point) -> Point {
        let pt = &pt.0;
        Point(ffi::wxWindow_ConvertDialogToPixels(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
    }
    fn convert_dialog_to_pixels1(&self, sz: &Size) -> Size {
        let sz = &sz.0;
        Size(ffi::wxWindow_ConvertDialogToPixels1(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
    }
    fn convert_pixels_to_dialog(&self, pt: &Point) -> Point {
        let pt = &pt.0;
        Point(ffi::wxWindow_ConvertPixelsToDialog(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
    }
    fn convert_pixels_to_dialog1(&self, sz: &Size) -> Size {
        let sz = &sz.0;
        Size(ffi::wxWindow_ConvertPixelsToDialog1(&self.pinned::<ffi::wxWindow>().as_mut(), sz))
    }
    fn screen_to_client(&self, x: *mut i32, y: *mut i32) {
        unsafe { ffi::wxWindow_ScreenToClient(&self.pinned::<ffi::wxWindow>().as_mut(), x, y) }
    }
    fn screen_to_client1(&self, pt: &Point) -> Point {
        let pt = &pt.0;
        Point(ffi::wxWindow_ScreenToClient1(&self.pinned::<ffi::wxWindow>().as_mut(), pt))
    }
    fn clear_background(&self) {
        ffi::wxWindow_ClearBackground(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn freeze(&self) {
        ffi::wxWindow_Freeze(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn thaw(&self) {
        ffi::wxWindow_Thaw(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_frozen(&self) -> bool {
        ffi::wxWindow_IsFrozen(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> i32 {
        ffi::wxWindow_GetCharHeight(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_char_width(&self) -> i32 {
        ffi::wxWindow_GetCharWidth(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    fn get_dpi(&self) -> Size {
        Size(ffi::wxWindow_GetDPI(&self.pinned::<ffi::wxWindow>().as_mut()))
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
        let string = &crate::ffi::NewString(string);
        Size(ffi::wxWindow_GetTextExtent1(&self.pinned::<ffi::wxWindow>().as_mut(), string))
    }
    // BLOCKED: fn GetUpdateRegion()
    fn get_update_client_rect(&self) -> Rect {
        Rect(ffi::wxWindow_GetUpdateClientRect(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn has_transparent_background(&self) -> bool {
        ffi::wxWindow_HasTransparentBackground(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn refresh(&self, erase_background: bool, rect: *const ffi::wxRect) {
        unsafe { ffi::wxWindow_Refresh(self.pinned::<ffi::wxWindow>().as_mut(), erase_background, rect) }
    }
    fn refresh_rect(&self, rect: &Rect, erase_background: bool) {
        let rect = &rect.0;
        ffi::wxWindow_RefreshRect(self.pinned::<ffi::wxWindow>().as_mut(), rect, erase_background)
    }
    fn update(&self) {
        ffi::wxWindow_Update(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_background_colour(&self, colour: &ffi::wxColour) -> bool {
        ffi::wxWindow_SetBackgroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour)
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut ffi::wxString) -> bool {
        unsafe { ffi::wxWindow_IsTransparentBackgroundSupported(&self.pinned::<ffi::wxWindow>().as_mut(), reason) }
    }
    fn set_font(&self, font: &ffi::wxFont) -> bool {
        ffi::wxWindow_SetFont(self.pinned::<ffi::wxWindow>().as_mut(), font)
    }
    fn set_foreground_colour(&self, colour: &ffi::wxColour) -> bool {
        ffi::wxWindow_SetForegroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour)
    }
    fn set_own_background_colour(&self, colour: &ffi::wxColour) {
        ffi::wxWindow_SetOwnBackgroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour)
    }
    fn inherits_background_colour(&self) -> bool {
        ffi::wxWindow_InheritsBackgroundColour(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn use_bg_col(&self) -> bool {
        ffi::wxWindow_UseBgCol(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn use_background_colour(&self) -> bool {
        ffi::wxWindow_UseBackgroundColour(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_own_font(&self, font: &ffi::wxFont) {
        ffi::wxWindow_SetOwnFont(self.pinned::<ffi::wxWindow>().as_mut(), font)
    }
    fn set_own_foreground_colour(&self, colour: &ffi::wxColour) {
        ffi::wxWindow_SetOwnForegroundColour(self.pinned::<ffi::wxWindow>().as_mut(), colour)
    }
    fn use_foreground_colour(&self) -> bool {
        ffi::wxWindow_UseForegroundColour(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn inherits_foreground_colour(&self) -> bool {
        ffi::wxWindow_InheritsForegroundColour(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_palette(&self, pal: &ffi::wxPalette) {
        ffi::wxWindow_SetPalette(self.pinned::<ffi::wxWindow>().as_mut(), pal)
    }
    fn should_inherit_colours(&self) -> bool {
        ffi::wxWindow_ShouldInheritColours(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_theme_enabled(&self, enable: bool) {
        ffi::wxWindow_SetThemeEnabled(self.pinned::<ffi::wxWindow>().as_mut(), enable)
    }
    fn get_theme_enabled(&self) -> bool {
        ffi::wxWindow_GetThemeEnabled(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn can_set_transparent(&self) -> bool {
        ffi::wxWindow_CanSetTransparent(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        ffi::wxWindow_SetTransparent(self.pinned::<ffi::wxWindow>().as_mut(), alpha)
    }
    fn get_event_handler(&self) -> *mut ffi::wxEvtHandler {
        ffi::wxWindow_GetEventHandler(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn handle_as_navigation_key(&self, event: &ffi::wxKeyEvent) -> bool {
        ffi::wxWindow_HandleAsNavigationKey(self.pinned::<ffi::wxWindow>().as_mut(), event)
    }
    fn handle_window_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        ffi::wxWindow_HandleWindowEvent(&self.pinned::<ffi::wxWindow>().as_mut(), event)
    }
    fn process_window_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        ffi::wxWindow_ProcessWindowEvent(self.pinned::<ffi::wxWindow>().as_mut(), event)
    }
    fn process_window_event_locally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        ffi::wxWindow_ProcessWindowEventLocally(self.pinned::<ffi::wxWindow>().as_mut(), event)
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut ffi::wxEvtHandler {
        ffi::wxWindow_PopEventHandler(self.pinned::<ffi::wxWindow>().as_mut(), delete_handler)
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
        ffi::wxWindow_GetExtraStyle(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_window_style_flag(&self) -> i32 {
        ffi::wxWindow_GetWindowStyleFlag(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_window_style(&self) -> i32 {
        ffi::wxWindow_GetWindowStyle(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn has_extra_style(&self, ex_flag: i32) -> bool {
        ffi::wxWindow_HasExtraStyle(&self.pinned::<ffi::wxWindow>().as_mut(), ex_flag)
    }
    fn has_flag(&self, flag: i32) -> bool {
        ffi::wxWindow_HasFlag(&self.pinned::<ffi::wxWindow>().as_mut(), flag)
    }
    fn set_extra_style(&self, ex_style: i32) {
        ffi::wxWindow_SetExtraStyle(self.pinned::<ffi::wxWindow>().as_mut(), ex_style)
    }
    fn set_window_style_flag(&self, style: i32) {
        ffi::wxWindow_SetWindowStyleFlag(self.pinned::<ffi::wxWindow>().as_mut(), style)
    }
    fn set_window_style(&self, style: i32) {
        ffi::wxWindow_SetWindowStyle(self.pinned::<ffi::wxWindow>().as_mut(), style)
    }
    fn toggle_window_style(&self, flag: i32) -> bool {
        ffi::wxWindow_ToggleWindowStyle(self.pinned::<ffi::wxWindow>().as_mut(), flag)
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
        ffi::wxWindow_Navigate(self.pinned::<ffi::wxWindow>().as_mut(), flags)
    }
    fn navigate_in(&self, flags: i32) -> bool {
        ffi::wxWindow_NavigateIn(self.pinned::<ffi::wxWindow>().as_mut(), flags)
    }
    fn lower(&self) {
        ffi::wxWindow_Lower(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn raise(&self) {
        ffi::wxWindow_Raise(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn hide(&self) -> bool {
        ffi::wxWindow_Hide(self.pinned::<ffi::wxWindow>().as_mut())
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        ffi::wxWindow_IsEnabled(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_exposed(&self, x: i32, y: i32) -> bool {
        ffi::wxWindow_IsExposed(&self.pinned::<ffi::wxWindow>().as_mut(), x, y)
    }
    fn is_exposed1(&self, pt: &mut ffi::wxPoint) -> bool {
        ffi::wxWindow_IsExposed1(&self.pinned::<ffi::wxWindow>().as_mut(), pt)
    }
    fn is_exposed2(&self, x: i32, y: i32, w: i32, h: i32) -> bool {
        ffi::wxWindow_IsExposed2(&self.pinned::<ffi::wxWindow>().as_mut(), x, y, w, h)
    }
    fn is_exposed3(&self, rect: &mut ffi::wxRect) -> bool {
        ffi::wxWindow_IsExposed3(&self.pinned::<ffi::wxWindow>().as_mut(), rect)
    }
    fn is_shown(&self) -> bool {
        ffi::wxWindow_IsShown(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_shown_on_screen(&self) -> bool {
        ffi::wxWindow_IsShownOnScreen(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn disable(&self) -> bool {
        ffi::wxWindow_Disable(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn enable(&self, enable: bool) -> bool {
        ffi::wxWindow_Enable(self.pinned::<ffi::wxWindow>().as_mut(), enable)
    }
    fn show(&self, show: bool) -> bool {
        ffi::wxWindow_Show(self.pinned::<ffi::wxWindow>().as_mut(), show)
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    fn get_help_text(&self) -> WxString {
        WxString(ffi::wxWindow_GetHelpText(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn set_help_text(&self, help_text: &str) {
        let help_text = &crate::ffi::NewString(help_text);
        ffi::wxWindow_SetHelpText(self.pinned::<ffi::wxWindow>().as_mut(), help_text)
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut ffi::wxToolTip {
        ffi::wxWindow_GetToolTip(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_tool_tip_text(&self) -> WxString {
        WxString(ffi::wxWindow_GetToolTipText(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    fn set_tool_tip(&self, tip_string: &str) {
        let tip_string = &crate::ffi::NewString(tip_string);
        ffi::wxWindow_SetToolTip(self.pinned::<ffi::wxWindow>().as_mut(), tip_string)
    }
    fn set_tool_tip1(&self, tip: *mut ffi::wxToolTip) {
        unsafe { ffi::wxWindow_SetToolTip1(self.pinned::<ffi::wxWindow>().as_mut(), tip) }
    }
    fn unset_tool_tip(&self) {
        ffi::wxWindow_UnsetToolTip(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_popup_menu_selection_from_user(&self, menu: Pin<&mut ffi::wxMenu>, pos: &Point) -> i32 {
        let pos = &pos.0;
        ffi::wxWindow_GetPopupMenuSelectionFromUser(self.pinned::<ffi::wxWindow>().as_mut(), menu, pos)
    }
    fn get_popup_menu_selection_from_user1(&self, menu: Pin<&mut ffi::wxMenu>, x: i32, y: i32) -> i32 {
        ffi::wxWindow_GetPopupMenuSelectionFromUser1(self.pinned::<ffi::wxWindow>().as_mut(), menu, x, y)
    }
    fn popup_menu(&self, menu: *mut ffi::wxMenu, pos: &Point) -> bool {
        unsafe {
            let pos = &pos.0;
            ffi::wxWindow_PopupMenu(self.pinned::<ffi::wxWindow>().as_mut(), menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut ffi::wxMenu, x: i32, y: i32) -> bool {
        unsafe { ffi::wxWindow_PopupMenu1(self.pinned::<ffi::wxWindow>().as_mut(), menu, x, y) }
    }
    fn get_validator(&self) -> *mut ffi::wxValidator {
        ffi::wxWindow_GetValidator(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_validator(&self, validator: &Validator) {
        let validator = &validator.pinned::<ffi::wxValidator>();
        ffi::wxWindow_SetValidator(self.pinned::<ffi::wxWindow>().as_mut(), validator)
    }
    fn transfer_data_from_window(&self) -> bool {
        ffi::wxWindow_TransferDataFromWindow(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn transfer_data_to_window(&self) -> bool {
        ffi::wxWindow_TransferDataToWindow(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn validate(&self) -> bool {
        ffi::wxWindow_Validate(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_id(&self) -> i32 {
        ffi::wxWindow_GetId(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_label(&self) -> WxString {
        WxString(ffi::wxWindow_GetLabel(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: i32, width: i32, width_total: i32) -> i32 {
        ffi::wxWindow_AdjustForLayoutDirection(&self.pinned::<ffi::wxWindow>().as_mut(), x, width, width_total)
    }
    fn get_name(&self) -> WxString {
        WxString(ffi::wxWindow_GetName(&self.pinned::<ffi::wxWindow>().as_mut()))
    }
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: i32) {
        ffi::wxWindow_SetId(self.pinned::<ffi::wxWindow>().as_mut(), winid)
    }
    fn set_label(&self, label: &str) {
        let label = &crate::ffi::NewString(label);
        ffi::wxWindow_SetLabel(self.pinned::<ffi::wxWindow>().as_mut(), label)
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        let name = &crate::ffi::NewString(name);
        ffi::wxWindow_SetName(self.pinned::<ffi::wxWindow>().as_mut(), name)
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut ffi::wxAcceleratorTable {
        ffi::wxWindow_GetAcceleratorTable(self.pinned::<ffi::wxWindow>().as_mut())
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: &ffi::wxAcceleratorTable) {
        ffi::wxWindow_SetAcceleratorTable(self.pinned::<ffi::wxWindow>().as_mut(), accel)
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        ffi::wxWindow_Close(self.pinned::<ffi::wxWindow>().as_mut(), force)
    }
    fn destroy(&self) -> bool {
        ffi::wxWindow_Destroy(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_being_deleted(&self) -> bool {
        ffi::wxWindow_IsBeingDeleted(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_drop_target(&self) -> *mut ffi::wxDropTarget {
        ffi::wxWindow_GetDropTarget(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_drop_target(&self, target: *mut ffi::wxDropTarget) {
        unsafe { ffi::wxWindow_SetDropTarget(self.pinned::<ffi::wxWindow>().as_mut(), target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        ffi::wxWindow_DragAcceptFiles(self.pinned::<ffi::wxWindow>().as_mut(), accept)
    }
    fn get_containing_sizer(&self) -> *mut ffi::wxSizer {
        ffi::wxWindow_GetContainingSizer(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_sizer(&self) -> *mut ffi::wxSizer {
        ffi::wxWindow_GetSizer(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_sizer(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizer(self.pinned::<ffi::wxWindow>().as_mut(), sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { ffi::wxWindow_SetSizerAndFit(self.pinned::<ffi::wxWindow>().as_mut(), sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut ffi::wxLayoutConstraints {
        ffi::wxWindow_GetConstraints(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_constraints(&self, constraints: *mut ffi::wxLayoutConstraints) {
        unsafe { ffi::wxWindow_SetConstraints(self.pinned::<ffi::wxWindow>().as_mut(), constraints) }
    }
    fn layout(&self) -> bool {
        ffi::wxWindow_Layout(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        ffi::wxWindow_SetAutoLayout(self.pinned::<ffi::wxWindow>().as_mut(), auto_layout)
    }
    fn get_auto_layout(&self) -> bool {
        ffi::wxWindow_GetAutoLayout(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn capture_mouse(&self) {
        ffi::wxWindow_CaptureMouse(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn get_caret(&self) -> *mut ffi::wxCaret {
        ffi::wxWindow_GetCaret(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        ffi::wxWindow_HasCapture(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn release_mouse(&self) {
        ffi::wxWindow_ReleaseMouse(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_caret(&self, caret: *mut ffi::wxCaret) {
        unsafe { ffi::wxWindow_SetCaret(self.pinned::<ffi::wxWindow>().as_mut(), caret) }
    }
    fn set_cursor(&self, cursor: &ffi::wxCursor) -> bool {
        ffi::wxWindow_SetCursor(self.pinned::<ffi::wxWindow>().as_mut(), cursor)
    }
    fn warp_pointer(&self, x: i32, y: i32) {
        ffi::wxWindow_WarpPointer(self.pinned::<ffi::wxWindow>().as_mut(), x, y)
    }
    fn enable_touch_events(&self, events_mask: i32) -> bool {
        ffi::wxWindow_EnableTouchEvents(self.pinned::<ffi::wxWindow>().as_mut(), events_mask)
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest1()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder1()
    fn do_update_window_ui(&self, event: Pin<&mut ffi::wxUpdateUIEvent>) {
        ffi::wxWindow_DoUpdateWindowUI(self.pinned::<ffi::wxWindow>().as_mut(), event)
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        ffi::wxWindow_HasMultiplePages(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn inherit_attributes(&self) {
        ffi::wxWindow_InheritAttributes(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn init_dialog(&self) {
        ffi::wxWindow_InitDialog(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_double_buffered(&self) -> bool {
        ffi::wxWindow_IsDoubleBuffered(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn set_double_buffered(&self, on: bool) {
        ffi::wxWindow_SetDoubleBuffered(self.pinned::<ffi::wxWindow>().as_mut(), on)
    }
    fn is_retained(&self) -> bool {
        ffi::wxWindow_IsRetained(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_this_enabled(&self) -> bool {
        ffi::wxWindow_IsThisEnabled(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn is_top_level(&self) -> bool {
        ffi::wxWindow_IsTopLevel(&self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn on_internal_idle(&self) {
        ffi::wxWindow_OnInternalIdle(self.pinned::<ffi::wxWindow>().as_mut())
    }
    fn send_idle_events(&self, event: Pin<&mut ffi::wxIdleEvent>) -> bool {
        ffi::wxWindow_SendIdleEvents(self.pinned::<ffi::wxWindow>().as_mut(), event)
    }
    fn register_hot_key(&self, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool {
        ffi::wxWindow_RegisterHotKey(self.pinned::<ffi::wxWindow>().as_mut(), hotkey_id, modifiers, virtual_key_code)
    }
    fn unregister_hot_key(&self, hotkey_id: i32) -> bool {
        ffi::wxWindow_UnregisterHotKey(self.pinned::<ffi::wxWindow>().as_mut(), hotkey_id)
    }
    fn update_window_ui(&self, flags: i32) {
        ffi::wxWindow_UpdateWindowUI(self.pinned::<ffi::wxWindow>().as_mut(), flags)
    }
    // CXX_UNSUPPORTED: fn GetClassDefaultAttributes()
    fn find_focus() -> *mut ffi::wxWindow {
        ffi::wxWindow_FindFocus()
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
        ffi::wxWindow_GetCapture()
    }
    fn new_control_id(count: i32) -> i32 {
        ffi::wxWindow_NewControlId(count)
    }
    fn unreserve_control_id(id: i32, count: i32) {
        ffi::wxWindow_UnreserveControlId(id, count)
    }
    // DTOR: fn ~wxWindow()
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let pos = &pos.0;
            let size = &size.0;
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
            let pos = &pos.0;
            let size = &size.0;
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
            Control(ffi::NewControl(parent, id, pos, size, style, validator, name))
        }
    }
    pub fn new1() -> Control {
        Control(ffi::NewControl1())
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
            let pos = &pos.0;
            let size = &size.0;
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
            ffi::wxControl_Create(self.pinned::<ffi::wxControl>().as_mut(), parent, id, pos, size, style, validator, name)
        }
    }
    fn command(&self, event: Pin<&mut ffi::wxCommandEvent>) {
        ffi::wxControl_Command(self.pinned::<ffi::wxControl>().as_mut(), event)
    }
    fn get_label(&self) -> WxString {
        WxString(ffi::wxControl_GetLabel(&self.pinned::<ffi::wxControl>().as_mut()))
    }
    fn get_label_text(&self) -> WxString {
        WxString(ffi::wxControl_GetLabelText(&self.pinned::<ffi::wxControl>().as_mut()))
    }
    fn get_size_from_text_size(&self, xlen: i32, ylen: i32) -> Size {
        Size(ffi::wxControl_GetSizeFromTextSize(&self.pinned::<ffi::wxControl>().as_mut(), xlen, ylen))
    }
    fn get_size_from_text_size1(&self, tsize: &Size) -> Size {
        let tsize = &tsize.0;
        Size(ffi::wxControl_GetSizeFromTextSize1(&self.pinned::<ffi::wxControl>().as_mut(), tsize))
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        let text = &crate::ffi::NewString(text);
        Size(ffi::wxControl_GetSizeFromText(&self.pinned::<ffi::wxControl>().as_mut(), text))
    }
    fn set_label(&self, label: &str) {
        let label = &crate::ffi::NewString(label);
        ffi::wxControl_SetLabel(self.pinned::<ffi::wxControl>().as_mut(), label)
    }
    fn set_label_text(&self, text: &str) {
        let text = &crate::ffi::NewString(text);
        ffi::wxControl_SetLabelText(self.pinned::<ffi::wxControl>().as_mut(), text)
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        let markup = &crate::ffi::NewString(markup);
        ffi::wxControl_SetLabelMarkup(self.pinned::<ffi::wxControl>().as_mut(), markup)
    }
    fn get_label_text1(label: &str) -> WxString {
        let label = &crate::ffi::NewString(label);
        WxString(ffi::wxControl_GetLabelText1(label))
    }
    fn remove_mnemonics(str: &str) -> WxString {
        let str = &crate::ffi::NewString(str);
        WxString(ffi::wxControl_RemoveMnemonics(str))
    }
    fn escape_mnemonics(text: &str) -> WxString {
        let text = &crate::ffi::NewString(text);
        WxString(ffi::wxControl_EscapeMnemonics(text))
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
        AnyButton(ffi::NewAnyButton())
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
        ffi::wxAnyButton_SetBitmapCurrent(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap)
    }
    fn set_bitmap_disabled(&self, bitmap: &ffi::wxBitmap) {
        ffi::wxAnyButton_SetBitmapDisabled(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap)
    }
    fn set_bitmap_focus(&self, bitmap: &ffi::wxBitmap) {
        ffi::wxAnyButton_SetBitmapFocus(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap)
    }
    fn set_bitmap_label(&self, bitmap: &ffi::wxBitmap) {
        ffi::wxAnyButton_SetBitmapLabel(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap)
    }
    fn set_bitmap_pressed(&self, bitmap: &ffi::wxBitmap) {
        ffi::wxAnyButton_SetBitmapPressed(self.pinned::<ffi::wxAnyButton>().as_mut(), bitmap)
    }
    fn get_bitmap_margins(&self) -> Size {
        Size(ffi::wxAnyButton_GetBitmapMargins(self.pinned::<ffi::wxAnyButton>().as_mut()))
    }
    fn set_bitmap_margins(&self, x: i32, y: i32) {
        ffi::wxAnyButton_SetBitmapMargins(self.pinned::<ffi::wxAnyButton>().as_mut(), x, y)
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        let sz = &sz.0;
        ffi::wxAnyButton_SetBitmapMargins1(self.pinned::<ffi::wxAnyButton>().as_mut(), sz)
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
        Button(ffi::NewButton())
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, label: &str, pos: &Point, size: &Size, style: i32, validator: &Validator, name: &str) -> Button {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let label = &crate::ffi::NewString(label);
            let pos = &pos.0;
            let size = &size.0;
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
            Button(ffi::NewButton1(parent, id, label, pos, size, style, validator, name))
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
            let pos = &pos.0;
            let size = &size.0;
            let validator = &validator.pinned::<ffi::wxValidator>();
            let name = &crate::ffi::NewString(name);
            ffi::wxButton_Create(self.pinned::<ffi::wxButton>().as_mut(), parent, id, label, pos, size, style, validator, name)
        }
    }
    fn get_auth_needed(&self) -> bool {
        ffi::wxButton_GetAuthNeeded(&self.pinned::<ffi::wxButton>().as_mut())
    }
    fn get_label(&self) -> WxString {
        WxString(ffi::wxButton_GetLabel(&self.pinned::<ffi::wxButton>().as_mut()))
    }
    fn set_auth_needed(&self, needed: bool) {
        ffi::wxButton_SetAuthNeeded(self.pinned::<ffi::wxButton>().as_mut(), needed)
    }
    fn set_default(&self) -> *mut ffi::wxWindow {
        ffi::wxButton_SetDefault(self.pinned::<ffi::wxButton>().as_mut())
    }
    fn set_label(&self, label: &str) {
        let label = &crate::ffi::NewString(label);
        ffi::wxButton_SetLabel(self.pinned::<ffi::wxButton>().as_mut(), label)
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
        ffi::wxNonOwnedWindow_SetShape(self.pinned::<ffi::wxNonOwnedWindow>().as_mut(), region)
    }
    fn set_shape1(&self, path: &ffi::wxGraphicsPath) -> bool {
        ffi::wxNonOwnedWindow_SetShape1(self.pinned::<ffi::wxNonOwnedWindow>().as_mut(), path)
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
        TopLevelWindow(ffi::NewTopLevelWindow())
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> TopLevelWindow {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.0;
            let size = &size.0;
            let name = &crate::ffi::NewString(name);
            TopLevelWindow(ffi::NewTopLevelWindow1(parent, id, title, pos, size, style, name))
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
            let pos = &pos.0;
            let size = &size.0;
            let name = &crate::ffi::NewString(name);
            ffi::wxTopLevelWindow_Create(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), parent, id, title, pos, size, style, name)
        }
    }
    fn can_set_transparent(&self) -> bool {
        ffi::wxTopLevelWindow_CanSetTransparent(self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn center_on_screen(&self, direction: i32) {
        ffi::wxTopLevelWindow_CenterOnScreen(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), direction)
    }
    fn centre_on_screen(&self, direction: i32) {
        ffi::wxTopLevelWindow_CentreOnScreen(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), direction)
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        ffi::wxTopLevelWindow_EnableCloseButton(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable)
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        ffi::wxTopLevelWindow_EnableMaximizeButton(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable)
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        ffi::wxTopLevelWindow_EnableMinimizeButton(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable)
    }
    fn get_default_item(&self) -> *mut ffi::wxWindow {
        ffi::wxTopLevelWindow_GetDefaultItem(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    // CXX_UNSUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    fn get_title(&self) -> WxString {
        WxString(ffi::wxTopLevelWindow_GetTitle(&self.pinned::<ffi::wxTopLevelWindow>().as_mut()))
    }
    fn iconize(&self, iconize: bool) {
        ffi::wxTopLevelWindow_Iconize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), iconize)
    }
    fn is_active(&self) -> bool {
        ffi::wxTopLevelWindow_IsActive(self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn is_always_maximized(&self) -> bool {
        ffi::wxTopLevelWindow_IsAlwaysMaximized(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn is_full_screen(&self) -> bool {
        ffi::wxTopLevelWindow_IsFullScreen(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn is_iconized(&self) -> bool {
        ffi::wxTopLevelWindow_IsIconized(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn is_maximized(&self) -> bool {
        ffi::wxTopLevelWindow_IsMaximized(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn layout(&self) -> bool {
        ffi::wxTopLevelWindow_Layout(self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn maximize(&self, maximize: bool) {
        ffi::wxTopLevelWindow_Maximize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), maximize)
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: i32) {
        ffi::wxTopLevelWindow_RequestUserAttention(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), flags)
    }
    fn restore(&self) {
        ffi::wxTopLevelWindow_Restore(self.pinned::<ffi::wxTopLevelWindow>().as_mut())
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
        ffi::wxTopLevelWindow_GetTmpDefaultItem(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn set_icon(&self, icon: &ffi::wxIcon) {
        ffi::wxTopLevelWindow_SetIcon(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), icon)
    }
    fn set_icons(&self, icons: &ffi::wxIconBundle) {
        ffi::wxTopLevelWindow_SetIcons(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), icons)
    }
    fn set_max_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxTopLevelWindow_SetMaxSize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), size)
    }
    fn set_min_size(&self, size: &Size) {
        let size = &size.0;
        ffi::wxTopLevelWindow_SetMinSize(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), size)
    }
    fn set_size_hints(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        ffi::wxTopLevelWindow_SetSizeHints(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), min_w, min_h, max_w, max_h, inc_w, inc_h)
    }
    fn set_size_hints1(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        let min_size = &min_size.0;
        let max_size = &max_size.0;
        let inc_size = &inc_size.0;
        ffi::wxTopLevelWindow_SetSizeHints1(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), min_size, max_size, inc_size)
    }
    fn set_title(&self, title: &str) {
        let title = &crate::ffi::NewString(title);
        ffi::wxTopLevelWindow_SetTitle(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), title)
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        ffi::wxTopLevelWindow_SetTransparent(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), alpha)
    }
    fn should_prevent_app_exit(&self) -> bool {
        ffi::wxTopLevelWindow_ShouldPreventAppExit(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn osx_set_modified(&self, modified: bool) {
        ffi::wxTopLevelWindow_OSXSetModified(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), modified)
    }
    fn osx_is_modified(&self) -> bool {
        ffi::wxTopLevelWindow_OSXIsModified(&self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn set_represented_filename(&self, filename: &str) {
        let filename = &crate::ffi::NewString(filename);
        ffi::wxTopLevelWindow_SetRepresentedFilename(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), filename)
    }
    fn show_without_activating(&self) {
        ffi::wxTopLevelWindow_ShowWithoutActivating(self.pinned::<ffi::wxTopLevelWindow>().as_mut())
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        ffi::wxTopLevelWindow_EnableFullScreenView(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), enable)
    }
    fn show_full_screen(&self, show: bool, style: i32) -> bool {
        ffi::wxTopLevelWindow_ShowFullScreen(self.pinned::<ffi::wxTopLevelWindow>().as_mut(), show, style)
    }
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    fn get_default_size() -> Size {
        Size(ffi::wxTopLevelWindow_GetDefaultSize())
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
        Frame(ffi::NewFrame())
    }
    pub fn new1<T: WindowMethods>(parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> Frame {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.0;
            let size = &size.0;
            let name = &crate::ffi::NewString(name);
            Frame(ffi::NewFrame1(parent, id, title, pos, size, style, name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait FrameMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxFrame()
    fn centre(&self, direction: i32) {
        ffi::wxFrame_Centre(self.pinned::<ffi::wxFrame>().as_mut(), direction)
    }
    fn create<T: WindowMethods>(&self, parent: Option<&T>, id: i32, title: &str, pos: &Point, size: &Size, style: i32, name: &str) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            let title = &crate::ffi::NewString(title);
            let pos = &pos.0;
            let size = &size.0;
            let name = &crate::ffi::NewString(name);
            ffi::wxFrame_Create(self.pinned::<ffi::wxFrame>().as_mut(), parent, id, title, pos, size, style, name)
        }
    }
    fn create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut ffi::wxStatusBar {
        let name = &crate::ffi::NewString(name);
        ffi::wxFrame_CreateStatusBar(self.pinned::<ffi::wxFrame>().as_mut(), number, style, id, name)
    }
    fn create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut ffi::wxToolBar {
        let name = &crate::ffi::NewString(name);
        ffi::wxFrame_CreateToolBar(self.pinned::<ffi::wxFrame>().as_mut(), style, id, name)
    }
    fn do_give_help(&self, text: &str, show: bool) {
        let text = &crate::ffi::NewString(text);
        ffi::wxFrame_DoGiveHelp(self.pinned::<ffi::wxFrame>().as_mut(), text, show)
    }
    fn get_client_area_origin(&self) -> Point {
        Point(ffi::wxFrame_GetClientAreaOrigin(&self.pinned::<ffi::wxFrame>().as_mut()))
    }
    fn get_menu_bar(&self) -> *mut ffi::wxMenuBar {
        ffi::wxFrame_GetMenuBar(&self.pinned::<ffi::wxFrame>().as_mut())
    }
    fn get_status_bar(&self) -> *mut ffi::wxStatusBar {
        ffi::wxFrame_GetStatusBar(&self.pinned::<ffi::wxFrame>().as_mut())
    }
    fn get_status_bar_pane(&self) -> i32 {
        ffi::wxFrame_GetStatusBarPane(&self.pinned::<ffi::wxFrame>().as_mut())
    }
    fn get_tool_bar(&self) -> *mut ffi::wxToolBar {
        ffi::wxFrame_GetToolBar(&self.pinned::<ffi::wxFrame>().as_mut())
    }
    fn on_create_status_bar(&self, number: i32, style: i32, id: i32, name: &str) -> *mut ffi::wxStatusBar {
        let name = &crate::ffi::NewString(name);
        ffi::wxFrame_OnCreateStatusBar(self.pinned::<ffi::wxFrame>().as_mut(), number, style, id, name)
    }
    fn on_create_tool_bar(&self, style: i32, id: i32, name: &str) -> *mut ffi::wxToolBar {
        let name = &crate::ffi::NewString(name);
        ffi::wxFrame_OnCreateToolBar(self.pinned::<ffi::wxFrame>().as_mut(), style, id, name)
    }
    fn process_command(&self, id: i32) -> bool {
        ffi::wxFrame_ProcessCommand(self.pinned::<ffi::wxFrame>().as_mut(), id)
    }
    fn set_menu_bar(&self, menu_bar: *mut ffi::wxMenuBar) {
        unsafe { ffi::wxFrame_SetMenuBar(self.pinned::<ffi::wxFrame>().as_mut(), menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut ffi::wxStatusBar) {
        unsafe { ffi::wxFrame_SetStatusBar(self.pinned::<ffi::wxFrame>().as_mut(), status_bar) }
    }
    fn set_status_bar_pane(&self, n: i32) {
        ffi::wxFrame_SetStatusBarPane(self.pinned::<ffi::wxFrame>().as_mut(), n)
    }
    fn set_status_text(&self, text: &str, number: i32) {
        let text = &crate::ffi::NewString(text);
        ffi::wxFrame_SetStatusText(self.pinned::<ffi::wxFrame>().as_mut(), text, number)
    }
    fn set_status_widths(&self, n: i32, widths_field: *const i32) {
        unsafe { ffi::wxFrame_SetStatusWidths(self.pinned::<ffi::wxFrame>().as_mut(), n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut ffi::wxToolBar) {
        unsafe { ffi::wxFrame_SetToolBar(self.pinned::<ffi::wxFrame>().as_mut(), tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: i32) {
        let text = &crate::ffi::NewString(text);
        ffi::wxFrame_PushStatusText(self.pinned::<ffi::wxFrame>().as_mut(), text, number)
    }
    fn pop_status_text(&self, number: i32) {
        ffi::wxFrame_PopStatusText(self.pinned::<ffi::wxFrame>().as_mut(), number)
    }
}

// wxPoint
pub struct Point(ffi::wxPoint);
impl Point {
    pub fn new() -> Point {
        Point(ffi::NewPoint())
    }
    pub fn new1(x: i32, y: i32) -> Point {
        Point(ffi::NewPoint1(x, y))
    }
    pub fn new2(pt: &ffi::wxRealPoint) -> Point {
        Point(ffi::NewPoint2(pt))
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
    fn is_fully_specified(&self) -> bool {
        ffi::wxPoint_IsFullySpecified(&self.0)
    }
    fn set_defaults(&mut self, pt: &Point) {
        let pt = &pt.0;
        ffi::wxPoint_SetDefaults(&mut self.0, pt)
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
pub struct Rect(ffi::wxRect);
impl Rect {
    pub fn new() -> Rect {
        Rect(ffi::NewRect())
    }
    pub fn new1(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect(ffi::NewRect1(x, y, width, height))
    }
    pub fn new2(top_left: &Point, bottom_right: &Point) -> Rect {
        let top_left = &top_left.0;
        let bottom_right = &bottom_right.0;
        Rect(ffi::NewRect2(top_left, bottom_right))
    }
    pub fn new3(pos: &Point, size: &Size) -> Rect {
        let pos = &pos.0;
        let size = &size.0;
        Rect(ffi::NewRect3(pos, size))
    }
    pub fn new4(size: &Size) -> Rect {
        let size = &size.0;
        Rect(ffi::NewRect4(size))
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
    fn centre_in(&self, r: &Rect, dir: i32) -> Rect {
        let r = &r.0;
        Rect(ffi::wxRect_CentreIn(&self.0, r, dir))
    }
    fn center_in(&self, r: &Rect, dir: i32) -> Rect {
        let r = &r.0;
        Rect(ffi::wxRect_CenterIn(&self.0, r, dir))
    }
    fn contains(&self, x: i32, y: i32) -> bool {
        ffi::wxRect_Contains(&self.0, x, y)
    }
    fn contains1(&self, pt: &Point) -> bool {
        let pt = &pt.0;
        ffi::wxRect_Contains1(&self.0, pt)
    }
    fn contains2(&self, rect: &Rect) -> bool {
        let rect = &rect.0;
        ffi::wxRect_Contains2(&self.0, rect)
    }
    // BLOCKED: fn Deflate()
    // BLOCKED: fn Deflate1()
    // BLOCKED: fn Deflate2()
    fn deflate3(&self, dx: i32, dy: i32) -> Rect {
        Rect(ffi::wxRect_Deflate3(&self.0, dx, dy))
    }
    fn get_bottom(&self) -> i32 {
        ffi::wxRect_GetBottom(&self.0)
    }
    fn get_bottom_left(&self) -> Point {
        Point(ffi::wxRect_GetBottomLeft(&self.0))
    }
    fn get_bottom_right(&self) -> Point {
        Point(ffi::wxRect_GetBottomRight(&self.0))
    }
    fn get_height(&self) -> i32 {
        ffi::wxRect_GetHeight(&self.0)
    }
    fn get_left(&self) -> i32 {
        ffi::wxRect_GetLeft(&self.0)
    }
    fn get_position(&self) -> Point {
        Point(ffi::wxRect_GetPosition(&self.0))
    }
    fn get_right(&self) -> i32 {
        ffi::wxRect_GetRight(&self.0)
    }
    fn get_size(&self) -> Size {
        Size(ffi::wxRect_GetSize(&self.0))
    }
    fn get_top(&self) -> i32 {
        ffi::wxRect_GetTop(&self.0)
    }
    fn get_top_left(&self) -> Point {
        Point(ffi::wxRect_GetTopLeft(&self.0))
    }
    fn get_top_right(&self) -> Point {
        Point(ffi::wxRect_GetTopRight(&self.0))
    }
    fn get_width(&self) -> i32 {
        ffi::wxRect_GetWidth(&self.0)
    }
    fn get_x(&self) -> i32 {
        ffi::wxRect_GetX(&self.0)
    }
    fn get_y(&self) -> i32 {
        ffi::wxRect_GetY(&self.0)
    }
    // BLOCKED: fn Inflate()
    // BLOCKED: fn Inflate1()
    // BLOCKED: fn Inflate2()
    fn inflate3(&self, dx: i32, dy: i32) -> Rect {
        Rect(ffi::wxRect_Inflate3(&self.0, dx, dy))
    }
    // BLOCKED: fn Intersect()
    fn intersect1(&self, rect: &Rect) -> Rect {
        let rect = &rect.0;
        Rect(ffi::wxRect_Intersect1(&self.0, rect))
    }
    fn intersects(&self, rect: &Rect) -> bool {
        let rect = &rect.0;
        ffi::wxRect_Intersects(&self.0, rect)
    }
    fn is_empty(&self) -> bool {
        ffi::wxRect_IsEmpty(&self.0)
    }
    fn offset(&mut self, dx: i32, dy: i32) {
        ffi::wxRect_Offset(&mut self.0, dx, dy)
    }
    fn offset1(&mut self, pt: &Point) {
        let pt = &pt.0;
        ffi::wxRect_Offset1(&mut self.0, pt)
    }
    fn set_height(&mut self, height: i32) {
        ffi::wxRect_SetHeight(&mut self.0, height)
    }
    fn set_position(&mut self, pos: &Point) {
        let pos = &pos.0;
        ffi::wxRect_SetPosition(&mut self.0, pos)
    }
    fn set_size(&mut self, s: &Size) {
        let s = &s.0;
        ffi::wxRect_SetSize(&mut self.0, s)
    }
    fn set_width(&mut self, width: i32) {
        ffi::wxRect_SetWidth(&mut self.0, width)
    }
    fn set_x(&mut self, x: i32) {
        ffi::wxRect_SetX(&mut self.0, x)
    }
    fn set_y(&mut self, y: i32) {
        ffi::wxRect_SetY(&mut self.0, y)
    }
    fn set_left(&mut self, left: i32) {
        ffi::wxRect_SetLeft(&mut self.0, left)
    }
    fn set_right(&mut self, right: i32) {
        ffi::wxRect_SetRight(&mut self.0, right)
    }
    fn set_top(&mut self, top: i32) {
        ffi::wxRect_SetTop(&mut self.0, top)
    }
    fn set_bottom(&mut self, bottom: i32) {
        ffi::wxRect_SetBottom(&mut self.0, bottom)
    }
    fn set_top_left(&mut self, p: &Point) {
        let p = &p.0;
        ffi::wxRect_SetTopLeft(&mut self.0, p)
    }
    fn set_bottom_right(&mut self, p: &Point) {
        let p = &p.0;
        ffi::wxRect_SetBottomRight(&mut self.0, p)
    }
    fn set_top_right(&mut self, p: &Point) {
        let p = &p.0;
        ffi::wxRect_SetTopRight(&mut self.0, p)
    }
    fn set_bottom_left(&mut self, p: &Point) {
        let p = &p.0;
        ffi::wxRect_SetBottomLeft(&mut self.0, p)
    }
    fn union(&self, rect: &Rect) -> Rect {
        let rect = &rect.0;
        Rect(ffi::wxRect_Union(&self.0, rect))
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
pub struct Size(ffi::wxSize);
impl Size {
    pub fn new() -> Size {
        Size(ffi::NewSize())
    }
    pub fn new1(width: i32, height: i32) -> Size {
        Size(ffi::NewSize1(width, height))
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
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
    fn dec_by(&mut self, pt: &Point) {
        let pt = &pt.0;
        ffi::wxSize_DecBy(&mut self.0, pt)
    }
    fn dec_by1(&mut self, size: &Size) {
        let size = &size.0;
        ffi::wxSize_DecBy1(&mut self.0, size)
    }
    fn dec_by2(&mut self, dx: i32, dy: i32) {
        ffi::wxSize_DecBy2(&mut self.0, dx, dy)
    }
    fn dec_by3(&mut self, d: i32) {
        ffi::wxSize_DecBy3(&mut self.0, d)
    }
    fn dec_to(&mut self, size: &Size) {
        let size = &size.0;
        ffi::wxSize_DecTo(&mut self.0, size)
    }
    fn dec_to_if_specified(&mut self, size: &Size) {
        let size = &size.0;
        ffi::wxSize_DecToIfSpecified(&mut self.0, size)
    }
    fn get_height(&self) -> i32 {
        ffi::wxSize_GetHeight(&self.0)
    }
    fn get_width(&self) -> i32 {
        ffi::wxSize_GetWidth(&self.0)
    }
    fn inc_by(&mut self, pt: &Point) {
        let pt = &pt.0;
        ffi::wxSize_IncBy(&mut self.0, pt)
    }
    fn inc_by1(&mut self, size: &Size) {
        let size = &size.0;
        ffi::wxSize_IncBy1(&mut self.0, size)
    }
    fn inc_by2(&mut self, dx: i32, dy: i32) {
        ffi::wxSize_IncBy2(&mut self.0, dx, dy)
    }
    fn inc_by3(&mut self, d: i32) {
        ffi::wxSize_IncBy3(&mut self.0, d)
    }
    fn inc_to(&mut self, size: &Size) {
        let size = &size.0;
        ffi::wxSize_IncTo(&mut self.0, size)
    }
    fn is_fully_specified(&self) -> bool {
        ffi::wxSize_IsFullySpecified(&self.0)
    }
    // BLOCKED: fn Scale()
    fn set(&mut self, width: i32, height: i32) {
        ffi::wxSize_Set(&mut self.0, width, height)
    }
    fn set_defaults(&mut self, size_default: &Size) {
        let size_default = &size_default.0;
        ffi::wxSize_SetDefaults(&mut self.0, size_default)
    }
    fn set_height(&mut self, height: i32) {
        ffi::wxSize_SetHeight(&mut self.0, height)
    }
    fn set_width(&mut self, width: i32) {
        ffi::wxSize_SetWidth(&mut self.0, width)
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
        Validator(ffi::NewValidator())
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ValidatorMethods: EvtHandlerMethods {
    // DTOR: fn ~wxValidator()
    fn clone(&self) -> *mut ffi::wxObject {
        ffi::wxValidator_Clone(&self.pinned::<ffi::wxValidator>().as_mut())
    }
    fn get_window(&self) -> *mut ffi::wxWindow {
        ffi::wxValidator_GetWindow(&self.pinned::<ffi::wxValidator>().as_mut())
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
        ffi::wxValidator_TransferFromWindow(self.pinned::<ffi::wxValidator>().as_mut())
    }
    fn transfer_to_window(&self) -> bool {
        ffi::wxValidator_TransferToWindow(self.pinned::<ffi::wxValidator>().as_mut())
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
        ffi::wxValidator_SuppressBellOnError(suppress)
    }
    fn is_silent() -> bool {
        ffi::wxValidator_IsSilent()
    }
}

