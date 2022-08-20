#![allow(non_upper_case_globals)]

use super::*;

// wxLog
wx_class! { Log =
    LogIsOwned<true>(wxLog) impl
        LogMethods
}
impl<const OWNED: bool> LogIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for LogIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLog_delete(self.0) }
        }
    }
}

// wxLogChain
wx_class! { LogChain =
    LogChainIsOwned<true>(wxLogChain) impl
        LogChainMethods,
        LogMethods
}
impl<const OWNED: bool> LogChainIsOwned<OWNED> {
    pub fn new<L: LogMethods>(logger: Option<&L>) -> LogChainIsOwned<OWNED> {
        unsafe {
            let logger = match logger {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            LogChainIsOwned(ffi::wxLogChain_new(logger))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogChainIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogChainIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogChainIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogChain_delete(self.0) }
        }
    }
}

// wxLogInterposer
wx_class! { LogInterposer =
    LogInterposerIsOwned<true>(wxLogInterposer) impl
        LogInterposerMethods,
        LogChainMethods,
        LogMethods
}
impl<const OWNED: bool> LogInterposerIsOwned<OWNED> {
    pub fn new() -> LogInterposerIsOwned<OWNED> {
        unsafe { LogInterposerIsOwned(ffi::wxLogInterposer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogInterposerIsOwned<OWNED>> for LogChainIsOwned<OWNED> {
    fn from(o: LogInterposerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LogInterposerIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogInterposerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogInterposerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogInterposer_delete(self.0) }
        }
    }
}
