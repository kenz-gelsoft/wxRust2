use super::*;

// wxDC
pub trait DCMethods: ObjectMethods {
    /// Convert device X coordinate to logical coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    fn device_to_logical_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalX(self.as_ptr(), x) }
    }
    /// Convert device X coordinate to relative logical coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    fn device_to_logical_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalXRel(self.as_ptr(), x) }
    }
    /// Converts device Y coordinate to logical coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    fn device_to_logical_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalY(self.as_ptr(), y) }
    }
    /// Convert device Y coordinate to relative logical coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    fn device_to_logical_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalYRel(self.as_ptr(), y) }
    }
    /// Converts logical X coordinate to device coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    fn logical_to_device_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceX(self.as_ptr(), x) }
    }
    /// Converts logical X coordinate to relative device coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    fn logical_to_device_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceXRel(self.as_ptr(), x) }
    }
    /// Converts logical Y coordinate to device coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    fn logical_to_device_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceY(self.as_ptr(), y) }
    }
    /// Converts logical Y coordinate to relative device coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    fn logical_to_device_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceYRel(self.as_ptr(), y) }
    }
    /// Converts device (x, y) coordinates to logical coordinates taking into account all applied transformations like the current mapping mode, scale factors, device origin, axes orientation, affine transformation.
    fn device_to_logical_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_DeviceToLogical(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn device_to_logical_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_DeviceToLogical1(self.as_ptr(), pt))
        }
    }
    /// Converts device x, y coordinates to relative logical coordinates taking into account all applied transformations like the current mapping mode, scale factors, affine transformation.
    fn device_to_logical_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_DeviceToLogicalRel(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn device_to_logical_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_DeviceToLogicalRel1(self.as_ptr(), dim))
        }
    }
    /// Converts logical (x, y) coordinates to device coordinates taking into account all applied transformations like the current mapping mode, scale factors, device origin, axes orientation, affine transformation.
    fn logical_to_device_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_LogicalToDevice(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn logical_to_device_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_LogicalToDevice1(self.as_ptr(), pt))
        }
    }
    /// Converts logical x, y coordinates to relative device coordinates taking into account all applied transformations like the current mapping mode, scale factors, affine transformation.
    fn logical_to_device_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_LogicalToDeviceRel(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn logical_to_device_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_LogicalToDeviceRel1(self.as_ptr(), dim))
        }
    }
    /// Clears the device context using the current background brush.
    fn clear(&self) {
        unsafe { ffi::wxDC_Clear(self.as_ptr()) }
    }
    /// Draws an arc from the given start to the given end point.
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    /// Draw a bitmap on the device context at the specified point.
    fn draw_bitmap_coord<B: BitmapMethods>(&self, bitmap: &B, x: c_int, y: c_int, use_mask: bool) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDC_DrawBitmap(self.as_ptr(), bitmap, x, y, use_mask)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    /// Draws a check mark inside the given rectangle.
    fn draw_check_mark_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawCheckMark(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_check_mark_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawCheckMark1(self.as_ptr(), rect)
        }
    }
    /// Draws a circle with the given centre and radius.
    fn draw_circle_coord(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { ffi::wxDC_DrawCircle(self.as_ptr(), x, y, radius) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_circle_point<P: PointMethods>(&self, pt: &P, radius: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawCircle1(self.as_ptr(), pt, radius)
        }
    }
    /// Draws an ellipse contained in the rectangle specified either with the given top left corner and the given size or directly.
    fn draw_ellipse_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawEllipse(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_ellipse_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, size: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let size = size.as_ptr();
            ffi::wxDC_DrawEllipse1(self.as_ptr(), pt, size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_ellipse_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawEllipse2(self.as_ptr(), rect)
        }
    }
    /// Draws an arc of an ellipse.
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    /// Draw an icon on the display (does nothing if the device context is PostScript).
    fn draw_icon_coord<I: IconMethods>(&self, icon: &I, x: c_int, y: c_int) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDC_DrawIcon(self.as_ptr(), icon, x, y)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_icon_point<I: IconMethods, P: PointMethods>(&self, icon: &I, pt: &P) {
        unsafe {
            let icon = icon.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawIcon1(self.as_ptr(), icon, pt)
        }
    }
    /// Draw optional bitmap and the text into the given rectangle and aligns it as specified by alignment parameter; it also will emphasize the character with the given index if it is != -1 and return the bounding rectangle if required.
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    /// Draws a line from the first point to the second.
    fn draw_line_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { ffi::wxDC_DrawLine(self.as_ptr(), x1, y1, x2, y2) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_line_point<P: PointMethods, P2: PointMethods>(&self, pt1: &P, pt2: &P2) {
        unsafe {
            let pt1 = pt1.as_ptr();
            let pt2 = pt2.as_ptr();
            ffi::wxDC_DrawLine1(self.as_ptr(), pt1, pt2)
        }
    }
    // NOT_SUPPORTED: fn DrawLines()
    /// This method uses a list of wxPoints, adding the optional offset coordinate.
    fn draw_lines(&self, points: *const c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { ffi::wxDC_DrawLines1(self.as_ptr(), points, xoffset, yoffset) }
    }
    /// Draws a point using the color of the current pen.
    fn draw_point_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_DrawPoint(self.as_ptr(), x, y) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_point_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawPoint1(self.as_ptr(), pt)
        }
    }
    // NOT_SUPPORTED: fn DrawPolygon()
    // NOT_SUPPORTED: fn DrawPolygon1()
    // NOT_SUPPORTED: fn DrawPolyPolygon()
    /// Draws a rectangle with the given corner coordinate and size.
    fn draw_rectangle_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawRectangle(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_rectangle_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawRectangle1(self.as_ptr(), pt, sz)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_rectangle_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRectangle2(self.as_ptr(), rect)
        }
    }
    /// Draws the text rotated by angle degrees (positive angles are counterclockwise; the full angle is 360 degrees).
    fn draw_rotated_text_coord(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawRotatedText(self.as_ptr(), text, x, y, angle)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_rotated_text_point<P: PointMethods>(&self, text: &str, point: &P, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let point = point.as_ptr();
            ffi::wxDC_DrawRotatedText1(self.as_ptr(), text, point, angle)
        }
    }
    /// Draws a rectangle with the given top left corner, and with the given size.
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_rounded_rectangle_rect<R: RectMethods>(&self, rect: &R, radius: c_double) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRoundedRectangle2(self.as_ptr(), rect, radius)
        }
    }
    // NOT_SUPPORTED: fn DrawSpline()
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_spline_pointlist(&self, points: *const c_void) {
        unsafe { ffi::wxDC_DrawSpline1(self.as_ptr(), points) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_spline_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int) {
        unsafe { ffi::wxDC_DrawSpline2(self.as_ptr(), x1, y1, x2, y2, x3, y3) }
    }
    /// Draws a text string at the specified point, using the current text font, and the current text foreground and background colours.
    fn draw_text_coord(&self, text: &str, x: c_int, y: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawText(self.as_ptr(), text, x, y)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn draw_text_point<P: PointMethods>(&self, text: &str, pt: &P) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawText1(self.as_ptr(), text, pt)
        }
    }
    /// Fill the area specified by rect with a radial gradient, starting from initialColour at the centre of the circle and fading to destColour on the circle outside.
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
    /// Fill the area specified by rect with a radial gradient, starting from initialColour at the centre of the circle and fading to destColour on the circle outside.
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
    /// Fill the area specified by rect with a linear gradient, starting from initialColour and eventually fading to destColour.
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
    /// Displays a cross hair using the current pen.
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CrossHair(self.as_ptr(), x, y) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn cross_hair_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_CrossHair1(self.as_ptr(), pt)
        }
    }
    /// Destroys the current clipping region so that none of the DC is clipped.
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxDC_DestroyClippingRegion(self.as_ptr()) }
    }
    /// Gets the rectangle surrounding the current clipping region.
    fn get_clipping_box_coord(
        &self,
        x: *mut c_void,
        y: *mut c_void,
        width: *mut c_void,
        height: *mut c_void,
    ) -> bool {
        unsafe { ffi::wxDC_GetClippingBox(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn get_clipping_box_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_GetClippingBox1(self.as_ptr(), rect)
        }
    }
    /// Sets the clipping region for this device context to the intersection of the given region described by the parameters of this method and the previously set clipping region.
    fn set_clipping_region_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_SetClippingRegion(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_clipping_region_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_SetClippingRegion1(self.as_ptr(), pt, sz)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn set_clipping_region_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_SetClippingRegion2(self.as_ptr(), rect)
        }
    }
    /// Sets the clipping region for this device context.
    fn set_device_clipping_region<R: RegionMethods>(&self, region: &R) {
        unsafe {
            let region = region.as_ptr();
            ffi::wxDC_SetDeviceClippingRegion(self.as_ptr(), region)
        }
    }
    /// Gets the character height of the currently set font.
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharHeight(self.as_ptr()) }
    }
    /// Gets the average character width of the currently set font.
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontMetrics()
    /// Gets the dimensions of the string using the currently selected font.
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
    /// Gets the dimensions of the string using the currently selected font.
    fn get_multi_line_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetMultiLineTextExtent1(self.as_ptr(), string))
        }
    }
    /// Fills the widths array with the widths from the beginning of text to the corresponding character of text.
    fn get_partial_text_extents<A: ArrayIntMethods>(&self, text: &str, widths: &A) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let widths = widths.as_ptr();
            ffi::wxDC_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    /// Gets the dimensions of the string using the currently selected font.
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
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetTextExtent1(self.as_ptr(), string))
        }
    }
    /// Returns the current background mode: wxBRUSHSTYLE_SOLID or wxBRUSHSTYLE_TRANSPARENT.
    fn get_background_mode(&self) -> c_int {
        unsafe { ffi::wxDC_GetBackgroundMode(self.as_ptr()) }
    }
    /// Gets the current font.
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxDC_GetFont(self.as_ptr())) }
    }
    /// Gets the current layout direction of the device context.
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxDC_GetLayoutDirection(self.as_ptr()) }
    }
    /// Gets the current text background colour.
    fn get_text_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextBackground(self.as_ptr())) }
    }
    /// Gets the current text foreground colour.
    fn get_text_foreground(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextForeground(self.as_ptr())) }
    }
    /// Change the current background mode.
    fn set_background_mode(&self, mode: c_int) {
        unsafe { ffi::wxDC_SetBackgroundMode(self.as_ptr(), mode) }
    }
    /// Sets the current font for the DC.
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDC_SetFont(self.as_ptr(), font)
        }
    }
    /// Sets the current text background colour for the DC.
    fn set_text_background<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextBackground(self.as_ptr(), colour)
        }
    }
    /// Sets the current text foreground colour for the DC.
    fn set_text_foreground<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextForeground(self.as_ptr(), colour)
        }
    }
    /// Sets the current layout direction for the device context.
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxDC_SetLayoutDirection(self.as_ptr(), dir) }
    }
    /// Adds the specified point to the bounding box which can be retrieved with MinX(), MaxX() and MinY(), MaxY() functions.
    fn calc_bounding_box(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CalcBoundingBox(self.as_ptr(), x, y) }
    }
    /// Gets the maximum horizontal extent used in drawing commands so far.
    fn max_x(&self) -> c_int {
        unsafe { ffi::wxDC_MaxX(self.as_ptr()) }
    }
    /// Gets the maximum vertical extent used in drawing commands so far.
    fn max_y(&self) -> c_int {
        unsafe { ffi::wxDC_MaxY(self.as_ptr()) }
    }
    /// Gets the minimum horizontal extent used in drawing commands so far.
    fn min_x(&self) -> c_int {
        unsafe { ffi::wxDC_MinX(self.as_ptr()) }
    }
    /// Gets the minimum vertical extent used in drawing commands so far.
    fn min_y(&self) -> c_int {
        unsafe { ffi::wxDC_MinY(self.as_ptr()) }
    }
    /// Resets the bounding box: after a call to this function, the bounding box doesn't contain anything.
    fn reset_bounding_box(&self) {
        unsafe { ffi::wxDC_ResetBoundingBox(self.as_ptr()) }
    }
    /// Starts a document (only relevant when outputting to a printer).
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDC_StartDoc(self.as_ptr(), message)
        }
    }
    /// Starts a document page (only relevant when outputting to a printer).
    fn start_page(&self) {
        unsafe { ffi::wxDC_StartPage(self.as_ptr()) }
    }
    /// Ends a document (only relevant when outputting to a printer).
    fn end_doc(&self) {
        unsafe { ffi::wxDC_EndDoc(self.as_ptr()) }
    }
    /// Ends a document page (only relevant when outputting to a printer).
    fn end_page(&self) {
        unsafe { ffi::wxDC_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Blit()
    // NOT_SUPPORTED: fn StretchBlit()
    /// Gets the brush used for painting the background.
    fn get_background(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBackground(self.as_ptr())) }
    }
    /// Gets the current brush.
    fn get_brush(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBrush(self.as_ptr())) }
    }
    // BLOCKED: fn GetPen()
    /// Sets the current background brush for the DC.
    fn set_background<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBackground(self.as_ptr(), brush)
        }
    }
    /// Sets the current brush for the DC.
    fn set_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBrush(self.as_ptr(), brush)
        }
    }
    /// Sets the current pen for the DC.
    fn set_pen<P: PenMethods>(&self, pen: &P) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxDC_SetPen(self.as_ptr(), pen)
        }
    }
    /// Copy attributes from another DC.
    fn copy_attributes<D: DCMethods>(&self, dc: &D) {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxDC_CopyAttributes(self.as_ptr(), dc)
        }
    }
    /// Returns the factor used for converting logical pixels to physical ones.
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDC_GetContentScaleFactor(self.as_ptr()) }
    }
    /// Returns the depth (number of bits/pixel) of this DC.
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxDC_GetDepth(self.as_ptr()) }
    }
    /// Returns the current device origin.
    fn get_device_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetDeviceOrigin(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetLogicalFunction()
    // NOT_SUPPORTED: fn GetMapMode()
    /// Gets in colour the colour at the specified location.
    fn get_pixel<C: ColourMethods>(&self, x: c_int, y: c_int, colour: Option<&C>) -> bool {
        unsafe {
            let colour = match colour {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_GetPixel(self.as_ptr(), x, y, colour)
        }
    }
    /// Returns the resolution of the device in pixels per inch.
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetPPI(self.as_ptr())) }
    }
    /// Convert DPI-independent pixel values to the value in pixels appropriate for the DC.
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_FromDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_FromDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert DPI-independent value in pixels to the value in pixels appropriate for the DC.
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_FromDIP2(self.as_ptr(), d) }
    }
    /// Convert pixel values of the current DC to DPI-independent pixel values.
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_ToDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_ToDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert pixel values of the current DC to DPI-independent pixel values.
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_ToDIP2(self.as_ptr(), d) }
    }
    /// Gets the horizontal and vertical extent of this device context in device units.
    fn get_size_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSize1(self.as_ptr())) }
    }
    /// Returns the horizontal and vertical resolution in millimetres.
    fn get_size_mm_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSizeMM(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    fn get_size_mm(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSizeMM1(self.as_ptr())) }
    }
    /// Gets the current user scale factor.
    fn get_user_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetUserScale(self.as_ptr(), x, y) }
    }
    /// Returns true if the DC is ok to use.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDC_IsOk(self.as_ptr()) }
    }
    /// Sets the x and y axis orientation (i.e. the direction from lowest to highest values on the axis).
    fn set_axis_orientation(&self, x_left_right: bool, y_bottom_up: bool) {
        unsafe { ffi::wxDC_SetAxisOrientation(self.as_ptr(), x_left_right, y_bottom_up) }
    }
    /// Sets the device origin (i.e. the origin in pixels after scaling has been applied).
    fn set_device_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetDeviceOrigin(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn SetMapMode()
    /// If this is a window DC or memory DC, assigns the given palette to the window or bitmap associated with the DC.
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxDC_SetPalette(self.as_ptr(), palette)
        }
    }
    /// Sets the user scaling factor, useful for applications which require 'zooming'.
    fn set_user_scale(&self, x_scale: c_double, y_scale: c_double) {
        unsafe { ffi::wxDC_SetUserScale(self.as_ptr(), x_scale, y_scale) }
    }
    /// Check if the use of transformation matrix is supported by the current system.
    fn can_use_transform_matrix(&self) -> bool {
        unsafe { ffi::wxDC_CanUseTransformMatrix(self.as_ptr()) }
    }
    /// Set the transformation matrix.
    fn set_transform_matrix<A: AffineMatrix2DMethods>(&self, matrix: &A) -> bool {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxDC_SetTransformMatrix(self.as_ptr(), matrix)
        }
    }
    /// Return the transformation matrix used by this device context.
    fn get_transform_matrix(&self) -> AffineMatrix2D {
        unsafe { AffineMatrix2D::from_ptr(ffi::wxDC_GetTransformMatrix(self.as_ptr())) }
    }
    /// Revert the transformation matrix to identity matrix.
    fn reset_transform_matrix(&self) {
        unsafe { ffi::wxDC_ResetTransformMatrix(self.as_ptr()) }
    }
    /// Does the DC support drawing bitmaps?
    fn can_draw_bitmap(&self) -> bool {
        unsafe { ffi::wxDC_CanDrawBitmap(self.as_ptr()) }
    }
    /// Does the DC support calculating the size required to draw text?
    fn can_get_text_extent(&self) -> bool {
        unsafe { ffi::wxDC_CanGetTextExtent(self.as_ptr()) }
    }
    /// Returns a value that can be used as a handle to the native drawing context, if this wxDC has something that could be thought of in that way.
    fn get_handle(&self) -> *mut c_void {
        unsafe { ffi::wxDC_GetHandle(self.as_ptr()) }
    }
    /// If supported by the platform and the type of DC, fetch the contents of the DC, or a subset of it, as a bitmap.
    fn get_as_bitmap<R: RectMethods>(&self, subrect: Option<&R>) -> Bitmap {
        unsafe {
            let subrect = match subrect {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Bitmap::from_ptr(ffi::wxDC_GetAsBitmap(self.as_ptr(), subrect))
        }
    }
    /// Set the scale to use for translating wxDC coordinates to the physical pixels.
    fn set_logical_scale(&self, x: c_double, y: c_double) {
        unsafe { ffi::wxDC_SetLogicalScale(self.as_ptr(), x, y) }
    }
    /// Return the scale set by the last call to SetLogicalScale().
    fn get_logical_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalScale(self.as_ptr(), x, y) }
    }
    /// Change the offset used for translating wxDC coordinates.
    fn set_logical_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetLogicalOrigin(self.as_ptr(), x, y) }
    }
    /// Return the coordinates of the logical point (0, 0).
    fn get_logical_origin_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalOrigin(self.as_ptr(), x, y) }
    }
    /// Does the DC support drawing bitmaps?
    fn get_logical_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetLogicalOrigin1(self.as_ptr())) }
    }
    /// If supported by the platform and the wxDC implementation, this method will return the wxGraphicsContext associated with the DC.
    fn get_graphics_context(&self) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxDC_GetGraphicsContext(self.as_ptr())) }
    }
    /// Associate a wxGraphicsContext with the DC.
    fn set_graphics_context<G: GraphicsContextMethods>(&self, ctx: Option<&G>) {
        unsafe {
            let ctx = match ctx {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDC_SetGraphicsContext(self.as_ptr(), ctx)
        }
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
    /// Set the font to use.
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
    /// Clears the layer, restoring the state at the last init.
    fn clear(&self) {
        unsafe { ffi::wxDCOverlay_Clear(self.as_ptr()) }
    }
}

// wxDCPenChanger
pub trait DCPenChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCPenChanger()
}

// wxDCTextColourChanger
pub trait DCTextColourChangerMethods: WxRustMethods {
    /// Set the colour to use.
    fn set<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxDCTextColourChanger_Set(self.as_ptr(), col)
        }
    }
    // DTOR: fn ~wxDCTextColourChanger()
}

// wxDataFormat
pub trait DataFormatMethods: WxRustMethods {
    /// Returns the name of a custom format (this function will fail for a standard format).
    fn get_id(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataFormat_GetId(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    /// Sets the format to be the custom format identified by the given name.
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
    /// The method will write the data of the format format to the buffer buf.
    fn get_data_here<D: DataFormatMethods>(&self, format: &D, buf: *mut c_void) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_GetDataHere(self.as_ptr(), format, buf)
        }
    }
    /// Returns the data size of the given format format.
    fn get_data_size<D: DataFormatMethods>(&self, format: &D) -> usize {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_GetDataSize(self.as_ptr(), format)
        }
    }
    // NOT_SUPPORTED: fn GetFormatCount()
    // NOT_SUPPORTED: fn GetPreferredFormat()
    /// Set the data in the format format of the length len provided in the buffer buf.
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
    /// Adds the dataObject to the list of supported objects and it becomes the preferred object if preferred is true.
    fn add<D: DataObjectSimpleMethods>(&self, data_object: Option<&D>, preferred: bool) {
        unsafe {
            let data_object = match data_object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataObjectComposite_Add(self.as_ptr(), data_object, preferred)
        }
    }
    /// Report the format passed to the SetData() method.
    fn get_received_format(&self) -> DataFormat {
        unsafe { DataFormat::from_ptr(ffi::wxDataObjectComposite_GetReceivedFormat(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetObject()
}

// wxDataObjectSimple
pub trait DataObjectSimpleMethods: DataObjectMethods {
    /// Copy the data to the buffer, return true on success.
    fn get_data_here_void(&self, buf: *mut c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_GetDataHere(self.as_ptr(), buf) }
    }
    /// Gets the size of our data.
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataObjectSimple_GetDataSize(self.as_ptr()) }
    }
    // BLOCKED: fn GetFormat()
    /// Copy the data from the buffer, return true on success.
    fn set_data_sz(&self, len: usize, buf: *const c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_SetData(self.as_ptr(), len, buf) }
    }
    /// Sets the supported format.
    fn set_format<D: DataFormatMethods>(&self, format: &D) {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObjectSimple_SetFormat(self.as_ptr(), format)
        }
    }
}

// wxDataViewBitmapRenderer
pub trait DataViewBitmapRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewBitmapRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewChoiceByIndexRenderer
pub trait DataViewChoiceByIndexRendererMethods: DataViewChoiceRendererMethods {}

// wxDataViewChoiceRenderer
pub trait DataViewChoiceRendererMethods: DataViewRendererMethods {
    /// Returns the choice referred to by index.
    fn get_choice(&self, index: usize) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxDataViewChoiceRenderer_GetChoice(
                self.as_ptr(),
                index,
            ))
            .into()
        }
    }
    /// Returns all choices.
    fn get_choices(&self) -> ArrayStringIsOwned<false> {
        unsafe {
            ArrayStringIsOwned::from_ptr(ffi::wxDataViewChoiceRenderer_GetChoices(self.as_ptr()))
        }
    }
}

// wxDataViewColumn
pub trait DataViewColumnMethods: SettableHeaderColumnMethods {
    /// Returns the index of the column of the model, which this wxDataViewColumn is displaying.
    fn get_model_column(&self) -> c_uint {
        unsafe { ffi::wxDataViewColumn_GetModelColumn(self.as_ptr()) }
    }
    /// Returns the owning wxDataViewCtrl.
    fn get_owner(&self) -> WeakRef<DataViewCtrl> {
        unsafe { WeakRef::<DataViewCtrl>::from(ffi::wxDataViewColumn_GetOwner(self.as_ptr())) }
    }
    /// Returns the renderer of this wxDataViewColumn.
    fn get_renderer(&self) -> Option<DataViewRendererIsOwned<false>> {
        unsafe { DataViewRenderer::option_from(ffi::wxDataViewColumn_GetRenderer(self.as_ptr())) }
    }
}

// wxDataViewCtrl
pub trait DataViewCtrlMethods: ControlMethods {
    // DTOR: fn ~wxDataViewCtrl()
    /// Call to allow using multiple columns for sorting.
    fn allow_multi_column_sort(&self, allow: bool) -> bool {
        unsafe { ffi::wxDataViewCtrl_AllowMultiColumnSort(self.as_ptr(), allow) }
    }
    /// Appends a wxDataViewColumn to the control.
    fn append_column<D: DataViewColumnMethods>(&self, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_AppendColumn(self.as_ptr(), col)
        }
    }
    /// Prepends a wxDataViewColumn to the control.
    fn prepend_column<D: DataViewColumnMethods>(&self, col: Option<&D>) -> bool {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_PrependColumn(self.as_ptr(), col)
        }
    }
    /// Inserts a wxDataViewColumn to the control.
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
    /// Associates a wxDataViewModel with the control.
    fn associate_model<D: DataViewModelMethods>(&self, model: Option<&D>) -> bool {
        unsafe {
            let model = match model {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_AssociateModel(self.as_ptr(), model)
        }
    }
    /// Removes all columns.
    fn clear_columns(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_ClearColumns(self.as_ptr()) }
    }
    /// Collapses the item.
    fn collapse<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Collapse(self.as_ptr(), item)
        }
    }
    /// Deletes given column.
    fn delete_column<D: DataViewColumnMethods>(&self, column: Option<&D>) -> bool {
        unsafe {
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_DeleteColumn(self.as_ptr(), column)
        }
    }
    /// Programmatically starts editing given cell of item.
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
    /// Enable drag operations using the given format.
    fn enable_drag_source<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewCtrl_EnableDragSource(self.as_ptr(), format)
        }
    }
    /// Enable drop operations using any of the specified formats.
    fn enable_drop_targets(&self, formats: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_EnableDropTargets(self.as_ptr(), formats) }
    }
    /// Enable drop operations using the given format.
    fn enable_drop_target<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewCtrl_EnableDropTarget(self.as_ptr(), format)
        }
    }
    /// Call this to ensure that the given item is visible.
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
    /// Expands the item.
    fn expand<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Expand(self.as_ptr(), item)
        }
    }
    /// Expands all ancestors of the item.
    fn expand_ancestors<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandAncestors(self.as_ptr(), item)
        }
    }
    /// Expand all children of the given item recursively.
    fn expand_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandChildren(self.as_ptr(), item)
        }
    }
    /// Returns pointer to the column.
    fn get_column(&self, pos: c_uint) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetColumn(self.as_ptr(), pos)) }
    }
    /// Returns the number of columns.
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewCtrl_GetColumnCount(self.as_ptr()) }
    }
    /// Returns the position of the column or -1 if not found in the control.
    fn get_column_position<D: DataViewColumnMethods>(&self, column: Option<&D>) -> c_int {
        unsafe {
            let column = match column {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_GetColumnPosition(self.as_ptr(), column)
        }
    }
    /// Returns column containing the expanders.
    fn get_expander_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetExpanderColumn(self.as_ptr())) }
    }
    /// Returns the currently focused item.
    fn get_current_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetCurrentItem(self.as_ptr())) }
    }
    /// Returns the column that currently has focus.
    fn get_current_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetCurrentColumn(self.as_ptr())) }
    }
    /// Returns indentation.
    fn get_indent(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetIndent(self.as_ptr()) }
    }
    /// Returns item rectangle.
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
    /// Returns the window corresponding to the main area of the control.
    fn get_main_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDataViewCtrl_GetMainWindow(self.as_ptr())) }
    }
    /// Returns pointer to the data model associated with the control (if any).
    fn get_model(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewCtrl_GetModel(self.as_ptr())) }
    }
    /// Returns the number of currently selected items.
    fn get_selected_items_count(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelectedItemsCount(self.as_ptr()) }
    }
    /// Returns first selected item or an invalid item if none is selected.
    fn get_selection(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetSelection(self.as_ptr())) }
    }
    /// Fills sel with currently selected items and returns their number.
    fn get_selections(&self, sel: *mut c_void) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelections(self.as_ptr(), sel) }
    }
    /// Returns the wxDataViewColumn currently responsible for sorting or NULL if none has been selected.
    fn get_sorting_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetSortingColumn(self.as_ptr())) }
    }
    // BLOCKED: fn GetSortingColumns()
    /// Returns true if any items are currently selected.
    fn has_selection(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_HasSelection(self.as_ptr()) }
    }
    /// Retrieves item and column at the given point.
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
    /// Return true if the item is expanded.
    fn is_expanded<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsExpanded(self.as_ptr(), item)
        }
    }
    /// Return true if using more than one column for sorting is allowed.
    fn is_multi_column_sort_allowed(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_IsMultiColumnSortAllowed(self.as_ptr()) }
    }
    /// Return true if the item is selected.
    fn is_selected<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsSelected(self.as_ptr(), item)
        }
    }
    /// Select the given item.
    fn select<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Select(self.as_ptr(), item)
        }
    }
    /// Select all items.
    fn select_all(&self) {
        unsafe { ffi::wxDataViewCtrl_SelectAll(self.as_ptr()) }
    }
    /// Set custom colour for the alternate rows used with wxDV_ROW_LINES style.
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    /// Set which column shall contain the tree-like expanders.
    fn set_expander_column<D: DataViewColumnMethods>(&self, col: Option<&D>) {
        unsafe {
            let col = match col {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewCtrl_SetExpanderColumn(self.as_ptr(), col)
        }
    }
    /// Changes the currently focused item.
    fn set_current_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_SetCurrentItem(self.as_ptr(), item)
        }
    }
    /// Set custom colours and/or font to use for the header.
    fn set_header_attr(&self, attr: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetHeaderAttr(self.as_ptr(), attr) }
    }
    /// Sets the indentation.
    fn set_indent(&self, indent: c_int) {
        unsafe { ffi::wxDataViewCtrl_SetIndent(self.as_ptr(), indent) }
    }
    /// Sets the selection to the array of wxDataViewItems.
    fn set_selections(&self, sel: *const c_void) {
        unsafe { ffi::wxDataViewCtrl_SetSelections(self.as_ptr(), sel) }
    }
    /// Unselect the given item.
    fn unselect<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Unselect(self.as_ptr(), item)
        }
    }
    /// Unselect all item.
    fn unselect_all(&self) {
        unsafe { ffi::wxDataViewCtrl_UnselectAll(self.as_ptr()) }
    }
    /// Sets the row height.
    fn set_row_height(&self, row_height: c_int) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetRowHeight(self.as_ptr(), row_height) }
    }
    /// Toggle sorting by the given column.
    fn toggle_sort_by_column(&self, column: c_int) {
        unsafe { ffi::wxDataViewCtrl_ToggleSortByColumn(self.as_ptr(), column) }
    }
    /// Return the number of items that can fit vertically in the visible area of the control.
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetCountPerPage(self.as_ptr()) }
    }
    /// Return the topmost visible item.
    fn get_top_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetTopItem(self.as_ptr())) }
    }
}

// wxDataViewCustomRenderer
pub trait DataViewCustomRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewCustomRenderer_GetDefaultType()).into() }
    }
    // DTOR: fn ~wxDataViewCustomRenderer()
    /// Override this to react to cell activation.
    fn activate_cell<
        R: RectMethods,
        D: DataViewModelMethods,
        D2: DataViewItemMethods,
        M: MouseEventMethods,
    >(
        &self,
        cell: &R,
        model: Option<&D>,
        item: &D2,
        col: c_uint,
        mouse_event: Option<&M>,
    ) -> bool {
        unsafe {
            let cell = cell.as_ptr();
            let model = match model {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let item = item.as_ptr();
            let mouse_event = match mouse_event {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Return the attribute to be used for rendering.
    fn get_attr(&self) -> DataViewItemAttrIsOwned<false> {
        unsafe {
            DataViewItemAttrIsOwned::from_ptr(ffi::wxDataViewCustomRenderer_GetAttr(self.as_ptr()))
        }
    }
    /// Return size required to show content.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDataViewCustomRenderer_GetSize(self.as_ptr())) }
    }
    // BLOCKED: fn LeftClick()
    // BLOCKED: fn Activate()
    // BLOCKED: fn Render()
    // BLOCKED: fn RenderText()
    /// Override this to start a drag operation.
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
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewDateRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewEvent
pub trait DataViewEventMethods: NotifyEventMethods {
    /// Returns the position of the column in the control or -1 if column field is unavailable for this event.
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetColumn(self.as_ptr()) }
    }
    /// Returns a pointer to the wxDataViewColumn from which the event was emitted or NULL.
    fn get_data_view_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe {
            DataViewColumn::option_from(ffi::wxDataViewEvent_GetDataViewColumn(self.as_ptr()))
        }
    }
    /// Returns the wxDataViewModel associated with the event.
    fn get_model(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewEvent_GetModel(self.as_ptr())) }
    }
    /// Returns the position of a context menu event in client coordinates.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDataViewEvent_GetPosition(self.as_ptr())) }
    }
    // BLOCKED: fn GetValue()
    /// Can be used to determine whether the new value is going to be accepted in wxEVT_DATAVIEW_ITEM_EDITING_DONE handler.
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxDataViewEvent_IsEditCancelled(self.as_ptr()) }
    }
    /// Sets the column index associated with this event.
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxDataViewEvent_SetColumn(self.as_ptr(), col) }
    }
    // BLOCKED: fn SetDataViewColumn()
    // BLOCKED: fn SetModel()
    /// Sets the value associated with this event.
    fn set_value(&self, value: *const c_void) {
        unsafe { ffi::wxDataViewEvent_SetValue(self.as_ptr(), value) }
    }
    /// Set wxDataObject for data transfer within a drag operation.
    fn set_data_object<D: DataObjectMethods>(&self, obj: Option<&D>) {
        unsafe {
            let obj = match obj {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewEvent_SetDataObject(self.as_ptr(), obj)
        }
    }
    /// Gets the wxDataFormat during a drop operation.
    fn get_data_format(&self) -> DataFormat {
        unsafe { DataFormat::from_ptr(ffi::wxDataViewEvent_GetDataFormat(self.as_ptr())) }
    }
    /// Gets the data size for a drop data transfer.
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataViewEvent_GetDataSize(self.as_ptr()) }
    }
    /// Gets the data buffer for a drop data transfer.
    fn get_data_buffer(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewEvent_GetDataBuffer(self.as_ptr()) }
    }
    /// Specify the kind of the drag operation to perform.
    fn set_drag_flags(&self, flags: c_int) {
        unsafe { ffi::wxDataViewEvent_SetDragFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetDropEffect()
    /// Return the first row that will be displayed.
    fn get_cache_from(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetCacheFrom(self.as_ptr()) }
    }
    /// Return the last row that will be displayed.
    fn get_cache_to(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetCacheTo(self.as_ptr()) }
    }
    /// Returns the index of the child item at which an item currently being dragged would be dropped.
    fn get_proposed_drop_index(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetProposedDropIndex(self.as_ptr()) }
    }
    /// Returns the item affected by the event.
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
    /// Gets the associated image.
    fn get_bitmap_bundle(&self) -> BitmapBundleIsOwned<false> {
        unsafe {
            BitmapBundleIsOwned::from_ptr(ffi::wxDataViewIconText_GetBitmapBundle(self.as_ptr()))
        }
    }
    /// Gets the icon.
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxDataViewIconText_GetIcon(self.as_ptr())) }
    }
    /// Gets the text.
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconText_GetText(self.as_ptr())).into() }
    }
    /// Sets the associated image.
    fn set_bitmap_bundle<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDataViewIconText_SetBitmapBundle(self.as_ptr(), bitmap)
        }
    }
    /// Set the icon.
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDataViewIconText_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Set the text.
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
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconTextRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewIndexListModel
pub trait DataViewIndexListModelMethods: DataViewListModelMethods {
    /// Returns the wxDataViewItem at the given row.
    fn get_item(&self, row: c_uint) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewIndexListModel_GetItem(self.as_ptr(), row)) }
    }
    /// Call this after if the data has to be read again from the model.
    fn reset(&self, new_size: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_Reset(self.as_ptr(), new_size) }
    }
    /// Call this after a row has been appended to the model.
    fn row_appended(&self) {
        unsafe { ffi::wxDataViewIndexListModel_RowAppended(self.as_ptr()) }
    }
    /// Call this after a row has been changed.
    fn row_changed(&self, row: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowChanged(self.as_ptr(), row) }
    }
    /// Call this after a row has been deleted.
    fn row_deleted(&self, row: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowDeleted(self.as_ptr(), row) }
    }
    /// Call this after a row has been inserted at the given position.
    fn row_inserted(&self, before: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowInserted(self.as_ptr(), before) }
    }
    /// Call this after a row has been prepended to the model.
    fn row_prepended(&self) {
        unsafe { ffi::wxDataViewIndexListModel_RowPrepended(self.as_ptr()) }
    }
    /// Call this after a value has been changed.
    fn row_value_changed(&self, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowValueChanged(self.as_ptr(), row, col) }
    }
    /// Call this after rows have been deleted.
    fn rows_deleted<A: ArrayIntMethods>(&self, rows: &A) {
        unsafe {
            let rows = rows.as_ptr();
            ffi::wxDataViewIndexListModel_RowsDeleted(self.as_ptr(), rows)
        }
    }
}

// wxDataViewItem
pub trait DataViewItemMethods: WxRustMethods {
    /// Returns the ID.
    fn get_id(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewItem_GetID(self.as_ptr()) }
    }
    /// Returns true if the ID is not NULL.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDataViewItem_IsOk(self.as_ptr()) }
    }
}

// wxDataViewItemAttr
pub trait DataViewItemAttrMethods: WxRustMethods {
    /// Call this to indicate that the item shall be displayed in bold text.
    fn set_bold(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetBold(self.as_ptr(), set) }
    }
    /// Call this to indicate that the item shall be displayed with that colour.
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetColour(self.as_ptr(), colour)
        }
    }
    /// Call this to set the background colour to use.
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Call this to indicate that the item shall be displayed in italic text.
    fn set_italic(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetItalic(self.as_ptr(), set) }
    }
    /// Call this to indicate that the item shall be displayed in strikethrough text.
    fn set_strikethrough(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetStrikethrough(self.as_ptr(), set) }
    }
    /// Returns true if the colour property has been set.
    fn has_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasColour(self.as_ptr()) }
    }
    /// Returns this attribute's colour.
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetColour(self.as_ptr())) }
    }
    /// Returns true if any property affecting the font has been set.
    fn has_font(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasFont(self.as_ptr()) }
    }
    /// Returns value of the bold property.
    fn get_bold(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetBold(self.as_ptr()) }
    }
    /// Returns value of the italics property.
    fn get_italic(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetItalic(self.as_ptr()) }
    }
    /// Returns true if the background colour property has been set.
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasBackgroundColour(self.as_ptr()) }
    }
    /// Returns the colour to be used for the background.
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    /// Returns true if none of the properties have been set.
    fn is_default(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_IsDefault(self.as_ptr()) }
    }
    /// Return the font based on the given one with this attribute applied to it.
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
    /// Returns index of the selected row or wxNOT_FOUND.
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
    /// Delete all items (= all rows).
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Returns the number of items (=rows) in the control.
    fn get_item_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListCtrl_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    /// Sets the value in the store and update the control.
    fn set_value(&self, value: *const c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_SetValue(self.as_ptr(), value, row, col) }
    }
    /// Returns the value from the store.
    fn get_value(&self, value: *mut c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_GetValue(self.as_ptr(), value, row, col) }
    }
    /// Sets the value in the store and update the control.
    fn set_text_value(&self, value: &str, row: c_uint, col: c_uint) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxDataViewListCtrl_SetTextValue(self.as_ptr(), value, row, col)
        }
    }
    /// Returns the value from the store.
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
    /// Sets the value in the store and update the control.
    fn set_toggle_value(&self, value: bool, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_SetToggleValue(self.as_ptr(), value, row, col) }
    }
    /// Returns the value from the store.
    fn get_toggle_value(&self, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListCtrl_GetToggleValue(self.as_ptr(), row, col) }
    }
    // NOT_SUPPORTED: fn SetItemData()
    // DTOR: fn ~wxDataViewListCtrl()
    /// Creates the control and a wxDataViewListStore as its internal model.
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
    /// Returns the position of given item or wxNOT_FOUND if it's not a valid item.
    fn item_to_row<D: DataViewItemMethods>(&self, item: &D) -> c_int {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewListCtrl_ItemToRow(self.as_ptr(), item)
        }
    }
    /// Returns the wxDataViewItem at the given row.
    fn row_to_item(&self, row: c_int) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewListCtrl_RowToItem(self.as_ptr(), row)) }
    }
}

// wxDataViewListModel
pub trait DataViewListModelMethods: DataViewModelMethods {
    // DTOR: fn ~wxDataViewListModel()
    /// Override this to indicate that the row has special font attributes.
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
    /// Override this if you want to disable specific items.
    fn is_enabled_by_row(&self, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListModel_IsEnabledByRow(self.as_ptr(), row, col) }
    }
    /// Returns the number of items (or rows) in the list.
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListModel_GetCount(self.as_ptr()) }
    }
    /// Returns the position of given item.
    fn get_row<D: DataViewItemMethods>(&self, item: &D) -> c_uint {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewListModel_GetRow(self.as_ptr(), item)
        }
    }
    /// Override this to allow getting values from the model.
    fn get_value_by_row(&self, variant: *mut c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListModel_GetValueByRow(self.as_ptr(), variant, row, col) }
    }
    /// Called in order to set a value in the model.
    fn set_value_by_row(&self, variant: *const c_void, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListModel_SetValueByRow(self.as_ptr(), variant, row, col) }
    }
}

// wxDataViewListStore
pub trait DataViewListStoreMethods: DataViewIndexListModelMethods {
    // DTOR: fn ~wxDataViewListStore()
    /// Prepends a data column.
    fn prepend_column(&self, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_PrependColumn(self.as_ptr(), varianttype)
        }
    }
    /// Inserts a data column before pos.
    fn insert_column(&self, pos: c_uint, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_InsertColumn(self.as_ptr(), pos, varianttype)
        }
    }
    /// Appends a data column.
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
    /// Delete all item (=all rows) in the store.
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewListStore_DeleteAllItems(self.as_ptr()) }
    }
    /// Returns the number of items (=rows) in the control.
    fn get_item_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListStore_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    // NOT_SUPPORTED: fn SetItemData()
}

// wxDataViewModel
pub trait DataViewModelMethods: RefCounterMethods {
    /// Adds a wxDataViewModelNotifier to the model.
    fn add_notifier<D: DataViewModelNotifierMethods>(&self, notifier: Option<&D>) {
        unsafe {
            let notifier = match notifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewModel_AddNotifier(self.as_ptr(), notifier)
        }
    }
    /// Change the value of the given item and update the control to reflect it.
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
    /// Called to inform the model that all of its data has been changed.
    fn cleared(&self) -> bool {
        unsafe { ffi::wxDataViewModel_Cleared(self.as_ptr()) }
    }
    /// The compare function to be used by the control.
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
    /// Override this to indicate that the item has special font attributes.
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
    /// Override this to indicate that the item should be disabled.
    fn is_enabled<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_IsEnabled(self.as_ptr(), item, col)
        }
    }
    /// Override this so the control can query the child items of an item.
    fn get_children<D: DataViewItemMethods>(&self, item: &D, children: *mut c_void) -> c_uint {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_GetChildren(self.as_ptr(), item, children)
        }
    }
    /// Override this to indicate which wxDataViewItem representing the parent of item or an invalid wxDataViewItem if the root item is the parent item.
    fn get_parent<D: DataViewItemMethods>(&self, item: &D) -> DataViewItem {
        unsafe {
            let item = item.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewModel_GetParent(self.as_ptr(), item))
        }
    }
    /// Override this to indicate the value of item.
    fn get_value<D: DataViewItemMethods>(&self, variant: *mut c_void, item: &D, col: c_uint) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_GetValue(self.as_ptr(), variant, item, col)
        }
    }
    /// Override this method to indicate if a container item merely acts as a headline (or for categorisation) or if it also acts a normal item with entries for further columns.
    fn has_container_columns<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_HasContainerColumns(self.as_ptr(), item)
        }
    }
    /// Override this to indicate that the model provides a default compare function that the control should use if no wxDataViewColumn has been chosen for sorting.
    fn has_default_compare(&self) -> bool {
        unsafe { ffi::wxDataViewModel_HasDefaultCompare(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HasValue()
    /// Override this to indicate if item is a container, i.e. if it can have child items.
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_IsContainer(self.as_ptr(), item)
        }
    }
    /// Call this to inform the model that an item has been added to the data.
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
    /// Call this to inform the model that an item has changed.
    fn item_changed<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_ItemChanged(self.as_ptr(), item)
        }
    }
    /// Call this to inform the model that an item has been deleted from the data.
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
    /// Call this to inform the model that several items have been added to the data.
    fn items_added<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModel_ItemsAdded(self.as_ptr(), parent, items)
        }
    }
    /// Call this to inform the model that several items have changed.
    fn items_changed(&self, items: *const c_void) -> bool {
        unsafe { ffi::wxDataViewModel_ItemsChanged(self.as_ptr(), items) }
    }
    /// Call this to inform the model that several items have been deleted.
    fn items_deleted<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModel_ItemsDeleted(self.as_ptr(), parent, items)
        }
    }
    /// Remove the notifier from the list of notifiers.
    fn remove_notifier<D: DataViewModelNotifierMethods>(&self, notifier: Option<&D>) {
        unsafe {
            let notifier = match notifier {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewModel_RemoveNotifier(self.as_ptr(), notifier)
        }
    }
    /// Call this to initiate a resort after the sort function has been changed.
    fn resort(&self) {
        unsafe { ffi::wxDataViewModel_Resort(self.as_ptr()) }
    }
    /// This gets called in order to set a value in the data model.
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
    /// Call this to inform this model that a value in the model has been changed.
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
    /// Called by owning model.
    fn cleared(&self) -> bool {
        unsafe { ffi::wxDataViewModelNotifier_Cleared(self.as_ptr()) }
    }
    /// Get owning wxDataViewModel.
    fn get_owner(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewModelNotifier_GetOwner(self.as_ptr())) }
    }
    /// Called by owning model.
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
    /// Called by owning model.
    fn item_changed<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ItemChanged(self.as_ptr(), item)
        }
    }
    /// Called by owning model.
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
    /// Called by owning model.
    fn items_added<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModelNotifier_ItemsAdded(self.as_ptr(), parent, items)
        }
    }
    /// Called by owning model.
    fn items_changed(&self, items: *const c_void) -> bool {
        unsafe { ffi::wxDataViewModelNotifier_ItemsChanged(self.as_ptr(), items) }
    }
    /// Called by owning model.
    fn items_deleted<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModelNotifier_ItemsDeleted(self.as_ptr(), parent, items)
        }
    }
    /// Called by owning model.
    fn resort(&self) {
        unsafe { ffi::wxDataViewModelNotifier_Resort(self.as_ptr()) }
    }
    /// Set owner of this notifier.
    fn set_owner<D: DataViewModelMethods>(&self, owner: Option<&D>) {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewModelNotifier_SetOwner(self.as_ptr(), owner)
        }
    }
    /// Called by owning model.
    fn value_changed<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ValueChanged(self.as_ptr(), item, col)
        }
    }
}

// wxDataViewProgressRenderer
pub trait DataViewProgressRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewProgressRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewRenderer
pub trait DataViewRendererMethods: ObjectMethods {
    /// Enable or disable replacing parts of the item text with ellipsis to make it fit the column width.
    fn enable_ellipsize(&self, mode: c_int) {
        unsafe { ffi::wxDataViewRenderer_EnableEllipsize(self.as_ptr(), mode) }
    }
    /// Disable replacing parts of the item text with ellipsis.
    fn disable_ellipsize(&self) {
        unsafe { ffi::wxDataViewRenderer_DisableEllipsize(self.as_ptr()) }
    }
    // BLOCKED: fn GetAccessibleDescription()
    /// Returns the alignment.
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxDataViewRenderer_GetAlignment(self.as_ptr()) }
    }
    /// Returns the ellipsize mode used by the renderer.
    fn get_ellipsize_mode(&self) -> c_int {
        unsafe { ffi::wxDataViewRenderer_GetEllipsizeMode(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetMode()
    /// Returns pointer to the owning wxDataViewColumn.
    fn get_owner(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewRenderer_GetOwner(self.as_ptr())) }
    }
    /// This methods retrieves the value from the renderer in order to transfer the value back to the data model.
    fn get_value(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_GetValue(self.as_ptr(), value) }
    }
    /// Returns a string with the type of the wxVariant supported by this renderer.
    fn get_variant_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewRenderer_GetVariantType(self.as_ptr())).into() }
    }
    /// Check if the given variant type is compatible with the type expected by this renderer.
    fn is_compatible_variant_type(&self, variant_type: &str) -> bool {
        unsafe {
            let variant_type = WxString::from(variant_type);
            let variant_type = variant_type.as_ptr();
            ffi::wxDataViewRenderer_IsCompatibleVariantType(self.as_ptr(), variant_type)
        }
    }
    /// Sets the alignment of the renderer's content.
    fn set_alignment(&self, align: c_int) {
        unsafe { ffi::wxDataViewRenderer_SetAlignment(self.as_ptr(), align) }
    }
    /// Sets the owning wxDataViewColumn.
    fn set_owner<D: DataViewColumnMethods>(&self, owner: Option<&D>) {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewRenderer_SetOwner(self.as_ptr(), owner)
        }
    }
    /// Set the value of the renderer (and thus its cell) to value.
    fn set_value(&self, value: *const c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_SetValue(self.as_ptr(), value) }
    }
    /// Set the transformer object to be used to customize values before they are rendered.
    fn set_value_adjuster(&self, transformer: *mut c_void) {
        unsafe { ffi::wxDataViewRenderer_SetValueAdjuster(self.as_ptr(), transformer) }
    }
    /// Before data is committed to the data model, it is passed to this method where it can be checked for validity.
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
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewTextRenderer_GetDefaultType()).into() }
    }
    /// Enable interpretation of markup in the item data.
    fn enable_markup(&self, enable: bool) {
        unsafe { ffi::wxDataViewTextRenderer_EnableMarkup(self.as_ptr(), enable) }
    }
}

// wxDataViewToggleRenderer
pub trait DataViewToggleRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewToggleRenderer_GetDefaultType()).into() }
    }
    /// Switch to using radiobutton-like appearance instead of the default checkbox-like one.
    fn show_as_radio(&self) {
        unsafe { ffi::wxDataViewToggleRenderer_ShowAsRadio(self.as_ptr()) }
    }
}

// wxDataViewTreeCtrl
pub trait DataViewTreeCtrlMethods: DataViewCtrlMethods {
    // DTOR: fn ~wxDataViewTreeCtrl()
    /// Appends a container to the given parent.
    fn append_container<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Appends an item to the given parent.
    fn append_item<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_AppendItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    /// Creates the control and a wxDataViewTreeStore as its internal model.
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
    /// Calls the identical method from wxDataViewTreeStore.
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteChildren(self.as_ptr(), item)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteItem(self.as_ptr(), item)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeCtrl_GetChildCount(self.as_ptr(), parent)
        }
    }
    /// Returns the image list.
    fn get_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxDataViewTreeCtrl_GetImageList(self.as_ptr())) }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            ClientData::option_from(ffi::wxDataViewTreeCtrl_GetItemData(self.as_ptr(), item))
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn get_item_expanded_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemExpandedIcon(
                self.as_ptr(),
                item,
            ))
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemIcon(self.as_ptr(), item))
        }
    }
    // BLOCKED: fn GetItemParent()
    /// Calls the identical method from wxDataViewTreeStore.
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeCtrl_GetItemText(self.as_ptr(), item)).into()
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
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
    /// Calls the same method from wxDataViewTreeStore but uses an index position in the image list instead of a wxIcon.
    fn insert_container<D: DataViewItemMethods, D2: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Calls the same method from wxDataViewTreeStore but uses an index position in the image list instead of a wxIcon.
    fn insert_item<D: DataViewItemMethods, D2: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Returns true if item is a container.
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_IsContainer(self.as_ptr(), item)
        }
    }
    /// Calls the same method from wxDataViewTreeStore but uses an index position in the image list instead of a wxIcon.
    fn prepend_container<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        expanded: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Calls the same method from wxDataViewTreeStore but uses an index position in the image list instead of a wxIcon.
    fn prepend_item<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: c_int,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeCtrl_PrependItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    /// Sets the image list.
    fn set_image_list<I: ImageListMethods>(&self, imagelist: Option<&I>) {
        unsafe {
            let imagelist = match imagelist {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewTreeCtrl_SetImageList(self.as_ptr(), imagelist)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    fn set_item_data<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        item: &D,
        data: Option<&C>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewTreeCtrl_SetItemData(self.as_ptr(), item, data)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
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
    /// Calls the identical method from wxDataViewTreeStore.
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
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
    /// Append a container.
    fn append_container<
        D: DataViewItemMethods,
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
        C: ClientDataMethods,
    >(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        expanded: &B2,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let expanded = expanded.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Append an item.
    fn append_item<D: DataViewItemMethods, B: BitmapBundleMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_AppendItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    /// Delete all item in the model.
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeStore_DeleteAllItems(self.as_ptr()) }
    }
    /// Delete all children of the item, but not the item itself.
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_DeleteChildren(self.as_ptr(), item)
        }
    }
    /// Delete this item.
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_DeleteItem(self.as_ptr(), item)
        }
    }
    /// Return the number of children of item.
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeStore_GetChildCount(self.as_ptr(), parent)
        }
    }
    /// Returns the client data associated with the item.
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            ClientData::option_from(ffi::wxDataViewTreeStore_GetItemData(self.as_ptr(), item))
        }
    }
    /// Returns the icon to display in expanded containers.
    fn get_item_expanded_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeStore_GetItemExpandedIcon(
                self.as_ptr(),
                item,
            ))
        }
    }
    /// Returns the icon of the item.
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeStore_GetItemIcon(self.as_ptr(), item))
        }
    }
    /// Returns the text of the item.
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeStore_GetItemText(self.as_ptr(), item)).into()
        }
    }
    /// Returns the nth child item of item.
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
    /// Inserts a container after previous.
    fn insert_container<
        D: DataViewItemMethods,
        D2: DataViewItemMethods,
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
        C: ClientDataMethods,
    >(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: &B,
        expanded: &B2,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let expanded = expanded.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Inserts an item after previous.
    fn insert_item<
        D: DataViewItemMethods,
        D2: DataViewItemMethods,
        B: BitmapBundleMethods,
        C: ClientDataMethods,
    >(
        &self,
        parent: &D,
        previous: &D2,
        text: &str,
        icon: &B,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let previous = previous.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Inserts a container before the first child item or parent.
    fn prepend_container<
        D: DataViewItemMethods,
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
        C: ClientDataMethods,
    >(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        expanded: &B2,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let expanded = expanded.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
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
    /// Inserts an item before the first child item or parent.
    fn prepend_item<D: DataViewItemMethods, B: BitmapBundleMethods, C: ClientDataMethods>(
        &self,
        parent: &D,
        text: &str,
        icon: &B,
        data: Option<&C>,
    ) -> DataViewItem {
        unsafe {
            let parent = parent.as_ptr();
            let text = WxString::from(text);
            let text = text.as_ptr();
            let icon = icon.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            DataViewItem::from_ptr(ffi::wxDataViewTreeStore_PrependItem(
                self.as_ptr(),
                parent,
                text,
                icon,
                data,
            ))
        }
    }
    /// Sets the client data associated with the item.
    fn set_item_data<D: DataViewItemMethods, C: ClientDataMethods>(
        &self,
        item: &D,
        data: Option<&C>,
    ) {
        unsafe {
            let item = item.as_ptr();
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDataViewTreeStore_SetItemData(self.as_ptr(), item, data)
        }
    }
    /// Sets the expanded icon for the item.
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
    /// Sets the icon for the item.
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeStore_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
}

// wxDataViewVirtualListModel
pub trait DataViewVirtualListModelMethods: DataViewListModelMethods {
    /// Returns the wxDataViewItem at the given row.
    fn get_item(&self, row: c_uint) -> DataViewItem {
        unsafe {
            DataViewItem::from_ptr(ffi::wxDataViewVirtualListModel_GetItem(self.as_ptr(), row))
        }
    }
    /// Call this after if the data has to be read again from the model.
    fn reset(&self, new_size: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_Reset(self.as_ptr(), new_size) }
    }
    /// Call this after a row has been appended to the model.
    fn row_appended(&self) {
        unsafe { ffi::wxDataViewVirtualListModel_RowAppended(self.as_ptr()) }
    }
    /// Call this after a row has been changed.
    fn row_changed(&self, row: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowChanged(self.as_ptr(), row) }
    }
    /// Call this after a row has been deleted.
    fn row_deleted(&self, row: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowDeleted(self.as_ptr(), row) }
    }
    /// Call this after a row has been inserted at the given position.
    fn row_inserted(&self, before: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowInserted(self.as_ptr(), before) }
    }
    /// Call this after a row has been prepended to the model.
    fn row_prepended(&self) {
        unsafe { ffi::wxDataViewVirtualListModel_RowPrepended(self.as_ptr()) }
    }
    /// Call this after a value has been changed.
    fn row_value_changed(&self, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowValueChanged(self.as_ptr(), row, col) }
    }
    /// Call this after rows have been deleted.
    fn rows_deleted<A: ArrayIntMethods>(&self, rows: &A) {
        unsafe {
            let rows = rows.as_ptr();
            ffi::wxDataViewVirtualListModel_RowsDeleted(self.as_ptr(), rows)
        }
    }
}

// wxDateEvent
pub trait DateEventMethods: CommandEventMethods {
    /// Returns the date.
    fn get_date(&self) -> DateTimeIsOwned<false> {
        unsafe { DateTimeIsOwned::from_ptr(ffi::wxDateEvent_GetDate(self.as_ptr())) }
    }
    /// Sets the date carried by the event, normally only used by the library internally.
    fn set_date<D: DateTimeMethods>(&self, date: &D) {
        unsafe {
            let date = date.as_ptr();
            ffi::wxDateEvent_SetDate(self.as_ptr(), date)
        }
    }
}

// wxDatePickerCtrl
pub trait DatePickerCtrlMethods: ControlMethods {
    /// Create the control window.
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
    /// If the control had been previously limited to a range of dates using SetRange(), returns the lower and upper bounds of this range.
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
    /// Returns the currently entered date.
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDatePickerCtrl_GetValue(self.as_ptr())) }
    }
    /// Set the text to show when there is no valid value.
    fn set_null_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDatePickerCtrl_SetNullText(self.as_ptr(), text)
        }
    }
    /// Sets the valid range for the date selection.
    fn set_range<D: DateTimeMethods, D2: DateTimeMethods>(&self, dt1: &D, dt2: &D2) {
        unsafe {
            let dt1 = dt1.as_ptr();
            let dt2 = dt2.as_ptr();
            ffi::wxDatePickerCtrl_SetRange(self.as_ptr(), dt1, dt2)
        }
    }
    /// Changes the current value of the control.
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
    /// Adds an identifier to be regarded as a main button for the non-scrolling area of a dialog.
    fn add_main_button_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_AddMainButtonId(self.as_ptr(), id) }
    }
    /// Returns true if this dialog can and should perform layout adaptation using DoLayoutAdaptation(), usually if the dialog is too large to fit on the display.
    fn can_do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_CanDoLayoutAdaptation(self.as_ptr()) }
    }
    /// Creates a sizer with standard buttons.
    fn create_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxDialog_CreateButtonSizer(self.as_ptr(), flags)) }
    }
    /// Creates a sizer with standard buttons using CreateButtonSizer() separated from the rest of the dialog contents by a horizontal wxStaticLine.
    fn create_separated_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe {
            Sizer::option_from(ffi::wxDialog_CreateSeparatedButtonSizer(
                self.as_ptr(),
                flags,
            ))
        }
    }
    /// Returns the sizer containing the given one with a separating wxStaticLine if necessarily.
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
    /// Creates a wxStdDialogButtonSizer with standard buttons.
    fn create_std_dialog_button_sizer(
        &self,
        flags: c_long,
    ) -> Option<StdDialogButtonSizerIsOwned<false>> {
        unsafe {
            StdDialogButtonSizer::option_from(ffi::wxDialog_CreateStdDialogButtonSizer(
                self.as_ptr(),
                flags,
            ))
        }
    }
    /// Splits text up at newlines and places the lines into wxStaticText objects with the specified maximum width in a vertical wxBoxSizer.
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
    /// Performs layout adaptation, usually if the dialog is too large to fit on the display.
    fn do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_DoLayoutAdaptation(self.as_ptr()) }
    }
    /// Ends a modal dialog, passing a value to be returned from the ShowModal() invocation.
    fn end_modal(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_EndModal(self.as_ptr(), ret_code) }
    }
    /// Gets the identifier of the button which works like standard OK button in this dialog.
    fn get_affirmative_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetAffirmativeId(self.as_ptr()) }
    }
    /// Override this to return a window containing the main content of the dialog.
    fn get_content_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDialog_GetContentWindow(self.as_ptr())) }
    }
    /// Gets the identifier of the button to map presses of ESC button to.
    fn get_escape_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetEscapeId(self.as_ptr()) }
    }
    /// Returns true if the dialog has been adapted, usually by making it scrollable to work with a small display.
    fn get_layout_adaptation_done(&self) -> bool {
        unsafe { ffi::wxDialog_GetLayoutAdaptationDone(self.as_ptr()) }
    }
    /// Gets a value representing the aggressiveness of search for buttons and sizers to be in the non-scrolling part of a layout-adapted dialog.
    fn get_layout_adaptation_level(&self) -> c_int {
        unsafe { ffi::wxDialog_GetLayoutAdaptationLevel(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetLayoutAdaptationMode()
    /// Returns an array of identifiers to be regarded as the main buttons for the non-scrolling area of a dialog.
    fn get_main_button_ids(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxDialog_GetMainButtonIds(self.as_ptr())) }
    }
    /// Gets the return code for this window.
    fn get_return_code(&self) -> c_int {
        unsafe { ffi::wxDialog_GetReturnCode(self.as_ptr()) }
    }
    // BLOCKED: fn GetToolBar()
    /// Returns true if id is in the array of identifiers to be regarded as the main buttons for the non-scrolling area of a dialog.
    fn is_main_button_id(&self, id: c_int) -> bool {
        unsafe { ffi::wxDialog_IsMainButtonId(self.as_ptr(), id) }
    }
    /// Returns true if the dialog box is modal, false otherwise.
    fn is_modal(&self) -> bool {
        unsafe { ffi::wxDialog_IsModal(self.as_ptr()) }
    }
    /// Sets the identifier to be used as OK button.
    fn set_affirmative_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetAffirmativeId(self.as_ptr(), id) }
    }
    /// Sets the identifier of the button which should work like the standard "Cancel" button in this dialog.
    fn set_escape_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetEscapeId(self.as_ptr(), id) }
    }
    /// Marks the dialog as having been adapted, usually by making it scrollable to work with a small display.
    fn set_layout_adaptation_done(&self, done: bool) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationDone(self.as_ptr(), done) }
    }
    /// Sets the aggressiveness of search for buttons and sizers to be in the non-scrolling part of a layout-adapted dialog.
    fn set_layout_adaptation_level(&self, level: c_int) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationLevel(self.as_ptr(), level) }
    }
    // NOT_SUPPORTED: fn SetLayoutAdaptationMode()
    /// Sets the return code for this window.
    fn set_return_code(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_SetReturnCode(self.as_ptr(), ret_code) }
    }
    /// Shows an application-modal dialog.
    fn show_modal(&self) -> c_int {
        unsafe { ffi::wxDialog_ShowModal(self.as_ptr()) }
    }
    /// Shows a dialog modal to the parent top level window only.
    fn show_window_modal(&self) {
        unsafe { ffi::wxDialog_ShowWindowModal(self.as_ptr()) }
    }
    // BLOCKED: fn ShowWindowModalThenDo()
    /// A static function enabling or disabling layout adaptation for all dialogs.
    fn enable_layout_adaptation(enable: bool) {
        unsafe { ffi::wxDialog_EnableLayoutAdaptation(enable) }
    }
    /// A static function getting the current layout adapter object.
    fn get_layout_adapter() -> Option<DialogLayoutAdapterIsOwned<false>> {
        unsafe { DialogLayoutAdapter::option_from(ffi::wxDialog_GetLayoutAdapter()) }
    }
    /// A static function returning true if layout adaptation is enabled for all dialogs.
    fn is_layout_adaptation_enabled() -> bool {
        unsafe { ffi::wxDialog_IsLayoutAdaptationEnabled() }
    }
    /// A static function for setting the current layout adapter object, returning the old adapter.
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
    /// Override this to returns true if adaptation can and should be done.
    fn can_do_layout_adaptation<D: DialogMethods>(&self, dialog: Option<&D>) -> bool {
        unsafe {
            let dialog = match dialog {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDialogLayoutAdapter_CanDoLayoutAdaptation(self.as_ptr(), dialog)
        }
    }
    /// Override this to perform layout adaptation, such as making parts of the dialog scroll and resizing the dialog to fit the display.
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
    /// Returns the message that will be displayed on the dialog.
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirDialog_GetMessage(self.as_ptr())).into() }
    }
    /// Returns the default or user-selected path.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirDialog_GetPath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the full paths of the chosen directories.
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxDirDialog_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Sets the message that will be displayed on the dialog.
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDirDialog_SetMessage(self.as_ptr(), message)
        }
    }
    /// Sets the default path.
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
    /// Creates the widgets with the given parameters.
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
    /// Returns the absolute path of the currently selected directory as a wxFileName object.
    fn get_dir_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxDirPickerCtrl_GetDirName(self.as_ptr())) }
    }
    /// Returns the absolute path of the currently selected directory.
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirPickerCtrl_GetPath(self.as_ptr())).into() }
    }
    /// Just like SetPath() but this function takes a wxFileName object.
    fn set_dir_name<F: FileNameMethods>(&self, dirname: &F) {
        unsafe {
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetDirName(self.as_ptr(), dirname)
        }
    }
    /// Set the directory to show when starting to browse for directories.
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDirPickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    /// Sets the absolute path of the currently selected directory.
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
    /// Changes the video mode of this display to the mode specified in the mode parameter.
    fn change_mode(&self, mode: *const c_void) -> bool {
        unsafe { ffi::wxDisplay_ChangeMode(self.as_ptr(), mode) }
    }
    /// Returns the client area of the display.
    fn get_client_area(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetClientArea(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetCurrentMode()
    /// Returns the bounding rectangle of the display whose index was passed to the constructor.
    fn get_geometry(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetGeometry(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetModes()
    /// Returns the display's name.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDisplay_GetName(self.as_ptr())).into() }
    }
    /// Returns display resolution in pixels per inch.
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetPPI(self.as_ptr())) }
    }
    /// Returns scaling factor used by this display.
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDisplay_GetScaleFactor(self.as_ptr()) }
    }
    /// Returns true if the display is the primary display.
    fn is_primary(&self) -> bool {
        unsafe { ffi::wxDisplay_IsPrimary(self.as_ptr()) }
    }
    /// Returns the number of connected displays.
    fn get_count() -> c_uint {
        unsafe { ffi::wxDisplay_GetCount() }
    }
    /// Returns the index of the display on which the given point lies, or wxNOT_FOUND if the point is not on any connected display.
    fn get_from_point<P: PointMethods>(pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDisplay_GetFromPoint(pt)
        }
    }
    /// Returns the index of the display on which the given window lies.
    fn get_from_window<W: WindowMethods>(win: Option<&W>) -> c_int {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxDisplay_GetFromWindow(win)
        }
    }
    /// Returns default display resolution for the current platform in pixels per inch.
    fn get_std_ppi_value() -> c_int {
        unsafe { ffi::wxDisplay_GetStdPPIValue() }
    }
    /// Returns default display resolution for the current platform as wxSize.
    fn get_std_ppi() -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetStdPPI()) }
    }
}

// wxDisplayChangedEvent
pub trait DisplayChangedEventMethods: EventMethods {}

// wxDragImage
pub trait DragImageMethods: ObjectMethods {
    /// Start dragging the image, in a window or full screen.
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
    /// Start dragging the image, using the first window to capture the mouse and the second to specify the bounding area.
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
    /// Draws the image on the device context with top-left corner at the given position.
    fn do_draw_image<D: DCMethods, P: PointMethods>(&self, dc: &D, pos: &P) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxDragImage_DoDrawImage(self.as_ptr(), dc, pos)
        }
    }
    /// Call this when the drag has finished.
    fn end_drag(&self) -> bool {
        unsafe { ffi::wxDragImage_EndDrag(self.as_ptr()) }
    }
    /// Returns the rectangle enclosing the image, assuming that the image is drawn with its top-left corner at the given point.
    fn get_image_rect<P: PointMethods>(&self, pos: &P) -> Rect {
        unsafe {
            let pos = pos.as_ptr();
            Rect::from_ptr(ffi::wxDragImage_GetImageRect(self.as_ptr(), pos))
        }
    }
    /// Hides the image.
    fn hide(&self) -> bool {
        unsafe { ffi::wxDragImage_Hide(self.as_ptr()) }
    }
    /// Call this to move the image to a new position.
    fn move_<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDragImage_Move(self.as_ptr(), pt)
        }
    }
    /// Shows the image.
    fn show(&self) -> bool {
        unsafe { ffi::wxDragImage_Show(self.as_ptr()) }
    }
    /// Override this if you wish to draw the window contents to the backing bitmap yourself.
    fn update_backing_from_window<
        D: DCMethods,
        M: MemoryDCMethods,
        R: RectMethods,
        R2: RectMethods,
    >(
        &self,
        window_dc: &D,
        dest_dc: &M,
        source_rect: &R,
        dest_rect: &R2,
    ) -> bool {
        unsafe {
            let window_dc = window_dc.as_ptr();
            let dest_dc = dest_dc.as_ptr();
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
    /// Returns the number of files dropped.
    fn get_number_of_files(&self) -> c_int {
        unsafe { ffi::wxDropFilesEvent_GetNumberOfFiles(self.as_ptr()) }
    }
    /// Returns the position at which the files were dropped.
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDropFilesEvent_GetPosition(self.as_ptr())) }
    }
}

// wxDropSource
pub trait DropSourceMethods: WxRustMethods {
    // NOT_SUPPORTED: fn DoDragDrop()
    /// Returns the wxDataObject object that has been assigned previously.
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDropSource_GetDataObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GiveFeedback()
    // NOT_SUPPORTED: fn SetCursor()
    // NOT_SUPPORTED: fn SetIcon()
    /// Sets the data wxDataObject associated with the drop source.
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
    /// This method may only be called from within OnData().
    fn get_data(&self) -> bool {
        unsafe { ffi::wxDropTarget_GetData(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn OnData()
    // NOT_SUPPORTED: fn OnDragOver()
    /// Called when the user drops a data object on the target.
    fn on_drop(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxDropTarget_OnDrop(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn OnEnter()
    /// Called when the mouse leaves the drop target.
    fn on_leave(&self) {
        unsafe { ffi::wxDropTarget_OnLeave(self.as_ptr()) }
    }
    /// Returns the data wxDataObject associated with the drop target.
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDropTarget_GetDataObject(self.as_ptr())) }
    }
    /// Sets the data wxDataObject associated with the drop target and deletes any previously associated data object.
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
