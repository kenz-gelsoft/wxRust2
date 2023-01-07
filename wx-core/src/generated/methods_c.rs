use super::*;

// wxCalculateLayoutEvent
/// This trait represents [C++ `wxCalculateLayoutEvent` class](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html)'s methods and inheritance.
///
/// See [`CalculateLayoutEventInRust`] documentation for the class usage.
pub trait CalculateLayoutEventMethods: EventMethods {
    /// Returns the flags associated with this event.
    ///
    /// See [C++ `wxCalculateLayoutEvent::GetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a3131a5223b43cdccaa514830a6d80c1e).
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxCalculateLayoutEvent_GetFlags(self.as_ptr()) }
    }
    /// Before the event handler is entered, returns the remaining parent client area that the window could occupy.
    ///
    /// See [C++ `wxCalculateLayoutEvent::GetRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a667b48fa98605b93d7929fe9f001f8ec).
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxCalculateLayoutEvent_GetRect(self.as_ptr())) }
    }
    /// Sets the flags associated with this event.
    ///
    /// See [C++ `wxCalculateLayoutEvent::SetFlags()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a004b0f2a708dd3ff682d50492e7863a8).
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxCalculateLayoutEvent_SetFlags(self.as_ptr(), flags) }
    }
    /// Call this to specify the new remaining parent client area, after the space occupied by the window has been subtracted.
    ///
    /// See [C++ `wxCalculateLayoutEvent::SetRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calculate_layout_event.html#a471737747c444846393309de6e24eabc).
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxCalculateLayoutEvent_SetRect(self.as_ptr(), rect)
        }
    }
}

// wxCalendarCtrl
/// This trait represents [C++ `wxCalendarCtrl` class](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html)'s methods and inheritance.
///
/// See [`CalendarCtrlInRust`] documentation for the class usage.
pub trait CalendarCtrlMethods: ControlMethods {
    /// Restrict the dates that can be selected in the control to the specified range.
    ///
    /// See [C++ `wxCalendarCtrl::SetDateRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a79094f14e7b500012099907dea5f3211).
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
    /// Returns the limits currently being used.
    ///
    /// See [C++ `wxCalendarCtrl::GetDateRange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ab35b63b845a16ba3861e73fe881b64d1).
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
    /// Creates the control.
    ///
    /// See [C++ `wxCalendarCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#aad5c70c750a9f9df456fa8402f7adb7e).
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
    /// This function should be used instead of changing wxCAL_SHOW_HOLIDAYS style bit directly.
    ///
    /// See [C++ `wxCalendarCtrl::EnableHolidayDisplay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ab357bcc9d60dda85a331511bb303228d).
    fn enable_holiday_display(&self, display: bool) {
        unsafe { ffi::wxCalendarCtrl_EnableHolidayDisplay(self.as_ptr(), display) }
    }
    /// This function should be used instead of changing wxCAL_NO_MONTH_CHANGE style bit.
    ///
    /// See [C++ `wxCalendarCtrl::EnableMonthChange()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a201fb9498116d7e02f853b75d3e300e8).
    fn enable_month_change(&self, enable: bool) -> bool {
        unsafe { ffi::wxCalendarCtrl_EnableMonthChange(self.as_ptr(), enable) }
    }
    // BLOCKED: fn EnableYearChange()
    /// Returns the attribute for the given date (should be in the range 1...31).
    ///
    /// See [C++ `wxCalendarCtrl::GetAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#afa23290a3f6bbec4496578801e72c159).
    fn get_attr(&self, day: usize) -> Option<CalendarDateAttrInRust<false>> {
        unsafe { CalendarDateAttr::option_from(ffi::wxCalendarCtrl_GetAttr(self.as_ptr(), day)) }
    }
    /// Gets the currently selected date.
    ///
    /// See [C++ `wxCalendarCtrl::GetDate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a419a268760a3d428659ff3efb213b8cf).
    fn get_date(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxCalendarCtrl_GetDate(self.as_ptr())) }
    }
    /// Gets the background colour of the header part of the calendar window.
    ///
    /// See [C++ `wxCalendarCtrl::GetHeaderColourBg()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a3db6a6be0d2cec679acabd7e095779e1).
    fn get_header_colour_bg(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourBg(self.as_ptr())) }
    }
    /// Gets the foreground colour of the header part of the calendar window.
    ///
    /// See [C++ `wxCalendarCtrl::GetHeaderColourFg()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ace569b6138e8c37516f214b5492bfb03).
    fn get_header_colour_fg(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourFg(self.as_ptr())) }
    }
    /// Gets the background highlight colour.
    ///
    /// See [C++ `wxCalendarCtrl::GetHighlightColourBg()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a59a2dd56e4ffd96e1b9279cf965cf8af).
    fn get_highlight_colour_bg(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourBg(self.as_ptr())) }
    }
    /// Gets the foreground highlight colour.
    ///
    /// See [C++ `wxCalendarCtrl::GetHighlightColourFg()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ad69b653c2bc19eb17b95ad4e1bb8cdd2).
    fn get_highlight_colour_fg(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourFg(self.as_ptr())) }
    }
    /// Return the background colour currently used for holiday highlighting.
    ///
    /// See [C++ `wxCalendarCtrl::GetHolidayColourBg()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a5bfcd8b7c80b83693aec3a5584c18faa).
    fn get_holiday_colour_bg(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourBg(self.as_ptr())) }
    }
    /// Return the foreground colour currently used for holiday highlighting.
    ///
    /// See [C++ `wxCalendarCtrl::GetHolidayColourFg()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a24bb3a3f41f6a4e0059ea5a1cccb6eb8).
    fn get_holiday_colour_fg(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourFg(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn HitTest()
    /// Clears any attributes associated with the given day (in the range 1...31).
    ///
    /// See [C++ `wxCalendarCtrl::ResetAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#afc59e68f03a92e602822dbec608aece8).
    fn reset_attr(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_ResetAttr(self.as_ptr(), day) }
    }
    /// Associates the attribute with the specified date (in the range 1...31).
    ///
    /// See [C++ `wxCalendarCtrl::SetAttr()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#abeded70d9cfa8d42bc7f6f6610c40a15).
    fn set_attr<C: CalendarDateAttrMethods>(&self, day: usize, attr: Option<&C>) {
        unsafe {
            let attr = match attr {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCalendarCtrl_SetAttr(self.as_ptr(), day, attr)
        }
    }
    /// Sets the current date.
    ///
    /// See [C++ `wxCalendarCtrl::SetDate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#ae10603c5b221e24665bdff196edbfb80).
    fn set_date<D: DateTimeMethods>(&self, date: &D) -> bool {
        unsafe {
            let date = date.as_ptr();
            ffi::wxCalendarCtrl_SetDate(self.as_ptr(), date)
        }
    }
    /// Set the colours used for painting the weekdays at the top of the control.
    ///
    /// See [C++ `wxCalendarCtrl::SetHeaderColours()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a904833f654ad90958de59b61cb9e1e64).
    fn set_header_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHeaderColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    /// Set the colours to be used for highlighting the currently selected date.
    ///
    /// See [C++ `wxCalendarCtrl::SetHighlightColours()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a3f391058d675744a49cf9afaa2b786c3).
    fn set_highlight_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHighlightColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    /// Marks the specified day as being a holiday in the current month.
    ///
    /// See [C++ `wxCalendarCtrl::SetHoliday()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a801ea4e8f0bcec1c6ec5e8d75a393150).
    fn set_holiday(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_SetHoliday(self.as_ptr(), day) }
    }
    /// Sets the colours to be used for the holidays highlighting.
    ///
    /// See [C++ `wxCalendarCtrl::SetHolidayColours()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a3a751664b6aec52ce4252bdb099ec7eb).
    fn set_holiday_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHolidayColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    /// Mark or unmark the day.
    ///
    /// See [C++ `wxCalendarCtrl::Mark()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_ctrl.html#a7273a1c06a5a838c0a2980dececdf4f0).
    fn mark(&self, day: usize, mark: bool) {
        unsafe { ffi::wxCalendarCtrl_Mark(self.as_ptr(), day, mark) }
    }
}

// wxCalendarDateAttr
/// This trait represents [C++ `wxCalendarDateAttr` class](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html)'s methods and inheritance.
///
/// See [`CalendarDateAttrInRust`] documentation for the class usage.
pub trait CalendarDateAttrMethods: WxRustMethods {
    /// Returns the background colour set for the calendar date.
    ///
    /// See [C++ `wxCalendarDateAttr::GetBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a2d01d011d63f439566338654aa94e6b7).
    fn get_background_colour(&self) -> ColourInRust<false> {
        unsafe {
            ColourInRust::from_ptr(ffi::wxCalendarDateAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    // NOT_SUPPORTED: fn GetBorder()
    /// Returns the border colour set for the calendar date.
    ///
    /// See [C++ `wxCalendarDateAttr::GetBorderColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a5a6dc761f338621ef9aab76d35fbb941).
    fn get_border_colour(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarDateAttr_GetBorderColour(self.as_ptr())) }
    }
    /// Returns the font set for the calendar date.
    ///
    /// See [C++ `wxCalendarDateAttr::GetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#aae0094a81cf4d134b866532bdaa48e66).
    fn get_font(&self) -> FontInRust<false> {
        unsafe { FontInRust::from_ptr(ffi::wxCalendarDateAttr_GetFont(self.as_ptr())) }
    }
    /// Returns the text colour set for the calendar date.
    ///
    /// See [C++ `wxCalendarDateAttr::GetTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a60a1c912c70d11e17fe79cb29a424477).
    fn get_text_colour(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxCalendarDateAttr_GetTextColour(self.as_ptr())) }
    }
    /// Returns true if a non-default text background colour is set.
    ///
    /// See [C++ `wxCalendarDateAttr::HasBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a9b5be15042eb07e8e5deca3be824e613).
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBackgroundColour(self.as_ptr()) }
    }
    /// Returns true if a non-default (i.e. any) border is set.
    ///
    /// See [C++ `wxCalendarDateAttr::HasBorder()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a9ff98a88950216fd97eb1dbe975fdcfc).
    fn has_border(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorder(self.as_ptr()) }
    }
    /// Returns true if a non-default border colour is set.
    ///
    /// See [C++ `wxCalendarDateAttr::HasBorderColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a0a4be19fc55c06f84e828fd1a9c0d403).
    fn has_border_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorderColour(self.as_ptr()) }
    }
    /// Returns true if a non-default font is set.
    ///
    /// See [C++ `wxCalendarDateAttr::HasFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a1310e8e587988d4ebef12a666f1647f9).
    fn has_font(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasFont(self.as_ptr()) }
    }
    /// Returns true if a non-default text foreground colour is set.
    ///
    /// See [C++ `wxCalendarDateAttr::HasTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#ad4e272dfff91d86619c69848c0cdc508).
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasTextColour(self.as_ptr()) }
    }
    /// Returns true if this calendar day is displayed as a holiday.
    ///
    /// See [C++ `wxCalendarDateAttr::IsHoliday()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a49a93b8ea20f2669c9d9fb04d38297cd).
    fn is_holiday(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_IsHoliday(self.as_ptr()) }
    }
    /// Sets the text background colour to use.
    ///
    /// See [C++ `wxCalendarDateAttr::SetBackgroundColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a2069e44cf2779b3751e9f2aac51016f4).
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxCalendarDateAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    // NOT_SUPPORTED: fn SetBorder()
    /// Sets the border colour to use.
    ///
    /// See [C++ `wxCalendarDateAttr::SetBorderColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a4122d3da6381cad1064c782fa8f85734).
    fn set_border_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxCalendarDateAttr_SetBorderColour(self.as_ptr(), col)
        }
    }
    /// Sets the font to use.
    ///
    /// See [C++ `wxCalendarDateAttr::SetFont()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a4109c931605578813c8ef2bdeb978c8c).
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxCalendarDateAttr_SetFont(self.as_ptr(), font)
        }
    }
    /// If holiday is true, this calendar day will be displayed as a holiday.
    ///
    /// See [C++ `wxCalendarDateAttr::SetHoliday()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a96a3d5518f288fb91456c31c4ef53147).
    fn set_holiday(&self, holiday: bool) {
        unsafe { ffi::wxCalendarDateAttr_SetHoliday(self.as_ptr(), holiday) }
    }
    /// Sets the text (foreground) colour to use.
    ///
    /// See [C++ `wxCalendarDateAttr::SetTextColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a89f88fc34b15f0d466d1cdfc4e30440d).
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxCalendarDateAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    /// Used (internally) by the generic wxCalendarCtrl::Mark().
    ///
    /// See [C++ `wxCalendarDateAttr::GetMark()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#a49e21270d2613902f656c36113f516fd).
    fn get_mark() -> CalendarDateAttrInRust<false> {
        unsafe { CalendarDateAttrInRust::from_ptr(ffi::wxCalendarDateAttr_GetMark()) }
    }
    /// Set the attributes that will be used to Mark() days on the generic wxCalendarCtrl.
    ///
    /// See [C++ `wxCalendarDateAttr::SetMark()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_calendar_date_attr.html#aa7aeed4f898b583e702a9fe4a56f7629).
    fn set_mark<C: CalendarDateAttrMethods>(m: &C) {
        unsafe {
            let m = m.as_ptr();
            ffi::wxCalendarDateAttr_SetMark(m)
        }
    }
}

// wxCalendarEvent
/// This trait represents [C++ `wxCalendarEvent` class](https://docs.wxwidgets.org/3.2/classwx_calendar_event.html)'s methods and inheritance.
///
/// See [`CalendarEventInRust`] documentation for the class usage.
pub trait CalendarEventMethods: DateEventMethods {
    // NOT_SUPPORTED: fn GetWeekDay()
    // NOT_SUPPORTED: fn SetWeekDay()
}

// wxCaret
/// This trait represents [C++ `wxCaret` class](https://docs.wxwidgets.org/3.2/classwx_caret.html)'s methods and inheritance.
///
/// See [`CaretInRust`] documentation for the class usage.
pub trait CaretMethods: WxRustMethods {
    /// Creates a caret with the given size (in pixels) and associates it with the window (same as the equivalent constructors).
    ///
    /// See [C++ `wxCaret::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a46ffb44f7380258dea2d2bf6493d0214).
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
    ///
    /// See [C++ `wxCaret::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#ae63995452133a279aa4c37befc5a9567).
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
    /// Get the caret position (in pixels).
    ///
    /// See [C++ `wxCaret::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a9f8e3316ef96a75ae7669a4fdbbd11e0).
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxCaret_GetPosition(self.as_ptr(), x, y) }
    }
    ///
    /// See [C++ `wxCaret::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#aed175423fab541033c47d7c5af3f179a).
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCaret_GetPosition1(self.as_ptr())) }
    }
    /// Get the caret size.
    ///
    /// See [C++ `wxCaret::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a9fb436c1ee5bfce27e69b2db95fd5e87).
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxCaret_GetSize(self.as_ptr(), width, height) }
    }
    ///
    /// See [C++ `wxCaret::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a820c86115f443026adaeff7529055e4c).
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxCaret_GetSize1(self.as_ptr())) }
    }
    /// Get the window the caret is associated with.
    ///
    /// See [C++ `wxCaret::GetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a96596df1e23f92cc05f6649975c060f2).
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCaret_GetWindow(self.as_ptr())) }
    }
    /// Hides the caret, same as Show(false).
    ///
    /// See [C++ `wxCaret::Hide()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a272c1c4887bf5b9d16213b2bd616033a).
    fn hide(&self) {
        unsafe { ffi::wxCaret_Hide(self.as_ptr()) }
    }
    /// Returns true if the caret was created successfully.
    ///
    /// See [C++ `wxCaret::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#afc9ee90ae1895d5cbe4bccb4e64d6da7).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCaret_IsOk(self.as_ptr()) }
    }
    /// Returns true if the caret is visible and false if it is permanently hidden (if it is blinking and not shown currently but will be after the next blink, this method still returns true).
    ///
    /// See [C++ `wxCaret::IsVisible()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a90a2cf1db432b991a922ec68e7acf679).
    fn is_visible(&self) -> bool {
        unsafe { ffi::wxCaret_IsVisible(self.as_ptr()) }
    }
    /// Move the caret to given position (in logical coordinates).
    ///
    /// See [C++ `wxCaret::Move()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#ae490dc3156c4c686bc37f601edc33388).
    fn move_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxCaret_Move(self.as_ptr(), x, y) }
    }
    ///
    /// See [C++ `wxCaret::Move()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a4b1e2c18b950f771b44172b752d6eb1b).
    fn move_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxCaret_Move1(self.as_ptr(), pt)
        }
    }
    /// Changes the size of the caret.
    ///
    /// See [C++ `wxCaret::SetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a0dd4f34b8e000a967701bb2821ed8bb2).
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxCaret_SetSize(self.as_ptr(), width, height) }
    }
    ///
    /// See [C++ `wxCaret::SetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#afaed1930afff173d1a5ec005e6086509).
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxCaret_SetSize1(self.as_ptr(), size)
        }
    }
    /// Shows or hides the caret.
    ///
    /// See [C++ `wxCaret::Show()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#a5112409bd3a83148f364c4b08086fe28).
    fn show(&self, show: bool) {
        unsafe { ffi::wxCaret_Show(self.as_ptr(), show) }
    }
    /// Returns the blink time which is measured in milliseconds and is the time elapsed between 2 inversions of the caret (blink time of the caret is the same for all carets, so this functions is static).
    ///
    /// See [C++ `wxCaret::GetBlinkTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#ab9f9aed8b31730a8d451ce12aeffab3a).
    fn get_blink_time() -> c_int {
        unsafe { ffi::wxCaret_GetBlinkTime() }
    }
    /// Sets the blink time for all the carets.
    ///
    /// See [C++ `wxCaret::SetBlinkTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_caret.html#ae6018e5419476a7437940b92d10ec318).
    fn set_blink_time(milliseconds: c_int) {
        unsafe { ffi::wxCaret_SetBlinkTime(milliseconds) }
    }
}

// wxCheckBox
/// This trait represents [C++ `wxCheckBox` class](https://docs.wxwidgets.org/3.2/classwx_check_box.html)'s methods and inheritance.
///
/// See [`CheckBoxInRust`] documentation for the class usage.
pub trait CheckBoxMethods: ControlMethods {
    // DTOR: fn ~wxCheckBox()
    /// Creates the checkbox for two-step construction.
    ///
    /// See [C++ `wxCheckBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a4d9f2efaf4be717ca142fcbe854447a9).
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
    /// Gets the state of a 2-state checkbox.
    ///
    /// See [C++ `wxCheckBox::GetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#ad7e2e349016d60012d0d92f10e709758).
    fn get_value(&self) -> bool {
        unsafe { ffi::wxCheckBox_GetValue(self.as_ptr()) }
    }
    /// Gets the state of a 3-state checkbox.
    ///
    /// See [C++ `wxCheckBox::Get3StateValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#ab5551ef4c7f01a1bdb0cdbd20273b0bf).
    fn get3_state_value(&self) -> c_int {
        unsafe { ffi::wxCheckBox_Get3StateValue(self.as_ptr()) }
    }
    /// Returns whether or not the checkbox is a 3-state checkbox.
    ///
    /// See [C++ `wxCheckBox::Is3State()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a1306f3deee56e14b079ae38fd4ca199f).
    fn is3_state(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3State(self.as_ptr()) }
    }
    /// Returns whether or not the user can set the checkbox to the third state.
    ///
    /// See [C++ `wxCheckBox::Is3rdStateAllowedForUser()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#aada3c6eb6a7a17508276880ec3414f0b).
    fn is3rd_state_allowed_for_user(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3rdStateAllowedForUser(self.as_ptr()) }
    }
    /// This is just a maybe more readable synonym for GetValue(): just as the latter, it returns true if the checkbox is checked and false otherwise.
    ///
    /// See [C++ `wxCheckBox::IsChecked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a783aefab1b1388572f2886f6bac0e7fe).
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCheckBox_IsChecked(self.as_ptr()) }
    }
    /// Sets the checkbox to the given state.
    ///
    /// See [C++ `wxCheckBox::SetValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#a6b19d59054f3f038ffaa279bf99552fe).
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxCheckBox_SetValue(self.as_ptr(), state) }
    }
    /// Sets the checkbox to the given state.
    ///
    /// See [C++ `wxCheckBox::Set3StateValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_box.html#aa5c882ce796f17f5742c8f81613e8a90).
    fn set3_state_value(&self, state: c_int) {
        unsafe { ffi::wxCheckBox_Set3StateValue(self.as_ptr(), state) }
    }
}

// wxCheckListBox
/// This trait represents [C++ `wxCheckListBox` class](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html)'s methods and inheritance.
///
/// See [`CheckListBoxInRust`] documentation for the class usage.
pub trait CheckListBoxMethods: ListBoxMethods {
    // DTOR: fn ~wxCheckListBox()
    /// Checks the given item.
    ///
    /// See [C++ `wxCheckListBox::Check()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#a6843e58a97dfe738a1d2e59f5a284495).
    fn check(&self, item: c_uint, check: bool) {
        unsafe { ffi::wxCheckListBox_Check(self.as_ptr(), item, check) }
    }
    /// Returns true if the given item is checked, false otherwise.
    ///
    /// See [C++ `wxCheckListBox::IsChecked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#ad170533dca59d6b1a2c2919777498fe1).
    fn is_checked(&self, item: c_uint) -> bool {
        unsafe { ffi::wxCheckListBox_IsChecked(self.as_ptr(), item) }
    }
    /// Return the indices of the checked items.
    ///
    /// See [C++ `wxCheckListBox::GetCheckedItems()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_check_list_box.html#aeab606997fdc192bac9670f6b614a1b2).
    fn get_checked_items<A: ArrayIntMethods>(&self, checked_items: &A) -> c_uint {
        unsafe {
            let checked_items = checked_items.as_ptr();
            ffi::wxCheckListBox_GetCheckedItems(self.as_ptr(), checked_items)
        }
    }
}

// wxChildFocusEvent
/// This trait represents [C++ `wxChildFocusEvent` class](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html)'s methods and inheritance.
///
/// See [`ChildFocusEventInRust`] documentation for the class usage.
pub trait ChildFocusEventMethods: CommandEventMethods {
    /// Returns the direct child which receives the focus, or a (grand-)parent of the control receiving the focus.
    ///
    /// See [C++ `wxChildFocusEvent::GetWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_child_focus_event.html#aba66351df67066955a548e97f8f4213a).
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxChildFocusEvent_GetWindow(self.as_ptr())) }
    }
}

// wxChoice
/// This trait represents [C++ `wxChoice` class](https://docs.wxwidgets.org/3.2/classwx_choice.html)'s methods and inheritance.
///
/// See [`ChoiceInRust`] documentation for the class usage.
pub trait ChoiceMethods: ControlMethods {
    // DTOR: fn ~wxChoice()
    // NOT_SUPPORTED: fn Create()
    ///
    /// See [C++ `wxChoice::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#a91a72be7d1296d35589b152f7d86de3c).
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
    /// Gets the number of columns in this choice item.
    ///
    /// See [C++ `wxChoice::GetColumns()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#a7b79381bc9b407b8e4539fff2e09064a).
    fn get_columns(&self) -> c_int {
        unsafe { ffi::wxChoice_GetColumns(self.as_ptr()) }
    }
    /// Unlike wxControlWithItems::GetSelection() which only returns the accepted selection value (the selection in the control once the user closes the dropdown list), this function returns the current selection.
    ///
    /// See [C++ `wxChoice::GetCurrentSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#a6832c25f20b1191a1457ec8304d9aa6d).
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxChoice_GetCurrentSelection(self.as_ptr()) }
    }
    /// Sets the number of columns in this choice item.
    ///
    /// See [C++ `wxChoice::SetColumns()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#a141ac5a6d0ee752b1e756a55ba9ee169).
    fn set_columns(&self, n: c_int) {
        unsafe { ffi::wxChoice_SetColumns(self.as_ptr(), n) }
    }
    ///
    /// See [C++ `wxChoice::IsSorted()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choice.html#af4d7f5f57deed46e444f46311314d54c).
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxChoice_IsSorted(self.as_ptr()) }
    }
}

// wxChoicebook
/// This trait represents [C++ `wxChoicebook` class](https://docs.wxwidgets.org/3.2/classwx_choicebook.html)'s methods and inheritance.
///
/// See [`ChoicebookInRust`] documentation for the class usage.
pub trait ChoicebookMethods: BookCtrlBaseMethods {
    /// Returns the wxChoice associated with the control.
    ///
    /// See [C++ `wxChoicebook::GetChoiceCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_choicebook.html#a882185d0482d311f6b2ab0a4d106a7c2).
    fn get_choice_ctrl(&self) -> WeakRef<Choice> {
        unsafe { WeakRef::<Choice>::from(ffi::wxChoicebook_GetChoiceCtrl(self.as_ptr())) }
    }
}

// wxClientDC
/// This trait represents [C++ `wxClientDC` class](https://docs.wxwidgets.org/3.2/classwx_client_d_c.html)'s methods and inheritance.
///
/// See [`ClientDCInRust`] documentation for the class usage.
pub trait ClientDCMethods: WindowDCMethods {}

// wxClipboard
/// This trait represents [C++ `wxClipboard` class](https://docs.wxwidgets.org/3.2/classwx_clipboard.html)'s methods and inheritance.
///
/// See [`ClipboardInRust`] documentation for the class usage.
pub trait ClipboardMethods: ObjectMethods {
    // DTOR: fn ~wxClipboard()
    /// Call this function to add the data object to the clipboard.
    ///
    /// See [C++ `wxClipboard::AddData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#ae226633489bee0881839e5332fca714f).
    fn add_data<D: DataObjectMethods>(&self, data: Option<&D>) -> bool {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClipboard_AddData(self.as_ptr(), data)
        }
    }
    /// Clears the global clipboard object and the system's clipboard if possible.
    ///
    /// See [C++ `wxClipboard::Clear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#adf8ae31ddd1081f79951b4a782b84db1).
    fn clear(&self) {
        unsafe { ffi::wxClipboard_Clear(self.as_ptr()) }
    }
    /// Call this function to close the clipboard, having opened it with Open().
    ///
    /// See [C++ `wxClipboard::Close()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#afe9ac8bdb2af7c20cf68d36d460ac4fe).
    fn close(&self) {
        unsafe { ffi::wxClipboard_Close(self.as_ptr()) }
    }
    /// Flushes the clipboard: this means that the data which is currently on clipboard will stay available even after the application exits (possibly eating memory), otherwise the clipboard will be emptied on exit.
    ///
    /// See [C++ `wxClipboard::Flush()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a10997196ffd3b2cf8d823033f291f932).
    fn flush(&self) -> bool {
        unsafe { ffi::wxClipboard_Flush(self.as_ptr()) }
    }
    /// Call this function to fill data with data on the clipboard, if available in the required format.
    ///
    /// See [C++ `wxClipboard::GetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#af9edcd205c3749a317bd6b1e8a5716cc).
    fn get_data<D: DataObjectMethods>(&self, data: &D) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxClipboard_GetData(self.as_ptr(), data)
        }
    }
    /// Returns true if the clipboard has been opened.
    ///
    /// See [C++ `wxClipboard::IsOpened()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#ab18cfdeb2e139cfb0e87811cd133b744).
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxClipboard_IsOpened(self.as_ptr()) }
    }
    /// Returns true if there is data which matches the data format of the given data object currently available on the clipboard.
    ///
    /// See [C++ `wxClipboard::IsSupported()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a6c56dbf02b1807ce61cac8134a534336).
    fn is_supported<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxClipboard_IsSupported(self.as_ptr(), format)
        }
    }
    /// Returns true if we are using the primary selection, false if clipboard one.
    ///
    /// See [C++ `wxClipboard::IsUsingPrimarySelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a6f814afdfebeba1399667ae008515b30).
    fn is_using_primary_selection(&self) -> bool {
        unsafe { ffi::wxClipboard_IsUsingPrimarySelection(self.as_ptr()) }
    }
    /// Call this function to open the clipboard before calling SetData() and GetData().
    ///
    /// See [C++ `wxClipboard::Open()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a27dd0b1a54a368c9a786c8616eac667d).
    fn open(&self) -> bool {
        unsafe { ffi::wxClipboard_Open(self.as_ptr()) }
    }
    /// Call this function to set the data object to the clipboard.
    ///
    /// See [C++ `wxClipboard::SetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#afae7236718f30437c15709a911d4bab6).
    fn set_data<D: DataObjectMethods>(&self, data: Option<&D>) -> bool {
        unsafe {
            let data = match data {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxClipboard_SetData(self.as_ptr(), data)
        }
    }
    /// On platforms supporting it (all X11-based ports), wxClipboard uses the CLIPBOARD X11 selection by default.
    ///
    /// See [C++ `wxClipboard::UsePrimarySelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#aec2787c528e42791547a424e647b9dc2).
    fn use_primary_selection(&self, primary: bool) {
        unsafe { ffi::wxClipboard_UsePrimarySelection(self.as_ptr(), primary) }
    }
    /// Returns the global instance (wxTheClipboard) of the clipboard object.
    ///
    /// See [C++ `wxClipboard::Get()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_clipboard.html#a6f3a1f27cd3de59bfafd538827bc89a8).
    fn get() -> Option<ClipboardInRust<false>> {
        unsafe { Clipboard::option_from(ffi::wxClipboard_Get()) }
    }
}

// wxClipboardTextEvent
/// This trait represents [C++ `wxClipboardTextEvent` class](https://docs.wxwidgets.org/3.2/classwx_clipboard_text_event.html)'s methods and inheritance.
///
/// See [`ClipboardTextEventInRust`] documentation for the class usage.
pub trait ClipboardTextEventMethods: CommandEventMethods {}

// wxCloseEvent
/// This trait represents [C++ `wxCloseEvent` class](https://docs.wxwidgets.org/3.2/classwx_close_event.html)'s methods and inheritance.
///
/// See [`CloseEventInRust`] documentation for the class usage.
pub trait CloseEventMethods: EventMethods {
    /// Returns true if you can veto a system shutdown or a window close event.
    ///
    /// See [C++ `wxCloseEvent::CanVeto()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html#acb23de85447edb56bf806cc382f311ec).
    fn can_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_CanVeto(self.as_ptr()) }
    }
    /// Returns true if the user is just logging off or false if the system is shutting down.
    ///
    /// See [C++ `wxCloseEvent::GetLoggingOff()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html#ac366162fc32083e502090a9d75c4bf16).
    fn get_logging_off(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetLoggingOff(self.as_ptr()) }
    }
    /// Sets the 'can veto' flag.
    ///
    /// See [C++ `wxCloseEvent::SetCanVeto()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html#a25150c209983febf02b80a6f93dfc6ba).
    fn set_can_veto(&self, can_veto: bool) {
        unsafe { ffi::wxCloseEvent_SetCanVeto(self.as_ptr(), can_veto) }
    }
    /// Sets the 'logging off' flag.
    ///
    /// See [C++ `wxCloseEvent::SetLoggingOff()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html#af8059e703d942f5962a0b2f6dedc3bd6).
    fn set_logging_off(&self, logging_off: bool) {
        unsafe { ffi::wxCloseEvent_SetLoggingOff(self.as_ptr(), logging_off) }
    }
    /// Call this from your event handler to veto a system shutdown or to signal to the calling application that a window close did not happen.
    ///
    /// See [C++ `wxCloseEvent::Veto()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html#a0ec337c77613d5d8407831cd64e16417).
    fn veto(&self, veto: bool) {
        unsafe { ffi::wxCloseEvent_Veto(self.as_ptr(), veto) }
    }
    /// Returns whether the Veto flag was set.
    ///
    /// See [C++ `wxCloseEvent::GetVeto()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_close_event.html#a35320f500ee6c929198ef94534227a64).
    fn get_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetVeto(self.as_ptr()) }
    }
}

// wxCollapsiblePane
/// This trait represents [C++ `wxCollapsiblePane` class](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html)'s methods and inheritance.
///
/// See [`CollapsiblePaneInRust`] documentation for the class usage.
pub trait CollapsiblePaneMethods: ControlMethods {
    ///
    /// See [C++ `wxCollapsiblePane::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#ae609270807e8655796abedfa7397b502).
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
    /// Collapses or expands the pane window.
    ///
    /// See [C++ `wxCollapsiblePane::Collapse()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#aaeebf5484692f4e262ea3760c372ecdc).
    fn collapse(&self, collapse: bool) {
        unsafe { ffi::wxCollapsiblePane_Collapse(self.as_ptr(), collapse) }
    }
    /// Same as calling Collapse(false).
    ///
    /// See [C++ `wxCollapsiblePane::Expand()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#a23700d2702ddb248e97cd80672c7becf).
    fn expand(&self) {
        unsafe { ffi::wxCollapsiblePane_Expand(self.as_ptr()) }
    }
    /// Returns a pointer to the pane window.
    ///
    /// See [C++ `wxCollapsiblePane::GetPane()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#a23e7c5cc46d6bf833c376e2906e88e95).
    fn get_pane(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCollapsiblePane_GetPane(self.as_ptr())) }
    }
    /// Returns true if the pane window is currently hidden.
    ///
    /// See [C++ `wxCollapsiblePane::IsCollapsed()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#a56d105d3c0f420307dd54ffde2f23e00).
    fn is_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsCollapsed(self.as_ptr()) }
    }
    /// Returns true if the pane window is currently shown.
    ///
    /// See [C++ `wxCollapsiblePane::IsExpanded()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane.html#aa083432eb79edcaa28ff9d1e614a88c0).
    fn is_expanded(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsExpanded(self.as_ptr()) }
    }
}

// wxCollapsiblePaneEvent
/// This trait represents [C++ `wxCollapsiblePaneEvent` class](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html)'s methods and inheritance.
///
/// See [`CollapsiblePaneEventInRust`] documentation for the class usage.
pub trait CollapsiblePaneEventMethods: CommandEventMethods {
    /// Returns true if the pane has been collapsed.
    ///
    /// See [C++ `wxCollapsiblePaneEvent::GetCollapsed()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html#a8277689d2875865a4aa7cb896c05ff8c).
    fn get_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePaneEvent_GetCollapsed(self.as_ptr()) }
    }
    /// Sets this as a collapsed pane event (if collapsed is true) or as an expanded pane event (if collapsed is false).
    ///
    /// See [C++ `wxCollapsiblePaneEvent::SetCollapsed()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_collapsible_pane_event.html#ad4d117a8ce01c0e3d966abd93444a9b3).
    fn set_collapsed(&self, collapsed: bool) {
        unsafe { ffi::wxCollapsiblePaneEvent_SetCollapsed(self.as_ptr(), collapsed) }
    }
}

// wxColour
/// This trait represents [C++ `wxColour` class](https://docs.wxwidgets.org/3.2/classwx_colour.html)'s methods and inheritance.
///
/// See [`ColourInRust`] documentation for the class usage.
pub trait ColourMethods: ObjectMethods {
    // NOT_SUPPORTED: fn Alpha()
    // NOT_SUPPORTED: fn Blue()
    /// Returns the alpha value, on platforms where alpha is not yet supported, this always returns wxALPHA_OPAQUE.
    ///
    /// See [C++ `wxColour::GetAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a5a10b23750b22a36e6707f6bca1717c3).
    fn get_alpha(&self) -> c_uint {
        unsafe { ffi::wxColour_GetAlpha(self.as_ptr()) }
    }
    /// Returns the blue intensity as unsigned int.
    ///
    /// See [C++ `wxColour::GetBlue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a2e08925f421d35f6d9a658d0ce9e67ba).
    fn get_blue(&self) -> c_uint {
        unsafe { ffi::wxColour_GetBlue(self.as_ptr()) }
    }
    /// Returns the green intensity as unsigned int.
    ///
    /// See [C++ `wxColour::GetGreen()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#ad65896b8495a31e02afc0714cd670e11).
    fn get_green(&self) -> c_uint {
        unsafe { ffi::wxColour_GetGreen(self.as_ptr()) }
    }
    /// Returns the red intensity as unsigned int.
    ///
    /// See [C++ `wxColour::GetRed()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a80dca84553adddc70115db25060963cf).
    fn get_red(&self) -> c_uint {
        unsafe { ffi::wxColour_GetRed(self.as_ptr()) }
    }
    /// Converts this colour to a wxString using the given flags.
    ///
    /// See [C++ `wxColour::GetAsString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a7fb83960cea726dc4a37560b03f4bed3).
    fn get_as_string(&self, flags: c_long) -> String {
        unsafe { WxString::from_ptr(ffi::wxColour_GetAsString(self.as_ptr(), flags)).into() }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGBA()
    // NOT_SUPPORTED: fn GetRGB()
    // NOT_SUPPORTED: fn GetRGBA()
    /// Return the perceived brightness of the colour.
    ///
    /// See [C++ `wxColour::GetLuminance()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a2e9b5da1063e8467a0eabc14d214e729).
    fn get_luminance(&self) -> c_double {
        unsafe { ffi::wxColour_GetLuminance(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    // NOT_SUPPORTED: fn Green()
    /// Returns true if the colour object is valid (the colour has been initialised with RGB values).
    ///
    /// See [C++ `wxColour::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a90921aeb085612b86fffb1507dbca461).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxColour_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Red()
    /// Returns true if the color can be described using RGB values, i.e.
    ///
    /// See [C++ `wxColour::IsSolid()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a33db79b6c89d81e0dccd240dd5e08005).
    fn is_solid(&self) -> bool {
        unsafe { ffi::wxColour_IsSolid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Set()
    // NOT_SUPPORTED: fn Set1()
    ///
    /// See [C++ `wxColour::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a08e9f56265647b8b5e1349b76eb728e3).
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
    /// Assigns the same value to r, g, b: 0 if on is false, 255 otherwise.
    ///
    /// See [C++ `wxColour::MakeMono()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a0ba8c496e7553e34643e53e70de57760).
    fn make_mono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool) {
        unsafe { ffi::wxColour_MakeMono(r, g, b, on) }
    }
    // NOT_SUPPORTED: fn MakeDisabled1()
    /// Create a grey colour from (in/out) rgb parameters using integer arithmetic.
    ///
    /// See [C++ `wxColour::MakeGrey()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a45e74c88fdab10e07164abe02c1b011a).
    fn make_grey(r: *mut c_void, g: *mut c_void, b: *mut c_void) {
        unsafe { ffi::wxColour_MakeGrey(r, g, b) }
    }
    /// Create a grey colour from (in/out) rgb parameters using floating point arithmetic.
    ///
    /// See [C++ `wxColour::MakeGrey()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#a9020f882bba4f92bb68fcc7fd6d7cff4).
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
    /// Utility function that simply darkens or lightens a color, based on the specified percentage ialpha.
    ///
    /// See [C++ `wxColour::ChangeLightness()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour.html#af225e6fafd00001ac3612aaf26fe5501).
    fn change_lightness(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int) {
        unsafe { ffi::wxColour_ChangeLightness1(r, g, b, ialpha) }
    }
}

// wxColourData
/// This trait represents [C++ `wxColourData` class](https://docs.wxwidgets.org/3.2/classwx_colour_data.html)'s methods and inheritance.
///
/// See [`ColourDataInRust`] documentation for the class usage.
pub trait ColourDataMethods: ObjectMethods {
    // DTOR: fn ~wxColourData()
    /// Under Windows, determines whether the Windows colour dialog will display the full dialog with custom colour selection controls.
    ///
    /// See [C++ `wxColourData::GetChooseFull()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a3ab8a0182362ec82cb62f47d9ce0406c).
    fn get_choose_full(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseFull(self.as_ptr()) }
    }
    /// Indicates whether the colour dialog will display alpha values and an opacity selector.
    ///
    /// See [C++ `wxColourData::GetChooseAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#aca531e328674dac7d86912f89547f815).
    fn get_choose_alpha(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseAlpha(self.as_ptr()) }
    }
    /// Gets the current colour associated with the colour dialog.
    ///
    /// See [C++ `wxColourData::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a1e9b73d93d5e49627faa8ce926318606).
    fn get_colour(&self) -> ColourInRust<false> {
        unsafe { ColourInRust::from_ptr(ffi::wxColourData_GetColour(self.as_ptr())) }
    }
    /// Returns custom colours associated with the colour dialog.
    ///
    /// See [C++ `wxColourData::GetCustomColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a5d2c170f3f5ba62d4bb77d08263fcf79).
    fn get_custom_colour(&self, i: c_int) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourData_GetCustomColour(self.as_ptr(), i)) }
    }
    /// Under Windows, tells the Windows colour dialog to display the full dialog with custom colour selection controls.
    ///
    /// See [C++ `wxColourData::SetChooseFull()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a2e5e80d982cbcb17cac113a72efef2bf).
    fn set_choose_full(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseFull(self.as_ptr(), flag) }
    }
    /// Tells the colour dialog to show alpha values and an opacity selector (slider).
    ///
    /// See [C++ `wxColourData::SetChooseAlpha()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#ac6593f8d1c84e343c81a7e9377ad6f7e).
    fn set_choose_alpha(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseAlpha(self.as_ptr(), flag) }
    }
    /// Sets the default colour for the colour dialog.
    ///
    /// See [C++ `wxColourData::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a66d6dd56885c84a9f83e28a75fe2132e).
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetColour(self.as_ptr(), colour)
        }
    }
    /// Sets custom colours for the colour dialog.
    ///
    /// See [C++ `wxColourData::SetCustomColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#afff3c5561456c17bcf6329a81dcfc716).
    fn set_custom_colour<C: ColourMethods>(&self, i: c_int, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetCustomColour(self.as_ptr(), i, colour)
        }
    }
    /// Converts the colours saved in this class in a string form, separating the various colours with a comma.
    ///
    /// See [C++ `wxColourData::ToString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#aa507012d3715813d5eefcf936335e2af).
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxColourData_ToString(self.as_ptr())).into() }
    }
    /// Decodes the given string, which should be in the same format returned by ToString(), and sets the internal colours.
    ///
    /// See [C++ `wxColourData::FromString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_data.html#a6d0b9f7cfd1d628f212bc19ef963cab8).
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
/// This trait represents [C++ `wxColourDatabase` class](https://docs.wxwidgets.org/3.2/classwx_colour_database.html)'s methods and inheritance.
///
/// See [`ColourDatabaseInRust`] documentation for the class usage.
pub trait ColourDatabaseMethods: WxRustMethods {
    /// Adds a colour to the database.
    ///
    /// See [C++ `wxColourDatabase::AddColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html#ab52ecdf5e71d9ed0229236e799fc7dea).
    fn add_colour<C: ColourMethods>(&self, colour_name: &str, colour: &C) {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxColourDatabase_AddColour(self.as_ptr(), colour_name, colour)
        }
    }
    /// Finds a colour given the name.
    ///
    /// See [C++ `wxColourDatabase::Find()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html#acedbf1ae7fe119a7a3aebfae8b0a14e9).
    fn find(&self, colour_name: &str) -> Colour {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            Colour::from_ptr(ffi::wxColourDatabase_Find(self.as_ptr(), colour_name))
        }
    }
    /// Finds a colour name given the colour.
    ///
    /// See [C++ `wxColourDatabase::FindName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_database.html#a03ad9407e4751d348903557a496786be).
    fn find_name<C: ColourMethods>(&self, colour: &C) -> String {
        unsafe {
            let colour = colour.as_ptr();
            WxString::from_ptr(ffi::wxColourDatabase_FindName(self.as_ptr(), colour)).into()
        }
    }
}

// wxColourDialog
/// This trait represents [C++ `wxColourDialog` class](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html)'s methods and inheritance.
///
/// See [`ColourDialogInRust`] documentation for the class usage.
pub trait ColourDialogMethods: DialogMethods {
    // DTOR: fn ~wxColourDialog()
    /// Same as wxColourDialog().
    ///
    /// See [C++ `wxColourDialog::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html#a0ee38e7ad14c33201a810ba72b2ce54f).
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
    /// Returns the colour data associated with the colour dialog.
    ///
    /// See [C++ `wxColourDialog::GetColourData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_dialog.html#a527d63fdb2b83eb9ee80d1c4b5da5c09).
    fn get_colour_data(&self) -> ColourDataInRust<false> {
        unsafe { ColourDataInRust::from_ptr(ffi::wxColourDialog_GetColourData(self.as_ptr())) }
    }
}

// wxColourPickerCtrl
/// This trait represents [C++ `wxColourPickerCtrl` class](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html)'s methods and inheritance.
///
/// See [`ColourPickerCtrlInRust`] documentation for the class usage.
pub trait ColourPickerCtrlMethods: PickerBaseMethods {
    /// Creates a colour picker with the given arguments.
    ///
    /// See [C++ `wxColourPickerCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a40b58f3b01060f0cff332fa305e568d4).
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
    /// Returns the currently selected colour.
    ///
    /// See [C++ `wxColourPickerCtrl::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#af4fd994b2c47ee7d5197769c4dd8ed5b).
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerCtrl_GetColour(self.as_ptr())) }
    }
    /// Sets the currently selected colour.
    ///
    /// See [C++ `wxColourPickerCtrl::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_ctrl.html#a5dc2a4de5662ca009800551b738feb7e).
    fn set_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxColourPickerCtrl_SetColour(self.as_ptr(), col)
        }
    }
    // BLOCKED: fn SetColour1()
}

// wxColourPickerEvent
/// This trait represents [C++ `wxColourPickerEvent` class](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html)'s methods and inheritance.
///
/// See [`ColourPickerEventInRust`] documentation for the class usage.
pub trait ColourPickerEventMethods: CommandEventMethods {
    /// Retrieve the colour the user has just selected.
    ///
    /// See [C++ `wxColourPickerEvent::GetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#a4d7388f7e40d69b40f2469c84b9a590d).
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerEvent_GetColour(self.as_ptr())) }
    }
    /// Set the colour associated with the event.
    ///
    /// See [C++ `wxColourPickerEvent::SetColour()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_colour_picker_event.html#a7f5ec104ad3c539ed19afd10f48ee361).
    fn set_colour<C: ColourMethods>(&self, pos: &C) {
        unsafe {
            let pos = pos.as_ptr();
            ffi::wxColourPickerEvent_SetColour(self.as_ptr(), pos)
        }
    }
}

// wxComboBox
/// This trait represents [C++ `wxComboBox` class](https://docs.wxwidgets.org/3.2/classwx_combo_box.html)'s methods and inheritance.
///
/// See [`ComboBoxInRust`] documentation for the class usage.
pub trait ComboBoxMethods: ControlMethods {
    // DTOR: fn ~wxComboBox()
    // NOT_SUPPORTED: fn Create()
    ///
    /// See [C++ `wxComboBox::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#ac14230b50e82a84f0d3b53ef08fe1c77).
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
    /// Returns the item being selected right now.
    ///
    /// See [C++ `wxComboBox::GetCurrentSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#a411ef01361129aab405724591bb6ed4a).
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxComboBox_GetCurrentSelection(self.as_ptr()) }
    }
    /// Returns true if the list of combobox choices is empty.
    ///
    /// See [C++ `wxComboBox::IsListEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#aa5049fd50f096dd39b1eb6514c48f5fc).
    fn is_list_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsListEmpty(self.as_ptr()) }
    }
    /// Returns true if the text of the combobox is empty.
    ///
    /// See [C++ `wxComboBox::IsTextEmpty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#a6cefa855ee03dec74c955ace0401acdb).
    fn is_text_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsTextEmpty(self.as_ptr()) }
    }
    /// Shows the list box portion of the combo box.
    ///
    /// See [C++ `wxComboBox::Popup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#a325d7d2afc9e1e17f7b5fb13f4186d0c).
    fn popup(&self) {
        unsafe { ffi::wxComboBox_Popup(self.as_ptr()) }
    }
    /// Hides the list box portion of the combo box.
    ///
    /// See [C++ `wxComboBox::Dismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_box.html#acb37b8dfdeb57a514d87ae21a5e73853).
    fn dismiss(&self) {
        unsafe { ffi::wxComboBox_Dismiss(self.as_ptr()) }
    }
}

// wxComboCtrl
/// This trait represents [C++ `wxComboCtrl` class](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html)'s methods and inheritance.
///
/// See [`ComboCtrlInRust`] documentation for the class usage.
pub trait ComboCtrlMethods: ControlMethods {
    // DTOR: fn ~wxComboCtrl()
    /// Creates the combo control for two-step construction.
    ///
    /// See [C++ `wxComboCtrl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#aab2390bd7650a56e3d2c243140d28f98).
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
    /// Dismisses the popup window.
    ///
    /// See [C++ `wxComboCtrl::Dismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a56867b1c18641c21fcd518ce43efbf99).
    fn dismiss(&self) {
        unsafe { ffi::wxComboCtrl_Dismiss(self.as_ptr()) }
    }
    /// Enables or disables popup animation, if any, depending on the value of the argument.
    ///
    /// See [C++ `wxComboCtrl::EnablePopupAnimation()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ae90d978c054d1ce215758c6073c43d31).
    fn enable_popup_animation(&self, enable: bool) {
        unsafe { ffi::wxComboCtrl_EnablePopupAnimation(self.as_ptr(), enable) }
    }
    /// Returns true if given key combination should toggle the popup.
    ///
    /// See [C++ `wxComboCtrl::IsKeyPopupToggle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a523036a15ae181059d03f97969fd123b).
    fn is_key_popup_toggle<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxComboCtrl_IsKeyPopupToggle(self.as_ptr(), event)
        }
    }
    /// Prepare background of combo control or an item in a dropdown list in a way typical on platform.
    ///
    /// See [C++ `wxComboCtrl::PrepareBackground()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#abd4fb20092e56779c74c7910a0c0ce9b).
    fn prepare_background<D: DCMethods, R: RectMethods>(&self, dc: &D, rect: &R, flags: c_int) {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxComboCtrl_PrepareBackground(self.as_ptr(), dc, rect, flags)
        }
    }
    /// Returns true if focus indicator should be drawn in the control.
    ///
    /// See [C++ `wxComboCtrl::ShouldDrawFocus()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#af7d1e035c5ed8d0e34baf6e483e855bb).
    fn should_draw_focus(&self) -> bool {
        unsafe { ffi::wxComboCtrl_ShouldDrawFocus(self.as_ptr()) }
    }
    /// Returns disabled button bitmap that has been set with SetButtonBitmaps().
    ///
    /// See [C++ `wxComboCtrl::GetBitmapDisabled()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a0c6a1caeb1e10cea1785e50601dd558e).
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapDisabled(self.as_ptr())) }
    }
    /// Returns button mouse hover bitmap that has been set with SetButtonBitmaps().
    ///
    /// See [C++ `wxComboCtrl::GetBitmapHover()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a5b0598d42b0286bcb54289df74a504cd).
    fn get_bitmap_hover(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapHover(self.as_ptr())) }
    }
    /// Returns default button bitmap that has been set with SetButtonBitmaps().
    ///
    /// See [C++ `wxComboCtrl::GetBitmapNormal()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a6f7e4b8d9ea13906632fc3b26687dc0d).
    fn get_bitmap_normal(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapNormal(self.as_ptr())) }
    }
    /// Returns depressed button bitmap that has been set with SetButtonBitmaps().
    ///
    /// See [C++ `wxComboCtrl::GetBitmapPressed()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#abf35264f5bdc70988d6d265c0543a20e).
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapPressed(self.as_ptr())) }
    }
    /// Returns current size of the dropdown button.
    ///
    /// See [C++ `wxComboCtrl::GetButtonSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ab6e1596a4b709465e0b07362a3c7dad2).
    fn get_button_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxComboCtrl_GetButtonSize(self.as_ptr())) }
    }
    /// Returns custom painted area in control.
    ///
    /// See [C++ `wxComboCtrl::GetCustomPaintWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ae4d95947e551ad7818b60aabf718e84c).
    fn get_custom_paint_width(&self) -> c_int {
        unsafe { ffi::wxComboCtrl_GetCustomPaintWidth(self.as_ptr()) }
    }
    /// Returns the margins used by the control.
    ///
    /// See [C++ `wxComboCtrl::GetMargins()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a3ee7141dc6069fa9b520cd994d176eff).
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxComboCtrl_GetMargins(self.as_ptr())) }
    }
    /// Returns current popup interface that has been set with SetPopupControl().
    ///
    /// See [C++ `wxComboCtrl::GetPopupControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ac3d072940f2c6c2c92e6fe629bf4c4a8).
    fn get_popup_control(&self) -> Option<ComboPopupInRust<false>> {
        unsafe { ComboPopup::option_from(ffi::wxComboCtrl_GetPopupControl(self.as_ptr())) }
    }
    /// Returns popup window containing the popup control.
    ///
    /// See [C++ `wxComboCtrl::GetPopupWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a9db18fe1b974c97607895012ae52299c).
    fn get_popup_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxComboCtrl_GetPopupWindow(self.as_ptr())) }
    }
    /// Get the text control which is part of the combo control.
    ///
    /// See [C++ `wxComboCtrl::GetTextCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a404cfaf76e31db5d58c3711d6dc1ea85).
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxComboCtrl_GetTextCtrl(self.as_ptr())) }
    }
    // BLOCKED: fn GetTextIndent()
    /// Returns area covered by the text field (includes everything except borders and the dropdown button).
    ///
    /// See [C++ `wxComboCtrl::GetTextRect()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ac25a4b9b325106dd5ec285ec2b368bdf).
    fn get_text_rect(&self) -> RectInRust<false> {
        unsafe { RectInRust::from_ptr(ffi::wxComboCtrl_GetTextRect(self.as_ptr())) }
    }
    /// Dismisses the popup window.
    ///
    /// See [C++ `wxComboCtrl::HidePopup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a73d2d6ede7bdc0db63c08f81cd09a5e7).
    fn hide_popup(&self, generate_event: bool) {
        unsafe { ffi::wxComboCtrl_HidePopup(self.as_ptr(), generate_event) }
    }
    /// Returns true if the popup is currently shown.
    ///
    /// See [C++ `wxComboCtrl::IsPopupShown()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#afb29fcf34147d9c38694806e1afc1dde).
    fn is_popup_shown(&self) -> bool {
        unsafe { ffi::wxComboCtrl_IsPopupShown(self.as_ptr()) }
    }
    /// Returns true if the popup window is in the given state.
    ///
    /// See [C++ `wxComboCtrl::IsPopupWindowState()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a499f87e1aa355eb69b1a61bea9165965).
    fn is_popup_window_state(&self, state: c_int) -> bool {
        unsafe { ffi::wxComboCtrl_IsPopupWindowState(self.as_ptr(), state) }
    }
    /// Implement in a derived class to define what happens on dropdown button click.
    ///
    /// See [C++ `wxComboCtrl::OnButtonClick()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a3ce1fce108f3a84216fa9983d6b89a37).
    fn on_button_click(&self) {
        unsafe { ffi::wxComboCtrl_OnButtonClick(self.as_ptr()) }
    }
    /// Shows the popup portion of the combo control.
    ///
    /// See [C++ `wxComboCtrl::Popup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#afda0208f5c2a31d504e230c36546fe8d).
    fn popup(&self) {
        unsafe { ffi::wxComboCtrl_Popup(self.as_ptr()) }
    }
    /// Sets custom dropdown button graphics.
    ///
    /// See [C++ `wxComboCtrl::SetButtonBitmaps()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ab0af76f57bdea68224a5c6bf5fecdb01).
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
    /// Sets size and position of dropdown button.
    ///
    /// See [C++ `wxComboCtrl::SetButtonPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#aa0ffd3528d66f15bfd3c0da78117ebad).
    fn set_button_position(&self, width: c_int, height: c_int, side: c_int, spacing_x: c_int) {
        unsafe { ffi::wxComboCtrl_SetButtonPosition(self.as_ptr(), width, height, side, spacing_x) }
    }
    /// Set width, in pixels, of custom painted area in control without wxCB_READONLY style.
    ///
    /// See [C++ `wxComboCtrl::SetCustomPaintWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#af0b54e5e4f71efc63464ceb44f1497d5).
    fn set_custom_paint_width(&self, width: c_int) {
        unsafe { ffi::wxComboCtrl_SetCustomPaintWidth(self.as_ptr(), width) }
    }
    /// Uses the given window instead of the default text control as the main window of the combo control.
    ///
    /// See [C++ `wxComboCtrl::SetMainControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a7e060b74078d635287080f42016ac26a).
    fn set_main_control<W: WindowMethods>(&self, win: Option<&W>) {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxComboCtrl_SetMainControl(self.as_ptr(), win)
        }
    }
    /// Attempts to set the control margins.
    ///
    /// See [C++ `wxComboCtrl::SetMargins()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ae2b983f401d92ccc38f2bed47fb4d76b).
    fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxComboCtrl_SetMargins(self.as_ptr(), pt)
        }
    }
    ///
    /// See [C++ `wxComboCtrl::SetMargins()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a61fb396d486c81bcb733ff0b35301904).
    fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
        unsafe { ffi::wxComboCtrl_SetMargins1(self.as_ptr(), left, top) }
    }
    /// Set side of the control to which the popup will align itself.
    ///
    /// See [C++ `wxComboCtrl::SetPopupAnchor()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#aac64729c09efde24fbc14d43aa688202).
    fn set_popup_anchor(&self, anchor_side: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupAnchor(self.as_ptr(), anchor_side) }
    }
    /// Set popup interface class derived from wxComboPopup.
    ///
    /// See [C++ `wxComboCtrl::SetPopupControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#afb0b22cb38395040699eebcb679fd86a).
    fn set_popup_control<C: ComboPopupMethods>(&self, popup: Option<&C>) {
        unsafe {
            let popup = match popup {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxComboCtrl_SetPopupControl(self.as_ptr(), popup)
        }
    }
    /// Extends popup size horizontally, relative to the edges of the combo control.
    ///
    /// See [C++ `wxComboCtrl::SetPopupExtents()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#add059c3997d6697bc98365f039da17f5).
    fn set_popup_extents(&self, ext_left: c_int, ext_right: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupExtents(self.as_ptr(), ext_left, ext_right) }
    }
    /// Sets preferred maximum height of the popup.
    ///
    /// See [C++ `wxComboCtrl::SetPopupMaxHeight()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a24f043ce5c1287904a899c22f2dc7ce9).
    fn set_popup_max_height(&self, height: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupMaxHeight(self.as_ptr(), height) }
    }
    /// Sets minimum width of the popup.
    ///
    /// See [C++ `wxComboCtrl::SetPopupMinWidth()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#aef690d395a8ef2500d2c4ddd2d345d6d).
    fn set_popup_min_width(&self, width: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupMinWidth(self.as_ptr(), width) }
    }
    /// Sets the text for the text field without affecting the popup.
    ///
    /// See [C++ `wxComboCtrl::SetText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a738267959dcf2b73430bd310173ebedb).
    fn set_text(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboCtrl_SetText(self.as_ptr(), value)
        }
    }
    /// Set a custom window style for the embedded wxTextCtrl.
    ///
    /// See [C++ `wxComboCtrl::SetTextCtrlStyle()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a00188332f7b6200c98f5f65966d5b151).
    fn set_text_ctrl_style(&self, style: c_int) {
        unsafe { ffi::wxComboCtrl_SetTextCtrlStyle(self.as_ptr(), style) }
    }
    // BLOCKED: fn SetTextIndent()
    /// Changes value of the control as if user had done it by selecting an item from a combo box drop-down list.
    ///
    /// See [C++ `wxComboCtrl::SetValueByUser()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#aaa42a7fdfb9061e4901086abcc7120e7).
    fn set_value_by_user(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboCtrl_SetValueByUser(self.as_ptr(), value)
        }
    }
    /// Show the popup.
    ///
    /// See [C++ `wxComboCtrl::ShowPopup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#ae87a04c471bfcac5ab0d370ac9edddbc).
    fn show_popup(&self) {
        unsafe { ffi::wxComboCtrl_ShowPopup(self.as_ptr()) }
    }
    /// Enable or disable usage of an alternative popup window, which guarantees ability to focus the popup control, and allows common native controls to function normally.
    ///
    /// See [C++ `wxComboCtrl::UseAltPopupWindow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a5b8f90d12149fe7e3f1e38e0dd0b839a).
    fn use_alt_popup_window(&self, enable: bool) {
        unsafe { ffi::wxComboCtrl_UseAltPopupWindow(self.as_ptr(), enable) }
    }
    /// Returns features supported by wxComboCtrl.
    ///
    /// See [C++ `wxComboCtrl::GetFeatures()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_ctrl.html#a10be4d02e6e2bb6a3708aa19ebf45b97).
    fn get_features() -> c_int {
        unsafe { ffi::wxComboCtrl_GetFeatures() }
    }
}

// wxComboPopup
/// This trait represents [C++ `wxComboPopup` class](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html)'s methods and inheritance.
///
/// See [`ComboPopupInRust`] documentation for the class usage.
pub trait ComboPopupMethods: WxRustMethods {
    /// The derived class must implement this to create the popup control.
    ///
    /// See [C++ `wxComboPopup::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a42375d0b5b9d39c0e48d7ca00b05e310).
    fn create<W: WindowMethods>(&self, parent: Option<&W>) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxComboPopup_Create(self.as_ptr(), parent)
        }
    }
    /// You only need to implement this member function if you create your popup class in non-standard way.
    ///
    /// See [C++ `wxComboPopup::DestroyPopup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a7bab875909925651797460484ab237df).
    fn destroy_popup(&self) {
        unsafe { ffi::wxComboPopup_DestroyPopup(self.as_ptr()) }
    }
    /// Utility function that hides the popup.
    ///
    /// See [C++ `wxComboPopup::Dismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#aefa27348fb0160af43197223238396e5).
    fn dismiss(&self) {
        unsafe { ffi::wxComboPopup_Dismiss(self.as_ptr()) }
    }
    /// Implement to customize matching of value string to an item container entry.
    ///
    /// See [C++ `wxComboPopup::FindItem()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a2d19c49312ff3098d11e3e465afcf284).
    fn find_item(&self, item: &str, true_item: *mut c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxComboPopup_FindItem(self.as_ptr(), item, true_item)
        }
    }
    /// The derived class may implement this to return adjusted size for the popup control, according to the variables given.
    ///
    /// See [C++ `wxComboPopup::GetAdjustedSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#ae8ff1d6570b177c2c6b7a101035cf2ff).
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
    /// Returns pointer to the associated parent wxComboCtrl.
    ///
    /// See [C++ `wxComboPopup::GetComboCtrl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a00c95abb2f0e404d00499d94cb5c1a4f).
    fn get_combo_ctrl(&self) -> WeakRef<ComboCtrl> {
        unsafe { WeakRef::<ComboCtrl>::from(ffi::wxComboPopup_GetComboCtrl(self.as_ptr())) }
    }
    /// The derived class must implement this to return pointer to the associated control created in Create().
    ///
    /// See [C++ `wxComboPopup::GetControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#abcbe87fe988a1d0f123883404c1a4110).
    fn get_control(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxComboPopup_GetControl(self.as_ptr())) }
    }
    /// The derived class must implement this to return string representation of the value.
    ///
    /// See [C++ `wxComboPopup::GetStringValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#addba104e8e7944c758905e2d1540e389).
    fn get_string_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxComboPopup_GetStringValue(self.as_ptr())).into() }
    }
    /// The derived class must implement this to initialize its internal variables.
    ///
    /// See [C++ `wxComboPopup::Init()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#aff98da6f4c273ba768979c94ce82508e).
    fn init(&self) {
        unsafe { ffi::wxComboPopup_Init(self.as_ptr()) }
    }
    /// Utility method that returns true if Create has been called.
    ///
    /// See [C++ `wxComboPopup::IsCreated()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a8386b9f1c286671344fdbb23b5a52817).
    fn is_created(&self) -> bool {
        unsafe { ffi::wxComboPopup_IsCreated(self.as_ptr()) }
    }
    /// The derived class may implement this to return true if it wants to delay call to Create() until the popup is shown for the first time.
    ///
    /// See [C++ `wxComboPopup::LazyCreate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a3203d64b41c9689b0379314630324280).
    fn lazy_create(&self) -> bool {
        unsafe { ffi::wxComboPopup_LazyCreate(self.as_ptr()) }
    }
    /// The derived class may implement this to do something when the parent wxComboCtrl gets double-clicked.
    ///
    /// See [C++ `wxComboPopup::OnComboDoubleClick()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a26c6a69e340db96f47b88e2ecb296f2e).
    fn on_combo_double_click(&self) {
        unsafe { ffi::wxComboPopup_OnComboDoubleClick(self.as_ptr()) }
    }
    /// The derived class may implement this to receive key events from the parent wxComboCtrl.
    ///
    /// See [C++ `wxComboPopup::OnComboKeyEvent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a0cac7a0495f53119ad34583ad0591695).
    fn on_combo_key_event<K: KeyEventMethods>(&self, event: &K) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxComboPopup_OnComboKeyEvent(self.as_ptr(), event)
        }
    }
    /// The derived class may implement this to do special processing when popup is hidden.
    ///
    /// See [C++ `wxComboPopup::OnDismiss()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a24abde370c9f0faa8dc592a395797d7a).
    fn on_dismiss(&self) {
        unsafe { ffi::wxComboPopup_OnDismiss(self.as_ptr()) }
    }
    /// The derived class may implement this to do special processing when popup is shown.
    ///
    /// See [C++ `wxComboPopup::OnPopup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#a1a9e2d8553b1ed3641109f45ab7bfe8d).
    fn on_popup(&self) {
        unsafe { ffi::wxComboPopup_OnPopup(self.as_ptr()) }
    }
    /// The derived class may implement this to paint the parent wxComboCtrl.
    ///
    /// See [C++ `wxComboPopup::PaintComboControl()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#ae64038e6f785d2c6b4826991f0f75b18).
    fn paint_combo_control<D: DCMethods, R: RectMethods>(&self, dc: &D, rect: &R) {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxComboPopup_PaintComboControl(self.as_ptr(), dc, rect)
        }
    }
    /// The derived class must implement this to receive string value changes from wxComboCtrl.
    ///
    /// See [C++ `wxComboPopup::SetStringValue()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_combo_popup.html#ad55221c409f2e1edbfcbf75719dc3864).
    fn set_string_value(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboPopup_SetStringValue(self.as_ptr(), value)
        }
    }
}

// wxCommand
/// This trait represents [C++ `wxCommand` class](https://docs.wxwidgets.org/3.2/classwx_command.html)'s methods and inheritance.
///
/// See [`CommandInRust`] documentation for the class usage.
pub trait CommandMethods: ObjectMethods {
    // DTOR: fn ~wxCommand()
    /// Returns true if the command can be undone, false otherwise.
    ///
    /// See [C++ `wxCommand::CanUndo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command.html#ace7a97cc386f7908833949a4402d4eab).
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxCommand_CanUndo(self.as_ptr()) }
    }
    /// Override this member function to execute the appropriate action when called.
    ///
    /// See [C++ `wxCommand::Do()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command.html#a858fb8c1b0ab1d3400b9bc2605a5e5f7).
    fn do_(&self) -> bool {
        unsafe { ffi::wxCommand_Do(self.as_ptr()) }
    }
    /// Returns the command name.
    ///
    /// See [C++ `wxCommand::GetName()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command.html#a4d2dfba5e33420ff78331f9cb364c820).
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommand_GetName(self.as_ptr())).into() }
    }
    /// Override this member function to un-execute a previous Do.
    ///
    /// See [C++ `wxCommand::Undo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command.html#afed9df8cfda3579429ba8353098859dc).
    fn undo(&self) -> bool {
        unsafe { ffi::wxCommand_Undo(self.as_ptr()) }
    }
}

// wxCommandEvent
/// This trait represents [C++ `wxCommandEvent` class](https://docs.wxwidgets.org/3.2/classwx_command_event.html)'s methods and inheritance.
///
/// See [`CommandEventInRust`] documentation for the class usage.
pub trait CommandEventMethods: EventMethods {
    /// Returns client data pointer for a listbox or choice selection event (not valid for a deselection).
    ///
    /// See [C++ `wxCommandEvent::GetClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a3d41eef5b753ff2fd0b822bd1e0c6af9).
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientData(self.as_ptr()) }
    }
    /// Returns client object pointer for a listbox or choice selection event (not valid for a deselection).
    ///
    /// See [C++ `wxCommandEvent::GetClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a48e7b34762b82cdc9d1b5d58d1e6a17f).
    fn get_client_object(&self) -> Option<ClientDataInRust<false>> {
        unsafe { ClientData::option_from(ffi::wxCommandEvent_GetClientObject(self.as_ptr())) }
    }
    /// Returns extra information dependent on the event objects type.
    ///
    /// See [C++ `wxCommandEvent::GetExtraLong()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#abdba7c3e93f0dbf99f5382e60e7a461a).
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxCommandEvent_GetExtraLong(self.as_ptr()) }
    }
    /// Returns the integer identifier corresponding to a listbox, choice or radiobox selection (only if the event was a selection, not a deselection), or a boolean value representing the value of a checkbox.
    ///
    /// See [C++ `wxCommandEvent::GetInt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a24ad2aab26646c762ca3092a7ffd5ccd).
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetInt(self.as_ptr()) }
    }
    /// Returns item index for a listbox or choice selection event (not valid for a deselection).
    ///
    /// See [C++ `wxCommandEvent::GetSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a93609c04b8fd7001259efb5d65645fe6).
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetSelection(self.as_ptr()) }
    }
    /// Returns item string for a listbox or choice selection event.
    ///
    /// See [C++ `wxCommandEvent::GetString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a6aef039e72e358840a7c20da6473e9ae).
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandEvent_GetString(self.as_ptr())).into() }
    }
    /// This method can be used with checkbox and menu events: for the checkboxes, the method returns true for a selection event and false for a deselection one.
    ///
    /// See [C++ `wxCommandEvent::IsChecked()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a3e5510a00ebb9fb5e7bf7dc5e89f3588).
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsChecked(self.as_ptr()) }
    }
    /// For a listbox or similar event, returns true if it is a selection, false if it is a deselection.
    ///
    /// See [C++ `wxCommandEvent::IsSelection()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a44f4144f027fcb7feadb4c23bcbfeb5f).
    fn is_selection(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsSelection(self.as_ptr()) }
    }
    /// Sets the client data for this event.
    ///
    /// See [C++ `wxCommandEvent::SetClientData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a1778d16766134e08cafcdb49b06855ce).
    fn set_client_data(&self, client_data: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientData(self.as_ptr(), client_data) }
    }
    /// Sets the client object for this event.
    ///
    /// See [C++ `wxCommandEvent::SetClientObject()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a752e5a0fb8992b062c4dec6866171586).
    fn set_client_object<C: ClientDataMethods>(&self, client_object: Option<&C>) {
        unsafe {
            let client_object = match client_object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandEvent_SetClientObject(self.as_ptr(), client_object)
        }
    }
    /// Sets the m_extraLong member.
    ///
    /// See [C++ `wxCommandEvent::SetExtraLong()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a13ac3c581b99d110bac2c6b74803d2df).
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxCommandEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    /// Sets the m_commandInt member.
    ///
    /// See [C++ `wxCommandEvent::SetInt()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#aeaed91ffb1d02f07d7ea40f029f95a7c).
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxCommandEvent_SetInt(self.as_ptr(), int_command) }
    }
    /// Sets the m_commandString member.
    ///
    /// See [C++ `wxCommandEvent::SetString()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_event.html#a06ca56ac6680fe3b3178d8abd913d450).
    fn set_string(&self, string: &str) {
        unsafe {
            let string = WxString::from(string);
            let string = string.as_ptr();
            ffi::wxCommandEvent_SetString(self.as_ptr(), string)
        }
    }
}

// wxCommandLinkButton
/// This trait represents [C++ `wxCommandLinkButton` class](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html)'s methods and inheritance.
///
/// See [`CommandLinkButtonInRust`] documentation for the class usage.
pub trait CommandLinkButtonMethods: ButtonMethods {
    /// Button creation function for two-step creation.
    ///
    /// See [C++ `wxCommandLinkButton::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a7e31d9eaa8da537f9509ea5bd9df2600).
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
    /// Sets a new main label and note for the button.
    ///
    /// See [C++ `wxCommandLinkButton::SetMainLabelAndNote()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a0e496ccc683c5060796415d108381597).
    fn set_main_label_and_note(&self, main_label: &str, note: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabelAndNote(self.as_ptr(), main_label, note)
        }
    }
    /// Changes the main label.
    ///
    /// See [C++ `wxCommandLinkButton::SetMainLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a6cecd5ecdb7eff45519d02a6766c5b07).
    fn set_main_label(&self, main_label: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabel(self.as_ptr(), main_label)
        }
    }
    /// Changes the note.
    ///
    /// See [C++ `wxCommandLinkButton::SetNote()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a67c8e23fe483fed7ea7e9e337d880c90).
    fn set_note(&self, note: &str) {
        unsafe {
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetNote(self.as_ptr(), note)
        }
    }
    /// Returns the current main label.
    ///
    /// See [C++ `wxCommandLinkButton::GetMainLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#ae8ef37ce37239c678d2ad08f8074b507).
    fn get_main_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetMainLabel(self.as_ptr())).into() }
    }
    /// Returns the currently used note.
    ///
    /// See [C++ `wxCommandLinkButton::GetNote()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_link_button.html#a85330e2fe839cf011bde99a916c6a5fd).
    fn get_note(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetNote(self.as_ptr())).into() }
    }
}

// wxCommandProcessor
/// This trait represents [C++ `wxCommandProcessor` class](https://docs.wxwidgets.org/3.2/classwx_command_processor.html)'s methods and inheritance.
///
/// See [`CommandProcessorInRust`] documentation for the class usage.
pub trait CommandProcessorMethods: ObjectMethods {
    // DTOR: fn ~wxCommandProcessor()
    /// Returns true if the currently-active command can be undone, false otherwise.
    ///
    /// See [C++ `wxCommandProcessor::CanUndo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a69e81fc811ca19833bd7b684ecf2da12).
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_CanUndo(self.as_ptr()) }
    }
    /// Returns true if the currently-active command can be redone, false otherwise.
    ///
    /// See [C++ `wxCommandProcessor::CanRedo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#aeb9ad26a1ad14936db1bb1497e58e59a).
    fn can_redo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_CanRedo(self.as_ptr()) }
    }
    /// Deletes all commands in the list and sets the current command pointer to NULL.
    ///
    /// See [C++ `wxCommandProcessor::ClearCommands()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a9d315d50c36aa91195f7954984b4cd9d).
    fn clear_commands(&self) {
        unsafe { ffi::wxCommandProcessor_ClearCommands(self.as_ptr()) }
    }
    // BLOCKED: fn GetCommands()
    /// Returns the current command.
    ///
    /// See [C++ `wxCommandProcessor::GetCurrentCommand()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a026a00fa665813b2ae5ffb64d8d90b0e).
    fn get_current_command(&self) -> Option<CommandInRust<false>> {
        unsafe { Command::option_from(ffi::wxCommandProcessor_GetCurrentCommand(self.as_ptr())) }
    }
    /// Returns the edit menu associated with the command processor.
    ///
    /// See [C++ `wxCommandProcessor::GetEditMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#ab8e304fbf624ca6a4848decfd18e2d2a).
    fn get_edit_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxCommandProcessor_GetEditMenu(self.as_ptr())) }
    }
    /// Returns the maximum number of commands that the command processor stores.
    ///
    /// See [C++ `wxCommandProcessor::GetMaxCommands()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a9f540ccdea952ee0f80301c5b179579d).
    fn get_max_commands(&self) -> c_int {
        unsafe { ffi::wxCommandProcessor_GetMaxCommands(self.as_ptr()) }
    }
    /// Returns the string that will be appended to the Redo menu item.
    ///
    /// See [C++ `wxCommandProcessor::GetRedoAccelerator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a977ea62bc71c6b87ed91ba80b1de9e63).
    fn get_redo_accelerator(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetRedoAccelerator(self.as_ptr())).into()
        }
    }
    /// Returns the string that will be shown for the redo menu item.
    ///
    /// See [C++ `wxCommandProcessor::GetRedoMenuLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#aecf786d0eaf85b36ed83f3296dbf97b2).
    fn get_redo_menu_label(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetRedoMenuLabel(self.as_ptr())).into()
        }
    }
    /// Returns the string that will be appended to the Undo menu item.
    ///
    /// See [C++ `wxCommandProcessor::GetUndoAccelerator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a64e31026ba6ea99d5e58b2f7ac73cf95).
    fn get_undo_accelerator(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetUndoAccelerator(self.as_ptr())).into()
        }
    }
    /// Returns the string that will be shown for the undo menu item.
    ///
    /// See [C++ `wxCommandProcessor::GetUndoMenuLabel()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#abd868af91e716f06d46e936fdb682ff9).
    fn get_undo_menu_label(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetUndoMenuLabel(self.as_ptr())).into()
        }
    }
    /// Initializes the command processor, setting the current command to the last in the list (if any), and updating the edit menu (if one has been specified).
    ///
    /// See [C++ `wxCommandProcessor::Initialize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a2ca9168da9f416cfd39307f4d9686612).
    fn initialize(&self) {
        unsafe { ffi::wxCommandProcessor_Initialize(self.as_ptr()) }
    }
    /// Returns a boolean value that indicates if changes have been made since the last save operation.
    ///
    /// See [C++ `wxCommandProcessor::IsDirty()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a5d4297924743ae662c4bda5150682215).
    fn is_dirty(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_IsDirty(self.as_ptr()) }
    }
    /// You must call this method whenever the project is saved if you plan to use IsDirty().
    ///
    /// See [C++ `wxCommandProcessor::MarkAsSaved()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a5ab022fc24cbc1ebd86a0c2edb4b4de8).
    fn mark_as_saved(&self) {
        unsafe { ffi::wxCommandProcessor_MarkAsSaved(self.as_ptr()) }
    }
    /// Executes (redoes) the current command (the command that has just been undone if any).
    ///
    /// See [C++ `wxCommandProcessor::Redo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a5f51b031f29220385f7e270f7d08afc4).
    fn redo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_Redo(self.as_ptr()) }
    }
    /// Tells the command processor to update the Undo and Redo items on this menu as appropriate.
    ///
    /// See [C++ `wxCommandProcessor::SetEditMenu()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#ac165f00d1d87bf0814be4156503ef954).
    fn set_edit_menu<M: MenuMethods>(&self, menu: Option<&M>) {
        unsafe {
            let menu = match menu {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandProcessor_SetEditMenu(self.as_ptr(), menu)
        }
    }
    /// Sets the menu labels according to the currently set menu and the current command state.
    ///
    /// See [C++ `wxCommandProcessor::SetMenuStrings()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a186f43c64bc8154da368504ccc83cfc7).
    fn set_menu_strings(&self) {
        unsafe { ffi::wxCommandProcessor_SetMenuStrings(self.as_ptr()) }
    }
    /// Sets the string that will be appended to the Redo menu item.
    ///
    /// See [C++ `wxCommandProcessor::SetRedoAccelerator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a1561f0d451067751bcd0d774213fb365).
    fn set_redo_accelerator(&self, accel: &str) {
        unsafe {
            let accel = WxString::from(accel);
            let accel = accel.as_ptr();
            ffi::wxCommandProcessor_SetRedoAccelerator(self.as_ptr(), accel)
        }
    }
    /// Sets the string that will be appended to the Undo menu item.
    ///
    /// See [C++ `wxCommandProcessor::SetUndoAccelerator()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a258a7fd2a1250220e119849eafe55934).
    fn set_undo_accelerator(&self, accel: &str) {
        unsafe {
            let accel = WxString::from(accel);
            let accel = accel.as_ptr();
            ffi::wxCommandProcessor_SetUndoAccelerator(self.as_ptr(), accel)
        }
    }
    /// Submits a new command to the command processor.
    ///
    /// See [C++ `wxCommandProcessor::Submit()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a5c4a270152e0ca5d57d4e0c0c5e9e34a).
    fn submit<C: CommandMethods>(&self, command: Option<&C>, store_it: bool) -> bool {
        unsafe {
            let command = match command {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandProcessor_Submit(self.as_ptr(), command, store_it)
        }
    }
    /// Just store the command without executing it.
    ///
    /// See [C++ `wxCommandProcessor::Store()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#a3b3264402f092540ad075aba0f4d2352).
    fn store<C: CommandMethods>(&self, command: Option<&C>) {
        unsafe {
            let command = match command {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxCommandProcessor_Store(self.as_ptr(), command)
        }
    }
    /// Undoes the last command executed.
    ///
    /// See [C++ `wxCommandProcessor::Undo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_command_processor.html#af26a98ba5b9d0a3f9c290714aab28891).
    fn undo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_Undo(self.as_ptr()) }
    }
}

// wxContextMenuEvent
/// This trait represents [C++ `wxContextMenuEvent` class](https://docs.wxwidgets.org/3.2/classwx_context_menu_event.html)'s methods and inheritance.
///
/// See [`ContextMenuEventInRust`] documentation for the class usage.
pub trait ContextMenuEventMethods: CommandEventMethods {
    /// Returns the position in screen coordinates at which the menu should be shown.
    ///
    /// See [C++ `wxContextMenuEvent::GetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_context_menu_event.html#a6017c932b59f627bc62dbc2f2347f940).
    fn get_position(&self) -> PointInRust<false> {
        unsafe { PointInRust::from_ptr(ffi::wxContextMenuEvent_GetPosition(self.as_ptr())) }
    }
    /// Sets the position at which the menu should be shown.
    ///
    /// See [C++ `wxContextMenuEvent::SetPosition()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_context_menu_event.html#a24d0fdf2c831940a85b4e80e2c5d15a8).
    fn set_position<P: PointMethods>(&self, point: &P) {
        unsafe {
            let point = point.as_ptr();
            ffi::wxContextMenuEvent_SetPosition(self.as_ptr(), point)
        }
    }
}

// wxControl
/// This trait represents [C++ `wxControl` class](https://docs.wxwidgets.org/3.2/classwx_control.html)'s methods and inheritance.
///
/// See [`ControlInRust`] documentation for the class usage.
pub trait ControlMethods: WindowMethods {
    ///
    /// See [C++ `wxControl::Create()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#abe23789c94c86907463a0e8434be822a).
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
    /// Simulates the effect of the user issuing a command to the item.
    ///
    /// See [C++ `wxControl::Command()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a8cd7ac81debaf506d6d146528c3d9a82).
    fn command<C: CommandEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxControl_Command(self.as_ptr(), event)
        }
    }
    /// Returns the control's label without mnemonics.
    ///
    /// See [C++ `wxControl::GetLabelText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a4ec6a7ee61553cd1df77d50491cee820).
    fn get_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxControl_GetLabelText(self.as_ptr())).into() }
    }
    /// Determine the size needed by the control to leave the given area for its text.
    ///
    /// See [C++ `wxControl::GetSizeFromTextSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a252c7408d6dfa8d70e8dbe88cea9695d).
    fn get_size_from_text_size_int(&self, xlen: c_int, ylen: c_int) -> Size {
        unsafe {
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize(
                self.as_ptr(),
                xlen,
                ylen,
            ))
        }
    }
    /// This is an overloaded member function, provided for convenience. It differs from the above function only in what argument(s) it accepts.
    ///
    /// See [C++ `wxControl::GetSizeFromTextSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a4d9169af074418aa3528f7b5fca61f94).
    fn get_size_from_text_size_size<S: SizeMethods>(&self, tsize: &S) -> Size {
        unsafe {
            let tsize = tsize.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr(), tsize))
        }
    }
    /// Determine the minimum size needed by the control to display the given text.
    ///
    /// See [C++ `wxControl::GetSizeFromText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a65e0bb72969361c14a0959f3aae57d61).
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromText(self.as_ptr(), text))
        }
    }
    /// Sets the control's label to exactly the given string.
    ///
    /// See [C++ `wxControl::SetLabelText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#ae092899c3fe658831a9c796755a65eb7).
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxControl_SetLabelText(self.as_ptr(), text)
        }
    }
    /// Sets the controls label to a string using markup.
    ///
    /// See [C++ `wxControl::SetLabelMarkup()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#afeb308dc3b54d8d735b33cb250395503).
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = WxString::from(markup);
            let markup = markup.as_ptr();
            ffi::wxControl_SetLabelMarkup(self.as_ptr(), markup)
        }
    }
    /// Returns the given label string without mnemonics ("&" characters).
    ///
    /// See [C++ `wxControl::GetLabelText()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#ae892fefe6b88168c158e11e975633665).
    fn get_label_text_str(label: &str) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WxString::from_ptr(ffi::wxControl_GetLabelText1(label)).into()
        }
    }
    /// Returns the given str string without mnemonics ("&" characters).
    ///
    /// See [C++ `wxControl::RemoveMnemonics()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#ab7df4ca0dfde3a6409833cf470ee02e6).
    fn remove_mnemonics(str: &str) -> String {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            WxString::from_ptr(ffi::wxControl_RemoveMnemonics(str)).into()
        }
    }
    /// Escapes the special mnemonics characters ("&") in the given string.
    ///
    /// See [C++ `wxControl::EscapeMnemonics()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#abdbd9c0684856546671e759dd95b23d4).
    fn escape_mnemonics(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxControl_EscapeMnemonics(text)).into()
        }
    }
    /// Replaces parts of the label string with ellipsis, if needed, so that it fits into maxWidth pixels if possible.
    ///
    /// See [C++ `wxControl::Ellipsize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_control.html#a0bb834cae2a8986aceddb89f84ef4ed1).
    fn ellipsize<D: DCMethods>(
        label: &str,
        dc: &D,
        mode: c_int,
        max_width: c_int,
        flags: c_int,
    ) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            let dc = dc.as_ptr();
            WxString::from_ptr(ffi::wxControl_Ellipsize(label, dc, mode, max_width, flags)).into()
        }
    }
}

// wxControlWithItems
/// This trait represents [C++ `wxControlWithItems` class](https://docs.wxwidgets.org/3.2/classwx_control_with_items.html)'s methods and inheritance.
///
/// See [`ControlWithItemsInRust`] documentation for the class usage.
pub trait ControlWithItemsMethods: ControlMethods {}

// wxCursor
/// This trait represents [C++ `wxCursor` class](https://docs.wxwidgets.org/3.2/classwx_cursor.html)'s methods and inheritance.
///
/// See [`CursorInRust`] documentation for the class usage.
pub trait CursorMethods: GDIObjectMethods {
    // DTOR: fn ~wxCursor()
    /// Returns true if cursor data is present.
    ///
    /// See [C++ `wxCursor::IsOk()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#a46bfc04111995a2492053c34dfb9aa23).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCursor_IsOk(self.as_ptr()) }
    }
    /// Returns the coordinates of the cursor hot spot.
    ///
    /// See [C++ `wxCursor::GetHotSpot()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_cursor.html#aa0339e59355eeb2ef0a1fe955a25f23d).
    fn get_hot_spot(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCursor_GetHotSpot(self.as_ptr())) }
    }
    // BLOCKED: fn operator=()
}

// wxCustomDataObject
/// This trait represents [C++ `wxCustomDataObject` class](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html)'s methods and inheritance.
///
/// See [`CustomDataObjectInRust`] documentation for the class usage.
pub trait CustomDataObjectMethods: DataObjectSimpleMethods {
    // DTOR: fn ~wxCustomDataObject()
    /// This function is called to allocate size bytes of memory from SetData().
    ///
    /// See [C++ `wxCustomDataObject::Alloc()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a29cee14d29cb4b82ac487b62506e5699).
    fn alloc(&self, size: usize) -> *mut c_void {
        unsafe { ffi::wxCustomDataObject_Alloc(self.as_ptr(), size) }
    }
    /// This function is called when the data is freed, you may override it to anything you want (or may be nothing at all).
    ///
    /// See [C++ `wxCustomDataObject::Free()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a4978a39c76eaba4fc880aea751139b1c).
    fn free(&self) {
        unsafe { ffi::wxCustomDataObject_Free(self.as_ptr()) }
    }
    /// Returns a pointer to the data.
    ///
    /// See [C++ `wxCustomDataObject::GetData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a6cfcfae971116cbec934a0d9282a048a).
    fn get_data(&self) -> *mut c_void {
        unsafe { ffi::wxCustomDataObject_GetData(self.as_ptr()) }
    }
    /// Returns the data size in bytes.
    ///
    /// See [C++ `wxCustomDataObject::GetSize()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#ac04d7b07187b5cd4abe469bab536cdac).
    fn get_size(&self) -> usize {
        unsafe { ffi::wxCustomDataObject_GetSize(self.as_ptr()) }
    }
    /// Like SetData(), but doesn't copy the data - instead the object takes ownership of the pointer.
    ///
    /// See [C++ `wxCustomDataObject::TakeData()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_custom_data_object.html#a133a6a8044d298b68b50945c422ae4a8).
    fn take_data(&self, size: usize, data: *mut c_void) {
        unsafe { ffi::wxCustomDataObject_TakeData(self.as_ptr(), size, data) }
    }
}
