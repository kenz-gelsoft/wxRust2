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
        // CXX_UNSUPPORTED: fn operator new(self: Pin<&mut wxObject>, size: size_t, filename: &wxString, lineNum: i32) -> *mut void;

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
        // BLOCKED: unsafe fn AddFilter(filter: *mut wxEventFilter);
        // BLOCKED: unsafe fn RemoveFilter(filter: *mut wxEventFilter);
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
        fn SetCanFocus(self: Pin<&mut wxWindow>, canFocus: bool);
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
        // BLOCKED: fn GetUpdateRegion(self: &wxWindow) -> &wxRegion;
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
        fn GetEventHandler(self: &wxWindow) -> *mut wxEvtHandler;
        fn HandleAsNavigationKey(self: Pin<&mut wxWindow>, event: &wxKeyEvent) -> bool;
        fn HandleWindowEvent(self: &wxWindow, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessWindowEvent(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn ProcessWindowEventLocally(self: Pin<&mut wxWindow>, event: Pin<&mut wxEvent>) -> bool;
        fn PopEventHandler(self: Pin<&mut wxWindow>, deleteHandler: bool) -> *mut wxEvtHandler;
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
        fn GetToolTip(self: &wxWindow) -> *mut wxToolTip;
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
        fn GetValidator(self: Pin<&mut wxWindow>) -> *mut wxValidator;
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
        unsafe fn SetSizer(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, deleteOld: bool);
        unsafe fn SetSizerAndFit(self: Pin<&mut wxWindow>, sizer: *mut wxSizer, deleteOld: bool);
        fn GetConstraints(self: &wxWindow) -> *mut wxLayoutConstraints;
        unsafe fn SetConstraints(self: Pin<&mut wxWindow>, constraints: *mut wxLayoutConstraints);
        fn Layout(self: Pin<&mut wxWindow>) -> bool;
        fn SetAutoLayout(self: Pin<&mut wxWindow>, autoLayout: bool);
        fn GetAutoLayout(self: &wxWindow) -> bool;
        fn CaptureMouse(self: Pin<&mut wxWindow>);
        fn GetCaret(self: &wxWindow) -> *mut wxCaret;
        // BLOCKED: fn GetCursor(self: &wxWindow) -> &wxCursor;
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
        // BLOCKED: fn FindFocus() -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindowById(id: i32, parent: *const wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindowByLabel(label: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // BLOCKED: unsafe fn FindWindowByName(name: &wxString, parent: *const wxWindow) -> *mut wxWindow;
        // BLOCKED: fn GetCapture() -> *mut wxWindow;
        // BLOCKED: fn NewControlId(count: i32) -> i32;
        // BLOCKED: fn UnreserveControlId(id: i32, count: i32);
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
        // CXX_UNSUPPORTED: fn GetLabelText(label: &wxString) -> wxString;
        // CXX_UNSUPPORTED: fn RemoveMnemonics(str: &wxString) -> wxString;
        // CXX_UNSUPPORTED: fn EscapeMnemonics(text: &wxString) -> wxString;
        // CXX_UNSUPPORTED: fn Ellipsize(label: &wxString, dc: &wxDC, mode: wxEllipsizeMode, maxWidth: i32, flags: i32) -> wxString;

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
        // CXX_UNSUPPORTED: unsafe fn GetDefaultSize(win: *mut wxWindow) -> wxSize;
    }
    unsafe extern "C++" {
        fn NewObject() -> *mut wxObject;
        #[rust_name = "NewObject1"]
        fn NewObject1(other: &wxObject) -> *mut wxObject;
        fn NewEvtHandler() -> *mut wxEvtHandler;
        fn NewWindow() -> *mut wxWindow;
        #[rust_name = "NewWindow1"]
        unsafe fn NewWindow1(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, name: &wxString) -> *mut wxWindow;
        unsafe fn NewControl(parent: *mut wxWindow, id: i32, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxControl;
        #[rust_name = "NewControl1"]
        fn NewControl1() -> *mut wxControl;
        fn NewAnyButton() -> *mut wxAnyButton;
        fn NewButton() -> *mut wxButton;
        #[rust_name = "NewButton1"]
        unsafe fn NewButton1(parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) -> *mut wxButton;
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
}
trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn GetClassInfo(&self) -> *mut ffi::wxClassInfo {
        self.pinned::<ffi::wxObject>().as_mut().GetClassInfo()
    }
    fn GetRefData(&self) -> *mut ffi::wxObjectRefData {
        self.pinned::<ffi::wxObject>().as_mut().GetRefData()
    }
    fn IsKindOf(&self, info: *const ffi::wxClassInfo) -> bool {
        unsafe { self.pinned::<ffi::wxObject>().as_mut().IsKindOf(info) }
    }
    fn IsSameAs(&self, obj: &ffi::wxObject) -> bool {
        self.pinned::<ffi::wxObject>().as_mut().IsSameAs(obj)
    }
    fn Ref(&self, clone: &ffi::wxObject) {
        self.pinned::<ffi::wxObject>().as_mut().Ref(clone)
    }
    fn SetRefData(&self, data: *mut ffi::wxObjectRefData) {
        unsafe { self.pinned::<ffi::wxObject>().as_mut().SetRefData(data) }
    }
    fn UnRef(&self) {
        self.pinned::<ffi::wxObject>().as_mut().UnRef()
    }
    fn UnShare(&self) {
        self.pinned::<ffi::wxObject>().as_mut().UnShare()
    }
    // BLOCKED: fn operator delete()
    // CXX_UNSUPPORTED: fn operator new()
}

// wxEvtHandler
wx_class! { EvtHandler(wxEvtHandler) impl
    EvtHandlerMethods
}
impl EvtHandler {
    pub fn new() -> EvtHandler {
        EvtHandler(ffi::NewEvtHandler())
    }
}
trait EvtHandlerMethods: WxRustMethods {
    fn QueueEvent(&self, event: *mut ffi::wxEvent) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().QueueEvent(event) }
    }
    fn AddPendingEvent(&self, event: &ffi::wxEvent) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().AddPendingEvent(event)
    }
    // CXX_UNSUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter()
    fn ProcessEvent(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().ProcessEvent(event)
    }
    fn ProcessEventLocally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().ProcessEventLocally(event)
    }
    fn SafelyProcessEvent(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().SafelyProcessEvent(event)
    }
    fn ProcessPendingEvents(&self) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().ProcessPendingEvents()
    }
    fn DeletePendingEvents(&self) {
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
    fn GetClientObject(&self) -> *mut ffi::wxClientData {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetClientObject()
    }
    // BLOCKED: fn SetClientData()
    fn SetClientObject(&self, data: *mut ffi::wxClientData) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().SetClientObject(data) }
    }
    fn GetEvtHandlerEnabled(&self) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetEvtHandlerEnabled()
    }
    fn GetNextHandler(&self) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetNextHandler()
    }
    fn GetPreviousHandler(&self) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxEvtHandler>().as_mut().GetPreviousHandler()
    }
    fn SetEvtHandlerEnabled(&self, enabled: bool) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().SetEvtHandlerEnabled(enabled)
    }
    fn SetNextHandler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().SetNextHandler(handler) }
    }
    fn SetPreviousHandler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxEvtHandler>().as_mut().SetPreviousHandler(handler) }
    }
    fn Unlink(&self) {
        self.pinned::<ffi::wxEvtHandler>().as_mut().Unlink()
    }
    fn IsUnlinked(&self) -> bool {
        self.pinned::<ffi::wxEvtHandler>().as_mut().IsUnlinked()
    }
    // BLOCKED: fn AddFilter()
    // BLOCKED: fn RemoveFilter()
    // DTOR: fn ~wxEvtHandler()
}

// wxWindow
wx_class! { Window(wxWindow) impl
    WindowMethods
}
impl Window {
    pub fn new() -> Window {
        Window(ffi::NewWindow())
    }
    pub fn new1(parent: *mut ffi::wxWindow, id: i32, pos: &ffi::wxPoint, size: &ffi::wxSize, style: i32, name: &ffi::wxString) -> Window {
        unsafe { Window(ffi::NewWindow1(parent, id, pos, size, style, name)) }
    }
}
trait WindowMethods: WxRustMethods {
    fn AcceptsFocus(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().AcceptsFocus()
    }
    fn AcceptsFocusFromKeyboard(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().AcceptsFocusFromKeyboard()
    }
    fn AcceptsFocusRecursively(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().AcceptsFocusRecursively()
    }
    fn DisableFocusFromKeyboard(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().DisableFocusFromKeyboard()
    }
    fn IsFocusable(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsFocusable()
    }
    fn CanAcceptFocus(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanAcceptFocus()
    }
    fn CanAcceptFocusFromKeyboard(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanAcceptFocusFromKeyboard()
    }
    fn HasFocus(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasFocus()
    }
    fn SetCanFocus(&self, canFocus: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetCanFocus(canFocus)
    }
    fn EnableVisibleFocus(&self, enable: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().EnableVisibleFocus(enable)
    }
    fn SetFocus(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().SetFocus()
    }
    fn SetFocusFromKbd(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().SetFocusFromKbd()
    }
    // BLOCKED: fn AddChild()
    fn DestroyChildren(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().DestroyChildren()
    }
    // BLOCKED: fn FindWindow()
    // BLOCKED: fn FindWindow()
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn GetChildren()
    // BLOCKED: fn RemoveChild()
    fn GetGrandParent(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetGrandParent()
    }
    fn GetNextSibling(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetNextSibling()
    }
    fn GetParent(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetParent()
    }
    fn GetPrevSibling(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxWindow>().as_mut().GetPrevSibling()
    }
    // BLOCKED: fn IsDescendant()
    // BLOCKED: fn Reparent()
    fn AlwaysShowScrollbars(&self, hflag: bool, vflag: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().AlwaysShowScrollbars(hflag, vflag)
    }
    fn GetScrollPos(&self, orientation: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetScrollPos(orientation)
    }
    fn GetScrollRange(&self, orientation: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetScrollRange(orientation)
    }
    fn GetScrollThumb(&self, orientation: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetScrollThumb(orientation)
    }
    fn CanScroll(&self, orient: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanScroll(orient)
    }
    fn HasScrollbar(&self, orient: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasScrollbar(orient)
    }
    fn IsScrollbarAlwaysShown(&self, orient: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsScrollbarAlwaysShown(orient)
    }
    fn ScrollLines(&self, lines: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ScrollLines(lines)
    }
    fn ScrollPages(&self, pages: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ScrollPages(pages)
    }
    fn ScrollWindow(&self, dx: i32, dy: i32, rect: *const ffi::wxRect) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().ScrollWindow(dx, dy, rect) }
    }
    fn LineUp(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().LineUp()
    }
    fn LineDown(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().LineDown()
    }
    fn PageUp(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().PageUp()
    }
    fn PageDown(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().PageDown()
    }
    fn SetScrollPos(&self, orientation: i32, pos: i32, refresh: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetScrollPos(orientation, pos, refresh)
    }
    fn SetScrollbar(&self, orientation: i32, position: i32, thumbSize: i32, range: i32, refresh: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetScrollbar(orientation, position, thumbSize, range, refresh)
    }
    fn BeginRepositioningChildren(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().BeginRepositioningChildren()
    }
    fn EndRepositioningChildren(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().EndRepositioningChildren()
    }
    fn CacheBestSize(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().CacheBestSize(size)
    }
    // CXX_UNSUPPORTED: fn ClientToWindowSize()
    // CXX_UNSUPPORTED: fn WindowToClientSize()
    fn Fit(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Fit()
    }
    fn FitInside(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().FitInside()
    }
    // CXX_UNSUPPORTED: fn FromDIP()
    // CXX_UNSUPPORTED: fn FromDIP()
    // BLOCKED: fn FromDIP()
    // CXX_UNSUPPORTED: fn ToDIP()
    // CXX_UNSUPPORTED: fn ToDIP()
    // BLOCKED: fn ToDIP()
    // CXX_UNSUPPORTED: fn GetBestSize()
    fn GetBestHeight(&self, width: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetBestHeight(width)
    }
    fn GetBestWidth(&self, height: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetBestWidth(height)
    }
    fn GetClientSize(&self, width: *mut i32, height: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetClientSize(width, height) }
    }
    // CXX_UNSUPPORTED: fn GetClientSize()
    // CXX_UNSUPPORTED: fn GetEffectiveMinSize()
    // CXX_UNSUPPORTED: fn GetMaxClientSize()
    // CXX_UNSUPPORTED: fn GetMaxSize()
    // CXX_UNSUPPORTED: fn GetMinClientSize()
    // CXX_UNSUPPORTED: fn GetMinSize()
    fn GetMinWidth(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMinWidth()
    }
    fn GetMinHeight(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMinHeight()
    }
    fn GetMaxWidth(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMaxWidth()
    }
    fn GetMaxHeight(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetMaxHeight()
    }
    fn GetSize(&self, width: *mut i32, height: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetSize(width, height) }
    }
    // CXX_UNSUPPORTED: fn GetSize()
    // CXX_UNSUPPORTED: fn GetVirtualSize()
    fn GetVirtualSize1(&self, width: *mut i32, height: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetVirtualSize1(width, height) }
    }
    // CXX_UNSUPPORTED: fn GetBestVirtualSize()
    fn GetContentScaleFactor(&self) -> f64 {
        self.pinned::<ffi::wxWindow>().as_mut().GetContentScaleFactor()
    }
    fn GetDPIScaleFactor(&self) -> f64 {
        self.pinned::<ffi::wxWindow>().as_mut().GetDPIScaleFactor()
    }
    // CXX_UNSUPPORTED: fn GetWindowBorderSize()
    fn InformFirstDirection(&self, direction: i32, size: i32, availableOtherDir: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().InformFirstDirection(direction, size, availableOtherDir)
    }
    fn InvalidateBestSize(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().InvalidateBestSize()
    }
    fn PostSizeEvent(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().PostSizeEvent()
    }
    fn PostSizeEventToParent(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().PostSizeEventToParent()
    }
    fn SendSizeEvent(&self, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SendSizeEvent(flags)
    }
    fn SendSizeEventToParent(&self, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SendSizeEventToParent(flags)
    }
    fn SetClientSize(&self, width: i32, height: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetClientSize(width, height)
    }
    fn SetClientSize1(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetClientSize1(size)
    }
    fn SetClientSize2(&self, rect: &ffi::wxRect) {
        self.pinned::<ffi::wxWindow>().as_mut().SetClientSize2(rect)
    }
    fn SetContainingSizer(&self, sizer: *mut ffi::wxSizer) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetContainingSizer(sizer) }
    }
    fn SetInitialSize(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetInitialSize(size)
    }
    fn SetMaxClientSize(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetMaxClientSize(size)
    }
    fn SetMaxSize(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetMaxSize(size)
    }
    fn SetMinClientSize(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetMinClientSize(size)
    }
    fn SetMinSize(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetMinSize(size)
    }
    // BLOCKED: fn SetSize()
    // BLOCKED: fn SetSize()
    // BLOCKED: fn SetSize()
    // BLOCKED: fn SetSize()
    fn SetSizeHints(&self, minSize: &ffi::wxSize, maxSize: &ffi::wxSize, incSize: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetSizeHints(minSize, maxSize, incSize)
    }
    fn SetSizeHints1(&self, minW: i32, minH: i32, maxW: i32, maxH: i32, incW: i32, incH: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetSizeHints1(minW, minH, maxW, maxH, incW, incH)
    }
    fn SetVirtualSize(&self, width: i32, height: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetVirtualSize(width, height)
    }
    fn SetVirtualSize1(&self, size: &ffi::wxSize) {
        self.pinned::<ffi::wxWindow>().as_mut().SetVirtualSize1(size)
    }
    // CXX_UNSUPPORTED: fn FromDIP()
    // CXX_UNSUPPORTED: fn FromDIP()
    // BLOCKED: fn FromDIP()
    // CXX_UNSUPPORTED: fn ToDIP()
    // CXX_UNSUPPORTED: fn ToDIP()
    // BLOCKED: fn ToDIP()
    fn Center(&self, dir: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Center(dir)
    }
    fn CenterOnParent(&self, dir: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().CenterOnParent(dir)
    }
    fn Centre(&self, direction: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Centre(direction)
    }
    fn CentreOnParent(&self, direction: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().CentreOnParent(direction)
    }
    fn GetPosition(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetPosition(x, y) }
    }
    // CXX_UNSUPPORTED: fn GetPosition()
    // CXX_UNSUPPORTED: fn GetRect()
    fn GetScreenPosition(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetScreenPosition(x, y) }
    }
    // CXX_UNSUPPORTED: fn GetScreenPosition()
    // CXX_UNSUPPORTED: fn GetScreenRect()
    // CXX_UNSUPPORTED: fn GetClientAreaOrigin()
    // CXX_UNSUPPORTED: fn GetClientRect()
    fn Move(&self, x: i32, y: i32, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Move(x, y, flags)
    }
    fn Move1(&self, pt: &ffi::wxPoint, flags: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().Move1(pt, flags)
    }
    fn SetPosition(&self, pt: &ffi::wxPoint) {
        self.pinned::<ffi::wxWindow>().as_mut().SetPosition(pt)
    }
    fn ClientToScreen(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().ClientToScreen(x, y) }
    }
    // CXX_UNSUPPORTED: fn ClientToScreen()
    // CXX_UNSUPPORTED: fn ConvertDialogToPixels()
    // CXX_UNSUPPORTED: fn ConvertDialogToPixels()
    // CXX_UNSUPPORTED: fn ConvertPixelsToDialog()
    // CXX_UNSUPPORTED: fn ConvertPixelsToDialog()
    fn ScreenToClient(&self, x: *mut i32, y: *mut i32) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().ScreenToClient(x, y) }
    }
    // CXX_UNSUPPORTED: fn ScreenToClient()
    fn ClearBackground(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().ClearBackground()
    }
    fn Freeze(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Freeze()
    }
    fn Thaw(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Thaw()
    }
    fn IsFrozen(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsFrozen()
    }
    // CXX_UNSUPPORTED: fn GetBackgroundColour()
    // CXX_UNSUPPORTED: fn GetBackgroundStyle()
    fn GetCharHeight(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetCharHeight()
    }
    fn GetCharWidth(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetCharWidth()
    }
    // CXX_UNSUPPORTED: fn GetDefaultAttributes()
    // CXX_UNSUPPORTED: fn GetDPI()
    // CXX_UNSUPPORTED: fn GetFont()
    // CXX_UNSUPPORTED: fn GetForegroundColour()
    fn GetTextExtent(&self, string: &ffi::wxString, w: *mut i32, h: *mut i32, descent: *mut i32, externalLeading: *mut i32, font: *const ffi::wxFont) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().GetTextExtent(string, w, h, descent, externalLeading, font) }
    }
    // CXX_UNSUPPORTED: fn GetTextExtent()
    // BLOCKED: fn GetUpdateRegion()
    // CXX_UNSUPPORTED: fn GetUpdateClientRect()
    fn HasTransparentBackground(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasTransparentBackground()
    }
    fn Refresh(&self, eraseBackground: bool, rect: *const ffi::wxRect) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().Refresh(eraseBackground, rect) }
    }
    fn RefreshRect(&self, rect: &ffi::wxRect, eraseBackground: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().RefreshRect(rect, eraseBackground)
    }
    fn Update(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Update()
    }
    fn SetBackgroundColour(&self, colour: &ffi::wxColour) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetBackgroundColour(colour)
    }
    // CXX_UNSUPPORTED: fn SetBackgroundStyle()
    fn IsTransparentBackgroundSupported(&self, reason: *mut ffi::wxString) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().IsTransparentBackgroundSupported(reason) }
    }
    fn SetFont(&self, font: &ffi::wxFont) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetFont(font)
    }
    fn SetForegroundColour(&self, colour: &ffi::wxColour) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetForegroundColour(colour)
    }
    fn SetOwnBackgroundColour(&self, colour: &ffi::wxColour) {
        self.pinned::<ffi::wxWindow>().as_mut().SetOwnBackgroundColour(colour)
    }
    fn InheritsBackgroundColour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().InheritsBackgroundColour()
    }
    fn UseBgCol(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UseBgCol()
    }
    fn UseBackgroundColour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UseBackgroundColour()
    }
    fn SetOwnFont(&self, font: &ffi::wxFont) {
        self.pinned::<ffi::wxWindow>().as_mut().SetOwnFont(font)
    }
    fn SetOwnForegroundColour(&self, colour: &ffi::wxColour) {
        self.pinned::<ffi::wxWindow>().as_mut().SetOwnForegroundColour(colour)
    }
    fn UseForegroundColour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UseForegroundColour()
    }
    fn InheritsForegroundColour(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().InheritsForegroundColour()
    }
    fn SetPalette(&self, pal: &ffi::wxPalette) {
        self.pinned::<ffi::wxWindow>().as_mut().SetPalette(pal)
    }
    fn ShouldInheritColours(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ShouldInheritColours()
    }
    fn SetThemeEnabled(&self, enable: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetThemeEnabled(enable)
    }
    fn GetThemeEnabled(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().GetThemeEnabled()
    }
    fn CanSetTransparent(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().CanSetTransparent()
    }
    fn SetTransparent(&self, alpha: u8) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetTransparent(alpha)
    }
    fn GetEventHandler(&self) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxWindow>().as_mut().GetEventHandler()
    }
    fn HandleAsNavigationKey(&self, event: &ffi::wxKeyEvent) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HandleAsNavigationKey(event)
    }
    fn HandleWindowEvent(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HandleWindowEvent(event)
    }
    fn ProcessWindowEvent(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ProcessWindowEvent(event)
    }
    fn ProcessWindowEventLocally(&self, event: Pin<&mut ffi::wxEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ProcessWindowEventLocally(event)
    }
    fn PopEventHandler(&self, deleteHandler: bool) -> *mut ffi::wxEvtHandler {
        self.pinned::<ffi::wxWindow>().as_mut().PopEventHandler(deleteHandler)
    }
    fn PushEventHandler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().PushEventHandler(handler) }
    }
    fn RemoveEventHandler(&self, handler: *mut ffi::wxEvtHandler) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().RemoveEventHandler(handler) }
    }
    fn SetEventHandler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetEventHandler(handler) }
    }
    fn SetNextHandler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetNextHandler(handler) }
    }
    fn SetPreviousHandler(&self, handler: *mut ffi::wxEvtHandler) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetPreviousHandler(handler) }
    }
    // BLOCKED: fn GetExtraStyle()
    // BLOCKED: fn GetWindowStyleFlag()
    // BLOCKED: fn GetWindowStyle()
    fn HasExtraStyle(&self, exFlag: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasExtraStyle(exFlag)
    }
    fn HasFlag(&self, flag: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasFlag(flag)
    }
    // BLOCKED: fn SetExtraStyle()
    // BLOCKED: fn SetWindowStyleFlag()
    // BLOCKED: fn SetWindowStyle()
    fn ToggleWindowStyle(&self, flag: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().ToggleWindowStyle(flag)
    }
    fn MoveAfterInTabOrder(&self, win: *mut ffi::wxWindow) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().MoveAfterInTabOrder(win) }
    }
    fn MoveBeforeInTabOrder(&self, win: *mut ffi::wxWindow) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().MoveBeforeInTabOrder(win) }
    }
    fn Navigate(&self, flags: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Navigate(flags)
    }
    fn NavigateIn(&self, flags: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().NavigateIn(flags)
    }
    fn Lower(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Lower()
    }
    fn Raise(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().Raise()
    }
    fn Hide(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Hide()
    }
    // CXX_UNSUPPORTED: fn HideWithEffect()
    fn IsEnabled(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsEnabled()
    }
    // BLOCKED: fn IsExposed()
    // BLOCKED: fn IsExposed()
    // BLOCKED: fn IsExposed()
    // BLOCKED: fn IsExposed()
    fn IsShown(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsShown()
    }
    fn IsShownOnScreen(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsShownOnScreen()
    }
    fn Disable(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Disable()
    }
    fn Enable(&self, enable: bool) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Enable(enable)
    }
    fn Show(&self, show: bool) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Show(show)
    }
    // CXX_UNSUPPORTED: fn ShowWithEffect()
    // CXX_UNSUPPORTED: fn GetHelpText()
    fn SetHelpText(&self, helpText: &ffi::wxString) {
        self.pinned::<ffi::wxWindow>().as_mut().SetHelpText(helpText)
    }
    // CXX_UNSUPPORTED: fn GetHelpTextAtPoint()
    fn GetToolTip(&self) -> *mut ffi::wxToolTip {
        self.pinned::<ffi::wxWindow>().as_mut().GetToolTip()
    }
    // CXX_UNSUPPORTED: fn GetToolTipText()
    fn SetToolTip(&self, tipString: &ffi::wxString) {
        self.pinned::<ffi::wxWindow>().as_mut().SetToolTip(tipString)
    }
    fn SetToolTip1(&self, tip: *mut ffi::wxToolTip) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetToolTip1(tip) }
    }
    fn UnsetToolTip(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().UnsetToolTip()
    }
    fn GetPopupMenuSelectionFromUser(&self, menu: Pin<&mut ffi::wxMenu>, pos: &ffi::wxPoint) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetPopupMenuSelectionFromUser(menu, pos)
    }
    fn GetPopupMenuSelectionFromUser1(&self, menu: Pin<&mut ffi::wxMenu>, x: i32, y: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetPopupMenuSelectionFromUser1(menu, x, y)
    }
    fn PopupMenu(&self, menu: *mut ffi::wxMenu, pos: &ffi::wxPoint) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().PopupMenu(menu, pos) }
    }
    fn PopupMenu1(&self, menu: *mut ffi::wxMenu, x: i32, y: i32) -> bool {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().PopupMenu1(menu, x, y) }
    }
    fn GetValidator(&self) -> *mut ffi::wxValidator {
        self.pinned::<ffi::wxWindow>().as_mut().GetValidator()
    }
    fn SetValidator(&self, validator: &ffi::wxValidator) {
        self.pinned::<ffi::wxWindow>().as_mut().SetValidator(validator)
    }
    fn TransferDataFromWindow(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().TransferDataFromWindow()
    }
    fn TransferDataToWindow(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().TransferDataToWindow()
    }
    fn Validate(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Validate()
    }
    fn GetId(&self) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().GetId()
    }
    // CXX_UNSUPPORTED: fn GetLabel()
    // CXX_UNSUPPORTED: fn GetLayoutDirection()
    fn AdjustForLayoutDirection(&self, x: i32, width: i32, widthTotal: i32) -> i32 {
        self.pinned::<ffi::wxWindow>().as_mut().AdjustForLayoutDirection(x, width, widthTotal)
    }
    // CXX_UNSUPPORTED: fn GetName()
    // CXX_UNSUPPORTED: fn GetWindowVariant()
    fn SetId(&self, winid: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().SetId(winid)
    }
    fn SetLabel(&self, label: &ffi::wxString) {
        self.pinned::<ffi::wxWindow>().as_mut().SetLabel(label)
    }
    // CXX_UNSUPPORTED: fn SetLayoutDirection()
    fn SetName(&self, name: &ffi::wxString) {
        self.pinned::<ffi::wxWindow>().as_mut().SetName(name)
    }
    // CXX_UNSUPPORTED: fn SetWindowVariant()
    fn GetAcceleratorTable(&self) -> *mut ffi::wxAcceleratorTable {
        self.pinned::<ffi::wxWindow>().as_mut().GetAcceleratorTable()
    }
    // CXX_UNSUPPORTED: fn GetAccessible()
    fn SetAcceleratorTable(&self, accel: &ffi::wxAcceleratorTable) {
        self.pinned::<ffi::wxWindow>().as_mut().SetAcceleratorTable(accel)
    }
    // CXX_UNSUPPORTED: fn SetAccessible()
    fn Close(&self, force: bool) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Close(force)
    }
    fn Destroy(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Destroy()
    }
    fn IsBeingDeleted(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsBeingDeleted()
    }
    fn GetDropTarget(&self) -> *mut ffi::wxDropTarget {
        self.pinned::<ffi::wxWindow>().as_mut().GetDropTarget()
    }
    fn SetDropTarget(&self, target: *mut ffi::wxDropTarget) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetDropTarget(target) }
    }
    fn DragAcceptFiles(&self, accept: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().DragAcceptFiles(accept)
    }
    fn GetContainingSizer(&self) -> *mut ffi::wxSizer {
        self.pinned::<ffi::wxWindow>().as_mut().GetContainingSizer()
    }
    fn GetSizer(&self) -> *mut ffi::wxSizer {
        self.pinned::<ffi::wxWindow>().as_mut().GetSizer()
    }
    fn SetSizer(&self, sizer: *mut ffi::wxSizer, deleteOld: bool) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetSizer(sizer, deleteOld) }
    }
    fn SetSizerAndFit(&self, sizer: *mut ffi::wxSizer, deleteOld: bool) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetSizerAndFit(sizer, deleteOld) }
    }
    fn GetConstraints(&self) -> *mut ffi::wxLayoutConstraints {
        self.pinned::<ffi::wxWindow>().as_mut().GetConstraints()
    }
    fn SetConstraints(&self, constraints: *mut ffi::wxLayoutConstraints) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetConstraints(constraints) }
    }
    fn Layout(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().Layout()
    }
    fn SetAutoLayout(&self, autoLayout: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetAutoLayout(autoLayout)
    }
    fn GetAutoLayout(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().GetAutoLayout()
    }
    fn CaptureMouse(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().CaptureMouse()
    }
    fn GetCaret(&self) -> *mut ffi::wxCaret {
        self.pinned::<ffi::wxWindow>().as_mut().GetCaret()
    }
    // BLOCKED: fn GetCursor()
    fn HasCapture(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasCapture()
    }
    fn ReleaseMouse(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().ReleaseMouse()
    }
    fn SetCaret(&self, caret: *mut ffi::wxCaret) {
        unsafe { self.pinned::<ffi::wxWindow>().as_mut().SetCaret(caret) }
    }
    fn SetCursor(&self, cursor: &ffi::wxCursor) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SetCursor(cursor)
    }
    fn WarpPointer(&self, x: i32, y: i32) {
        self.pinned::<ffi::wxWindow>().as_mut().WarpPointer(x, y)
    }
    fn EnableTouchEvents(&self, eventsMask: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().EnableTouchEvents(eventsMask)
    }
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn HitTest()
    // CXX_UNSUPPORTED: fn GetBorder()
    // CXX_UNSUPPORTED: fn GetBorder()
    fn DoUpdateWindowUI(&self, event: Pin<&mut ffi::wxUpdateUIEvent>) {
        self.pinned::<ffi::wxWindow>().as_mut().DoUpdateWindowUI(event)
    }
    // CXX_UNSUPPORTED: fn GetHandle()
    fn HasMultiplePages(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().HasMultiplePages()
    }
    fn InheritAttributes(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().InheritAttributes()
    }
    fn InitDialog(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().InitDialog()
    }
    fn IsDoubleBuffered(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsDoubleBuffered()
    }
    fn SetDoubleBuffered(&self, on: bool) {
        self.pinned::<ffi::wxWindow>().as_mut().SetDoubleBuffered(on)
    }
    fn IsRetained(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsRetained()
    }
    fn IsThisEnabled(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsThisEnabled()
    }
    fn IsTopLevel(&self) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().IsTopLevel()
    }
    fn OnInternalIdle(&self) {
        self.pinned::<ffi::wxWindow>().as_mut().OnInternalIdle()
    }
    fn SendIdleEvents(&self, event: Pin<&mut ffi::wxIdleEvent>) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().SendIdleEvents(event)
    }
    fn RegisterHotKey(&self, hotkeyId: i32, modifiers: i32, virtualKeyCode: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().RegisterHotKey(hotkeyId, modifiers, virtualKeyCode)
    }
    fn UnregisterHotKey(&self, hotkeyId: i32) -> bool {
        self.pinned::<ffi::wxWindow>().as_mut().UnregisterHotKey(hotkeyId)
    }
    // BLOCKED: fn UpdateWindowUI()
    // CXX_UNSUPPORTED: fn GetClassDefaultAttributes()
    // BLOCKED: fn FindFocus()
    // BLOCKED: fn FindWindowById()
    // BLOCKED: fn FindWindowByLabel()
    // BLOCKED: fn FindWindowByName()
    // BLOCKED: fn GetCapture()
    // BLOCKED: fn NewControlId()
    // BLOCKED: fn UnreserveControlId()
    // DTOR: fn ~wxWindow()
    // BLOCKED: fn Create()
}

// wxControl
wx_class! { Control(wxControl) impl
    ControlMethods
}
impl Control {
    pub fn new(parent: *mut ffi::wxWindow, id: i32, pos: &ffi::wxPoint, size: &ffi::wxSize, style: i32, validator: &ffi::wxValidator, name: &ffi::wxString) -> Control {
        unsafe { Control(ffi::NewControl(parent, id, pos, size, style, validator, name)) }
    }
    pub fn new1() -> Control {
        Control(ffi::NewControl1())
    }
}
trait ControlMethods: WxRustMethods {
    // BLOCKED: fn Create()
    fn Command(&self, event: Pin<&mut ffi::wxCommandEvent>) {
        self.pinned::<ffi::wxControl>().as_mut().Command(event)
    }
    // CXX_UNSUPPORTED: fn GetLabel()
    // CXX_UNSUPPORTED: fn GetLabelText()
    // CXX_UNSUPPORTED: fn GetSizeFromTextSize()
    // CXX_UNSUPPORTED: fn GetSizeFromTextSize()
    // CXX_UNSUPPORTED: fn GetSizeFromText()
    fn SetLabel(&self, label: &ffi::wxString) {
        self.pinned::<ffi::wxControl>().as_mut().SetLabel(label)
    }
    fn SetLabelText(&self, text: &ffi::wxString) {
        self.pinned::<ffi::wxControl>().as_mut().SetLabelText(text)
    }
    fn SetLabelMarkup(&self, markup: &ffi::wxString) -> bool {
        self.pinned::<ffi::wxControl>().as_mut().SetLabelMarkup(markup)
    }
    // CXX_UNSUPPORTED: fn GetLabelText()
    // CXX_UNSUPPORTED: fn RemoveMnemonics()
    // CXX_UNSUPPORTED: fn EscapeMnemonics()
    // CXX_UNSUPPORTED: fn Ellipsize()
}

// wxAnyButton
wx_class! { AnyButton(wxAnyButton) impl
    AnyButtonMethods
}
impl AnyButton {
    pub fn new() -> AnyButton {
        AnyButton(ffi::NewAnyButton())
    }
}
trait AnyButtonMethods: WxRustMethods {
    // DTOR: fn ~wxAnyButton()
    // CXX_UNSUPPORTED: fn GetBitmap()
    // CXX_UNSUPPORTED: fn GetBitmapCurrent()
    // CXX_UNSUPPORTED: fn GetBitmapDisabled()
    // CXX_UNSUPPORTED: fn GetBitmapFocus()
    // CXX_UNSUPPORTED: fn GetBitmapLabel()
    // CXX_UNSUPPORTED: fn GetBitmapPressed()
    // CXX_UNSUPPORTED: fn SetBitmap()
    fn SetBitmapCurrent(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapCurrent(bitmap)
    }
    fn SetBitmapDisabled(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapDisabled(bitmap)
    }
    fn SetBitmapFocus(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapFocus(bitmap)
    }
    fn SetBitmapLabel(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapLabel(bitmap)
    }
    fn SetBitmapPressed(&self, bitmap: &ffi::wxBitmap) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapPressed(bitmap)
    }
    // CXX_UNSUPPORTED: fn GetBitmapMargins()
    fn SetBitmapMargins(&self, x: i32, y: i32) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapMargins(x, y)
    }
    fn SetBitmapMargins1(&self, sz: &ffi::wxSize) {
        self.pinned::<ffi::wxAnyButton>().as_mut().SetBitmapMargins1(sz)
    }
    // CXX_UNSUPPORTED: fn SetBitmapPosition()
}

// wxButton
wx_class! { Button(wxButton) impl
    ButtonMethods
}
impl Button {
    pub fn new() -> Button {
        Button(ffi::NewButton())
    }
    pub fn new1(parent: *mut ffi::wxWindow, id: i32, label: &ffi::wxString, pos: &ffi::wxPoint, size: &ffi::wxSize, style: i32, validator: &ffi::wxValidator, name: &ffi::wxString) -> Button {
        unsafe { Button(ffi::NewButton1(parent, id, label, pos, size, style, validator, name)) }
    }
}
trait ButtonMethods: WxRustMethods {
    // BLOCKED: fn Create()
    fn GetAuthNeeded(&self) -> bool {
        self.pinned::<ffi::wxButton>().as_mut().GetAuthNeeded()
    }
    // CXX_UNSUPPORTED: fn GetLabel()
    fn SetAuthNeeded(&self, needed: bool) {
        self.pinned::<ffi::wxButton>().as_mut().SetAuthNeeded(needed)
    }
    fn SetDefault(&self) -> *mut ffi::wxWindow {
        self.pinned::<ffi::wxButton>().as_mut().SetDefault()
    }
    fn SetLabel(&self, label: &ffi::wxString) {
        self.pinned::<ffi::wxButton>().as_mut().SetLabel(label)
    }
    // CXX_UNSUPPORTED: fn GetDefaultSize()
}

