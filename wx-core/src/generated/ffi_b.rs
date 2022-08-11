use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

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
