use super::*;

// wxGBPosition
pub trait GBPositionMethods: WxRustMethods {
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetCol(self.as_ptr()) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetRow(self.as_ptr()) }
    }
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGBPosition_SetCol(self.as_ptr(), col) }
    }
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGBPosition_SetRow(self.as_ptr(), row) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGBSizerItem
pub trait GBSizerItemMethods: SizerItemMethods {
    fn get_end_pos(&self, row: *mut c_void, col: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetEndPos(self.as_ptr(), row, col) }
    }
    fn get_pos(&self) -> GBPosition {
        unsafe { GBPosition::from_ptr(ffi::wxGBSizerItem_GetPos(self.as_ptr())) }
    }
    fn get_pos_int(&self, row: *mut c_void, col: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetPos1(self.as_ptr(), row, col) }
    }
    fn get_span(&self) -> GBSpan {
        unsafe { GBSpan::from_ptr(ffi::wxGBSizerItem_GetSpan(self.as_ptr())) }
    }
    fn get_span_int(&self, rowspan: *mut c_void, colspan: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetSpan1(self.as_ptr(), rowspan, colspan) }
    }
    fn intersects_gbsizeritem<G: GBSizerItemMethods>(&self, other: &G) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxGBSizerItem_Intersects(self.as_ptr(), other)
        }
    }
    fn intersects_gbposition<G: GBPositionMethods, G2: GBSpanMethods>(
        &self,
        pos: &G,
        span: &G2,
    ) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            ffi::wxGBSizerItem_Intersects1(self.as_ptr(), pos, span)
        }
    }
    fn set_pos<G: GBPositionMethods>(&self, pos: &G) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGBSizerItem_SetPos(self.as_ptr(), pos)
        }
    }
    fn set_span<G: GBSpanMethods>(&self, span: &G) -> bool {
        unsafe {
            let span = span.as_ptr();
            ffi::wxGBSizerItem_SetSpan(self.as_ptr(), span)
        }
    }
    fn get_gb_sizer(&self) -> Option<GridBagSizerIsOwned<false>> {
        unsafe { GridBagSizer::option_from(ffi::wxGBSizerItem_GetGBSizer(self.as_ptr())) }
    }
    fn set_gb_sizer<G: GridBagSizerMethods>(&self, sizer: Option<&G>) {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGBSizerItem_SetGBSizer(self.as_ptr(), sizer)
        }
    }
}

// wxGBSpan
pub trait GBSpanMethods: WxRustMethods {
    fn get_colspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetColspan(self.as_ptr()) }
    }
    fn get_rowspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetRowspan(self.as_ptr()) }
    }
    fn set_colspan(&self, colspan: c_int) {
        unsafe { ffi::wxGBSpan_SetColspan(self.as_ptr(), colspan) }
    }
    fn set_rowspan(&self, rowspan: c_int) {
        unsafe { ffi::wxGBSpan_SetRowspan(self.as_ptr(), rowspan) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGCDC
pub trait GCDCMethods: DCMethods {
    // DTOR: fn ~wxGCDC()
}

// wxGDIObject
pub trait GDIObjectMethods: ObjectMethods {}

// wxGIFHandler
pub trait GIFHandlerMethods: ImageHandlerMethods {
    fn save_animation(
        &self,
        images: *const c_void,
        stream: *mut c_void,
        verbose: bool,
        delay_milli_secs: c_int,
    ) -> bool {
        unsafe {
            ffi::wxGIFHandler_SaveAnimation(
                self.as_ptr(),
                images,
                stream,
                verbose,
                delay_milli_secs,
            )
        }
    }
}

// wxGauge
pub trait GaugeMethods: ControlMethods {
    // DTOR: fn ~wxGauge()
    fn create_int<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        range: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxGauge_Create(
                self.as_ptr(),
                parent,
                id,
                range,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGauge_GetRange(self.as_ptr()) }
    }
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGauge_GetValue(self.as_ptr()) }
    }
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxGauge_IsVertical(self.as_ptr()) }
    }
    fn pulse(&self) {
        unsafe { ffi::wxGauge_Pulse(self.as_ptr()) }
    }
    fn set_range(&self, range: c_int) {
        unsafe { ffi::wxGauge_SetRange(self.as_ptr(), range) }
    }
    fn set_value(&self, pos: c_int) {
        unsafe { ffi::wxGauge_SetValue(self.as_ptr(), pos) }
    }
}

// wxGenericAboutDialog
pub trait GenericAboutDialogMethods: WxRustMethods {
    fn create<A: AboutDialogInfoMethods, W: WindowMethods>(
        &self,
        info: &A,
        parent: Option<&W>,
    ) -> bool {
        unsafe {
            let info = info.as_ptr();
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGenericAboutDialog_Create(self.as_ptr(), info, parent)
        }
    }
}

// wxGenericDirCtrl
pub trait GenericDirCtrlMethods: ControlMethods {
    // DTOR: fn ~wxGenericDirCtrl()
    fn collapse_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_CollapsePath(self.as_ptr(), path)
        }
    }
    fn collapse_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_CollapseTree(self.as_ptr()) }
    }
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        dir: &str,
        pos: &P,
        size: &S,
        style: c_long,
        filter: &str,
        default_filter: c_int,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxGenericDirCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                dir,
                pos,
                size,
                style,
                filter,
                default_filter,
                name,
            )
        }
    }
    fn expand_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_ExpandPath(self.as_ptr(), path)
        }
    }
    fn get_default_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetDefaultPath(self.as_ptr())).into() }
    }
    fn get_file_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilePath(self.as_ptr())).into() }
    }
    fn get_file_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetFilePaths(self.as_ptr(), paths)
        }
    }
    fn get_filter(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilter(self.as_ptr())).into() }
    }
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxGenericDirCtrl_GetFilterIndex(self.as_ptr()) }
    }
    fn get_filter_list_ctrl(&self) -> *mut c_void {
        unsafe { ffi::wxGenericDirCtrl_GetFilterListCtrl(self.as_ptr()) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetPath(self.as_ptr())).into() }
    }
    // BLOCKED: fn GetPath1()
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    fn get_root_id(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxGenericDirCtrl_GetRootId(self.as_ptr())) }
    }
    fn get_tree_ctrl(&self) -> WeakRef<TreeCtrl> {
        unsafe { WeakRef::<TreeCtrl>::from(ffi::wxGenericDirCtrl_GetTreeCtrl(self.as_ptr())) }
    }
    fn init(&self) {
        unsafe { ffi::wxGenericDirCtrl_Init(self.as_ptr()) }
    }
    fn re_create_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_ReCreateTree(self.as_ptr()) }
    }
    fn set_default_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetDefaultPath(self.as_ptr(), path)
        }
    }
    fn set_filter(&self, filter: &str) {
        unsafe {
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            ffi::wxGenericDirCtrl_SetFilter(self.as_ptr(), filter)
        }
    }
    fn set_filter_index(&self, n: c_int) {
        unsafe { ffi::wxGenericDirCtrl_SetFilterIndex(self.as_ptr(), n) }
    }
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetPath(self.as_ptr(), path)
        }
    }
    fn show_hidden(&self, show: bool) {
        unsafe { ffi::wxGenericDirCtrl_ShowHidden(self.as_ptr(), show) }
    }
    fn select_path(&self, path: &str, select: bool) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SelectPath(self.as_ptr(), path, select)
        }
    }
    fn select_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_SelectPaths(self.as_ptr(), paths)
        }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxGenericDirCtrl_UnselectAll(self.as_ptr()) }
    }
}

// wxGenericProgressDialog
pub trait GenericProgressDialogMethods: DialogMethods {
    // DTOR: fn ~wxGenericProgressDialog()
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGenericProgressDialog_GetValue(self.as_ptr()) }
    }
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGenericProgressDialog_GetRange(self.as_ptr()) }
    }
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericProgressDialog_GetMessage(self.as_ptr())).into() }
    }
    fn pulse(&self, newmsg: &str, skip: *mut c_void) -> bool {
        unsafe {
            let newmsg = WxString::from(newmsg);
            let newmsg = newmsg.as_ptr();
            ffi::wxGenericProgressDialog_Pulse(self.as_ptr(), newmsg, skip)
        }
    }
    fn resume(&self) {
        unsafe { ffi::wxGenericProgressDialog_Resume(self.as_ptr()) }
    }
    fn set_range(&self, maximum: c_int) {
        unsafe { ffi::wxGenericProgressDialog_SetRange(self.as_ptr(), maximum) }
    }
    fn was_cancelled(&self) -> bool {
        unsafe { ffi::wxGenericProgressDialog_WasCancelled(self.as_ptr()) }
    }
    fn was_skipped(&self) -> bool {
        unsafe { ffi::wxGenericProgressDialog_WasSkipped(self.as_ptr()) }
    }
    fn update_int(&self, value: c_int, newmsg: &str, skip: *mut c_void) -> bool {
        unsafe {
            let newmsg = WxString::from(newmsg);
            let newmsg = newmsg.as_ptr();
            ffi::wxGenericProgressDialog_Update(self.as_ptr(), value, newmsg, skip)
        }
    }
}

// wxGenericValidator
pub trait GenericValidatorMethods: ValidatorMethods {
    // DTOR: fn ~wxGenericValidator()
}

// wxGraphicsBrush
pub trait GraphicsBrushMethods: GraphicsObjectMethods {}

// wxGraphicsContext
pub trait GraphicsContextMethods: GraphicsObjectMethods {
    fn create_window<W: WindowMethods>(
        window: Option<&W>,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create(window))
        }
    }
    fn create_windowdc<W: WindowDCMethods>(window_dc: &W) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create1(window_dc))
        }
    }
    fn create_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create2(memory_dc))
        }
    }
    // BLOCKED: fn Create3()
    fn create_enhmetafiledc(meta_file_dc: *const c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_Create4(meta_file_dc)) }
    }
    fn create_from_unknown_dc<D: DCMethods>(dc: &D) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let dc = dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromUnknownDC(dc))
        }
    }
    fn create_image<I: ImageMethods>(image: &I) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let image = image.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create5(image))
        }
    }
    fn create_from_native(context: *mut c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromNative(context)) }
    }
    fn create_from_native_window(window: *mut c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromNativeWindow(window))
        }
    }
    // NOT_SUPPORTED: fn CreateFromNativeHDC()
    fn create() -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_Create6()) }
    }
    fn reset_clip(&self) {
        unsafe { ffi::wxGraphicsContext_ResetClip(self.as_ptr()) }
    }
    fn clip<R: RegionMethods>(&self, region: &R) {
        unsafe {
            let region = region.as_ptr();
            ffi::wxGraphicsContext_Clip(self.as_ptr(), region)
        }
    }
    // NOT_SUPPORTED: fn Clip1()
    fn get_clip_box(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetClipBox(self.as_ptr(), x, y, w, h) }
    }
    // NOT_SUPPORTED: fn CreateMatrix()
    fn create_matrix<A: AffineMatrix2DBaseMethods>(&self, mat: &A) -> GraphicsMatrix {
        unsafe {
            let mat = mat.as_ptr();
            GraphicsMatrix::from_ptr(ffi::wxGraphicsContext_CreateMatrix1(self.as_ptr(), mat))
        }
    }
    fn concat_transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsContext_ConcatTransform(self.as_ptr(), matrix)
        }
    }
    fn get_transform(&self) -> GraphicsMatrix {
        unsafe { GraphicsMatrix::from_ptr(ffi::wxGraphicsContext_GetTransform(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    fn set_transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsContext_SetTransform(self.as_ptr(), matrix)
        }
    }
    // NOT_SUPPORTED: fn Translate()
    fn create_brush<B: BrushMethods>(&self, brush: &B) -> GraphicsBrush {
        unsafe {
            let brush = brush.as_ptr();
            GraphicsBrush::from_ptr(ffi::wxGraphicsContext_CreateBrush(self.as_ptr(), brush))
        }
    }
    // NOT_SUPPORTED: fn CreateLinearGradientBrush()
    // NOT_SUPPORTED: fn CreateLinearGradientBrush1()
    // NOT_SUPPORTED: fn CreateRadialGradientBrush()
    // NOT_SUPPORTED: fn CreateRadialGradientBrush1()
    fn set_brush_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxGraphicsContext_SetBrush(self.as_ptr(), brush)
        }
    }
    fn set_brush_graphicsbrush<G: GraphicsBrushMethods>(&self, brush: &G) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxGraphicsContext_SetBrush1(self.as_ptr(), brush)
        }
    }
    fn create_pen_pen<P: PenMethods>(&self, pen: &P) -> GraphicsPen {
        unsafe {
            let pen = pen.as_ptr();
            GraphicsPen::from_ptr(ffi::wxGraphicsContext_CreatePen(self.as_ptr(), pen))
        }
    }
    fn create_pen_graphicspeninfo(&self, info: *const c_void) -> GraphicsPen {
        unsafe { GraphicsPen::from_ptr(ffi::wxGraphicsContext_CreatePen1(self.as_ptr(), info)) }
    }
    fn set_pen_pen<P: PenMethods>(&self, pen: &P) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxGraphicsContext_SetPen(self.as_ptr(), pen)
        }
    }
    fn set_pen_graphicspen<G: GraphicsPenMethods>(&self, pen: &G) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxGraphicsContext_SetPen1(self.as_ptr(), pen)
        }
    }
    // NOT_SUPPORTED: fn DrawBitmap()
    // NOT_SUPPORTED: fn DrawBitmap1()
    // NOT_SUPPORTED: fn DrawEllipse()
    // NOT_SUPPORTED: fn DrawIcon()
    // NOT_SUPPORTED: fn DrawLines()
    // NOT_SUPPORTED: fn DrawPath()
    // NOT_SUPPORTED: fn DrawRectangle()
    // NOT_SUPPORTED: fn DrawRoundedRectangle()
    // NOT_SUPPORTED: fn DrawText()
    // NOT_SUPPORTED: fn DrawText1()
    // NOT_SUPPORTED: fn DrawText2()
    // NOT_SUPPORTED: fn DrawText3()
    fn create_path(&self) -> GraphicsPath {
        unsafe { GraphicsPath::from_ptr(ffi::wxGraphicsContext_CreatePath(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn FillPath()
    // NOT_SUPPORTED: fn StrokeLine()
    fn stroke_lines_point2ddouble(
        &self,
        n: usize,
        begin_points: *const c_void,
        end_points: *const c_void,
    ) {
        unsafe { ffi::wxGraphicsContext_StrokeLines(self.as_ptr(), n, begin_points, end_points) }
    }
    fn stroke_lines(&self, n: usize, points: *const c_void) {
        unsafe { ffi::wxGraphicsContext_StrokeLines1(self.as_ptr(), n, points) }
    }
    fn stroke_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsContext_StrokePath(self.as_ptr(), path)
        }
    }
    fn create_font_font<F: FontMethods, C: ColourMethods>(
        &self,
        font: &F,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let font = font.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsContext_CreateFont(self.as_ptr(), font, col))
        }
    }
    fn create_font_double<C: ColourMethods>(
        &self,
        size_in_pixels: c_double,
        facename: &str,
        flags: c_int,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsContext_CreateFont1(
                self.as_ptr(),
                size_in_pixels,
                facename,
                flags,
                col,
            ))
        }
    }
    fn set_font_font<F: FontMethods, C: ColourMethods>(&self, font: &F, colour: &C) {
        unsafe {
            let font = font.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxGraphicsContext_SetFont(self.as_ptr(), font, colour)
        }
    }
    fn set_font_graphicsfont<G: GraphicsFontMethods>(&self, font: &G) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxGraphicsContext_SetFont1(self.as_ptr(), font)
        }
    }
    fn get_partial_text_extents(&self, text: &str, widths: *mut c_void) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxGraphicsContext_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    fn get_text_extent(
        &self,
        text: &str,
        width: *mut c_void,
        height: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
    ) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxGraphicsContext_GetTextExtent(
                self.as_ptr(),
                text,
                width,
                height,
                descent,
                external_leading,
            )
        }
    }
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxGraphicsContext_StartDoc(self.as_ptr(), message)
        }
    }
    fn end_doc(&self) {
        unsafe { ffi::wxGraphicsContext_EndDoc(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn StartPage()
    fn end_page(&self) {
        unsafe { ffi::wxGraphicsContext_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn CreateBitmap()
    // NOT_SUPPORTED: fn CreateBitmapFromImage()
    // NOT_SUPPORTED: fn CreateSubBitmap()
    // NOT_SUPPORTED: fn BeginLayer()
    fn end_layer(&self) {
        unsafe { ffi::wxGraphicsContext_EndLayer(self.as_ptr()) }
    }
    fn push_state(&self) {
        unsafe { ffi::wxGraphicsContext_PushState(self.as_ptr()) }
    }
    fn pop_state(&self) {
        unsafe { ffi::wxGraphicsContext_PopState(self.as_ptr()) }
    }
    fn flush(&self) {
        unsafe { ffi::wxGraphicsContext_Flush(self.as_ptr()) }
    }
    fn get_native_context(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsContext_GetNativeContext(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAntialiasMode()
    // NOT_SUPPORTED: fn GetAntialiasMode()
    // NOT_SUPPORTED: fn SetInterpolationQuality()
    // NOT_SUPPORTED: fn GetInterpolationQuality()
    // NOT_SUPPORTED: fn SetCompositionMode()
    // NOT_SUPPORTED: fn GetCompositionMode()
    fn get_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetSize(self.as_ptr(), width, height) }
    }
    fn get_dpi(&self, dpi_x: *mut c_void, dpi_y: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetDPI(self.as_ptr(), dpi_x, dpi_y) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGraphicsContext_GetWindow(self.as_ptr())) }
    }
    fn should_offset(&self) -> bool {
        unsafe { ffi::wxGraphicsContext_ShouldOffset(self.as_ptr()) }
    }
    fn enable_offset(&self, enable: bool) {
        unsafe { ffi::wxGraphicsContext_EnableOffset(self.as_ptr(), enable) }
    }
    fn disable_offset(&self) {
        unsafe { ffi::wxGraphicsContext_DisableOffset(self.as_ptr()) }
    }
    fn offset_enabled(&self) -> bool {
        unsafe { ffi::wxGraphicsContext_OffsetEnabled(self.as_ptr()) }
    }
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxGraphicsContext_FromDIP(self.as_ptr(), sz))
        }
    }
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxGraphicsContext_FromDIP1(self.as_ptr(), pt))
        }
    }
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxGraphicsContext_FromDIP2(self.as_ptr(), d) }
    }
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxGraphicsContext_ToDIP(self.as_ptr(), sz))
        }
    }
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxGraphicsContext_ToDIP1(self.as_ptr(), pt))
        }
    }
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxGraphicsContext_ToDIP2(self.as_ptr(), d) }
    }
}

// wxGraphicsFont
pub trait GraphicsFontMethods: GraphicsObjectMethods {}

// wxGraphicsGradientStop
pub trait GraphicsGradientStopMethods: WxRustMethods {
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxGraphicsGradientStop_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxGraphicsGradientStop_SetColour(self.as_ptr(), col)
        }
    }
    // NOT_SUPPORTED: fn GetPosition()
    // NOT_SUPPORTED: fn SetPosition()
}

// wxGraphicsGradientStops
pub trait GraphicsGradientStopsMethods: WxRustMethods {
    fn add<G: GraphicsGradientStopMethods>(&self, stop: &G) {
        unsafe {
            let stop = stop.as_ptr();
            ffi::wxGraphicsGradientStops_Add(self.as_ptr(), stop)
        }
    }
    // NOT_SUPPORTED: fn Add1()
    // NOT_SUPPORTED: fn Item()
    fn get_count(&self) -> usize {
        unsafe { ffi::wxGraphicsGradientStops_GetCount(self.as_ptr()) }
    }
    // BLOCKED: fn SetStartColour()
    fn get_start_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetStartColour(self.as_ptr())) }
    }
    // BLOCKED: fn SetEndColour()
    fn get_end_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetEndColour(self.as_ptr())) }
    }
}

// wxGraphicsMatrix
pub trait GraphicsMatrixMethods: GraphicsObjectMethods {
    fn concat<G: GraphicsMatrixMethods>(&self, t: Option<&G>) {
        unsafe {
            let t = match t {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGraphicsMatrix_Concat(self.as_ptr(), t)
        }
    }
    // BLOCKED: fn Concat1()
    fn get(
        &self,
        a: *mut c_void,
        b: *mut c_void,
        c: *mut c_void,
        d: *mut c_void,
        tx: *mut c_void,
        ty: *mut c_void,
    ) {
        unsafe { ffi::wxGraphicsMatrix_Get(self.as_ptr(), a, b, c, d, tx, ty) }
    }
    fn get_native_matrix(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsMatrix_GetNativeMatrix(self.as_ptr()) }
    }
    fn invert(&self) {
        unsafe { ffi::wxGraphicsMatrix_Invert(self.as_ptr()) }
    }
    // BLOCKED: fn IsEqual()
    fn is_equal<G: GraphicsMatrixMethods>(&self, t: &G) -> bool {
        unsafe {
            let t = t.as_ptr();
            ffi::wxGraphicsMatrix_IsEqual1(self.as_ptr(), t)
        }
    }
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxGraphicsMatrix_IsIdentity(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Set()
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformDistance(self.as_ptr(), dx, dy) }
    }
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn Translate()
}

// wxGraphicsObject
pub trait GraphicsObjectMethods: ObjectMethods {
    fn get_renderer(&self) -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsObject_GetRenderer(self.as_ptr())) }
    }
    fn is_null(&self) -> bool {
        unsafe { ffi::wxGraphicsObject_IsNull(self.as_ptr()) }
    }
}

// wxGraphicsPath
pub trait GraphicsPathMethods: GraphicsObjectMethods {
    // NOT_SUPPORTED: fn AddArc()
    // NOT_SUPPORTED: fn AddArc1()
    // NOT_SUPPORTED: fn AddArcToPoint()
    // NOT_SUPPORTED: fn AddCircle()
    // NOT_SUPPORTED: fn AddCurveToPoint()
    fn add_curve_to_point(&self, c1: *const c_void, c2: *const c_void, e: *const c_void) {
        unsafe { ffi::wxGraphicsPath_AddCurveToPoint1(self.as_ptr(), c1, c2, e) }
    }
    // NOT_SUPPORTED: fn AddEllipse()
    // NOT_SUPPORTED: fn AddLineToPoint()
    fn add_line_to_point(&self, p: *const c_void) {
        unsafe { ffi::wxGraphicsPath_AddLineToPoint1(self.as_ptr(), p) }
    }
    fn add_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsPath_AddPath(self.as_ptr(), path)
        }
    }
    // NOT_SUPPORTED: fn AddQuadCurveToPoint()
    // NOT_SUPPORTED: fn AddRectangle()
    // NOT_SUPPORTED: fn AddRoundedRectangle()
    fn close_subpath(&self) {
        unsafe { ffi::wxGraphicsPath_CloseSubpath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    // NOT_SUPPORTED: fn GetBox()
    fn get_box(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetBox1(self.as_ptr(), x, y, w, h) }
    }
    fn get_current_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetCurrentPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetCurrentPoint1()
    fn get_native_path(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsPath_GetNativePath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn MoveToPoint()
    fn move_to_point(&self, p: *const c_void) {
        unsafe { ffi::wxGraphicsPath_MoveToPoint1(self.as_ptr(), p) }
    }
    fn transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsPath_Transform(self.as_ptr(), matrix)
        }
    }
    fn un_get_native_path(&self, p: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_UnGetNativePath(self.as_ptr(), p) }
    }
}

// wxGraphicsPen
pub trait GraphicsPenMethods: GraphicsObjectMethods {}

// wxGraphicsRenderer
pub trait GraphicsRendererMethods: ObjectMethods {
    // NOT_SUPPORTED: fn CreateBitmap()
    // NOT_SUPPORTED: fn CreateBitmapFromImage()
    fn create_image_from_bitmap(&self, bmp: *const c_void) -> Image {
        unsafe {
            Image::from_ptr(ffi::wxGraphicsRenderer_CreateImageFromBitmap(
                self.as_ptr(),
                bmp,
            ))
        }
    }
    // NOT_SUPPORTED: fn CreateBitmapFromNativeBitmap()
    fn create_context_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext(
                self.as_ptr(),
                window,
            ))
        }
    }
    fn create_context_windowdc<W: WindowDCMethods>(
        &self,
        window_dc: &W,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext1(
                self.as_ptr(),
                window_dc,
            ))
        }
    }
    fn create_context_memorydc<M: MemoryDCMethods>(
        &self,
        memory_dc: &M,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext2(
                self.as_ptr(),
                memory_dc,
            ))
        }
    }
    // BLOCKED: fn CreateContext3()
    fn create_context_enhmetafiledc(
        &self,
        meta_file_dc: *const c_void,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContext4(
                self.as_ptr(),
                meta_file_dc,
            ))
        }
    }
    fn create_context_from_unknown_dc<D: DCMethods>(
        &self,
        dc: &D,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let dc = dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromUnknownDC(
                self.as_ptr(),
                dc,
            ))
        }
    }
    fn create_context_from_image<I: ImageMethods>(
        &self,
        image: &I,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let image = image.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromImage(
                self.as_ptr(),
                image,
            ))
        }
    }
    fn create_brush<B: BrushMethods>(&self, brush: &B) -> GraphicsBrush {
        unsafe {
            let brush = brush.as_ptr();
            GraphicsBrush::from_ptr(ffi::wxGraphicsRenderer_CreateBrush(self.as_ptr(), brush))
        }
    }
    fn create_context_from_native_context(
        &self,
        context: *mut c_void,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromNativeContext(
                self.as_ptr(),
                context,
            ))
        }
    }
    fn create_context_from_native_window(
        &self,
        window: *mut c_void,
    ) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateContextFromNativeWindow(
                self.as_ptr(),
                window,
            ))
        }
    }
    fn create_measuring_context(&self) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateMeasuringContext(
                self.as_ptr(),
            ))
        }
    }
    fn create_font_font<F: FontMethods, C: ColourMethods>(
        &self,
        font: &F,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let font = font.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsRenderer_CreateFont(self.as_ptr(), font, col))
        }
    }
    fn create_font_double<C: ColourMethods>(
        &self,
        size_in_pixels: c_double,
        facename: &str,
        flags: c_int,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let facename = WxString::from(facename);
            let facename = facename.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsRenderer_CreateFont1(
                self.as_ptr(),
                size_in_pixels,
                facename,
                flags,
                col,
            ))
        }
    }
    fn create_font_at_dpi<F: FontMethods, R: RealPointMethods, C: ColourMethods>(
        &self,
        font: &F,
        dpi: &R,
        col: &C,
    ) -> GraphicsFont {
        unsafe {
            let font = font.as_ptr();
            let dpi = dpi.as_ptr();
            let col = col.as_ptr();
            GraphicsFont::from_ptr(ffi::wxGraphicsRenderer_CreateFontAtDPI(
                self.as_ptr(),
                font,
                dpi,
                col,
            ))
        }
    }
    // NOT_SUPPORTED: fn CreateLinearGradientBrush()
    // NOT_SUPPORTED: fn CreateMatrix()
    fn create_path(&self) -> GraphicsPath {
        unsafe { GraphicsPath::from_ptr(ffi::wxGraphicsRenderer_CreatePath(self.as_ptr())) }
    }
    fn create_pen(&self, info: *const c_void) -> GraphicsPen {
        unsafe { GraphicsPen::from_ptr(ffi::wxGraphicsRenderer_CreatePen(self.as_ptr(), info)) }
    }
    // NOT_SUPPORTED: fn CreateRadialGradientBrush()
    // NOT_SUPPORTED: fn CreateSubBitmap()
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGraphicsRenderer_GetName(self.as_ptr())).into() }
    }
    fn get_version(&self, major: *mut c_void, minor: *mut c_void, micro: *mut c_void) {
        unsafe { ffi::wxGraphicsRenderer_GetVersion(self.as_ptr(), major, minor, micro) }
    }
    // NOT_SUPPORTED: fn CreateContextFromNativeHDC()
    fn get_default_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetDefaultRenderer()) }
    }
    fn get_cairo_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetCairoRenderer()) }
    }
    fn get_gdi_plus_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetGDIPlusRenderer()) }
    }
    fn get_direct2_d_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetDirect2DRenderer()) }
    }
}

// wxGridBagSizer
pub trait GridBagSizerMethods: FlexGridSizerMethods {
    fn add_window_gbposition<
        W: WindowMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        &self,
        window: Option<&W>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxGridBagSizer_Add(
                self.as_ptr(),
                window,
                pos,
                span,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn add_sizer_gbposition<
        S: SizerMethods,
        G: GBPositionMethods,
        G2: GBSpanMethods,
        O: ObjectMethods,
    >(
        &self,
        sizer: Option<&S>,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxGridBagSizer_Add1(
                self.as_ptr(),
                sizer,
                pos,
                span,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn add_gbsizeritem<G: GBSizerItemMethods>(
        &self,
        item: Option<&G>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxGridBagSizer_Add2(self.as_ptr(), item))
        }
    }
    fn add_int_gbposition<G: GBPositionMethods, G2: GBSpanMethods, O: ObjectMethods>(
        &self,
        width: c_int,
        height: c_int,
        pos: &G,
        span: &G2,
        flag: c_int,
        border: c_int,
        user_data: Option<&O>,
    ) -> Option<SizerItemIsOwned<false>> {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            SizerItem::option_from(ffi::wxGridBagSizer_Add3(
                self.as_ptr(),
                width,
                height,
                pos,
                span,
                flag,
                border,
                user_data,
            ))
        }
    }
    fn check_for_intersection_gbsizeritem<G: GBSizerItemMethods, G2: GBSizerItemMethods>(
        &self,
        item: Option<&G>,
        exclude_item: Option<&G2>,
    ) -> bool {
        unsafe {
            let item = match item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let exclude_item = match exclude_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_CheckForIntersection(self.as_ptr(), item, exclude_item)
        }
    }
    fn check_for_intersection_gbposition<
        G: GBPositionMethods,
        G2: GBSpanMethods,
        G3: GBSizerItemMethods,
    >(
        &self,
        pos: &G,
        span: &G2,
        exclude_item: Option<&G3>,
    ) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            let span = span.as_ptr();
            let exclude_item = match exclude_item {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridBagSizer_CheckForIntersection1(self.as_ptr(), pos, span, exclude_item)
        }
    }
    fn find_item_window<W: WindowMethods>(
        &self,
        window: Option<&W>,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItem(self.as_ptr(), window))
        }
    }
    fn find_item_sizer<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItem1(self.as_ptr(), sizer))
        }
    }
    fn find_item_at_point<P: PointMethods>(&self, pt: &P) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let pt = pt.as_ptr();
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemAtPoint(self.as_ptr(), pt))
        }
    }
    fn find_item_at_position<G: GBPositionMethods>(
        &self,
        pos: &G,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let pos = pos.as_ptr();
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemAtPosition(self.as_ptr(), pos))
        }
    }
    fn find_item_with_data<O: ObjectMethods>(
        &self,
        user_data: Option<&O>,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let user_data = match user_data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemWithData(
                self.as_ptr(),
                user_data,
            ))
        }
    }
    fn get_cell_size(&self, row: c_int, col: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetCellSize(self.as_ptr(), row, col)) }
    }
    fn get_empty_cell_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetEmptyCellSize(self.as_ptr())) }
    }
    fn get_item_position_window<W: WindowMethods>(&self, window: Option<&W>) -> GBPosition {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition(self.as_ptr(), window))
        }
    }
    fn get_item_position_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> GBPosition {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition1(self.as_ptr(), sizer))
        }
    }
    fn get_item_position_sz(&self, index: usize) -> GBPosition {
        unsafe { GBPosition::from_ptr(ffi::wxGridBagSizer_GetItemPosition2(self.as_ptr(), index)) }
    }
    fn get_item_span_window<W: WindowMethods>(&self, window: Option<&W>) -> GBSpan {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan(self.as_ptr(), window))
        }
    }
    fn get_item_span_sizer<S: SizerMethods>(&self, sizer: Option<&S>) -> GBSpan {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan1(self.as_ptr(), sizer))
        }
    }
    fn get_item_span_sz(&self, index: usize) -> GBSpan {
        unsafe { GBSpan::from_ptr(ffi::wxGridBagSizer_GetItemSpan2(self.as_ptr(), index)) }
    }
    fn set_empty_cell_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxGridBagSizer_SetEmptyCellSize(self.as_ptr(), sz)
        }
    }
    fn set_item_position_window<W: WindowMethods, G: GBPositionMethods>(
        &self,
        window: Option<&W>,
        pos: &G,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition(self.as_ptr(), window, pos)
        }
    }
    fn set_item_position_sizer<S: SizerMethods, G: GBPositionMethods>(
        &self,
        sizer: Option<&S>,
        pos: &G,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition1(self.as_ptr(), sizer, pos)
        }
    }
    fn set_item_position_sz<G: GBPositionMethods>(&self, index: usize, pos: &G) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGridBagSizer_SetItemPosition2(self.as_ptr(), index, pos)
        }
    }
    fn set_item_span_window<W: WindowMethods, G: GBSpanMethods>(
        &self,
        window: Option<&W>,
        span: &G,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan(self.as_ptr(), window, span)
        }
    }
    fn set_item_span_sizer<S: SizerMethods, G: GBSpanMethods>(
        &self,
        sizer: Option<&S>,
        span: &G,
    ) -> bool {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan1(self.as_ptr(), sizer, span)
        }
    }
    fn set_item_span_sz<G: GBSpanMethods>(&self, index: usize, span: &G) -> bool {
        unsafe {
            let span = span.as_ptr();
            ffi::wxGridBagSizer_SetItemSpan2(self.as_ptr(), index, span)
        }
    }
}

// wxGridEditorCreatedEvent
pub trait GridEditorCreatedEventMethods: CommandEventMethods {
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetCol(self.as_ptr()) }
    }
    fn get_control(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxGridEditorCreatedEvent_GetControl(self.as_ptr())) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetRow(self.as_ptr()) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGridEditorCreatedEvent_GetWindow(self.as_ptr())) }
    }
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetCol(self.as_ptr(), col) }
    }
    fn set_control<C: ControlMethods>(&self, ctrl: Option<&C>) {
        unsafe {
            let ctrl = match ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetControl(self.as_ptr(), ctrl)
        }
    }
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetRow(self.as_ptr(), row) }
    }
    fn set_window<W: WindowMethods>(&self, window: Option<&W>) {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetWindow(self.as_ptr(), window)
        }
    }
}

// wxGridEvent
pub trait GridEventMethods: NotifyEventMethods {
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_AltDown(self.as_ptr()) }
    }
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ControlDown(self.as_ptr()) }
    }
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetCol(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridEvent_GetPosition(self.as_ptr())) }
    }
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetRow(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_MetaDown(self.as_ptr()) }
    }
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridEvent_Selecting(self.as_ptr()) }
    }
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridRangeSelectEvent
pub trait GridRangeSelectEventMethods: NotifyEventMethods {
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_AltDown(self.as_ptr()) }
    }
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ControlDown(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetBottomRightCoords()
    fn get_bottom_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetBottomRow(self.as_ptr()) }
    }
    fn get_left_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetLeftCol(self.as_ptr()) }
    }
    fn get_right_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetRightCol(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetTopLeftCoords()
    fn get_top_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetTopRow(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_MetaDown(self.as_ptr()) }
    }
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_Selecting(self.as_ptr()) }
    }
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizeEvent
pub trait GridSizeEventMethods: NotifyEventMethods {
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_AltDown(self.as_ptr()) }
    }
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ControlDown(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridSizeEvent_GetPosition(self.as_ptr())) }
    }
    fn get_row_or_col(&self) -> c_int {
        unsafe { ffi::wxGridSizeEvent_GetRowOrCol(self.as_ptr()) }
    }
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_MetaDown(self.as_ptr()) }
    }
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizer
pub trait GridSizerMethods: SizerMethods {
    fn get_cols(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetCols(self.as_ptr()) }
    }
    fn get_rows(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetRows(self.as_ptr()) }
    }
    fn get_effective_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveColsCount(self.as_ptr()) }
    }
    fn get_effective_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveRowsCount(self.as_ptr()) }
    }
    fn get_h_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetHGap(self.as_ptr()) }
    }
    fn get_v_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetVGap(self.as_ptr()) }
    }
    fn set_cols(&self, cols: c_int) {
        unsafe { ffi::wxGridSizer_SetCols(self.as_ptr(), cols) }
    }
    fn set_h_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetHGap(self.as_ptr(), gap) }
    }
    fn set_rows(&self, rows: c_int) {
        unsafe { ffi::wxGridSizer_SetRows(self.as_ptr(), rows) }
    }
    fn set_v_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetVGap(self.as_ptr(), gap) }
    }
}

// wxGridTableBase
pub trait GridTableBaseMethods: ObjectMethods {
    fn is_empty_cell(&self, row: c_int, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_IsEmptyCell(self.as_ptr(), row, col) }
    }
    fn is_empty(&self, coords: *const c_void) -> bool {
        unsafe { ffi::wxGridTableBase_IsEmpty(self.as_ptr(), coords) }
    }
    fn get_value(&self, row: c_int, col: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxGridTableBase_GetValue(self.as_ptr(), row, col)).into() }
    }
    fn set_value(&self, row: c_int, col: c_int, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxGridTableBase_SetValue(self.as_ptr(), row, col, value)
        }
    }
    fn get_type_name(&self, row: c_int, col: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetTypeName(self.as_ptr(), row, col)).into()
        }
    }
    fn can_get_value_as(&self, row: c_int, col: c_int, type_name: &str) -> bool {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_CanGetValueAs(self.as_ptr(), row, col, type_name)
        }
    }
    fn can_set_value_as(&self, row: c_int, col: c_int, type_name: &str) -> bool {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_CanSetValueAs(self.as_ptr(), row, col, type_name)
        }
    }
    fn get_value_as_long(&self, row: c_int, col: c_int) -> c_long {
        unsafe { ffi::wxGridTableBase_GetValueAsLong(self.as_ptr(), row, col) }
    }
    fn get_value_as_double(&self, row: c_int, col: c_int) -> c_double {
        unsafe { ffi::wxGridTableBase_GetValueAsDouble(self.as_ptr(), row, col) }
    }
    fn get_value_as_bool(&self, row: c_int, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_GetValueAsBool(self.as_ptr(), row, col) }
    }
    fn get_value_as_custom(&self, row: c_int, col: c_int, type_name: &str) -> *mut c_void {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_GetValueAsCustom(self.as_ptr(), row, col, type_name)
        }
    }
    fn set_value_as_long(&self, row: c_int, col: c_int, value: c_long) {
        unsafe { ffi::wxGridTableBase_SetValueAsLong(self.as_ptr(), row, col, value) }
    }
    fn set_value_as_double(&self, row: c_int, col: c_int, value: c_double) {
        unsafe { ffi::wxGridTableBase_SetValueAsDouble(self.as_ptr(), row, col, value) }
    }
    fn set_value_as_bool(&self, row: c_int, col: c_int, value: bool) {
        unsafe { ffi::wxGridTableBase_SetValueAsBool(self.as_ptr(), row, col, value) }
    }
    fn set_value_as_custom(&self, row: c_int, col: c_int, type_name: &str, value: *mut c_void) {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_SetValueAsCustom(self.as_ptr(), row, col, type_name, value)
        }
    }
    fn set_view(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridTableBase_SetView(self.as_ptr(), grid) }
    }
    fn get_view(&self) -> *mut c_void {
        unsafe { ffi::wxGridTableBase_GetView(self.as_ptr()) }
    }
    fn clear(&self) {
        unsafe { ffi::wxGridTableBase_Clear(self.as_ptr()) }
    }
    fn insert_rows(&self, pos: usize, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_InsertRows(self.as_ptr(), pos, num_rows) }
    }
    fn append_rows(&self, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_AppendRows(self.as_ptr(), num_rows) }
    }
    fn delete_rows(&self, pos: usize, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_DeleteRows(self.as_ptr(), pos, num_rows) }
    }
    fn insert_cols(&self, pos: usize, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_InsertCols(self.as_ptr(), pos, num_cols) }
    }
    fn append_cols(&self, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_AppendCols(self.as_ptr(), num_cols) }
    }
    fn delete_cols(&self, pos: usize, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_DeleteCols(self.as_ptr(), pos, num_cols) }
    }
    fn get_row_label_value(&self, row: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetRowLabelValue(self.as_ptr(), row)).into()
        }
    }
    fn get_col_label_value(&self, col: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetColLabelValue(self.as_ptr(), col)).into()
        }
    }
    fn get_corner_label_value(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetCornerLabelValue(self.as_ptr())).into()
        }
    }
    fn set_row_label_value(&self, row: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxGridTableBase_SetRowLabelValue(self.as_ptr(), row, label)
        }
    }
    fn set_col_label_value(&self, col: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxGridTableBase_SetColLabelValue(self.as_ptr(), col, label)
        }
    }
    // BLOCKED: fn SetCornerLabelValue()
    fn set_attr_provider(&self, attr_provider: *mut c_void) {
        unsafe { ffi::wxGridTableBase_SetAttrProvider(self.as_ptr(), attr_provider) }
    }
    fn get_attr_provider(&self) -> *mut c_void {
        unsafe { ffi::wxGridTableBase_GetAttrProvider(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAttr()
    // NOT_SUPPORTED: fn GetAttrPtr()
    fn set_attr(&self, attr: *mut c_void, row: c_int, col: c_int) {
        unsafe { ffi::wxGridTableBase_SetAttr(self.as_ptr(), attr, row, col) }
    }
    fn set_row_attr(&self, attr: *mut c_void, row: c_int) {
        unsafe { ffi::wxGridTableBase_SetRowAttr(self.as_ptr(), attr, row) }
    }
    fn set_col_attr(&self, attr: *mut c_void, col: c_int) {
        unsafe { ffi::wxGridTableBase_SetColAttr(self.as_ptr(), attr, col) }
    }
    fn can_have_attributes(&self) -> bool {
        unsafe { ffi::wxGridTableBase_CanHaveAttributes(self.as_ptr()) }
    }
    fn can_measure_col_using_same_attr(&self, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_CanMeasureColUsingSameAttr(self.as_ptr(), col) }
    }
    // DTOR: fn ~wxGridTableBase()
    fn get_number_rows(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetNumberRows(self.as_ptr()) }
    }
    fn get_number_cols(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetNumberCols(self.as_ptr()) }
    }
    fn get_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetRowsCount(self.as_ptr()) }
    }
    fn get_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetColsCount(self.as_ptr()) }
    }
}

// wxGridUpdateLocker
pub trait GridUpdateLockerMethods: WxRustMethods {
    // DTOR: fn ~wxGridUpdateLocker()
    fn create(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridUpdateLocker_Create(self.as_ptr(), grid) }
    }
}
