#![allow(non_upper_case_globals)]

use super::*;

// wxDataObject
wx_class! { DataObject =
    DataObjectIsOwned<true>(wxDataObject) impl
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectIsOwned<OWNED> {
    //  ENUM: Direction
    pub const Get: c_int = 0x01;
    pub const Set: c_int = 0x02;
    pub const Both: c_int = 0x03;

    // BLOCKED: fn wxDataObject()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObject_delete(self.0) }
        }
    }
}

// wxDataObjectSimple
wx_class! { DataObjectSimple =
    DataObjectSimpleIsOwned<true>(wxDataObjectSimple) impl
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> DataObjectSimpleIsOwned<OWNED> {
    pub fn new(format: *const c_void) -> DataObjectSimpleIsOwned<OWNED> {
        unsafe { DataObjectSimpleIsOwned(ffi::wxDataObjectSimple_new(format)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DataObjectSimpleIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: DataObjectSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for DataObjectSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataObjectSimple_delete(self.0) }
        }
    }
}

// wxDateEvent
wx_class! { DateEvent =
    DateEventIsOwned<true>(wxDateEvent) impl
        DateEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> DateEventIsOwned<OWNED> {
    pub fn new() -> DateEventIsOwned<OWNED> {
        unsafe { DateEventIsOwned(ffi::wxDateEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxDateEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DateEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DateEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DateEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDateEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DateEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

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

// wxDialog
wx_class! { Dialog =
    DialogIsOwned<true>(wxDialog) impl
        DialogMethods,
        // TopLevelWindowMethods,
        NonOwnedWindowMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> DialogIsOwned<OWNED> {
    pub fn new_2step() -> DialogIsOwned<OWNED> {
        unsafe { DialogIsOwned(ffi::wxDialog_new()) }
    }
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> DialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            DialogIsOwned(ffi::wxDialog_new1(
                parent, id, title, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDialog_CLASSINFO()) }
    }
}
impl<const OWNED: bool> TopLevelWindowMethods for DialogIsOwned<OWNED> {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        title: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let title = WxString::from(title);
            let title = title.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxDialog_Create(self.as_ptr(), parent, id, title, pos, size, style, name)
        }
    }
    fn set_icon<I: IconMethods>(&self, icon: &I) {
        unsafe {
            let icon = icon.as_ptr();
            ffi::wxDialog_SetIcon(self.as_ptr(), icon)
        }
    }
}
impl<const OWNED: bool> WindowMethods for DialogIsOwned<OWNED> {
    fn centre(&self, direction: c_int) {
        unsafe { ffi::wxDialog_Centre(self.as_ptr(), direction) }
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
