use super::*;

extern "C" {

    // wxXPMHandler
    pub fn wxXPMHandler_CLASSINFO() -> *mut c_void;
    pub fn wxXPMHandler_new() -> *mut c_void;

}
