use super::*;

extern "C" {

    // wxRefCounter
    pub fn wxRefCounter_delete(self_: *mut c_void);
    pub fn wxRefCounter_new() -> *mut c_void;
    pub fn wxRefCounter_DecRef(self_: *mut c_void);
    pub fn wxRefCounter_GetRefCount(self_: *const c_void) -> c_int;
    pub fn wxRefCounter_IncRef(self_: *mut c_void);

}
