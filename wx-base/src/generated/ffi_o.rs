use super::*;

extern "C" {

    // wxObject
    pub fn wxObject_CLASSINFO() -> *mut c_void;
    pub fn wxObject_new() -> *mut c_void;
    pub fn wxObject_new1(other: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxObject_~wxObject(self_: *mut c_void);
    pub fn wxObject_GetClassInfo(self_: *const c_void) -> *mut c_void;
    pub fn wxObject_GetRefData(self_: *const c_void) -> *mut c_void;
    pub fn wxObject_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
    pub fn wxObject_IsSameAs(self_: *const c_void, obj: *const c_void) -> bool;
    pub fn wxObject_Ref(self_: *mut c_void, clone: *const c_void);
    pub fn wxObject_SetRefData(self_: *mut c_void, data: *mut c_void);
    pub fn wxObject_UnRef(self_: *mut c_void);
    pub fn wxObject_UnShare(self_: *mut c_void);
    // BLOCKED: pub fn wxObject_operator delete(self_: *mut c_void, buf: *mut c_void);
    // BLOCKED: pub fn wxObject_operator new(self_: *mut c_void, size: usize, filename: *const c_void, line_num: c_int) -> *mut c_void;

    // wxObjectRefData
    pub fn wxObjectRefData_delete(self_: *mut c_void);

}
