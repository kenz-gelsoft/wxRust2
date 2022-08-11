use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxCheckBox
    pub fn wxCheckBox_CLASSINFO() -> *mut c_void;
    pub fn wxCheckBox_new() -> *mut c_void;
    pub fn wxCheckBox_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxCheckBox_~wxCheckBox(self_: *mut c_void);
    pub fn wxCheckBox_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCheckBox_GetValue(self_: *const c_void) -> bool;
    pub fn wxCheckBox_Get3StateValue(self_: *const c_void) -> c_int;
    pub fn wxCheckBox_Is3State(self_: *const c_void) -> bool;
    pub fn wxCheckBox_Is3rdStateAllowedForUser(self_: *const c_void) -> bool;
    pub fn wxCheckBox_IsChecked(self_: *const c_void) -> bool;
    pub fn wxCheckBox_SetValue(self_: *mut c_void, state: bool);
    pub fn wxCheckBox_Set3StateValue(self_: *mut c_void, state: c_int);

    // wxCheckListBox
    pub fn wxCheckListBox_CLASSINFO() -> *mut c_void;
    pub fn wxCheckListBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCheckListBox_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxCheckListBox_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCheckListBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n_strings: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxCheckListBox_Create1(
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
    // DTOR: pub fn wxCheckListBox_~wxCheckListBox(self_: *mut c_void);
    pub fn wxCheckListBox_Check(self_: *mut c_void, item: c_uint, check: bool);
    pub fn wxCheckListBox_IsChecked(self_: *const c_void, item: c_uint) -> bool;
    pub fn wxCheckListBox_GetCheckedItems(
        self_: *const c_void,
        checked_items: *mut c_void,
    ) -> c_uint;
    // Mix-in(s) to wxCheckListBox
    pub fn wxCheckListBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxChoice
    pub fn wxChoice_CLASSINFO() -> *mut c_void;
    pub fn wxChoice_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxChoice_new1(parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxChoice_new2(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        choices: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxChoice_~wxChoice(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxChoice_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxChoice_Create1(
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
    pub fn wxChoice_GetColumns(self_: *const c_void) -> c_int;
    pub fn wxChoice_GetCurrentSelection(self_: *const c_void) -> c_int;
    pub fn wxChoice_SetColumns(self_: *mut c_void, n: c_int);
    pub fn wxChoice_IsSorted(self_: *const c_void) -> bool;
    // Mix-in(s) to wxChoice
    pub fn wxChoice_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxColour
    pub fn wxColour_CLASSINFO() -> *mut c_void;
    pub fn wxColour_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_new1(red: unsigned char, green: unsigned char, blue: unsigned char, alpha: unsigned char) -> *mut c_void;
    pub fn wxColour_new2(colour_name: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_new3(col_rgb: unsigned long) -> *mut c_void;
    pub fn wxColour_new4(colour: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_Alpha(self_: *const c_void) -> unsigned char;
    // NOT_SUPPORTED: pub fn wxColour_Blue(self_: *const c_void) -> unsigned char;
    pub fn wxColour_GetAlpha(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetBlue(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetGreen(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetRed(self_: *const c_void) -> c_uint;
    pub fn wxColour_GetAsString(self_: *const c_void, flags: c_long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColour_SetRGB(self_: *mut c_void, col_rgb: wxUint32);
    // NOT_SUPPORTED: pub fn wxColour_SetRGBA(self_: *mut c_void, col_rgba: wxUint32);
    // NOT_SUPPORTED: pub fn wxColour_GetRGB(self_: *const c_void) -> wxUint32;
    // NOT_SUPPORTED: pub fn wxColour_GetRGBA(self_: *const c_void) -> wxUint32;
    pub fn wxColour_GetLuminance(self_: *const c_void) -> c_double;
    // NOT_SUPPORTED: pub fn wxColour_GetPixel(self_: *const c_void) -> wxIntPtr;
    // NOT_SUPPORTED: pub fn wxColour_Green(self_: *const c_void) -> unsigned char;
    pub fn wxColour_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxColour_Red(self_: *const c_void) -> unsigned char;
    pub fn wxColour_IsSolid(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxColour_Set(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char, alpha: unsigned char);
    // NOT_SUPPORTED: pub fn wxColour_Set1(self_: *mut c_void, rgb: unsigned long);
    pub fn wxColour_Set2(self_: *mut c_void, str: *const c_void) -> bool;
    // BLOCKED: pub fn wxColour_operator!=(self_: *const c_void, colour: *const c_void) -> bool;
    // BLOCKED: pub fn wxColour_operator=(self_: *mut c_void, colour: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxColour_operator==(self_: *const c_void, colour: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxColour_MakeDisabled(self_: *mut c_void, brightness: unsigned char) -> *mut c_void;
    // BLOCKED: pub fn wxColour_ChangeLightness(self_: *const c_void, ialpha: c_int) -> wxColour;
    pub fn wxColour_MakeMono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool);
    // NOT_SUPPORTED: pub fn wxColour_MakeDisabled1(r: *mut c_void, g: *mut c_void, b: *mut c_void, brightness: unsigned char);
    pub fn wxColour_MakeGrey(r: *mut c_void, g: *mut c_void, b: *mut c_void);
    pub fn wxColour_MakeGrey1(
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    );
    // NOT_SUPPORTED: pub fn wxColour_AlphaBlend(fg: unsigned char, bg: unsigned char, alpha: c_double) -> unsigned char;
    pub fn wxColour_ChangeLightness1(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int);

    // wxColourPickerCtrl
    pub fn wxColourPickerCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxColourPickerCtrl_new() -> *mut c_void;
    pub fn wxColourPickerCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        colour: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxColourPickerCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        colour: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxColourPickerCtrl_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxColourPickerCtrl_SetColour(self_: *mut c_void, col: *const c_void);
    // BLOCKED: pub fn wxColourPickerCtrl_SetColour1(self_: *mut c_void, colname: *const c_void);

    // wxComboBox
    pub fn wxComboBox_CLASSINFO() -> *mut c_void;
    pub fn wxComboBox_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxComboBox_new1(parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> *mut c_void;
    pub fn wxComboBox_new2(
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
    // DTOR: pub fn wxComboBox_~wxComboBox(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxComboBox_Create(self_: *mut c_void, parent: *mut c_void, id: c_int, value: *const c_void, pos: *const c_void, size: *const c_void, n: c_int, choices: wxString, style: c_long, validator: *const c_void, name: *const c_void) -> bool;
    pub fn wxComboBox_Create1(
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
    pub fn wxComboBox_GetCurrentSelection(self_: *const c_void) -> c_int;
    pub fn wxComboBox_IsListEmpty(self_: *const c_void) -> bool;
    pub fn wxComboBox_IsTextEmpty(self_: *const c_void) -> bool;
    pub fn wxComboBox_Popup(self_: *mut c_void);
    pub fn wxComboBox_Dismiss(self_: *mut c_void);
    // Mix-in(s) to wxComboBox
    pub fn wxComboBox_AsItemContainer(obj: *mut c_void) -> *mut c_void;
    pub fn wxComboBox_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxCommandEvent
    pub fn wxCommandEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCommandEvent_new(command_event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxCommandEvent_GetClientData(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_GetClientObject(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_GetExtraLong(self_: *const c_void) -> c_long;
    pub fn wxCommandEvent_GetInt(self_: *const c_void) -> c_int;
    pub fn wxCommandEvent_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxCommandEvent_GetString(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandEvent_IsChecked(self_: *const c_void) -> bool;
    pub fn wxCommandEvent_IsSelection(self_: *const c_void) -> bool;
    pub fn wxCommandEvent_SetClientData(self_: *mut c_void, client_data: *mut c_void);
    pub fn wxCommandEvent_SetClientObject(self_: *mut c_void, client_object: *mut c_void);
    pub fn wxCommandEvent_SetExtraLong(self_: *mut c_void, extra_long: c_long);
    pub fn wxCommandEvent_SetInt(self_: *mut c_void, int_command: c_int);
    pub fn wxCommandEvent_SetString(self_: *mut c_void, string: *const c_void);

    // wxControl
    pub fn wxControl_CLASSINFO() -> *mut c_void;
    pub fn wxControl_new(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxControl_new1() -> *mut c_void;
    pub fn wxControl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxControl_Command(self_: *mut c_void, event: *mut c_void);
    pub fn wxControl_GetLabelText(self_: *const c_void) -> *mut c_void;
    pub fn wxControl_GetSizeFromTextSize(
        self_: *const c_void,
        xlen: c_int,
        ylen: c_int,
    ) -> *mut c_void;
    pub fn wxControl_GetSizeFromTextSize1(
        self_: *const c_void,
        tsize: *const c_void,
    ) -> *mut c_void;
    pub fn wxControl_GetSizeFromText(self_: *const c_void, text: *const c_void) -> *mut c_void;
    pub fn wxControl_SetLabelText(self_: *mut c_void, text: *const c_void);
    pub fn wxControl_SetLabelMarkup(self_: *mut c_void, markup: *const c_void) -> bool;
    pub fn wxControl_GetLabelText1(label: *const c_void) -> *mut c_void;
    pub fn wxControl_RemoveMnemonics(str: *const c_void) -> *mut c_void;
    pub fn wxControl_EscapeMnemonics(text: *const c_void) -> *mut c_void;
    pub fn wxControl_Ellipsize(
        label: *const c_void,
        dc: *const c_void,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> *mut c_void;

}
