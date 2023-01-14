use super::*;

// wxDateTime
wxwidgets! {
    /// wxDateTime class represents an absolute moment in time.
    /// - [`DateTime`] represents a C++ `wxDateTime` class instance which your code has ownership, [`DateTimeFromCpp`]`<true>` represents one which don't own.
    /// - Use [`DateTime`]'s `new()` to create an instance of this class.
    /// - See [C++ `wxDateTime` class's documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html) for more details.
    #[doc(alias = "wxDateTime")]
    #[doc(alias = "DateTime")]
    class DateTime
        = DateTimeFromCpp<false>(wxDateTime) impl
        DateTimeMethods
}
impl<const FROM_CPP: bool> DateTimeFromCpp<FROM_CPP> {
    //  ENUM: TZ
    pub const Local: c_int = 0;
    pub const GMT_12: c_int = 0 + 1;
    pub const GMT_11: c_int = 0 + 2;
    pub const GMT_10: c_int = 0 + 3;
    pub const GMT_9: c_int = 0 + 4;
    pub const GMT_8: c_int = 0 + 5;
    pub const GMT_7: c_int = 0 + 6;
    pub const GMT_6: c_int = 0 + 7;
    pub const GMT_5: c_int = 0 + 8;
    pub const GMT_4: c_int = 0 + 9;
    pub const GMT_3: c_int = 0 + 10;
    pub const GMT_2: c_int = 0 + 11;
    pub const GMT_1: c_int = 0 + 12;
    pub const GMT0: c_int = 0 + 13;
    pub const GMT1: c_int = 0 + 14;
    pub const GMT2: c_int = 0 + 15;
    pub const GMT3: c_int = 0 + 16;
    pub const GMT4: c_int = 0 + 17;
    pub const GMT5: c_int = 0 + 18;
    pub const GMT6: c_int = 0 + 19;
    pub const GMT7: c_int = 0 + 20;
    pub const GMT8: c_int = 0 + 21;
    pub const GMT9: c_int = 0 + 22;
    pub const GMT10: c_int = 0 + 23;
    pub const GMT11: c_int = 0 + 24;
    pub const GMT12: c_int = 0 + 25;
    pub const GMT13: c_int = 0 + 26;
    //  SKIP: WET
    //  SKIP: WEST
    //  SKIP: CET
    //  SKIP: CEST
    //  SKIP: EET
    //  SKIP: EEST
    //  SKIP: MSK
    //  SKIP: MSD
    //  SKIP: AST
    //  SKIP: ADT
    //  SKIP: EST
    //  SKIP: EDT
    //  SKIP: CST
    //  SKIP: CDT
    //  SKIP: MST
    //  SKIP: MDT
    //  SKIP: PST
    //  SKIP: PDT
    //  SKIP: HST
    //  SKIP: AKST
    //  SKIP: AKDT
    //  SKIP: A_WST
    //  SKIP: A_CST
    //  SKIP: A_EST
    //  SKIP: A_ESST
    //  SKIP: NZST
    //  SKIP: NZDT
    //  SKIP: UTC

    //  ENUM: Calendar
    pub const Gregorian: c_int = 0;
    pub const Julian: c_int = 0 + 1;

    //  ENUM: Country
    pub const Country_Unknown: c_int = 0;
    pub const Country_Default: c_int = 0 + 1;
    pub const Country_WesternEurope_Start: c_int = 0 + 2;
    //  SKIP: Country_EEC
    //  SKIP: France
    //  SKIP: Germany
    //  SKIP: UK
    //  SKIP: Country_WesternEurope_End
    //  SKIP: Russia
    //  SKIP: USA

    //  ENUM: Month
    pub const Jan: c_int = 0;
    pub const Feb: c_int = 0 + 1;
    pub const Mar: c_int = 0 + 2;
    pub const Apr: c_int = 0 + 3;
    pub const May: c_int = 0 + 4;
    pub const Jun: c_int = 0 + 5;
    pub const Jul: c_int = 0 + 6;
    pub const Aug: c_int = 0 + 7;
    pub const Sep: c_int = 0 + 8;
    pub const Oct: c_int = 0 + 9;
    pub const Nov: c_int = 0 + 10;
    pub const Dec: c_int = 0 + 11;
    pub const Inv_Month: c_int = 0 + 12;

    //  ENUM: WeekDay
    pub const Sun: c_int = 0;
    pub const Mon: c_int = 0 + 1;
    pub const Tue: c_int = 0 + 2;
    pub const Wed: c_int = 0 + 3;
    pub const Thu: c_int = 0 + 4;
    pub const Fri: c_int = 0 + 5;
    pub const Sat: c_int = 0 + 6;
    pub const Inv_WeekDay: c_int = 0 + 7;

    //  ENUM: Year
    //  SKIP: Inv_Year

    //  ENUM: NameFlags
    pub const Name_Full: c_int = 0x01;
    pub const Name_Abbr: c_int = 0x02;

    //  ENUM: WeekFlags
    pub const Default_First: c_int = 0;
    pub const Monday_First: c_int = 0 + 1;
    pub const Sunday_First: c_int = 0 + 2;

    /// Default constructor.
    ///
    /// See [C++ `wxDateTime::wxDateTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#a4cc372429453a21632d64f34f635d853).
    pub fn new() -> DateTimeFromCpp<FROM_CPP> {
        unsafe { DateTimeFromCpp(ffi::wxDateTime_new()) }
    }
    /// Copy constructor.
    ///
    /// See [C++ `wxDateTime::wxDateTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aca2bc3b942d920e01e496841bd759001).
    pub fn new_with_datetime<D: DateTimeMethods>(date: &D) -> DateTimeFromCpp<FROM_CPP> {
        unsafe {
            let date = date.as_ptr();
            DateTimeFromCpp(ffi::wxDateTime_new1(date))
        }
    }
    // NOT_SUPPORTED: fn wxDateTime2()
    // BLOCKED: fn wxDateTime3()
    /// Same as Set().
    ///
    /// See [C++ `wxDateTime::wxDateTime()`'s documentation](https://docs.wxwidgets.org/3.2/classwx_date_time.html#aa2c38922eafec2a94fb5ee9221c0f6b9).
    pub fn new_with_double(jdn: c_double) -> DateTimeFromCpp<FROM_CPP> {
        unsafe { DateTimeFromCpp(ffi::wxDateTime_new4(jdn)) }
    }
    // NOT_SUPPORTED: fn wxDateTime5()
    // NOT_SUPPORTED: fn wxDateTime6()
    // BLOCKED: fn wxDateTime7()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for DateTimeFromCpp<true> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const FROM_CPP: bool> Drop for DateTimeFromCpp<FROM_CPP> {
    fn drop(&mut self) {
        if !FROM_CPP {
            unsafe { ffi::wxDateTime_delete(self.0) }
        }
    }
}
