use super::*;

// wxDC
pub trait DCMethods: ObjectMethods {
    fn device_to_logical_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalX(self.as_ptr(), x) }
    }
    fn device_to_logical_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalXRel(self.as_ptr(), x) }
    }
    fn device_to_logical_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalY(self.as_ptr(), y) }
    }
    fn device_to_logical_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalYRel(self.as_ptr(), y) }
    }
    fn logical_to_device_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceX(self.as_ptr(), x) }
    }
    fn logical_to_device_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceXRel(self.as_ptr(), x) }
    }
    fn logical_to_device_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceY(self.as_ptr(), y) }
    }
    fn logical_to_device_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceYRel(self.as_ptr(), y) }
    }
    fn device_to_logical_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_DeviceToLogical(self.as_ptr(), x, y)) }
    }
    fn device_to_logical_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_DeviceToLogical1(self.as_ptr(), pt))
        }
    }
    fn device_to_logical_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_DeviceToLogicalRel(self.as_ptr(), x, y)) }
    }
    fn device_to_logical_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_DeviceToLogicalRel1(self.as_ptr(), dim))
        }
    }
    fn logical_to_device_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_LogicalToDevice(self.as_ptr(), x, y)) }
    }
    fn logical_to_device_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_LogicalToDevice1(self.as_ptr(), pt))
        }
    }
    fn logical_to_device_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_LogicalToDeviceRel(self.as_ptr(), x, y)) }
    }
    fn logical_to_device_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_LogicalToDeviceRel1(self.as_ptr(), dim))
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxDC_Clear(self.as_ptr()) }
    }
    fn draw_arc_coord(
        &self,
        x_start: c_int,
        y_start: c_int,
        x_end: c_int,
        y_end: c_int,
        xc: c_int,
        yc: c_int,
    ) {
        unsafe { ffi::wxDC_DrawArc(self.as_ptr(), x_start, y_start, x_end, y_end, xc, yc) }
    }
    fn draw_arc_point<P: PointMethods, P2: PointMethods, P3: PointMethods>(
        &self,
        pt_start: &P,
        pt_end: &P2,
        centre: &P3,
    ) {
        unsafe {
            let pt_start = pt_start.as_ptr();
            let pt_end = pt_end.as_ptr();
            let centre = centre.as_ptr();
            ffi::wxDC_DrawArc1(self.as_ptr(), pt_start, pt_end, centre)
        }
    }
    fn draw_bitmap_coord<B: BitmapMethods>(&self, bitmap: &B, x: c_int, y: c_int, use_mask: bool) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDC_DrawBitmap(self.as_ptr(), bitmap, x, y, use_mask)
        }
    }
    fn draw_bitmap_point<B: BitmapMethods, P: PointMethods>(
        &self,
        bmp: &B,
        pt: &P,
        use_mask: bool,
    ) {
        unsafe {
            let bmp = bmp.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawBitmap1(self.as_ptr(), bmp, pt, use_mask)
        }
    }
    fn draw_check_mark_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawCheckMark(self.as_ptr(), x, y, width, height) }
    }
    fn draw_check_mark_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawCheckMark1(self.as_ptr(), rect)
        }
    }
    fn draw_circle_coord(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { ffi::wxDC_DrawCircle(self.as_ptr(), x, y, radius) }
    }
    fn draw_circle_point<P: PointMethods>(&self, pt: &P, radius: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawCircle1(self.as_ptr(), pt, radius)
        }
    }
    fn draw_ellipse_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawEllipse(self.as_ptr(), x, y, width, height) }
    }
    fn draw_ellipse_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, size: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let size = size.as_ptr();
            ffi::wxDC_DrawEllipse1(self.as_ptr(), pt, size)
        }
    }
    fn draw_ellipse_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawEllipse2(self.as_ptr(), rect)
        }
    }
    fn draw_elliptic_arc_coord(
        &self,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        start: c_double,
        end: c_double,
    ) {
        unsafe { ffi::wxDC_DrawEllipticArc(self.as_ptr(), x, y, width, height, start, end) }
    }
    fn draw_elliptic_arc_point<P: PointMethods, S: SizeMethods>(
        &self,
        pt: &P,
        sz: &S,
        sa: c_double,
        ea: c_double,
    ) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawEllipticArc1(self.as_ptr(), pt, sz, sa, ea)
        }
    }
    fn draw_icon_coord<I: IconMethods>(&self, icon: &I, x: c_int, y: c_int) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDC_DrawIcon(self.as_ptr(), icon, x, y)
        }
    }
    fn draw_icon_point<I: IconMethods, P: PointMethods>(&self, icon: &I, pt: &P) {
        unsafe {
            let icon = icon.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawIcon1(self.as_ptr(), icon, pt)
        }
    }
    fn draw_label_bitmap<B: BitmapMethods, R: RectMethods, R2: RectMethods>(
        &self,
        text: &str,
        bitmap: &B,
        rect: &R,
        alignment: c_int,
        index_accel: c_int,
        rect_bounding: Option<&R2>,
    ) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let bitmap = bitmap.as_ptr();
            let rect = rect.as_ptr();
            let rect_bounding = match rect_bounding {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_DrawLabel(
                self.as_ptr(),
                text,
                bitmap,
                rect,
                alignment,
                index_accel,
                rect_bounding,
            )
        }
    }
    fn draw_label_rect<R: RectMethods>(
        &self,
        text: &str,
        rect: &R,
        alignment: c_int,
        index_accel: c_int,
    ) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxDC_DrawLabel1(self.as_ptr(), text, rect, alignment, index_accel)
        }
    }
    fn draw_line_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { ffi::wxDC_DrawLine(self.as_ptr(), x1, y1, x2, y2) }
    }
    fn draw_line_point<P: PointMethods, P2: PointMethods>(&self, pt1: &P, pt2: &P2) {
        unsafe {
            let pt1 = pt1.as_ptr();
            let pt2 = pt2.as_ptr();
            ffi::wxDC_DrawLine1(self.as_ptr(), pt1, pt2)
        }
    }
    // NOT_SUPPORTED: fn DrawLines()
    fn draw_lines(&self, points: *const c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { ffi::wxDC_DrawLines1(self.as_ptr(), points, xoffset, yoffset) }
    }
    fn draw_point_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_DrawPoint(self.as_ptr(), x, y) }
    }
    fn draw_point_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawPoint1(self.as_ptr(), pt)
        }
    }
    // NOT_SUPPORTED: fn DrawPolygon()
    // NOT_SUPPORTED: fn DrawPolygon1()
    // NOT_SUPPORTED: fn DrawPolyPolygon()
    fn draw_rectangle_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawRectangle(self.as_ptr(), x, y, width, height) }
    }
    fn draw_rectangle_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawRectangle1(self.as_ptr(), pt, sz)
        }
    }
    fn draw_rectangle_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRectangle2(self.as_ptr(), rect)
        }
    }
    fn draw_rotated_text_coord(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawRotatedText(self.as_ptr(), text, x, y, angle)
        }
    }
    fn draw_rotated_text_point<P: PointMethods>(&self, text: &str, point: &P, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let point = point.as_ptr();
            ffi::wxDC_DrawRotatedText1(self.as_ptr(), text, point, angle)
        }
    }
    fn draw_rounded_rectangle_coord(
        &self,
        x: c_int,
        y: c_int,
        width: c_int,
        height: c_int,
        radius: c_double,
    ) {
        unsafe { ffi::wxDC_DrawRoundedRectangle(self.as_ptr(), x, y, width, height, radius) }
    }
    fn draw_rounded_rectangle_point<P: PointMethods, S: SizeMethods>(
        &self,
        pt: &P,
        sz: &S,
        radius: c_double,
    ) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawRoundedRectangle1(self.as_ptr(), pt, sz, radius)
        }
    }
    fn draw_rounded_rectangle_rect<R: RectMethods>(&self, rect: &R, radius: c_double) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRoundedRectangle2(self.as_ptr(), rect, radius)
        }
    }
    // NOT_SUPPORTED: fn DrawSpline()
    fn draw_spline_pointlist(&self, points: *const c_void) {
        unsafe { ffi::wxDC_DrawSpline1(self.as_ptr(), points) }
    }
    fn draw_spline_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int) {
        unsafe { ffi::wxDC_DrawSpline2(self.as_ptr(), x1, y1, x2, y2, x3, y3) }
    }
    fn draw_text_coord(&self, text: &str, x: c_int, y: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawText(self.as_ptr(), text, x, y)
        }
    }
    fn draw_text_point<P: PointMethods>(&self, text: &str, pt: &P) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawText1(self.as_ptr(), text, pt)
        }
    }
    fn gradient_fill_concentric<R: RectMethods, C: ColourMethods, C2: ColourMethods>(
        &self,
        rect: &R,
        initial_colour: &C,
        dest_colour: &C2,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            let initial_colour = initial_colour.as_ptr();
            let dest_colour = dest_colour.as_ptr();
            ffi::wxDC_GradientFillConcentric(self.as_ptr(), rect, initial_colour, dest_colour)
        }
    }
    fn gradient_fill_concentric_point<
        R: RectMethods,
        C: ColourMethods,
        C2: ColourMethods,
        P: PointMethods,
    >(
        &self,
        rect: &R,
        initial_colour: &C,
        dest_colour: &C2,
        circle_center: &P,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            let initial_colour = initial_colour.as_ptr();
            let dest_colour = dest_colour.as_ptr();
            let circle_center = circle_center.as_ptr();
            ffi::wxDC_GradientFillConcentric1(
                self.as_ptr(),
                rect,
                initial_colour,
                dest_colour,
                circle_center,
            )
        }
    }
    fn gradient_fill_linear<R: RectMethods, C: ColourMethods, C2: ColourMethods>(
        &self,
        rect: &R,
        initial_colour: &C,
        dest_colour: &C2,
        n_direction: c_int,
    ) {
        unsafe {
            let rect = rect.as_ptr();
            let initial_colour = initial_colour.as_ptr();
            let dest_colour = dest_colour.as_ptr();
            ffi::wxDC_GradientFillLinear(
                self.as_ptr(),
                rect,
                initial_colour,
                dest_colour,
                n_direction,
            )
        }
    }
    // NOT_SUPPORTED: fn FloodFill()
    // NOT_SUPPORTED: fn FloodFill1()
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CrossHair(self.as_ptr(), x, y) }
    }
    fn cross_hair_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_CrossHair1(self.as_ptr(), pt)
        }
    }
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxDC_DestroyClippingRegion(self.as_ptr()) }
    }
    fn get_clipping_box_coord(
        &self,
        x: *mut c_void,
        y: *mut c_void,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxDC_GetClippingBox(self.as_ptr(), x, y, width, height) }
    }
    fn get_clipping_box_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_GetClippingBox1(self.as_ptr(), rect)
        }
    }
    fn set_clipping_region_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_SetClippingRegion(self.as_ptr(), x, y, width, height) }
    }
    fn set_clipping_region_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_SetClippingRegion1(self.as_ptr(), pt, sz)
        }
    }
    fn set_clipping_region_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_SetClippingRegion2(self.as_ptr(), rect)
        }
    }
    fn set_device_clipping_region(&self, region: *const c_void) {
        unsafe { ffi::wxDC_SetDeviceClippingRegion(self.as_ptr(), region) }
    }
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharHeight(self.as_ptr()) }
    }
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontMetrics()
    fn get_multi_line_text_extent_coord<F: FontMethods>(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        height_line: *mut c_void,
        font: Option<&F>,
    ) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            let font = match font {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetMultiLineTextExtent(self.as_ptr(), string, w, h, height_line, font)
        }
    }
    fn get_multi_line_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetMultiLineTextExtent1(self.as_ptr(), string))
        }
    }
    fn get_partial_text_extents<A: ArrayIntMethods>(&self, text: &str, widths: &A) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let widths = widths.as_ptr();
            ffi::wxDC_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    fn get_text_extent_coord<F: FontMethods>(
        &self,
        string: &str,
        w: *mut c_void,
        h: *mut c_void,
        descent: *mut c_void,
        external_leading: *mut c_void,
        font: Option<&F>,
    ) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            let font = match font {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetTextExtent(self.as_ptr(), string, w, h, descent, external_leading, font)
        }
    }
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetTextExtent1(self.as_ptr(), string))
        }
    }
    fn get_background_mode(&self) -> c_int {
        unsafe { ffi::wxDC_GetBackgroundMode(self.as_ptr()) }
    }
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxDC_GetFont(self.as_ptr())) }
    }
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxDC_GetLayoutDirection(self.as_ptr()) }
    }
    fn get_text_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextBackground(self.as_ptr())) }
    }
    fn get_text_foreground(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextForeground(self.as_ptr())) }
    }
    fn set_background_mode(&self, mode: c_int) {
        unsafe { ffi::wxDC_SetBackgroundMode(self.as_ptr(), mode) }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDC_SetFont(self.as_ptr(), font)
        }
    }
    fn set_text_background<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextBackground(self.as_ptr(), colour)
        }
    }
    fn set_text_foreground<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextForeground(self.as_ptr(), colour)
        }
    }
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxDC_SetLayoutDirection(self.as_ptr(), dir) }
    }
    fn calc_bounding_box(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CalcBoundingBox(self.as_ptr(), x, y) }
    }
    fn max_x(&self) -> c_int {
        unsafe { ffi::wxDC_MaxX(self.as_ptr()) }
    }
    fn max_y(&self) -> c_int {
        unsafe { ffi::wxDC_MaxY(self.as_ptr()) }
    }
    fn min_x(&self) -> c_int {
        unsafe { ffi::wxDC_MinX(self.as_ptr()) }
    }
    fn min_y(&self) -> c_int {
        unsafe { ffi::wxDC_MinY(self.as_ptr()) }
    }
    fn reset_bounding_box(&self) {
        unsafe { ffi::wxDC_ResetBoundingBox(self.as_ptr()) }
    }
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDC_StartDoc(self.as_ptr(), message)
        }
    }
    fn start_page(&self) {
        unsafe { ffi::wxDC_StartPage(self.as_ptr()) }
    }
    fn end_doc(&self) {
        unsafe { ffi::wxDC_EndDoc(self.as_ptr()) }
    }
    fn end_page(&self) {
        unsafe { ffi::wxDC_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Blit()
    // NOT_SUPPORTED: fn StretchBlit()
    fn get_background(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBackground(self.as_ptr())) }
    }
    fn get_brush(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBrush(self.as_ptr())) }
    }
    // BLOCKED: fn GetPen()
    fn set_background<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBackground(self.as_ptr(), brush)
        }
    }
    fn set_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBrush(self.as_ptr(), brush)
        }
    }
    fn set_pen(&self, pen: *const c_void) {
        unsafe { ffi::wxDC_SetPen(self.as_ptr(), pen) }
    }
    fn copy_attributes<D: DCMethods>(&self, dc: &D) {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxDC_CopyAttributes(self.as_ptr(), dc)
        }
    }
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDC_GetContentScaleFactor(self.as_ptr()) }
    }
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxDC_GetDepth(self.as_ptr()) }
    }
    fn get_device_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetDeviceOrigin(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetLogicalFunction()
    // NOT_SUPPORTED: fn GetMapMode()
    fn get_pixel<C: ColourMethods>(&self, x: c_int, y: c_int, colour: Option<&C>) -> bool {
        unsafe {
            let colour = match colour {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetPixel(self.as_ptr(), x, y, colour)
        }
    }
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetPPI(self.as_ptr())) }
    }
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_FromDIP(self.as_ptr(), sz))
        }
    }
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_FromDIP1(self.as_ptr(), pt))
        }
    }
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_FromDIP2(self.as_ptr(), d) }
    }
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_ToDIP(self.as_ptr(), sz))
        }
    }
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_ToDIP1(self.as_ptr(), pt))
        }
    }
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_ToDIP2(self.as_ptr(), d) }
    }
    fn get_size_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSize1(self.as_ptr())) }
    }
    fn get_size_mm_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSizeMM(self.as_ptr(), width, height) }
    }
    fn get_size_mm(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSizeMM1(self.as_ptr())) }
    }
    fn get_user_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetUserScale(self.as_ptr(), x, y) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDC_IsOk(self.as_ptr()) }
    }
    fn set_axis_orientation(&self, x_left_right: bool, y_bottom_up: bool) {
        unsafe { ffi::wxDC_SetAxisOrientation(self.as_ptr(), x_left_right, y_bottom_up) }
    }
    fn set_device_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetDeviceOrigin(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn SetMapMode()
    fn set_palette(&self, palette: *const c_void) {
        unsafe { ffi::wxDC_SetPalette(self.as_ptr(), palette) }
    }
    fn set_user_scale(&self, x_scale: c_double, y_scale: c_double) {
        unsafe { ffi::wxDC_SetUserScale(self.as_ptr(), x_scale, y_scale) }
    }
    fn can_use_transform_matrix(&self) -> bool {
        unsafe { ffi::wxDC_CanUseTransformMatrix(self.as_ptr()) }
    }
    fn set_transform_matrix<A: AffineMatrix2DMethods>(&self, matrix: &A) -> bool {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxDC_SetTransformMatrix(self.as_ptr(), matrix)
        }
    }
    fn get_transform_matrix(&self) -> AffineMatrix2D {
        unsafe { AffineMatrix2D::from_ptr(ffi::wxDC_GetTransformMatrix(self.as_ptr())) }
    }
    fn reset_transform_matrix(&self) {
        unsafe { ffi::wxDC_ResetTransformMatrix(self.as_ptr()) }
    }
    fn can_draw_bitmap(&self) -> bool {
        unsafe { ffi::wxDC_CanDrawBitmap(self.as_ptr()) }
    }
    fn can_get_text_extent(&self) -> bool {
        unsafe { ffi::wxDC_CanGetTextExtent(self.as_ptr()) }
    }
    fn get_handle(&self) -> *mut c_void {
        unsafe { ffi::wxDC_GetHandle(self.as_ptr()) }
    }
    fn get_as_bitmap<R: RectMethods>(&self, subrect: Option<&R>) -> Bitmap {
        unsafe {
            let subrect = match subrect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Bitmap::from_ptr(ffi::wxDC_GetAsBitmap(self.as_ptr(), subrect))
        }
    }
    fn set_logical_scale(&self, x: c_double, y: c_double) {
        unsafe { ffi::wxDC_SetLogicalScale(self.as_ptr(), x, y) }
    }
    fn get_logical_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalScale(self.as_ptr(), x, y) }
    }
    fn set_logical_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetLogicalOrigin(self.as_ptr(), x, y) }
    }
    fn get_logical_origin_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalOrigin(self.as_ptr(), x, y) }
    }
    fn get_logical_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetLogicalOrigin1(self.as_ptr())) }
    }
    fn get_graphics_context(&self) -> *mut c_void {
        unsafe { ffi::wxDC_GetGraphicsContext(self.as_ptr()) }
    }
    fn set_graphics_context(&self, ctx: *mut c_void) {
        unsafe { ffi::wxDC_SetGraphicsContext(self.as_ptr(), ctx) }
    }
}

// wxDCBrushChanger
pub trait DCBrushChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCBrushChanger()
}

// wxDCClipper
pub trait DCClipperMethods: WxRustMethods {
    // DTOR: fn ~wxDCClipper()
}

// wxDCFontChanger
pub trait DCFontChangerMethods: WxRustMethods {
    fn set<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDCFontChanger_Set(self.as_ptr(), font)
        }
    }
    // DTOR: fn ~wxDCFontChanger()
}

// wxDCOverlay
pub trait DCOverlayMethods: WxRustMethods {
    // DTOR: fn ~wxDCOverlay()
    fn clear(&self) {
        unsafe { ffi::wxDCOverlay_Clear(self.as_ptr()) }
    }
}

// wxDCPenChanger
pub trait DCPenChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCPenChanger()
}

// wxDCTextBgColourChanger
pub trait DCTextBgColourChangerMethods: WxRustMethods {
    fn set<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxDCTextBgColourChanger_Set(self.as_ptr(), col)
        }
    }
    // DTOR: fn ~wxDCTextBgColourChanger()
}

// wxDCTextBgModeChanger
pub trait DCTextBgModeChangerMethods: WxRustMethods {}

// wxDCTextColourChanger
pub trait DCTextColourChangerMethods: WxRustMethods {
    fn set<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxDCTextColourChanger_Set(self.as_ptr(), col)
        }
    }
    // DTOR: fn ~wxDCTextColourChanger()
}

// wxDPIChangedEvent
pub trait DPIChangedEventMethods: EventMethods {
    fn get_old_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDPIChangedEvent_GetOldDPI(self.as_ptr())) }
    }
    fn get_new_dpi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDPIChangedEvent_GetNewDPI(self.as_ptr())) }
    }
    // BLOCKED: fn Scale()
    fn scale_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDPIChangedEvent_ScaleX(self.as_ptr(), x) }
    }
    fn scale_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDPIChangedEvent_ScaleY(self.as_ptr(), y) }
    }
}

// wxDataFormat
pub trait DataFormatMethods: WxRustMethods {
    fn get_id(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataFormat_GetId(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    fn set_id(&self, format: &str) {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            ffi::wxDataFormat_SetId(self.as_ptr(), format)
        }
    }
    // NOT_SUPPORTED: fn SetType()
    // BLOCKED: fn operator!=()
    // NOT_SUPPORTED: fn operator!=1()
    // BLOCKED: fn operator==()
    // NOT_SUPPORTED: fn operator==1()
}

// wxDataObject
pub trait DataObjectMethods: WxRustMethods {
    // DTOR: fn ~wxDataObject()
    // NOT_SUPPORTED: fn GetAllFormats()
    fn get_data_here<D: DataFormatMethods>(&self, format: &D, buf: *mut c_void) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_GetDataHere(self.as_ptr(), format, buf)
        }
    }
    fn get_data_size<D: DataFormatMethods>(&self, format: &D) -> usize {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_GetDataSize(self.as_ptr(), format)
        }
    }
    // NOT_SUPPORTED: fn GetFormatCount()
    // NOT_SUPPORTED: fn GetPreferredFormat()
    fn set_data<D: DataFormatMethods>(&self, format: &D, len: usize, buf: *const c_void) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_SetData(self.as_ptr(), format, len, buf)
        }
    }
    // NOT_SUPPORTED: fn IsSupported()
}

// wxDataObjectComposite
pub trait DataObjectCompositeMethods: DataObjectMethods {
    fn add<D: DataObjectSimpleMethods>(&self, data_object: Option<&D>, preferred: bool) {
        unsafe {
            let data_object = match data_object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataObjectComposite_Add(self.as_ptr(), data_object, preferred)
        }
    }
    fn get_received_format(&self) -> DataFormat {
        unsafe { DataFormat::from_ptr(ffi::wxDataObjectComposite_GetReceivedFormat(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetObject()
}

// wxDataObjectSimple
pub trait DataObjectSimpleMethods: DataObjectMethods {
    fn get_data_here_void(&self, buf: *mut c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_GetDataHere(self.as_ptr(), buf) }
    }
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataObjectSimple_GetDataSize(self.as_ptr()) }
    }
    // BLOCKED: fn GetFormat()
    fn set_data_sz(&self, len: usize, buf: *const c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_SetData(self.as_ptr(), len, buf) }
    }
    fn set_format<D: DataFormatMethods>(&self, format: &D) {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObjectSimple_SetFormat(self.as_ptr(), format)
        }
    }
}

// wxDataViewBitmapRenderer
pub trait DataViewBitmapRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewBitmapRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewCheckIconTextRenderer
pub trait DataViewCheckIconTextRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewCheckIconTextRenderer_GetDefaultType()).into() }
    }
    fn allow3rd_state_for_user(&self, allow: bool) {
        unsafe { ffi::wxDataViewCheckIconTextRenderer_Allow3rdStateForUser(self.as_ptr(), allow) }
    }
}

// wxDataViewChoiceByIndexRenderer
pub trait DataViewChoiceByIndexRendererMethods: DataViewChoiceRendererMethods {}

// wxDataViewChoiceRenderer
pub trait DataViewChoiceRendererMethods: DataViewRendererMethods {
    fn get_choice(&self, index: usize) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxDataViewChoiceRenderer_GetChoice(
                self.as_ptr(),
                index,
            ))
            .into()
        }
    }
    fn get_choices(&self) -> ArrayStringIsOwned<false> {
        unsafe {
            ArrayStringIsOwned::from_ptr(ffi::wxDataViewChoiceRenderer_GetChoices(self.as_ptr()))
        }
    }
}

// wxDataViewColumn
pub trait DataViewColumnMethods: SettableHeaderColumnMethods {
    fn get_model_column(&self) -> c_uint {
        unsafe { ffi::wxDataViewColumn_GetModelColumn(self.as_ptr()) }
    }
    fn get_owner(&self) -> WeakRef<DataViewCtrl> {
        unsafe { WeakRef::<DataViewCtrl>::from(ffi::wxDataViewColumn_GetOwner(self.as_ptr())) }
    }
    fn get_renderer(&self) -> Option<DataViewRendererIsOwned<false>> {
        unsafe { DataViewRenderer::option_from(ffi::wxDataViewColumn_GetRenderer(self.as_ptr())) }
    }
}

// wxDataViewCtrl
pub trait DataViewCtrlMethods: ControlMethods {
    // DTOR: fn ~wxDataViewCtrl()
    fn allow_multi_column_sort(&self, allow: bool) -> bool {
        unsafe { ffi::wxDataViewCtrl_AllowMultiColumnSort(self.as_ptr(), allow) }
    }
    fn append_column<D: DataViewColumnMethods>(&self, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_AppendColumn(self.as_ptr(), col)
        }
    }
    fn prepend_column<D: DataViewColumnMethods>(&self, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_PrependColumn(self.as_ptr(), col)
        }
    }
    fn insert_column<D: DataViewColumnMethods>(&self, pos: c_uint, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_InsertColumn(self.as_ptr(), pos, col)
        }
    }
    // NOT_SUPPORTED: fn AppendBitmapColumn()
    // NOT_SUPPORTED: fn AppendBitmapColumn1()
    // NOT_SUPPORTED: fn PrependBitmapColumn()
    // NOT_SUPPORTED: fn PrependBitmapColumn1()
    // NOT_SUPPORTED: fn AppendDateColumn()
    // NOT_SUPPORTED: fn AppendDateColumn1()
    // NOT_SUPPORTED: fn PrependDateColumn()
    // NOT_SUPPORTED: fn PrependDateColumn1()
    // NOT_SUPPORTED: fn AppendIconTextColumn()
    // NOT_SUPPORTED: fn AppendIconTextColumn1()
    // NOT_SUPPORTED: fn PrependIconTextColumn()
    // NOT_SUPPORTED: fn PrependIconTextColumn1()
    // NOT_SUPPORTED: fn AppendProgressColumn()
    // NOT_SUPPORTED: fn AppendProgressColumn1()
    // NOT_SUPPORTED: fn PrependProgressColumn()
    // NOT_SUPPORTED: fn PrependProgressColumn1()
    // NOT_SUPPORTED: fn AppendTextColumn()
    // NOT_SUPPORTED: fn AppendTextColumn1()
    // NOT_SUPPORTED: fn PrependTextColumn()
    // NOT_SUPPORTED: fn PrependTextColumn1()
    // NOT_SUPPORTED: fn AppendToggleColumn()
    // NOT_SUPPORTED: fn AppendToggleColumn1()
    // NOT_SUPPORTED: fn PrependToggleColumn()
    // NOT_SUPPORTED: fn PrependToggleColumn1()
    fn associate_model<D: DataViewModelMethods>(&self, model: Option<&D>) -> bool {
        unsafe {
            let model = match model {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_AssociateModel(self.as_ptr(), model)
        }
    }
    fn clear_columns(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_ClearColumns(self.as_ptr()) }
    }
    fn collapse<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Collapse(self.as_ptr(), item)
        }
    }
    fn delete_column<D: DataViewColumnMethods>(&self, column: Option<&D>) -> bool {
        unsafe {
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_DeleteColumn(self.as_ptr(), column)
        }
    }
    fn edit_item<D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        item: &D,
        column: Option<&D2>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_EditItem(self.as_ptr(), item, column)
        }
    }
    fn enable_drag_source<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewCtrl_EnableDragSource(self.as_ptr(), format)
        }
    }
    fn enable_drop_targets(&self, formats: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_EnableDropTargets(self.as_ptr(), formats) }
    }
    fn enable_drop_target<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewCtrl_EnableDropTarget(self.as_ptr(), format)
        }
    }
    fn ensure_visible<D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        item: &D,
        column: Option<&D2>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_EnsureVisible(self.as_ptr(), item, column)
        }
    }
    fn expand<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Expand(self.as_ptr(), item)
        }
    }
    fn expand_ancestors<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandAncestors(self.as_ptr(), item)
        }
    }
    fn expand_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandChildren(self.as_ptr(), item)
        }
    }
    fn get_column(&self, pos: c_uint) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetColumn(self.as_ptr(), pos)) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn get_column_position<D: DataViewColumnMethods>(&self, column: Option<&D>) -> c_int {
        unsafe {
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_GetColumnPosition(self.as_ptr(), column)
        }
    }
    fn get_expander_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetExpanderColumn(self.as_ptr())) }
    }
    fn get_current_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetCurrentItem(self.as_ptr())) }
    }
    fn get_current_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetCurrentColumn(self.as_ptr())) }
    }
    fn get_indent(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetIndent(self.as_ptr()) }
    }
    fn get_item_rect<D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        item: &D,
        col: Option<&D2>,
    ) -> Rect {
        unsafe {
            let item = item.as_ptr();
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Rect::from_ptr(ffi::wxDataViewCtrl_GetItemRect(self.as_ptr(), item, col))
        }
    }
    fn get_main_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDataViewCtrl_GetMainWindow(self.as_ptr())) }
    }
    fn get_model(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewCtrl_GetModel(self.as_ptr())) }
    }
    fn get_selected_items_count(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelectedItemsCount(self.as_ptr()) }
    }
    fn get_selection(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetSelection(self.as_ptr())) }
    }
    fn get_selections(&self, sel: *mut c_void) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelections(self.as_ptr(), sel) }
    }
    fn get_sorting_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetSortingColumn(self.as_ptr())) }
    }
    // BLOCKED: fn GetSortingColumns()
    fn has_selection(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_HasSelection(self.as_ptr()) }
    }
    fn hit_test<P: PointMethods, D: DataViewItemMethods, D2: DataViewColumnMethods>(
        &self,
        point: &P,
        item: &D,
        col: Option<&D2>,
    ) {
        unsafe {
            let point = point.as_ptr();
            let item = item.as_ptr();
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_HitTest(self.as_ptr(), point, item, col)
        }
    }
    fn is_expanded<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsExpanded(self.as_ptr(), item)
        }
    }
    fn is_multi_column_sort_allowed(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_IsMultiColumnSortAllowed(self.as_ptr()) }
    }
    fn is_selected<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsSelected(self.as_ptr(), item)
        }
    }
    fn select<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Select(self.as_ptr(), item)
        }
    }
    fn select_all(&self) {
        unsafe { ffi::wxDataViewCtrl_SelectAll(self.as_ptr()) }
    }
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    fn set_expander_column<D: DataViewColumnMethods>(&self, col: Option<&D>) {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_SetExpanderColumn(self.as_ptr(), col)
        }
    }
    fn set_current_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_SetCurrentItem(self.as_ptr(), item)
        }
    }
    fn set_header_attr(&self, attr: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetHeaderAttr(self.as_ptr(), attr) }
    }
    fn set_indent(&self, indent: c_int) {
        unsafe { ffi::wxDataViewCtrl_SetIndent(self.as_ptr(), indent) }
    }
    fn set_selections(&self, sel: *const c_void) {
        unsafe { ffi::wxDataViewCtrl_SetSelections(self.as_ptr(), sel) }
    }
    fn unselect<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Unselect(self.as_ptr(), item)
        }
    }
    fn unselect_all(&self) {
        unsafe { ffi::wxDataViewCtrl_UnselectAll(self.as_ptr()) }
    }
    fn set_row_height(&self, row_height: c_int) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetRowHeight(self.as_ptr(), row_height) }
    }
    fn toggle_sort_by_column(&self, column: c_int) {
        unsafe { ffi::wxDataViewCtrl_ToggleSortByColumn(self.as_ptr(), column) }
    }
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetCountPerPage(self.as_ptr()) }
    }
    fn get_top_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetTopItem(self.as_ptr())) }
    }
}

// wxDataViewCustomRenderer
pub trait DataViewCustomRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewCustomRenderer_GetDefaultType()).into() }
    }
    // DTOR: fn ~wxDataViewCustomRenderer()
    fn activate_cell<R: RectMethods, D: DataViewModelMethods, D2: DataViewItemMethods>(
        &self,
        cell: &R,
        model: Option<&D>,
        item: &D2,
        col: c_uint,
        mouse_event: *const c_void,
    ) -> bool {
        unsafe {
            let cell = cell.as_ptr();
            let model = match model {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let item = item.as_ptr();
            ffi::wxDataViewCustomRenderer_ActivateCell(
                self.as_ptr(),
                cell,
                model,
                item,
                col,
                mouse_event,
            )
        }
    }
    fn get_attr(&self) -> DataViewItemAttrIsOwned<false> {
        unsafe {
            DataViewItemAttrIsOwned::from_ptr(ffi::wxDataViewCustomRenderer_GetAttr(self.as_ptr()))
        }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDataViewCustomRenderer_GetSize(self.as_ptr())) }
    }
    // BLOCKED: fn LeftClick()
    // BLOCKED: fn Activate()
    // BLOCKED: fn Render()
    // BLOCKED: fn RenderText()
    fn start_drag<
        P: PointMethods,
        R: RectMethods,
        D: DataViewModelMethods,
        D2: DataViewItemMethods,
    >(
        &self,
        cursor: &P,
        cell: &R,
        model: Option<&D>,
        item: &D2,
        col: c_uint,
    ) -> bool {
        unsafe {
            let cursor = cursor.as_ptr();
            let cell = cell.as_ptr();
            let model = match model {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let item = item.as_ptr();
            ffi::wxDataViewCustomRenderer_StartDrag(self.as_ptr(), cursor, cell, model, item, col)
        }
    }
}

// wxDataViewDateRenderer
pub trait DataViewDateRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewDateRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewEvent
pub trait DataViewEventMethods: NotifyEventMethods {
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetColumn(self.as_ptr()) }
    }
    fn get_data_view_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe {
            DataViewColumn::option_from(ffi::wxDataViewEvent_GetDataViewColumn(self.as_ptr()))
        }
    }
    fn get_model(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewEvent_GetModel(self.as_ptr())) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDataViewEvent_GetPosition(self.as_ptr())) }
    }
    // BLOCKED: fn GetValue()
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxDataViewEvent_IsEditCancelled(self.as_ptr()) }
    }
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxDataViewEvent_SetColumn(self.as_ptr(), col) }
    }
    // BLOCKED: fn SetDataViewColumn()
    // BLOCKED: fn SetModel()
    fn set_value(&self, value: *const c_void) {
        unsafe { ffi::wxDataViewEvent_SetValue(self.as_ptr(), value) }
    }
    fn set_data_object<D: DataObjectMethods>(&self, obj: Option<&D>) {
        unsafe {
            let obj = match obj {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewEvent_SetDataObject(self.as_ptr(), obj)
        }
    }
    fn get_data_format(&self) -> DataFormat {
        unsafe { DataFormat::from_ptr(ffi::wxDataViewEvent_GetDataFormat(self.as_ptr())) }
    }
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataViewEvent_GetDataSize(self.as_ptr()) }
    }
    fn get_data_buffer(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewEvent_GetDataBuffer(self.as_ptr()) }
    }
    fn set_drag_flags(&self, flags: c_int) {
        unsafe { ffi::wxDataViewEvent_SetDragFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetDropEffect()
    fn get_cache_from(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetCacheFrom(self.as_ptr()) }
    }
    fn get_cache_to(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetCacheTo(self.as_ptr()) }
    }
    fn get_proposed_drop_index(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetProposedDropIndex(self.as_ptr()) }
    }
    fn get_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewEvent_GetItem(self.as_ptr())) }
    }
    // BLOCKED: fn SetItem()
    fn set_position(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDataViewEvent_SetPosition(self.as_ptr(), x, y) }
    }
    fn set_cache(&self, from: c_int, to: c_int) {
        unsafe { ffi::wxDataViewEvent_SetCache(self.as_ptr(), from, to) }
    }
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDataViewEvent_GetDataObject(self.as_ptr())) }
    }
    fn set_data_format<D: DataFormatMethods>(&self, format: &D) {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewEvent_SetDataFormat(self.as_ptr(), format)
        }
    }
    fn set_data_size(&self, size: usize) {
        unsafe { ffi::wxDataViewEvent_SetDataSize(self.as_ptr(), size) }
    }
    fn set_data_buffer(&self, buf: *mut c_void) {
        unsafe { ffi::wxDataViewEvent_SetDataBuffer(self.as_ptr(), buf) }
    }
    fn get_drag_flags(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetDragFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetDropEffect()
}

// wxDataViewIconText
pub trait DataViewIconTextMethods: ObjectMethods {
    fn get_bitmap_bundle(&self) -> BitmapBundleIsOwned<false> {
        unsafe {
            BitmapBundleIsOwned::from_ptr(ffi::wxDataViewIconText_GetBitmapBundle(self.as_ptr()))
        }
    }
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxDataViewIconText_GetIcon(self.as_ptr())) }
    }
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconText_GetText(self.as_ptr())).into() }
    }
    fn set_bitmap_bundle<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDataViewIconText_SetBitmapBundle(self.as_ptr(), bitmap)
        }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDataViewIconText_SetIcon(self.as_ptr(), icon)
        }
    }
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDataViewIconText_SetText(self.as_ptr(), text)
        }
    }
}

// wxDataViewIconTextRenderer
pub trait DataViewIconTextRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconTextRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewIndexListModel
pub trait DataViewIndexListModelMethods: DataViewListModelMethods {
    fn get_item(&self, row: c_uint) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewIndexListModel_GetItem(self.as_ptr(), row)) }
    }
    fn reset(&self, new_size: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_Reset(self.as_ptr(), new_size) }
    }
    fn row_appended(&self) {
        unsafe { ffi::wxDataViewIndexListModel_RowAppended(self.as_ptr()) }
    }
    fn row_changed(&self, row: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowChanged(self.as_ptr(), row) }
    }
    fn row_deleted(&self, row: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowDeleted(self.as_ptr(), row) }
    }
    fn row_inserted(&self, before: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowInserted(self.as_ptr(), before) }
    }
    fn row_prepended(&self) {
        unsafe { ffi::wxDataViewIndexListModel_RowPrepended(self.as_ptr()) }
    }
    fn row_value_changed(&self, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowValueChanged(self.as_ptr(), row, col) }
    }
    fn rows_deleted<A: ArrayIntMethods>(&self, rows: &A) {
        unsafe {
            let rows = rows.as_ptr();
            ffi::wxDataViewIndexListModel_RowsDeleted(self.as_ptr(), rows)
        }
    }
}

// wxDataViewItem
pub trait DataViewItemMethods: WxRustMethods {
    fn get_id(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewItem_GetID(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDataViewItem_IsOk(self.as_ptr()) }
    }
}

// wxDataViewItemAttr
pub trait DataViewItemAttrMethods: WxRustMethods {
    fn set_bold(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetBold(self.as_ptr(), set) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetColour(self.as_ptr(), colour)
        }
    }
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    fn set_italic(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetItalic(self.as_ptr(), set) }
    }
    fn set_strikethrough(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetStrikethrough(self.as_ptr(), set) }
    }
    fn has_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasColour(self.as_ptr()) }
    }
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetColour(self.as_ptr())) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasFont(self.as_ptr()) }
    }
    fn get_bold(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetBold(self.as_ptr()) }
    }
    fn get_italic(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetItalic(self.as_ptr()) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    fn is_default(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_IsDefault(self.as_ptr()) }
    }
    fn get_effective_font<F: FontMethods>(&self, font: &F) -> Font {
        unsafe {
            let font = font.as_ptr();
            Font::from_ptr(ffi::wxDataViewItemAttr_GetEffectiveFont(
                self.as_ptr(),
                font,
            ))
        }
    }
}

// wxDataViewListCtrl
pub trait DataViewListCtrlMethods: DataViewCtrlMethods {
    fn get_selected_row(&self) -> c_int {
        unsafe { ffi::wxDataViewListCtrl_GetSelectedRow(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SelectRow()
    // NOT_SUPPORTED: fn UnselectRow()
    // NOT_SUPPORTED: fn IsRowSelected()
    // BLOCKED: fn AppendColumn()
    // NOT_SUPPORTED: fn AppendTextColumn()
    // NOT_SUPPORTED: fn AppendToggleColumn()
    // NOT_SUPPORTED: fn AppendProgressColumn()
    // NOT_SUPPORTED: fn AppendIconTextColumn()
    // BLOCKED: fn InsertColumn()
    // BLOCKED: fn PrependColumn()
    // NOT_SUPPORTED: fn AppendItem()
    // NOT_SUPPORTED: fn PrependItem()
    // NOT_SUPPORTED: fn InsertItem()
    // NOT_SUPPORTED: fn DeleteItem()
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn get_item_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListCtrl_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    fn set_value(&self, value: *const c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_SetValue(self.as_ptr(), value, row, col) }
    }
    fn get_value(&self, value: *mut c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_GetValue(self.as_ptr(), value, row, col) }
    }
    fn set_text_value(&self, value: &str, row: c_uint, col: c_uint) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxDataViewListCtrl_SetTextValue(self.as_ptr(), value, row, col)
        }
    }
    fn get_text_value(&self, row: c_uint, col: c_uint) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxDataViewListCtrl_GetTextValue(
                self.as_ptr(),
                row,
                col,
            ))
            .into()
        }
    }
    fn set_toggle_value(&self, value: bool, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_SetToggleValue(self.as_ptr(), value, row, col) }
    }
    fn get_toggle_value(&self, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListCtrl_GetToggleValue(self.as_ptr(), row, col) }
    }
    // NOT_SUPPORTED: fn SetItemData()
    // DTOR: fn ~wxDataViewListCtrl()
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            ffi::wxDataViewListCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator)
        }
    }
    // BLOCKED: fn GetStore()
    fn get_store(&self) -> Option<DataViewListStoreIsOwned<false>> {
        unsafe { DataViewListStore::option_from(ffi::wxDataViewListCtrl_GetStore1(self.as_ptr())) }
    }
    fn item_to_row<D: DataViewItemMethods>(&self, item: &D) -> c_int {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewListCtrl_ItemToRow(self.as_ptr(), item)
        }
    }
    fn row_to_item(&self, row: c_int) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewListCtrl_RowToItem(self.as_ptr(), row)) }
    }
}

// wxDataViewListModel
pub trait DataViewListModelMethods: DataViewModelMethods {
    // DTOR: fn ~wxDataViewListModel()
    fn get_attr_by_row<D: DataViewItemAttrMethods>(
        &self,
        row: c_uint,
        col: c_uint,
        attr: &D,
    ) -> bool {
        unsafe {
            let attr = attr.as_ptr();
            ffi::wxDataViewListModel_GetAttrByRow(self.as_ptr(), row, col, attr)
        }
    }
    fn is_enabled_by_row(&self, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListModel_IsEnabledByRow(self.as_ptr(), row, col) }
    }
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListModel_GetCount(self.as_ptr()) }
    }
    fn get_row<D: DataViewItemMethods>(&self, item: &D) -> c_uint {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewListModel_GetRow(self.as_ptr(), item)
        }
    }
    fn get_value_by_row(&self, variant: *mut c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListModel_GetValueByRow(self.as_ptr(), variant, row, col) }
    }
    fn set_value_by_row(&self, variant: *const c_void, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListModel_SetValueByRow(self.as_ptr(), variant, row, col) }
    }
}

// wxDataViewListStore
pub trait DataViewListStoreMethods: DataViewIndexListModelMethods {
    // DTOR: fn ~wxDataViewListStore()
    fn prepend_column(&self, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_PrependColumn(self.as_ptr(), varianttype)
        }
    }
    fn insert_column(&self, pos: c_uint, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_InsertColumn(self.as_ptr(), pos, varianttype)
        }
    }
    fn append_column(&self, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_AppendColumn(self.as_ptr(), varianttype)
        }
    }
    // NOT_SUPPORTED: fn AppendItem()
    // NOT_SUPPORTED: fn PrependItem()
    // NOT_SUPPORTED: fn InsertItem()
    // NOT_SUPPORTED: fn DeleteItem()
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewListStore_DeleteAllItems(self.as_ptr()) }
    }
    fn get_item_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListStore_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    // NOT_SUPPORTED: fn SetItemData()
}

// wxDataViewModel
pub trait DataViewModelMethods: RefCounterMethods {
    fn add_notifier<D: DataViewModelNotifierMethods>(&self, notifier: Option<&D>) {
        unsafe {
            let notifier = match notifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewModel_AddNotifier(self.as_ptr(), notifier)
        }
    }
    fn change_value<D: DataViewItemMethods>(
        &self,
        variant: *const c_void,
        item: &D,
        col: c_uint,
    ) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_ChangeValue(self.as_ptr(), variant, item, col)
        }
    }
    fn cleared(&self) -> bool {
        unsafe { ffi::wxDataViewModel_Cleared(self.as_ptr()) }
    }
    fn compare<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        item1: &D,
        item2: &D2,
        column: c_uint,
        ascending: bool,
    ) -> c_int {
        unsafe {
            let item1 = item1.as_ptr();
            let item2 = item2.as_ptr();
            ffi::wxDataViewModel_Compare(self.as_ptr(), item1, item2, column, ascending)
        }
    }
    fn get_attr<D: DataViewItemMethods, D2: DataViewItemAttrMethods>(
        &self,
        item: &D,
        col: c_uint,
        attr: &D2,
    ) -> bool {
        unsafe {
            let item = item.as_ptr();
            let attr = attr.as_ptr();
            ffi::wxDataViewModel_GetAttr(self.as_ptr(), item, col, attr)
        }
    }
    fn is_enabled<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_IsEnabled(self.as_ptr(), item, col)
        }
    }
    fn get_children<D: DataViewItemMethods>(&self, item: &D, children: *mut c_void) -> c_uint {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_GetChildren(self.as_ptr(), item, children)
        }
    }
    fn get_parent<D: DataViewItemMethods>(&self, item: &D) -> DataViewItem {
        unsafe {
            let item = item.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewModel_GetParent(self.as_ptr(), item))
        }
    }
    fn get_value<D: DataViewItemMethods>(&self, variant: *mut c_void, item: &D, col: c_uint) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_GetValue(self.as_ptr(), variant, item, col)
        }
    }
    fn has_container_columns<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_HasContainerColumns(self.as_ptr(), item)
        }
    }
    fn has_default_compare(&self) -> bool {
        unsafe { ffi::wxDataViewModel_HasDefaultCompare(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HasValue()
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_IsContainer(self.as_ptr(), item)
        }
    }
    fn item_added<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        parent: &D,
        item: &D2,
    ) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            let item = item.as_ptr();
            ffi::wxDataViewModel_ItemAdded(self.as_ptr(), parent, item)
        }
    }
    fn item_changed<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_ItemChanged(self.as_ptr(), item)
        }
    }
    fn item_deleted<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        parent: &D,
        item: &D2,
    ) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            let item = item.as_ptr();
            ffi::wxDataViewModel_ItemDeleted(self.as_ptr(), parent, item)
        }
    }
    fn items_added<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModel_ItemsAdded(self.as_ptr(), parent, items)
        }
    }
    fn items_changed(&self, items: *const c_void) -> bool {
        unsafe { ffi::wxDataViewModel_ItemsChanged(self.as_ptr(), items) }
    }
    fn items_deleted<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModel_ItemsDeleted(self.as_ptr(), parent, items)
        }
    }
    fn remove_notifier<D: DataViewModelNotifierMethods>(&self, notifier: Option<&D>) {
        unsafe {
            let notifier = match notifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewModel_RemoveNotifier(self.as_ptr(), notifier)
        }
    }
    fn resort(&self) {
        unsafe { ffi::wxDataViewModel_Resort(self.as_ptr()) }
    }
    fn set_value<D: DataViewItemMethods>(
        &self,
        variant: *const c_void,
        item: &D,
        col: c_uint,
    ) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_SetValue(self.as_ptr(), variant, item, col)
        }
    }
    fn value_changed<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_ValueChanged(self.as_ptr(), item, col)
        }
    }
    fn is_list_model(&self) -> bool {
        unsafe { ffi::wxDataViewModel_IsListModel(self.as_ptr()) }
    }
    fn is_virtual_list_model(&self) -> bool {
        unsafe { ffi::wxDataViewModel_IsVirtualListModel(self.as_ptr()) }
    }
}

// wxDataViewModelNotifier
pub trait DataViewModelNotifierMethods: WxRustMethods {
    // DTOR: fn ~wxDataViewModelNotifier()
    fn cleared(&self) -> bool {
        unsafe { ffi::wxDataViewModelNotifier_Cleared(self.as_ptr()) }
    }
    fn get_owner(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewModelNotifier_GetOwner(self.as_ptr())) }
    }
    fn item_added<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        parent: &D,
        item: &D2,
    ) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ItemAdded(self.as_ptr(), parent, item)
        }
    }
    fn item_changed<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ItemChanged(self.as_ptr(), item)
        }
    }
    fn item_deleted<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        parent: &D,
        item: &D2,
    ) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ItemDeleted(self.as_ptr(), parent, item)
        }
    }
    fn items_added<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModelNotifier_ItemsAdded(self.as_ptr(), parent, items)
        }
    }
    fn items_changed(&self, items: *const c_void) -> bool {
        unsafe { ffi::wxDataViewModelNotifier_ItemsChanged(self.as_ptr(), items) }
    }
    fn items_deleted<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModelNotifier_ItemsDeleted(self.as_ptr(), parent, items)
        }
    }
    fn resort(&self) {
        unsafe { ffi::wxDataViewModelNotifier_Resort(self.as_ptr()) }
    }
    fn set_owner<D: DataViewModelMethods>(&self, owner: Option<&D>) {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewModelNotifier_SetOwner(self.as_ptr(), owner)
        }
    }
    fn value_changed<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ValueChanged(self.as_ptr(), item, col)
        }
    }
}

// wxDataViewProgressRenderer
pub trait DataViewProgressRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewProgressRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewRenderer
pub trait DataViewRendererMethods: ObjectMethods {
    fn enable_ellipsize(&self, mode: c_int) {
        unsafe { ffi::wxDataViewRenderer_EnableEllipsize(self.as_ptr(), mode) }
    }
    fn disable_ellipsize(&self) {
        unsafe { ffi::wxDataViewRenderer_DisableEllipsize(self.as_ptr()) }
    }
    // BLOCKED: fn GetAccessibleDescription()
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxDataViewRenderer_GetAlignment(self.as_ptr()) }
    }
    fn get_ellipsize_mode(&self) -> c_int {
        unsafe { ffi::wxDataViewRenderer_GetEllipsizeMode(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetMode()
    fn get_owner(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewRenderer_GetOwner(self.as_ptr())) }
    }
    fn get_value(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_GetValue(self.as_ptr(), value) }
    }
    fn get_variant_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewRenderer_GetVariantType(self.as_ptr())).into() }
    }
    fn is_compatible_variant_type(&self, variant_type: &str) -> bool {
        unsafe {
            let variant_type = WxString::from(variant_type);
            let variant_type = variant_type.as_ptr();
            ffi::wxDataViewRenderer_IsCompatibleVariantType(self.as_ptr(), variant_type)
        }
    }
    fn set_alignment(&self, align: c_int) {
        unsafe { ffi::wxDataViewRenderer_SetAlignment(self.as_ptr(), align) }
    }
    fn set_owner<D: DataViewColumnMethods>(&self, owner: Option<&D>) {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewRenderer_SetOwner(self.as_ptr(), owner)
        }
    }
    fn set_value(&self, value: *const c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_SetValue(self.as_ptr(), value) }
    }
    fn set_value_adjuster<D: DataViewValueAdjusterMethods>(&self, transformer: Option<&D>) {
        unsafe {
            let transformer = match transformer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewRenderer_SetValueAdjuster(self.as_ptr(), transformer)
        }
    }
    fn validate(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_Validate(self.as_ptr(), value) }
    }
    fn has_editor_ctrl(&self) -> bool {
        unsafe { ffi::wxDataViewRenderer_HasEditorCtrl(self.as_ptr()) }
    }
    // BLOCKED: fn CreateEditorCtrl()
    fn get_value_from_editor_ctrl<W: WindowMethods>(
        &self,
        editor: Option<&W>,
        value: *mut c_void,
    ) -> bool {
        unsafe {
            let editor = match editor {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewRenderer_GetValueFromEditorCtrl(self.as_ptr(), editor, value)
        }
    }
    // BLOCKED: fn StartEditing()
    fn cancel_editing(&self) {
        unsafe { ffi::wxDataViewRenderer_CancelEditing(self.as_ptr()) }
    }
    fn finish_editing(&self) -> bool {
        unsafe { ffi::wxDataViewRenderer_FinishEditing(self.as_ptr()) }
    }
    fn get_editor_ctrl(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDataViewRenderer_GetEditorCtrl(self.as_ptr())) }
    }
}

// wxDataViewSpinRenderer
pub trait DataViewSpinRendererMethods: DataViewCustomRendererMethods {}

// wxDataViewTextRenderer
pub trait DataViewTextRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewTextRenderer_GetDefaultType()).into() }
    }
    fn enable_markup(&self, enable: bool) {
        unsafe { ffi::wxDataViewTextRenderer_EnableMarkup(self.as_ptr(), enable) }
    }
}

// wxDataViewToggleRenderer
pub trait DataViewToggleRendererMethods: DataViewRendererMethods {
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewToggleRenderer_GetDefaultType()).into() }
    }
    fn show_as_radio(&self) {
        unsafe { ffi::wxDataViewToggleRenderer_ShowAsRadio(self.as_ptr()) }
    }
}

// wxDataViewTreeCtrl
pub trait DataViewTreeCtrlMethods: DataViewCtrlMethods {
    // DTOR: fn ~wxDataViewTreeCtrl()
    fn append_container<D: DataViewItemMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_AppendContainer(
                self.as_ptr(),
                parent,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn append_item<D: DataViewItemMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_AppendItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            ffi::wxDataViewTreeCtrl_Create(self.as_ptr(), parent, id, pos, size, style, validator)
        }
    }
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeCtrl_DeleteAllItems(self.as_ptr()) }
    }
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteChildren(self.as_ptr(), item)
        }
    }
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteItem(self.as_ptr(), item)
        }
    }
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeCtrl_GetChildCount(self.as_ptr(), parent)
        }
    }
    fn get_image_list(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewTreeCtrl_GetImageList(self.as_ptr()) }
    }
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> *mut c_void {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_GetItemData(self.as_ptr(), item)
        }
    }
    fn get_item_expanded_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemExpandedIcon(
                self.as_ptr(),
                item,
            ))
        }
    }
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemIcon(self.as_ptr(), item))
        }
    }
    // BLOCKED: fn GetItemParent()
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeCtrl_GetItemText(self.as_ptr(), item)).into()
        }
    }
    fn get_nth_child<D: DataViewItemMethods>(&self, parent: &D, pos: c_uint) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_GetNthChild(
                self.as_ptr(),
                parent,
                pos,
            ))
        }
    }
    // BLOCKED: fn GetStore()
    fn get_store(&self) -> Option<DataViewTreeStoreIsOwned<false>> {
        unsafe { DataViewTreeStore::option_from(ffi::wxDataViewTreeCtrl_GetStore1(self.as_ptr())) }
    }
    fn insert_container<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_InsertContainer(
                self.as_ptr(),
                parent,
                previous,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn insert_item<D: DataViewItemMethods, D2: DataViewItemMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: c_int,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_InsertItem(
                self.as_ptr(),
                parent,
                previous,
                text,
                icon,
                data,
            ))
        }
    }
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_IsContainer(self.as_ptr(), item)
        }
    }
    fn prepend_container<D: DataViewItemMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_PrependContainer(
                self.as_ptr(),
                parent,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn prepend_item<D: DataViewItemMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_PrependItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    fn set_image_list(&self, imagelist: *mut c_void) {
        unsafe { ffi::wxDataViewTreeCtrl_SetImageList(self.as_ptr(), imagelist) }
    }
    fn set_item_data<D: DataViewItemMethods>(&self, item: &D, data: *mut c_void) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemData(self.as_ptr(), item, data)
        }
    }
    fn set_item_expanded_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(
        &self,
        item: &D,
        icon: &B,
    ) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemExpandedIcon(self.as_ptr(), item, icon)
        }
    }
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
    fn set_item_text<D: DataViewItemMethods>(&self, item: &D, text: &str) {
        unsafe {
            let item = item.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemText(self.as_ptr(), item, text)
        }
    }
}

// wxDataViewTreeStore
pub trait DataViewTreeStoreMethods: DataViewModelMethods {
    // DTOR: fn ~wxDataViewTreeStore()
    fn append_container<D: DataViewItemMethods, B: BitmapBundleMethods, B2: BitmapBundleMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        expanded: &B2,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let expanded = expanded.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_AppendContainer(
                self.as_ptr(),
                parent,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn append_item<D: DataViewItemMethods, B: BitmapBundleMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_AppendItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeStore_DeleteAllItems(self.as_ptr()) }
    }
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_DeleteChildren(self.as_ptr(), item)
        }
    }
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_DeleteItem(self.as_ptr(), item)
        }
    }
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeStore_GetChildCount(self.as_ptr(), parent)
        }
    }
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> *mut c_void {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_GetItemData(self.as_ptr(), item)
        }
    }
    fn get_item_expanded_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeStore_GetItemExpandedIcon(
                self.as_ptr(),
                item,
            ))
        }
    }
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeStore_GetItemIcon(self.as_ptr(), item))
        }
    }
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeStore_GetItemText(self.as_ptr(), item)).into()
        }
    }
    fn get_nth_child<D: DataViewItemMethods>(&self, parent: &D, pos: c_uint) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_GetNthChild(
                self.as_ptr(),
                parent,
                pos,
            ))
        }
    }
    fn insert_container<
        D: DataViewItemMethods,
        D2: DataViewItemMethods,
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
    >(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: &B,
        expanded: &B2,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let expanded = expanded.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_InsertContainer(
                self.as_ptr(),
                parent,
                previous,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn insert_item<D: DataViewItemMethods, D2: DataViewItemMethods, B: BitmapBundleMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: &B,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_InsertItem(
                self.as_ptr(),
                parent,
                previous,
                text,
                icon,
                data,
            ))
        }
    }
    fn prepend_container<
        D: DataViewItemMethods,
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
    >(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        expanded: &B2,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let expanded = expanded.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_PrependContainer(
                self.as_ptr(),
                parent,
                text,
                icon,
                expanded,
                data,
            ))
        }
    }
    fn prepend_item<D: DataViewItemMethods, B: BitmapBundleMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        data: *mut c_void,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_PrependItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    fn set_item_data<D: DataViewItemMethods>(&self, item: &D, data: *mut c_void) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_SetItemData(self.as_ptr(), item, data)
        }
    }
    fn set_item_expanded_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(
        &self,
        item: &D,
        icon: &B,
    ) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeStore_SetItemExpandedIcon(self.as_ptr(), item, icon)
        }
    }
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeStore_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
}

// wxDataViewValueAdjuster
pub trait DataViewValueAdjusterMethods: WxRustMethods {
    // NOT_SUPPORTED: fn MakeHighlighted()
}

// wxDataViewVirtualListModel
pub trait DataViewVirtualListModelMethods: DataViewListModelMethods {
    fn get_item(&self, row: c_uint) -> DataViewItem {
        unsafe {
            DataViewItem::from_ptr(ffi::wxDataViewVirtualListModel_GetItem(self.as_ptr(), row))
        }
    }
    fn reset(&self, new_size: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_Reset(self.as_ptr(), new_size) }
    }
    fn row_appended(&self) {
        unsafe { ffi::wxDataViewVirtualListModel_RowAppended(self.as_ptr()) }
    }
    fn row_changed(&self, row: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowChanged(self.as_ptr(), row) }
    }
    fn row_deleted(&self, row: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowDeleted(self.as_ptr(), row) }
    }
    fn row_inserted(&self, before: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowInserted(self.as_ptr(), before) }
    }
    fn row_prepended(&self) {
        unsafe { ffi::wxDataViewVirtualListModel_RowPrepended(self.as_ptr()) }
    }
    fn row_value_changed(&self, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowValueChanged(self.as_ptr(), row, col) }
    }
    fn rows_deleted<A: ArrayIntMethods>(&self, rows: &A) {
        unsafe {
            let rows = rows.as_ptr();
            ffi::wxDataViewVirtualListModel_RowsDeleted(self.as_ptr(), rows)
        }
    }
}

// wxDateEvent
pub trait DateEventMethods: CommandEventMethods {
    fn get_date(&self) -> DateTimeIsOwned<false> {
        unsafe { DateTimeIsOwned::from_ptr(ffi::wxDateEvent_GetDate(self.as_ptr())) }
    }
    fn set_date<D: DateTimeMethods>(&self, date: &D) {
        unsafe {
            let date = date.as_ptr();
            ffi::wxDateEvent_SetDate(self.as_ptr(), date)
        }
    }
}

// wxDatePickerCtrl
pub trait DatePickerCtrlMethods: ControlMethods {
    fn create_datetime<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        dt: &D,
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
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDatePickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                dt,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        dt1: Option<&D>,
        dt2: Option<&D2>,
    ) -> bool {
        unsafe {
            let dt1 = match dt1 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt2 = match dt2 {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDatePickerCtrl_GetRange(self.as_ptr(), dt1, dt2)
        }
    }
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDatePickerCtrl_GetValue(self.as_ptr())) }
    }
    fn set_null_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDatePickerCtrl_SetNullText(self.as_ptr(), text)
        }
    }
    fn set_range<D: DateTimeMethods, D2: DateTimeMethods>(&self, dt1: &D, dt2: &D2) {
        unsafe {
            let dt1 = dt1.as_ptr();
            let dt2 = dt2.as_ptr();
            ffi::wxDatePickerCtrl_SetRange(self.as_ptr(), dt1, dt2)
        }
    }
    fn set_value<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDatePickerCtrl_SetValue(self.as_ptr(), dt)
        }
    }
}

// wxDelegateRendererNative
pub trait DelegateRendererNativeMethods: RendererNativeMethods {}

// wxDialog
pub trait DialogMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxDialog()
    fn add_main_button_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_AddMainButtonId(self.as_ptr(), id) }
    }
    fn can_do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_CanDoLayoutAdaptation(self.as_ptr()) }
    }
    fn create_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxDialog_CreateButtonSizer(self.as_ptr(), flags)) }
    }
    fn create_separated_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe {
            Sizer::option_from(ffi::wxDialog_CreateSeparatedButtonSizer(
                self.as_ptr(),
                flags,
            ))
        }
    }
    fn create_separated_sizer<S: SizerMethods>(
        &self,
        sizer: Option<&S>,
    ) -> Option<SizerIsOwned<false>> {
        unsafe {
            let sizer = match sizer {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Sizer::option_from(ffi::wxDialog_CreateSeparatedSizer(self.as_ptr(), sizer))
        }
    }
    fn create_std_dialog_button_sizer(&self, flags: c_long) -> *mut c_void {
        unsafe { ffi::wxDialog_CreateStdDialogButtonSizer(self.as_ptr(), flags) }
    }
    fn create_text_sizer(&self, message: &str, width_max: c_int) -> Option<SizerIsOwned<false>> {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            Sizer::option_from(ffi::wxDialog_CreateTextSizer(
                self.as_ptr(),
                message,
                width_max,
            ))
        }
    }
    fn do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_DoLayoutAdaptation(self.as_ptr()) }
    }
    fn end_modal(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_EndModal(self.as_ptr(), ret_code) }
    }
    fn get_affirmative_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetAffirmativeId(self.as_ptr()) }
    }
    fn get_content_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDialog_GetContentWindow(self.as_ptr())) }
    }
    fn get_escape_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetEscapeId(self.as_ptr()) }
    }
    fn get_layout_adaptation_done(&self) -> bool {
        unsafe { ffi::wxDialog_GetLayoutAdaptationDone(self.as_ptr()) }
    }
    fn get_layout_adaptation_level(&self) -> c_int {
        unsafe { ffi::wxDialog_GetLayoutAdaptationLevel(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetLayoutAdaptationMode()
    fn get_main_button_ids(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxDialog_GetMainButtonIds(self.as_ptr())) }
    }
    fn get_return_code(&self) -> c_int {
        unsafe { ffi::wxDialog_GetReturnCode(self.as_ptr()) }
    }
    // BLOCKED: fn GetToolBar()
    fn is_main_button_id(&self, id: c_int) -> bool {
        unsafe { ffi::wxDialog_IsMainButtonId(self.as_ptr(), id) }
    }
    fn is_modal(&self) -> bool {
        unsafe { ffi::wxDialog_IsModal(self.as_ptr()) }
    }
    fn set_affirmative_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetAffirmativeId(self.as_ptr(), id) }
    }
    fn set_escape_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetEscapeId(self.as_ptr(), id) }
    }
    fn set_layout_adaptation_done(&self, done: bool) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationDone(self.as_ptr(), done) }
    }
    fn set_layout_adaptation_level(&self, level: c_int) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationLevel(self.as_ptr(), level) }
    }
    // NOT_SUPPORTED: fn SetLayoutAdaptationMode()
    fn set_return_code(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_SetReturnCode(self.as_ptr(), ret_code) }
    }
    fn show_modal(&self) -> c_int {
        unsafe { ffi::wxDialog_ShowModal(self.as_ptr()) }
    }
    fn show_window_modal(&self) {
        unsafe { ffi::wxDialog_ShowWindowModal(self.as_ptr()) }
    }
    // BLOCKED: fn ShowWindowModalThenDo()
    fn enable_layout_adaptation(enable: bool) {
        unsafe { ffi::wxDialog_EnableLayoutAdaptation(enable) }
    }
    fn get_layout_adapter() -> Option<DialogLayoutAdapterIsOwned<false>> {
        unsafe { DialogLayoutAdapter::option_from(ffi::wxDialog_GetLayoutAdapter()) }
    }
    fn is_layout_adaptation_enabled() -> bool {
        unsafe { ffi::wxDialog_IsLayoutAdaptationEnabled() }
    }
    fn set_layout_adapter<D: DialogLayoutAdapterMethods>(
        adapter: Option<&D>,
    ) -> Option<DialogLayoutAdapterIsOwned<false>> {
        unsafe {
            let adapter = match adapter {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DialogLayoutAdapter::option_from(ffi::wxDialog_SetLayoutAdapter(adapter))
        }
    }
}

// wxDialogLayoutAdapter
pub trait DialogLayoutAdapterMethods: WxRustMethods {
    fn can_do_layout_adaptation<D: DialogMethods>(&self, dialog: Option<&D>) -> bool {
        unsafe {
            let dialog = match dialog {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDialogLayoutAdapter_CanDoLayoutAdaptation(self.as_ptr(), dialog)
        }
    }
    fn do_layout_adaptation<D: DialogMethods>(&self, dialog: Option<&D>) -> bool {
        unsafe {
            let dialog = match dialog {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDialogLayoutAdapter_DoLayoutAdaptation(self.as_ptr(), dialog)
        }
    }
}

// wxDirDialog
pub trait DirDialogMethods: DialogMethods {
    // DTOR: fn ~wxDirDialog()
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirDialog_GetMessage(self.as_ptr())).into() }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirDialog_GetPath(self.as_ptr())).into() }
    }
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxDirDialog_GetPaths(self.as_ptr(), paths)
        }
    }
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDirDialog_SetMessage(self.as_ptr(), message)
        }
    }
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxDirDialog_SetPath(self.as_ptr(), path)
        }
    }
}

// wxDirPickerCtrl
pub trait DirPickerCtrlMethods: PickerBaseMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
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
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDirPickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                path,
                message,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_dir_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxDirPickerCtrl_GetDirName(self.as_ptr())) }
    }
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirPickerCtrl_GetPath(self.as_ptr())).into() }
    }
    fn set_dir_name<F: FileNameMethods>(&self, dirname: &F) {
        unsafe {
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetDirName(self.as_ptr(), dirname)
        }
    }
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDirPickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    fn set_path(&self, dirname: &str) {
        unsafe {
            let dirname = WxString::from(dirname);
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetPath(self.as_ptr(), dirname)
        }
    }
}

// wxDisplay
pub trait DisplayMethods: WxRustMethods {
    // DTOR: fn ~wxDisplay()
    fn change_mode(&self, mode: *const c_void) -> bool {
        unsafe { ffi::wxDisplay_ChangeMode(self.as_ptr(), mode) }
    }
    fn get_client_area(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetClientArea(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetCurrentMode()
    fn get_geometry(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetGeometry(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetModes()
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDisplay_GetName(self.as_ptr())).into() }
    }
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetPPI(self.as_ptr())) }
    }
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDisplay_GetScaleFactor(self.as_ptr()) }
    }
    fn is_primary(&self) -> bool {
        unsafe { ffi::wxDisplay_IsPrimary(self.as_ptr()) }
    }
    fn get_count() -> c_uint {
        unsafe { ffi::wxDisplay_GetCount() }
    }
    fn get_from_point<P: PointMethods>(pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDisplay_GetFromPoint(pt)
        }
    }
    fn get_from_window<W: WindowMethods>(win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDisplay_GetFromWindow(win)
        }
    }
    fn get_std_ppi_value() -> c_int {
        unsafe { ffi::wxDisplay_GetStdPPIValue() }
    }
    fn get_std_ppi() -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetStdPPI()) }
    }
}

// wxDisplayChangedEvent
pub trait DisplayChangedEventMethods: EventMethods {}

// wxDragImage
pub trait DragImageMethods: ObjectMethods {
    fn begin_drag_bool<P: PointMethods, W: WindowMethods, R: RectMethods>(
        &self,
        hotspot: &P,
        window: Option<&W>,
        full_screen: bool,
        rect: Option<&R>,
    ) -> bool {
        unsafe {
            let hotspot = hotspot.as_ptr();
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let rect = match rect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDragImage_BeginDrag(self.as_ptr(), hotspot, window, full_screen, rect)
        }
    }
    fn begin_drag_window<P: PointMethods, W: WindowMethods, W2: WindowMethods>(
        &self,
        hotspot: &P,
        window: Option<&W>,
        bounding_window: Option<&W2>,
    ) -> bool {
        unsafe {
            let hotspot = hotspot.as_ptr();
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let bounding_window = match bounding_window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDragImage_BeginDrag1(self.as_ptr(), hotspot, window, bounding_window)
        }
    }
    fn do_draw_image<D: DCMethods, P: PointMethods>(&self, dc: &D, pos: &P) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxDragImage_DoDrawImage(self.as_ptr(), dc, pos)
        }
    }
    fn end_drag(&self) -> bool {
        unsafe { ffi::wxDragImage_EndDrag(self.as_ptr()) }
    }
    fn get_image_rect<P: PointMethods>(&self, pos: &P) -> Rect {
        unsafe {
            let pos = pos.as_ptr();
            Rect::from_ptr(ffi::wxDragImage_GetImageRect(self.as_ptr(), pos))
        }
    }
    fn hide(&self) -> bool {
        unsafe { ffi::wxDragImage_Hide(self.as_ptr()) }
    }
    fn move_<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDragImage_Move(self.as_ptr(), pt)
        }
    }
    fn show(&self) -> bool {
        unsafe { ffi::wxDragImage_Show(self.as_ptr()) }
    }
    fn update_backing_from_window<D: DCMethods, R: RectMethods, R2: RectMethods>(
        &self,
        window_dc: &D,
        dest_dc: *mut c_void,
        source_rect: &R,
        dest_rect: &R2,
    ) -> bool {
        unsafe {
            let window_dc = window_dc.as_ptr();
            let source_rect = source_rect.as_ptr();
            let dest_rect = dest_rect.as_ptr();
            ffi::wxDragImage_UpdateBackingFromWindow(
                self.as_ptr(),
                window_dc,
                dest_dc,
                source_rect,
                dest_rect,
            )
        }
    }
}

// wxDropFilesEvent
pub trait DropFilesEventMethods: EventMethods {
    // BLOCKED: fn GetFiles()
    fn get_number_of_files(&self) -> c_int {
        unsafe { ffi::wxDropFilesEvent_GetNumberOfFiles(self.as_ptr()) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDropFilesEvent_GetPosition(self.as_ptr())) }
    }
}

// wxDropSource
pub trait DropSourceMethods: WxRustMethods {
    // NOT_SUPPORTED: fn DoDragDrop()
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDropSource_GetDataObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GiveFeedback()
    // NOT_SUPPORTED: fn SetCursor()
    // NOT_SUPPORTED: fn SetIcon()
    fn set_data<D: DataObjectMethods>(&self, data: &D) {
        unsafe {
            let data = data.as_ptr();
            ffi::wxDropSource_SetData(self.as_ptr(), data)
        }
    }
}

// wxDropTarget
pub trait DropTargetMethods: WxRustMethods {
    // DTOR: fn ~wxDropTarget()
    fn get_data(&self) -> bool {
        unsafe { ffi::wxDropTarget_GetData(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn OnData()
    // NOT_SUPPORTED: fn OnDragOver()
    fn on_drop(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxDropTarget_OnDrop(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn OnEnter()
    fn on_leave(&self) {
        unsafe { ffi::wxDropTarget_OnLeave(self.as_ptr()) }
    }
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDropTarget_GetDataObject(self.as_ptr())) }
    }
    fn set_data_object<D: DataObjectMethods>(&self, data: Option<&D>) {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDropTarget_SetDataObject(self.as_ptr(), data)
        }
    }
    // NOT_SUPPORTED: fn SetDefaultAction()
    // NOT_SUPPORTED: fn GetDefaultAction()
}
