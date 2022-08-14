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

    // wxDateEvent
    pub fn wxDateEvent_CLASSINFO() -> *mut c_void;
    pub fn wxDateEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateEvent_new1(win: *mut c_void, dt: *const c_void, type_: wxEventType) -> *mut c_void;
    pub fn wxDateEvent_GetDate(self_: *const c_void) -> *mut c_void;
    pub fn wxDateEvent_SetDate(self_: *mut c_void, date: *const c_void);

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

    // wxDialog
    pub fn wxDialog_CLASSINFO() -> *mut c_void;
    pub fn wxDialog_new() -> *mut c_void;
    pub fn wxDialog_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxDialog_~wxDialog(self_: *mut c_void);
    pub fn wxDialog_AddMainButtonId(self_: *mut c_void, id: c_int);
    pub fn wxDialog_CanDoLayoutAdaptation(self_: *mut c_void) -> bool;
    pub fn wxDialog_Centre(self_: *mut c_void, direction: c_int);
    pub fn wxDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxDialog_CreateButtonSizer(self_: *mut c_void, flags: c_long) -> *mut c_void;
    pub fn wxDialog_CreateSeparatedButtonSizer(self_: *mut c_void, flags: c_long) -> *mut c_void;
    pub fn wxDialog_CreateSeparatedSizer(self_: *mut c_void, sizer: *mut c_void) -> *mut c_void;
    pub fn wxDialog_CreateStdDialogButtonSizer(self_: *mut c_void, flags: c_long) -> *mut c_void;
    pub fn wxDialog_CreateTextSizer(
        self_: *mut c_void,
        message: *const c_void,
        width_max: c_int,
    ) -> *mut c_void;
    pub fn wxDialog_DoLayoutAdaptation(self_: *mut c_void) -> bool;
    pub fn wxDialog_EndModal(self_: *mut c_void, ret_code: c_int);
    pub fn wxDialog_GetAffirmativeId(self_: *const c_void) -> c_int;
    pub fn wxDialog_GetContentWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxDialog_GetEscapeId(self_: *const c_void) -> c_int;
    pub fn wxDialog_GetLayoutAdaptationDone(self_: *const c_void) -> bool;
    pub fn wxDialog_GetLayoutAdaptationLevel(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDialog_GetLayoutAdaptationMode(self_: *const c_void) -> wxDialogLayoutAdaptationMode;
    pub fn wxDialog_GetMainButtonIds(self_: *mut c_void) -> *mut c_void;
    pub fn wxDialog_GetReturnCode(self_: *const c_void) -> c_int;
    pub fn wxDialog_GetToolBar(self_: *const c_void) -> *mut c_void;
    pub fn wxDialog_IsMainButtonId(self_: *const c_void, id: c_int) -> bool;
    pub fn wxDialog_IsModal(self_: *const c_void) -> bool;
    pub fn wxDialog_SetAffirmativeId(self_: *mut c_void, id: c_int);
    pub fn wxDialog_SetEscapeId(self_: *mut c_void, id: c_int);
    pub fn wxDialog_SetIcon(self_: *mut c_void, icon: *const c_void);
    pub fn wxDialog_SetLayoutAdaptationDone(self_: *mut c_void, done: bool);
    pub fn wxDialog_SetLayoutAdaptationLevel(self_: *mut c_void, level: c_int);
    // NOT_SUPPORTED: pub fn wxDialog_SetLayoutAdaptationMode(self_: *mut c_void, mode: wxDialogLayoutAdaptationMode);
    pub fn wxDialog_SetReturnCode(self_: *mut c_void, ret_code: c_int);
    pub fn wxDialog_ShowModal(self_: *mut c_void) -> c_int;
    pub fn wxDialog_ShowWindowModal(self_: *mut c_void);
    // BLOCKED: pub fn wxDialog_ShowWindowModalThenDo(self_: *mut c_void, on_end_modal: *const c_void);
    pub fn wxDialog_EnableLayoutAdaptation(enable: bool);
    pub fn wxDialog_GetLayoutAdapter() -> *mut c_void;
    pub fn wxDialog_IsLayoutAdaptationEnabled() -> bool;
    pub fn wxDialog_SetLayoutAdapter(adapter: *mut c_void) -> *mut c_void;

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
