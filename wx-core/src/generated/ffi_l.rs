use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxListBox
    pub fn wxListBox_CLASSINFO() -> *mut c_void;
    pub fn wxListBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxListBox_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxListBox_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxListBox_~wxListBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxListBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxListBox_Create1(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxListBox_Deselect(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetStringSelection(self_: *mut c_void, s: *const c_void, select: bool)
        -> bool;
    pub fn wxListBox_SetStringSelection1(self_: *mut c_void, s: *const c_void) -> bool;
    pub fn wxListBox_GetSelections(self_: *const c_void, selections: *mut c_void) -> c_int;
    pub fn wxListBox_HitTest(self_: *const c_void, point: *const c_void) -> c_int;
    pub fn wxListBox_HitTest1(self_: *const c_void, x: c_int, y: c_int) -> c_int;
    // BLOCKED: pub fn wxListBox_InsertItems(self_: *mut c_void, n_items: c_uint, items: *const c_void, pos: c_uint);
    pub fn wxListBox_InsertItems1(self_: *mut c_void, items: *const c_void, pos: c_uint);
    pub fn wxListBox_IsSelected(self_: *const c_void, n: c_int) -> bool;
    pub fn wxListBox_SetFirstItem(self_: *mut c_void, n: c_int);
    pub fn wxListBox_SetFirstItem1(self_: *mut c_void, string: *const c_void);
    pub fn wxListBox_EnsureVisible(self_: *mut c_void, n: c_int);
    pub fn wxListBox_IsSorted(self_: *const c_void) -> bool;
    pub fn wxListBox_GetCountPerPage(self_: *const c_void) -> c_int;
    pub fn wxListBox_GetTopItem(self_: *const c_void) -> c_int;
    // BLOCKED: pub fn wxListBox_MSWSetTabStops(self_: *mut c_void, tab_stops: *const c_void) -> bool;
    // Mix-in(s) to wxListBox
    pub fn wxListBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;

}
