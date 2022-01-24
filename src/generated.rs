#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::os::raw::c_char;
use std::pin::Pin;

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

        type wxPoint;
        type wxSize;
        type wxString;
        type wxValidator;
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

        // CLASS: wxObject
        type wxObject;
        // CTOR: fn wxObject(self: Pin<&mut wxObject>);
        // CTOR: fn wxObject(self: Pin<&mut wxObject>, other: &wxObject);
        // CXX_UNSUPPORTED: fn ~wxObject(self: Pin<&mut wxObject>);
        unsafe fn GetClassInfo(self: &wxObject) -> *mut wxClassInfo;
        unsafe fn GetRefData(self: &wxObject) -> *mut wxObjectRefData;
        unsafe fn IsKindOf(self: &wxObject, info: *const wxClassInfo) -> bool;
        fn IsSameAs(self: &wxObject, obj: &wxObject) -> bool;
        fn Ref(self: Pin<&mut wxObject>, clone: &wxObject);
        unsafe fn SetRefData(self: Pin<&mut wxObject>, data: *mut wxObjectRefData);
        fn UnRef(self: Pin<&mut wxObject>);
        fn UnShare(self: Pin<&mut wxObject>);
        // BLOCKED: unsafe fn operator delete(self: Pin<&mut wxObject>, buf: *mut void);
        // CXX_UNSUPPORTED: unsafe fn operator new(self: Pin<&mut wxObject>, size: size_t, filename: &wxString, lineNum: i32) -> *mut void;

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
        // CXX_UNSUPPORTED: unsafe fn Connect(self: Pin<&mut wxEvtHandler>, id: i32, lastId: i32, eventType: wxEventType, function: wxObjectEventFunction, userData: *mut wxObject, eventSink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect(self: Pin<&mut wxEvtHandler>, id: i32, eventType: wxEventType, function: wxObjectEventFunction, userData: *mut wxObject, eventSink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Connect(self: Pin<&mut wxEvtHandler>, eventType: wxEventType, function: wxObjectEventFunction, userData: *mut wxObject, eventSink: *mut wxEvtHandler);
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: Pin<&mut wxEvtHandler>, eventType: wxEventType, function: wxObjectEventFunction, userData: *mut wxObject, eventSink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: Pin<&mut wxEvtHandler>, id: i32, eventType: wxEventType, function: wxObjectEventFunction, userData: *mut wxObject, eventSink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Disconnect(self: Pin<&mut wxEvtHandler>, id: i32, lastId: i32, eventType: wxEventType, function: wxObjectEventFunction, userData: *mut wxObject, eventSink: *mut wxEvtHandler) -> bool;
        // CXX_UNSUPPORTED: unsafe fn Bind(self: Pin<&mut wxEvtHandler>, eventType: &EventTag, functor: Functor, id: i32, lastId: i32, userData: *mut wxObject);
        // BLOCKED: unsafe fn Bind(self: Pin<&mut wxEvtHandler>, eventType: &EventTag, method: *mut void(Class::, handler: *mut EventHandler, id: i32, lastId: i32, userData: *mut wxObject);
        // CXX_UNSUPPORTED: unsafe fn Unbind(self: Pin<&mut wxEvtHandler>, eventType: &EventTag, functor: Functor, id: i32, lastId: i32, userData: *mut wxObject) -> bool;
        // BLOCKED: unsafe fn Unbind(self: Pin<&mut wxEvtHandler>, eventType: &EventTag, method: *mut void(Class::, handler: *mut EventHandler, id: i32, lastId: i32, userData: *mut wxObject) -> bool;
        // BLOCKED: unsafe fn GetClientData(self: &wxEvtHandler) -> *mut void;
        unsafe fn GetClientObject(self: &wxEvtHandler) -> *mut wxClientData;
        // BLOCKED: unsafe fn SetClientData(self: Pin<&mut wxEvtHandler>, data: *mut void);
        unsafe fn SetClientObject(self: Pin<&mut wxEvtHandler>, data: *mut wxClientData);
        fn GetEvtHandlerEnabled(self: &wxEvtHandler) -> bool;
        unsafe fn GetNextHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        unsafe fn GetPreviousHandler(self: &wxEvtHandler) -> *mut wxEvtHandler;
        fn SetEvtHandlerEnabled(self: Pin<&mut wxEvtHandler>, enabled: bool);
        unsafe fn SetNextHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        unsafe fn SetPreviousHandler(self: Pin<&mut wxEvtHandler>, handler: *mut wxEvtHandler);
        fn Unlink(self: Pin<&mut wxEvtHandler>);
        fn IsUnlinked(self: &wxEvtHandler) -> bool;
        // BLOCKED: unsafe fn AddFilter(filter: *mut wxEventFilter);
        // BLOCKED: unsafe fn RemoveFilter(filter: *mut wxEventFilter);
        // CTOR: fn wxEvtHandler(self: Pin<&mut wxEvtHandler>);
        // CXX_UNSUPPORTED: fn ~wxEvtHandler(self: Pin<&mut wxEvtHandler>);

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
        fn SetCanFocus(self: Pin<&mut wxWindow>, canFocus: bool);
        fn EnableVisibleFocus(self: Pin<&mut wxWindow>, enable: bool);
        fn SetFocus(self: Pin<&mut wxWindow>);
        fn SetFocusFromKbd(self: Pin<&mut wxWindow>);
        // BLOCKED: unsafe fn AddChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        fn DestroyChildren(self: Pin<&mut wxWindow>) -> bool;
        // BLOCKED: unsafe fn FindWindow(self: &wxWindow, id: i32) -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindow(self: &wxWindow, name: &wxString) -> *mut wxWindow;
        fn GetChildren(self: Pin<&mut wxWindow>) -> Pin<&mut wxWindowList>;
        #[rust_name = "GetChildren1"]
        fn GetChildren(self: &wxWindow) -> &wxWindowList;
        // BLOCKED: unsafe fn RemoveChild(self: Pin<&mut wxWindow>, child: *mut wxWindow);
        unsafe fn GetGrandParent(self: &wxWindow) -> *mut wxWindow;
        unsafe fn GetNextSibling(self: &wxWindow) -> *mut wxWindow;
        unsafe fn GetParent(self: &wxWindow) -> *mut wxWindow;
        unsafe fn GetPrevSibling(self: &wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn IsDescendant(self: &wxWindow, win: *mut wxWindow) -> bool;
        // BLOCKED: unsafe fn Reparent(self: Pin<&mut wxWindow>, newParent: *mut wxWindow) -> bool;
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
        fn SetScrollbar(self: Pin<&mut wxWindow>, orientation: i32, position: i32, thumbSize: i32, range: i32, refresh: bool);
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
        fn InformFirstDirection(self: Pin<&mut wxWindow>, direction: i32, size: i32, availableOtherDir: i32) -> bool;
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
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, x: i32, y: i32, width: i32, height: i32, sizeFlags: i32);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, rect: &wxRect);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // BLOCKED: fn SetSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        fn SetSizeHints(self: Pin<&mut wxWindow>, minSize: &wxSize, maxSize: &wxSize, incSize: &wxSize);
        #[rust_name = "SetSizeHints1"]
        fn SetSizeHints(self: Pin<&mut wxWindow>, minW: i32, minH: i32, maxW: i32, maxH: i32, incW: i32, incH: i32);
        fn SetVirtualSize(self: Pin<&mut wxWindow>, width: i32, height: i32);
        #[rust_name = "SetVirtualSize1"]
        fn SetVirtualSize(self: Pin<&mut wxWindow>, size: &wxSize);
        // CXX_UNSUPPORTED: unsafe fn FromDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: unsafe fn FromDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // BLOCKED: unsafe fn FromDIP(d: i32, w: *const wxWindow) -> i32;
        // CXX_UNSUPPORTED: unsafe fn ToDIP(sz: &wxSize, w: *const wxWindow) -> wxSize;
        // CXX_UNSUPPORTED: unsafe fn ToDIP(pt: &wxPoint, w: *const wxWindow) -> wxPoint;
        // BLOCKED: unsafe fn ToDIP(d: i32, w: *const wxWindow) -> i32;
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
        unsafe fn GetTextExtent(self: &wxWindow, string: &wxString, w: *mut i32, h: *mut i32, descent: *mut i32, externalLeading: *mut i32, font: *const wxFont);
        // CXX_UNSUPPORTED: fn GetTextExtent(self: &wxWindow, string: &wxString) -> wxSize;
        fn GetUpdateRegion(self: &wxWindow) -> &wxRegion;
        // CXX_UNSUPPORTED: fn GetUpdateClientRect(self: &wxWindow) -> wxRect;
        fn HasTransparentBackground(self: Pin<&mut wxWindow>) -> bool;
        unsafe fn Refresh(self: Pin<&mut wxWindow>, eraseBackground: bool, rect: *const wxRect);
        fn RefreshRect(self: Pin<&mut wxWindow>, rect: &wxRect, eraseBackground: bool);
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
        unsafe fn GetEventHandler(self: &wxWindow) -> *mut wxEvtHandler;
        fn HandleAsNavigationKey(self: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        fn HandleWindowEvent(self: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessWindowEvent(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessWindowEventLocally(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        unsafe fn PopEventHandler(self: Pin<&mut wxWindow>, deleteHandler: bool) -> *mut wxEvtHandler;
        unsafe fn PushEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn RemoveEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler) -> bool;
        unsafe fn SetEventHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn SetNextHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        unsafe fn SetPreviousHandler(self: Pin<&mut wxWindow>, handler: *mut wxEvtHandler);
        // BLOCKED: fn GetExtraStyle(self: &wxWindow) -> i32;
        // BLOCKED: fn GetWindowStyleFlag(self: &wxWindow) -> i32;
        // BLOCKED: fn GetWindowStyle(self: &wxWindow) -> i32;
        fn HasExtraStyle(self: &wxWindow, exFlag: i32) -> bool;
        fn HasFlag(self: &wxWindow, flag: i32) -> bool;
        // BLOCKED: fn SetExtraStyle(self: Pin<&mut wxWindow>, exStyle: i32);
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
        fn SetHelpText(self: Pin<&mut wxWindow>, helpText: &wxString);
        // CXX_UNSUPPORTED: fn GetHelpTextAtPoint(self: &wxWindow, point: &wxPoint, origin: wxHelpEvent::Origin) -> wxString;
        unsafe fn GetToolTip(self: &wxWindow) -> *mut wxToolTip;
        // CXX_UNSUPPORTED: fn GetToolTipText(self: &wxWindow) -> wxString;
        fn SetToolTip(self: Pin<&mut wxWindow>, tipString: &wxString);
        #[rust_name = "SetToolTip1"]
        unsafe fn SetToolTip(self: Pin<&mut wxWindow>, tip: *mut wxToolTip);
        fn UnsetToolTip(self: Pin<&mut wxWindow>);
        fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, pos: &wxPoint) -> i32;
        #[rust_name = "GetPopupMenuSelectionFromUser1"]
        fn GetPopupMenuSelectionFromUser(self: Pin<&mut wxWindow>, menu: Pin<&mut wxMenu>, x: i32, y: i32) -> i32;
        unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, pos: &wxPoint) -> bool;
        #[rust_name = "PopupMenu1"]
        unsafe fn PopupMenu(self: Pin<&mut wxWindow>, menu: *mut wxMenu, x: i32, y: i32) -> bool;
        unsafe fn GetValidator(self: Pin<&mut wxWindow>) -> *mut wxValidator;
        fn SetValidator(self: Pin<&mut wxWindow>, validator: &wxValidator);
        fn TransferDataFromWindow(self: Pin<&mut wxWindow>) -> bool;
        fn TransferDataToWindow(self: Pin<&mut wxWindow>) -> bool;
        fn Validate(self: Pin<&mut wxWindow>) -> bool;
        fn GetId(self: &wxWindow) -> i32;
        // CXX_UNSUPPORTED: fn GetLabel(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: fn GetLayoutDirection(self: &wxWindow) -> wxLayoutDirection;
        fn AdjustForLayoutDirection(self: &wxWindow, x: i32, width: i32, widthTotal: i32) -> i32;
        // CXX_UNSUPPORTED: fn GetName(self: &wxWindow) -> wxString;
        // CXX_UNSUPPORTED: fn GetWindowVariant(self: &wxWindow) -> wxWindowVariant;
        fn SetId(self: Pin<&mut wxWindow>, winid: i32);
        fn SetLabel(self: Pin<&mut wxWindow>, label: &wxString);
        // CXX_UNSUPPORTED: fn SetLayoutDirection(self: Pin<&mut wxWindow>, dir: wxLayoutDirection);
        fn SetName(self: Pin<&mut wxWindow>, name: &wxString);
        // CXX_UNSUPPORTED: fn SetWindowVariant(self: Pin<&mut wxWindow>, variant: wxWindowVariant);
        unsafe fn GetAcceleratorTable(self: Pin<&mut wxWindow>) -> *mut wxAcceleratorTable;
        // CXX_UNSUPPORTED: unsafe fn GetAccessible(self: Pin<&mut wxWindow>) -> *mut wxAccessible;
        fn SetAcceleratorTable(self: Pin<&mut wxWindow>, accel: &wxAcceleratorTable);
        // CXX_UNSUPPORTED: unsafe fn SetAccessible(self: Pin<&mut wxWindow>, accessible: *mut wxAccessible);
        fn Close(self: Pin<&mut wxWindow>, force: bool) -> bool;
        fn Destroy(self: Pin<&mut wxWindow>) -> bool;
        fn IsBeingDeleted(self: &wxWindow) -> bool;
        unsafe fn GetDropTarget(self: &wxWindow) -> *mut wxDropTarget;
        unsafe fn SetDropTarget(self: Pin<&mut wxWindow>, target: *mut wxDropTarget);
        fn DragAcceptFiles(self: Pin<&mut wxWindow>, accept: bool);
        unsafe fn GetContainingSizer(self: &wxWindow) -> *mut wxSizer;
        unsafe fn GetSizer(self: &wxWindow) -> *mut wxSizer;
        unsafe fn SetSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, deleteOld: bool);
        unsafe fn SetSizerAndFit(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, deleteOld: bool);
        unsafe fn GetConstraints(self: &wxWindow) -> *mut wxLayoutConstraints;
        unsafe fn SetConstraints(self: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        fn Layout(self: Pin<&mut wxWindow>) -> bool;
        fn SetAutoLayout(self: Pin<&mut wxWindow>, autoLayout: bool);
        fn GetAutoLayout(self: &wxWindow) -> bool;
        fn CaptureMouse(self: Pin<&mut wxWindow>);
        unsafe fn GetCaret(self: &wxWindow) -> *mut wxCaret;
        fn GetCursor(self: &wxWindow) -> &wxCursor;
        fn HasCapture(self: &wxWindow) -> bool;
        fn ReleaseMouse(self: Pin<&mut wxWindow>);
        unsafe fn SetCaret(self: Pin<&mut wxWindow>, caret: *mut wxCaret);
        fn SetCursor(self: Pin<&mut wxWindow>, cursor: &wxCursor) -> bool;
        fn WarpPointer(self: Pin<&mut wxWindow>, x: i32, y: i32);
        fn EnableTouchEvents(self: Pin<&mut wxWindow>, eventsMask: i32) -> bool;
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
        fn RegisterHotKey(self: Pin<&mut wxWindow>, hotkeyId: i32, modifiers: i32, virtualKeyCode: i32) -> bool;
        fn UnregisterHotKey(self: Pin<&mut wxWindow>, hotkeyId: i32) -> bool;
        // BLOCKED: fn UpdateWindowUI(self: Pin<&mut wxWindow>, flags: i32);
        // CXX_UNSUPPORTED: fn GetClassDefaultAttributes(variant: wxWindowVariant) -> wxVisualAttributes;
        // BLOCKED: unsafe fn FindFocus() -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn GetCapture() -> *mut wxWindow;
        // BLOCKED: fn NewControlId(count: i32) -> i32;
        // BLOCKED: fn UnreserveControlId(id: i32, count: i32);
        // CTOR: fn wxWindow(self: Pin<&mut wxWindow>);
        // CTOR: unsafe fn wxWindow(self: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString);
        // CXX_UNSUPPORTED: fn ~wxWindow(self: Pin<&mut wxWindow>);
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> bool;

        // CLASS: wxControl
        type wxControl;
        // CTOR: unsafe fn wxControl(self: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
        // CTOR: fn wxControl(self: Pin<&mut wxControl>);
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
        // CXX_UNSUPPORTED: fn GetLabelText(label: &wxString) -> wxString;
        // CXX_UNSUPPORTED: fn RemoveMnemonics(str: &wxString) -> wxString;
        // CXX_UNSUPPORTED: fn EscapeMnemonics(text: &wxString) -> wxString;
        // CXX_UNSUPPORTED: fn Ellipsize(label: &wxString, dc: &wxDC, mode: wxEllipsizeMode, maxWidth: i32, flags: i32) -> wxString;

        // CLASS: wxAnyButton
        type wxAnyButton;
        // CTOR: fn wxAnyButton(self: Pin<&mut wxAnyButton>);
        // CXX_UNSUPPORTED: fn ~wxAnyButton(self: Pin<&mut wxAnyButton>);
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
        // CTOR: fn wxButton(self: Pin<&mut wxButton>);
        // CTOR: unsafe fn wxButton(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
        // BLOCKED: unsafe fn Create(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> bool;
        fn GetAuthNeeded(self: &wxButton) -> bool;
        // CXX_UNSUPPORTED: fn GetLabel(self: &wxButton) -> wxString;
        fn SetAuthNeeded(self: Pin<&mut wxButton>, needed: bool);
        unsafe fn SetDefault(self: Pin<&mut wxButton>) -> *mut wxWindow;
        fn SetLabel(self: Pin<&mut wxButton>, label: &wxString);
        // CXX_UNSUPPORTED: unsafe fn GetDefaultSize(win: *mut wxWindow) -> wxSize;
    }
}

pub trait ObjectMethods {
    unsafe fn as_ptr(&self) -> UnsafeAnyPtr;
    fn pinned<T>(&self) -> Pin<&mut T> {
        unsafe { Pin::new_unchecked(&mut *(self.as_ptr() as *mut _)) }
    }
}

// wxObject
wx_class! { Object(wxObject) impl
}

impl wxObject {
    pub fn new(self: Pin<&mut wxObject>) -> Object {
        Object(ffi::NewObject())
    }
    pub fn new1(self: Pin<&mut wxObject>, other: &wxObject) -> Object {
        Object(ffi::NewObject(other))
    }
}
// wxEvtHandler
wx_class! { EvtHandler(wxEvtHandler) impl
}

impl wxEvtHandler {
    pub fn new(self: Pin<&mut wxEvtHandler>) -> EvtHandler {
        EvtHandler(ffi::NewEvtHandler())
    }
}
// wxWindow
wx_class! { Window(wxWindow) impl
}

impl wxWindow {
    pub fn new(self: Pin<&mut wxWindow>) -> Window {
        Window(ffi::NewWindow())
    }
    pub fn new1(self: Pin<&mut wxWindow>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> Window {
        Window(ffi::NewWindow(parent, id, pos, size, style, name))
    }
}
// wxControl
wx_class! { Control(wxControl) impl
}

impl wxControl {
    pub fn new(self: Pin<&mut wxControl>, parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Control {
        Control(ffi::NewControl(parent, id, pos, size, style, validator, name))
    }
    pub fn new1(self: Pin<&mut wxControl>) -> Control {
        Control(ffi::NewControl())
    }
}
// wxAnyButton
wx_class! { AnyButton(wxAnyButton) impl
}

impl wxAnyButton {
    pub fn new(self: Pin<&mut wxAnyButton>) -> AnyButton {
        AnyButton(ffi::NewAnyButton())
    }
}
// wxButton
wx_class! { Button(wxButton) impl
}

impl wxButton {
    pub fn new(self: Pin<&mut wxButton>) -> Button {
        Button(ffi::NewButton())
    }
    pub fn new1(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> Button {
        Button(ffi::NewButton(parent, id, label, pos, size, style, validator, name))
    }
}
