use super::*;

// wxMessageOutput
wxwidgets! {
    /// Simple class allowing to write strings to various output channels.
    /// - [`MessageOutput`] represents a C++ `wxMessageOutput` class instance which your code has ownership, [`MessageOutputFromCpp`]`<false>` represents one which don't own.
    /// - Use [`MessageOutput`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxMessageOutput` class's documentation](https://docs.wxwidgets.org/3.2/classwx_message_output.html) for more details.
    #[doc(alias = "wxMessageOutput")]
    #[doc(alias = "MessageOutput")]
    class MessageOutput
        = MessageOutputFromCpp<true>(wxMessageOutput) impl
        MessageOutputMethods
}
impl<const FROM_CPP: bool> MessageOutputFromCpp<FROM_CPP> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for MessageOutputFromCpp<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for MessageOutputFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if FROM_CPP {
            unsafe { ffi::wxMessageOutput_delete(self.0) }
        }
    }
}
