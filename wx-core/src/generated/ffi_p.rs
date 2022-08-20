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

    // wxPrintData
    pub fn wxPrintData_CLASSINFO() -> *mut c_void;
    pub fn wxPrintData_new() -> *mut c_void;
    pub fn wxPrintData_new1(data: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPrintData_~wxPrintData(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPrintData_GetBin(self_: *const c_void) -> wxPrintBin;
    pub fn wxPrintData_GetCollate(self_: *const c_void) -> bool;
    pub fn wxPrintData_GetColour(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPrintData_GetDuplex(self_: *const c_void) -> wxDuplexMode;
    pub fn wxPrintData_GetNoCopies(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPrintData_GetOrientation(self_: *const c_void) -> wxPrintOrientation;
    // NOT_SUPPORTED: pub fn wxPrintData_GetPaperId(self_: *const c_void) -> wxPaperSize;
    pub fn wxPrintData_GetPrinterName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPrintData_GetQuality(self_: *const c_void) -> wxPrintQuality;
    pub fn wxPrintData_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPrintData_SetBin(self_: *mut c_void, flag: wxPrintBin);
    pub fn wxPrintData_SetCollate(self_: *mut c_void, flag: bool);
    pub fn wxPrintData_SetColour(self_: *mut c_void, flag: bool);
    // NOT_SUPPORTED: pub fn wxPrintData_SetDuplex(self_: *mut c_void, mode: wxDuplexMode);
    pub fn wxPrintData_SetNoCopies(self_: *mut c_void, n: c_int);
    // NOT_SUPPORTED: pub fn wxPrintData_SetOrientation(self_: *mut c_void, orientation: wxPrintOrientation);
    // NOT_SUPPORTED: pub fn wxPrintData_SetPaperId(self_: *mut c_void, paper_id: wxPaperSize);
    pub fn wxPrintData_SetPaperSize(self_: *mut c_void, size: *const c_void);
    pub fn wxPrintData_SetPrinterName(self_: *mut c_void, printer_name: *const c_void);
    // NOT_SUPPORTED: pub fn wxPrintData_SetQuality(self_: *mut c_void, quality: wxPrintQuality);
    // BLOCKED: pub fn wxPrintData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;
    pub fn wxPrintData_GetFilename(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintData_SetFilename(self_: *mut c_void, filename: *const c_void);
    // NOT_SUPPORTED: pub fn wxPrintData_GetPrintMode(self_: *const c_void) -> wxPrintMode;
    // NOT_SUPPORTED: pub fn wxPrintData_SetPrintMode(self_: *mut c_void, print_mode: wxPrintMode);

    // wxPrintDialog
    pub fn wxPrintDialog_CLASSINFO() -> *mut c_void;
    pub fn wxPrintDialog_new(parent: *mut c_void, data: *mut c_void) -> *mut c_void;
    pub fn wxPrintDialog_new1(parent: *mut c_void, data: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxPrintDialog_~wxPrintDialog(self_: *mut c_void);
    pub fn wxPrintDialog_GetPrintDC(self_: *mut c_void) -> *mut c_void;
    pub fn wxPrintDialog_GetPrintDialogData(self_: *mut c_void) -> *mut c_void;
    pub fn wxPrintDialog_GetPrintData(self_: *mut c_void) -> *mut c_void;
    pub fn wxPrintDialog_ShowModal(self_: *mut c_void) -> c_int;

    // wxPrintDialogData
    pub fn wxPrintDialogData_CLASSINFO() -> *mut c_void;
    pub fn wxPrintDialogData_new() -> *mut c_void;
    pub fn wxPrintDialogData_new1(dialog_data: *const c_void) -> *mut c_void;
    pub fn wxPrintDialogData_new2(print_data: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPrintDialogData_~wxPrintDialogData(self_: *mut c_void);
    pub fn wxPrintDialogData_EnableHelp(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_EnablePageNumbers(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_EnablePrintToFile(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_EnableSelection(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_GetAllPages(self_: *const c_void) -> bool;
    pub fn wxPrintDialogData_GetCollate(self_: *const c_void) -> bool;
    pub fn wxPrintDialogData_GetFromPage(self_: *const c_void) -> c_int;
    pub fn wxPrintDialogData_GetMaxPage(self_: *const c_void) -> c_int;
    pub fn wxPrintDialogData_GetMinPage(self_: *const c_void) -> c_int;
    pub fn wxPrintDialogData_GetNoCopies(self_: *const c_void) -> c_int;
    pub fn wxPrintDialogData_GetPrintData(self_: *mut c_void) -> *mut c_void;
    pub fn wxPrintDialogData_GetPrintToFile(self_: *const c_void) -> bool;
    pub fn wxPrintDialogData_GetSelection(self_: *const c_void) -> bool;
    pub fn wxPrintDialogData_GetToPage(self_: *const c_void) -> c_int;
    pub fn wxPrintDialogData_IsOk(self_: *const c_void) -> bool;
    pub fn wxPrintDialogData_SetCollate(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_SetFromPage(self_: *mut c_void, page: c_int);
    pub fn wxPrintDialogData_SetMaxPage(self_: *mut c_void, page: c_int);
    pub fn wxPrintDialogData_SetMinPage(self_: *mut c_void, page: c_int);
    pub fn wxPrintDialogData_SetNoCopies(self_: *mut c_void, n: c_int);
    pub fn wxPrintDialogData_SetPrintData(self_: *mut c_void, print_data: *const c_void);
    pub fn wxPrintDialogData_SetPrintToFile(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_SetSelection(self_: *mut c_void, flag: bool);
    // BLOCKED: pub fn wxPrintDialogData_SetSetupDialog(self_: *mut c_void, flag: bool);
    pub fn wxPrintDialogData_SetToPage(self_: *mut c_void, page: c_int);
    // BLOCKED: pub fn wxPrintDialogData_operator=(self_: *mut c_void, data: *const c_void);
    // BLOCKED: pub fn wxPrintDialogData_operator=1(self_: *mut c_void, data: *const c_void);

    // wxPrinter
    pub fn wxPrinter_CLASSINFO() -> *mut c_void;
    pub fn wxPrinter_new(data: *mut c_void) -> *mut c_void;
    pub fn wxPrinter_CreateAbortWindow(
        self_: *mut c_void,
        parent: *mut c_void,
        printout: *mut c_void,
    ) -> *mut c_void;
    pub fn wxPrinter_GetAbort(self_: *const c_void) -> bool;
    pub fn wxPrinter_GetPrintDialogData(self_: *const c_void) -> *mut c_void;
    pub fn wxPrinter_Print(
        self_: *mut c_void,
        parent: *mut c_void,
        printout: *mut c_void,
        prompt: bool,
    ) -> bool;
    pub fn wxPrinter_PrintDialog(self_: *mut c_void, parent: *mut c_void) -> *mut c_void;
    pub fn wxPrinter_ReportError(
        self_: *mut c_void,
        parent: *mut c_void,
        printout: *mut c_void,
        message: *const c_void,
    );
    pub fn wxPrinter_Setup(self_: *mut c_void, parent: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPrinter_GetLastError() -> wxPrinterError;

    // wxPrinterDC
    pub fn wxPrinterDC_CLASSINFO() -> *mut c_void;
    pub fn wxPrinterDC_new(print_data: *const c_void) -> *mut c_void;
    pub fn wxPrinterDC_GetPaperRect(self_: *const c_void) -> *mut c_void;

    // wxPrintout
    pub fn wxPrintout_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxPrintout_new(title: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxPrintout_~wxPrintout(self_: *mut c_void);
    pub fn wxPrintout_FitThisSizeToPage(self_: *mut c_void, image_size: *const c_void);
    pub fn wxPrintout_FitThisSizeToPageMargins(
        self_: *mut c_void,
        image_size: *const c_void,
        page_setup_data: *const c_void,
    );
    pub fn wxPrintout_FitThisSizeToPaper(self_: *mut c_void, image_size: *const c_void);
    pub fn wxPrintout_GetDC(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintout_GetLogicalPageMarginsRect(
        self_: *const c_void,
        page_setup_data: *const c_void,
    ) -> *mut c_void;
    pub fn wxPrintout_GetLogicalPageRect(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintout_GetLogicalPaperRect(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintout_GetPPIPrinter(self_: *const c_void, w: *mut c_void, h: *mut c_void);
    pub fn wxPrintout_GetPPIScreen(self_: *const c_void, w: *mut c_void, h: *mut c_void);
    pub fn wxPrintout_GetPageInfo(
        self_: *mut c_void,
        min_page: *mut c_void,
        max_page: *mut c_void,
        page_from: *mut c_void,
        page_to: *mut c_void,
    );
    pub fn wxPrintout_GetPageSizeMM(self_: *const c_void, w: *mut c_void, h: *mut c_void);
    pub fn wxPrintout_GetPageSizePixels(self_: *const c_void, w: *mut c_void, h: *mut c_void);
    pub fn wxPrintout_GetPaperRectPixels(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintout_GetTitle(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintout_HasPage(self_: *mut c_void, page_num: c_int) -> bool;
    pub fn wxPrintout_IsPreview(self_: *const c_void) -> bool;
    pub fn wxPrintout_GetPreview(self_: *const c_void) -> *mut c_void;
    pub fn wxPrintout_MapScreenSizeToDevice(self_: *mut c_void);
    pub fn wxPrintout_MapScreenSizeToPage(self_: *mut c_void);
    pub fn wxPrintout_MapScreenSizeToPageMargins(
        self_: *mut c_void,
        page_setup_data: *const c_void,
    );
    pub fn wxPrintout_MapScreenSizeToPaper(self_: *mut c_void);
    pub fn wxPrintout_OffsetLogicalOrigin(self_: *mut c_void, xoff: c_int, yoff: c_int);
    pub fn wxPrintout_OnBeginDocument(
        self_: *mut c_void,
        start_page: c_int,
        end_page: c_int,
    ) -> bool;
    pub fn wxPrintout_OnBeginPrinting(self_: *mut c_void);
    pub fn wxPrintout_OnEndDocument(self_: *mut c_void);
    pub fn wxPrintout_OnEndPrinting(self_: *mut c_void);
    pub fn wxPrintout_OnPreparePrinting(self_: *mut c_void);
    pub fn wxPrintout_OnPrintPage(self_: *mut c_void, page_num: c_int) -> bool;
    pub fn wxPrintout_SetLogicalOrigin(self_: *mut c_void, x: c_int, y: c_int);

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
