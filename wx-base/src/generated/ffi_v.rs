use super::*;

extern "C" {

    // wxVariantData
    pub fn wxVariantData_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxVariantData_new() -> *mut c_void;
    pub fn wxVariantData_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxVariantData_DecRef(self_: *mut c_void);
    pub fn wxVariantData_Eq(self_: *const c_void, data: *mut c_void) -> bool;
    // BLOCKED: pub fn wxVariantData_GetAny(self_: *const c_void, any: *mut c_void) -> bool;
    pub fn wxVariantData_GetType(self_: *const c_void) -> *mut c_void;
    pub fn wxVariantData_GetValueClassInfo(self_: *mut c_void) -> *mut c_void;
    pub fn wxVariantData_IncRef(self_: *mut c_void);
    // BLOCKED: pub fn wxVariantData_Read(self_: *mut c_void, stream: *mut c_void) -> bool;
    pub fn wxVariantData_Read1(self_: *mut c_void, string: *mut c_void) -> bool;
    // BLOCKED: pub fn wxVariantData_Write(self_: *const c_void, stream: *mut c_void) -> bool;
    pub fn wxVariantData_Write1(self_: *const c_void, string: *mut c_void) -> bool;

}
