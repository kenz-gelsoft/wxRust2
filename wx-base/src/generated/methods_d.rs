use super::*;

// wxDateTime
pub trait DateTimeMethods: WxRustMethods {
    /// Reset time to midnight (00:00:00) without changing the date.
    fn reset_time(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_ResetTime(self.as_ptr());
            &self
        }
    }
    // NOT_SUPPORTED: fn Set()
    // BLOCKED: fn Set1()
    /// Sets the date and time from the broken down representation in the wxDateTime::Tm structure.
    fn set_tm(&self, tm: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Set2(self.as_ptr(), tm);
            &self
        }
    }
    /// Sets the date from the so-called Julian Day Number.
    fn set_double(&self, jdn: c_double) -> &Self {
        unsafe {
            ffi::wxDateTime_Set3(self.as_ptr(), jdn);
            &self
        }
    }
    // NOT_SUPPORTED: fn Set4()
    // NOT_SUPPORTED: fn Set5()
    // NOT_SUPPORTED: fn SetDay()
    // NOT_SUPPORTED: fn SetFromDOS()
    // NOT_SUPPORTED: fn SetHour()
    // NOT_SUPPORTED: fn SetMillisecond()
    // NOT_SUPPORTED: fn SetMinute()
    // NOT_SUPPORTED: fn SetMonth()
    // NOT_SUPPORTED: fn SetSecond()
    /// Sets the date and time of to the current values.
    fn set_to_current(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_SetToCurrent(self.as_ptr());
            &self
        }
    }
    /// Sets the year without changing other date components.
    fn set_year(&self, year: c_int) -> &Self {
        unsafe {
            ffi::wxDateTime_SetYear(self.as_ptr(), year);
            &self
        }
    }
    // NOT_SUPPORTED: fn operator=()
    // BLOCKED: fn operator=1()
    // NOT_SUPPORTED: fn GetAsDOS()
    /// Initialize using the Windows SYSTEMTIME structure.
    fn set_from_msw_sys_time(&self, st: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_SetFromMSWSysTime(self.as_ptr(), st);
            &self
        }
    }
    /// Returns the date and time in the Windows SYSTEMTIME format.
    fn get_as_msw_sys_time(&self, st: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAsMSWSysTime(self.as_ptr(), st) }
    }
    /// Returns the century of this date.
    fn get_century_timezone(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetCentury(self.as_ptr(), tz) }
    }
    /// Returns the object having the same date component as this one but time of 00:00:00.
    fn get_date_only(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_GetDateOnly(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetDay()
    // NOT_SUPPORTED: fn GetDayOfYear()
    // NOT_SUPPORTED: fn GetHour()
    // NOT_SUPPORTED: fn GetMillisecond()
    // NOT_SUPPORTED: fn GetMinute()
    // NOT_SUPPORTED: fn GetMonth()
    // NOT_SUPPORTED: fn GetSecond()
    // NOT_SUPPORTED: fn GetTicks()
    // NOT_SUPPORTED: fn GetValue()
    // NOT_SUPPORTED: fn GetTm()
    // NOT_SUPPORTED: fn GetWeekDay()
    /// Returns the year to which the week containing this date belongs.
    fn get_week_based_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetWeekBasedYear(self.as_ptr(), tz) }
    }
    // NOT_SUPPORTED: fn GetWeekOfMonth()
    // NOT_SUPPORTED: fn GetWeekOfYear()
    /// Returns the year in the given timezone (local one by default).
    fn get_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetYear(self.as_ptr(), tz) }
    }
    /// Returns true if the object represents a valid time moment.
    fn is_valid(&self) -> bool {
        unsafe { ffi::wxDateTime_IsValid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsWorkDay()
    /// Returns true if this date precedes the given one.
    fn is_earlier_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEarlierThan(self.as_ptr(), datetime)
        }
    }
    /// Returns true if the two dates are strictly identical.
    fn is_equal_to<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEqualTo(self.as_ptr(), datetime)
        }
    }
    /// Returns true if the date is equal to another one up to the given time interval, i.e. if the absolute difference between the two dates is less than this interval.
    fn is_equal_up_to<D: DateTimeMethods>(&self, dt: &D, ts: *const c_void) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsEqualUpTo(self.as_ptr(), dt, ts)
        }
    }
    /// Returns true if this date is later than the given one.
    fn is_later_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsLaterThan(self.as_ptr(), datetime)
        }
    }
    /// Returns true if the date is the same without comparing the time parts.
    fn is_same_date<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameDate(self.as_ptr(), dt)
        }
    }
    /// Returns true if the time is the same (although dates may differ).
    fn is_same_time<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameTime(self.as_ptr(), dt)
        }
    }
    /// Returns true if this date lies strictly between the two given dates.
    fn is_strictly_between<D: DateTimeMethods, D2: DateTimeMethods>(
        &self,
        t1: &D,
        t2: &D2,
    ) -> bool {
        unsafe {
            let t1 = t1.as_ptr();
            let t2 = t2.as_ptr();
            ffi::wxDateTime_IsStrictlyBetween(self.as_ptr(), t1, t2)
        }
    }
    /// Returns true if IsStrictlyBetween() is true or if the date is equal to one of the limit values.
    fn is_between<D: DateTimeMethods, D2: DateTimeMethods>(&self, t1: &D, t2: &D2) -> bool {
        unsafe {
            let t1 = t1.as_ptr();
            let t2 = t2.as_ptr();
            ffi::wxDateTime_IsBetween(self.as_ptr(), t1, t2)
        }
    }
    // BLOCKED: fn Add()
    /// Adds the given date span to this object.
    fn add_datespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Add2()
    /// Adds the given time span to this object.
    fn add_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add3(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract()
    /// Subtracts the given time span from this object.
    fn subtract_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Subtract1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract2()
    /// Subtracts the given date span from this object.
    fn subtract_datespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Subtract3(self.as_ptr(), diff);
            &self
        }
    }
    // NOT_SUPPORTED: fn Subtract4()
    // NOT_SUPPORTED: fn DiffAsDateSpan()
    // BLOCKED: fn operator+=()
    // BLOCKED: fn operator+()
    // BLOCKED: fn operator-=()
    // BLOCKED: fn operator-()
    // BLOCKED: fn operator+=1()
    // BLOCKED: fn operator+1()
    // BLOCKED: fn operator-=1()
    // BLOCKED: fn operator-1()
    // NOT_SUPPORTED: fn operator-2()
    /// This function does the same as the standard ANSI C strftime(3) function (http://www.cplusplus.com/reference/clibrary/ctime/strftime.html).
    fn format(&self, format: &str, tz: *const c_void) -> String {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            WxString::from_ptr(ffi::wxDateTime_Format(self.as_ptr(), format, tz)).into()
        }
    }
    /// Identical to calling Format() with "%x" argument (which means "preferred date representation for the current locale").
    fn format_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatDate(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn FormatISOCombined()
    /// This function returns the date representation in the ISO 8601 format "YYYY-MM-DD".
    fn format_iso_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISODate(self.as_ptr())).into() }
    }
    /// This function returns the time representation in the ISO 8601 format "HH:MM:SS".
    fn format_iso_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISOTime(self.as_ptr())).into() }
    }
    /// Identical to calling Format() with "%X" argument (which means "preferred time representation for the current locale").
    fn format_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatTime(self.as_ptr())).into() }
    }
    // BLOCKED: fn ParseDate()
    /// Parses the string datetime containing the date and time in free format.
    fn parse_date_time(&self, datetime: &str, end: *mut c_void) -> bool {
        unsafe {
            let datetime = WxString::from(datetime);
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_ParseDateTime(self.as_ptr(), datetime, end)
        }
    }
    /// This function parses the string date according to the given format.
    fn parse_format<D: DateTimeMethods>(
        &self,
        date: &str,
        format: &str,
        date_def: &D,
        end: *mut c_void,
    ) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            let format = WxString::from(format);
            let format = format.as_ptr();
            let date_def = date_def.as_ptr();
            ffi::wxDateTime_ParseFormat(self.as_ptr(), date, format, date_def, end)
        }
    }
    // BLOCKED: fn ParseFormat1()
    // BLOCKED: fn ParseFormat2()
    // NOT_SUPPORTED: fn ParseISOCombined()
    /// This function parses the date in ISO 8601 format "YYYY-MM-DD".
    fn parse_iso_date(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISODate(self.as_ptr(), date)
        }
    }
    /// This function parses the time in ISO 8601 format "HH:MM:SS".
    fn parse_iso_time(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISOTime(self.as_ptr(), date)
        }
    }
    /// Parses the string date looking for a date formatted according to the RFC 822 in it.
    fn parse_rfc822_date(&self, date: &str, end: *mut c_void) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseRfc822Date(self.as_ptr(), date, end)
        }
    }
    /// This functions is like ParseDateTime(), but only allows the time to be specified in the input string.
    fn parse_time(&self, time: &str, end: *mut c_void) -> bool {
        unsafe {
            let time = WxString::from(time);
            let time = time.as_ptr();
            ffi::wxDateTime_ParseTime(self.as_ptr(), time, end)
        }
    }
    // NOT_SUPPORTED: fn GetLastMonthDay()
    // NOT_SUPPORTED: fn GetLastWeekDay()
    // NOT_SUPPORTED: fn GetNextWeekDay()
    // NOT_SUPPORTED: fn GetPrevWeekDay()
    // NOT_SUPPORTED: fn GetWeekDay1()
    // NOT_SUPPORTED: fn GetWeekDayInSameWeek()
    // NOT_SUPPORTED: fn GetYearDay()
    // NOT_SUPPORTED: fn SetToLastMonthDay()
    // NOT_SUPPORTED: fn SetToLastWeekDay()
    // NOT_SUPPORTED: fn SetToNextWeekDay()
    // NOT_SUPPORTED: fn SetToPrevWeekDay()
    // NOT_SUPPORTED: fn SetToWeekDay()
    // NOT_SUPPORTED: fn SetToWeekDayInSameWeek()
    // NOT_SUPPORTED: fn SetToYearDay()
    /// Synonym for GetJulianDayNumber().
    fn get_jdn(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJDN(self.as_ptr()) }
    }
    /// Returns the JDN corresponding to this date.
    fn get_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJulianDayNumber(self.as_ptr()) }
    }
    /// Synonym for GetModifiedJulianDayNumber().
    fn get_mjd(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetMJD(self.as_ptr()) }
    }
    /// Returns the "Modified Julian Day Number" (MJD) which is, by definition, is equal to JDN - 2400000.5.
    fn get_modified_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetModifiedJulianDayNumber(self.as_ptr()) }
    }
    /// Return the Rata Die number of this date.
    fn get_rata_die(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetRataDie(self.as_ptr()) }
    }
    /// Transform the date from the given time zone to the local one.
    fn from_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_FromTimezone(self.as_ptr(), tz, no_dst)) }
    }
    // NOT_SUPPORTED: fn IsDST()
    /// Same as FromTimezone() but modifies the object in place.
    fn make_from_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeFromTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    /// Modifies the object in place to represent the date in another time zone.
    fn make_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    /// This is the same as calling MakeTimezone() with the argument GMT0.
    fn make_utc(&self, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeUTC(self.as_ptr(), no_dst);
            &self
        }
    }
    /// Transform the date to the given time zone.
    fn to_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToTimezone(self.as_ptr(), tz, no_dst)) }
    }
    /// This is the same as calling ToTimezone() with the argument GMT0.
    fn to_utc(&self, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToUTC(self.as_ptr(), no_dst)) }
    }
    /// Converts the year in absolute notation (i.e. a number which can be negative, positive or zero) to the year in BC/AD notation.
    fn convert_year_to_bc(year: c_int) -> c_int {
        unsafe { ffi::wxDateTime_ConvertYearToBC(year) }
    }
    /// Returns the translations of the strings AM and PM used for time formatting for the current locale.
    fn get_am_pm_strings(am: *mut c_void, pm: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAmPmStrings(am, pm) }
    }
    // NOT_SUPPORTED: fn GetBeginDST()
    // NOT_SUPPORTED: fn GetEndDST()
    /// Get the current century, i.e. first two digits of the year, in given calendar (only Gregorian is currently supported).
    fn get_century_int(year: c_int) -> c_int {
        unsafe { ffi::wxDateTime_GetCentury1(year) }
    }
    // NOT_SUPPORTED: fn GetCountry()
    // NOT_SUPPORTED: fn GetCurrentMonth()
    // NOT_SUPPORTED: fn GetCurrentYear()
    // NOT_SUPPORTED: fn GetEnglishMonthName()
    // NOT_SUPPORTED: fn GetEnglishWeekDayName()
    // NOT_SUPPORTED: fn GetMonthName()
    // NOT_SUPPORTED: fn GetNumberOfDays()
    // NOT_SUPPORTED: fn GetNumberOfDays1()
    // NOT_SUPPORTED: fn GetTimeNow()
    // BLOCKED: fn GetTmNow()
    /// Returns the current time broken down.
    fn get_tm_now() -> *mut c_void {
        unsafe { ffi::wxDateTime_GetTmNow1() }
    }
    // NOT_SUPPORTED: fn GetWeekDayName()
    // NOT_SUPPORTED: fn IsDSTApplicable()
    /// Acquires the first weekday of a week based on locale and/or OS settings.
    fn get_first_week_day(first_day: *mut c_void) -> bool {
        unsafe { ffi::wxDateTime_GetFirstWeekDay(first_day) }
    }
    // NOT_SUPPORTED: fn IsLeapYear()
    // NOT_SUPPORTED: fn IsWestEuropeanCountry()
    /// Returns the object corresponding to the current time in local time zone.
    fn now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Now()) }
    }
    // NOT_SUPPORTED: fn SetCountry()
    // NOT_SUPPORTED: fn SetToWeekOfYear()
    /// Returns the object corresponding to the midnight of the current day (i.e. the same as Now(), but the time part is set to 0).
    fn today() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Today()) }
    }
    /// Returns the object corresponding to the current time including the milliseconds.
    fn u_now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_UNow()) }
    }
}
