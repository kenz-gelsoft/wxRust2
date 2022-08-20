use super::*;

extern "C" {

    // wxOverlay
    pub fn wxOverlay_delete(self_: *mut c_void);
    pub fn wxOverlay_new() -> *mut c_void;
    // DTOR: pub fn wxOverlay_~wxOverlay(self_: *mut c_void);
    pub fn wxOverlay_Reset(self_: *mut c_void);

    // wxOwnerDrawnComboBox
    pub fn wxOwnerDrawnComboBox_CLASSINFO() -> *mut c_void;
    pub fn wxOwnerDrawnComboBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxOwnerDrawnComboBox_new1(parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxOwnerDrawnComboBox_new2(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxOwnerDrawnComboBox_~wxOwnerDrawnComboBox(self_: *mut c_void);
    pub fn wxOwnerDrawnComboBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxOwnerDrawnComboBox_Create1(self_: *mut c_void, parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxOwnerDrawnComboBox_Create2(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxOwnerDrawnComboBox_IsListEmpty(self_: *const c_void) -> bool;
    pub fn wxOwnerDrawnComboBox_IsTextEmpty(self_: *const c_void) -> bool;
    pub fn wxOwnerDrawnComboBox_GetWidestItem(self_: *mut c_void) -> c_int;
    pub fn wxOwnerDrawnComboBox_GetWidestItemWidth(self_: *mut c_void) -> c_int;
    // Mix-in(s) to wxOwnerDrawnComboBox
    pub fn wxOwnerDrawnComboBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;
    pub fn wxOwnerDrawnComboBox_AsTextEntry(obj: *mut c_void) -> *mut c_void;

}
