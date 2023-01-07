use super::*;

// wxDateTime
/// This trait represents [C++ `wxDateTime` class](https://docs.wxwidgets.org/3.2/classwx_date_time.html)'s methods and inheritance.
///
/// See [`DateTimeInRust`] documentation for the class usage.
pub trait DateTimeMethods: WxRustMethods {
    /// Reset time to midnight (00:00:00) without changing the date.
    ///
    /// See [C++ `wxDateTime::ResetTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#acb537b1ef0b83b5a6724140c823a547c).
    fn reset_time(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_ResetTime(self.as_ptr());
            &self
        }
    }
    // NOT_SUPPORTED: fn Set()
    // BLOCKED: fn Set1()
    /// Sets the date and time from the broken down representation in the wxDateTime::Tm structure.
    ///
    /// See [C++ `wxDateTime::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aceceac43ca2c9741401e7e34c588f087).
    fn set_tm(&self, tm: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Set2(self.as_ptr(), tm);
            &self
        }
    }
    /// Sets the date from the so-called Julian Day Number.
    ///
    /// See [C++ `wxDateTime::Set()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ae24090c2c5f3ab830d5edacc2c075f60).
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
    ///
    /// See [C++ `wxDateTime::SetToCurrent()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a10db5e371bada75b91435f4367cbdaa1).
    fn set_to_current(&self) -> &Self {
        unsafe {
            ffi::wxDateTime_SetToCurrent(self.as_ptr());
            &self
        }
    }
    /// Sets the year without changing other date components.
    ///
    /// See [C++ `wxDateTime::SetYear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ab574229577dcf8b1e430873f6701fa43).
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
    ///
    /// See [C++ `wxDateTime::SetFromMSWSysTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ae6eb1bdd0c691d82493a40e714705860).
    fn set_from_msw_sys_time(&self, st: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_SetFromMSWSysTime(self.as_ptr(), st);
            &self
        }
    }
    /// Returns the date and time in the Windows SYSTEMTIME format.
    ///
    /// See [C++ `wxDateTime::GetAsMSWSysTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a9f760ab782406fd9d4db1e983b5490c3).
    fn get_as_msw_sys_time(&self, st: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAsMSWSysTime(self.as_ptr(), st) }
    }
    /// Returns the century of this date.
    ///
    /// See [C++ `wxDateTime::GetCentury()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a7646edf22af11b72e503046c0bff3e17).
    fn get_century_timezone(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetCentury(self.as_ptr(), tz) }
    }
    /// Returns the object having the same date component as this one but time of 00:00:00.
    ///
    /// See [C++ `wxDateTime::GetDateOnly()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a347ff4b250b96471b4ba10806b5e9fc4).
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
    ///
    /// See [C++ `wxDateTime::GetWeekBasedYear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ad083f14ab1094ed025f1634523f2ea44).
    fn get_week_based_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetWeekBasedYear(self.as_ptr(), tz) }
    }
    // NOT_SUPPORTED: fn GetWeekOfMonth()
    // NOT_SUPPORTED: fn GetWeekOfYear()
    /// Returns the year in the given timezone (local one by default).
    ///
    /// See [C++ `wxDateTime::GetYear()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ab20a8e0aa4aefe4dda3f93b1e75123d3).
    fn get_year(&self, tz: *const c_void) -> c_int {
        unsafe { ffi::wxDateTime_GetYear(self.as_ptr(), tz) }
    }
    /// Returns true if the object represents a valid time moment.
    ///
    /// See [C++ `wxDateTime::IsValid()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#afd5350c926b486e77478d7c7d4fd7b9c).
    fn is_valid(&self) -> bool {
        unsafe { ffi::wxDateTime_IsValid(self.as_ptr()) }
    }
    // NOT_SUPPORTED: fn IsWorkDay()
    /// Returns true if this date precedes the given one.
    ///
    /// See [C++ `wxDateTime::IsEarlierThan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a6d83ca062bb51235db017b3d855dbb5d).
    fn is_earlier_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEarlierThan(self.as_ptr(), datetime)
        }
    }
    /// Returns true if the two dates are strictly identical.
    ///
    /// See [C++ `wxDateTime::IsEqualTo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ac0cefa368b364f2bde861b668fb1b71f).
    fn is_equal_to<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsEqualTo(self.as_ptr(), datetime)
        }
    }
    /// Returns true if the date is equal to another one up to the given time interval, i.e. if the absolute difference between the two dates is less than this interval.
    ///
    /// See [C++ `wxDateTime::IsEqualUpTo()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aff57fac0e65af21cb4d55b83a3aed269).
    fn is_equal_up_to<D: DateTimeMethods>(&self, dt: &D, ts: *const c_void) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsEqualUpTo(self.as_ptr(), dt, ts)
        }
    }
    /// Returns true if this date is later than the given one.
    ///
    /// See [C++ `wxDateTime::IsLaterThan()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a4d3321c1e76f4c132fd2d0b302498824).
    fn is_later_than<D: DateTimeMethods>(&self, datetime: &D) -> bool {
        unsafe {
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_IsLaterThan(self.as_ptr(), datetime)
        }
    }
    /// Returns true if the date is the same without comparing the time parts.
    ///
    /// See [C++ `wxDateTime::IsSameDate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#abb147359541012d11ca48f6db2be7c32).
    fn is_same_date<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameDate(self.as_ptr(), dt)
        }
    }
    /// Returns true if the time is the same (although dates may differ).
    ///
    /// See [C++ `wxDateTime::IsSameTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a7d22bbf0ccab5c71a7173d8fa8a7cd41).
    fn is_same_time<D: DateTimeMethods>(&self, dt: &D) -> bool {
        unsafe {
            let dt = dt.as_ptr();
            ffi::wxDateTime_IsSameTime(self.as_ptr(), dt)
        }
    }
    /// Returns true if this date lies strictly between the two given dates.
    ///
    /// See [C++ `wxDateTime::IsStrictlyBetween()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a550f265925b235de51e99a50222963fa).
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
    ///
    /// See [C++ `wxDateTime::IsBetween()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a0a5d356a798f5c95c729e6b5193dd9b1).
    fn is_between<D: DateTimeMethods, D2: DateTimeMethods>(&self, t1: &D, t2: &D2) -> bool {
        unsafe {
            let t1 = t1.as_ptr();
            let t2 = t2.as_ptr();
            ffi::wxDateTime_IsBetween(self.as_ptr(), t1, t2)
        }
    }
    // BLOCKED: fn Add()
    /// Adds the given date span to this object.
    ///
    /// See [C++ `wxDateTime::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a18e32b187b6d6c94be921d4716b6366f).
    fn add_datespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Add2()
    /// Adds the given time span to this object.
    ///
    /// See [C++ `wxDateTime::Add()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ab4cae178a6004ba47a6fbac302c35d3d).
    fn add_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Add3(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract()
    /// Subtracts the given time span from this object.
    ///
    /// See [C++ `wxDateTime::Subtract()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a78a988d03c8cca570d2f0294d987bc24).
    fn subtract_timespan(&self, diff: *const c_void) -> &Self {
        unsafe {
            ffi::wxDateTime_Subtract1(self.as_ptr(), diff);
            &self
        }
    }
    // BLOCKED: fn Subtract2()
    /// Subtracts the given date span from this object.
    ///
    /// See [C++ `wxDateTime::Subtract()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aa8405d7b2d413775039a8ef5d65b9b80).
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
    ///
    /// See [C++ `wxDateTime::Format()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a6a60f071d74712eba58153113f5af23a).
    fn format(&self, format: &str, tz: *const c_void) -> String {
        unsafe {
            let format = WxString::from(format);
            let format = format.as_ptr();
            WxString::from_ptr(ffi::wxDateTime_Format(self.as_ptr(), format, tz)).into()
        }
    }
    /// Identical to calling Format() with "%x" argument (which means "preferred date representation for the current locale").
    ///
    /// See [C++ `wxDateTime::FormatDate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aeca948cb9628daabf2953a6a635a74b3).
    fn format_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatDate(self.as_ptr())).into() }
    }
    // NOT_SUPPORTED: fn FormatISOCombined()
    /// This function returns the date representation in the ISO 8601 format "YYYY-MM-DD".
    ///
    /// See [C++ `wxDateTime::FormatISODate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aed6224e62881d28d60e54eefcb6798f1).
    fn format_iso_date(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISODate(self.as_ptr())).into() }
    }
    /// This function returns the time representation in the ISO 8601 format "HH:MM:SS".
    ///
    /// See [C++ `wxDateTime::FormatISOTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aa370d17862e81fb89d78e6e087374320).
    fn format_iso_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatISOTime(self.as_ptr())).into() }
    }
    /// Identical to calling Format() with "%X" argument (which means "preferred time representation for the current locale").
    ///
    /// See [C++ `wxDateTime::FormatTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a6f9e71f21079e007368040322005804f).
    fn format_time(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxDateTime_FormatTime(self.as_ptr())).into() }
    }
    // BLOCKED: fn ParseDate()
    /// Parses the string datetime containing the date and time in free format.
    ///
    /// See [C++ `wxDateTime::ParseDateTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a332a41f998bc3be6c0a9a81449112485).
    fn parse_date_time(&self, datetime: &str, end: *mut c_void) -> bool {
        unsafe {
            let datetime = WxString::from(datetime);
            let datetime = datetime.as_ptr();
            ffi::wxDateTime_ParseDateTime(self.as_ptr(), datetime, end)
        }
    }
    /// This function parses the string date according to the given format.
    ///
    /// See [C++ `wxDateTime::ParseFormat()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a5d01d56c310d505a093f1a7f25086a1b).
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
    ///
    /// See [C++ `wxDateTime::ParseISODate()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a416e9e5e7395dd11a059cd116da7e6a2).
    fn parse_iso_date(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISODate(self.as_ptr(), date)
        }
    }
    /// This function parses the time in ISO 8601 format "HH:MM:SS".
    ///
    /// See [C++ `wxDateTime::ParseISOTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#adc4c1fb178f73958b151eabd36625f59).
    fn parse_iso_time(&self, date: &str) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseISOTime(self.as_ptr(), date)
        }
    }
    /// Parses the string date looking for a date formatted according to the RFC 822 in it.
    ///
    /// See [C++ `wxDateTime::ParseRfc822Date()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ab0f2485c52d0d3cf0196aaf0062d2f83).
    fn parse_rfc822_date(&self, date: &str, end: *mut c_void) -> bool {
        unsafe {
            let date = WxString::from(date);
            let date = date.as_ptr();
            ffi::wxDateTime_ParseRfc822Date(self.as_ptr(), date, end)
        }
    }
    /// This functions is like ParseDateTime(), but only allows the time to be specified in the input string.
    ///
    /// See [C++ `wxDateTime::ParseTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ac903b3227620e1b88019e76b807327e8).
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
    ///
    /// See [C++ `wxDateTime::GetJDN()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a0dd134019f8aa14da326f692575cbd22).
    fn get_jdn(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJDN(self.as_ptr()) }
    }
    /// Returns the JDN corresponding to this date.
    ///
    /// See [C++ `wxDateTime::GetJulianDayNumber()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a32b6bf7b2af58f8f3430796008544943).
    fn get_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetJulianDayNumber(self.as_ptr()) }
    }
    /// Synonym for GetModifiedJulianDayNumber().
    ///
    /// See [C++ `wxDateTime::GetMJD()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#afd05be08550faff9bd1001c91313e637).
    fn get_mjd(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetMJD(self.as_ptr()) }
    }
    /// Returns the "Modified Julian Day Number" (MJD) which is, by definition, is equal to JDN - 2400000.5.
    ///
    /// See [C++ `wxDateTime::GetModifiedJulianDayNumber()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ab44bcea2d91e3562217e83f4a4c583ac).
    fn get_modified_julian_day_number(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetModifiedJulianDayNumber(self.as_ptr()) }
    }
    /// Return the Rata Die number of this date.
    ///
    /// See [C++ `wxDateTime::GetRataDie()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a77384d94d7fbbb393c5b3ac239e744e5).
    fn get_rata_die(&self) -> c_double {
        unsafe { ffi::wxDateTime_GetRataDie(self.as_ptr()) }
    }
    /// Transform the date from the given time zone to the local one.
    ///
    /// See [C++ `wxDateTime::FromTimezone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a64ecc639352e2fdb393d07131ef1f402).
    fn from_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_FromTimezone(self.as_ptr(), tz, no_dst)) }
    }
    // NOT_SUPPORTED: fn IsDST()
    /// Same as FromTimezone() but modifies the object in place.
    ///
    /// See [C++ `wxDateTime::MakeFromTimezone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ac3311659f9a0c46a19a563e683f076b8).
    fn make_from_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeFromTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    /// Modifies the object in place to represent the date in another time zone.
    ///
    /// See [C++ `wxDateTime::MakeTimezone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a4782b88f89f129c7e8127716cf3df58b).
    fn make_timezone(&self, tz: *const c_void, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeTimezone(self.as_ptr(), tz, no_dst);
            &self
        }
    }
    /// This is the same as calling MakeTimezone() with the argument GMT0.
    ///
    /// See [C++ `wxDateTime::MakeUTC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a6c2fdc74ad77726e164b8139fbf2eaf3).
    fn make_utc(&self, no_dst: bool) -> &Self {
        unsafe {
            ffi::wxDateTime_MakeUTC(self.as_ptr(), no_dst);
            &self
        }
    }
    /// Transform the date to the given time zone.
    ///
    /// See [C++ `wxDateTime::ToTimezone()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ac7c94044cece74f9e8b357204c460f5f).
    fn to_timezone(&self, tz: *const c_void, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToTimezone(self.as_ptr(), tz, no_dst)) }
    }
    /// This is the same as calling ToTimezone() with the argument GMT0.
    ///
    /// See [C++ `wxDateTime::ToUTC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a528886a6a80ec4490b2a16679c10a683).
    fn to_utc(&self, no_dst: bool) -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_ToUTC(self.as_ptr(), no_dst)) }
    }
    /// Converts the year in absolute notation (i.e. a number which can be negative, positive or zero) to the year in BC/AD notation.
    ///
    /// See [C++ `wxDateTime::ConvertYearToBC()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a52e2fe22ededf8aaf3922a3e8136ddb0).
    fn convert_year_to_bc(year: c_int) -> c_int {
        unsafe { ffi::wxDateTime_ConvertYearToBC(year) }
    }
    /// Returns the translations of the strings AM and PM used for time formatting for the current locale.
    ///
    /// See [C++ `wxDateTime::GetAmPmStrings()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a9ef535b0db4eeb117858a5b31ac00ae5).
    fn get_am_pm_strings(am: *mut c_void, pm: *mut c_void) {
        unsafe { ffi::wxDateTime_GetAmPmStrings(am, pm) }
    }
    // NOT_SUPPORTED: fn GetBeginDST()
    // NOT_SUPPORTED: fn GetEndDST()
    /// Get the current century, i.e. first two digits of the year, in given calendar (only Gregorian is currently supported).
    ///
    /// See [C++ `wxDateTime::GetCentury()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aa7da8657207e8872291e757bc6065b61).
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
    ///
    /// See [C++ `wxDateTime::GetTmNow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ae55f6ba5606beb7a16e8eae572c90c52).
    fn get_tm_now() -> *mut c_void {
        unsafe { ffi::wxDateTime_GetTmNow1() }
    }
    // NOT_SUPPORTED: fn GetWeekDayName()
    // NOT_SUPPORTED: fn IsDSTApplicable()
    /// Acquires the first weekday of a week based on locale and/or OS settings.
    ///
    /// See [C++ `wxDateTime::GetFirstWeekDay()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#ae192eb65585f8b80e127c96ec1f97e83).
    fn get_first_week_day(first_day: *mut c_void) -> bool {
        unsafe { ffi::wxDateTime_GetFirstWeekDay(first_day) }
    }
    // NOT_SUPPORTED: fn IsLeapYear()
    // NOT_SUPPORTED: fn IsWestEuropeanCountry()
    /// Returns the object corresponding to the current time in local time zone.
    ///
    /// See [C++ `wxDateTime::Now()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a6e6c37a0414bb4831e2cc03b37f498a2).
    fn now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Now()) }
    }
    // NOT_SUPPORTED: fn SetCountry()
    // NOT_SUPPORTED: fn SetToWeekOfYear()
    /// Returns the object corresponding to the midnight of the current day (i.e. the same as Now(), but the time part is set to 0).
    ///
    /// See [C++ `wxDateTime::Today()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#af730202238b9acd5a5df2977fa40c569).
    fn today() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_Today()) }
    }
    /// Returns the object corresponding to the current time including the milliseconds.
    ///
    /// See [C++ `wxDateTime::UNow()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a0db1c9bac7b2d27d589cad2a38d1438c).
    fn u_now() -> DateTime {
        unsafe { DateTime::from_ptr(ffi::wxDateTime_UNow()) }
    }
}
