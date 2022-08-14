use super::*;

extern "C" {

    // wxDC
    pub fn wxDC_CLASSINFO() -> *mut c_void;
    pub fn wxDC_DeviceToLogicalX(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_DeviceToLogicalXRel(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_DeviceToLogicalY(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_DeviceToLogicalYRel(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceX(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceXRel(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceY(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_LogicalToDeviceYRel(self_: *const c_void, y: c_int) -> c_int;
    pub fn wxDC_DeviceToLogical(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_DeviceToLogical1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_DeviceToLogicalRel(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_DeviceToLogicalRel1(self_: *const c_void, dim: *const c_void) -> *mut c_void;
    pub fn wxDC_LogicalToDevice(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_LogicalToDevice1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_LogicalToDeviceRel(self_: *const c_void, x: c_int, y: c_int) -> *mut c_void;
    pub fn wxDC_LogicalToDeviceRel1(self_: *const c_void, dim: *const c_void) -> *mut c_void;
    pub fn wxDC_Clear(self_: *mut c_void);
    pub fn wxDC_DrawArc(
        self_: *mut c_void,
        x_start: c_int,
        y_start: c_int,
        x_end: c_int,
        y_end: c_int,
        xc: c_int,
        yc: c_int,
    );
    pub fn wxDC_DrawArc1(
        self_: *mut c_void,
        pt_start: *const c_void,
        pt_end: *const c_void,
        centre: *const c_void,
    );
    pub fn wxDC_DrawBitmap(
        self_: *mut c_void,
        bitmap: *const c_void,
        x: c_int,
        y: c_int,
        use_mask: bool,
    );
    pub fn wxDC_DrawBitmap1(
        self_: *mut c_void,
        bmp: *const c_void,
        pt: *const c_void,
        use_mask: bool,
    );
    pub fn wxDC_DrawCheckMark(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn wxDC_DrawCheckMark1(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_DrawCircle(self_: *mut c_void, x: c_int, y: c_int, radius: c_int);
    pub fn wxDC_DrawCircle1(self_: *mut c_void, pt: *const c_void, radius: c_int);
    pub fn wxDC_DrawEllipse(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn wxDC_DrawEllipse1(self_: *mut c_void, pt: *const c_void, size: *const c_void);
    pub fn wxDC_DrawEllipse2(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_DrawEllipticArc(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        start: c_double,
        end: c_double,
    );
    pub fn wxDC_DrawEllipticArc1(
        self_: *mut c_void,
        pt: *const c_void,
        sz: *const c_void,
        sa: c_double,
        ea: c_double,
    );
    pub fn wxDC_DrawIcon(self_: *mut c_void, icon: *const c_void, x: c_int, y: c_int);
    pub fn wxDC_DrawIcon1(self_: *mut c_void, icon: *const c_void, pt: *const c_void);
    pub fn wxDC_DrawLabel(
        self_: *mut c_void,
        text: *const c_void,
        bitmap: *const c_void,
        rect: *const c_void,
        alignment: c_int,
        index_accel: c_int,
        rect_bounding: *mut c_void,
    );
    pub fn wxDC_DrawLabel1(
        self_: *mut c_void,
        text: *const c_void,
        rect: *const c_void,
        alignment: c_int,
        index_accel: c_int,
    );
    pub fn wxDC_DrawLine(self_: *mut c_void, x1: c_int, y1: c_int, x2: c_int, y2: c_int);
    pub fn wxDC_DrawLine1(self_: *mut c_void, pt1: *const c_void, pt2: *const c_void);
    // NOT_SUPPORTED: pub fn wxDC_DrawLines(self_: *mut c_void, n: c_int, points: wxPoint, xoffset: c_int, yoffset: c_int);
    pub fn wxDC_DrawLines1(
        self_: *mut c_void,
        points: *const c_void,
        xoffset: c_int,
        yoffset: c_int,
    );
    pub fn wxDC_DrawPoint(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_DrawPoint1(self_: *mut c_void, pt: *const c_void);
    // NOT_SUPPORTED: pub fn wxDC_DrawPolygon(self_: *mut c_void, n: c_int, points: wxPoint, xoffset: c_int, yoffset: c_int, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxDC_DrawPolygon1(self_: *mut c_void, points: *const c_void, xoffset: c_int, yoffset: c_int, fill_style: wxPolygonFillMode);
    // NOT_SUPPORTED: pub fn wxDC_DrawPolyPolygon(self_: *mut c_void, n: c_int, count: c_int, points: wxPoint, xoffset: c_int, yoffset: c_int, fill_style: wxPolygonFillMode);
    pub fn wxDC_DrawRectangle(self_: *mut c_void, x: c_int, y: c_int, width: c_int, height: c_int);
    pub fn wxDC_DrawRectangle1(self_: *mut c_void, pt: *const c_void, sz: *const c_void);
    pub fn wxDC_DrawRectangle2(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_DrawRotatedText(
        self_: *mut c_void,
        text: *const c_void,
        x: c_int,
        y: c_int,
        angle: c_double,
    );
    pub fn wxDC_DrawRotatedText1(
        self_: *mut c_void,
        text: *const c_void,
        point: *const c_void,
        angle: c_double,
    );
    pub fn wxDC_DrawRoundedRectangle(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        radius: c_double,
    );
    pub fn wxDC_DrawRoundedRectangle1(
        self_: *mut c_void,
        pt: *const c_void,
        sz: *const c_void,
        radius: c_double,
    );
    pub fn wxDC_DrawRoundedRectangle2(self_: *mut c_void, rect: *const c_void, radius: c_double);
    // NOT_SUPPORTED: pub fn wxDC_DrawSpline(self_: *mut c_void, n: c_int, points: wxPoint);
    pub fn wxDC_DrawSpline1(self_: *mut c_void, points: *const c_void);
    pub fn wxDC_DrawSpline2(
        self_: *mut c_void,
        x1: c_int,
        y1: c_int,
        x2: c_int,
        y2: c_int,
        x3: c_int,
        y3: c_int,
    );
    pub fn wxDC_DrawText(self_: *mut c_void, text: *const c_void, x: c_int, y: c_int);
    pub fn wxDC_DrawText1(self_: *mut c_void, text: *const c_void, pt: *const c_void);
    pub fn wxDC_GradientFillConcentric(
        self_: *mut c_void,
        rect: *const c_void,
        initial_colour: *const c_void,
        dest_colour: *const c_void,
    );
    pub fn wxDC_GradientFillConcentric1(
        self_: *mut c_void,
        rect: *const c_void,
        initial_colour: *const c_void,
        dest_colour: *const c_void,
        circle_center: *const c_void,
    );
    pub fn wxDC_GradientFillLinear(
        self_: *mut c_void,
        rect: *const c_void,
        initial_colour: *const c_void,
        dest_colour: *const c_void,
        n_direction: c_int,
    );
    // NOT_SUPPORTED: pub fn wxDC_FloodFill(self_: *mut c_void, x: c_int, y: c_int, colour: *const c_void, style: wxFloodFillStyle) -> bool;
    // NOT_SUPPORTED: pub fn wxDC_FloodFill1(self_: *mut c_void, pt: *const c_void, col: *const c_void, style: wxFloodFillStyle) -> bool;
    pub fn wxDC_CrossHair(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_CrossHair1(self_: *mut c_void, pt: *const c_void);
    pub fn wxDC_DestroyClippingRegion(self_: *mut c_void);
    pub fn wxDC_GetClippingBox(
        self_: *const c_void,
        x: *mut c_void,
        y: *mut c_void,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool;
    pub fn wxDC_GetClippingBox1(self_: *const c_void, rect: *mut c_void) -> bool;
    pub fn wxDC_SetClippingRegion(
        self_: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    );
    pub fn wxDC_SetClippingRegion1(self_: *mut c_void, pt: *const c_void, sz: *const c_void);
    pub fn wxDC_SetClippingRegion2(self_: *mut c_void, rect: *const c_void);
    pub fn wxDC_SetDeviceClippingRegion(self_: *mut c_void, region: *const c_void);
    pub fn wxDC_GetCharHeight(self_: *const c_void) -> c_int;
    pub fn wxDC_GetCharWidth(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDC_GetFontMetrics(self_: *const c_void) -> wxFontMetrics;
    pub fn wxDC_GetMultiLineTextExtent(
        self_: *const c_void,
        string: *const c_void,
        w: *mut c_void,
        h: *mut c_void,
        height_line: *mut c_void,
        font: *const c_void,
    );
    pub fn wxDC_GetMultiLineTextExtent1(self_: *const c_void, string: *const c_void)
        -> *mut c_void;
    pub fn wxDC_GetPartialTextExtents(
        self_: *const c_void,
        text: *const c_void,
        widths: *mut c_void,
    ) -> bool;
    pub fn wxDC_GetTextExtent(
        self_: *const c_void,
        string: *const c_void,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: *const c_void,
    );
    pub fn wxDC_GetTextExtent1(self_: *const c_void, string: *const c_void) -> *mut c_void;
    pub fn wxDC_GetBackgroundMode(self_: *const c_void) -> c_int;
    pub fn wxDC_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetLayoutDirection(self_: *const c_void) -> c_int;
    pub fn wxDC_GetTextBackground(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetTextForeground(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_SetBackgroundMode(self_: *mut c_void, mode: c_int);
    pub fn wxDC_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxDC_SetTextBackground(self_: *mut c_void, colour: *const c_void);
    pub fn wxDC_SetTextForeground(self_: *mut c_void, colour: *const c_void);
    pub fn wxDC_SetLayoutDirection(self_: *mut c_void, dir: c_int);
    pub fn wxDC_CalcBoundingBox(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_MaxX(self_: *const c_void) -> c_int;
    pub fn wxDC_MaxY(self_: *const c_void) -> c_int;
    pub fn wxDC_MinX(self_: *const c_void) -> c_int;
    pub fn wxDC_MinY(self_: *const c_void) -> c_int;
    pub fn wxDC_ResetBoundingBox(self_: *mut c_void);
    pub fn wxDC_StartDoc(self_: *mut c_void, message: *const c_void) -> bool;
    pub fn wxDC_StartPage(self_: *mut c_void);
    pub fn wxDC_EndDoc(self_: *mut c_void);
    pub fn wxDC_EndPage(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDC_Blit(self_: *mut c_void, xdest: c_int, ydest: c_int, width: c_int, height: c_int, source: *mut c_void, xsrc: c_int, ysrc: c_int, logical_func: wxRasterOperationMode, use_mask: bool, xsrc_mask: c_int, ysrc_mask: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxDC_StretchBlit(self_: *mut c_void, xdest: c_int, ydest: c_int, dst_width: c_int, dst_height: c_int, source: *mut c_void, xsrc: c_int, ysrc: c_int, src_width: c_int, src_height: c_int, logical_func: wxRasterOperationMode, use_mask: bool, xsrc_mask: c_int, ysrc_mask: c_int) -> bool;
    pub fn wxDC_GetBackground(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetBrush(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDC_GetPen(self_: *const c_void) -> *const c_void;
    pub fn wxDC_SetBackground(self_: *mut c_void, brush: *const c_void);
    pub fn wxDC_SetBrush(self_: *mut c_void, brush: *const c_void);
    pub fn wxDC_SetPen(self_: *mut c_void, pen: *const c_void);
    pub fn wxDC_CopyAttributes(self_: *mut c_void, dc: *const c_void);
    pub fn wxDC_GetContentScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxDC_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxDC_GetDeviceOrigin(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDC_GetLogicalFunction(self_: *const c_void) -> wxRasterOperationMode;
    // NOT_SUPPORTED: pub fn wxDC_GetMapMode(self_: *const c_void) -> wxMappingMode;
    pub fn wxDC_GetPixel(self_: *const c_void, x: c_int, y: c_int, colour: *mut c_void) -> bool;
    pub fn wxDC_GetPPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_FromDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxDC_FromDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_FromDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxDC_ToDIP(self_: *const c_void, sz: *const c_void) -> *mut c_void;
    pub fn wxDC_ToDIP1(self_: *const c_void, pt: *const c_void) -> *mut c_void;
    pub fn wxDC_ToDIP2(self_: *const c_void, d: c_int) -> c_int;
    pub fn wxDC_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxDC_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetSizeMM(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxDC_GetSizeMM1(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetUserScale(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxDC_IsOk(self_: *const c_void) -> bool;
    pub fn wxDC_SetAxisOrientation(self_: *mut c_void, x_left_right: bool, y_bottom_up: bool);
    pub fn wxDC_SetDeviceOrigin(self_: *mut c_void, x: c_int, y: c_int);
    // NOT_SUPPORTED: pub fn wxDC_SetLogicalFunction(self_: *mut c_void, function: wxRasterOperationMode);
    // NOT_SUPPORTED: pub fn wxDC_SetMapMode(self_: *mut c_void, mode: wxMappingMode);
    pub fn wxDC_SetPalette(self_: *mut c_void, palette: *const c_void);
    pub fn wxDC_SetUserScale(self_: *mut c_void, x_scale: c_double, y_scale: c_double);
    pub fn wxDC_CanUseTransformMatrix(self_: *const c_void) -> bool;
    pub fn wxDC_SetTransformMatrix(self_: *mut c_void, matrix: *const c_void) -> bool;
    pub fn wxDC_GetTransformMatrix(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_ResetTransformMatrix(self_: *mut c_void);
    pub fn wxDC_CanDrawBitmap(self_: *const c_void) -> bool;
    pub fn wxDC_CanGetTextExtent(self_: *const c_void) -> bool;
    pub fn wxDC_GetHandle(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetAsBitmap(self_: *const c_void, subrect: *const c_void) -> *mut c_void;
    pub fn wxDC_SetLogicalScale(self_: *mut c_void, x: c_double, y: c_double);
    pub fn wxDC_GetLogicalScale(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxDC_SetLogicalOrigin(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDC_GetLogicalOrigin(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxDC_GetLogicalOrigin1(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_GetGraphicsContext(self_: *const c_void) -> *mut c_void;
    pub fn wxDC_SetGraphicsContext(self_: *mut c_void, ctx: *mut c_void);

    // wxDCBrushChanger
    pub fn wxDCBrushChanger_delete(self_: *mut c_void);
    pub fn wxDCBrushChanger_new(dc: *mut c_void, brush: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDCBrushChanger_~wxDCBrushChanger(self_: *mut c_void);

    // wxDCClipper
    pub fn wxDCClipper_delete(self_: *mut c_void);
    pub fn wxDCClipper_new(dc: *mut c_void, region: *const c_void) -> *mut c_void;
    pub fn wxDCClipper_new1(dc: *mut c_void, rect: *const c_void) -> *mut c_void;
    pub fn wxDCClipper_new2(dc: *mut c_void, x: c_int, y: c_int, w: c_int, h: c_int)
        -> *mut c_void;
    // DTOR: pub fn wxDCClipper_~wxDCClipper(self_: *mut c_void);

    // wxDCFontChanger
    pub fn wxDCFontChanger_delete(self_: *mut c_void);
    pub fn wxDCFontChanger_new(dc: *mut c_void) -> *mut c_void;
    pub fn wxDCFontChanger_new1(dc: *mut c_void, font: *const c_void) -> *mut c_void;
    pub fn wxDCFontChanger_Set(self_: *mut c_void, font: *const c_void);
    // DTOR: pub fn wxDCFontChanger_~wxDCFontChanger(self_: *mut c_void);

    // wxDCOverlay
    pub fn wxDCOverlay_delete(self_: *mut c_void);
    pub fn wxDCOverlay_new(
        overlay: *mut c_void,
        dc: *mut c_void,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
    ) -> *mut c_void;
    pub fn wxDCOverlay_new1(overlay: *mut c_void, dc: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxDCOverlay_~wxDCOverlay(self_: *mut c_void);
    pub fn wxDCOverlay_Clear(self_: *mut c_void);

    // wxDCPenChanger
    pub fn wxDCPenChanger_delete(self_: *mut c_void);
    pub fn wxDCPenChanger_new(dc: *mut c_void, pen: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDCPenChanger_~wxDCPenChanger(self_: *mut c_void);

    // wxDCTextBgColourChanger
    pub fn wxDCTextBgColourChanger_delete(self_: *mut c_void);
    pub fn wxDCTextBgColourChanger_new(dc: *mut c_void) -> *mut c_void;
    pub fn wxDCTextBgColourChanger_new1(dc: *mut c_void, col: *const c_void) -> *mut c_void;
    pub fn wxDCTextBgColourChanger_Set(self_: *mut c_void, col: *const c_void);
    // DTOR: pub fn wxDCTextBgColourChanger_~wxDCTextBgColourChanger(self_: *mut c_void);

    // wxDCTextBgModeChanger
    pub fn wxDCTextBgModeChanger_delete(self_: *mut c_void);

    // wxDCTextColourChanger
    pub fn wxDCTextColourChanger_delete(self_: *mut c_void);
    pub fn wxDCTextColourChanger_new(dc: *mut c_void) -> *mut c_void;
    pub fn wxDCTextColourChanger_new1(dc: *mut c_void, col: *const c_void) -> *mut c_void;
    pub fn wxDCTextColourChanger_Set(self_: *mut c_void, col: *const c_void);
    // DTOR: pub fn wxDCTextColourChanger_~wxDCTextColourChanger(self_: *mut c_void);

    // wxDPIChangedEvent
    pub fn wxDPIChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDPIChangedEvent_GetOldDPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDPIChangedEvent_GetNewDPI(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDPIChangedEvent_Scale(self_: *const c_void, sz: wxSize) -> wxSize;
    pub fn wxDPIChangedEvent_ScaleX(self_: *const c_void, x: c_int) -> c_int;
    pub fn wxDPIChangedEvent_ScaleY(self_: *const c_void, y: c_int) -> c_int;

    // wxDataFormat
    pub fn wxDataFormat_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDataFormat_new(format: wxDataFormatId) -> *mut c_void;
    pub fn wxDataFormat_new1(format: *const c_void) -> *mut c_void;
    pub fn wxDataFormat_GetId(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataFormat_GetType(self_: *const c_void) -> wxDataFormatId;
    pub fn wxDataFormat_SetId(self_: *mut c_void, format: *const c_void);
    // NOT_SUPPORTED: pub fn wxDataFormat_SetType(self_: *mut c_void, type_: wxDataFormatId);
    // BLOCKED: pub fn wxDataFormat_operator!=(self_: *const c_void, format: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDataFormat_operator!=1(self_: *const c_void, format: wxDataFormatId) -> bool;
    // BLOCKED: pub fn wxDataFormat_operator==(self_: *const c_void, format: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDataFormat_operator==1(self_: *const c_void, format: wxDataFormatId) -> bool;

    // wxDataObject
    pub fn wxDataObject_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDataObject_new() -> *mut c_void;
    // DTOR: pub fn wxDataObject_~wxDataObject(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDataObject_GetAllFormats(self_: *const c_void, formats: *mut c_void, dir: Direction);
    pub fn wxDataObject_GetDataHere(
        self_: *const c_void,
        format: *const c_void,
        buf: *mut c_void,
    ) -> bool;
    pub fn wxDataObject_GetDataSize(self_: *const c_void, format: *const c_void) -> usize;
    // NOT_SUPPORTED: pub fn wxDataObject_GetFormatCount(self_: *const c_void, dir: Direction) -> usize;
    // NOT_SUPPORTED: pub fn wxDataObject_GetPreferredFormat(self_: *const c_void, dir: Direction) -> *mut c_void;
    pub fn wxDataObject_SetData(
        self_: *mut c_void,
        format: *const c_void,
        len: usize,
        buf: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxDataObject_IsSupported(self_: *const c_void, format: *const c_void, dir: Direction) -> bool;

    // wxDataObjectComposite
    pub fn wxDataObjectComposite_delete(self_: *mut c_void);
    pub fn wxDataObjectComposite_new() -> *mut c_void;
    pub fn wxDataObjectComposite_Add(self_: *mut c_void, data_object: *mut c_void, preferred: bool);
    pub fn wxDataObjectComposite_GetReceivedFormat(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataObjectComposite_GetObject(self_: *const c_void, format: *const c_void, dir: wxDataObject::Direction) -> *mut c_void;

    // wxDataObjectSimple
    pub fn wxDataObjectSimple_delete(self_: *mut c_void);
    pub fn wxDataObjectSimple_new(format: *const c_void) -> *mut c_void;
    pub fn wxDataObjectSimple_GetDataHere(self_: *const c_void, buf: *mut c_void) -> bool;
    pub fn wxDataObjectSimple_GetDataSize(self_: *const c_void) -> usize;
    // BLOCKED: pub fn wxDataObjectSimple_GetFormat(self_: *const c_void) -> *mut c_void;
    pub fn wxDataObjectSimple_SetData(self_: *mut c_void, len: usize, buf: *const c_void) -> bool;
    pub fn wxDataObjectSimple_SetFormat(self_: *mut c_void, format: *const c_void);

    // wxDataViewBitmapRenderer
    pub fn wxDataViewBitmapRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewBitmapRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewBitmapRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;

    // wxDataViewCheckIconTextRenderer
    pub fn wxDataViewCheckIconTextRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewCheckIconTextRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCheckIconTextRenderer_new(mode: wxDataViewCellMode, align: c_int) -> *mut c_void;
    pub fn wxDataViewCheckIconTextRenderer_Allow3rdStateForUser(self_: *mut c_void, allow: bool);

    // wxDataViewChoiceByIndexRenderer
    pub fn wxDataViewChoiceByIndexRenderer_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewChoiceByIndexRenderer_new(choices: *const c_void, mode: wxDataViewCellMode, alignment: c_int) -> *mut c_void;

    // wxDataViewChoiceRenderer
    pub fn wxDataViewChoiceRenderer_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewChoiceRenderer_new(choices: *const c_void, mode: wxDataViewCellMode, alignment: c_int) -> *mut c_void;
    pub fn wxDataViewChoiceRenderer_GetChoice(self_: *const c_void, index: usize) -> *mut c_void;
    pub fn wxDataViewChoiceRenderer_GetChoices(self_: *const c_void) -> *mut c_void;

    // wxDataViewColumn
    pub fn wxDataViewColumn_delete(self_: *mut c_void);
    pub fn wxDataViewColumn_new(
        title: *const c_void,
        renderer: *mut c_void,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxDataViewColumn_new1(
        bitmap: *const c_void,
        renderer: *mut c_void,
        model_column: c_uint,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxDataViewColumn_GetModelColumn(self_: *const c_void) -> c_uint;
    pub fn wxDataViewColumn_GetOwner(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewColumn_GetRenderer(self_: *const c_void) -> *mut c_void;

    // wxDataViewCtrl
    pub fn wxDataViewCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewCtrl_new() -> *mut c_void;
    pub fn wxDataViewCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDataViewCtrl_~wxDataViewCtrl(self_: *mut c_void);
    pub fn wxDataViewCtrl_AllowMultiColumnSort(self_: *mut c_void, allow: bool) -> bool;
    pub fn wxDataViewCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDataViewCtrl_AppendColumn(self_: *mut c_void, col: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_PrependColumn(self_: *mut c_void, col: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_InsertColumn(self_: *mut c_void, pos: c_uint, col: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendBitmapColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendBitmapColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependBitmapColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependBitmapColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendDateColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendDateColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependDateColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependDateColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendIconTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendIconTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependIconTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependIconTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendProgressColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendProgressColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependProgressColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependProgressColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependTextColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependTextColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendToggleColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_AppendToggleColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependToggleColumn(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCtrl_PrependToggleColumn1(self_: *mut c_void, label: *const c_void, model_column: c_uint, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    pub fn wxDataViewCtrl_AssociateModel(self_: *mut c_void, model: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_ClearColumns(self_: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_Collapse(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_DeleteColumn(self_: *mut c_void, column: *mut c_void) -> bool;
    pub fn wxDataViewCtrl_EditItem(self_: *mut c_void, item: *const c_void, column: *const c_void);
    pub fn wxDataViewCtrl_EnableDragSource(self_: *mut c_void, format: *const c_void) -> bool;
    pub fn wxDataViewCtrl_EnableDropTargets(self_: *mut c_void, formats: *const c_void) -> bool;
    pub fn wxDataViewCtrl_EnableDropTarget(self_: *mut c_void, format: *const c_void) -> bool;
    pub fn wxDataViewCtrl_EnsureVisible(
        self_: *mut c_void,
        item: *const c_void,
        column: *const c_void,
    );
    pub fn wxDataViewCtrl_Expand(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_ExpandAncestors(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_ExpandChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_GetColumn(self_: *const c_void, pos: c_uint) -> *mut c_void;
    pub fn wxDataViewCtrl_GetColumnCount(self_: *const c_void) -> c_uint;
    pub fn wxDataViewCtrl_GetColumnPosition(self_: *const c_void, column: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetExpanderColumn(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetCurrentItem(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetCurrentColumn(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetIndent(self_: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetItemRect(
        self_: *const c_void,
        item: *const c_void,
        col: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewCtrl_GetMainWindow(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetModel(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetSelectedItemsCount(self_: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_GetSelections(self_: *const c_void, sel: *mut c_void) -> c_int;
    pub fn wxDataViewCtrl_GetSortingColumn(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDataViewCtrl_GetSortingColumns(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCtrl_HasSelection(self_: *const c_void) -> bool;
    pub fn wxDataViewCtrl_HitTest(
        self_: *const c_void,
        point: *const c_void,
        item: *mut c_void,
        col: *mut c_void,
    );
    pub fn wxDataViewCtrl_IsExpanded(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxDataViewCtrl_IsMultiColumnSortAllowed(self_: *const c_void) -> bool;
    pub fn wxDataViewCtrl_IsSelected(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxDataViewCtrl_Select(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_SelectAll(self_: *mut c_void);
    pub fn wxDataViewCtrl_SetAlternateRowColour(self_: *mut c_void, colour: *const c_void) -> bool;
    pub fn wxDataViewCtrl_SetExpanderColumn(self_: *mut c_void, col: *mut c_void);
    pub fn wxDataViewCtrl_SetCurrentItem(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_SetHeaderAttr(self_: *mut c_void, attr: *const c_void) -> bool;
    pub fn wxDataViewCtrl_SetIndent(self_: *mut c_void, indent: c_int);
    pub fn wxDataViewCtrl_SetSelections(self_: *mut c_void, sel: *const c_void);
    pub fn wxDataViewCtrl_Unselect(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewCtrl_UnselectAll(self_: *mut c_void);
    pub fn wxDataViewCtrl_SetRowHeight(self_: *mut c_void, row_height: c_int) -> bool;
    pub fn wxDataViewCtrl_ToggleSortByColumn(self_: *mut c_void, column: c_int);
    pub fn wxDataViewCtrl_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxDataViewCtrl_GetTopItem(self_: *const c_void) -> *mut c_void;

    // wxDataViewCustomRenderer
    pub fn wxDataViewCustomRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewCustomRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewCustomRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;
    // DTOR: pub fn wxDataViewCustomRenderer_~wxDataViewCustomRenderer(self_: *mut c_void);
    pub fn wxDataViewCustomRenderer_ActivateCell(
        self_: *mut c_void,
        cell: *const c_void,
        model: *mut c_void,
        item: *const c_void,
        col: c_uint,
        mouse_event: *const c_void,
    ) -> bool;
    pub fn wxDataViewCustomRenderer_GetAttr(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewCustomRenderer_GetSize(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDataViewCustomRenderer_LeftClick(self_: *mut c_void, cursor: wxPoint, cell: wxRect, model: *mut c_void, item: *const c_void, col: c_uint) -> bool;
    // BLOCKED: pub fn wxDataViewCustomRenderer_Activate(self_: *mut c_void, cell: wxRect, model: *mut c_void, item: *const c_void, col: c_uint) -> bool;
    // BLOCKED: pub fn wxDataViewCustomRenderer_Render(self_: *mut c_void, cell: wxRect, dc: *mut c_void, state: c_int) -> bool;
    // BLOCKED: pub fn wxDataViewCustomRenderer_RenderText(self_: *mut c_void, text: *const c_void, xoffset: c_int, cell: wxRect, dc: *mut c_void, state: c_int);
    pub fn wxDataViewCustomRenderer_StartDrag(
        self_: *mut c_void,
        cursor: *const c_void,
        cell: *const c_void,
        model: *mut c_void,
        item: *const c_void,
        col: c_uint,
    ) -> bool;

    // wxDataViewDateRenderer
    pub fn wxDataViewDateRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewDateRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewDateRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;

    // wxDataViewEvent
    pub fn wxDataViewEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewEvent_new1(evt_type: wxEventType, dvc: *mut c_void, column: *mut c_void, item: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewEvent_new2(evt_type: wxEventType, dvc: *mut c_void, item: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_new3(event: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_GetColumn(self_: *const c_void) -> c_int;
    pub fn wxDataViewEvent_GetDataViewColumn(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_GetModel(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDataViewEvent_GetValue(self_: *const c_void) -> *const c_void;
    pub fn wxDataViewEvent_IsEditCancelled(self_: *const c_void) -> bool;
    pub fn wxDataViewEvent_SetColumn(self_: *mut c_void, col: c_int);
    // BLOCKED: pub fn wxDataViewEvent_SetDataViewColumn(self_: *mut c_void, col: *mut c_void);
    // BLOCKED: pub fn wxDataViewEvent_SetModel(self_: *mut c_void, model: *mut c_void);
    pub fn wxDataViewEvent_SetValue(self_: *mut c_void, value: *const c_void);
    pub fn wxDataViewEvent_SetDataObject(self_: *mut c_void, obj: *mut c_void);
    pub fn wxDataViewEvent_GetDataFormat(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_GetDataSize(self_: *const c_void) -> usize;
    pub fn wxDataViewEvent_GetDataBuffer(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_SetDragFlags(self_: *mut c_void, flags: c_int);
    // NOT_SUPPORTED: pub fn wxDataViewEvent_GetDropEffect(self_: *const c_void) -> wxDragResult;
    pub fn wxDataViewEvent_GetCacheFrom(self_: *const c_void) -> c_int;
    pub fn wxDataViewEvent_GetCacheTo(self_: *const c_void) -> c_int;
    pub fn wxDataViewEvent_GetProposedDropIndex(self_: *const c_void) -> c_int;
    pub fn wxDataViewEvent_GetItem(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDataViewEvent_SetItem(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewEvent_SetPosition(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxDataViewEvent_SetCache(self_: *mut c_void, from: c_int, to: c_int);
    pub fn wxDataViewEvent_GetDataObject(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewEvent_SetDataFormat(self_: *mut c_void, format: *const c_void);
    pub fn wxDataViewEvent_SetDataSize(self_: *mut c_void, size: usize);
    pub fn wxDataViewEvent_SetDataBuffer(self_: *mut c_void, buf: *mut c_void);
    pub fn wxDataViewEvent_GetDragFlags(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDataViewEvent_SetDropEffect(self_: *mut c_void, effect: wxDragResult);

    // wxDataViewIconText
    pub fn wxDataViewIconText_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewIconText_new(text: *const c_void, bitmap: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_new1(other: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_GetBitmapBundle(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_GetIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewIconText_SetBitmapBundle(self_: *mut c_void, bitmap: *const c_void);
    pub fn wxDataViewIconText_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxDataViewIconText_SetText(self_: *mut c_void, text: *const c_void);

    // wxDataViewIconTextRenderer
    pub fn wxDataViewIconTextRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewIconTextRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewIconTextRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;

    // wxDataViewIndexListModel
    pub fn wxDataViewIndexListModel_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDataViewIndexListModel_new(initial_size: c_uint) -> *mut c_void;
    pub fn wxDataViewIndexListModel_GetItem(self_: *const c_void, row: c_uint) -> *mut c_void;
    pub fn wxDataViewIndexListModel_Reset(self_: *mut c_void, new_size: c_uint);
    pub fn wxDataViewIndexListModel_RowAppended(self_: *mut c_void);
    pub fn wxDataViewIndexListModel_RowChanged(self_: *mut c_void, row: c_uint);
    pub fn wxDataViewIndexListModel_RowDeleted(self_: *mut c_void, row: c_uint);
    pub fn wxDataViewIndexListModel_RowInserted(self_: *mut c_void, before: c_uint);
    pub fn wxDataViewIndexListModel_RowPrepended(self_: *mut c_void);
    pub fn wxDataViewIndexListModel_RowValueChanged(self_: *mut c_void, row: c_uint, col: c_uint);
    pub fn wxDataViewIndexListModel_RowsDeleted(self_: *mut c_void, rows: *const c_void);

    // wxDataViewItem
    pub fn wxDataViewItem_delete(self_: *mut c_void);
    pub fn wxDataViewItem_new() -> *mut c_void;
    pub fn wxDataViewItem_new1(item: *const c_void) -> *mut c_void;
    pub fn wxDataViewItem_new2(id: *mut c_void) -> *mut c_void;
    pub fn wxDataViewItem_GetID(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewItem_IsOk(self_: *const c_void) -> bool;

    // wxDataViewItemAttr
    pub fn wxDataViewItemAttr_delete(self_: *mut c_void);
    pub fn wxDataViewItemAttr_new() -> *mut c_void;
    pub fn wxDataViewItemAttr_SetBold(self_: *mut c_void, set: bool);
    pub fn wxDataViewItemAttr_SetColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxDataViewItemAttr_SetBackgroundColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxDataViewItemAttr_SetItalic(self_: *mut c_void, set: bool);
    pub fn wxDataViewItemAttr_SetStrikethrough(self_: *mut c_void, set: bool);
    pub fn wxDataViewItemAttr_HasColour(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewItemAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetBold(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetItalic(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewItemAttr_IsDefault(self_: *const c_void) -> bool;
    pub fn wxDataViewItemAttr_GetEffectiveFont(
        self_: *const c_void,
        font: *const c_void,
    ) -> *mut c_void;

    // wxDataViewListCtrl
    pub fn wxDataViewListCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewListCtrl_GetSelectedRow(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_SelectRow(self_: *mut c_void, row: unsigned);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_UnselectRow(self_: *mut c_void, row: unsigned);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_IsRowSelected(self_: *const c_void, row: unsigned) -> bool;
    // BLOCKED: pub fn wxDataViewListCtrl_AppendColumn(self_: *mut c_void, column: *mut c_void, varianttype: *const c_void);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_AppendTextColumn(self_: *mut c_void, label: *const c_void, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_AppendToggleColumn(self_: *mut c_void, label: *const c_void, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_AppendProgressColumn(self_: *mut c_void, label: *const c_void, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_AppendIconTextColumn(self_: *mut c_void, label: *const c_void, mode: wxDataViewCellMode, width: c_int, align: c_int, flags: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxDataViewListCtrl_InsertColumn(self_: *mut c_void, pos: c_uint, column: *mut c_void, varianttype: *const c_void);
    // BLOCKED: pub fn wxDataViewListCtrl_PrependColumn(self_: *mut c_void, column: *mut c_void, varianttype: *const c_void);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_AppendItem(self_: *mut c_void, values: *const c_void, data: wxUIntPtr);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_PrependItem(self_: *mut c_void, values: *const c_void, data: wxUIntPtr);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_InsertItem(self_: *mut c_void, row: c_uint, values: *const c_void, data: wxUIntPtr);
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_DeleteItem(self_: *mut c_void, row: unsigned);
    pub fn wxDataViewListCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxDataViewListCtrl_GetItemCount(self_: *const c_void) -> c_uint;
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_GetItemData(self_: *const c_void, item: *const c_void) -> wxUIntPtr;
    pub fn wxDataViewListCtrl_SetValue(
        self_: *mut c_void,
        value: *const c_void,
        row: c_uint,
        col: c_uint,
    );
    pub fn wxDataViewListCtrl_GetValue(
        self_: *mut c_void,
        value: *mut c_void,
        row: c_uint,
        col: c_uint,
    );
    pub fn wxDataViewListCtrl_SetTextValue(
        self_: *mut c_void,
        value: *const c_void,
        row: c_uint,
        col: c_uint,
    );
    pub fn wxDataViewListCtrl_GetTextValue(
        self_: *const c_void,
        row: c_uint,
        col: c_uint,
    ) -> *mut c_void;
    pub fn wxDataViewListCtrl_SetToggleValue(
        self_: *mut c_void,
        value: bool,
        row: c_uint,
        col: c_uint,
    );
    pub fn wxDataViewListCtrl_GetToggleValue(
        self_: *const c_void,
        row: c_uint,
        col: c_uint,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxDataViewListCtrl_SetItemData(self_: *mut c_void, item: *const c_void, data: wxUIntPtr);
    pub fn wxDataViewListCtrl_new() -> *mut c_void;
    pub fn wxDataViewListCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDataViewListCtrl_~wxDataViewListCtrl(self_: *mut c_void);
    pub fn wxDataViewListCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
    ) -> bool;
    // BLOCKED: pub fn wxDataViewListCtrl_GetStore(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewListCtrl_GetStore1(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewListCtrl_ItemToRow(self_: *const c_void, item: *const c_void) -> c_int;
    pub fn wxDataViewListCtrl_RowToItem(self_: *const c_void, row: c_int) -> *mut c_void;

    // wxDataViewListModel
    pub fn wxDataViewListModel_delete(self_: *mut c_void);
    // DTOR: pub fn wxDataViewListModel_~wxDataViewListModel(self_: *mut c_void);
    pub fn wxDataViewListModel_GetAttrByRow(
        self_: *const c_void,
        row: c_uint,
        col: c_uint,
        attr: *mut c_void,
    ) -> bool;
    pub fn wxDataViewListModel_IsEnabledByRow(
        self_: *const c_void,
        row: c_uint,
        col: c_uint,
    ) -> bool;
    pub fn wxDataViewListModel_GetCount(self_: *const c_void) -> c_uint;
    pub fn wxDataViewListModel_GetRow(self_: *const c_void, item: *const c_void) -> c_uint;
    pub fn wxDataViewListModel_GetValueByRow(
        self_: *const c_void,
        variant: *mut c_void,
        row: c_uint,
        col: c_uint,
    );
    pub fn wxDataViewListModel_SetValueByRow(
        self_: *mut c_void,
        variant: *const c_void,
        row: c_uint,
        col: c_uint,
    ) -> bool;

    // wxDataViewListStore
    pub fn wxDataViewListStore_delete(self_: *mut c_void);
    pub fn wxDataViewListStore_new() -> *mut c_void;
    // DTOR: pub fn wxDataViewListStore_~wxDataViewListStore(self_: *mut c_void);
    pub fn wxDataViewListStore_PrependColumn(self_: *mut c_void, varianttype: *const c_void);
    pub fn wxDataViewListStore_InsertColumn(
        self_: *mut c_void,
        pos: c_uint,
        varianttype: *const c_void,
    );
    pub fn wxDataViewListStore_AppendColumn(self_: *mut c_void, varianttype: *const c_void);
    // NOT_SUPPORTED: pub fn wxDataViewListStore_AppendItem(self_: *mut c_void, values: *const c_void, data: wxUIntPtr);
    // NOT_SUPPORTED: pub fn wxDataViewListStore_PrependItem(self_: *mut c_void, values: *const c_void, data: wxUIntPtr);
    // NOT_SUPPORTED: pub fn wxDataViewListStore_InsertItem(self_: *mut c_void, row: c_uint, values: *const c_void, data: wxUIntPtr);
    // NOT_SUPPORTED: pub fn wxDataViewListStore_DeleteItem(self_: *mut c_void, pos: unsigned);
    pub fn wxDataViewListStore_DeleteAllItems(self_: *mut c_void);
    pub fn wxDataViewListStore_GetItemCount(self_: *const c_void) -> c_uint;
    // NOT_SUPPORTED: pub fn wxDataViewListStore_GetItemData(self_: *const c_void, item: *const c_void) -> wxUIntPtr;
    // NOT_SUPPORTED: pub fn wxDataViewListStore_SetItemData(self_: *mut c_void, item: *const c_void, data: wxUIntPtr);

    // wxDataViewModel
    pub fn wxDataViewModel_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDataViewModel_new() -> *mut c_void;
    pub fn wxDataViewModel_AddNotifier(self_: *mut c_void, notifier: *mut c_void);
    pub fn wxDataViewModel_ChangeValue(
        self_: *mut c_void,
        variant: *const c_void,
        item: *const c_void,
        col: c_uint,
    ) -> bool;
    pub fn wxDataViewModel_Cleared(self_: *mut c_void) -> bool;
    pub fn wxDataViewModel_Compare(
        self_: *const c_void,
        item1: *const c_void,
        item2: *const c_void,
        column: c_uint,
        ascending: bool,
    ) -> c_int;
    pub fn wxDataViewModel_GetAttr(
        self_: *const c_void,
        item: *const c_void,
        col: c_uint,
        attr: *mut c_void,
    ) -> bool;
    pub fn wxDataViewModel_IsEnabled(
        self_: *const c_void,
        item: *const c_void,
        col: c_uint,
    ) -> bool;
    pub fn wxDataViewModel_GetChildren(
        self_: *const c_void,
        item: *const c_void,
        children: *mut c_void,
    ) -> c_uint;
    pub fn wxDataViewModel_GetParent(self_: *const c_void, item: *const c_void) -> *mut c_void;
    pub fn wxDataViewModel_GetValue(
        self_: *const c_void,
        variant: *mut c_void,
        item: *const c_void,
        col: c_uint,
    );
    pub fn wxDataViewModel_HasContainerColumns(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxDataViewModel_HasDefaultCompare(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDataViewModel_HasValue(self_: *const c_void, item: *const c_void, col: unsigned) -> bool;
    pub fn wxDataViewModel_IsContainer(self_: *const c_void, item: *const c_void) -> bool;
    pub fn wxDataViewModel_ItemAdded(
        self_: *mut c_void,
        parent: *const c_void,
        item: *const c_void,
    ) -> bool;
    pub fn wxDataViewModel_ItemChanged(self_: *mut c_void, item: *const c_void) -> bool;
    pub fn wxDataViewModel_ItemDeleted(
        self_: *mut c_void,
        parent: *const c_void,
        item: *const c_void,
    ) -> bool;
    pub fn wxDataViewModel_ItemsAdded(
        self_: *mut c_void,
        parent: *const c_void,
        items: *const c_void,
    ) -> bool;
    pub fn wxDataViewModel_ItemsChanged(self_: *mut c_void, items: *const c_void) -> bool;
    pub fn wxDataViewModel_ItemsDeleted(
        self_: *mut c_void,
        parent: *const c_void,
        items: *const c_void,
    ) -> bool;
    pub fn wxDataViewModel_RemoveNotifier(self_: *mut c_void, notifier: *mut c_void);
    pub fn wxDataViewModel_Resort(self_: *mut c_void);
    pub fn wxDataViewModel_SetValue(
        self_: *mut c_void,
        variant: *const c_void,
        item: *const c_void,
        col: c_uint,
    ) -> bool;
    pub fn wxDataViewModel_ValueChanged(
        self_: *mut c_void,
        item: *const c_void,
        col: c_uint,
    ) -> bool;
    pub fn wxDataViewModel_IsListModel(self_: *const c_void) -> bool;
    pub fn wxDataViewModel_IsVirtualListModel(self_: *const c_void) -> bool;

    // wxDataViewModelNotifier
    pub fn wxDataViewModelNotifier_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDataViewModelNotifier_new() -> *mut c_void;
    // DTOR: pub fn wxDataViewModelNotifier_~wxDataViewModelNotifier(self_: *mut c_void);
    pub fn wxDataViewModelNotifier_Cleared(self_: *mut c_void) -> bool;
    pub fn wxDataViewModelNotifier_GetOwner(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewModelNotifier_ItemAdded(
        self_: *mut c_void,
        parent: *const c_void,
        item: *const c_void,
    ) -> bool;
    pub fn wxDataViewModelNotifier_ItemChanged(self_: *mut c_void, item: *const c_void) -> bool;
    pub fn wxDataViewModelNotifier_ItemDeleted(
        self_: *mut c_void,
        parent: *const c_void,
        item: *const c_void,
    ) -> bool;
    pub fn wxDataViewModelNotifier_ItemsAdded(
        self_: *mut c_void,
        parent: *const c_void,
        items: *const c_void,
    ) -> bool;
    pub fn wxDataViewModelNotifier_ItemsChanged(self_: *mut c_void, items: *const c_void) -> bool;
    pub fn wxDataViewModelNotifier_ItemsDeleted(
        self_: *mut c_void,
        parent: *const c_void,
        items: *const c_void,
    ) -> bool;
    pub fn wxDataViewModelNotifier_Resort(self_: *mut c_void);
    pub fn wxDataViewModelNotifier_SetOwner(self_: *mut c_void, owner: *mut c_void);
    pub fn wxDataViewModelNotifier_ValueChanged(
        self_: *mut c_void,
        item: *const c_void,
        col: c_uint,
    ) -> bool;

    // wxDataViewProgressRenderer
    pub fn wxDataViewProgressRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewProgressRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewProgressRenderer_new(label: *const c_void, varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;

    // wxDataViewRenderer
    pub fn wxDataViewRenderer_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;
    pub fn wxDataViewRenderer_EnableEllipsize(self_: *mut c_void, mode: c_int);
    pub fn wxDataViewRenderer_DisableEllipsize(self_: *mut c_void);
    // BLOCKED: pub fn wxDataViewRenderer_GetAccessibleDescription(self_: *const c_void) -> wxString;
    pub fn wxDataViewRenderer_GetAlignment(self_: *const c_void) -> c_int;
    pub fn wxDataViewRenderer_GetEllipsizeMode(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDataViewRenderer_GetMode(self_: *const c_void) -> wxDataViewCellMode;
    pub fn wxDataViewRenderer_GetOwner(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewRenderer_GetValue(self_: *const c_void, value: *mut c_void) -> bool;
    pub fn wxDataViewRenderer_GetVariantType(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewRenderer_IsCompatibleVariantType(
        self_: *const c_void,
        variant_type: *const c_void,
    ) -> bool;
    pub fn wxDataViewRenderer_SetAlignment(self_: *mut c_void, align: c_int);
    pub fn wxDataViewRenderer_SetOwner(self_: *mut c_void, owner: *mut c_void);
    pub fn wxDataViewRenderer_SetValue(self_: *mut c_void, value: *const c_void) -> bool;
    pub fn wxDataViewRenderer_SetValueAdjuster(self_: *mut c_void, transformer: *mut c_void);
    pub fn wxDataViewRenderer_Validate(self_: *mut c_void, value: *mut c_void) -> bool;
    pub fn wxDataViewRenderer_HasEditorCtrl(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxDataViewRenderer_CreateEditorCtrl(self_: *mut c_void, parent: *mut c_void, label_rect: wxRect, value: *const c_void) -> *mut c_void;
    pub fn wxDataViewRenderer_GetValueFromEditorCtrl(
        self_: *mut c_void,
        editor: *mut c_void,
        value: *mut c_void,
    ) -> bool;
    // BLOCKED: pub fn wxDataViewRenderer_StartEditing(self_: *mut c_void, item: *const c_void, label_rect: wxRect) -> bool;
    pub fn wxDataViewRenderer_CancelEditing(self_: *mut c_void);
    pub fn wxDataViewRenderer_FinishEditing(self_: *mut c_void) -> bool;
    pub fn wxDataViewRenderer_GetEditorCtrl(self_: *mut c_void) -> *mut c_void;

    // wxDataViewSpinRenderer
    pub fn wxDataViewSpinRenderer_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewSpinRenderer_new(min: c_int, max: c_int, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;

    // wxDataViewTextRenderer
    pub fn wxDataViewTextRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewTextRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewTextRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;
    pub fn wxDataViewTextRenderer_EnableMarkup(self_: *mut c_void, enable: bool);

    // wxDataViewToggleRenderer
    pub fn wxDataViewToggleRenderer_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewToggleRenderer_GetDefaultType() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataViewToggleRenderer_new(varianttype: *const c_void, mode: wxDataViewCellMode, align: c_int) -> *mut c_void;
    pub fn wxDataViewToggleRenderer_ShowAsRadio(self_: *mut c_void);

    // wxDataViewTreeCtrl
    pub fn wxDataViewTreeCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDataViewTreeCtrl_new() -> *mut c_void;
    pub fn wxDataViewTreeCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDataViewTreeCtrl_~wxDataViewTreeCtrl(self_: *mut c_void);
    pub fn wxDataViewTreeCtrl_AppendContainer(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_AppendItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
    ) -> bool;
    pub fn wxDataViewTreeCtrl_DeleteAllItems(self_: *mut c_void);
    pub fn wxDataViewTreeCtrl_DeleteChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewTreeCtrl_DeleteItem(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewTreeCtrl_GetChildCount(self_: *const c_void, parent: *const c_void) -> c_int;
    pub fn wxDataViewTreeCtrl_GetImageList(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemData(self_: *const c_void, item: *const c_void)
        -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemExpandedIcon(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetItemIcon(self_: *const c_void, item: *const c_void)
        -> *mut c_void;
    // BLOCKED: pub fn wxDataViewTreeCtrl_GetItemParent(self_: *const c_void, item: wxDataViewItem) -> wxDataViewItem;
    pub fn wxDataViewTreeCtrl_GetItemText(self_: *const c_void, item: *const c_void)
        -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetNthChild(
        self_: *const c_void,
        parent: *const c_void,
        pos: c_uint,
    ) -> *mut c_void;
    // BLOCKED: pub fn wxDataViewTreeCtrl_GetStore(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_GetStore1(self_: *const c_void) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_InsertContainer(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_InsertItem(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        icon: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_IsContainer(self_: *mut c_void, item: *const c_void) -> bool;
    pub fn wxDataViewTreeCtrl_PrependContainer(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_PrependItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: c_int,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeCtrl_SetImageList(self_: *mut c_void, imagelist: *mut c_void);
    pub fn wxDataViewTreeCtrl_SetItemData(
        self_: *mut c_void,
        item: *const c_void,
        data: *mut c_void,
    );
    pub fn wxDataViewTreeCtrl_SetItemExpandedIcon(
        self_: *mut c_void,
        item: *const c_void,
        icon: *const c_void,
    );
    pub fn wxDataViewTreeCtrl_SetItemIcon(
        self_: *mut c_void,
        item: *const c_void,
        icon: *const c_void,
    );
    pub fn wxDataViewTreeCtrl_SetItemText(
        self_: *mut c_void,
        item: *const c_void,
        text: *const c_void,
    );

    // wxDataViewTreeStore
    pub fn wxDataViewTreeStore_delete(self_: *mut c_void);
    pub fn wxDataViewTreeStore_new() -> *mut c_void;
    // DTOR: pub fn wxDataViewTreeStore_~wxDataViewTreeStore(self_: *mut c_void);
    pub fn wxDataViewTreeStore_AppendContainer(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: *const c_void,
        expanded: *const c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_AppendItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: *const c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_DeleteAllItems(self_: *mut c_void);
    pub fn wxDataViewTreeStore_DeleteChildren(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewTreeStore_DeleteItem(self_: *mut c_void, item: *const c_void);
    pub fn wxDataViewTreeStore_GetChildCount(self_: *const c_void, parent: *const c_void) -> c_int;
    pub fn wxDataViewTreeStore_GetItemData(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_GetItemExpandedIcon(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_GetItemIcon(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_GetItemText(
        self_: *const c_void,
        item: *const c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_GetNthChild(
        self_: *const c_void,
        parent: *const c_void,
        pos: c_uint,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_InsertContainer(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        icon: *const c_void,
        expanded: *const c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_InsertItem(
        self_: *mut c_void,
        parent: *const c_void,
        previous: *const c_void,
        text: *const c_void,
        icon: *const c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_PrependContainer(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: *const c_void,
        expanded: *const c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_PrependItem(
        self_: *mut c_void,
        parent: *const c_void,
        text: *const c_void,
        icon: *const c_void,
        data: *mut c_void,
    ) -> *mut c_void;
    pub fn wxDataViewTreeStore_SetItemData(
        self_: *mut c_void,
        item: *const c_void,
        data: *mut c_void,
    );
    pub fn wxDataViewTreeStore_SetItemExpandedIcon(
        self_: *mut c_void,
        item: *const c_void,
        icon: *const c_void,
    );
    pub fn wxDataViewTreeStore_SetItemIcon(
        self_: *mut c_void,
        item: *const c_void,
        icon: *const c_void,
    );

    // wxDataViewValueAdjuster
    pub fn wxDataViewValueAdjuster_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDataViewValueAdjuster_MakeHighlighted(self_: *const c_void, value: *const c_void) -> wxVariant;

    // wxDataViewVirtualListModel
    pub fn wxDataViewVirtualListModel_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDataViewVirtualListModel_new(initial_size: c_uint) -> *mut c_void;
    pub fn wxDataViewVirtualListModel_GetItem(self_: *const c_void, row: c_uint) -> *mut c_void;
    pub fn wxDataViewVirtualListModel_Reset(self_: *mut c_void, new_size: c_uint);
    pub fn wxDataViewVirtualListModel_RowAppended(self_: *mut c_void);
    pub fn wxDataViewVirtualListModel_RowChanged(self_: *mut c_void, row: c_uint);
    pub fn wxDataViewVirtualListModel_RowDeleted(self_: *mut c_void, row: c_uint);
    pub fn wxDataViewVirtualListModel_RowInserted(self_: *mut c_void, before: c_uint);
    pub fn wxDataViewVirtualListModel_RowPrepended(self_: *mut c_void);
    pub fn wxDataViewVirtualListModel_RowValueChanged(self_: *mut c_void, row: c_uint, col: c_uint);
    pub fn wxDataViewVirtualListModel_RowsDeleted(self_: *mut c_void, rows: *const c_void);

    // wxDateEvent
    pub fn wxDateEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDateEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateEvent_new1(win: *mut c_void, dt: *const c_void, type_: wxEventType) -> *mut c_void;
    pub fn wxDateEvent_GetDate(self_: *const c_void) -> *mut c_void;
    pub fn wxDateEvent_SetDate(self_: *mut c_void, date: *const c_void);

    // wxDatePickerCtrl
    pub fn wxDatePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDatePickerCtrl_new() -> *mut c_void;
    pub fn wxDatePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxDatePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDatePickerCtrl_GetRange(
        self_: *const c_void,
        dt1: *mut c_void,
        dt2: *mut c_void,
    ) -> bool;
    pub fn wxDatePickerCtrl_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxDatePickerCtrl_SetNullText(self_: *mut c_void, text: *const c_void);
    pub fn wxDatePickerCtrl_SetRange(self_: *mut c_void, dt1: *const c_void, dt2: *const c_void);
    pub fn wxDatePickerCtrl_SetValue(self_: *mut c_void, dt: *const c_void);

    // wxDelegateRendererNative
    pub fn wxDelegateRendererNative_delete(self_: *mut c_void);
    pub fn wxDelegateRendererNative_new() -> *mut c_void;
    pub fn wxDelegateRendererNative_new1(renderer_native: *mut c_void) -> *mut c_void;

    // wxDialog
    pub fn wxDialog_CLASSINFO() -> *mut c_void;
    pub fn wxDialog_new() -> *mut c_void;
    pub fn wxDialog_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDialog_~wxDialog(self_: *mut c_void);
    pub fn wxDialog_AddMainButtonId(self_: *mut c_void, id: c_int);
    pub fn wxDialog_CanDoLayoutAdaptation(self_: *mut c_void) -> bool;
    pub fn wxDialog_Centre(self_: *mut c_void, direction: c_int);
    pub fn wxDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxDialog_CreateButtonSizer(self_: *mut c_void, flags: c_long) -> *mut c_void;
    pub fn wxDialog_CreateSeparatedButtonSizer(self_: *mut c_void, flags: c_long) -> *mut c_void;
    pub fn wxDialog_CreateSeparatedSizer(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxDialog_CreateStdDialogButtonSizer(self_: *mut c_void, flags: c_long) -> *mut c_void;
    pub fn wxDialog_CreateTextSizer(
        self_: *mut c_void,
        message: *const c_void,
        width_max: c_int,
    ) -> *mut c_void;
    pub fn wxDialog_DoLayoutAdaptation(self_: *mut c_void) -> bool;
    pub fn wxDialog_EndModal(self_: *mut c_void, ret_code: c_int);
    pub fn wxDialog_GetAffirmativeId(self_: *const c_void) -> c_int;
    pub fn wxDialog_GetContentWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxDialog_GetEscapeId(self_: *const c_void) -> c_int;
    pub fn wxDialog_GetLayoutAdaptationDone(self_: *const c_void) -> bool;
    pub fn wxDialog_GetLayoutAdaptationLevel(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDialog_GetLayoutAdaptationMode(self_: *const c_void) -> wxDialogLayoutAdaptationMode;
    pub fn wxDialog_GetMainButtonIds(self_: *mut c_void) -> *mut c_void;
    pub fn wxDialog_GetReturnCode(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxDialog_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxDialog_IsMainButtonId(self_: *const c_void, id: c_int) -> bool;
    pub fn wxDialog_IsModal(self_: *const c_void) -> bool;
    pub fn wxDialog_SetAffirmativeId(self_: *mut c_void, id: c_int);
    pub fn wxDialog_SetEscapeId(self_: *mut c_void, id: c_int);
    pub fn wxDialog_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxDialog_SetLayoutAdaptationDone(self_: *mut c_void, done: bool);
    pub fn wxDialog_SetLayoutAdaptationLevel(self_: *mut c_void, level: c_int);
    // NOT_SUPPORTED: pub fn wxDialog_SetLayoutAdaptationMode(self_: *mut c_void, mode: wxDialogLayoutAdaptationMode);
    pub fn wxDialog_SetReturnCode(self_: *mut c_void, ret_code: c_int);
    pub fn wxDialog_ShowModal(self_: *mut c_void) -> c_int;
    pub fn wxDialog_ShowWindowModal(self_: *mut c_void);
    // BLOCKED: pub fn wxDialog_ShowWindowModalThenDo(self_: *mut c_void, on_end_modal: *const c_void);
    pub fn wxDialog_EnableLayoutAdaptation(enable: bool);
    pub fn wxDialog_GetLayoutAdapter() -> *mut c_void;
    pub fn wxDialog_IsLayoutAdaptationEnabled() -> bool;
    pub fn wxDialog_SetLayoutAdapter(adapter: *mut c_void) -> *mut c_void;

    // wxDialogLayoutAdapter
    pub fn wxDialogLayoutAdapter_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDialogLayoutAdapter_new() -> *mut c_void;
    pub fn wxDialogLayoutAdapter_CanDoLayoutAdaptation(
        self_: *mut c_void,
        dialog: *mut c_void,
    ) -> bool;
    pub fn wxDialogLayoutAdapter_DoLayoutAdaptation(
        self_: *mut c_void,
        dialog: *mut c_void,
    ) -> bool;

    // wxDirDialog
    pub fn wxDirDialog_CLASSINFO() -> *mut c_void;
    pub fn wxDirDialog_new(
        parent: *mut c_void,
        message: *const c_void,
        default_path: *const c_void,
        style: c_long,
        pos: *const c_void,
        size: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDirDialog_~wxDirDialog(self_: *mut c_void);
    pub fn wxDirDialog_GetMessage(self_: *const c_void) -> *mut c_void;
    pub fn wxDirDialog_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxDirDialog_GetPaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxDirDialog_SetMessage(self_: *mut c_void, message: *const c_void);
    pub fn wxDirDialog_SetPath(self_: *mut c_void, path: *const c_void);

    // wxDirPickerCtrl
    pub fn wxDirPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDirPickerCtrl_new() -> *mut c_void;
    pub fn wxDirPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxDirPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDirPickerCtrl_GetDirName(self_: *const c_void) -> *mut c_void;
    pub fn wxDirPickerCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxDirPickerCtrl_SetDirName(self_: *mut c_void, dirname: *const c_void);
    pub fn wxDirPickerCtrl_SetInitialDirectory(self_: *mut c_void, dir: *const c_void);
    pub fn wxDirPickerCtrl_SetPath(self_: *mut c_void, dirname: *const c_void);

    // wxDisplay
    pub fn wxDisplay_delete(self_: *mut c_void);
    pub fn wxDisplay_new() -> *mut c_void;
    pub fn wxDisplay_new1(index: c_uint) -> *mut c_void;
    pub fn wxDisplay_new2(window: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDisplay_~wxDisplay(self_: *mut c_void);
    pub fn wxDisplay_ChangeMode(self_: *mut c_void, mode: *const c_void) -> bool;
    pub fn wxDisplay_GetClientArea(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDisplay_GetCurrentMode(self_: *const c_void) -> wxVideoMode;
    pub fn wxDisplay_GetGeometry(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDisplay_GetModes(self_: *const c_void, mode: *const c_void) -> wxArrayVideoModes;
    pub fn wxDisplay_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxDisplay_GetPPI(self_: *const c_void) -> *mut c_void;
    pub fn wxDisplay_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxDisplay_IsPrimary(self_: *const c_void) -> bool;
    pub fn wxDisplay_GetCount() -> c_uint;
    pub fn wxDisplay_GetFromPoint(pt: *const c_void) -> c_int;
    pub fn wxDisplay_GetFromWindow(win: *const c_void) -> c_int;
    pub fn wxDisplay_GetStdPPIValue() -> c_int;
    pub fn wxDisplay_GetStdPPI() -> *mut c_void;

    // wxDisplayChangedEvent
    pub fn wxDisplayChangedEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDisplayChangedEvent_new() -> *mut c_void;

    // wxDragImage
    pub fn wxDragImage_CLASSINFO() -> *mut c_void;
    pub fn wxDragImage_new() -> *mut c_void;
    pub fn wxDragImage_new1(image: *const c_void, cursor: *const c_void) -> *mut c_void;
    pub fn wxDragImage_new2(image: *const c_void, cursor: *const c_void) -> *mut c_void;
    pub fn wxDragImage_new3(text: *const c_void, cursor: *const c_void) -> *mut c_void;
    pub fn wxDragImage_new4(tree_ctrl: *const c_void, id: *mut c_void) -> *mut c_void;
    pub fn wxDragImage_new5(list_ctrl: *const c_void, id: c_long) -> *mut c_void;
    pub fn wxDragImage_BeginDrag(
        self_: *mut c_void,
        hotspot: *const c_void,
        window: *mut c_void,
        full_screen: bool,
        rect: *mut c_void,
    ) -> bool;
    pub fn wxDragImage_BeginDrag1(
        self_: *mut c_void,
        hotspot: *const c_void,
        window: *mut c_void,
        bounding_window: *mut c_void,
    ) -> bool;
    pub fn wxDragImage_DoDrawImage(
        self_: *const c_void,
        dc: *mut c_void,
        pos: *const c_void,
    ) -> bool;
    pub fn wxDragImage_EndDrag(self_: *mut c_void) -> bool;
    pub fn wxDragImage_GetImageRect(self_: *const c_void, pos: *const c_void) -> *mut c_void;
    pub fn wxDragImage_Hide(self_: *mut c_void) -> bool;
    pub fn wxDragImage_Move(self_: *mut c_void, pt: *const c_void) -> bool;
    pub fn wxDragImage_Show(self_: *mut c_void) -> bool;
    pub fn wxDragImage_UpdateBackingFromWindow(
        self_: *const c_void,
        window_dc: *mut c_void,
        dest_dc: *mut c_void,
        source_rect: *const c_void,
        dest_rect: *const c_void,
    ) -> bool;

    // wxDropFilesEvent
    pub fn wxDropFilesEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDropFilesEvent_new(id: wxEventType, no_files: c_int, files: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDropFilesEvent_GetFiles(self_: *const c_void) -> *mut c_void;
    pub fn wxDropFilesEvent_GetNumberOfFiles(self_: *const c_void) -> c_int;
    pub fn wxDropFilesEvent_GetPosition(self_: *const c_void) -> *mut c_void;

    // wxDropSource
    pub fn wxDropSource_delete(self_: *mut c_void);
    pub fn wxDropSource_new(
        win: *mut c_void,
        icon_copy: *const c_void,
        icon_move: *const c_void,
        icon_none: *const c_void,
    ) -> *mut c_void;
    pub fn wxDropSource_new1(
        data: *mut c_void,
        win: *mut c_void,
        icon_copy: *const c_void,
        icon_move: *const c_void,
        icon_none: *const c_void,
    ) -> *mut c_void;
    pub fn wxDropSource_new2(
        win: *mut c_void,
        icon_copy: *const c_void,
        icon_move: *const c_void,
        icon_none: *const c_void,
    ) -> *mut c_void;
    pub fn wxDropSource_new3(
        data: *mut c_void,
        win: *mut c_void,
        icon_copy: *const c_void,
        icon_move: *const c_void,
        icon_none: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDropSource_DoDragDrop(self_: *mut c_void, flags: c_int) -> wxDragResult;
    pub fn wxDropSource_GetDataObject(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDropSource_GiveFeedback(self_: *mut c_void, effect: wxDragResult) -> bool;
    // NOT_SUPPORTED: pub fn wxDropSource_SetCursor(self_: *mut c_void, res: wxDragResult, cursor: *const c_void);
    // NOT_SUPPORTED: pub fn wxDropSource_SetIcon(self_: *mut c_void, res: wxDragResult, icon: *const c_void);
    pub fn wxDropSource_SetData(self_: *mut c_void, data: *mut c_void);

    // wxDropTarget
    pub fn wxDropTarget_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDropTarget_new(data: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxDropTarget_~wxDropTarget(self_: *mut c_void);
    pub fn wxDropTarget_GetData(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDropTarget_OnData(self_: *mut c_void, x: c_int, y: c_int, def_result: wxDragResult) -> wxDragResult;
    // NOT_SUPPORTED: pub fn wxDropTarget_OnDragOver(self_: *mut c_void, x: c_int, y: c_int, def_result: wxDragResult) -> wxDragResult;
    pub fn wxDropTarget_OnDrop(self_: *mut c_void, x: c_int, y: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxDropTarget_OnEnter(self_: *mut c_void, x: c_int, y: c_int, def_result: wxDragResult) -> wxDragResult;
    pub fn wxDropTarget_OnLeave(self_: *mut c_void);
    pub fn wxDropTarget_GetDataObject(self_: *const c_void) -> *mut c_void;
    pub fn wxDropTarget_SetDataObject(self_: *mut c_void, data: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDropTarget_SetDefaultAction(self_: *mut c_void, action: wxDragResult);
    // NOT_SUPPORTED: pub fn wxDropTarget_GetDefaultAction(self_: *mut c_void) -> wxDragResult;

}
