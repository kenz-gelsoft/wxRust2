use super::*;

extern "C" {

    // wxIcon
    pub fn wxIcon_CLASSINFO() -> *mut c_void;
    pub fn wxIcon_new() -> *mut c_void;
    pub fn wxIcon_new1(icon: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIcon_new2(bits: char, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxIcon_new3(bits: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxIcon_new4(name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> *mut c_void;
    pub fn wxIcon_new5(loc: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxIcon_~wxIcon(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxIcon_CreateFromHICON(self_: *mut c_void, icon: WXHICON) -> bool;
    // NOT_SUPPORTED: pub fn wxIcon_ConvertToDisabled(self_: *const c_void, brightness: unsigned char) -> *mut c_void;
    pub fn wxIcon_CopyFromBitmap(self_: *mut c_void, bmp: *const c_void);
    pub fn wxIcon_GetDepth(self_: *const c_void) -> c_int;
    pub fn wxIcon_GetHeight(self_: *const c_void) -> c_int;
    pub fn wxIcon_GetLogicalHeight(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetLogicalSize(self_: *const c_void) -> *mut c_void;
    pub fn wxIcon_GetLogicalWidth(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetScaleFactor(self_: *const c_void) -> c_double;
    pub fn wxIcon_GetSize(self_: *const c_void) -> *mut c_void;
    pub fn wxIcon_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxIcon_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxIcon_LoadFile(self_: *mut c_void, name: *const c_void, type_: wxBitmapType, desired_width: c_int, desired_height: c_int) -> bool;
    pub fn wxIcon_SetDepth(self_: *mut c_void, depth: c_int);
    pub fn wxIcon_SetHeight(self_: *mut c_void, height: c_int);
    pub fn wxIcon_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxIcon_operator=(self_: *mut c_void, icon: *const c_void) -> *mut c_void;

    // wxItemContainer
    pub fn wxItemContainer_delete(self_: *mut c_void);
    pub fn wxItemContainer_Append(self_: *mut c_void, item: *const c_void) -> c_int;
    pub fn wxItemContainer_Append1(
        self_: *mut c_void,
        item: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append2(
        self_: *mut c_void,
        item: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append3(self_: *mut c_void, items: *const c_void) -> c_int;
    // BLOCKED: pub fn wxItemContainer_Append4(self_: *mut c_void, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Append5(
        self_: *mut c_void,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append6(
        self_: *mut c_void,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append7(self_: *mut c_void, n: c_uint, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Append8(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Append9(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Clear(self_: *mut c_void);
    pub fn wxItemContainer_Delete(self_: *mut c_void, n: c_uint);
    pub fn wxItemContainer_DetachClientObject(self_: *mut c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_HasClientData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_HasClientObjectData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_HasClientUntypedData(self_: *const c_void) -> bool;
    pub fn wxItemContainer_GetClientData(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_GetClientObject(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainer_SetClientData(self_: *mut c_void, n: c_uint, data: *mut c_void);
    pub fn wxItemContainer_SetClientObject(self_: *mut c_void, n: c_uint, data: *mut c_void);
    pub fn wxItemContainer_Insert(self_: *mut c_void, item: *const c_void, pos: c_uint) -> c_int;
    pub fn wxItemContainer_Insert1(
        self_: *mut c_void,
        item: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert2(
        self_: *mut c_void,
        item: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert3(self_: *mut c_void, items: *const c_void, pos: c_uint) -> c_int;
    // BLOCKED: pub fn wxItemContainer_Insert4(self_: *mut c_void, items: *const c_void) -> c_int;
    pub fn wxItemContainer_Insert5(
        self_: *mut c_void,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert6(
        self_: *mut c_void,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert7(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
    ) -> c_int;
    pub fn wxItemContainer_Insert8(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Insert9(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        pos: c_uint,
        client_data: *mut c_void,
    ) -> c_int;
    pub fn wxItemContainer_Set(self_: *mut c_void, items: *const c_void);
    // BLOCKED: pub fn wxItemContainer_Set1(self_: *mut c_void, items: *const c_void);
    pub fn wxItemContainer_Set2(self_: *mut c_void, items: *const c_void, client_data: *mut c_void);
    pub fn wxItemContainer_Set3(self_: *mut c_void, items: *const c_void, client_data: *mut c_void);
    pub fn wxItemContainer_Set4(self_: *mut c_void, n: c_uint, items: *const c_void);
    pub fn wxItemContainer_Set5(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    );
    pub fn wxItemContainer_Set6(
        self_: *mut c_void,
        n: c_uint,
        items: *const c_void,
        client_data: *mut c_void,
    );

    // wxItemContainerImmutable
    pub fn wxItemContainerImmutable_delete(self_: *mut c_void);
    pub fn wxItemContainerImmutable_SetSelection(self_: *mut c_void, n: c_int);
    pub fn wxItemContainerImmutable_GetSelection(self_: *const c_void) -> c_int;
    pub fn wxItemContainerImmutable_SetStringSelection(
        self_: *mut c_void,
        string: *const c_void,
    ) -> bool;
    pub fn wxItemContainerImmutable_GetStringSelection(self_: *const c_void) -> *mut c_void;
    pub fn wxItemContainerImmutable_Select(self_: *mut c_void, n: c_int);
    // BLOCKED: pub fn wxItemContainerImmutable_new() -> *mut c_void;
    pub fn wxItemContainerImmutable_GetCount(self_: *const c_void) -> c_uint;
    pub fn wxItemContainerImmutable_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxItemContainerImmutable_GetString(self_: *const c_void, n: c_uint) -> *mut c_void;
    pub fn wxItemContainerImmutable_GetStrings(self_: *const c_void) -> *mut c_void;
    pub fn wxItemContainerImmutable_SetString(self_: *mut c_void, n: c_uint, string: *const c_void);
    pub fn wxItemContainerImmutable_FindString(
        self_: *const c_void,
        string: *const c_void,
        case_sensitive: bool,
    ) -> c_int;

}
