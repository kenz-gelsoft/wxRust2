use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxAnyButton
    pub fn wxAnyButton_new() -> *mut c_void;
    // DTOR: pub fn wxAnyButton_~wxAnyButton(self_: *mut c_void);
    pub fn wxAnyButton_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapCurrent(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapDisabled(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapFocus(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxAnyButton_GetBitmapPressed(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxAnyButton_SetBitmap(self_: *mut c_void, bitmap: *const c_void, dir: wxDirection);
    pub fn wxAnyButton_SetBitmapCurrent(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapDisabled(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapFocus(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapLabel(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_SetBitmapPressed(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxAnyButton_GetBitmapMargins(self_: *mut c_void) -> *mut c_void;
    pub fn wxAnyButton_SetBitmapMargins(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxAnyButton_SetBitmapMargins1(self_: *mut c_void, sz: *const c_void);
    // NOT_SUPPORTED: pub fn wxAnyButton_SetBitmapPosition(self_: *mut c_void, dir: wxDirection);

    // wxArtProvider
    // DTOR: pub fn wxArtProvider_~wxArtProvider(self_: *mut c_void);
    pub fn wxArtProvider_Delete(provider: *mut c_void) -> bool;
    pub fn wxArtProvider_GetBitmap(
        id: *const c_void,
        client: *const c_void,
        size: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxArtProvider_GetIcon(id: *const c_void, client: *const c_void, size: *const c_void) -> wxIcon;
    pub fn wxArtProvider_GetNativeSizeHint(client: *const c_void) -> *mut c_void;
    pub fn wxArtProvider_GetSizeHint(client: *const c_void, platform_default: bool) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxArtProvider_GetIconBundle(id: *const c_void, client: *const c_void) -> wxIconBundle;
    pub fn wxArtProvider_HasNativeProvider() -> bool;
    // BLOCKED: pub fn wxArtProvider_Insert(provider: *mut c_void);
    pub fn wxArtProvider_Pop() -> bool;
    pub fn wxArtProvider_Push(provider: *mut c_void);
    pub fn wxArtProvider_PushBack(provider: *mut c_void);
    pub fn wxArtProvider_Remove(provider: *mut c_void) -> bool;
    pub fn wxArtProvider_GetMessageBoxIconId(flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxArtProvider_GetMessageBoxIcon(flags: c_int) -> wxIcon;

    // wxBitmap
    pub fn wxBitmap_new() -> *mut c_void;
    pub fn wxBitmap_new1(bitmap: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_new2(bits: char, width: c_int, height: c_int, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new3(width: c_int, height: c_int, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new4(sz: *const c_void, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new5(bits: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_new6(name: *const c_void, type_: wxBitmapType) -> *mut c_void;
    pub fn wxBitmap_new7(img: *const c_void, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new8(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBitmap_~wxBitmap(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmap_ConvertToImage(self_: *const c_void) -> wxImage;
    pub fn wxBitmap_CopyFromIcon(self_: *mut c_void, icon: *const c_void) -> bool;
    pub fn wxBitmap_Create(self_: *mut c_void, width: c_int, height: c_int, depth: c_int) -> bool;
    pub fn wxBitmap_Create1(self_: *mut c_void, sz: *const c_void, depth: c_int) -> bool;
    pub fn wxBitmap_Create2(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        dc: *const c_void,
    ) -> bool;
    pub fn wxBitmap_CreateScaled(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        depth: c_int,
        logical_scale: c_double,
    ) -> bool;
    pub fn wxBitmap_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxBitmap_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxBitmap_GetMask(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetPalette(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetSubBitmap(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    pub fn wxBitmap_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxBitmap_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxBitmap_LoadFile(self_: *mut c_void, name: *const c_void, type_: wxBitmapType) -> bool;
    // NOT_SUPPORTED: pub fn wxBitmap_SaveFile(self_: *const c_void, name: *const c_void, type_: wxBitmapType, palette: *const c_void) -> bool;
    pub fn wxBitmap_SetDepth(self_: *mut c_void, depth: c_int);
    pub fn wxBitmap_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxBitmap_SetMask(self_: *mut c_void, mask: *mut c_void);
    pub fn wxBitmap_SetPalette(self_: *mut c_void, palette: *const c_void);
    pub fn wxBitmap_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxBitmap_AddHandler(handler: *mut c_void);
    pub fn wxBitmap_CleanUpHandlers();
    pub fn wxBitmap_FindHandler(name: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_FindHandler1(extension: *const c_void, bitmap_type: wxBitmapType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_FindHandler2(bitmap_type: wxBitmapType) -> *mut c_void;
    // BLOCKED: pub fn wxBitmap_GetHandlers() -> *mut c_void;
    pub fn wxBitmap_InitStandardHandlers();
    pub fn wxBitmap_InsertHandler(handler: *mut c_void);
    pub fn wxBitmap_NewFromPNGData(data: *const c_void, size: usize) -> *mut c_void;
    pub fn wxBitmap_RemoveHandler(name: *const c_void) -> bool;

    // wxBookCtrlBase
    pub fn wxBookCtrlBase_GetPageImage(self_: *const c_void, n_page: usize) -> c_int;
    pub fn wxBookCtrlBase_SetPageImage(self_: *mut c_void, page: usize, image: c_int) -> bool;
    pub fn wxBookCtrlBase_GetPageText(self_: *const c_void, n_page: usize) -> *mut c_void;
    pub fn wxBookCtrlBase_SetPageText(self_: *mut c_void, page: usize, text: *const c_void)
        -> bool;
    pub fn wxBookCtrlBase_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxBookCtrlBase_GetCurrentPage(self_: *const c_void) -> *mut c_void;
    pub fn wxBookCtrlBase_SetSelection(self_: *mut c_void, page: usize) -> c_int;
    pub fn wxBookCtrlBase_AdvanceSelection(self_: *mut c_void, forward: bool);
    pub fn wxBookCtrlBase_ChangeSelection(self_: *mut c_void, page: usize) -> c_int;
    pub fn wxBookCtrlBase_FindPage(self_: *const c_void, page: *const c_void) -> c_int;
    pub fn wxBookCtrlBase_SetPageSize(self_: *mut c_void, size: *const c_void);
    pub fn wxBookCtrlBase_HitTest(
        self_: *const c_void,
        pt: *const c_void,
        flags: *mut c_void,
    ) -> c_int;
    pub fn wxBookCtrlBase_AddPage(
        self_: *mut c_void,
        page: *mut c_void,
        text: *const c_void,
        select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxBookCtrlBase_DeleteAllPages(self_: *mut c_void) -> bool;
    pub fn wxBookCtrlBase_DeletePage(self_: *mut c_void, page: usize) -> bool;
    pub fn wxBookCtrlBase_InsertPage(
        self_: *mut c_void,
        index: usize,
        page: *mut c_void,
        text: *const c_void,
        select: bool,
        image_id: c_int,
    ) -> bool;
    pub fn wxBookCtrlBase_RemovePage(self_: *mut c_void, page: usize) -> bool;
    pub fn wxBookCtrlBase_GetPageCount(self_: *const c_void) -> usize;
    pub fn wxBookCtrlBase_GetPage(self_: *const c_void, page: usize) -> *mut c_void;
    // BLOCKED: pub fn wxBookCtrlBase_new() -> *mut c_void;
    // BLOCKED: pub fn wxBookCtrlBase_new1(parent: *mut c_void, winid: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxBookCtrlBase_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxBoxSizer
    pub fn wxBoxSizer_new(orient: c_int) -> *mut c_void;
    pub fn wxBoxSizer_GetOrientation(self_: *const c_void) -> c_int;
    pub fn wxBoxSizer_SetOrientation(self_: *mut c_void, orient: c_int);

    // wxButton
    pub fn wxButton_new() -> *mut c_void;
    pub fn wxButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxButton_GetAuthNeeded(self_: *const c_void) -> bool;
    pub fn wxButton_SetAuthNeeded(self_: *mut c_void, needed: bool);
    pub fn wxButton_SetDefault(self_: *mut c_void) -> *mut c_void;
    pub fn wxButton_GetDefaultSize(win: *mut c_void) -> *mut c_void;

    // wxCheckBox
    pub fn wxCheckBox_new() -> *mut c_void;
    pub fn wxCheckBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxCheckBox_~wxCheckBox(self_: *mut c_void);
    pub fn wxCheckBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCheckBox_GetValue(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxCheckBox_Get3StateValue(self_: *const c_void) -> wxCheckBoxState;
    pub fn wxCheckBox_Is3State(self_: *const c_void) -> bool;
    pub fn wxCheckBox_Is3rdStateAllowedForUser(self_: *const c_void) -> bool;
    pub fn wxCheckBox_IsChecked(self_: *const c_void) -> bool;
    pub fn wxCheckBox_SetValue(self_: *mut c_void, state: bool);
    // NOT_SUPPORTED: pub fn wxCheckBox_Set3StateValue(self_: *mut c_void, state: wxCheckBoxState);

    // wxCommandEvent
    // NOT_SUPPORTED: pub fn wxCommandEvent_new(command_event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxCommandEvent_GetClientData(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_GetClientObject(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_GetExtraLong(self_: *const c_void) -> c_long;
    pub fn wxCommandEvent_GetInt(self_: *const c_void) -> c_int;
    pub fn wxCommandEvent_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxCommandEvent_GetString(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_IsChecked(self_: *const c_void) -> bool;
    pub fn wxCommandEvent_IsSelection(self_: *const c_void) -> bool;
    pub fn wxCommandEvent_SetClientData(self_: *mut c_void, client_data: *mut c_void);
    pub fn wxCommandEvent_SetClientObject(self_: *mut c_void, client_object: *mut c_void);
    pub fn wxCommandEvent_SetExtraLong(self_: *mut c_void, extra_long: c_long);
    pub fn wxCommandEvent_SetInt(self_: *mut c_void, int_command: c_int);
    pub fn wxCommandEvent_SetString(self_: *mut c_void, string: *const c_void);

    // wxControl
    pub fn wxControl_new(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxControl_new1() -> *mut c_void;
    pub fn wxControl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxControl_Command(self_: *mut c_void, event: *mut c_void);
    pub fn wxControl_GetLabelText(self_: *const c_void) -> *mut c_void;
    pub fn wxControl_GetSizeFromTextSize(
        self_: *const c_void,
        xlen: c_int,
        ylen: c_int,
    ) -> *mut c_void;
    pub fn wxControl_GetSizeFromTextSize1(
        self_: *const c_void,
        tsize: *const c_void,
    ) -> *mut c_void;
    pub fn wxControl_GetSizeFromText(self_: *const c_void, text: *const c_void) -> *mut c_void;
    pub fn wxControl_SetLabelText(self_: *mut c_void, text: *const c_void);
    pub fn wxControl_SetLabelMarkup(self_: *mut c_void, markup: *const c_void) -> bool;
    pub fn wxControl_GetLabelText1(label: *const c_void) -> *mut c_void;
    pub fn wxControl_RemoveMnemonics(str: *const c_void) -> *mut c_void;
    pub fn wxControl_EscapeMnemonics(text: *const c_void) -> *mut c_void;
    pub fn wxControl_Ellipsize(
        label: *const c_void,
        dc: *const c_void,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> *mut c_void;

    // wxFrame
    pub fn wxFrame_new() -> *mut c_void;
    pub fn wxFrame_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxFrame_~wxFrame(self_: *mut c_void);
    pub fn wxFrame_Centre(self_: *mut c_void, direction: c_int);
    pub fn wxFrame_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxFrame_CreateStatusBar(
        self_: *mut c_void,
        number: c_int,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_CreateToolBar(
        self_: *mut c_void,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_DoGiveHelp(self_: *mut c_void, text: *const c_void, show: bool);
    pub fn wxFrame_GetMenuBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_GetStatusBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_GetStatusBarPane(self_: *const c_void) -> c_int;
    pub fn wxFrame_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxFrame_OnCreateStatusBar(
        self_: *mut c_void,
        number: c_int,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_OnCreateToolBar(
        self_: *mut c_void,
        style: c_long,
        id: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxFrame_ProcessCommand(self_: *mut c_void, id: c_int) -> bool;
    pub fn wxFrame_SetMenuBar(self_: *mut c_void, menu_bar: *mut c_void);
    pub fn wxFrame_SetStatusBar(self_: *mut c_void, status_bar: *mut c_void);
    pub fn wxFrame_SetStatusBarPane(self_: *mut c_void, n: c_int);
    pub fn wxFrame_SetStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
    pub fn wxFrame_SetStatusWidths(self_: *mut c_void, n: c_int, widths_field: *const c_void);
    pub fn wxFrame_SetToolBar(self_: *mut c_void, tool_bar: *mut c_void);
    pub fn wxFrame_MSWGetTaskBarButton(self_: *mut c_void) -> *mut c_void;
    pub fn wxFrame_PushStatusText(self_: *mut c_void, text: *const c_void, number: c_int);
    pub fn wxFrame_PopStatusText(self_: *mut c_void, number: c_int);

    // wxGDIObject
    // BLOCKED: pub fn wxGDIObject_new() -> *mut c_void;

    // wxListBox
    pub fn wxListBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxListBox_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxListBox_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListBox_~wxListBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxListBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxListBox_Deselect(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetStringSelection(self_: *mut c_void, s: *const c_void, select: bool)
        -> bool;
    pub fn wxListBox_SetStringSelection1(self_: *mut c_void, s: *const c_void) -> bool;
    pub fn wxListBox_GetSelections(self_: *const c_void, selections: *mut c_void) -> c_int;
    pub fn wxListBox_HitTest(self_: *const c_void, point: *const c_void) -> c_int;
    pub fn wxListBox_HitTest1(self_: *const c_void, x: c_int, y: c_int) -> c_int;
    // NOT_SUPPORTED: pub fn wxListBox_InsertItems(self_: *mut c_void, n_items: unsigned int, items: *const c_void, pos: unsigned int);
    // NOT_SUPPORTED: pub fn wxListBox_InsertItems1(self_: *mut c_void, items: *const c_void, pos: unsigned int);
    pub fn wxListBox_IsSelected(self_: *const c_void, n: c_int) -> bool;
    pub fn wxListBox_SetFirstItem(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetFirstItem1(self_: *mut c_void, string: *const c_void);
    pub fn wxListBox_EnsureVisible(self_: *mut c_void, n: c_int);
    pub fn wxListBox_IsSorted(self_: *const c_void) -> bool;
    pub fn wxListBox_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxListBox_GetTopItem(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxListBox_MSWSetTabStops(self_: *mut c_void, tab_stops: *const c_void);

    // wxMenu
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

    // wxNonOwnedWindow
    pub fn wxNonOwnedWindow_SetShape(self_: *mut c_void, region: *const c_void) -> bool;
    pub fn wxNonOwnedWindow_SetShape1(self_: *mut c_void, path: *const c_void) -> bool;

    // wxNotebook
    pub fn wxNotebook_new() -> *mut c_void;
    pub fn wxNotebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxNotebook_~wxNotebook(self_: *mut c_void);
    // BLOCKED: pub fn wxNotebook_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;
    pub fn wxNotebook_GetRowCount(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxNotebook_GetThemeBackgroundColour(self_: *const c_void) -> wxColour;
    // BLOCKED: pub fn wxNotebook_OnSelChange(self_: *mut c_void, event: *mut c_void);
    pub fn wxNotebook_SetPadding(self_: *mut c_void, padding: *const c_void);

    // wxPanel
    pub fn wxPanel_new() -> *mut c_void;
    pub fn wxPanel_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPanel_~wxPanel(self_: *mut c_void);
    pub fn wxPanel_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxPanel_OnSysColourChanged(self_: *mut c_void, event: *mut c_void);
    pub fn wxPanel_SetFocusIgnoringChildren(self_: *mut c_void);

    // wxPoint
    pub fn wxPoint_delete(self_: *mut c_void);
    pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
    pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
    // BLOCKED: pub fn wxPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxPoint_new() -> *mut c_void;
    pub fn wxPoint_new1(x: c_int, y: c_int) -> *mut c_void;
    pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;

    // wxRect
    pub fn wxRect_delete(self_: *mut c_void);
    pub fn wxRect_new() -> *mut c_void;
    pub fn wxRect_new1(x: c_int, y: c_int, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxRect_new2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
    pub fn wxRect_new3(pos: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxRect_new4(size: *const c_void) -> *mut c_void;
    pub fn wxRect_CentreIn(self_: *const c_void, r: *const c_void, dir: c_int) -> *mut c_void;
    pub fn wxRect_CenterIn(self_: *const c_void, r: *const c_void, dir: c_int) -> *mut c_void;
    pub fn wxRect_Contains(self_: *const c_void, x: c_int, y: c_int) -> bool;
    pub fn wxRect_Contains1(self_: *const c_void, pt: *const c_void) -> bool;
    pub fn wxRect_Contains2(self_: *const c_void, rect: *const c_void) -> bool;
    // BLOCKED: pub fn wxRect_Deflate(self_: *mut c_void, dx: c_int, dy: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Deflate1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Deflate2(self_: *mut c_void, diff: c_int) -> *mut c_void;
    pub fn wxRect_Deflate3(self_: *const c_void, dx: c_int, dy: c_int) -> *mut c_void;
    pub fn wxRect_GetBottom(self_: *const c_void) -> c_int;
    pub fn wxRect_GetBottomLeft(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetBottomRight(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxRect_GetLeft(self_: *const c_void) -> c_int;
    pub fn wxRect_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetRight(self_: *const c_void) -> c_int;
    pub fn wxRect_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetTop(self_: *const c_void) -> c_int;
    pub fn wxRect_GetTopLeft(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetTopRight(self_: *const c_void) -> *mut c_void;
    pub fn wxRect_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxRect_GetX(self_: *const c_void) -> c_int;
    pub fn wxRect_GetY(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxRect_Inflate(self_: *mut c_void, dx: c_int, dy: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Inflate1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Inflate2(self_: *mut c_void, diff: c_int) -> *mut c_void;
    pub fn wxRect_Inflate3(self_: *const c_void, dx: c_int, dy: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Intersect(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxRect_Intersect1(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxRect_Intersects(self_: *const c_void, rect: *const c_void) -> bool;
    pub fn wxRect_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxRect_Offset(self_: *mut c_void, dx: c_int, dy: c_int);
    pub fn wxRect_Offset1(self_: *mut c_void, pt: *const c_void);
    pub fn wxRect_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxRect_SetPosition(self_: *mut c_void, pos: *const c_void);
    pub fn wxRect_SetSize(self_: *mut c_void, s: *const c_void);
    pub fn wxRect_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxRect_SetX(self_: *mut c_void, x: c_int);
    pub fn wxRect_SetY(self_: *mut c_void, y: c_int);
    pub fn wxRect_SetLeft(self_: *mut c_void, left: c_int);
    pub fn wxRect_SetRight(self_: *mut c_void, right: c_int);
    pub fn wxRect_SetTop(self_: *mut c_void, top: c_int);
    pub fn wxRect_SetBottom(self_: *mut c_void, bottom: c_int);
    pub fn wxRect_SetTopLeft(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_SetBottomRight(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_SetTopRight(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_SetBottomLeft(self_: *mut c_void, p: *const c_void);
    pub fn wxRect_Union(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_Union1(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator!=(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;
    // BLOCKED: pub fn wxRect_operator+(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
    // BLOCKED: pub fn wxRect_operator+=(self_: *mut c_void, r: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator*(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> wxRect;
    // BLOCKED: pub fn wxRect_operator*=(self_: *mut c_void, r: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator=(self_: *mut c_void, rect: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRect_operator==(self_: *mut c_void, r1: *const c_void, r2: *const c_void) -> bool;

    // wxSize
    pub fn wxSize_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxSize_operator=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator==(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
    // BLOCKED: pub fn wxSize_operator!=(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> bool;
    // BLOCKED: pub fn wxSize_operator+(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator-(self_: *mut c_void, s1: *const c_void, s2: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator+=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator-=(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxSize_new() -> *mut c_void;
    pub fn wxSize_new1(width: c_int, height: c_int) -> *mut c_void;
    pub fn wxSize_DecBy(self_: *mut c_void, pt: *const c_void);
    pub fn wxSize_DecBy1(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_DecBy2(self_: *mut c_void, dx: c_int, dy: c_int);
    pub fn wxSize_DecBy3(self_: *mut c_void, d: c_int);
    pub fn wxSize_DecTo(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_DecToIfSpecified(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxSize_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxSize_IncBy(self_: *mut c_void, pt: *const c_void);
    pub fn wxSize_IncBy1(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_IncBy2(self_: *mut c_void, dx: c_int, dy: c_int);
    pub fn wxSize_IncBy3(self_: *mut c_void, d: c_int);
    pub fn wxSize_IncTo(self_: *mut c_void, size: *const c_void);
    pub fn wxSize_IsFullySpecified(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxSize_Scale(self_: *mut c_void, xscale: c_double, yscale: c_double) -> *mut c_void;
    pub fn wxSize_Set(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxSize_SetDefaults(self_: *mut c_void, size_default: *const c_void);
    pub fn wxSize_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxSize_SetWidth(self_: *mut c_void, width: c_int);

    // wxSizer
    // BLOCKED: pub fn wxSizer_new() -> *mut c_void;
    // DTOR: pub fn wxSizer_~wxSizer(self_: *mut c_void);
    pub fn wxSizer_Add(
        self_: *mut c_void,
        window: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add1(
        self_: *mut c_void,
        window: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add2(
        self_: *mut c_void,
        sizer: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add3(
        self_: *mut c_void,
        sizer: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add4(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add5(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Add6(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxSizer_AddSpacer(self_: *mut c_void, size: c_int) -> *mut c_void;
    pub fn wxSizer_AddStretchSpacer(self_: *mut c_void, prop: c_int) -> *mut c_void;
    pub fn wxSizer_CalcMin(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizer_Clear(self_: *mut c_void, delete_windows: bool);
    pub fn wxSizer_ComputeFittingClientSize(self_: *mut c_void, window: *mut c_void)
        -> *mut c_void;
    pub fn wxSizer_ComputeFittingWindowSize(self_: *mut c_void, window: *mut c_void)
        -> *mut c_void;
    pub fn wxSizer_Detach(self_: *mut c_void, window: *mut c_void) -> bool;
    pub fn wxSizer_Detach1(self_: *mut c_void, sizer: *mut c_void) -> bool;
    pub fn wxSizer_Detach2(self_: *mut c_void, index: c_int) -> bool;
    pub fn wxSizer_Fit(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxSizer_FitInside(self_: *mut c_void, window: *mut c_void);
    pub fn wxSizer_InformFirstDirection(
        self_: *mut c_void,
        direction: c_int,
        size: c_int,
        available_other_dir: c_int,
    ) -> bool;
    // BLOCKED: pub fn wxSizer_GetChildren(self_: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSizer_GetChildren1(self_: *const c_void) -> *const c_void;
    pub fn wxSizer_GetContainingWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_SetContainingWindow(self_: *mut c_void, window: *mut c_void);
    pub fn wxSizer_GetItemCount(self_: *const c_void) -> usize;
    pub fn wxSizer_GetItem(self_: *mut c_void, window: *mut c_void, recursive: bool)
        -> *mut c_void;
    pub fn wxSizer_GetItem1(self_: *mut c_void, sizer: *mut c_void, recursive: bool)
        -> *mut c_void;
    pub fn wxSizer_GetItem2(self_: *mut c_void, index: usize) -> *mut c_void;
    pub fn wxSizer_GetItemById(self_: *mut c_void, id: c_int, recursive: bool) -> *mut c_void;
    pub fn wxSizer_GetMinSize(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizer_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxSizer_Hide(self_: *mut c_void, window: *mut c_void, recursive: bool) -> bool;
    pub fn wxSizer_Hide1(self_: *mut c_void, sizer: *mut c_void, recursive: bool) -> bool;
    pub fn wxSizer_Hide2(self_: *mut c_void, index: usize) -> bool;
    pub fn wxSizer_Insert(
        self_: *mut c_void,
        index: usize,
        window: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert1(
        self_: *mut c_void,
        index: usize,
        window: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert2(
        self_: *mut c_void,
        index: usize,
        sizer: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert3(
        self_: *mut c_void,
        index: usize,
        sizer: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert4(
        self_: *mut c_void,
        index: usize,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert5(
        self_: *mut c_void,
        index: usize,
        width: c_int,
        height: c_int,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Insert6(self_: *mut c_void, index: usize, item: *mut c_void) -> *mut c_void;
    pub fn wxSizer_InsertSpacer(self_: *mut c_void, index: usize, size: c_int) -> *mut c_void;
    pub fn wxSizer_InsertStretchSpacer(
        self_: *mut c_void,
        index: usize,
        prop: c_int,
    ) -> *mut c_void;
    pub fn wxSizer_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxSizer_IsShown(self_: *const c_void, window: *mut c_void) -> bool;
    pub fn wxSizer_IsShown1(self_: *const c_void, sizer: *mut c_void) -> bool;
    pub fn wxSizer_IsShown2(self_: *const c_void, index: usize) -> bool;
    pub fn wxSizer_Layout(self_: *mut c_void);
    pub fn wxSizer_Prepend(
        self_: *mut c_void,
        window: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend1(
        self_: *mut c_void,
        window: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend2(
        self_: *mut c_void,
        sizer: *mut c_void,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend3(
        self_: *mut c_void,
        sizer: *mut c_void,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend4(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        proportion: c_int,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend5(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        flags: *const c_void,
    ) -> *mut c_void;
    pub fn wxSizer_Prepend6(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxSizer_PrependSpacer(self_: *mut c_void, size: c_int) -> *mut c_void;
    pub fn wxSizer_PrependStretchSpacer(self_: *mut c_void, prop: c_int) -> *mut c_void;
    pub fn wxSizer_RepositionChildren(self_: *mut c_void, min_size: *const c_void);
    // BLOCKED: pub fn wxSizer_Remove(self_: *mut c_void, window: *mut c_void) -> bool;
    pub fn wxSizer_Remove1(self_: *mut c_void, sizer: *mut c_void) -> bool;
    pub fn wxSizer_Remove2(self_: *mut c_void, index: c_int) -> bool;
    pub fn wxSizer_Replace(
        self_: *mut c_void,
        oldwin: *mut c_void,
        newwin: *mut c_void,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Replace1(
        self_: *mut c_void,
        oldsz: *mut c_void,
        newsz: *mut c_void,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Replace2(self_: *mut c_void, index: usize, newitem: *mut c_void) -> bool;
    pub fn wxSizer_SetDimension(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn wxSizer_SetDimension1(self_: *mut c_void, pos: *const c_void, size: *const c_void);
    pub fn wxSizer_SetItemMinSize(
        self_: *mut c_void,
        window: *mut c_void,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize1(
        self_: *mut c_void,
        window: *mut c_void,
        size: *const c_void,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize2(
        self_: *mut c_void,
        sizer: *mut c_void,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize3(
        self_: *mut c_void,
        sizer: *mut c_void,
        size: *const c_void,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize4(
        self_: *mut c_void,
        index: usize,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxSizer_SetItemMinSize5(self_: *mut c_void, index: usize, size: *const c_void) -> bool;
    pub fn wxSizer_SetMinSize(self_: *mut c_void, size: *const c_void);
    pub fn wxSizer_SetMinSize1(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxSizer_SetSizeHints(self_: *mut c_void, window: *mut c_void);
    // BLOCKED: pub fn wxSizer_SetVirtualSizeHints(self_: *mut c_void, window: *mut c_void);
    pub fn wxSizer_Show(
        self_: *mut c_void,
        window: *mut c_void,
        show: bool,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Show1(
        self_: *mut c_void,
        sizer: *mut c_void,
        show: bool,
        recursive: bool,
    ) -> bool;
    pub fn wxSizer_Show2(self_: *mut c_void, index: usize, show: bool) -> bool;
    pub fn wxSizer_ShowItems(self_: *mut c_void, show: bool);

    // wxSizerFlags
    pub fn wxSizerFlags_delete(self_: *mut c_void);
    pub fn wxSizerFlags_new(proportion: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Align(self_: *mut c_void, alignment: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Border(
        self_: *mut c_void,
        direction: c_int,
        borderinpixels: c_int,
    ) -> *mut c_void;
    pub fn wxSizerFlags_Border1(self_: *mut c_void, direction: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Bottom(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Center(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Centre(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CenterHorizontal(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CenterVertical(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CentreHorizontal(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_CentreVertical(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_DoubleBorder(self_: *mut c_void, direction: c_int) -> *mut c_void;
    pub fn wxSizerFlags_DoubleHorzBorder(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Expand(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_FixedMinSize(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_ReserveSpaceEvenIfHidden(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Left(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Proportion(self_: *mut c_void, proportion: c_int) -> *mut c_void;
    pub fn wxSizerFlags_Right(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Shaped(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_Top(self_: *mut c_void) -> *mut c_void;
    pub fn wxSizerFlags_TripleBorder(self_: *mut c_void, direction: c_int) -> *mut c_void;
    pub fn wxSizerFlags_GetDefaultBorder() -> c_int;
    // NOT_SUPPORTED: pub fn wxSizerFlags_GetDefaultBorderFractional() -> float;

    // wxStaticBitmap
    pub fn wxStaticBitmap_new() -> *mut c_void;
    pub fn wxStaticBitmap_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticBitmap_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxStaticBitmap_GetBitmap(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxStaticBitmap_GetIcon(self_: *const c_void) -> wxIcon;
    pub fn wxStaticBitmap_SetBitmap(self_: *mut c_void, label: *const c_void);
    pub fn wxStaticBitmap_SetIcon(self_: *mut c_void, label: *const c_void);
    // NOT_SUPPORTED: pub fn wxStaticBitmap_SetScaleMode(self_: *mut c_void, scale_mode: ScaleMode);
    // NOT_SUPPORTED: pub fn wxStaticBitmap_GetScaleMode(self_: *const c_void) -> ScaleMode;

    // wxStaticBoxSizer
    pub fn wxStaticBoxSizer_new(box_: *mut c_void, orient: c_int) -> *mut c_void;
    pub fn wxStaticBoxSizer_new1(
        orient: c_int,
        parent: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticBoxSizer_GetStaticBox(self_: *const c_void) -> *mut c_void;

    // wxToolBar
    pub fn wxToolBar_new() -> *mut c_void;
    pub fn wxToolBar_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxToolBar_~wxToolBar(self_: *mut c_void);
    pub fn wxToolBar_AddCheckTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap1: *const c_void,
        bmp_disabled: *const c_void,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddControl(
        self_: *mut c_void,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddRadioTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap1: *const c_void,
        bmp_disabled: *const c_void,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_AddSeparator(self_: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddStretchableSpace(self_: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddTool(self_: *mut c_void, tool: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_AddTool1(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        short_help: *const c_void,
        kind: c_int,
    ) -> *mut c_void;
    pub fn wxToolBar_AddTool2(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_ClearTools(self_: *mut c_void);
    pub fn wxToolBar_DeleteTool(self_: *mut c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_DeleteToolByPos(self_: *mut c_void, pos: usize) -> bool;
    pub fn wxToolBar_EnableTool(self_: *mut c_void, tool_id: c_int, enable: bool);
    pub fn wxToolBar_FindById(self_: *const c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_FindControl(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_FindToolForPosition(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxToolBar_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxToolBar_GetToolBitmapSize(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxToolBar_GetToolByPos(self_: *mut c_void, pos: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolByPos1(self_: *const c_void, pos: c_int) -> *const c_void;
    pub fn wxToolBar_GetToolClientData(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolEnabled(self_: *const c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_GetToolLongHelp(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolPacking(self_: *const c_void) -> c_int;
    pub fn wxToolBar_GetToolPos(self_: *const c_void, tool_id: c_int) -> c_int;
    pub fn wxToolBar_GetToolSeparation(self_: *const c_void) -> c_int;
    pub fn wxToolBar_GetToolShortHelp(self_: *const c_void, tool_id: c_int) -> *mut c_void;
    pub fn wxToolBar_GetToolSize(self_: *const c_void) -> *mut c_void;
    pub fn wxToolBar_GetToolState(self_: *const c_void, tool_id: c_int) -> bool;
    pub fn wxToolBar_GetToolsCount(self_: *const c_void) -> usize;
    pub fn wxToolBar_InsertControl(
        self_: *mut c_void,
        pos: usize,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_InsertSeparator(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxToolBar_InsertStretchableSpace(self_: *mut c_void, pos: usize) -> *mut c_void;
    pub fn wxToolBar_InsertTool(
        self_: *mut c_void,
        pos: usize,
        tool_id: c_int,
        label: *const c_void,
        bitmap: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        short_help: *const c_void,
        long_help: *const c_void,
        client_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_InsertTool1(self_: *mut c_void, pos: usize, tool: *mut c_void) -> *mut c_void;
    pub fn wxToolBar_OnLeftClick(self_: *mut c_void, tool_id: c_int, toggle_down: bool) -> bool;
    pub fn wxToolBar_OnMouseEnter(self_: *mut c_void, tool_id: c_int);
    pub fn wxToolBar_OnRightClick(self_: *mut c_void, tool_id: c_int, x: c_long, y: c_long);
    pub fn wxToolBar_Realize(self_: *mut c_void) -> bool;
    pub fn wxToolBar_RemoveTool(self_: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxToolBar_SetDropdownMenu(self_: *mut c_void, id: c_int, menu: *mut c_void) -> bool;
    pub fn wxToolBar_SetMargins(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxToolBar_SetMargins1(self_: *mut c_void, size: *const c_void);
    pub fn wxToolBar_SetToolBitmapSize(self_: *mut c_void, size: *const c_void);
    pub fn wxToolBar_SetToolClientData(self_: *mut c_void, id: c_int, client_data: *mut c_void);
    pub fn wxToolBar_SetToolDisabledBitmap(self_: *mut c_void, id: c_int, bitmap: *const c_void);
    pub fn wxToolBar_SetToolLongHelp(
        self_: *mut c_void,
        tool_id: c_int,
        help_string: *const c_void,
    );
    pub fn wxToolBar_SetToolNormalBitmap(self_: *mut c_void, id: c_int, bitmap: *const c_void);
    pub fn wxToolBar_SetToolPacking(self_: *mut c_void, packing: c_int);
    pub fn wxToolBar_SetToolSeparation(self_: *mut c_void, separation: c_int);
    pub fn wxToolBar_SetToolShortHelp(
        self_: *mut c_void,
        tool_id: c_int,
        help_string: *const c_void,
    );
    pub fn wxToolBar_ToggleTool(self_: *mut c_void, tool_id: c_int, toggle: bool);
    pub fn wxToolBar_CreateTool(
        self_: *mut c_void,
        tool_id: c_int,
        label: *const c_void,
        bmp_normal: *const c_void,
        bmp_disabled: *const c_void,
        kind: c_int,
        client_data: *mut c_void,
        short_help: *const c_void,
        long_help: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_CreateTool1(
        self_: *mut c_void,
        control: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxToolBar_CreateSeparator(self_: *mut c_void) -> *mut c_void;

    // wxTopLevelWindow
    pub fn wxTopLevelWindow_new() -> *mut c_void;
    pub fn wxTopLevelWindow_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxTopLevelWindow_~wxTopLevelWindow(self_: *mut c_void);
    pub fn wxTopLevelWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxTopLevelWindow_CenterOnScreen(self_: *mut c_void, direction: c_int);
    pub fn wxTopLevelWindow_CentreOnScreen(self_: *mut c_void, direction: c_int);
    pub fn wxTopLevelWindow_EnableCloseButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_EnableMaximizeButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_EnableMinimizeButton(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_GetDefaultItem(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxTopLevelWindow_GetIcon(self_: *const c_void) -> wxIcon;
    // BLOCKED: pub fn wxTopLevelWindow_GetIcons(self_: *const c_void) -> *const c_void;
    pub fn wxTopLevelWindow_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_Iconize(self_: *mut c_void, iconize: bool);
    pub fn wxTopLevelWindow_IsActive(self_: *mut c_void) -> bool;
    pub fn wxTopLevelWindow_IsAlwaysMaximized(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsFullScreen(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsIconized(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_IsMaximized(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_IsUsingNativeDecorations(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_Maximize(self_: *mut c_void, maximize: bool);
    pub fn wxTopLevelWindow_MSWGetSystemMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_RequestUserAttention(self_: *mut c_void, flags: c_int);
    pub fn wxTopLevelWindow_Restore(self_: *mut c_void);
    // BLOCKED: pub fn wxTopLevelWindow_RestoreToGeometry(self_: *mut c_void, ser: *mut c_void) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_SaveGeometry(self_: *const c_void, ser: *const c_void) -> bool;
    pub fn wxTopLevelWindow_SetDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_SetTmpDefaultItem(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_GetTmpDefaultItem(self_: *const c_void) -> *mut c_void;
    pub fn wxTopLevelWindow_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxTopLevelWindow_SetIcons(self_: *mut c_void, icons: *const c_void);
    pub fn wxTopLevelWindow_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxTopLevelWindow_ShouldPreventAppExit(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_OSXSetModified(self_: *mut c_void, modified: bool);
    pub fn wxTopLevelWindow_OSXIsModified(self_: *const c_void) -> bool;
    pub fn wxTopLevelWindow_SetRepresentedFilename(self_: *mut c_void, filename: *const c_void);
    pub fn wxTopLevelWindow_ShowWithoutActivating(self_: *mut c_void);
    pub fn wxTopLevelWindow_EnableFullScreenView(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxTopLevelWindow_ShowFullScreen(self_: *mut c_void, show: bool, style: c_long) -> bool;
    // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorations(self_: *mut c_void, native: bool);
    // BLOCKED: pub fn wxTopLevelWindow_UseNativeDecorationsByDefault(self_: *mut c_void, native: bool);
    pub fn wxTopLevelWindow_GetDefaultSize() -> *mut c_void;

    // wxValidator
    pub fn wxValidator_new() -> *mut c_void;
    // DTOR: pub fn wxValidator_~wxValidator(self_: *mut c_void);
    pub fn wxValidator_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_SetWindow(self_: *mut c_void, window: *mut c_void);
    pub fn wxValidator_TransferFromWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_TransferToWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_Validate(self_: *mut c_void, parent: *mut c_void) -> bool;
    pub fn wxValidator_SuppressBellOnError(suppress: bool);
    pub fn wxValidator_IsSilent() -> bool;

    // wxWindow
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
    // BLOCKED: pub fn wxWindow_GetChildren1(self_: *const c_void) -> *const c_void;
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
    // NOT_SUPPORTED: pub fn wxWindow_GetBackgroundColour(self_: *const c_void) -> wxColour;
    // NOT_SUPPORTED: pub fn wxWindow_GetBackgroundStyle(self_: *const c_void) -> wxBackgroundStyle;
    pub fn wxWindow_GetCharHeight(self_: *const c_void) -> c_int;
    pub fn wxWindow_GetCharWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxWindow_GetDefaultAttributes(self_: *const c_void) -> wxVisualAttributes;
    pub fn wxWindow_GetDPI(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxWindow_GetFont(self_: *const c_void) -> wxFont;
    // NOT_SUPPORTED: pub fn wxWindow_GetForegroundColour(self_: *const c_void) -> wxColour;
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
    // NOT_SUPPORTED: pub fn wxWindow_HideWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: unsigned int) -> bool;
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
    // NOT_SUPPORTED: pub fn wxWindow_ShowWithEffect(self_: *mut c_void, effect: wxShowEffect, timeout: unsigned int) -> bool;
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
    pub fn wxWrapSizer_new(orient: c_int, flags: c_int) -> *mut c_void;

}
