use super::*;

extern "C" {

    // wxNonOwnedWindow
    pub fn wxNonOwnedWindow_CLASSINFO() -> *mut c_void;
    pub fn wxNonOwnedWindow_SetShape(self_: *mut c_void, region: *const c_void) -> bool;
    pub fn wxNonOwnedWindow_SetShape1(self_: *mut c_void, path: *const c_void) -> bool;

    // wxNotebook
    pub fn wxNotebook_CLASSINFO() -> *mut c_void;
    pub fn wxNotebook_new() -> *mut c_void;
    pub fn wxNotebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxNotebook_~wxNotebook(self_: *mut c_void);
    // BLOCKED: pub fn wxNotebook_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> bool;
    pub fn wxNotebook_GetRowCount(self_: *const c_void) -> c_int;
    pub fn wxNotebook_GetThemeBackgroundColour(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxNotebook_OnSelChange(self_: *mut c_void, event: *mut c_void);
    pub fn wxNotebook_SetPadding(self_: *mut c_void, padding: *const c_void);

    // wxNotifyEvent
    pub fn wxNotifyEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxNotifyEvent_new(event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxNotifyEvent_Allow(self_: *mut c_void);
    pub fn wxNotifyEvent_IsAllowed(self_: *const c_void) -> bool;
    pub fn wxNotifyEvent_Veto(self_: *mut c_void);

}
