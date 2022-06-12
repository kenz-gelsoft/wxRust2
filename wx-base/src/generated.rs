#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]

use std::mem;
use std::os::raw::{c_double, c_int, c_long, c_uchar, c_void};
use std::ptr;

use super::*;
use methods::*;

use crate::wx_class;

mod ffi;
pub mod methods;

// wxDateTime
wx_class! { DateTime =
    DateTimeIsOwned<true>(wxDateTime) impl
        DateTimeMethods
}
impl<const OWNED: bool> DateTimeIsOwned<OWNED> {
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
    pub const WET: c_int = GMT0;
    pub const WEST: c_int = GMT1;
    pub const CET: c_int = GMT1;
    pub const CEST: c_int = GMT2;
    pub const EET: c_int = GMT2;
    pub const EEST: c_int = GMT3;
    pub const MSK: c_int = GMT3;
    pub const MSD: c_int = GMT4;
    pub const AST: c_int = GMT_4;
    pub const ADT: c_int = GMT_3;
    pub const EST: c_int = GMT_5;
    pub const EDT: c_int = GMT_4;
    pub const CST: c_int = GMT_6;
    pub const CDT: c_int = GMT_5;
    pub const MST: c_int = GMT_7;
    pub const MDT: c_int = GMT_6;
    pub const PST: c_int = GMT_8;
    pub const PDT: c_int = GMT_7;
    pub const HST: c_int = GMT_10;
    pub const AKST: c_int = GMT_9;
    pub const AKDT: c_int = GMT_8;
    pub const A_WST: c_int = GMT8;
    pub const A_CST: c_int = GMT13 + 1;
    pub const A_EST: c_int = GMT10;
    pub const A_ESST: c_int = GMT11;
    pub const NZST: c_int = GMT12;
    pub const NZDT: c_int = GMT13;
    pub const UTC: c_int = GMT0;

    //  ENUM: Calendar
    pub const Gregorian: c_int = 0;
    pub const Julian: c_int = 0 + 1;

    //  ENUM: Country
    pub const Country_Unknown: c_int = 0;
    pub const Country_Default: c_int = 0 + 1;
    pub const Country_WesternEurope_Start: c_int = 0 + 2;
    pub const Country_EEC: c_int = Country_WesternEurope_Start;
    pub const France: c_int = Country_WesternEurope_Start + 1;
    pub const Germany: c_int = Country_WesternEurope_Start + 2;
    pub const UK: c_int = Country_WesternEurope_Start + 3;
    pub const Country_WesternEurope_End: c_int = UK;
    pub const Russia: c_int = UK + 1;
    pub const USA: c_int = UK + 2;

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
    pub const Inv_Year: c_int = SHRT_MIN;

    //  ENUM: NameFlags
    pub const Name_Full: c_int = 0x01;
    pub const Name_Abbr: c_int = 0x02;

    //  ENUM: WeekFlags
    pub const Default_First: c_int = 0;
    pub const Monday_First: c_int = 0 + 1;
    pub const Sunday_First: c_int = 0 + 2;

    pub fn new() -> DateTimeIsOwned<OWNED> {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_new()) }
    }
    pub fn new_with_datetime<D: DateTimeMethods>(date: &D) -> DateTimeIsOwned<OWNED> {
        unsafe {
            let date = date.as_ptr();
            DateTimeIsOwned(ffi::wxDateTime_new1(date))
        }
    }
    // NOT_SUPPORTED: fn wxDateTime2()
    // BLOCKED: fn wxDateTime3()
    pub fn new_with_double(jdn: c_double) -> DateTimeIsOwned<OWNED> {
        unsafe { DateTimeIsOwned(ffi::wxDateTime_new4(jdn)) }
    }
    // NOT_SUPPORTED: fn wxDateTime5()
    // NOT_SUPPORTED: fn wxDateTime6()
    // BLOCKED: fn wxDateTime7()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DateTimeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDateTime_delete(self.0) }
        }
    }
}

// wxEvent
wx_class! { Event =
    EventIsOwned<true>(wxEvent) impl
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> EventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for EventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxEvtHandler
wx_class! { EvtHandler =
    EvtHandlerIsOwned<true>(wxEvtHandler) impl
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> EvtHandlerIsOwned<OWNED> {
    pub fn new() -> EvtHandlerIsOwned<OWNED> {
        unsafe { EvtHandlerIsOwned(ffi::wxEvtHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}

// wxObject
wx_class! { Object =
    ObjectIsOwned<true>(wxObject) impl
        ObjectMethods
}
impl<const OWNED: bool> ObjectIsOwned<OWNED> {
    pub fn new() -> ObjectIsOwned<OWNED> {
        unsafe { ObjectIsOwned(ffi::wxObject_new()) }
    }
    pub fn new_with_object<O: ObjectMethods>(other: &O) -> ObjectIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            ObjectIsOwned(ffi::wxObject_new1(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
