use super::*;

extern "C" {

    // wxDataObject
    pub fn wxDataObject_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxDataObject_new() -> *mut c_void;
    // DTOR: pub fn wxDataObject_~wxDataObject(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDataObject_GetAllFormats(self_: *const c_void, formats: *mut c_void, dir: Direction);
    pub fn wxDataObject_GetDataHere(
        self_: *const c_void,
        format: *const c_void,
        buf: *mut c_void,
    ) -> bool;
    pub fn wxDataObject_GetDataSize(self_: *const c_void, format: *const c_void) -> usize;
    // NOT_SUPPORTED: pub fn wxDataObject_GetFormatCount(self_: *const c_void, dir: Direction) -> usize;
    // NOT_SUPPORTED: pub fn wxDataObject_GetPreferredFormat(self_: *const c_void, dir: Direction) -> wxDataFormat;
    pub fn wxDataObject_SetData(
        self_: *mut c_void,
        format: *const c_void,
        len: usize,
        buf: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxDataObject_IsSupported(self_: *const c_void, format: *const c_void, dir: Direction) -> bool;

    // wxDataObjectSimple
    pub fn wxDataObjectSimple_delete(self_: *mut c_void);
    pub fn wxDataObjectSimple_new(format: *const c_void) -> *mut c_void;
    pub fn wxDataObjectSimple_GetDataHere(self_: *const c_void, buf: *mut c_void) -> bool;
    pub fn wxDataObjectSimple_GetDataSize(self_: *const c_void) -> usize;
    // BLOCKED: pub fn wxDataObjectSimple_GetFormat(self_: *const c_void) -> *const c_void;
    pub fn wxDataObjectSimple_SetData(self_: *mut c_void, len: usize, buf: *const c_void) -> bool;
    pub fn wxDataObjectSimple_SetFormat(self_: *mut c_void, format: *const c_void);

    // wxDatePickerCtrl
    pub fn wxDatePickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDatePickerCtrl_new() -> *mut c_void;
    pub fn wxDatePickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxDatePickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        dt: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDatePickerCtrl_GetRange(
        self_: *const c_void,
        dt1: *mut c_void,
        dt2: *mut c_void,
    ) -> bool;
    pub fn wxDatePickerCtrl_GetValue(self_: *const c_void) -> *mut c_void;
    pub fn wxDatePickerCtrl_SetNullText(self_: *mut c_void, text: *const c_void);
    pub fn wxDatePickerCtrl_SetRange(self_: *mut c_void, dt1: *const c_void, dt2: *const c_void);
    pub fn wxDatePickerCtrl_SetValue(self_: *mut c_void, dt: *const c_void);

    // wxDirPickerCtrl
    pub fn wxDirPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxDirPickerCtrl_new() -> *mut c_void;
    pub fn wxDirPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxDirPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        path: *const c_void,
        message: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxDirPickerCtrl_GetDirName(self_: *const c_void) -> *mut c_void;
    pub fn wxDirPickerCtrl_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxDirPickerCtrl_SetDirName(self_: *mut c_void, dirname: *const c_void);
    pub fn wxDirPickerCtrl_SetInitialDirectory(self_: *mut c_void, dir: *const c_void);
    pub fn wxDirPickerCtrl_SetPath(self_: *mut c_void, dirname: *const c_void);

}
