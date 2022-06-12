use std::os::raw::{c_int, c_long, c_uint, c_void};

use super::*;
use crate::WeakRef;

pub trait WxRustMethods {
    type Unowned;
    unsafe fn as_ptr(&self) -> *mut c_void;
    unsafe fn from_ptr(ptr: *mut c_void) -> Self;
    unsafe fn from_unowned_ptr(ptr: *mut c_void) -> Self::Unowned;
    unsafe fn with_ptr<F: Fn(&Self)>(ptr: *mut c_void, closure: F);
    unsafe fn option_from(ptr: *mut c_void) -> Option<Self::Unowned>
    where
        Self: Sized,
    {
        if ptr.is_null() {
            None
        } else {
            Some(Self::from_unowned_ptr(ptr))
        }
    }
}

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
        unsafe { DateTimeIsOwned(ffi::wxDateTime_GetDateOnly(self.as_ptr())) }
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
    fn add_datespan(&self, diff: *const c_void) -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_Add(self.as_ptr(), diff)) }
    }
    fn add_datespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add1(self.as_ptr(), diff);
            &self
        }
    }
    fn add_timespan(&self, diff: *const c_void) -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_Add2(self.as_ptr(), diff)) }
    }
    fn add_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add3(self.as_ptr(), diff);
            &self
        }
    }
    fn subtract_timespan(&self, diff: *const c_void) -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_Subtract(self.as_ptr(), diff)) }
    }
    fn subtract_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Subtract1(self.as_ptr(), diff);
            &self
        }
    }
    fn subtract_datespan(&self, diff: *const c_void) -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_Subtract2(self.as_ptr(), diff)) }
    }
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
            let format = wx_base::wx_string_from(format);
            wx_base::from_wx_string(ffi::wxDateTime_Format(self.as_ptr(), format, tz))
        }
    }
    fn format_date(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxDateTime_FormatDate(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn FormatISOCombined()
    fn format_iso_date(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxDateTime_FormatISODate(self.as_ptr())) }
    }
    fn format_iso_time(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxDateTime_FormatISOTime(self.as_ptr())) }
    }
    fn format_time(&self) -> String {
        unsafe { wx_base::from_wx_string(ffi::wxDateTime_FormatTime(self.as_ptr())) }
    }
    fn parse_date(&self, date: &str, end: *mut c_void) -> bool {
        unsafe {
            let date = wx_base::wx_string_from(date);
            ffi::wxDateTime_ParseDate(self.as_ptr(), date, end)
        }
    }
    fn parse_date_time(&self, datetime: &str, end: *mut c_void) -> bool {
        unsafe {
            let datetime = wx_base::wx_string_from(datetime);
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
            let date = wx_base::wx_string_from(date);
            let format = wx_base::wx_string_from(format);
            let date_def = date_def.as_ptr();
            ffi::wxDateTime_ParseFormat(self.as_ptr(), date, format, date_def, end)
        }
    }
    // BLOCKED: fn ParseFormat1()
    // BLOCKED: fn ParseFormat2()
    // NOT_SUPPORTED: fn ParseISOCombined()
    fn parse_iso_date(&self, date: &str) -> bool {
        unsafe {
            let date = wx_base::wx_string_from(date);
            ffi::wxDateTime_ParseISODate(self.as_ptr(), date)
        }
    }
    fn parse_iso_time(&self, date: &str) -> bool {
        unsafe {
            let date = wx_base::wx_string_from(date);
            ffi::wxDateTime_ParseISOTime(self.as_ptr(), date)
        }
    }
    fn parse_rfc822_date(&self, date: &str, end: *mut c_void) -> bool {
        unsafe {
            let date = wx_base::wx_string_from(date);
            ffi::wxDateTime_ParseRfc822Date(self.as_ptr(), date, end)
        }
    }
    fn parse_time(&self, time: &str, end: *mut c_void) -> bool {
        unsafe {
            let time = wx_base::wx_string_from(time);
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
        unsafe { DateTimeIsOwned(ffi::wxDateTime_FromTimezone(self.as_ptr(), tz, no_dst)) }
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
        unsafe { DateTimeIsOwned(ffi::wxDateTime_ToTimezone(self.as_ptr(), tz, no_dst)) }
    }
    fn to_utc(&self, no_dst: bool) -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_ToUTC(self.as_ptr(), no_dst)) }
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
        unsafe { DateTimeIsOwned(ffi::wxDateTime_Now()) }
    }
    // NOT_SUPPORTED: fn SetCountry()
    // NOT_SUPPORTED: fn SetToWeekOfYear()
    fn today() -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_Today()) }
    }
    fn u_now() -> DateTime {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_UNow()) }
    }
}

// wxEvent
pub trait EventMethods: ObjectMethods {
    fn clone(&self) -> Event {
        unsafe { Event::from_ptr(ffi::wxEvent_Clone(self.as_ptr())) }
    }
    fn get_event_object(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventObject(self.as_ptr())) }
    }
    // NOT_SUPPORTED: fn GetEventType()
    // NOT_SUPPORTED: fn GetEventCategory()
    fn get_id(&self) -> c_int {
        unsafe { ffi::wxEvent_GetId(self.as_ptr()) }
    }
    fn get_event_user_data(&self) -> Option<ObjectIsOwned<false>> {
        unsafe { Object::option_from(ffi::wxEvent_GetEventUserData(self.as_ptr())) }
    }
    fn get_skipped(&self) -> bool {
        unsafe { ffi::wxEvent_GetSkipped(self.as_ptr()) }
    }
    fn get_timestamp(&self) -> c_long {
        unsafe { ffi::wxEvent_GetTimestamp(self.as_ptr()) }
    }
    fn is_command_event(&self) -> bool {
        unsafe { ffi::wxEvent_IsCommandEvent(self.as_ptr()) }
    }
    fn resume_propagation(&self, propagation_level: c_int) {
        unsafe { ffi::wxEvent_ResumePropagation(self.as_ptr(), propagation_level) }
    }
    fn set_event_object<O: ObjectMethods>(&self, object: Option<&O>) {
        unsafe {
            let object = match object {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvent_SetEventObject(self.as_ptr(), object)
        }
    }
    // NOT_SUPPORTED: fn SetEventType()
    fn set_id(&self, id: c_int) {
        unsafe { ffi::wxEvent_SetId(self.as_ptr(), id) }
    }
    fn set_timestamp(&self, time_stamp: c_long) {
        unsafe { ffi::wxEvent_SetTimestamp(self.as_ptr(), time_stamp) }
    }
    fn should_propagate(&self) -> bool {
        unsafe { ffi::wxEvent_ShouldPropagate(self.as_ptr()) }
    }
    fn skip(&self, skip: bool) {
        unsafe { ffi::wxEvent_Skip(self.as_ptr(), skip) }
    }
    fn stop_propagation(&self) -> c_int {
        unsafe { ffi::wxEvent_StopPropagation(self.as_ptr()) }
    }
}

// wxEvtHandler
pub trait EvtHandlerMethods: ObjectMethods {
    fn queue_event<E: EventMethods>(&self, event: Option<&E>) {
        unsafe {
            let event = match event {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_QueueEvent(self.as_ptr(), event)
        }
    }
    fn add_pending_event<E: EventMethods>(&self, event: &E) {
        unsafe {
            let event = event.as_ptr();
            ffi::wxEvtHandler_AddPendingEvent(self.as_ptr(), event)
        }
    }
    // NOT_SUPPORTED: fn CallAfter()
    // BLOCKED: fn CallAfter1()
    fn process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEvent(self.as_ptr(), event) }
    }
    fn process_event_locally(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_ProcessEventLocally(self.as_ptr(), event) }
    }
    fn safely_process_event(&self, event: *mut c_void) -> bool {
        unsafe { ffi::wxEvtHandler_SafelyProcessEvent(self.as_ptr(), event) }
    }
    fn process_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_ProcessPendingEvents(self.as_ptr()) }
    }
    fn delete_pending_events(&self) {
        unsafe { ffi::wxEvtHandler_DeletePendingEvents(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn Connect()
    // NOT_SUPPORTED: fn Connect1()
    // NOT_SUPPORTED: fn Connect2()
    // NOT_SUPPORTED: fn Disconnect()
    // NOT_SUPPORTED: fn Disconnect1()
    // NOT_SUPPORTED: fn Disconnect2()
    // NOT_SUPPORTED: fn Bind()
    // BLOCKED: fn Bind1()
    // NOT_SUPPORTED: fn Unbind()
    // BLOCKED: fn Unbind1()
    // BLOCKED: fn GetClientData()
    fn get_client_object(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_GetClientObject(self.as_ptr()) }
    }
    // BLOCKED: fn SetClientData()
    fn set_client_object(&self, data: *mut c_void) {
        unsafe { ffi::wxEvtHandler_SetClientObject(self.as_ptr(), data) }
    }
    fn get_evt_handler_enabled(&self) -> bool {
        unsafe { ffi::wxEvtHandler_GetEvtHandlerEnabled(self.as_ptr()) }
    }
    fn get_next_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetNextHandler(self.as_ptr())) }
    }
    fn get_previous_handler(&self) -> WeakRef<EvtHandler> {
        unsafe { WeakRef::<EvtHandler>::from(ffi::wxEvtHandler_GetPreviousHandler(self.as_ptr())) }
    }
    fn set_evt_handler_enabled(&self, enabled: bool) {
        unsafe { ffi::wxEvtHandler_SetEvtHandlerEnabled(self.as_ptr(), enabled) }
    }
    fn set_next_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetNextHandler(self.as_ptr(), handler)
        }
    }
    fn set_previous_handler<E: EvtHandlerMethods>(&self, handler: Option<&E>) {
        unsafe {
            let handler = match handler {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxEvtHandler_SetPreviousHandler(self.as_ptr(), handler)
        }
    }
    fn unlink(&self) {
        unsafe { ffi::wxEvtHandler_Unlink(self.as_ptr()) }
    }
    fn is_unlinked(&self) -> bool {
        unsafe { ffi::wxEvtHandler_IsUnlinked(self.as_ptr()) }
    }
    fn add_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_AddFilter(filter) }
    }
    fn remove_filter(filter: *mut c_void) {
        unsafe { ffi::wxEvtHandler_RemoveFilter(filter) }
    }
    // DTOR: fn ~wxEvtHandler()
}

// wxObject
pub trait ObjectMethods: WxRustMethods {
    // DTOR: fn ~wxObject()
    fn get_class_info(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetClassInfo(self.as_ptr()) }
    }
    fn get_ref_data(&self) -> *mut c_void {
        unsafe { ffi::wxObject_GetRefData(self.as_ptr()) }
    }
    fn is_kind_of(&self, info: *const c_void) -> bool {
        unsafe { ffi::wxObject_IsKindOf(self.as_ptr(), info) }
    }
    fn is_same_as<O: ObjectMethods>(&self, obj: &O) -> bool {
        unsafe {
            let obj = obj.as_ptr();
            ffi::wxObject_IsSameAs(self.as_ptr(), obj)
        }
    }
    fn ref_<O: ObjectMethods>(&self, clone: &O) {
        unsafe {
            let clone = clone.as_ptr();
            ffi::wxObject_Ref(self.as_ptr(), clone)
        }
    }
    fn set_ref_data(&self, data: *mut c_void) {
        unsafe { ffi::wxObject_SetRefData(self.as_ptr(), data) }
    }
    fn un_ref(&self) {
        unsafe { ffi::wxObject_UnRef(self.as_ptr()) }
    }
    fn un_share(&self) {
        unsafe { ffi::wxObject_UnShare(self.as_ptr()) }
    }
    // BLOCKED: fn operator delete()
    // BLOCKED: fn operator new()
}
