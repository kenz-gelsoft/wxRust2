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

// wxAny
wx_class! { Any =
    AnyIsOwned<true>(wxAny) impl
        AnyMethods
}
impl<const OWNED: bool> AnyIsOwned<OWNED> {
    pub fn new() -> AnyIsOwned<OWNED> {
        unsafe { AnyIsOwned(ffi::wxAny_new()) }
    }
    pub fn new_with_t(value: *const c_void) -> AnyIsOwned<OWNED> {
        unsafe { AnyIsOwned(ffi::wxAny_new1(value)) }
    }
    pub fn new_with_any<A: AnyMethods>(any: &A) -> AnyIsOwned<OWNED> {
        unsafe {
            let any = any.as_ptr();
            AnyIsOwned(ffi::wxAny_new2(any))
        }
    }
    pub fn new_with_variant(variant: *const c_void) -> AnyIsOwned<OWNED> {
        unsafe { AnyIsOwned(ffi::wxAny_new3(variant)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for AnyIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAny_delete(self.0) }
        }
    }
}

// wxAnyValueType
wx_class! { AnyValueType =
    AnyValueTypeIsOwned<true>(wxAnyValueType) impl
        AnyValueTypeMethods
}
impl<const OWNED: bool> AnyValueTypeIsOwned<OWNED> {
    pub fn new() -> AnyValueTypeIsOwned<OWNED> {
        unsafe { AnyValueTypeIsOwned(ffi::wxAnyValueType_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for AnyValueTypeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAnyValueType_delete(self.0) }
        }
    }
}

// wxAppTraits
wx_class! { AppTraits =
    AppTraitsIsOwned<true>(wxAppTraits) impl
        AppTraitsMethods
}
impl<const OWNED: bool> AppTraitsIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for AppTraitsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxAppTraits_delete(self.0) }
        }
    }
}

// wxArchiveClassFactory
wx_class! { ArchiveClassFactory =
    ArchiveClassFactoryIsOwned<true>(wxArchiveClassFactory) impl
        ArchiveClassFactoryMethods,
        ObjectMethods
}
impl<const OWNED: bool> ArchiveClassFactoryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ArchiveClassFactoryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ArchiveClassFactoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ArchiveClassFactoryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxArchiveClassFactory_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ArchiveClassFactoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxArchiveEntry
wx_class! { ArchiveEntry =
    ArchiveEntryIsOwned<true>(wxArchiveEntry) impl
        ArchiveEntryMethods,
        ObjectMethods
}
impl<const OWNED: bool> ArchiveEntryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ArchiveEntryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ArchiveEntryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ArchiveEntryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxArchiveEntry_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ArchiveEntryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxArchiveIterator
wx_class! { ArchiveIterator =
    ArchiveIteratorIsOwned<true>(wxArchiveIterator) impl
        ArchiveIteratorMethods
}
impl<const OWNED: bool> ArchiveIteratorIsOwned<OWNED> {
    pub fn new() -> ArchiveIteratorIsOwned<OWNED> {
        unsafe { ArchiveIteratorIsOwned(ffi::wxArchiveIterator_new()) }
    }
    pub fn new_with_arc(arc: *mut c_void) -> ArchiveIteratorIsOwned<OWNED> {
        unsafe { ArchiveIteratorIsOwned(ffi::wxArchiveIterator_new1(arc)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ArchiveIteratorIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxArchiveIterator_delete(self.0) }
        }
    }
}

// wxArchiveNotifier
wx_class! { ArchiveNotifier =
    ArchiveNotifierIsOwned<true>(wxArchiveNotifier) impl
        ArchiveNotifierMethods
}
impl<const OWNED: bool> ArchiveNotifierIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ArchiveNotifierIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxArchiveNotifier_delete(self.0) }
        }
    }
}

// wxClassInfo
wx_class! { ClassInfo =
    ClassInfoIsOwned<true>(wxClassInfo) impl
        ClassInfoMethods
}
impl<const OWNED: bool> ClassInfoIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxClassInfo()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ClassInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClassInfo_delete(self.0) }
        }
    }
}

// wxClient
wx_class! { Client =
    ClientIsOwned<true>(wxClient) impl
        ClientMethods,
        ObjectMethods
}
impl<const OWNED: bool> ClientIsOwned<OWNED> {
    pub fn new() -> ClientIsOwned<OWNED> {
        unsafe { ClientIsOwned(ffi::wxClient_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ClientIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ClientIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ClientIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxClient_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ClientIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxClientData
wx_class! { ClientData =
    ClientDataIsOwned<true>(wxClientData) impl
        ClientDataMethods
}
impl<const OWNED: bool> ClientDataIsOwned<OWNED> {
    pub fn new() -> ClientDataIsOwned<OWNED> {
        unsafe { ClientDataIsOwned(ffi::wxClientData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ClientDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClientData_delete(self.0) }
        }
    }
}

// wxClientDataContainer
wx_class! { ClientDataContainer =
    ClientDataContainerIsOwned<true>(wxClientDataContainer) impl
        ClientDataContainerMethods
}
impl<const OWNED: bool> ClientDataContainerIsOwned<OWNED> {
    pub fn new() -> ClientDataContainerIsOwned<OWNED> {
        unsafe { ClientDataContainerIsOwned(ffi::wxClientDataContainer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ClientDataContainerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxClientDataContainer_delete(self.0) }
        }
    }
}

// wxCmdLineParser
wx_class! { CmdLineParser =
    CmdLineParserIsOwned<true>(wxCmdLineParser) impl
        CmdLineParserMethods
}
impl<const OWNED: bool> CmdLineParserIsOwned<OWNED> {
    pub fn new() -> CmdLineParserIsOwned<OWNED> {
        unsafe { CmdLineParserIsOwned(ffi::wxCmdLineParser_new()) }
    }
    pub fn new_with_int_char(argc: c_int, argv: *mut c_void) -> CmdLineParserIsOwned<OWNED> {
        unsafe { CmdLineParserIsOwned(ffi::wxCmdLineParser_new1(argc, argv)) }
    }
    pub fn new_with_int_wchar_t(argc: c_int, argv: *mut c_void) -> CmdLineParserIsOwned<OWNED> {
        unsafe { CmdLineParserIsOwned(ffi::wxCmdLineParser_new2(argc, argv)) }
    }
    pub fn new_with_str(cmdline: &str) -> CmdLineParserIsOwned<OWNED> {
        unsafe {
            let cmdline = WxString::from(cmdline);
            let cmdline = cmdline.as_ptr();
            CmdLineParserIsOwned(ffi::wxCmdLineParser_new3(cmdline))
        }
    }
    pub fn new_with_cmdlineentrydesc(desc: *const c_void) -> CmdLineParserIsOwned<OWNED> {
        unsafe { CmdLineParserIsOwned(ffi::wxCmdLineParser_new4(desc)) }
    }
    pub fn new_with_cmdlineentrydesc_int(
        desc: *const c_void,
        argc: c_int,
        argv: *mut c_void,
    ) -> CmdLineParserIsOwned<OWNED> {
        unsafe { CmdLineParserIsOwned(ffi::wxCmdLineParser_new5(desc, argc, argv)) }
    }
    pub fn new_with_cmdlineentrydesc_str(
        desc: *const c_void,
        cmdline: &str,
    ) -> CmdLineParserIsOwned<OWNED> {
        unsafe {
            let cmdline = WxString::from(cmdline);
            let cmdline = cmdline.as_ptr();
            CmdLineParserIsOwned(ffi::wxCmdLineParser_new6(desc, cmdline))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for CmdLineParserIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCmdLineParser_delete(self.0) }
        }
    }
}

// wxCondition
wx_class! { Condition =
    ConditionIsOwned<true>(wxCondition) impl
        ConditionMethods
}
impl<const OWNED: bool> ConditionIsOwned<OWNED> {
    pub fn new<M: MutexMethods>(mutex: &M) -> ConditionIsOwned<OWNED> {
        unsafe {
            let mutex = mutex.as_ptr();
            ConditionIsOwned(ffi::wxCondition_new(mutex))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ConditionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCondition_delete(self.0) }
        }
    }
}

// wxConfigPathChanger
wx_class! { ConfigPathChanger =
    ConfigPathChangerIsOwned<true>(wxConfigPathChanger) impl
        ConfigPathChangerMethods
}
impl<const OWNED: bool> ConfigPathChangerIsOwned<OWNED> {
    pub fn new(p_container: *const c_void, str_entry: &str) -> ConfigPathChangerIsOwned<OWNED> {
        unsafe {
            let str_entry = WxString::from(str_entry);
            let str_entry = str_entry.as_ptr();
            ConfigPathChangerIsOwned(ffi::wxConfigPathChanger_new(p_container, str_entry))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ConfigPathChangerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxConfigPathChanger_delete(self.0) }
        }
    }
}

// wxConnection
wx_class! { Connection =
    ConnectionIsOwned<true>(wxConnection) impl
        ConnectionMethods,
        ObjectMethods
}
impl<const OWNED: bool> ConnectionIsOwned<OWNED> {
    pub fn new() -> ConnectionIsOwned<OWNED> {
        unsafe { ConnectionIsOwned(ffi::wxConnection_new()) }
    }
    pub fn new_with_void(buffer: *mut c_void, size: usize) -> ConnectionIsOwned<OWNED> {
        unsafe { ConnectionIsOwned(ffi::wxConnection_new1(buffer, size)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ConnectionIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ConnectionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ConnectionIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxConnection_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ConnectionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxConnectionBase
wx_class! { ConnectionBase =
    ConnectionBaseIsOwned<true>(wxConnectionBase) impl
        ConnectionBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> ConnectionBaseIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ConnectionBaseIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ConnectionBaseIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ConnectionBaseIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxConnectionBase_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ConnectionBaseIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxCriticalSection
wx_class! { CriticalSection =
    CriticalSectionIsOwned<true>(wxCriticalSection) impl
        CriticalSectionMethods
}
impl<const OWNED: bool> CriticalSectionIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxCriticalSection()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for CriticalSectionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCriticalSection_delete(self.0) }
        }
    }
}

// wxCriticalSectionLocker
wx_class! { CriticalSectionLocker =
    CriticalSectionLockerIsOwned<true>(wxCriticalSectionLocker) impl
        CriticalSectionLockerMethods
}
impl<const OWNED: bool> CriticalSectionLockerIsOwned<OWNED> {
    pub fn new<C: CriticalSectionMethods>(
        criticalsection: &C,
    ) -> CriticalSectionLockerIsOwned<OWNED> {
        unsafe {
            let criticalsection = criticalsection.as_ptr();
            CriticalSectionLockerIsOwned(ffi::wxCriticalSectionLocker_new(criticalsection))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for CriticalSectionLockerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxCriticalSectionLocker_delete(self.0) }
        }
    }
}

// wxDDEClient
wx_class! { DDEClient =
    DDEClientIsOwned<true>(wxDDEClient) impl
        DDEClientMethods,
        ObjectMethods
}
impl<const OWNED: bool> DDEClientIsOwned<OWNED> {
    pub fn new() -> DDEClientIsOwned<OWNED> {
        unsafe { DDEClientIsOwned(ffi::wxDDEClient_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DDEClientIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DDEClientIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DDEClientIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDDEClient_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DDEClientIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDDEConnection
wx_class! { DDEConnection =
    DDEConnectionIsOwned<true>(wxDDEConnection) impl
        DDEConnectionMethods,
        ConnectionBaseMethods,
        ObjectMethods
}
impl<const OWNED: bool> DDEConnectionIsOwned<OWNED> {
    pub fn new() -> DDEConnectionIsOwned<OWNED> {
        unsafe { DDEConnectionIsOwned(ffi::wxDDEConnection_new()) }
    }
    pub fn new_with_void(buffer: *mut c_void, size: usize) -> DDEConnectionIsOwned<OWNED> {
        unsafe { DDEConnectionIsOwned(ffi::wxDDEConnection_new1(buffer, size)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<DDEConnectionIsOwned<OWNED>> for ConnectionBaseIsOwned<OWNED> {
    fn from(o: DDEConnectionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<DDEConnectionIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: DDEConnectionIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for DDEConnectionIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxDDEConnection_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for DDEConnectionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxDDEServer
wx_class! { DDEServer =
    DDEServerIsOwned<true>(wxDDEServer) impl
        DDEServerMethods
}
impl<const OWNED: bool> DDEServerIsOwned<OWNED> {
    pub fn new() -> DDEServerIsOwned<OWNED> {
        unsafe { DDEServerIsOwned(ffi::wxDDEServer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DDEServerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDDEServer_delete(self.0) }
        }
    }
}

// wxDataInputStream
wx_class! { DataInputStream =
    DataInputStreamIsOwned<true>(wxDataInputStream) impl
        DataInputStreamMethods
}
impl<const OWNED: bool> DataInputStreamIsOwned<OWNED> {
    pub fn new(stream: *mut c_void, conv: *const c_void) -> DataInputStreamIsOwned<OWNED> {
        unsafe { DataInputStreamIsOwned(ffi::wxDataInputStream_new(stream, conv)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataInputStreamIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataInputStream_delete(self.0) }
        }
    }
}

// wxDataOutputStream
wx_class! { DataOutputStream =
    DataOutputStreamIsOwned<true>(wxDataOutputStream) impl
        DataOutputStreamMethods
}
impl<const OWNED: bool> DataOutputStreamIsOwned<OWNED> {
    pub fn new(stream: *mut c_void, conv: *const c_void) -> DataOutputStreamIsOwned<OWNED> {
        unsafe { DataOutputStreamIsOwned(ffi::wxDataOutputStream_new(stream, conv)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DataOutputStreamIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDataOutputStream_delete(self.0) }
        }
    }
}

// wxDateSpan
wx_class! { DateSpan =
    DateSpanIsOwned<true>(wxDateSpan) impl
        DateSpanMethods
}
impl<const OWNED: bool> DateSpanIsOwned<OWNED> {
    pub fn new(years: c_int, months: c_int, weeks: c_int, days: c_int) -> DateSpanIsOwned<OWNED> {
        unsafe { DateSpanIsOwned(ffi::wxDateSpan_new(years, months, weeks, days)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DateSpanIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDateSpan_delete(self.0) }
        }
    }
}

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

// wxDateTimeHolidayAuthority
wx_class! { DateTimeHolidayAuthority =
    DateTimeHolidayAuthorityIsOwned<true>(wxDateTimeHolidayAuthority) impl
        DateTimeHolidayAuthorityMethods
}
impl<const OWNED: bool> DateTimeHolidayAuthorityIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DateTimeHolidayAuthorityIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDateTimeHolidayAuthority_delete(self.0) }
        }
    }
}

// wxDateTimeWorkDays
wx_class! { DateTimeWorkDays =
    DateTimeWorkDaysIsOwned<true>(wxDateTimeWorkDays) impl
        DateTimeWorkDaysMethods
}
impl<const OWNED: bool> DateTimeWorkDaysIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DateTimeWorkDaysIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDateTimeWorkDays_delete(self.0) }
        }
    }
}

// wxDebugContext
wx_class! { DebugContext =
    DebugContextIsOwned<true>(wxDebugContext) impl
        DebugContextMethods
}
impl<const OWNED: bool> DebugContextIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DebugContextIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDebugContext_delete(self.0) }
        }
    }
}

// wxDir
wx_class! { Dir =
    DirIsOwned<true>(wxDir) impl
        DirMethods
}
impl<const OWNED: bool> DirIsOwned<OWNED> {
    pub fn new() -> DirIsOwned<OWNED> {
        unsafe { DirIsOwned(ffi::wxDir_new()) }
    }
    pub fn new_with_str(dir: &str) -> DirIsOwned<OWNED> {
        unsafe {
            let dir = WxString::from(dir);
            let dir = dir.as_ptr();
            DirIsOwned(ffi::wxDir_new1(dir))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DirIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDir_delete(self.0) }
        }
    }
}

// wxDirTraverser
wx_class! { DirTraverser =
    DirTraverserIsOwned<true>(wxDirTraverser) impl
        DirTraverserMethods
}
impl<const OWNED: bool> DirTraverserIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DirTraverserIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDirTraverser_delete(self.0) }
        }
    }
}

// wxDynamicLibrary
wx_class! { DynamicLibrary =
    DynamicLibraryIsOwned<true>(wxDynamicLibrary) impl
        DynamicLibraryMethods
}
impl<const OWNED: bool> DynamicLibraryIsOwned<OWNED> {
    pub fn new() -> DynamicLibraryIsOwned<OWNED> {
        unsafe { DynamicLibraryIsOwned(ffi::wxDynamicLibrary_new()) }
    }
    pub fn new_with_str(name: &str, flags: c_int) -> DynamicLibraryIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            DynamicLibraryIsOwned(ffi::wxDynamicLibrary_new1(name, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DynamicLibraryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDynamicLibrary_delete(self.0) }
        }
    }
}

// wxDynamicLibraryDetails
wx_class! { DynamicLibraryDetails =
    DynamicLibraryDetailsIsOwned<true>(wxDynamicLibraryDetails) impl
        DynamicLibraryDetailsMethods
}
impl<const OWNED: bool> DynamicLibraryDetailsIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for DynamicLibraryDetailsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxDynamicLibraryDetails_delete(self.0) }
        }
    }
}

// wxEncodingConverter
wx_class! { EncodingConverter =
    EncodingConverterIsOwned<true>(wxEncodingConverter) impl
        EncodingConverterMethods,
        ObjectMethods
}
impl<const OWNED: bool> EncodingConverterIsOwned<OWNED> {
    pub fn new() -> EncodingConverterIsOwned<OWNED> {
        unsafe { EncodingConverterIsOwned(ffi::wxEncodingConverter_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<EncodingConverterIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EncodingConverterIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EncodingConverterIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEncodingConverter_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for EncodingConverterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
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
impl<const OWNED: bool> From<EventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEvent_CLASSINFO()) }
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
impl<const OWNED: bool> From<EvtHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: EvtHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for EvtHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxEvtHandler_CLASSINFO()) }
    }
}
// Mix-in(s) to wxEvtHandler
impl<const OWNED: bool> TrackableMethods for EvtHandlerIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxEvtHandler_AsTrackable(self.as_ptr()) }
    }
}

// wxFFile
wx_class! { FFile =
    FFileIsOwned<true>(wxFFile) impl
        FFileMethods
}
impl<const OWNED: bool> FFileIsOwned<OWNED> {
    pub fn new() -> FFileIsOwned<OWNED> {
        unsafe { FFileIsOwned(ffi::wxFFile_new()) }
    }
    pub fn new_with_file(fp: *mut c_void) -> FFileIsOwned<OWNED> {
        unsafe { FFileIsOwned(ffi::wxFFile_new1(fp)) }
    }
    pub fn new_with_str(filename: &str, mode: &str) -> FFileIsOwned<OWNED> {
        unsafe {
            let filename = WxString::from(filename);
            let filename = filename.as_ptr();
            let mode = WxString::from(mode);
            let mode = mode.as_ptr();
            FFileIsOwned(ffi::wxFFile_new2(filename, mode))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FFileIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFFile_delete(self.0) }
        }
    }
}

// wxFSFile
wx_class! { FSFile =
    FSFileIsOwned<true>(wxFSFile) impl
        FSFileMethods,
        ObjectMethods
}
impl<const OWNED: bool> FSFileIsOwned<OWNED> {
    pub fn new(
        stream: *mut c_void,
        location: &str,
        mimetype: &str,
        anchor: &str,
        modif: ffi::wxDateTime,
    ) -> FSFileIsOwned<OWNED> {
        unsafe {
            let location = WxString::from(location);
            let location = location.as_ptr();
            let mimetype = WxString::from(mimetype);
            let mimetype = mimetype.as_ptr();
            let anchor = WxString::from(anchor);
            let anchor = anchor.as_ptr();
            FSFileIsOwned(ffi::wxFSFile_new(stream, location, mimetype, anchor, modif))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FSFileIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FSFileIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FSFileIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFSFile_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FSFileIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFSVolume
wx_class! { FSVolume =
    FSVolumeIsOwned<true>(wxFSVolume) impl
        FSVolumeMethods
}
impl<const OWNED: bool> FSVolumeIsOwned<OWNED> {
    pub fn new() -> FSVolumeIsOwned<OWNED> {
        unsafe { FSVolumeIsOwned(ffi::wxFSVolume_new()) }
    }
    pub fn new_with_str(name: &str) -> FSVolumeIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            FSVolumeIsOwned(ffi::wxFSVolume_new1(name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FSVolumeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFSVolume_delete(self.0) }
        }
    }
}

// wxFile
wx_class! { File =
    FileIsOwned<true>(wxFile) impl
        FileMethods
}
impl<const OWNED: bool> FileIsOwned<OWNED> {
    //  ENUM: OpenMode
    pub const read: c_int = 0;
    pub const write: c_int = 0 + 1;
    pub const read_write: c_int = 0 + 2;
    pub const write_append: c_int = 0 + 3;
    pub const write_excl: c_int = 0 + 4;

    //  ENUM: @17
    pub const fd_invalid: c_int = -1;
    pub const fd_stdin: c_int = -1 + 1;
    pub const fd_stdout: c_int = -1 + 2;
    pub const fd_stderr: c_int = -1 + 3;

    pub fn new() -> FileIsOwned<OWNED> {
        unsafe { FileIsOwned(ffi::wxFile_new()) }
    }
    // NOT_SUPPORTED: fn wxFile1()
    pub fn new_with_int(fd: c_int) -> FileIsOwned<OWNED> {
        unsafe { FileIsOwned(ffi::wxFile_new2(fd)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FileIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFile_delete(self.0) }
        }
    }
}

// wxFileName
wx_class! { FileName =
    FileNameIsOwned<true>(wxFileName) impl
        FileNameMethods
}
impl<const OWNED: bool> FileNameIsOwned<OWNED> {
    pub fn new() -> FileNameIsOwned<OWNED> {
        unsafe { FileNameIsOwned(ffi::wxFileName_new()) }
    }
    pub fn new_with_filename<F: FileNameMethods>(filename: &F) -> FileNameIsOwned<OWNED> {
        unsafe {
            let filename = filename.as_ptr();
            FileNameIsOwned(ffi::wxFileName_new1(filename))
        }
    }
    // NOT_SUPPORTED: fn wxFileName2()
    // NOT_SUPPORTED: fn wxFileName3()
    // NOT_SUPPORTED: fn wxFileName4()
    // NOT_SUPPORTED: fn wxFileName5()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FileNameIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileName_delete(self.0) }
        }
    }
}

// wxFileSystem
wx_class! { FileSystem =
    FileSystemIsOwned<true>(wxFileSystem) impl
        FileSystemMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileSystemIsOwned<OWNED> {
    pub fn new() -> FileSystemIsOwned<OWNED> {
        unsafe { FileSystemIsOwned(ffi::wxFileSystem_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileSystemIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileSystemIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileSystemIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileSystem_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileSystemIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileSystemHandler
wx_class! { FileSystemHandler =
    FileSystemHandlerIsOwned<true>(wxFileSystemHandler) impl
        FileSystemHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileSystemHandlerIsOwned<OWNED> {
    pub fn new() -> FileSystemHandlerIsOwned<OWNED> {
        unsafe { FileSystemHandlerIsOwned(ffi::wxFileSystemHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileSystemHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileSystemHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileSystemHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileSystemHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileSystemHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileSystemWatcher
wx_class! { FileSystemWatcher =
    FileSystemWatcherIsOwned<true>(wxFileSystemWatcher) impl
        FileSystemWatcherMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileSystemWatcherIsOwned<OWNED> {
    pub fn new() -> FileSystemWatcherIsOwned<OWNED> {
        unsafe { FileSystemWatcherIsOwned(ffi::wxFileSystemWatcher_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileSystemWatcherIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: FileSystemWatcherIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileSystemWatcherIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileSystemWatcherIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileSystemWatcherIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileSystemWatcher_CLASSINFO()) }
    }
}
// Mix-in(s) to wxFileSystemWatcher
impl<const OWNED: bool> TrackableMethods for FileSystemWatcherIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxFileSystemWatcher_AsTrackable(self.as_ptr()) }
    }
}

// wxFileSystemWatcherEvent
wx_class! { FileSystemWatcherEvent =
    FileSystemWatcherEventIsOwned<true>(wxFileSystemWatcherEvent) impl
        FileSystemWatcherEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> FileSystemWatcherEventIsOwned<OWNED> {
    pub fn new_with_int(
        change_type: c_int,
        watchid: c_int,
    ) -> FileSystemWatcherEventIsOwned<OWNED> {
        unsafe {
            FileSystemWatcherEventIsOwned(ffi::wxFileSystemWatcherEvent_new(change_type, watchid))
        }
    }
    // NOT_SUPPORTED: fn wxFileSystemWatcherEvent1()
    pub fn new_with_filename<F: FileNameMethods, F2: FileNameMethods>(
        change_type: c_int,
        path: &F,
        new_path: &F2,
        watchid: c_int,
    ) -> FileSystemWatcherEventIsOwned<OWNED> {
        unsafe {
            let path = path.as_ptr();
            let new_path = new_path.as_ptr();
            FileSystemWatcherEventIsOwned(ffi::wxFileSystemWatcherEvent_new2(
                change_type,
                path,
                new_path,
                watchid,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FileSystemWatcherEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: FileSystemWatcherEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<FileSystemWatcherEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FileSystemWatcherEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FileSystemWatcherEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFileSystemWatcherEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FileSystemWatcherEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxFileType
wx_class! { FileType =
    FileTypeIsOwned<true>(wxFileType) impl
        FileTypeMethods
}
impl<const OWNED: bool> FileTypeIsOwned<OWNED> {
    pub fn new<F: FileTypeInfoMethods>(ft_info: &F) -> FileTypeIsOwned<OWNED> {
        unsafe {
            let ft_info = ft_info.as_ptr();
            FileTypeIsOwned(ffi::wxFileType_new(ft_info))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for FileTypeIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxFileType_delete(self.0) }
        }
    }
}

// wxFilterClassFactory
wx_class! { FilterClassFactory =
    FilterClassFactoryIsOwned<true>(wxFilterClassFactory) impl
        FilterClassFactoryMethods,
        ObjectMethods
}
impl<const OWNED: bool> FilterClassFactoryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<FilterClassFactoryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: FilterClassFactoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for FilterClassFactoryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxFilterClassFactory_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for FilterClassFactoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHashMap
wx_class! { HashMap =
    HashMapIsOwned<true>(wxHashMap) impl
        HashMapMethods
}
impl<const OWNED: bool> HashMapIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxHashMap()
    pub fn new<H: HashMapMethods>(map: &H) -> HashMapIsOwned<OWNED> {
        unsafe {
            let map = map.as_ptr();
            HashMapIsOwned(ffi::wxHashMap_new1(map))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for HashMapIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHashMap_delete(self.0) }
        }
    }
}

// wxHashSet
wx_class! { HashSet =
    HashSetIsOwned<true>(wxHashSet) impl
        HashSetMethods
}
impl<const OWNED: bool> HashSetIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxHashSet()
    pub fn new<H: HashSetMethods>(set: &H) -> HashSetIsOwned<OWNED> {
        unsafe {
            let set = set.as_ptr();
            HashSetIsOwned(ffi::wxHashSet_new1(set))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for HashSetIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHashSet_delete(self.0) }
        }
    }
}

// wxHashTable
wx_class! { HashTable =
    HashTableIsOwned<true>(wxHashTable) impl
        HashTableMethods,
        ObjectMethods
}
impl<const OWNED: bool> HashTableIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxHashTable()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<HashTableIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HashTableIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HashTableIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHashTable_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HashTableIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxIconLocation
wx_class! { IconLocation =
    IconLocationIsOwned<true>(wxIconLocation) impl
        IconLocationMethods
}
impl<const OWNED: bool> IconLocationIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for IconLocationIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxIconLocation_delete(self.0) }
        }
    }
}

// wxIdleEvent
wx_class! { IdleEvent =
    IdleEventIsOwned<true>(wxIdleEvent) impl
        IdleEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> IdleEventIsOwned<OWNED> {
    pub fn new() -> IdleEventIsOwned<OWNED> {
        unsafe { IdleEventIsOwned(ffi::wxIdleEvent_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<IdleEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: IdleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<IdleEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: IdleEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for IdleEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxIdleEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for IdleEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxInitializer
wx_class! { Initializer =
    InitializerIsOwned<true>(wxInitializer) impl
        InitializerMethods
}
impl<const OWNED: bool> InitializerIsOwned<OWNED> {
    pub fn new(argc: c_int, argv: *mut c_void) -> InitializerIsOwned<OWNED> {
        unsafe { InitializerIsOwned(ffi::wxInitializer_new(argc, argv)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for InitializerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxInitializer_delete(self.0) }
        }
    }
}

// wxLocale
wx_class! { Locale =
    LocaleIsOwned<true>(wxLocale) impl
        LocaleMethods
}
impl<const OWNED: bool> LocaleIsOwned<OWNED> {
    pub fn new() -> LocaleIsOwned<OWNED> {
        unsafe { LocaleIsOwned(ffi::wxLocale_new()) }
    }
    pub fn new_with_int(language: c_int, flags: c_int) -> LocaleIsOwned<OWNED> {
        unsafe { LocaleIsOwned(ffi::wxLocale_new1(language, flags)) }
    }
    pub fn new_with_str(
        name: &str,
        short_name: &str,
        locale: &str,
        b_load_default: bool,
    ) -> LocaleIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let short_name = WxString::from(short_name);
            let short_name = short_name.as_ptr();
            let locale = WxString::from(locale);
            let locale = locale.as_ptr();
            LocaleIsOwned(ffi::wxLocale_new2(name, short_name, locale, b_load_default))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for LocaleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLocale_delete(self.0) }
        }
    }
}

// wxLog
wx_class! { Log =
    LogIsOwned<true>(wxLog) impl
        LogMethods
}
impl<const OWNED: bool> LogIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for LogIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLog_delete(self.0) }
        }
    }
}

// wxLogBuffer
wx_class! { LogBuffer =
    LogBufferIsOwned<true>(wxLogBuffer) impl
        LogBufferMethods,
        LogMethods
}
impl<const OWNED: bool> LogBufferIsOwned<OWNED> {
    pub fn new() -> LogBufferIsOwned<OWNED> {
        unsafe { LogBufferIsOwned(ffi::wxLogBuffer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogBufferIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogBufferIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogBufferIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogBuffer_delete(self.0) }
        }
    }
}

// wxLogChain
wx_class! { LogChain =
    LogChainIsOwned<true>(wxLogChain) impl
        LogChainMethods,
        LogMethods
}
impl<const OWNED: bool> LogChainIsOwned<OWNED> {
    pub fn new<L: LogMethods>(logger: Option<&L>) -> LogChainIsOwned<OWNED> {
        unsafe {
            let logger = match logger {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            LogChainIsOwned(ffi::wxLogChain_new(logger))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogChainIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogChainIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogChainIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogChain_delete(self.0) }
        }
    }
}

// wxLogFormatter
wx_class! { LogFormatter =
    LogFormatterIsOwned<true>(wxLogFormatter) impl
        LogFormatterMethods
}
impl<const OWNED: bool> LogFormatterIsOwned<OWNED> {
    pub fn new() -> LogFormatterIsOwned<OWNED> {
        unsafe { LogFormatterIsOwned(ffi::wxLogFormatter_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for LogFormatterIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogFormatter_delete(self.0) }
        }
    }
}

// wxLogInterposer
wx_class! { LogInterposer =
    LogInterposerIsOwned<true>(wxLogInterposer) impl
        LogInterposerMethods,
        LogChainMethods,
        LogMethods
}
impl<const OWNED: bool> LogInterposerIsOwned<OWNED> {
    pub fn new() -> LogInterposerIsOwned<OWNED> {
        unsafe { LogInterposerIsOwned(ffi::wxLogInterposer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogInterposerIsOwned<OWNED>> for LogChainIsOwned<OWNED> {
    fn from(o: LogInterposerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LogInterposerIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogInterposerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogInterposerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogInterposer_delete(self.0) }
        }
    }
}

// wxLogInterposerTemp
wx_class! { LogInterposerTemp =
    LogInterposerTempIsOwned<true>(wxLogInterposerTemp) impl
        LogInterposerTempMethods,
        LogChainMethods,
        LogMethods
}
impl<const OWNED: bool> LogInterposerTempIsOwned<OWNED> {
    pub fn new() -> LogInterposerTempIsOwned<OWNED> {
        unsafe { LogInterposerTempIsOwned(ffi::wxLogInterposerTemp_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogInterposerTempIsOwned<OWNED>> for LogChainIsOwned<OWNED> {
    fn from(o: LogInterposerTempIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<LogInterposerTempIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogInterposerTempIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogInterposerTempIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogInterposerTemp_delete(self.0) }
        }
    }
}

// wxLogNull
wx_class! { LogNull =
    LogNullIsOwned<true>(wxLogNull) impl
        LogNullMethods
}
impl<const OWNED: bool> LogNullIsOwned<OWNED> {
    pub fn new() -> LogNullIsOwned<OWNED> {
        unsafe { LogNullIsOwned(ffi::wxLogNull_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for LogNullIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogNull_delete(self.0) }
        }
    }
}

// wxLogStderr
wx_class! { LogStderr =
    LogStderrIsOwned<true>(wxLogStderr) impl
        LogStderrMethods,
        LogMethods
}
impl<const OWNED: bool> LogStderrIsOwned<OWNED> {
    pub fn new(fp: *mut c_void, conv: *const c_void) -> LogStderrIsOwned<OWNED> {
        unsafe { LogStderrIsOwned(ffi::wxLogStderr_new(fp, conv)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogStderrIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogStderrIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogStderrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogStderr_delete(self.0) }
        }
    }
}

// wxLogStream
wx_class! { LogStream =
    LogStreamIsOwned<true>(wxLogStream) impl
        LogStreamMethods,
        LogMethods
}
impl<const OWNED: bool> LogStreamIsOwned<OWNED> {
    pub fn new(ostr: *mut c_void, conv: *const c_void) -> LogStreamIsOwned<OWNED> {
        unsafe { LogStreamIsOwned(ffi::wxLogStream_new(ostr, conv)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<LogStreamIsOwned<OWNED>> for LogIsOwned<OWNED> {
    fn from(o: LogStreamIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for LogStreamIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxLogStream_delete(self.0) }
        }
    }
}

// wxMemoryBuffer
wx_class! { MemoryBuffer =
    MemoryBufferIsOwned<true>(wxMemoryBuffer) impl
        MemoryBufferMethods
}
impl<const OWNED: bool> MemoryBufferIsOwned<OWNED> {
    pub fn new_with_memorybuffer<M: MemoryBufferMethods>(src: &M) -> MemoryBufferIsOwned<OWNED> {
        unsafe {
            let src = src.as_ptr();
            MemoryBufferIsOwned(ffi::wxMemoryBuffer_new(src))
        }
    }
    pub fn new_with_sz(size: usize) -> MemoryBufferIsOwned<OWNED> {
        unsafe { MemoryBufferIsOwned(ffi::wxMemoryBuffer_new1(size)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for MemoryBufferIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMemoryBuffer_delete(self.0) }
        }
    }
}

// wxMemoryFSHandler
wx_class! { MemoryFSHandler =
    MemoryFSHandlerIsOwned<true>(wxMemoryFSHandler) impl
        MemoryFSHandlerMethods,
        FileSystemHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MemoryFSHandlerIsOwned<OWNED> {
    pub fn new() -> MemoryFSHandlerIsOwned<OWNED> {
        unsafe { MemoryFSHandlerIsOwned(ffi::wxMemoryFSHandler_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MemoryFSHandlerIsOwned<OWNED>> for FileSystemHandlerIsOwned<OWNED> {
    fn from(o: MemoryFSHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MemoryFSHandlerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MemoryFSHandlerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MemoryFSHandlerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMemoryFSHandler_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for MemoryFSHandlerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMessageOutput
wx_class! { MessageOutput =
    MessageOutputIsOwned<true>(wxMessageOutput) impl
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for MessageOutputIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutput_delete(self.0) }
        }
    }
}

// wxMessageOutputBest
wx_class! { MessageOutputBest =
    MessageOutputBestIsOwned<true>(wxMessageOutputBest) impl
        MessageOutputBestMethods,
        MessageOutputStderrMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputBestIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxMessageOutputBest()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MessageOutputBestIsOwned<OWNED>>
    for MessageOutputStderrIsOwned<OWNED>
{
    fn from(o: MessageOutputBestIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageOutputBestIsOwned<OWNED>> for MessageOutputIsOwned<OWNED> {
    fn from(o: MessageOutputBestIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for MessageOutputBestIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutputBest_delete(self.0) }
        }
    }
}

// wxMessageOutputDebug
wx_class! { MessageOutputDebug =
    MessageOutputDebugIsOwned<true>(wxMessageOutputDebug) impl
        MessageOutputDebugMethods,
        MessageOutputStderrMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputDebugIsOwned<OWNED> {
    pub fn new() -> MessageOutputDebugIsOwned<OWNED> {
        unsafe { MessageOutputDebugIsOwned(ffi::wxMessageOutputDebug_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MessageOutputDebugIsOwned<OWNED>>
    for MessageOutputStderrIsOwned<OWNED>
{
    fn from(o: MessageOutputDebugIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MessageOutputDebugIsOwned<OWNED>> for MessageOutputIsOwned<OWNED> {
    fn from(o: MessageOutputDebugIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for MessageOutputDebugIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutputDebug_delete(self.0) }
        }
    }
}

// wxMessageOutputStderr
wx_class! { MessageOutputStderr =
    MessageOutputStderrIsOwned<true>(wxMessageOutputStderr) impl
        MessageOutputStderrMethods,
        MessageOutputMethods
}
impl<const OWNED: bool> MessageOutputStderrIsOwned<OWNED> {
    pub fn new(fp: *mut c_void) -> MessageOutputStderrIsOwned<OWNED> {
        unsafe { MessageOutputStderrIsOwned(ffi::wxMessageOutputStderr_new(fp)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MessageOutputStderrIsOwned<OWNED>> for MessageOutputIsOwned<OWNED> {
    fn from(o: MessageOutputStderrIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for MessageOutputStderrIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMessageOutputStderr_delete(self.0) }
        }
    }
}

// wxMimeTypesManager
wx_class! { MimeTypesManager =
    MimeTypesManagerIsOwned<true>(wxMimeTypesManager) impl
        MimeTypesManagerMethods
}
impl<const OWNED: bool> MimeTypesManagerIsOwned<OWNED> {
    pub fn new() -> MimeTypesManagerIsOwned<OWNED> {
        unsafe { MimeTypesManagerIsOwned(ffi::wxMimeTypesManager_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for MimeTypesManagerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMimeTypesManager_delete(self.0) }
        }
    }
}

// wxModule
wx_class! { Module =
    ModuleIsOwned<true>(wxModule) impl
        ModuleMethods,
        ObjectMethods
}
impl<const OWNED: bool> ModuleIsOwned<OWNED> {
    pub fn new() -> ModuleIsOwned<OWNED> {
        unsafe { ModuleIsOwned(ffi::wxModule_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ModuleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ModuleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ModuleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxModule_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ModuleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxMultiChoiceDialog
wx_class! { MultiChoiceDialog =
    MultiChoiceDialogIsOwned<true>(wxMultiChoiceDialog) impl
        MultiChoiceDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> MultiChoiceDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        n: c_int,
        choices: *const c_void,
        style: c_long,
        pos: &P,
    ) -> MultiChoiceDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            MultiChoiceDialogIsOwned(ffi::wxMultiChoiceDialog_new(
                parent, message, caption, n, choices, style, pos,
            ))
        }
    }
    pub fn new<W: WindowMethods, A: ArrayStringMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        choices: &A,
        style: c_long,
        pos: &P,
    ) -> MultiChoiceDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let choices = choices.as_ptr();
            let pos = pos.as_ptr();
            MultiChoiceDialogIsOwned(ffi::wxMultiChoiceDialog_new1(
                parent, message, caption, choices, style, pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<MultiChoiceDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: MultiChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MultiChoiceDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: MultiChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MultiChoiceDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: MultiChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MultiChoiceDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: MultiChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MultiChoiceDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: MultiChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<MultiChoiceDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: MultiChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for MultiChoiceDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxMultiChoiceDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxMultiChoiceDialog
impl<const OWNED: bool> TrackableMethods for MultiChoiceDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxMultiChoiceDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxMutex
wx_class! { Mutex =
    MutexIsOwned<true>(wxMutex) impl
        MutexMethods
}
impl<const OWNED: bool> MutexIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxMutex()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for MutexIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMutex_delete(self.0) }
        }
    }
}

// wxMutexLocker
wx_class! { MutexLocker =
    MutexLockerIsOwned<true>(wxMutexLocker) impl
        MutexLockerMethods
}
impl<const OWNED: bool> MutexLockerIsOwned<OWNED> {
    pub fn new<M: MutexMethods>(mutex: &M) -> MutexLockerIsOwned<OWNED> {
        unsafe {
            let mutex = mutex.as_ptr();
            MutexLockerIsOwned(ffi::wxMutexLocker_new(mutex))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for MutexLockerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxMutexLocker_delete(self.0) }
        }
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
impl<const OWNED: bool> DynamicCast for ObjectIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxObject_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxObjectRefData
wx_class! { ObjectRefData =
    ObjectRefDataIsOwned<true>(wxObjectRefData) impl
        ObjectRefDataMethods
}
impl<const OWNED: bool> ObjectRefDataIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ObjectRefDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObjectRefData_delete(self.0) }
        }
    }
}

// wxPlatformInfo
wx_class! { PlatformInfo =
    PlatformInfoIsOwned<true>(wxPlatformInfo) impl
        PlatformInfoMethods
}
impl<const OWNED: bool> PlatformInfoIsOwned<OWNED> {
    pub fn new() -> PlatformInfoIsOwned<OWNED> {
        unsafe { PlatformInfoIsOwned(ffi::wxPlatformInfo_new()) }
    }
    // NOT_SUPPORTED: fn wxPlatformInfo1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PlatformInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPlatformInfo_delete(self.0) }
        }
    }
}

// wxPosition
wx_class! { Position =
    PositionIsOwned<true>(wxPosition) impl
        PositionMethods
}
impl<const OWNED: bool> PositionIsOwned<OWNED> {
    pub fn new() -> PositionIsOwned<OWNED> {
        unsafe { PositionIsOwned(ffi::wxPosition_new()) }
    }
    pub fn new_with_int(row: c_int, col: c_int) -> PositionIsOwned<OWNED> {
        unsafe { PositionIsOwned(ffi::wxPosition_new1(row, col)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PositionIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPosition_delete(self.0) }
        }
    }
}

// wxPostScriptDC
wx_class! { PostScriptDC =
    PostScriptDCIsOwned<true>(wxPostScriptDC) impl
        PostScriptDCMethods,
        DCMethods,
        ObjectMethods
}
impl<const OWNED: bool> PostScriptDCIsOwned<OWNED> {
    pub fn new() -> PostScriptDCIsOwned<OWNED> {
        unsafe { PostScriptDCIsOwned(ffi::wxPostScriptDC_new()) }
    }
    pub fn new_with_printdata<P: PrintDataMethods>(print_data: &P) -> PostScriptDCIsOwned<OWNED> {
        unsafe {
            let print_data = print_data.as_ptr();
            PostScriptDCIsOwned(ffi::wxPostScriptDC_new1(print_data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PostScriptDCIsOwned<OWNED>> for DCIsOwned<OWNED> {
    fn from(o: PostScriptDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PostScriptDCIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PostScriptDCIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PostScriptDCIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPostScriptDC_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PostScriptDCIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPowerEvent
wx_class! { PowerEvent =
    PowerEventIsOwned<true>(wxPowerEvent) impl
        PowerEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> PowerEventIsOwned<OWNED> {
    pub fn new() -> PowerEventIsOwned<OWNED> {
        unsafe { PowerEventIsOwned(ffi::wxPowerEvent_new()) }
    }
    // NOT_SUPPORTED: fn wxPowerEvent1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<PowerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: PowerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<PowerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: PowerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for PowerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxPowerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for PowerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxPowerResource
wx_class! { PowerResource =
    PowerResourceIsOwned<true>(wxPowerResource) impl
        PowerResourceMethods
}
impl<const OWNED: bool> PowerResourceIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PowerResourceIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPowerResource_delete(self.0) }
        }
    }
}

// wxPowerResourceBlocker
wx_class! { PowerResourceBlocker =
    PowerResourceBlockerIsOwned<true>(wxPowerResourceBlocker) impl
        PowerResourceBlockerMethods
}
impl<const OWNED: bool> PowerResourceBlockerIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxPowerResourceBlocker()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for PowerResourceBlockerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxPowerResourceBlocker_delete(self.0) }
        }
    }
}

// wxProcess
wx_class! { Process =
    ProcessIsOwned<true>(wxProcess) impl
        ProcessMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> ProcessIsOwned<OWNED> {
    pub fn new_with_evthandler<E: EvtHandlerMethods>(
        parent: Option<&E>,
        id: c_int,
    ) -> ProcessIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            ProcessIsOwned(ffi::wxProcess_new(parent, id))
        }
    }
    pub fn new_with_int(flags: c_int) -> ProcessIsOwned<OWNED> {
        unsafe { ProcessIsOwned(ffi::wxProcess_new1(flags)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ProcessIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: ProcessIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ProcessIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ProcessIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ProcessIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxProcess_CLASSINFO()) }
    }
}
// Mix-in(s) to wxProcess
impl<const OWNED: bool> TrackableMethods for ProcessIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxProcess_AsTrackable(self.as_ptr()) }
    }
}

// wxProcessEvent
wx_class! { ProcessEvent =
    ProcessEventIsOwned<true>(wxProcessEvent) impl
        ProcessEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> ProcessEventIsOwned<OWNED> {
    pub fn new(id: c_int, pid: c_int, exitcode: c_int) -> ProcessEventIsOwned<OWNED> {
        unsafe { ProcessEventIsOwned(ffi::wxProcessEvent_new(id, pid, exitcode)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ProcessEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: ProcessEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ProcessEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ProcessEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ProcessEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxProcessEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ProcessEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxRecursionGuard
wx_class! { RecursionGuard =
    RecursionGuardIsOwned<true>(wxRecursionGuard) impl
        RecursionGuardMethods
}
impl<const OWNED: bool> RecursionGuardIsOwned<OWNED> {
    pub fn new<R: RecursionGuardFlagMethods>(flag: &R) -> RecursionGuardIsOwned<OWNED> {
        unsafe {
            let flag = flag.as_ptr();
            RecursionGuardIsOwned(ffi::wxRecursionGuard_new(flag))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RecursionGuardIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRecursionGuard_delete(self.0) }
        }
    }
}

// wxRecursionGuardFlag
wx_class! { RecursionGuardFlag =
    RecursionGuardFlagIsOwned<true>(wxRecursionGuardFlag) impl
        RecursionGuardFlagMethods
}
impl<const OWNED: bool> RecursionGuardFlagIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RecursionGuardFlagIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRecursionGuardFlag_delete(self.0) }
        }
    }
}

// wxRefCounter

// wxRegEx
wx_class! { RegEx =
    RegExIsOwned<true>(wxRegEx) impl
        RegExMethods
}
impl<const OWNED: bool> RegExIsOwned<OWNED> {
    pub fn new() -> RegExIsOwned<OWNED> {
        unsafe { RegExIsOwned(ffi::wxRegEx_new()) }
    }
    pub fn new_with_str(expr: &str, flags: c_int) -> RegExIsOwned<OWNED> {
        unsafe {
            let expr = WxString::from(expr);
            let expr = expr.as_ptr();
            RegExIsOwned(ffi::wxRegEx_new1(expr, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RegExIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRegEx_delete(self.0) }
        }
    }
}

// wxRegKey
wx_class! { RegKey =
    RegKeyIsOwned<true>(wxRegKey) impl
        RegKeyMethods
}
impl<const OWNED: bool> RegKeyIsOwned<OWNED> {
    //  ENUM: AccessMode
    pub const Read: c_int = 0;
    pub const Write: c_int = 0 + 1;

    //  ENUM: StdKey
    pub const HKCR: c_int = 0;
    pub const HKCU: c_int = 0 + 1;
    pub const HKLM: c_int = 0 + 2;
    pub const HKUSR: c_int = 0 + 3;
    pub const HKPD: c_int = 0 + 4;
    pub const HKCC: c_int = 0 + 5;
    pub const HKDD: c_int = 0 + 6;
    pub const HKMAX: c_int = 0 + 7;

    //  ENUM: ValueType
    pub const Type_None: c_int = 0;
    pub const Type_String: c_int = 0 + 1;
    pub const Type_Expand_String: c_int = 0 + 2;
    pub const Type_Binary: c_int = 0 + 3;
    pub const Type_Dword: c_int = 0 + 4;
    pub const Type_Dword_little_endian: c_int = 0 + 5;
    pub const Type_Dword_big_endian: c_int = 0 + 6;
    pub const Type_Link: c_int = 0 + 7;
    pub const Type_Multi_String: c_int = 0 + 8;
    pub const Type_Resource_list: c_int = 0 + 9;
    pub const Type_Full_resource_descriptor: c_int = 0 + 10;
    pub const Type_Resource_requirements_list: c_int = 0 + 11;

    //  ENUM: WOW64ViewMode
    pub const WOW64ViewMode_Default: c_int = 0;
    pub const WOW64ViewMode_32: c_int = 0 + 1;
    pub const WOW64ViewMode_64: c_int = 0 + 2;

    // NOT_SUPPORTED: fn wxRegKey()
    // NOT_SUPPORTED: fn wxRegKey1()
    // NOT_SUPPORTED: fn wxRegKey2()
    pub fn new<R: RegKeyMethods>(key_parent: &R, str_key: &str) -> RegKeyIsOwned<OWNED> {
        unsafe {
            let key_parent = key_parent.as_ptr();
            let str_key = WxString::from(str_key);
            let str_key = str_key.as_ptr();
            RegKeyIsOwned(ffi::wxRegKey_new3(key_parent, str_key))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for RegKeyIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxRegKey_delete(self.0) }
        }
    }
}

// wxSecretStore
wx_class! { SecretStore =
    SecretStoreIsOwned<true>(wxSecretStore) impl
        SecretStoreMethods
}
impl<const OWNED: bool> SecretStoreIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SecretStoreIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSecretStore_delete(self.0) }
        }
    }
}

// wxSecretValue
wx_class! { SecretValue =
    SecretValueIsOwned<true>(wxSecretValue) impl
        SecretValueMethods
}
impl<const OWNED: bool> SecretValueIsOwned<OWNED> {
    pub fn new() -> SecretValueIsOwned<OWNED> {
        unsafe { SecretValueIsOwned(ffi::wxSecretValue_new()) }
    }
    pub fn new_with_sz(size: usize, data: *const c_void) -> SecretValueIsOwned<OWNED> {
        unsafe { SecretValueIsOwned(ffi::wxSecretValue_new1(size, data)) }
    }
    pub fn new_with_str(secret: &str) -> SecretValueIsOwned<OWNED> {
        unsafe {
            let secret = WxString::from(secret);
            let secret = secret.as_ptr();
            SecretValueIsOwned(ffi::wxSecretValue_new2(secret))
        }
    }
    pub fn new_with_secretvalue<S: SecretValueMethods>(other: &S) -> SecretValueIsOwned<OWNED> {
        unsafe {
            let other = other.as_ptr();
            SecretValueIsOwned(ffi::wxSecretValue_new3(other))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SecretValueIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSecretValue_delete(self.0) }
        }
    }
}

// wxSemaphore
wx_class! { Semaphore =
    SemaphoreIsOwned<true>(wxSemaphore) impl
        SemaphoreMethods
}
impl<const OWNED: bool> SemaphoreIsOwned<OWNED> {
    pub fn new(initialcount: c_int, maxcount: c_int) -> SemaphoreIsOwned<OWNED> {
        unsafe { SemaphoreIsOwned(ffi::wxSemaphore_new(initialcount, maxcount)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SemaphoreIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSemaphore_delete(self.0) }
        }
    }
}

// wxServer
wx_class! { Server =
    ServerIsOwned<true>(wxServer) impl
        ServerMethods
}
impl<const OWNED: bool> ServerIsOwned<OWNED> {
    pub fn new() -> ServerIsOwned<OWNED> {
        unsafe { ServerIsOwned(ffi::wxServer_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ServerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxServer_delete(self.0) }
        }
    }
}

// wxSharedClientDataContainer
wx_class! { SharedClientDataContainer =
    SharedClientDataContainerIsOwned<true>(wxSharedClientDataContainer) impl
        SharedClientDataContainerMethods
}
impl<const OWNED: bool> SharedClientDataContainerIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SharedClientDataContainerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSharedClientDataContainer_delete(self.0) }
        }
    }
}

// wxSingleChoiceDialog
wx_class! { SingleChoiceDialog =
    SingleChoiceDialogIsOwned<true>(wxSingleChoiceDialog) impl
        SingleChoiceDialogMethods,
        DialogMethods,
        TopLevelWindowMethods,
        NonOwnedWindowMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> SingleChoiceDialogIsOwned<OWNED> {
    pub fn new<W: WindowMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        n: c_int,
        choices: *const c_void,
        client_data: *mut c_void,
        style: c_long,
        pos: &P,
    ) -> SingleChoiceDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let pos = pos.as_ptr();
            SingleChoiceDialogIsOwned(ffi::wxSingleChoiceDialog_new(
                parent,
                message,
                caption,
                n,
                choices,
                client_data,
                style,
                pos,
            ))
        }
    }
    pub fn new<W: WindowMethods, A: ArrayStringMethods, P: PointMethods>(
        parent: Option<&W>,
        message: &str,
        caption: &str,
        choices: &A,
        client_data: *mut c_void,
        style: c_long,
        pos: &P,
    ) -> SingleChoiceDialogIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let message = WxString::from(message);
            let message = message.as_ptr();
            let caption = WxString::from(caption);
            let caption = caption.as_ptr();
            let choices = choices.as_ptr();
            let pos = pos.as_ptr();
            SingleChoiceDialogIsOwned(ffi::wxSingleChoiceDialog_new1(
                parent,
                message,
                caption,
                choices,
                client_data,
                style,
                pos,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SingleChoiceDialogIsOwned<OWNED>> for DialogIsOwned<OWNED> {
    fn from(o: SingleChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SingleChoiceDialogIsOwned<OWNED>> for TopLevelWindowIsOwned<OWNED> {
    fn from(o: SingleChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SingleChoiceDialogIsOwned<OWNED>> for NonOwnedWindowIsOwned<OWNED> {
    fn from(o: SingleChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SingleChoiceDialogIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: SingleChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SingleChoiceDialogIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: SingleChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<SingleChoiceDialogIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SingleChoiceDialogIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SingleChoiceDialogIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSingleChoiceDialog_CLASSINFO()) }
    }
}
// Mix-in(s) to wxSingleChoiceDialog
impl<const OWNED: bool> TrackableMethods for SingleChoiceDialogIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxSingleChoiceDialog_AsTrackable(self.as_ptr()) }
    }
}

// wxSingleInstanceChecker
wx_class! { SingleInstanceChecker =
    SingleInstanceCheckerIsOwned<true>(wxSingleInstanceChecker) impl
        SingleInstanceCheckerMethods
}
impl<const OWNED: bool> SingleInstanceCheckerIsOwned<OWNED> {
    pub fn new() -> SingleInstanceCheckerIsOwned<OWNED> {
        unsafe { SingleInstanceCheckerIsOwned(ffi::wxSingleInstanceChecker_new()) }
    }
    pub fn new_with_str(name: &str, path: &str) -> SingleInstanceCheckerIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let path = WxString::from(path);
            let path = path.as_ptr();
            SingleInstanceCheckerIsOwned(ffi::wxSingleInstanceChecker_new1(name, path))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for SingleInstanceCheckerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxSingleInstanceChecker_delete(self.0) }
        }
    }
}

// wxStackFrame
wx_class! { StackFrame =
    StackFrameIsOwned<true>(wxStackFrame) impl
        StackFrameMethods
}
impl<const OWNED: bool> StackFrameIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StackFrameIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStackFrame_delete(self.0) }
        }
    }
}

// wxStackWalker
wx_class! { StackWalker =
    StackWalkerIsOwned<true>(wxStackWalker) impl
        StackWalkerMethods
}
impl<const OWNED: bool> StackWalkerIsOwned<OWNED> {
    pub fn new(argv0: *const c_void) -> StackWalkerIsOwned<OWNED> {
        unsafe { StackWalkerIsOwned(ffi::wxStackWalker_new(argv0)) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StackWalkerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStackWalker_delete(self.0) }
        }
    }
}

// wxStandardPaths
wx_class! { StandardPaths =
    StandardPathsIsOwned<true>(wxStandardPaths) impl
        StandardPathsMethods
}
impl<const OWNED: bool> StandardPathsIsOwned<OWNED> {
    //  ENUM: ResourceCat
    pub const ResourceCat_None: c_int = 0;
    pub const ResourceCat_Messages: c_int = 0 + 1;

    //  ENUM: Dir
    pub const Dir_Cache: c_int = 0;
    pub const Dir_Documents: c_int = 0 + 1;
    pub const Dir_Desktop: c_int = 0 + 2;
    pub const Dir_Downloads: c_int = 0 + 3;
    pub const Dir_Music: c_int = 0 + 4;
    pub const Dir_Pictures: c_int = 0 + 5;
    pub const Dir_Videos: c_int = 0 + 6;

    //  ENUM: FileLayout
    pub const FileLayout_Classic: c_int = 0;
    pub const FileLayout_XDG: c_int = 0 + 1;

    //  ENUM: ConfigFileConv
    pub const ConfigFileConv_Dot: c_int = 0;
    pub const ConfigFileConv_Ext: c_int = 0 + 1;

    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StandardPathsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStandardPaths_delete(self.0) }
        }
    }
}

// wxStopWatch
wx_class! { StopWatch =
    StopWatchIsOwned<true>(wxStopWatch) impl
        StopWatchMethods
}
impl<const OWNED: bool> StopWatchIsOwned<OWNED> {
    pub fn new() -> StopWatchIsOwned<OWNED> {
        unsafe { StopWatchIsOwned(ffi::wxStopWatch_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StopWatchIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStopWatch_delete(self.0) }
        }
    }
}

// wxStreamBuffer
wx_class! { StreamBuffer =
    StreamBufferIsOwned<true>(wxStreamBuffer) impl
        StreamBufferMethods
}
impl<const OWNED: bool> StreamBufferIsOwned<OWNED> {
    //  ENUM: BufMode
    //   DUP: read
    //   DUP: write
    //   DUP: read_write

    // NOT_SUPPORTED: fn wxStreamBuffer()
    pub fn new_with_sz_inputstream(
        bufsize: usize,
        stream: *mut c_void,
    ) -> StreamBufferIsOwned<OWNED> {
        unsafe { StreamBufferIsOwned(ffi::wxStreamBuffer_new1(bufsize, stream)) }
    }
    pub fn new_with_sz_outputstream(
        bufsize: usize,
        stream: *mut c_void,
    ) -> StreamBufferIsOwned<OWNED> {
        unsafe { StreamBufferIsOwned(ffi::wxStreamBuffer_new2(bufsize, stream)) }
    }
    // NOT_SUPPORTED: fn wxStreamBuffer3()
    pub fn new_with_streambuffer<S: StreamBufferMethods>(buffer: &S) -> StreamBufferIsOwned<OWNED> {
        unsafe {
            let buffer = buffer.as_ptr();
            StreamBufferIsOwned(ffi::wxStreamBuffer_new4(buffer))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for StreamBufferIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStreamBuffer_delete(self.0) }
        }
    }
}

// wxStringClientData
wx_class! { StringClientData =
    StringClientDataIsOwned<true>(wxStringClientData) impl
        StringClientDataMethods,
        ClientDataMethods
}
impl<const OWNED: bool> StringClientDataIsOwned<OWNED> {
    pub fn new() -> StringClientDataIsOwned<OWNED> {
        unsafe { StringClientDataIsOwned(ffi::wxStringClientData_new()) }
    }
    pub fn new_with_str(data: &str) -> StringClientDataIsOwned<OWNED> {
        unsafe {
            let data = WxString::from(data);
            let data = data.as_ptr();
            StringClientDataIsOwned(ffi::wxStringClientData_new1(data))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StringClientDataIsOwned<OWNED>> for ClientDataIsOwned<OWNED> {
    fn from(o: StringClientDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for StringClientDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxStringClientData_delete(self.0) }
        }
    }
}

// wxStringTokenizer
wx_class! { StringTokenizer =
    StringTokenizerIsOwned<true>(wxStringTokenizer) impl
        StringTokenizerMethods,
        ObjectMethods
}
impl<const OWNED: bool> StringTokenizerIsOwned<OWNED> {
    pub fn new() -> StringTokenizerIsOwned<OWNED> {
        unsafe { StringTokenizerIsOwned(ffi::wxStringTokenizer_new()) }
    }
    // NOT_SUPPORTED: fn wxStringTokenizer1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<StringTokenizerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: StringTokenizerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for StringTokenizerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxStringTokenizer_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for StringTokenizerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxSystemOptions
wx_class! { SystemOptions =
    SystemOptionsIsOwned<true>(wxSystemOptions) impl
        SystemOptionsMethods,
        ObjectMethods
}
impl<const OWNED: bool> SystemOptionsIsOwned<OWNED> {
    pub fn new() -> SystemOptionsIsOwned<OWNED> {
        unsafe { SystemOptionsIsOwned(ffi::wxSystemOptions_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<SystemOptionsIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: SystemOptionsIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for SystemOptionsIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxSystemOptions_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for SystemOptionsIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTarClassFactory
wx_class! { TarClassFactory =
    TarClassFactoryIsOwned<true>(wxTarClassFactory) impl
        TarClassFactoryMethods,
        ArchiveClassFactoryMethods,
        ObjectMethods
}
impl<const OWNED: bool> TarClassFactoryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TarClassFactoryIsOwned<OWNED>> for ArchiveClassFactoryIsOwned<OWNED> {
    fn from(o: TarClassFactoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TarClassFactoryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TarClassFactoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TarClassFactoryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTarClassFactory_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TarClassFactoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTempFFile
wx_class! { TempFFile =
    TempFFileIsOwned<true>(wxTempFFile) impl
        TempFFileMethods
}
impl<const OWNED: bool> TempFFileIsOwned<OWNED> {
    pub fn new() -> TempFFileIsOwned<OWNED> {
        unsafe { TempFFileIsOwned(ffi::wxTempFFile_new()) }
    }
    pub fn new_with_str(str_name: &str) -> TempFFileIsOwned<OWNED> {
        unsafe {
            let str_name = WxString::from(str_name);
            let str_name = str_name.as_ptr();
            TempFFileIsOwned(ffi::wxTempFFile_new1(str_name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TempFFileIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTempFFile_delete(self.0) }
        }
    }
}

// wxTempFile
wx_class! { TempFile =
    TempFileIsOwned<true>(wxTempFile) impl
        TempFileMethods
}
impl<const OWNED: bool> TempFileIsOwned<OWNED> {
    pub fn new() -> TempFileIsOwned<OWNED> {
        unsafe { TempFileIsOwned(ffi::wxTempFile_new()) }
    }
    pub fn new_with_str(str_name: &str) -> TempFileIsOwned<OWNED> {
        unsafe {
            let str_name = WxString::from(str_name);
            let str_name = str_name.as_ptr();
            TempFileIsOwned(ffi::wxTempFile_new1(str_name))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for TempFileIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxTempFile_delete(self.0) }
        }
    }
}

// wxThreadHelper
wx_class! { ThreadHelper =
    ThreadHelperIsOwned<true>(wxThreadHelper) impl
        ThreadHelperMethods
}
impl<const OWNED: bool> ThreadHelperIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxThreadHelper()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ThreadHelperIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxThreadHelper_delete(self.0) }
        }
    }
}

// wxTimer
wx_class! { Timer =
    TimerIsOwned<true>(wxTimer) impl
        TimerMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerIsOwned<OWNED> {
    pub fn new() -> TimerIsOwned<OWNED> {
        unsafe { TimerIsOwned(ffi::wxTimer_new()) }
    }
    pub fn new_with_evthandler<E: EvtHandlerMethods>(
        owner: Option<&E>,
        id: c_int,
    ) -> TimerIsOwned<OWNED> {
        unsafe {
            let owner = match owner {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            TimerIsOwned(ffi::wxTimer_new1(owner, id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TimerIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: TimerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimerIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TimerIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimerIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTimer_CLASSINFO()) }
    }
}
// Mix-in(s) to wxTimer
impl<const OWNED: bool> TrackableMethods for TimerIsOwned<OWNED> {
    fn as_trackable(&self) -> *mut c_void {
        unsafe { ffi::wxTimer_AsTrackable(self.as_ptr()) }
    }
}

// wxTimerEvent
wx_class! { TimerEvent =
    TimerEventIsOwned<true>(wxTimerEvent) impl
        TimerEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> TimerEventIsOwned<OWNED> {
    pub fn new<T: TimerMethods>(timer: &T) -> TimerEventIsOwned<OWNED> {
        unsafe {
            let timer = timer.as_ptr();
            TimerEventIsOwned(ffi::wxTimerEvent_new(timer))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<TimerEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: TimerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<TimerEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: TimerEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for TimerEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxTimerEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for TimerEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxTrackable

// wxUILocale
wx_class! { UILocale =
    UILocaleIsOwned<true>(wxUILocale) impl
        UILocaleMethods
}
impl<const OWNED: bool> UILocaleIsOwned<OWNED> {
    pub fn new<L: LocaleIdentMethods>(locale_id: &L) -> UILocaleIsOwned<OWNED> {
        unsafe {
            let locale_id = locale_id.as_ptr();
            UILocaleIsOwned(ffi::wxUILocale_new(locale_id))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for UILocaleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxUILocale_delete(self.0) }
        }
    }
}

// wxURI
wx_class! { URI =
    URIIsOwned<true>(wxURI) impl
        URIMethods,
        ObjectMethods
}
impl<const OWNED: bool> URIIsOwned<OWNED> {
    pub fn new() -> URIIsOwned<OWNED> {
        unsafe { URIIsOwned(ffi::wxURI_new()) }
    }
    pub fn new_with_str(uri: &str) -> URIIsOwned<OWNED> {
        unsafe {
            let uri = WxString::from(uri);
            let uri = uri.as_ptr();
            URIIsOwned(ffi::wxURI_new1(uri))
        }
    }
    pub fn new_with_uri<U: URIMethods>(uri: &U) -> URIIsOwned<OWNED> {
        unsafe {
            let uri = uri.as_ptr();
            URIIsOwned(ffi::wxURI_new2(uri))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<URIIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: URIIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for URIIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxURI_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for URIIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxUniCharRef
wx_class! { UniCharRef =
    UniCharRefIsOwned<true>(wxUniCharRef) impl
        UniCharRefMethods
}
impl<const OWNED: bool> UniCharRefIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for UniCharRefIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxUniCharRef_delete(self.0) }
        }
    }
}

// wxVariantData
wx_class! { VariantData =
    VariantDataIsOwned<true>(wxVariantData) impl
        VariantDataMethods,
        ObjectRefDataMethods
}
impl<const OWNED: bool> VariantDataIsOwned<OWNED> {
    pub fn new() -> VariantDataIsOwned<OWNED> {
        unsafe { VariantDataIsOwned(ffi::wxVariantData_new()) }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<VariantDataIsOwned<OWNED>> for ObjectRefDataIsOwned<OWNED> {
    fn from(o: VariantDataIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for VariantDataIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVariantData_delete(self.0) }
        }
    }
}

// wxVersionInfo
wx_class! { VersionInfo =
    VersionInfoIsOwned<true>(wxVersionInfo) impl
        VersionInfoMethods
}
impl<const OWNED: bool> VersionInfoIsOwned<OWNED> {
    pub fn new(
        name: &str,
        major: c_int,
        minor: c_int,
        micro: c_int,
        revision: c_int,
        description: &str,
        copyright: &str,
    ) -> VersionInfoIsOwned<OWNED> {
        unsafe {
            let name = WxString::from(name);
            let name = name.as_ptr();
            let description = WxString::from(description);
            let description = description.as_ptr();
            let copyright = WxString::from(copyright);
            let copyright = copyright.as_ptr();
            VersionInfoIsOwned(ffi::wxVersionInfo_new(
                name,
                major,
                minor,
                micro,
                revision,
                description,
                copyright,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for VersionInfoIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxVersionInfo_delete(self.0) }
        }
    }
}

// wxWindowUpdateLocker
wx_class! { WindowUpdateLocker =
    WindowUpdateLockerIsOwned<true>(wxWindowUpdateLocker) impl
        WindowUpdateLockerMethods
}
impl<const OWNED: bool> WindowUpdateLockerIsOwned<OWNED> {
    pub fn new() -> WindowUpdateLockerIsOwned<OWNED> {
        unsafe { WindowUpdateLockerIsOwned(ffi::wxWindowUpdateLocker_new()) }
    }
    pub fn new_with_window<W: WindowMethods>(win: Option<&W>) -> WindowUpdateLockerIsOwned<OWNED> {
        unsafe {
            let win = match win {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            WindowUpdateLockerIsOwned(ffi::wxWindowUpdateLocker_new1(win))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for WindowUpdateLockerIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxWindowUpdateLocker_delete(self.0) }
        }
    }
}

// wxZipClassFactory
wx_class! { ZipClassFactory =
    ZipClassFactoryIsOwned<true>(wxZipClassFactory) impl
        ZipClassFactoryMethods,
        ArchiveClassFactoryMethods,
        ObjectMethods
}
impl<const OWNED: bool> ZipClassFactoryIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> From<ZipClassFactoryIsOwned<OWNED>> for ArchiveClassFactoryIsOwned<OWNED> {
    fn from(o: ZipClassFactoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<ZipClassFactoryIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: ZipClassFactoryIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for ZipClassFactoryIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxZipClassFactory_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for ZipClassFactoryIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxZipNotifier
wx_class! { ZipNotifier =
    ZipNotifierIsOwned<true>(wxZipNotifier) impl
        ZipNotifierMethods
}
impl<const OWNED: bool> ZipNotifierIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Drop for ZipNotifierIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxZipNotifier_delete(self.0) }
        }
    }
}
