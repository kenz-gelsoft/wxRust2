use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxDateTime
    pub fn wxDateTime_delete(self_: *mut c_void);
    pub fn wxDateTime_new() -> *mut c_void;
    pub fn wxDateTime_new1(date: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_new2(timet: time_t) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_new3(tm: *const c_void) -> *mut c_void;
    pub fn wxDateTime_new4(jdn: c_double) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_new5(hour: wxDateTime_t, minute: wxDateTime_t, second: wxDateTime_t, millisec: wxDateTime_t) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_new6(day: wxDateTime_t, month: Month, year: c_int, hour: wxDateTime_t, minute: wxDateTime_t, second: wxDateTime_t, millisec: wxDateTime_t) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_new7(st: *const c_void) -> *mut c_void;
    pub fn wxDateTime_ResetTime(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_Set(self_: *mut c_void, timet: time_t) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_Set1(self_: *mut c_void, tm: *const c_void) -> *mut c_void;
    pub fn wxDateTime_Set2(self_: *mut c_void, tm: *const c_void) -> *mut c_void;
    pub fn wxDateTime_Set3(self_: *mut c_void, jdn: c_double) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_Set4(self_: *mut c_void, hour: wxDateTime_t, minute: wxDateTime_t, second: wxDateTime_t, millisec: wxDateTime_t) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_Set5(self_: *mut c_void, day: wxDateTime_t, month: Month, year: c_int, hour: wxDateTime_t, minute: wxDateTime_t, second: wxDateTime_t, millisec: wxDateTime_t) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetDay(self_: *mut c_void, day: unsigned short) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetFromDOS(self_: *mut c_void, ddt: unsigned long) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetHour(self_: *mut c_void, hour: unsigned short) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetMillisecond(self_: *mut c_void, millisecond: unsigned short) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetMinute(self_: *mut c_void, minute: unsigned short) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetMonth(self_: *mut c_void, month: Month) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetSecond(self_: *mut c_void, second: unsigned short) -> *mut c_void;
    pub fn wxDateTime_SetToCurrent(self_: *mut c_void) -> *mut c_void;
    pub fn wxDateTime_SetYear(self_: *mut c_void, year: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_operator=(self_: *mut c_void, timet: time_t) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator=1(self_: *mut c_void, tm: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetAsDOS(self_: *const c_void) -> unsigned long;
    pub fn wxDateTime_SetFromMSWSysTime(self_: *mut c_void, st: *const c_void) -> *mut c_void;
    pub fn wxDateTime_GetAsMSWSysTime(self_: *const c_void, st: *mut c_void);
    pub fn wxDateTime_GetCentury(self_: *const c_void, tz: *const c_void) -> c_int;
    pub fn wxDateTime_GetDateOnly(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetDay(self_: *const c_void, tz: *const c_void) -> unsigned short;
    // NOT_SUPPORTED: pub fn wxDateTime_GetDayOfYear(self_: *const c_void, tz: *const c_void) -> unsigned short;
    // NOT_SUPPORTED: pub fn wxDateTime_GetHour(self_: *const c_void, tz: *const c_void) -> unsigned short;
    // NOT_SUPPORTED: pub fn wxDateTime_GetMillisecond(self_: *const c_void, tz: *const c_void) -> unsigned short;
    // NOT_SUPPORTED: pub fn wxDateTime_GetMinute(self_: *const c_void, tz: *const c_void) -> unsigned short;
    // NOT_SUPPORTED: pub fn wxDateTime_GetMonth(self_: *const c_void, tz: *const c_void) -> Month;
    // NOT_SUPPORTED: pub fn wxDateTime_GetSecond(self_: *const c_void, tz: *const c_void) -> unsigned short;
    // NOT_SUPPORTED: pub fn wxDateTime_GetTicks(self_: *const c_void) -> time_t;
    // NOT_SUPPORTED: pub fn wxDateTime_GetValue(self_: *const c_void) -> wxLongLong;
    // NOT_SUPPORTED: pub fn wxDateTime_GetTm(self_: *const c_void, tz: *const c_void) -> Tm;
    // NOT_SUPPORTED: pub fn wxDateTime_GetWeekDay(self_: *const c_void, tz: *const c_void) -> WeekDay;
    pub fn wxDateTime_GetWeekBasedYear(self_: *const c_void, tz: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxDateTime_GetWeekOfMonth(self_: *const c_void, flags: WeekFlags, tz: *const c_void) -> wxDateTime_t;
    // NOT_SUPPORTED: pub fn wxDateTime_GetWeekOfYear(self_: *const c_void, flags: WeekFlags, tz: *const c_void) -> wxDateTime_t;
    pub fn wxDateTime_GetYear(self_: *const c_void, tz: *const c_void) -> c_int;
    pub fn wxDateTime_IsValid(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_IsWorkDay(self_: *const c_void, country: Country) -> bool;
    pub fn wxDateTime_IsEarlierThan(self_: *const c_void, datetime: *const c_void) -> bool;
    pub fn wxDateTime_IsEqualTo(self_: *const c_void, datetime: *const c_void) -> bool;
    pub fn wxDateTime_IsEqualUpTo(
        self_: *const c_void,
        dt: *const c_void,
        ts: *const c_void,
    ) -> bool;
    pub fn wxDateTime_IsLaterThan(self_: *const c_void, datetime: *const c_void) -> bool;
    pub fn wxDateTime_IsSameDate(self_: *const c_void, dt: *const c_void) -> bool;
    pub fn wxDateTime_IsSameTime(self_: *const c_void, dt: *const c_void) -> bool;
    pub fn wxDateTime_IsStrictlyBetween(
        self_: *const c_void,
        t1: *const c_void,
        t2: *const c_void,
    ) -> bool;
    pub fn wxDateTime_IsBetween(self_: *const c_void, t1: *const c_void, t2: *const c_void)
        -> bool;
    // BLOCKED: pub fn wxDateTime_Add(self_: *const c_void, diff: *const c_void) -> wxDateTime;
    pub fn wxDateTime_Add1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_Add2(self_: *const c_void, diff: *const c_void) -> wxDateTime;
    pub fn wxDateTime_Add3(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_Subtract(self_: *const c_void, diff: *const c_void) -> wxDateTime;
    pub fn wxDateTime_Subtract1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_Subtract2(self_: *const c_void, diff: *const c_void) -> wxDateTime;
    pub fn wxDateTime_Subtract3(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_Subtract4(self_: *const c_void, dt: *const c_void) -> wxTimeSpan;
    // NOT_SUPPORTED: pub fn wxDateTime_DiffAsDateSpan(self_: *const c_void, dt: *const c_void) -> wxDateSpan;
    // BLOCKED: pub fn wxDateTime_operator+=(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator+(self_: *const c_void, ds: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator-=(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator-(self_: *const c_void, ds: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator+=1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator+1(self_: *const c_void, ts: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator-=1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator-1(self_: *const c_void, ts: *const c_void) -> wxDateTime;
    // NOT_SUPPORTED: pub fn wxDateTime_operator-2(self_: *const c_void, dt2: *const c_void) -> wxTimeSpan;
    pub fn wxDateTime_Format(
        self_: *const c_void,
        format: *const c_void,
        tz: *const c_void,
    ) -> *mut c_void;
    pub fn wxDateTime_FormatDate(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_FormatISOCombined(self_: *const c_void, sep: char) -> *mut c_void;
    pub fn wxDateTime_FormatISODate(self_: *const c_void) -> *mut c_void;
    pub fn wxDateTime_FormatISOTime(self_: *const c_void) -> *mut c_void;
    pub fn wxDateTime_FormatTime(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_ParseDate(self_: *mut c_void, date: *const c_void, end: *mut c_void) -> bool;
    pub fn wxDateTime_ParseDateTime(
        self_: *mut c_void,
        datetime: *const c_void,
        end: *mut c_void,
    ) -> bool;
    pub fn wxDateTime_ParseFormat(
        self_: *mut c_void,
        date: *const c_void,
        format: *const c_void,
        date_def: *const c_void,
        end: *mut c_void,
    ) -> bool;
    // BLOCKED: pub fn wxDateTime_ParseFormat1(self_: *mut c_void, date: *const c_void, format: *const c_void, end: *mut c_void) -> bool;
    // BLOCKED: pub fn wxDateTime_ParseFormat2(self_: *mut c_void, date: *const c_void, end: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_ParseISOCombined(self_: *mut c_void, date: *const c_void, sep: char) -> bool;
    pub fn wxDateTime_ParseISODate(self_: *mut c_void, date: *const c_void) -> bool;
    pub fn wxDateTime_ParseISOTime(self_: *mut c_void, date: *const c_void) -> bool;
    pub fn wxDateTime_ParseRfc822Date(
        self_: *mut c_void,
        date: *const c_void,
        end: *mut c_void,
    ) -> bool;
    pub fn wxDateTime_ParseTime(self_: *mut c_void, time: *const c_void, end: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_GetLastMonthDay(self_: *const c_void, month: Month, year: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetLastWeekDay(self_: *mut c_void, weekday: WeekDay, month: Month, year: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetNextWeekDay(self_: *const c_void, weekday: WeekDay) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetPrevWeekDay(self_: *const c_void, weekday: WeekDay) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetWeekDay1(self_: *const c_void, weekday: WeekDay, n: c_int, month: Month, year: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetWeekDayInSameWeek(self_: *const c_void, weekday: WeekDay, flags: WeekFlags) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetYearDay(self_: *const c_void, yday: wxDateTime_t) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToLastMonthDay(self_: *mut c_void, month: Month, year: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToLastWeekDay(self_: *mut c_void, weekday: WeekDay, month: Month, year: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToNextWeekDay(self_: *mut c_void, weekday: WeekDay) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToPrevWeekDay(self_: *mut c_void, weekday: WeekDay) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToWeekDay(self_: *mut c_void, weekday: WeekDay, n: c_int, month: Month, year: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToWeekDayInSameWeek(self_: *mut c_void, weekday: WeekDay, flags: WeekFlags) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetToYearDay(self_: *mut c_void, yday: wxDateTime_t) -> *mut c_void;
    pub fn wxDateTime_GetJDN(self_: *const c_void) -> c_double;
    pub fn wxDateTime_GetJulianDayNumber(self_: *const c_void) -> c_double;
    pub fn wxDateTime_GetMJD(self_: *const c_void) -> c_double;
    pub fn wxDateTime_GetModifiedJulianDayNumber(self_: *const c_void) -> c_double;
    pub fn wxDateTime_GetRataDie(self_: *const c_void) -> c_double;
    pub fn wxDateTime_FromTimezone(
        self_: *const c_void,
        tz: *const c_void,
        no_dst: bool,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_IsDST(self_: *const c_void, country: Country) -> c_int;
    pub fn wxDateTime_MakeFromTimezone(
        self_: *mut c_void,
        tz: *const c_void,
        no_dst: bool,
    ) -> *mut c_void;
    pub fn wxDateTime_MakeTimezone(
        self_: *mut c_void,
        tz: *const c_void,
        no_dst: bool,
    ) -> *mut c_void;
    pub fn wxDateTime_MakeUTC(self_: *mut c_void, no_dst: bool) -> *mut c_void;
    pub fn wxDateTime_ToTimezone(
        self_: *const c_void,
        tz: *const c_void,
        no_dst: bool,
    ) -> *mut c_void;
    pub fn wxDateTime_ToUTC(self_: *const c_void, no_dst: bool) -> *mut c_void;
    pub fn wxDateTime_ConvertYearToBC(year: c_int) -> c_int;
    pub fn wxDateTime_GetAmPmStrings(am: *mut c_void, pm: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDateTime_GetBeginDST(year: c_int, country: Country) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetEndDST(year: c_int, country: Country) -> *mut c_void;
    pub fn wxDateTime_GetCentury1(year: c_int) -> c_int;
    // NOT_SUPPORTED: pub fn wxDateTime_GetCountry() -> Country;
    // NOT_SUPPORTED: pub fn wxDateTime_GetCurrentMonth(cal: Calendar) -> Month;
    // NOT_SUPPORTED: pub fn wxDateTime_GetCurrentYear(cal: Calendar) -> c_int;
    // NOT_SUPPORTED: pub fn wxDateTime_GetEnglishMonthName(month: Month, flags: NameFlags) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetEnglishWeekDayName(weekday: WeekDay, flags: NameFlags) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetMonthName(month: Month, flags: NameFlags) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetNumberOfDays(year: c_int, cal: Calendar) -> wxDateTime_t;
    // NOT_SUPPORTED: pub fn wxDateTime_GetNumberOfDays1(month: Month, year: c_int, cal: Calendar) -> wxDateTime_t;
    // NOT_SUPPORTED: pub fn wxDateTime_GetTimeNow() -> time_t;
    // BLOCKED: pub fn wxDateTime_GetTmNow(tm: *mut c_void) -> *mut c_void;
    pub fn wxDateTime_GetTmNow1() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_GetWeekDayName(weekday: WeekDay, flags: NameFlags) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_IsDSTApplicable(year: c_int, country: Country) -> bool;
    pub fn wxDateTime_GetFirstWeekDay(first_day: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_IsLeapYear(year: c_int, cal: Calendar) -> bool;
    // NOT_SUPPORTED: pub fn wxDateTime_IsWestEuropeanCountry(country: Country) -> bool;
    pub fn wxDateTime_Now() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDateTime_SetCountry(country: Country);
    // NOT_SUPPORTED: pub fn wxDateTime_SetToWeekOfYear(year: c_int, num_week: wxDateTime_t, weekday: WeekDay) -> *mut c_void;
    pub fn wxDateTime_Today() -> *mut c_void;
    pub fn wxDateTime_UNow() -> *mut c_void;

}
