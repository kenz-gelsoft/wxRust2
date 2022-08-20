use super::*;

extern "C" {

    // wxMessageOutput
    pub fn wxMessageOutput_delete(self_: *mut c_void);
    pub fn wxMessageOutput_Get() -> *mut c_void;
    pub fn wxMessageOutput_Set(msgout: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMessageOutput_Printf(self_: *mut c_void, format: *const c_void, None: ...);
    pub fn wxMessageOutput_Output(self_: *mut c_void, str: *const c_void);

}
