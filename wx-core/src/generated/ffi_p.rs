use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxPanel
    pub fn wxPanel_CLASSINFO() -> *mut c_void;
    pub fn wxPanel_new() -> *mut c_void;
    pub fn wxPanel_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPanel_~wxPanel(self_: *mut c_void);
    pub fn wxPanel_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxPanel_OnSysColourChanged(self_: *mut c_void, event: *mut c_void);
    pub fn wxPanel_SetFocusIgnoringChildren(self_: *mut c_void);

    // wxPickerBase
    pub fn wxPickerBase_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxPickerBase_new() -> *mut c_void;
    // DTOR: pub fn wxPickerBase_~wxPickerBase(self_: *mut c_void);
    pub fn wxPickerBase_CreateBase(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        text: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxPickerBase_GetInternalMargin(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_GetPickerCtrlProportion(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_GetTextCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPickerBase_GetPickerCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPickerBase_GetTextCtrlProportion(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_HasTextCtrl(self_: *const c_void) -> bool;
    pub fn wxPickerBase_IsPickerCtrlGrowable(self_: *const c_void) -> bool;
    pub fn wxPickerBase_IsTextCtrlGrowable(self_: *const c_void) -> bool;
    pub fn wxPickerBase_SetInternalMargin(self_: *mut c_void, margin: c_int);
    pub fn wxPickerBase_SetPickerCtrlGrowable(self_: *mut c_void, grow: bool);
    pub fn wxPickerBase_SetPickerCtrlProportion(self_: *mut c_void, prop: c_int);
    pub fn wxPickerBase_SetTextCtrlGrowable(self_: *mut c_void, grow: bool);
    pub fn wxPickerBase_SetTextCtrlProportion(self_: *mut c_void, prop: c_int);
    pub fn wxPickerBase_SetTextCtrl(self_: *mut c_void, text: *mut c_void);
    pub fn wxPickerBase_SetPickerCtrl(self_: *mut c_void, picker: *mut c_void);
    pub fn wxPickerBase_UpdatePickerFromTextCtrl(self_: *mut c_void);
    pub fn wxPickerBase_UpdateTextCtrlFromPicker(self_: *mut c_void);

    // wxPoint
    pub fn wxPoint_delete(self_: *mut c_void);
    pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
    pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
    // BLOCKED: pub fn wxPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxPoint_new() -> *mut c_void;
    pub fn wxPoint_new1(x: c_int, y: c_int) -> *mut c_void;
    pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;

}
