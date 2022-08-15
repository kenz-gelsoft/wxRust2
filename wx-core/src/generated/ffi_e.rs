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

    // wxEraseEvent
    pub fn wxEraseEvent_CLASSINFO() -> *mut c_void;
    pub fn wxEraseEvent_new(id: c_int, dc: *mut c_void) -> *mut c_void;
    pub fn wxEraseEvent_GetDC(self_: *const c_void) -> *mut c_void;

    // wxEventBlocker
    pub fn wxEventBlocker_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxEventBlocker_new(win: *mut c_void, type_: wxEventType) -> *mut c_void;
    // DTOR: pub fn wxEventBlocker_~wxEventBlocker(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEventBlocker_Block(self_: *mut c_void, event_type: wxEventType);

    // wxExtHelpController
    pub fn wxExtHelpController_CLASSINFO() -> *mut c_void;
    pub fn wxExtHelpController_new(parent_window: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxExtHelpController_~wxExtHelpController(self_: *mut c_void);
    pub fn wxExtHelpController_DisplayHelp(self_: *mut c_void, relative_url: *const c_void)
        -> bool;

}
