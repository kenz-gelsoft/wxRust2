use super::*;

// wxQuantize
pub trait QuantizeMethods: ObjectMethods {
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
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetFlags(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetOrientation()
    fn get_requested_length(&self) -> c_int {
        unsafe { ffi::wxQueryLayoutInfoEvent_GetRequestedLength(self.as_ptr()) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxQueryLayoutInfoEvent_GetSize(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn SetAlignment()
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetFlags(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SetOrientation()
    fn set_requested_length(&self, length: c_int) {
        unsafe { ffi::wxQueryLayoutInfoEvent_SetRequestedLength(self.as_ptr(), length) }
    }
    fn set_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxQueryLayoutInfoEvent_SetSize(self.as_ptr(), size)
        }
    }
}
