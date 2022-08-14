use super::*;

extern "C" {

    // wxMDIChildFrame
    pub fn wxMDIChildFrame_CLASSINFO() -> *mut c_void;
    pub fn wxMDIChildFrame_new() -> *mut c_void;
    pub fn wxMDIChildFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMDIChildFrame_~wxMDIChildFrame(self_: *mut c_void);
    pub fn wxMDIChildFrame_Activate(self_: *mut c_void);
    pub fn wxMDIChildFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxMDIChildFrame_GetMDIParent(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIChildFrame_Restore(self_: *mut c_void);

    // wxMDIParentFrame
    pub fn wxMDIParentFrame_CLASSINFO() -> *mut c_void;
    pub fn wxMDIParentFrame_new() -> *mut c_void;
    pub fn wxMDIParentFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxMDIParentFrame_~wxMDIParentFrame(self_: *mut c_void);
    pub fn wxMDIParentFrame_ActivateNext(self_: *mut c_void);
    pub fn wxMDIParentFrame_ActivatePrevious(self_: *mut c_void);
    pub fn wxMDIParentFrame_ArrangeIcons(self_: *mut c_void);
    pub fn wxMDIParentFrame_Cascade(self_: *mut c_void);
    pub fn wxMDIParentFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxMDIParentFrame_GetActiveChild(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_GetClientWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_GetWindowMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_OnCreateClient(self_: *mut c_void) -> *mut c_void;
    pub fn wxMDIParentFrame_SetWindowMenu(self_: *mut c_void, menu: *mut c_void);
    // NOT_SUPPORTED: pub fn wxMDIParentFrame_Tile(self_: *mut c_void, orient: wxOrientation);
    pub fn wxMDIParentFrame_IsTDI() -> bool;

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

}
