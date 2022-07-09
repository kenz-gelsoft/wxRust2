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

    // wxEvent
    // NOT_SUPPORTED: pub fn wxEvent_new(id: c_int, event_type: wxEventType) -> *mut c_void;
    pub fn wxEvent_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxEvent_GetEventObject(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxEvent_GetEventType(self_: *const c_void) -> wxEventType;
    // NOT_SUPPORTED: pub fn wxEvent_GetEventCategory(self_: *const c_void) -> wxEventCategory;
    pub fn wxEvent_GetId(self_: *const c_void) -> c_int;
    pub fn wxEvent_GetEventUserData(self_: *const c_void) -> *mut c_void;
    pub fn wxEvent_GetSkipped(self_: *const c_void) -> bool;
    pub fn wxEvent_GetTimestamp(self_: *const c_void) -> c_long;
    pub fn wxEvent_IsCommandEvent(self_: *const c_void) -> bool;
    pub fn wxEvent_ResumePropagation(self_: *mut c_void, propagation_level: c_int);
    pub fn wxEvent_SetEventObject(self_: *mut c_void, object: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvent_SetEventType(self_: *mut c_void, type_: wxEventType);
    pub fn wxEvent_SetId(self_: *mut c_void, id: c_int);
    pub fn wxEvent_SetTimestamp(self_: *mut c_void, time_stamp: c_long);
    pub fn wxEvent_ShouldPropagate(self_: *const c_void) -> bool;
    pub fn wxEvent_Skip(self_: *mut c_void, skip: bool);
    pub fn wxEvent_StopPropagation(self_: *mut c_void) -> c_int;

    // wxEvtHandler
    pub fn wxEvtHandler_QueueEvent(self_: *mut c_void, event: *mut c_void);
    pub fn wxEvtHandler_AddPendingEvent(self_: *mut c_void, event: *const c_void);
    // NOT_SUPPORTED: pub fn wxEvtHandler_CallAfter(self_: *mut c_void, method: *mut c_void, x1: T1, None: ...);
    // BLOCKED: pub fn wxEvtHandler_CallAfter1(self_: *mut c_void, functor: *const c_void);
    pub fn wxEvtHandler_ProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
    pub fn wxEvtHandler_ProcessEventLocally(self_: *mut c_void, event: *mut c_void) -> bool;
    pub fn wxEvtHandler_SafelyProcessEvent(self_: *mut c_void, event: *mut c_void) -> bool;
    pub fn wxEvtHandler_ProcessPendingEvents(self_: *mut c_void);
    pub fn wxEvtHandler_DeletePendingEvents(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvtHandler_Connect(self_: *mut c_void, id: c_int, last_id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvtHandler_Connect1(self_: *mut c_void, id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvtHandler_Connect2(self_: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect(self_: *mut c_void, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect1(self_: *mut c_void, id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxEvtHandler_Disconnect2(self_: *mut c_void, id: c_int, last_id: c_int, event_type: wxEventType, function: wxObjectEventFunction, user_data: *mut c_void, event_sink: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxEvtHandler_Bind(self_: *mut c_void, event_type: *const c_void, functor: Functor, id: c_int, last_id: c_int, user_data: *mut c_void);
    // BLOCKED: pub fn wxEvtHandler_Bind1(self_: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: c_int, last_id: c_int, user_data: *mut c_void);
    // NOT_SUPPORTED: pub fn wxEvtHandler_Unbind(self_: *mut c_void, event_type: *const c_void, functor: Functor, id: c_int, last_id: c_int, user_data: *mut c_void) -> bool;
    // BLOCKED: pub fn wxEvtHandler_Unbind1(self_: *mut c_void, event_type: *const c_void, method: *mut c_void, handler: *mut c_void, id: c_int, last_id: c_int, user_data: *mut c_void) -> bool;
    // BLOCKED: pub fn wxEvtHandler_GetClientData(self_: *const c_void) -> *mut c_void;
    pub fn wxEvtHandler_GetClientObject(self_: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxEvtHandler_SetClientData(self_: *mut c_void, data: *mut c_void);
    pub fn wxEvtHandler_SetClientObject(self_: *mut c_void, data: *mut c_void);
    pub fn wxEvtHandler_GetEvtHandlerEnabled(self_: *const c_void) -> bool;
    pub fn wxEvtHandler_GetNextHandler(self_: *const c_void) -> *mut c_void;
    pub fn wxEvtHandler_GetPreviousHandler(self_: *const c_void) -> *mut c_void;
    pub fn wxEvtHandler_SetEvtHandlerEnabled(self_: *mut c_void, enabled: bool);
    pub fn wxEvtHandler_SetNextHandler(self_: *mut c_void, handler: *mut c_void);
    pub fn wxEvtHandler_SetPreviousHandler(self_: *mut c_void, handler: *mut c_void);
    pub fn wxEvtHandler_Unlink(self_: *mut c_void);
    pub fn wxEvtHandler_IsUnlinked(self_: *const c_void) -> bool;
    pub fn wxEvtHandler_AddFilter(filter: *mut c_void);
    pub fn wxEvtHandler_RemoveFilter(filter: *mut c_void);
    pub fn wxEvtHandler_new() -> *mut c_void;
    // DTOR: pub fn wxEvtHandler_~wxEvtHandler(self_: *mut c_void);

    // wxFileName
    pub fn wxFileName_delete(self_: *mut c_void);
    pub fn wxFileName_new() -> *mut c_void;
    pub fn wxFileName_new1(filename: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new2(fullpath: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new3(path: *const c_void, name: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new4(path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_new5(volume: *const c_void, path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_AppendDir(self_: *mut c_void, dir: *const c_void) -> bool;
    pub fn wxFileName_Assign(self_: *mut c_void, filepath: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileName_Assign1(self_: *mut c_void, fullpath: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign2(self_: *mut c_void, volume: *const c_void, path: *const c_void, name: *const c_void, ext: *const c_void, has_ext: bool, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign3(self_: *mut c_void, volume: *const c_void, path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign4(self_: *mut c_void, path: *const c_void, name: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_Assign5(self_: *mut c_void, path: *const c_void, name: *const c_void, ext: *const c_void, format: wxPathFormat);
    pub fn wxFileName_AssignCwd(self_: *mut c_void, volume: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileName_AssignDir(self_: *mut c_void, dir: *const c_void, format: wxPathFormat);
    pub fn wxFileName_AssignHomeDir(self_: *mut c_void);
    pub fn wxFileName_AssignTempFileName(self_: *mut c_void, prefix: *const c_void);
    pub fn wxFileName_AssignTempFileName1(
        self_: *mut c_void,
        prefix: *const c_void,
        file_temp: *mut c_void,
    );
    pub fn wxFileName_AssignTempFileName2(
        self_: *mut c_void,
        prefix: *const c_void,
        file_temp: *mut c_void,
    );
    pub fn wxFileName_Clear(self_: *mut c_void);
    pub fn wxFileName_ClearExt(self_: *mut c_void);
    pub fn wxFileName_DirExists(self_: *const c_void) -> bool;
    pub fn wxFileName_DontFollowLink(self_: *mut c_void);
    pub fn wxFileName_Exists(self_: *const c_void, flags: c_int) -> bool;
    pub fn wxFileName_FileExists(self_: *const c_void) -> bool;
    pub fn wxFileName_GetDirCount(self_: *const c_void) -> usize;
    pub fn wxFileName_GetDirs(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetExt(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetFullName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetFullPath(self_: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetHumanReadableSize(self_: *const c_void, failmsg: *const c_void, precision: c_int, conv: wxSizeConvention) -> *mut c_void;
    pub fn wxFileName_GetLongPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetModificationTime(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPath(self_: *const c_void, flags: c_int, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathWithSep(self_: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_GetShortPath(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetSize(self_: *const c_void) -> wxULongLong;
    pub fn wxFileName_GetTimes(
        self_: *const c_void,
        dt_access: *mut c_void,
        dt_mod: *mut c_void,
        dt_create: *mut c_void,
    ) -> bool;
    pub fn wxFileName_GetVolume(self_: *const c_void) -> *mut c_void;
    pub fn wxFileName_HasExt(self_: *const c_void) -> bool;
    pub fn wxFileName_HasName(self_: *const c_void) -> bool;
    pub fn wxFileName_HasVolume(self_: *const c_void) -> bool;
    pub fn wxFileName_InsertDir(self_: *mut c_void, before: usize, dir: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsAbsolute(self_: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_IsDir(self_: *const c_void) -> bool;
    pub fn wxFileName_IsDirReadable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsDirWritable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsFileExecutable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsFileReadable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsFileWritable(self_: *const c_void) -> bool;
    pub fn wxFileName_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsRelative(self_: *const c_void, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_MakeAbsolute(self_: *mut c_void, cwd: *const c_void, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_MakeRelativeTo(self_: *mut c_void, path_base: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_Mkdir(self_: *const c_void, perm: c_int, flags: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_Normalize(self_: *mut c_void, flags: c_int, cwd: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_PrependDir(self_: *mut c_void, dir: *const c_void);
    pub fn wxFileName_RemoveDir(self_: *mut c_void, pos: usize);
    pub fn wxFileName_RemoveLastDir(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxFileName_ReplaceEnvVariable(self_: *mut c_void, envname: *const c_void, replacement_fmt_string: *const c_void, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_ReplaceHomeDir(self_: *mut c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_ResolveLink(self_: *mut c_void) -> *mut c_void;
    pub fn wxFileName_Rmdir(self_: *const c_void, flags: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_SameAs(self_: *const c_void, filepath: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_SetCwd(self_: *const c_void) -> bool;
    pub fn wxFileName_SetEmptyExt(self_: *mut c_void);
    pub fn wxFileName_SetExt(self_: *mut c_void, ext: *const c_void);
    pub fn wxFileName_SetFullName(self_: *mut c_void, fullname: *const c_void);
    pub fn wxFileName_SetName(self_: *mut c_void, name: *const c_void);
    // NOT_SUPPORTED: pub fn wxFileName_SetPath(self_: *mut c_void, path: *const c_void, format: wxPathFormat);
    pub fn wxFileName_SetPermissions(self_: *mut c_void, permissions: c_int) -> bool;
    pub fn wxFileName_SetTimes(
        self_: *const c_void,
        dt_access: *const c_void,
        dt_mod: *const c_void,
        dt_create: *const c_void,
    ) -> bool;
    pub fn wxFileName_SetVolume(self_: *mut c_void, volume: *const c_void);
    pub fn wxFileName_ShouldFollowLink(self_: *const c_void) -> bool;
    pub fn wxFileName_Touch(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator!=(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator!=1(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator==(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator==1(self_: *const c_void, filename: *const c_void) -> bool;
    // BLOCKED: pub fn wxFileName_operator=(self_: *mut c_void, filename: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxFileName_operator=1(self_: *mut c_void, filename: *const c_void) -> *mut c_void;
    pub fn wxFileName_CreateTempFileName(
        prefix: *const c_void,
        file_temp: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFileName_CreateTempFileName1(
        prefix: *const c_void,
        file_temp: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFileName_DirExists1(dir: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_DirName(dir: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_Exists1(path: *const c_void, flags: c_int) -> bool;
    pub fn wxFileName_FileExists1(file: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_FileName(file: *const c_void, format: wxPathFormat) -> *mut c_void;
    pub fn wxFileName_GetCwd(volume: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetForbiddenChars(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetFormat(format: wxPathFormat) -> wxPathFormat;
    pub fn wxFileName_GetHomeDir() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetHumanReadableSize1(bytes: *const c_void, nullsize: *const c_void, precision: c_int, conv: wxSizeConvention) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathSeparator(format: wxPathFormat) -> wxUniChar;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathSeparators(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetPathTerminators(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetSize1(filename: *const c_void) -> wxULongLong;
    pub fn wxFileName_GetTempDir() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetVolumeSeparator(format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_GetVolumeString(drive: char, flags: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_IsCaseSensitive(format: wxPathFormat) -> bool;
    pub fn wxFileName_IsDirReadable1(dir: *const c_void) -> bool;
    pub fn wxFileName_IsDirWritable1(dir: *const c_void) -> bool;
    pub fn wxFileName_IsFileExecutable1(file: *const c_void) -> bool;
    pub fn wxFileName_IsFileReadable1(file: *const c_void) -> bool;
    pub fn wxFileName_IsFileWritable1(file: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsPathSeparator(ch: wxChar, format: wxPathFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxFileName_IsMSWUniqueVolumeNamePath(path: *const c_void, format: wxPathFormat) -> bool;
    pub fn wxFileName_Mkdir1(dir: *const c_void, perm: c_int, flags: c_int) -> bool;
    pub fn wxFileName_Rmdir1(dir: *const c_void, flags: c_int) -> bool;
    pub fn wxFileName_SetCwd1(cwd: *const c_void) -> bool;
    pub fn wxFileName_URLToFileName(url: *const c_void) -> *mut c_void;
    pub fn wxFileName_FileNameToURL(filename: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileName_SplitPath(fullpath: *const c_void, volume: *mut c_void, path: *mut c_void, name: *mut c_void, ext: *mut c_void, has_ext: *mut c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_SplitPath1(fullpath: *const c_void, volume: *mut c_void, path: *mut c_void, name: *mut c_void, ext: *mut c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_SplitPath2(fullpath: *const c_void, path: *mut c_void, name: *mut c_void, ext: *mut c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxFileName_SplitVolume(fullpath: *const c_void, volume: *mut c_void, path: *mut c_void, format: wxPathFormat);
    pub fn wxFileName_StripExtension(fullname: *const c_void) -> *mut c_void;

    // wxObject
    pub fn wxObject_new() -> *mut c_void;
    pub fn wxObject_new1(other: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxObject_~wxObject(self_: *mut c_void);
    pub fn wxObject_GetClassInfo(self_: *const c_void) -> *mut c_void;
    pub fn wxObject_GetRefData(self_: *const c_void) -> *mut c_void;
    pub fn wxObject_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
    pub fn wxObject_IsSameAs(self_: *const c_void, obj: *const c_void) -> bool;
    pub fn wxObject_Ref(self_: *mut c_void, clone: *const c_void);
    pub fn wxObject_SetRefData(self_: *mut c_void, data: *mut c_void);
    pub fn wxObject_UnRef(self_: *mut c_void);
    pub fn wxObject_UnShare(self_: *mut c_void);
    // BLOCKED: pub fn wxObject_operator delete(self_: *mut c_void, buf: *mut c_void);
    // BLOCKED: pub fn wxObject_operator new(self_: *mut c_void, size: usize, filename: *const c_void, line_num: c_int) -> *mut c_void;

}
