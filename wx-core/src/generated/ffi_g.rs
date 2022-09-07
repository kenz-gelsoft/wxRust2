use super::*;

extern "C" {

    // wxGBPosition
    pub fn wxGBPosition_delete(self_: *mut c_void);
    pub fn wxGBPosition_new() -> *mut c_void;
    pub fn wxGBPosition_new1(row: c_int, col: c_int) -> *mut c_void;
    pub fn wxGBPosition_GetCol(self_: *const c_void) -> c_int;
    pub fn wxGBPosition_GetRow(self_: *const c_void) -> c_int;
    pub fn wxGBPosition_SetCol(self_: *mut c_void, col: c_int);
    pub fn wxGBPosition_SetRow(self_: *mut c_void, row: c_int);
    // BLOCKED: pub fn wxGBPosition_operator!=(self_: *const c_void, p: *const c_void) -> bool;
    // BLOCKED: pub fn wxGBPosition_operator==(self_: *const c_void, p: *const c_void) -> bool;

    // wxGBSizerItem
    pub fn wxGBSizerItem_CLASSINFO() -> *mut c_void;
    pub fn wxGBSizerItem_new(
        width: c_int,
        height: c_int,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGBSizerItem_new1(
        window: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGBSizerItem_new2(
        sizer: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGBSizerItem_GetEndPos(self_: *mut c_void, row: *mut c_void, col: *mut c_void);
    pub fn wxGBSizerItem_GetPos(self_: *const c_void) -> *mut c_void;
    pub fn wxGBSizerItem_GetPos1(self_: *const c_void, row: *mut c_void, col: *mut c_void);
    pub fn wxGBSizerItem_GetSpan(self_: *const c_void) -> *mut c_void;
    pub fn wxGBSizerItem_GetSpan1(self_: *const c_void, rowspan: *mut c_void, colspan: *mut c_void);
    pub fn wxGBSizerItem_Intersects(self_: *mut c_void, other: *const c_void) -> bool;
    pub fn wxGBSizerItem_Intersects1(
        self_: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
    ) -> bool;
    pub fn wxGBSizerItem_SetPos(self_: *mut c_void, pos: *const c_void) -> bool;
    pub fn wxGBSizerItem_SetSpan(self_: *mut c_void, span: *const c_void) -> bool;
    pub fn wxGBSizerItem_GetGBSizer(self_: *const c_void) -> *mut c_void;
    pub fn wxGBSizerItem_SetGBSizer(self_: *mut c_void, sizer: *mut c_void);

    // wxGBSpan
    pub fn wxGBSpan_delete(self_: *mut c_void);
    pub fn wxGBSpan_new() -> *mut c_void;
    pub fn wxGBSpan_new1(rowspan: c_int, colspan: c_int) -> *mut c_void;
    pub fn wxGBSpan_GetColspan(self_: *const c_void) -> c_int;
    pub fn wxGBSpan_GetRowspan(self_: *const c_void) -> c_int;
    pub fn wxGBSpan_SetColspan(self_: *mut c_void, colspan: c_int);
    pub fn wxGBSpan_SetRowspan(self_: *mut c_void, rowspan: c_int);
    // BLOCKED: pub fn wxGBSpan_operator!=(self_: *const c_void, o: *const c_void) -> bool;
    // BLOCKED: pub fn wxGBSpan_operator==(self_: *const c_void, o: *const c_void) -> bool;

    // wxGCDC
    pub fn wxGCDC_CLASSINFO() -> *mut c_void;
    pub fn wxGCDC_new(window_dc: *const c_void) -> *mut c_void;
    pub fn wxGCDC_new1(memory_dc: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxGCDC_new2(printer_dc: *const c_void) -> *mut c_void;
    pub fn wxGCDC_new3(context: *mut c_void) -> *mut c_void;
    pub fn wxGCDC_new4(emf_dc: *const c_void) -> *mut c_void;
    pub fn wxGCDC_new5() -> *mut c_void;
    // DTOR: pub fn wxGCDC_~wxGCDC(self_: *mut c_void);

    // wxGDIObject
    pub fn wxGDIObject_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxGDIObject_new() -> *mut c_void;

    // wxGIFHandler
    pub fn wxGIFHandler_CLASSINFO() -> *mut c_void;
    pub fn wxGIFHandler_new() -> *mut c_void;
    pub fn wxGIFHandler_SaveAnimation(
        self_: *mut c_void,
        images: *const c_void,
        stream: *mut c_void,
        verbose: bool,
        delay_milli_secs: c_int,
    ) -> bool;

    // wxGauge
    pub fn wxGauge_CLASSINFO() -> *mut c_void;
    pub fn wxGauge_new() -> *mut c_void;
    pub fn wxGauge_new1(
        parent: *mut c_void,
        id: c_int,
        range: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxGauge_~wxGauge(self_: *mut c_void);
    pub fn wxGauge_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        range: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxGauge_GetRange(self_: *const c_void) -> c_int;
    pub fn wxGauge_GetValue(self_: *const c_void) -> c_int;
    pub fn wxGauge_IsVertical(self_: *const c_void) -> bool;
    pub fn wxGauge_Pulse(self_: *mut c_void);
    pub fn wxGauge_SetRange(self_: *mut c_void, range: c_int);
    pub fn wxGauge_SetValue(self_: *mut c_void, pos: c_int);

    // wxGenericAboutDialog
    pub fn wxGenericAboutDialog_delete(self_: *mut c_void);
    pub fn wxGenericAboutDialog_new() -> *mut c_void;
    pub fn wxGenericAboutDialog_new1(info: *const c_void, parent: *mut c_void) -> *mut c_void;
    pub fn wxGenericAboutDialog_Create(
        self_: *mut c_void,
        info: *const c_void,
        parent: *mut c_void,
    ) -> bool;

    // wxGenericDirCtrl
    pub fn wxGenericDirCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxGenericDirCtrl_new() -> *mut c_void;
    pub fn wxGenericDirCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dir: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        filter: *const c_void,
        default_filter: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxGenericDirCtrl_~wxGenericDirCtrl(self_: *mut c_void);
    pub fn wxGenericDirCtrl_CollapsePath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxGenericDirCtrl_CollapseTree(self_: *mut c_void);
    pub fn wxGenericDirCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dir: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        filter: *const c_void,
        default_filter: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxGenericDirCtrl_ExpandPath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxGenericDirCtrl_GetDefaultPath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilePath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilePaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxGenericDirCtrl_GetFilter(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxGenericDirCtrl_GetFilterListCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxGenericDirCtrl_GetPath1(self_: *const c_void, item_id: wxTreeItemId) -> wxString;
    pub fn wxGenericDirCtrl_GetPaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxGenericDirCtrl_GetRootId(self_: *mut c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetTreeCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_Init(self_: *mut c_void);
    pub fn wxGenericDirCtrl_ReCreateTree(self_: *mut c_void);
    pub fn wxGenericDirCtrl_SetDefaultPath(self_: *mut c_void, path: *const c_void);
    pub fn wxGenericDirCtrl_SetFilter(self_: *mut c_void, filter: *const c_void);
    pub fn wxGenericDirCtrl_SetFilterIndex(self_: *mut c_void, n: c_int);
    pub fn wxGenericDirCtrl_SetPath(self_: *mut c_void, path: *const c_void);
    pub fn wxGenericDirCtrl_ShowHidden(self_: *mut c_void, show: bool);
    pub fn wxGenericDirCtrl_SelectPath(self_: *mut c_void, path: *const c_void, select: bool);
    pub fn wxGenericDirCtrl_SelectPaths(self_: *mut c_void, paths: *const c_void);
    pub fn wxGenericDirCtrl_UnselectAll(self_: *mut c_void);

    // wxGenericProgressDialog
    pub fn wxGenericProgressDialog_CLASSINFO() -> *mut c_void;
    pub fn wxGenericProgressDialog_new(
        title: *const c_void,
        message: *const c_void,
        maximum: c_int,
        parent: *mut c_void,
        style: c_int,
    ) -> *mut c_void;
    // DTOR: pub fn wxGenericProgressDialog_~wxGenericProgressDialog(self_: *mut c_void);
    pub fn wxGenericProgressDialog_GetValue(self_: *const c_void) -> c_int;
    pub fn wxGenericProgressDialog_GetRange(self_: *const c_void) -> c_int;
    pub fn wxGenericProgressDialog_GetMessage(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericProgressDialog_Pulse(
        self_: *mut c_void,
        newmsg: *const c_void,
        skip: *mut c_void,
    ) -> bool;
    pub fn wxGenericProgressDialog_Resume(self_: *mut c_void);
    pub fn wxGenericProgressDialog_SetRange(self_: *mut c_void, maximum: c_int);
    pub fn wxGenericProgressDialog_WasCancelled(self_: *const c_void) -> bool;
    pub fn wxGenericProgressDialog_WasSkipped(self_: *const c_void) -> bool;
    pub fn wxGenericProgressDialog_Update(
        self_: *mut c_void,
        value: c_int,
        newmsg: *const c_void,
        skip: *mut c_void,
    ) -> bool;

    // wxGenericValidator
    pub fn wxGenericValidator_CLASSINFO() -> *mut c_void;
    pub fn wxGenericValidator_new(validator: *const c_void) -> *mut c_void;
    pub fn wxGenericValidator_new1(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new2(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new3(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new4(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new5(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new6(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new7(val_ptr: *mut c_void) -> *mut c_void;
    pub fn wxGenericValidator_new8(val_ptr: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxGenericValidator_~wxGenericValidator(self_: *mut c_void);

    // wxGraphicsBrush
    pub fn wxGraphicsBrush_CLASSINFO() -> *mut c_void;

    // wxGraphicsContext
    pub fn wxGraphicsContext_CLASSINFO() -> *mut c_void;
    pub fn wxGraphicsContext_Create(window: *mut c_void) -> *mut c_void;
    pub fn wxGraphicsContext_Create1(window_dc: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_Create2(memory_dc: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxGraphicsContext_Create3(printer_dc: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_Create4(meta_file_dc: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_CreateFromUnknownDC(dc: *mut c_void) -> *mut c_void;
    pub fn wxGraphicsContext_Create5(image: *mut c_void) -> *mut c_void;
    pub fn wxGraphicsContext_CreateFromNative(context: *mut c_void) -> *mut c_void;
    pub fn wxGraphicsContext_CreateFromNativeWindow(window: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateFromNativeHDC(dc: WXHDC) -> *mut c_void;
    pub fn wxGraphicsContext_Create6() -> *mut c_void;
    pub fn wxGraphicsContext_ResetClip(self_: *mut c_void);
    pub fn wxGraphicsContext_Clip(self_: *mut c_void, region: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_Clip1(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    pub fn wxGraphicsContext_GetClipBox(
        self_: *mut c_void,
        x: *mut c_void,
        y: *mut c_void,
        w: *mut c_void,
        h: *mut c_void,
    );
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateMatrix(self_: *const c_void, a: wxDouble, b: wxDouble, c: wxDouble, d: wxDouble, tx: wxDouble, ty: wxDouble) -> *mut c_void;
    pub fn wxGraphicsContext_CreateMatrix1(self_: *const c_void, mat: *const c_void)
        -> *mut c_void;
    pub fn wxGraphicsContext_ConcatTransform(self_: *mut c_void, matrix: *const c_void);
    pub fn wxGraphicsContext_GetTransform(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_Rotate(self_: *mut c_void, angle: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_Scale(self_: *mut c_void, x_scale: wxDouble, y_scale: wxDouble);
    pub fn wxGraphicsContext_SetTransform(self_: *mut c_void, matrix: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_Translate(self_: *mut c_void, dx: wxDouble, dy: wxDouble);
    pub fn wxGraphicsContext_CreateBrush(self_: *const c_void, brush: *const c_void)
        -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateLinearGradientBrush(self_: *const c_void, x1: wxDouble, y1: wxDouble, x2: wxDouble, y2: wxDouble, c1: *const c_void, c2: *const c_void, matrix: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateLinearGradientBrush1(self_: *const c_void, x1: wxDouble, y1: wxDouble, x2: wxDouble, y2: wxDouble, stops: *const c_void, matrix: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateRadialGradientBrush(self_: *const c_void, start_x: wxDouble, start_y: wxDouble, end_x: wxDouble, end_y: wxDouble, radius: wxDouble, o_color: *const c_void, c_color: *const c_void, matrix: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateRadialGradientBrush1(self_: *mut c_void, start_x: wxDouble, start_y: wxDouble, end_x: wxDouble, end_y: wxDouble, radius: wxDouble, stops: *const c_void, matrix: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_SetBrush(self_: *mut c_void, brush: *const c_void);
    pub fn wxGraphicsContext_SetBrush1(self_: *mut c_void, brush: *const c_void);
    pub fn wxGraphicsContext_CreatePen(self_: *const c_void, pen: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_CreatePen1(self_: *const c_void, info: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_SetPen(self_: *mut c_void, pen: *const c_void);
    pub fn wxGraphicsContext_SetPen1(self_: *mut c_void, pen: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawBitmap(self_: *mut c_void, bmp: *const c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawBitmap1(self_: *mut c_void, bmp: *const c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawEllipse(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawIcon(self_: *mut c_void, icon: *const c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawLines(self_: *mut c_void, n: usize, points: *const c_void, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawPath(self_: *mut c_void, path: *const c_void, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawRectangle(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawRoundedRectangle(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble, radius: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawText(self_: *mut c_void, str: *const c_void, x: wxDouble, y: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawText1(self_: *mut c_void, str: *const c_void, x: wxDouble, y: wxDouble, angle: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawText2(self_: *mut c_void, str: *const c_void, x: wxDouble, y: wxDouble, background_brush: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_DrawText3(self_: *mut c_void, str: *const c_void, x: wxDouble, y: wxDouble, angle: wxDouble, background_brush: *const c_void);
    pub fn wxGraphicsContext_CreatePath(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_FillPath(self_: *mut c_void, path: *const c_void, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_StrokeLine(self_: *mut c_void, x1: wxDouble, y1: wxDouble, x2: wxDouble, y2: wxDouble);
    pub fn wxGraphicsContext_StrokeLines(
        self_: *mut c_void,
        n: usize,
        begin_points: *const c_void,
        end_points: *const c_void,
    );
    pub fn wxGraphicsContext_StrokeLines1(self_: *mut c_void, n: usize, points: *const c_void);
    pub fn wxGraphicsContext_StrokePath(self_: *mut c_void, path: *const c_void);
    pub fn wxGraphicsContext_CreateFont(
        self_: *const c_void,
        font: *const c_void,
        col: *const c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsContext_CreateFont1(
        self_: *const c_void,
        size_in_pixels: c_double,
        facename: *const c_void,
        flags: c_int,
        col: *const c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsContext_SetFont(
        self_: *mut c_void,
        font: *const c_void,
        colour: *const c_void,
    );
    pub fn wxGraphicsContext_SetFont1(self_: *mut c_void, font: *const c_void);
    pub fn wxGraphicsContext_GetPartialTextExtents(
        self_: *const c_void,
        text: *const c_void,
        widths: *mut c_void,
    );
    pub fn wxGraphicsContext_GetTextExtent(
        self_: *const c_void,
        text: *const c_void,
        width: *mut c_void,
        height: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
    );
    pub fn wxGraphicsContext_StartDoc(self_: *mut c_void, message: *const c_void) -> bool;
    pub fn wxGraphicsContext_EndDoc(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_StartPage(self_: *mut c_void, width: wxDouble, height: wxDouble);
    pub fn wxGraphicsContext_EndPage(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateBitmap(self_: *mut c_void, bitmap: *const c_void) -> wxGraphicsBitmap;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateBitmapFromImage(self_: *mut c_void, image: *const c_void) -> wxGraphicsBitmap;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_CreateSubBitmap(self_: *mut c_void, bitmap: *const c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble) -> wxGraphicsBitmap;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_BeginLayer(self_: *mut c_void, opacity: wxDouble);
    pub fn wxGraphicsContext_EndLayer(self_: *mut c_void);
    pub fn wxGraphicsContext_PushState(self_: *mut c_void);
    pub fn wxGraphicsContext_PopState(self_: *mut c_void);
    pub fn wxGraphicsContext_Flush(self_: *mut c_void);
    pub fn wxGraphicsContext_GetNativeContext(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_SetAntialiasMode(self_: *mut c_void, antialias: wxAntialiasMode) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_GetAntialiasMode(self_: *const c_void) -> wxAntialiasMode;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_SetInterpolationQuality(self_: *mut c_void, interpolation: wxInterpolationQuality) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_GetInterpolationQuality(self_: *const c_void) -> wxInterpolationQuality;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_SetCompositionMode(self_: *mut c_void, op: wxCompositionMode) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsContext_GetCompositionMode(self_: *const c_void) -> wxCompositionMode;
    pub fn wxGraphicsContext_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxGraphicsContext_GetDPI(self_: *const c_void, dpi_x: *mut c_void, dpi_y: *mut c_void);
    pub fn wxGraphicsContext_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_ShouldOffset(self_: *const c_void) -> bool;
    pub fn wxGraphicsContext_EnableOffset(self_: *mut c_void, enable: bool);
    pub fn wxGraphicsContext_DisableOffset(self_: *mut c_void);
    pub fn wxGraphicsContext_OffsetEnabled(self_: *const c_void) -> bool;
    pub fn wxGraphicsContext_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_FromDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxGraphicsContext_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxGraphicsContext_ToDIP2(self_: *const c_void, d: c_int) -> c_int;

    // wxGraphicsFont
    pub fn wxGraphicsFont_CLASSINFO() -> *mut c_void;

    // wxGraphicsGradientStop
    pub fn wxGraphicsGradientStop_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStop_new(col: wxColour, pos: float) -> *mut c_void;
    pub fn wxGraphicsGradientStop_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsGradientStop_SetColour(self_: *mut c_void, col: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStop_GetPosition(self_: *const c_void) -> float;
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStop_SetPosition(self_: *mut c_void, pos: float);

    // wxGraphicsGradientStops
    pub fn wxGraphicsGradientStops_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxGraphicsGradientStops_new(start_col: wxColour, end_col: wxColour) -> *mut c_void;
    pub fn wxGraphicsGradientStops_Add(self_: *mut c_void, stop: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStops_Add1(self_: *mut c_void, col: wxColour, pos: float);
    // NOT_SUPPORTED: pub fn wxGraphicsGradientStops_Item(self_: *const c_void, n: unsigned) -> *mut c_void;
    pub fn wxGraphicsGradientStops_GetCount(self_: *const c_void) -> usize;
    // BLOCKED: pub fn wxGraphicsGradientStops_SetStartColour(self_: *mut c_void, col: wxColour);
    pub fn wxGraphicsGradientStops_GetStartColour(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxGraphicsGradientStops_SetEndColour(self_: *mut c_void, col: wxColour);
    pub fn wxGraphicsGradientStops_GetEndColour(self_: *const c_void) -> *mut c_void;

    // wxGraphicsMatrix
    pub fn wxGraphicsMatrix_CLASSINFO() -> *mut c_void;
    pub fn wxGraphicsMatrix_Concat(self_: *mut c_void, t: *const c_void);
    // BLOCKED: pub fn wxGraphicsMatrix_Concat1(self_: *mut c_void, t: *const c_void);
    pub fn wxGraphicsMatrix_Get(
        self_: *const c_void,
        a: *mut c_void,
        b: *mut c_void,
        c: *mut c_void,
        d: *mut c_void,
        tx: *mut c_void,
        ty: *mut c_void,
    );
    pub fn wxGraphicsMatrix_GetNativeMatrix(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsMatrix_Invert(self_: *mut c_void);
    // BLOCKED: pub fn wxGraphicsMatrix_IsEqual(self_: *const c_void, t: *const c_void) -> bool;
    pub fn wxGraphicsMatrix_IsEqual1(self_: *const c_void, t: *const c_void) -> bool;
    pub fn wxGraphicsMatrix_IsIdentity(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Rotate(self_: *mut c_void, angle: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Scale(self_: *mut c_void, x_scale: wxDouble, y_scale: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Set(self_: *mut c_void, a: wxDouble, b: wxDouble, c: wxDouble, d: wxDouble, tx: wxDouble, ty: wxDouble);
    pub fn wxGraphicsMatrix_TransformDistance(
        self_: *const c_void,
        dx: *mut c_void,
        dy: *mut c_void,
    );
    pub fn wxGraphicsMatrix_TransformPoint(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsMatrix_Translate(self_: *mut c_void, dx: wxDouble, dy: wxDouble);

    // wxGraphicsObject
    pub fn wxGraphicsObject_CLASSINFO() -> *mut c_void;
    pub fn wxGraphicsObject_GetRenderer(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsObject_IsNull(self_: *const c_void) -> bool;

    // wxGraphicsPath
    pub fn wxGraphicsPath_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddArc(self_: *mut c_void, x: wxDouble, y: wxDouble, r: wxDouble, start_angle: wxDouble, end_angle: wxDouble, clockwise: bool);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddArc1(self_: *mut c_void, c: *const c_void, r: wxDouble, start_angle: wxDouble, end_angle: wxDouble, clockwise: bool);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddArcToPoint(self_: *mut c_void, x1: wxDouble, y1: wxDouble, x2: wxDouble, y2: wxDouble, r: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddCircle(self_: *mut c_void, x: wxDouble, y: wxDouble, r: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddCurveToPoint(self_: *mut c_void, cx1: wxDouble, cy1: wxDouble, cx2: wxDouble, cy2: wxDouble, x: wxDouble, y: wxDouble);
    pub fn wxGraphicsPath_AddCurveToPoint1(
        self_: *mut c_void,
        c1: *const c_void,
        c2: *const c_void,
        e: *const c_void,
    );
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddEllipse(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddLineToPoint(self_: *mut c_void, x: wxDouble, y: wxDouble);
    pub fn wxGraphicsPath_AddLineToPoint1(self_: *mut c_void, p: *const c_void);
    pub fn wxGraphicsPath_AddPath(self_: *mut c_void, path: *const c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddQuadCurveToPoint(self_: *mut c_void, cx: wxDouble, cy: wxDouble, x: wxDouble, y: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddRectangle(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_AddRoundedRectangle(self_: *mut c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble, radius: wxDouble);
    pub fn wxGraphicsPath_CloseSubpath(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_Contains(self_: *const c_void, c: *const c_void, fill_style: wxPolygonFillMode) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_Contains1(self_: *const c_void, x: wxDouble, y: wxDouble, fill_style: wxPolygonFillMode) -> bool;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_GetBox(self_: *const c_void) -> wxRect2DDouble;
    pub fn wxGraphicsPath_GetBox1(
        self_: *const c_void,
        x: *mut c_void,
        y: *mut c_void,
        w: *mut c_void,
        h: *mut c_void,
    );
    pub fn wxGraphicsPath_GetCurrentPoint(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGraphicsPath_GetCurrentPoint1(self_: *const c_void) -> wxPoint2DDouble;
    pub fn wxGraphicsPath_GetNativePath(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsPath_MoveToPoint(self_: *mut c_void, x: wxDouble, y: wxDouble);
    pub fn wxGraphicsPath_MoveToPoint1(self_: *mut c_void, p: *const c_void);
    pub fn wxGraphicsPath_Transform(self_: *mut c_void, matrix: *const c_void);
    pub fn wxGraphicsPath_UnGetNativePath(self_: *const c_void, p: *mut c_void);

    // wxGraphicsPen
    pub fn wxGraphicsPen_CLASSINFO() -> *mut c_void;

    // wxGraphicsRenderer
    pub fn wxGraphicsRenderer_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateBitmap(self_: *mut c_void, bitmap: *const c_void) -> wxGraphicsBitmap;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateBitmapFromImage(self_: *mut c_void, image: *const c_void) -> wxGraphicsBitmap;
    pub fn wxGraphicsRenderer_CreateImageFromBitmap(
        self_: *mut c_void,
        bmp: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateBitmapFromNativeBitmap(self_: *mut c_void, bitmap: *mut c_void) -> wxGraphicsBitmap;
    pub fn wxGraphicsRenderer_CreateContext(self_: *mut c_void, window: *mut c_void)
        -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContext1(
        self_: *mut c_void,
        window_dc: *const c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContext2(
        self_: *mut c_void,
        memory_dc: *const c_void,
    ) -> *mut c_void;
    // BLOCKED: pub fn wxGraphicsRenderer_CreateContext3(self_: *mut c_void, printer_dc: *const c_void) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContext4(
        self_: *mut c_void,
        meta_file_dc: *const c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContextFromUnknownDC(
        self_: *mut c_void,
        dc: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContextFromImage(
        self_: *mut c_void,
        image: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateBrush(self_: *mut c_void, brush: *const c_void) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContextFromNativeContext(
        self_: *mut c_void,
        context: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateContextFromNativeWindow(
        self_: *mut c_void,
        window: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateMeasuringContext(self_: *mut c_void) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateFont(
        self_: *mut c_void,
        font: *const c_void,
        col: *const c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateFont1(
        self_: *mut c_void,
        size_in_pixels: c_double,
        facename: *const c_void,
        flags: c_int,
        col: *const c_void,
    ) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreateFontAtDPI(
        self_: *mut c_void,
        font: *const c_void,
        dpi: *const c_void,
        col: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateLinearGradientBrush(self_: *mut c_void, x1: wxDouble, y1: wxDouble, x2: wxDouble, y2: wxDouble, stops: *const c_void, matrix: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateMatrix(self_: *mut c_void, a: wxDouble, b: wxDouble, c: wxDouble, d: wxDouble, tx: wxDouble, ty: wxDouble) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreatePath(self_: *mut c_void) -> *mut c_void;
    pub fn wxGraphicsRenderer_CreatePen(self_: *mut c_void, info: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateRadialGradientBrush(self_: *mut c_void, start_x: wxDouble, start_y: wxDouble, end_x: wxDouble, end_y: wxDouble, radius: wxDouble, stops: *const c_void, matrix: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateSubBitmap(self_: *mut c_void, bitmap: *const c_void, x: wxDouble, y: wxDouble, w: wxDouble, h: wxDouble) -> wxGraphicsBitmap;
    pub fn wxGraphicsRenderer_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxGraphicsRenderer_GetVersion(
        self_: *const c_void,
        major: *mut c_void,
        minor: *mut c_void,
        micro: *mut c_void,
    );
    // NOT_SUPPORTED: pub fn wxGraphicsRenderer_CreateContextFromNativeHDC(dc: WXHDC) -> *mut c_void;
    pub fn wxGraphicsRenderer_GetDefaultRenderer() -> *mut c_void;
    pub fn wxGraphicsRenderer_GetCairoRenderer() -> *mut c_void;
    pub fn wxGraphicsRenderer_GetGDIPlusRenderer() -> *mut c_void;
    // BLOCKED: pub fn wxGraphicsRenderer_GetDirect2DRenderer() -> *mut c_void;

    // wxGridBagSizer
    pub fn wxGridBagSizer_CLASSINFO() -> *mut c_void;
    pub fn wxGridBagSizer_new(vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxGridBagSizer_Add(
        self_: *mut c_void,
        window: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_Add1(
        self_: *mut c_void,
        sizer: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_Add2(self_: *mut c_void, item: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_Add3(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        pos: *const c_void,
        span: *const c_void,
        flag: c_int,
        border: c_int,
        user_data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_CheckForIntersection(
        self_: *mut c_void,
        item: *mut c_void,
        exclude_item: *mut c_void,
    ) -> bool;
    pub fn wxGridBagSizer_CheckForIntersection1(
        self_: *mut c_void,
        pos: *const c_void,
        span: *const c_void,
        exclude_item: *mut c_void,
    ) -> bool;
    pub fn wxGridBagSizer_FindItem(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_FindItem1(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_FindItemAtPoint(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxGridBagSizer_FindItemAtPosition(self_: *mut c_void, pos: *const c_void)
        -> *mut c_void;
    pub fn wxGridBagSizer_FindItemWithData(
        self_: *mut c_void,
        user_data: *const c_void,
    ) -> *mut c_void;
    pub fn wxGridBagSizer_GetCellSize(self_: *const c_void, row: c_int, col: c_int) -> *mut c_void;
    pub fn wxGridBagSizer_GetEmptyCellSize(self_: *const c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemPosition(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemPosition1(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemPosition2(self_: *mut c_void, index: usize) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemSpan(self_: *mut c_void, window: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemSpan1(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxGridBagSizer_GetItemSpan2(self_: *mut c_void, index: usize) -> *mut c_void;
    pub fn wxGridBagSizer_SetEmptyCellSize(self_: *mut c_void, sz: *const c_void);
    pub fn wxGridBagSizer_SetItemPosition(
        self_: *mut c_void,
        window: *mut c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemPosition1(
        self_: *mut c_void,
        sizer: *mut c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemPosition2(
        self_: *mut c_void,
        index: usize,
        pos: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemSpan(
        self_: *mut c_void,
        window: *mut c_void,
        span: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemSpan1(
        self_: *mut c_void,
        sizer: *mut c_void,
        span: *const c_void,
    ) -> bool;
    pub fn wxGridBagSizer_SetItemSpan2(
        self_: *mut c_void,
        index: usize,
        span: *const c_void,
    ) -> bool;

    // wxGridEditorCreatedEvent
    pub fn wxGridEditorCreatedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridEditorCreatedEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, row: c_int, col: c_int, ctrl: *mut c_void) -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_GetCol(self_: *const c_void) -> c_int;
    pub fn wxGridEditorCreatedEvent_GetControl(self_: *mut c_void) -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_GetRow(self_: *const c_void) -> c_int;
    pub fn wxGridEditorCreatedEvent_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxGridEditorCreatedEvent_SetCol(self_: *mut c_void, col: c_int);
    pub fn wxGridEditorCreatedEvent_SetControl(self_: *mut c_void, ctrl: *mut c_void);
    pub fn wxGridEditorCreatedEvent_SetRow(self_: *mut c_void, row: c_int);
    pub fn wxGridEditorCreatedEvent_SetWindow(self_: *mut c_void, window: *mut c_void);

    // wxGridEvent
    pub fn wxGridEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, row: c_int, col: c_int, x: c_int, y: c_int, sel: bool, kbd: *const c_void) -> *mut c_void;
    pub fn wxGridEvent_AltDown(self_: *const c_void) -> bool;
    pub fn wxGridEvent_ControlDown(self_: *const c_void) -> bool;
    pub fn wxGridEvent_GetCol(self_: *const c_void) -> c_int;
    pub fn wxGridEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxGridEvent_GetRow(self_: *const c_void) -> c_int;
    pub fn wxGridEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxGridEvent_Selecting(self_: *const c_void) -> bool;
    pub fn wxGridEvent_ShiftDown(self_: *const c_void) -> bool;

    // wxGridRangeSelectEvent
    pub fn wxGridRangeSelectEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridRangeSelectEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridRangeSelectEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, top_left: *const c_void, bottom_right: *const c_void, sel: bool, kbd: *const c_void) -> *mut c_void;
    pub fn wxGridRangeSelectEvent_AltDown(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_ControlDown(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxGridRangeSelectEvent_GetBottomRightCoords(self_: *const c_void) -> wxGridCellCoords;
    pub fn wxGridRangeSelectEvent_GetBottomRow(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_GetLeftCol(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_GetRightCol(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxGridRangeSelectEvent_GetTopLeftCoords(self_: *const c_void) -> wxGridCellCoords;
    pub fn wxGridRangeSelectEvent_GetTopRow(self_: *const c_void) -> c_int;
    pub fn wxGridRangeSelectEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_Selecting(self_: *const c_void) -> bool;
    pub fn wxGridRangeSelectEvent_ShiftDown(self_: *const c_void) -> bool;

    // wxGridSizeEvent
    pub fn wxGridSizeEvent_CLASSINFO() -> *mut c_void;
    pub fn wxGridSizeEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridSizeEvent_new1(id: c_int, type_: wxEventType, obj: *mut c_void, row_or_col: c_int, x: c_int, y: c_int, kbd: *const c_void) -> *mut c_void;
    pub fn wxGridSizeEvent_AltDown(self_: *const c_void) -> bool;
    pub fn wxGridSizeEvent_ControlDown(self_: *const c_void) -> bool;
    pub fn wxGridSizeEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxGridSizeEvent_GetRowOrCol(self_: *const c_void) -> c_int;
    pub fn wxGridSizeEvent_MetaDown(self_: *const c_void) -> bool;
    pub fn wxGridSizeEvent_ShiftDown(self_: *const c_void) -> bool;

    // wxGridSizer
    pub fn wxGridSizer_CLASSINFO() -> *mut c_void;
    pub fn wxGridSizer_new(cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxGridSizer_new1(cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxGridSizer_new2(rows: c_int, cols: c_int, vgap: c_int, hgap: c_int) -> *mut c_void;
    pub fn wxGridSizer_new3(rows: c_int, cols: c_int, gap: *const c_void) -> *mut c_void;
    pub fn wxGridSizer_GetCols(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetRows(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetEffectiveColsCount(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetEffectiveRowsCount(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetHGap(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_GetVGap(self_: *const c_void) -> c_int;
    pub fn wxGridSizer_SetCols(self_: *mut c_void, cols: c_int);
    pub fn wxGridSizer_SetHGap(self_: *mut c_void, gap: c_int);
    pub fn wxGridSizer_SetRows(self_: *mut c_void, rows: c_int);
    pub fn wxGridSizer_SetVGap(self_: *mut c_void, gap: c_int);

    // wxGridTableBase
    pub fn wxGridTableBase_CLASSINFO() -> *mut c_void;
    pub fn wxGridTableBase_IsEmptyCell(self_: *mut c_void, row: c_int, col: c_int) -> bool;
    pub fn wxGridTableBase_IsEmpty(self_: *mut c_void, coords: *const c_void) -> bool;
    pub fn wxGridTableBase_GetValue(self_: *mut c_void, row: c_int, col: c_int) -> *mut c_void;
    pub fn wxGridTableBase_SetValue(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        value: *const c_void,
    );
    pub fn wxGridTableBase_GetTypeName(self_: *mut c_void, row: c_int, col: c_int) -> *mut c_void;
    pub fn wxGridTableBase_CanGetValueAs(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        type_name: *const c_void,
    ) -> bool;
    pub fn wxGridTableBase_CanSetValueAs(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        type_name: *const c_void,
    ) -> bool;
    pub fn wxGridTableBase_GetValueAsLong(self_: *mut c_void, row: c_int, col: c_int) -> c_long;
    pub fn wxGridTableBase_GetValueAsDouble(self_: *mut c_void, row: c_int, col: c_int)
        -> c_double;
    pub fn wxGridTableBase_GetValueAsBool(self_: *mut c_void, row: c_int, col: c_int) -> bool;
    pub fn wxGridTableBase_GetValueAsCustom(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        type_name: *const c_void,
    ) -> *mut c_void;
    pub fn wxGridTableBase_SetValueAsLong(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        value: c_long,
    );
    pub fn wxGridTableBase_SetValueAsDouble(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        value: c_double,
    );
    pub fn wxGridTableBase_SetValueAsBool(self_: *mut c_void, row: c_int, col: c_int, value: bool);
    pub fn wxGridTableBase_SetValueAsCustom(
        self_: *mut c_void,
        row: c_int,
        col: c_int,
        type_name: *const c_void,
        value: *mut c_void,
    );
    pub fn wxGridTableBase_SetView(self_: *mut c_void, grid: *mut c_void);
    pub fn wxGridTableBase_GetView(self_: *const c_void) -> *mut c_void;
    pub fn wxGridTableBase_Clear(self_: *mut c_void);
    pub fn wxGridTableBase_InsertRows(self_: *mut c_void, pos: usize, num_rows: usize) -> bool;
    pub fn wxGridTableBase_AppendRows(self_: *mut c_void, num_rows: usize) -> bool;
    pub fn wxGridTableBase_DeleteRows(self_: *mut c_void, pos: usize, num_rows: usize) -> bool;
    pub fn wxGridTableBase_InsertCols(self_: *mut c_void, pos: usize, num_cols: usize) -> bool;
    pub fn wxGridTableBase_AppendCols(self_: *mut c_void, num_cols: usize) -> bool;
    pub fn wxGridTableBase_DeleteCols(self_: *mut c_void, pos: usize, num_cols: usize) -> bool;
    pub fn wxGridTableBase_GetRowLabelValue(self_: *mut c_void, row: c_int) -> *mut c_void;
    pub fn wxGridTableBase_GetColLabelValue(self_: *mut c_void, col: c_int) -> *mut c_void;
    pub fn wxGridTableBase_GetCornerLabelValue(self_: *const c_void) -> *mut c_void;
    pub fn wxGridTableBase_SetRowLabelValue(self_: *mut c_void, row: c_int, label: *const c_void);
    pub fn wxGridTableBase_SetColLabelValue(self_: *mut c_void, col: c_int, label: *const c_void);
    // BLOCKED: pub fn wxGridTableBase_SetCornerLabelValue(self_: *mut c_void, None: *const c_void);
    pub fn wxGridTableBase_SetAttrProvider(self_: *mut c_void, attr_provider: *mut c_void);
    pub fn wxGridTableBase_GetAttrProvider(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridTableBase_GetAttr(self_: *mut c_void, row: c_int, col: c_int, kind: wxGridCellAttr::wxAttrKind) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGridTableBase_GetAttrPtr(self_: *mut c_void, row: c_int, col: c_int, kind: wxGridCellAttr::wxAttrKind) -> wxGridCellAttrPtr;
    pub fn wxGridTableBase_SetAttr(self_: *mut c_void, attr: *mut c_void, row: c_int, col: c_int);
    pub fn wxGridTableBase_SetRowAttr(self_: *mut c_void, attr: *mut c_void, row: c_int);
    pub fn wxGridTableBase_SetColAttr(self_: *mut c_void, attr: *mut c_void, col: c_int);
    pub fn wxGridTableBase_CanHaveAttributes(self_: *mut c_void) -> bool;
    pub fn wxGridTableBase_CanMeasureColUsingSameAttr(self_: *const c_void, col: c_int) -> bool;
    // BLOCKED: pub fn wxGridTableBase_new() -> *mut c_void;
    // DTOR: pub fn wxGridTableBase_~wxGridTableBase(self_: *mut c_void);
    pub fn wxGridTableBase_GetNumberRows(self_: *mut c_void) -> c_int;
    pub fn wxGridTableBase_GetNumberCols(self_: *mut c_void) -> c_int;
    pub fn wxGridTableBase_GetRowsCount(self_: *const c_void) -> c_int;
    pub fn wxGridTableBase_GetColsCount(self_: *const c_void) -> c_int;

    // wxGridUpdateLocker
    pub fn wxGridUpdateLocker_delete(self_: *mut c_void);
    pub fn wxGridUpdateLocker_new(grid: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxGridUpdateLocker_~wxGridUpdateLocker(self_: *mut c_void);
    pub fn wxGridUpdateLocker_Create(self_: *mut c_void, grid: *mut c_void);

}
