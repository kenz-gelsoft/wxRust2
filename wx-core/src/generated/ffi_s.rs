use super::*;

extern "C" {

    // wxSearchCtrl
    pub fn wxSearchCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxSearchCtrl_new() -> *mut c_void;
    pub fn wxSearchCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSearchCtrl_~wxSearchCtrl(self_: *mut c_void);
    pub fn wxSearchCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxSearchCtrl_GetMenu(self_: *mut c_void) -> *mut c_void;
    pub fn wxSearchCtrl_IsSearchButtonVisible(self_: *const c_void) -> bool;
    pub fn wxSearchCtrl_IsCancelButtonVisible(self_: *const c_void) -> bool;
    pub fn wxSearchCtrl_SetMenu(self_: *mut c_void, menu: *mut c_void);
    pub fn wxSearchCtrl_ShowCancelButton(self_: *mut c_void, show: bool);
    pub fn wxSearchCtrl_ShowSearchButton(self_: *mut c_void, show: bool);
    pub fn wxSearchCtrl_SetDescriptiveText(self_: *mut c_void, text: *const c_void);
    pub fn wxSearchCtrl_GetDescriptiveText(self_: *const c_void) -> *mut c_void;
    // Mix-in(s) to wxSearchCtrl
    pub fn wxSearchCtrl_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxSettableHeaderColumn
    pub fn wxSettableHeaderColumn_delete(self_: *mut c_void);
    pub fn wxSettableHeaderColumn_SetTitle(self_: *mut c_void, title: *const c_void);
    pub fn wxSettableHeaderColumn_SetBitmap(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxSettableHeaderColumn_SetWidth(self_: *mut c_void, width: c_int);
    pub fn wxSettableHeaderColumn_SetMinWidth(self_: *mut c_void, min_width: c_int);
    pub fn wxSettableHeaderColumn_SetAlignment(self_: *mut c_void, align: c_int);
    pub fn wxSettableHeaderColumn_SetFlags(self_: *mut c_void, flags: c_int);
    pub fn wxSettableHeaderColumn_ChangeFlag(self_: *mut c_void, flag: c_int, set: bool);
    pub fn wxSettableHeaderColumn_SetFlag(self_: *mut c_void, flag: c_int);
    pub fn wxSettableHeaderColumn_ClearFlag(self_: *mut c_void, flag: c_int);
    pub fn wxSettableHeaderColumn_ToggleFlag(self_: *mut c_void, flag: c_int);
    pub fn wxSettableHeaderColumn_SetResizeable(self_: *mut c_void, resizable: bool);
    pub fn wxSettableHeaderColumn_SetSortable(self_: *mut c_void, sortable: bool);
    pub fn wxSettableHeaderColumn_SetReorderable(self_: *mut c_void, reorderable: bool);
    pub fn wxSettableHeaderColumn_SetHidden(self_: *mut c_void, hidden: bool);
    pub fn wxSettableHeaderColumn_UnsetAsSortKey(self_: *mut c_void);
    pub fn wxSettableHeaderColumn_SetSortOrder(self_: *mut c_void, ascending: bool);
    pub fn wxSettableHeaderColumn_ToggleSortOrder(self_: *mut c_void);

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
    // BLOCKED: pub fn wxSize_operator/1(self_: *mut c_void, sz: *const c_void, factor: c_double) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*1(self_: *mut c_void, sz: *const c_void, factor: c_double) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*2(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator*3(self_: *mut c_void, factor: c_double, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxSize_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator/=1(self_: *mut c_void, factor: c_double) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxSize_operator*=1(self_: *mut c_void, factor: c_double) -> *mut c_void;
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
    pub fn wxSizer_CLASSINFO() -> *mut c_void;
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
    pub fn wxSizer_GetChildren(self_: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSizer_GetChildren1(self_: *const c_void) -> *mut c_void;
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
    pub fn wxSizerFlags_DisableConsistencyChecks();
    pub fn wxSizerFlags_GetDefaultBorder() -> c_int;
    // NOT_SUPPORTED: pub fn wxSizerFlags_GetDefaultBorderFractional() -> float;

    // wxSlider
    pub fn wxSlider_CLASSINFO() -> *mut c_void;
    pub fn wxSlider_new() -> *mut c_void;
    pub fn wxSlider_new1(
        parent: *mut c_void,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSlider_~wxSlider(self_: *mut c_void);
    pub fn wxSlider_ClearSel(self_: *mut c_void);
    pub fn wxSlider_ClearTicks(self_: *mut c_void);
    pub fn wxSlider_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: c_int,
        min_value: c_int,
        max_value: c_int,
        point: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxSlider_GetLineSize(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetMax(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetMin(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetPageSize(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetSelEnd(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetSelStart(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetThumbLength(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetTickFreq(self_: *const c_void) -> c_int;
    pub fn wxSlider_GetValue(self_: *const c_void) -> c_int;
    pub fn wxSlider_SetLineSize(self_: *mut c_void, line_size: c_int);
    pub fn wxSlider_SetMin(self_: *mut c_void, min_value: c_int);
    pub fn wxSlider_SetMax(self_: *mut c_void, max_value: c_int);
    pub fn wxSlider_SetPageSize(self_: *mut c_void, page_size: c_int);
    pub fn wxSlider_SetRange(self_: *mut c_void, min_value: c_int, max_value: c_int);
    pub fn wxSlider_SetSelection(self_: *mut c_void, start_pos: c_int, end_pos: c_int);
    pub fn wxSlider_SetThumbLength(self_: *mut c_void, len: c_int);
    pub fn wxSlider_SetTick(self_: *mut c_void, tick_pos: c_int);
    pub fn wxSlider_SetTickFreq(self_: *mut c_void, freq: c_int);
    pub fn wxSlider_SetValue(self_: *mut c_void, value: c_int);

    // wxSpinButton
    pub fn wxSpinButton_CLASSINFO() -> *mut c_void;
    pub fn wxSpinButton_new() -> *mut c_void;
    pub fn wxSpinButton_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxSpinButton_~wxSpinButton(self_: *mut c_void);
    pub fn wxSpinButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxSpinButton_GetIncrement(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_GetMax(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_GetMin(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_GetValue(self_: *const c_void) -> c_int;
    pub fn wxSpinButton_SetIncrement(self_: *mut c_void, value: c_int);
    pub fn wxSpinButton_SetRange(self_: *mut c_void, min: c_int, max: c_int);
    pub fn wxSpinButton_SetValue(self_: *mut c_void, value: c_int);

    // wxSpinCtrl
    pub fn wxSpinCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxSpinCtrl_new() -> *mut c_void;
    pub fn wxSpinCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxSpinCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_int,
        max: c_int,
        initial: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxSpinCtrl_GetBase(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetMax(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetMin(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetTextValue(self_: *const c_void) -> *mut c_void;
    pub fn wxSpinCtrl_GetValue(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_GetIncrement(self_: *const c_void) -> c_int;
    pub fn wxSpinCtrl_SetBase(self_: *mut c_void, base: c_int) -> bool;
    pub fn wxSpinCtrl_SetRange(self_: *mut c_void, min_val: c_int, max_val: c_int);
    pub fn wxSpinCtrl_SetSelection(self_: *mut c_void, from: c_long, to: c_long);
    pub fn wxSpinCtrl_SetValue(self_: *mut c_void, text: *const c_void);
    pub fn wxSpinCtrl_SetValue1(self_: *mut c_void, value: c_int);
    pub fn wxSpinCtrl_SetIncrement(self_: *mut c_void, value: c_int);

    // wxSpinCtrlDouble
    pub fn wxSpinCtrlDouble_CLASSINFO() -> *mut c_void;
    pub fn wxSpinCtrlDouble_new() -> *mut c_void;
    pub fn wxSpinCtrlDouble_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxSpinCtrlDouble_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        min: c_double,
        max: c_double,
        initial: c_double,
        inc: c_double,
        name: *const c_void,
    ) -> bool;
    pub fn wxSpinCtrlDouble_GetDigits(self_: *const c_void) -> c_uint;
    pub fn wxSpinCtrlDouble_GetIncrement(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_GetMax(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_GetMin(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_GetTextValue(self_: *const c_void) -> *mut c_void;
    pub fn wxSpinCtrlDouble_GetValue(self_: *const c_void) -> c_double;
    pub fn wxSpinCtrlDouble_SetDigits(self_: *mut c_void, digits: c_uint);
    pub fn wxSpinCtrlDouble_SetIncrement(self_: *mut c_void, inc: c_double);
    pub fn wxSpinCtrlDouble_SetRange(self_: *mut c_void, min_val: c_double, max_val: c_double);
    pub fn wxSpinCtrlDouble_SetValue(self_: *mut c_void, text: *const c_void);
    pub fn wxSpinCtrlDouble_SetValue1(self_: *mut c_void, value: c_double);

    // wxStaticBitmap
    pub fn wxStaticBitmap_CLASSINFO() -> *mut c_void;
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
    pub fn wxStaticBitmap_GetIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxStaticBitmap_SetBitmap(self_: *mut c_void, label: *const c_void);
    pub fn wxStaticBitmap_SetIcon(self_: *mut c_void, label: *const c_void);
    // NOT_SUPPORTED: pub fn wxStaticBitmap_SetScaleMode(self_: *mut c_void, scale_mode: ScaleMode);
    // NOT_SUPPORTED: pub fn wxStaticBitmap_GetScaleMode(self_: *const c_void) -> ScaleMode;

    // wxStaticBox
    pub fn wxStaticBox_CLASSINFO() -> *mut c_void;
    pub fn wxStaticBox_new() -> *mut c_void;
    pub fn wxStaticBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // BLOCKED: pub fn wxStaticBox_new2(parent: *mut c_void, id: c_int, label: *mut c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxStaticBox_~wxStaticBox(self_: *mut c_void);
    pub fn wxStaticBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    // BLOCKED: pub fn wxStaticBox_Create1(self_: *mut c_void, parent: *mut c_void, id: c_int, label: *mut c_void, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;

    // wxStaticBoxSizer
    pub fn wxStaticBoxSizer_CLASSINFO() -> *mut c_void;
    pub fn wxStaticBoxSizer_new(box_: *mut c_void, orient: c_int) -> *mut c_void;
    pub fn wxStaticBoxSizer_new1(
        orient: c_int,
        parent: *mut c_void,
        label: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticBoxSizer_GetStaticBox(self_: *const c_void) -> *mut c_void;

    // wxStaticText
    pub fn wxStaticText_CLASSINFO() -> *mut c_void;
    pub fn wxStaticText_new() -> *mut c_void;
    pub fn wxStaticText_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxStaticText_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxStaticText_IsEllipsized(self_: *const c_void) -> bool;
    pub fn wxStaticText_Wrap(self_: *mut c_void, width: c_int);

}
