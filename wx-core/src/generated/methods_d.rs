use super::*;

// wxDC
/// This trait represents [C++ `wxDC` class](https://docs.wxwidgets.org/3.2/classwx_d_c.html)'s methods and inheritance.
///
/// See [`DCIsOwned`] documentation for the class usage.
pub trait DCMethods: ObjectMethods {
    /// Convert device X coordinate to logical coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    ///
    /// See [C++ `wxDC::DeviceToLogicalX()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a17971d3a2e3834f5b261aedd493ae08e).
    fn device_to_logical_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalX(self.as_ptr(), x) }
    }
    /// Convert device X coordinate to relative logical coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    ///
    /// See [C++ `wxDC::DeviceToLogicalXRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a173351b84f5c8ef0952d39e49021a84e).
    fn device_to_logical_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalXRel(self.as_ptr(), x) }
    }
    /// Converts device Y coordinate to logical coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    ///
    /// See [C++ `wxDC::DeviceToLogicalY()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#af669c71c68fb759927ec2e5721a6793e).
    fn device_to_logical_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalY(self.as_ptr(), y) }
    }
    /// Convert device Y coordinate to relative logical coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    ///
    /// See [C++ `wxDC::DeviceToLogicalYRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a13fe5057e20314f2d383907719b278b8).
    fn device_to_logical_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_DeviceToLogicalYRel(self.as_ptr(), y) }
    }
    /// Converts logical X coordinate to device coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    ///
    /// See [C++ `wxDC::LogicalToDeviceX()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ace891d047e6f95d8f4d3a83475ab228e).
    fn logical_to_device_x(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceX(self.as_ptr(), x) }
    }
    /// Converts logical X coordinate to relative device coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    ///
    /// See [C++ `wxDC::LogicalToDeviceXRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a68bac9e6f4bb21a3a38250c977bf2e5e).
    fn logical_to_device_x_rel(&self, x: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceXRel(self.as_ptr(), x) }
    }
    /// Converts logical Y coordinate to device coordinate, using the current mapping mode, user scale factor, device origin and axis orientation.
    ///
    /// See [C++ `wxDC::LogicalToDeviceY()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a373fbecf8b5fe2391d858150f94f8154).
    fn logical_to_device_y(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceY(self.as_ptr(), y) }
    }
    /// Converts logical Y coordinate to relative device coordinate, using the current mapping mode and user scale factor but ignoring the axis orientation.
    ///
    /// See [C++ `wxDC::LogicalToDeviceYRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#af9d891f58f39682fdd49c5aa5fc57ff2).
    fn logical_to_device_y_rel(&self, y: c_int) -> c_int {
        unsafe { ffi::wxDC_LogicalToDeviceYRel(self.as_ptr(), y) }
    }
    /// Converts device (x, y) coordinates to logical coordinates taking into account all applied transformations like the current mapping mode, scale factors, device origin, axes orientation, affine transformation.
    ///
    /// See [C++ `wxDC::DeviceToLogical()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae8cd3da121a3ed9a9759d2e76d544165).
    fn device_to_logical_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_DeviceToLogical(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DeviceToLogical()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae8097d0ed3b03f2a76597f797d382e20).
    fn device_to_logical_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_DeviceToLogical1(self.as_ptr(), pt))
        }
    }
    /// Converts device x, y coordinates to relative logical coordinates taking into account all applied transformations like the current mapping mode, scale factors, affine transformation.
    ///
    /// See [C++ `wxDC::DeviceToLogicalRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a2b00b767c72f50b9926c25c7e0507e86).
    fn device_to_logical_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_DeviceToLogicalRel(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DeviceToLogicalRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a1244cd725789bf0acdaac7302f6c8ce3).
    fn device_to_logical_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_DeviceToLogicalRel1(self.as_ptr(), dim))
        }
    }
    /// Converts logical (x, y) coordinates to device coordinates taking into account all applied transformations like the current mapping mode, scale factors, device origin, axes orientation, affine transformation.
    ///
    /// See [C++ `wxDC::LogicalToDevice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a683615521ce8c9d9291d319a752f159a).
    fn logical_to_device_coord(&self, x: c_int, y: c_int) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_LogicalToDevice(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::LogicalToDevice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a482af66a591bb42868c48c2137db5e5a).
    fn logical_to_device_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_LogicalToDevice1(self.as_ptr(), pt))
        }
    }
    /// Converts logical x, y coordinates to relative device coordinates taking into account all applied transformations like the current mapping mode, scale factors, affine transformation.
    ///
    /// See [C++ `wxDC::LogicalToDeviceRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ace81a88211420b1c18405d258d66ee4f).
    fn logical_to_device_rel_int(&self, x: c_int, y: c_int) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_LogicalToDeviceRel(self.as_ptr(), x, y)) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::LogicalToDeviceRel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ac1ce1dfb6f8e3e2e0391584de9d0314c).
    fn logical_to_device_rel_size<S: SizeMethods>(&self, dim: &S) -> Size {
        unsafe {
            let dim = dim.as_ptr();
            Size::from_ptr(ffi::wxDC_LogicalToDeviceRel1(self.as_ptr(), dim))
        }
    }
    /// Clears the device context using the current background brush.
    ///
    /// See [C++ `wxDC::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#acf301dfd75b0f31d969ecb9daec21e85).
    fn clear(&self) {
        unsafe { ffi::wxDC_Clear(self.as_ptr()) }
    }
    /// Draws an arc from the given start to the given end point.
    ///
    /// See [C++ `wxDC::DrawArc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a348f8cd1ba0ffcf62b8145628b0a5492).
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
    ///
    /// See [C++ `wxDC::DrawArc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#abe49b852e96ff500ef6333bfc044890f).
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
    ///
    /// See [C++ `wxDC::DrawBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#af982eb2d3b10c5617ef3559d51a1defc).
    fn draw_bitmap_coord<B: BitmapMethods>(&self, bitmap: &B, x: c_int, y: c_int, use_mask: bool) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDC_DrawBitmap(self.as_ptr(), bitmap, x, y, use_mask)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9449053951eceeb7984125cd4a694fd8).
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
    ///
    /// See [C++ `wxDC::DrawCheckMark()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a8ad34eda4c0b75ca905d466c6328fe91).
    fn draw_check_mark_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawCheckMark(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawCheckMark()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a14fe0d838374721e98d6254cdd1484a6).
    fn draw_check_mark_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawCheckMark1(self.as_ptr(), rect)
        }
    }
    /// Draws a circle with the given centre and radius.
    ///
    /// See [C++ `wxDC::DrawCircle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a62fd1c810d532e53a25e3b2e6fd621f7).
    fn draw_circle_coord(&self, x: c_int, y: c_int, radius: c_int) {
        unsafe { ffi::wxDC_DrawCircle(self.as_ptr(), x, y, radius) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawCircle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a560192c6dcb33c5bde404f3647234657).
    fn draw_circle_point<P: PointMethods>(&self, pt: &P, radius: c_int) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_DrawCircle1(self.as_ptr(), pt, radius)
        }
    }
    /// Draws an ellipse contained in the rectangle specified either with the given top left corner and the given size or directly.
    ///
    /// See [C++ `wxDC::DrawEllipse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a340697f08f5fd08d9db383ffcef642c2).
    fn draw_ellipse_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawEllipse(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawEllipse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a8a7a8478797c599e91125168669a5f36).
    fn draw_ellipse_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, size: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let size = size.as_ptr();
            ffi::wxDC_DrawEllipse1(self.as_ptr(), pt, size)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawEllipse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a637167c299d7e58832cf9c71f6f64fd5).
    fn draw_ellipse_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawEllipse2(self.as_ptr(), rect)
        }
    }
    /// Draws an arc of an ellipse.
    ///
    /// See [C++ `wxDC::DrawEllipticArc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a821e0b82707ca0379273cca67913da06).
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
    ///
    /// See [C++ `wxDC::DrawEllipticArc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a5f18a7aee18b69c8721aec67ea0a3532).
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
    ///
    /// See [C++ `wxDC::DrawIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a890d011a03308a28039d4940dd04264e).
    fn draw_icon_coord<I: IconMethods>(&self, icon: &I, x: c_int, y: c_int) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDC_DrawIcon(self.as_ptr(), icon, x, y)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a5719d8c36a03d1679828c5d89d33dd67).
    fn draw_icon_point<I: IconMethods, P: PointMethods>(&self, icon: &I, pt: &P) {
        unsafe {
            let icon = icon.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawIcon1(self.as_ptr(), icon, pt)
        }
    }
    /// Draw optional bitmap and the text into the given rectangle and aligns it as specified by alignment parameter; it also will emphasize the character with the given index if it is != -1 and return the bounding rectangle if required.
    ///
    /// See [C++ `wxDC::DrawLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae28d7aa2e17a850f4ca15c042a870152).
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
    ///
    /// See [C++ `wxDC::DrawLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9e4ddef5a10fdcee96ff9ef2fded9ee3).
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
    ///
    /// See [C++ `wxDC::DrawLine()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a12f2c236d4d320acec0bc6fe20e8399d).
    fn draw_line_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int) {
        unsafe { ffi::wxDC_DrawLine(self.as_ptr(), x1, y1, x2, y2) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawLine()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a34f84b8b73cf782e6d9ab0f629598b6f).
    fn draw_line_point<P: PointMethods, P2: PointMethods>(&self, pt1: &P, pt2: &P2) {
        unsafe {
            let pt1 = pt1.as_ptr();
            let pt2 = pt2.as_ptr();
            ffi::wxDC_DrawLine1(self.as_ptr(), pt1, pt2)
        }
    }
    // NOT_SUPPORTED: fn DrawLines()
    /// This method uses a list of wxPoints, adding the optional offset coordinate.
    ///
    /// See [C++ `wxDC::DrawLines()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a23cbb356e46890c99fcb5304d077f888).
    fn draw_lines(&self, points: *const c_void, xoffset: c_int, yoffset: c_int) {
        unsafe { ffi::wxDC_DrawLines1(self.as_ptr(), points, xoffset, yoffset) }
    }
    /// Draws a point using the color of the current pen.
    ///
    /// See [C++ `wxDC::DrawPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a8380aab866e8f3947e0898cf08969d9f).
    fn draw_point_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_DrawPoint(self.as_ptr(), x, y) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9b75ad987adc0c01d13c34db101b3539).
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
    ///
    /// See [C++ `wxDC::DrawRectangle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a918b9ae3447a2fc13f4c38c628a45c01).
    fn draw_rectangle_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_DrawRectangle(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawRectangle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a5499c7359d84871343e4875902c06a69).
    fn draw_rectangle_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_DrawRectangle1(self.as_ptr(), pt, sz)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawRectangle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae0a8c6fe2eb45f6f03339b049db6b2b8).
    fn draw_rectangle_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRectangle2(self.as_ptr(), rect)
        }
    }
    /// Draws the text rotated by angle degrees (positive angles are counterclockwise; the full angle is 360 degrees).
    ///
    /// See [C++ `wxDC::DrawRotatedText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aae5aff11939d5c55ab5c50987e4f2521).
    fn draw_rotated_text_coord(&self, text: &str, x: c_int, y: c_int, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawRotatedText(self.as_ptr(), text, x, y, angle)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawRotatedText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a791bc8502da484f1d3163e360cf91e8a).
    fn draw_rotated_text_point<P: PointMethods>(&self, text: &str, point: &P, angle: c_double) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let point = point.as_ptr();
            ffi::wxDC_DrawRotatedText1(self.as_ptr(), text, point, angle)
        }
    }
    /// Draws a rectangle with the given top left corner, and with the given size.
    ///
    /// See [C++ `wxDC::DrawRoundedRectangle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a94db29e2a40a16dc19ac852d05cd65b0).
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
    ///
    /// See [C++ `wxDC::DrawRoundedRectangle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9e4b8ab031042016d434606eb7744c9c).
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
    ///
    /// See [C++ `wxDC::DrawRoundedRectangle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a926fb2251b9185e58f15c41f680aaf2a).
    fn draw_rounded_rectangle_rect<R: RectMethods>(&self, rect: &R, radius: c_double) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_DrawRoundedRectangle2(self.as_ptr(), rect, radius)
        }
    }
    // NOT_SUPPORTED: fn DrawSpline()
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawSpline()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aacfb4bab651f9eb572e42cd2870c40c6).
    fn draw_spline_pointlist(&self, points: *const c_void) {
        unsafe { ffi::wxDC_DrawSpline1(self.as_ptr(), points) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawSpline()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a60c358ff3cfbf2b6f6d1918b527de98a).
    fn draw_spline_coord(&self, x1: c_int, y1: c_int, x2: c_int, y2: c_int, x3: c_int, y3: c_int) {
        unsafe { ffi::wxDC_DrawSpline2(self.as_ptr(), x1, y1, x2, y2, x3, y3) }
    }
    /// Draws a text string at the specified point, using the current text font, and the current text foreground and background colours.
    ///
    /// See [C++ `wxDC::DrawText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a11d35ce34fccb57b0efc7dc91168660b).
    fn draw_text_coord(&self, text: &str, x: c_int, y: c_int) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDC_DrawText(self.as_ptr(), text, x, y)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::DrawText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a17f5674c449449b730c63f773534d721).
    fn draw_text_point<P: PointMethods>(&self, text: &str, pt: &P) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let pt = pt.as_ptr();
            ffi::wxDC_DrawText1(self.as_ptr(), text, pt)
        }
    }
    /// Fill the area specified by rect with a radial gradient, starting from initialColour at the centre of the circle and fading to destColour on the circle outside.
    ///
    /// See [C++ `wxDC::GradientFillConcentric()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a323802ed579056fce98220f5d1778076).
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
    ///
    /// See [C++ `wxDC::GradientFillConcentric()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a2f0cd1850aefeda55b25cf56d55ac495).
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
    ///
    /// See [C++ `wxDC::GradientFillLinear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9cfbde2fcde06ffacf323f3a9dd1b020).
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
    ///
    /// See [C++ `wxDC::CrossHair()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#af8bab4cd7ffc3050974236a32afa1e1d).
    fn cross_hair_coord(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CrossHair(self.as_ptr(), x, y) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::CrossHair()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ad5ad1b17105668a9cba6f4f6cc902a02).
    fn cross_hair_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDC_CrossHair1(self.as_ptr(), pt)
        }
    }
    /// Destroys the current clipping region so that none of the DC is clipped.
    ///
    /// See [C++ `wxDC::DestroyClippingRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae0b0fc593c4559b9ac70e121bd28e3b4).
    fn destroy_clipping_region(&self) {
        unsafe { ffi::wxDC_DestroyClippingRegion(self.as_ptr()) }
    }
    /// Gets the rectangle surrounding the current clipping region.
    ///
    /// See [C++ `wxDC::GetClippingBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a866cebe05c603892235701239f8dbd65).
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
    ///
    /// See [C++ `wxDC::GetClippingBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a5a379f8f932a5d7d2e05f0820932f444).
    fn get_clipping_box_rect<R: RectMethods>(&self, rect: &R) -> bool {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_GetClippingBox1(self.as_ptr(), rect)
        }
    }
    /// Sets the clipping region for this device context to the intersection of the given region described by the parameters of this method and the previously set clipping region.
    ///
    /// See [C++ `wxDC::SetClippingRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a21ce8b27db0da5d68b8571d0ff39114b).
    fn set_clipping_region_coord(&self, x: c_int, y: c_int, width: c_int, height: c_int) {
        unsafe { ffi::wxDC_SetClippingRegion(self.as_ptr(), x, y, width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::SetClippingRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a800f7ca2a4a7588ff68d808eb06191e1).
    fn set_clipping_region_point<P: PointMethods, S: SizeMethods>(&self, pt: &P, sz: &S) {
        unsafe {
            let pt = pt.as_ptr();
            let sz = sz.as_ptr();
            ffi::wxDC_SetClippingRegion1(self.as_ptr(), pt, sz)
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::SetClippingRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9a397f713db57f7999b5851a5b25dd84).
    fn set_clipping_region_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxDC_SetClippingRegion2(self.as_ptr(), rect)
        }
    }
    /// Sets the clipping region for this device context.
    ///
    /// See [C++ `wxDC::SetDeviceClippingRegion()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a382a46c105ebad94e848e74e9cc0b4b1).
    fn set_device_clipping_region<R: RegionMethods>(&self, region: &R) {
        unsafe {
            let region = region.as_ptr();
            ffi::wxDC_SetDeviceClippingRegion(self.as_ptr(), region)
        }
    }
    /// Gets the character height of the currently set font.
    ///
    /// See [C++ `wxDC::GetCharHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a426166f27052b98dffcee07b6d743098).
    fn get_char_height(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharHeight(self.as_ptr()) }
    }
    /// Gets the average character width of the currently set font.
    ///
    /// See [C++ `wxDC::GetCharWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a3079e3786b0d9160462e15cfe61422c4).
    fn get_char_width(&self) -> c_int {
        unsafe { ffi::wxDC_GetCharWidth(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetFontMetrics()
    /// Gets the dimensions of the string using the currently selected font.
    ///
    /// See [C++ `wxDC::GetMultiLineTextExtent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a3f59fa4cc99e7f8c84b945373d986a45).
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
    ///
    /// See [C++ `wxDC::GetMultiLineTextExtent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a827892cc43071baaad813cbddbba021b).
    fn get_multi_line_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetMultiLineTextExtent1(self.as_ptr(), string))
        }
    }
    /// Fills the widths array with the widths from the beginning of text to the corresponding character of text.
    ///
    /// See [C++ `wxDC::GetPartialTextExtents()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9caf42966be72457715b3188fe285220).
    fn get_partial_text_extents<A: ArrayIntMethods>(&self, text: &str, widths: &A) -> bool {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            let widths = widths.as_ptr();
            ffi::wxDC_GetPartialTextExtents(self.as_ptr(), text, widths)
        }
    }
    /// Gets the dimensions of the string using the currently selected font.
    ///
    /// See [C++ `wxDC::GetTextExtent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a8ad68582fbd5373b4af6bad906defeb4).
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
    ///
    /// See [C++ `wxDC::GetTextExtent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a1f9b9b72ba8c88c1217b6210af4c22c6).
    fn get_text_extent(&self, string: &str) -> Size {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            Size::from_ptr(ffi::wxDC_GetTextExtent1(self.as_ptr(), string))
        }
    }
    /// Returns the current background mode: wxBRUSHSTYLE_SOLID or wxBRUSHSTYLE_TRANSPARENT.
    ///
    /// See [C++ `wxDC::GetBackgroundMode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae4f34b8c09b8f7af5c848906ead10e64).
    fn get_background_mode(&self) -> c_int {
        unsafe { ffi::wxDC_GetBackgroundMode(self.as_ptr()) }
    }
    /// Gets the current font.
    ///
    /// See [C++ `wxDC::GetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a159ff92c61d4886e52d688ac4cb1128b).
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxDC_GetFont(self.as_ptr())) }
    }
    /// Gets the current layout direction of the device context.
    ///
    /// See [C++ `wxDC::GetLayoutDirection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a69139fe0c3b10e077a5ad4e9a8860b7f).
    fn get_layout_direction(&self) -> c_int {
        unsafe { ffi::wxDC_GetLayoutDirection(self.as_ptr()) }
    }
    /// Gets the current text background colour.
    ///
    /// See [C++ `wxDC::GetTextBackground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a9252db582396706072cc1e107eed401d).
    fn get_text_background(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextBackground(self.as_ptr())) }
    }
    /// Gets the current text foreground colour.
    ///
    /// See [C++ `wxDC::GetTextForeground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a6ce1f3e3beaf3af14bbff878b670b597).
    fn get_text_foreground(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDC_GetTextForeground(self.as_ptr())) }
    }
    /// Change the current background mode.
    ///
    /// See [C++ `wxDC::SetBackgroundMode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a86c405ae265e6fdb4e393c4c9ada73c0).
    fn set_background_mode(&self, mode: c_int) {
        unsafe { ffi::wxDC_SetBackgroundMode(self.as_ptr(), mode) }
    }
    /// Sets the current font for the DC.
    ///
    /// See [C++ `wxDC::SetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#afab18239d707cd403235b36a987171a8).
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDC_SetFont(self.as_ptr(), font)
        }
    }
    /// Sets the current text background colour for the DC.
    ///
    /// See [C++ `wxDC::SetTextBackground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a3ed22bd0a0b835d4d085261bb022766b).
    fn set_text_background<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextBackground(self.as_ptr(), colour)
        }
    }
    /// Sets the current text foreground colour for the DC.
    ///
    /// See [C++ `wxDC::SetTextForeground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aeac811df9a1688ce875117f3049849d6).
    fn set_text_foreground<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDC_SetTextForeground(self.as_ptr(), colour)
        }
    }
    /// Sets the current layout direction for the device context.
    ///
    /// See [C++ `wxDC::SetLayoutDirection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a16196571f402cabf506619e8bf9f1586).
    fn set_layout_direction(&self, dir: c_int) {
        unsafe { ffi::wxDC_SetLayoutDirection(self.as_ptr(), dir) }
    }
    /// Adds the specified point to the bounding box which can be retrieved with MinX(), MaxX() and MinY(), MaxY() functions.
    ///
    /// See [C++ `wxDC::CalcBoundingBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a850699d4fdc9006421b085d2d37fa0c0).
    fn calc_bounding_box(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_CalcBoundingBox(self.as_ptr(), x, y) }
    }
    /// Gets the maximum horizontal extent used in drawing commands so far.
    ///
    /// See [C++ `wxDC::MaxX()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a0021372eada83e1fa73a5364100377fc).
    fn max_x(&self) -> c_int {
        unsafe { ffi::wxDC_MaxX(self.as_ptr()) }
    }
    /// Gets the maximum vertical extent used in drawing commands so far.
    ///
    /// See [C++ `wxDC::MaxY()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a6484caea9b032fc399f900e46deb6be3).
    fn max_y(&self) -> c_int {
        unsafe { ffi::wxDC_MaxY(self.as_ptr()) }
    }
    /// Gets the minimum horizontal extent used in drawing commands so far.
    ///
    /// See [C++ `wxDC::MinX()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a8f205424c5badc4f76b77aa71887f5ff).
    fn min_x(&self) -> c_int {
        unsafe { ffi::wxDC_MinX(self.as_ptr()) }
    }
    /// Gets the minimum vertical extent used in drawing commands so far.
    ///
    /// See [C++ `wxDC::MinY()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a74abb8dd4308d7594e8d7a234d19e04f).
    fn min_y(&self) -> c_int {
        unsafe { ffi::wxDC_MinY(self.as_ptr()) }
    }
    /// Resets the bounding box: after a call to this function, the bounding box doesn't contain anything.
    ///
    /// See [C++ `wxDC::ResetBoundingBox()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a35ed8c0c64315ec85588142d44f83af8).
    fn reset_bounding_box(&self) {
        unsafe { ffi::wxDC_ResetBoundingBox(self.as_ptr()) }
    }
    /// Starts a document (only relevant when outputting to a printer).
    ///
    /// See [C++ `wxDC::StartDoc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ad6572581c9d31dc349b6a7462426856c).
    fn start_doc(&self, message: &str) -> bool {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDC_StartDoc(self.as_ptr(), message)
        }
    }
    /// Starts a document page (only relevant when outputting to a printer).
    ///
    /// See [C++ `wxDC::StartPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a94c855ceb9f2fd5dcd1cf61396c13576).
    fn start_page(&self) {
        unsafe { ffi::wxDC_StartPage(self.as_ptr()) }
    }
    /// Ends a document (only relevant when outputting to a printer).
    ///
    /// See [C++ `wxDC::EndDoc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a95a506a0153d24dc352577161d45081c).
    fn end_doc(&self) {
        unsafe { ffi::wxDC_EndDoc(self.as_ptr()) }
    }
    /// Ends a document page (only relevant when outputting to a printer).
    ///
    /// See [C++ `wxDC::EndPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a3867f84557ecaf68bfeacffea74e8902).
    fn end_page(&self) {
        unsafe { ffi::wxDC_EndPage(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Blit()
    // NOT_SUPPORTED: fn StretchBlit()
    /// Gets the brush used for painting the background.
    ///
    /// See [C++ `wxDC::GetBackground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#adfd305c5b5bc597f6c976fe511d30d08).
    fn get_background(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBackground(self.as_ptr())) }
    }
    /// Gets the current brush.
    ///
    /// See [C++ `wxDC::GetBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a2c7a9220abfcd672bf9b0839aef6e874).
    fn get_brush(&self) -> BrushIsOwned<false> {
        unsafe { BrushIsOwned::from_ptr(ffi::wxDC_GetBrush(self.as_ptr())) }
    }
    // BLOCKED: fn GetPen()
    /// Sets the current background brush for the DC.
    ///
    /// See [C++ `wxDC::SetBackground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ad0139f6542f619244b80d4db7f685f86).
    fn set_background<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBackground(self.as_ptr(), brush)
        }
    }
    /// Sets the current brush for the DC.
    ///
    /// See [C++ `wxDC::SetBrush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a13978b2624116987a59ff729c4f81a96).
    fn set_brush<B: BrushMethods>(&self, brush: &B) {
        unsafe {
            let brush = brush.as_ptr();
            ffi::wxDC_SetBrush(self.as_ptr(), brush)
        }
    }
    /// Sets the current pen for the DC.
    ///
    /// See [C++ `wxDC::SetPen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a0d229733fbc83c7e4c483c0714d090b2).
    fn set_pen<P: PenMethods>(&self, pen: &P) {
        unsafe {
            let pen = pen.as_ptr();
            ffi::wxDC_SetPen(self.as_ptr(), pen)
        }
    }
    /// Copy attributes from another DC.
    ///
    /// See [C++ `wxDC::CopyAttributes()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ad1258b299c3a92344f1bdedbb7fc3acc).
    fn copy_attributes<D: DCMethods>(&self, dc: &D) {
        unsafe {
            let dc = dc.as_ptr();
            ffi::wxDC_CopyAttributes(self.as_ptr(), dc)
        }
    }
    /// Returns the factor used for converting logical pixels to physical ones.
    ///
    /// See [C++ `wxDC::GetContentScaleFactor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a6c20d102c499c335547b63156d2bb631).
    fn get_content_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDC_GetContentScaleFactor(self.as_ptr()) }
    }
    /// Returns the depth (number of bits/pixel) of this DC.
    ///
    /// See [C++ `wxDC::GetDepth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a798700d907980c88c717e86a4a06e924).
    fn get_depth(&self) -> c_int {
        unsafe { ffi::wxDC_GetDepth(self.as_ptr()) }
    }
    /// Returns the current device origin.
    ///
    /// See [C++ `wxDC::GetDeviceOrigin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a1ce47e2939c15b7290540a4ca1736eb5).
    fn get_device_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetDeviceOrigin(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetLogicalFunction()
    // NOT_SUPPORTED: fn GetMapMode()
    /// Gets in colour the colour at the specified location.
    ///
    /// See [C++ `wxDC::GetPixel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#abe0f8a22ec58783bd728f01e493040d1).
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
    ///
    /// See [C++ `wxDC::GetPPI()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a5e5a7832177ceceee0b5c35d146a1d6a).
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetPPI(self.as_ptr())) }
    }
    /// Convert DPI-independent pixel values to the value in pixels appropriate for the DC.
    ///
    /// See [C++ `wxDC::FromDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a3772199152ffa9cedd329f1737ed2056).
    fn from_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_FromDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::FromDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a49f34844c09f1960eeca84bf0db6cd99).
    fn from_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_FromDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert DPI-independent value in pixels to the value in pixels appropriate for the DC.
    ///
    /// See [C++ `wxDC::FromDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aef329d3fd4ab4e8e7d6c0ebd35462dc9).
    fn from_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_FromDIP2(self.as_ptr(), d) }
    }
    /// Convert pixel values of the current DC to DPI-independent pixel values.
    ///
    /// See [C++ `wxDC::ToDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ae512fadbd38d4a82aa10ed2028408da9).
    fn to_dip_size<S: SizeMethods>(&self, sz: &S) -> Size {
        unsafe {
            let sz = sz.as_ptr();
            Size::from_ptr(ffi::wxDC_ToDIP(self.as_ptr(), sz))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::ToDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ac012d95ee8c1b35912ad8e485b6cbbbf).
    fn to_dip_point<P: PointMethods>(&self, pt: &P) -> Point {
        unsafe {
            let pt = pt.as_ptr();
            Point::from_ptr(ffi::wxDC_ToDIP1(self.as_ptr(), pt))
        }
    }
    /// Convert pixel values of the current DC to DPI-independent pixel values.
    ///
    /// See [C++ `wxDC::ToDIP()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a1cf1813ac567d368b0e917d947e6c1fc).
    fn to_dip_int(&self, d: c_int) -> c_int {
        unsafe { ffi::wxDC_ToDIP2(self.as_ptr(), d) }
    }
    /// Gets the horizontal and vertical extent of this device context in device units.
    ///
    /// See [C++ `wxDC::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aee6d97d655f20039745b5f6fb43a2d51).
    fn get_size_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSize(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a27377e56df8a908085a3f8f5ef772303).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSize1(self.as_ptr())) }
    }
    /// Returns the horizontal and vertical resolution in millimetres.
    ///
    /// See [C++ `wxDC::GetSizeMM()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a33e35c892ed21eb3d512598753c080e2).
    fn get_size_mm_coord(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxDC_GetSizeMM(self.as_ptr(), width, height) }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxDC::GetSizeMM()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#adb021c4bd833267efc58956487262082).
    fn get_size_mm(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDC_GetSizeMM1(self.as_ptr())) }
    }
    /// Gets the current user scale factor.
    ///
    /// See [C++ `wxDC::GetUserScale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a711601938dff67d42ec45b61b4d13db3).
    fn get_user_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetUserScale(self.as_ptr(), x, y) }
    }
    /// Returns true if the DC is ok to use.
    ///
    /// See [C++ `wxDC::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#af00c59870ef918f0647b53f52bc29d6a).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDC_IsOk(self.as_ptr()) }
    }
    /// Sets the x and y axis orientation (i.e. the direction from lowest to highest values on the axis).
    ///
    /// See [C++ `wxDC::SetAxisOrientation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a1ada4defde484280fb24c4c47d24e0e8).
    fn set_axis_orientation(&self, x_left_right: bool, y_bottom_up: bool) {
        unsafe { ffi::wxDC_SetAxisOrientation(self.as_ptr(), x_left_right, y_bottom_up) }
    }
    /// Sets the device origin (i.e. the origin in pixels after scaling has been applied).
    ///
    /// See [C++ `wxDC::SetDeviceOrigin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a0a1c7d7d07d1faf3f7b89698bde769f3).
    fn set_device_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetDeviceOrigin(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn SetLogicalFunction()
    // NOT_SUPPORTED: fn SetMapMode()
    /// If this is a window DC or memory DC, assigns the given palette to the window or bitmap associated with the DC.
    ///
    /// See [C++ `wxDC::SetPalette()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#afc58b0f4653159e713377d38c84a120f).
    fn set_palette<P: PaletteMethods>(&self, palette: &P) {
        unsafe {
            let palette = palette.as_ptr();
            ffi::wxDC_SetPalette(self.as_ptr(), palette)
        }
    }
    /// Sets the user scaling factor, useful for applications which require 'zooming'.
    ///
    /// See [C++ `wxDC::SetUserScale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a190e43cf66ef402aa67f759d20f22eb0).
    fn set_user_scale(&self, x_scale: c_double, y_scale: c_double) {
        unsafe { ffi::wxDC_SetUserScale(self.as_ptr(), x_scale, y_scale) }
    }
    /// Check if the use of transformation matrix is supported by the current system.
    ///
    /// See [C++ `wxDC::CanUseTransformMatrix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a02b3bde61bfa924e984a9381c69698c1).
    fn can_use_transform_matrix(&self) -> bool {
        unsafe { ffi::wxDC_CanUseTransformMatrix(self.as_ptr()) }
    }
    /// Set the transformation matrix.
    ///
    /// See [C++ `wxDC::SetTransformMatrix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a6e3243fcb5d194ef5637f4bda11a49c3).
    fn set_transform_matrix<A: AffineMatrix2DMethods>(&self, matrix: &A) -> bool {
        unsafe {
            let matrix = matrix.as_ptr();
            ffi::wxDC_SetTransformMatrix(self.as_ptr(), matrix)
        }
    }
    /// Return the transformation matrix used by this device context.
    ///
    /// See [C++ `wxDC::GetTransformMatrix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ac50798a205d501ac7124bcdd9f67c760).
    fn get_transform_matrix(&self) -> AffineMatrix2D {
        unsafe { AffineMatrix2D::from_ptr(ffi::wxDC_GetTransformMatrix(self.as_ptr())) }
    }
    /// Revert the transformation matrix to identity matrix.
    ///
    /// See [C++ `wxDC::ResetTransformMatrix()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a41a9a4f616da21afdcad0fe1585ca066).
    fn reset_transform_matrix(&self) {
        unsafe { ffi::wxDC_ResetTransformMatrix(self.as_ptr()) }
    }
    /// Does the DC support drawing bitmaps?
    ///
    /// See [C++ `wxDC::CanDrawBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a1effb5b9e03fe331515137145bf22e18).
    fn can_draw_bitmap(&self) -> bool {
        unsafe { ffi::wxDC_CanDrawBitmap(self.as_ptr()) }
    }
    /// Does the DC support calculating the size required to draw text?
    ///
    /// See [C++ `wxDC::CanGetTextExtent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aa933c630eb41e2e9d78324a95f4118b5).
    fn can_get_text_extent(&self) -> bool {
        unsafe { ffi::wxDC_CanGetTextExtent(self.as_ptr()) }
    }
    /// Returns a value that can be used as a handle to the native drawing context, if this wxDC has something that could be thought of in that way.
    ///
    /// See [C++ `wxDC::GetHandle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a11f58bc271d3bd10f45d6f1fd5300eb8).
    fn get_handle(&self) -> *mut c_void {
        unsafe { ffi::wxDC_GetHandle(self.as_ptr()) }
    }
    /// If supported by the platform and the type of DC, fetch the contents of the DC, or a subset of it, as a bitmap.
    ///
    /// See [C++ `wxDC::GetAsBitmap()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ad29258b841e124462aa74a5581526e9e).
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
    ///
    /// See [C++ `wxDC::SetLogicalScale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aae1c728cdd2f43601f876b7d67067a39).
    fn set_logical_scale(&self, x: c_double, y: c_double) {
        unsafe { ffi::wxDC_SetLogicalScale(self.as_ptr(), x, y) }
    }
    /// Return the scale set by the last call to SetLogicalScale().
    ///
    /// See [C++ `wxDC::GetLogicalScale()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ab6a2a41a573b659dc2331c387bab4d16).
    fn get_logical_scale(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalScale(self.as_ptr(), x, y) }
    }
    /// Change the offset used for translating wxDC coordinates.
    ///
    /// See [C++ `wxDC::SetLogicalOrigin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a4ce7dda4ff2f3ece524b8d538b346b6f).
    fn set_logical_origin(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDC_SetLogicalOrigin(self.as_ptr(), x, y) }
    }
    /// Return the coordinates of the logical point (0, 0).
    ///
    /// See [C++ `wxDC::GetLogicalOrigin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a4acfbc9623a3c441e22f5257f9516fc0).
    fn get_logical_origin_coord(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxDC_GetLogicalOrigin(self.as_ptr(), x, y) }
    }
    /// Does the DC support drawing bitmaps?
    ///
    /// See [C++ `wxDC::GetLogicalOrigin()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#aebaa077e6b083daf17dfdb35dbe87823).
    fn get_logical_origin(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDC_GetLogicalOrigin1(self.as_ptr())) }
    }
    /// If supported by the platform and the wxDC implementation, this method will return the wxGraphicsContext associated with the DC.
    ///
    /// See [C++ `wxDC::GetGraphicsContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#a5e837724cf289516f37790d5f8c20844).
    fn get_graphics_context(&self) -> Option<GraphicsContextIsOwned<false>> {
        unsafe { GraphicsContext::option_from(ffi::wxDC_GetGraphicsContext(self.as_ptr())) }
    }
    /// Associate a wxGraphicsContext with the DC.
    ///
    /// See [C++ `wxDC::SetGraphicsContext()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c.html#ac4414e973766f48689f8094fef9bf487).
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
/// This trait represents [C++ `wxDCBrushChanger` class](https://docs.wxwidgets.org/3.2/classwx_d_c_brush_changer.html)'s methods and inheritance.
///
/// See [`DCBrushChangerIsOwned`] documentation for the class usage.
pub trait DCBrushChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCBrushChanger()
}

// wxDCClipper
/// This trait represents [C++ `wxDCClipper` class](https://docs.wxwidgets.org/3.2/classwx_d_c_clipper.html)'s methods and inheritance.
///
/// See [`DCClipperIsOwned`] documentation for the class usage.
pub trait DCClipperMethods: WxRustMethods {
    // DTOR: fn ~wxDCClipper()
}

// wxDCFontChanger
/// This trait represents [C++ `wxDCFontChanger` class](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html)'s methods and inheritance.
///
/// See [`DCFontChangerIsOwned`] documentation for the class usage.
pub trait DCFontChangerMethods: WxRustMethods {
    /// Set the font to use.
    ///
    /// See [C++ `wxDCFontChanger::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_font_changer.html#a31c6248bba6b47ae1cb34158c77e731b).
    fn set<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxDCFontChanger_Set(self.as_ptr(), font)
        }
    }
    // DTOR: fn ~wxDCFontChanger()
}

// wxDCOverlay
/// This trait represents [C++ `wxDCOverlay` class](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html)'s methods and inheritance.
///
/// See [`DCOverlayIsOwned`] documentation for the class usage.
pub trait DCOverlayMethods: WxRustMethods {
    // DTOR: fn ~wxDCOverlay()
    /// Clears the layer, restoring the state at the last init.
    ///
    /// See [C++ `wxDCOverlay::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_overlay.html#a2e74433df21a5f6f9f3ba2a6509d2450).
    fn clear(&self) {
        unsafe { ffi::wxDCOverlay_Clear(self.as_ptr()) }
    }
}

// wxDCPenChanger
/// This trait represents [C++ `wxDCPenChanger` class](https://docs.wxwidgets.org/3.2/classwx_d_c_pen_changer.html)'s methods and inheritance.
///
/// See [`DCPenChangerIsOwned`] documentation for the class usage.
pub trait DCPenChangerMethods: WxRustMethods {
    // DTOR: fn ~wxDCPenChanger()
}

// wxDCTextColourChanger
/// This trait represents [C++ `wxDCTextColourChanger` class](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html)'s methods and inheritance.
///
/// See [`DCTextColourChangerIsOwned`] documentation for the class usage.
pub trait DCTextColourChangerMethods: WxRustMethods {
    /// Set the colour to use.
    ///
    /// See [C++ `wxDCTextColourChanger::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_d_c_text_colour_changer.html#a9f88532c95e96699397e0a5cd1c140fe).
    fn set<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxDCTextColourChanger_Set(self.as_ptr(), col)
        }
    }
    // DTOR: fn ~wxDCTextColourChanger()
}

// wxDataFormat
/// This trait represents [C++ `wxDataFormat` class](https://docs.wxwidgets.org/3.2/classwx_data_format.html)'s methods and inheritance.
///
/// See [`DataFormatIsOwned`] documentation for the class usage.
pub trait DataFormatMethods: WxRustMethods {
    /// Returns the name of a custom format (this function will fail for a standard format).
    ///
    /// See [C++ `wxDataFormat::GetId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_format.html#a3cfb41966f4e1392a474c383696374dc).
    fn get_id(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataFormat_GetId(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetType()
    /// Sets the format to be the custom format identified by the given name.
    ///
    /// See [C++ `wxDataFormat::SetId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_format.html#a3b3748f25fc57f360b3ea5f68c238ee7).
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
/// This trait represents [C++ `wxDataObject` class](https://docs.wxwidgets.org/3.2/classwx_data_object.html)'s methods and inheritance.
///
/// See [`DataObjectIsOwned`] documentation for the class usage.
pub trait DataObjectMethods: WxRustMethods {
    // DTOR: fn ~wxDataObject()
    // NOT_SUPPORTED: fn GetAllFormats()
    /// The method will write the data of the format format to the buffer buf.
    ///
    /// See [C++ `wxDataObject::GetDataHere()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object.html#aa48761390b25d797d4cad393db752568).
    fn get_data_here<D: DataFormatMethods>(&self, format: &D, buf: *mut c_void) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_GetDataHere(self.as_ptr(), format, buf)
        }
    }
    /// Returns the data size of the given format format.
    ///
    /// See [C++ `wxDataObject::GetDataSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object.html#a10674c65e59f08ba318f942e410b8627).
    fn get_data_size<D: DataFormatMethods>(&self, format: &D) -> usize {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_GetDataSize(self.as_ptr(), format)
        }
    }
    // NOT_SUPPORTED: fn GetFormatCount()
    // NOT_SUPPORTED: fn GetPreferredFormat()
    /// Set the data in the format format of the length len provided in the buffer buf.
    ///
    /// See [C++ `wxDataObject::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object.html#ab73fa5f1cb933c67df20ddb1fad99071).
    fn set_data<D: DataFormatMethods>(&self, format: &D, len: usize, buf: *const c_void) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObject_SetData(self.as_ptr(), format, len, buf)
        }
    }
    // NOT_SUPPORTED: fn IsSupported()
}

// wxDataObjectComposite
/// This trait represents [C++ `wxDataObjectComposite` class](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html)'s methods and inheritance.
///
/// See [`DataObjectCompositeIsOwned`] documentation for the class usage.
pub trait DataObjectCompositeMethods: DataObjectMethods {
    /// Adds the dataObject to the list of supported objects and it becomes the preferred object if preferred is true.
    ///
    /// See [C++ `wxDataObjectComposite::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html#a7406b1fbd05e00de815e03eab39d1ce3).
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
    ///
    /// See [C++ `wxDataObjectComposite::GetReceivedFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_composite.html#a2ed91d70d40dae898aaafaa7595d0859).
    fn get_received_format(&self) -> DataFormat {
        unsafe { DataFormat::from_ptr(ffi::wxDataObjectComposite_GetReceivedFormat(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetObject()
}

// wxDataObjectSimple
/// This trait represents [C++ `wxDataObjectSimple` class](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html)'s methods and inheritance.
///
/// See [`DataObjectSimpleIsOwned`] documentation for the class usage.
pub trait DataObjectSimpleMethods: DataObjectMethods {
    /// Copy the data to the buffer, return true on success.
    ///
    /// See [C++ `wxDataObjectSimple::GetDataHere()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#a2d6fdc3eaa2bdd613f17c85c9600ecd2).
    fn get_data_here_void(&self, buf: *mut c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_GetDataHere(self.as_ptr(), buf) }
    }
    /// Gets the size of our data.
    ///
    /// See [C++ `wxDataObjectSimple::GetDataSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#a1188fa9b93e42d90a0710475d255eef3).
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataObjectSimple_GetDataSize(self.as_ptr()) }
    }
    // BLOCKED: fn GetFormat()
    /// Copy the data from the buffer, return true on success.
    ///
    /// See [C++ `wxDataObjectSimple::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#a064b6e42ceb86247318e7ab62bb47442).
    fn set_data_sz(&self, len: usize, buf: *const c_void) -> bool {
        unsafe { ffi::wxDataObjectSimple_SetData(self.as_ptr(), len, buf) }
    }
    /// Sets the supported format.
    ///
    /// See [C++ `wxDataObjectSimple::SetFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_object_simple.html#adf9d889c7fcc7e05cb6253b3b1e3cb0f).
    fn set_format<D: DataFormatMethods>(&self, format: &D) {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataObjectSimple_SetFormat(self.as_ptr(), format)
        }
    }
}

// wxDataViewBitmapRenderer
/// This trait represents [C++ `wxDataViewBitmapRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_bitmap_renderer.html)'s methods and inheritance.
///
/// See [`DataViewBitmapRendererIsOwned`] documentation for the class usage.
pub trait DataViewBitmapRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewBitmapRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_bitmap_renderer.html#a2959ca10bdb8f4b432ed39bdf58fe537).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewBitmapRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewChoiceByIndexRenderer
/// This trait represents [C++ `wxDataViewChoiceByIndexRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_by_index_renderer.html)'s methods and inheritance.
///
/// See [`DataViewChoiceByIndexRendererIsOwned`] documentation for the class usage.
pub trait DataViewChoiceByIndexRendererMethods: DataViewChoiceRendererMethods {}

// wxDataViewChoiceRenderer
/// This trait represents [C++ `wxDataViewChoiceRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_renderer.html)'s methods and inheritance.
///
/// See [`DataViewChoiceRendererIsOwned`] documentation for the class usage.
pub trait DataViewChoiceRendererMethods: DataViewRendererMethods {
    /// Returns the choice referred to by index.
    ///
    /// See [C++ `wxDataViewChoiceRenderer::GetChoice()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_renderer.html#a650942545f8ab70827e7e0882aef775b).
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
    ///
    /// See [C++ `wxDataViewChoiceRenderer::GetChoices()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_choice_renderer.html#ae1a00bb7d4c566f799256a8935e2707c).
    fn get_choices(&self) -> ArrayStringIsOwned<false> {
        unsafe {
            ArrayStringIsOwned::from_ptr(ffi::wxDataViewChoiceRenderer_GetChoices(self.as_ptr()))
        }
    }
}

// wxDataViewColumn
/// This trait represents [C++ `wxDataViewColumn` class](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html)'s methods and inheritance.
///
/// See [`DataViewColumnIsOwned`] documentation for the class usage.
pub trait DataViewColumnMethods: SettableHeaderColumnMethods {
    /// Returns the index of the column of the model, which this wxDataViewColumn is displaying.
    ///
    /// See [C++ `wxDataViewColumn::GetModelColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#a014d76ba96b8ce8a1c561d3213fd40b7).
    fn get_model_column(&self) -> c_uint {
        unsafe { ffi::wxDataViewColumn_GetModelColumn(self.as_ptr()) }
    }
    /// Returns the owning wxDataViewCtrl.
    ///
    /// See [C++ `wxDataViewColumn::GetOwner()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#adb8a72abe9b7a696c744d579f8a39f5f).
    fn get_owner(&self) -> WeakRef<DataViewCtrl> {
        unsafe { WeakRef::<DataViewCtrl>::from(ffi::wxDataViewColumn_GetOwner(self.as_ptr())) }
    }
    /// Returns the renderer of this wxDataViewColumn.
    ///
    /// See [C++ `wxDataViewColumn::GetRenderer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_column.html#a03f763111d1790ef0f49044ed09f1bb4).
    fn get_renderer(&self) -> Option<DataViewRendererIsOwned<false>> {
        unsafe { DataViewRenderer::option_from(ffi::wxDataViewColumn_GetRenderer(self.as_ptr())) }
    }
}

// wxDataViewCtrl
/// This trait represents [C++ `wxDataViewCtrl` class](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html)'s methods and inheritance.
///
/// See [`DataViewCtrlIsOwned`] documentation for the class usage.
pub trait DataViewCtrlMethods: ControlMethods {
    // DTOR: fn ~wxDataViewCtrl()
    /// Call to allow using multiple columns for sorting.
    ///
    /// See [C++ `wxDataViewCtrl::AllowMultiColumnSort()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a2daf77c3db98c36db429575b747a4389).
    fn allow_multi_column_sort(&self, allow: bool) -> bool {
        unsafe { ffi::wxDataViewCtrl_AllowMultiColumnSort(self.as_ptr(), allow) }
    }
    /// Appends a wxDataViewColumn to the control.
    ///
    /// See [C++ `wxDataViewCtrl::AppendColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#ad5f39c5a8ff76f35bfd08a456cefbf5f).
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
    ///
    /// See [C++ `wxDataViewCtrl::PrependColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a86ad5d9d32244f14fb6d6fc447f95fbc).
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
    ///
    /// See [C++ `wxDataViewCtrl::InsertColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a42128862bfc32b6157290b2257e739a3).
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
    ///
    /// See [C++ `wxDataViewCtrl::AssociateModel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#af4b6c168d14814d69875c42ed960108b).
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
    ///
    /// See [C++ `wxDataViewCtrl::ClearColumns()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#aec752b375a6edb1faecd16dc6c6dfac6).
    fn clear_columns(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_ClearColumns(self.as_ptr()) }
    }
    /// Collapses the item.
    ///
    /// See [C++ `wxDataViewCtrl::Collapse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#acec045cc2f40d30893842b8307190574).
    fn collapse<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Collapse(self.as_ptr(), item)
        }
    }
    /// Deletes given column.
    ///
    /// See [C++ `wxDataViewCtrl::DeleteColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a455cabf8a3faf5f2342626517b988b25).
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
    ///
    /// See [C++ `wxDataViewCtrl::EditItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a546fd8f32823ea2587ebaffbce984ce0).
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
    ///
    /// See [C++ `wxDataViewCtrl::EnableDragSource()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#ac5af3c3fc5990c7b4e3de87cc1087c96).
    fn enable_drag_source<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewCtrl_EnableDragSource(self.as_ptr(), format)
        }
    }
    /// Enable drop operations using any of the specified formats.
    ///
    /// See [C++ `wxDataViewCtrl::EnableDropTargets()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#aa942d7c6aa556b8b2a5506a0e5cd3de6).
    fn enable_drop_targets(&self, formats: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_EnableDropTargets(self.as_ptr(), formats) }
    }
    /// Enable drop operations using the given format.
    ///
    /// See [C++ `wxDataViewCtrl::EnableDropTarget()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a8110330a4fe556ee7eaceda17c661e4f).
    fn enable_drop_target<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewCtrl_EnableDropTarget(self.as_ptr(), format)
        }
    }
    /// Call this to ensure that the given item is visible.
    ///
    /// See [C++ `wxDataViewCtrl::EnsureVisible()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a892b67293dfc764d053fd764266c4326).
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
    ///
    /// See [C++ `wxDataViewCtrl::Expand()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#af1c473d9f4ef898be667873f8b562dd7).
    fn expand<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Expand(self.as_ptr(), item)
        }
    }
    /// Expands all ancestors of the item.
    ///
    /// See [C++ `wxDataViewCtrl::ExpandAncestors()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a764853c0ab0166e145b0c677e517a3d5).
    fn expand_ancestors<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandAncestors(self.as_ptr(), item)
        }
    }
    /// Expand all children of the given item recursively.
    ///
    /// See [C++ `wxDataViewCtrl::ExpandChildren()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a773cb1bdb4aca938d46db3239533c3b8).
    fn expand_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_ExpandChildren(self.as_ptr(), item)
        }
    }
    /// Returns pointer to the column.
    ///
    /// See [C++ `wxDataViewCtrl::GetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a180d390bdbb305f4e4f7e655ab3f53c9).
    fn get_column(&self, pos: c_uint) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetColumn(self.as_ptr(), pos)) }
    }
    /// Returns the number of columns.
    ///
    /// See [C++ `wxDataViewCtrl::GetColumnCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#aaa91e084fcd744360960d82dc8a0969d).
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewCtrl_GetColumnCount(self.as_ptr()) }
    }
    /// Returns the position of the column or -1 if not found in the control.
    ///
    /// See [C++ `wxDataViewCtrl::GetColumnPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a371aec8f2646787e0682dbcd248257d7).
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
    ///
    /// See [C++ `wxDataViewCtrl::GetExpanderColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#ab7b0674d41e845ae2172293d5dfdf1d1).
    fn get_expander_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetExpanderColumn(self.as_ptr())) }
    }
    /// Returns the currently focused item.
    ///
    /// See [C++ `wxDataViewCtrl::GetCurrentItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#afa975760a8ac2ba000861e8a6a3bd98c).
    fn get_current_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetCurrentItem(self.as_ptr())) }
    }
    /// Returns the column that currently has focus.
    ///
    /// See [C++ `wxDataViewCtrl::GetCurrentColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#aaee1feadf50bc3b35900ddb694410c9f).
    fn get_current_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetCurrentColumn(self.as_ptr())) }
    }
    /// Returns indentation.
    ///
    /// See [C++ `wxDataViewCtrl::GetIndent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#ab7e38a0bb67990143b1577c4f4b759b3).
    fn get_indent(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetIndent(self.as_ptr()) }
    }
    /// Returns item rectangle.
    ///
    /// See [C++ `wxDataViewCtrl::GetItemRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#aff0f7af39714f5beb5558bd802011ad2).
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
    ///
    /// See [C++ `wxDataViewCtrl::GetMainWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a59a36f218b233a7d26e2d6d3eccf839b).
    fn get_main_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDataViewCtrl_GetMainWindow(self.as_ptr())) }
    }
    /// Returns pointer to the data model associated with the control (if any).
    ///
    /// See [C++ `wxDataViewCtrl::GetModel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a8382fd6669ed2f0c7689d1ee7f9abd74).
    fn get_model(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewCtrl_GetModel(self.as_ptr())) }
    }
    /// Returns the number of currently selected items.
    ///
    /// See [C++ `wxDataViewCtrl::GetSelectedItemsCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a9c9b343b463d2bff5201ef6846d65b3b).
    fn get_selected_items_count(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelectedItemsCount(self.as_ptr()) }
    }
    /// Returns first selected item or an invalid item if none is selected.
    ///
    /// See [C++ `wxDataViewCtrl::GetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#afca6f3f199c44ae95a63052604c79f1c).
    fn get_selection(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetSelection(self.as_ptr())) }
    }
    /// Fills sel with currently selected items and returns their number.
    ///
    /// See [C++ `wxDataViewCtrl::GetSelections()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a262b359dce1de83570f8bfeeb678b47d).
    fn get_selections(&self, sel: *mut c_void) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetSelections(self.as_ptr(), sel) }
    }
    /// Returns the wxDataViewColumn currently responsible for sorting or NULL if none has been selected.
    ///
    /// See [C++ `wxDataViewCtrl::GetSortingColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#ae86cea223ed5272396a479864940005a).
    fn get_sorting_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewCtrl_GetSortingColumn(self.as_ptr())) }
    }
    // BLOCKED: fn GetSortingColumns()
    /// Returns true if any items are currently selected.
    ///
    /// See [C++ `wxDataViewCtrl::HasSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a22810354b766bfcdd2cdb2adbd19fb60).
    fn has_selection(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_HasSelection(self.as_ptr()) }
    }
    /// Retrieves item and column at the given point.
    ///
    /// See [C++ `wxDataViewCtrl::HitTest()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a986d43641872b4885c13d72df35de442).
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
    ///
    /// See [C++ `wxDataViewCtrl::IsExpanded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a79a0b51f58bf4d02b3920b3e73d67d66).
    fn is_expanded<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsExpanded(self.as_ptr(), item)
        }
    }
    /// Return true if using more than one column for sorting is allowed.
    ///
    /// See [C++ `wxDataViewCtrl::IsMultiColumnSortAllowed()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#aa1bd8165fc9a407a28565c824787fcd9).
    fn is_multi_column_sort_allowed(&self) -> bool {
        unsafe { ffi::wxDataViewCtrl_IsMultiColumnSortAllowed(self.as_ptr()) }
    }
    /// Return true if the item is selected.
    ///
    /// See [C++ `wxDataViewCtrl::IsSelected()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a517876b596c387a2837cdd6b1b9bcd89).
    fn is_selected<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_IsSelected(self.as_ptr(), item)
        }
    }
    /// Select the given item.
    ///
    /// See [C++ `wxDataViewCtrl::Select()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a3efbccfcaeacd81e1d33745333b4118f).
    fn select<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Select(self.as_ptr(), item)
        }
    }
    /// Select all items.
    ///
    /// See [C++ `wxDataViewCtrl::SelectAll()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a2731ddd9f9f0915b69beac95574ed348).
    fn select_all(&self) {
        unsafe { ffi::wxDataViewCtrl_SelectAll(self.as_ptr()) }
    }
    /// Set custom colour for the alternate rows used with wxDV_ROW_LINES style.
    ///
    /// See [C++ `wxDataViewCtrl::SetAlternateRowColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#adc477ce4e08c10fa49b2f2885a39da82).
    fn set_alternate_row_colour<C: ColourMethods>(&self, colour: &C) -> bool {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewCtrl_SetAlternateRowColour(self.as_ptr(), colour)
        }
    }
    /// Set which column shall contain the tree-like expanders.
    ///
    /// See [C++ `wxDataViewCtrl::SetExpanderColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a1dbbf7975e765e783a4c6e2fde7a4115).
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
    ///
    /// See [C++ `wxDataViewCtrl::SetCurrentItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a4cc4e7a506afcb1b646927bd91727ac6).
    fn set_current_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_SetCurrentItem(self.as_ptr(), item)
        }
    }
    /// Set custom colours and/or font to use for the header.
    ///
    /// See [C++ `wxDataViewCtrl::SetHeaderAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a26aff30acdbe2459e3d082d037e3f68b).
    fn set_header_attr(&self, attr: *const c_void) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetHeaderAttr(self.as_ptr(), attr) }
    }
    /// Sets the indentation.
    ///
    /// See [C++ `wxDataViewCtrl::SetIndent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a595ab054debffe1a3d906e3a748a382e).
    fn set_indent(&self, indent: c_int) {
        unsafe { ffi::wxDataViewCtrl_SetIndent(self.as_ptr(), indent) }
    }
    /// Sets the selection to the array of wxDataViewItems.
    ///
    /// See [C++ `wxDataViewCtrl::SetSelections()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#afc2024630264f2bc1c661f7d70624187).
    fn set_selections(&self, sel: *const c_void) {
        unsafe { ffi::wxDataViewCtrl_SetSelections(self.as_ptr(), sel) }
    }
    /// Unselect the given item.
    ///
    /// See [C++ `wxDataViewCtrl::Unselect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a8b5df79c5f0a80981da47b92b7e565ff).
    fn unselect<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewCtrl_Unselect(self.as_ptr(), item)
        }
    }
    /// Unselect all item.
    ///
    /// See [C++ `wxDataViewCtrl::UnselectAll()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a62f38a69855fcc8342a15950e6eb1356).
    fn unselect_all(&self) {
        unsafe { ffi::wxDataViewCtrl_UnselectAll(self.as_ptr()) }
    }
    /// Sets the row height.
    ///
    /// See [C++ `wxDataViewCtrl::SetRowHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a799a3c56989d986893fae2b0147e9553).
    fn set_row_height(&self, row_height: c_int) -> bool {
        unsafe { ffi::wxDataViewCtrl_SetRowHeight(self.as_ptr(), row_height) }
    }
    /// Toggle sorting by the given column.
    ///
    /// See [C++ `wxDataViewCtrl::ToggleSortByColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a84b66d5faa59df90591417d3b4a520f8).
    fn toggle_sort_by_column(&self, column: c_int) {
        unsafe { ffi::wxDataViewCtrl_ToggleSortByColumn(self.as_ptr(), column) }
    }
    /// Return the number of items that can fit vertically in the visible area of the control.
    ///
    /// See [C++ `wxDataViewCtrl::GetCountPerPage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a84dcb3dff7726312038765c5d34e8d41).
    fn get_count_per_page(&self) -> c_int {
        unsafe { ffi::wxDataViewCtrl_GetCountPerPage(self.as_ptr()) }
    }
    /// Return the topmost visible item.
    ///
    /// See [C++ `wxDataViewCtrl::GetTopItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_ctrl.html#a7eacf92e013cdf5159aeb3acd5fc620a).
    fn get_top_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewCtrl_GetTopItem(self.as_ptr())) }
    }
}

// wxDataViewCustomRenderer
/// This trait represents [C++ `wxDataViewCustomRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html)'s methods and inheritance.
///
/// See [`DataViewCustomRendererIsOwned`] documentation for the class usage.
pub trait DataViewCustomRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewCustomRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html#a30b8dfd2ed53f58fd31bd38e078244a4).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewCustomRenderer_GetDefaultType()).into() }
    }
    // DTOR: fn ~wxDataViewCustomRenderer()
    /// Override this to react to cell activation.
    ///
    /// See [C++ `wxDataViewCustomRenderer::ActivateCell()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html#a76e85467c89adae6612236d803a552fc).
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
    ///
    /// See [C++ `wxDataViewCustomRenderer::GetAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html#abb608aea38255f682d0f79cdcbabc54c).
    fn get_attr(&self) -> DataViewItemAttrIsOwned<false> {
        unsafe {
            DataViewItemAttrIsOwned::from_ptr(ffi::wxDataViewCustomRenderer_GetAttr(self.as_ptr()))
        }
    }
    /// Return size required to show content.
    ///
    /// See [C++ `wxDataViewCustomRenderer::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html#a0c5e1d559b46c9456836c27dce0ffa4d).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDataViewCustomRenderer_GetSize(self.as_ptr())) }
    }
    // BLOCKED: fn LeftClick()
    // BLOCKED: fn Activate()
    // BLOCKED: fn Render()
    // BLOCKED: fn RenderText()
    /// Override this to start a drag operation.
    ///
    /// See [C++ `wxDataViewCustomRenderer::StartDrag()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_custom_renderer.html#a36d6d5c64097bb48f67a712ddb7f97bf).
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
/// This trait represents [C++ `wxDataViewDateRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_date_renderer.html)'s methods and inheritance.
///
/// See [`DataViewDateRendererIsOwned`] documentation for the class usage.
pub trait DataViewDateRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewDateRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_date_renderer.html#a00acf6c620395c43d779b52623832106).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewDateRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewEvent
/// This trait represents [C++ `wxDataViewEvent` class](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html)'s methods and inheritance.
///
/// See [`DataViewEventIsOwned`] documentation for the class usage.
pub trait DataViewEventMethods: NotifyEventMethods {
    /// Returns the position of the column in the control or -1 if column field is unavailable for this event.
    ///
    /// See [C++ `wxDataViewEvent::GetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a07dcab4a49143f60ea5fb47de30b57ad).
    fn get_column(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetColumn(self.as_ptr()) }
    }
    /// Returns a pointer to the wxDataViewColumn from which the event was emitted or NULL.
    ///
    /// See [C++ `wxDataViewEvent::GetDataViewColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a1bbb9ce4bd2c7bff2c630e5988a38ffe).
    fn get_data_view_column(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe {
            DataViewColumn::option_from(ffi::wxDataViewEvent_GetDataViewColumn(self.as_ptr()))
        }
    }
    /// Returns the wxDataViewModel associated with the event.
    ///
    /// See [C++ `wxDataViewEvent::GetModel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a6c6bb8ff9e302e033fc0076df01a7c68).
    fn get_model(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewEvent_GetModel(self.as_ptr())) }
    }
    /// Returns the position of a context menu event in client coordinates.
    ///
    /// See [C++ `wxDataViewEvent::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a35617fabfda2023d8d98cd3d0341c676).
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDataViewEvent_GetPosition(self.as_ptr())) }
    }
    // BLOCKED: fn GetValue()
    /// Can be used to determine whether the new value is going to be accepted in wxEVT_DATAVIEW_ITEM_EDITING_DONE handler.
    ///
    /// See [C++ `wxDataViewEvent::IsEditCancelled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a983e5e639898a36da0d8e5092cee4654).
    fn is_edit_cancelled(&self) -> bool {
        unsafe { ffi::wxDataViewEvent_IsEditCancelled(self.as_ptr()) }
    }
    /// Sets the column index associated with this event.
    ///
    /// See [C++ `wxDataViewEvent::SetColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a66a231dc80ba6c36840251fef78294c5).
    fn set_column(&self, col: c_int) {
        unsafe { ffi::wxDataViewEvent_SetColumn(self.as_ptr(), col) }
    }
    // BLOCKED: fn SetDataViewColumn()
    // BLOCKED: fn SetModel()
    /// Sets the value associated with this event.
    ///
    /// See [C++ `wxDataViewEvent::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a59db8632419879f0e51d94b32b5ff914).
    fn set_value(&self, value: *const c_void) {
        unsafe { ffi::wxDataViewEvent_SetValue(self.as_ptr(), value) }
    }
    /// Set wxDataObject for data transfer within a drag operation.
    ///
    /// See [C++ `wxDataViewEvent::SetDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#ac56a99bcd34d37972addb6023966fef8).
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
    ///
    /// See [C++ `wxDataViewEvent::GetDataFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a4f08c15b0dee5c9fb9a87550f9f219bf).
    fn get_data_format(&self) -> DataFormat {
        unsafe { DataFormat::from_ptr(ffi::wxDataViewEvent_GetDataFormat(self.as_ptr())) }
    }
    /// Gets the data size for a drop data transfer.
    ///
    /// See [C++ `wxDataViewEvent::GetDataSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a88d0fc8c1812b355ca2da89d38a74b9f).
    fn get_data_size(&self) -> usize {
        unsafe { ffi::wxDataViewEvent_GetDataSize(self.as_ptr()) }
    }
    /// Gets the data buffer for a drop data transfer.
    ///
    /// See [C++ `wxDataViewEvent::GetDataBuffer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a98185d62a47a62b7a86f04247ce5c183).
    fn get_data_buffer(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewEvent_GetDataBuffer(self.as_ptr()) }
    }
    /// Specify the kind of the drag operation to perform.
    ///
    /// See [C++ `wxDataViewEvent::SetDragFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#aacb625c96175bf8f53411033237f4439).
    fn set_drag_flags(&self, flags: c_int) {
        unsafe { ffi::wxDataViewEvent_SetDragFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn GetDropEffect()
    /// Return the first row that will be displayed.
    ///
    /// See [C++ `wxDataViewEvent::GetCacheFrom()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a78130d7932802cb84c9274df4ad271e4).
    fn get_cache_from(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetCacheFrom(self.as_ptr()) }
    }
    /// Return the last row that will be displayed.
    ///
    /// See [C++ `wxDataViewEvent::GetCacheTo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a16bba58249dd10ec00336cfdf6b5272c).
    fn get_cache_to(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetCacheTo(self.as_ptr()) }
    }
    /// Returns the index of the child item at which an item currently being dragged would be dropped.
    ///
    /// See [C++ `wxDataViewEvent::GetProposedDropIndex()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a32a80304830c692d687d0c2eaa229246).
    fn get_proposed_drop_index(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetProposedDropIndex(self.as_ptr()) }
    }
    /// Returns the item affected by the event.
    ///
    /// See [C++ `wxDataViewEvent::GetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a12b6930d73aba4a8ba51ef6b1c09b84d).
    fn get_item(&self) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewEvent_GetItem(self.as_ptr())) }
    }
    // BLOCKED: fn SetItem()
    ///
    /// See [C++ `wxDataViewEvent::SetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#ab4ab7451fa21a879330110fa96107ab9).
    fn set_position(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxDataViewEvent_SetPosition(self.as_ptr(), x, y) }
    }
    ///
    /// See [C++ `wxDataViewEvent::SetCache()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a293ba83b240dbfb5d6ed78aada6792ea).
    fn set_cache(&self, from: c_int, to: c_int) {
        unsafe { ffi::wxDataViewEvent_SetCache(self.as_ptr(), from, to) }
    }
    ///
    /// See [C++ `wxDataViewEvent::GetDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a2fbee2bb8348b837079d60ccd989add8).
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDataViewEvent_GetDataObject(self.as_ptr())) }
    }
    ///
    /// See [C++ `wxDataViewEvent::SetDataFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a67f5b5297c0f414d24831e9c8605f6ea).
    fn set_data_format<D: DataFormatMethods>(&self, format: &D) {
        unsafe {
            let format = format.as_ptr();
            ffi::wxDataViewEvent_SetDataFormat(self.as_ptr(), format)
        }
    }
    ///
    /// See [C++ `wxDataViewEvent::SetDataSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a9c5bb283557e59ae23a1956bbb7216d0).
    fn set_data_size(&self, size: usize) {
        unsafe { ffi::wxDataViewEvent_SetDataSize(self.as_ptr(), size) }
    }
    ///
    /// See [C++ `wxDataViewEvent::SetDataBuffer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#a467b5a7586cab95f2bbe72b3528d4cb2).
    fn set_data_buffer(&self, buf: *mut c_void) {
        unsafe { ffi::wxDataViewEvent_SetDataBuffer(self.as_ptr(), buf) }
    }
    ///
    /// See [C++ `wxDataViewEvent::GetDragFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_event.html#ab4cf76370aa83e6f79c4b7313570749b).
    fn get_drag_flags(&self) -> c_int {
        unsafe { ffi::wxDataViewEvent_GetDragFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn SetDropEffect()
}

// wxDataViewIconText
/// This trait represents [C++ `wxDataViewIconText` class](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html)'s methods and inheritance.
///
/// See [`DataViewIconTextIsOwned`] documentation for the class usage.
pub trait DataViewIconTextMethods: ObjectMethods {
    /// Gets the associated image.
    ///
    /// See [C++ `wxDataViewIconText::GetBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#ad5e60844191755e2dbdd02a91cf857a4).
    fn get_bitmap_bundle(&self) -> BitmapBundleIsOwned<false> {
        unsafe {
            BitmapBundleIsOwned::from_ptr(ffi::wxDataViewIconText_GetBitmapBundle(self.as_ptr()))
        }
    }
    /// Gets the icon.
    ///
    /// See [C++ `wxDataViewIconText::GetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a433e73cfdc8c9cfd1a56aa1c16c9ab6d).
    fn get_icon(&self) -> Icon {
        unsafe { Icon::from_ptr(ffi::wxDataViewIconText_GetIcon(self.as_ptr())) }
    }
    /// Gets the text.
    ///
    /// See [C++ `wxDataViewIconText::GetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a809d7a78491a83b2b25d99f96d81b570).
    fn get_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconText_GetText(self.as_ptr())).into() }
    }
    /// Sets the associated image.
    ///
    /// See [C++ `wxDataViewIconText::SetBitmapBundle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#af640b312840b9b957d868778236fd8ab).
    fn set_bitmap_bundle<B: BitmapBundleMethods>(&self, bitmap: &B) {
        unsafe {
            let bitmap = bitmap.as_ptr();
            ffi::wxDataViewIconText_SetBitmapBundle(self.as_ptr(), bitmap)
        }
    }
    /// Set the icon.
    ///
    /// See [C++ `wxDataViewIconText::SetIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a1d24b561ea0d4b524f2cae323d0063e0).
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDataViewIconText_SetIcon(self.as_ptr(), icon)
        }
    }
    /// Set the text.
    ///
    /// See [C++ `wxDataViewIconText::SetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text.html#a506b582c8fd74deff82806efa898813d).
    fn set_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDataViewIconText_SetText(self.as_ptr(), text)
        }
    }
}

// wxDataViewIconTextRenderer
/// This trait represents [C++ `wxDataViewIconTextRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text_renderer.html)'s methods and inheritance.
///
/// See [`DataViewIconTextRendererIsOwned`] documentation for the class usage.
pub trait DataViewIconTextRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewIconTextRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_icon_text_renderer.html#a4aa051871e03e7335a448aa77da51499).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewIconTextRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewIndexListModel
/// This trait represents [C++ `wxDataViewIndexListModel` class](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html)'s methods and inheritance.
///
/// See [`DataViewIndexListModelIsOwned`] documentation for the class usage.
pub trait DataViewIndexListModelMethods: DataViewListModelMethods {
    /// Returns the wxDataViewItem at the given row.
    ///
    /// See [C++ `wxDataViewIndexListModel::GetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#a6659c0cc05f84f8004343933ccf76cd3).
    fn get_item(&self, row: c_uint) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewIndexListModel_GetItem(self.as_ptr(), row)) }
    }
    /// Call this after if the data has to be read again from the model.
    ///
    /// See [C++ `wxDataViewIndexListModel::Reset()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#aa6a67de8455657d6cdfaf09b82bc4575).
    fn reset(&self, new_size: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_Reset(self.as_ptr(), new_size) }
    }
    /// Call this after a row has been appended to the model.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowAppended()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#a5da484dc0f3becab36108e8eee5d99ab).
    fn row_appended(&self) {
        unsafe { ffi::wxDataViewIndexListModel_RowAppended(self.as_ptr()) }
    }
    /// Call this after a row has been changed.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#ad410f4e56acca24f479d96144ab323e0).
    fn row_changed(&self, row: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowChanged(self.as_ptr(), row) }
    }
    /// Call this after a row has been deleted.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#a471aaccb39cc69b2f960e44a77c2214a).
    fn row_deleted(&self, row: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowDeleted(self.as_ptr(), row) }
    }
    /// Call this after a row has been inserted at the given position.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowInserted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#a64c4c18d0c184f6c194084c982567562).
    fn row_inserted(&self, before: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowInserted(self.as_ptr(), before) }
    }
    /// Call this after a row has been prepended to the model.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowPrepended()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#a439de831e0f8a59f8cbabbc5d0ca92a6).
    fn row_prepended(&self) {
        unsafe { ffi::wxDataViewIndexListModel_RowPrepended(self.as_ptr()) }
    }
    /// Call this after a value has been changed.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowValueChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#a05efc3b43533df60186e62d8843f6c75).
    fn row_value_changed(&self, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewIndexListModel_RowValueChanged(self.as_ptr(), row, col) }
    }
    /// Call this after rows have been deleted.
    ///
    /// See [C++ `wxDataViewIndexListModel::RowsDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_index_list_model.html#abe50310f9d4c3ce668d73e0c9241f349).
    fn rows_deleted<A: ArrayIntMethods>(&self, rows: &A) {
        unsafe {
            let rows = rows.as_ptr();
            ffi::wxDataViewIndexListModel_RowsDeleted(self.as_ptr(), rows)
        }
    }
}

// wxDataViewItem
/// This trait represents [C++ `wxDataViewItem` class](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html)'s methods and inheritance.
///
/// See [`DataViewItemIsOwned`] documentation for the class usage.
pub trait DataViewItemMethods: WxRustMethods {
    /// Returns the ID.
    ///
    /// See [C++ `wxDataViewItem::GetID()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a85c2e30b8a2cf5c5f620aa461290e949).
    fn get_id(&self) -> *mut c_void {
        unsafe { ffi::wxDataViewItem_GetID(self.as_ptr()) }
    }
    /// Returns true if the ID is not NULL.
    ///
    /// See [C++ `wxDataViewItem::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item.html#a8efc8631d6cf9fd7bb100fb0045d887d).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxDataViewItem_IsOk(self.as_ptr()) }
    }
}

// wxDataViewItemAttr
/// This trait represents [C++ `wxDataViewItemAttr` class](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html)'s methods and inheritance.
///
/// See [`DataViewItemAttrIsOwned`] documentation for the class usage.
pub trait DataViewItemAttrMethods: WxRustMethods {
    /// Call this to indicate that the item shall be displayed in bold text.
    ///
    /// See [C++ `wxDataViewItemAttr::SetBold()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a77799bbfeb4a2d6d2b8dfe1eed63d6b8).
    fn set_bold(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetBold(self.as_ptr(), set) }
    }
    /// Call this to indicate that the item shall be displayed with that colour.
    ///
    /// See [C++ `wxDataViewItemAttr::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a4dc2d65d49bf66f6eb82abeae3bdc2c1).
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetColour(self.as_ptr(), colour)
        }
    }
    /// Call this to set the background colour to use.
    ///
    /// See [C++ `wxDataViewItemAttr::SetBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#ae52c06a3fbd22c7b3f6c9310d9fef6be).
    fn set_background_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxDataViewItemAttr_SetBackgroundColour(self.as_ptr(), colour)
        }
    }
    /// Call this to indicate that the item shall be displayed in italic text.
    ///
    /// See [C++ `wxDataViewItemAttr::SetItalic()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a4b97a2a572ed409ee487a5a01f550724).
    fn set_italic(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetItalic(self.as_ptr(), set) }
    }
    /// Call this to indicate that the item shall be displayed in strikethrough text.
    ///
    /// See [C++ `wxDataViewItemAttr::SetStrikethrough()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a671aabb1544facfd16b67b8c01f1830d).
    fn set_strikethrough(&self, set: bool) {
        unsafe { ffi::wxDataViewItemAttr_SetStrikethrough(self.as_ptr(), set) }
    }
    /// Returns true if the colour property has been set.
    ///
    /// See [C++ `wxDataViewItemAttr::HasColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a63e67d855447a91728a01876da70654d).
    fn has_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasColour(self.as_ptr()) }
    }
    /// Returns this attribute's colour.
    ///
    /// See [C++ `wxDataViewItemAttr::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a9b80872555a855204555047d34726a75).
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetColour(self.as_ptr())) }
    }
    /// Returns true if any property affecting the font has been set.
    ///
    /// See [C++ `wxDataViewItemAttr::HasFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a91c756bff410068be1113e4d4b61960a).
    fn has_font(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasFont(self.as_ptr()) }
    }
    /// Returns value of the bold property.
    ///
    /// See [C++ `wxDataViewItemAttr::GetBold()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a1db4b28a74883685b5c72e57afe580c5).
    fn get_bold(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetBold(self.as_ptr()) }
    }
    /// Returns value of the italics property.
    ///
    /// See [C++ `wxDataViewItemAttr::GetItalic()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a513f8a96ae5e83468615ed74b9d74e91).
    fn get_italic(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_GetItalic(self.as_ptr()) }
    }
    /// Returns true if the background colour property has been set.
    ///
    /// See [C++ `wxDataViewItemAttr::HasBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a256e12c9b4609a50baf9ba283e433bd2).
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_HasBackgroundColour(self.as_ptr()) }
    }
    /// Returns the colour to be used for the background.
    ///
    /// See [C++ `wxDataViewItemAttr::GetBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a34dfcc59a7c303c4ea4ad7c45ab383d4).
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxDataViewItemAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    /// Returns true if none of the properties have been set.
    ///
    /// See [C++ `wxDataViewItemAttr::IsDefault()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a08eb22a1f9b9a0653f4af37eebfa7028).
    fn is_default(&self) -> bool {
        unsafe { ffi::wxDataViewItemAttr_IsDefault(self.as_ptr()) }
    }
    /// Return the font based on the given one with this attribute applied to it.
    ///
    /// See [C++ `wxDataViewItemAttr::GetEffectiveFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_item_attr.html#a94924103d78fa6e35fc097d3a2e373f6).
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
/// This trait represents [C++ `wxDataViewListCtrl` class](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html)'s methods and inheritance.
///
/// See [`DataViewListCtrlIsOwned`] documentation for the class usage.
pub trait DataViewListCtrlMethods: DataViewCtrlMethods {
    /// Returns index of the selected row or wxNOT_FOUND.
    ///
    /// See [C++ `wxDataViewListCtrl::GetSelectedRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a481e5a13ed8d2099bcc7c56b3d1465e2).
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
    ///
    /// See [C++ `wxDataViewListCtrl::DeleteAllItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a3ff47949696f77f816851c22d1427177).
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewListCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Returns the number of items (=rows) in the control.
    ///
    /// See [C++ `wxDataViewListCtrl::GetItemCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a4afdcea2e71ff4f462937c4cacd99e7d).
    fn get_item_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListCtrl_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    /// Sets the value in the store and update the control.
    ///
    /// See [C++ `wxDataViewListCtrl::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#afa48329e396067530253546a847bf83a).
    fn set_value(&self, value: *const c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_SetValue(self.as_ptr(), value, row, col) }
    }
    /// Returns the value from the store.
    ///
    /// See [C++ `wxDataViewListCtrl::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#ab741f9e6c0ba81270aeee26c2e90f8c9).
    fn get_value(&self, value: *mut c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_GetValue(self.as_ptr(), value, row, col) }
    }
    /// Sets the value in the store and update the control.
    ///
    /// See [C++ `wxDataViewListCtrl::SetTextValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#afd747df820204b181026519827ed6234).
    fn set_text_value(&self, value: &str, row: c_uint, col: c_uint) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxDataViewListCtrl_SetTextValue(self.as_ptr(), value, row, col)
        }
    }
    /// Returns the value from the store.
    ///
    /// See [C++ `wxDataViewListCtrl::GetTextValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#aa46edefe3acbf87236caa19993695318).
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
    ///
    /// See [C++ `wxDataViewListCtrl::SetToggleValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a59fbc031d37a719a0e18b97dc5bcb97b).
    fn set_toggle_value(&self, value: bool, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListCtrl_SetToggleValue(self.as_ptr(), value, row, col) }
    }
    /// Returns the value from the store.
    ///
    /// See [C++ `wxDataViewListCtrl::GetToggleValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a09c9c7534e8776af71cb6c6abd0350b2).
    fn get_toggle_value(&self, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListCtrl_GetToggleValue(self.as_ptr(), row, col) }
    }
    // NOT_SUPPORTED: fn SetItemData()
    // DTOR: fn ~wxDataViewListCtrl()
    /// Creates the control and a wxDataViewListStore as its internal model.
    ///
    /// See [C++ `wxDataViewListCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a4528ac9c75b75d438cb73a5340fea99a).
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
    ///
    /// See [C++ `wxDataViewListCtrl::GetStore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a42ef4d50ff8e434528991d7118b20e8f).
    fn get_store(&self) -> Option<DataViewListStoreIsOwned<false>> {
        unsafe { DataViewListStore::option_from(ffi::wxDataViewListCtrl_GetStore1(self.as_ptr())) }
    }
    /// Returns the position of given item or wxNOT_FOUND if it's not a valid item.
    ///
    /// See [C++ `wxDataViewListCtrl::ItemToRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#a1fb0af5e3805a8b66d59b86dd0464f39).
    fn item_to_row<D: DataViewItemMethods>(&self, item: &D) -> c_int {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewListCtrl_ItemToRow(self.as_ptr(), item)
        }
    }
    /// Returns the wxDataViewItem at the given row.
    ///
    /// See [C++ `wxDataViewListCtrl::RowToItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_ctrl.html#adba85ea8543188db904da4913f7eff11).
    fn row_to_item(&self, row: c_int) -> DataViewItem {
        unsafe { DataViewItem::from_ptr(ffi::wxDataViewListCtrl_RowToItem(self.as_ptr(), row)) }
    }
}

// wxDataViewListModel
/// This trait represents [C++ `wxDataViewListModel` class](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html)'s methods and inheritance.
///
/// See [`DataViewListModelIsOwned`] documentation for the class usage.
pub trait DataViewListModelMethods: DataViewModelMethods {
    // DTOR: fn ~wxDataViewListModel()
    /// Override this to indicate that the row has special font attributes.
    ///
    /// See [C++ `wxDataViewListModel::GetAttrByRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html#a2fff48eb34020081a2b02cddec30bc97).
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
    ///
    /// See [C++ `wxDataViewListModel::IsEnabledByRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html#a51fb8e1249abd22dfb874dc00adbec02).
    fn is_enabled_by_row(&self, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListModel_IsEnabledByRow(self.as_ptr(), row, col) }
    }
    /// Returns the number of items (or rows) in the list.
    ///
    /// See [C++ `wxDataViewListModel::GetCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html#adda74dc8e6c95b12100832487fe9fdbf).
    fn get_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListModel_GetCount(self.as_ptr()) }
    }
    /// Returns the position of given item.
    ///
    /// See [C++ `wxDataViewListModel::GetRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html#aa5e888fd59a01fc4519c897368300855).
    fn get_row<D: DataViewItemMethods>(&self, item: &D) -> c_uint {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewListModel_GetRow(self.as_ptr(), item)
        }
    }
    /// Override this to allow getting values from the model.
    ///
    /// See [C++ `wxDataViewListModel::GetValueByRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html#a3a19138ea419ab8b981b260565069c7a).
    fn get_value_by_row(&self, variant: *mut c_void, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewListModel_GetValueByRow(self.as_ptr(), variant, row, col) }
    }
    /// Called in order to set a value in the model.
    ///
    /// See [C++ `wxDataViewListModel::SetValueByRow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_model.html#af0cf3adc92a3e62e62cfa2acfd11f227).
    fn set_value_by_row(&self, variant: *const c_void, row: c_uint, col: c_uint) -> bool {
        unsafe { ffi::wxDataViewListModel_SetValueByRow(self.as_ptr(), variant, row, col) }
    }
}

// wxDataViewListStore
/// This trait represents [C++ `wxDataViewListStore` class](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html)'s methods and inheritance.
///
/// See [`DataViewListStoreIsOwned`] documentation for the class usage.
pub trait DataViewListStoreMethods: DataViewIndexListModelMethods {
    // DTOR: fn ~wxDataViewListStore()
    /// Prepends a data column.
    ///
    /// See [C++ `wxDataViewListStore::PrependColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#af19bad463eb6a4291d6b5e2ef4097e4c).
    fn prepend_column(&self, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_PrependColumn(self.as_ptr(), varianttype)
        }
    }
    /// Inserts a data column before pos.
    ///
    /// See [C++ `wxDataViewListStore::InsertColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#aab90522970853a26206d31fb9fba8e9d).
    fn insert_column(&self, pos: c_uint, varianttype: &str) {
        unsafe {
            let varianttype = WxString::from(varianttype);
            let varianttype = varianttype.as_ptr();
            ffi::wxDataViewListStore_InsertColumn(self.as_ptr(), pos, varianttype)
        }
    }
    /// Appends a data column.
    ///
    /// See [C++ `wxDataViewListStore::AppendColumn()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#af08ff8fb12d19f533c34bf4ed19c6b83).
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
    ///
    /// See [C++ `wxDataViewListStore::DeleteAllItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#a76e88de57112a9e8c6230ddf774ca74e).
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewListStore_DeleteAllItems(self.as_ptr()) }
    }
    /// Returns the number of items (=rows) in the control.
    ///
    /// See [C++ `wxDataViewListStore::GetItemCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_list_store.html#a30fbefed3e771c987b74e80617339cb1).
    fn get_item_count(&self) -> c_uint {
        unsafe { ffi::wxDataViewListStore_GetItemCount(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetItemData()
    // NOT_SUPPORTED: fn SetItemData()
}

// wxDataViewModel
/// This trait represents [C++ `wxDataViewModel` class](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html)'s methods and inheritance.
///
/// See [`DataViewModelIsOwned`] documentation for the class usage.
pub trait DataViewModelMethods: RefCounterMethods {
    /// Adds a wxDataViewModelNotifier to the model.
    ///
    /// See [C++ `wxDataViewModel::AddNotifier()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#af975c0a3afb970a21b9691a92f44c6c8).
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
    ///
    /// See [C++ `wxDataViewModel::ChangeValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#ab52b55596d0724047be8cf62184f4468).
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
    ///
    /// See [C++ `wxDataViewModel::Cleared()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#ab1ce641ec025c4706f878d569a9516f4).
    fn cleared(&self) -> bool {
        unsafe { ffi::wxDataViewModel_Cleared(self.as_ptr()) }
    }
    /// The compare function to be used by the control.
    ///
    /// See [C++ `wxDataViewModel::Compare()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a2eefab97a9a73e5ff896222d3600c8bb).
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
    ///
    /// See [C++ `wxDataViewModel::GetAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a28b85b9aca3a1ea803940817d86b0b73).
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
    ///
    /// See [C++ `wxDataViewModel::IsEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a561564e63f8c975b4241ad5be74e6965).
    fn is_enabled<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_IsEnabled(self.as_ptr(), item, col)
        }
    }
    /// Override this so the control can query the child items of an item.
    ///
    /// See [C++ `wxDataViewModel::GetChildren()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a945bbb0523166c6b092af62c7ba7b2ac).
    fn get_children<D: DataViewItemMethods>(&self, item: &D, children: *mut c_void) -> c_uint {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_GetChildren(self.as_ptr(), item, children)
        }
    }
    /// Override this to indicate which wxDataViewItem representing the parent of item or an invalid wxDataViewItem if the root item is the parent item.
    ///
    /// See [C++ `wxDataViewModel::GetParent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a4e12a582dffdc40bf043a48e1c12a9fb).
    fn get_parent<D: DataViewItemMethods>(&self, item: &D) -> DataViewItem {
        unsafe {
            let item = item.as_ptr();
            DataViewItem::from_ptr(ffi::wxDataViewModel_GetParent(self.as_ptr(), item))
        }
    }
    /// Override this to indicate the value of item.
    ///
    /// See [C++ `wxDataViewModel::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a74d9f0ac2dd9935b7132da11c008c67f).
    fn get_value<D: DataViewItemMethods>(&self, variant: *mut c_void, item: &D, col: c_uint) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_GetValue(self.as_ptr(), variant, item, col)
        }
    }
    /// Override this method to indicate if a container item merely acts as a headline (or for categorisation) or if it also acts a normal item with entries for further columns.
    ///
    /// See [C++ `wxDataViewModel::HasContainerColumns()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a1c8983afee8875226c2b11500f228ae4).
    fn has_container_columns<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_HasContainerColumns(self.as_ptr(), item)
        }
    }
    /// Override this to indicate that the model provides a default compare function that the control should use if no wxDataViewColumn has been chosen for sorting.
    ///
    /// See [C++ `wxDataViewModel::HasDefaultCompare()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a909b2f8ec6493dbb8dff5b22f857eda7).
    fn has_default_compare(&self) -> bool {
        unsafe { ffi::wxDataViewModel_HasDefaultCompare(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn HasValue()
    /// Override this to indicate if item is a container, i.e. if it can have child items.
    ///
    /// See [C++ `wxDataViewModel::IsContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a2c61a09270fdda6720966742f0e4f09c).
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_IsContainer(self.as_ptr(), item)
        }
    }
    /// Call this to inform the model that an item has been added to the data.
    ///
    /// See [C++ `wxDataViewModel::ItemAdded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a37f9bf080fb368c6e964dc03aee46e5c).
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
    ///
    /// See [C++ `wxDataViewModel::ItemChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#abd4be4a8981cceaab024e0f69111e2f2).
    fn item_changed<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_ItemChanged(self.as_ptr(), item)
        }
    }
    /// Call this to inform the model that an item has been deleted from the data.
    ///
    /// See [C++ `wxDataViewModel::ItemDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a8bdcc58bf0e606c473ecd06c93ff0812).
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
    ///
    /// See [C++ `wxDataViewModel::ItemsAdded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a23b737d08f3a2249e7780c4a97e4277d).
    fn items_added<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModel_ItemsAdded(self.as_ptr(), parent, items)
        }
    }
    /// Call this to inform the model that several items have changed.
    ///
    /// See [C++ `wxDataViewModel::ItemsChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a9a3c99b9200ed1a72990973df07f4b1d).
    fn items_changed(&self, items: *const c_void) -> bool {
        unsafe { ffi::wxDataViewModel_ItemsChanged(self.as_ptr(), items) }
    }
    /// Call this to inform the model that several items have been deleted.
    ///
    /// See [C++ `wxDataViewModel::ItemsDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#af472d2e95b0e062045785cab17e72383).
    fn items_deleted<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModel_ItemsDeleted(self.as_ptr(), parent, items)
        }
    }
    /// Remove the notifier from the list of notifiers.
    ///
    /// See [C++ `wxDataViewModel::RemoveNotifier()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#ae0ce5ec556a84b6fd8612d36b0078d63).
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
    ///
    /// See [C++ `wxDataViewModel::Resort()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a2c2c525bd1617b19f34bc286a638b293).
    fn resort(&self) {
        unsafe { ffi::wxDataViewModel_Resort(self.as_ptr()) }
    }
    /// This gets called in order to set a value in the data model.
    ///
    /// See [C++ `wxDataViewModel::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a136dbef49beb09df1ffe5aa884a9c022).
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
    ///
    /// See [C++ `wxDataViewModel::ValueChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#afc162ff0230dd026ef99591696cbfb86).
    fn value_changed<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModel_ValueChanged(self.as_ptr(), item, col)
        }
    }
    ///
    /// See [C++ `wxDataViewModel::IsListModel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a550180cf1a3b529b025c98e0c16b9d85).
    fn is_list_model(&self) -> bool {
        unsafe { ffi::wxDataViewModel_IsListModel(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxDataViewModel::IsVirtualListModel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model.html#a503f49f7ef8767c208068c320c68783c).
    fn is_virtual_list_model(&self) -> bool {
        unsafe { ffi::wxDataViewModel_IsVirtualListModel(self.as_ptr()) }
    }
}

// wxDataViewModelNotifier
/// This trait represents [C++ `wxDataViewModelNotifier` class](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html)'s methods and inheritance.
///
/// See [`DataViewModelNotifierIsOwned`] documentation for the class usage.
pub trait DataViewModelNotifierMethods: WxRustMethods {
    // DTOR: fn ~wxDataViewModelNotifier()
    /// Called by owning model.
    ///
    /// See [C++ `wxDataViewModelNotifier::Cleared()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a81697c51b7b171647fcd80c1f794b7db).
    fn cleared(&self) -> bool {
        unsafe { ffi::wxDataViewModelNotifier_Cleared(self.as_ptr()) }
    }
    /// Get owning wxDataViewModel.
    ///
    /// See [C++ `wxDataViewModelNotifier::GetOwner()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#acc8ea9240f76c6c1712ca3d7ecfdcce4).
    fn get_owner(&self) -> Option<DataViewModelIsOwned<false>> {
        unsafe { DataViewModel::option_from(ffi::wxDataViewModelNotifier_GetOwner(self.as_ptr())) }
    }
    /// Called by owning model.
    ///
    /// See [C++ `wxDataViewModelNotifier::ItemAdded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a23924f37cb91eaa2949434c9988135ef).
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
    ///
    /// See [C++ `wxDataViewModelNotifier::ItemChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a96c28610f35beea1574e038c6322169c).
    fn item_changed<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ItemChanged(self.as_ptr(), item)
        }
    }
    /// Called by owning model.
    ///
    /// See [C++ `wxDataViewModelNotifier::ItemDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a0057d5c14f51ae17eb46d4b13f4a9830).
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
    ///
    /// See [C++ `wxDataViewModelNotifier::ItemsAdded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a1ffdebcc6be83c5bfb4d481b34cc7163).
    fn items_added<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModelNotifier_ItemsAdded(self.as_ptr(), parent, items)
        }
    }
    /// Called by owning model.
    ///
    /// See [C++ `wxDataViewModelNotifier::ItemsChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#aabab441910a2a4cc2932f74745f13f2f).
    fn items_changed(&self, items: *const c_void) -> bool {
        unsafe { ffi::wxDataViewModelNotifier_ItemsChanged(self.as_ptr(), items) }
    }
    /// Called by owning model.
    ///
    /// See [C++ `wxDataViewModelNotifier::ItemsDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#af9d314f5232bb943387d0dd6965504cd).
    fn items_deleted<D: DataViewItemMethods>(&self, parent: &D, items: *const c_void) -> bool {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewModelNotifier_ItemsDeleted(self.as_ptr(), parent, items)
        }
    }
    /// Called by owning model.
    ///
    /// See [C++ `wxDataViewModelNotifier::Resort()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a6382d03fe644df7af6e9b83698ae5423).
    fn resort(&self) {
        unsafe { ffi::wxDataViewModelNotifier_Resort(self.as_ptr()) }
    }
    /// Set owner of this notifier.
    ///
    /// See [C++ `wxDataViewModelNotifier::SetOwner()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a5df378b4f9b65381095ecf78b7872e9a).
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
    ///
    /// See [C++ `wxDataViewModelNotifier::ValueChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_model_notifier.html#a0c99e0ae9e1becff0cdedc0a8f3374c4).
    fn value_changed<D: DataViewItemMethods>(&self, item: &D, col: c_uint) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewModelNotifier_ValueChanged(self.as_ptr(), item, col)
        }
    }
}

// wxDataViewProgressRenderer
/// This trait represents [C++ `wxDataViewProgressRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_progress_renderer.html)'s methods and inheritance.
///
/// See [`DataViewProgressRendererIsOwned`] documentation for the class usage.
pub trait DataViewProgressRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewProgressRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_progress_renderer.html#ae4f67739a833aabba99612bda3c6c9b7).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewProgressRenderer_GetDefaultType()).into() }
    }
}

// wxDataViewRenderer
/// This trait represents [C++ `wxDataViewRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html)'s methods and inheritance.
///
/// See [`DataViewRendererIsOwned`] documentation for the class usage.
pub trait DataViewRendererMethods: ObjectMethods {
    /// Enable or disable replacing parts of the item text with ellipsis to make it fit the column width.
    ///
    /// See [C++ `wxDataViewRenderer::EnableEllipsize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#aeea4804c641149896b8e213bba672e64).
    fn enable_ellipsize(&self, mode: c_int) {
        unsafe { ffi::wxDataViewRenderer_EnableEllipsize(self.as_ptr(), mode) }
    }
    /// Disable replacing parts of the item text with ellipsis.
    ///
    /// See [C++ `wxDataViewRenderer::DisableEllipsize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#ad88c4e7dcf842d1ac56544ca85e72f9a).
    fn disable_ellipsize(&self) {
        unsafe { ffi::wxDataViewRenderer_DisableEllipsize(self.as_ptr()) }
    }
    // BLOCKED: fn GetAccessibleDescription()
    /// Returns the alignment.
    ///
    /// See [C++ `wxDataViewRenderer::GetAlignment()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a0137573f0ab266e4b05fe8237f830870).
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxDataViewRenderer_GetAlignment(self.as_ptr()) }
    }
    /// Returns the ellipsize mode used by the renderer.
    ///
    /// See [C++ `wxDataViewRenderer::GetEllipsizeMode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a452740b38d2d9ca5eb2fdb84ff50526f).
    fn get_ellipsize_mode(&self) -> c_int {
        unsafe { ffi::wxDataViewRenderer_GetEllipsizeMode(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetMode()
    /// Returns pointer to the owning wxDataViewColumn.
    ///
    /// See [C++ `wxDataViewRenderer::GetOwner()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#acb902597ff5a2023654a0ac1c045a381).
    fn get_owner(&self) -> Option<DataViewColumnIsOwned<false>> {
        unsafe { DataViewColumn::option_from(ffi::wxDataViewRenderer_GetOwner(self.as_ptr())) }
    }
    /// This methods retrieves the value from the renderer in order to transfer the value back to the data model.
    ///
    /// See [C++ `wxDataViewRenderer::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#ad7c52082d76074cae4ceaf94e55cf604).
    fn get_value(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_GetValue(self.as_ptr(), value) }
    }
    /// Returns a string with the type of the wxVariant supported by this renderer.
    ///
    /// See [C++ `wxDataViewRenderer::GetVariantType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#aaa95f8d49da1b296df8bedd1f76ecc28).
    fn get_variant_type(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewRenderer_GetVariantType(self.as_ptr())).into() }
    }
    /// Check if the given variant type is compatible with the type expected by this renderer.
    ///
    /// See [C++ `wxDataViewRenderer::IsCompatibleVariantType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a04ec07fe8f62bb2366ccbd7d1b6d2da8).
    fn is_compatible_variant_type(&self, variant_type: &str) -> bool {
        unsafe {
            let variant_type = WxString::from(variant_type);
            let variant_type = variant_type.as_ptr();
            ffi::wxDataViewRenderer_IsCompatibleVariantType(self.as_ptr(), variant_type)
        }
    }
    /// Sets the alignment of the renderer's content.
    ///
    /// See [C++ `wxDataViewRenderer::SetAlignment()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a2c67ceb437b6c2d2280ca651712dec1c).
    fn set_alignment(&self, align: c_int) {
        unsafe { ffi::wxDataViewRenderer_SetAlignment(self.as_ptr(), align) }
    }
    /// Sets the owning wxDataViewColumn.
    ///
    /// See [C++ `wxDataViewRenderer::SetOwner()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a388da7afbbf86971c1e945255344666c).
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
    ///
    /// See [C++ `wxDataViewRenderer::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#ac4494f39d056c1b0976481647b24117f).
    fn set_value(&self, value: *const c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_SetValue(self.as_ptr(), value) }
    }
    /// Set the transformer object to be used to customize values before they are rendered.
    ///
    /// See [C++ `wxDataViewRenderer::SetValueAdjuster()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a6e7ed0fa9820b5f9f0a97b8cd82677a0).
    fn set_value_adjuster(&self, transformer: *mut c_void) {
        unsafe { ffi::wxDataViewRenderer_SetValueAdjuster(self.as_ptr(), transformer) }
    }
    /// Before data is committed to the data model, it is passed to this method where it can be checked for validity.
    ///
    /// See [C++ `wxDataViewRenderer::Validate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a64c9df3410623498956ce5ed7524a9cf).
    fn validate(&self, value: *mut c_void) -> bool {
        unsafe { ffi::wxDataViewRenderer_Validate(self.as_ptr(), value) }
    }
    ///
    /// See [C++ `wxDataViewRenderer::HasEditorCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a71ba6f4bc0c54b83540660cdf40a64d0).
    fn has_editor_ctrl(&self) -> bool {
        unsafe { ffi::wxDataViewRenderer_HasEditorCtrl(self.as_ptr()) }
    }
    // BLOCKED: fn CreateEditorCtrl()
    ///
    /// See [C++ `wxDataViewRenderer::GetValueFromEditorCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a6d35e275733f4da63414bf98855278f7).
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
    ///
    /// See [C++ `wxDataViewRenderer::CancelEditing()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#a179780588351469d8c21a66bc2bc2b3d).
    fn cancel_editing(&self) {
        unsafe { ffi::wxDataViewRenderer_CancelEditing(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxDataViewRenderer::FinishEditing()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#adad0f7c140a4ad6fc2c0935435631beb).
    fn finish_editing(&self) -> bool {
        unsafe { ffi::wxDataViewRenderer_FinishEditing(self.as_ptr()) }
    }
    ///
    /// See [C++ `wxDataViewRenderer::GetEditorCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_renderer.html#aecc01bf475dd5607bef9a9e1ef137ac5).
    fn get_editor_ctrl(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDataViewRenderer_GetEditorCtrl(self.as_ptr())) }
    }
}

// wxDataViewSpinRenderer
/// This trait represents [C++ `wxDataViewSpinRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_spin_renderer.html)'s methods and inheritance.
///
/// See [`DataViewSpinRendererIsOwned`] documentation for the class usage.
pub trait DataViewSpinRendererMethods: DataViewCustomRendererMethods {}

// wxDataViewTextRenderer
/// This trait represents [C++ `wxDataViewTextRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_text_renderer.html)'s methods and inheritance.
///
/// See [`DataViewTextRendererIsOwned`] documentation for the class usage.
pub trait DataViewTextRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewTextRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_text_renderer.html#aa3f91a496b270197bfcf2a7631ff0e05).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewTextRenderer_GetDefaultType()).into() }
    }
    /// Enable interpretation of markup in the item data.
    ///
    /// See [C++ `wxDataViewTextRenderer::EnableMarkup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_text_renderer.html#a5be998c717630c1f33ccd07e18545a33).
    fn enable_markup(&self, enable: bool) {
        unsafe { ffi::wxDataViewTextRenderer_EnableMarkup(self.as_ptr(), enable) }
    }
}

// wxDataViewToggleRenderer
/// This trait represents [C++ `wxDataViewToggleRenderer` class](https://docs.wxwidgets.org/3.2/classwx_data_view_toggle_renderer.html)'s methods and inheritance.
///
/// See [`DataViewToggleRendererIsOwned`] documentation for the class usage.
pub trait DataViewToggleRendererMethods: DataViewRendererMethods {
    /// Returns the wxVariant type used with this renderer.
    ///
    /// See [C++ `wxDataViewToggleRenderer::GetDefaultType()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_toggle_renderer.html#aaace98054c513fb0e7cc230b1f371615).
    fn get_default_type() -> String {
        unsafe { WxString::from_ptr(ffi::wxDataViewToggleRenderer_GetDefaultType()).into() }
    }
    /// Switch to using radiobutton-like appearance instead of the default checkbox-like one.
    ///
    /// See [C++ `wxDataViewToggleRenderer::ShowAsRadio()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_toggle_renderer.html#af991786ed78d298c380f333cd086be39).
    fn show_as_radio(&self) {
        unsafe { ffi::wxDataViewToggleRenderer_ShowAsRadio(self.as_ptr()) }
    }
}

// wxDataViewTreeCtrl
/// This trait represents [C++ `wxDataViewTreeCtrl` class](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html)'s methods and inheritance.
///
/// See [`DataViewTreeCtrlIsOwned`] documentation for the class usage.
pub trait DataViewTreeCtrlMethods: DataViewCtrlMethods {
    // DTOR: fn ~wxDataViewTreeCtrl()
    /// Appends a container to the given parent.
    ///
    /// See [C++ `wxDataViewTreeCtrl::AppendContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a2e55315fc0380ab37933e4b3e01b2ef1).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::AppendItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a302a0d80b198c53724995c16668da0cc).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a15ca40c00d23cc0e7750d3a82425b1fc).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::DeleteAllItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a2c68581f90c9e5cf4f37f81309d06cde).
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeCtrl_DeleteAllItems(self.as_ptr()) }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::DeleteChildren()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aa4b452a0e55fbc02f74863dd30dfeb57).
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteChildren(self.as_ptr(), item)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::DeleteItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a7c35eae3fe177143c1dc9b6229ffc1cb).
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_DeleteItem(self.as_ptr(), item)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetChildCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aec531ff074e6084a480a9b7cd575abb8).
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeCtrl_GetChildCount(self.as_ptr(), parent)
        }
    }
    /// Returns the image list.
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a53db83152fced93887f5ad2cb1e40af4).
    fn get_image_list(&self) -> Option<ImageListIsOwned<false>> {
        unsafe { ImageList::option_from(ffi::wxDataViewTreeCtrl_GetImageList(self.as_ptr())) }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aa52f34ad4946bd2b4c0bf07c7e6d7521).
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            ClientData::option_from(ffi::wxDataViewTreeCtrl_GetItemData(self.as_ptr(), item))
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetItemExpandedIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aa8dcbdf791b3c17460355b9c7f391edf).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetItemIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aa2e441099fd868c0f41d582720603a49).
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeCtrl_GetItemIcon(self.as_ptr(), item))
        }
    }
    // BLOCKED: fn GetItemParent()
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetItemText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a650d79f326db7714716276f0a7089c32).
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeCtrl_GetItemText(self.as_ptr(), item)).into()
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetNthChild()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a06c608a005c9620774c310598791052c).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::GetStore()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a1500c5f73e443ceff35409fd27e0afcf).
    fn get_store(&self) -> Option<DataViewTreeStoreIsOwned<false>> {
        unsafe { DataViewTreeStore::option_from(ffi::wxDataViewTreeCtrl_GetStore1(self.as_ptr())) }
    }
    /// Calls the same method from wxDataViewTreeStore but uses an index position in the image list instead of a wxIcon.
    ///
    /// See [C++ `wxDataViewTreeCtrl::InsertContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a48f872c4f86d26f4978747fc3d416de0).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::InsertItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a0a77305d41b8bff9ff3289ac16fcde0b).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::IsContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aff737137e694f763a2e11477f5d1ecc6).
    fn is_container<D: DataViewItemMethods>(&self, item: &D) -> bool {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeCtrl_IsContainer(self.as_ptr(), item)
        }
    }
    /// Calls the same method from wxDataViewTreeStore but uses an index position in the image list instead of a wxIcon.
    ///
    /// See [C++ `wxDataViewTreeCtrl::PrependContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#aed76a400e693937090e42c250cb15c77).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::PrependItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#adac503bdd81957a729dcb364b568eb99).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::SetImageList()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a16ce4531cb31f801b2abd0d83c776df9).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::SetItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a4915dfeb26d83a281b622d706502680c).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::SetItemExpandedIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a756366a38cf0a7cf0c7b384df2b9d7e4).
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
    ///
    /// See [C++ `wxDataViewTreeCtrl::SetItemIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a24eb0b01061268c328d458d2de491d5d).
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeCtrl_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
    /// Calls the identical method from wxDataViewTreeStore.
    ///
    /// See [C++ `wxDataViewTreeCtrl::SetItemText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_ctrl.html#a5b94dc1ea2a5e963e10f49921f3679d0).
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
/// This trait represents [C++ `wxDataViewTreeStore` class](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html)'s methods and inheritance.
///
/// See [`DataViewTreeStoreIsOwned`] documentation for the class usage.
pub trait DataViewTreeStoreMethods: DataViewModelMethods {
    // DTOR: fn ~wxDataViewTreeStore()
    /// Append a container.
    ///
    /// See [C++ `wxDataViewTreeStore::AppendContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#abd7927f3a4a95e75454ce5064bece578).
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
    ///
    /// See [C++ `wxDataViewTreeStore::AppendItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#ac654e81ad1e70060c1a53a267159cbc5).
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
    ///
    /// See [C++ `wxDataViewTreeStore::DeleteAllItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a78222decc7d85840e1e880b07ee55fb4).
    fn delete_all_items(&self) {
        unsafe { ffi::wxDataViewTreeStore_DeleteAllItems(self.as_ptr()) }
    }
    /// Delete all children of the item, but not the item itself.
    ///
    /// See [C++ `wxDataViewTreeStore::DeleteChildren()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a5a9b7add7ce1a12bf8b8ef4b4723e598).
    fn delete_children<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_DeleteChildren(self.as_ptr(), item)
        }
    }
    /// Delete this item.
    ///
    /// See [C++ `wxDataViewTreeStore::DeleteItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#ac241f1385f4ff8f9cb5675aa3afdd193).
    fn delete_item<D: DataViewItemMethods>(&self, item: &D) {
        unsafe {
            let item = item.as_ptr();
            ffi::wxDataViewTreeStore_DeleteItem(self.as_ptr(), item)
        }
    }
    /// Return the number of children of item.
    ///
    /// See [C++ `wxDataViewTreeStore::GetChildCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a5a530523556b5e092649329181fe5d1c).
    fn get_child_count<D: DataViewItemMethods>(&self, parent: &D) -> c_int {
        unsafe {
            let parent = parent.as_ptr();
            ffi::wxDataViewTreeStore_GetChildCount(self.as_ptr(), parent)
        }
    }
    /// Returns the client data associated with the item.
    ///
    /// See [C++ `wxDataViewTreeStore::GetItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a299e1db9b0e7c9f5692b75c0f9cd3d29).
    fn get_item_data<D: DataViewItemMethods>(&self, item: &D) -> Option<ClientDataIsOwned<false>> {
        unsafe {
            let item = item.as_ptr();
            ClientData::option_from(ffi::wxDataViewTreeStore_GetItemData(self.as_ptr(), item))
        }
    }
    /// Returns the icon to display in expanded containers.
    ///
    /// See [C++ `wxDataViewTreeStore::GetItemExpandedIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a2f80cce35704cbfd1a2a8a7f14bd70e7).
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
    ///
    /// See [C++ `wxDataViewTreeStore::GetItemIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a657fe5dc836b35de270dcc67b75dcb0c).
    fn get_item_icon<D: DataViewItemMethods>(&self, item: &D) -> Icon {
        unsafe {
            let item = item.as_ptr();
            Icon::from_ptr(ffi::wxDataViewTreeStore_GetItemIcon(self.as_ptr(), item))
        }
    }
    /// Returns the text of the item.
    ///
    /// See [C++ `wxDataViewTreeStore::GetItemText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#aa49ab83077b041ec1ad1902dcd89faef).
    fn get_item_text<D: DataViewItemMethods>(&self, item: &D) -> String {
        unsafe {
            let item = item.as_ptr();
            WxString::from_ptr(ffi::wxDataViewTreeStore_GetItemText(self.as_ptr(), item)).into()
        }
    }
    /// Returns the nth child item of item.
    ///
    /// See [C++ `wxDataViewTreeStore::GetNthChild()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a26e76606e8b58070f7b8147fc12980fd).
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
    ///
    /// See [C++ `wxDataViewTreeStore::InsertContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a21b099314eddfefbadda4e91bc5d867e).
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
    ///
    /// See [C++ `wxDataViewTreeStore::InsertItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#ad89cf62a96756f99cfc5fdf3cce64a3b).
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
    ///
    /// See [C++ `wxDataViewTreeStore::PrependContainer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a016ba287df6c8e05bfb44aa0fcfe25ff).
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
    ///
    /// See [C++ `wxDataViewTreeStore::PrependItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a658535fea8fc2801c1362a747fe2c29a).
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
    ///
    /// See [C++ `wxDataViewTreeStore::SetItemData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#aabe65f7e0a908d616d13f459af8b1fdb).
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
    ///
    /// See [C++ `wxDataViewTreeStore::SetItemExpandedIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a88f8e3223779327e63b9cfcbdfff3ecf).
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
    ///
    /// See [C++ `wxDataViewTreeStore::SetItemIcon()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_tree_store.html#a664dcceb1149a340b8f9760111ba9ca1).
    fn set_item_icon<D: DataViewItemMethods, B: BitmapBundleMethods>(&self, item: &D, icon: &B) {
        unsafe {
            let item = item.as_ptr();
            let icon = icon.as_ptr();
            ffi::wxDataViewTreeStore_SetItemIcon(self.as_ptr(), item, icon)
        }
    }
}

// wxDataViewVirtualListModel
/// This trait represents [C++ `wxDataViewVirtualListModel` class](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html)'s methods and inheritance.
///
/// See [`DataViewVirtualListModelIsOwned`] documentation for the class usage.
pub trait DataViewVirtualListModelMethods: DataViewListModelMethods {
    /// Returns the wxDataViewItem at the given row.
    ///
    /// See [C++ `wxDataViewVirtualListModel::GetItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#a55fd843c0c8d1c97e00c6b2218cde938).
    fn get_item(&self, row: c_uint) -> DataViewItem {
        unsafe {
            DataViewItem::from_ptr(ffi::wxDataViewVirtualListModel_GetItem(self.as_ptr(), row))
        }
    }
    /// Call this after if the data has to be read again from the model.
    ///
    /// See [C++ `wxDataViewVirtualListModel::Reset()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#ad72afc0963dc7af4f0206a8fc3ef0d86).
    fn reset(&self, new_size: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_Reset(self.as_ptr(), new_size) }
    }
    /// Call this after a row has been appended to the model.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowAppended()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#aef8086b37cf12b98afc0ccbb080bcd68).
    fn row_appended(&self) {
        unsafe { ffi::wxDataViewVirtualListModel_RowAppended(self.as_ptr()) }
    }
    /// Call this after a row has been changed.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#aa7bef2abcd4d53a555a4777618fbee4f).
    fn row_changed(&self, row: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowChanged(self.as_ptr(), row) }
    }
    /// Call this after a row has been deleted.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#abd903d6a270396b1101d4d43a7ae3a68).
    fn row_deleted(&self, row: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowDeleted(self.as_ptr(), row) }
    }
    /// Call this after a row has been inserted at the given position.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowInserted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#af2b7ca1c906c1d6a9228ddde09eb5b6c).
    fn row_inserted(&self, before: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowInserted(self.as_ptr(), before) }
    }
    /// Call this after a row has been prepended to the model.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowPrepended()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#a68b74bd2ad4765c72247af66078ec833).
    fn row_prepended(&self) {
        unsafe { ffi::wxDataViewVirtualListModel_RowPrepended(self.as_ptr()) }
    }
    /// Call this after a value has been changed.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowValueChanged()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#a1b9dc8b21ae2fcec9ccc052f42f7cd43).
    fn row_value_changed(&self, row: c_uint, col: c_uint) {
        unsafe { ffi::wxDataViewVirtualListModel_RowValueChanged(self.as_ptr(), row, col) }
    }
    /// Call this after rows have been deleted.
    ///
    /// See [C++ `wxDataViewVirtualListModel::RowsDeleted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_data_view_virtual_list_model.html#acbcd350f3ee4e771ec53cbb45151c6a1).
    fn rows_deleted<A: ArrayIntMethods>(&self, rows: &A) {
        unsafe {
            let rows = rows.as_ptr();
            ffi::wxDataViewVirtualListModel_RowsDeleted(self.as_ptr(), rows)
        }
    }
}

// wxDateEvent
/// This trait represents [C++ `wxDateEvent` class](https://docs.wxwidgets.org/3.2/classwx_date_event.html)'s methods and inheritance.
///
/// See [`DateEventIsOwned`] documentation for the class usage.
pub trait DateEventMethods: CommandEventMethods {
    /// Returns the date.
    ///
    /// See [C++ `wxDateEvent::GetDate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_event.html#a45124ab6292bfadef1be0c9a2ce1411f).
    fn get_date(&self) -> DateTimeIsOwned<false> {
        unsafe { DateTimeIsOwned::from_ptr(ffi::wxDateEvent_GetDate(self.as_ptr())) }
    }
    /// Sets the date carried by the event, normally only used by the library internally.
    ///
    /// See [C++ `wxDateEvent::SetDate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_event.html#a1ef1b62865b967e7b4c50f6e04658d92).
    fn set_date<D: DateTimeMethods>(&self, date: &D) {
        unsafe {
            let date = date.as_ptr();
            ffi::wxDateEvent_SetDate(self.as_ptr(), date)
        }
    }
}

// wxDatePickerCtrl
/// This trait represents [C++ `wxDatePickerCtrl` class](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html)'s methods and inheritance.
///
/// See [`DatePickerCtrlIsOwned`] documentation for the class usage.
pub trait DatePickerCtrlMethods: ControlMethods {
    /// Create the control window.
    ///
    /// See [C++ `wxDatePickerCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#acd8017cd43d31da4e60e7933c2d76be9).
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
    ///
    /// See [C++ `wxDatePickerCtrl::GetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a1d5dd727e359d0c675c7b36eccf11a7c).
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
    ///
    /// See [C++ `wxDatePickerCtrl::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a80517ceac52322eaab5cf2d83322e4d7).
    fn get_value(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDatePickerCtrl_GetValue(self.as_ptr())) }
    }
    /// Set the text to show when there is no valid value.
    ///
    /// See [C++ `wxDatePickerCtrl::SetNullText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a1278ac9acac05409de9f839c958e2685).
    fn set_null_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxDatePickerCtrl_SetNullText(self.as_ptr(), text)
        }
    }
    /// Sets the valid range for the date selection.
    ///
    /// See [C++ `wxDatePickerCtrl::SetRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a7a3bd4d450dbbed0c4f3c570e29ba223).
    fn set_range<D: DateTimeMethods, D2: DateTimeMethods>(&self, dt1: &D, dt2: &D2) {
        unsafe {
            let dt1 = dt1.as_ptr();
            let dt2 = dt2.as_ptr();
            ffi::wxDatePickerCtrl_SetRange(self.as_ptr(), dt1, dt2)
        }
    }
    /// Changes the current value of the control.
    ///
    /// See [C++ `wxDatePickerCtrl::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_picker_ctrl.html#a421e1db72e2e88c0613aab3d7438dc70).
    fn set_value<D: DateTimeMethods>(&self, dt: &D) {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDatePickerCtrl_SetValue(self.as_ptr(), dt)
        }
    }
}

// wxDelegateRendererNative
/// This trait represents [C++ `wxDelegateRendererNative` class](https://docs.wxwidgets.org/3.2/classwx_delegate_renderer_native.html)'s methods and inheritance.
///
/// See [`DelegateRendererNativeIsOwned`] documentation for the class usage.
pub trait DelegateRendererNativeMethods: RendererNativeMethods {}

// wxDialog
/// This trait represents [C++ `wxDialog` class](https://docs.wxwidgets.org/3.2/classwx_dialog.html)'s methods and inheritance.
///
/// See [`DialogIsOwned`] documentation for the class usage.
pub trait DialogMethods: TopLevelWindowMethods {
    // DTOR: fn ~wxDialog()
    /// Adds an identifier to be regarded as a main button for the non-scrolling area of a dialog.
    ///
    /// See [C++ `wxDialog::AddMainButtonId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#af3a75b9d24abe5c79d1bb5397c67b108).
    fn add_main_button_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_AddMainButtonId(self.as_ptr(), id) }
    }
    /// Returns true if this dialog can and should perform layout adaptation using DoLayoutAdaptation(), usually if the dialog is too large to fit on the display.
    ///
    /// See [C++ `wxDialog::CanDoLayoutAdaptation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a233dd280efa8ee93c61792e439904731).
    fn can_do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_CanDoLayoutAdaptation(self.as_ptr()) }
    }
    /// Creates a sizer with standard buttons.
    ///
    /// See [C++ `wxDialog::CreateButtonSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a2af344ab6bbbc9a3af3681ac0c2f5e71).
    fn create_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe { Sizer::option_from(ffi::wxDialog_CreateButtonSizer(self.as_ptr(), flags)) }
    }
    /// Creates a sizer with standard buttons using CreateButtonSizer() separated from the rest of the dialog contents by a horizontal wxStaticLine.
    ///
    /// See [C++ `wxDialog::CreateSeparatedButtonSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#ac93ff63b02d4fbc749614a36458a4a67).
    fn create_separated_button_sizer(&self, flags: c_long) -> Option<SizerIsOwned<false>> {
        unsafe {
            Sizer::option_from(ffi::wxDialog_CreateSeparatedButtonSizer(
                self.as_ptr(),
                flags,
            ))
        }
    }
    /// Returns the sizer containing the given one with a separating wxStaticLine if necessarily.
    ///
    /// See [C++ `wxDialog::CreateSeparatedSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a0a59de4221bb9a0e0b9d0145dae4680d).
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
    ///
    /// See [C++ `wxDialog::CreateStdDialogButtonSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a2426ee7b5209520bb80ff67128e50f75).
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
    ///
    /// See [C++ `wxDialog::CreateTextSizer()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a65c9eb80fa95a9d02c6f662d2c8d6a7f).
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
    ///
    /// See [C++ `wxDialog::DoLayoutAdaptation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a919613599a9e919913146e21d09b44c0).
    fn do_layout_adaptation(&self) -> bool {
        unsafe { ffi::wxDialog_DoLayoutAdaptation(self.as_ptr()) }
    }
    /// Ends a modal dialog, passing a value to be returned from the ShowModal() invocation.
    ///
    /// See [C++ `wxDialog::EndModal()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a89b6085b05b63e98001311fafcfb21f0).
    fn end_modal(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_EndModal(self.as_ptr(), ret_code) }
    }
    /// Gets the identifier of the button which works like standard OK button in this dialog.
    ///
    /// See [C++ `wxDialog::GetAffirmativeId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a945999cf36ea7e9b961d48ad98c186ab).
    fn get_affirmative_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetAffirmativeId(self.as_ptr()) }
    }
    /// Override this to return a window containing the main content of the dialog.
    ///
    /// See [C++ `wxDialog::GetContentWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a7f1d8cf4b7041fe90f2ce61c4360a234).
    fn get_content_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxDialog_GetContentWindow(self.as_ptr())) }
    }
    /// Gets the identifier of the button to map presses of ESC button to.
    ///
    /// See [C++ `wxDialog::GetEscapeId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a2caade4625d8c6b5b7ffba07393992a1).
    fn get_escape_id(&self) -> c_int {
        unsafe { ffi::wxDialog_GetEscapeId(self.as_ptr()) }
    }
    /// Returns true if the dialog has been adapted, usually by making it scrollable to work with a small display.
    ///
    /// See [C++ `wxDialog::GetLayoutAdaptationDone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a0057d755530176a6bdb5ff67773b3676).
    fn get_layout_adaptation_done(&self) -> bool {
        unsafe { ffi::wxDialog_GetLayoutAdaptationDone(self.as_ptr()) }
    }
    /// Gets a value representing the aggressiveness of search for buttons and sizers to be in the non-scrolling part of a layout-adapted dialog.
    ///
    /// See [C++ `wxDialog::GetLayoutAdaptationLevel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a5c4d4b0f9f2ecd17820927bbb1161753).
    fn get_layout_adaptation_level(&self) -> c_int {
        unsafe { ffi::wxDialog_GetLayoutAdaptationLevel(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetLayoutAdaptationMode()
    /// Returns an array of identifiers to be regarded as the main buttons for the non-scrolling area of a dialog.
    ///
    /// See [C++ `wxDialog::GetMainButtonIds()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#ade1e5cfe8bc5a7a00fa76598664cae8d).
    fn get_main_button_ids(&self) -> ArrayIntIsOwned<false> {
        unsafe { ArrayIntIsOwned::from_ptr(ffi::wxDialog_GetMainButtonIds(self.as_ptr())) }
    }
    /// Gets the return code for this window.
    ///
    /// See [C++ `wxDialog::GetReturnCode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a89e7634a365ee74021591125b24d46c6).
    fn get_return_code(&self) -> c_int {
        unsafe { ffi::wxDialog_GetReturnCode(self.as_ptr()) }
    }
    // BLOCKED: fn GetToolBar()
    /// Returns true if id is in the array of identifiers to be regarded as the main buttons for the non-scrolling area of a dialog.
    ///
    /// See [C++ `wxDialog::IsMainButtonId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a2bdeb74702b91afc2475aaf7cf84de4c).
    fn is_main_button_id(&self, id: c_int) -> bool {
        unsafe { ffi::wxDialog_IsMainButtonId(self.as_ptr(), id) }
    }
    /// Returns true if the dialog box is modal, false otherwise.
    ///
    /// See [C++ `wxDialog::IsModal()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a9e64ed2d263b5a78af884acc93b978b0).
    fn is_modal(&self) -> bool {
        unsafe { ffi::wxDialog_IsModal(self.as_ptr()) }
    }
    /// Sets the identifier to be used as OK button.
    ///
    /// See [C++ `wxDialog::SetAffirmativeId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a72ea7e269bedb2552bfeaccdbac07939).
    fn set_affirmative_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetAffirmativeId(self.as_ptr(), id) }
    }
    /// Sets the identifier of the button which should work like the standard "Cancel" button in this dialog.
    ///
    /// See [C++ `wxDialog::SetEscapeId()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a585869988e308f549128a6a065f387c6).
    fn set_escape_id(&self, id: c_int) {
        unsafe { ffi::wxDialog_SetEscapeId(self.as_ptr(), id) }
    }
    /// Marks the dialog as having been adapted, usually by making it scrollable to work with a small display.
    ///
    /// See [C++ `wxDialog::SetLayoutAdaptationDone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#ab8b02ae4d97e300b4d7bbce6cc6af221).
    fn set_layout_adaptation_done(&self, done: bool) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationDone(self.as_ptr(), done) }
    }
    /// Sets the aggressiveness of search for buttons and sizers to be in the non-scrolling part of a layout-adapted dialog.
    ///
    /// See [C++ `wxDialog::SetLayoutAdaptationLevel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a9eaef1625f225bc25f8c92449d123a67).
    fn set_layout_adaptation_level(&self, level: c_int) {
        unsafe { ffi::wxDialog_SetLayoutAdaptationLevel(self.as_ptr(), level) }
    }
    // NOT_SUPPORTED: fn SetLayoutAdaptationMode()
    /// Sets the return code for this window.
    ///
    /// See [C++ `wxDialog::SetReturnCode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a0d04ed85ac5cd271a61514d446340673).
    fn set_return_code(&self, ret_code: c_int) {
        unsafe { ffi::wxDialog_SetReturnCode(self.as_ptr(), ret_code) }
    }
    /// Shows an application-modal dialog.
    ///
    /// See [C++ `wxDialog::ShowModal()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a6e078c3d0653f75ad3c34a37c0b54637).
    fn show_modal(&self) -> c_int {
        unsafe { ffi::wxDialog_ShowModal(self.as_ptr()) }
    }
    /// Shows a dialog modal to the parent top level window only.
    ///
    /// See [C++ `wxDialog::ShowWindowModal()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a5c61636f657c0ae9503c4dfa534e073e).
    fn show_window_modal(&self) {
        unsafe { ffi::wxDialog_ShowWindowModal(self.as_ptr()) }
    }
    // BLOCKED: fn ShowWindowModalThenDo()
    /// A static function enabling or disabling layout adaptation for all dialogs.
    ///
    /// See [C++ `wxDialog::EnableLayoutAdaptation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a016cfd686f75bcfc5f7b6f127be74c7c).
    fn enable_layout_adaptation(enable: bool) {
        unsafe { ffi::wxDialog_EnableLayoutAdaptation(enable) }
    }
    /// A static function getting the current layout adapter object.
    ///
    /// See [C++ `wxDialog::GetLayoutAdapter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a57b2381829d95e3d65f61d7dc03079e5).
    fn get_layout_adapter() -> Option<DialogLayoutAdapterIsOwned<false>> {
        unsafe { DialogLayoutAdapter::option_from(ffi::wxDialog_GetLayoutAdapter()) }
    }
    /// A static function returning true if layout adaptation is enabled for all dialogs.
    ///
    /// See [C++ `wxDialog::IsLayoutAdaptationEnabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#ae74a22f52a5978f9a8a2b4853c60c376).
    fn is_layout_adaptation_enabled() -> bool {
        unsafe { ffi::wxDialog_IsLayoutAdaptationEnabled() }
    }
    /// A static function for setting the current layout adapter object, returning the old adapter.
    ///
    /// See [C++ `wxDialog::SetLayoutAdapter()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog.html#a321e92831be659bbb6ddafe075d855d7).
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
/// This trait represents [C++ `wxDialogLayoutAdapter` class](https://docs.wxwidgets.org/3.2/classwx_dialog_layout_adapter.html)'s methods and inheritance.
///
/// See [`DialogLayoutAdapterIsOwned`] documentation for the class usage.
pub trait DialogLayoutAdapterMethods: WxRustMethods {
    /// Override this to returns true if adaptation can and should be done.
    ///
    /// See [C++ `wxDialogLayoutAdapter::CanDoLayoutAdaptation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog_layout_adapter.html#a08173985cfdfca4a8b5bdc5f10b64bc0).
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
    ///
    /// See [C++ `wxDialogLayoutAdapter::DoLayoutAdaptation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dialog_layout_adapter.html#adb59c3238b594b88faa4aab892b9bdd8).
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
/// This trait represents [C++ `wxDirDialog` class](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html)'s methods and inheritance.
///
/// See [`DirDialogIsOwned`] documentation for the class usage.
pub trait DirDialogMethods: DialogMethods {
    // DTOR: fn ~wxDirDialog()
    /// Returns the message that will be displayed on the dialog.
    ///
    /// See [C++ `wxDirDialog::GetMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#a6348c887a2cabbe2b827c2d8999292ec).
    fn get_message(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirDialog_GetMessage(self.as_ptr())).into() }
    }
    /// Returns the default or user-selected path.
    ///
    /// See [C++ `wxDirDialog::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#a7e377564ba462dda119d6e291f087bd5).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirDialog_GetPath(self.as_ptr())).into() }
    }
    /// Fills the array paths with the full paths of the chosen directories.
    ///
    /// See [C++ `wxDirDialog::GetPaths()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#a22e767dfaf6244bbc385a4c37867745c).
    fn get_paths<A: ArrayStringMethods>(&self, paths: &A) {
        unsafe {
            let paths = paths.as_ptr();
            ffi::wxDirDialog_GetPaths(self.as_ptr(), paths)
        }
    }
    /// Sets the message that will be displayed on the dialog.
    ///
    /// See [C++ `wxDirDialog::SetMessage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#a24fce7078762cc20624568df366f6dcc).
    fn set_message(&self, message: &str) {
        unsafe {
            let message = WxString::from(message);
            let message = message.as_ptr();
            ffi::wxDirDialog_SetMessage(self.as_ptr(), message)
        }
    }
    /// Sets the default path.
    ///
    /// See [C++ `wxDirDialog::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_dialog.html#af16d4eceadfcb89ae79f7235dfccc488).
    fn set_path(&self, path: &str) {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxDirDialog_SetPath(self.as_ptr(), path)
        }
    }
}

// wxDirPickerCtrl
/// This trait represents [C++ `wxDirPickerCtrl` class](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html)'s methods and inheritance.
///
/// See [`DirPickerCtrlIsOwned`] documentation for the class usage.
pub trait DirPickerCtrlMethods: PickerBaseMethods {
    /// Creates the widgets with the given parameters.
    ///
    /// See [C++ `wxDirPickerCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a64d40890f6f77bb0b11e1b73b463c580).
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
    ///
    /// See [C++ `wxDirPickerCtrl::GetDirName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#ada862c3ddb42d1a371af78b9e05827c1).
    fn get_dir_name(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxDirPickerCtrl_GetDirName(self.as_ptr())) }
    }
    /// Returns the absolute path of the currently selected directory.
    ///
    /// See [C++ `wxDirPickerCtrl::GetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a17398ac139cdfd14a0b604b6dc2008b5).
    fn get_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDirPickerCtrl_GetPath(self.as_ptr())).into() }
    }
    /// Just like SetPath() but this function takes a wxFileName object.
    ///
    /// See [C++ `wxDirPickerCtrl::SetDirName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#ab9d54a573c9ab81a1967099543d5b267).
    fn set_dir_name<F: FileNameMethods>(&self, dirname: &F) {
        unsafe {
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetDirName(self.as_ptr(), dirname)
        }
    }
    /// Set the directory to show when starting to browse for directories.
    ///
    /// See [C++ `wxDirPickerCtrl::SetInitialDirectory()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a628dc1acb50a1df99335f0ceff2da4c7).
    fn set_initial_directory(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxDirPickerCtrl_SetInitialDirectory(self.as_ptr(), dir)
        }
    }
    /// Sets the absolute path of the currently selected directory.
    ///
    /// See [C++ `wxDirPickerCtrl::SetPath()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_dir_picker_ctrl.html#a0a861d00a0877c830bc6e77d6077b9a7).
    fn set_path(&self, dirname: &str) {
        unsafe {
            let dirname = WxString::from(dirname);
            let dirname = dirname.as_ptr();
            ffi::wxDirPickerCtrl_SetPath(self.as_ptr(), dirname)
        }
    }
}

// wxDisplay
/// This trait represents [C++ `wxDisplay` class](https://docs.wxwidgets.org/3.2/classwx_display.html)'s methods and inheritance.
///
/// See [`DisplayIsOwned`] documentation for the class usage.
pub trait DisplayMethods: WxRustMethods {
    // DTOR: fn ~wxDisplay()
    /// Changes the video mode of this display to the mode specified in the mode parameter.
    ///
    /// See [C++ `wxDisplay::ChangeMode()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a43f3eb82f45be488c3829b4d97693e18).
    fn change_mode(&self, mode: *const c_void) -> bool {
        unsafe { ffi::wxDisplay_ChangeMode(self.as_ptr(), mode) }
    }
    /// Returns the client area of the display.
    ///
    /// See [C++ `wxDisplay::GetClientArea()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a2964fc964a6badf63791bf3318eee0bf).
    fn get_client_area(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetClientArea(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetCurrentMode()
    /// Returns the bounding rectangle of the display whose index was passed to the constructor.
    ///
    /// See [C++ `wxDisplay::GetGeometry()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#ab60df0f4e854dda42b890916362b03f9).
    fn get_geometry(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxDisplay_GetGeometry(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetModes()
    /// Returns the display's name.
    ///
    /// See [C++ `wxDisplay::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a91e2b0cb473fcc4b58d53fcadcecb753).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDisplay_GetName(self.as_ptr())).into() }
    }
    /// Returns display resolution in pixels per inch.
    ///
    /// See [C++ `wxDisplay::GetPPI()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#ad086b5e0e9188e28b07f9acefef09695).
    fn get_ppi(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetPPI(self.as_ptr())) }
    }
    /// Returns scaling factor used by this display.
    ///
    /// See [C++ `wxDisplay::GetScaleFactor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a5df07db77453e7159136fd80361c15d2).
    fn get_scale_factor(&self) -> c_double {
        unsafe { ffi::wxDisplay_GetScaleFactor(self.as_ptr()) }
    }
    /// Returns true if the display is the primary display.
    ///
    /// See [C++ `wxDisplay::IsPrimary()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#ab4ffe57d03f5fb29e9b4adb43613033d).
    fn is_primary(&self) -> bool {
        unsafe { ffi::wxDisplay_IsPrimary(self.as_ptr()) }
    }
    /// Returns the number of connected displays.
    ///
    /// See [C++ `wxDisplay::GetCount()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a6493e584d40a07c5f789f3027d8eea1d).
    fn get_count() -> c_uint {
        unsafe { ffi::wxDisplay_GetCount() }
    }
    /// Returns the index of the display on which the given point lies, or wxNOT_FOUND if the point is not on any connected display.
    ///
    /// See [C++ `wxDisplay::GetFromPoint()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#ae3cebdb3bab01a12d3d3516af75d3728).
    fn get_from_point<P: PointMethods>(pt: &P) -> c_int {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDisplay_GetFromPoint(pt)
        }
    }
    /// Returns the index of the display on which the given window lies.
    ///
    /// See [C++ `wxDisplay::GetFromWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#aaf13ecd5a870d5b8b10a6c6cd9710b7a).
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
    ///
    /// See [C++ `wxDisplay::GetStdPPIValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a2d2cbd937759fd021cc4a80a81a54731).
    fn get_std_ppi_value() -> c_int {
        unsafe { ffi::wxDisplay_GetStdPPIValue() }
    }
    /// Returns default display resolution for the current platform as wxSize.
    ///
    /// See [C++ `wxDisplay::GetStdPPI()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_display.html#a63d5e95d09d137a02faa412a896a5e22).
    fn get_std_ppi() -> Size {
        unsafe { Size::from_ptr(ffi::wxDisplay_GetStdPPI()) }
    }
}

// wxDisplayChangedEvent
/// This trait represents [C++ `wxDisplayChangedEvent` class](https://docs.wxwidgets.org/3.2/classwx_display_changed_event.html)'s methods and inheritance.
///
/// See [`DisplayChangedEventIsOwned`] documentation for the class usage.
pub trait DisplayChangedEventMethods: EventMethods {}

// wxDragImage
/// This trait represents [C++ `wxDragImage` class](https://docs.wxwidgets.org/3.2/classwx_drag_image.html)'s methods and inheritance.
///
/// See [`DragImageIsOwned`] documentation for the class usage.
pub trait DragImageMethods: ObjectMethods {
    /// Start dragging the image, in a window or full screen.
    ///
    /// See [C++ `wxDragImage::BeginDrag()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a7af71929aec6994d903864ed95c7f7b6).
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
    ///
    /// See [C++ `wxDragImage::BeginDrag()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a351e1e0cdcd0ade8154aac60ae65484d).
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
    ///
    /// See [C++ `wxDragImage::DoDrawImage()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#ae97fbe2de0826ae4e1cd69ea22e5c3d9).
    fn do_draw_image<D: DCMethods, P: PointMethods>(&self, dc: &D, pos: &P) -> bool {
        unsafe {
            let dc = dc.as_ptr();
            let pos = pos.as_ptr();
            ffi::wxDragImage_DoDrawImage(self.as_ptr(), dc, pos)
        }
    }
    /// Call this when the drag has finished.
    ///
    /// See [C++ `wxDragImage::EndDrag()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a481b3dbce5200247a46623431a99df13).
    fn end_drag(&self) -> bool {
        unsafe { ffi::wxDragImage_EndDrag(self.as_ptr()) }
    }
    /// Returns the rectangle enclosing the image, assuming that the image is drawn with its top-left corner at the given point.
    ///
    /// See [C++ `wxDragImage::GetImageRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a1e239cc90d6d4756a804c46d1a9b022e).
    fn get_image_rect<P: PointMethods>(&self, pos: &P) -> Rect {
        unsafe {
            let pos = pos.as_ptr();
            Rect::from_ptr(ffi::wxDragImage_GetImageRect(self.as_ptr(), pos))
        }
    }
    /// Hides the image.
    ///
    /// See [C++ `wxDragImage::Hide()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#aeda0c97def022fc3cacbc879f053f93a).
    fn hide(&self) -> bool {
        unsafe { ffi::wxDragImage_Hide(self.as_ptr()) }
    }
    /// Call this to move the image to a new position.
    ///
    /// See [C++ `wxDragImage::Move()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a51b7a0377bbdd79ad8d93c7bb1a67644).
    fn move_<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxDragImage_Move(self.as_ptr(), pt)
        }
    }
    /// Shows the image.
    ///
    /// See [C++ `wxDragImage::Show()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#a53ba3036e7cf017d32a548e95d9072e3).
    fn show(&self) -> bool {
        unsafe { ffi::wxDragImage_Show(self.as_ptr()) }
    }
    /// Override this if you wish to draw the window contents to the backing bitmap yourself.
    ///
    /// See [C++ `wxDragImage::UpdateBackingFromWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drag_image.html#ab8538845d502724f1ae523701724fdbf).
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
/// This trait represents [C++ `wxDropFilesEvent` class](https://docs.wxwidgets.org/3.2/classwx_drop_files_event.html)'s methods and inheritance.
///
/// See [`DropFilesEventIsOwned`] documentation for the class usage.
pub trait DropFilesEventMethods: EventMethods {
    // BLOCKED: fn GetFiles()
    /// Returns the number of files dropped.
    ///
    /// See [C++ `wxDropFilesEvent::GetNumberOfFiles()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_files_event.html#a715e19151462c985df616fe993e7ddcb).
    fn get_number_of_files(&self) -> c_int {
        unsafe { ffi::wxDropFilesEvent_GetNumberOfFiles(self.as_ptr()) }
    }
    /// Returns the position at which the files were dropped.
    ///
    /// See [C++ `wxDropFilesEvent::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_files_event.html#a7c4a0590cc9eb5dffdf798a0f29bc71e).
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxDropFilesEvent_GetPosition(self.as_ptr())) }
    }
}

// wxDropSource
/// This trait represents [C++ `wxDropSource` class](https://docs.wxwidgets.org/3.2/classwx_drop_source.html)'s methods and inheritance.
///
/// See [`DropSourceIsOwned`] documentation for the class usage.
pub trait DropSourceMethods: WxRustMethods {
    // NOT_SUPPORTED: fn DoDragDrop()
    /// Returns the wxDataObject object that has been assigned previously.
    ///
    /// See [C++ `wxDropSource::GetDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#a9391804c155eafbcce8f24cd22491f17).
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDropSource_GetDataObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GiveFeedback()
    // NOT_SUPPORTED: fn SetCursor()
    // NOT_SUPPORTED: fn SetIcon()
    /// Sets the data wxDataObject associated with the drop source.
    ///
    /// See [C++ `wxDropSource::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_source.html#aa3a84a8bf1e6817439ff6eb6570e8a3b).
    fn set_data<D: DataObjectMethods>(&self, data: &D) {
        unsafe {
            let data = data.as_ptr();
            ffi::wxDropSource_SetData(self.as_ptr(), data)
        }
    }
}

// wxDropTarget
/// This trait represents [C++ `wxDropTarget` class](https://docs.wxwidgets.org/3.2/classwx_drop_target.html)'s methods and inheritance.
///
/// See [`DropTargetIsOwned`] documentation for the class usage.
pub trait DropTargetMethods: WxRustMethods {
    // DTOR: fn ~wxDropTarget()
    /// This method may only be called from within OnData().
    ///
    /// See [C++ `wxDropTarget::GetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html#a179adb5f161e0a8abc17e65f4470c51d).
    fn get_data(&self) -> bool {
        unsafe { ffi::wxDropTarget_GetData(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn OnData()
    // NOT_SUPPORTED: fn OnDragOver()
    /// Called when the user drops a data object on the target.
    ///
    /// See [C++ `wxDropTarget::OnDrop()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html#a3448cb9505b9dd3f064e9c5208d82ae1).
    fn on_drop(&self, x: c_int, y: c_int) -> bool {
        unsafe { ffi::wxDropTarget_OnDrop(self.as_ptr(), x, y) }
    }
    // NOT_SUPPORTED: fn OnEnter()
    /// Called when the mouse leaves the drop target.
    ///
    /// See [C++ `wxDropTarget::OnLeave()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html#a8627e82d3f3ee58a09d67fc51c265113).
    fn on_leave(&self) {
        unsafe { ffi::wxDropTarget_OnLeave(self.as_ptr()) }
    }
    /// Returns the data wxDataObject associated with the drop target.
    ///
    /// See [C++ `wxDropTarget::GetDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html#ab7921ce232715427dc2f064dc97ed3d6).
    fn get_data_object(&self) -> Option<DataObjectIsOwned<false>> {
        unsafe { DataObject::option_from(ffi::wxDropTarget_GetDataObject(self.as_ptr())) }
    }
    /// Sets the data wxDataObject associated with the drop target and deletes any previously associated data object.
    ///
    /// See [C++ `wxDropTarget::SetDataObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_drop_target.html#a06c2858ba321c2095fc220b82c238a2d).
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
