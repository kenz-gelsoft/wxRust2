use super::*;

extern "C" {

    // wxLog
    pub fn wxLog_delete(self_: *mut c_void);
    pub fn wxLog_AddTraceMask(mask: *const c_void);
    pub fn wxLog_ClearTraceMasks();
    pub fn wxLog_GetTraceMasks() -> *mut c_void;
    pub fn wxLog_IsAllowedTraceMask(mask: *const c_void) -> bool;
    pub fn wxLog_RemoveTraceMask(mask: *const c_void);
    pub fn wxLog_DontCreateOnDemand();
    pub fn wxLog_GetActiveTarget() -> *mut c_void;
    pub fn wxLog_SetActiveTarget(logtarget: *mut c_void) -> *mut c_void;
    pub fn wxLog_SetThreadActiveTarget(logger: *mut c_void) -> *mut c_void;
    pub fn wxLog_FlushActive();
    pub fn wxLog_Resume();
    pub fn wxLog_Suspend();
    // NOT_SUPPORTED: pub fn wxLog_GetLogLevel() -> wxLogLevel;
    // NOT_SUPPORTED: pub fn wxLog_IsLevelEnabled(level: wxLogLevel, component: wxString) -> bool;
    // NOT_SUPPORTED: pub fn wxLog_SetComponentLevel(component: *const c_void, level: wxLogLevel);
    // NOT_SUPPORTED: pub fn wxLog_SetLogLevel(log_level: wxLogLevel);
    pub fn wxLog_EnableLogging(enable: bool) -> bool;
    pub fn wxLog_IsEnabled() -> bool;
    pub fn wxLog_GetRepetitionCounting() -> bool;
    pub fn wxLog_SetRepetitionCounting(repet_counting: bool);
    pub fn wxLog_GetTimestamp() -> *mut c_void;
    pub fn wxLog_SetTimestamp(format: *const c_void);
    pub fn wxLog_DisableTimestamp();
    pub fn wxLog_GetVerbose() -> bool;
    pub fn wxLog_SetVerbose(verbose: bool);
    pub fn wxLog_SetFormatter(self_: *mut c_void, formatter: *mut c_void) -> *mut c_void;
    pub fn wxLog_Flush(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxLog_LogRecord(self_: *mut c_void, level: wxLogLevel, msg: *const c_void, info: *const c_void);

    // wxLogChain
    pub fn wxLogChain_delete(self_: *mut c_void);
    pub fn wxLogChain_new(logger: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxLogChain_~wxLogChain(self_: *mut c_void);
    pub fn wxLogChain_DetachOldLog(self_: *mut c_void);
    pub fn wxLogChain_GetOldLog(self_: *const c_void) -> *mut c_void;
    pub fn wxLogChain_IsPassingMessages(self_: *const c_void) -> bool;
    pub fn wxLogChain_PassMessages(self_: *mut c_void, pass_messages: bool);
    pub fn wxLogChain_SetLog(self_: *mut c_void, logger: *mut c_void);

    // wxLogInterposer
    pub fn wxLogInterposer_delete(self_: *mut c_void);
    pub fn wxLogInterposer_new() -> *mut c_void;

}
