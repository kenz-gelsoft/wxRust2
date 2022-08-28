use super::*;

// wxMessageOutput
pub trait MessageOutputMethods: WxRustMethods {
    /// Return the global message output object.
    fn get() -> Option<MessageOutputIsOwned<false>> {
        unsafe { MessageOutput::option_from(ffi::wxMessageOutput_Get()) }
    }
    /// Sets the global message output object.
    fn set<M: MessageOutputMethods>(msgout: Option<&M>) -> Option<MessageOutputIsOwned<false>> {
        unsafe {
            let msgout = match msgout {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            MessageOutput::option_from(ffi::wxMessageOutput_Set(msgout))
        }
    }
    // NOT_SUPPORTED: fn Printf()
    /// Method called by Printf() to really output the text.
    fn output(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxMessageOutput_Output(self.as_ptr(), str)
        }
    }
}
