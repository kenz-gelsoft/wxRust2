use super::*;

extern "C" {

    // wxMask
    pub fn wxMask_CLASSINFO() -> *mut c_void;
    pub fn wxMask_new() -> *mut c_void;
    pub fn wxMask_new1(bitmap: *const c_void, index: c_int) -> *mut c_void;
    pub fn wxMask_new2(bitmap: *const c_void) -> *mut c_void;
    pub fn wxMask_new3(bitmap: *const c_void, colour: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxMask_~wxMask(self_: *mut c_void);
    pub fn wxMask_Create(self_: *mut c_void, bitmap: *const c_void, index: c_int) -> bool;
    pub fn wxMask_Create1(self_: *mut c_void, bitmap: *const c_void) -> bool;
    pub fn wxMask_Create2(self_: *mut c_void, bitmap: *const c_void, colour: *const c_void)
        -> bool;
    pub fn wxMask_GetBitmap(self_: *const c_void) -> *mut c_void;

    // wxMaximizeEvent
    pub fn wxMaximizeEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMaximizeEvent_new(id: c_int) -> *mut c_void;

    // wxMemoryDC
    pub fn wxMemoryDC_CLASSINFO() -> *mut c_void;
    pub fn wxMemoryDC_new() -> *mut c_void;
    pub fn wxMemoryDC_new1(dc: *mut c_void) -> *mut c_void;
    pub fn wxMemoryDC_new2(bitmap: *mut c_void) -> *mut c_void;
    pub fn wxMemoryDC_SelectObject(self_: *mut c_void, bitmap: *mut c_void);
    pub fn wxMemoryDC_SelectObjectAsSource(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxMemoryDC_GetSelectedBitmap(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMemoryDC_GetSelectedBitmap1(self_: *mut c_void) -> *mut c_void;

    // wxMenu
    pub fn wxMenu_CLASSINFO() -> *mut c_void;
    pub fn wxMenu_new() -> *mut c_void;
    pub fn wxMenu_new1(style: c_long) -> *mut c_void;
    pub fn wxMenu_new2(title: *const c_void, style: c_long) -> *mut c_void;
    // DTOR: pub fn wxMenu_~wxMenu(self_: *mut c_void);
    pub fn wxMenu_Append(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxMenu_Append1(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        sub_menu: *mut c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_Append2(self_: *mut c_void, menu_item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_AppendCheckItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_AppendRadioItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_AppendSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxMenu_AppendSubMenu(
        self_: *mut c_void,
        submenu: *mut c_void,
        text: *const c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_Break(self_: *mut c_void);
    pub fn wxMenu_Check(self_: *mut c_void, id: c_int, check: bool);
    pub fn wxMenu_Delete(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxMenu_Delete1(self_: *mut c_void, item: *mut c_void) -> bool;
    pub fn wxMenu_Destroy(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxMenu_Destroy1(self_: *mut c_void, item: *mut c_void) -> bool;
    pub fn wxMenu_Enable(self_: *mut c_void, id: c_int, enable: bool);
    pub fn wxMenu_FindChildItem(self_: *const c_void, id: c_int, pos: *mut c_void) -> *mut c_void;
    pub fn wxMenu_FindItem(self_: *const c_void, item_string: *const c_void) -> c_int;
    pub fn wxMenu_FindItem1(self_: *const c_void, id: c_int, menu: *mut c_void) -> *mut c_void;
    pub fn wxMenu_FindItemByPosition(self_: *const c_void, position: usize) -> *mut c_void;
    pub fn wxMenu_GetHelpString(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_GetLabel(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_GetLabelText(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_GetMenuItemCount(self_: *const c_void) -> usize;
    // BLOCKED: pub fn wxMenu_GetMenuItems(self_: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenu_GetMenuItems1(self_: *const c_void) -> *const c_void;
    pub fn wxMenu_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_Insert(self_: *mut c_void, pos: usize, menu_item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_Insert1(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxMenu_Insert2(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        text: *const c_void,
        submenu: *mut c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_InsertCheckItem(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_InsertRadioItem(
        self_: *mut c_void,
        pos: usize,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_InsertSeparator(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxMenu_IsChecked(self_: *const c_void, id: c_int) -> bool;
    pub fn wxMenu_IsEnabled(self_: *const c_void, id: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxMenu_MSWCommand(self_: *mut c_void, param: WXUINT, id: WXWORD) -> bool;
    pub fn wxMenu_Prepend(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_Prepend1(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxMenu_Prepend2(
        self_: *mut c_void,
        id: c_int,
        text: *const c_void,
        submenu: *mut c_void,
        help: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_PrependCheckItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_PrependRadioItem(
        self_: *mut c_void,
        id: c_int,
        item: *const c_void,
        help_string: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenu_PrependSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxMenu_Remove(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxMenu_Remove1(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxMenu_SetHelpString(self_: *mut c_void, id: c_int, help_string: *const c_void);
    pub fn wxMenu_SetLabel(self_: *mut c_void, id: c_int, label: *const c_void);
    pub fn wxMenu_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxMenu_UpdateUI(self_: *mut c_void, source: *mut c_void);
    pub fn wxMenu_SetInvokingWindow(self_: *mut c_void, win: *mut c_void);
    pub fn wxMenu_GetInvokingWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_GetStyle(self_: *const c_void) -> c_long;
    pub fn wxMenu_SetParent(self_: *mut c_void, parent: *mut c_void);
    pub fn wxMenu_GetParent(self_: *const c_void) -> *mut c_void;
    pub fn wxMenu_Attach(self_: *mut c_void, menubar: *mut c_void);
    pub fn wxMenu_Detach(self_: *mut c_void);
    pub fn wxMenu_IsAttached(self_: *const c_void) -> bool;

    // wxMenuBar
    pub fn wxMenuBar_CLASSINFO() -> *mut c_void;
    pub fn wxMenuBar_new(style: c_long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMenuBar_new1(n: usize, menus: *mut c_void, titles: wxString, style: c_long) -> *mut c_void;
    // DTOR: pub fn wxMenuBar_~wxMenuBar(self_: *mut c_void);
    pub fn wxMenuBar_Append(self_: *mut c_void, menu: *mut c_void, title: *const c_void) -> bool;
    pub fn wxMenuBar_Check(self_: *mut c_void, id: c_int, check: bool);
    pub fn wxMenuBar_Enable(self_: *mut c_void, id: c_int, enable: bool);
    pub fn wxMenuBar_IsEnabledTop(self_: *const c_void, pos: usize) -> bool;
    pub fn wxMenuBar_EnableTop(self_: *mut c_void, pos: usize, enable: bool);
    pub fn wxMenuBar_FindItem(self_: *const c_void, id: c_int, menu: *mut c_void) -> *mut c_void;
    pub fn wxMenuBar_FindMenu(self_: *const c_void, title: *const c_void) -> c_int;
    pub fn wxMenuBar_FindMenuItem(
        self_: *const c_void,
        menu_string: *const c_void,
        item_string: *const c_void,
    ) -> c_int;
    pub fn wxMenuBar_GetHelpString(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMenuBar_GetLabel(self_: *const c_void, id: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxMenuBar_GetLabelTop(self_: *const c_void, pos: usize) -> wxString;
    pub fn wxMenuBar_GetMenu(self_: *const c_void, menu_index: usize) -> *mut c_void;
    pub fn wxMenuBar_GetMenuCount(self_: *const c_void) -> usize;
    pub fn wxMenuBar_GetMenuLabel(self_: *const c_void, pos: usize) -> *mut c_void;
    pub fn wxMenuBar_GetMenuLabelText(self_: *const c_void, pos: usize) -> *mut c_void;
    pub fn wxMenuBar_Insert(
        self_: *mut c_void,
        pos: usize,
        menu: *mut c_void,
        title: *const c_void,
    ) -> bool;
    pub fn wxMenuBar_IsChecked(self_: *const c_void, id: c_int) -> bool;
    pub fn wxMenuBar_IsEnabled(self_: *const c_void, id: c_int) -> bool;
    pub fn wxMenuBar_Remove(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxMenuBar_Replace(
        self_: *mut c_void,
        pos: usize,
        menu: *mut c_void,
        title: *const c_void,
    ) -> *mut c_void;
    pub fn wxMenuBar_SetHelpString(self_: *mut c_void, id: c_int, help_string: *const c_void);
    pub fn wxMenuBar_SetLabel(self_: *mut c_void, id: c_int, label: *const c_void);
    // BLOCKED: pub fn wxMenuBar_SetLabelTop(self_: *mut c_void, pos: usize, label: *const c_void);
    pub fn wxMenuBar_SetMenuLabel(self_: *mut c_void, pos: usize, label: *const c_void);
    pub fn wxMenuBar_OSXGetAppleMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuBar_GetFrame(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuBar_IsAttached(self_: *const c_void) -> bool;
    pub fn wxMenuBar_Attach(self_: *mut c_void, frame: *mut c_void);
    pub fn wxMenuBar_Detach(self_: *mut c_void);
    pub fn wxMenuBar_MacSetCommonMenuBar(menubar: *mut c_void);
    pub fn wxMenuBar_MacGetCommonMenuBar() -> *mut c_void;

    // wxMenuEvent
    pub fn wxMenuEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMenuEvent_new(type_: wxEventType, id: c_int, menu: *mut c_void) -> *mut c_void;
    pub fn wxMenuEvent_GetMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuEvent_GetMenuId(self_: *const c_void) -> c_int;
    pub fn wxMenuEvent_IsPopup(self_: *const c_void) -> bool;

    // wxMenuItem
    pub fn wxMenuItem_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetBitmap1(self_: *const c_void, checked: bool) -> *mut c_void;
    pub fn wxMenuItem_GetBitmapBundle(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetDisabledBitmap(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetHelp(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetId(self_: *const c_void) -> c_int;
    pub fn wxMenuItem_GetItemLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetItemLabelText(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetKind(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxMenuItem_GetLabel(self_: *const c_void) -> wxString;
    pub fn wxMenuItem_GetMarginWidth(self_: *const c_void) -> c_int;
    pub fn wxMenuItem_GetMenu(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetName(self_: *const c_void) -> wxString;
    pub fn wxMenuItem_GetSubMenu(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetText(self_: *const c_void) -> *const c_void;
    // BLOCKED: pub fn wxMenuItem_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_GetAccel(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxMenuItem_GetAccelFromString(label: *const c_void) -> *mut c_void;
    pub fn wxMenuItem_IsCheck(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsCheckable(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsChecked(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsEnabled(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsRadio(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsSeparator(self_: *const c_void) -> bool;
    pub fn wxMenuItem_IsSubMenu(self_: *const c_void) -> bool;
    pub fn wxMenuItem_SetBackgroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxMenuItem_SetBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxMenuItem_SetBitmap1(self_: *mut c_void, bmp: *const c_void, checked: bool);
    pub fn wxMenuItem_SetBitmaps(
        self_: *mut c_void,
        checked: *const c_void,
        unchecked: *const c_void,
    );
    pub fn wxMenuItem_SetDisabledBitmap(self_: *mut c_void, disabled: *const c_void);
    pub fn wxMenuItem_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxMenuItem_SetHelp(self_: *mut c_void, help_string: *const c_void);
    pub fn wxMenuItem_SetItemLabel(self_: *mut c_void, label: *const c_void);
    pub fn wxMenuItem_SetMarginWidth(self_: *mut c_void, width: c_int);
    pub fn wxMenuItem_SetMenu(self_: *mut c_void, menu: *mut c_void);
    pub fn wxMenuItem_SetSubMenu(self_: *mut c_void, menu: *mut c_void);
    // BLOCKED: pub fn wxMenuItem_SetText(self_: *mut c_void, text: *const c_void);
    pub fn wxMenuItem_SetTextColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxMenuItem_SetAccel(self_: *mut c_void, accel: *mut c_void);
    pub fn wxMenuItem_AddExtraAccel(self_: *mut c_void, accel: *const c_void);
    pub fn wxMenuItem_ClearExtraAccels(self_: *mut c_void);
    pub fn wxMenuItem_new(
        parent_menu: *mut c_void,
        id: c_int,
        text: *const c_void,
        help_string: *const c_void,
        kind: c_int,
        sub_menu: *mut c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMenuItem_~wxMenuItem(self_: *mut c_void);
    pub fn wxMenuItem_Check(self_: *mut c_void, check: bool);
    pub fn wxMenuItem_Enable(self_: *mut c_void, enable: bool);
    // BLOCKED: pub fn wxMenuItem_GetLabelFromText(text: *const c_void) -> wxString;
    pub fn wxMenuItem_GetLabelText(text: *const c_void) -> *mut c_void;

    // wxMessageDialog
    pub fn wxMessageDialog_CLASSINFO() -> *mut c_void;
    pub fn wxMessageDialog_new(
        parent: *mut c_void,
        message: *const c_void,
        caption: *const c_void,
        style: c_long,
        pos: *const c_void,
    ) -> *mut c_void;
    pub fn wxMessageDialog_SetExtendedMessage(self_: *mut c_void, extended_message: *const c_void);
    pub fn wxMessageDialog_SetHelpLabel(self_: *mut c_void, help: *const c_void) -> bool;
    pub fn wxMessageDialog_SetMessage(self_: *mut c_void, message: *const c_void);
    pub fn wxMessageDialog_SetOKCancelLabels(
        self_: *mut c_void,
        ok: *const c_void,
        cancel: *const c_void,
    ) -> bool;
    pub fn wxMessageDialog_SetOKLabel(self_: *mut c_void, ok: *const c_void) -> bool;
    pub fn wxMessageDialog_SetYesNoCancelLabels(
        self_: *mut c_void,
        yes: *const c_void,
        no: *const c_void,
        cancel: *const c_void,
    ) -> bool;
    pub fn wxMessageDialog_SetYesNoLabels(
        self_: *mut c_void,
        yes: *const c_void,
        no: *const c_void,
    ) -> bool;
    pub fn wxMessageDialog_GetCaption(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetMessage(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetExtendedMessage(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetMessageDialogStyle(self_: *const c_void) -> c_long;
    pub fn wxMessageDialog_HasCustomLabels(self_: *const c_void) -> bool;
    pub fn wxMessageDialog_GetYesLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetNoLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetOKLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetCancelLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetHelpLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxMessageDialog_GetEffectiveIcon(self_: *const c_void) -> c_long;

    // wxMessageOutputMessageBox
    pub fn wxMessageOutputMessageBox_delete(self_: *mut c_void);
    pub fn wxMessageOutputMessageBox_new() -> *mut c_void;

    // wxMiniFrame
    pub fn wxMiniFrame_CLASSINFO() -> *mut c_void;
    pub fn wxMiniFrame_new() -> *mut c_void;
    pub fn wxMiniFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMiniFrame_~wxMiniFrame(self_: *mut c_void);
    pub fn wxMiniFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxMirrorDC
    pub fn wxMirrorDC_CLASSINFO() -> *mut c_void;
    pub fn wxMirrorDC_new(dc: *mut c_void, mirror: bool) -> *mut c_void;

    // wxMouseCaptureChangedEvent
    pub fn wxMouseCaptureChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMouseCaptureChangedEvent_new(
        window_id: c_int,
        gained_capture: *mut c_void,
    ) -> *mut c_void;
    pub fn wxMouseCaptureChangedEvent_GetCapturedWindow(self_: *const c_void) -> *mut c_void;

    // wxMouseCaptureLostEvent
    pub fn wxMouseCaptureLostEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMouseCaptureLostEvent_new(window_id: c_int) -> *mut c_void;

    // wxMouseEvent
    pub fn wxMouseEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMouseEvent_new(mouse_event_type: wxEventType) -> *mut c_void;
    pub fn wxMouseEvent_Aux1DClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux1Down(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux1Up(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux2DClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux2Down(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Aux2Up(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_Button(self_: *const c_void, but: wxMouseButton) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_ButtonDClick(self_: *const c_void, but: wxMouseButton) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_ButtonDown(self_: *const c_void, but: wxMouseButton) -> bool;
    // NOT_SUPPORTED: pub fn wxMouseEvent_ButtonUp(self_: *const c_void, but: wxMouseButton) -> bool;
    pub fn wxMouseEvent_Dragging(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Entering(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_GetButton(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetClickCount(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetLinesPerAction(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetColumnsPerAction(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_GetLogicalPosition(self_: *const c_void, dc: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMouseEvent_GetMagnification(self_: *const c_void) -> float;
    pub fn wxMouseEvent_GetWheelDelta(self_: *const c_void) -> c_int;
    pub fn wxMouseEvent_IsWheelInverted(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_GetWheelRotation(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxMouseEvent_GetWheelAxis(self_: *const c_void) -> wxMouseWheelAxis;
    pub fn wxMouseEvent_IsButton(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_IsPageScroll(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Leaving(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_LeftDClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_LeftDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_LeftUp(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Magnify(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MiddleDClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MiddleDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_MiddleUp(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_Moving(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_RightDClick(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_RightDown(self_: *const c_void) -> bool;
    pub fn wxMouseEvent_RightUp(self_: *const c_void) -> bool;

    // wxMouseEventsManager
    pub fn wxMouseEventsManager_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxMouseEventsManager_new() -> *mut c_void;
    // BLOCKED: pub fn wxMouseEventsManager_new1(win: *mut c_void) -> *mut c_void;
    pub fn wxMouseEventsManager_Create(self_: *mut c_void, win: *mut c_void) -> bool;

    // wxMoveEvent
    pub fn wxMoveEvent_CLASSINFO() -> *mut c_void;
    pub fn wxMoveEvent_new(pt: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxMoveEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxMoveEvent_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxMoveEvent_SetRect(self_: *mut c_void, rect: *const c_void);
    pub fn wxMoveEvent_SetPosition(self_: *mut c_void, pos: *const c_void);

}
