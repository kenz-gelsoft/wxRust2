use super::*;

// wxCalculateLayoutEvent
pub trait CalculateLayoutEventMethods: EventMethods {
    /// Returns the flags associated with this event.
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxCalculateLayoutEvent_GetFlags(self.as_ptr()) }
    }
    /// Before the event handler is entered, returns the remaining parent client area that the window could occupy.
    fn get_rect(&self) -> Rect {
        unsafe { Rect::from_ptr(ffi::wxCalculateLayoutEvent_GetRect(self.as_ptr())) }
    }
    /// Sets the flags associated with this event.
    fn set_flags(&self, flags: c_int) {
        unsafe { ffi::wxCalculateLayoutEvent_SetFlags(self.as_ptr(), flags) }
    }
    /// Call this to specify the new remaining parent client area, after the space occupied by the window has been subtracted.
    fn set_rect<R: RectMethods>(&self, rect: &R) {
        unsafe {
            let rect = rect.as_ptr();
            ffi::wxCalculateLayoutEvent_SetRect(self.as_ptr(), rect)
        }
    }
}

// wxCalendarCtrl
pub trait CalendarCtrlMethods: ControlMethods {
    /// Restrict the dates that can be selected in the control to the specified range.
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
    fn enable_holiday_display(&self, display: bool) {
        unsafe { ffi::wxCalendarCtrl_EnableHolidayDisplay(self.as_ptr(), display) }
    }
    /// This function should be used instead of changing wxCAL_NO_MONTH_CHANGE style bit.
    fn enable_month_change(&self, enable: bool) -> bool {
        unsafe { ffi::wxCalendarCtrl_EnableMonthChange(self.as_ptr(), enable) }
    }
    // BLOCKED: fn EnableYearChange()
    /// Returns the attribute for the given date (should be in the range 1...31).
    fn get_attr(&self, day: usize) -> Option<CalendarDateAttrIsOwned<false>> {
        unsafe { CalendarDateAttr::option_from(ffi::wxCalendarCtrl_GetAttr(self.as_ptr(), day)) }
    }
    /// Gets the currently selected date.
    fn get_date(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxCalendarCtrl_GetDate(self.as_ptr())) }
    }
    /// Gets the background colour of the header part of the calendar window.
    fn get_header_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourBg(self.as_ptr())) }
    }
    /// Gets the foreground colour of the header part of the calendar window.
    fn get_header_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHeaderColourFg(self.as_ptr())) }
    }
    /// Gets the background highlight colour.
    fn get_highlight_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourBg(self.as_ptr())) }
    }
    /// Gets the foreground highlight colour.
    fn get_highlight_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHighlightColourFg(self.as_ptr())) }
    }
    /// Return the background colour currently used for holiday highlighting.
    fn get_holiday_colour_bg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourBg(self.as_ptr())) }
    }
    /// Return the foreground colour currently used for holiday highlighting.
    fn get_holiday_colour_fg(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarCtrl_GetHolidayColourFg(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn HitTest()
    /// Clears any attributes associated with the given day (in the range 1...31).
    fn reset_attr(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_ResetAttr(self.as_ptr(), day) }
    }
    /// Associates the attribute with the specified date (in the range 1...31).
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
    fn set_date<D: DateTimeMethods>(&self, date: &D) -> bool {
        unsafe {
            let date = date.as_ptr();
            ffi::wxCalendarCtrl_SetDate(self.as_ptr(), date)
        }
    }
    /// Set the colours used for painting the weekdays at the top of the control.
    fn set_header_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHeaderColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    /// Set the colours to be used for highlighting the currently selected date.
    fn set_highlight_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHighlightColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    /// Marks the specified day as being a holiday in the current month.
    fn set_holiday(&self, day: usize) {
        unsafe { ffi::wxCalendarCtrl_SetHoliday(self.as_ptr(), day) }
    }
    /// Sets the colours to be used for the holidays highlighting.
    fn set_holiday_colours<C: ColourMethods, C2: ColourMethods>(&self, col_fg: &C, col_bg: &C2) {
        unsafe {
            let col_fg = col_fg.as_ptr();
            let col_bg = col_bg.as_ptr();
            ffi::wxCalendarCtrl_SetHolidayColours(self.as_ptr(), col_fg, col_bg)
        }
    }
    /// Mark or unmark the day.
    fn mark(&self, day: usize, mark: bool) {
        unsafe { ffi::wxCalendarCtrl_Mark(self.as_ptr(), day, mark) }
    }
}

// wxCalendarDateAttr
pub trait CalendarDateAttrMethods: WxRustMethods {
    /// Returns the background colour set for the calendar date.
    fn get_background_colour(&self) -> ColourIsOwned<false> {
        unsafe {
            ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetBackgroundColour(self.as_ptr()))
        }
    }
    // NOT_SUPPORTED: fn GetBorder()
    /// Returns the border colour set for the calendar date.
    fn get_border_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetBorderColour(self.as_ptr())) }
    }
    /// Returns the font set for the calendar date.
    fn get_font(&self) -> FontIsOwned<false> {
        unsafe { FontIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetFont(self.as_ptr())) }
    }
    /// Returns the text colour set for the calendar date.
    fn get_text_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetTextColour(self.as_ptr())) }
    }
    /// Returns true if a non-default text background colour is set.
    fn has_background_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBackgroundColour(self.as_ptr()) }
    }
    /// Returns true if a non-default (i.e. any) border is set.
    fn has_border(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorder(self.as_ptr()) }
    }
    /// Returns true if a non-default border colour is set.
    fn has_border_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasBorderColour(self.as_ptr()) }
    }
    /// Returns true if a non-default font is set.
    fn has_font(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasFont(self.as_ptr()) }
    }
    /// Returns true if a non-default text foreground colour is set.
    fn has_text_colour(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_HasTextColour(self.as_ptr()) }
    }
    /// Returns true if this calendar day is displayed as a holiday.
    fn is_holiday(&self) -> bool {
        unsafe { ffi::wxCalendarDateAttr_IsHoliday(self.as_ptr()) }
    }
    /// Sets the text background colour to use.
    fn set_background_colour<C: ColourMethods>(&self, col_back: &C) {
        unsafe {
            let col_back = col_back.as_ptr();
            ffi::wxCalendarDateAttr_SetBackgroundColour(self.as_ptr(), col_back)
        }
    }
    // NOT_SUPPORTED: fn SetBorder()
    /// Sets the border colour to use.
    fn set_border_colour<C: ColourMethods>(&self, col: &C) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxCalendarDateAttr_SetBorderColour(self.as_ptr(), col)
        }
    }
    /// Sets the font to use.
    fn set_font<F: FontMethods>(&self, font: &F) {
        unsafe {
            let font = font.as_ptr();
            ffi::wxCalendarDateAttr_SetFont(self.as_ptr(), font)
        }
    }
    /// If holiday is true, this calendar day will be displayed as a holiday.
    fn set_holiday(&self, holiday: bool) {
        unsafe { ffi::wxCalendarDateAttr_SetHoliday(self.as_ptr(), holiday) }
    }
    /// Sets the text (foreground) colour to use.
    fn set_text_colour<C: ColourMethods>(&self, col_text: &C) {
        unsafe {
            let col_text = col_text.as_ptr();
            ffi::wxCalendarDateAttr_SetTextColour(self.as_ptr(), col_text)
        }
    }
    /// Used (internally) by the generic wxCalendarCtrl::Mark().
    fn get_mark() -> CalendarDateAttrIsOwned<false> {
        unsafe { CalendarDateAttrIsOwned::from_ptr(ffi::wxCalendarDateAttr_GetMark()) }
    }
    /// Set the attributes that will be used to Mark() days on the generic wxCalendarCtrl.
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
    /// Creates a caret with the given size (in pixels) and associates it with the window (same as the equivalent constructors).
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
    /// Get the caret position (in pixels).
    fn get_position_int(&self, x: *mut c_void, y: *mut c_void) {
        unsafe { ffi::wxCaret_GetPosition(self.as_ptr(), x, y) }
    }
    fn get_position(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCaret_GetPosition1(self.as_ptr())) }
    }
    /// Get the caret size.
    fn get_size_int(&self, width: *mut c_void, height: *mut c_void) {
        unsafe { ffi::wxCaret_GetSize(self.as_ptr(), width, height) }
    }
    fn get_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxCaret_GetSize1(self.as_ptr())) }
    }
    /// Get the window the caret is associated with.
    fn get_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCaret_GetWindow(self.as_ptr())) }
    }
    /// Hides the caret, same as Show(false).
    fn hide(&self) {
        unsafe { ffi::wxCaret_Hide(self.as_ptr()) }
    }
    /// Returns true if the caret was created successfully.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCaret_IsOk(self.as_ptr()) }
    }
    /// Returns true if the caret is visible and false if it is permanently hidden (if it is blinking and not shown currently but will be after the next blink, this method still returns true).
    fn is_visible(&self) -> bool {
        unsafe { ffi::wxCaret_IsVisible(self.as_ptr()) }
    }
    /// Move the caret to given position (in logical coordinates).
    fn move_int(&self, x: c_int, y: c_int) {
        unsafe { ffi::wxCaret_Move(self.as_ptr(), x, y) }
    }
    fn move_point<P: PointMethods>(&self, pt: &P) {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxCaret_Move1(self.as_ptr(), pt)
        }
    }
    /// Changes the size of the caret.
    fn set_size_int(&self, width: c_int, height: c_int) {
        unsafe { ffi::wxCaret_SetSize(self.as_ptr(), width, height) }
    }
    fn set_size_size<S: SizeMethods>(&self, size: &S) {
        unsafe {
            let size = size.as_ptr();
            ffi::wxCaret_SetSize1(self.as_ptr(), size)
        }
    }
    /// Shows or hides the caret.
    fn show(&self, show: bool) {
        unsafe { ffi::wxCaret_Show(self.as_ptr(), show) }
    }
    /// Returns the blink time which is measured in milliseconds and is the time elapsed between 2 inversions of the caret (blink time of the caret is the same for all carets, so this functions is static).
    fn get_blink_time() -> c_int {
        unsafe { ffi::wxCaret_GetBlinkTime() }
    }
    /// Sets the blink time for all the carets.
    fn set_blink_time(milliseconds: c_int) {
        unsafe { ffi::wxCaret_SetBlinkTime(milliseconds) }
    }
}

// wxCheckBox
pub trait CheckBoxMethods: ControlMethods {
    // DTOR: fn ~wxCheckBox()
    /// Creates the checkbox for two-step construction.
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
    fn get_value(&self) -> bool {
        unsafe { ffi::wxCheckBox_GetValue(self.as_ptr()) }
    }
    /// Gets the state of a 3-state checkbox.
    fn get3_state_value(&self) -> c_int {
        unsafe { ffi::wxCheckBox_Get3StateValue(self.as_ptr()) }
    }
    /// Returns whether or not the checkbox is a 3-state checkbox.
    fn is3_state(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3State(self.as_ptr()) }
    }
    /// Returns whether or not the user can set the checkbox to the third state.
    fn is3rd_state_allowed_for_user(&self) -> bool {
        unsafe { ffi::wxCheckBox_Is3rdStateAllowedForUser(self.as_ptr()) }
    }
    /// This is just a maybe more readable synonym for GetValue(): just as the latter, it returns true if the checkbox is checked and false otherwise.
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCheckBox_IsChecked(self.as_ptr()) }
    }
    /// Sets the checkbox to the given state.
    fn set_value(&self, state: bool) {
        unsafe { ffi::wxCheckBox_SetValue(self.as_ptr(), state) }
    }
    /// Sets the checkbox to the given state.
    fn set3_state_value(&self, state: c_int) {
        unsafe { ffi::wxCheckBox_Set3StateValue(self.as_ptr(), state) }
    }
}

// wxCheckListBox
pub trait CheckListBoxMethods: ListBoxMethods {
    // DTOR: fn ~wxCheckListBox()
    /// Checks the given item.
    fn check(&self, item: c_uint, check: bool) {
        unsafe { ffi::wxCheckListBox_Check(self.as_ptr(), item, check) }
    }
    /// Returns true if the given item is checked, false otherwise.
    fn is_checked(&self, item: c_uint) -> bool {
        unsafe { ffi::wxCheckListBox_IsChecked(self.as_ptr(), item) }
    }
    /// Return the indices of the checked items.
    fn get_checked_items<A: ArrayIntMethods>(&self, checked_items: &A) -> c_uint {
        unsafe {
            let checked_items = checked_items.as_ptr();
            ffi::wxCheckListBox_GetCheckedItems(self.as_ptr(), checked_items)
        }
    }
}

// wxChildFocusEvent
pub trait ChildFocusEventMethods: CommandEventMethods {
    /// Returns the direct child which receives the focus, or a (grand-)parent of the control receiving the focus.
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
    /// Gets the number of columns in this choice item.
    fn get_columns(&self) -> c_int {
        unsafe { ffi::wxChoice_GetColumns(self.as_ptr()) }
    }
    /// Unlike wxControlWithItems::GetSelection() which only returns the accepted selection value (the selection in the control once the user closes the dropdown list), this function returns the current selection.
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxChoice_GetCurrentSelection(self.as_ptr()) }
    }
    /// Sets the number of columns in this choice item.
    fn set_columns(&self, n: c_int) {
        unsafe { ffi::wxChoice_SetColumns(self.as_ptr(), n) }
    }
    fn is_sorted(&self) -> bool {
        unsafe { ffi::wxChoice_IsSorted(self.as_ptr()) }
    }
}

// wxChoicebook
pub trait ChoicebookMethods: BookCtrlBaseMethods {
    /// Returns the wxChoice associated with the control.
    fn get_choice_ctrl(&self) -> WeakRef<Choice> {
        unsafe { WeakRef::<Choice>::from(ffi::wxChoicebook_GetChoiceCtrl(self.as_ptr())) }
    }
}

// wxClientDC
pub trait ClientDCMethods: WindowDCMethods {}

// wxClipboard
pub trait ClipboardMethods: ObjectMethods {
    // DTOR: fn ~wxClipboard()
    /// Call this function to add the data object to the clipboard.
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
    fn clear(&self) {
        unsafe { ffi::wxClipboard_Clear(self.as_ptr()) }
    }
    /// Call this function to close the clipboard, having opened it with Open().
    fn close(&self) {
        unsafe { ffi::wxClipboard_Close(self.as_ptr()) }
    }
    /// Flushes the clipboard: this means that the data which is currently on clipboard will stay available even after the application exits (possibly eating memory), otherwise the clipboard will be emptied on exit.
    fn flush(&self) -> bool {
        unsafe { ffi::wxClipboard_Flush(self.as_ptr()) }
    }
    /// Call this function to fill data with data on the clipboard, if available in the required format.
    fn get_data<D: DataObjectMethods>(&self, data: &D) -> bool {
        unsafe {
            let data = data.as_ptr();
            ffi::wxClipboard_GetData(self.as_ptr(), data)
        }
    }
    /// Returns true if the clipboard has been opened.
    fn is_opened(&self) -> bool {
        unsafe { ffi::wxClipboard_IsOpened(self.as_ptr()) }
    }
    /// Returns true if there is data which matches the data format of the given data object currently available on the clipboard.
    fn is_supported<D: DataFormatMethods>(&self, format: &D) -> bool {
        unsafe {
            let format = format.as_ptr();
            ffi::wxClipboard_IsSupported(self.as_ptr(), format)
        }
    }
    /// Returns true if we are using the primary selection, false if clipboard one.
    fn is_using_primary_selection(&self) -> bool {
        unsafe { ffi::wxClipboard_IsUsingPrimarySelection(self.as_ptr()) }
    }
    /// Call this function to open the clipboard before calling SetData() and GetData().
    fn open(&self) -> bool {
        unsafe { ffi::wxClipboard_Open(self.as_ptr()) }
    }
    /// Call this function to set the data object to the clipboard.
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
    fn use_primary_selection(&self, primary: bool) {
        unsafe { ffi::wxClipboard_UsePrimarySelection(self.as_ptr(), primary) }
    }
    /// Returns the global instance (wxTheClipboard) of the clipboard object.
    fn get() -> Option<ClipboardIsOwned<false>> {
        unsafe { Clipboard::option_from(ffi::wxClipboard_Get()) }
    }
}

// wxClipboardTextEvent
pub trait ClipboardTextEventMethods: CommandEventMethods {}

// wxCloseEvent
pub trait CloseEventMethods: EventMethods {
    /// Returns true if you can veto a system shutdown or a window close event.
    fn can_veto(&self) -> bool {
        unsafe { ffi::wxCloseEvent_CanVeto(self.as_ptr()) }
    }
    /// Returns true if the user is just logging off or false if the system is shutting down.
    fn get_logging_off(&self) -> bool {
        unsafe { ffi::wxCloseEvent_GetLoggingOff(self.as_ptr()) }
    }
    /// Sets the 'can veto' flag.
    fn set_can_veto(&self, can_veto: bool) {
        unsafe { ffi::wxCloseEvent_SetCanVeto(self.as_ptr(), can_veto) }
    }
    /// Sets the 'logging off' flag.
    fn set_logging_off(&self, logging_off: bool) {
        unsafe { ffi::wxCloseEvent_SetLoggingOff(self.as_ptr(), logging_off) }
    }
    /// Call this from your event handler to veto a system shutdown or to signal to the calling application that a window close did not happen.
    fn veto(&self, veto: bool) {
        unsafe { ffi::wxCloseEvent_Veto(self.as_ptr(), veto) }
    }
    /// Returns whether the Veto flag was set.
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
    /// Collapses or expands the pane window.
    fn collapse(&self, collapse: bool) {
        unsafe { ffi::wxCollapsiblePane_Collapse(self.as_ptr(), collapse) }
    }
    /// Same as calling Collapse(false).
    fn expand(&self) {
        unsafe { ffi::wxCollapsiblePane_Expand(self.as_ptr()) }
    }
    /// Returns a pointer to the pane window.
    fn get_pane(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxCollapsiblePane_GetPane(self.as_ptr())) }
    }
    /// Returns true if the pane window is currently hidden.
    fn is_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsCollapsed(self.as_ptr()) }
    }
    /// Returns true if the pane window is currently shown.
    fn is_expanded(&self) -> bool {
        unsafe { ffi::wxCollapsiblePane_IsExpanded(self.as_ptr()) }
    }
}

// wxCollapsiblePaneEvent
pub trait CollapsiblePaneEventMethods: CommandEventMethods {
    /// Returns true if the pane has been collapsed.
    fn get_collapsed(&self) -> bool {
        unsafe { ffi::wxCollapsiblePaneEvent_GetCollapsed(self.as_ptr()) }
    }
    /// Sets this as a collapsed pane event (if collapsed is true) or as an expanded pane event (if collapsed is false).
    fn set_collapsed(&self, collapsed: bool) {
        unsafe { ffi::wxCollapsiblePaneEvent_SetCollapsed(self.as_ptr(), collapsed) }
    }
}

// wxColour
pub trait ColourMethods: ObjectMethods {
    // NOT_SUPPORTED: fn Alpha()
    // NOT_SUPPORTED: fn Blue()
    /// Returns the alpha value, on platforms where alpha is not yet supported, this always returns wxALPHA_OPAQUE.
    fn get_alpha(&self) -> c_uint {
        unsafe { ffi::wxColour_GetAlpha(self.as_ptr()) }
    }
    /// Returns the blue intensity as unsigned int.
    fn get_blue(&self) -> c_uint {
        unsafe { ffi::wxColour_GetBlue(self.as_ptr()) }
    }
    /// Returns the green intensity as unsigned int.
    fn get_green(&self) -> c_uint {
        unsafe { ffi::wxColour_GetGreen(self.as_ptr()) }
    }
    /// Returns the red intensity as unsigned int.
    fn get_red(&self) -> c_uint {
        unsafe { ffi::wxColour_GetRed(self.as_ptr()) }
    }
    /// Converts this colour to a wxString using the given flags.
    fn get_as_string(&self, flags: c_long) -> String {
        unsafe { WxString::from_ptr(ffi::wxColour_GetAsString(self.as_ptr(), flags)).into() }
    }
    // NOT_SUPPORTED: fn SetRGB()
    // NOT_SUPPORTED: fn SetRGBA()
    // NOT_SUPPORTED: fn GetRGB()
    // NOT_SUPPORTED: fn GetRGBA()
    /// Return the perceived brightness of the colour.
    fn get_luminance(&self) -> c_double {
        unsafe { ffi::wxColour_GetLuminance(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetPixel()
    // NOT_SUPPORTED: fn Green()
    /// Returns true if the colour object is valid (the colour has been initialised with RGB values).
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxColour_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Red()
    /// Returns true if the color can be described using RGB values, i.e.
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
    /// Assigns the same value to r, g, b: 0 if on is false, 255 otherwise.
    fn make_mono(r: *mut c_void, g: *mut c_void, b: *mut c_void, on: bool) {
        unsafe { ffi::wxColour_MakeMono(r, g, b, on) }
    }
    // NOT_SUPPORTED: fn MakeDisabled1()
    /// Create a grey colour from (in/out) rgb parameters using integer arithmetic.
    fn make_grey(r: *mut c_void, g: *mut c_void, b: *mut c_void) {
        unsafe { ffi::wxColour_MakeGrey(r, g, b) }
    }
    /// Create a grey colour from (in/out) rgb parameters using floating point arithmetic.
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
    fn change_lightness(r: *mut c_void, g: *mut c_void, b: *mut c_void, ialpha: c_int) {
        unsafe { ffi::wxColour_ChangeLightness1(r, g, b, ialpha) }
    }
}

// wxColourData
pub trait ColourDataMethods: ObjectMethods {
    // DTOR: fn ~wxColourData()
    /// Under Windows, determines whether the Windows colour dialog will display the full dialog with custom colour selection controls.
    fn get_choose_full(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseFull(self.as_ptr()) }
    }
    /// Indicates whether the colour dialog will display alpha values and an opacity selector.
    fn get_choose_alpha(&self) -> bool {
        unsafe { ffi::wxColourData_GetChooseAlpha(self.as_ptr()) }
    }
    /// Gets the current colour associated with the colour dialog.
    fn get_colour(&self) -> ColourIsOwned<false> {
        unsafe { ColourIsOwned::from_ptr(ffi::wxColourData_GetColour(self.as_ptr())) }
    }
    /// Returns custom colours associated with the colour dialog.
    fn get_custom_colour(&self, i: c_int) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourData_GetCustomColour(self.as_ptr(), i)) }
    }
    /// Under Windows, tells the Windows colour dialog to display the full dialog with custom colour selection controls.
    fn set_choose_full(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseFull(self.as_ptr(), flag) }
    }
    /// Tells the colour dialog to show alpha values and an opacity selector (slider).
    fn set_choose_alpha(&self, flag: bool) {
        unsafe { ffi::wxColourData_SetChooseAlpha(self.as_ptr(), flag) }
    }
    /// Sets the default colour for the colour dialog.
    fn set_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetColour(self.as_ptr(), colour)
        }
    }
    /// Sets custom colours for the colour dialog.
    fn set_custom_colour<C: ColourMethods>(&self, i: c_int, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxColourData_SetCustomColour(self.as_ptr(), i, colour)
        }
    }
    /// Converts the colours saved in this class in a string form, separating the various colours with a comma.
    fn to_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxColourData_ToString(self.as_ptr())).into() }
    }
    /// Decodes the given string, which should be in the same format returned by ToString(), and sets the internal colours.
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
    /// Adds a colour to the database.
    fn add_colour<C: ColourMethods>(&self, colour_name: &str, colour: &C) {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            let colour = colour.as_ptr();
            ffi::wxColourDatabase_AddColour(self.as_ptr(), colour_name, colour)
        }
    }
    /// Finds a colour given the name.
    fn find(&self, colour_name: &str) -> Colour {
        unsafe {
            let colour_name = WxString::from(colour_name);
            let colour_name = colour_name.as_ptr();
            Colour::from_ptr(ffi::wxColourDatabase_Find(self.as_ptr(), colour_name))
        }
    }
    /// Finds a colour name given the colour.
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
    /// Same as wxColourDialog().
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
    fn get_colour_data(&self) -> ColourDataIsOwned<false> {
        unsafe { ColourDataIsOwned::from_ptr(ffi::wxColourDialog_GetColourData(self.as_ptr())) }
    }
}

// wxColourPickerCtrl
pub trait ColourPickerCtrlMethods: PickerBaseMethods {
    /// Creates a colour picker with the given arguments.
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
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerCtrl_GetColour(self.as_ptr())) }
    }
    /// Sets the currently selected colour.
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
    /// Retrieve the colour the user has just selected.
    fn get_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxColourPickerEvent_GetColour(self.as_ptr())) }
    }
    /// Set the colour associated with the event.
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
    /// Returns the item being selected right now.
    fn get_current_selection(&self) -> c_int {
        unsafe { ffi::wxComboBox_GetCurrentSelection(self.as_ptr()) }
    }
    /// Returns true if the list of combobox choices is empty.
    fn is_list_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsListEmpty(self.as_ptr()) }
    }
    /// Returns true if the text of the combobox is empty.
    fn is_text_empty(&self) -> bool {
        unsafe { ffi::wxComboBox_IsTextEmpty(self.as_ptr()) }
    }
    /// Shows the list box portion of the combo box.
    fn popup(&self) {
        unsafe { ffi::wxComboBox_Popup(self.as_ptr()) }
    }
    /// Hides the list box portion of the combo box.
    fn dismiss(&self) {
        unsafe { ffi::wxComboBox_Dismiss(self.as_ptr()) }
    }
}

// wxComboCtrl
pub trait ComboCtrlMethods: ControlMethods {
    // DTOR: fn ~wxComboCtrl()
    /// Creates the combo control for two-step construction.
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
    fn dismiss(&self) {
        unsafe { ffi::wxComboCtrl_Dismiss(self.as_ptr()) }
    }
    /// Enables or disables popup animation, if any, depending on the value of the argument.
    fn enable_popup_animation(&self, enable: bool) {
        unsafe { ffi::wxComboCtrl_EnablePopupAnimation(self.as_ptr(), enable) }
    }
    /// Returns true if given key combination should toggle the popup.
    fn is_key_popup_toggle<K: KeyEventMethods>(&self, event: &K) -> bool {
        unsafe {
            let event = event.as_ptr();
            ffi::wxComboCtrl_IsKeyPopupToggle(self.as_ptr(), event)
        }
    }
    /// Prepare background of combo control or an item in a dropdown list in a way typical on platform.
    fn prepare_background<D: DCMethods, R: RectMethods>(&self, dc: &D, rect: &R, flags: c_int) {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxComboCtrl_PrepareBackground(self.as_ptr(), dc, rect, flags)
        }
    }
    /// Returns true if focus indicator should be drawn in the control.
    fn should_draw_focus(&self) -> bool {
        unsafe { ffi::wxComboCtrl_ShouldDrawFocus(self.as_ptr()) }
    }
    /// Returns disabled button bitmap that has been set with SetButtonBitmaps().
    fn get_bitmap_disabled(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapDisabled(self.as_ptr())) }
    }
    /// Returns button mouse hover bitmap that has been set with SetButtonBitmaps().
    fn get_bitmap_hover(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapHover(self.as_ptr())) }
    }
    /// Returns default button bitmap that has been set with SetButtonBitmaps().
    fn get_bitmap_normal(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapNormal(self.as_ptr())) }
    }
    /// Returns depressed button bitmap that has been set with SetButtonBitmaps().
    fn get_bitmap_pressed(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxComboCtrl_GetBitmapPressed(self.as_ptr())) }
    }
    /// Returns current size of the dropdown button.
    fn get_button_size(&self) -> Size {
        unsafe { Size::from_ptr(ffi::wxComboCtrl_GetButtonSize(self.as_ptr())) }
    }
    /// Returns custom painted area in control.
    fn get_custom_paint_width(&self) -> c_int {
        unsafe { ffi::wxComboCtrl_GetCustomPaintWidth(self.as_ptr()) }
    }
    /// Returns the margins used by the control.
    fn get_margins(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxComboCtrl_GetMargins(self.as_ptr())) }
    }
    /// Returns current popup interface that has been set with SetPopupControl().
    fn get_popup_control(&self) -> Option<ComboPopupIsOwned<false>> {
        unsafe { ComboPopup::option_from(ffi::wxComboCtrl_GetPopupControl(self.as_ptr())) }
    }
    /// Returns popup window containing the popup control.
    fn get_popup_window(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxComboCtrl_GetPopupWindow(self.as_ptr())) }
    }
    /// Get the text control which is part of the combo control.
    fn get_text_ctrl(&self) -> WeakRef<TextCtrl> {
        unsafe { WeakRef::<TextCtrl>::from(ffi::wxComboCtrl_GetTextCtrl(self.as_ptr())) }
    }
    // BLOCKED: fn GetTextIndent()
    /// Returns area covered by the text field (includes everything except borders and the dropdown button).
    fn get_text_rect(&self) -> RectIsOwned<false> {
        unsafe { RectIsOwned::from_ptr(ffi::wxComboCtrl_GetTextRect(self.as_ptr())) }
    }
    /// Dismisses the popup window.
    fn hide_popup(&self, generate_event: bool) {
        unsafe { ffi::wxComboCtrl_HidePopup(self.as_ptr(), generate_event) }
    }
    /// Returns true if the popup is currently shown.
    fn is_popup_shown(&self) -> bool {
        unsafe { ffi::wxComboCtrl_IsPopupShown(self.as_ptr()) }
    }
    /// Returns true if the popup window is in the given state.
    fn is_popup_window_state(&self, state: c_int) -> bool {
        unsafe { ffi::wxComboCtrl_IsPopupWindowState(self.as_ptr(), state) }
    }
    /// Implement in a derived class to define what happens on dropdown button click.
    fn on_button_click(&self) {
        unsafe { ffi::wxComboCtrl_OnButtonClick(self.as_ptr()) }
    }
    /// Shows the popup portion of the combo control.
    fn popup(&self) {
        unsafe { ffi::wxComboCtrl_Popup(self.as_ptr()) }
    }
    /// Sets custom dropdown button graphics.
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
    fn set_button_position(&self, width: c_int, height: c_int, side: c_int, spacing_x: c_int) {
        unsafe { ffi::wxComboCtrl_SetButtonPosition(self.as_ptr(), width, height, side, spacing_x) }
    }
    /// Set width, in pixels, of custom painted area in control without wxCB_READONLY style.
    fn set_custom_paint_width(&self, width: c_int) {
        unsafe { ffi::wxComboCtrl_SetCustomPaintWidth(self.as_ptr(), width) }
    }
    /// Uses the given window instead of the default text control as the main window of the combo control.
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
    fn set_margins_point<P: PointMethods>(&self, pt: &P) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            ffi::wxComboCtrl_SetMargins(self.as_ptr(), pt)
        }
    }
    fn set_margins_coord(&self, left: c_int, top: c_int) -> bool {
        unsafe { ffi::wxComboCtrl_SetMargins1(self.as_ptr(), left, top) }
    }
    /// Set side of the control to which the popup will align itself.
    fn set_popup_anchor(&self, anchor_side: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupAnchor(self.as_ptr(), anchor_side) }
    }
    /// Set popup interface class derived from wxComboPopup.
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
    fn set_popup_extents(&self, ext_left: c_int, ext_right: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupExtents(self.as_ptr(), ext_left, ext_right) }
    }
    /// Sets preferred maximum height of the popup.
    fn set_popup_max_height(&self, height: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupMaxHeight(self.as_ptr(), height) }
    }
    /// Sets minimum width of the popup.
    fn set_popup_min_width(&self, width: c_int) {
        unsafe { ffi::wxComboCtrl_SetPopupMinWidth(self.as_ptr(), width) }
    }
    /// Sets the text for the text field without affecting the popup.
    fn set_text(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboCtrl_SetText(self.as_ptr(), value)
        }
    }
    /// Set a custom window style for the embedded wxTextCtrl.
    fn set_text_ctrl_style(&self, style: c_int) {
        unsafe { ffi::wxComboCtrl_SetTextCtrlStyle(self.as_ptr(), style) }
    }
    // BLOCKED: fn SetTextIndent()
    /// Changes value of the control as if user had done it by selecting an item from a combo box drop-down list.
    fn set_value_by_user(&self, value: &str) {
        unsafe {
            let value = WxString::from(value);
            let value = value.as_ptr();
            ffi::wxComboCtrl_SetValueByUser(self.as_ptr(), value)
        }
    }
    /// Show the popup.
    fn show_popup(&self) {
        unsafe { ffi::wxComboCtrl_ShowPopup(self.as_ptr()) }
    }
    /// Enable or disable usage of an alternative popup window, which guarantees ability to focus the popup control, and allows common native controls to function normally.
    fn use_alt_popup_window(&self, enable: bool) {
        unsafe { ffi::wxComboCtrl_UseAltPopupWindow(self.as_ptr(), enable) }
    }
    /// Returns features supported by wxComboCtrl.
    fn get_features() -> c_int {
        unsafe { ffi::wxComboCtrl_GetFeatures() }
    }
}

// wxComboPopup
pub trait ComboPopupMethods: WxRustMethods {
    /// The derived class must implement this to create the popup control.
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
    fn destroy_popup(&self) {
        unsafe { ffi::wxComboPopup_DestroyPopup(self.as_ptr()) }
    }
    /// Utility function that hides the popup.
    fn dismiss(&self) {
        unsafe { ffi::wxComboPopup_Dismiss(self.as_ptr()) }
    }
    /// Implement to customize matching of value string to an item container entry.
    fn find_item(&self, item: &str, true_item: *mut c_void) -> bool {
        unsafe {
            let item = WxString::from(item);
            let item = item.as_ptr();
            ffi::wxComboPopup_FindItem(self.as_ptr(), item, true_item)
        }
    }
    /// The derived class may implement this to return adjusted size for the popup control, according to the variables given.
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
    fn get_combo_ctrl(&self) -> WeakRef<ComboCtrl> {
        unsafe { WeakRef::<ComboCtrl>::from(ffi::wxComboPopup_GetComboCtrl(self.as_ptr())) }
    }
    /// The derived class must implement this to return pointer to the associated control created in Create().
    fn get_control(&self) -> WeakRef<Window> {
        unsafe { WeakRef::<Window>::from(ffi::wxComboPopup_GetControl(self.as_ptr())) }
    }
    /// The derived class must implement this to return string representation of the value.
    fn get_string_value(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxComboPopup_GetStringValue(self.as_ptr())).into() }
    }
    /// The derived class must implement this to initialize its internal variables.
    fn init(&self) {
        unsafe { ffi::wxComboPopup_Init(self.as_ptr()) }
    }
    /// Utility method that returns true if Create has been called.
    fn is_created(&self) -> bool {
        unsafe { ffi::wxComboPopup_IsCreated(self.as_ptr()) }
    }
    /// The derived class may implement this to return true if it wants to delay call to Create() until the popup is shown for the first time.
    fn lazy_create(&self) -> bool {
        unsafe { ffi::wxComboPopup_LazyCreate(self.as_ptr()) }
    }
    /// The derived class may implement this to do something when the parent wxComboCtrl gets double-clicked.
    fn on_combo_double_click(&self) {
        unsafe { ffi::wxComboPopup_OnComboDoubleClick(self.as_ptr()) }
    }
    /// The derived class may implement this to receive key events from the parent wxComboCtrl.
    fn on_combo_key_event<K: KeyEventMethods>(&self, event: &K) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxComboPopup_OnComboKeyEvent(self.as_ptr(), event)
        }
    }
    /// The derived class may implement this to do special processing when popup is hidden.
    fn on_dismiss(&self) {
        unsafe { ffi::wxComboPopup_OnDismiss(self.as_ptr()) }
    }
    /// The derived class may implement this to do special processing when popup is shown.
    fn on_popup(&self) {
        unsafe { ffi::wxComboPopup_OnPopup(self.as_ptr()) }
    }
    /// The derived class may implement this to paint the parent wxComboCtrl.
    fn paint_combo_control<D: DCMethods, R: RectMethods>(&self, dc: &D, rect: &R) {
        unsafe {
            let dc = dc.as_ptr();
            let rect = rect.as_ptr();
            ffi::wxComboPopup_PaintComboControl(self.as_ptr(), dc, rect)
        }
    }
    /// The derived class must implement this to receive string value changes from wxComboCtrl.
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
    /// Returns true if the command can be undone, false otherwise.
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxCommand_CanUndo(self.as_ptr()) }
    }
    /// Override this member function to execute the appropriate action when called.
    fn do_(&self) -> bool {
        unsafe { ffi::wxCommand_Do(self.as_ptr()) }
    }
    /// Returns the command name.
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommand_GetName(self.as_ptr())).into() }
    }
    /// Override this member function to un-execute a previous Do.
    fn undo(&self) -> bool {
        unsafe { ffi::wxCommand_Undo(self.as_ptr()) }
    }
}

// wxCommandEvent
pub trait CommandEventMethods: EventMethods {
    /// Returns client data pointer for a listbox or choice selection event (not valid for a deselection).
    fn get_client_data(&self) -> *mut c_void {
        unsafe { ffi::wxCommandEvent_GetClientData(self.as_ptr()) }
    }
    /// Returns client object pointer for a listbox or choice selection event (not valid for a deselection).
    fn get_client_object(&self) -> Option<ClientDataIsOwned<false>> {
        unsafe { ClientData::option_from(ffi::wxCommandEvent_GetClientObject(self.as_ptr())) }
    }
    /// Returns extra information dependent on the event objects type.
    fn get_extra_long(&self) -> c_long {
        unsafe { ffi::wxCommandEvent_GetExtraLong(self.as_ptr()) }
    }
    /// Returns the integer identifier corresponding to a listbox, choice or radiobox selection (only if the event was a selection, not a deselection), or a boolean value representing the value of a checkbox.
    fn get_int(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetInt(self.as_ptr()) }
    }
    /// Returns item index for a listbox or choice selection event (not valid for a deselection).
    fn get_selection(&self) -> c_int {
        unsafe { ffi::wxCommandEvent_GetSelection(self.as_ptr()) }
    }
    /// Returns item string for a listbox or choice selection event.
    fn get_string(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandEvent_GetString(self.as_ptr())).into() }
    }
    /// This method can be used with checkbox and menu events: for the checkboxes, the method returns true for a selection event and false for a deselection one.
    fn is_checked(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsChecked(self.as_ptr()) }
    }
    /// For a listbox or similar event, returns true if it is a selection, false if it is a deselection.
    fn is_selection(&self) -> bool {
        unsafe { ffi::wxCommandEvent_IsSelection(self.as_ptr()) }
    }
    /// Sets the client data for this event.
    fn set_client_data(&self, client_data: *mut c_void) {
        unsafe { ffi::wxCommandEvent_SetClientData(self.as_ptr(), client_data) }
    }
    /// Sets the client object for this event.
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
    fn set_extra_long(&self, extra_long: c_long) {
        unsafe { ffi::wxCommandEvent_SetExtraLong(self.as_ptr(), extra_long) }
    }
    /// Sets the m_commandInt member.
    fn set_int(&self, int_command: c_int) {
        unsafe { ffi::wxCommandEvent_SetInt(self.as_ptr(), int_command) }
    }
    /// Sets the m_commandString member.
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
    /// Button creation function for two-step creation.
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
    fn set_main_label(&self, main_label: &str) {
        unsafe {
            let main_label = WxString::from(main_label);
            let main_label = main_label.as_ptr();
            ffi::wxCommandLinkButton_SetMainLabel(self.as_ptr(), main_label)
        }
    }
    /// Changes the note.
    fn set_note(&self, note: &str) {
        unsafe {
            let note = WxString::from(note);
            let note = note.as_ptr();
            ffi::wxCommandLinkButton_SetNote(self.as_ptr(), note)
        }
    }
    /// Returns the current main label.
    fn get_main_label(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetMainLabel(self.as_ptr())).into() }
    }
    /// Returns the currently used note.
    fn get_note(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxCommandLinkButton_GetNote(self.as_ptr())).into() }
    }
}

// wxCommandProcessor
pub trait CommandProcessorMethods: ObjectMethods {
    // DTOR: fn ~wxCommandProcessor()
    /// Returns true if the currently-active command can be undone, false otherwise.
    fn can_undo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_CanUndo(self.as_ptr()) }
    }
    /// Returns true if the currently-active command can be redone, false otherwise.
    fn can_redo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_CanRedo(self.as_ptr()) }
    }
    /// Deletes all commands in the list and sets the current command pointer to NULL.
    fn clear_commands(&self) {
        unsafe { ffi::wxCommandProcessor_ClearCommands(self.as_ptr()) }
    }
    // BLOCKED: fn GetCommands()
    /// Returns the current command.
    fn get_current_command(&self) -> Option<CommandIsOwned<false>> {
        unsafe { Command::option_from(ffi::wxCommandProcessor_GetCurrentCommand(self.as_ptr())) }
    }
    /// Returns the edit menu associated with the command processor.
    fn get_edit_menu(&self) -> WeakRef<Menu> {
        unsafe { WeakRef::<Menu>::from(ffi::wxCommandProcessor_GetEditMenu(self.as_ptr())) }
    }
    /// Returns the maximum number of commands that the command processor stores.
    fn get_max_commands(&self) -> c_int {
        unsafe { ffi::wxCommandProcessor_GetMaxCommands(self.as_ptr()) }
    }
    /// Returns the string that will be appended to the Redo menu item.
    fn get_redo_accelerator(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetRedoAccelerator(self.as_ptr())).into()
        }
    }
    /// Returns the string that will be shown for the redo menu item.
    fn get_redo_menu_label(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetRedoMenuLabel(self.as_ptr())).into()
        }
    }
    /// Returns the string that will be appended to the Undo menu item.
    fn get_undo_accelerator(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetUndoAccelerator(self.as_ptr())).into()
        }
    }
    /// Returns the string that will be shown for the undo menu item.
    fn get_undo_menu_label(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxCommandProcessor_GetUndoMenuLabel(self.as_ptr())).into()
        }
    }
    /// Initializes the command processor, setting the current command to the last in the list (if any), and updating the edit menu (if one has been specified).
    fn initialize(&self) {
        unsafe { ffi::wxCommandProcessor_Initialize(self.as_ptr()) }
    }
    /// Returns a boolean value that indicates if changes have been made since the last save operation.
    fn is_dirty(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_IsDirty(self.as_ptr()) }
    }
    /// You must call this method whenever the project is saved if you plan to use IsDirty().
    fn mark_as_saved(&self) {
        unsafe { ffi::wxCommandProcessor_MarkAsSaved(self.as_ptr()) }
    }
    /// Executes (redoes) the current command (the command that has just been undone if any).
    fn redo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_Redo(self.as_ptr()) }
    }
    /// Tells the command processor to update the Undo and Redo items on this menu as appropriate.
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
    fn set_menu_strings(&self) {
        unsafe { ffi::wxCommandProcessor_SetMenuStrings(self.as_ptr()) }
    }
    /// Sets the string that will be appended to the Redo menu item.
    fn set_redo_accelerator(&self, accel: &str) {
        unsafe {
            let accel = WxString::from(accel);
            let accel = accel.as_ptr();
            ffi::wxCommandProcessor_SetRedoAccelerator(self.as_ptr(), accel)
        }
    }
    /// Sets the string that will be appended to the Undo menu item.
    fn set_undo_accelerator(&self, accel: &str) {
        unsafe {
            let accel = WxString::from(accel);
            let accel = accel.as_ptr();
            ffi::wxCommandProcessor_SetUndoAccelerator(self.as_ptr(), accel)
        }
    }
    /// Submits a new command to the command processor.
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
    fn undo(&self) -> bool {
        unsafe { ffi::wxCommandProcessor_Undo(self.as_ptr()) }
    }
}

// wxContextMenuEvent
pub trait ContextMenuEventMethods: CommandEventMethods {
    /// Returns the position in screen coordinates at which the menu should be shown.
    fn get_position(&self) -> PointIsOwned<false> {
        unsafe { PointIsOwned::from_ptr(ffi::wxContextMenuEvent_GetPosition(self.as_ptr())) }
    }
    /// Sets the position at which the menu should be shown.
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
    /// Simulates the effect of the user issuing a command to the item.
    fn command<C: CommandEventMethods>(&self, event: &C) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxControl_Command(self.as_ptr(), event)
        }
    }
    /// Returns the control's label without mnemonics.
    fn get_label_text(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxControl_GetLabelText(self.as_ptr())).into() }
    }
    /// Determine the size needed by the control to leave the given area for its text.
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
    fn get_size_from_text_size_size<S: SizeMethods>(&self, tsize: &S) -> Size {
        unsafe {
            let tsize = tsize.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromTextSize1(self.as_ptr(), tsize))
        }
    }
    /// Determine the minimum size needed by the control to display the given text.
    fn get_size_from_text(&self, text: &str) -> Size {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            Size::from_ptr(ffi::wxControl_GetSizeFromText(self.as_ptr(), text))
        }
    }
    /// Sets the control's label to exactly the given string.
    fn set_label_text(&self, text: &str) {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            ffi::wxControl_SetLabelText(self.as_ptr(), text)
        }
    }
    /// Sets the controls label to a string using markup.
    fn set_label_markup(&self, markup: &str) -> bool {
        unsafe {
            let markup = WxString::from(markup);
            let markup = markup.as_ptr();
            ffi::wxControl_SetLabelMarkup(self.as_ptr(), markup)
        }
    }
    /// Returns the given label string without mnemonics ("&" characters).
    fn get_label_text_str(label: &str) -> String {
        unsafe {
            let label = WxString::from(label);
            let label = label.as_ptr();
            WxString::from_ptr(ffi::wxControl_GetLabelText1(label)).into()
        }
    }
    /// Returns the given str string without mnemonics ("&" characters).
    fn remove_mnemonics(str: &str) -> String {
        unsafe {
            let str = WxString::from(str);
            let str = str.as_ptr();
            WxString::from_ptr(ffi::wxControl_RemoveMnemonics(str)).into()
        }
    }
    /// Escapes the special mnemonics characters ("&") in the given string.
    fn escape_mnemonics(text: &str) -> String {
        unsafe {
            let text = WxString::from(text);
            let text = text.as_ptr();
            WxString::from_ptr(ffi::wxControl_EscapeMnemonics(text)).into()
        }
    }
    /// Replaces parts of the label string with ellipsis, if needed, so that it fits into maxWidth pixels if possible.
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
pub trait ControlWithItemsMethods: ControlMethods {}

// wxCursor
pub trait CursorMethods: GDIObjectMethods {
    // DTOR: fn ~wxCursor()
    /// Returns true if cursor data is present.
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxCursor_IsOk(self.as_ptr()) }
    }
    /// Returns the coordinates of the cursor hot spot.
    fn get_hot_spot(&self) -> Point {
        unsafe { Point::from_ptr(ffi::wxCursor_GetHotSpot(self.as_ptr())) }
    }
    // BLOCKED: fn operator=()
}

// wxCustomDataObject
pub trait CustomDataObjectMethods: DataObjectSimpleMethods {
    // DTOR: fn ~wxCustomDataObject()
    /// This function is called to allocate size bytes of memory from SetData().
    fn alloc(&self, size: usize) -> *mut c_void {
        unsafe { ffi::wxCustomDataObject_Alloc(self.as_ptr(), size) }
    }
    /// This function is called when the data is freed, you may override it to anything you want (or may be nothing at all).
    fn free(&self) {
        unsafe { ffi::wxCustomDataObject_Free(self.as_ptr()) }
    }
    /// Returns a pointer to the data.
    fn get_data(&self) -> *mut c_void {
        unsafe { ffi::wxCustomDataObject_GetData(self.as_ptr()) }
    }
    /// Returns the data size in bytes.
    fn get_size(&self) -> usize {
        unsafe { ffi::wxCustomDataObject_GetSize(self.as_ptr()) }
    }
    /// Like SetData(), but doesn't copy the data - instead the object takes ownership of the pointer.
    fn take_data(&self, size: usize, data: *mut c_void) {
        unsafe { ffi::wxCustomDataObject_TakeData(self.as_ptr(), size, data) }
    }
}
