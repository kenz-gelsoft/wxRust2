use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxGDIObject
    pub fn wxGDIObject_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxGDIObject_new() -> *mut c_void;

    // wxGauge
    pub fn wxGauge_CLASSINFO() -> *mut c_void;
    pub fn wxGauge_new() -> *mut c_void;
    pub fn wxGauge_new1(
        parent: *mut c_void,
        id: c_int,
        range: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxGauge_~wxGauge(self_: *mut c_void);
    pub fn wxGauge_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        range: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxGauge_GetRange(self_: *const c_void) -> c_int;
    pub fn wxGauge_GetValue(self_: *const c_void) -> c_int;
    pub fn wxGauge_IsVertical(self_: *const c_void) -> bool;
    pub fn wxGauge_Pulse(self_: *mut c_void);
    pub fn wxGauge_SetRange(self_: *mut c_void, range: c_int);
    pub fn wxGauge_SetValue(self_: *mut c_void, pos: c_int);

    // wxGenericDirCtrl
    pub fn wxGenericDirCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxGenericDirCtrl_new() -> *mut c_void;
    pub fn wxGenericDirCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dir: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        filter: *const c_void,
        default_filter: c_int,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxGenericDirCtrl_~wxGenericDirCtrl(self_: *mut c_void);
    pub fn wxGenericDirCtrl_CollapsePath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxGenericDirCtrl_CollapseTree(self_: *mut c_void);
    pub fn wxGenericDirCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dir: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        filter: *const c_void,
        default_filter: c_int,
        name: *const c_void,
    ) -> bool;
    pub fn wxGenericDirCtrl_ExpandPath(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxGenericDirCtrl_GetDefaultPath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilePath(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilePaths(self_: *const c_void, paths: *mut c_void);
    pub fn wxGenericDirCtrl_GetFilter(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetFilterIndex(self_: *const c_void) -> c_int;
    pub fn wxGenericDirCtrl_GetFilterListCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxGenericDirCtrl_GetPath1(self_: *const c_void, item_id: wxTreeItemId) -> *mut c_void;
    pub fn wxGenericDirCtrl_GetPaths(self_: *const c_void, paths: *mut c_void);
    // NOT_SUPPORTED: pub fn wxGenericDirCtrl_GetRootId(self_: *mut c_void) -> wxTreeItemId;
    pub fn wxGenericDirCtrl_GetTreeCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxGenericDirCtrl_Init(self_: *mut c_void);
    pub fn wxGenericDirCtrl_ReCreateTree(self_: *mut c_void);
    pub fn wxGenericDirCtrl_SetDefaultPath(self_: *mut c_void, path: *const c_void);
    pub fn wxGenericDirCtrl_SetFilter(self_: *mut c_void, filter: *const c_void);
    pub fn wxGenericDirCtrl_SetFilterIndex(self_: *mut c_void, n: c_int);
    pub fn wxGenericDirCtrl_SetPath(self_: *mut c_void, path: *const c_void);
    pub fn wxGenericDirCtrl_ShowHidden(self_: *mut c_void, show: bool);
    pub fn wxGenericDirCtrl_SelectPath(self_: *mut c_void, path: *const c_void, select: bool);
    pub fn wxGenericDirCtrl_SelectPaths(self_: *mut c_void, paths: *const c_void);
    pub fn wxGenericDirCtrl_UnselectAll(self_: *mut c_void);

}
