use super::*;

extern "C" {

    // wxValidator
    pub fn wxValidator_CLASSINFO() -> *mut c_void;
    pub fn wxValidator_new() -> *mut c_void;
    // DTOR: pub fn wxValidator_~wxValidator(self_: *mut c_void);
    pub fn wxValidator_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_SetWindow(self_: *mut c_void, window: *mut c_void);
    pub fn wxValidator_TransferFromWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_TransferToWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_Validate(self_: *mut c_void, parent: *mut c_void) -> bool;
    pub fn wxValidator_SuppressBellOnError(suppress: bool);
    pub fn wxValidator_IsSilent() -> bool;

}
