use super::*;

// wxQuantize
pub trait QuantizeMethods: ObjectMethods {
    /// Converts input bitmap(s) into 8bit representation with custom palette.
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
pub trait QueryLayoutInfoEventMethods: EventMethods {
    // NOT_SUPPORTED: fn GetAlignment()
    /// Returns the flags associated with this event.
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetOrientation()
    /// Returns the requested length of the window in the direction of the window orientation.
    fn get_requested_length(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetRequestedLength(self.as_ptr()) }
    }
    /// Returns the size that the event handler specified to the event object as being the requested size of the window.
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxQueryLayoutInfoEvent_GetSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    /// Sets the flags associated with this event.
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SetOrientation()
    /// Sets the requested length of the window in the direction of the window orientation.
    fn set_requested_length(&self, length: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetRequestedLength(self.as_ptr(), length) }
    }
    /// Call this to let the calling code know what the size of the window is.
    fn set_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxQueryLayoutInfoEvent_SetSize(self.as_ptr(), size)
        }
    }
}
