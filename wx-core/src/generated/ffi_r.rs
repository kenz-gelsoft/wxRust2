use super::*;

extern "C" {

    // wxRadioBox
    pub fn wxRadioBox_CLASSINFO() -> *mut c_void;
    pub fn wxRadioBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxRadioBox_new1(parent: *mut c_void, id: c_int, label: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, major_dimension: c_int, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxRadioBox_new2(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        major_dimension: c_int,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxRadioBox_~wxRadioBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxRadioBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, label: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, major_dimension: c_int, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxRadioBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        major_dimension: c_int,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRadioBox_Enable(self_: *mut c_void, n: c_uint, enable: bool) -> bool;
    pub fn wxRadioBox_GetColumnCount(self_: *const c_void) -> c_uint;
    pub fn wxRadioBox_GetItemFromPoint(self_: *const c_void, pt: *const c_void) -> c_int;
    pub fn wxRadioBox_GetItemHelpText(self_: *const c_void, item: c_uint) -> *mut c_void;
    pub fn wxRadioBox_GetItemToolTip(self_: *const c_void, item: c_uint) -> *mut c_void;
    pub fn wxRadioBox_GetRowCount(self_: *const c_void) -> c_uint;
    pub fn wxRadioBox_IsItemEnabled(self_: *const c_void, n: c_uint) -> bool;
    pub fn wxRadioBox_IsItemShown(self_: *const c_void, n: c_uint) -> bool;
    pub fn wxRadioBox_SetItemHelpText(self_: *mut c_void, item: c_uint, helptext: *const c_void);
    pub fn wxRadioBox_SetItemToolTip(self_: *mut c_void, item: c_uint, text: *const c_void);
    pub fn wxRadioBox_Show(self_: *mut c_void, item: c_uint, show: bool) -> bool;
    // Mix-in(s) to wxRadioBox
    pub fn wxRadioBox_AsItemContainerImmutable(obj: *mut c_void) -> *mut c_void;

    // wxRadioButton
    pub fn wxRadioButton_CLASSINFO() -> *mut c_void;
    pub fn wxRadioButton_new() -> *mut c_void;
    pub fn wxRadioButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxRadioButton_~wxRadioButton(self_: *mut c_void);
    pub fn wxRadioButton_Create(
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
    pub fn wxRadioButton_GetValue(self_: *const c_void) -> bool;
    pub fn wxRadioButton_SetValue(self_: *mut c_void, value: bool);
    pub fn wxRadioButton_GetFirstInGroup(self_: *const c_void) -> *mut c_void;
    pub fn wxRadioButton_GetLastInGroup(self_: *const c_void) -> *mut c_void;
    pub fn wxRadioButton_GetPreviousInGroup(self_: *const c_void) -> *mut c_void;
    pub fn wxRadioButton_GetNextInGroup(self_: *const c_void) -> *mut c_void;

    // wxRealPoint
    pub fn wxRealPoint_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxRealPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxRealPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxRealPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxRealPoint;
    // BLOCKED: pub fn wxRealPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxRealPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxRealPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxRealPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxRealPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxRealPoint_new() -> *mut c_void;
    pub fn wxRealPoint_new1(x: c_double, y: c_double) -> *mut c_void;
    pub fn wxRealPoint_new2(pt: *const c_void) -> *mut c_void;

    // wxRearrangeCtrl
    pub fn wxRearrangeCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxRearrangeCtrl_new() -> *mut c_void;
    pub fn wxRearrangeCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxRearrangeCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRearrangeCtrl_GetList(self_: *const c_void) -> *mut c_void;

    // wxRearrangeDialog
    pub fn wxRearrangeDialog_CLASSINFO() -> *mut c_void;
    pub fn wxRearrangeDialog_new() -> *mut c_void;
    pub fn wxRearrangeDialog_new1(
        parent: *mut c_void,
        message: *const c_void,
        title: *const c_void,
        order: *const c_void,
        items: *const c_void,
        pos: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxRearrangeDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        message: *const c_void,
        title: *const c_void,
        order: *const c_void,
        items: *const c_void,
        pos: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRearrangeDialog_AddExtraControls(self_: *mut c_void, win: *mut c_void);
    pub fn wxRearrangeDialog_GetList(self_: *const c_void) -> *mut c_void;
    pub fn wxRearrangeDialog_GetOrder(self_: *const c_void) -> *mut c_void;

    // wxRearrangeList
    pub fn wxRearrangeList_CLASSINFO() -> *mut c_void;
    pub fn wxRearrangeList_new() -> *mut c_void;
    pub fn wxRearrangeList_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxRearrangeList_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        order: *const c_void,
        items: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxRearrangeList_GetCurrentOrder(self_: *const c_void) -> *mut c_void;
    pub fn wxRearrangeList_CanMoveCurrentUp(self_: *const c_void) -> bool;
    pub fn wxRearrangeList_CanMoveCurrentDown(self_: *const c_void) -> bool;
    pub fn wxRearrangeList_MoveCurrentUp(self_: *mut c_void) -> bool;
    pub fn wxRearrangeList_MoveCurrentDown(self_: *mut c_void) -> bool;
    // Mix-in(s) to wxRearrangeList
    pub fn wxRearrangeList_AsItemContainer(obj: *mut c_void) -> *mut c_void;

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

    // wxRegion
    pub fn wxRegion_CLASSINFO() -> *mut c_void;
    pub fn wxRegion_new() -> *mut c_void;
    pub fn wxRegion_new1(x: c_int, y: c_int, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxRegion_new2(top_left: *const c_void, bottom_right: *const c_void) -> *mut c_void;
    pub fn wxRegion_new3(rect: *const c_void) -> *mut c_void;
    pub fn wxRegion_new4(region: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxRegion_new5(n: usize, points: *const c_void, fill_style: wxPolygonFillMode) -> *mut c_void;
    pub fn wxRegion_new6(bmp: *const c_void) -> *mut c_void;
    pub fn wxRegion_new7(
        bmp: *const c_void,
        trans_colour: *const c_void,
        tolerance: c_int,
    ) -> *mut c_void;
    // DTOR: pub fn wxRegion_~wxRegion(self_: *mut c_void);
    pub fn wxRegion_Clear(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxRegion_Contains(self_: *const c_void, x: c_int, y: c_int) -> wxRegionContain;
    // NOT_SUPPORTED: pub fn wxRegion_Contains1(self_: *const c_void, pt: *const c_void) -> wxRegionContain;
    // NOT_SUPPORTED: pub fn wxRegion_Contains2(self_: *const c_void, x: c_int, y: c_int, width: c_int, height: c_int) -> wxRegionContain;
    // NOT_SUPPORTED: pub fn wxRegion_Contains3(self_: *const c_void, rect: *const c_void) -> wxRegionContain;
    pub fn wxRegion_ConvertToBitmap(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxRegion_GetBox(self_: *const c_void, x: *mut c_void, y: *mut c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxRegion_GetBox1(self_: *const c_void) -> *mut c_void;
    pub fn wxRegion_Intersect(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxRegion_Intersect1(self_: *mut c_void, rect: *const c_void) -> bool;
    pub fn wxRegion_Intersect2(self_: *mut c_void, region: *const c_void) -> bool;
    pub fn wxRegion_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxRegion_IsEqual(self_: *const c_void, region: *const c_void) -> bool;
    pub fn wxRegion_Offset(self_: *mut c_void, x: c_int, y: c_int) -> bool;
    pub fn wxRegion_Offset1(self_: *mut c_void, pt: *const c_void) -> bool;
    pub fn wxRegion_Subtract(self_: *mut c_void, rect: *const c_void) -> bool;
    pub fn wxRegion_Subtract1(self_: *mut c_void, region: *const c_void) -> bool;
    pub fn wxRegion_Union(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxRegion_Union1(self_: *mut c_void, rect: *const c_void) -> bool;
    pub fn wxRegion_Union2(self_: *mut c_void, region: *const c_void) -> bool;
    pub fn wxRegion_Union3(self_: *mut c_void, bmp: *const c_void) -> bool;
    pub fn wxRegion_Union4(
        self_: *mut c_void,
        bmp: *const c_void,
        trans_colour: *const c_void,
        tolerance: c_int,
    ) -> bool;
    pub fn wxRegion_Xor(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxRegion_Xor1(self_: *mut c_void, rect: *const c_void) -> bool;
    pub fn wxRegion_Xor2(self_: *mut c_void, region: *const c_void) -> bool;
    // BLOCKED: pub fn wxRegion_operator=(self_: *mut c_void, region: *const c_void) -> *mut c_void;

    // wxRegionIterator
    pub fn wxRegionIterator_CLASSINFO() -> *mut c_void;
    pub fn wxRegionIterator_new() -> *mut c_void;
    pub fn wxRegionIterator_new1(region: *const c_void) -> *mut c_void;
    pub fn wxRegionIterator_GetH(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxRegionIterator_GetW(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetX(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_GetY(self_: *const c_void) -> c_int;
    pub fn wxRegionIterator_HaveRects(self_: *const c_void) -> bool;
    pub fn wxRegionIterator_Reset(self_: *mut c_void);
    pub fn wxRegionIterator_Reset1(self_: *mut c_void, region: *const c_void);
    // BLOCKED: pub fn wxRegionIterator_operator++(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxRegionIterator_operator bool(self_: *const c_void);

    // wxRendererNative
    pub fn wxRendererNative_delete(self_: *mut c_void);
    // DTOR: pub fn wxRendererNative_~wxRendererNative(self_: *mut c_void);
    pub fn wxRendererNative_DrawCheckBox(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawComboBoxDropButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawDropArrow(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawFocusRect(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawGauge(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        value: c_int,
        max: c_int,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawHeaderButton(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, rect: *const c_void, flags: c_int, sort_arrow: wxHeaderSortIconType, params: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawHeaderButtonContents(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, rect: *const c_void, flags: c_int, sort_arrow: wxHeaderSortIconType, params: *mut c_void) -> c_int;
    pub fn wxRendererNative_DrawItemSelectionRect(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawItemText(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        text: *const c_void,
        rect: *const c_void,
        align: c_int,
        flags: c_int,
        ellipsize_mode: c_int,
    );
    pub fn wxRendererNative_DrawPushButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawCollapseButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_GetCollapseButtonSize(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
    ) -> *mut c_void;
    pub fn wxRendererNative_DrawSplitterBorder(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawSplitterSash(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, size: *const c_void, position: c_int, orient: wxOrientation, flags: c_int);
    pub fn wxRendererNative_DrawTreeItemButton(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawChoice(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawComboBox(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawTextCtrl(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_DrawRadioBitmap(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxRendererNative_DrawTitleBarBitmap(self_: *mut c_void, win: *mut c_void, dc: *mut c_void, rect: *const c_void, button: wxTitleBarButton, flags: c_int);
    pub fn wxRendererNative_DrawCheckMark(
        self_: *mut c_void,
        win: *mut c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxRendererNative_GetCheckBoxSize(
        self_: *mut c_void,
        win: *mut c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxRendererNative_GetCheckMarkSize(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxRendererNative_GetExpanderSize(self_: *mut c_void, win: *mut c_void) -> *mut c_void;
    pub fn wxRendererNative_GetHeaderButtonHeight(self_: *mut c_void, win: *mut c_void) -> c_int;
    pub fn wxRendererNative_GetHeaderButtonMargin(self_: *mut c_void, win: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxRendererNative_GetSplitterParams(self_: *mut c_void, win: *const c_void) -> wxSplitterRenderParams;
    // NOT_SUPPORTED: pub fn wxRendererNative_GetVersion(self_: *const c_void) -> wxRendererVersion;
    pub fn wxRendererNative_Get() -> *mut c_void;
    pub fn wxRendererNative_GetDefault() -> *mut c_void;
    pub fn wxRendererNative_GetGeneric() -> *mut c_void;
    pub fn wxRendererNative_Load(name: *const c_void) -> *mut c_void;
    pub fn wxRendererNative_Set(renderer: *mut c_void) -> *mut c_void;

    // wxRichToolTip
    pub fn wxRichToolTip_delete(self_: *mut c_void);
    pub fn wxRichToolTip_new(title: *const c_void, message: *const c_void) -> *mut c_void;
    pub fn wxRichToolTip_SetBackgroundColour(
        self_: *mut c_void,
        col: *const c_void,
        col_end: *const c_void,
    );
    pub fn wxRichToolTip_SetIcon(self_: *mut c_void, icon: c_int);
    pub fn wxRichToolTip_SetIcon1(self_: *mut c_void, icon: *const c_void);
    // NOT_SUPPORTED: pub fn wxRichToolTip_SetTimeout(self_: *mut c_void, milliseconds_timeout: unsigned, milliseconds_delay: unsigned);
    // NOT_SUPPORTED: pub fn wxRichToolTip_SetTipKind(self_: *mut c_void, tip_kind: wxTipKind);
    pub fn wxRichToolTip_SetTitleFont(self_: *mut c_void, font: *const c_void);
    pub fn wxRichToolTip_ShowFor(self_: *mut c_void, win: *mut c_void, rect: *const c_void);
    // DTOR: pub fn wxRichToolTip_~wxRichToolTip(self_: *mut c_void);

}
