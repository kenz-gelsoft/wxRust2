use super::*;

extern "C" {

    // wxBannerWindow
    pub fn wxBannerWindow_CLASSINFO() -> *mut c_void;
    pub fn wxBannerWindow_new() -> *mut c_void;
    // BLOCKED: pub fn wxBannerWindow_new1(parent: *mut c_void, dir: c_int) -> *mut c_void;
    pub fn wxBannerWindow_new2(
        parent: *mut c_void,
        winid: c_int,
        dir: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxBannerWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        dir: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxBannerWindow_SetBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxBannerWindow_SetText(self_: *mut c_void, title: *const c_void, message: *const c_void);
    pub fn wxBannerWindow_SetGradient(self_: *mut c_void, start: *const c_void, end: *const c_void);

    // wxBitmap
    pub fn wxBitmap_CLASSINFO() -> *mut c_void;
    pub fn wxBitmap_new() -> *mut c_void;
    pub fn wxBitmap_new1(bitmap: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_new2(bits: char, width: c_int, height: c_int, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new3(width: c_int, height: c_int, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new4(sz: *const c_void, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new5(width: c_int, height: c_int, dc: *const c_void) -> *mut c_void;
    pub fn wxBitmap_new6(bits: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmap_new7(name: *const c_void, type_: wxBitmapType) -> *mut c_void;
    pub fn wxBitmap_new8(img: *const c_void, depth: c_int) -> *mut c_void;
    pub fn wxBitmap_new9(img: *const c_void, dc: *const c_void) -> *mut c_void;
    pub fn wxBitmap_new10(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBitmap_~wxBitmap(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmap_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    pub fn wxBitmap_ConvertToImage(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_CopyFromIcon(self_: *mut c_void, icon: *const c_void) -> bool;
    pub fn wxBitmap_Create(self_: *mut c_void, width: c_int, height: c_int, depth: c_int) -> bool;
    pub fn wxBitmap_Create1(self_: *mut c_void, sz: *const c_void, depth: c_int) -> bool;
    pub fn wxBitmap_Create2(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        dc: *const c_void,
    ) -> bool;
    pub fn wxBitmap_CreateWithDIPSize(
        self_: *mut c_void,
        size: *const c_void,
        scale: c_double,
        depth: c_int,
    ) -> bool;
    pub fn wxBitmap_CreateWithDIPSize1(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        scale: c_double,
        depth: c_int,
    ) -> bool;
    pub fn wxBitmap_CreateScaled(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        depth: c_int,
        logical_scale: c_double,
    ) -> bool;
    pub fn wxBitmap_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxBitmap_GetDIPSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxBitmap_GetLogicalHeight(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetLogicalSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetLogicalWidth(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetMask(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetPalette(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetSubBitmap(self_: *const c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetScaledHeight(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetScaledSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetScaledWidth(self_: *const c_void) -> c_double;
    pub fn wxBitmap_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmap_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxBitmap_HasAlpha(self_: *const c_void) -> bool;
    pub fn wxBitmap_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxBitmap_LoadFile(self_: *mut c_void, name: *const c_void, type_: wxBitmapType) -> bool;
    // BLOCKED: pub fn wxBitmap_ResetAlpha(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmap_SaveFile(self_: *const c_void, name: *const c_void, type_: wxBitmapType, palette: *const c_void) -> bool;
    pub fn wxBitmap_SetDepth(self_: *mut c_void, depth: c_int);
    pub fn wxBitmap_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxBitmap_SetScaleFactor(self_: *mut c_void, scale: c_double);
    pub fn wxBitmap_SetMask(self_: *mut c_void, mask: *mut c_void);
    pub fn wxBitmap_SetPalette(self_: *mut c_void, palette: *const c_void);
    pub fn wxBitmap_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxBitmap_UseAlpha(self_: *mut c_void, use_: bool) -> bool;
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
    pub fn wxBitmap_Rescale(bmp: *mut c_void, size_needed: *const c_void);

    // wxBitmapBundle
    pub fn wxBitmapBundle_delete(self_: *mut c_void);
    pub fn wxBitmapBundle_new() -> *mut c_void;
    pub fn wxBitmapBundle_new1(bitmap: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new2(icon: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new3(image: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new4(xpm: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_new5(other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxBitmapBundle_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_Clear(self_: *mut c_void);
    pub fn wxBitmapBundle_IsOk(self_: *const c_void) -> bool;
    pub fn wxBitmapBundle_GetDefaultSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetPreferredBitmapSizeAtScale(
        self_: *const c_void,
        scale: c_double,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_GetPreferredBitmapSizeFor(
        self_: *const c_void,
        window: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_GetPreferredLogicalSizeFor(
        self_: *const c_void,
        window: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_GetBitmap(self_: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetBitmapFor(self_: *const c_void, window: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetIcon(self_: *const c_void, size: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_GetIconFor(self_: *const c_void, window: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_IsSameAs(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn wxBitmapBundle_FromBitmaps(bitmaps: *const c_void) -> wxBitmapBundle;
    pub fn wxBitmapBundle_FromBitmaps1(
        bitmap1: *const c_void,
        bitmap2: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_FromBitmap(bitmap: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromIconBundle(icon_bundle: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromImage(image: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromImpl(impl_: *mut c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromResources(name: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromFiles(
        path: *const c_void,
        filename: *const c_void,
        extension: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapBundle_FromFiles1(fullpathname: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxBitmapBundle_FromSVG(data: *mut c_void, size_def: *const c_void) -> wxBitmapBundle;
    pub fn wxBitmapBundle_FromSVG1(data: *const c_void, size_def: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromSVGFile(path: *const c_void, size_def: *const c_void) -> *mut c_void;
    pub fn wxBitmapBundle_FromSVGResource(
        name: *const c_void,
        size_def: *const c_void,
    ) -> *mut c_void;

    // wxBitmapButton
    pub fn wxBitmapButton_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapButton_new() -> *mut c_void;
    pub fn wxBitmapButton_new1(
        parent: *mut c_void,
        id: c_int,
        bitmap: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        bitmap: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxBitmapButton_CreateCloseButton(
        self_: *mut c_void,
        parent: *mut c_void,
        winid: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxBitmapButton_NewCloseButton(
        parent: *mut c_void,
        winid: c_int,
        name: *const c_void,
    ) -> *mut c_void;

    // wxBitmapComboBox
    pub fn wxBitmapComboBox_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapComboBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmapComboBox_new1(parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxBitmapComboBox_new2(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxBitmapComboBox_~wxBitmapComboBox(self_: *mut c_void);
    pub fn wxBitmapComboBox_Append(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_Append1(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_Append2(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    // NOT_SUPPORTED: pub fn wxBitmapComboBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxBitmapComboBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxBitmapComboBox_GetBitmapSize(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapComboBox_GetItemBitmap(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxBitmapComboBox_Insert(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        pos: c_uint,
    ) -> c_int;
    pub fn wxBitmapComboBox_Insert1(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_Insert2(
        self_: *mut c_void,
        item: *const c_void,
        bitmap: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxBitmapComboBox_SetItemBitmap(self_: *mut c_void, n: c_uint, bitmap: *const c_void);
    // Mix-in(s) to wxBitmapComboBox
    pub fn wxBitmapComboBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;
    pub fn wxBitmapComboBox_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxBitmapDataObject
    pub fn wxBitmapDataObject_delete(self_: *mut c_void);
    pub fn wxBitmapDataObject_new(bitmap: *const c_void) -> *mut c_void;
    pub fn wxBitmapDataObject_GetBitmap(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapDataObject_SetBitmap(self_: *mut c_void, bitmap: *const c_void);

    // wxBitmapHandler
    pub fn wxBitmapHandler_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapHandler_new() -> *mut c_void;
    // DTOR: pub fn wxBitmapHandler_~wxBitmapHandler(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBitmapHandler_Create(self_: *mut c_void, bitmap: *mut c_void, data: *const c_void, type_: wxBitmapType, width: c_int, height: c_int, depth: c_int) -> bool;
    pub fn wxBitmapHandler_GetExtension(self_: *const c_void) -> *mut c_void;
    pub fn wxBitmapHandler_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBitmapHandler_GetType(self_: *const c_void) -> wxBitmapType;
    // NOT_SUPPORTED: pub fn wxBitmapHandler_LoadFile(self_: *mut c_void, bitmap: *mut c_void, name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxBitmapHandler_SaveFile(self_: *const c_void, bitmap: *const c_void, name: *const c_void, type_: wxBitmapType, palette: *const c_void) -> bool;
    pub fn wxBitmapHandler_SetExtension(self_: *mut c_void, extension: *const c_void);
    pub fn wxBitmapHandler_SetName(self_: *mut c_void, name: *const c_void);
    // NOT_SUPPORTED: pub fn wxBitmapHandler_SetType(self_: *mut c_void, type_: wxBitmapType);

    // wxBitmapToggleButton
    pub fn wxBitmapToggleButton_CLASSINFO() -> *mut c_void;
    pub fn wxBitmapToggleButton_new() -> *mut c_void;
    pub fn wxBitmapToggleButton_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxBitmapToggleButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        val: *const c_void,
        name: *const c_void,
    ) -> bool;

    // wxBookCtrlBase
    pub fn wxBookCtrlBase_CLASSINFO() -> *mut c_void;
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

    // wxBookCtrlEvent
    pub fn wxBookCtrlEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBookCtrlEvent_new(event_type: wxEventType, id: c_int, sel: c_int, old_sel: c_int) -> *mut c_void;
    pub fn wxBookCtrlEvent_GetOldSelection(self_: *const c_void) -> c_int;
    pub fn wxBookCtrlEvent_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxBookCtrlEvent_SetOldSelection(self_: *mut c_void, page: c_int);
    pub fn wxBookCtrlEvent_SetSelection(self_: *mut c_void, page: c_int);

    // wxBoxSizer
    pub fn wxBoxSizer_CLASSINFO() -> *mut c_void;
    pub fn wxBoxSizer_new(orient: c_int) -> *mut c_void;
    pub fn wxBoxSizer_GetOrientation(self_: *const c_void) -> c_int;
    pub fn wxBoxSizer_SetOrientation(self_: *mut c_void, orient: c_int);

    // wxBrush
    pub fn wxBrush_CLASSINFO() -> *mut c_void;
    pub fn wxBrush_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBrush_new1(colour: *const c_void, style: wxBrushStyle) -> *mut c_void;
    pub fn wxBrush_new2(stipple_bitmap: *const c_void) -> *mut c_void;
    pub fn wxBrush_new3(brush: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBrush_~wxBrush(self_: *mut c_void);
    pub fn wxBrush_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxBrush_GetStipple(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxBrush_GetStyle(self_: *const c_void) -> wxBrushStyle;
    pub fn wxBrush_IsHatch(self_: *const c_void) -> bool;
    pub fn wxBrush_IsOk(self_: *const c_void) -> bool;
    pub fn wxBrush_IsNonTransparent(self_: *const c_void) -> bool;
    pub fn wxBrush_IsTransparent(self_: *const c_void) -> bool;
    pub fn wxBrush_SetColour(self_: *mut c_void, colour: *const c_void);
    // NOT_SUPPORTED: pub fn wxBrush_SetColour1(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char);
    pub fn wxBrush_SetStipple(self_: *mut c_void, bitmap: *const c_void);
    // NOT_SUPPORTED: pub fn wxBrush_SetStyle(self_: *mut c_void, style: wxBrushStyle);
    // BLOCKED: pub fn wxBrush_operator!=(self_: *const c_void, brush: *const c_void) -> bool;
    // BLOCKED: pub fn wxBrush_operator==(self_: *const c_void, brush: *const c_void) -> bool;

    // wxBrushList
    pub fn wxBrushList_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxBrushList_FindOrCreateBrush(self_: *mut c_void, colour: *const c_void, style: wxBrushStyle) -> *mut c_void;

    // wxBusyCursor
    pub fn wxBusyCursor_delete(self_: *mut c_void);
    pub fn wxBusyCursor_new(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxBusyCursor_~wxBusyCursor(self_: *mut c_void);

    // wxBusyInfo
    pub fn wxBusyInfo_delete(self_: *mut c_void);
    pub fn wxBusyInfo_new(flags: *const c_void) -> *mut c_void;
    pub fn wxBusyInfo_new1(msg: *const c_void, parent: *mut c_void) -> *mut c_void;
    pub fn wxBusyInfo_UpdateText(self_: *mut c_void, str: *const c_void);
    pub fn wxBusyInfo_UpdateLabel(self_: *mut c_void, str: *const c_void);
    // DTOR: pub fn wxBusyInfo_~wxBusyInfo(self_: *mut c_void);

    // wxButton
    pub fn wxButton_CLASSINFO() -> *mut c_void;
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

}
