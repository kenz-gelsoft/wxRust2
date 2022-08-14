use super::*;

// wxCalculateLayoutEvent
pub trait CalculateLayoutEventMethods: EventMethods {
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxCalculateLayoutEvent_GetFlags(self.as_ptr()) }
    }
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxCalculateLayoutEvent_GetRect(self.as_ptr())) }
    }
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxCalculateLayoutEvent_SetFlags(self.as_ptr(), flags) }
    }
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxCalculateLayoutEvent_SetRect(self.as_ptr(), rect)
        }
    }
}

// wxCalendarCtrl
pub trait CalendarCtrlMethods: ControlMethods {
    fn set_date_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        lowerdate: &D,
        upperdate: &D2,
    ) -> bool {
        unsafe {
            let lowerdate = lowerdate.as_ptr();
            let upperdate = upperdate.as_ptr();
            ffi::wxCalendarCtrl_SetDateRange(self.as_ptr(), lowerdate, upperdate)
        }
    }
    fn get_date_range<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        lowerdate: Option<&D>,
        upperdate: Option<&D2>,
    ) -> bool {
        unsafe {
            let lowerdate = match lowerdate {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let upperdate = match upperdate {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCalendarCtrl_GetDateRange(self.as_ptr(), lowerdate, upperdate)
        }
    }
    // DTOR: fn ~wxCalendarCtrl()
    fn create_datetime<W: WindowMethods, D: DateTimeMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        date: &D,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let date = date.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCalendarCtrl_Create(self.as_ptr(), parent, id, date, pos, size, style, name)
        }
    }
    fn enable_holiday_display(&self, display: bool) {
        unsafe { ffi::wxCalendarCtrl_EnableHolidayDisplay(self.as_ptr(), display) }
    }
    fn enable_month_change(&self, enable: bool) -> bool {
        unsafe { ffi::wxCalendarCtrl_EnableMonthChange(self.as_ptr(), enable) }
    }
    // BLOCKED: fn EnableYearChange()
    fn get_attr(&self, day: usize) -> Option<CalendarDateAttrIsOwned<false>> {
        unsafe { CalendarDateAttr::option_from(ffi::wxCalendarCtrl_GetAttr(self.as_ptr(), day)) }
    }
    fn get_date(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxCalendarCtrl_GetDate(self.as_ptr())) }
    }
    fn get_header_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourBg(self.as_ptr())) }
    }
    fn get_header_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourFg(self.as_ptr())) }
    }
    fn get_highlight_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourBg(self.as_ptr())) }
    }
    fn get_highlight_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourFg(self.as_ptr())) }
    }
    fn get_holiday_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourBg(self.as_ptr())) }
    }
    fn get_holiday_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourFg(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn HitTest()
    fn reset_attr(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_ResetAttr(self.as_ptr(), day) }
    }
    fn set_attr<C: CalendarDateAttrMethods>(&self, day: usize, attr: Option<&C>) {
        unsafe {
            let attr = match attr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCalendarCtrl_SetAttr(self.as_ptr(), day, attr)
        }
    }
    fn set_date<D: DateTimeMethods>(&self, date: &D) -> bool {
        unsafe {
            let date = date.as_ptr();
            ffi::wxCalendarCtrl_SetDate(self.as_ptr(), date)
        }
    }
    fn set_header_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHeaderColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    fn set_highlight_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHighlightColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    fn set_holiday(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_SetHoliday(self.as_ptr(), day) }
    }
    fn set_holiday_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHolidayColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    fn mark(&self, day: usize, mark: bool) {
        unsafe { ffi::wxCalendarCtrl_Mark(self.as_ptr(), day, mark) }
    }
}

// wxCalendarDateAttr
pub trait CalendarDateAttrMethods: WxRustMethods {
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    // NOT_SUPPORTED: fn GetBorder()
    fn get_border_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetBorderColour(self.as_ptr())) }
    }
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetFont(self.as_ptr())) }
    }
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetTextColour(self.as_ptr())) }
    }
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBackgroundColour(self.as_ptr()) }
    }
    fn has_border(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorder(self.as_ptr()) }
    }
    fn has_border_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorderColour(self.as_ptr()) }
    }
    fn has_font(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasFont(self.as_ptr()) }
    }
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasTextColour(self.as_ptr()) }
    }
    fn is_holiday(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_IsHoliday(self.as_ptr()) }
    }
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxCalendarDateAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    // NOT_SUPPORTED: fn SetBorder()
    fn set_border_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxCalendarDateAttr_SetBorderColour(self.as_ptr(), col)
        }
    }
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxCalendarDateAttr_SetFont(self.as_ptr(), font)
        }
    }
    fn set_holiday(&self, holiday: bool) {
        unsafe { ffi::wxCalendarDateAttr_SetHoliday(self.as_ptr(), holiday) }
    }
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxCalendarDateAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    fn get_mark() -> CalendarDateAttrIsOwned<false> {
        unsafe { CalendarDateAttrIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetMark()) }
    }
    fn set_mark<C: CalendarDateAttrMethods>(m: &C) {
        unsafe {
            let m = m.as_ptr();
            ffi::wxCalendarDateAttr_SetMark(m)
        }
    }
}

// wxCalendarEvent
pub trait CalendarEventMethods: DateEventMethods {
    // NOT_SUPPORTED: fn GetWeekDay()
    // NOT_SUPPORTED: fn SetWeekDay()
}

// wxCaret
pub trait CaretMethods: WxRustMethods {
    fn create_int<W: WindowMethods>(
        &self,
        window: Option<&W>,
        width: c_int,
        height: c_int,
    ) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCaret_Create(self.as_ptr(), window, width, height)
        }
    }
    fn create_size<W: WindowMethods, S: SizeMethods>(&self, window: Option<&W>, size: &S) -> bool {
        unsafe {
            let window = match window {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let size = size.as_ptr();
            ffi::wxCaret_Create1(self.as_ptr(), window, size)
        }
    }
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxCaret_GetPosition(self.as_ptr(), x, y) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCaret_GetPosition1(self.as_ptr())) }
    }
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxCaret_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxCaret_GetSize1(self.as_ptr())) }
    }
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCaret_GetWindow(self.as_ptr())) }
    }
    fn hide(&self) {
        unsafe { ffi::wxCaret_Hide(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCaret_IsOk(self.as_ptr()) }
    }
    fn is_visible(&self) -> bool {
        unsafe { ffi::wxCaret_IsVisible(self.as_ptr()) }
    }
    fn move_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxCaret_Move(self.as_ptr(), x, y) }
    }
    fn move_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxCaret_Move1(self.as_ptr(), pt)
        }
    }
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxCaret_SetSize(self.as_ptr(), width, height) }
    }
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxCaret_SetSize1(self.as_ptr(), size)
        }
    }
    fn show(&self, show: bool) {
        unsafe { ffi::wxCaret_Show(self.as_ptr(), show) }
    }
    fn get_blink_time() -> c_int {
        unsafe { ffi::wxCaret_GetBlinkTime() }
    }
    fn set_blink_time(milliseconds: c_int) {
        unsafe { ffi::wxCaret_SetBlinkTime(milliseconds) }
    }
}

// wxCheckBox
pub trait CheckBoxMethods: ControlMethods {
    // DTOR: fn ~wxCheckBox()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCheckBox_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_value(&self) -> bool {
        unsafe { ffi::wxCheckBox_GetValue(self.as_ptr()) }
    }
    fn get3_state_value(&self) -> c_int {
        unsafe { ffi::wxCheckBox_Get3StateValue(self.as_ptr()) }
    }
    fn is3_state(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3State(self.as_ptr()) }
    }
    fn is3rd_state_allowed_for_user(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3rdStateAllowedForUser(self.as_ptr()) }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCheckBox_IsChecked(self.as_ptr()) }
    }
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxCheckBox_SetValue(self.as_ptr(), state) }
    }
    fn set3_state_value(&self, state: c_int) {
        unsafe { ffi::wxCheckBox_Set3StateValue(self.as_ptr(), state) }
    }
}

// wxCheckListBox
pub trait CheckListBoxMethods: ListBoxMethods {
    // DTOR: fn ~wxCheckListBox()
    fn check(&self, item: c_uint, check: bool) {
        unsafe { ffi::wxCheckListBox_Check(self.as_ptr(), item, check) }
    }
    fn is_checked(&self, item: c_uint) -> bool {
        unsafe { ffi::wxCheckListBox_IsChecked(self.as_ptr(), item) }
    }
    fn get_checked_items<A: ArrayIntMethods>(&self, checked_items: &A) -> c_uint {
        unsafe {
            let checked_items = checked_items.as_ptr();
            ffi::wxCheckListBox_GetCheckedItems(self.as_ptr(), checked_items)
        }
    }
}

// wxChildFocusEvent
pub trait ChildFocusEventMethods: CommandEventMethods {
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxChildFocusEvent_GetWindow(self.as_ptr())) }
    }
}

// wxChoice
pub trait ChoiceMethods: ControlMethods {
    // DTOR: fn ~wxChoice()
    // NOT_SUPPORTED: fn Create()
    fn create_arraystring<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxChoice_Create1(
                self.as_ptr(),
                parent,
                id,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    fn get_columns(&self) -> c_int {
        unsafe { ffi::wxChoice_GetColumns(self.as_ptr()) }
    }
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxChoice_GetCurrentSelection(self.as_ptr()) }
    }
    fn set_columns(&self, n: c_int) {
        unsafe { ffi::wxChoice_SetColumns(self.as_ptr(), n) }
    }
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxChoice_IsSorted(self.as_ptr()) }
    }
}

// wxChoicebook
pub trait ChoicebookMethods: BookCtrlBaseMethods {
    fn get_choice_ctrl(&self) -> WeakRef<Choice> {
        unsafe { WeakRef::<Choice>::from(ffi::wxChoicebook_GetChoiceCtrl(self.as_ptr())) }
    }
}

// wxClipboard
pub trait ClipboardMethods: ObjectMethods {
    // DTOR: fn ~wxClipboard()
    fn add_data<D: DataObjectMethods>(&self, data: Option<&D>) -> bool {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClipboard_AddData(self.as_ptr(), data)
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxClipboard_Clear(self.as_ptr()) }
    }
    fn close(&self) {
        unsafe { ffi::wxClipboard_Close(self.as_ptr()) }
    }
    fn flush(&self) -> bool {
        unsafe { ffi::wxClipboard_Flush(self.as_ptr()) }
    }
    fn get_data<D: DataObjectMethods>(&self, data: &D) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxClipboard_GetData(self.as_ptr(), data)
        }
    }
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxClipboard_IsOpened(self.as_ptr()) }
    }
    fn is_supported(&self, format: *const c_void) -> bool {
        unsafe { ffi::wxClipboard_IsSupported(self.as_ptr(), format) }
    }
    fn is_using_primary_selection(&self) -> bool {
        unsafe { ffi::wxClipboard_IsUsingPrimarySelection(self.as_ptr()) }
    }
    fn open(&self) -> bool {
        unsafe { ffi::wxClipboard_Open(self.as_ptr()) }
    }
    fn set_data<D: DataObjectMethods>(&self, data: Option<&D>) -> bool {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClipboard_SetData(self.as_ptr(), data)
        }
    }
    fn use_primary_selection(&self, primary: bool) {
        unsafe { ffi::wxClipboard_UsePrimarySelection(self.as_ptr(), primary) }
    }
    fn get() -> Option<ClipboardIsOwned<false>> {
        unsafe { Clipboard::option_from(ffi::wxClipboard_Get()) }
    }
}

// wxClipboardTextEvent
pub trait ClipboardTextEventMethods: CommandEventMethods {}

// wxCloseEvent
pub trait CloseEventMethods: EventMethods {
    fn can_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_CanVeto(self.as_ptr()) }
    }
    fn get_logging_off(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetLoggingOff(self.as_ptr()) }
    }
    fn set_can_veto(&self, can_veto: bool) {
        unsafe { ffi::wxCloseEvent_SetCanVeto(self.as_ptr(), can_veto) }
    }
    fn set_logging_off(&self, logging_off: bool) {
        unsafe { ffi::wxCloseEvent_SetLoggingOff(self.as_ptr(), logging_off) }
    }
    fn veto(&self, veto: bool) {
        unsafe { ffi::wxCloseEvent_Veto(self.as_ptr(), veto) }
    }
    fn get_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetVeto(self.as_ptr()) }
    }
}

// wxCollapsiblePane
pub trait CollapsiblePaneMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCollapsiblePane_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn collapse(&self, collapse: bool) {
        unsafe { ffi::wxCollapsiblePane_Collapse(self.as_ptr(), collapse) }
    }
    fn expand(&self) {
        unsafe { ffi::wxCollapsiblePane_Expand(self.as_ptr()) }
    }
    fn get_pane(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCollapsiblePane_GetPane(self.as_ptr())) }
    }
    fn is_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsCollapsed(self.as_ptr()) }
    }
    fn is_expanded(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsExpanded(self.as_ptr()) }
    }
}

// wxCollapsiblePaneEvent
pub trait CollapsiblePaneEventMethods: CommandEventMethods {
    fn get_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePaneEvent_GetCollapsed(self.as_ptr()) }
    }
    fn set_collapsed(&self, collapsed: bool) {
        unsafe { ffi::wxCollapsiblePaneEvent_SetCollapsed(self.as_ptr(), collapsed) }
    }
}

// wxColour
pub trait ColourMethods: ObjectMethods {
    // NOT_SUPPORTED: fn Alpha()
    // NOT_SUPPORTED: fn Blue()
    fn get_alpha(&self) -> c_uint {
        unsafe { ffi::wxColour_GetAlpha(self.as_ptr()) }
    }
    fn get_blue(&self) -> c_uint {
        unsafe { ffi::wxColour_GetBlue(self.as_ptr()) }
    }
    fn get_green(&self) -> c_uint {
        unsafe { ffi::wxColour_GetGreen(self.as_ptr()) }
    }
    fn get_red(&self) -> c_uint {
        unsafe { ffi::wxColour_GetRed(self.as_ptr()) }
    }
    fn get_as_string(&self, flags: c_long) -> String {
        unsafe { WxString::from_ptr(ffi::wxColour_GetAsString(self.as_ptr(), flags)).into() }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGBA()
    // NOT_SUPPORTED: fn GetRGB()
    // NOT_SUPPORTED: fn GetRGBA()
    fn get_luminance(&self) -> c_double {
        unsafe { ffi::wxColour_GetLuminance(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    // NOT_SUPPORTED: fn Green()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxColour_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Red()
    fn is_solid(&self) -> bool {
        unsafe { ffi::wxColour_IsSolid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Set()
    // NOT_SUPPORTED: fn Set1()
    fn set(&self, str: &str) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxColour_Set2(self.as_ptr(), str)
        }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator==()
    // NOT_SUPPORTED: fn MakeDisabled()
    // BLOCKED: fn ChangeLightness()
    fn make_mono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool) {
        unsafe { ffi::wxColour_MakeMono(r, g, b, on) }
    }
    // NOT_SUPPORTED: fn MakeDisabled1()
    fn make_grey(r: *mut c_void, g: *mut c_void, b: *mut c_void) {
        unsafe { ffi::wxColour_MakeGrey(r, g, b) }
    }
    fn make_grey_double(
        r: *mut c_void,
        g: *mut c_void,
        b: *mut c_void,
        weight_r: c_double,
        weight_g: c_double,
        weight_b: c_double,
    ) {
        unsafe { ffi::wxColour_MakeGrey1(r, g, b, weight_r, weight_g, weight_b) }
    }
    // NOT_SUPPORTED: fn AlphaBlend()
    fn change_lightness(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int) {
        unsafe { ffi::wxColour_ChangeLightness1(r, g, b, ialpha) }
    }
}

// wxColourData
pub trait ColourDataMethods: ObjectMethods {
    // DTOR: fn ~wxColourData()
    fn get_choose_full(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseFull(self.as_ptr()) }
    }
    fn get_choose_alpha(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseAlpha(self.as_ptr()) }
    }
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxColourData_GetColour(self.as_ptr())) }
    }
    fn get_custom_colour(&self, i: c_int) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourData_GetCustomColour(self.as_ptr(), i)) }
    }
    fn set_choose_full(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseFull(self.as_ptr(), flag) }
    }
    fn set_choose_alpha(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseAlpha(self.as_ptr(), flag) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetColour(self.as_ptr(), colour)
        }
    }
    fn set_custom_colour<C: ColourMethods>(&self, i: c_int, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetCustomColour(self.as_ptr(), i, colour)
        }
    }
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxColourData_ToString(self.as_ptr())).into() }
    }
    fn from_string(&self, str: &str) -> bool {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            ffi::wxColourData_FromString(self.as_ptr(), str)
        }
    }
    // BLOCKED: fn operator=()
}

// wxColourDatabase
pub trait ColourDatabaseMethods: WxRustMethods {
    fn add_colour<C: ColourMethods>(&self, colour_name: &str, colour: &C) {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxColourDatabase_AddColour(self.as_ptr(), colour_name, colour)
        }
    }
    fn find(&self, colour_name: &str) -> Colour {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            Colour::from_ptr(ffi::wxColourDatabase_Find(self.as_ptr(), colour_name))
        }
    }
    fn find_name<C: ColourMethods>(&self, colour: &C) -> String {
        unsafe {
            let colour = colour.as_ptr();
            WxString::from_ptr(ffi::wxColourDatabase_FindName(self.as_ptr(), colour)).into()
        }
    }
}

// wxColourDialog
pub trait ColourDialogMethods: DialogMethods {
    // DTOR: fn ~wxColourDialog()
    fn create_colourdata<W: WindowMethods, C: ColourDataMethods>(
        &self,
        parent: Option<&W>,
        data: Option<&C>,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxColourDialog_Create(self.as_ptr(), parent, data)
        }
    }
    fn get_colour_data(&self) -> ColourDataIsOwned<false> {
        unsafe { ColourDataIsOwned::from_ptr(ffi::wxColourDialog_GetColourData(self.as_ptr())) }
    }
}

// wxColourDialogEvent
pub trait ColourDialogEventMethods: CommandEventMethods {
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourDialogEvent_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourDialogEvent_SetColour(self.as_ptr(), colour)
        }
    }
}

// wxColourPickerCtrl
pub trait ColourPickerCtrlMethods: PickerBaseMethods {
    fn create_colour<
        W: WindowMethods,
        C: ColourMethods,
        P: PointMethods,
        S: SizeMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        colour: &C,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let colour = colour.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxColourPickerCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                colour,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerCtrl_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxColourPickerCtrl_SetColour(self.as_ptr(), col)
        }
    }
    // BLOCKED: fn SetColour1()
}

// wxColourPickerEvent
pub trait ColourPickerEventMethods: CommandEventMethods {
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerEvent_GetColour(self.as_ptr())) }
    }
    fn set_colour<C: ColourMethods>(&self, pos: &C) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxColourPickerEvent_SetColour(self.as_ptr(), pos)
        }
    }
}

// wxComboBox
pub trait ComboBoxMethods: ControlMethods {
    // DTOR: fn ~wxComboBox()
    // NOT_SUPPORTED: fn Create()
    fn create_str<
        W: WindowMethods,
        P: PointMethods,
        S: SizeMethods,
        A: ArrayStringMethods,
        V: ValidatorMethods,
    >(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        choices: &A,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let choices = choices.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxComboBox_Create1(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                choices,
                style,
                validator,
                name,
            )
        }
    }
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxComboBox_GetCurrentSelection(self.as_ptr()) }
    }
    fn is_list_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsListEmpty(self.as_ptr()) }
    }
    fn is_text_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsTextEmpty(self.as_ptr()) }
    }
    fn popup(&self) {
        unsafe { ffi::wxComboBox_Popup(self.as_ptr()) }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxComboBox_Dismiss(self.as_ptr()) }
    }
}

// wxComboCtrl
pub trait ComboCtrlMethods: ControlMethods {
    // DTOR: fn ~wxComboCtrl()
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        value: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let value = WxString::from(value);
            let value = value.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxComboCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                value,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxComboCtrl_Dismiss(self.as_ptr()) }
    }
    fn enable_popup_animation(&self, enable: bool) {
        unsafe { ffi::wxComboCtrl_EnablePopupAnimation(self.as_ptr(), enable) }
    }
    fn is_key_popup_toggle(&self, event: *const c_void) -> bool {
        unsafe { ffi::wxComboCtrl_IsKeyPopupToggle(self.as_ptr(), event) }
    }
    fn prepare_background<R: RectMethods>(&self, dc: *mut c_void, rect: &R, flags: c_int) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxComboCtrl_PrepareBackground(self.as_ptr(), dc, rect, flags)
        }
    }
    fn should_draw_focus(&self) -> bool {
        unsafe { ffi::wxComboCtrl_ShouldDrawFocus(self.as_ptr()) }
    }
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapDisabled(self.as_ptr())) }
    }
    fn get_bitmap_hover(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapHover(self.as_ptr())) }
    }
    fn get_bitmap_normal(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapNormal(self.as_ptr())) }
    }
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapPressed(self.as_ptr())) }
    }
    fn get_button_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxComboCtrl_GetButtonSize(self.as_ptr())) }
    }
    fn get_custom_paint_width(&self) -> c_int {
        unsafe { ffi::wxComboCtrl_GetCustomPaintWidth(self.as_ptr()) }
    }
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxComboCtrl_GetMargins(self.as_ptr())) }
    }
    fn get_popup_control(&self) -> Option<ComboPopupIsOwned<false>> {
        unsafe { ComboPopup::option_from(ffi::wxComboCtrl_GetPopupControl(self.as_ptr())) }
    }
    fn get_popup_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxComboCtrl_GetPopupWindow(self.as_ptr())) }
    }
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxComboCtrl_GetTextCtrl(self.as_ptr())) }
    }
    // BLOCKED: fn GetTextIndent()
    fn get_text_rect(&self) -> RectIsOwned<false> {
        unsafe { RectIsOwned::from_ptr(ffi::wxComboCtrl_GetTextRect(self.as_ptr())) }
    }
    fn hide_popup(&self, generate_event: bool) {
        unsafe { ffi::wxComboCtrl_HidePopup(self.as_ptr(), generate_event) }
    }
    fn is_popup_shown(&self) -> bool {
        unsafe { ffi::wxComboCtrl_IsPopupShown(self.as_ptr()) }
    }
    fn is_popup_window_state(&self, state: c_int) -> bool {
        unsafe { ffi::wxComboCtrl_IsPopupWindowState(self.as_ptr(), state) }
    }
    fn on_button_click(&self) {
        unsafe { ffi::wxComboCtrl_OnButtonClick(self.as_ptr()) }
    }
    fn popup(&self) {
        unsafe { ffi::wxComboCtrl_Popup(self.as_ptr()) }
    }
    fn set_button_bitmaps<
        B: BitmapBundleMethods,
        B2: BitmapBundleMethods,
        B3: BitmapBundleMethods,
        B4: BitmapBundleMethods,
    >(
        &self,
        bmp_normal: &B,
        push_button_bg: bool,
        bmp_pressed: &B2,
        bmp_hover: &B3,
        bmp_disabled: &B4,
    ) {
        unsafe {
            let bmp_normal = bmp_normal.as_ptr();
            let bmp_pressed = bmp_pressed.as_ptr();
            let bmp_hover = bmp_hover.as_ptr();
            let bmp_disabled = bmp_disabled.as_ptr();
            ffi::wxComboCtrl_SetButtonBitmaps(
                self.as_ptr(),
                bmp_normal,
                push_button_bg,
                bmp_pressed,
                bmp_hover,
                bmp_disabled,
            )
        }
    }
    fn set_button_position(&self, width: c_int, height: c_int, side: c_int, spacing_x: c_int) {
        unsafe { ffi::wxComboCtrl_SetButtonPosition(self.as_ptr(), width, height, side, spacing_x) }
    }
    fn set_custom_paint_width(&self, width: c_int) {
        unsafe { ffi::wxComboCtrl_SetCustomPaintWidth(self.as_ptr(), width) }
    }
    fn set_main_control<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxComboCtrl_SetMainControl(self.as_ptr(), win)
        }
    }
    fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxComboCtrl_SetMargins(self.as_ptr(), pt)
        }
    }
    fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
        unsafe { ffi::wxComboCtrl_SetMargins1(self.as_ptr(), left, top) }
    }
    fn set_popup_anchor(&self, anchor_side: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupAnchor(self.as_ptr(), anchor_side) }
    }
    fn set_popup_control<C: ComboPopupMethods>(&self, popup: Option<&C>) {
        unsafe {
            let popup = match popup {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxComboCtrl_SetPopupControl(self.as_ptr(), popup)
        }
    }
    fn set_popup_extents(&self, ext_left: c_int, ext_right: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupExtents(self.as_ptr(), ext_left, ext_right) }
    }
    fn set_popup_max_height(&self, height: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupMaxHeight(self.as_ptr(), height) }
    }
    fn set_popup_min_width(&self, width: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupMinWidth(self.as_ptr(), width) }
    }
    fn set_text(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboCtrl_SetText(self.as_ptr(), value)
        }
    }
    fn set_text_ctrl_style(&self, style: c_int) {
        unsafe { ffi::wxComboCtrl_SetTextCtrlStyle(self.as_ptr(), style) }
    }
    // BLOCKED: fn SetTextIndent()
    fn set_value_by_user(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboCtrl_SetValueByUser(self.as_ptr(), value)
        }
    }
    fn show_popup(&self) {
        unsafe { ffi::wxComboCtrl_ShowPopup(self.as_ptr()) }
    }
    fn use_alt_popup_window(&self, enable: bool) {
        unsafe { ffi::wxComboCtrl_UseAltPopupWindow(self.as_ptr(), enable) }
    }
    fn get_features() -> c_int {
        unsafe { ffi::wxComboCtrl_GetFeatures() }
    }
}

// wxComboPopup
pub trait ComboPopupMethods: WxRustMethods {
    fn create<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxComboPopup_Create(self.as_ptr(), parent)
        }
    }
    fn destroy_popup(&self) {
        unsafe { ffi::wxComboPopup_DestroyPopup(self.as_ptr()) }
    }
    fn dismiss(&self) {
        unsafe { ffi::wxComboPopup_Dismiss(self.as_ptr()) }
    }
    fn find_item(&self, item: &str, true_item: *mut c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxComboPopup_FindItem(self.as_ptr(), item, true_item)
        }
    }
    fn get_adjusted_size(&self, min_width: c_int, pref_height: c_int, max_height: c_int) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxComboPopup_GetAdjustedSize(
                self.as_ptr(),
                min_width,
                pref_height,
                max_height,
            ))
        }
    }
    fn get_combo_ctrl(&self) -> WeakRef<ComboCtrl> {
        unsafe { WeakRef::<ComboCtrl>::from(ffi::wxComboPopup_GetComboCtrl(self.as_ptr())) }
    }
    fn get_control(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxComboPopup_GetControl(self.as_ptr())) }
    }
    fn get_string_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxComboPopup_GetStringValue(self.as_ptr())).into() }
    }
    fn init(&self) {
        unsafe { ffi::wxComboPopup_Init(self.as_ptr()) }
    }
    fn is_created(&self) -> bool {
        unsafe { ffi::wxComboPopup_IsCreated(self.as_ptr()) }
    }
    fn lazy_create(&self) -> bool {
        unsafe { ffi::wxComboPopup_LazyCreate(self.as_ptr()) }
    }
    fn on_combo_double_click(&self) {
        unsafe { ffi::wxComboPopup_OnComboDoubleClick(self.as_ptr()) }
    }
    fn on_combo_key_event(&self, event: *mut c_void) {
        unsafe { ffi::wxComboPopup_OnComboKeyEvent(self.as_ptr(), event) }
    }
    fn on_dismiss(&self) {
        unsafe { ffi::wxComboPopup_OnDismiss(self.as_ptr()) }
    }
    fn on_popup(&self) {
        unsafe { ffi::wxComboPopup_OnPopup(self.as_ptr()) }
    }
    fn paint_combo_control<R: RectMethods>(&self, dc: *mut c_void, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxComboPopup_PaintComboControl(self.as_ptr(), dc, rect)
        }
    }
    fn set_string_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboPopup_SetStringValue(self.as_ptr(), value)
        }
    }
}

// wxCommand
pub trait CommandMethods: ObjectMethods {
    // DTOR: fn ~wxCommand()
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxCommand_CanUndo(self.as_ptr()) }
    }
    fn do_(&self) -> bool {
        unsafe { ffi::wxCommand_Do(self.as_ptr()) }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommand_GetName(self.as_ptr())).into() }
    }
    fn undo(&self) -> bool {
        unsafe { ffi::wxCommand_Undo(self.as_ptr()) }
    }
}

// wxCommandEvent
pub trait CommandEventMethods: EventMethods {
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientData(self.as_ptr()) }
    }
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientObject(self.as_ptr()) }
    }
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxCommandEvent_GetExtraLong(self.as_ptr()) }
    }
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetInt(self.as_ptr()) }
    }
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetSelection(self.as_ptr()) }
    }
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandEvent_GetString(self.as_ptr())).into() }
    }
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsChecked(self.as_ptr()) }
    }
    fn is_selection(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsSelection(self.as_ptr()) }
    }
    fn set_client_data(&self, client_data: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientData(self.as_ptr(), client_data) }
    }
    fn set_client_object(&self, client_object: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientObject(self.as_ptr(), client_object) }
    }
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxCommandEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxCommandEvent_SetInt(self.as_ptr(), int_command) }
    }
    fn set_string(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxCommandEvent_SetString(self.as_ptr(), string)
        }
    }
}

// wxCommandLinkButton
pub trait CommandLinkButtonMethods: ButtonMethods {
    fn create_str_str<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        main_label: &str,
        note: &str,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxCommandLinkButton_Create(
                self.as_ptr(),
                parent,
                id,
                main_label,
                note,
                pos,
                size,
                style,
                validator,
                name,
            )
        }
    }
    fn set_main_label_and_note(&self, main_label: &str, note: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabelAndNote(self.as_ptr(), main_label, note)
        }
    }
    fn set_main_label(&self, main_label: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabel(self.as_ptr(), main_label)
        }
    }
    fn set_note(&self, note: &str) {
        unsafe {
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetNote(self.as_ptr(), note)
        }
    }
    fn get_main_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetMainLabel(self.as_ptr())).into() }
    }
    fn get_note(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetNote(self.as_ptr())).into() }
    }
}

// wxCommandProcessor
pub trait CommandProcessorMethods: ObjectMethods {
    // DTOR: fn ~wxCommandProcessor()
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_CanUndo(self.as_ptr()) }
    }
    fn can_redo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_CanRedo(self.as_ptr()) }
    }
    fn clear_commands(&self) {
        unsafe { ffi::wxCommandProcessor_ClearCommands(self.as_ptr()) }
    }
    // BLOCKED: fn GetCommands()
    fn get_current_command(&self) -> Option<CommandIsOwned<false>> {
        unsafe { Command::option_from(ffi::wxCommandProcessor_GetCurrentCommand(self.as_ptr())) }
    }
    fn get_edit_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxCommandProcessor_GetEditMenu(self.as_ptr())) }
    }
    fn get_max_commands(&self) -> c_int {
        unsafe { ffi::wxCommandProcessor_GetMaxCommands(self.as_ptr()) }
    }
    fn get_redo_accelerator(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetRedoAccelerator(self.as_ptr())).into()
        }
    }
    fn get_redo_menu_label(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetRedoMenuLabel(self.as_ptr())).into()
        }
    }
    fn get_undo_accelerator(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetUndoAccelerator(self.as_ptr())).into()
        }
    }
    fn get_undo_menu_label(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetUndoMenuLabel(self.as_ptr())).into()
        }
    }
    fn initialize(&self) {
        unsafe { ffi::wxCommandProcessor_Initialize(self.as_ptr()) }
    }
    fn is_dirty(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_IsDirty(self.as_ptr()) }
    }
    fn mark_as_saved(&self) {
        unsafe { ffi::wxCommandProcessor_MarkAsSaved(self.as_ptr()) }
    }
    fn redo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_Redo(self.as_ptr()) }
    }
    fn set_edit_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandProcessor_SetEditMenu(self.as_ptr(), menu)
        }
    }
    fn set_menu_strings(&self) {
        unsafe { ffi::wxCommandProcessor_SetMenuStrings(self.as_ptr()) }
    }
    fn set_redo_accelerator(&self, accel: &str) {
        unsafe {
            let accel = WxString::from(accel);
            let accel = accel.as_ptr();
            ffi::wxCommandProcessor_SetRedoAccelerator(self.as_ptr(), accel)
        }
    }
    fn set_undo_accelerator(&self, accel: &str) {
        unsafe {
            let accel = WxString::from(accel);
            let accel = accel.as_ptr();
            ffi::wxCommandProcessor_SetUndoAccelerator(self.as_ptr(), accel)
        }
    }
    fn submit<C: CommandMethods>(&self, command: Option<&C>, store_it: bool) -> bool {
        unsafe {
            let command = match command {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandProcessor_Submit(self.as_ptr(), command, store_it)
        }
    }
    fn store<C: CommandMethods>(&self, command: Option<&C>) {
        unsafe {
            let command = match command {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandProcessor_Store(self.as_ptr(), command)
        }
    }
    fn undo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_Undo(self.as_ptr()) }
    }
}

// wxContextMenuEvent
pub trait ContextMenuEventMethods: CommandEventMethods {
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxContextMenuEvent_GetPosition(self.as_ptr())) }
    }
    fn set_position<P: PointMethods>(&self, point: &P) {
        unsafe {
            let point = point.as_ptr();
            ffi::wxContextMenuEvent_SetPosition(self.as_ptr(), point)
        }
    }
}

// wxControl
pub trait ControlMethods: WindowMethods {
    fn create_validator<W: WindowMethods, P: PointMethods, S: SizeMethods, V: ValidatorMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        validator: &V,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let validator = validator.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxControl_Create(self.as_ptr(), parent, id, pos, size, style, validator, name)
        }
    }
    fn command<C: CommandEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxControl_Command(self.as_ptr(), event)
        }
    }
    fn get_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxControl_GetLabelText(self.as_ptr())).into() }
    }
    fn get_size_from_text_size_int(&self, xlen: c_int, ylen: c_int) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize(
                self.as_ptr(),
                xlen,
                ylen,
            ))
        }
    }
    fn get_size_from_text_size_size<S: SizeMethods>(&self, tsize: &S) -> Size {
        unsafe {
            let tsize = tsize.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr(), tsize))
        }
    }
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromText(self.as_ptr(), text))
        }
    }
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxControl_SetLabelText(self.as_ptr(), text)
        }
    }
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = WxString::from(markup);
            let markup = markup.as_ptr();
            ffi::wxControl_SetLabelMarkup(self.as_ptr(), markup)
        }
    }
    fn get_label_text_str(label: &str) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WxString::from_ptr(ffi::wxControl_GetLabelText1(label)).into()
        }
    }
    fn remove_mnemonics(str: &str) -> String {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            WxString::from_ptr(ffi::wxControl_RemoveMnemonics(str)).into()
        }
    }
    fn escape_mnemonics(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxControl_EscapeMnemonics(text)).into()
        }
    }
    fn ellipsize(
        label: &str,
        dc: *const c_void,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WxString::from_ptr(ffi::wxControl_Ellipsize(label, dc, mode, max_width, flags)).into()
        }
    }
}

// wxControlWithItems
pub trait ControlWithItemsMethods: ControlMethods {}

// wxCredentialEntryDialog
pub trait CredentialEntryDialogMethods: DialogMethods {
    fn create_str<W: WindowMethods>(
        &self,
        parent: Option<&W>,
        message: &str,
        title: &str,
        cred: *const c_void,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxCredentialEntryDialog_Create(self.as_ptr(), parent, message, title, cred)
        }
    }
    // NOT_SUPPORTED: fn GetCredentials()
    fn set_user(&self, user: &str) {
        unsafe {
            let user = WxString::from(user);
            let user = user.as_ptr();
            ffi::wxCredentialEntryDialog_SetUser(self.as_ptr(), user)
        }
    }
    fn set_password(&self, password: &str) {
        unsafe {
            let password = WxString::from(password);
            let password = password.as_ptr();
            ffi::wxCredentialEntryDialog_SetPassword(self.as_ptr(), password)
        }
    }
}

// wxCursor
pub trait CursorMethods: GDIObjectMethods {
    // DTOR: fn ~wxCursor()
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCursor_IsOk(self.as_ptr()) }
    }
    fn get_hot_spot(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCursor_GetHotSpot(self.as_ptr())) }
    }
    // BLOCKED: fn operator=()
}

// wxCustomDataObject
pub trait CustomDataObjectMethods: DataObjectSimpleMethods {
    // DTOR: fn ~wxCustomDataObject()
    fn alloc(&self, size: usize) -> *mut c_void {
        unsafe { ffi::wxCustomDataObject_Alloc(self.as_ptr(), size) }
    }
    fn free(&self) {
        unsafe { ffi::wxCustomDataObject_Free(self.as_ptr()) }
    }
    fn get_data(&self) -> *mut c_void {
        unsafe { ffi::wxCustomDataObject_GetData(self.as_ptr()) }
    }
    fn get_size(&self) -> usize {
        unsafe { ffi::wxCustomDataObject_GetSize(self.as_ptr()) }
    }
    fn take_data(&self, size: usize, data: *mut c_void) {
        unsafe { ffi::wxCustomDataObject_TakeData(self.as_ptr(), size, data) }
    }
}
