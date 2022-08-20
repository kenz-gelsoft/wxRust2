use super::*;

extern "C" {

    // wxPCXHandler
    pub fn wxPCXHandler_CLASSINFO() -> *mut c_void;
    pub fn wxPCXHandler_new() -> *mut c_void;

    // wxPNGHandler
    pub fn wxPNGHandler_CLASSINFO() -> *mut c_void;
    pub fn wxPNGHandler_new() -> *mut c_void;

    // wxPNMHandler
    pub fn wxPNMHandler_CLASSINFO() -> *mut c_void;
    pub fn wxPNMHandler_new() -> *mut c_void;

    // wxPaintDC
    pub fn wxPaintDC_CLASSINFO() -> *mut c_void;
    pub fn wxPaintDC_new(window: *mut c_void) -> *mut c_void;

    // wxPaintEvent
    pub fn wxPaintEvent_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxPaintEvent_new(window: *mut c_void) -> *mut c_void;

    // wxPalette
    pub fn wxPalette_CLASSINFO() -> *mut c_void;
    pub fn wxPalette_new() -> *mut c_void;
    pub fn wxPalette_new1(palette: *const c_void) -> *mut c_void;
    pub fn wxPalette_new2(
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPalette_~wxPalette(self_: *mut c_void);
    pub fn wxPalette_Create(
        self_: *mut c_void,
        n: c_int,
        red: *const c_void,
        green: *const c_void,
        blue: *const c_void,
    ) -> bool;
    pub fn wxPalette_GetColoursCount(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPalette_GetPixel(self_: *const c_void, red: unsigned char, green: unsigned char, blue: unsigned char) -> c_int;
    pub fn wxPalette_GetRGB(
        self_: *const c_void,
        pixel: c_int,
        red: *mut c_void,
        green: *mut c_void,
        blue: *mut c_void,
    ) -> bool;
    pub fn wxPalette_IsOk(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxPalette_operator=(self_: *mut c_void, palette: *const c_void) -> *mut c_void;

    // wxPanel
    pub fn wxPanel_CLASSINFO() -> *mut c_void;
    pub fn wxPanel_new() -> *mut c_void;
    pub fn wxPanel_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxPanel_~wxPanel(self_: *mut c_void);
    pub fn wxPanel_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxPanel_OnSysColourChanged(self_: *mut c_void, event: *mut c_void);
    pub fn wxPanel_SetFocusIgnoringChildren(self_: *mut c_void);

    // wxPasswordEntryDialog
    pub fn wxPasswordEntryDialog_CLASSINFO() -> *mut c_void;
    pub fn wxPasswordEntryDialog_new(
        parent: *mut c_void,
        message: *const c_void,
        caption: *const c_void,
        default_value: *const c_void,
        style: c_long,
        pos: *const c_void,
    ) -> *mut c_void;

    // wxPen
    pub fn wxPen_CLASSINFO() -> *mut c_void;
    pub fn wxPen_new() -> *mut c_void;
    pub fn wxPen_new1(info: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPen_new2(colour: *const c_void, width: c_int, style: wxPenStyle) -> *mut c_void;
    pub fn wxPen_new3(stipple: *const c_void, width: c_int) -> *mut c_void;
    pub fn wxPen_new4(pen: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPen_~wxPen(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPen_GetCap(self_: *const c_void) -> wxPenCap;
    // NOT_SUPPORTED: pub fn wxPen_GetQuality(self_: *const c_void) -> wxPenQuality;
    pub fn wxPen_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxPen_GetDashes(self_: *const c_void, dashes: *mut c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPen_GetJoin(self_: *const c_void) -> wxPenJoin;
    pub fn wxPen_GetStipple(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPen_GetStyle(self_: *const c_void) -> wxPenStyle;
    pub fn wxPen_GetWidth(self_: *const c_void) -> c_int;
    pub fn wxPen_IsOk(self_: *const c_void) -> bool;
    pub fn wxPen_IsNonTransparent(self_: *const c_void) -> bool;
    pub fn wxPen_IsTransparent(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPen_SetCap(self_: *mut c_void, cap_style: wxPenCap);
    // NOT_SUPPORTED: pub fn wxPen_SetQuality(self_: *mut c_void, quality: wxPenQuality);
    pub fn wxPen_SetColour(self_: *mut c_void, colour: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPen_SetColour1(self_: *mut c_void, red: unsigned char, green: unsigned char, blue: unsigned char);
    pub fn wxPen_SetDashes(self_: *mut c_void, n: c_int, dash: *const c_void);
    // NOT_SUPPORTED: pub fn wxPen_SetJoin(self_: *mut c_void, join_style: wxPenJoin);
    pub fn wxPen_SetStipple(self_: *mut c_void, stipple: *const c_void);
    // NOT_SUPPORTED: pub fn wxPen_SetStyle(self_: *mut c_void, style: wxPenStyle);
    pub fn wxPen_SetWidth(self_: *mut c_void, width: c_int);
    // BLOCKED: pub fn wxPen_operator!=(self_: *const c_void, pen: *const c_void) -> bool;
    // BLOCKED: pub fn wxPen_operator=(self_: *mut c_void, pen: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPen_operator==(self_: *const c_void, pen: *const c_void) -> bool;

    // wxPenList
    pub fn wxPenList_delete(self_: *mut c_void);
    pub fn wxPenList_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPenList_FindOrCreatePen(self_: *mut c_void, colour: *const c_void, width: c_int, style: wxPenStyle) -> *mut c_void;

    // wxPersistenceManager
    pub fn wxPersistenceManager_delete(self_: *mut c_void);
    pub fn wxPersistenceManager_Set(manager: *mut c_void);
    pub fn wxPersistenceManager_Get() -> *mut c_void;
    pub fn wxPersistenceManager_DisableSaving(self_: *mut c_void);
    pub fn wxPersistenceManager_DisableRestoring(self_: *mut c_void);
    // BLOCKED: pub fn wxPersistenceManager_Register(self_: *mut c_void, obj: *mut c_void) -> *mut c_void;
    pub fn wxPersistenceManager_Register1(
        self_: *mut c_void,
        obj: *mut c_void,
        po: *mut c_void,
    ) -> *mut c_void;
    pub fn wxPersistenceManager_Find(self_: *const c_void, obj: *mut c_void) -> *mut c_void;
    pub fn wxPersistenceManager_Unregister(self_: *mut c_void, obj: *mut c_void);
    pub fn wxPersistenceManager_Save(self_: *mut c_void, obj: *mut c_void);
    pub fn wxPersistenceManager_Restore(self_: *mut c_void, obj: *mut c_void) -> bool;
    pub fn wxPersistenceManager_SaveAndUnregister(self_: *mut c_void, obj: *mut c_void);
    // BLOCKED: pub fn wxPersistenceManager_RegisterAndRestore(self_: *mut c_void, obj: *mut c_void) -> bool;
    pub fn wxPersistenceManager_RegisterAndRestore1(
        self_: *mut c_void,
        obj: *mut c_void,
        po: *mut c_void,
    ) -> bool;

    // wxPickerBase
    pub fn wxPickerBase_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxPickerBase_new() -> *mut c_void;
    // DTOR: pub fn wxPickerBase_~wxPickerBase(self_: *mut c_void);
    pub fn wxPickerBase_CreateBase(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        text: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxPickerBase_GetInternalMargin(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_GetPickerCtrlProportion(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_GetTextCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPickerBase_GetPickerCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPickerBase_GetTextCtrlProportion(self_: *const c_void) -> c_int;
    pub fn wxPickerBase_HasTextCtrl(self_: *const c_void) -> bool;
    pub fn wxPickerBase_IsPickerCtrlGrowable(self_: *const c_void) -> bool;
    pub fn wxPickerBase_IsTextCtrlGrowable(self_: *const c_void) -> bool;
    pub fn wxPickerBase_SetInternalMargin(self_: *mut c_void, margin: c_int);
    pub fn wxPickerBase_SetPickerCtrlGrowable(self_: *mut c_void, grow: bool);
    pub fn wxPickerBase_SetPickerCtrlProportion(self_: *mut c_void, prop: c_int);
    pub fn wxPickerBase_SetTextCtrlGrowable(self_: *mut c_void, grow: bool);
    pub fn wxPickerBase_SetTextCtrlProportion(self_: *mut c_void, prop: c_int);
    pub fn wxPickerBase_SetTextCtrl(self_: *mut c_void, text: *mut c_void);
    pub fn wxPickerBase_SetPickerCtrl(self_: *mut c_void, picker: *mut c_void);
    pub fn wxPickerBase_UpdatePickerFromTextCtrl(self_: *mut c_void);
    pub fn wxPickerBase_UpdateTextCtrlFromPicker(self_: *mut c_void);

    // wxPoint
    pub fn wxPoint_delete(self_: *mut c_void);
    pub fn wxPoint_IsFullySpecified(self_: *const c_void) -> bool;
    pub fn wxPoint_SetDefaults(self_: *mut c_void, pt: *const c_void);
    // BLOCKED: pub fn wxPoint_operator=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator==(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator!=(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> bool;
    // BLOCKED: pub fn wxPoint_operator+(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-(self_: *mut c_void, p1: *const c_void, p2: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=(self_: *mut c_void, pt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator+1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-1(self_: *mut c_void, pt: *const c_void, sz: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator-2(self_: *mut c_void, sz: *const c_void, pt: *const c_void) -> wxPoint;
    // BLOCKED: pub fn wxPoint_operator+=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator-=1(self_: *mut c_void, sz: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator/(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*(self_: *mut c_void, sz: *const c_void, factor: c_int) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator*1(self_: *mut c_void, factor: c_int, sz: *const c_void) -> wxSize;
    // BLOCKED: pub fn wxPoint_operator/=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxPoint_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxPoint_new() -> *mut c_void;
    pub fn wxPoint_new1(x: c_int, y: c_int) -> *mut c_void;
    pub fn wxPoint_new2(pt: *const c_void) -> *mut c_void;

    // wxPopupTransientWindow
    pub fn wxPopupTransientWindow_CLASSINFO() -> *mut c_void;
    pub fn wxPopupTransientWindow_new() -> *mut c_void;
    pub fn wxPopupTransientWindow_new1(parent: *mut c_void, flags: c_int) -> *mut c_void;
    pub fn wxPopupTransientWindow_Popup(self_: *mut c_void, focus: *mut c_void);
    pub fn wxPopupTransientWindow_Dismiss(self_: *mut c_void);
    pub fn wxPopupTransientWindow_ProcessLeftDown(self_: *mut c_void, event: *mut c_void) -> bool;

    // wxPopupWindow
    pub fn wxPopupWindow_CLASSINFO() -> *mut c_void;
    pub fn wxPopupWindow_new() -> *mut c_void;
    pub fn wxPopupWindow_new1(parent: *mut c_void, flags: c_int) -> *mut c_void;
    pub fn wxPopupWindow_Create(self_: *mut c_void, parent: *mut c_void, flags: c_int) -> bool;
    pub fn wxPopupWindow_Position(
        self_: *mut c_void,
        pt_origin: *const c_void,
        size_popup: *const c_void,
    );

    // wxPreferencesEditor
    pub fn wxPreferencesEditor_delete(self_: *mut c_void);
    pub fn wxPreferencesEditor_new(title: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPreferencesEditor_~wxPreferencesEditor(self_: *mut c_void);
    pub fn wxPreferencesEditor_AddPage(self_: *mut c_void, page: *mut c_void);
    pub fn wxPreferencesEditor_Show(self_: *mut c_void, parent: *mut c_void);
    pub fn wxPreferencesEditor_Dismiss(self_: *mut c_void);
    pub fn wxPreferencesEditor_ShouldApplyChangesImmediately() -> bool;
    pub fn wxPreferencesEditor_ShownModally() -> bool;

    // wxPreferencesPage
    pub fn wxPreferencesPage_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxPreferencesPage_new() -> *mut c_void;
    // DTOR: pub fn wxPreferencesPage_~wxPreferencesPage(self_: *mut c_void);
    pub fn wxPreferencesPage_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxPreferencesPage_GetIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxPreferencesPage_GetLargeIcon(self_: *const c_void) -> *mut c_void;
    pub fn wxPreferencesPage_CreateWindow(self_: *mut c_void, parent: *mut c_void) -> *mut c_void;

    // wxPropertySheetDialog
    pub fn wxPropertySheetDialog_CLASSINFO() -> *mut c_void;
    pub fn wxPropertySheetDialog_new() -> *mut c_void;
    pub fn wxPropertySheetDialog_new1(
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxPropertySheetDialog_AddBookCtrl(self_: *mut c_void, sizer: *mut c_void);
    pub fn wxPropertySheetDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        title: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxPropertySheetDialog_CreateBookCtrl(self_: *mut c_void) -> *mut c_void;
    pub fn wxPropertySheetDialog_CreateButtons(self_: *mut c_void, flags: c_int);
    pub fn wxPropertySheetDialog_GetBookCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxPropertySheetDialog_GetInnerSizer(self_: *const c_void) -> *mut c_void;
    pub fn wxPropertySheetDialog_SetInnerSizer(self_: *mut c_void, sizer: *mut c_void);
    pub fn wxPropertySheetDialog_GetSheetStyle(self_: *const c_void) -> c_long;
    pub fn wxPropertySheetDialog_LayoutDialog(self_: *mut c_void, centre_flags: c_int);
    pub fn wxPropertySheetDialog_SetBookCtrl(self_: *mut c_void, book_ctrl: *mut c_void);
    pub fn wxPropertySheetDialog_SetSheetStyle(self_: *mut c_void, style: c_long);
    pub fn wxPropertySheetDialog_SetSheetOuterBorder(self_: *mut c_void, border: c_int);
    pub fn wxPropertySheetDialog_GetSheetOuterBorder(self_: *const c_void) -> c_int;
    pub fn wxPropertySheetDialog_SetSheetInnerBorder(self_: *mut c_void, border: c_int);
    pub fn wxPropertySheetDialog_GetSheetInnerBorder(self_: *const c_void) -> c_int;

}
