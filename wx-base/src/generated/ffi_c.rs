use super::*;

extern "C" {

    // wxClassInfo
    pub fn wxClassInfo_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxClassInfo_new(class_name: *const c_void, base_class1: *const c_void, base_class2: *const c_void, size: c_int, fn: wxObjectConstructorFn) -> *mut c_void;
    pub fn wxClassInfo_CreateObject(self_: *const c_void) -> *mut c_void;
    pub fn wxClassInfo_GetBaseClassName1(self_: *const c_void) -> *const c_void;
    pub fn wxClassInfo_GetBaseClassName2(self_: *const c_void) -> *const c_void;
    pub fn wxClassInfo_GetClassName(self_: *const c_void) -> *const c_void;
    pub fn wxClassInfo_GetSize(self_: *const c_void) -> c_int;
    pub fn wxClassInfo_IsDynamic(self_: *const c_void) -> bool;
    pub fn wxClassInfo_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
    pub fn wxClassInfo_FindClass(class_name: *const c_void) -> *mut c_void;

    // wxClientData
    pub fn wxClientData_delete(self_: *mut c_void);
    pub fn wxClientData_new() -> *mut c_void;
    // DTOR: pub fn wxClientData_~wxClientData(self_: *mut c_void);

}
