#![allow(non_upper_case_globals)]

use super::*;

// wxDatePickerCtrl
wx_class! { DatePickerCtrl =
    DatePickerCtrlIsOwned<true>(wxDatePickerCtrl) impl
        DatePickerCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DatePickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DatePickerCtrlIsOwned<OWNED> {
        unsafe { DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new()) }
    }
    pub fn new<
        W: WindowMethods,
        D: DateTimeMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        parent: Option<&W>,
        id: c_int,
        dt: &D,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DatePickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt = dt.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DatePickerCtrlIsOwned(ffi::wxDatePickerCtrl_new1(
                parent, id, dt, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DatePickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DatePickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DatePickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDatePickerCtrl_CLASSINFO()) }
    }
}

// wxDirPickerCtrl
wx_class! { DirPickerCtrl =
    DirPickerCtrlIsOwned<true>(wxDirPickerCtrl) impl
        DirPickerCtrlMethods,
        PickerBaseMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DirPickerCtrlIsOwned<OWNED> {
    pub fn new_2step() -> DirPickerCtrlIsOwned<OWNED> {
        unsafe { DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        parent: Option<&W>,
        id: c_int,
        path: &str,
        message: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> DirPickerCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let path = WxString::from(path);
            let path = path.as_ptr();
            let message = WxString::from(message);
            let message = message.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DirPickerCtrlIsOwned(ffi::wxDirPickerCtrl_new1(
                parent, id, path, message, pos, size, style, validator, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for PickerBaseIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DirPickerCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DirPickerCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DirPickerCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDirPickerCtrl_CLASSINFO()) }
    }
}
