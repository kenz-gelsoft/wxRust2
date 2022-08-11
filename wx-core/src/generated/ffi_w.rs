use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxWindow
    pub fn wxWindow_CLASSINFO() -> *mut c_void;
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
    pub fn wxWindow_GetChildren1(self_: *const c_void) -> *mut c_void;
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
    pub fn wxWindow_SetScrollbar(
        self_: *mut c_void,
        orientation: c_int,
        position: c_int,
        thumb_size: c_int,
        range: c_int,
        refresh: bool,
    );
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
    pub fn wxWindow_FromPhys(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxWindow_ToPhys(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys2(self_: *const c_void, d: c_int) -> c_int;
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
    pub fn wxWindow_InformFirstDirection(
        self_: *mut c_void,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool;
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
    pub fn wxWindow_SetSize(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        size_flags: c_int,
    );
    pub fn wxWindow_SetSize1(self_: *mut c_void, rect: *const c_void);
    pub fn wxWindow_SetSize2(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_SetSize3(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxWindow_SetSizeHints(
        self_: *mut c_void,
        min_size: *const c_void,
        max_size: *const c_void,
        inc_size: *const c_void,
    );
    pub fn wxWindow_SetSizeHints1(
        self_: *mut c_void,
        min_w: c_int,
        min_h: c_int,
        max_w: c_int,
        max_h: c_int,
        inc_w: c_int,
        inc_h: c_int,
    );
    pub fn wxWindow_SetVirtualSize(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxWindow_SetVirtualSize1(self_: *mut c_void, size: *const c_void);
    pub fn wxWindow_FromDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromDIP5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_ToDIP3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToDIP4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToDIP5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_FromPhys3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_FromPhys5(d: c_int, w: *const c_void) -> c_int;
    pub fn wxWindow_ToPhys3(sz: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys4(pt: *const c_void, w: *const c_void) -> *mut c_void;
    pub fn wxWindow_ToPhys5(d: c_int, w: *const c_void) -> c_int;
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
    pub fn wxWindow_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWindow_GetBackgroundStyle(self_: *const c_void) -> wxBackgroundStyle;
    pub fn wxWindow_GetCharHeight(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetCharWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxWindow_GetDefaultAttributes(self_: *const c_void) -> wxVisualAttributes;
    pub fn wxWindow_GetDPI(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetForegroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetTextExtent(
        self_: *const c_void,
        string: *const c_void,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: *const c_void,
    );
    pub fn wxWindow_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxWindow_GetUpdateRegion(self_: *const c_void) -> *const c_void;
    pub fn wxWindow_GetUpdateClientRect(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_HasTransparentBackground(self_: *mut c_void) -> bool;
    pub fn wxWindow_Refresh(self_: *mut c_void, erase_background: bool, rect: *const c_void);
    pub fn wxWindow_RefreshRect(self_: *mut c_void, rect: *const c_void, erase_background: bool);
    pub fn wxWindow_Update(self_: *mut c_void);
    pub fn wxWindow_SetBackgroundColour(self_: *mut c_void, colour: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxWindow_SetBackgroundStyle(self_: *mut c_void, style: wxBackgroundStyle) -> bool;
    pub fn wxWindow_IsTransparentBackgroundSupported(
        self_: *const c_void,
        reason: *mut c_void,
    ) -> bool;
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
    // NOT_SUPPORTED: pub fn wxWindow_HideWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: c_uint) -> bool;
    pub fn wxWindow_IsEnabled(self_: *const c_void) -> bool;
    pub fn wxWindow_IsExposed(self_: *const c_void, x: c_int, y: c_int) -> bool;
    pub fn wxWindow_IsExposed1(self_: *const c_void, pt: *mut c_void) -> bool;
    pub fn wxWindow_IsExposed2(
        self_: *const c_void,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
    ) -> bool;
    pub fn wxWindow_IsExposed3(self_: *const c_void, rect: *mut c_void) -> bool;
    pub fn wxWindow_IsShown(self_: *const c_void) -> bool;
    pub fn wxWindow_IsShownOnScreen(self_: *const c_void) -> bool;
    pub fn wxWindow_Disable(self_: *mut c_void) -> bool;
    pub fn wxWindow_Enable(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxWindow_Show(self_: *mut c_void, show: bool) -> bool;
    // NOT_SUPPORTED: pub fn wxWindow_ShowWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: c_uint) -> bool;
    pub fn wxWindow_GetHelpText(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetHelpText(self_: *mut c_void, help_text: *const c_void);
    // NOT_SUPPORTED: pub fn wxWindow_GetHelpTextAtPoint(self_: *const c_void, point: *const c_void, origin: wxHelpEvent::Origin) -> *mut c_void;
    pub fn wxWindow_GetToolTip(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetToolTipText(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_SetToolTip(self_: *mut c_void, tip_string: *const c_void);
    pub fn wxWindow_SetToolTip1(self_: *mut c_void, tip: *mut c_void);
    pub fn wxWindow_UnsetToolTip(self_: *mut c_void);
    pub fn wxWindow_GetPopupMenuSelectionFromUser(
        self_: *mut c_void,
        menu: *mut c_void,
        pos: *const c_void,
    ) -> c_int;
    pub fn wxWindow_GetPopupMenuSelectionFromUser1(
        self_: *mut c_void,
        menu: *mut c_void,
        x: c_int,
        y: c_int,
    ) -> c_int;
    pub fn wxWindow_PopupMenu(self_: *mut c_void, menu: *mut c_void, pos: *const c_void) -> bool;
    pub fn wxWindow_PopupMenu1(self_: *mut c_void, menu: *mut c_void, x: c_int, y: c_int) -> bool;
    pub fn wxWindow_GetValidator(self_: *mut c_void) -> *mut c_void;
    pub fn wxWindow_SetValidator(self_: *mut c_void, validator: *const c_void);
    pub fn wxWindow_TransferDataFromWindow(self_: *mut c_void) -> bool;
    pub fn wxWindow_TransferDataToWindow(self_: *mut c_void) -> bool;
    pub fn wxWindow_Validate(self_: *mut c_void) -> bool;
    pub fn wxWindow_GetId(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxWindow_GetLayoutDirection(self_: *const c_void) -> c_int;
    pub fn wxWindow_AdjustForLayoutDirection(
        self_: *const c_void,
        x: c_int,
        width: c_int,
        width_total: c_int,
    ) -> c_int;
    pub fn wxWindow_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWindow_GetWindowVariant(self_: *const c_void) -> wxWindowVariant;
    pub fn wxWindow_SetId(self_: *mut c_void, winid: c_int);
    pub fn wxWindow_SetLabel(self_: *mut c_void, label: *const c_void);
    pub fn wxWindow_SetLayoutDirection(self_: *mut c_void, dir: c_int);
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
    pub fn wxWindow_RegisterHotKey(
        self_: *mut c_void,
        hotkey_id: c_int,
        modifiers: c_int,
        virtual_key_code: c_int,
    ) -> bool;
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
    pub fn wxWindow_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxWindow_~wxWindow(self_: *mut c_void);
    pub fn wxWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxWrapSizer
    pub fn wxWrapSizer_CLASSINFO() -> *mut c_void;
    pub fn wxWrapSizer_new(orient: c_int, flags: c_int) -> *mut c_void;

}
