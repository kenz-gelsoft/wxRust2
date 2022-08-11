use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

// wxDateTime
pub trait DateTimeMethods: WxRustMethods {
    fn reset_time(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_ResetTime(self.as_ptr());
            &self
        }
    }
    // NOT_SUPPORTED: fn Set()
    // BLOCKED: fn Set1()
    fn set_tm(&self, tm: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Set2(self.as_ptr(), tm);
            &self
        }
    }
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
    fn set_to_current(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_SetToCurrent(self.as_ptr());
            &self
        }
    }
    fn set_year(&self, year: c_int) -> &Self {
        unsafe {
            ffi::wxDateTime_SetYear(self.as_ptr(), year);
            &self
        }
    }
    // NOT_SUPPORTED: fn operator=()
    // BLOCKED: fn operator=1()
    // NOT_SUPPORTED: fn GetAsDOS()
    fn set_from_msw_sys_time(&self, st: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_SetFromMSWSysTime(self.as_ptr(), st);
            &self
        }
    }
    fn get_as_msw_sys_time(&self, st: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAsMSWSysTime(self.as_ptr(), st) }
    }
    fn get_century_timezone(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetCentury(self.as_ptr(), tz) }
    }
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
    fn get_week_based_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetWeekBasedYear(self.as_ptr(), tz) }
    }
    // NOT_SUPPORTED: fn GetWeekOfMonth()
    // NOT_SUPPORTED: fn GetWeekOfYear()
    fn get_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetYear(self.as_ptr(), tz) }
    }
    fn is_valid(&self) -> bool {
        unsafe { ffi::wxDateTime_IsValid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsWorkDay()
    fn is_earlier_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEarlierThan(self.as_ptr(), datetime)
        }
    }
    fn is_equal_to<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEqualTo(self.as_ptr(), datetime)
        }
    }
    fn is_equal_up_to<D: DateTimeMethods>(&self, dt: &D, ts: *const c_void) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsEqualUpTo(self.as_ptr(), dt, ts)
        }
    }
    fn is_later_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsLaterThan(self.as_ptr(), datetime)
        }
    }
    fn is_same_date<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameDate(self.as_ptr(), dt)
        }
    }
    fn is_same_time<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameTime(self.as_ptr(), dt)
        }
    }
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
    fn is_between<D: DateTimeMethods, D2: DateTimeMethods>(&self, t1: &D, t2: &D2) -> bool {
        unsafe {
            let t1 = t1.as_ptr();
            let t2 = t2.as_ptr();
            ffi::wxDateTime_IsBetween(self.as_ptr(), t1, t2)
        }
    }
    // BLOCKED: fn Add()
    fn add_datespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Add2()
    fn add_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add3(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract()
    fn subtract_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Subtract1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract2()
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
    fn format(&self, format: &str, tz: *const c_void) -> String {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            WxString::from_ptr(ffi::wxDateTime_Format(self.as_ptr(), format, tz)).into()
        }
    }
    fn format_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatDate(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn FormatISOCombined()
    fn format_iso_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISODate(self.as_ptr())).into() }
    }
    fn format_iso_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISOTime(self.as_ptr())).into() }
    }
    fn format_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatTime(self.as_ptr())).into() }
    }
    // BLOCKED: fn ParseDate()
    fn parse_date_time(&self, datetime: &str, end: *mut c_void) -> bool {
        unsafe {
            let datetime = WxString::from(datetime);
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_ParseDateTime(self.as_ptr(), datetime, end)
        }
    }
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
    fn parse_iso_date(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISODate(self.as_ptr(), date)
        }
    }
    fn parse_iso_time(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISOTime(self.as_ptr(), date)
        }
    }
    fn parse_rfc822_date(&self, date: &str, end: *mut c_void) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseRfc822Date(self.as_ptr(), date, end)
        }
    }
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
    fn get_jdn(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJDN(self.as_ptr()) }
    }
    fn get_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJulianDayNumber(self.as_ptr()) }
    }
    fn get_mjd(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetMJD(self.as_ptr()) }
    }
    fn get_modified_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetModifiedJulianDayNumber(self.as_ptr()) }
    }
    fn get_rata_die(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetRataDie(self.as_ptr()) }
    }
    fn from_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_FromTimezone(self.as_ptr(), tz, no_dst)) }
    }
    // NOT_SUPPORTED: fn IsDST()
    fn make_from_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeFromTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    fn make_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    fn make_utc(&self, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeUTC(self.as_ptr(), no_dst);
            &self
        }
    }
    fn to_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToTimezone(self.as_ptr(), tz, no_dst)) }
    }
    fn to_utc(&self, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToUTC(self.as_ptr(), no_dst)) }
    }
    fn convert_year_to_bc(year: c_int) -> c_int {
        unsafe { ffi::wxDateTime_ConvertYearToBC(year) }
    }
    fn get_am_pm_strings(am: *mut c_void, pm: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAmPmStrings(am, pm) }
    }
    // NOT_SUPPORTED: fn GetBeginDST()
    // NOT_SUPPORTED: fn GetEndDST()
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
    fn get_tm_now() -> *mut c_void {
        unsafe { ffi::wxDateTime_GetTmNow1() }
    }
    // NOT_SUPPORTED: fn GetWeekDayName()
    // NOT_SUPPORTED: fn IsDSTApplicable()
    fn get_first_week_day(first_day: *mut c_void) -> bool {
        unsafe { ffi::wxDateTime_GetFirstWeekDay(first_day) }
    }
    // NOT_SUPPORTED: fn IsLeapYear()
    // NOT_SUPPORTED: fn IsWestEuropeanCountry()
    fn now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Now()) }
    }
    // NOT_SUPPORTED: fn SetCountry()
    // NOT_SUPPORTED: fn SetToWeekOfYear()
    fn today() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Today()) }
    }
    fn u_now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_UNow()) }
    }
}
