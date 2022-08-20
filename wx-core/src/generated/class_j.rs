#![allow(non_upper_case_globals)]

use super::*;

// wxJPEGHandler
wx_class! { JPEGHandler =
    JPEGHandlerIsOwned<true>(wxJPEGHandler) impl
        JPEGHandlerMethods,
        // ImageHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> JPEGHandlerIsOwned<OWNED> {
    pub fn new() -> JPEGHandlerIsOwned<OWNED> {
        unsafe { JPEGHandlerIsOwned(ffi::wxJPEGHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<JPEGHandlerIsOwned<OWNED>> for ImageHandlerIsOwned<OWNED> {
    fn from(o: JPEGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<JPEGHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JPEGHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JPEGHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJPEGHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JPEGHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
impl<const OWNED: bool> ImageHandlerMethods for JPEGHandlerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn GetLibraryVersionInfo()
}

// wxJoystick
wx_class! { Joystick =
    JoystickIsOwned<true>(wxJoystick) impl
        JoystickMethods,
        ObjectMethods
}
impl<const OWNED: bool> JoystickIsOwned<OWNED> {
    pub fn new(joystick: c_int) -> JoystickIsOwned<OWNED> {
        unsafe { JoystickIsOwned(ffi::wxJoystick_new(joystick)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<JoystickIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JoystickIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JoystickIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJoystick_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JoystickIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxJoystickEvent
wx_class! { JoystickEvent =
    JoystickEventIsOwned<true>(wxJoystickEvent) impl
        JoystickEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> JoystickEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxJoystickEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<JoystickEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: JoystickEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<JoystickEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: JoystickEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for JoystickEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxJoystickEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for JoystickEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
