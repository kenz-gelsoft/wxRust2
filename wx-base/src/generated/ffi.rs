use std::os::raw::{c_double, c_int, c_long, c_uchar, c_uint, c_void};

pub use crate::ffi::*;

extern "C" {

    // wxAppTraits
    pub fn wxAppTraits_delete(self_: *mut c_void);
    pub fn wxAppTraits_CreateConfig(self_: *mut c_void) -> *mut c_void;
    pub fn wxAppTraits_CreateEventLoop(self_: *mut c_void) -> *mut c_void;
    pub fn wxAppTraits_CreateFontMapper(self_: *mut c_void) -> *mut c_void;
    pub fn wxAppTraits_CreateLogTarget(self_: *mut c_void) -> *mut c_void;
    pub fn wxAppTraits_CreateMessageOutput(self_: *mut c_void) -> *mut c_void;
    pub fn wxAppTraits_CreateRenderer(self_: *mut c_void) -> *mut c_void;
    pub fn wxAppTraits_GetDesktopEnvironment(self_: *const c_void) -> *mut c_void;
    pub fn wxAppTraits_GetStandardPaths(self_: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxAppTraits_GetToolkitVersion(self_: *const c_void, major: *mut c_void, minor: *mut c_void, micro: *mut c_void) -> wxPortId;
    pub fn wxAppTraits_HasStderr(self_: *mut c_void) -> bool;
    pub fn wxAppTraits_IsUsingUniversalWidgets(self_: *const c_void) -> bool;
    pub fn wxAppTraits_ShowAssertDialog(self_: *mut c_void, msg: *const c_void) -> bool;
    pub fn wxAppTraits_SafeMessageBox(
        self_: *mut c_void,
        text: *const c_void,
        title: *const c_void,
    ) -> bool;
    pub fn wxAppTraits_GetAssertStackTrace(self_: *mut c_void) -> *mut c_void;

    // wxArchiveEntry
    pub fn wxArchiveEntry_CLASSINFO() -> *mut c_void;
    pub fn wxArchiveEntry_Clone(self_: *const c_void) -> *mut c_void;
    pub fn wxArchiveEntry_GetDateTime(self_: *const c_void) -> *mut c_void;
    pub fn wxArchiveEntry_SetDateTime(self_: *mut c_void, dt: *const c_void);
    // NOT_SUPPORTED: pub fn wxArchiveEntry_GetName(self_: *const c_void, format: wxPathFormat) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxArchiveEntry_SetName(self_: *mut c_void, name: *const c_void, format: wxPathFormat);
    // NOT_SUPPORTED: pub fn wxArchiveEntry_GetSize(self_: *const c_void) -> wxFileOffset;
    // NOT_SUPPORTED: pub fn wxArchiveEntry_SetSize(self_: *mut c_void, size: wxFileOffset);
    // NOT_SUPPORTED: pub fn wxArchiveEntry_GetInternalFormat(self_: *const c_void) -> wxPathFormat;
    pub fn wxArchiveEntry_GetInternalName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxArchiveEntry_GetOffset(self_: *const c_void) -> wxFileOffset;
    pub fn wxArchiveEntry_IsDir(self_: *const c_void) -> bool;
    pub fn wxArchiveEntry_SetIsDir(self_: *mut c_void, is_dir: bool);
    pub fn wxArchiveEntry_IsReadOnly(self_: *const c_void) -> bool;
    pub fn wxArchiveEntry_SetIsReadOnly(self_: *mut c_void, is_read_only: bool);
    pub fn wxArchiveEntry_SetNotifier(self_: *mut c_void, notifier: *mut c_void);
    pub fn wxArchiveEntry_UnsetNotifier(self_: *mut c_void);

    // wxArchiveNotifier
    pub fn wxArchiveNotifier_delete(self_: *mut c_void);
    pub fn wxArchiveNotifier_OnEntryUpdated(self_: *mut c_void, entry: *mut c_void);

    // wxClassInfo
    pub fn wxClassInfo_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxClassInfo_new(class_name: *const c_void, base_class1: *const c_void, base_class2: *const c_void, size: c_int, fn: wxObjectConstructorFn) -> *mut c_void;
    pub fn wxClassInfo_CreateObject(self_: *const c_void) -> *mut c_void;
    pub fn wxClassInfo_GetBaseClassName1(self_: *const c_void) -> *const c_void;
    pub fn wxClassInfo_GetBaseClassName2(self_: *const c_void) -> *const c_void;
    pub fn wxClassInfo_GetClassName(self_: *const c_void) -> *const c_void;
    pub fn wxClassInfo_GetSize(self_: *const c_void) -> c_int;
    pub fn wxClassInfo_IsDynamic(self_: *const c_void) -> bool;
    pub fn wxClassInfo_IsKindOf(self_: *const c_void, info: *const c_void) -> bool;
    pub fn wxClassInfo_FindClass(class_name: *const c_void) -> *mut c_void;

    // wxClient
    pub fn wxClient_CLASSINFO() -> *mut c_void;
    pub fn wxClient_new() -> *mut c_void;
    pub fn wxClient_MakeConnection(
        self_: *mut c_void,
        host: *const c_void,
        service: *const c_void,
        topic: *const c_void,
    ) -> *mut c_void;
    pub fn wxClient_OnMakeConnection(self_: *mut c_void) -> *mut c_void;
    pub fn wxClient_ValidHost(self_: *mut c_void, host: *const c_void) -> bool;

    // wxClientData
    pub fn wxClientData_delete(self_: *mut c_void);
    pub fn wxClientData_new() -> *mut c_void;
    // DTOR: pub fn wxClientData_~wxClientData(self_: *mut c_void);

    // wxClientDataContainer
    pub fn wxClientDataContainer_delete(self_: *mut c_void);
    pub fn wxClientDataContainer_new() -> *mut c_void;
    // DTOR: pub fn wxClientDataContainer_~wxClientDataContainer(self_: *mut c_void);
    pub fn wxClientDataContainer_GetClientData(self_: *const c_void) -> *mut c_void;
    pub fn wxClientDataContainer_GetClientObject(self_: *const c_void) -> *mut c_void;
    pub fn wxClientDataContainer_SetClientData(self_: *mut c_void, data: *mut c_void);
    pub fn wxClientDataContainer_SetClientObject(self_: *mut c_void, data: *mut c_void);

    // wxCmdLineParser
    pub fn wxCmdLineParser_delete(self_: *mut c_void);
    pub fn wxCmdLineParser_new() -> *mut c_void;
    pub fn wxCmdLineParser_new1(argc: c_int, argv: *mut c_void) -> *mut c_void;
    pub fn wxCmdLineParser_new2(argc: c_int, argv: *mut c_void) -> *mut c_void;
    pub fn wxCmdLineParser_new3(cmdline: *const c_void) -> *mut c_void;
    pub fn wxCmdLineParser_new4(desc: *const c_void) -> *mut c_void;
    pub fn wxCmdLineParser_new5(desc: *const c_void, argc: c_int, argv: *mut c_void)
        -> *mut c_void;
    pub fn wxCmdLineParser_new6(desc: *const c_void, cmdline: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxCmdLineParser_~wxCmdLineParser(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxCmdLineParser_AddLongOption(self_: *mut c_void, lng: *const c_void, desc: *const c_void, type_: wxCmdLineParamType, flags: c_int);
    pub fn wxCmdLineParser_AddLongSwitch(
        self_: *mut c_void,
        lng: *const c_void,
        desc: *const c_void,
        flags: c_int,
    );
    // NOT_SUPPORTED: pub fn wxCmdLineParser_AddOption(self_: *mut c_void, name: *const c_void, lng: *const c_void, desc: *const c_void, type_: wxCmdLineParamType, flags: c_int);
    // NOT_SUPPORTED: pub fn wxCmdLineParser_AddParam(self_: *mut c_void, desc: *const c_void, type_: wxCmdLineParamType, flags: c_int);
    pub fn wxCmdLineParser_AddSwitch(
        self_: *mut c_void,
        name: *const c_void,
        lng: *const c_void,
        desc: *const c_void,
        flags: c_int,
    );
    pub fn wxCmdLineParser_AddUsageText(self_: *mut c_void, text: *const c_void);
    pub fn wxCmdLineParser_AreLongOptionsEnabled(self_: *const c_void) -> bool;
    pub fn wxCmdLineParser_DisableLongOptions(self_: *mut c_void);
    pub fn wxCmdLineParser_EnableLongOptions(self_: *mut c_void, enable: bool);
    pub fn wxCmdLineParser_Found(self_: *const c_void, name: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxCmdLineParser_FoundSwitch(self_: *const c_void, name: *const c_void) -> wxCmdLineSwitchState;
    pub fn wxCmdLineParser_Found1(
        self_: *const c_void,
        name: *const c_void,
        value: *mut c_void,
    ) -> bool;
    pub fn wxCmdLineParser_Found2(
        self_: *const c_void,
        name: *const c_void,
        value: *mut c_void,
    ) -> bool;
    pub fn wxCmdLineParser_Found3(
        self_: *const c_void,
        name: *const c_void,
        value: *mut c_void,
    ) -> bool;
    pub fn wxCmdLineParser_Found4(
        self_: *const c_void,
        name: *const c_void,
        value: *mut c_void,
    ) -> bool;
    pub fn wxCmdLineParser_GetParam(self_: *const c_void, n: usize) -> *mut c_void;
    pub fn wxCmdLineParser_GetParamCount(self_: *const c_void) -> usize;
    pub fn wxCmdLineParser_GetArguments(self_: *const c_void) -> *mut c_void;
    pub fn wxCmdLineParser_Parse(self_: *mut c_void, give_usage: bool) -> c_int;
    pub fn wxCmdLineParser_SetCmdLine(self_: *mut c_void, argc: c_int, argv: *mut c_void);
    pub fn wxCmdLineParser_SetCmdLine1(self_: *mut c_void, argc: c_int, argv: *mut c_void);
    pub fn wxCmdLineParser_SetCmdLine2(self_: *mut c_void, cmdline: *const c_void);
    pub fn wxCmdLineParser_SetDesc(self_: *mut c_void, desc: *const c_void);
    pub fn wxCmdLineParser_SetLogo(self_: *mut c_void, logo: *const c_void);
    pub fn wxCmdLineParser_SetSwitchChars(self_: *mut c_void, switch_chars: *const c_void);
    pub fn wxCmdLineParser_Usage(self_: *const c_void);
    pub fn wxCmdLineParser_GetUsageString(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxCmdLineParser_ConvertStringToArgs(cmdline: *const c_void, flags: wxCmdLineSplitType) -> *mut c_void;

    // wxCondition
    pub fn wxCondition_delete(self_: *mut c_void);
    pub fn wxCondition_new(mutex: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxCondition_~wxCondition(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxCondition_Broadcast(self_: *mut c_void) -> wxCondError;
    pub fn wxCondition_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxCondition_Signal(self_: *mut c_void) -> wxCondError;
    // NOT_SUPPORTED: pub fn wxCondition_Wait(self_: *mut c_void) -> wxCondError;
    // NOT_SUPPORTED: pub fn wxCondition_Wait1(self_: *mut c_void, predicate: *const c_void) -> wxCondError;
    // NOT_SUPPORTED: pub fn wxCondition_WaitTimeout(self_: *mut c_void, milliseconds: unsigned long) -> wxCondError;

    // wxConfigPathChanger
    pub fn wxConfigPathChanger_delete(self_: *mut c_void);
    pub fn wxConfigPathChanger_new(
        p_container: *const c_void,
        str_entry: *const c_void,
    ) -> *mut c_void;
    // DTOR: pub fn wxConfigPathChanger_~wxConfigPathChanger(self_: *mut c_void);
    pub fn wxConfigPathChanger_Name(self_: *const c_void) -> *mut c_void;
    pub fn wxConfigPathChanger_UpdateIfDeleted(self_: *mut c_void);

    // wxConnection
    pub fn wxConnection_CLASSINFO() -> *mut c_void;
    pub fn wxConnection_new() -> *mut c_void;
    pub fn wxConnection_new1(buffer: *mut c_void, size: usize) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxConnection_Advise(self_: *mut c_void, item: *const c_void, data: *const c_void, size: usize, format: wxIPCFormat) -> bool;
    pub fn wxConnection_Advise1(
        self_: *mut c_void,
        item: *const c_void,
        data: *const c_void,
    ) -> bool;
    pub fn wxConnection_Advise2(
        self_: *mut c_void,
        item: *const c_void,
        data: *const c_void,
    ) -> bool;
    pub fn wxConnection_Advise3(self_: *mut c_void, item: *const c_void, data: wxString) -> bool;
    pub fn wxConnection_Disconnect(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_Execute(self_: *mut c_void, data: *const c_void, size: usize, format: wxIPCFormat) -> bool;
    pub fn wxConnection_Execute1(self_: *mut c_void, data: *const c_void) -> bool;
    pub fn wxConnection_Execute2(self_: *mut c_void, data: *const c_void) -> bool;
    pub fn wxConnection_Execute3(self_: *mut c_void, data: wxString) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_OnAdvise(self_: *mut c_void, topic: *const c_void, item: *const c_void, data: *const c_void, size: usize, format: wxIPCFormat) -> bool;
    pub fn wxConnection_OnDisconnect(self_: *mut c_void) -> bool;
    pub fn wxConnection_OnExec(
        self_: *mut c_void,
        topic: *const c_void,
        data: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_OnPoke(self_: *mut c_void, topic: *const c_void, item: *const c_void, data: *const c_void, size: usize, format: wxIPCFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_OnRequest(self_: *mut c_void, topic: *const c_void, item: *const c_void, size: *mut c_void, format: wxIPCFormat) -> *const c_void;
    pub fn wxConnection_OnStartAdvise(
        self_: *mut c_void,
        topic: *const c_void,
        item: *const c_void,
    ) -> bool;
    pub fn wxConnection_OnStopAdvise(
        self_: *mut c_void,
        topic: *const c_void,
        item: *const c_void,
    ) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_Poke(self_: *mut c_void, item: *const c_void, data: *const c_void, size: usize, format: wxIPCFormat) -> bool;
    pub fn wxConnection_Poke1(self_: *mut c_void, item: *const c_void, data: *const c_void)
        -> bool;
    pub fn wxConnection_Poke2(self_: *mut c_void, item: *const c_void, data: *const c_void)
        -> bool;
    pub fn wxConnection_Poke3(self_: *mut c_void, item: *const c_void, data: wxString) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_Request(self_: *mut c_void, item: *const c_void, size: *mut c_void, format: wxIPCFormat) -> *const c_void;
    pub fn wxConnection_StartAdvise(self_: *mut c_void, item: *const c_void) -> bool;
    pub fn wxConnection_StopAdvise(self_: *mut c_void, item: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_IsTextFormat(format: wxIPCFormat) -> bool;
    // NOT_SUPPORTED: pub fn wxConnection_GetTextFromData(data: *const c_void, size: usize, format: wxIPCFormat) -> *mut c_void;

    // wxConnectionBase
    pub fn wxConnectionBase_CLASSINFO() -> *mut c_void;

    // wxCriticalSection
    pub fn wxCriticalSection_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxCriticalSection_new(crit_sec_type: wxCriticalSectionType) -> *mut c_void;
    // DTOR: pub fn wxCriticalSection_~wxCriticalSection(self_: *mut c_void);
    pub fn wxCriticalSection_Enter(self_: *mut c_void);
    pub fn wxCriticalSection_TryEnter(self_: *mut c_void) -> bool;
    pub fn wxCriticalSection_Leave(self_: *mut c_void);

    // wxCriticalSectionLocker
    pub fn wxCriticalSectionLocker_delete(self_: *mut c_void);
    pub fn wxCriticalSectionLocker_new(criticalsection: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxCriticalSectionLocker_~wxCriticalSectionLocker(self_: *mut c_void);

    // wxDataInputStream
    pub fn wxDataInputStream_delete(self_: *mut c_void);
    pub fn wxDataInputStream_new(stream: *mut c_void, conv: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDataInputStream_~wxDataInputStream(self_: *mut c_void);
    pub fn wxDataInputStream_BigEndianOrdered(self_: *mut c_void, be_order: bool);
    pub fn wxDataInputStream_GetConv(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxDataInputStream_Read8(self_: *mut c_void) -> wxUint8;
    pub fn wxDataInputStream_Read81(self_: *mut c_void, buffer: *mut c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataInputStream_Read16(self_: *mut c_void) -> wxUint16;
    pub fn wxDataInputStream_Read161(self_: *mut c_void, buffer: *mut c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataInputStream_Read32(self_: *mut c_void) -> wxUint32;
    pub fn wxDataInputStream_Read321(self_: *mut c_void, buffer: *mut c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataInputStream_Read64(self_: *mut c_void) -> wxUint64;
    pub fn wxDataInputStream_Read641(self_: *mut c_void, buffer: *mut c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataInputStream_ReadFloat(self_: *mut c_void) -> float;
    pub fn wxDataInputStream_ReadFloat1(self_: *mut c_void, buffer: *mut c_void, size: usize);
    pub fn wxDataInputStream_ReadDouble(self_: *mut c_void) -> c_double;
    pub fn wxDataInputStream_ReadDouble1(self_: *mut c_void, buffer: *mut c_void, size: usize);
    pub fn wxDataInputStream_ReadString(self_: *mut c_void) -> *mut c_void;
    pub fn wxDataInputStream_SetConv(self_: *mut c_void, conv: *const c_void);
    pub fn wxDataInputStream_UseBasicPrecisions(self_: *mut c_void);
    pub fn wxDataInputStream_UseExtendedPrecision(self_: *mut c_void);

    // wxDataOutputStream
    pub fn wxDataOutputStream_delete(self_: *mut c_void);
    pub fn wxDataOutputStream_new(stream: *mut c_void, conv: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDataOutputStream_~wxDataOutputStream(self_: *mut c_void);
    pub fn wxDataOutputStream_BigEndianOrdered(self_: *mut c_void, be_order: bool);
    pub fn wxDataOutputStream_GetConv(self_: *const c_void) -> *mut c_void;
    pub fn wxDataOutputStream_SetConv(self_: *mut c_void, conv: *const c_void);
    pub fn wxDataOutputStream_UseBasicPrecisions(self_: *mut c_void);
    pub fn wxDataOutputStream_UseExtendedPrecision(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDataOutputStream_Write8(self_: *mut c_void, i8: wxUint8);
    pub fn wxDataOutputStream_Write81(self_: *mut c_void, buffer: *const c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataOutputStream_Write16(self_: *mut c_void, i16: wxUint16);
    pub fn wxDataOutputStream_Write161(self_: *mut c_void, buffer: *const c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataOutputStream_Write32(self_: *mut c_void, i32: wxUint32);
    pub fn wxDataOutputStream_Write321(self_: *mut c_void, buffer: *const c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataOutputStream_Write64(self_: *mut c_void, i64: wxUint64);
    pub fn wxDataOutputStream_Write641(self_: *mut c_void, buffer: *const c_void, size: usize);
    // NOT_SUPPORTED: pub fn wxDataOutputStream_WriteFloat(self_: *mut c_void, f: float);
    pub fn wxDataOutputStream_WriteFloat1(self_: *mut c_void, buffer: *const c_void, size: usize);
    pub fn wxDataOutputStream_WriteDouble(self_: *mut c_void, d: c_double);
    pub fn wxDataOutputStream_WriteDouble1(self_: *mut c_void, buffer: *const c_void, size: usize);
    pub fn wxDataOutputStream_WriteString(self_: *mut c_void, string: *const c_void);

    // wxDateSpan
    pub fn wxDateSpan_delete(self_: *mut c_void);
    pub fn wxDateSpan_new(years: c_int, months: c_int, weeks: c_int, days: c_int) -> *mut c_void;
    pub fn wxDateSpan_Add(self_: *const c_void, other: *const c_void) -> *mut c_void;
    pub fn wxDateSpan_Add1(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    pub fn wxDateSpan_GetDays(self_: *const c_void) -> c_int;
    pub fn wxDateSpan_GetMonths(self_: *const c_void) -> c_int;
    pub fn wxDateSpan_GetTotalMonths(self_: *const c_void) -> c_int;
    pub fn wxDateSpan_GetTotalDays(self_: *const c_void) -> c_int;
    pub fn wxDateSpan_GetWeeks(self_: *const c_void) -> c_int;
    pub fn wxDateSpan_GetYears(self_: *const c_void) -> c_int;
    pub fn wxDateSpan_Multiply(self_: *const c_void, factor: c_int) -> *mut c_void;
    pub fn wxDateSpan_Multiply1(self_: *mut c_void, factor: c_int) -> *mut c_void;
    pub fn wxDateSpan_Neg(self_: *mut c_void) -> *mut c_void;
    pub fn wxDateSpan_Negate(self_: *const c_void) -> *mut c_void;
    pub fn wxDateSpan_SetDays(self_: *mut c_void, n: c_int) -> *mut c_void;
    pub fn wxDateSpan_SetMonths(self_: *mut c_void, n: c_int) -> *mut c_void;
    pub fn wxDateSpan_SetWeeks(self_: *mut c_void, n: c_int) -> *mut c_void;
    pub fn wxDateSpan_SetYears(self_: *mut c_void, n: c_int) -> *mut c_void;
    pub fn wxDateSpan_Subtract(self_: *const c_void, other: *const c_void) -> *mut c_void;
    pub fn wxDateSpan_Subtract1(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateSpan_operator+=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateSpan_operator-=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateSpan_operator-(self_: *mut c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateSpan_operator*=(self_: *mut c_void, factor: c_int) -> *mut c_void;
    // BLOCKED: pub fn wxDateSpan_operator!=(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn wxDateSpan_operator==(self_: *const c_void, other: *const c_void) -> bool;
    pub fn wxDateSpan_Day() -> *mut c_void;
    pub fn wxDateSpan_Days(days: c_int) -> *mut c_void;
    pub fn wxDateSpan_Month() -> *mut c_void;
    pub fn wxDateSpan_Months(mon: c_int) -> *mut c_void;
    pub fn wxDateSpan_Week() -> *mut c_void;
    pub fn wxDateSpan_Weeks(weeks: c_int) -> *mut c_void;
    pub fn wxDateSpan_Year() -> *mut c_void;
    pub fn wxDateSpan_Years(years: c_int) -> *mut c_void;

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
    pub fn wxDateTime_DiffAsDateSpan(self_: *const c_void, dt: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator+=(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator+(self_: *const c_void, ds: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator-=(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator-(self_: *const c_void, ds: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator+=1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator+1(self_: *const c_void, ts: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator-=1(self_: *mut c_void, diff: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxDateTime_operator-1(self_: *const c_void, ts: *const c_void) -> wxDateTime;
    // BLOCKED: pub fn wxDateTime_operator-2(self_: *const c_void, dt2: *const c_void) -> wxTimeSpan;
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

    // wxDateTimeHolidayAuthority
    pub fn wxDateTimeHolidayAuthority_delete(self_: *mut c_void);

    // wxDateTimeWorkDays
    pub fn wxDateTimeWorkDays_delete(self_: *mut c_void);

    // wxDir
    pub fn wxDir_delete(self_: *mut c_void);
    pub fn wxDir_new() -> *mut c_void;
    pub fn wxDir_new1(dir: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxDir_~wxDir(self_: *mut c_void);
    pub fn wxDir_Close(self_: *mut c_void);
    pub fn wxDir_GetFirst(
        self_: *const c_void,
        filename: *mut c_void,
        filespec: *const c_void,
        flags: c_int,
    ) -> bool;
    pub fn wxDir_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxDir_GetNameWithSep(self_: *const c_void) -> *mut c_void;
    pub fn wxDir_GetNext(self_: *const c_void, filename: *mut c_void) -> bool;
    pub fn wxDir_HasFiles(self_: *const c_void, filespec: *const c_void) -> bool;
    pub fn wxDir_HasSubDirs(self_: *const c_void, dirspec: *const c_void) -> bool;
    pub fn wxDir_IsOpened(self_: *const c_void) -> bool;
    pub fn wxDir_Open(self_: *mut c_void, dir: *const c_void) -> bool;
    pub fn wxDir_Traverse(
        self_: *const c_void,
        sink: *mut c_void,
        filespec: *const c_void,
        flags: c_int,
    ) -> usize;
    pub fn wxDir_Exists(dir: *const c_void) -> bool;
    pub fn wxDir_FindFirst(
        dirname: *const c_void,
        filespec: *const c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxDir_GetAllFiles(
        dirname: *const c_void,
        files: *mut c_void,
        filespec: *const c_void,
        flags: c_int,
    ) -> usize;
    // NOT_SUPPORTED: pub fn wxDir_GetTotalSize(dir: *const c_void, files_skipped: *mut c_void) -> wxULongLong;
    pub fn wxDir_Make(dir: *const c_void, perm: c_int, flags: c_int) -> bool;
    pub fn wxDir_Remove(dir: *const c_void, flags: c_int) -> bool;

    // wxDirTraverser
    pub fn wxDirTraverser_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxDirTraverser_OnDir(self_: *mut c_void, dirname: *const c_void) -> wxDirTraverseResult;
    // NOT_SUPPORTED: pub fn wxDirTraverser_OnFile(self_: *mut c_void, filename: *const c_void) -> wxDirTraverseResult;
    // NOT_SUPPORTED: pub fn wxDirTraverser_OnOpenError(self_: *mut c_void, openerrorname: *const c_void) -> wxDirTraverseResult;

    // wxEncodingConverter
    pub fn wxEncodingConverter_CLASSINFO() -> *mut c_void;
    pub fn wxEncodingConverter_Convert(
        self_: *const c_void,
        input: *const c_void,
        output: *mut c_void,
    ) -> bool;
    pub fn wxEncodingConverter_Convert1(
        self_: *const c_void,
        input: *const c_void,
        output: *mut c_void,
    ) -> bool;
    pub fn wxEncodingConverter_Convert2(
        self_: *const c_void,
        input: *const c_void,
        output: *mut c_void,
    ) -> bool;
    pub fn wxEncodingConverter_Convert3(
        self_: *const c_void,
        input: *const c_void,
        output: *mut c_void,
    ) -> bool;
    pub fn wxEncodingConverter_Convert4(self_: *const c_void, str: *mut c_void) -> bool;
    pub fn wxEncodingConverter_Convert5(self_: *const c_void, str: *mut c_void) -> bool;
    pub fn wxEncodingConverter_Convert6(self_: *const c_void, input: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxEncodingConverter_Init(self_: *mut c_void, input_enc: wxFontEncoding, output_enc: wxFontEncoding, method: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxEncodingConverter_GetAllEquivalents(enc: wxFontEncoding) -> wxFontEncodingArray;
    // NOT_SUPPORTED: pub fn wxEncodingConverter_GetPlatformEquivalents(enc: wxFontEncoding, platform: c_int) -> wxFontEncodingArray;
    pub fn wxEncodingConverter_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxEncodingConverter_CanConvert(enc_in: wxFontEncoding, enc_out: wxFontEncoding) -> bool;

    // wxEvent
    pub fn wxEvent_CLASSINFO() -> *mut c_void;
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
    pub fn wxEvtHandler_CLASSINFO() -> *mut c_void;
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

    // wxFFile
    pub fn wxFFile_delete(self_: *mut c_void);
    pub fn wxFFile_new() -> *mut c_void;
    pub fn wxFFile_new1(fp: *mut c_void) -> *mut c_void;
    pub fn wxFFile_new2(filename: *const c_void, mode: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxFFile_~wxFFile(self_: *mut c_void);
    pub fn wxFFile_Attach(self_: *mut c_void, fp: *mut c_void, name: *const c_void);
    pub fn wxFFile_Close(self_: *mut c_void) -> bool;
    pub fn wxFFile_Detach(self_: *mut c_void) -> *mut c_void;
    pub fn wxFFile_Eof(self_: *const c_void) -> bool;
    pub fn wxFFile_Error(self_: *const c_void) -> bool;
    pub fn wxFFile_Flush(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFFile_GetKind(self_: *const c_void) -> wxFileKind;
    pub fn wxFFile_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxFFile_IsOpened(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFFile_Length(self_: *const c_void) -> wxFileOffset;
    pub fn wxFFile_Open(self_: *mut c_void, filename: *const c_void, mode: *const c_void) -> bool;
    pub fn wxFFile_Read(self_: *mut c_void, buffer: *mut c_void, count: usize) -> usize;
    pub fn wxFFile_ReadAll(self_: *mut c_void, str: *mut c_void, conv: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFFile_Seek(self_: *mut c_void, ofs: wxFileOffset, mode: wxSeekMode) -> bool;
    // NOT_SUPPORTED: pub fn wxFFile_SeekEnd(self_: *mut c_void, ofs: wxFileOffset) -> bool;
    // NOT_SUPPORTED: pub fn wxFFile_Tell(self_: *const c_void) -> wxFileOffset;
    pub fn wxFFile_Write(self_: *mut c_void, str: *const c_void, conv: *const c_void) -> bool;
    pub fn wxFFile_Write1(self_: *mut c_void, buffer: *const c_void, count: usize) -> usize;
    pub fn wxFFile_fp(self_: *const c_void) -> *mut c_void;

    // wxFSFile
    pub fn wxFSFile_CLASSINFO() -> *mut c_void;
    pub fn wxFSFile_new(
        stream: *mut c_void,
        location: *const c_void,
        mimetype: *const c_void,
        anchor: *const c_void,
        modif: wxDateTime,
    ) -> *mut c_void;
    pub fn wxFSFile_DetachStream(self_: *mut c_void) -> *mut c_void;
    pub fn wxFSFile_GetAnchor(self_: *const c_void) -> *mut c_void;
    pub fn wxFSFile_GetLocation(self_: *const c_void) -> *mut c_void;
    pub fn wxFSFile_GetMimeType(self_: *const c_void) -> *mut c_void;
    pub fn wxFSFile_GetModificationTime(self_: *const c_void) -> *mut c_void;
    pub fn wxFSFile_GetStream(self_: *const c_void) -> *mut c_void;

    // wxFSVolume
    pub fn wxFSVolume_delete(self_: *mut c_void);
    pub fn wxFSVolume_new() -> *mut c_void;
    pub fn wxFSVolume_new1(name: *const c_void) -> *mut c_void;
    pub fn wxFSVolume_Create(self_: *mut c_void, name: *const c_void) -> bool;
    pub fn wxFSVolume_IsOk(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFSVolume_GetKind(self_: *const c_void) -> wxFSVolumeKind;
    pub fn wxFSVolume_GetFlags(self_: *const c_void) -> c_int;
    pub fn wxFSVolume_IsWritable(self_: *const c_void) -> bool;
    pub fn wxFSVolume_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxFSVolume_GetDisplayName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFSVolume_GetIcon(self_: *const c_void, type_: wxFSIconType) -> *mut c_void;
    pub fn wxFSVolume_GetVolumes(flags_set: c_int, flags_unset: c_int) -> *mut c_void;
    pub fn wxFSVolume_CancelSearch();

    // wxFile
    pub fn wxFile_delete(self_: *mut c_void);
    pub fn wxFile_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFile_new1(filename: *const c_void, mode: wxFile::OpenMode) -> *mut c_void;
    pub fn wxFile_new2(fd: c_int) -> *mut c_void;
    // DTOR: pub fn wxFile_~wxFile(self_: *mut c_void);
    pub fn wxFile_GetLastError(self_: *const c_void) -> c_int;
    pub fn wxFile_ClearLastError(self_: *mut c_void);
    pub fn wxFile_Attach(self_: *mut c_void, fd: c_int);
    pub fn wxFile_Close(self_: *mut c_void) -> bool;
    pub fn wxFile_Create(
        self_: *mut c_void,
        filename: *const c_void,
        overwrite: bool,
        access: c_int,
    ) -> bool;
    pub fn wxFile_Detach(self_: *mut c_void) -> c_int;
    pub fn wxFile_Eof(self_: *const c_void) -> bool;
    pub fn wxFile_Flush(self_: *mut c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFile_GetKind(self_: *const c_void) -> wxFileKind;
    pub fn wxFile_IsOpened(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFile_Length(self_: *const c_void) -> wxFileOffset;
    // NOT_SUPPORTED: pub fn wxFile_Open(self_: *mut c_void, filename: *const c_void, mode: wxFile::OpenMode, access: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxFile_Read(self_: *mut c_void, buffer: *mut c_void, count: usize) -> ssize_t;
    pub fn wxFile_ReadAll(self_: *mut c_void, str: *mut c_void, conv: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxFile_Seek(self_: *mut c_void, ofs: wxFileOffset, mode: wxSeekMode) -> wxFileOffset;
    // NOT_SUPPORTED: pub fn wxFile_SeekEnd(self_: *mut c_void, ofs: wxFileOffset) -> wxFileOffset;
    // NOT_SUPPORTED: pub fn wxFile_Tell(self_: *const c_void) -> wxFileOffset;
    pub fn wxFile_Write(self_: *mut c_void, buffer: *const c_void, count: usize) -> usize;
    pub fn wxFile_Write1(self_: *mut c_void, s: *const c_void, conv: *const c_void) -> bool;
    pub fn wxFile_fd(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxFile_Access(name: *const c_void, mode: wxFile::OpenMode) -> bool;
    pub fn wxFile_Exists(filename: *const c_void) -> bool;

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
    // NOT_SUPPORTED: pub fn wxFileName_GetAbsolutePath(self_: *const c_void, cwd: *const c_void, format: wxPathFormat) -> *mut c_void;
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

    // wxFileSystem
    pub fn wxFileSystem_CLASSINFO() -> *mut c_void;
    pub fn wxFileSystem_new() -> *mut c_void;
    pub fn wxFileSystem_ChangePathTo(self_: *mut c_void, location: *const c_void, is_dir: bool);
    pub fn wxFileSystem_FindFileInPath(
        self_: *mut c_void,
        p_str: *mut c_void,
        path: *const c_void,
        file: *const c_void,
    ) -> bool;
    pub fn wxFileSystem_FindFirst(
        self_: *mut c_void,
        wildcard: *const c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxFileSystem_FindNext(self_: *mut c_void) -> *mut c_void;
    pub fn wxFileSystem_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileSystem_OpenFile(
        self_: *mut c_void,
        location: *const c_void,
        flags: c_int,
    ) -> *mut c_void;
    pub fn wxFileSystem_AddHandler(handler: *mut c_void);
    pub fn wxFileSystem_RemoveHandler(handler: *mut c_void) -> *mut c_void;
    pub fn wxFileSystem_FileNameToURL(filename: *const c_void) -> *mut c_void;
    pub fn wxFileSystem_HasHandlerForPath(location: *const c_void) -> bool;
    pub fn wxFileSystem_URLToFileName(url: *const c_void) -> *mut c_void;

    // wxFileSystemWatcher
    pub fn wxFileSystemWatcher_CLASSINFO() -> *mut c_void;
    pub fn wxFileSystemWatcher_new() -> *mut c_void;
    // DTOR: pub fn wxFileSystemWatcher_~wxFileSystemWatcher(self_: *mut c_void);
    pub fn wxFileSystemWatcher_Add(self_: *mut c_void, path: *const c_void, events: c_int) -> bool;
    pub fn wxFileSystemWatcher_AddTree(
        self_: *mut c_void,
        path: *const c_void,
        events: c_int,
        filter: *const c_void,
    ) -> bool;
    pub fn wxFileSystemWatcher_Remove(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxFileSystemWatcher_RemoveTree(self_: *mut c_void, path: *const c_void) -> bool;
    pub fn wxFileSystemWatcher_RemoveAll(self_: *mut c_void) -> bool;
    pub fn wxFileSystemWatcher_GetWatchedPathsCount(self_: *const c_void) -> c_int;
    pub fn wxFileSystemWatcher_GetWatchedPaths(self_: *const c_void, paths: *mut c_void) -> c_int;
    pub fn wxFileSystemWatcher_SetOwner(self_: *mut c_void, handler: *mut c_void);

    // wxFileSystemWatcherEvent
    pub fn wxFileSystemWatcherEvent_CLASSINFO() -> *mut c_void;
    pub fn wxFileSystemWatcherEvent_new(change_type: c_int, watchid: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileSystemWatcherEvent_new1(change_type: c_int, warning_type: wxFSWWarningType, error_msg: *const c_void, watchid: c_int) -> *mut c_void;
    pub fn wxFileSystemWatcherEvent_new2(
        change_type: c_int,
        path: *const c_void,
        new_path: *const c_void,
        watchid: c_int,
    ) -> *mut c_void;
    pub fn wxFileSystemWatcherEvent_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileSystemWatcherEvent_GetNewPath(self_: *const c_void) -> *mut c_void;
    pub fn wxFileSystemWatcherEvent_GetChangeType(self_: *const c_void) -> c_int;
    pub fn wxFileSystemWatcherEvent_IsError(self_: *const c_void) -> bool;
    pub fn wxFileSystemWatcherEvent_GetErrorDescription(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFileSystemWatcherEvent_GetWarningType(self_: *const c_void) -> wxFSWWarningType;
    pub fn wxFileSystemWatcherEvent_ToString(self_: *const c_void) -> *mut c_void;

    // wxFileType
    pub fn wxFileType_delete(self_: *mut c_void);
    pub fn wxFileType_new(ft_info: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxFileType_~wxFileType(self_: *mut c_void);
    pub fn wxFileType_GetDescription(self_: *const c_void, desc: *mut c_void) -> bool;
    pub fn wxFileType_GetExtensions(self_: *mut c_void, extensions: *mut c_void) -> bool;
    pub fn wxFileType_GetIcon(self_: *const c_void, icon_loc: *mut c_void) -> bool;
    pub fn wxFileType_GetMimeType(self_: *const c_void, mime_type: *mut c_void) -> bool;
    pub fn wxFileType_GetMimeTypes(self_: *const c_void, mime_types: *mut c_void) -> bool;
    pub fn wxFileType_GetOpenCommand(
        self_: *mut c_void,
        command: *mut c_void,
        params: *const c_void,
    ) -> bool;
    pub fn wxFileType_GetOpenCommand1(self_: *const c_void, filename: *const c_void)
        -> *mut c_void;
    pub fn wxFileType_GetPrintCommand(
        self_: *const c_void,
        command: *mut c_void,
        params: *const c_void,
    ) -> bool;
    pub fn wxFileType_GetExpandedCommand(
        self_: *const c_void,
        verb: *const c_void,
        params: *const c_void,
    ) -> *mut c_void;
    pub fn wxFileType_GetAllCommands(
        self_: *const c_void,
        verbs: *mut c_void,
        commands: *mut c_void,
        params: *const c_void,
    ) -> usize;
    pub fn wxFileType_ExpandCommand(command: *const c_void, params: *const c_void) -> *mut c_void;

    // wxFilterClassFactory
    pub fn wxFilterClassFactory_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFilterClassFactory_CanHandle(self_: *const c_void, protocol: *const c_void, type_: wxStreamProtocolType) -> bool;
    pub fn wxFilterClassFactory_GetNext(self_: *const c_void) -> *mut c_void;
    pub fn wxFilterClassFactory_GetProtocol(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxFilterClassFactory_GetProtocols(self_: *const c_void, type_: wxStreamProtocolType) -> *const c_void;
    pub fn wxFilterClassFactory_NewStream(self_: *const c_void, stream: *mut c_void)
        -> *mut c_void;
    pub fn wxFilterClassFactory_NewStream1(
        self_: *const c_void,
        stream: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFilterClassFactory_NewStream2(
        self_: *const c_void,
        stream: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFilterClassFactory_NewStream3(
        self_: *const c_void,
        stream: *mut c_void,
    ) -> *mut c_void;
    pub fn wxFilterClassFactory_PopExtension(
        self_: *const c_void,
        location: *const c_void,
    ) -> *mut c_void;
    pub fn wxFilterClassFactory_PushFront(self_: *mut c_void);
    pub fn wxFilterClassFactory_Remove(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxFilterClassFactory_Find(protocol: *const c_void, type_: wxStreamProtocolType) -> *mut c_void;
    pub fn wxFilterClassFactory_GetFirst() -> *mut c_void;

    // wxHashTable
    pub fn wxHashTable_CLASSINFO() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxHashTable_new(key_type: wxKeyType, size: usize) -> *mut c_void;
    // DTOR: pub fn wxHashTable_~wxHashTable(self_: *mut c_void);
    pub fn wxHashTable_BeginFind(self_: *mut c_void);
    pub fn wxHashTable_Clear(self_: *mut c_void);
    pub fn wxHashTable_Delete(self_: *mut c_void, key: c_long) -> *mut c_void;
    pub fn wxHashTable_Delete1(self_: *mut c_void, key: *const c_void) -> *mut c_void;
    pub fn wxHashTable_DeleteContents(self_: *mut c_void, flag: bool);
    pub fn wxHashTable_Get(self_: *mut c_void, key: c_long) -> *mut c_void;
    pub fn wxHashTable_Get1(self_: *mut c_void, key: *const c_void) -> *mut c_void;
    pub fn wxHashTable_GetCount(self_: *const c_void) -> usize;
    pub fn wxHashTable_Next(self_: *mut c_void) -> *mut c_void;
    pub fn wxHashTable_Put(self_: *mut c_void, key: c_long, object: *mut c_void);
    pub fn wxHashTable_Put1(self_: *mut c_void, key: *const c_void, object: *mut c_void);
    pub fn wxHashTable_MakeKey(string: *const c_void) -> c_long;

    // wxIconLocation
    pub fn wxIconLocation_delete(self_: *mut c_void);
    pub fn wxIconLocation_IsOk(self_: *const c_void) -> bool;
    pub fn wxIconLocation_SetFileName(self_: *mut c_void, filename: *const c_void);
    pub fn wxIconLocation_GetFileName(self_: *const c_void) -> *mut c_void;

    // wxIdleEvent
    pub fn wxIdleEvent_CLASSINFO() -> *mut c_void;
    pub fn wxIdleEvent_new() -> *mut c_void;
    pub fn wxIdleEvent_MoreRequested(self_: *const c_void) -> bool;
    pub fn wxIdleEvent_RequestMore(self_: *mut c_void, need_more: bool);
    // NOT_SUPPORTED: pub fn wxIdleEvent_GetMode() -> wxIdleMode;
    // NOT_SUPPORTED: pub fn wxIdleEvent_SetMode(mode: wxIdleMode);

    // wxInitializer
    pub fn wxInitializer_delete(self_: *mut c_void);
    pub fn wxInitializer_new(argc: c_int, argv: *mut c_void) -> *mut c_void;
    pub fn wxInitializer_IsOk(self_: *const c_void) -> bool;
    // DTOR: pub fn wxInitializer_~wxInitializer(self_: *mut c_void);

    // wxLocale
    pub fn wxLocale_delete(self_: *mut c_void);
    pub fn wxLocale_new() -> *mut c_void;
    pub fn wxLocale_new1(language: c_int, flags: c_int) -> *mut c_void;
    pub fn wxLocale_new2(
        name: *const c_void,
        short_name: *const c_void,
        locale: *const c_void,
        b_load_default: bool,
    ) -> *mut c_void;
    // DTOR: pub fn wxLocale_~wxLocale(self_: *mut c_void);
    pub fn wxLocale_AddCatalog(self_: *mut c_void, domain: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxLocale_AddCatalog1(self_: *mut c_void, domain: *const c_void, msg_id_language: wxLanguage) -> bool;
    // NOT_SUPPORTED: pub fn wxLocale_AddCatalog2(self_: *mut c_void, domain: *const c_void, msg_id_language: wxLanguage, msg_id_charset: *const c_void) -> bool;
    pub fn wxLocale_GetCanonicalName(self_: *const c_void) -> *mut c_void;
    pub fn wxLocale_GetHeaderValue(
        self_: *const c_void,
        header: *const c_void,
        domain: *const c_void,
    ) -> *mut c_void;
    pub fn wxLocale_GetLanguage(self_: *const c_void) -> c_int;
    pub fn wxLocale_GetLocale(self_: *const c_void) -> *mut c_void;
    pub fn wxLocale_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxLocale_GetString(
        self_: *const c_void,
        orig_string: *const c_void,
        domain: *const c_void,
    ) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxLocale_GetString1(self_: *const c_void, orig_string: *const c_void, orig_string2: *const c_void, n: unsigned, domain: *const c_void) -> *mut c_void;
    pub fn wxLocale_GetSysName(self_: *const c_void) -> *mut c_void;
    pub fn wxLocale_Init(self_: *mut c_void, language: c_int, flags: c_int) -> bool;
    pub fn wxLocale_Init1(
        self_: *mut c_void,
        name: *const c_void,
        short_name: *const c_void,
        locale: *const c_void,
        b_load_default: bool,
    ) -> bool;
    pub fn wxLocale_IsLoaded(self_: *const c_void, domain: *const c_void) -> bool;
    pub fn wxLocale_IsOk(self_: *const c_void) -> bool;
    pub fn wxLocale_AddCatalogLookupPathPrefix(prefix: *const c_void);
    pub fn wxLocale_AddLanguage(info: *const c_void);
    pub fn wxLocale_FindLanguageInfo(locale: *const c_void) -> *const c_void;
    pub fn wxLocale_GetLanguageInfo(lang: c_int) -> *const c_void;
    pub fn wxLocale_GetLanguageName(lang: c_int) -> *mut c_void;
    pub fn wxLocale_GetLanguageCanonicalName(lang: c_int) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxLocale_GetSystemEncoding() -> wxFontEncoding;
    pub fn wxLocale_GetSystemEncodingName() -> *mut c_void;
    pub fn wxLocale_GetSystemLanguage() -> c_int;
    // NOT_SUPPORTED: pub fn wxLocale_GetInfo(index: wxLocaleInfo, cat: wxLocaleCategory) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxLocale_GetOSInfo(index: wxLocaleInfo, cat: wxLocaleCategory) -> *mut c_void;
    pub fn wxLocale_IsAvailable(lang: c_int) -> bool;

    // wxLog
    pub fn wxLog_delete(self_: *mut c_void);
    pub fn wxLog_AddTraceMask(mask: *const c_void);
    pub fn wxLog_ClearTraceMasks();
    pub fn wxLog_GetTraceMasks() -> *mut c_void;
    pub fn wxLog_IsAllowedTraceMask(mask: *const c_void) -> bool;
    pub fn wxLog_RemoveTraceMask(mask: *const c_void);
    pub fn wxLog_DontCreateOnDemand();
    pub fn wxLog_GetActiveTarget() -> *mut c_void;
    pub fn wxLog_SetActiveTarget(logtarget: *mut c_void) -> *mut c_void;
    pub fn wxLog_SetThreadActiveTarget(logger: *mut c_void) -> *mut c_void;
    pub fn wxLog_FlushActive();
    pub fn wxLog_Resume();
    pub fn wxLog_Suspend();
    // NOT_SUPPORTED: pub fn wxLog_GetLogLevel() -> wxLogLevel;
    // NOT_SUPPORTED: pub fn wxLog_IsLevelEnabled(level: wxLogLevel, component: wxString) -> bool;
    // NOT_SUPPORTED: pub fn wxLog_SetComponentLevel(component: *const c_void, level: wxLogLevel);
    // NOT_SUPPORTED: pub fn wxLog_SetLogLevel(log_level: wxLogLevel);
    pub fn wxLog_EnableLogging(enable: bool) -> bool;
    pub fn wxLog_IsEnabled() -> bool;
    pub fn wxLog_GetRepetitionCounting() -> bool;
    pub fn wxLog_SetRepetitionCounting(repet_counting: bool);
    pub fn wxLog_GetTimestamp() -> *mut c_void;
    pub fn wxLog_SetTimestamp(format: *const c_void);
    pub fn wxLog_DisableTimestamp();
    pub fn wxLog_GetVerbose() -> bool;
    pub fn wxLog_SetVerbose(verbose: bool);
    pub fn wxLog_SetFormatter(self_: *mut c_void, formatter: *mut c_void) -> *mut c_void;
    pub fn wxLog_Flush(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxLog_LogRecord(self_: *mut c_void, level: wxLogLevel, msg: *const c_void, info: *const c_void);

    // wxLogBuffer
    pub fn wxLogBuffer_delete(self_: *mut c_void);
    pub fn wxLogBuffer_new() -> *mut c_void;
    pub fn wxLogBuffer_GetBuffer(self_: *const c_void) -> *mut c_void;

    // wxLogChain
    pub fn wxLogChain_delete(self_: *mut c_void);
    pub fn wxLogChain_new(logger: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxLogChain_~wxLogChain(self_: *mut c_void);
    pub fn wxLogChain_DetachOldLog(self_: *mut c_void);
    pub fn wxLogChain_GetOldLog(self_: *const c_void) -> *mut c_void;
    pub fn wxLogChain_IsPassingMessages(self_: *const c_void) -> bool;
    pub fn wxLogChain_PassMessages(self_: *mut c_void, pass_messages: bool);
    pub fn wxLogChain_SetLog(self_: *mut c_void, logger: *mut c_void);

    // wxLogFormatter
    pub fn wxLogFormatter_delete(self_: *mut c_void);
    pub fn wxLogFormatter_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxLogFormatter_Format(self_: *const c_void, level: wxLogLevel, msg: *const c_void, info: *const c_void) -> *mut c_void;

    // wxLogInterposer
    pub fn wxLogInterposer_delete(self_: *mut c_void);
    pub fn wxLogInterposer_new() -> *mut c_void;

    // wxLogInterposerTemp
    pub fn wxLogInterposerTemp_delete(self_: *mut c_void);
    pub fn wxLogInterposerTemp_new() -> *mut c_void;

    // wxLogNull
    pub fn wxLogNull_delete(self_: *mut c_void);
    pub fn wxLogNull_new() -> *mut c_void;
    // DTOR: pub fn wxLogNull_~wxLogNull(self_: *mut c_void);

    // wxLogStderr
    pub fn wxLogStderr_delete(self_: *mut c_void);
    pub fn wxLogStderr_new(fp: *mut c_void, conv: *const c_void) -> *mut c_void;

    // wxLogStream
    pub fn wxLogStream_delete(self_: *mut c_void);
    pub fn wxLogStream_new(ostr: *mut c_void, conv: *const c_void) -> *mut c_void;

    // wxMemoryBuffer
    pub fn wxMemoryBuffer_delete(self_: *mut c_void);
    pub fn wxMemoryBuffer_new(src: *const c_void) -> *mut c_void;
    pub fn wxMemoryBuffer_new1(size: usize) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMemoryBuffer_AppendByte(self_: *mut c_void, data: char);
    pub fn wxMemoryBuffer_AppendData(self_: *mut c_void, data: *const c_void, len: usize);
    pub fn wxMemoryBuffer_Clear(self_: *mut c_void);
    pub fn wxMemoryBuffer_GetAppendBuf(self_: *mut c_void, size_needed: usize) -> *mut c_void;
    pub fn wxMemoryBuffer_GetBufSize(self_: *const c_void) -> usize;
    pub fn wxMemoryBuffer_GetData(self_: *const c_void) -> *mut c_void;
    pub fn wxMemoryBuffer_GetDataLen(self_: *const c_void) -> usize;
    pub fn wxMemoryBuffer_GetWriteBuf(self_: *mut c_void, size_needed: usize) -> *mut c_void;
    pub fn wxMemoryBuffer_IsEmpty(self_: *const c_void) -> bool;
    pub fn wxMemoryBuffer_SetBufSize(self_: *mut c_void, size: usize);
    pub fn wxMemoryBuffer_SetDataLen(self_: *mut c_void, size: usize);
    pub fn wxMemoryBuffer_UngetAppendBuf(self_: *mut c_void, size_used: usize);
    pub fn wxMemoryBuffer_UngetWriteBuf(self_: *mut c_void, size_used: usize);

    // wxMessageOutput
    pub fn wxMessageOutput_delete(self_: *mut c_void);
    pub fn wxMessageOutput_Get() -> *mut c_void;
    pub fn wxMessageOutput_Set(msgout: *mut c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxMessageOutput_Printf(self_: *mut c_void, format: *const c_void, None: ...);
    pub fn wxMessageOutput_Output(self_: *mut c_void, str: *const c_void);

    // wxMessageOutputBest
    pub fn wxMessageOutputBest_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxMessageOutputBest_new(flags: wxMessageOutputFlags) -> *mut c_void;

    // wxMessageOutputDebug
    pub fn wxMessageOutputDebug_delete(self_: *mut c_void);
    pub fn wxMessageOutputDebug_new() -> *mut c_void;

    // wxMessageOutputStderr
    pub fn wxMessageOutputStderr_delete(self_: *mut c_void);
    pub fn wxMessageOutputStderr_new(fp: *mut c_void) -> *mut c_void;

    // wxMimeTypesManager
    pub fn wxMimeTypesManager_delete(self_: *mut c_void);
    pub fn wxMimeTypesManager_new() -> *mut c_void;
    // DTOR: pub fn wxMimeTypesManager_~wxMimeTypesManager(self_: *mut c_void);
    pub fn wxMimeTypesManager_AddFallbacks(self_: *mut c_void, fallbacks: *const c_void);
    pub fn wxMimeTypesManager_GetFileTypeFromExtension(
        self_: *mut c_void,
        extension: *const c_void,
    ) -> *mut c_void;
    pub fn wxMimeTypesManager_GetFileTypeFromMimeType(
        self_: *mut c_void,
        mime_type: *const c_void,
    ) -> *mut c_void;
    pub fn wxMimeTypesManager_Associate(self_: *mut c_void, ft_info: *const c_void) -> *mut c_void;
    pub fn wxMimeTypesManager_Unassociate(self_: *mut c_void, ft: *mut c_void) -> bool;
    pub fn wxMimeTypesManager_EnumAllFileTypes(self_: *mut c_void, mimetypes: *mut c_void)
        -> usize;
    pub fn wxMimeTypesManager_IsOfType(mime_type: *const c_void, wildcard: *const c_void) -> bool;

    // wxMutex
    pub fn wxMutex_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxMutex_new(type_: wxMutexType) -> *mut c_void;
    // DTOR: pub fn wxMutex_~wxMutex(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxMutex_Lock(self_: *mut c_void) -> wxMutexError;
    // NOT_SUPPORTED: pub fn wxMutex_LockTimeout(self_: *mut c_void, msec: unsigned long) -> wxMutexError;
    // NOT_SUPPORTED: pub fn wxMutex_TryLock(self_: *mut c_void) -> wxMutexError;
    // NOT_SUPPORTED: pub fn wxMutex_Unlock(self_: *mut c_void) -> wxMutexError;

    // wxMutexLocker
    pub fn wxMutexLocker_delete(self_: *mut c_void);
    pub fn wxMutexLocker_new(mutex: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxMutexLocker_~wxMutexLocker(self_: *mut c_void);
    pub fn wxMutexLocker_IsOk(self_: *const c_void) -> bool;

    // wxObject
    pub fn wxObject_CLASSINFO() -> *mut c_void;
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

    // wxPlatformInfo
    pub fn wxPlatformInfo_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetArchitecture(self_: *const c_void) -> wxArchitecture;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetBitness(self_: *const c_void) -> wxBitness;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetEndianness(self_: *const c_void) -> wxEndianness;
    pub fn wxPlatformInfo_GetCpuArchitectureName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetNativeCpuArchitectureName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetOSMajorVersion(self_: *const c_void) -> c_int;
    pub fn wxPlatformInfo_GetOSMinorVersion(self_: *const c_void) -> c_int;
    pub fn wxPlatformInfo_GetOSMicroVersion(self_: *const c_void) -> c_int;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetOperatingSystemId(self_: *const c_void) -> wxOperatingSystemId;
    pub fn wxPlatformInfo_GetOperatingSystemDescription(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetPortId(self_: *const c_void) -> wxPortId;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetLinuxDistributionInfo(self_: *const c_void) -> wxLinuxDistributionInfo;
    pub fn wxPlatformInfo_GetDesktopEnvironment(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetToolkitMajorVersion(self_: *const c_void) -> c_int;
    pub fn wxPlatformInfo_GetToolkitMinorVersion(self_: *const c_void) -> c_int;
    pub fn wxPlatformInfo_GetToolkitMicroVersion(self_: *const c_void) -> c_int;
    pub fn wxPlatformInfo_GetArchName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetBitnessName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetEndiannessName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetOperatingSystemFamilyName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetOperatingSystemIdName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetPortIdName(self_: *const c_void) -> *mut c_void;
    pub fn wxPlatformInfo_GetPortIdShortName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_SetArchitecture(self_: *mut c_void, n: wxArchitecture);
    // NOT_SUPPORTED: pub fn wxPlatformInfo_SetBitness(self_: *mut c_void, n: wxBitness);
    // NOT_SUPPORTED: pub fn wxPlatformInfo_SetEndianness(self_: *mut c_void, n: wxEndianness);
    pub fn wxPlatformInfo_SetOSVersion(
        self_: *mut c_void,
        major: c_int,
        minor: c_int,
        micro: c_int,
    );
    // NOT_SUPPORTED: pub fn wxPlatformInfo_SetOperatingSystemId(self_: *mut c_void, n: wxOperatingSystemId);
    // NOT_SUPPORTED: pub fn wxPlatformInfo_SetPortId(self_: *mut c_void, n: wxPortId);
    pub fn wxPlatformInfo_SetToolkitVersion(
        self_: *mut c_void,
        major: c_int,
        minor: c_int,
        micro: c_int,
    );
    pub fn wxPlatformInfo_SetOperatingSystemDescription(self_: *mut c_void, desc: *const c_void);
    pub fn wxPlatformInfo_SetDesktopEnvironment(self_: *mut c_void, de: *const c_void);
    pub fn wxPlatformInfo_SetLinuxDistributionInfo(self_: *mut c_void, di: *const c_void);
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetBitness1(bitness: *const c_void) -> wxBitness;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetArch(arch: *const c_void) -> wxArchitecture;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetEndianness1(end: *const c_void) -> wxEndianness;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetOperatingSystemId1(name: *const c_void) -> wxOperatingSystemId;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetPortId1(portname: *const c_void) -> wxPortId;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetArchName1(arch: wxArchitecture) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetBitnessName1(bitness: wxBitness) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetEndiannessName1(end: wxEndianness) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetOperatingSystemFamilyName1(os: wxOperatingSystemId) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetOperatingSystemIdName1(os: wxOperatingSystemId) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetPortIdName1(port: wxPortId, using_universal: bool) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_GetPortIdShortName1(port: wxPortId, using_universal: bool) -> *mut c_void;
    pub fn wxPlatformInfo_GetOperatingSystemDirectory() -> *mut c_void;
    pub fn wxPlatformInfo_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxPlatformInfo_new1(pid: wxPortId, tk_major: c_int, tk_minor: c_int, id: wxOperatingSystemId, os_major: c_int, os_minor: c_int, bitness: wxBitness, endian: wxEndianness) -> *mut c_void;
    pub fn wxPlatformInfo_CheckOSVersion(
        self_: *const c_void,
        major: c_int,
        minor: c_int,
        micro: c_int,
    ) -> bool;
    pub fn wxPlatformInfo_CheckToolkitVersion(
        self_: *const c_void,
        major: c_int,
        minor: c_int,
        micro: c_int,
    ) -> bool;
    pub fn wxPlatformInfo_IsOk(self_: *const c_void) -> bool;
    pub fn wxPlatformInfo_IsUsingUniversalWidgets(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxPlatformInfo_operator!=(self_: *const c_void, t: *const c_void) -> bool;
    // BLOCKED: pub fn wxPlatformInfo_operator==(self_: *const c_void, t: *const c_void) -> bool;
    pub fn wxPlatformInfo_Get() -> *mut c_void;

    // wxPosition
    pub fn wxPosition_delete(self_: *mut c_void);
    // BLOCKED: pub fn wxPosition_operator==(self_: *const c_void, pos: *const c_void) -> bool;
    // BLOCKED: pub fn wxPosition_operator!=(self_: *const c_void, pos: *const c_void) -> bool;
    // BLOCKED: pub fn wxPosition_operator+=(self_: *mut c_void, pos: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPosition_operator-=(self_: *mut c_void, pos: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPosition_operator+=1(self_: *mut c_void, size: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPosition_operator-=1(self_: *mut c_void, size: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxPosition_operator+(self_: *const c_void, pos: *const c_void) -> wxPosition;
    // BLOCKED: pub fn wxPosition_operator-(self_: *const c_void, pos: *const c_void) -> wxPosition;
    // BLOCKED: pub fn wxPosition_operator+1(self_: *const c_void, size: *const c_void) -> wxPosition;
    // BLOCKED: pub fn wxPosition_operator-1(self_: *const c_void, size: *const c_void) -> wxPosition;
    pub fn wxPosition_new() -> *mut c_void;
    pub fn wxPosition_new1(row: c_int, col: c_int) -> *mut c_void;
    pub fn wxPosition_GetCol(self_: *const c_void) -> c_int;
    pub fn wxPosition_GetColumn(self_: *const c_void) -> c_int;
    pub fn wxPosition_GetRow(self_: *const c_void) -> c_int;
    pub fn wxPosition_SetCol(self_: *mut c_void, column: c_int);
    pub fn wxPosition_SetColumn(self_: *mut c_void, column: c_int);
    pub fn wxPosition_SetRow(self_: *mut c_void, row: c_int);

    // wxPostScriptDC
    pub fn wxPostScriptDC_CLASSINFO() -> *mut c_void;
    pub fn wxPostScriptDC_new() -> *mut c_void;
    pub fn wxPostScriptDC_new1(print_data: *const c_void) -> *mut c_void;

    // wxPowerResource
    pub fn wxPowerResource_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPowerResource_Acquire(kind: wxPowerResourceKind, reason: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxPowerResource_Release(kind: wxPowerResourceKind);

    // wxPowerResourceBlocker
    pub fn wxPowerResourceBlocker_delete(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxPowerResourceBlocker_new(kind: wxPowerResourceKind, reason: *const c_void) -> *mut c_void;
    pub fn wxPowerResourceBlocker_IsInEffect(self_: *const c_void) -> bool;
    // DTOR: pub fn wxPowerResourceBlocker_~wxPowerResourceBlocker(self_: *mut c_void);

    // wxProcess
    pub fn wxProcess_CLASSINFO() -> *mut c_void;
    pub fn wxProcess_new(parent: *mut c_void, id: c_int) -> *mut c_void;
    pub fn wxProcess_new1(flags: c_int) -> *mut c_void;
    // DTOR: pub fn wxProcess_~wxProcess(self_: *mut c_void);
    pub fn wxProcess_Activate(self_: *const c_void) -> bool;
    pub fn wxProcess_CloseOutput(self_: *mut c_void);
    pub fn wxProcess_Detach(self_: *mut c_void);
    pub fn wxProcess_GetErrorStream(self_: *const c_void) -> *mut c_void;
    pub fn wxProcess_GetInputStream(self_: *const c_void) -> *mut c_void;
    pub fn wxProcess_GetOutputStream(self_: *const c_void) -> *mut c_void;
    pub fn wxProcess_GetPid(self_: *const c_void) -> c_long;
    pub fn wxProcess_IsErrorAvailable(self_: *const c_void) -> bool;
    pub fn wxProcess_IsInputAvailable(self_: *const c_void) -> bool;
    pub fn wxProcess_IsInputOpened(self_: *const c_void) -> bool;
    pub fn wxProcess_OnTerminate(self_: *mut c_void, pid: c_int, status: c_int);
    pub fn wxProcess_Redirect(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxProcess_SetPriority(self_: *mut c_void, priority: unsigned);
    pub fn wxProcess_Exists(pid: c_int) -> bool;
    // NOT_SUPPORTED: pub fn wxProcess_Kill(pid: c_int, sig: wxSignal, flags: c_int) -> wxKillError;
    pub fn wxProcess_Open(cmd: *const c_void, flags: c_int) -> *mut c_void;

    // wxProcessEvent
    pub fn wxProcessEvent_CLASSINFO() -> *mut c_void;
    pub fn wxProcessEvent_new(id: c_int, pid: c_int, exitcode: c_int) -> *mut c_void;
    pub fn wxProcessEvent_GetExitCode(self_: *const c_void) -> c_int;
    pub fn wxProcessEvent_GetPid(self_: *const c_void) -> c_int;

    // wxRecursionGuard
    pub fn wxRecursionGuard_delete(self_: *mut c_void);
    pub fn wxRecursionGuard_new(flag: *mut c_void) -> *mut c_void;
    // DTOR: pub fn wxRecursionGuard_~wxRecursionGuard(self_: *mut c_void);
    pub fn wxRecursionGuard_IsInside(self_: *const c_void) -> bool;

    // wxRecursionGuardFlag
    pub fn wxRecursionGuardFlag_delete(self_: *mut c_void);

    // wxRegEx
    pub fn wxRegEx_delete(self_: *mut c_void);
    pub fn wxRegEx_new() -> *mut c_void;
    pub fn wxRegEx_new1(expr: *const c_void, flags: c_int) -> *mut c_void;
    // DTOR: pub fn wxRegEx_~wxRegEx(self_: *mut c_void);
    pub fn wxRegEx_Compile(self_: *mut c_void, pattern: *const c_void, flags: c_int) -> bool;
    pub fn wxRegEx_GetMatch(
        self_: *const c_void,
        start: *mut c_void,
        len: *mut c_void,
        index: usize,
    ) -> bool;
    pub fn wxRegEx_GetMatch1(
        self_: *const c_void,
        text: *const c_void,
        index: usize,
    ) -> *mut c_void;
    pub fn wxRegEx_GetMatchCount(self_: *const c_void) -> usize;
    pub fn wxRegEx_IsValid(self_: *const c_void) -> bool;
    pub fn wxRegEx_Matches(self_: *const c_void, text: *const c_void, flags: c_int) -> bool;
    pub fn wxRegEx_Matches1(
        self_: *const c_void,
        text: *const c_void,
        flags: c_int,
        len: usize,
    ) -> bool;
    pub fn wxRegEx_Matches2(self_: *const c_void, text: *const c_void, flags: c_int) -> bool;
    pub fn wxRegEx_Replace(
        self_: *const c_void,
        text: *mut c_void,
        replacement: *const c_void,
        max_matches: usize,
    ) -> c_int;
    pub fn wxRegEx_ReplaceAll(
        self_: *const c_void,
        text: *mut c_void,
        replacement: *const c_void,
    ) -> c_int;
    pub fn wxRegEx_ReplaceFirst(
        self_: *const c_void,
        text: *mut c_void,
        replacement: *const c_void,
    ) -> c_int;
    pub fn wxRegEx_QuoteMeta(str: *const c_void) -> *mut c_void;
    pub fn wxRegEx_ConvertFromBasic(bre: *const c_void) -> *mut c_void;
    pub fn wxRegEx_GetLibraryVersionInfo() -> *mut c_void;

    // wxSecretStore
    pub fn wxSecretStore_delete(self_: *mut c_void);
    pub fn wxSecretStore_GetDefault() -> *mut c_void;
    pub fn wxSecretStore_IsOk(self_: *const c_void, errmsg: *mut c_void) -> bool;
    pub fn wxSecretStore_Save(
        self_: *mut c_void,
        service: *const c_void,
        username: *const c_void,
        password: *const c_void,
    ) -> bool;
    pub fn wxSecretStore_Load(
        self_: *const c_void,
        service: *const c_void,
        username: *mut c_void,
        password: *mut c_void,
    ) -> bool;
    pub fn wxSecretStore_Delete(self_: *mut c_void, service: *const c_void) -> bool;

    // wxSecretValue
    pub fn wxSecretValue_delete(self_: *mut c_void);
    pub fn wxSecretValue_new() -> *mut c_void;
    pub fn wxSecretValue_new1(size: usize, data: *const c_void) -> *mut c_void;
    pub fn wxSecretValue_new2(secret: *const c_void) -> *mut c_void;
    pub fn wxSecretValue_new3(other: *const c_void) -> *mut c_void;
    // BLOCKED: pub fn wxSecretValue_operator=(self_: *mut c_void, other: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxSecretValue_~wxSecretValue(self_: *mut c_void);
    pub fn wxSecretValue_IsOk(self_: *const c_void) -> bool;
    // BLOCKED: pub fn wxSecretValue_operator==(self_: *const c_void, other: *const c_void) -> bool;
    // BLOCKED: pub fn wxSecretValue_operator!=(self_: *const c_void, other: *const c_void) -> bool;
    pub fn wxSecretValue_GetSize(self_: *const c_void) -> usize;
    pub fn wxSecretValue_GetData(self_: *const c_void) -> *const c_void;
    pub fn wxSecretValue_GetAsString(self_: *const c_void, conv: *const c_void) -> *mut c_void;
    pub fn wxSecretValue_Wipe(size: usize, data: *mut c_void);
    pub fn wxSecretValue_WipeString(str: *mut c_void);

    // wxSemaphore
    pub fn wxSemaphore_delete(self_: *mut c_void);
    pub fn wxSemaphore_new(initialcount: c_int, maxcount: c_int) -> *mut c_void;
    // DTOR: pub fn wxSemaphore_~wxSemaphore(self_: *mut c_void);
    // NOT_SUPPORTED: pub fn wxSemaphore_Post(self_: *mut c_void) -> wxSemaError;
    // NOT_SUPPORTED: pub fn wxSemaphore_TryWait(self_: *mut c_void) -> wxSemaError;
    // NOT_SUPPORTED: pub fn wxSemaphore_Wait(self_: *mut c_void) -> wxSemaError;
    // NOT_SUPPORTED: pub fn wxSemaphore_WaitTimeout(self_: *mut c_void, timeout_millis: unsigned long) -> wxSemaError;

    // wxServer
    pub fn wxServer_delete(self_: *mut c_void);
    pub fn wxServer_new() -> *mut c_void;
    pub fn wxServer_Create(self_: *mut c_void, service: *const c_void) -> bool;
    pub fn wxServer_OnAcceptConnection(self_: *mut c_void, topic: *const c_void) -> *mut c_void;

    // wxSharedClientDataContainer
    pub fn wxSharedClientDataContainer_delete(self_: *mut c_void);
    pub fn wxSharedClientDataContainer_GetClientData(self_: *const c_void) -> *mut c_void;
    pub fn wxSharedClientDataContainer_GetClientObject(self_: *const c_void) -> *mut c_void;
    pub fn wxSharedClientDataContainer_SetClientData(self_: *mut c_void, data: *mut c_void);
    pub fn wxSharedClientDataContainer_SetClientObject(self_: *mut c_void, data: *mut c_void);

    // wxSingleInstanceChecker
    pub fn wxSingleInstanceChecker_delete(self_: *mut c_void);
    pub fn wxSingleInstanceChecker_new() -> *mut c_void;
    pub fn wxSingleInstanceChecker_new1(name: *const c_void, path: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxSingleInstanceChecker_~wxSingleInstanceChecker(self_: *mut c_void);
    pub fn wxSingleInstanceChecker_Create(
        self_: *mut c_void,
        name: *const c_void,
        path: *const c_void,
    ) -> bool;
    pub fn wxSingleInstanceChecker_CreateDefault(self_: *mut c_void) -> bool;
    pub fn wxSingleInstanceChecker_IsAnotherRunning(self_: *const c_void) -> bool;

    // wxStackFrame
    pub fn wxStackFrame_delete(self_: *mut c_void);
    pub fn wxStackFrame_GetAddress(self_: *const c_void) -> *mut c_void;
    pub fn wxStackFrame_GetFileName(self_: *const c_void) -> *mut c_void;
    pub fn wxStackFrame_GetLevel(self_: *const c_void) -> usize;
    pub fn wxStackFrame_GetLine(self_: *const c_void) -> usize;
    pub fn wxStackFrame_GetModule(self_: *const c_void) -> *mut c_void;
    pub fn wxStackFrame_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxStackFrame_GetOffset(self_: *const c_void) -> usize;
    pub fn wxStackFrame_GetParam(
        self_: *const c_void,
        n: usize,
        type_: *mut c_void,
        name: *mut c_void,
        value: *mut c_void,
    ) -> bool;
    pub fn wxStackFrame_GetParamCount(self_: *const c_void) -> usize;
    pub fn wxStackFrame_HasSourceLocation(self_: *const c_void) -> bool;

    // wxStandardPaths
    pub fn wxStandardPaths_delete(self_: *mut c_void);
    pub fn wxStandardPaths_DontIgnoreAppSubDir(self_: *mut c_void);
    pub fn wxStandardPaths_GetAppDocumentsDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetConfigDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetDataDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetDocumentsDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetExecutablePath(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetInstallPrefix(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetLocalDataDir(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxStandardPaths_GetLocalizedResourcesDir(self_: *const c_void, lang: *const c_void, category: ResourceCat) -> *mut c_void;
    pub fn wxStandardPaths_GetPluginsDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetResourcesDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetTempDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetUserConfigDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_GetUserDataDir(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxStandardPaths_GetUserDir(self_: *const c_void, user_dir: Dir) -> *mut c_void;
    pub fn wxStandardPaths_GetUserLocalDataDir(self_: *const c_void) -> *mut c_void;
    pub fn wxStandardPaths_IgnoreAppSubDir(self_: *mut c_void, subdir_pattern: *const c_void);
    pub fn wxStandardPaths_IgnoreAppBuildSubDirs(self_: *mut c_void);
    pub fn wxStandardPaths_SetInstallPrefix(self_: *mut c_void, prefix: *const c_void);
    pub fn wxStandardPaths_UseAppInfo(self_: *mut c_void, info: c_int);
    // NOT_SUPPORTED: pub fn wxStandardPaths_SetFileLayout(self_: *mut c_void, layout: FileLayout);
    // NOT_SUPPORTED: pub fn wxStandardPaths_GetFileLayout(self_: *const c_void) -> FileLayout;
    // NOT_SUPPORTED: pub fn wxStandardPaths_MakeConfigFileName(self_: *const c_void, basename: *const c_void, conv: ConfigFileConv) -> *mut c_void;
    pub fn wxStandardPaths_Get() -> *mut c_void;
    pub fn wxStandardPaths_MSWGetShellDir(csidl: c_int) -> *mut c_void;

    // wxStopWatch
    pub fn wxStopWatch_delete(self_: *mut c_void);
    pub fn wxStopWatch_new() -> *mut c_void;
    pub fn wxStopWatch_Pause(self_: *mut c_void);
    pub fn wxStopWatch_Resume(self_: *mut c_void);
    pub fn wxStopWatch_Start(self_: *mut c_void, milliseconds: c_long);
    pub fn wxStopWatch_Time(self_: *const c_void) -> c_long;
    // NOT_SUPPORTED: pub fn wxStopWatch_TimeInMicro(self_: *const c_void) -> wxLongLong;

    // wxStringClientData
    pub fn wxStringClientData_delete(self_: *mut c_void);
    pub fn wxStringClientData_new() -> *mut c_void;
    pub fn wxStringClientData_new1(data: *const c_void) -> *mut c_void;
    pub fn wxStringClientData_GetData(self_: *const c_void) -> *mut c_void;
    pub fn wxStringClientData_SetData(self_: *mut c_void, data: *const c_void);

    // wxStringTokenizer
    pub fn wxStringTokenizer_CLASSINFO() -> *mut c_void;
    pub fn wxStringTokenizer_new() -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxStringTokenizer_new1(str: *const c_void, delims: *const c_void, mode: wxStringTokenizerMode) -> *mut c_void;
    pub fn wxStringTokenizer_CountTokens(self_: *const c_void) -> usize;
    // NOT_SUPPORTED: pub fn wxStringTokenizer_GetLastDelimiter(self_: *const c_void) -> wxChar;
    pub fn wxStringTokenizer_GetNextToken(self_: *mut c_void) -> *mut c_void;
    pub fn wxStringTokenizer_GetPosition(self_: *const c_void) -> usize;
    pub fn wxStringTokenizer_GetString(self_: *const c_void) -> *mut c_void;
    pub fn wxStringTokenizer_HasMoreTokens(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxStringTokenizer_SetString(self_: *mut c_void, str: *const c_void, delims: *const c_void, mode: wxStringTokenizerMode);

    // wxSystemOptions
    pub fn wxSystemOptions_CLASSINFO() -> *mut c_void;
    pub fn wxSystemOptions_new() -> *mut c_void;
    pub fn wxSystemOptions_GetOption(name: *const c_void) -> *mut c_void;
    pub fn wxSystemOptions_GetOptionInt(name: *const c_void) -> c_int;
    pub fn wxSystemOptions_HasOption(name: *const c_void) -> bool;
    pub fn wxSystemOptions_IsFalse(name: *const c_void) -> bool;
    pub fn wxSystemOptions_SetOption(name: *const c_void, value: *const c_void);
    pub fn wxSystemOptions_SetOption1(name: *const c_void, value: c_int);

    // wxTempFFile
    pub fn wxTempFFile_delete(self_: *mut c_void);
    pub fn wxTempFFile_new() -> *mut c_void;
    pub fn wxTempFFile_new1(str_name: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxTempFFile_~wxTempFFile(self_: *mut c_void);
    pub fn wxTempFFile_Commit(self_: *mut c_void) -> bool;
    pub fn wxTempFFile_Discard(self_: *mut c_void);
    pub fn wxTempFFile_Flush(self_: *mut c_void) -> bool;
    pub fn wxTempFFile_IsOpened(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTempFFile_Length(self_: *const c_void) -> wxFileOffset;
    pub fn wxTempFFile_Open(self_: *mut c_void, str_name: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTempFFile_Seek(self_: *mut c_void, ofs: wxFileOffset, mode: wxSeekMode) -> bool;
    // NOT_SUPPORTED: pub fn wxTempFFile_Tell(self_: *const c_void) -> wxFileOffset;
    pub fn wxTempFFile_Write(self_: *mut c_void, str: *const c_void, conv: *const c_void) -> bool;

    // wxTempFile
    pub fn wxTempFile_delete(self_: *mut c_void);
    pub fn wxTempFile_new() -> *mut c_void;
    pub fn wxTempFile_new1(str_name: *const c_void) -> *mut c_void;
    // DTOR: pub fn wxTempFile_~wxTempFile(self_: *mut c_void);
    pub fn wxTempFile_Commit(self_: *mut c_void) -> bool;
    pub fn wxTempFile_Discard(self_: *mut c_void);
    pub fn wxTempFile_Flush(self_: *mut c_void) -> bool;
    pub fn wxTempFile_IsOpened(self_: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTempFile_Length(self_: *const c_void) -> wxFileOffset;
    pub fn wxTempFile_Open(self_: *mut c_void, str_name: *const c_void) -> bool;
    // NOT_SUPPORTED: pub fn wxTempFile_Seek(self_: *mut c_void, ofs: wxFileOffset, mode: wxSeekMode) -> wxFileOffset;
    // NOT_SUPPORTED: pub fn wxTempFile_Tell(self_: *const c_void) -> wxFileOffset;
    pub fn wxTempFile_Write(self_: *mut c_void, str: *const c_void, conv: *const c_void) -> bool;

    // wxTimer
    pub fn wxTimer_CLASSINFO() -> *mut c_void;
    pub fn wxTimer_new() -> *mut c_void;
    pub fn wxTimer_new1(owner: *mut c_void, id: c_int) -> *mut c_void;
    // DTOR: pub fn wxTimer_~wxTimer(self_: *mut c_void);
    pub fn wxTimer_GetId(self_: *const c_void) -> c_int;
    pub fn wxTimer_GetInterval(self_: *const c_void) -> c_int;
    pub fn wxTimer_GetOwner(self_: *const c_void) -> *mut c_void;
    pub fn wxTimer_IsOneShot(self_: *const c_void) -> bool;
    pub fn wxTimer_IsRunning(self_: *const c_void) -> bool;
    pub fn wxTimer_Notify(self_: *mut c_void);
    pub fn wxTimer_SetOwner(self_: *mut c_void, owner: *mut c_void, id: c_int);
    pub fn wxTimer_Start(self_: *mut c_void, milliseconds: c_int, one_shot: bool) -> bool;
    pub fn wxTimer_StartOnce(self_: *mut c_void, milliseconds: c_int) -> bool;
    pub fn wxTimer_Stop(self_: *mut c_void);

    // wxTimerEvent
    pub fn wxTimerEvent_CLASSINFO() -> *mut c_void;
    pub fn wxTimerEvent_new(timer: *mut c_void) -> *mut c_void;
    pub fn wxTimerEvent_GetInterval(self_: *const c_void) -> c_int;
    pub fn wxTimerEvent_GetTimer(self_: *const c_void) -> *mut c_void;

    // wxUILocale
    pub fn wxUILocale_delete(self_: *mut c_void);
    pub fn wxUILocale_UseDefault() -> bool;
    pub fn wxUILocale_GetCurrent() -> *mut c_void;
    pub fn wxUILocale_FromTag(tag: *const c_void) -> *mut c_void;
    pub fn wxUILocale_AddLanguage(info: *const c_void);
    pub fn wxUILocale_FindLanguageInfo(locale: *const c_void) -> *const c_void;
    pub fn wxUILocale_FindLanguageInfo1(locale_id: *const c_void) -> *const c_void;
    pub fn wxUILocale_GetLanguageInfo(lang: c_int) -> *const c_void;
    pub fn wxUILocale_GetLanguageName(lang: c_int) -> *mut c_void;
    pub fn wxUILocale_GetLanguageCanonicalName(lang: c_int) -> *mut c_void;
    pub fn wxUILocale_GetSystemLanguage() -> c_int;
    pub fn wxUILocale_GetSystemLocale() -> c_int;
    pub fn wxUILocale_new(locale_id: *const c_void) -> *mut c_void;
    pub fn wxUILocale_CompareStrings(
        self_: *const c_void,
        lhs: *const c_void,
        rhs: *const c_void,
        flags: c_int,
    ) -> c_int;
    pub fn wxUILocale_GetName(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxUILocale_GetLocaleId(self_: *const c_void) -> wxLocaleIdent;
    // NOT_SUPPORTED: pub fn wxUILocale_GetInfo(self_: *const c_void, index: wxLocaleInfo, cat: wxLocaleCategory) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxUILocale_GetLocalizedName(self_: *const c_void, name: wxLocaleName, form: wxLocaleForm) -> *mut c_void;
    pub fn wxUILocale_GetLayoutDirection(self_: *const c_void) -> c_int;
    pub fn wxUILocale_IsSupported(self_: *const c_void) -> bool;

    // wxURI
    pub fn wxURI_CLASSINFO() -> *mut c_void;
    pub fn wxURI_new() -> *mut c_void;
    pub fn wxURI_new1(uri: *const c_void) -> *mut c_void;
    pub fn wxURI_new2(uri: *const c_void) -> *mut c_void;
    pub fn wxURI_BuildURI(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_BuildUnescapedURI(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_Create(self_: *mut c_void, uri: *const c_void) -> bool;
    pub fn wxURI_GetFragment(self_: *const c_void) -> *mut c_void;
    // NOT_SUPPORTED: pub fn wxURI_GetHostType(self_: *const c_void) -> wxURIHostType;
    pub fn wxURI_GetPassword(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetPath(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetPort(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetQuery(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetScheme(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetServer(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetUser(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_GetUserInfo(self_: *const c_void) -> *mut c_void;
    pub fn wxURI_HasFragment(self_: *const c_void) -> bool;
    pub fn wxURI_HasPath(self_: *const c_void) -> bool;
    pub fn wxURI_HasPort(self_: *const c_void) -> bool;
    pub fn wxURI_HasQuery(self_: *const c_void) -> bool;
    pub fn wxURI_HasScheme(self_: *const c_void) -> bool;
    pub fn wxURI_HasServer(self_: *const c_void) -> bool;
    pub fn wxURI_HasUserInfo(self_: *const c_void) -> bool;
    pub fn wxURI_IsReference(self_: *const c_void) -> bool;
    pub fn wxURI_Resolve(self_: *mut c_void, base: *const c_void, flags: c_int);
    // BLOCKED: pub fn wxURI_operator==(self_: *const c_void, uricomp: *const c_void) -> bool;
    pub fn wxURI_Unescape(uri: *const c_void) -> *mut c_void;

    // wxUniCharRef
    pub fn wxUniCharRef_delete(self_: *mut c_void);

    // wxVersionInfo
    pub fn wxVersionInfo_delete(self_: *mut c_void);
    pub fn wxVersionInfo_new(
        name: *const c_void,
        major: c_int,
        minor: c_int,
        micro: c_int,
        revision: c_int,
        description: *const c_void,
        copyright: *const c_void,
    ) -> *mut c_void;
    pub fn wxVersionInfo_GetName(self_: *const c_void) -> *mut c_void;
    pub fn wxVersionInfo_GetMajor(self_: *const c_void) -> c_int;
    pub fn wxVersionInfo_GetMinor(self_: *const c_void) -> c_int;
    pub fn wxVersionInfo_GetMicro(self_: *const c_void) -> c_int;
    pub fn wxVersionInfo_GetRevision(self_: *const c_void) -> c_int;
    pub fn wxVersionInfo_ToString(self_: *const c_void) -> *mut c_void;
    pub fn wxVersionInfo_GetVersionString(self_: *const c_void) -> *mut c_void;
    pub fn wxVersionInfo_HasDescription(self_: *const c_void) -> bool;
    pub fn wxVersionInfo_GetDescription(self_: *mut c_void) -> *mut c_void;
    pub fn wxVersionInfo_HasCopyright(self_: *const c_void) -> bool;
    pub fn wxVersionInfo_GetCopyright(self_: *const c_void) -> *mut c_void;

    // wxWindowUpdateLocker
    pub fn wxWindowUpdateLocker_delete(self_: *mut c_void);
    pub fn wxWindowUpdateLocker_new() -> *mut c_void;
    pub fn wxWindowUpdateLocker_new1(win: *mut c_void) -> *mut c_void;
    pub fn wxWindowUpdateLocker_Lock(self_: *mut c_void, win: *mut c_void);
    // DTOR: pub fn wxWindowUpdateLocker_~wxWindowUpdateLocker(self_: *mut c_void);

    // wxZipNotifier
    pub fn wxZipNotifier_delete(self_: *mut c_void);
    pub fn wxZipNotifier_OnEntryUpdated(self_: *mut c_void, entry: *mut c_void);

}
