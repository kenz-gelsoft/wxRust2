use super::*;

// wxGBPosition
pub trait GBPositionMethods: WxRustMethods {
    /// Get the current column value.
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetCol(self.as_ptr()) }
    }
    /// Get the current row value.
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGBPosition_GetRow(self.as_ptr()) }
    }
    /// Set a new column value.
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGBPosition_SetCol(self.as_ptr(), col) }
    }
    /// Set a new row value.
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGBPosition_SetRow(self.as_ptr(), row) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator==()
}

// wxGBSizerItem
pub trait GBSizerItemMethods: SizerItemMethods {
    /// Get the row and column of the endpoint of this item.
    fn get_end_pos(&self, row: *mut c_void, col: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetEndPos(self.as_ptr(), row, col) }
    }
    /// Get the grid position of the item.
    fn get_pos(&self) -> GBPosition {
        unsafe { GBPosition::from_ptr(ffi::wxGBSizerItem_GetPos(self.as_ptr())) }
    }
    fn get_pos_int(&self, row: *mut c_void, col: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetPos1(self.as_ptr(), row, col) }
    }
    /// Get the row and column spanning of the item.
    fn get_span(&self) -> GBSpan {
        unsafe { GBSpan::from_ptr(ffi::wxGBSizerItem_GetSpan(self.as_ptr())) }
    }
    fn get_span_int(&self, rowspan: *mut c_void, colspan: *mut c_void) {
        unsafe { ffi::wxGBSizerItem_GetSpan1(self.as_ptr(), rowspan, colspan) }
    }
    /// Returns true if this item and the other item intersect.
    fn intersects_gbsizeritem<G: GBSizerItemMethods>(&self, other: &G) -> bool {
        unsafe {
            let other = other.as_ptr();
            ffi::wxGBSizerItem_Intersects(self.as_ptr(), other)
        }
    }
    /// Returns true if the given pos/span would intersect with this item.
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
    /// If the item is already a member of a sizer then first ensure that there is no other item that would intersect with this one at the new position, then set the new position.
    fn set_pos<G: GBPositionMethods>(&self, pos: &G) -> bool {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxGBSizerItem_SetPos(self.as_ptr(), pos)
        }
    }
    /// If the item is already a member of a sizer then first ensure that there is no other item that would intersect with this one with its new spanning size, then set the new spanning.
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
    /// Get the current colspan value.
    fn get_colspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetColspan(self.as_ptr()) }
    }
    /// Get the current rowspan value.
    fn get_rowspan(&self) -> c_int {
        unsafe { ffi::wxGBSpan_GetRowspan(self.as_ptr()) }
    }
    /// Set a new colspan value.
    fn set_colspan(&self, colspan: c_int) {
        unsafe { ffi::wxGBSpan_SetColspan(self.as_ptr(), colspan) }
    }
    /// Set a new rowspan value.
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
    /// Save the animated gif.
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
    /// Creates the gauge for two-step construction.
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
    /// Returns the maximum position of the gauge.
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGauge_GetRange(self.as_ptr()) }
    }
    /// Returns the current position of the gauge.
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGauge_GetValue(self.as_ptr()) }
    }
    /// Returns true if the gauge is vertical (has wxGA_VERTICAL style) and false otherwise.
    fn is_vertical(&self) -> bool {
        unsafe { ffi::wxGauge_IsVertical(self.as_ptr()) }
    }
    /// Switch the gauge to indeterminate mode (if required) and makes the gauge move a bit to indicate the user that some progress has been made.
    fn pulse(&self) {
        unsafe { ffi::wxGauge_Pulse(self.as_ptr()) }
    }
    /// Sets the range (maximum value) of the gauge.
    fn set_range(&self, range: c_int) {
        unsafe { ffi::wxGauge_SetRange(self.as_ptr(), range) }
    }
    /// Sets the position of the gauge.
    fn set_value(&self, pos: c_int) {
        unsafe { ffi::wxGauge_SetValue(self.as_ptr(), pos) }
    }
}

// wxGenericAboutDialog
pub trait GenericAboutDialogMethods: WxRustMethods {
    /// Initializes the dialog created using the default constructor.
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
    /// Collapse the given path.
    fn collapse_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_CollapsePath(self.as_ptr(), path)
        }
    }
    /// Collapses the entire tree.
    fn collapse_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_CollapseTree(self.as_ptr()) }
    }
    /// Create function for two-step construction.
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
    /// Tries to expand as much of the given path as possible, so that the filename or directory is visible in the tree control.
    fn expand_path(&self, path: &str) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_ExpandPath(self.as_ptr(), path)
        }
    }
    /// Gets the default path.
    fn get_default_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetDefaultPath(self.as_ptr())).into() }
    }
    /// Gets selected filename path only (else empty string).
    fn get_file_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilePath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the currently selected filepaths.
    fn get_file_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetFilePaths(self.as_ptr(), paths)
        }
    }
    /// Returns the filter string.
    fn get_filter(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetFilter(self.as_ptr())).into() }
    }
    /// Returns the current filter index (zero-based).
    fn get_filter_index(&self) -> c_int {
        unsafe { ffi::wxGenericDirCtrl_GetFilterIndex(self.as_ptr()) }
    }
    /// Returns a pointer to the filter list control (if present).
    fn get_filter_list_ctrl(&self) -> *mut c_void {
        unsafe { ffi::wxGenericDirCtrl_GetFilterListCtrl(self.as_ptr()) }
    }
    /// Gets the currently-selected directory or filename.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericDirCtrl_GetPath(self.as_ptr())).into() }
    }
    // BLOCKED: fn GetPath1()
    /// Fills the array paths with the selected directories and filenames.
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Returns the root id for the tree control.
    fn get_root_id(&self) -> TreeItemId {
        unsafe { TreeItemId::from_ptr(ffi::wxGenericDirCtrl_GetRootId(self.as_ptr())) }
    }
    /// Returns a pointer to the tree control.
    fn get_tree_ctrl(&self) -> WeakRef<TreeCtrl> {
        unsafe { WeakRef::<TreeCtrl>::from(ffi::wxGenericDirCtrl_GetTreeCtrl(self.as_ptr())) }
    }
    /// Initializes variables.
    fn init(&self) {
        unsafe { ffi::wxGenericDirCtrl_Init(self.as_ptr()) }
    }
    /// Collapse and expand the tree, thus re-creating it from scratch.
    fn re_create_tree(&self) {
        unsafe { ffi::wxGenericDirCtrl_ReCreateTree(self.as_ptr()) }
    }
    /// Sets the default path.
    fn set_default_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SetDefaultPath(self.as_ptr(), path)
        }
    }
    /// Sets the filter string.
    fn set_filter(&self, filter: &str) {
        unsafe {
            let filter = WxString::from(filter);
            let filter = filter.as_ptr();
            ffi::wxGenericDirCtrl_SetFilter(self.as_ptr(), filter)
        }
    }
    /// Sets the current filter index (zero-based).
    fn set_filter_index(&self, n: c_int) {
        unsafe { ffi::wxGenericDirCtrl_SetFilterIndex(self.as_ptr(), n) }
    }
    /// Sets the current path.
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
    /// Selects the given item.
    fn select_path(&self, path: &str, select: bool) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxGenericDirCtrl_SelectPath(self.as_ptr(), path, select)
        }
    }
    /// Selects only the specified paths, clearing any previous selection.
    fn select_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxGenericDirCtrl_SelectPaths(self.as_ptr(), paths)
        }
    }
    /// Removes the selection from all currently selected items.
    fn unselect_all(&self) {
        unsafe { ffi::wxGenericDirCtrl_UnselectAll(self.as_ptr()) }
    }
}

// wxGenericProgressDialog
pub trait GenericProgressDialogMethods: DialogMethods {
    // DTOR: fn ~wxGenericProgressDialog()
    /// Returns the last value passed to the Update() function or wxNOT_FOUND if the dialog has no progress bar.
    fn get_value(&self) -> c_int {
        unsafe { ffi::wxGenericProgressDialog_GetValue(self.as_ptr()) }
    }
    /// Returns the maximum value of the progress meter, as passed to the constructor or wxNOT_FOUND if the dialog has no progress bar.
    fn get_range(&self) -> c_int {
        unsafe { ffi::wxGenericProgressDialog_GetRange(self.as_ptr()) }
    }
    /// Returns the last message passed to the Update() function; if you always passed wxEmptyString to Update() then the message set through the constructor is returned.
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGenericProgressDialog_GetMessage(self.as_ptr())).into() }
    }
    /// Like Update() but makes the gauge control run in indeterminate mode.
    fn pulse(&self, newmsg: &str, skip: *mut c_void) -> bool {
        unsafe {
            let newmsg = WxString::from(newmsg);
            let newmsg = newmsg.as_ptr();
            ffi::wxGenericProgressDialog_Pulse(self.as_ptr(), newmsg, skip)
        }
    }
    /// Can be used to continue with the dialog, after the user had clicked the "Abort" button.
    fn resume(&self) {
        unsafe { ffi::wxGenericProgressDialog_Resume(self.as_ptr()) }
    }
    /// Changes the maximum value of the progress meter given in the constructor.
    fn set_range(&self, maximum: c_int) {
        unsafe { ffi::wxGenericProgressDialog_SetRange(self.as_ptr(), maximum) }
    }
    /// Returns true if the "Cancel" button was pressed.
    fn was_cancelled(&self) -> bool {
        unsafe { ffi::wxGenericProgressDialog_WasCancelled(self.as_ptr()) }
    }
    /// Returns true if the "Skip" button was pressed.
    fn was_skipped(&self) -> bool {
        unsafe { ffi::wxGenericProgressDialog_WasSkipped(self.as_ptr()) }
    }
    /// Updates the dialog, setting the progress bar to the new value and updating the message if new one is specified.
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
    /// Creates a wxGraphicsContext from a wxWindow.
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
    /// Creates a wxGraphicsContext from a wxWindowDC.
    fn create_windowdc<W: WindowDCMethods>(window_dc: &W) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let window_dc = window_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create1(window_dc))
        }
    }
    /// Creates a wxGraphicsContext from a wxMemoryDC.
    fn create_memorydc<M: MemoryDCMethods>(memory_dc: &M) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let memory_dc = memory_dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create2(memory_dc))
        }
    }
    // BLOCKED: fn Create3()
    /// Creates a wxGraphicsContext from a wxEnhMetaFileDC.
    fn create_enhmetafiledc(meta_file_dc: *const c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_Create4(meta_file_dc)) }
    }
    /// Creates a wxGraphicsContext from a DC of unknown specific type.
    fn create_from_unknown_dc<D: DCMethods>(dc: &D) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let dc = dc.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromUnknownDC(dc))
        }
    }
    /// Creates a wxGraphicsContext associated with a wxImage.
    fn create_image<I: ImageMethods>(image: &I) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            let image = image.as_ptr();
            GraphicsContext::option_from(ffi::wxGraphicsContext_Create5(image))
        }
    }
    /// Creates a wxGraphicsContext from a native context.
    fn create_from_native(context: *mut c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromNative(context)) }
    }
    /// Creates a wxGraphicsContext from a native window.
    fn create_from_native_window(window: *mut c_void) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsContext_CreateFromNativeWindow(window))
        }
    }
    // NOT_SUPPORTED: fn CreateFromNativeHDC()
    /// Create a lightweight context that can be used only for measuring text.
    fn create() -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxGraphicsContext_Create6()) }
    }
    /// Resets the clipping to original shape.
    fn reset_clip(&self) {
        unsafe { ffi::wxGraphicsContext_ResetClip(self.as_ptr()) }
    }
    /// Sets the clipping region to the intersection of the given region and the previously set clipping region.
    fn clip<R: RegionMethods>(&self, region: &R) {
        unsafe {
            let region = region.as_ptr();
            ffi::wxGraphicsContext_Clip(self.as_ptr(), region)
        }
    }
    // NOT_SUPPORTED: fn Clip1()
    /// Returns bounding box of the current clipping region.
    fn get_clip_box(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetClipBox(self.as_ptr(), x, y, w, h) }
    }
    // NOT_SUPPORTED: fn CreateMatrix()
    /// Creates a native affine transformation matrix from the passed generic one.
    fn create_matrix<A: AffineMatrix2DBaseMethods>(&self, mat: &A) -> GraphicsMatrix {
        unsafe {
            let mat = mat.as_ptr();
            GraphicsMatrix::from_ptr(ffi::wxGraphicsContext_CreateMatrix1(self.as_ptr(), mat))
        }
    }
    /// Concatenates the passed in transform with the current transform of this context.
    fn concat_transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsContext_ConcatTransform(self.as_ptr(), matrix)
        }
    }
    /// Gets the current transformation matrix of this context.
    fn get_transform(&self) -> GraphicsMatrix {
        unsafe { GraphicsMatrix::from_ptr(ffi::wxGraphicsContext_GetTransform(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    /// Sets the current transformation matrix of this context.
    fn set_transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsContext_SetTransform(self.as_ptr(), matrix)
        }
    }
    // NOT_SUPPORTED: fn Translate()
    /// Creates a native brush from a wxBrush.
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
    /// Sets the brush for filling paths.
    fn set_brush_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxGraphicsContext_SetBrush(self.as_ptr(), brush)
        }
    }
    /// Sets the brush for filling paths.
    fn set_brush_graphicsbrush<G: GraphicsBrushMethods>(&self, brush: &G) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxGraphicsContext_SetBrush1(self.as_ptr(), brush)
        }
    }
    /// Creates a native pen from a wxPen.
    fn create_pen_pen<P: PenMethods>(&self, pen: &P) -> GraphicsPen {
        unsafe {
            let pen = pen.as_ptr();
            GraphicsPen::from_ptr(ffi::wxGraphicsContext_CreatePen(self.as_ptr(), pen))
        }
    }
    /// Creates a native pen from a wxGraphicsPenInfo.
    fn create_pen_graphicspeninfo(&self, info: *const c_void) -> GraphicsPen {
        unsafe { GraphicsPen::from_ptr(ffi::wxGraphicsContext_CreatePen1(self.as_ptr(), info)) }
    }
    /// Sets the pen used for stroking.
    fn set_pen_pen<P: PenMethods>(&self, pen: &P) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxGraphicsContext_SetPen(self.as_ptr(), pen)
        }
    }
    /// Sets the pen used for stroking.
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
    /// Creates a native graphics path which is initially empty.
    fn create_path(&self) -> GraphicsPath {
        unsafe { GraphicsPath::from_ptr(ffi::wxGraphicsContext_CreatePath(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn FillPath()
    // NOT_SUPPORTED: fn StrokeLine()
    /// Stroke disconnected lines from begin to end points, fastest method available for this purpose.
    fn stroke_lines_point2ddouble(
        &self,
        n: usize,
        begin_points: *const c_void,
        end_points: *const c_void,
    ) {
        unsafe { ffi::wxGraphicsContext_StrokeLines(self.as_ptr(), n, begin_points, end_points) }
    }
    /// Stroke lines connecting all the points.
    fn stroke_lines(&self, n: usize, points: *const c_void) {
        unsafe { ffi::wxGraphicsContext_StrokeLines1(self.as_ptr(), n, points) }
    }
    /// Strokes along a path with the current pen.
    fn stroke_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsContext_StrokePath(self.as_ptr(), path)
        }
    }
    /// Creates a native graphics font from a wxFont and a text colour.
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
    /// Creates a font object with the specified attributes.
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
    /// Sets the font for drawing text.
    fn set_font_font<F: FontMethods, C: ColourMethods>(&self, font: &F, colour: &C) {
        unsafe {
            let font = font.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxGraphicsContext_SetFont(self.as_ptr(), font, colour)
        }
    }
    /// Sets the font for drawing text.
    fn set_font_graphicsfont<G: GraphicsFontMethods>(&self, font: &G) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxGraphicsContext_SetFont1(self.as_ptr(), font)
        }
    }
    /// Fills the widths array with the widths from the beginning of text to the corresponding character of text.
    fn get_partial_text_extents(&self, text: &str, widths: *mut c_void) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxGraphicsContext_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    /// Gets the dimensions of the string using the currently selected font.
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
    /// Begin a new document (relevant only for printing / pdf etc.) If there is a progress dialog, message will be shown.
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxGraphicsContext_StartDoc(self.as_ptr(), message)
        }
    }
    /// Done with that document (relevant only for printing / pdf etc.)
    fn end_doc(&self) {
        unsafe { ffi::wxGraphicsContext_EndDoc(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn StartPage()
    /// Ends the current page (relevant only for printing / pdf etc.)
    fn end_page(&self) {
        unsafe { ffi::wxGraphicsContext_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn CreateBitmap()
    // NOT_SUPPORTED: fn CreateBitmapFromImage()
    // NOT_SUPPORTED: fn CreateSubBitmap()
    // NOT_SUPPORTED: fn BeginLayer()
    /// Composites back the drawings into the context with the opacity given at the BeginLayer() call.
    fn end_layer(&self) {
        unsafe { ffi::wxGraphicsContext_EndLayer(self.as_ptr()) }
    }
    /// Push the current state (like transformations, clipping region and quality settings) of the context on a stack.
    fn push_state(&self) {
        unsafe { ffi::wxGraphicsContext_PushState(self.as_ptr()) }
    }
    /// Sets current state of the context to the state saved by a preceding call to PushState() and removes that state from the stack of saved states.
    fn pop_state(&self) {
        unsafe { ffi::wxGraphicsContext_PopState(self.as_ptr()) }
    }
    /// Make sure that the current content of this context is immediately visible.
    fn flush(&self) {
        unsafe { ffi::wxGraphicsContext_Flush(self.as_ptr()) }
    }
    /// Returns the native context (CGContextRef for Core Graphics, Graphics pointer for GDIPlus and cairo_t pointer for cairo).
    fn get_native_context(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsContext_GetNativeContext(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetAntialiasMode()
    // NOT_SUPPORTED: fn GetAntialiasMode()
    // NOT_SUPPORTED: fn SetInterpolationQuality()
    // NOT_SUPPORTED: fn GetInterpolationQuality()
    // NOT_SUPPORTED: fn SetCompositionMode()
    // NOT_SUPPORTED: fn GetCompositionMode()
    /// Returns the size of the graphics context in device coordinates.
    fn get_size(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetSize(self.as_ptr(), width, height) }
    }
    /// Returns the resolution of the graphics context in device points per inch.
    fn get_dpi(&self, dpi_x: *mut c_void, dpi_y: *mut c_void) {
        unsafe { ffi::wxGraphicsContext_GetDPI(self.as_ptr(), dpi_x, dpi_y) }
    }
    /// Returns the associated window if any.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGraphicsContext_GetWindow(self.as_ptr())) }
    }
    /// Helper to determine if a 0.5 offset should be applied for the drawing operation.
    fn should_offset(&self) -> bool {
        unsafe { ffi::wxGraphicsContext_ShouldOffset(self.as_ptr()) }
    }
    /// Indicates whether the context should try to offset for pixel boundaries.
    fn enable_offset(&self, enable: bool) {
        unsafe { ffi::wxGraphicsContext_EnableOffset(self.as_ptr(), enable) }
    }
    /// Helper to determine if a 0.5 offset should be applied for the drawing operation.
    fn disable_offset(&self) {
        unsafe { ffi::wxGraphicsContext_DisableOffset(self.as_ptr()) }
    }
    /// Helper to determine if a 0.5 offset should be applied for the drawing operation.
    fn offset_enabled(&self) -> bool {
        unsafe { ffi::wxGraphicsContext_OffsetEnabled(self.as_ptr()) }
    }
    /// Convert DPI-independent pixel values to the value in pixels appropriate for the graphics context.
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxGraphicsContext_FromDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxGraphicsContext_FromDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert DPI-independent value in pixels to the value in pixels appropriate for the graphics context.
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxGraphicsContext_FromDIP2(self.as_ptr(), d) }
    }
    /// Convert pixel values of the current graphics context to DPI-independent pixel values.
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxGraphicsContext_ToDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxGraphicsContext_ToDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert pixel values of the current graphics context to DPI-independent pixel values.
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxGraphicsContext_ToDIP2(self.as_ptr(), d) }
    }
}

// wxGraphicsFont
pub trait GraphicsFontMethods: GraphicsObjectMethods {}

// wxGraphicsGradientStop
pub trait GraphicsGradientStopMethods: WxRustMethods {
    /// Return the stop colour.
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxGraphicsGradientStop_GetColour(self.as_ptr())) }
    }
    /// Change the stop colour.
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
    /// Add a new stop.
    fn add<G: GraphicsGradientStopMethods>(&self, stop: &G) {
        unsafe {
            let stop = stop.as_ptr();
            ffi::wxGraphicsGradientStops_Add(self.as_ptr(), stop)
        }
    }
    // NOT_SUPPORTED: fn Add1()
    // NOT_SUPPORTED: fn Item()
    /// Returns the number of stops.
    fn get_count(&self) -> usize {
        unsafe { ffi::wxGraphicsGradientStops_GetCount(self.as_ptr()) }
    }
    // BLOCKED: fn SetStartColour()
    /// Returns the start colour.
    fn get_start_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetStartColour(self.as_ptr())) }
    }
    // BLOCKED: fn SetEndColour()
    /// Returns the end colour.
    fn get_end_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxGraphicsGradientStops_GetEndColour(self.as_ptr())) }
    }
}

// wxGraphicsMatrix
pub trait GraphicsMatrixMethods: GraphicsObjectMethods {
    /// Concatenates the matrix passed with the current matrix.
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
    /// Returns the component values of the matrix via the argument pointers.
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
    /// Returns the native representation of the matrix.
    fn get_native_matrix(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsMatrix_GetNativeMatrix(self.as_ptr()) }
    }
    /// Inverts the matrix.
    fn invert(&self) {
        unsafe { ffi::wxGraphicsMatrix_Invert(self.as_ptr()) }
    }
    // BLOCKED: fn IsEqual()
    /// Returns true if the elements of the transformation matrix are equal.
    fn is_equal<G: GraphicsMatrixMethods>(&self, t: &G) -> bool {
        unsafe {
            let t = t.as_ptr();
            ffi::wxGraphicsMatrix_IsEqual1(self.as_ptr(), t)
        }
    }
    /// Return true if this is the identity matrix.
    fn is_identity(&self) -> bool {
        unsafe { ffi::wxGraphicsMatrix_IsIdentity(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Rotate()
    // NOT_SUPPORTED: fn Scale()
    // NOT_SUPPORTED: fn Set()
    /// Applies this matrix to a distance (ie.
    fn transform_distance(&self, dx: *mut c_void, dy: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformDistance(self.as_ptr(), dx, dy) }
    }
    /// Applies this matrix to a point.
    fn transform_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsMatrix_TransformPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn Translate()
}

// wxGraphicsObject
pub trait GraphicsObjectMethods: ObjectMethods {
    /// Returns the renderer that was used to create this instance, or NULL if it has not been initialized yet.
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
    /// Adds a cubic bezier curve from the current point, using two control points and an end point.
    fn add_curve_to_point(&self, c1: *const c_void, c2: *const c_void, e: *const c_void) {
        unsafe { ffi::wxGraphicsPath_AddCurveToPoint1(self.as_ptr(), c1, c2, e) }
    }
    // NOT_SUPPORTED: fn AddEllipse()
    // NOT_SUPPORTED: fn AddLineToPoint()
    /// Adds a straight line from the current point to p.
    fn add_line_to_point(&self, p: *const c_void) {
        unsafe { ffi::wxGraphicsPath_AddLineToPoint1(self.as_ptr(), p) }
    }
    /// Adds another path onto the current path.
    fn add_path<G: GraphicsPathMethods>(&self, path: &G) {
        unsafe {
            let path = path.as_ptr();
            ffi::wxGraphicsPath_AddPath(self.as_ptr(), path)
        }
    }
    // NOT_SUPPORTED: fn AddQuadCurveToPoint()
    // NOT_SUPPORTED: fn AddRectangle()
    // NOT_SUPPORTED: fn AddRoundedRectangle()
    /// Closes the current sub-path.
    fn close_subpath(&self) {
        unsafe { ffi::wxGraphicsPath_CloseSubpath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Contains()
    // NOT_SUPPORTED: fn Contains1()
    // NOT_SUPPORTED: fn GetBox()
    /// Gets the bounding box enclosing all points (possibly including control points).
    fn get_box(&self, x: *mut c_void, y: *mut c_void, w: *mut c_void, h: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetBox1(self.as_ptr(), x, y, w, h) }
    }
    /// Gets the last point of the current path, (0,0) if not yet set.
    fn get_current_point(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxGraphicsPath_GetCurrentPoint(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn GetCurrentPoint1()
    /// Returns the native path (CGPathRef for Core Graphics, Path pointer for GDIPlus and a cairo_path_t pointer for cairo).
    fn get_native_path(&self) -> *mut c_void {
        unsafe { ffi::wxGraphicsPath_GetNativePath(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn MoveToPoint()
    /// Begins a new subpath at p.
    fn move_to_point(&self, p: *const c_void) {
        unsafe { ffi::wxGraphicsPath_MoveToPoint1(self.as_ptr(), p) }
    }
    /// Transforms each point of this path by the matrix.
    fn transform<G: GraphicsMatrixMethods>(&self, matrix: &G) {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxGraphicsPath_Transform(self.as_ptr(), matrix)
        }
    }
    /// Gives back the native path returned by GetNativePath() because there might be some deallocations necessary (e.g.
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
    /// Creates a wxImage from a wxGraphicsBitmap.
    fn create_image_from_bitmap(&self, bmp: *const c_void) -> Image {
        unsafe {
            Image::from_ptr(ffi::wxGraphicsRenderer_CreateImageFromBitmap(
                self.as_ptr(),
                bmp,
            ))
        }
    }
    // NOT_SUPPORTED: fn CreateBitmapFromNativeBitmap()
    /// Creates a wxGraphicsContext from a wxWindow.
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
    /// Creates a wxGraphicsContext from a wxWindowDC.
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
    /// Creates a wxGraphicsContext from a wxMemoryDC.
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
    /// Creates a wxGraphicsContext from a wxEnhMetaFileDC.
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
    /// Creates a wxGraphicsContext from a DC of unknown specific type.
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
    /// Creates a wxGraphicsContext associated with a wxImage.
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
    /// Creates a native brush from a wxBrush.
    fn create_brush<B: BrushMethods>(&self, brush: &B) -> GraphicsBrush {
        unsafe {
            let brush = brush.as_ptr();
            GraphicsBrush::from_ptr(ffi::wxGraphicsRenderer_CreateBrush(self.as_ptr(), brush))
        }
    }
    /// Creates a wxGraphicsContext from a native context.
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
    /// Creates a wxGraphicsContext from a native window.
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
    /// Creates a wxGraphicsContext that can be used for measuring texts only.
    fn create_measuring_context(&self) -> Option<GraphicsContextIsOwned<false>> {
        unsafe {
            GraphicsContext::option_from(ffi::wxGraphicsRenderer_CreateMeasuringContext(
                self.as_ptr(),
            ))
        }
    }
    /// Creates a native graphics font from a wxFont and a text colour.
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
    /// Creates a graphics font with the given characteristics.
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
    /// Creates a native graphics font from a wxFont and a text colour.
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
    /// Creates a native graphics path which is initially empty.
    fn create_path(&self) -> GraphicsPath {
        unsafe { GraphicsPath::from_ptr(ffi::wxGraphicsRenderer_CreatePath(self.as_ptr())) }
    }
    /// Creates a native pen from its description.
    fn create_pen(&self, info: *const c_void) -> GraphicsPen {
        unsafe { GraphicsPen::from_ptr(ffi::wxGraphicsRenderer_CreatePen(self.as_ptr(), info)) }
    }
    // NOT_SUPPORTED: fn CreateRadialGradientBrush()
    // NOT_SUPPORTED: fn CreateSubBitmap()
    /// Returns the name of the technology used by the renderer.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxGraphicsRenderer_GetName(self.as_ptr())).into() }
    }
    /// Returns the version major, minor and micro/build of the technology used by the renderer.
    fn get_version(&self, major: *mut c_void, minor: *mut c_void, micro: *mut c_void) {
        unsafe { ffi::wxGraphicsRenderer_GetVersion(self.as_ptr(), major, minor, micro) }
    }
    // NOT_SUPPORTED: fn CreateContextFromNativeHDC()
    /// Returns the default renderer on this platform.
    fn get_default_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetDefaultRenderer()) }
    }
    /// Returns Cairo renderer.
    fn get_cairo_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetCairoRenderer()) }
    }
    /// Returns GDI+ renderer (MSW only).
    fn get_gdi_plus_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetGDIPlusRenderer()) }
    }
    /// Returns Direct2D renderer (MSW only).
    fn get_direct2_d_renderer() -> Option<GraphicsRendererIsOwned<false>> {
        unsafe { GraphicsRenderer::option_from(ffi::wxGraphicsRenderer_GetDirect2DRenderer()) }
    }
}

// wxGridBagSizer
pub trait GridBagSizerMethods: FlexGridSizerMethods {
    /// Adds the given item to the given position.
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
    /// Adds a spacer to the given position.
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
    /// Look at all items and see if any intersect (or would overlap) the given item.
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
    /// Find the sizer item for the given window or subsizer, returns NULL if not found.
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
    /// Return the sizer item located at the point given in pt, or NULL if there is no item at that point.
    fn find_item_at_point<P: PointMethods>(&self, pt: &P) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let pt = pt.as_ptr();
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemAtPoint(self.as_ptr(), pt))
        }
    }
    /// Return the sizer item for the given grid cell, or NULL if there is no item at that position.
    fn find_item_at_position<G: GBPositionMethods>(
        &self,
        pos: &G,
    ) -> Option<GBSizerItemIsOwned<false>> {
        unsafe {
            let pos = pos.as_ptr();
            GBSizerItem::option_from(ffi::wxGridBagSizer_FindItemAtPosition(self.as_ptr(), pos))
        }
    }
    /// Return the sizer item that has a matching user data (it only compares pointer values) or NULL if not found.
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
    /// Get the size of the specified cell, including hgap and vgap.
    fn get_cell_size(&self, row: c_int, col: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetCellSize(self.as_ptr(), row, col)) }
    }
    /// Get the size used for cells in the grid with no item.
    fn get_empty_cell_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxGridBagSizer_GetEmptyCellSize(self.as_ptr())) }
    }
    /// Get the grid position of the specified item.
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
    /// Get the row/col spanning of the specified item.
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
    /// Set the size used for cells in the grid with no item.
    fn set_empty_cell_size<S: SizeMethods>(&self, sz: &S) {
        unsafe {
            let sz = sz.as_ptr();
            ffi::wxGridBagSizer_SetEmptyCellSize(self.as_ptr(), sz)
        }
    }
    /// Set the grid position of the specified item.
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
    /// Set the row/col spanning of the specified item.
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
    /// Returns the column at which the event occurred.
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetCol(self.as_ptr()) }
    }
    /// Returns the edit control.
    fn get_control(&self) -> WeakRef<Control> {
        unsafe { WeakRef::<Control>::from(ffi::wxGridEditorCreatedEvent_GetControl(self.as_ptr())) }
    }
    /// Returns the row at which the event occurred.
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEditorCreatedEvent_GetRow(self.as_ptr()) }
    }
    /// Returns the edit window.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxGridEditorCreatedEvent_GetWindow(self.as_ptr())) }
    }
    /// Sets the column at which the event occurred.
    fn set_col(&self, col: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetCol(self.as_ptr(), col) }
    }
    /// Sets the edit control.
    fn set_control<C: ControlMethods>(&self, ctrl: Option<&C>) {
        unsafe {
            let ctrl = match ctrl {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxGridEditorCreatedEvent_SetControl(self.as_ptr(), ctrl)
        }
    }
    /// Sets the row at which the event occurred.
    fn set_row(&self, row: c_int) {
        unsafe { ffi::wxGridEditorCreatedEvent_SetRow(self.as_ptr(), row) }
    }
    /// Sets the edit window.
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
    /// Returns true if the Alt key was down at the time of the event.
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_AltDown(self.as_ptr()) }
    }
    /// Returns true if the Control key was down at the time of the event.
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ControlDown(self.as_ptr()) }
    }
    /// Column at which the event occurred.
    fn get_col(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetCol(self.as_ptr()) }
    }
    /// Position in pixels at which the event occurred.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridEvent_GetPosition(self.as_ptr())) }
    }
    /// Row at which the event occurred.
    fn get_row(&self) -> c_int {
        unsafe { ffi::wxGridEvent_GetRow(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the user is selecting grid cells, or false if deselecting.
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridEvent_Selecting(self.as_ptr()) }
    }
    /// Returns true if the Shift key was down at the time of the event.
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridRangeSelectEvent
pub trait GridRangeSelectEventMethods: NotifyEventMethods {
    /// Returns true if the Alt key was down at the time of the event.
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_AltDown(self.as_ptr()) }
    }
    /// Returns true if the Control key was down at the time of the event.
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ControlDown(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetBottomRightCoords()
    /// Bottom row of the rectangular area that was (de)selected.
    fn get_bottom_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetBottomRow(self.as_ptr()) }
    }
    /// Left column of the rectangular area that was (de)selected.
    fn get_left_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetLeftCol(self.as_ptr()) }
    }
    /// Right column of the rectangular area that was (de)selected.
    fn get_right_col(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetRightCol(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetTopLeftCoords()
    /// Top row of the rectangular area that was (de)selected.
    fn get_top_row(&self) -> c_int {
        unsafe { ffi::wxGridRangeSelectEvent_GetTopRow(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the area was selected, false otherwise.
    fn selecting(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_Selecting(self.as_ptr()) }
    }
    /// Returns true if the Shift key was down at the time of the event.
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridRangeSelectEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizeEvent
pub trait GridSizeEventMethods: NotifyEventMethods {
    /// Returns true if the Alt key was down at the time of the event.
    fn alt_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_AltDown(self.as_ptr()) }
    }
    /// Returns true if the Control key was down at the time of the event.
    fn control_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ControlDown(self.as_ptr()) }
    }
    /// Position in pixels at which the event occurred.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxGridSizeEvent_GetPosition(self.as_ptr())) }
    }
    /// Row or column at that was resized.
    fn get_row_or_col(&self) -> c_int {
        unsafe { ffi::wxGridSizeEvent_GetRowOrCol(self.as_ptr()) }
    }
    /// Returns true if the Meta key was down at the time of the event.
    fn meta_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_MetaDown(self.as_ptr()) }
    }
    /// Returns true if the Shift key was down at the time of the event.
    fn shift_down(&self) -> bool {
        unsafe { ffi::wxGridSizeEvent_ShiftDown(self.as_ptr()) }
    }
}

// wxGridSizer
pub trait GridSizerMethods: SizerMethods {
    /// Returns the number of columns that has been specified for the sizer.
    fn get_cols(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetCols(self.as_ptr()) }
    }
    /// Returns the number of rows that has been specified for the sizer.
    fn get_rows(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetRows(self.as_ptr()) }
    }
    /// Returns the number of columns currently used by the sizer.
    fn get_effective_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveColsCount(self.as_ptr()) }
    }
    /// Returns the number of rows currently used by the sizer.
    fn get_effective_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetEffectiveRowsCount(self.as_ptr()) }
    }
    /// Returns the horizontal gap (in pixels) between cells in the sizer.
    fn get_h_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetHGap(self.as_ptr()) }
    }
    /// Returns the vertical gap (in pixels) between the cells in the sizer.
    fn get_v_gap(&self) -> c_int {
        unsafe { ffi::wxGridSizer_GetVGap(self.as_ptr()) }
    }
    /// Sets the number of columns in the sizer.
    fn set_cols(&self, cols: c_int) {
        unsafe { ffi::wxGridSizer_SetCols(self.as_ptr(), cols) }
    }
    /// Sets the horizontal gap (in pixels) between cells in the sizer.
    fn set_h_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetHGap(self.as_ptr(), gap) }
    }
    /// Sets the number of rows in the sizer.
    fn set_rows(&self, rows: c_int) {
        unsafe { ffi::wxGridSizer_SetRows(self.as_ptr(), rows) }
    }
    /// Sets the vertical gap (in pixels) between the cells in the sizer.
    fn set_v_gap(&self, gap: c_int) {
        unsafe { ffi::wxGridSizer_SetVGap(self.as_ptr(), gap) }
    }
}

// wxGridTableBase
pub trait GridTableBaseMethods: ObjectMethods {
    /// May be overridden to implement testing for empty cells.
    fn is_empty_cell(&self, row: c_int, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_IsEmptyCell(self.as_ptr(), row, col) }
    }
    /// Same as IsEmptyCell() but taking wxGridCellCoords.
    fn is_empty(&self, coords: *const c_void) -> bool {
        unsafe { ffi::wxGridTableBase_IsEmpty(self.as_ptr(), coords) }
    }
    /// Must be overridden to implement accessing the table values as text.
    fn get_value(&self, row: c_int, col: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxGridTableBase_GetValue(self.as_ptr(), row, col)).into() }
    }
    /// Must be overridden to implement setting the table values as text.
    fn set_value(&self, row: c_int, col: c_int, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxGridTableBase_SetValue(self.as_ptr(), row, col, value)
        }
    }
    /// Returns the type of the value in the given cell.
    fn get_type_name(&self, row: c_int, col: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetTypeName(self.as_ptr(), row, col)).into()
        }
    }
    /// Returns true if the value of the given cell can be accessed as if it were of the specified type.
    fn can_get_value_as(&self, row: c_int, col: c_int, type_name: &str) -> bool {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_CanGetValueAs(self.as_ptr(), row, col, type_name)
        }
    }
    /// Returns true if the value of the given cell can be set as if it were of the specified type.
    fn can_set_value_as(&self, row: c_int, col: c_int, type_name: &str) -> bool {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_CanSetValueAs(self.as_ptr(), row, col, type_name)
        }
    }
    /// Returns the value of the given cell as a long.
    fn get_value_as_long(&self, row: c_int, col: c_int) -> c_long {
        unsafe { ffi::wxGridTableBase_GetValueAsLong(self.as_ptr(), row, col) }
    }
    /// Returns the value of the given cell as a double.
    fn get_value_as_double(&self, row: c_int, col: c_int) -> c_double {
        unsafe { ffi::wxGridTableBase_GetValueAsDouble(self.as_ptr(), row, col) }
    }
    /// Returns the value of the given cell as a boolean.
    fn get_value_as_bool(&self, row: c_int, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_GetValueAsBool(self.as_ptr(), row, col) }
    }
    /// Returns the value of the given cell as a user-defined type.
    fn get_value_as_custom(&self, row: c_int, col: c_int, type_name: &str) -> *mut c_void {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_GetValueAsCustom(self.as_ptr(), row, col, type_name)
        }
    }
    /// Sets the value of the given cell as a long.
    fn set_value_as_long(&self, row: c_int, col: c_int, value: c_long) {
        unsafe { ffi::wxGridTableBase_SetValueAsLong(self.as_ptr(), row, col, value) }
    }
    /// Sets the value of the given cell as a double.
    fn set_value_as_double(&self, row: c_int, col: c_int, value: c_double) {
        unsafe { ffi::wxGridTableBase_SetValueAsDouble(self.as_ptr(), row, col, value) }
    }
    /// Sets the value of the given cell as a boolean.
    fn set_value_as_bool(&self, row: c_int, col: c_int, value: bool) {
        unsafe { ffi::wxGridTableBase_SetValueAsBool(self.as_ptr(), row, col, value) }
    }
    /// Sets the value of the given cell as a user-defined type.
    fn set_value_as_custom(&self, row: c_int, col: c_int, type_name: &str, value: *mut c_void) {
        unsafe {
            let type_name = WxString::from(type_name);
            let type_name = type_name.as_ptr();
            ffi::wxGridTableBase_SetValueAsCustom(self.as_ptr(), row, col, type_name, value)
        }
    }
    /// Called by the grid when the table is associated with it.
    fn set_view(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridTableBase_SetView(self.as_ptr(), grid) }
    }
    /// Returns the last grid passed to SetView().
    fn get_view(&self) -> *mut c_void {
        unsafe { ffi::wxGridTableBase_GetView(self.as_ptr()) }
    }
    /// Clear the table contents.
    fn clear(&self) {
        unsafe { ffi::wxGridTableBase_Clear(self.as_ptr()) }
    }
    /// Insert additional rows into the table.
    fn insert_rows(&self, pos: usize, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_InsertRows(self.as_ptr(), pos, num_rows) }
    }
    /// Append additional rows at the end of the table.
    fn append_rows(&self, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_AppendRows(self.as_ptr(), num_rows) }
    }
    /// Delete rows from the table.
    fn delete_rows(&self, pos: usize, num_rows: usize) -> bool {
        unsafe { ffi::wxGridTableBase_DeleteRows(self.as_ptr(), pos, num_rows) }
    }
    /// Exactly the same as InsertRows() but for columns.
    fn insert_cols(&self, pos: usize, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_InsertCols(self.as_ptr(), pos, num_cols) }
    }
    /// Exactly the same as AppendRows() but for columns.
    fn append_cols(&self, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_AppendCols(self.as_ptr(), num_cols) }
    }
    /// Exactly the same as DeleteRows() but for columns.
    fn delete_cols(&self, pos: usize, num_cols: usize) -> bool {
        unsafe { ffi::wxGridTableBase_DeleteCols(self.as_ptr(), pos, num_cols) }
    }
    /// Return the label of the specified row.
    fn get_row_label_value(&self, row: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetRowLabelValue(self.as_ptr(), row)).into()
        }
    }
    /// Return the label of the specified column.
    fn get_col_label_value(&self, col: c_int) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetColLabelValue(self.as_ptr(), col)).into()
        }
    }
    /// Return the label of the grid's corner.
    fn get_corner_label_value(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxGridTableBase_GetCornerLabelValue(self.as_ptr())).into()
        }
    }
    /// Set the given label for the specified row.
    fn set_row_label_value(&self, row: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxGridTableBase_SetRowLabelValue(self.as_ptr(), row, label)
        }
    }
    /// Exactly the same as SetRowLabelValue() but for columns.
    fn set_col_label_value(&self, col: c_int, label: &str) {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            ffi::wxGridTableBase_SetColLabelValue(self.as_ptr(), col, label)
        }
    }
    // BLOCKED: fn SetCornerLabelValue()
    /// Associate this attributes provider with the table.
    fn set_attr_provider(&self, attr_provider: *mut c_void) {
        unsafe { ffi::wxGridTableBase_SetAttrProvider(self.as_ptr(), attr_provider) }
    }
    /// Returns the attribute provider currently being used.
    fn get_attr_provider(&self) -> *mut c_void {
        unsafe { ffi::wxGridTableBase_GetAttrProvider(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAttr()
    // NOT_SUPPORTED: fn GetAttrPtr()
    /// Set attribute of the specified cell.
    fn set_attr(&self, attr: *mut c_void, row: c_int, col: c_int) {
        unsafe { ffi::wxGridTableBase_SetAttr(self.as_ptr(), attr, row, col) }
    }
    /// Set attribute of the specified row.
    fn set_row_attr(&self, attr: *mut c_void, row: c_int) {
        unsafe { ffi::wxGridTableBase_SetRowAttr(self.as_ptr(), attr, row) }
    }
    /// Set attribute of the specified column.
    fn set_col_attr(&self, attr: *mut c_void, col: c_int) {
        unsafe { ffi::wxGridTableBase_SetColAttr(self.as_ptr(), attr, col) }
    }
    /// Returns true if this table supports attributes or false otherwise.
    fn can_have_attributes(&self) -> bool {
        unsafe { ffi::wxGridTableBase_CanHaveAttributes(self.as_ptr()) }
    }
    /// Override to return true if the same attribute can be used for measuring all cells in the given column.
    fn can_measure_col_using_same_attr(&self, col: c_int) -> bool {
        unsafe { ffi::wxGridTableBase_CanMeasureColUsingSameAttr(self.as_ptr(), col) }
    }
    // DTOR: fn ~wxGridTableBase()
    /// Must be overridden to return the number of rows in the table.
    fn get_number_rows(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetNumberRows(self.as_ptr()) }
    }
    /// Must be overridden to return the number of columns in the table.
    fn get_number_cols(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetNumberCols(self.as_ptr()) }
    }
    /// Return the number of rows in the table.
    fn get_rows_count(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetRowsCount(self.as_ptr()) }
    }
    /// Return the number of columns in the table.
    fn get_cols_count(&self) -> c_int {
        unsafe { ffi::wxGridTableBase_GetColsCount(self.as_ptr()) }
    }
}

// wxGridUpdateLocker
pub trait GridUpdateLockerMethods: WxRustMethods {
    // DTOR: fn ~wxGridUpdateLocker()
    /// This method can be called if the object had been constructed using the default constructor.
    fn create(&self, grid: *mut c_void) {
        unsafe { ffi::wxGridUpdateLocker_Create(self.as_ptr(), grid) }
    }
}
