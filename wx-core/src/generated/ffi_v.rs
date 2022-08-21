use super::*;

extern "C" {

    // wxVListBox
    pub fn wxVListBox_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxVListBox_new() -> *mut c_void;
    // BLOCKED: pub fn wxVListBox_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxVListBox_~wxVListBox(self_: *mut c_void);
    pub fn wxVListBox_Clear(self_: *mut c_void);
    pub fn wxVListBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxVListBox_DeselectAll(self_: *mut c_void) -> bool;
    pub fn wxVListBox_GetFirstSelected(self_: *const c_void, cookie: *mut c_void) -> c_int;
    pub fn wxVListBox_GetItemCount(self_: *const c_void) -> usize;
    pub fn wxVListBox_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxVListBox_GetItemRect(self_: *const c_void, item: usize) -> *mut c_void;
    pub fn wxVListBox_GetNextSelected(self_: *const c_void, cookie: *mut c_void) -> c_int;
    pub fn wxVListBox_GetSelectedCount(self_: *const c_void) -> usize;
    pub fn wxVListBox_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxVListBox_GetSelectionBackground(self_: *const c_void) -> *mut c_void;
    pub fn wxVListBox_HasMultipleSelection(self_: *const c_void) -> bool;
    pub fn wxVListBox_IsCurrent(self_: *const c_void, item: usize) -> bool;
    pub fn wxVListBox_IsSelected(self_: *const c_void, item: usize) -> bool;
    pub fn wxVListBox_Select(self_: *mut c_void, item: usize, select: bool) -> bool;
    pub fn wxVListBox_SelectAll(self_: *mut c_void) -> bool;
    pub fn wxVListBox_SelectRange(self_: *mut c_void, from: usize, to: usize) -> bool;
    pub fn wxVListBox_SetItemCount(self_: *mut c_void, count: usize);
    pub fn wxVListBox_SetMargins(self_: *mut c_void, pt: *const c_void);
    pub fn wxVListBox_SetMargins1(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxVListBox_SetSelection(self_: *mut c_void, selection: c_int);
    pub fn wxVListBox_SetSelectionBackground(self_: *mut c_void, col: *const c_void);
    pub fn wxVListBox_Toggle(self_: *mut c_void, item: usize);

    // wxVScrolledWindow
    pub fn wxVScrolledWindow_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxVScrolledWindow_new() -> *mut c_void;
    // BLOCKED: pub fn wxVScrolledWindow_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, style: c_long, name: *const c_void) -> *mut c_void;
    pub fn wxVScrolledWindow_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;

    // wxValidator
    pub fn wxValidator_CLASSINFO() -> *mut c_void;
    pub fn wxValidator_new() -> *mut c_void;
    // DTOR: pub fn wxValidator_~wxValidator(self_: *mut c_void);
    pub fn wxValidator_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxValidator_SetWindow(self_: *mut c_void, window: *mut c_void);
    pub fn wxValidator_TransferFromWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_TransferToWindow(self_: *mut c_void) -> bool;
    pub fn wxValidator_Validate(self_: *mut c_void, parent: *mut c_void) -> bool;
    pub fn wxValidator_SuppressBellOnError(suppress: bool);
    pub fn wxValidator_IsSilent() -> bool;

}
