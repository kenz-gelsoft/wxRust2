use super::*;

extern "C" {

    // wxHeaderColumn
    pub fn wxHeaderColumn_delete(self_: *mut c_void);
    pub fn wxHeaderColumn_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderColumn_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderColumn_GetBitmapBundle(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderColumn_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_GetMinWidth(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_GetAlignment(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxHeaderColumn_HasFlag(self_: *const c_void, flag: c_int) -> bool;
    pub fn wxHeaderColumn_IsResizeable(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsSortable(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsReorderable(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsHidden(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsShown(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsSortKey(self_: *const c_void) -> bool;
    pub fn wxHeaderColumn_IsSortOrderAscending(self_: *const c_void) -> bool;

    // wxHeaderColumnSimple
    pub fn wxHeaderColumnSimple_delete(self_: *mut c_void);
    pub fn wxHeaderColumnSimple_new(
        title: *const c_void,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxHeaderColumnSimple_new1(
        bitmap: *const c_void,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;

    // wxHeaderCtrl
    pub fn wxHeaderCtrl_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxHeaderCtrl_new() -> *mut c_void;
    // BLOCKED: pub fn wxHeaderCtrl_new1(parent: *mut c_void, winid: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxHeaderCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxHeaderCtrl_SetColumnCount(self_: *mut c_void, count: c_uint);
    pub fn wxHeaderCtrl_GetColumnCount(self_: *const c_void) -> c_uint;
    pub fn wxHeaderCtrl_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxHeaderCtrl_UpdateColumn(self_: *mut c_void, idx: c_uint);
    pub fn wxHeaderCtrl_SetColumnsOrder(self_: *mut c_void, order: *const c_void);
    pub fn wxHeaderCtrl_GetColumnsOrder(self_: *const c_void) -> *mut c_void;
    pub fn wxHeaderCtrl_GetColumnAt(self_: *const c_void, pos: c_uint) -> c_uint;
    pub fn wxHeaderCtrl_GetColumnPos(self_: *const c_void, idx: c_uint) -> c_uint;
    pub fn wxHeaderCtrl_ResetColumnsOrder(self_: *mut c_void);
    pub fn wxHeaderCtrl_ShowColumnsMenu(
        self_: *mut c_void,
        pt: *const c_void,
        title: *const c_void,
    ) -> bool;
    pub fn wxHeaderCtrl_AddColumnsItems(
        self_: *mut c_void,
        menu: *mut c_void,
        id_columns_base: c_int,
    );
    pub fn wxHeaderCtrl_ShowCustomizeDialog(self_: *mut c_void) -> bool;
    pub fn wxHeaderCtrl_GetColumnTitleWidth(self_: *mut c_void, col: *const c_void) -> c_int;
    pub fn wxHeaderCtrl_GetColumnTitleWidth1(self_: *mut c_void, idx: c_uint) -> c_int;
    pub fn wxHeaderCtrl_MoveColumnInOrderArray(order: *mut c_void, idx: c_uint, pos: c_uint);

    // wxHeaderCtrlSimple
    pub fn wxHeaderCtrlSimple_CLASSINFO() -> *mut c_void;
    pub fn wxHeaderCtrlSimple_new() -> *mut c_void;
    pub fn wxHeaderCtrlSimple_new1(
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxHeaderCtrlSimple_InsertColumn(self_: *mut c_void, col: *const c_void, idx: c_uint);
    pub fn wxHeaderCtrlSimple_AppendColumn(self_: *mut c_void, col: *const c_void);
    pub fn wxHeaderCtrlSimple_DeleteColumn(self_: *mut c_void, idx: c_uint);
    pub fn wxHeaderCtrlSimple_ShowColumn(self_: *mut c_void, idx: c_uint, show: bool);
    pub fn wxHeaderCtrlSimple_HideColumn(self_: *mut c_void, idx: c_uint);
    pub fn wxHeaderCtrlSimple_ShowSortIndicator(self_: *mut c_void, idx: c_uint, sort_order: bool);
    pub fn wxHeaderCtrlSimple_RemoveSortIndicator(self_: *mut c_void);

    // wxHelpControllerBase
    pub fn wxHelpControllerBase_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxHelpControllerBase_new(parent_window: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxHelpControllerBase_~wxHelpControllerBase(self_: *mut c_void);
    pub fn wxHelpControllerBase_DisplayBlock(self_: *mut c_void, block_no: c_long) -> bool;
    pub fn wxHelpControllerBase_DisplayContents(self_: *mut c_void) -> bool;
    pub fn wxHelpControllerBase_DisplayContextPopup(self_: *mut c_void, context_id: c_int) -> bool;
    pub fn wxHelpControllerBase_DisplaySection(self_: *mut c_void, section: *const c_void) -> bool;
    pub fn wxHelpControllerBase_DisplaySection1(self_: *mut c_void, section_no: c_int) -> bool;
    pub fn wxHelpControllerBase_DisplayTextPopup(
        self_: *mut c_void,
        text: *const c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxHelpControllerBase_GetFrameParameters(
        self_: *mut c_void,
        size: *mut c_void,
        pos: *mut c_void,
        new_frame_each_time: *mut c_void,
    ) -> *mut c_void;
    pub fn wxHelpControllerBase_GetParentWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxHelpControllerBase_Initialize(self_: *mut c_void, file: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxHelpControllerBase_KeywordSearch(self_: *mut c_void, key_word: *const c_void, mode: wxHelpSearchMode) -> bool;
    pub fn wxHelpControllerBase_LoadFile(self_: *mut c_void, file: *const c_void) -> bool;
    pub fn wxHelpControllerBase_OnQuit(self_: *mut c_void);
    pub fn wxHelpControllerBase_Quit(self_: *mut c_void) -> bool;
    pub fn wxHelpControllerBase_SetFrameParameters(
        self_: *mut c_void,
        title_format: *const c_void,
        size: *const c_void,
        pos: *const c_void,
        new_frame_each_time: bool,
    );
    pub fn wxHelpControllerBase_SetParentWindow(self_: *mut c_void, parent_window: *mut c_void);
    pub fn wxHelpControllerBase_SetViewer(self_: *mut c_void, viewer: *const c_void, flags: c_long);

    // wxHyperlinkCtrl
    pub fn wxHyperlinkCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxHyperlinkCtrl_new() -> *mut c_void;
    pub fn wxHyperlinkCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        url: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxHyperlinkCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        url: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxHyperlinkCtrl_GetHoverColour(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_GetNormalColour(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_GetVisited(self_: *const c_void) -> bool;
    pub fn wxHyperlinkCtrl_GetVisitedColour(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkCtrl_SetHoverColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxHyperlinkCtrl_SetNormalColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxHyperlinkCtrl_SetURL(self_: *mut c_void, url: *const c_void);
    pub fn wxHyperlinkCtrl_SetVisited(self_: *mut c_void, visited: bool);
    pub fn wxHyperlinkCtrl_SetVisitedColour(self_: *mut c_void, colour: *const c_void);

}
