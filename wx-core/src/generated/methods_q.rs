use super::*;

// wxQuantize
/// This trait represents [C++ `wxQuantize` class](https://docs.wxwidgets.org/3.2/classwx_quantize.html)'s methods and inheritance.
///
/// See [`QuantizeIsOwned`] documentation for the class usage.
pub trait QuantizeMethods: ObjectMethods {
    /// Converts input bitmap(s) into 8bit representation with custom palette.
    ///
    /// See [C++ `wxQuantize::DoQuantize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html#af08475e1608d344b597799d1c575b13c).
    fn do_quantize(
        w: c_uint,
        h: c_uint,
        in_rows: *mut c_void,
        out_rows: *mut c_void,
        palette: *mut c_void,
        desired_no_colours: c_int,
    ) {
        unsafe { ffi::wxQuantize_DoQuantize(w, h, in_rows, out_rows, palette, desired_no_colours) }
    }
    /// Reduce the colours in the source image and put the result into the destination image.
    ///
    /// See [C++ `wxQuantize::Quantize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html#a1c1c9e1fef4fff61f0aafc8dd3e6c65c).
    fn quantize_palette<I: ImageMethods, I2: ImageMethods, P: PaletteMethods>(
        src: &I,
        dest: &I2,
        p_palette: Option<&P>,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool {
        unsafe {
            let src = src.as_ptr();
            let dest = dest.as_ptr();
            let p_palette = match p_palette {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxQuantize_Quantize(
                src,
                dest,
                p_palette,
                desired_no_colours,
                eight_bit_data,
                flags,
            )
        }
    }
    /// This version sets a palette in the destination image so you don't have to manage it yourself.
    ///
    /// See [C++ `wxQuantize::Quantize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_quantize.html#aa99e73bbc0f939bb4d7f40ca761ce31f).
    fn quantize_int<I: ImageMethods, I2: ImageMethods>(
        src: &I,
        dest: &I2,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool {
        unsafe {
            let src = src.as_ptr();
            let dest = dest.as_ptr();
            ffi::wxQuantize_Quantize1(src, dest, desired_no_colours, eight_bit_data, flags)
        }
    }
}

// wxQueryLayoutInfoEvent
/// This trait represents [C++ `wxQueryLayoutInfoEvent` class](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html)'s methods and inheritance.
///
/// See [`QueryLayoutInfoEventIsOwned`] documentation for the class usage.
pub trait QueryLayoutInfoEventMethods: EventMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    /// Returns the flags associated with this event.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::GetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#a5473a15165e30136dbd3f37df3c56214).
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetOrientation()
    /// Returns the requested length of the window in the direction of the window orientation.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::GetRequestedLength()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#ad21f4fce3a1843e868bf6dabf200e491).
    fn get_requested_length(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetRequestedLength(self.as_ptr()) }
    }
    /// Returns the size that the event handler specified to the event object as being the requested size of the window.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#a6350973483b59a95f172f64926936c36).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxQueryLayoutInfoEvent_GetSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    /// Sets the flags associated with this event.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::SetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#a5cefb50301df556df7841cd33be945c7).
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SetOrientation()
    /// Sets the requested length of the window in the direction of the window orientation.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::SetRequestedLength()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#ad844a6dfbe69cf48303823d2573810cf).
    fn set_requested_length(&self, length: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetRequestedLength(self.as_ptr(), length) }
    }
    /// Call this to let the calling code know what the size of the window is.
    ///
    /// See [C++ `wxQueryLayoutInfoEvent::SetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_query_layout_info_event.html#a47ec467f4476fb22fccebb2ad7365bb4).
    fn set_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxQueryLayoutInfoEvent_SetSize(self.as_ptr(), size)
        }
    }
}
