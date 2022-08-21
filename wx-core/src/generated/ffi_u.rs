use super::*;

extern "C" {

    // wxUIActionSimulator
    pub fn wxUIActionSimulator_delete(self_: *mut c_void);
    pub fn wxUIActionSimulator_new() -> *mut c_void;
    pub fn wxUIActionSimulator_MouseMove(self_: *mut c_void, x: c_long, y: c_long) -> bool;
    pub fn wxUIActionSimulator_MouseMove1(self_: *mut c_void, point: *const c_void) -> bool;
    pub fn wxUIActionSimulator_MouseDown(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseUp(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseClick(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseDblClick(self_: *mut c_void, button: c_int) -> bool;
    pub fn wxUIActionSimulator_MouseDragDrop(
        self_: *mut c_void,
        x1: c_long,
        y1: c_long,
        x2: c_long,
        y2: c_long,
        button: c_int,
    ) -> bool;
    pub fn wxUIActionSimulator_KeyDown(
        self_: *mut c_void,
        keycode: c_int,
        modifiers: c_int,
    ) -> bool;
    pub fn wxUIActionSimulator_KeyUp(self_: *mut c_void, keycode: c_int, modifiers: c_int) -> bool;
    pub fn wxUIActionSimulator_Char(self_: *mut c_void, keycode: c_int, modifiers: c_int) -> bool;
    pub fn wxUIActionSimulator_Select(self_: *mut c_void, text: *const c_void) -> bool;
    pub fn wxUIActionSimulator_Text(self_: *mut c_void, text: *const c_void) -> bool;

    // wxURLDataObject
    pub fn wxURLDataObject_delete(self_: *mut c_void);
    pub fn wxURLDataObject_new(url: *const c_void) -> *mut c_void;
    pub fn wxURLDataObject_GetURL(self_: *const c_void) -> *mut c_void;
    pub fn wxURLDataObject_SetURL(self_: *mut c_void, url: *const c_void);

    // wxUpdateUIEvent
    pub fn wxUpdateUIEvent_CLASSINFO() -> *mut c_void;
    pub fn wxUpdateUIEvent_new(command_id: c_int) -> *mut c_void;
    pub fn wxUpdateUIEvent_Check(self_: *mut c_void, check: bool);
    pub fn wxUpdateUIEvent_Enable(self_: *mut c_void, enable: bool);
    pub fn wxUpdateUIEvent_GetChecked(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetEnabled(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_IsCheckable(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetChecked(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetEnabled(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetShown(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetSetText(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetShown(self_: *const c_void) -> bool;
    pub fn wxUpdateUIEvent_GetText(self_: *const c_void) -> *mut c_void;
    pub fn wxUpdateUIEvent_SetText(self_: *mut c_void, text: *const c_void);
    pub fn wxUpdateUIEvent_Show(self_: *mut c_void, show: bool);
    pub fn wxUpdateUIEvent_CanUpdate(window: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxUpdateUIEvent_GetMode() -> wxUpdateUIMode;
    pub fn wxUpdateUIEvent_GetUpdateInterval() -> c_long;
    pub fn wxUpdateUIEvent_ResetUpdateTime();
    // NOT_SUPPORTED: pub fn wxUpdateUIEvent_SetMode(mode: wxUpdateUIMode);
    pub fn wxUpdateUIEvent_SetUpdateInterval(update_interval: c_long);

}
