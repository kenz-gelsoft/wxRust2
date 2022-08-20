use super::*;

// wxLog
pub trait LogMethods: WxRustMethods {
    fn add_trace_mask(mask: &str) {
        unsafe {
            let mask = WxString::from(mask);
            let mask = mask.as_ptr();
            ffi::wxLog_AddTraceMask(mask)
        }
    }
    fn clear_trace_masks() {
        unsafe { ffi::wxLog_ClearTraceMasks() }
    }
    fn get_trace_masks() -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxLog_GetTraceMasks()) }
    }
    fn is_allowed_trace_mask(mask: &str) -> bool {
        unsafe {
            let mask = WxString::from(mask);
            let mask = mask.as_ptr();
            ffi::wxLog_IsAllowedTraceMask(mask)
        }
    }
    fn remove_trace_mask(mask: &str) {
        unsafe {
            let mask = WxString::from(mask);
            let mask = mask.as_ptr();
            ffi::wxLog_RemoveTraceMask(mask)
        }
    }
    fn dont_create_on_demand() {
        unsafe { ffi::wxLog_DontCreateOnDemand() }
    }
    fn get_active_target() -> Option<LogIsOwned<false>> {
        unsafe { Log::option_from(ffi::wxLog_GetActiveTarget()) }
    }
    fn set_active_target<L: LogMethods>(logtarget: Option<&L>) -> Option<LogIsOwned<false>> {
        unsafe {
            let logtarget = match logtarget {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Log::option_from(ffi::wxLog_SetActiveTarget(logtarget))
        }
    }
    fn set_thread_active_target<L: LogMethods>(logger: Option<&L>) -> Option<LogIsOwned<false>> {
        unsafe {
            let logger = match logger {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            Log::option_from(ffi::wxLog_SetThreadActiveTarget(logger))
        }
    }
    fn flush_active() {
        unsafe { ffi::wxLog_FlushActive() }
    }
    fn resume() {
        unsafe { ffi::wxLog_Resume() }
    }
    fn suspend() {
        unsafe { ffi::wxLog_Suspend() }
    }
    // NOT_SUPPORTED: fn GetLogLevel()
    // NOT_SUPPORTED: fn IsLevelEnabled()
    // NOT_SUPPORTED: fn SetComponentLevel()
    // NOT_SUPPORTED: fn SetLogLevel()
    fn enable_logging(enable: bool) -> bool {
        unsafe { ffi::wxLog_EnableLogging(enable) }
    }
    fn is_enabled() -> bool {
        unsafe { ffi::wxLog_IsEnabled() }
    }
    fn get_repetition_counting() -> bool {
        unsafe { ffi::wxLog_GetRepetitionCounting() }
    }
    fn set_repetition_counting(repet_counting: bool) {
        unsafe { ffi::wxLog_SetRepetitionCounting(repet_counting) }
    }
    fn get_timestamp() -> String {
        unsafe { WxString::from_ptr(ffi::wxLog_GetTimestamp()).into() }
    }
    fn set_timestamp(format: &str) {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            ffi::wxLog_SetTimestamp(format)
        }
    }
    fn disable_timestamp() {
        unsafe { ffi::wxLog_DisableTimestamp() }
    }
    fn get_verbose() -> bool {
        unsafe { ffi::wxLog_GetVerbose() }
    }
    fn set_verbose(verbose: bool) {
        unsafe { ffi::wxLog_SetVerbose(verbose) }
    }
    fn set_formatter(&self, formatter: *mut c_void) -> *mut c_void {
        unsafe { ffi::wxLog_SetFormatter(self.as_ptr(), formatter) }
    }
    fn flush(&self) {
        unsafe { ffi::wxLog_Flush(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn LogRecord()
}

// wxLogChain
pub trait LogChainMethods: LogMethods {
    // DTOR: fn ~wxLogChain()
    fn detach_old_log(&self) {
        unsafe { ffi::wxLogChain_DetachOldLog(self.as_ptr()) }
    }
    fn get_old_log(&self) -> Option<LogIsOwned<false>> {
        unsafe { Log::option_from(ffi::wxLogChain_GetOldLog(self.as_ptr())) }
    }
    fn is_passing_messages(&self) -> bool {
        unsafe { ffi::wxLogChain_IsPassingMessages(self.as_ptr()) }
    }
    fn pass_messages(&self, pass_messages: bool) {
        unsafe { ffi::wxLogChain_PassMessages(self.as_ptr(), pass_messages) }
    }
    fn set_log<L: LogMethods>(&self, logger: Option<&L>) {
        unsafe {
            let logger = match logger {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxLogChain_SetLog(self.as_ptr(), logger)
        }
    }
}

// wxLogInterposer
pub trait LogInterposerMethods: LogChainMethods {}
