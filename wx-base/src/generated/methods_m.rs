use super::*;

// wxMessageOutput
/// This trait represents C++ [`wxMessageOutput`](https://docs.wxwidgets.org/3.2/classwx_message_output.html) class's methods and inheritance.
///
/// See [`MessageOutputIsOwned`] documentation for the class usage.
pub trait MessageOutputMethods: WxRustMethods {
    /// Return the global message output object.
    ///
    /// [See `wxMessageOutput::Get()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_output.html#aad01aa8c43d59976cd19d8b81311b089)
    fn get() -> Option<MessageOutputIsOwned<false>> {
        unsafe { MessageOutput::option_from(ffi::wxMessageOutput_Get()) }
    }
    /// Sets the global message output object.
    ///
    /// [See `wxMessageOutput::Set()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_output.html#a14dc1b2024d1b440e1fb574167f80442)
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
    ///
    /// [See `wxMessageOutput::Output()`'s original doc.](https://docs.wxwidgets.org/3.2/classwx_message_output.html#ae8b0b6734006e716bd80fd053a7b4e4d)
    fn output(&self, str: &str) {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxMessageOutput_Output(self.as_ptr(), str)
        }
    }
}
