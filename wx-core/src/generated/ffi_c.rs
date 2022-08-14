use super::*;

extern "C" {

    // wxCalculateLayoutEvent
    pub fn wxCalculateLayoutEvent_CLASSINFO() -> *mut c_void;
    pub fn wxCalculateLayoutEvent_new(id: c_int) -> *mut c_void;
    pub fn wxCalculateLayoutEvent_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxCalculateLayoutEvent_GetRect(self_: *const c_void) -> *mut c_void;
    pub fn wxCalculateLayoutEvent_SetFlags(self_: *mut c_void, flags: c_int);
    pub fn wxCalculateLayoutEvent_SetRect(self_: *mut c_void, rect: *const c_void);

    // wxCalendarCtrl
    pub fn wxCalendarCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxCalendarCtrl_SetDateRange(
        self_: *mut c_void,
        lowerdate: *const c_void,
        upperdate: *const c_void,
    ) -> bool;
    pub fn wxCalendarCtrl_GetDateRange(
        self_: *const c_void,
        lowerdate: *mut c_void,
        upperdate: *mut c_void,
    ) -> bool;
    pub fn wxCalendarCtrl_new() -> *mut c_void;
    pub fn wxCalendarCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        date: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxCalendarCtrl_~wxCalendarCtrl(self_: *mut c_void);
    pub fn wxCalendarCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        date: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxCalendarCtrl_EnableHolidayDisplay(self_: *mut c_void, display: bool);
    pub fn wxCalendarCtrl_EnableMonthChange(self_: *mut c_void, enable: bool) -> bool;
    pub fn wxCalendarCtrl_EnableYearChange(self_: *mut c_void, enable: bool);
    pub fn wxCalendarCtrl_GetAttr(self_: *const c_void, day: usize) -> *mut c_void;
    pub fn wxCalendarCtrl_GetDate(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHeaderColourBg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHeaderColourFg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHighlightColourBg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHighlightColourFg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHolidayColourBg(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarCtrl_GetHolidayColourFg(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarCtrl_HitTest(self_: *mut c_void, pos: *const c_void, date: *mut c_void, wd: *mut c_void) -> wxCalendarHitTestResult;
    pub fn wxCalendarCtrl_ResetAttr(self_: *mut c_void, day: usize);
    pub fn wxCalendarCtrl_SetAttr(self_: *mut c_void, day: usize, attr: *mut c_void);
    pub fn wxCalendarCtrl_SetDate(self_: *mut c_void, date: *const c_void) -> bool;
    pub fn wxCalendarCtrl_SetHeaderColours(
        self_: *mut c_void,
        col_fg: *const c_void,
        col_bg: *const c_void,
    );
    pub fn wxCalendarCtrl_SetHighlightColours(
        self_: *mut c_void,
        col_fg: *const c_void,
        col_bg: *const c_void,
    );
    pub fn wxCalendarCtrl_SetHoliday(self_: *mut c_void, day: usize);
    pub fn wxCalendarCtrl_SetHolidayColours(
        self_: *mut c_void,
        col_fg: *const c_void,
        col_bg: *const c_void,
    );
    pub fn wxCalendarCtrl_Mark(self_: *mut c_void, day: usize, mark: bool);

    // wxCalendarDateAttr
    pub fn wxCalendarDateAttr_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_new(col_text: *const c_void, col_back: *const c_void, col_border: *const c_void, font: *const c_void, border: wxCalendarDateBorder) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_new1(border: wxCalendarDateBorder, col_border: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_GetBackgroundColour(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_GetBorder(self_: *const c_void) -> wxCalendarDateBorder;
    pub fn wxCalendarDateAttr_GetBorderColour(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_GetFont(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_GetTextColour(self_: *const c_void) -> *mut c_void;
    pub fn wxCalendarDateAttr_HasBackgroundColour(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasBorder(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasBorderColour(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasFont(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_HasTextColour(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_IsHoliday(self_: *const c_void) -> bool;
    pub fn wxCalendarDateAttr_SetBackgroundColour(self_: *mut c_void, col_back: *const c_void);
    // NOT_SUPPORTED: pub fn wxCalendarDateAttr_SetBorder(self_: *mut c_void, border: wxCalendarDateBorder);
    pub fn wxCalendarDateAttr_SetBorderColour(self_: *mut c_void, col: *const c_void);
    pub fn wxCalendarDateAttr_SetFont(self_: *mut c_void, font: *const c_void);
    pub fn wxCalendarDateAttr_SetHoliday(self_: *mut c_void, holiday: bool);
    pub fn wxCalendarDateAttr_SetTextColour(self_: *mut c_void, col_text: *const c_void);
    pub fn wxCalendarDateAttr_GetMark() -> *mut c_void;
    pub fn wxCalendarDateAttr_SetMark(m: *const c_void);

    // wxCalendarEvent
    pub fn wxCalendarEvent_CLASSINFO() -> *mut c_void;
    pub fn wxCalendarEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarEvent_new1(win: *mut c_void, dt: *const c_void, type_: wxEventType) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCalendarEvent_GetWeekDay(self_: *const c_void) -> wxDateTime::WeekDay;
    // NOT_SUPPORTED: pub fn wxCalendarEvent_SetWeekDay(self_: *mut c_void, day: wxDateTime::WeekDay);

    // wxCaret
    pub fn wxCaret_delete(self_: *mut c_void);
    pub fn wxCaret_new() -> *mut c_void;
    pub fn wxCaret_new1(window: *mut c_void, width: c_int, height: c_int) -> *mut c_void;
    pub fn wxCaret_new2(window: *mut c_void, size: *const c_void) -> *mut c_void;
    pub fn wxCaret_Create(
        self_: *mut c_void,
        window: *mut c_void,
        width: c_int,
        height: c_int,
    ) -> bool;
    pub fn wxCaret_Create1(self_: *mut c_void, window: *mut c_void, size: *const c_void) -> bool;
    pub fn wxCaret_GetPosition(self_: *const c_void, x: *mut c_void, y: *mut c_void);
    pub fn wxCaret_GetPosition1(self_: *const c_void) -> *mut c_void;
    pub fn wxCaret_GetSize(self_: *const c_void, width: *mut c_void, height: *mut c_void);
    pub fn wxCaret_GetSize1(self_: *const c_void) -> *mut c_void;
    pub fn wxCaret_GetWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxCaret_Hide(self_: *mut c_void);
    pub fn wxCaret_IsOk(self_: *const c_void) -> bool;
    pub fn wxCaret_IsVisible(self_: *const c_void) -> bool;
    pub fn wxCaret_Move(self_: *mut c_void, x: c_int, y: c_int);
    pub fn wxCaret_Move1(self_: *mut c_void, pt: *const c_void);
    pub fn wxCaret_SetSize(self_: *mut c_void, width: c_int, height: c_int);
    pub fn wxCaret_SetSize1(self_: *mut c_void, size: *const c_void);
    pub fn wxCaret_Show(self_: *mut c_void, show: bool);
    pub fn wxCaret_GetBlinkTime() -> c_int;
    pub fn wxCaret_SetBlinkTime(milliseconds: c_int);

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

    // wxChildFocusEvent
    pub fn wxChildFocusEvent_CLASSINFO() -> *mut c_void;
    pub fn wxChildFocusEvent_new(win: *mut c_void) -> *mut c_void;
    pub fn wxChildFocusEvent_GetWindow(self_: *const c_void) -> *mut c_void;

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

    // wxChoicebook
    pub fn wxChoicebook_CLASSINFO() -> *mut c_void;
    pub fn wxChoicebook_new() -> *mut c_void;
    pub fn wxChoicebook_new1(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxChoicebook_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        name: *const c_void,
    ) -> bool;
    pub fn wxChoicebook_GetChoiceCtrl(self_: *const c_void) -> *mut c_void;

    // wxClipboard
    pub fn wxClipboard_CLASSINFO() -> *mut c_void;
    pub fn wxClipboard_new() -> *mut c_void;
    // DTOR: pub fn wxClipboard_~wxClipboard(self_: *mut c_void);
    pub fn wxClipboard_AddData(self_: *mut c_void, data: *mut c_void) -> bool;
    pub fn wxClipboard_Clear(self_: *mut c_void);
    pub fn wxClipboard_Close(self_: *mut c_void);
    pub fn wxClipboard_Flush(self_: *mut c_void) -> bool;
    pub fn wxClipboard_GetData(self_: *mut c_void, data: *mut c_void) -> bool;
    pub fn wxClipboard_IsOpened(self_: *const c_void) -> bool;
    pub fn wxClipboard_IsSupported(self_: *mut c_void, format: *const c_void) -> bool;
    pub fn wxClipboard_IsUsingPrimarySelection(self_: *const c_void) -> bool;
    pub fn wxClipboard_Open(self_: *mut c_void) -> bool;
    pub fn wxClipboard_SetData(self_: *mut c_void, data: *mut c_void) -> bool;
    pub fn wxClipboard_UsePrimarySelection(self_: *mut c_void, primary: bool);
    pub fn wxClipboard_Get() -> *mut c_void;

    // wxClipboardTextEvent
    pub fn wxClipboardTextEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxClipboardTextEvent_new(command_type: wxEventType, id: c_int) -> *mut c_void;

    // wxCloseEvent
    pub fn wxCloseEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCloseEvent_new(command_event_type: wxEventType, id: c_int) -> *mut c_void;
    pub fn wxCloseEvent_CanVeto(self_: *const c_void) -> bool;
    pub fn wxCloseEvent_GetLoggingOff(self_: *const c_void) -> bool;
    pub fn wxCloseEvent_SetCanVeto(self_: *mut c_void, can_veto: bool);
    pub fn wxCloseEvent_SetLoggingOff(self_: *mut c_void, logging_off: bool);
    pub fn wxCloseEvent_Veto(self_: *mut c_void, veto: bool);
    pub fn wxCloseEvent_GetVeto(self_: *const c_void) -> bool;

    // wxCollapsibleHeaderCtrl
    pub fn wxCollapsibleHeaderCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxCollapsibleHeaderCtrl_new() -> *mut c_void;
    pub fn wxCollapsibleHeaderCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxCollapsibleHeaderCtrl_Create(
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
    pub fn wxCollapsibleHeaderCtrl_SetCollapsed(self_: *mut c_void, collapsed: bool);
    pub fn wxCollapsibleHeaderCtrl_IsCollapsed(self_: *const c_void) -> bool;

    // wxCollapsiblePane
    pub fn wxCollapsiblePane_CLASSINFO() -> *mut c_void;
    pub fn wxCollapsiblePane_new() -> *mut c_void;
    pub fn wxCollapsiblePane_new1(
        parent: *mut c_void,
        id: c_int,
        label: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxCollapsiblePane_Create(
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
    pub fn wxCollapsiblePane_Collapse(self_: *mut c_void, collapse: bool);
    pub fn wxCollapsiblePane_Expand(self_: *mut c_void);
    pub fn wxCollapsiblePane_GetPane(self_: *const c_void) -> *mut c_void;
    pub fn wxCollapsiblePane_IsCollapsed(self_: *const c_void) -> bool;
    pub fn wxCollapsiblePane_IsExpanded(self_: *const c_void) -> bool;

    // wxCollapsiblePaneEvent
    pub fn wxCollapsiblePaneEvent_CLASSINFO() -> *mut c_void;
    pub fn wxCollapsiblePaneEvent_new(
        generator: *mut c_void,
        id: c_int,
        collapsed: bool,
    ) -> *mut c_void;
    pub fn wxCollapsiblePaneEvent_GetCollapsed(self_: *const c_void) -> bool;
    pub fn wxCollapsiblePaneEvent_SetCollapsed(self_: *mut c_void, collapsed: bool);

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

    // wxColourData
    pub fn wxColourData_CLASSINFO() -> *mut c_void;
    pub fn wxColourData_new() -> *mut c_void;
    // DTOR: pub fn wxColourData_~wxColourData(self_: *mut c_void);
    pub fn wxColourData_GetChooseFull(self_: *const c_void) -> bool;
    pub fn wxColourData_GetChooseAlpha(self_: *const c_void) -> bool;
    pub fn wxColourData_GetColour(self_: *mut c_void) -> *mut c_void;
    pub fn wxColourData_GetCustomColour(self_: *const c_void, i: c_int) -> *mut c_void;
    pub fn wxColourData_SetChooseFull(self_: *mut c_void, flag: bool);
    pub fn wxColourData_SetChooseAlpha(self_: *mut c_void, flag: bool);
    pub fn wxColourData_SetColour(self_: *mut c_void, colour: *const c_void);
    pub fn wxColourData_SetCustomColour(self_: *mut c_void, i: c_int, colour: *const c_void);
    pub fn wxColourData_ToString(self_: *const c_void) -> *mut c_void;
    pub fn wxColourData_FromString(self_: *mut c_void, str: *const c_void) -> bool;
    // BLOCKED: pub fn wxColourData_operator=(self_: *mut c_void, data: *const c_void) -> *mut c_void;

    // wxColourDatabase
    pub fn wxColourDatabase_delete(self_: *mut c_void);
    pub fn wxColourDatabase_new() -> *mut c_void;
    pub fn wxColourDatabase_AddColour(
        self_: *mut c_void,
        colour_name: *const c_void,
        colour: *const c_void,
    );
    pub fn wxColourDatabase_Find(self_: *const c_void, colour_name: *const c_void) -> *mut c_void;
    pub fn wxColourDatabase_FindName(self_: *const c_void, colour: *const c_void) -> *mut c_void;

    // wxColourDialog
    pub fn wxColourDialog_CLASSINFO() -> *mut c_void;
    pub fn wxColourDialog_new(parent: *mut c_void, data: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxColourDialog_~wxColourDialog(self_: *mut c_void);
    pub fn wxColourDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        data: *const c_void,
    ) -> bool;
    pub fn wxColourDialog_GetColourData(self_: *mut c_void) -> *mut c_void;

    // wxColourDialogEvent
    pub fn wxColourDialogEvent_CLASSINFO() -> *mut c_void;
    pub fn wxColourDialogEvent_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxColourDialogEvent_new1(evt_type: wxEventType, dialog: *mut c_void, colour: *const c_void) -> *mut c_void;
    pub fn wxColourDialogEvent_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxColourDialogEvent_SetColour(self_: *mut c_void, colour: *const c_void);

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

    // wxColourPickerEvent
    pub fn wxColourPickerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxColourPickerEvent_new() -> *mut c_void;
    pub fn wxColourPickerEvent_new1(
        generator: *mut c_void,
        id: c_int,
        colour: *const c_void,
    ) -> *mut c_void;
    pub fn wxColourPickerEvent_GetColour(self_: *const c_void) -> *mut c_void;
    pub fn wxColourPickerEvent_SetColour(self_: *mut c_void, pos: *const c_void);

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

    // wxComboCtrl
    pub fn wxComboCtrl_CLASSINFO() -> *mut c_void;
    pub fn wxComboCtrl_new() -> *mut c_void;
    pub fn wxComboCtrl_new1(
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxComboCtrl_~wxComboCtrl(self_: *mut c_void);
    pub fn wxComboCtrl_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        value: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxComboCtrl_Dismiss(self_: *mut c_void);
    pub fn wxComboCtrl_EnablePopupAnimation(self_: *mut c_void, enable: bool);
    pub fn wxComboCtrl_IsKeyPopupToggle(self_: *const c_void, event: *const c_void) -> bool;
    pub fn wxComboCtrl_PrepareBackground(
        self_: *const c_void,
        dc: *mut c_void,
        rect: *const c_void,
        flags: c_int,
    );
    pub fn wxComboCtrl_ShouldDrawFocus(self_: *const c_void) -> bool;
    pub fn wxComboCtrl_GetBitmapDisabled(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetBitmapHover(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetBitmapNormal(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetBitmapPressed(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetButtonSize(self_: *mut c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetCustomPaintWidth(self_: *const c_void) -> c_int;
    pub fn wxComboCtrl_GetMargins(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetPopupControl(self_: *mut c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetPopupWindow(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_GetTextCtrl(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxComboCtrl_GetTextIndent(self_: *const c_void) -> c_int;
    pub fn wxComboCtrl_GetTextRect(self_: *const c_void) -> *mut c_void;
    pub fn wxComboCtrl_HidePopup(self_: *mut c_void, generate_event: bool);
    pub fn wxComboCtrl_IsPopupShown(self_: *const c_void) -> bool;
    pub fn wxComboCtrl_IsPopupWindowState(self_: *const c_void, state: c_int) -> bool;
    pub fn wxComboCtrl_OnButtonClick(self_: *mut c_void);
    pub fn wxComboCtrl_Popup(self_: *mut c_void);
    pub fn wxComboCtrl_SetButtonBitmaps(
        self_: *mut c_void,
        bmp_normal: *const c_void,
        push_button_bg: bool,
        bmp_pressed: *const c_void,
        bmp_hover: *const c_void,
        bmp_disabled: *const c_void,
    );
    pub fn wxComboCtrl_SetButtonPosition(
        self_: *mut c_void,
        width: c_int,
        height: c_int,
        side: c_int,
        spacing_x: c_int,
    );
    pub fn wxComboCtrl_SetCustomPaintWidth(self_: *mut c_void, width: c_int);
    pub fn wxComboCtrl_SetMainControl(self_: *mut c_void, win: *mut c_void);
    pub fn wxComboCtrl_SetMargins(self_: *mut c_void, pt: *const c_void) -> bool;
    pub fn wxComboCtrl_SetMargins1(self_: *mut c_void, left: c_int, top: c_int) -> bool;
    pub fn wxComboCtrl_SetPopupAnchor(self_: *mut c_void, anchor_side: c_int);
    pub fn wxComboCtrl_SetPopupControl(self_: *mut c_void, popup: *mut c_void);
    pub fn wxComboCtrl_SetPopupExtents(self_: *mut c_void, ext_left: c_int, ext_right: c_int);
    pub fn wxComboCtrl_SetPopupMaxHeight(self_: *mut c_void, height: c_int);
    pub fn wxComboCtrl_SetPopupMinWidth(self_: *mut c_void, width: c_int);
    pub fn wxComboCtrl_SetText(self_: *mut c_void, value: *const c_void);
    pub fn wxComboCtrl_SetTextCtrlStyle(self_: *mut c_void, style: c_int);
    // BLOCKED: pub fn wxComboCtrl_SetTextIndent(self_: *mut c_void, indent: c_int);
    pub fn wxComboCtrl_SetValueByUser(self_: *mut c_void, value: *const c_void);
    pub fn wxComboCtrl_ShowPopup(self_: *mut c_void);
    pub fn wxComboCtrl_UseAltPopupWindow(self_: *mut c_void, enable: bool);
    pub fn wxComboCtrl_GetFeatures() -> c_int;
    // Mix-in(s) to wxComboCtrl
    pub fn wxComboCtrl_AsTextEntry(obj: *mut c_void) -> *mut c_void;

    // wxComboPopup
    pub fn wxComboPopup_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxComboPopup_new() -> *mut c_void;
    pub fn wxComboPopup_Create(self_: *mut c_void, parent: *mut c_void) -> bool;
    pub fn wxComboPopup_DestroyPopup(self_: *mut c_void);
    pub fn wxComboPopup_Dismiss(self_: *mut c_void);
    pub fn wxComboPopup_FindItem(
        self_: *mut c_void,
        item: *const c_void,
        true_item: *mut c_void,
    ) -> bool;
    pub fn wxComboPopup_GetAdjustedSize(
        self_: *mut c_void,
        min_width: c_int,
        pref_height: c_int,
        max_height: c_int,
    ) -> *mut c_void;
    pub fn wxComboPopup_GetComboCtrl(self_: *const c_void) -> *mut c_void;
    pub fn wxComboPopup_GetControl(self_: *mut c_void) -> *mut c_void;
    pub fn wxComboPopup_GetStringValue(self_: *const c_void) -> *mut c_void;
    pub fn wxComboPopup_Init(self_: *mut c_void);
    pub fn wxComboPopup_IsCreated(self_: *const c_void) -> bool;
    pub fn wxComboPopup_LazyCreate(self_: *mut c_void) -> bool;
    pub fn wxComboPopup_OnComboDoubleClick(self_: *mut c_void);
    pub fn wxComboPopup_OnComboKeyEvent(self_: *mut c_void, event: *mut c_void);
    pub fn wxComboPopup_OnDismiss(self_: *mut c_void);
    pub fn wxComboPopup_OnPopup(self_: *mut c_void);
    pub fn wxComboPopup_PaintComboControl(self_: *mut c_void, dc: *mut c_void, rect: *const c_void);
    pub fn wxComboPopup_SetStringValue(self_: *mut c_void, value: *const c_void);

    // wxCommand
    pub fn wxCommand_CLASSINFO() -> *mut c_void;
    // BLOCKED: pub fn wxCommand_new(can_undo: bool, name: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxCommand_~wxCommand(self_: *mut c_void);
    pub fn wxCommand_CanUndo(self_: *const c_void) -> bool;
    pub fn wxCommand_Do(self_: *mut c_void) -> bool;
    pub fn wxCommand_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxCommand_Undo(self_: *mut c_void) -> bool;

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

    // wxCommandLinkButton
    pub fn wxCommandLinkButton_CLASSINFO() -> *mut c_void;
    pub fn wxCommandLinkButton_new() -> *mut c_void;
    pub fn wxCommandLinkButton_new1(
        parent: *mut c_void,
        id: c_int,
        main_label: *const c_void,
        note: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> *mut c_void;
    pub fn wxCommandLinkButton_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        id: c_int,
        main_label: *const c_void,
        note: *const c_void,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
        validator: *const c_void,
        name: *const c_void,
    ) -> bool;
    pub fn wxCommandLinkButton_SetMainLabelAndNote(
        self_: *mut c_void,
        main_label: *const c_void,
        note: *const c_void,
    );
    pub fn wxCommandLinkButton_SetMainLabel(self_: *mut c_void, main_label: *const c_void);
    pub fn wxCommandLinkButton_SetNote(self_: *mut c_void, note: *const c_void);
    pub fn wxCommandLinkButton_GetMainLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandLinkButton_GetNote(self_: *const c_void) -> *mut c_void;

    // wxCommandProcessor
    pub fn wxCommandProcessor_CLASSINFO() -> *mut c_void;
    pub fn wxCommandProcessor_new(max_commands: c_int) -> *mut c_void;
    // DTOR: pub fn wxCommandProcessor_~wxCommandProcessor(self_: *mut c_void);
    pub fn wxCommandProcessor_CanUndo(self_: *const c_void) -> bool;
    pub fn wxCommandProcessor_CanRedo(self_: *const c_void) -> bool;
    pub fn wxCommandProcessor_ClearCommands(self_: *mut c_void);
    // BLOCKED: pub fn wxCommandProcessor_GetCommands(self_: *mut c_void) -> *mut c_void;
    pub fn wxCommandProcessor_GetCurrentCommand(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandProcessor_GetEditMenu(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandProcessor_GetMaxCommands(self_: *const c_void) -> c_int;
    pub fn wxCommandProcessor_GetRedoAccelerator(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandProcessor_GetRedoMenuLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandProcessor_GetUndoAccelerator(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandProcessor_GetUndoMenuLabel(self_: *const c_void) -> *mut c_void;
    pub fn wxCommandProcessor_Initialize(self_: *mut c_void);
    pub fn wxCommandProcessor_IsDirty(self_: *const c_void) -> bool;
    pub fn wxCommandProcessor_MarkAsSaved(self_: *mut c_void);
    pub fn wxCommandProcessor_Redo(self_: *mut c_void) -> bool;
    pub fn wxCommandProcessor_SetEditMenu(self_: *mut c_void, menu: *mut c_void);
    pub fn wxCommandProcessor_SetMenuStrings(self_: *mut c_void);
    pub fn wxCommandProcessor_SetRedoAccelerator(self_: *mut c_void, accel: *const c_void);
    pub fn wxCommandProcessor_SetUndoAccelerator(self_: *mut c_void, accel: *const c_void);
    pub fn wxCommandProcessor_Submit(
        self_: *mut c_void,
        command: *mut c_void,
        store_it: bool,
    ) -> bool;
    pub fn wxCommandProcessor_Store(self_: *mut c_void, command: *mut c_void);
    pub fn wxCommandProcessor_Undo(self_: *mut c_void) -> bool;

    // wxContextHelp
    pub fn wxContextHelp_CLASSINFO() -> *mut c_void;
    pub fn wxContextHelp_new(window: *mut c_void, do_now: bool) -> *mut c_void;
    // DTOR: pub fn wxContextHelp_~wxContextHelp(self_: *mut c_void);
    pub fn wxContextHelp_BeginContextHelp(self_: *mut c_void, window: *mut c_void) -> bool;
    pub fn wxContextHelp_EndContextHelp(self_: *mut c_void) -> bool;

    // wxContextHelpButton
    pub fn wxContextHelpButton_CLASSINFO() -> *mut c_void;
    pub fn wxContextHelpButton_new(
        parent: *mut c_void,
        id: c_int,
        pos: *const c_void,
        size: *const c_void,
        style: c_long,
    ) -> *mut c_void;

    // wxContextMenuEvent
    pub fn wxContextMenuEvent_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxContextMenuEvent_new(type_: wxEventType, id: c_int, pos: *const c_void) -> *mut c_void;
    pub fn wxContextMenuEvent_GetPosition(self_: *const c_void) -> *mut c_void;
    pub fn wxContextMenuEvent_SetPosition(self_: *mut c_void, point: *const c_void);

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

    // wxControlWithItems
    pub fn wxControlWithItems_CLASSINFO() -> *mut c_void;
    // Mix-in(s) to wxControlWithItems
    pub fn wxControlWithItems_AsItemContainer(obj: *mut c_void) -> *mut c_void;

    // wxCredentialEntryDialog
    pub fn wxCredentialEntryDialog_CLASSINFO() -> *mut c_void;
    pub fn wxCredentialEntryDialog_new() -> *mut c_void;
    pub fn wxCredentialEntryDialog_new1(
        parent: *mut c_void,
        message: *const c_void,
        title: *const c_void,
        cred: *const c_void,
    ) -> *mut c_void;
    pub fn wxCredentialEntryDialog_Create(
        self_: *mut c_void,
        parent: *mut c_void,
        message: *const c_void,
        title: *const c_void,
        cred: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxCredentialEntryDialog_GetCredentials(self_: *const c_void) -> wxWebCredentials;
    pub fn wxCredentialEntryDialog_SetUser(self_: *mut c_void, user: *const c_void);
    pub fn wxCredentialEntryDialog_SetPassword(self_: *mut c_void, password: *const c_void);

    // wxCursor
    pub fn wxCursor_CLASSINFO() -> *mut c_void;
    pub fn wxCursor_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCursor_new1(bits: char, width: c_int, height: c_int, hot_spot_x: c_int, hot_spot_y: c_int, mask_bits: char) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCursor_new2(cursor_name: *const c_void, type_: wxBitmapType, hot_spot_x: c_int, hot_spot_y: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCursor_new3(cursor_id: wxStockCursor) -> *mut c_void;
    pub fn wxCursor_new4(image: *const c_void) -> *mut c_void;
    pub fn wxCursor_new5(xpm_data: *const c_void) -> *mut c_void;
    pub fn wxCursor_new6(cursor: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxCursor_~wxCursor(self_: *mut c_void);
    pub fn wxCursor_IsOk(self_: *const c_void) -> bool;
    pub fn wxCursor_GetHotSpot(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxCursor_operator=(self_: *mut c_void, cursor: *const c_void) -> *mut c_void;

    // wxCustomDataObject
    pub fn wxCustomDataObject_delete(self_: *mut c_void);
    pub fn wxCustomDataObject_new(format: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxCustomDataObject_~wxCustomDataObject(self_: *mut c_void);
    pub fn wxCustomDataObject_Alloc(self_: *mut c_void, size: usize) -> *mut c_void;
    pub fn wxCustomDataObject_Free(self_: *mut c_void);
    pub fn wxCustomDataObject_GetData(self_: *const c_void) -> *mut c_void;
    pub fn wxCustomDataObject_GetSize(self_: *const c_void) -> usize;
    pub fn wxCustomDataObject_TakeData(self_: *mut c_void, size: usize, data: *mut c_void);

}
