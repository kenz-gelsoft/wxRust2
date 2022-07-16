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

// wxFileName
pub trait FileNameMethods: WxRustMethods {
    fn append_dir(&self, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_AppendDir(self.as_ptr(), dir)
        }
    }
    fn assign<F: FileNameMethods>(&self, filepath: &F) {
        unsafe {
            let filepath = filepath.as_ptr();
            ffi::wxFileName_Assign(self.as_ptr(), filepath)
        }
    }
    // NOT_SUPPORTED: fn Assign1()
    // NOT_SUPPORTED: fn Assign2()
    // NOT_SUPPORTED: fn Assign3()
    // NOT_SUPPORTED: fn Assign4()
    // NOT_SUPPORTED: fn Assign5()
    fn assign_cwd(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_AssignCwd(self.as_ptr(), volume)
        }
    }
    // NOT_SUPPORTED: fn AssignDir()
    fn assign_home_dir(&self) {
        unsafe { ffi::wxFileName_AssignHomeDir(self.as_ptr()) }
    }
    fn assign_temp_file_name(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName(self.as_ptr(), prefix)
        }
    }
    fn assign_temp_file_name_file(&self, prefix: &str, file_temp: *mut c_void) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName1(self.as_ptr(), prefix, file_temp)
        }
    }
    fn assign_temp_file_name_ffile(&self, prefix: &str, file_temp: *mut c_void) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxFileName_AssignTempFileName2(self.as_ptr(), prefix, file_temp)
        }
    }
    fn clear(&self) {
        unsafe { ffi::wxFileName_Clear(self.as_ptr()) }
    }
    fn clear_ext(&self) {
        unsafe { ffi::wxFileName_ClearExt(self.as_ptr()) }
    }
    fn dir_exists(&self) -> bool {
        unsafe { ffi::wxFileName_DirExists(self.as_ptr()) }
    }
    fn dont_follow_link(&self) {
        unsafe { ffi::wxFileName_DontFollowLink(self.as_ptr()) }
    }
    fn exists_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Exists(self.as_ptr(), flags) }
    }
    fn file_exists(&self) -> bool {
        unsafe { ffi::wxFileName_FileExists(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn GetAbsolutePath()
    fn get_dir_count(&self) -> usize {
        unsafe { ffi::wxFileName_GetDirCount(self.as_ptr()) }
    }
    fn get_dirs(&self) -> ArrayStringIsOwned<false> {
        unsafe { ArrayStringIsOwned::from_ptr(ffi::wxFileName_GetDirs(self.as_ptr())) }
    }
    fn get_ext(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetExt(self.as_ptr())).into() }
    }
    fn get_full_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetFullName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetFullPath()
    // NOT_SUPPORTED: fn GetHumanReadableSize()
    fn get_long_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetLongPath(self.as_ptr())).into() }
    }
    fn get_modification_time(&self) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxFileName_GetModificationTime(self.as_ptr())) }
    }
    fn get_name(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetName(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetPath()
    // NOT_SUPPORTED: fn GetPathWithSep()
    fn get_short_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetShortPath(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetSize()
    fn get_times<D: DateTimeMethods, D2: DateTimeMethods, D3: DateTimeMethods>(
        &self,
        dt_access: Option<&D>,
        dt_mod: Option<&D2>,
        dt_create: Option<&D3>,
    ) -> bool {
        unsafe {
            let dt_access = match dt_access {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_mod = match dt_mod {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_create = match dt_create {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_GetTimes(self.as_ptr(), dt_access, dt_mod, dt_create)
        }
    }
    fn get_volume(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetVolume(self.as_ptr())).into() }
    }
    fn has_ext(&self) -> bool {
        unsafe { ffi::wxFileName_HasExt(self.as_ptr()) }
    }
    fn has_name(&self) -> bool {
        unsafe { ffi::wxFileName_HasName(self.as_ptr()) }
    }
    fn has_volume(&self) -> bool {
        unsafe { ffi::wxFileName_HasVolume(self.as_ptr()) }
    }
    fn insert_dir(&self, before: usize, dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_InsertDir(self.as_ptr(), before, dir)
        }
    }
    // NOT_SUPPORTED: fn IsAbsolute()
    fn is_dir(&self) -> bool {
        unsafe { ffi::wxFileName_IsDir(self.as_ptr()) }
    }
    fn is_dir_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirReadable(self.as_ptr()) }
    }
    fn is_dir_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsDirWritable(self.as_ptr()) }
    }
    fn is_file_executable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileExecutable(self.as_ptr()) }
    }
    fn is_file_readable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileReadable(self.as_ptr()) }
    }
    fn is_file_writable(&self) -> bool {
        unsafe { ffi::wxFileName_IsFileWritable(self.as_ptr()) }
    }
    fn is_ok(&self) -> bool {
        unsafe { ffi::wxFileName_IsOk(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsRelative()
    // NOT_SUPPORTED: fn MakeAbsolute()
    // NOT_SUPPORTED: fn MakeRelativeTo()
    fn mkdir_int(&self, perm: c_int, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Mkdir(self.as_ptr(), perm, flags) }
    }
    // NOT_SUPPORTED: fn Normalize()
    fn prepend_dir(&self, dir: &str) {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_PrependDir(self.as_ptr(), dir)
        }
    }
    fn remove_dir(&self, pos: usize) {
        unsafe { ffi::wxFileName_RemoveDir(self.as_ptr(), pos) }
    }
    fn remove_last_dir(&self) {
        unsafe { ffi::wxFileName_RemoveLastDir(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn ReplaceEnvVariable()
    // NOT_SUPPORTED: fn ReplaceHomeDir()
    fn resolve_link(&self) -> FileName {
        unsafe { FileName::from_ptr(ffi::wxFileName_ResolveLink(self.as_ptr())) }
    }
    fn rmdir_int(&self, flags: c_int) -> bool {
        unsafe { ffi::wxFileName_Rmdir(self.as_ptr(), flags) }
    }
    // NOT_SUPPORTED: fn SameAs()
    fn set_cwd(&self) -> bool {
        unsafe { ffi::wxFileName_SetCwd(self.as_ptr()) }
    }
    fn set_empty_ext(&self) {
        unsafe { ffi::wxFileName_SetEmptyExt(self.as_ptr()) }
    }
    fn set_ext(&self, ext: &str) {
        unsafe {
            let ext = WxString::from(ext);
            let ext = ext.as_ptr();
            ffi::wxFileName_SetExt(self.as_ptr(), ext)
        }
    }
    fn set_full_name(&self, fullname: &str) {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            ffi::wxFileName_SetFullName(self.as_ptr(), fullname)
        }
    }
    fn set_name(&self, name: &str) {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxFileName_SetName(self.as_ptr(), name)
        }
    }
    // NOT_SUPPORTED: fn SetPath()
    fn set_permissions(&self, permissions: c_int) -> bool {
        unsafe { ffi::wxFileName_SetPermissions(self.as_ptr(), permissions) }
    }
    fn set_times<D: DateTimeMethods, D2: DateTimeMethods, D3: DateTimeMethods>(
        &self,
        dt_access: Option<&D>,
        dt_mod: Option<&D2>,
        dt_create: Option<&D3>,
    ) -> bool {
        unsafe {
            let dt_access = match dt_access {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_mod = match dt_mod {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let dt_create = match dt_create {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ffi::wxFileName_SetTimes(self.as_ptr(), dt_access, dt_mod, dt_create)
        }
    }
    fn set_volume(&self, volume: &str) {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            ffi::wxFileName_SetVolume(self.as_ptr(), volume)
        }
    }
    fn should_follow_link(&self) -> bool {
        unsafe { ffi::wxFileName_ShouldFollowLink(self.as_ptr()) }
    }
    fn touch(&self) -> bool {
        unsafe { ffi::wxFileName_Touch(self.as_ptr()) }
    }
    // BLOCKED: fn operator!=()
    // BLOCKED: fn operator!=1()
    // BLOCKED: fn operator==()
    // BLOCKED: fn operator==1()
    // BLOCKED: fn operator=()
    // BLOCKED: fn operator=1()
    fn create_temp_file_name_file(prefix: &str, file_temp: *mut c_void) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName(prefix, file_temp)).into()
        }
    }
    fn create_temp_file_name_ffile(prefix: &str, file_temp: *mut c_void) -> String {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            WxString::from_ptr(ffi::wxFileName_CreateTempFileName1(prefix, file_temp)).into()
        }
    }
    fn dir_exists_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_DirExists1(dir)
        }
    }
    // NOT_SUPPORTED: fn DirName()
    fn exists_str(path: &str, flags: c_int) -> bool {
        unsafe {
            let path = WxString::from(path);
            let path = path.as_ptr();
            ffi::wxFileName_Exists1(path, flags)
        }
    }
    fn file_exists_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_FileExists1(file)
        }
    }
    // NOT_SUPPORTED: fn FileName()
    fn get_cwd(volume: &str) -> String {
        unsafe {
            let volume = WxString::from(volume);
            let volume = volume.as_ptr();
            WxString::from_ptr(ffi::wxFileName_GetCwd(volume)).into()
        }
    }
    // NOT_SUPPORTED: fn GetForbiddenChars()
    // NOT_SUPPORTED: fn GetFormat()
    fn get_home_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetHomeDir()).into() }
    }
    // NOT_SUPPORTED: fn GetHumanReadableSize1()
    // NOT_SUPPORTED: fn GetPathSeparator()
    // NOT_SUPPORTED: fn GetPathSeparators()
    // NOT_SUPPORTED: fn GetPathTerminators()
    // NOT_SUPPORTED: fn GetSize1()
    fn get_temp_dir() -> String {
        unsafe { WxString::from_ptr(ffi::wxFileName_GetTempDir()).into() }
    }
    // NOT_SUPPORTED: fn GetVolumeSeparator()
    // NOT_SUPPORTED: fn GetVolumeString()
    // NOT_SUPPORTED: fn IsCaseSensitive()
    fn is_dir_readable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirReadable1(dir)
        }
    }
    fn is_dir_writable_str(dir: &str) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_IsDirWritable1(dir)
        }
    }
    fn is_file_executable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileExecutable1(file)
        }
    }
    fn is_file_readable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileReadable1(file)
        }
    }
    fn is_file_writable_str(file: &str) -> bool {
        unsafe {
            let file = WxString::from(file);
            let file = file.as_ptr();
            ffi::wxFileName_IsFileWritable1(file)
        }
    }
    // NOT_SUPPORTED: fn IsPathSeparator()
    // NOT_SUPPORTED: fn IsMSWUniqueVolumeNamePath()
    fn mkdir_str(dir: &str, perm: c_int, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Mkdir1(dir, perm, flags)
        }
    }
    fn rmdir_str(dir: &str, flags: c_int) -> bool {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            ffi::wxFileName_Rmdir1(dir, flags)
        }
    }
    fn set_cwd_str(cwd: &str) -> bool {
        unsafe {
            let cwd = WxString::from(cwd);
            let cwd = cwd.as_ptr();
            ffi::wxFileName_SetCwd1(cwd)
        }
    }
    fn url_to_file_name(url: &str) -> FileName {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            FileName::from_ptr(ffi::wxFileName_URLToFileName(url))
        }
    }
    fn file_name_to_url<F: FileNameMethods>(filename: &F) -> String {
        unsafe {
            let filename = filename.as_ptr();
            WxString::from_ptr(ffi::wxFileName_FileNameToURL(filename)).into()
        }
    }
    // NOT_SUPPORTED: fn SplitPath()
    // NOT_SUPPORTED: fn SplitPath1()
    // NOT_SUPPORTED: fn SplitPath2()
    // NOT_SUPPORTED: fn SplitVolume()
    fn strip_extension(fullname: &str) -> String {
        unsafe {
            let fullname = WxString::from(fullname);
            let fullname = fullname.as_ptr();
            WxString::from_ptr(ffi::wxFileName_StripExtension(fullname)).into()
        }
    }
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

// wxStandardPaths
pub trait StandardPathsMethods: WxRustMethods {
    fn dont_ignore_app_sub_dir(&self) {
        unsafe { ffi::wxStandardPaths_DontIgnoreAppSubDir(self.as_ptr()) }
    }
    fn get_app_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetAppDocumentsDir(self.as_ptr())).into() }
    }
    fn get_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetConfigDir(self.as_ptr())).into() }
    }
    fn get_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDataDir(self.as_ptr())).into() }
    }
    fn get_documents_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetDocumentsDir(self.as_ptr())).into() }
    }
    fn get_executable_path(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetExecutablePath(self.as_ptr())).into() }
    }
    fn get_install_prefix(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetInstallPrefix(self.as_ptr())).into() }
    }
    fn get_local_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetLocalDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetLocalizedResourcesDir()
    fn get_plugins_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetPluginsDir(self.as_ptr())).into() }
    }
    fn get_resources_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetResourcesDir(self.as_ptr())).into() }
    }
    fn get_temp_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetTempDir(self.as_ptr())).into() }
    }
    fn get_user_config_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserConfigDir(self.as_ptr())).into() }
    }
    fn get_user_data_dir(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_GetUserDataDir(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn GetUserDir()
    fn get_user_local_data_dir(&self) -> String {
        unsafe {
            WxString::from_ptr(ffi::wxStandardPaths_GetUserLocalDataDir(self.as_ptr())).into()
        }
    }
    fn ignore_app_sub_dir(&self, subdir_pattern: &str) {
        unsafe {
            let subdir_pattern = WxString::from(subdir_pattern);
            let subdir_pattern = subdir_pattern.as_ptr();
            ffi::wxStandardPaths_IgnoreAppSubDir(self.as_ptr(), subdir_pattern)
        }
    }
    fn ignore_app_build_sub_dirs(&self) {
        unsafe { ffi::wxStandardPaths_IgnoreAppBuildSubDirs(self.as_ptr()) }
    }
    fn set_install_prefix(&self, prefix: &str) {
        unsafe {
            let prefix = WxString::from(prefix);
            let prefix = prefix.as_ptr();
            ffi::wxStandardPaths_SetInstallPrefix(self.as_ptr(), prefix)
        }
    }
    fn use_app_info(&self, info: c_int) {
        unsafe { ffi::wxStandardPaths_UseAppInfo(self.as_ptr(), info) }
    }
    // NOT_SUPPORTED: fn SetFileLayout()
    // NOT_SUPPORTED: fn GetFileLayout()
    // NOT_SUPPORTED: fn MakeConfigFileName()
    fn get() -> StandardPathsIsOwned<false> {
        unsafe { StandardPathsIsOwned::from_ptr(ffi::wxStandardPaths_Get()) }
    }
    fn msw_get_shell_dir(csidl: c_int) -> String {
        unsafe { WxString::from_ptr(ffi::wxStandardPaths_MSWGetShellDir(csidl)).into() }
    }
}
