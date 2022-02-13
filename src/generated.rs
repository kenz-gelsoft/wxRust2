#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::c_char;
use std::pin::Pin;
use std::ptr;

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
        type wxRect;
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

        // CLASS: wxObject
        type wxObject;
        // CTOR: fn wxObject() -> Object;
        // CTOR: fn wxObject(other: &wxObject) -> Object;
        // DTOR: fn ~wxObject(self: Pin<&mut wxObject>);
        fn GetClassInfo(self: &wxObject) -> *mut wxClassInfo;
        fn GetRefData(self: &wxObject) -> *mut wxObjectRefData;
        unsafe fn IsKindOf(self: &wxObject, info: *const wxClassInfo) -> bool;
        fn IsSameAs(self: &wxObject, obj: &wxObject) -> bool;
        fn Ref(self: Pin<&mut wxObject>, clone: &wxObject);
        unsafe fn SetRefData(self: Pin<&mut wxObject>, data: *mut wxObjectRefData);
        fn UnRef(self: Pin<&mut wxObject>);
        fn UnShare(self: Pin<&mut wxObject>);
        // BLOCKED: unsafe fn operator delete(self: Pin<&mut wxObject>, buf: *mut void);
        // CXX_UNSUPPORTED: fn operator new(self: Pin<&mut wxObject>, size: size_t, filename: &wxString, line_num: i32) -> *mut void;

        // CLASS: wxEvtHandler
        type wxEvtHandler;
        unsafe fn QueueEvent(self: Pin<&mut wxEvtHandler>, event: *mut wxEvent);
        fn AddPendingEvent(self: Pin<&mut wxEvtHandler>, event: &wxEvent);
        // CXX_UNSUPPORTED: unsafe fn CallAfter(self: Pin<&mut wxEvtHandler>, method: *mut void(T::, x1: T1, None: ...);
        // BLOCKED: fn CallAfter(self: Pin<&mut wxEvtHandler>, functor: &T);
        fn ProcessEvent(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessEventLocally(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        fn SafelyProcessEvent(self: Pin<&mut wxEvtHandler>, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessPendingEvents(self: Pin<&mut wxEvtHandler>);
        fn DeletePendingEvents(self: Pin<&mut wxEvtHandler>);
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
        fn GetClientObject(self: &wxEvtHandler) -> *mut wxClientData;
        // BLOCKED: unsafe fn SetClientData(self: Pin<&mut wxEvtHandler>, data: *mut void);
        unsafe fn SetClientObject(self: Pin<&mut wxEvtHandler>, data: *mut wxClientData);
        fn GetEvtHandlerEnabled(self: &wxEvtHandler) -> bool;
        fn GetNextHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        fn GetPreviousHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        fn SetEvtHandlerEnabled(self: Pin<&mut wxEvtHandler>, enabled: bool);
        unsafe fn SetNextHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        unsafe fn SetPreviousHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        fn Unlink(self: Pin<&mut wxEvtHandler>);
        fn IsUnlinked(self: &wxEvtHandler) -> bool;
        // STATIC: unsafe fn AddFilter(filter: *mut wxEventFilter);
        // STATIC: unsafe fn RemoveFilter(filter: *mut wxEventFilter);
        // CTOR: fn wxEvtHandler() -> EvtHandler;
        // DTOR: fn ~wxEvtHandler(self: Pin<&mut wxEvtHandler>);

        // CLASS: wxWindow
        type wxWindow;
        fn AcceptsFocus(self: &wxWindow) -> bool;
        fn AcceptsFocusFromKeyboard(self: &wxWindow) -> bool;
        fn AcceptsFocusRecursively(self: &wxWindow) -> bool;
        fn DisableFocusFromKeyboard(self: Pin<&mut wxWindow>);
        fn IsFocusable(self: &wxWindow) -> bool;
        fn CanAcceptFocus(self: &wxWindow) -> bool;
        fn CanAcceptFocusFromKeyboard(self: &wxWindow) -> bool;
        fn HasFocus(self: &wxWindow) -> bool;
        fn SetCanFocus(self: Pin<&mut wxWindow>, can_focus: bool);
        fn EnableVisibleFocus(self: Pin<&mut wxWindow>, enable: bool);
        fn SetFocus(self: Pin<&mut wxWindow>);
        fn SetFocusFromKbd(self: Pin<&mut wxWindow>);
        // BLOCKED: unsafe fn AddChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        fn DestroyChildren(self: Pin<&mut wxWindow>) -> bool;
        // BLOCKED: fn FindWindow(self: &wxWindow, id: i32) -> *mut wxWindow;
        // BLOCKED: fn FindWindow(self: &wxWindow, name: &wxString) -> *mut wxWindow;
        // BLOCKED: fn GetChildren(self: Pin<&mut wxWindow>) -> Pin<&mut wxWindowList>;
        // BLOCKED: fn GetChildren(self: &wxWindow) -> &wxWindowList;
        // BLOCKED: unsafe fn RemoveChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        fn GetGrandParent(self: &wxWindow) -> *mut wxWindow;
        fn GetNextSibling(self: &wxWindow) -> *mut wxWindow;
        fn GetParent(self: &wxWindow) -> *mut wxWindow;
        fn GetPrevSibling(self: &wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn IsDescendant(self: &wxWindow, win: *mut wxWindow) -> bool;
        // BLOCKED: unsafe fn Reparent(self: Pin<&mut wxWindow>, new_parent: *mut wxWindow) -> bool;
        fn AlwaysShowScrollbars(self: Pin<&mut wxWindow>, hflag: bool, vflag: bool);
        fn GetScrollPos(self: &wxWindow, orientation: i32) -> i32;
        fn GetScrollRange(self: &wxWindow, orientation: i32) -> i32;
        fn GetScrollThumb(self: &wxWindow, orientation: i32) -> i32;
        fn CanScroll(self: &wxWindow, orient: i32) -> bool;
        fn HasScrollbar(self: &wxWindow, orient: i32) -> bool;
        fn IsScrollbarAlwaysShown(self: &wxWindow, orient: i32) -> bool;
        fn ScrollLines(self: Pin<&mut wxWindow>, lines: i32) -> bool;
        fn ScrollPages(self: Pin<&mut wxWindow>, pages: i32) -> bool;
        unsafe fn ScrollWindow(self: Pin<&mut wxWindow>, dx: i32, dy: i32, rect: *const wxRect);
        fn LineUp(self: Pin<&mut wxWindow>) -> bool;
        fn LineDown(self: Pin<&mut wxWindow>) -> bool;
        fn PageUp(self: Pin<&mut wxWindow>) -> bool;
        fn PageDown(self: Pin<&mut wxWindow>) -> bool;
        fn SetScrollPos(self: Pin<&mut wxWindow>, orientation: i32, pos: i32, refresh: bool);
        fn SetScrollbar(self: Pin<&mut wxWindow>, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool);
        fn BeginRepositioningChildren(self: Pin<&mut wxWindow>) -> bool;
        fn EndRepositioningChildren(self: Pin<&mut wxWindow>);
        fn CacheBestSize(self: &wxWindow, size: &wxSize);
        // CXX_UNSUPPORTED: fn ClientToWindowSize(self: &wxWindow, size: &wxSize) -> wxSize;
        // CXX_UNSUPPORTED: fn WindowToClientSize(self: &wxWindow, size: &wxSize) -> wxSize;
        fn Fit(self: Pin<&mut wxWindow>);
        fn FitInside(self: Pin<&mut wxWindow>);
        // CXX_UNSUPPORTED: fn FromDIP(self: &wxWindow, sz: &wxSize) -> wxSize;
        // CXX_UNSUPPORTED: fn FromDIP(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // BLOCKED: fn FromDIP(self: &wxWindow, d: i32) -> i32;
        // CXX_UNSUPPORTED: fn ToDIP(self: &wxWindow, sz: &wxSize) -> wxSize;
        // CXX_UNSUPPORTED: fn ToDIP(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // BLOCKED: fn ToDIP(self: &wxWindow, d: i32) -> i32;
        // CXX_UNSUPPORTED: fn GetBestSize(self: &wxWindow) -> wxSize;
        fn GetBestHeight(self: &wxWindow, width: i32) -> i32;
        fn GetBestWidth(self: &wxWindow, height: i32) -> i32;
        unsafe fn GetClientSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // CXX_UNSUPPORTED: fn GetClientSize(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetEffectiveMinSize(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetMaxClientSize(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetMaxSize(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetMinClientSize(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetMinSize(self: &wxWindow) -> wxSize;
        fn GetMinWidth(self: &wxWindow) -> i32;
        fn GetMinHeight(self: &wxWindow) -> i32;
        fn GetMaxWidth(self: &wxWindow) -> i32;
        fn GetMaxHeight(self: &wxWindow) -> i32;
        unsafe fn GetSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // CXX_UNSUPPORTED: fn GetSize(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetVirtualSize(self: &wxWindow) -> wxSize;
        #[rust_name = "GetVirtualSize1"]
        unsafe fn GetVirtualSize(self: &wxWindow, width: *mut i32, height: *mut i32);
        // CXX_UNSUPPORTED: fn GetBestVirtualSize(self: &wxWindow) -> wxSize;
        fn GetContentScaleFactor(self: &wxWindow) -> f64;
        fn GetDPIScaleFactor(self: &wxWindow) -> f64;
        // CXX_UNSUPPORTED: fn GetWindowBorderSize(self: &wxWindow) -> wxSize;
        fn InformFirstDirection(self: Pin<&mut wxWindow>, direction: i32, size: i32, available_other_dir: i32) -> bool;
        fn InvalidateBestSize(self: Pin<&mut wxWindow>);
        fn PostSizeEvent(self: Pin<&mut wxWindow>);
        fn PostSizeEventToParent(self: Pin<&mut wxWindow>);
        fn SendSizeEvent(self: Pin<&mut wxWindow>, flags: i32);
        fn SendSizeEventToParent(self: Pin<&mut wxWindow>, flags: i32);
        fn SetClientSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "SetClientSize1"]
        fn SetClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        #[rust_name = "SetClientSize2"]
        fn SetClientSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        unsafe fn SetContainingSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer);
        fn SetInitialSize(self: Pin<&mut wxWindow>, size: &wxSize);
        fn SetMaxClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        fn SetMaxSize(self: Pin<&mut wxWindow>, size: &wxSize);
        fn SetMinClientSize(self: Pin<&mut wxWindow>, size: &wxSize);
        fn SetMinSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, x: i32, y: i32, width: i32, height: i32, size_flags: i32);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        fn SetSizeHints(self: Pin<&mut wxWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        #[rust_name = "SetSizeHints1"]
        fn SetSizeHints(self: Pin<&mut wxWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        fn SetVirtualSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "SetVirtualSize1"]
        fn SetVirtualSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // STATIC: unsafe fn FromDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // STATIC: unsafe fn FromDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // STATIC: unsafe fn FromDIP(d: i32, w: *const wxWindow) -> i32;
        // STATIC: unsafe fn ToDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // STATIC: unsafe fn ToDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // STATIC: unsafe fn ToDIP(d: i32, w: *const wxWindow) -> i32;
        fn Center(self: Pin<&mut wxWindow>, dir: i32);
        fn CenterOnParent(self: Pin<&mut wxWindow>, dir: i32);
        fn Centre(self: Pin<&mut wxWindow>, direction: i32);
        fn CentreOnParent(self: Pin<&mut wxWindow>, direction: i32);
        unsafe fn GetPosition(self: &wxWindow, x: *mut i32, y: *mut i32);
        // CXX_UNSUPPORTED: fn GetPosition(self: &wxWindow) -> wxPoint;
        // CXX_UNSUPPORTED: fn GetRect(self: &wxWindow) -> wxRect;
        unsafe fn GetScreenPosition(self: &wxWindow, x: *mut i32, y: *mut i32);
        // CXX_UNSUPPORTED: fn GetScreenPosition(self: &wxWindow) -> wxPoint;
        // CXX_UNSUPPORTED: fn GetScreenRect(self: &wxWindow) -> wxRect;
        // CXX_UNSUPPORTED: fn GetClientAreaOrigin(self: &wxWindow) -> wxPoint;
        // CXX_UNSUPPORTED: fn GetClientRect(self: &wxWindow) -> wxRect;
        fn Move(self: Pin<&mut wxWindow>, x: i32, y: i32, flags: i32);
        #[rust_name = "Move1"]
        fn Move(self: Pin<&mut wxWindow>, pt: &wxPoint, flags: i32);
        fn SetPosition(self: Pin<&mut wxWindow>, pt: &wxPoint);
        unsafe fn ClientToScreen(self: &wxWindow, x: *mut i32, y: *mut i32);
        // CXX_UNSUPPORTED: fn ClientToScreen(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // CXX_UNSUPPORTED: fn ConvertDialogToPixels(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // CXX_UNSUPPORTED: fn ConvertDialogToPixels(self: &wxWindow, sz: &wxSize) -> wxSize;
        // CXX_UNSUPPORTED: fn ConvertPixelsToDialog(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        // CXX_UNSUPPORTED: fn ConvertPixelsToDialog(self: &wxWindow, sz: &wxSize) -> wxSize;
        unsafe fn ScreenToClient(self: &wxWindow, x: *mut i32, y: *mut i32);
        // CXX_UNSUPPORTED: fn ScreenToClient(self: &wxWindow, pt: &wxPoint) -> wxPoint;
        fn ClearBackground(self: Pin<&mut wxWindow>);
        fn Freeze(self: Pin<&mut wxWindow>);
        fn Thaw(self: Pin<&mut wxWindow>);
        fn IsFrozen(self: &wxWindow) -> bool;
        // CXX_UNSUPPORTED: fn GetBackgroundColour(self: &wxWindow) -> wxColour;
        // CXX_UNSUPPORTED: fn GetBackgroundStyle(self: &wxWindow) -> wxBackgroundStyle;
        fn GetCharHeight(self: &wxWindow) -> i32;
        fn GetCharWidth(self: &wxWindow) -> i32;
        // CXX_UNSUPPORTED: fn GetDefaultAttributes(self: &wxWindow) -> wxVisualAttributes;
        // CXX_UNSUPPORTED: fn GetDPI(self: &wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: fn GetFont(self: &wxWindow) -> wxFont;
        // CXX_UNSUPPORTED: fn GetForegroundColour(self: &wxWindow) -> wxColour;
        unsafe fn GetTextExtent(self: &wxWindow, string: &wxString, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const wxFont);
        // CXX_UNSUPPORTED: fn GetTextExtent(self: &wxWindow, string: &wxString) -> wxSize;
        // BLOCKED: fn GetUpdateRegion(self: &wxWindow) -> &wxRegion;
        // CXX_UNSUPPORTED: fn GetUpdateClientRect(self: &wxWindow) -> wxRect;
        fn HasTransparentBackground(self: Pin<&mut wxWindow>) -> bool;
        unsafe fn Refresh(self: Pin<&mut wxWindow>, erase_background: bool, rect: *const wxRect);
        fn RefreshRect(self: Pin<&mut wxWindow>, rect: &wxRect, erase_background: bool);
        fn Update(self: Pin<&mut wxWindow>);
        fn SetBackgroundColour(self: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        // CXX_UNSUPPORTED: fn SetBackgroundStyle(self: Pin<&mut wxWindow>, style: wxBackgroundStyle) -> bool;
        unsafe fn IsTransparentBackgroundSupported(self: &wxWindow, reason: *mut wxString) -> bool;
        fn SetFont(self: Pin<&mut wxWindow>, font: &wxFont) -> bool;
        fn SetForegroundColour(self: Pin<&mut wxWindow>, colour: &wxColour) -> bool;
        fn SetOwnBackgroundColour(self: Pin<&mut wxWindow>, colour: &wxColour);
        fn InheritsBackgroundColour(self: &wxWindow) -> bool;
        fn UseBgCol(self: &wxWindow) -> bool;
        fn UseBackgroundColour(self: &wxWindow) -> bool;
        fn SetOwnFont(self: Pin<&mut wxWindow>, font: &wxFont);
        fn SetOwnForegroundColour(self: Pin<&mut wxWindow>, colour: &wxColour);
        fn UseForegroundColour(self: &wxWindow) -> bool;
        fn InheritsForegroundColour(self: &wxWindow) -> bool;
        fn SetPalette(self: Pin<&mut wxWindow>, pal: &wxPalette);
        fn ShouldInheritColours(self: &wxWindow) -> bool;
        fn SetThemeEnabled(self: Pin<&mut wxWindow>, enable: bool);
        fn GetThemeEnabled(self: &wxWindow) -> bool;
        fn CanSetTransparent(self: Pin<&mut wxWindow>) -> bool;
        fn SetTransparent(self: Pin<&mut wxWindow>, alpha: u8) -> bool;
        fn GetEventHandler(self: &wxWindow) -> *mut wxEvtHandler;
        fn HandleAsNavigationKey(self: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        fn HandleWindowEvent(self: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessWindowEvent(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessWindowEventLocally(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn PopEventHandler(self: Pin<&mut wxWindow>, delete_handler: bool) -> *mut wxEvtHandler;
        unsafe fn PushEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn RemoveEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler) -> bool;
        unsafe fn SetEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn SetNextHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn SetPreviousHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // BLOCKED: fn GetExtraStyle(self: &wxWindow) -> i32;
        // BLOCKED: fn GetWindowStyleFlag(self: &wxWindow) -> i32;
        // BLOCKED: fn GetWindowStyle(self: &wxWindow) -> i32;
        fn HasExtraStyle(self: &wxWindow, ex_flag: i32) -> bool;
        fn HasFlag(self: &wxWindow, flag: i32) -> bool;
        // BLOCKED: fn SetExtraStyle(self: Pin<&mut wxWindow>, ex_style: i32);
        // BLOCKED: fn SetWindowStyleFlag(self: Pin<&mut wxWindow>, style: i32);
        // BLOCKED: fn SetWindowStyle(self: Pin<&mut wxWindow>, style: i32);
        fn ToggleWindowStyle(self: Pin<&mut wxWindow>, flag: i32) -> bool;
        unsafe fn MoveAfterInTabOrder(self: Pin<&mut wxWindow>, win: *mut wxWindow);
        unsafe fn MoveBeforeInTabOrder(self: Pin<&mut wxWindow>, win: *mut wxWindow);
        fn Navigate(self: Pin<&mut wxWindow>, flags: i32) -> bool;
        fn NavigateIn(self: Pin<&mut wxWindow>, flags: i32) -> bool;
        fn Lower(self: Pin<&mut wxWindow>);
        fn Raise(self: Pin<&mut wxWindow>);
        fn Hide(self: Pin<&mut wxWindow>) -> bool;
        // CXX_UNSUPPORTED: fn HideWithEffect(self: Pin<&mut wxWindow>, effect: wxShowEffect, timeout: u32) -> bool;
        fn IsEnabled(self: &wxWindow) -> bool;
        // BLOCKED: fn IsExposed(self: &wxWindow, x: i32, y: i32) -> bool;
        // BLOCKED: fn IsExposed(self: &wxWindow, pt: Pin<&mut wxPoint>) -> bool;
        // BLOCKED: fn IsExposed(self: &wxWindow, x: i32, y: i32, w: i32, h: i32) -> bool;
        // BLOCKED: fn IsExposed(self: &wxWindow, rect: Pin<&mut wxRect>) -> bool;
        fn IsShown(self: &wxWindow) -> bool;
        fn IsShownOnScreen(self: &wxWindow) -> bool;
        fn Disable(self: Pin<&mut wxWindow>) -> bool;
        fn Enable(self: Pin<&mut wxWindow>, enable: bool) -> bool;
        fn Show(self: Pin<&mut wxWindow>, show: bool) -> bool;
        // CXX_UNSUPPORTED: fn ShowWithEffect(self: Pin<&mut wxWindow>, effect: wxShowEffect, timeout: u32) -> bool;
        // CXX_UNSUPPORTED: fn GetHelpText(self: &wxWindow) -> wxString;
        fn SetHelpText(self: Pin<&mut wxWindow>, help_text: &wxString);
        // CXX_UNSUPPORTED: fn GetHelpTextAtPoint(self: &wxWindow, point: &wxPoint, origin: wxHelpEvent::Origin) -> wxString;
        fn GetToolTip(self: &wxWindow) -> *mut wxToolTip;
        // CXX_UNSUPPORTED: fn GetToolTipText(self: &wxWindow) -> wxString;
        fn SetToolTip(self: Pin<&mut wxWindow>, tip_string: &wxString);
        #[rust_name = "SetToolTip1"]
        unsafe fn SetToolTip(self: Pin<&mut wxWindow>, tip: *mut wxToolTip);
        fn UnsetToolTip(self: Pin<&mut wxWindow>);
        fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, pos: &wxPoint) -> i32;
        #[rust_name = "GetPopupMenuSelectionFromUser1"]
        fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, x: i32, y: i32) -> i32;
        unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, pos: &wxPoint) -> bool;
        #[rust_name = "PopupMenu1"]
        unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        fn GetValidator(self: Pin<&mut wxWindow>) -> *mut wxValidator;
        fn SetValidator(self: Pin<&mut wxWindow>, validator: &wxValidator);
        fn TransferDataFromWindow(self: Pin<&mut wxWindow>) -> bool;
        fn TransferDataToWindow(self: Pin<&mut wxWindow>) -> bool;
        fn Validate(self: Pin<&mut wxWindow>) -> bool;
        fn GetId(self: &wxWindow) -> i32;
        // CXX_UNSUPPORTED: fn GetLabel(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: fn GetLayoutDirection(self: &wxWindow) -> wxLayoutDirection;
        fn AdjustForLayoutDirection(self: &wxWindow, x: i32, width: i32, width_total: i32) -> i32;
        // CXX_UNSUPPORTED: fn GetName(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: fn GetWindowVariant(self: &wxWindow) -> wxWindowVariant;
        fn SetId(self: Pin<&mut wxWindow>, winid: i32);
        fn SetLabel(self: Pin<&mut wxWindow>, label: &wxString);
        // CXX_UNSUPPORTED: fn SetLayoutDirection(self: Pin<&mut wxWindow>, dir: wxLayoutDirection);
        fn SetName(self: Pin<&mut wxWindow>, name: &wxString);
        // CXX_UNSUPPORTED: fn SetWindowVariant(self: Pin<&mut wxWindow>, variant: wxWindowVariant);
        fn GetAcceleratorTable(self: Pin<&mut wxWindow>) -> *mut wxAcceleratorTable;
        // CXX_UNSUPPORTED: fn GetAccessible(self: Pin<&mut wxWindow>) -> *mut wxAccessible;
        fn SetAcceleratorTable(self: Pin<&mut wxWindow>, accel: &wxAcceleratorTable);
        // CXX_UNSUPPORTED: unsafe fn SetAccessible(self: Pin<&mut wxWindow>, accessible: *mut wxAccessible);
        fn Close(self: Pin<&mut wxWindow>, force: bool) -> bool;
        fn Destroy(self: Pin<&mut wxWindow>) -> bool;
        fn IsBeingDeleted(self: &wxWindow) -> bool;
        fn GetDropTarget(self: &wxWindow) -> *mut wxDropTarget;
        unsafe fn SetDropTarget(self: Pin<&mut wxWindow>, target: *mut wxDropTarget);
        fn DragAcceptFiles(self: Pin<&mut wxWindow>, accept: bool);
        fn GetContainingSizer(self: &wxWindow) -> *mut wxSizer;
        fn GetSizer(self: &wxWindow) -> *mut wxSizer;
        unsafe fn SetSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        unsafe fn SetSizerAndFit(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, delete_old: bool);
        fn GetConstraints(self: &wxWindow) -> *mut wxLayoutConstraints;
        unsafe fn SetConstraints(self: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        fn Layout(self: Pin<&mut wxWindow>) -> bool;
        fn SetAutoLayout(self: Pin<&mut wxWindow>, auto_layout: bool);
        fn GetAutoLayout(self: &wxWindow) -> bool;
        fn CaptureMouse(self: Pin<&mut wxWindow>);
        fn GetCaret(self: &wxWindow) -> *mut wxCaret;
        // BLOCKED: fn GetCursor(self: &wxWindow) -> &wxCursor;
        fn HasCapture(self: &wxWindow) -> bool;
        fn ReleaseMouse(self: Pin<&mut wxWindow>);
        unsafe fn SetCaret(self: Pin<&mut wxWindow>, caret: *mut wxCaret);
        fn SetCursor(self: Pin<&mut wxWindow>, cursor: &wxCursor) -> bool;
        fn WarpPointer(self: Pin<&mut wxWindow>, x: i32, y: i32);
        fn EnableTouchEvents(self: Pin<&mut wxWindow>, events_mask: i32) -> bool;
        // CXX_UNSUPPORTED: fn HitTest(self: &wxWindow, x: i32, y: i32) -> wxHitTest;
        // CXX_UNSUPPORTED: fn HitTest(self: &wxWindow, pt: &wxPoint) -> wxHitTest;
        // CXX_UNSUPPORTED: fn GetBorder(self: &wxWindow, flags: i32) -> wxBorder;
        // CXX_UNSUPPORTED: fn GetBorder(self: &wxWindow) -> wxBorder;
        fn DoUpdateWindowUI(self: Pin<&mut wxWindow>, event: Pin<&mut wxUpdateUIEvent>);
        // CXX_UNSUPPORTED: fn GetHandle(self: &wxWindow) -> WXWidget;
        fn HasMultiplePages(self: &wxWindow) -> bool;
        fn InheritAttributes(self: Pin<&mut wxWindow>);
        fn InitDialog(self: Pin<&mut wxWindow>);
        fn IsDoubleBuffered(self: &wxWindow) -> bool;
        fn SetDoubleBuffered(self: Pin<&mut wxWindow>, on: bool);
        fn IsRetained(self: &wxWindow) -> bool;
        fn IsThisEnabled(self: &wxWindow) -> bool;
        fn IsTopLevel(self: &wxWindow) -> bool;
        fn OnInternalIdle(self: Pin<&mut wxWindow>);
        fn SendIdleEvents(self: Pin<&mut wxWindow>, event: Pin<&mut wxIdleEvent>) -> bool;
        fn RegisterHotKey(self: Pin<&mut wxWindow>, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool;
        fn UnregisterHotKey(self: Pin<&mut wxWindow>, hotkey_id: i32) -> bool;
        // BLOCKED: fn UpdateWindowUI(self: Pin<&mut wxWindow>, flags: i32);
        // STATIC: fn GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        // STATIC: fn FindFocus() -> *mut wxWindow;
        // STATIC: unsafe fn FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        // STATIC: unsafe fn FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // STATIC: unsafe fn FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // STATIC: fn GetCapture() -> *mut wxWindow;
        // STATIC: fn NewControlId(count: i32) -> i32;
        // STATIC: fn UnreserveControlId(id: i32, count: i32);
        // CTOR: fn wxWindow() -> Window;
        // CTOR: unsafe fn wxWindow(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Window;
        // DTOR: fn ~wxWindow(self: Pin<&mut wxWindow>);
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;

        // CLASS: wxControl
        type wxControl;
        // CTOR: unsafe fn wxControl(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Control;
        // CTOR: fn wxControl() -> Control;
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        fn Command(self: Pin<&mut wxControl>, event: Pin<&mut wxCommandEvent>);
        // CXX_UNSUPPORTED: fn GetLabel(self: &wxControl) -> wxString;
        // CXX_UNSUPPORTED: fn GetLabelText(self: &wxControl) -> wxString;
        // CXX_UNSUPPORTED: fn GetSizeFromTextSize(self: &wxControl, xlen: i32, ylen: i32) -> wxSize;
        // CXX_UNSUPPORTED: fn GetSizeFromTextSize(self: &wxControl, tsize: &wxSize) -> wxSize;
        // CXX_UNSUPPORTED: fn GetSizeFromText(self: &wxControl, text: &wxString) -> wxSize;
        fn SetLabel(self: Pin<&mut wxControl>, label: &wxString);
        fn SetLabelText(self: Pin<&mut wxControl>, text: &wxString);
        fn SetLabelMarkup(self: Pin<&mut wxControl>, markup: &wxString) -> bool;
        // STATIC: fn GetLabelText(label: &wxString) -> wxString;
        // STATIC: fn RemoveMnemonics(str: &wxString) -> wxString;
        // STATIC: fn EscapeMnemonics(text: &wxString) -> wxString;
        // STATIC: fn Ellipsize(label: &wxString, dc: &wxDC, mode: wxEllipsizeMode, max_width: i32, flags: i32) -> wxString;

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
        fn SetBitmapCurrent(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn SetBitmapDisabled(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn SetBitmapFocus(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn SetBitmapLabel(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        fn SetBitmapPressed(self: Pin<&mut wxAnyButton>, bitmap: &wxBitmap);
        // CXX_UNSUPPORTED: fn GetBitmapMargins(self: Pin<&mut wxAnyButton>) -> wxSize;
        fn SetBitmapMargins(self: Pin<&mut wxAnyButton>, x: i32, y: i32);
        #[rust_name = "SetBitmapMargins1"]
        fn SetBitmapMargins(self: Pin<&mut wxAnyButton>, sz: &wxSize);
        // CXX_UNSUPPORTED: fn SetBitmapPosition(self: Pin<&mut wxAnyButton>, dir: wxDirection);

        // CLASS: wxButton
        type wxButton;
        // CTOR: fn wxButton() -> Button;
        // CTOR: unsafe fn wxButton(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Button;
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        fn GetAuthNeeded(self: &wxButton) -> bool;
        // CXX_UNSUPPORTED: fn GetLabel(self: &wxButton) -> wxString;
        fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
        fn SetDefault(self: Pin<&mut wxButton>) -> *mut wxWindow;
        fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
        // STATIC: unsafe fn GetDefaultSize(win: *mut wxWindow) -> wxSize;

        // CLASS: wxNonOwnedWindow
        type wxNonOwnedWindow;
        fn SetShape(self: Pin<&mut wxNonOwnedWindow>, region: &wxRegion) -> bool;
        #[rust_name = "SetShape1"]
        fn SetShape(self: Pin<&mut wxNonOwnedWindow>, path: &wxGraphicsPath) -> bool;

        // CLASS: wxTopLevelWindow
        type wxTopLevelWindow;
        // CTOR: fn wxTopLevelWindow() -> TopLevelWindow;
        // CTOR: unsafe fn wxTopLevelWindow(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> TopLevelWindow;
        // DTOR: fn ~wxTopLevelWindow(self: Pin<&mut wxTopLevelWindow>);
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxTopLevelWindow>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        fn CanSetTransparent(self: Pin<&mut wxTopLevelWindow>) -> bool;
        fn CenterOnScreen(self: Pin<&mut wxTopLevelWindow>, direction: i32);
        fn CentreOnScreen(self: Pin<&mut wxTopLevelWindow>, direction: i32);
        fn EnableCloseButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn EnableMaximizeButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn EnableMinimizeButton(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        fn GetDefaultItem(self: &wxTopLevelWindow) -> *mut wxWindow;
        // CXX_UNSUPPORTED: fn GetIcon(self: &wxTopLevelWindow) -> wxIcon;
        // BLOCKED: fn GetIcons(self: &wxTopLevelWindow) -> &wxIconBundle;
        // CXX_UNSUPPORTED: fn GetTitle(self: &wxTopLevelWindow) -> wxString;
        fn Iconize(self: Pin<&mut wxTopLevelWindow>, iconize: bool);
        fn IsActive(self: Pin<&mut wxTopLevelWindow>) -> bool;
        fn IsAlwaysMaximized(self: &wxTopLevelWindow) -> bool;
        fn IsFullScreen(self: &wxTopLevelWindow) -> bool;
        fn IsIconized(self: &wxTopLevelWindow) -> bool;
        fn IsMaximized(self: &wxTopLevelWindow) -> bool;
        // BLOCKED: fn IsUsingNativeDecorations(self: &wxTopLevelWindow) -> bool;
        fn Layout(self: Pin<&mut wxTopLevelWindow>) -> bool;
        fn Maximize(self: Pin<&mut wxTopLevelWindow>, maximize: bool);
        // BLOCKED: fn MSWGetSystemMenu(self: &wxTopLevelWindow) -> *mut wxMenu;
        fn RequestUserAttention(self: Pin<&mut wxTopLevelWindow>, flags: i32);
        fn Restore(self: Pin<&mut wxTopLevelWindow>);
        // BLOCKED: fn RestoreToGeometry(self: Pin<&mut wxTopLevelWindow>, ser: Pin<&mut GeometrySerializer>) -> bool;
        // BLOCKED: fn SaveGeometry(self: &wxTopLevelWindow, ser: &GeometrySerializer) -> bool;
        unsafe fn SetDefaultItem(self: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        unsafe fn SetTmpDefaultItem(self: Pin<&mut wxTopLevelWindow>, win: *mut wxWindow) -> *mut wxWindow;
        fn GetTmpDefaultItem(self: &wxTopLevelWindow) -> *mut wxWindow;
        fn SetIcon(self: Pin<&mut wxTopLevelWindow>, icon: &wxIcon);
        fn SetIcons(self: Pin<&mut wxTopLevelWindow>, icons: &wxIconBundle);
        fn SetMaxSize(self: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        fn SetMinSize(self: Pin<&mut wxTopLevelWindow>, size: &wxSize);
        fn SetSizeHints(self: Pin<&mut wxTopLevelWindow>, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32);
        #[rust_name = "SetSizeHints1"]
        fn SetSizeHints(self: Pin<&mut wxTopLevelWindow>, min_size: &wxSize, max_size: &wxSize, inc_size: &wxSize);
        fn SetTitle(self: Pin<&mut wxTopLevelWindow>, title: &wxString);
        fn SetTransparent(self: Pin<&mut wxTopLevelWindow>, alpha: u8) -> bool;
        fn ShouldPreventAppExit(self: &wxTopLevelWindow) -> bool;
        fn OSXSetModified(self: Pin<&mut wxTopLevelWindow>, modified: bool);
        fn OSXIsModified(self: &wxTopLevelWindow) -> bool;
        fn SetRepresentedFilename(self: Pin<&mut wxTopLevelWindow>, filename: &wxString);
        fn ShowWithoutActivating(self: Pin<&mut wxTopLevelWindow>);
        fn EnableFullScreenView(self: Pin<&mut wxTopLevelWindow>, enable: bool) -> bool;
        // BLOCKED: fn ShowFullScreen(self: Pin<&mut wxTopLevelWindow>, show: bool, style: i32) -> bool;
        // BLOCKED: fn UseNativeDecorations(self: Pin<&mut wxTopLevelWindow>, native: bool);
        // BLOCKED: fn UseNativeDecorationsByDefault(self: Pin<&mut wxTopLevelWindow>, native: bool);
        // STATIC: fn GetDefaultSize() -> wxSize;

        // CLASS: wxFrame
        type wxFrame;
        // CTOR: fn wxFrame() -> Frame;
        // CTOR: unsafe fn wxFrame(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Frame;
        // DTOR: fn ~wxFrame(self: Pin<&mut wxFrame>);
        fn Centre(self: Pin<&mut wxFrame>, direction: i32);
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxFrame>, parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;
        // BLOCKED: fn CreateStatusBar(self: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        // BLOCKED: fn CreateToolBar(self: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        fn DoGiveHelp(self: Pin<&mut wxFrame>, text: &wxString, show: bool);
        // CXX_UNSUPPORTED: fn GetClientAreaOrigin(self: &wxFrame) -> wxPoint;
        fn GetMenuBar(self: &wxFrame) -> *mut wxMenuBar;
        fn GetStatusBar(self: &wxFrame) -> *mut wxStatusBar;
        fn GetStatusBarPane(self: &wxFrame) -> i32;
        fn GetToolBar(self: &wxFrame) -> *mut wxToolBar;
        // BLOCKED: fn OnCreateStatusBar(self: Pin<&mut wxFrame>, number: i32, style: i32, id: i32, name: &wxString) -> *mut wxStatusBar;
        // BLOCKED: fn OnCreateToolBar(self: Pin<&mut wxFrame>, style: i32, id: i32, name: &wxString) -> *mut wxToolBar;
        fn ProcessCommand(self: Pin<&mut wxFrame>, id: i32) -> bool;
        unsafe fn SetMenuBar(self: Pin<&mut wxFrame>, menu_bar: *mut wxMenuBar);
        unsafe fn SetStatusBar(self: Pin<&mut wxFrame>, status_bar: *mut wxStatusBar);
        fn SetStatusBarPane(self: Pin<&mut wxFrame>, n: i32);
        fn SetStatusText(self: Pin<&mut wxFrame>, text: &wxString, number: i32);
        unsafe fn SetStatusWidths(self: Pin<&mut wxFrame>, n: i32, widths_field: *const i32);
        unsafe fn SetToolBar(self: Pin<&mut wxFrame>, tool_bar: *mut wxToolBar);
        // BLOCKED: fn MSWGetTaskBarButton(self: Pin<&mut wxFrame>) -> *mut wxTaskBarButton;
        fn PushStatusText(self: Pin<&mut wxFrame>, text: &wxString, number: i32);
        fn PopStatusText(self: Pin<&mut wxFrame>, number: i32);

        // CLASS: wxPoint
        type wxPoint;
        fn IsFullySpecified(self: &wxPoint) -> bool;
        fn SetDefaults(self: Pin<&mut wxPoint>, pt: &wxPoint);
        // BLOCKED: fn operator=(self: Pin<&mut wxPoint>, pt: &wxPoint) -> Pin<&mut wxPoint>;
        // BLOCKED: fn operator==(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> bool;
        // BLOCKED: fn operator!=(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> bool;
        // CXX_UNSUPPORTED: fn operator+(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> wxPoint;
        // CXX_UNSUPPORTED: fn operator-(self: Pin<&mut wxPoint>, p1: &wxPoint, p2: &wxPoint) -> wxPoint;
        // BLOCKED: fn operator+=(self: Pin<&mut wxPoint>, pt: &wxPoint) -> Pin<&mut wxPoint>;
        // BLOCKED: fn operator-=(self: Pin<&mut wxPoint>, pt: &wxPoint) -> Pin<&mut wxPoint>;
        // CXX_UNSUPPORTED: fn operator+(self: Pin<&mut wxPoint>, pt: &wxPoint, sz: &wxSize) -> wxPoint;
        // CXX_UNSUPPORTED: fn operator-(self: Pin<&mut wxPoint>, pt: &wxPoint, sz: &wxSize) -> wxPoint;
        // CXX_UNSUPPORTED: fn operator+(self: Pin<&mut wxPoint>, sz: &wxSize, pt: &wxPoint) -> wxPoint;
        // CXX_UNSUPPORTED: fn operator-(self: Pin<&mut wxPoint>, sz: &wxSize, pt: &wxPoint) -> wxPoint;
        // BLOCKED: fn operator+=(self: Pin<&mut wxPoint>, sz: &wxSize) -> Pin<&mut wxPoint>;
        // BLOCKED: fn operator-=(self: Pin<&mut wxPoint>, sz: &wxSize) -> Pin<&mut wxPoint>;
        // CXX_UNSUPPORTED: fn operator/(self: Pin<&mut wxPoint>, sz: &wxPoint, factor: i32) -> wxSize;
        // CXX_UNSUPPORTED: fn operator*(self: Pin<&mut wxPoint>, sz: &wxPoint, factor: i32) -> wxSize;
        // CXX_UNSUPPORTED: fn operator*(self: Pin<&mut wxPoint>, factor: i32, sz: &wxSize) -> wxSize;
        // BLOCKED: fn operator/=(self: Pin<&mut wxPoint>, factor: i32) -> Pin<&mut wxSize>;
        // BLOCKED: fn operator*=(self: Pin<&mut wxPoint>, factor: i32) -> Pin<&mut wxSize>;
        // CTOR: fn wxPoint() -> Point;
        // CTOR: fn wxPoint(x: i32, y: i32) -> Point;
        // CTOR: fn wxPoint(pt: &wxRealPoint) -> Point;

        // CLASS: wxSize
        type wxSize;
        // BLOCKED: fn operator=(self: Pin<&mut wxSize>, sz: &wxSize) -> Pin<&mut wxSize>;
        // BLOCKED: fn operator==(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> bool;
        // BLOCKED: fn operator!=(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> bool;
        // CXX_UNSUPPORTED: fn operator+(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> wxSize;
        // CXX_UNSUPPORTED: fn operator-(self: Pin<&mut wxSize>, s1: &wxSize, s2: &wxSize) -> wxSize;
        // BLOCKED: fn operator+=(self: Pin<&mut wxSize>, sz: &wxSize) -> Pin<&mut wxSize>;
        // BLOCKED: fn operator-=(self: Pin<&mut wxSize>, sz: &wxSize) -> Pin<&mut wxSize>;
        // CXX_UNSUPPORTED: fn operator/(self: Pin<&mut wxSize>, sz: &wxSize, factor: i32) -> wxSize;
        // CXX_UNSUPPORTED: fn operator*(self: Pin<&mut wxSize>, sz: &wxSize, factor: i32) -> wxSize;
        // CXX_UNSUPPORTED: fn operator*(self: Pin<&mut wxSize>, factor: i32, sz: &wxSize) -> wxSize;
        // BLOCKED: fn operator/=(self: Pin<&mut wxSize>, factor: i32) -> Pin<&mut wxSize>;
        // BLOCKED: fn operator*=(self: Pin<&mut wxSize>, factor: i32) -> Pin<&mut wxSize>;
        // CTOR: fn wxSize() -> Size;
        // CTOR: fn wxSize(width: i32, height: i32) -> Size;
        fn DecBy(self: Pin<&mut wxSize>, pt: &wxPoint);
        #[rust_name = "DecBy1"]
        fn DecBy(self: Pin<&mut wxSize>, size: &wxSize);
        #[rust_name = "DecBy2"]
        fn DecBy(self: Pin<&mut wxSize>, dx: i32, dy: i32);
        #[rust_name = "DecBy3"]
        fn DecBy(self: Pin<&mut wxSize>, d: i32);
        fn DecTo(self: Pin<&mut wxSize>, size: &wxSize);
        fn DecToIfSpecified(self: Pin<&mut wxSize>, size: &wxSize);
        fn GetHeight(self: &wxSize) -> i32;
        fn GetWidth(self: &wxSize) -> i32;
        fn IncBy(self: Pin<&mut wxSize>, pt: &wxPoint);
        #[rust_name = "IncBy1"]
        fn IncBy(self: Pin<&mut wxSize>, size: &wxSize);
        #[rust_name = "IncBy2"]
        fn IncBy(self: Pin<&mut wxSize>, dx: i32, dy: i32);
        #[rust_name = "IncBy3"]
        fn IncBy(self: Pin<&mut wxSize>, d: i32);
        fn IncTo(self: Pin<&mut wxSize>, size: &wxSize);
        fn IsFullySpecified(self: &wxSize) -> bool;
        // BLOCKED: fn Scale(self: Pin<&mut wxSize>, xscale: f64, yscale: f64) -> Pin<&mut wxSize>;
        fn Set(self: Pin<&mut wxSize>, width: i32, height: i32);
        fn SetDefaults(self: Pin<&mut wxSize>, size_default: &wxSize);
        fn SetHeight(self: Pin<&mut wxSize>, height: i32);
        fn SetWidth(self: Pin<&mut wxSize>, width: i32);

        // CLASS: wxValidator
        type wxValidator;
        // CTOR: fn wxValidator() -> Validator;
        // DTOR: fn ~wxValidator(self: Pin<&mut wxValidator>);
        fn Clone(self: &wxValidator) -> *mut wxObject;
        fn GetWindow(self: &wxValidator) -> *mut wxWindow;
        unsafe fn SetWindow(self: Pin<&mut wxValidator>, window: *mut wxWindow);
        fn TransferFromWindow(self: Pin<&mut wxValidator>) -> bool;
        fn TransferToWindow(self: Pin<&mut wxValidator>) -> bool;
        unsafe fn Validate(self: Pin<&mut wxValidator>, parent: *mut wxWindow) -> bool;
        // STATIC: fn SuppressBellOnError(suppress: bool);
        // STATIC: fn IsSilent() -> bool;
    }
    unsafe extern "C++" {
        // CLASS: wxObject
        fn NewObject() -> *mut wxObject;
        #[rust_name = "NewObject1"]
        fn NewObject(other: &wxObject) -> *mut wxObject;
        // CLASS: wxEvtHandler
        fn NewEvtHandler() -> *mut wxEvtHandler;
        // CLASS: wxWindow
        fn NewWindow() -> *mut wxWindow;
        #[rust_name = "NewWindow1"]
        unsafe fn NewWindow(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxWindow;
        // CLASS: wxControl
        unsafe fn NewControl(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxControl;
        #[rust_name = "NewControl1"]
        fn NewControl() -> *mut wxControl;
        // CLASS: wxAnyButton
        fn NewAnyButton() -> *mut wxAnyButton;
        // CLASS: wxButton
        fn NewButton() -> *mut wxButton;
        #[rust_name = "NewButton1"]
        unsafe fn NewButton(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxButton;
        // CLASS: wxNonOwnedWindow
        // CLASS: wxTopLevelWindow
        fn NewTopLevelWindow() -> *mut wxTopLevelWindow;
        #[rust_name = "NewTopLevelWindow1"]
        unsafe fn NewTopLevelWindow(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxTopLevelWindow;
        // CLASS: wxFrame
        fn NewFrame() -> *mut wxFrame;
        #[rust_name = "NewFrame1"]
        unsafe fn NewFrame(parent: *mut wxWindow, id: i32, title: &wxString, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxFrame;
        // CLASS: wxPoint
        fn NewPoint() -> *mut wxPoint;
        #[rust_name = "NewPoint1"]
        fn NewPoint(x: i32, y: i32) -> *mut wxPoint;
        #[rust_name = "NewPoint2"]
        fn NewPoint(pt: &wxRealPoint) -> *mut wxPoint;
        // CLASS: wxSize
        fn NewSize() -> *mut wxSize;
        #[rust_name = "NewSize1"]
        fn NewSize(width: i32, height: i32) -> *mut wxSize;
        // CLASS: wxValidator
        fn NewValidator() -> *mut wxValidator;
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
    pub fn new1(other: &ffi::wxObject) -> Object {
        Object(ffi::NewObject1(other))
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut ffi::wxClassInfo {
        self.pinned::<ffi::wxObject>().as_mut().GetClassInfo()
    }
    fn get_ref_data(&self) -> *mut ffi::wxObjectRefData {
        self.pinned::<ffi::wxObject>().as_mut().GetRefData()
    }
    fn is_kind_of(&self, info: *const ffi::wxClassInfo) -> bool {
        unsafe { self.pinned::<ffi::wxObject>().as_mut().IsKindOf(info) }
    }
    fn is_same_as(&self, obj: &ffi::wxObject) -> bool {
        self.pinned::<ffi::wxObject>().as_mut().IsSameAs(obj)
    }
    fn ref_(&self, clone: &ffi::wxObject) {
        self.pinned::<ffi::wxObject>().as_mut().Ref(clone)
    }
    fn set_ref_data(&self, data: *mut ffi::wxObjectRefData) {
        unsafe { self.pinned::<ffi::wxObject>().as_mut().SetRefData(data) }
    }
    fn un_ref(&self) {
        self.pinned::<ffi::wxObject>().as_mut().UnRef()
    }
    fn un_share(&self) {
        self.pinned::<ffi::wxObject>().as_mut().UnShare()
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
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().QueueEvent(event) }
    }
    fn add_pending_event(&self, event: &ffi::wxEvent) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().AddPendingEvent(event)
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter()
    fn process_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().ProcessEvent(event)
    }
    fn process_event_locally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().ProcessEventLocally(event)
    }
    fn safely_process_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().SafelyProcessEvent(event)
    }
    fn process_pending_events(&self) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().ProcessPendingEvents()
    }
    fn delete_pending_events(&self) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().DeletePendingEvents()
    }
    // CXX_UNSUPPORTED: fn Connect()
    // CXX_UNSUPPORTED: fn Connect()
    // CXX_UNSUPPORTED: fn Connect()
    // CXX_UNSUPPORTED: fn Disconnect()
    // CXX_UNSUPPORTED: fn Disconnect()
    // CXX_UNSUPPORTED: fn Disconnect()
    // CXX_UNSUPPORTED: fn Bind()
    // BLOCKED: fn Bind()
    // CXX_UNSUPPORTED: fn Unbind()
    // BLOCKED: fn Unbind()
    // BLOCKED: fn GetClientData()
    fn get_client_object(&self) -> *mut ffi::wxClientData {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetClientObject()
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut ffi::wxClientData) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().SetClientObject(data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetEvtHandlerEnabled()
    }
    fn get_next_handler(&self) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetNextHandler()
    }
    fn get_previous_handler(&self) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetPreviousHandler()
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().SetEvtHandlerEnabled(enabled)
    }
    fn set_next_handler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().SetNextHandler(handler) }
    }
    fn set_previous_handler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().SetPreviousHandler(handler) }
    }
    fn unlink(&self) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().Unlink()
    }
    fn is_unlinked(&self) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().IsUnlinked()
    }
    // STATIC: fn AddFilter()
    // STATIC: fn RemoveFilter()
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
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
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
        self.pinned::<ffi::wxWindow>().as_mut().AcceptsFocus()
    }
    fn accepts_focus_from_keyboard(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().AcceptsFocusFromKeyboard()
    }
    fn accepts_focus_recursively(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().AcceptsFocusRecursively()
    }
    fn disable_focus_from_keyboard(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().DisableFocusFromKeyboard()
    }
    fn is_focusable(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsFocusable()
    }
    fn can_accept_focus(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanAcceptFocus()
    }
    fn can_accept_focus_from_keyboard(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanAcceptFocusFromKeyboard()
    }
    fn has_focus(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasFocus()
    }
    fn set_can_focus(&self, can_focus: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetCanFocus(can_focus)
    }
    fn enable_visible_focus(&self, enable: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().EnableVisibleFocus(enable)
    }
    fn set_focus(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().SetFocus()
    }
    fn set_focus_from_kbd(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().SetFocusFromKbd()
    }
    // BLOCKED: fn AddChild()
    fn destroy_children(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().DestroyChildren()
    }
    // BLOCKED: fn FindWindow()
    // BLOCKED: fn FindWindow()
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn RemoveChild()
    fn get_grand_parent(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetGrandParent()
    }
    fn get_next_sibling(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetNextSibling()
    }
    fn get_parent(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetParent()
    }
    fn get_prev_sibling(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetPrevSibling()
    }
    // BLOCKED: fn IsDescendant()
    // BLOCKED: fn Reparent()
    fn always_show_scrollbars(&self, hflag: bool, vflag: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().AlwaysShowScrollbars(hflag, vflag)
    }
    fn get_scroll_pos(&self, orientation: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetScrollPos(orientation)
    }
    fn get_scroll_range(&self, orientation: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetScrollRange(orientation)
    }
    fn get_scroll_thumb(&self, orientation: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetScrollThumb(orientation)
    }
    fn can_scroll(&self, orient: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanScroll(orient)
    }
    fn has_scrollbar(&self, orient: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasScrollbar(orient)
    }
    fn is_scrollbar_always_shown(&self, orient: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsScrollbarAlwaysShown(orient)
    }
    fn scroll_lines(&self, lines: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ScrollLines(lines)
    }
    fn scroll_pages(&self, pages: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ScrollPages(pages)
    }
    fn scroll_window(&self, dx: i32, dy: i32, rect: *const ffi::wxRect) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().ScrollWindow(dx, dy, rect) }
    }
    fn line_up(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().LineUp()
    }
    fn line_down(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().LineDown()
    }
    fn page_up(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().PageUp()
    }
    fn page_down(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().PageDown()
    }
    fn set_scroll_pos(&self, orientation: i32, pos: i32, refresh: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetScrollPos(orientation, pos, refresh)
    }
    fn set_scrollbar(&self, orientation: i32, position: i32, thumb_size: i32, range: i32, refresh: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetScrollbar(orientation, position, thumb_size, range, refresh)
    }
    fn begin_repositioning_children(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().BeginRepositioningChildren()
    }
    fn end_repositioning_children(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().EndRepositioningChildren()
    }
    fn cache_best_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().CacheBestSize(size)
    }
    // CXX_UNSUPPORTED: fn ClientToWindowSize()
    // CXX_UNSUPPORTED: fn WindowToClientSize()
    fn fit(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Fit()
    }
    fn fit_inside(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().FitInside()
    }
    // CXX_UNSUPPORTED: fn FromDIP()
    // CXX_UNSUPPORTED: fn FromDIP()
    // BLOCKED: fn FromDIP()
    // CXX_UNSUPPORTED: fn ToDIP()
    // CXX_UNSUPPORTED: fn ToDIP()
    // BLOCKED: fn ToDIP()
    // CXX_UNSUPPORTED: fn GetBestSize()
    fn get_best_height(&self, width: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetBestHeight(width)
    }
    fn get_best_width(&self, height: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetBestWidth(height)
    }
    fn get_client_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetClientSize(width, height) }
    }
    // CXX_UNSUPPORTED: fn GetClientSize()
    // CXX_UNSUPPORTED: fn GetEffectiveMinSize()
    // CXX_UNSUPPORTED: fn GetMaxClientSize()
    // CXX_UNSUPPORTED: fn GetMaxSize()
    // CXX_UNSUPPORTED: fn GetMinClientSize()
    // CXX_UNSUPPORTED: fn GetMinSize()
    fn get_min_width(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMinWidth()
    }
    fn get_min_height(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMinHeight()
    }
    fn get_max_width(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMaxWidth()
    }
    fn get_max_height(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMaxHeight()
    }
    fn get_size(&self, width: *mut i32, height: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetSize(width, height) }
    }
    // CXX_UNSUPPORTED: fn GetSize()
    // CXX_UNSUPPORTED: fn GetVirtualSize()
    fn get_virtual_size1(&self, width: *mut i32, height: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetVirtualSize1(width, height) }
    }
    // CXX_UNSUPPORTED: fn GetBestVirtualSize()
    fn get_content_scale_factor(&self) -> f64 {
        self.pinned::<ffi::wxWindow>().as_mut().GetContentScaleFactor()
    }
    fn get_dpi_scale_factor(&self) -> f64 {
        self.pinned::<ffi::wxWindow>().as_mut().GetDPIScaleFactor()
    }
    // CXX_UNSUPPORTED: fn GetWindowBorderSize()
    fn inform_first_direction(&self, direction: i32, size: i32, available_other_dir: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().InformFirstDirection(direction, size, available_other_dir)
    }
    fn invalidate_best_size(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().InvalidateBestSize()
    }
    fn post_size_event(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().PostSizeEvent()
    }
    fn post_size_event_to_parent(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().PostSizeEventToParent()
    }
    fn send_size_event(&self, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SendSizeEvent(flags)
    }
    fn send_size_event_to_parent(&self, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SendSizeEventToParent(flags)
    }
    fn set_client_size(&self, width: i32, height: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetClientSize(width, height)
    }
    fn set_client_size1(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetClientSize1(size)
    }
    fn set_client_size2(&self, rect: &ffi::wxRect) {
        self.pinned::<ffi::wxWindow>().as_mut().SetClientSize2(rect)
    }
    fn set_containing_sizer(&self, sizer: *mut ffi::wxSizer) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetContainingSizer(sizer) }
    }
    fn set_initial_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetInitialSize(size)
    }
    fn set_max_client_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetMaxClientSize(size)
    }
    fn set_max_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetMaxSize(size)
    }
    fn set_min_client_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetMinClientSize(size)
    }
    fn set_min_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetMinSize(size)
    }
    // BLOCKED: fn SetSize()
    // BLOCKED: fn SetSize()
    // BLOCKED: fn SetSize()
    // BLOCKED: fn SetSize()
    fn set_size_hints(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        let min_size = &min_size.pinned::<ffi::wxSize>();
        let max_size = &max_size.pinned::<ffi::wxSize>();
        let inc_size = &inc_size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetSizeHints(min_size, max_size, inc_size)
    }
    fn set_size_hints1(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetSizeHints1(min_w, min_h, max_w, max_h, inc_w, inc_h)
    }
    fn set_virtual_size(&self, width: i32, height: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetVirtualSize(width, height)
    }
    fn set_virtual_size1(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxWindow>().as_mut().SetVirtualSize1(size)
    }
    // STATIC: fn FromDIP()
    // STATIC: fn FromDIP()
    // STATIC: fn FromDIP()
    // STATIC: fn ToDIP()
    // STATIC: fn ToDIP()
    // STATIC: fn ToDIP()
    fn center(&self, dir: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Center(dir)
    }
    fn center_on_parent(&self, dir: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().CenterOnParent(dir)
    }
    fn centre(&self, direction: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Centre(direction)
    }
    fn centre_on_parent(&self, direction: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().CentreOnParent(direction)
    }
    fn get_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetPosition(x, y) }
    }
    // CXX_UNSUPPORTED: fn GetPosition()
    // CXX_UNSUPPORTED: fn GetRect()
    fn get_screen_position(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetScreenPosition(x, y) }
    }
    // CXX_UNSUPPORTED: fn GetScreenPosition()
    // CXX_UNSUPPORTED: fn GetScreenRect()
    // CXX_UNSUPPORTED: fn GetClientAreaOrigin()
    // CXX_UNSUPPORTED: fn GetClientRect()
    fn move_(&self, x: i32, y: i32, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Move(x, y, flags)
    }
    fn move1(&self, pt: &Point, flags: i32) {
        let pt = &pt.pinned::<ffi::wxPoint>();
        self.pinned::<ffi::wxWindow>().as_mut().Move1(pt, flags)
    }
    fn set_position(&self, pt: &Point) {
        let pt = &pt.pinned::<ffi::wxPoint>();
        self.pinned::<ffi::wxWindow>().as_mut().SetPosition(pt)
    }
    fn client_to_screen(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().ClientToScreen(x, y) }
    }
    // CXX_UNSUPPORTED: fn ClientToScreen()
    // CXX_UNSUPPORTED: fn ConvertDialogToPixels()
    // CXX_UNSUPPORTED: fn ConvertDialogToPixels()
    // CXX_UNSUPPORTED: fn ConvertPixelsToDialog()
    // CXX_UNSUPPORTED: fn ConvertPixelsToDialog()
    fn screen_to_client(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().ScreenToClient(x, y) }
    }
    // CXX_UNSUPPORTED: fn ScreenToClient()
    fn clear_background(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().ClearBackground()
    }
    fn freeze(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Freeze()
    }
    fn thaw(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Thaw()
    }
    fn is_frozen(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsFrozen()
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn get_char_height(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetCharHeight()
    }
    fn get_char_width(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetCharWidth()
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    // CXX_UNSUPPORTED: fn GetDPI()
    // CXX_UNSUPPORTED: fn GetFont()
    // CXX_UNSUPPORTED: fn GetForegroundColour()
    fn get_text_extent(&self, string: &str, w: *mut i32, h: *mut i32, descent: *mut i32, external_leading: *mut i32, font: *const ffi::wxFont) {
        unsafe {
            let string = &crate::ffi::NewString(string);
            self.pinned::<ffi::wxWindow>().as_mut().GetTextExtent(string, w, h, descent, external_leading, font)
        }
    }
    // CXX_UNSUPPORTED: fn GetTextExtent()
    // BLOCKED: fn GetUpdateRegion()
    // CXX_UNSUPPORTED: fn GetUpdateClientRect()
    fn has_transparent_background(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasTransparentBackground()
    }
    fn refresh(&self, erase_background: bool, rect: *const ffi::wxRect) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().Refresh(erase_background, rect) }
    }
    fn refresh_rect(&self, rect: &ffi::wxRect, erase_background: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().RefreshRect(rect, erase_background)
    }
    fn update(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Update()
    }
    fn set_background_colour(&self, colour: &ffi::wxColour) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetBackgroundColour(colour)
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn is_transparent_background_supported(&self, reason: *mut ffi::wxString) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().IsTransparentBackgroundSupported(reason) }
    }
    fn set_font(&self, font: &ffi::wxFont) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetFont(font)
    }
    fn set_foreground_colour(&self, colour: &ffi::wxColour) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetForegroundColour(colour)
    }
    fn set_own_background_colour(&self, colour: &ffi::wxColour) {
        self.pinned::<ffi::wxWindow>().as_mut().SetOwnBackgroundColour(colour)
    }
    fn inherits_background_colour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().InheritsBackgroundColour()
    }
    fn use_bg_col(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UseBgCol()
    }
    fn use_background_colour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UseBackgroundColour()
    }
    fn set_own_font(&self, font: &ffi::wxFont) {
        self.pinned::<ffi::wxWindow>().as_mut().SetOwnFont(font)
    }
    fn set_own_foreground_colour(&self, colour: &ffi::wxColour) {
        self.pinned::<ffi::wxWindow>().as_mut().SetOwnForegroundColour(colour)
    }
    fn use_foreground_colour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UseForegroundColour()
    }
    fn inherits_foreground_colour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().InheritsForegroundColour()
    }
    fn set_palette(&self, pal: &ffi::wxPalette) {
        self.pinned::<ffi::wxWindow>().as_mut().SetPalette(pal)
    }
    fn should_inherit_colours(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ShouldInheritColours()
    }
    fn set_theme_enabled(&self, enable: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetThemeEnabled(enable)
    }
    fn get_theme_enabled(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().GetThemeEnabled()
    }
    fn can_set_transparent(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanSetTransparent()
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetTransparent(alpha)
    }
    fn get_event_handler(&self) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxWindow>().as_mut().GetEventHandler()
    }
    fn handle_as_navigation_key(&self, event: &ffi::wxKeyEvent) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HandleAsNavigationKey(event)
    }
    fn handle_window_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HandleWindowEvent(event)
    }
    fn process_window_event(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ProcessWindowEvent(event)
    }
    fn process_window_event_locally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ProcessWindowEventLocally(event)
    }
    fn pop_event_handler(&self, delete_handler: bool) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxWindow>().as_mut().PopEventHandler(delete_handler)
    }
    fn push_event_handler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().PushEventHandler(handler) }
    }
    fn remove_event_handler(&self, handler: *mut ffi::wxEvtHandler) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().RemoveEventHandler(handler) }
    }
    fn set_event_handler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetEventHandler(handler) }
    }
    fn set_next_handler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetNextHandler(handler) }
    }
    fn set_previous_handler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetPreviousHandler(handler) }
    }
    // BLOCKED: fn GetExtraStyle()
    // BLOCKED: fn GetWindowStyleFlag()
    // BLOCKED: fn GetWindowStyle()
    fn has_extra_style(&self, ex_flag: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasExtraStyle(ex_flag)
    }
    fn has_flag(&self, flag: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasFlag(flag)
    }
    // BLOCKED: fn SetExtraStyle()
    // BLOCKED: fn SetWindowStyleFlag()
    // BLOCKED: fn SetWindowStyle()
    fn toggle_window_style(&self, flag: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ToggleWindowStyle(flag)
    }
    fn move_after_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            self.pinned::<ffi::wxWindow>().as_mut().MoveAfterInTabOrder(win)
        }
    }
    fn move_before_in_tab_order<T: WindowMethods>(&self, win: Option<&T>) {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            self.pinned::<ffi::wxWindow>().as_mut().MoveBeforeInTabOrder(win)
        }
    }
    fn navigate(&self, flags: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Navigate(flags)
    }
    fn navigate_in(&self, flags: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().NavigateIn(flags)
    }
    fn lower(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Lower()
    }
    fn raise(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Raise()
    }
    fn hide(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Hide()
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn is_enabled(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsEnabled()
    }
    // BLOCKED: fn IsExposed()
    // BLOCKED: fn IsExposed()
    // BLOCKED: fn IsExposed()
    // BLOCKED: fn IsExposed()
    fn is_shown(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsShown()
    }
    fn is_shown_on_screen(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsShownOnScreen()
    }
    fn disable(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Disable()
    }
    fn enable(&self, enable: bool) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Enable(enable)
    }
    fn show(&self, show: bool) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Show(show)
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    // CXX_UNSUPPORTED: fn GetHelpText()
    fn set_help_text(&self, help_text: &str) {
        let help_text = &crate::ffi::NewString(help_text);
        self.pinned::<ffi::wxWindow>().as_mut().SetHelpText(help_text)
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn get_tool_tip(&self) -> *mut ffi::wxToolTip {
        self.pinned::<ffi::wxWindow>().as_mut().GetToolTip()
    }
    // CXX_UNSUPPORTED: fn GetToolTipText()
    fn set_tool_tip(&self, tip_string: &str) {
        let tip_string = &crate::ffi::NewString(tip_string);
        self.pinned::<ffi::wxWindow>().as_mut().SetToolTip(tip_string)
    }
    fn set_tool_tip1(&self, tip: *mut ffi::wxToolTip) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetToolTip1(tip) }
    }
    fn unset_tool_tip(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().UnsetToolTip()
    }
    fn get_popup_menu_selection_from_user(&self, menu: Pin<&mut ffi::wxMenu>, pos: &Point) -> i32 {
        let pos = &pos.pinned::<ffi::wxPoint>();
        self.pinned::<ffi::wxWindow>().as_mut().GetPopupMenuSelectionFromUser(menu, pos)
    }
    fn get_popup_menu_selection_from_user1(&self, menu: Pin<&mut ffi::wxMenu>, x: i32, y: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetPopupMenuSelectionFromUser1(menu, x, y)
    }
    fn popup_menu(&self, menu: *mut ffi::wxMenu, pos: &Point) -> bool {
        unsafe {
            let pos = &pos.pinned::<ffi::wxPoint>();
            self.pinned::<ffi::wxWindow>().as_mut().PopupMenu(menu, pos)
        }
    }
    fn popup_menu1(&self, menu: *mut ffi::wxMenu, x: i32, y: i32) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().PopupMenu1(menu, x, y) }
    }
    fn get_validator(&self) -> *mut ffi::wxValidator {
        self.pinned::<ffi::wxWindow>().as_mut().GetValidator()
    }
    fn set_validator(&self, validator: &Validator) {
        let validator = &validator.pinned::<ffi::wxValidator>();
        self.pinned::<ffi::wxWindow>().as_mut().SetValidator(validator)
    }
    fn transfer_data_from_window(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().TransferDataFromWindow()
    }
    fn transfer_data_to_window(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().TransferDataToWindow()
    }
    fn validate(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Validate()
    }
    fn get_id(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetId()
    }
    // CXX_UNSUPPORTED: fn GetLabel()
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn adjust_for_layout_direction(&self, x: i32, width: i32, width_total: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().AdjustForLayoutDirection(x, width, width_total)
    }
    // CXX_UNSUPPORTED: fn GetName()
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn set_id(&self, winid: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetId(winid)
    }
    fn set_label(&self, label: &str) {
        let label = &crate::ffi::NewString(label);
        self.pinned::<ffi::wxWindow>().as_mut().SetLabel(label)
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn set_name(&self, name: &str) {
        let name = &crate::ffi::NewString(name);
        self.pinned::<ffi::wxWindow>().as_mut().SetName(name)
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn get_accelerator_table(&self) -> *mut ffi::wxAcceleratorTable {
        self.pinned::<ffi::wxWindow>().as_mut().GetAcceleratorTable()
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn set_accelerator_table(&self, accel: &ffi::wxAcceleratorTable) {
        self.pinned::<ffi::wxWindow>().as_mut().SetAcceleratorTable(accel)
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn close(&self, force: bool) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Close(force)
    }
    fn destroy(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Destroy()
    }
    fn is_being_deleted(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsBeingDeleted()
    }
    fn get_drop_target(&self) -> *mut ffi::wxDropTarget {
        self.pinned::<ffi::wxWindow>().as_mut().GetDropTarget()
    }
    fn set_drop_target(&self, target: *mut ffi::wxDropTarget) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetDropTarget(target) }
    }
    fn drag_accept_files(&self, accept: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().DragAcceptFiles(accept)
    }
    fn get_containing_sizer(&self) -> *mut ffi::wxSizer {
        self.pinned::<ffi::wxWindow>().as_mut().GetContainingSizer()
    }
    fn get_sizer(&self) -> *mut ffi::wxSizer {
        self.pinned::<ffi::wxWindow>().as_mut().GetSizer()
    }
    fn set_sizer(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetSizer(sizer, delete_old) }
    }
    fn set_sizer_and_fit(&self, sizer: *mut ffi::wxSizer, delete_old: bool) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetSizerAndFit(sizer, delete_old) }
    }
    fn get_constraints(&self) -> *mut ffi::wxLayoutConstraints {
        self.pinned::<ffi::wxWindow>().as_mut().GetConstraints()
    }
    fn set_constraints(&self, constraints: *mut ffi::wxLayoutConstraints) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetConstraints(constraints) }
    }
    fn layout(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Layout()
    }
    fn set_auto_layout(&self, auto_layout: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetAutoLayout(auto_layout)
    }
    fn get_auto_layout(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().GetAutoLayout()
    }
    fn capture_mouse(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().CaptureMouse()
    }
    fn get_caret(&self) -> *mut ffi::wxCaret {
        self.pinned::<ffi::wxWindow>().as_mut().GetCaret()
    }
    // BLOCKED: fn GetCursor()
    fn has_capture(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasCapture()
    }
    fn release_mouse(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().ReleaseMouse()
    }
    fn set_caret(&self, caret: *mut ffi::wxCaret) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetCaret(caret) }
    }
    fn set_cursor(&self, cursor: &ffi::wxCursor) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetCursor(cursor)
    }
    fn warp_pointer(&self, x: i32, y: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().WarpPointer(x, y)
    }
    fn enable_touch_events(&self, events_mask: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().EnableTouchEvents(events_mask)
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder()
    fn do_update_window_ui(&self, event: Pin<&mut ffi::wxUpdateUIEvent>) {
        self.pinned::<ffi::wxWindow>().as_mut().DoUpdateWindowUI(event)
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn has_multiple_pages(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasMultiplePages()
    }
    fn inherit_attributes(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().InheritAttributes()
    }
    fn init_dialog(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().InitDialog()
    }
    fn is_double_buffered(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsDoubleBuffered()
    }
    fn set_double_buffered(&self, on: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetDoubleBuffered(on)
    }
    fn is_retained(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsRetained()
    }
    fn is_this_enabled(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsThisEnabled()
    }
    fn is_top_level(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsTopLevel()
    }
    fn on_internal_idle(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().OnInternalIdle()
    }
    fn send_idle_events(&self, event: Pin<&mut ffi::wxIdleEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SendIdleEvents(event)
    }
    fn register_hot_key(&self, hotkey_id: i32, modifiers: i32, virtual_key_code: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().RegisterHotKey(hotkey_id, modifiers, virtual_key_code)
    }
    fn unregister_hot_key(&self, hotkey_id: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UnregisterHotKey(hotkey_id)
    }
    // BLOCKED: fn UpdateWindowUI()
    // STATIC: fn GetClassDefaultAttributes()
    // STATIC: fn FindFocus()
    // STATIC: fn FindWindowById()
    // STATIC: fn FindWindowByLabel()
    // STATIC: fn FindWindowByName()
    // STATIC: fn GetCapture()
    // STATIC: fn NewControlId()
    // STATIC: fn UnreserveControlId()
    // DTOR: fn ~wxWindow()
    // BLOCKED: fn Create()
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
    // BLOCKED: fn Create()
    fn command(&self, event: Pin<&mut ffi::wxCommandEvent>) {
        self.pinned::<ffi::wxControl>().as_mut().Command(event)
    }
    // CXX_UNSUPPORTED: fn GetLabel()
    // CXX_UNSUPPORTED: fn GetLabelText()
    // CXX_UNSUPPORTED: fn GetSizeFromTextSize()
    // CXX_UNSUPPORTED: fn GetSizeFromTextSize()
    // CXX_UNSUPPORTED: fn GetSizeFromText()
    fn set_label(&self, label: &str) {
        let label = &crate::ffi::NewString(label);
        self.pinned::<ffi::wxControl>().as_mut().SetLabel(label)
    }
    fn set_label_text(&self, text: &str) {
        let text = &crate::ffi::NewString(text);
        self.pinned::<ffi::wxControl>().as_mut().SetLabelText(text)
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        let markup = &crate::ffi::NewString(markup);
        self.pinned::<ffi::wxControl>().as_mut().SetLabelMarkup(markup)
    }
    // STATIC: fn GetLabelText()
    // STATIC: fn RemoveMnemonics()
    // STATIC: fn EscapeMnemonics()
    // STATIC: fn Ellipsize()
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
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapCurrent(bitmap)
    }
    fn set_bitmap_disabled(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapDisabled(bitmap)
    }
    fn set_bitmap_focus(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapFocus(bitmap)
    }
    fn set_bitmap_label(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapLabel(bitmap)
    }
    fn set_bitmap_pressed(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapPressed(bitmap)
    }
    // CXX_UNSUPPORTED: fn GetBitmapMargins()
    fn set_bitmap_margins(&self, x: i32, y: i32) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapMargins(x, y)
    }
    fn set_bitmap_margins1(&self, sz: &Size) {
        let sz = &sz.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapMargins1(sz)
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
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
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
    // BLOCKED: fn Create()
    fn get_auth_needed(&self) -> bool {
        self.pinned::<ffi::wxButton>().as_mut().GetAuthNeeded()
    }
    // CXX_UNSUPPORTED: fn GetLabel()
    fn set_auth_needed(&self, needed: bool) {
        self.pinned::<ffi::wxButton>().as_mut().SetAuthNeeded(needed)
    }
    fn set_default(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxButton>().as_mut().SetDefault()
    }
    fn set_label(&self, label: &str) {
        let label = &crate::ffi::NewString(label);
        self.pinned::<ffi::wxButton>().as_mut().SetLabel(label)
    }
    // STATIC: fn GetDefaultSize()
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
        self.pinned::<ffi::wxNonOwnedWindow>().as_mut().SetShape(region)
    }
    fn set_shape1(&self, path: &ffi::wxGraphicsPath) -> bool {
        self.pinned::<ffi::wxNonOwnedWindow>().as_mut().SetShape1(path)
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
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
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
    // BLOCKED: fn Create()
    fn can_set_transparent(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().CanSetTransparent()
    }
    fn center_on_screen(&self, direction: i32) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().CenterOnScreen(direction)
    }
    fn centre_on_screen(&self, direction: i32) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().CentreOnScreen(direction)
    }
    fn enable_close_button(&self, enable: bool) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().EnableCloseButton(enable)
    }
    fn enable_maximize_button(&self, enable: bool) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().EnableMaximizeButton(enable)
    }
    fn enable_minimize_button(&self, enable: bool) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().EnableMinimizeButton(enable)
    }
    fn get_default_item(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().GetDefaultItem()
    }
    // CXX_UNSUPPORTED: fn GetIcon()
    // BLOCKED: fn GetIcons()
    // CXX_UNSUPPORTED: fn GetTitle()
    fn iconize(&self, iconize: bool) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().Iconize(iconize)
    }
    fn is_active(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().IsActive()
    }
    fn is_always_maximized(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().IsAlwaysMaximized()
    }
    fn is_full_screen(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().IsFullScreen()
    }
    fn is_iconized(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().IsIconized()
    }
    fn is_maximized(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().IsMaximized()
    }
    // BLOCKED: fn IsUsingNativeDecorations()
    fn layout(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().Layout()
    }
    fn maximize(&self, maximize: bool) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().Maximize(maximize)
    }
    // BLOCKED: fn MSWGetSystemMenu()
    fn request_user_attention(&self, flags: i32) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().RequestUserAttention(flags)
    }
    fn restore(&self) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().Restore()
    }
    // BLOCKED: fn RestoreToGeometry()
    // BLOCKED: fn SaveGeometry()
    fn set_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetDefaultItem(win)
        }
    }
    fn set_tmp_default_item<T: WindowMethods>(&self, win: Option<&T>) -> *mut ffi::wxWindow {
        unsafe {
            let win = match win {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetTmpDefaultItem(win)
        }
    }
    fn get_tmp_default_item(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().GetTmpDefaultItem()
    }
    fn set_icon(&self, icon: &ffi::wxIcon) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetIcon(icon)
    }
    fn set_icons(&self, icons: &ffi::wxIconBundle) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetIcons(icons)
    }
    fn set_max_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetMaxSize(size)
    }
    fn set_min_size(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetMinSize(size)
    }
    fn set_size_hints(&self, min_w: i32, min_h: i32, max_w: i32, max_h: i32, inc_w: i32, inc_h: i32) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetSizeHints(min_w, min_h, max_w, max_h, inc_w, inc_h)
    }
    fn set_size_hints1(&self, min_size: &Size, max_size: &Size, inc_size: &Size) {
        let min_size = &min_size.pinned::<ffi::wxSize>();
        let max_size = &max_size.pinned::<ffi::wxSize>();
        let inc_size = &inc_size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetSizeHints1(min_size, max_size, inc_size)
    }
    fn set_title(&self, title: &str) {
        let title = &crate::ffi::NewString(title);
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetTitle(title)
    }
    fn set_transparent(&self, alpha: u8) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetTransparent(alpha)
    }
    fn should_prevent_app_exit(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().ShouldPreventAppExit()
    }
    fn osx_set_modified(&self, modified: bool) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().OSXSetModified(modified)
    }
    fn osx_is_modified(&self) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().OSXIsModified()
    }
    fn set_represented_filename(&self, filename: &str) {
        let filename = &crate::ffi::NewString(filename);
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().SetRepresentedFilename(filename)
    }
    fn show_without_activating(&self) {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().ShowWithoutActivating()
    }
    fn enable_full_screen_view(&self, enable: bool) -> bool {
        self.pinned::<ffi::wxTopLevelWindow>().as_mut().EnableFullScreenView(enable)
    }
    // BLOCKED: fn ShowFullScreen()
    // BLOCKED: fn UseNativeDecorations()
    // BLOCKED: fn UseNativeDecorationsByDefault()
    // STATIC: fn GetDefaultSize()
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
            let pos = &pos.pinned::<ffi::wxPoint>();
            let size = &size.pinned::<ffi::wxSize>();
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
        self.pinned::<ffi::wxFrame>().as_mut().Centre(direction)
    }
    // BLOCKED: fn Create()
    // BLOCKED: fn CreateStatusBar()
    // BLOCKED: fn CreateToolBar()
    fn do_give_help(&self, text: &str, show: bool) {
        let text = &crate::ffi::NewString(text);
        self.pinned::<ffi::wxFrame>().as_mut().DoGiveHelp(text, show)
    }
    // CXX_UNSUPPORTED: fn GetClientAreaOrigin()
    fn get_menu_bar(&self) -> *mut ffi::wxMenuBar {
        self.pinned::<ffi::wxFrame>().as_mut().GetMenuBar()
    }
    fn get_status_bar(&self) -> *mut ffi::wxStatusBar {
        self.pinned::<ffi::wxFrame>().as_mut().GetStatusBar()
    }
    fn get_status_bar_pane(&self) -> i32 {
        self.pinned::<ffi::wxFrame>().as_mut().GetStatusBarPane()
    }
    fn get_tool_bar(&self) -> *mut ffi::wxToolBar {
        self.pinned::<ffi::wxFrame>().as_mut().GetToolBar()
    }
    // BLOCKED: fn OnCreateStatusBar()
    // BLOCKED: fn OnCreateToolBar()
    fn process_command(&self, id: i32) -> bool {
        self.pinned::<ffi::wxFrame>().as_mut().ProcessCommand(id)
    }
    fn set_menu_bar(&self, menu_bar: *mut ffi::wxMenuBar) {
        unsafe { self.pinned::<ffi::wxFrame>().as_mut().SetMenuBar(menu_bar) }
    }
    fn set_status_bar(&self, status_bar: *mut ffi::wxStatusBar) {
        unsafe { self.pinned::<ffi::wxFrame>().as_mut().SetStatusBar(status_bar) }
    }
    fn set_status_bar_pane(&self, n: i32) {
        self.pinned::<ffi::wxFrame>().as_mut().SetStatusBarPane(n)
    }
    fn set_status_text(&self, text: &str, number: i32) {
        let text = &crate::ffi::NewString(text);
        self.pinned::<ffi::wxFrame>().as_mut().SetStatusText(text, number)
    }
    fn set_status_widths(&self, n: i32, widths_field: *const i32) {
        unsafe { self.pinned::<ffi::wxFrame>().as_mut().SetStatusWidths(n, widths_field) }
    }
    fn set_tool_bar(&self, tool_bar: *mut ffi::wxToolBar) {
        unsafe { self.pinned::<ffi::wxFrame>().as_mut().SetToolBar(tool_bar) }
    }
    // BLOCKED: fn MSWGetTaskBarButton()
    fn push_status_text(&self, text: &str, number: i32) {
        let text = &crate::ffi::NewString(text);
        self.pinned::<ffi::wxFrame>().as_mut().PushStatusText(text, number)
    }
    fn pop_status_text(&self, number: i32) {
        self.pinned::<ffi::wxFrame>().as_mut().PopStatusText(number)
    }
}

// wxPoint
wx_class! { Point(wxPoint) impl
    PointMethods
}
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
}
pub trait PointMethods: WxRustMethods {
    fn is_fully_specified(&self) -> bool {
        self.pinned::<ffi::wxPoint>().as_mut().IsFullySpecified()
    }
    fn set_defaults(&self, pt: &Point) {
        let pt = &pt.pinned::<ffi::wxPoint>();
        self.pinned::<ffi::wxPoint>().as_mut().SetDefaults(pt)
    }
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // CXX_UNSUPPORTED: fn operator+()
    // CXX_UNSUPPORTED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // CXX_UNSUPPORTED: fn operator+()
    // CXX_UNSUPPORTED: fn operator-()
    // CXX_UNSUPPORTED: fn operator+()
    // CXX_UNSUPPORTED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // CXX_UNSUPPORTED: fn operator/()
    // CXX_UNSUPPORTED: fn operator*()
    // CXX_UNSUPPORTED: fn operator*()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
}

// wxSize
wx_class! { Size(wxSize) impl
    SizeMethods
}
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
}
pub trait SizeMethods: WxRustMethods {
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator!=()
    // CXX_UNSUPPORTED: fn operator+()
    // CXX_UNSUPPORTED: fn operator-()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator-=()
    // CXX_UNSUPPORTED: fn operator/()
    // CXX_UNSUPPORTED: fn operator*()
    // CXX_UNSUPPORTED: fn operator*()
    // BLOCKED: fn operator/=()
    // BLOCKED: fn operator*=()
    fn dec_by(&self, pt: &Point) {
        let pt = &pt.pinned::<ffi::wxPoint>();
        self.pinned::<ffi::wxSize>().as_mut().DecBy(pt)
    }
    fn dec_by1(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxSize>().as_mut().DecBy1(size)
    }
    fn dec_by2(&self, dx: i32, dy: i32) {
        self.pinned::<ffi::wxSize>().as_mut().DecBy2(dx, dy)
    }
    fn dec_by3(&self, d: i32) {
        self.pinned::<ffi::wxSize>().as_mut().DecBy3(d)
    }
    fn dec_to(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxSize>().as_mut().DecTo(size)
    }
    fn dec_to_if_specified(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxSize>().as_mut().DecToIfSpecified(size)
    }
    fn get_height(&self) -> i32 {
        self.pinned::<ffi::wxSize>().as_mut().GetHeight()
    }
    fn get_width(&self) -> i32 {
        self.pinned::<ffi::wxSize>().as_mut().GetWidth()
    }
    fn inc_by(&self, pt: &Point) {
        let pt = &pt.pinned::<ffi::wxPoint>();
        self.pinned::<ffi::wxSize>().as_mut().IncBy(pt)
    }
    fn inc_by1(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxSize>().as_mut().IncBy1(size)
    }
    fn inc_by2(&self, dx: i32, dy: i32) {
        self.pinned::<ffi::wxSize>().as_mut().IncBy2(dx, dy)
    }
    fn inc_by3(&self, d: i32) {
        self.pinned::<ffi::wxSize>().as_mut().IncBy3(d)
    }
    fn inc_to(&self, size: &Size) {
        let size = &size.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxSize>().as_mut().IncTo(size)
    }
    fn is_fully_specified(&self) -> bool {
        self.pinned::<ffi::wxSize>().as_mut().IsFullySpecified()
    }
    // BLOCKED: fn Scale()
    fn set(&self, width: i32, height: i32) {
        self.pinned::<ffi::wxSize>().as_mut().Set(width, height)
    }
    fn set_defaults(&self, size_default: &Size) {
        let size_default = &size_default.pinned::<ffi::wxSize>();
        self.pinned::<ffi::wxSize>().as_mut().SetDefaults(size_default)
    }
    fn set_height(&self, height: i32) {
        self.pinned::<ffi::wxSize>().as_mut().SetHeight(height)
    }
    fn set_width(&self, width: i32) {
        self.pinned::<ffi::wxSize>().as_mut().SetWidth(width)
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
        self.pinned::<ffi::wxValidator>().as_mut().Clone()
    }
    fn get_window(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxValidator>().as_mut().GetWindow()
    }
    fn set_window<T: WindowMethods>(&self, window: Option<&T>) {
        unsafe {
            let window = match window {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            self.pinned::<ffi::wxValidator>().as_mut().SetWindow(window)
        }
    }
    fn transfer_from_window(&self) -> bool {
        self.pinned::<ffi::wxValidator>().as_mut().TransferFromWindow()
    }
    fn transfer_to_window(&self) -> bool {
        self.pinned::<ffi::wxValidator>().as_mut().TransferToWindow()
    }
    fn validate<T: WindowMethods>(&self, parent: Option<&T>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => Pin::<&mut ffi::wxWindow>::into_inner_unchecked(r.pinned::<ffi::wxWindow>()),
                None => ptr::null_mut(),
            };
            self.pinned::<ffi::wxValidator>().as_mut().Validate(parent)
        }
    }
    // STATIC: fn SuppressBellOnError()
    // STATIC: fn IsSilent()
}

