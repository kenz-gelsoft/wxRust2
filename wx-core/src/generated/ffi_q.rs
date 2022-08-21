use super::*;

extern "C" {

    // wxQuantize
    pub fn wxQuantize_CLASSINFO() -> *mut c_void;
    pub fn wxQuantize_new() -> *mut c_void;
    pub fn wxQuantize_DoQuantize(
        w: c_uint,
        h: c_uint,
        in_rows: *mut c_void,
        out_rows: *mut c_void,
        palette: *mut c_void,
        desired_no_colours: c_int,
    );
    pub fn wxQuantize_Quantize(
        src: *const c_void,
        dest: *mut c_void,
        p_palette: *mut c_void,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool;
    pub fn wxQuantize_Quantize1(
        src: *const c_void,
        dest: *mut c_void,
        desired_no_colours: c_int,
        eight_bit_data: *mut c_void,
        flags: c_int,
    ) -> bool;

    // wxQueryLayoutInfoEvent
    pub fn wxQueryLayoutInfoEvent_CLASSINFO() -> *mut c_void;
    pub fn wxQueryLayoutInfoEvent_new(id: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_GetAlignment(self_: *const c_void) -> wxLayoutAlignment;
    pub fn wxQueryLayoutInfoEvent_GetFlags(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_GetOrientation(self_: *const c_void) -> wxLayoutOrientation;
    pub fn wxQueryLayoutInfoEvent_GetRequestedLength(self_: *const c_void) -> c_int;
    pub fn wxQueryLayoutInfoEvent_GetSize(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_SetAlignment(self_: *mut c_void, alignment: wxLayoutAlignment);
    pub fn wxQueryLayoutInfoEvent_SetFlags(self_: *mut c_void, flags: c_int);
    // NOT_SUPPORTED: pub fn wxQueryLayoutInfoEvent_SetOrientation(self_: *mut c_void, orientation: wxLayoutOrientation);
    pub fn wxQueryLayoutInfoEvent_SetRequestedLength(self_: *mut c_void, length: c_int);
    pub fn wxQueryLayoutInfoEvent_SetSize(self_: *mut c_void, size: *const c_void);

}
