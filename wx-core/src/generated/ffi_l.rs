use super::*;

extern "C" {

    // wxLayoutAlgorithm
    pub fn wxLayoutAlgorithm_CLASSINFO() -> *mut c_void;
    pub fn wxLayoutAlgorithm_new() -> *mut c_void;
    // DTOR: pub fn wxLayoutAlgorithm_~wxLayoutAlgorithm(self_: *mut c_void);
    pub fn wxLayoutAlgorithm_LayoutFrame(
        self_: *mut c_void,
        frame: *mut c_void,
        main_window: *mut c_void,
    ) -> bool;
    pub fn wxLayoutAlgorithm_LayoutMDIFrame(
        self_: *mut c_void,
        frame: *mut c_void,
        rect: *mut c_void,
    ) -> bool;
    pub fn wxLayoutAlgorithm_LayoutWindow(
        self_: *mut c_void,
        parent: *mut c_void,
        main_window: *mut c_void,
    ) -> bool;

    // wxListBox
    pub fn wxListBox_CLASSINFO() -> *mut c_void;
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
    // BLOCKED: pub fn wxListBox_InsertItems(self_: *mut c_void, n_items: c_uint, items: *const c_void, pos: c_uint);
    pub fn wxListBox_InsertItems1(self_: *mut c_void, items: *const c_void, pos: c_uint);
    pub fn wxListBox_IsSelected(self_: *const c_void, n: c_int) -> bool;
    pub fn wxListBox_SetFirstItem(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetFirstItem1(self_: *mut c_void, string: *const c_void);
    pub fn wxListBox_EnsureVisible(self_: *mut c_void, n: c_int);
    pub fn wxListBox_IsSorted(self_: *const c_void) -> bool;
    pub fn wxListBox_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxListBox_GetTopItem(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxListBox_MSWSetTabStops(self_: *mut c_void, tab_stops: *const c_void) -> bool;
    // Mix-in(s) to wxListBox
    pub fn wxListBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxListCtrl
    pub fn wxListCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxListCtrl_new() -> *mut c_void;
    pub fn wxListCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListCtrl_~wxListCtrl(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListCtrl_AppendColumn(self_: *mut c_void, heading: *const c_void, format: wxListColumnFormat, width: c_int) -> c_long;
    pub fn wxListCtrl_Arrange(self_: *mut c_void, flag: c_int) -> bool;
    pub fn wxListCtrl_AssignImageList(self_: *mut c_void, image_list: *mut c_void, which: c_int);
    pub fn wxListCtrl_ClearAll(self_: *mut c_void);
    pub fn wxListCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxListCtrl_DeleteAllColumns(self_: *mut c_void) -> bool;
    pub fn wxListCtrl_DeleteAllItems(self_: *mut c_void) -> bool;
    pub fn wxListCtrl_DeleteColumn(self_: *mut c_void, col: c_int) -> bool;
    pub fn wxListCtrl_DeleteItem(self_: *mut c_void, item: c_long) -> bool;
    pub fn wxListCtrl_EditLabel(
        self_: *mut c_void,
        item: c_long,
        text_control_class: *mut c_void,
    ) -> *mut c_void;
    pub fn wxListCtrl_EnableAlternateRowColours(self_: *mut c_void, enable: bool);
    pub fn wxListCtrl_EnableBellOnNoMatch(self_: *mut c_void, on: bool);
    pub fn wxListCtrl_EndEditLabel(self_: *mut c_void, cancel: bool) -> bool;
    pub fn wxListCtrl_EnsureVisible(self_: *mut c_void, item: c_long) -> bool;
    pub fn wxListCtrl_FindItem(
        self_: *mut c_void,
        start: c_long,
        str: *const c_void,
        partial: bool,
    ) -> c_long;
    // NOT_SUPPORTED: pub fn wxListCtrl_FindItem1(self_: *mut c_void, start: c_long, data: wxUIntPtr) -> c_long;
    pub fn wxListCtrl_FindItem2(
        self_: *mut c_void,
        start: c_long,
        pt: *const c_void,
        direction: c_int,
    ) -> c_long;
    pub fn wxListCtrl_GetColumn(self_: *const c_void, col: c_int, item: *mut c_void) -> bool;
    pub fn wxListCtrl_GetColumnCount(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetColumnIndexFromOrder(self_: *const c_void, pos: c_int) -> c_int;
    pub fn wxListCtrl_GetColumnOrder(self_: *const c_void, col: c_int) -> c_int;
    pub fn wxListCtrl_GetColumnWidth(self_: *const c_void, col: c_int) -> c_int;
    pub fn wxListCtrl_GetColumnsOrder(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetEditControl(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetImageList(self_: *const c_void, which: c_int) -> *mut c_void;
    pub fn wxListCtrl_GetItem(self_: *const c_void, info: *mut c_void) -> bool;
    pub fn wxListCtrl_GetItemBackgroundColour(self_: *const c_void, item: c_long) -> *mut c_void;
    pub fn wxListCtrl_GetItemCount(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListCtrl_GetItemData(self_: *const c_void, item: c_long) -> wxUIntPtr;
    pub fn wxListCtrl_GetItemFont(self_: *const c_void, item: c_long) -> *mut c_void;
    pub fn wxListCtrl_GetItemPosition(self_: *const c_void, item: c_long, pos: *mut c_void)
        -> bool;
    pub fn wxListCtrl_GetItemRect(
        self_: *const c_void,
        item: c_long,
        rect: *mut c_void,
        code: c_int,
    ) -> bool;
    pub fn wxListCtrl_GetItemSpacing(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetItemState(self_: *const c_void, item: c_long, state_mask: c_long)
        -> c_int;
    pub fn wxListCtrl_GetItemText(self_: *const c_void, item: c_long, col: c_int) -> *mut c_void;
    pub fn wxListCtrl_GetItemTextColour(self_: *const c_void, item: c_long) -> *mut c_void;
    pub fn wxListCtrl_GetNextItem(
        self_: *const c_void,
        item: c_long,
        geometry: c_int,
        state: c_int,
    ) -> c_long;
    pub fn wxListCtrl_GetSelectedItemCount(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetSubItemRect(
        self_: *const c_void,
        item: c_long,
        sub_item: c_long,
        rect: *mut c_void,
        code: c_int,
    ) -> bool;
    pub fn wxListCtrl_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_GetTopItem(self_: *const c_void) -> c_long;
    pub fn wxListCtrl_GetViewRect(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_SetAlternateRowColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxListCtrl_GetAlternateRowColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListCtrl_HitTest(
        self_: *const c_void,
        point: *const c_void,
        flags: *mut c_void,
        ptr_sub_item: *mut c_void,
    ) -> c_long;
    pub fn wxListCtrl_InReportView(self_: *const c_void) -> bool;
    pub fn wxListCtrl_InsertColumn(self_: *mut c_void, col: c_long, info: *const c_void) -> c_long;
    pub fn wxListCtrl_InsertColumn1(
        self_: *mut c_void,
        col: c_long,
        heading: *const c_void,
        format: c_int,
        width: c_int,
    ) -> c_long;
    pub fn wxListCtrl_InsertItem(self_: *mut c_void, info: *mut c_void) -> c_long;
    pub fn wxListCtrl_InsertItem1(
        self_: *mut c_void,
        index: c_long,
        label: *const c_void,
    ) -> c_long;
    pub fn wxListCtrl_InsertItem2(self_: *mut c_void, index: c_long, image_index: c_int) -> c_long;
    pub fn wxListCtrl_InsertItem3(
        self_: *mut c_void,
        index: c_long,
        label: *const c_void,
        image_index: c_int,
    ) -> c_long;
    pub fn wxListCtrl_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxListCtrl_IsVirtual(self_: *const c_void) -> bool;
    pub fn wxListCtrl_RefreshItem(self_: *mut c_void, item: c_long);
    pub fn wxListCtrl_RefreshItems(self_: *mut c_void, item_from: c_long, item_to: c_long);
    pub fn wxListCtrl_ScrollList(self_: *mut c_void, dx: c_int, dy: c_int) -> bool;
    pub fn wxListCtrl_SetColumn(self_: *mut c_void, col: c_int, item: *mut c_void) -> bool;
    pub fn wxListCtrl_SetColumnWidth(self_: *mut c_void, col: c_int, width: c_int) -> bool;
    pub fn wxListCtrl_SetColumnsOrder(self_: *mut c_void, orders: *const c_void) -> bool;
    pub fn wxListCtrl_SetHeaderAttr(self_: *mut c_void, attr: *const c_void) -> bool;
    pub fn wxListCtrl_SetImageList(self_: *mut c_void, image_list: *mut c_void, which: c_int);
    pub fn wxListCtrl_SetNormalImages(self_: *mut c_void, images: *const c_void);
    pub fn wxListCtrl_SetSmallImages(self_: *mut c_void, images: *const c_void);
    pub fn wxListCtrl_IsVisible(self_: *const c_void, item: c_long) -> bool;
    pub fn wxListCtrl_SetItem(self_: *mut c_void, info: *mut c_void) -> bool;
    pub fn wxListCtrl_SetItem1(
        self_: *mut c_void,
        index: c_long,
        column: c_int,
        label: *const c_void,
        image_id: c_int,
    ) -> bool;
    pub fn wxListCtrl_SetItemBackgroundColour(self_: *mut c_void, item: c_long, col: *const c_void);
    pub fn wxListCtrl_SetItemColumnImage(
        self_: *mut c_void,
        item: c_long,
        column: c_long,
        image: c_int,
    ) -> bool;
    pub fn wxListCtrl_SetItemCount(self_: *mut c_void, count: c_long);
    pub fn wxListCtrl_SetItemData(self_: *mut c_void, item: c_long, data: c_long) -> bool;
    pub fn wxListCtrl_SetItemFont(self_: *mut c_void, item: c_long, font: *const c_void);
    pub fn wxListCtrl_SetItemImage(
        self_: *mut c_void,
        item: c_long,
        image: c_int,
        sel_image: c_int,
    ) -> bool;
    pub fn wxListCtrl_SetItemPosition(self_: *mut c_void, item: c_long, pos: *const c_void)
        -> bool;
    // NOT_SUPPORTED: pub fn wxListCtrl_SetItemPtrData(self_: *mut c_void, item: c_long, data: wxUIntPtr) -> bool;
    pub fn wxListCtrl_SetItemState(
        self_: *mut c_void,
        item: c_long,
        state: c_long,
        state_mask: c_long,
    ) -> bool;
    pub fn wxListCtrl_SetItemText(self_: *mut c_void, item: c_long, text: *const c_void);
    pub fn wxListCtrl_SetItemTextColour(self_: *mut c_void, item: c_long, col: *const c_void);
    pub fn wxListCtrl_SetSingleStyle(self_: *mut c_void, style: c_long, add: bool);
    pub fn wxListCtrl_SetTextColour(self_: *mut c_void, col: *const c_void);
    // NOT_SUPPORTED: pub fn wxListCtrl_SortItems(self_: *mut c_void, fn_sort_call_back: wxListCtrlCompare, data: wxIntPtr) -> bool;
    pub fn wxListCtrl_HasCheckBoxes(self_: *const c_void) -> bool;
    pub fn wxListCtrl_EnableCheckBoxes(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxListCtrl_IsItemChecked(self_: *const c_void, item: c_long) -> bool;
    pub fn wxListCtrl_CheckItem(self_: *mut c_void, item: c_long, check: bool);
    pub fn wxListCtrl_ExtendRulesAndAlternateColour(self_: *mut c_void, extend: bool);
    pub fn wxListCtrl_ShowSortIndicator(self_: *mut c_void, col: c_int, ascending: bool);
    pub fn wxListCtrl_RemoveSortIndicator(self_: *mut c_void);
    pub fn wxListCtrl_GetSortIndicator(self_: *const c_void) -> c_int;
    pub fn wxListCtrl_GetUpdatedAscendingSortIndicator(self_: *const c_void, col: c_int) -> bool;
    pub fn wxListCtrl_IsAscendingSortIndicator(self_: *const c_void) -> bool;

    // wxListEvent
    pub fn wxListEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxListEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxListEvent_GetCacheFrom(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetCacheTo(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetColumn(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListEvent_GetData(self_: *const c_void) -> wxUIntPtr;
    pub fn wxListEvent_GetImage(self_: *const c_void) -> c_int;
    pub fn wxListEvent_GetIndex(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetItem(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_GetKeyCode(self_: *const c_void) -> c_int;
    pub fn wxListEvent_GetLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_GetMask(self_: *const c_void) -> c_long;
    pub fn wxListEvent_GetPoint(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxListEvent_IsEditCancelled(self_: *const c_void) -> bool;
    pub fn wxListEvent_SetKeyCode(self_: *mut c_void, code: c_int);
    pub fn wxListEvent_SetIndex(self_: *mut c_void, index: c_long);
    pub fn wxListEvent_SetColumn(self_: *mut c_void, col: c_int);
    pub fn wxListEvent_SetPoint(self_: *mut c_void, point: *const c_void);
    pub fn wxListEvent_SetItem(self_: *mut c_void, item: *const c_void);
    pub fn wxListEvent_SetCacheFrom(self_: *mut c_void, cache_from: c_long);
    pub fn wxListEvent_SetCacheTo(self_: *mut c_void, cache_to: c_long);

    // wxListItem
    pub fn wxListItem_CLASSINFO() -> *mut c_void;
    pub fn wxListItem_new() -> *mut c_void;
    pub fn wxListItem_Clear(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListItem_GetAlign(self_: *const c_void) -> wxListColumnFormat;
    pub fn wxListItem_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetColumn(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListItem_GetData(self_: *const c_void) -> wxUIntPtr;
    pub fn wxListItem_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetId(self_: *const c_void) -> c_long;
    pub fn wxListItem_GetImage(self_: *const c_void) -> c_int;
    pub fn wxListItem_GetMask(self_: *const c_void) -> c_long;
    pub fn wxListItem_GetState(self_: *const c_void) -> c_long;
    pub fn wxListItem_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxListItem_GetWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxListItem_SetAlign(self_: *mut c_void, align: wxListColumnFormat);
    pub fn wxListItem_SetBackgroundColour(self_: *mut c_void, col_back: *const c_void);
    pub fn wxListItem_SetColumn(self_: *mut c_void, col: c_int);
    pub fn wxListItem_SetData(self_: *mut c_void, data: c_long);
    pub fn wxListItem_SetData1(self_: *mut c_void, data: *mut c_void);
    pub fn wxListItem_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxListItem_SetId(self_: *mut c_void, id: c_long);
    pub fn wxListItem_SetImage(self_: *mut c_void, image: c_int);
    pub fn wxListItem_SetMask(self_: *mut c_void, mask: c_long);
    pub fn wxListItem_SetState(self_: *mut c_void, state: c_long);
    pub fn wxListItem_SetStateMask(self_: *mut c_void, state_mask: c_long);
    pub fn wxListItem_SetText(self_: *mut c_void, text: *const c_void);
    pub fn wxListItem_SetTextColour(self_: *mut c_void, col_text: *const c_void);
    pub fn wxListItem_SetWidth(self_: *mut c_void, width: c_int);

    // wxListView
    pub fn wxListView_CLASSINFO() -> *mut c_void;
    pub fn wxListView_new() -> *mut c_void;
    pub fn wxListView_new1(
        parent: *mut c_void,
        winid: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListView_~wxListView(self_: *mut c_void);
    pub fn wxListView_ClearColumnImage(self_: *mut c_void, col: c_int);
    pub fn wxListView_Focus(self_: *mut c_void, index: c_long);
    pub fn wxListView_GetFirstSelected(self_: *const c_void) -> c_long;
    pub fn wxListView_GetFocusedItem(self_: *const c_void) -> c_long;
    pub fn wxListView_GetNextSelected(self_: *const c_void, item: c_long) -> c_long;
    pub fn wxListView_IsSelected(self_: *const c_void, index: c_long) -> bool;
    pub fn wxListView_Select(self_: *mut c_void, n: c_long, on: bool);
    pub fn wxListView_SetColumnImage(self_: *mut c_void, col: c_int, image: c_int);

    // wxListbook
    pub fn wxListbook_CLASSINFO() -> *mut c_void;
    pub fn wxListbook_new() -> *mut c_void;
    pub fn wxListbook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxListbook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxListbook_GetListView(self_: *const c_void) -> *mut c_void;

}
