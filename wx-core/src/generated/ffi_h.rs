use super::*;

extern "C" {

    // wxHScrolledWindow
    pub fn wxHScrolledWindow_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxHScrolledWindow_new() -> *mut c_void;
    // BLOCKED: pub fn wxHScrolledWindow_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxHScrolledWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxHTMLDataObject
    pub fn wxHTMLDataObject_delete(self_: *mut c_void);
    pub fn wxHTMLDataObject_new(html: *const c_void) -> *mut c_void;
    pub fn wxHTMLDataObject_GetHTML(self_: *const c_void) -> *mut c_void;
    pub fn wxHTMLDataObject_SetHTML(self_: *mut c_void, html: *const c_void);

    // wxHVScrolledWindow
    pub fn wxHVScrolledWindow_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxHVScrolledWindow_new() -> *mut c_void;
    // BLOCKED: pub fn wxHVScrolledWindow_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxHVScrolledWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

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

    // wxHeaderCtrlEvent
    pub fn wxHeaderCtrlEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHeaderCtrlEvent_new(command_type: wxEventType, winid: c_int) -> *mut c_void;
    pub fn wxHeaderCtrlEvent_new1(event: *const c_void) -> *mut c_void;
    pub fn wxHeaderCtrlEvent_GetColumn(self_: *const c_void) -> c_int;
    pub fn wxHeaderCtrlEvent_SetColumn(self_: *mut c_void, col: c_int);
    pub fn wxHeaderCtrlEvent_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxHeaderCtrlEvent_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxHeaderCtrlEvent_GetNewOrder(self_: *const c_void) -> c_uint;
    pub fn wxHeaderCtrlEvent_SetNewOrder(self_: *mut c_void, order: c_uint);

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

    // wxHelpEvent
    pub fn wxHelpEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHelpEvent_new(type_: wxEventType, winid: c_int, pt: *const c_void, origin: wxHelpEvent::Origin) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHelpEvent_GetOrigin(self_: *const c_void) -> wxHelpEvent::Origin;
    pub fn wxHelpEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHelpEvent_SetOrigin(self_: *mut c_void, origin: wxHelpEvent::Origin);
    pub fn wxHelpEvent_SetPosition(self_: *mut c_void, pt: *const c_void);

    // wxHelpProvider
    pub fn wxHelpProvider_delete(self_: *mut c_void);
    // DTOR: pub fn wxHelpProvider_~wxHelpProvider(self_: *mut c_void);
    pub fn wxHelpProvider_AddHelp(self_: *mut c_void, window: *mut c_void, text: *const c_void);
    pub fn wxHelpProvider_AddHelp1(self_: *mut c_void, id: c_int, text: *const c_void);
    pub fn wxHelpProvider_GetHelp(self_: *mut c_void, window: *const c_void) -> *mut c_void;
    pub fn wxHelpProvider_RemoveHelp(self_: *mut c_void, window: *mut c_void);
    pub fn wxHelpProvider_ShowHelp(self_: *mut c_void, window: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxHelpProvider_ShowHelpAtPoint(self_: *mut c_void, window: *mut c_void, point: *const c_void, origin: wxHelpEvent::Origin) -> bool;
    pub fn wxHelpProvider_Get() -> *mut c_void;
    pub fn wxHelpProvider_Set(help_provider: *mut c_void) -> *mut c_void;

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

    // wxHyperlinkEvent
    pub fn wxHyperlinkEvent_CLASSINFO() -> *mut c_void;
    pub fn wxHyperlinkEvent_new(
        generator: *mut c_void,
        id: c_int,
        url: *const c_void,
    ) -> *mut c_void;
    pub fn wxHyperlinkEvent_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxHyperlinkEvent_SetURL(self_: *mut c_void, url: *const c_void);

}
