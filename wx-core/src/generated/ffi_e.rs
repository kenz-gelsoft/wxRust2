use super::*;

extern "C" {

    // wxEditableListBox
    pub fn wxEditableListBox_CLASSINFO() -> *mut c_void;
    pub fn wxEditableListBox_new() -> *mut c_void;
    pub fn wxEditableListBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxEditableListBox_~wxEditableListBox(self_: *mut c_void);
    pub fn wxEditableListBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxEditableListBox_SetStrings(self_: *mut c_void, strings: *const c_void);
    pub fn wxEditableListBox_GetStrings(self_: *const c_void, strings: *mut c_void);

}
