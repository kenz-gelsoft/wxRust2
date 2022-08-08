#pragma once
#include <wx/wx.h>
#include <wx/filename.h>
#include <wx/stdpaths.h>

typedef wxDateTime::TimeZone TimeZone;
typedef wxDateTime::Tm       Tm;
typedef wxDateTime::WeekDay  WeekDay;

extern "C" {

// CLASS: wxAny
void wxAny_delete(wxAny *self);
wxAny *wxAny_new();
wxAny *wxAny_new1(const T * value);
wxAny *wxAny_new2(const wxAny * any);
wxAny *wxAny_new3(const wxVariant * variant);
bool wxAny_CheckType(const wxAny * self);
bool wxAny_GetAs(const wxAny * self, T * value);
bool wxAny_GetAs1(const wxAny * self, wxVariant * value);
const wxAnyValueType * wxAny_GetType(const wxAny * self);
bool wxAny_HasSameType(const wxAny * self, const wxAny * other);
bool wxAny_IsNull(const wxAny * self);
void wxAny_MakeNull(wxAny * self);

// CLASS: wxAnyValueType
void wxAnyValueType_delete(wxAnyValueType *self);
wxAnyValueType *wxAnyValueType_new();
bool wxAnyValueType_CheckType(const wxAnyValueType * self);
bool wxAnyValueType_ConvertValue(const wxAnyValueType * self, const wxAnyValueBuffer * src, wxAnyValueType * dst_type, wxAnyValueBuffer * dst);
void wxAnyValueType_CopyBuffer(const wxAnyValueType * self, const wxAnyValueBuffer * src, wxAnyValueBuffer * dst);
void wxAnyValueType_DeleteValue(const wxAnyValueType * self, wxAnyValueBuffer * buf);
bool wxAnyValueType_IsSameType(const wxAnyValueType * self, const wxAnyValueType * other_type);

// CLASS: wxAppTraits
void wxAppTraits_delete(wxAppTraits *self);
wxConfigBase * wxAppTraits_CreateConfig(wxAppTraits * self);
wxEventLoopBase * wxAppTraits_CreateEventLoop(wxAppTraits * self);
wxFontMapper * wxAppTraits_CreateFontMapper(wxAppTraits * self);
wxLog * wxAppTraits_CreateLogTarget(wxAppTraits * self);
wxMessageOutput * wxAppTraits_CreateMessageOutput(wxAppTraits * self);
wxRendererNative * wxAppTraits_CreateRenderer(wxAppTraits * self);
wxString *wxAppTraits_GetDesktopEnvironment(const wxAppTraits * self);
wxStandardPaths * wxAppTraits_GetStandardPaths(wxAppTraits * self);
bool wxAppTraits_HasStderr(wxAppTraits * self);
bool wxAppTraits_IsUsingUniversalWidgets(const wxAppTraits * self);
bool wxAppTraits_ShowAssertDialog(wxAppTraits * self, const wxString * msg);
bool wxAppTraits_SafeMessageBox(wxAppTraits * self, const wxString * text, const wxString * title);
wxString *wxAppTraits_GetAssertStackTrace(wxAppTraits * self);

// CLASS: wxArchiveClassFactory
wxClassInfo *wxArchiveClassFactory_CLASSINFO();
wxMBConv * wxArchiveClassFactory_GetConv(const wxArchiveClassFactory * self);
void wxArchiveClassFactory_SetConv(wxArchiveClassFactory * self, wxMBConv * conv);
const wxArchiveClassFactory * wxArchiveClassFactory_GetNext(const wxArchiveClassFactory * self);
wxString *wxArchiveClassFactory_GetProtocol(const wxArchiveClassFactory * self);
wxArchiveEntry * wxArchiveClassFactory_NewEntry(const wxArchiveClassFactory * self);
wxArchiveInputStream * wxArchiveClassFactory_NewStream(const wxArchiveClassFactory * self, wxInputStream * stream);
wxArchiveOutputStream * wxArchiveClassFactory_NewStream1(const wxArchiveClassFactory * self, wxOutputStream * stream);
wxArchiveInputStream * wxArchiveClassFactory_NewStream2(const wxArchiveClassFactory * self, wxInputStream * stream);
wxArchiveOutputStream * wxArchiveClassFactory_NewStream3(const wxArchiveClassFactory * self, wxOutputStream * stream);
void wxArchiveClassFactory_PushFront(wxArchiveClassFactory * self);
void wxArchiveClassFactory_Remove(wxArchiveClassFactory * self);
const wxArchiveClassFactory * wxArchiveClassFactory_GetFirst();

// CLASS: wxArchiveEntry
wxClassInfo *wxArchiveEntry_CLASSINFO();
wxArchiveEntry * wxArchiveEntry_Clone(const wxArchiveEntry * self);
wxDateTime *wxArchiveEntry_GetDateTime(const wxArchiveEntry * self);
void wxArchiveEntry_SetDateTime(wxArchiveEntry * self, const wxDateTime * dt);
wxString *wxArchiveEntry_GetInternalName(const wxArchiveEntry * self);
bool wxArchiveEntry_IsDir(const wxArchiveEntry * self);
void wxArchiveEntry_SetIsDir(wxArchiveEntry * self, bool is_dir);
bool wxArchiveEntry_IsReadOnly(const wxArchiveEntry * self);
void wxArchiveEntry_SetIsReadOnly(wxArchiveEntry * self, bool is_read_only);
void wxArchiveEntry_SetNotifier(wxArchiveEntry * self, wxArchiveNotifier * notifier);
void wxArchiveEntry_UnsetNotifier(wxArchiveEntry * self);

// CLASS: wxArchiveIterator
void wxArchiveIterator_delete(wxArchiveIterator *self);
wxArchiveIterator *wxArchiveIterator_new();
wxArchiveIterator *wxArchiveIterator_new1(Arc * arc);

// CLASS: wxArchiveNotifier
void wxArchiveNotifier_delete(wxArchiveNotifier *self);
void wxArchiveNotifier_OnEntryUpdated(wxArchiveNotifier * self, wxArchiveEntry * entry);

// CLASS: wxClassInfo
void wxClassInfo_delete(wxClassInfo *self);
wxObject * wxClassInfo_CreateObject(const wxClassInfo * self);
const wxChar * wxClassInfo_GetBaseClassName1(const wxClassInfo * self);
const wxChar * wxClassInfo_GetBaseClassName2(const wxClassInfo * self);
const wxChar * wxClassInfo_GetClassName(const wxClassInfo * self);
int wxClassInfo_GetSize(const wxClassInfo * self);
bool wxClassInfo_IsDynamic(const wxClassInfo * self);
bool wxClassInfo_IsKindOf(const wxClassInfo * self, const wxClassInfo * info);
wxClassInfo * wxClassInfo_FindClass(const wxString * class_name);

// CLASS: wxClient
wxClassInfo *wxClient_CLASSINFO();
wxClient *wxClient_new();
wxConnectionBase * wxClient_MakeConnection(wxClient * self, const wxString * host, const wxString * service, const wxString * topic);
wxConnectionBase * wxClient_OnMakeConnection(wxClient * self);
bool wxClient_ValidHost(wxClient * self, const wxString * host);

// CLASS: wxClientData
void wxClientData_delete(wxClientData *self);
wxClientData *wxClientData_new();

// CLASS: wxClientDataContainer
void wxClientDataContainer_delete(wxClientDataContainer *self);
wxClientDataContainer *wxClientDataContainer_new();
void * wxClientDataContainer_GetClientData(const wxClientDataContainer * self);
wxClientData * wxClientDataContainer_GetClientObject(const wxClientDataContainer * self);
void wxClientDataContainer_SetClientData(wxClientDataContainer * self, void * data);
void wxClientDataContainer_SetClientObject(wxClientDataContainer * self, wxClientData * data);

// CLASS: wxCmdLineParser
void wxCmdLineParser_delete(wxCmdLineParser *self);
wxCmdLineParser *wxCmdLineParser_new();
wxCmdLineParser *wxCmdLineParser_new1(int argc, char ** argv);
wxCmdLineParser *wxCmdLineParser_new2(int argc, wchar_t ** argv);
wxCmdLineParser *wxCmdLineParser_new3(const wxString * cmdline);
wxCmdLineParser *wxCmdLineParser_new4(const wxCmdLineEntryDesc * desc);
wxCmdLineParser *wxCmdLineParser_new5(const wxCmdLineEntryDesc * desc, int argc, char ** argv);
wxCmdLineParser *wxCmdLineParser_new6(const wxCmdLineEntryDesc * desc, const wxString * cmdline);
void wxCmdLineParser_AddLongSwitch(wxCmdLineParser * self, const wxString * lng, const wxString * desc, int flags);
void wxCmdLineParser_AddSwitch(wxCmdLineParser * self, const wxString * name, const wxString * lng, const wxString * desc, int flags);
void wxCmdLineParser_AddUsageText(wxCmdLineParser * self, const wxString * text);
bool wxCmdLineParser_AreLongOptionsEnabled(const wxCmdLineParser * self);
void wxCmdLineParser_DisableLongOptions(wxCmdLineParser * self);
void wxCmdLineParser_EnableLongOptions(wxCmdLineParser * self, bool enable);
bool wxCmdLineParser_Found(const wxCmdLineParser * self, const wxString * name);
bool wxCmdLineParser_Found1(const wxCmdLineParser * self, const wxString * name, wxString * value);
bool wxCmdLineParser_Found2(const wxCmdLineParser * self, const wxString * name, long * value);
bool wxCmdLineParser_Found3(const wxCmdLineParser * self, const wxString * name, double * value);
bool wxCmdLineParser_Found4(const wxCmdLineParser * self, const wxString * name, wxDateTime * value);
wxString *wxCmdLineParser_GetParam(const wxCmdLineParser * self, size_t n);
size_t wxCmdLineParser_GetParamCount(const wxCmdLineParser * self);
wxCmdLineArgs *wxCmdLineParser_GetArguments(const wxCmdLineParser * self);
int wxCmdLineParser_Parse(wxCmdLineParser * self, bool give_usage);
void wxCmdLineParser_SetCmdLine(wxCmdLineParser * self, int argc, char ** argv);
void wxCmdLineParser_SetCmdLine1(wxCmdLineParser * self, int argc, wchar_t ** argv);
void wxCmdLineParser_SetCmdLine2(wxCmdLineParser * self, const wxString * cmdline);
void wxCmdLineParser_SetDesc(wxCmdLineParser * self, const wxCmdLineEntryDesc * desc);
void wxCmdLineParser_SetLogo(wxCmdLineParser * self, const wxString * logo);
void wxCmdLineParser_SetSwitchChars(wxCmdLineParser * self, const wxString * switch_chars);
void wxCmdLineParser_Usage(const wxCmdLineParser * self);
wxString *wxCmdLineParser_GetUsageString(const wxCmdLineParser * self);

// CLASS: wxCondition
void wxCondition_delete(wxCondition *self);
wxCondition *wxCondition_new(wxMutex * mutex);
bool wxCondition_IsOk(const wxCondition * self);

// CLASS: wxConfigPathChanger
void wxConfigPathChanger_delete(wxConfigPathChanger *self);
wxConfigPathChanger *wxConfigPathChanger_new(const wxConfigBase * p_container, const wxString * str_entry);
wxString *wxConfigPathChanger_Name(const wxConfigPathChanger * self);
void wxConfigPathChanger_UpdateIfDeleted(wxConfigPathChanger * self);

// CLASS: wxConnection
wxClassInfo *wxConnection_CLASSINFO();
wxConnection *wxConnection_new();
wxConnection *wxConnection_new1(void * buffer, size_t size);
bool wxConnection_Advise1(wxConnection * self, const wxString * item, const char * data);
bool wxConnection_Advise2(wxConnection * self, const wxString * item, const wchar_t * data);
bool wxConnection_Advise3(wxConnection * self, const wxString * item, const wxString data);
bool wxConnection_Disconnect(wxConnection * self);
bool wxConnection_Execute1(wxConnection * self, const char * data);
bool wxConnection_Execute2(wxConnection * self, const wchar_t * data);
bool wxConnection_Execute3(wxConnection * self, const wxString data);
bool wxConnection_OnDisconnect(wxConnection * self);
bool wxConnection_OnExec(wxConnection * self, const wxString * topic, const wxString * data);
bool wxConnection_OnStartAdvise(wxConnection * self, const wxString * topic, const wxString * item);
bool wxConnection_OnStopAdvise(wxConnection * self, const wxString * topic, const wxString * item);
bool wxConnection_Poke1(wxConnection * self, const wxString * item, const char * data);
bool wxConnection_Poke2(wxConnection * self, const wxString * item, const wchar_t * data);
bool wxConnection_Poke3(wxConnection * self, const wxString * item, const wxString data);
bool wxConnection_StartAdvise(wxConnection * self, const wxString * item);
bool wxConnection_StopAdvise(wxConnection * self, const wxString * item);

// CLASS: wxConnectionBase
wxClassInfo *wxConnectionBase_CLASSINFO();

// CLASS: wxCriticalSection
void wxCriticalSection_delete(wxCriticalSection *self);
void wxCriticalSection_Enter(wxCriticalSection * self);
bool wxCriticalSection_TryEnter(wxCriticalSection * self);
void wxCriticalSection_Leave(wxCriticalSection * self);

// CLASS: wxCriticalSectionLocker
void wxCriticalSectionLocker_delete(wxCriticalSectionLocker *self);
wxCriticalSectionLocker *wxCriticalSectionLocker_new(wxCriticalSection * criticalsection);

// CLASS: wxDDEClient
wxClassInfo *wxDDEClient_CLASSINFO();
wxDDEClient *wxDDEClient_new();
wxConnectionBase * wxDDEClient_MakeConnection(wxDDEClient * self, const wxString * host, const wxString * service, const wxString * topic);
wxConnectionBase * wxDDEClient_OnMakeConnection(wxDDEClient * self);
bool wxDDEClient_ValidHost(wxDDEClient * self, const wxString * host);

// CLASS: wxDDEConnection
wxClassInfo *wxDDEConnection_CLASSINFO();
wxDDEConnection *wxDDEConnection_new();
wxDDEConnection *wxDDEConnection_new1(void * buffer, size_t size);
bool wxDDEConnection_Advise1(wxDDEConnection * self, const wxString * item, const char * data);
bool wxDDEConnection_Advise2(wxDDEConnection * self, const wxString * item, const wchar_t * data);
bool wxDDEConnection_Advise3(wxDDEConnection * self, const wxString * item, const wxString data);
bool wxDDEConnection_Disconnect(wxDDEConnection * self);
bool wxDDEConnection_Execute1(wxDDEConnection * self, const char * data);
bool wxDDEConnection_Execute2(wxDDEConnection * self, const wchar_t * data);
bool wxDDEConnection_Execute3(wxDDEConnection * self, const wxString data);
bool wxDDEConnection_OnDisconnect(wxDDEConnection * self);
bool wxDDEConnection_OnStartAdvise(wxDDEConnection * self, const wxString * topic, const wxString * item);
bool wxDDEConnection_OnStopAdvise(wxDDEConnection * self, const wxString * topic, const wxString * item);
bool wxDDEConnection_Poke1(wxDDEConnection * self, const wxString * item, const char * data);
bool wxDDEConnection_Poke2(wxDDEConnection * self, const wxString * item, const wchar_t * data);
bool wxDDEConnection_Poke3(wxDDEConnection * self, const wxString * item, const wxString data);
bool wxDDEConnection_StartAdvise(wxDDEConnection * self, const wxString * item);
bool wxDDEConnection_StopAdvise(wxDDEConnection * self, const wxString * item);

// CLASS: wxDDEServer
void wxDDEServer_delete(wxDDEServer *self);
wxDDEServer *wxDDEServer_new();
bool wxDDEServer_Create(wxDDEServer * self, const wxString * service);
wxConnectionBase * wxDDEServer_OnAcceptConnection(wxDDEServer * self, const wxString * topic);

// CLASS: wxDataInputStream
void wxDataInputStream_delete(wxDataInputStream *self);
wxDataInputStream *wxDataInputStream_new(wxInputStream * stream, const wxMBConv * conv);
void wxDataInputStream_BigEndianOrdered(wxDataInputStream * self, bool be_order);
wxMBConv * wxDataInputStream_GetConv(const wxDataInputStream * self);
void wxDataInputStream_Read81(wxDataInputStream * self, wxUint8 * buffer, size_t size);
void wxDataInputStream_Read161(wxDataInputStream * self, wxUint16 * buffer, size_t size);
void wxDataInputStream_Read321(wxDataInputStream * self, wxUint32 * buffer, size_t size);
void wxDataInputStream_Read641(wxDataInputStream * self, wxUint64 * buffer, size_t size);
void wxDataInputStream_ReadFloat1(wxDataInputStream * self, float * buffer, size_t size);
double wxDataInputStream_ReadDouble(wxDataInputStream * self);
void wxDataInputStream_ReadDouble1(wxDataInputStream * self, double * buffer, size_t size);
wxString *wxDataInputStream_ReadString(wxDataInputStream * self);
void wxDataInputStream_SetConv(wxDataInputStream * self, const wxMBConv * conv);
void wxDataInputStream_UseBasicPrecisions(wxDataInputStream * self);
void wxDataInputStream_UseExtendedPrecision(wxDataInputStream * self);

// CLASS: wxDataOutputStream
void wxDataOutputStream_delete(wxDataOutputStream *self);
wxDataOutputStream *wxDataOutputStream_new(wxOutputStream * stream, const wxMBConv * conv);
void wxDataOutputStream_BigEndianOrdered(wxDataOutputStream * self, bool be_order);
wxMBConv * wxDataOutputStream_GetConv(const wxDataOutputStream * self);
void wxDataOutputStream_SetConv(wxDataOutputStream * self, const wxMBConv * conv);
void wxDataOutputStream_UseBasicPrecisions(wxDataOutputStream * self);
void wxDataOutputStream_UseExtendedPrecision(wxDataOutputStream * self);
void wxDataOutputStream_Write81(wxDataOutputStream * self, const wxUint8 * buffer, size_t size);
void wxDataOutputStream_Write161(wxDataOutputStream * self, const wxUint16 * buffer, size_t size);
void wxDataOutputStream_Write321(wxDataOutputStream * self, const wxUint32 * buffer, size_t size);
void wxDataOutputStream_Write641(wxDataOutputStream * self, const wxUint64 * buffer, size_t size);
void wxDataOutputStream_WriteFloat1(wxDataOutputStream * self, const float * buffer, size_t size);
void wxDataOutputStream_WriteDouble(wxDataOutputStream * self, double d);
void wxDataOutputStream_WriteDouble1(wxDataOutputStream * self, const double * buffer, size_t size);
void wxDataOutputStream_WriteString(wxDataOutputStream * self, const wxString * string);

// CLASS: wxDateSpan
void wxDateSpan_delete(wxDateSpan *self);
wxDateSpan *wxDateSpan_new(int years, int months, int weeks, int days);
wxDateSpan *wxDateSpan_Add(const wxDateSpan * self, const wxDateSpan * other);
wxDateSpan * wxDateSpan_Add1(wxDateSpan * self, const wxDateSpan * other);
int wxDateSpan_GetDays(const wxDateSpan * self);
int wxDateSpan_GetMonths(const wxDateSpan * self);
int wxDateSpan_GetTotalMonths(const wxDateSpan * self);
int wxDateSpan_GetTotalDays(const wxDateSpan * self);
int wxDateSpan_GetWeeks(const wxDateSpan * self);
int wxDateSpan_GetYears(const wxDateSpan * self);
wxDateSpan *wxDateSpan_Multiply(const wxDateSpan * self, int factor);
wxDateSpan * wxDateSpan_Multiply1(wxDateSpan * self, int factor);
wxDateSpan * wxDateSpan_Neg(wxDateSpan * self);
wxDateSpan *wxDateSpan_Negate(const wxDateSpan * self);
wxDateSpan * wxDateSpan_SetDays(wxDateSpan * self, int n);
wxDateSpan * wxDateSpan_SetMonths(wxDateSpan * self, int n);
wxDateSpan * wxDateSpan_SetWeeks(wxDateSpan * self, int n);
wxDateSpan * wxDateSpan_SetYears(wxDateSpan * self, int n);
wxDateSpan *wxDateSpan_Subtract(const wxDateSpan * self, const wxDateSpan * other);
wxDateSpan * wxDateSpan_Subtract1(wxDateSpan * self, const wxDateSpan * other);
wxDateSpan *wxDateSpan_Day();
wxDateSpan *wxDateSpan_Days(int days);
wxDateSpan *wxDateSpan_Month();
wxDateSpan *wxDateSpan_Months(int mon);
wxDateSpan *wxDateSpan_Week();
wxDateSpan *wxDateSpan_Weeks(int weeks);
wxDateSpan *wxDateSpan_Year();
wxDateSpan *wxDateSpan_Years(int years);

// CLASS: wxDateTime
void wxDateTime_delete(wxDateTime *self);
wxDateTime *wxDateTime_new();
wxDateTime *wxDateTime_new1(const wxDateTime * date);
wxDateTime *wxDateTime_new4(double jdn);
wxDateTime * wxDateTime_ResetTime(wxDateTime * self);
wxDateTime * wxDateTime_Set2(wxDateTime * self, const Tm * tm);
wxDateTime * wxDateTime_Set3(wxDateTime * self, double jdn);
wxDateTime * wxDateTime_SetToCurrent(wxDateTime * self);
wxDateTime * wxDateTime_SetYear(wxDateTime * self, int year);
#ifdef __WXMSW__
wxDateTime * wxDateTime_SetFromMSWSysTime(wxDateTime * self, const struct _SYSTEMTIME * st);
void wxDateTime_GetAsMSWSysTime(const wxDateTime * self, struct _SYSTEMTIME * st);
#endif
int wxDateTime_GetCentury(const wxDateTime * self, const TimeZone * tz);
wxDateTime *wxDateTime_GetDateOnly(const wxDateTime * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxDateTime_GetWeekBasedYear(const wxDateTime * self, const TimeZone * tz);
#endif
int wxDateTime_GetYear(const wxDateTime * self, const TimeZone * tz);
bool wxDateTime_IsValid(const wxDateTime * self);
bool wxDateTime_IsEarlierThan(const wxDateTime * self, const wxDateTime * datetime);
bool wxDateTime_IsEqualTo(const wxDateTime * self, const wxDateTime * datetime);
bool wxDateTime_IsEqualUpTo(const wxDateTime * self, const wxDateTime * dt, const wxTimeSpan * ts);
bool wxDateTime_IsLaterThan(const wxDateTime * self, const wxDateTime * datetime);
bool wxDateTime_IsSameDate(const wxDateTime * self, const wxDateTime * dt);
bool wxDateTime_IsSameTime(const wxDateTime * self, const wxDateTime * dt);
bool wxDateTime_IsStrictlyBetween(const wxDateTime * self, const wxDateTime * t1, const wxDateTime * t2);
bool wxDateTime_IsBetween(const wxDateTime * self, const wxDateTime * t1, const wxDateTime * t2);
wxDateTime * wxDateTime_Add1(wxDateTime * self, const wxDateSpan * diff);
wxDateTime * wxDateTime_Add3(wxDateTime * self, const wxTimeSpan * diff);
wxDateTime * wxDateTime_Subtract1(wxDateTime * self, const wxTimeSpan * diff);
wxDateTime * wxDateTime_Subtract3(wxDateTime * self, const wxDateSpan * diff);
wxDateSpan *wxDateTime_DiffAsDateSpan(const wxDateTime * self, const wxDateTime * dt);
wxString *wxDateTime_Format(const wxDateTime * self, const wxString * format, const TimeZone * tz);
wxString *wxDateTime_FormatDate(const wxDateTime * self);
wxString *wxDateTime_FormatISODate(const wxDateTime * self);
wxString *wxDateTime_FormatISOTime(const wxDateTime * self);
wxString *wxDateTime_FormatTime(const wxDateTime * self);
bool wxDateTime_ParseDateTime(wxDateTime * self, const wxString * datetime, wxString::const_iterator * end);
bool wxDateTime_ParseFormat(wxDateTime * self, const wxString * date, const wxString * format, const wxDateTime * date_def, wxString::const_iterator * end);
bool wxDateTime_ParseISODate(wxDateTime * self, const wxString * date);
bool wxDateTime_ParseISOTime(wxDateTime * self, const wxString * date);
bool wxDateTime_ParseRfc822Date(wxDateTime * self, const wxString * date, wxString::const_iterator * end);
bool wxDateTime_ParseTime(wxDateTime * self, const wxString * time, wxString::const_iterator * end);
double wxDateTime_GetJDN(const wxDateTime * self);
double wxDateTime_GetJulianDayNumber(const wxDateTime * self);
double wxDateTime_GetMJD(const wxDateTime * self);
double wxDateTime_GetModifiedJulianDayNumber(const wxDateTime * self);
double wxDateTime_GetRataDie(const wxDateTime * self);
wxDateTime *wxDateTime_FromTimezone(const wxDateTime * self, const TimeZone * tz, bool no_dst);
wxDateTime * wxDateTime_MakeFromTimezone(wxDateTime * self, const TimeZone * tz, bool no_dst);
wxDateTime * wxDateTime_MakeTimezone(wxDateTime * self, const TimeZone * tz, bool no_dst);
wxDateTime * wxDateTime_MakeUTC(wxDateTime * self, bool no_dst);
wxDateTime *wxDateTime_ToTimezone(const wxDateTime * self, const TimeZone * tz, bool no_dst);
wxDateTime *wxDateTime_ToUTC(const wxDateTime * self, bool no_dst);
int wxDateTime_ConvertYearToBC(int year);
void wxDateTime_GetAmPmStrings(wxString * am, wxString * pm);
int wxDateTime_GetCentury1(int year);
tm * wxDateTime_GetTmNow1();
#if wxCHECK_VERSION(3, 1, 0)
bool wxDateTime_GetFirstWeekDay(WeekDay * first_day);
#endif
wxDateTime *wxDateTime_Now();
wxDateTime *wxDateTime_Today();
wxDateTime *wxDateTime_UNow();

// CLASS: wxDateTimeHolidayAuthority
void wxDateTimeHolidayAuthority_delete(wxDateTimeHolidayAuthority *self);

// CLASS: wxDateTimeWorkDays
void wxDateTimeWorkDays_delete(wxDateTimeWorkDays *self);

// CLASS: wxDebugContext
void wxDebugContext_delete(wxDebugContext *self);
int wxDebugContext_Check(bool check_all);
bool wxDebugContext_Dump();
bool wxDebugContext_GetCheckPrevious();
bool wxDebugContext_GetDebugMode();
int wxDebugContext_GetLevel();
bool wxDebugContext_PrintClasses();
bool wxDebugContext_PrintStatistics(bool detailed);
void wxDebugContext_SetCheckPrevious(bool check);
void wxDebugContext_SetCheckpoint(bool all);
void wxDebugContext_SetDebugMode(bool debug);
void wxDebugContext_SetLevel(int level);

// CLASS: wxDir
void wxDir_delete(wxDir *self);
wxDir *wxDir_new();
wxDir *wxDir_new1(const wxString * dir);
void wxDir_Close(wxDir * self);
bool wxDir_GetFirst(const wxDir * self, wxString * filename, const wxString * filespec, int flags);
wxString *wxDir_GetName(const wxDir * self);
wxString *wxDir_GetNameWithSep(const wxDir * self);
bool wxDir_GetNext(const wxDir * self, wxString * filename);
bool wxDir_HasFiles(const wxDir * self, const wxString * filespec);
bool wxDir_HasSubDirs(const wxDir * self, const wxString * dirspec);
bool wxDir_IsOpened(const wxDir * self);
bool wxDir_Open(wxDir * self, const wxString * dir);
size_t wxDir_Traverse(const wxDir * self, wxDirTraverser * sink, const wxString * filespec, int flags);
bool wxDir_Exists(const wxString * dir);
wxString *wxDir_FindFirst(const wxString * dirname, const wxString * filespec, int flags);
size_t wxDir_GetAllFiles(const wxString * dirname, wxArrayString * files, const wxString * filespec, int flags);
bool wxDir_Make(const wxString * dir, int perm, int flags);
bool wxDir_Remove(const wxString * dir, int flags);

// CLASS: wxDirTraverser
void wxDirTraverser_delete(wxDirTraverser *self);

// CLASS: wxDynamicLibrary
void wxDynamicLibrary_delete(wxDynamicLibrary *self);
wxDynamicLibrary *wxDynamicLibrary_new();
wxDynamicLibrary *wxDynamicLibrary_new1(const wxString * name, int flags);
void * wxDynamicLibrary_GetSymbol(const wxDynamicLibrary * self, const wxString * name, bool * success);
void * wxDynamicLibrary_GetSymbolAorW(const wxDynamicLibrary * self, const wxString * name);
bool wxDynamicLibrary_HasSymbol(const wxDynamicLibrary * self, const wxString * name);
bool wxDynamicLibrary_IsLoaded(const wxDynamicLibrary * self);
bool wxDynamicLibrary_Load(wxDynamicLibrary * self, const wxString * name, int flags);
void wxDynamicLibrary_Unload(wxDynamicLibrary * self);
void * wxDynamicLibrary_GetModuleFromAddress(const void * addr, wxString * path);

// CLASS: wxDynamicLibraryDetails
void wxDynamicLibraryDetails_delete(wxDynamicLibraryDetails *self);
bool wxDynamicLibraryDetails_GetAddress(const wxDynamicLibraryDetails * self, void * addr, size_t * len);
wxString *wxDynamicLibraryDetails_GetName(const wxDynamicLibraryDetails * self);
wxString *wxDynamicLibraryDetails_GetPath(const wxDynamicLibraryDetails * self);
wxString *wxDynamicLibraryDetails_GetVersion(const wxDynamicLibraryDetails * self);

// CLASS: wxEncodingConverter
wxClassInfo *wxEncodingConverter_CLASSINFO();
bool wxEncodingConverter_Convert(const wxEncodingConverter * self, const char * input, char * output);
bool wxEncodingConverter_Convert1(const wxEncodingConverter * self, const wchar_t * input, wchar_t * output);
bool wxEncodingConverter_Convert2(const wxEncodingConverter * self, const char * input, wchar_t * output);
bool wxEncodingConverter_Convert3(const wxEncodingConverter * self, const wchar_t * input, char * output);
bool wxEncodingConverter_Convert4(const wxEncodingConverter * self, char * str);
bool wxEncodingConverter_Convert5(const wxEncodingConverter * self, wchar_t * str);
wxString *wxEncodingConverter_Convert6(const wxEncodingConverter * self, const wxString * input);
wxEncodingConverter *wxEncodingConverter_new();

// CLASS: wxEvent
wxClassInfo *wxEvent_CLASSINFO();
wxEvent * wxEvent_Clone(const wxEvent * self);
wxObject * wxEvent_GetEventObject(const wxEvent * self);
int wxEvent_GetId(const wxEvent * self);
wxObject * wxEvent_GetEventUserData(const wxEvent * self);
bool wxEvent_GetSkipped(const wxEvent * self);
long wxEvent_GetTimestamp(const wxEvent * self);
bool wxEvent_IsCommandEvent(const wxEvent * self);
void wxEvent_ResumePropagation(wxEvent * self, int propagation_level);
void wxEvent_SetEventObject(wxEvent * self, wxObject * object);
void wxEvent_SetId(wxEvent * self, int id);
void wxEvent_SetTimestamp(wxEvent * self, long time_stamp);
bool wxEvent_ShouldPropagate(const wxEvent * self);
void wxEvent_Skip(wxEvent * self, bool skip);
int wxEvent_StopPropagation(wxEvent * self);

// CLASS: wxEvtHandler
wxClassInfo *wxEvtHandler_CLASSINFO();
void wxEvtHandler_QueueEvent(wxEvtHandler * self, wxEvent * event);
void wxEvtHandler_AddPendingEvent(wxEvtHandler * self, const wxEvent * event);
bool wxEvtHandler_ProcessEvent(wxEvtHandler * self, wxEvent * event);
bool wxEvtHandler_ProcessEventLocally(wxEvtHandler * self, wxEvent * event);
bool wxEvtHandler_SafelyProcessEvent(wxEvtHandler * self, wxEvent * event);
void wxEvtHandler_ProcessPendingEvents(wxEvtHandler * self);
void wxEvtHandler_DeletePendingEvents(wxEvtHandler * self);
wxClientData * wxEvtHandler_GetClientObject(const wxEvtHandler * self);
void wxEvtHandler_SetClientObject(wxEvtHandler * self, wxClientData * data);
bool wxEvtHandler_GetEvtHandlerEnabled(const wxEvtHandler * self);
wxEvtHandler * wxEvtHandler_GetNextHandler(const wxEvtHandler * self);
wxEvtHandler * wxEvtHandler_GetPreviousHandler(const wxEvtHandler * self);
void wxEvtHandler_SetEvtHandlerEnabled(wxEvtHandler * self, bool enabled);
void wxEvtHandler_SetNextHandler(wxEvtHandler * self, wxEvtHandler * handler);
void wxEvtHandler_SetPreviousHandler(wxEvtHandler * self, wxEvtHandler * handler);
void wxEvtHandler_Unlink(wxEvtHandler * self);
bool wxEvtHandler_IsUnlinked(const wxEvtHandler * self);
void wxEvtHandler_AddFilter(wxEventFilter * filter);
void wxEvtHandler_RemoveFilter(wxEventFilter * filter);
wxEvtHandler *wxEvtHandler_new();
// Mix-in(s) to wxEvtHandler
wxTrackable *wxEvtHandler_AsTrackable(wxEvtHandler* obj);

// CLASS: wxFFile
void wxFFile_delete(wxFFile *self);
wxFFile *wxFFile_new();
wxFFile *wxFFile_new1(FILE * fp);
wxFFile *wxFFile_new2(const wxString * filename, const wxString * mode);
void wxFFile_Attach(wxFFile * self, FILE * fp, const wxString * name);
bool wxFFile_Close(wxFFile * self);
FILE * wxFFile_Detach(wxFFile * self);
bool wxFFile_Eof(const wxFFile * self);
bool wxFFile_Error(const wxFFile * self);
bool wxFFile_Flush(wxFFile * self);
wxString *wxFFile_GetName(const wxFFile * self);
bool wxFFile_IsOpened(const wxFFile * self);
bool wxFFile_Open(wxFFile * self, const wxString * filename, const wxString * mode);
size_t wxFFile_Read(wxFFile * self, void * buffer, size_t count);
bool wxFFile_ReadAll(wxFFile * self, wxString * str, const wxMBConv * conv);
bool wxFFile_Write(wxFFile * self, const wxString * str, const wxMBConv * conv);
size_t wxFFile_Write1(wxFFile * self, const void * buffer, size_t count);
FILE * wxFFile_fp(const wxFFile * self);

// CLASS: wxFSFile
wxClassInfo *wxFSFile_CLASSINFO();
wxFSFile *wxFSFile_new(wxInputStream * stream, const wxString * location, const wxString * mimetype, const wxString * anchor, wxDateTime modif);
wxInputStream * wxFSFile_DetachStream(wxFSFile * self);
wxString *wxFSFile_GetAnchor(const wxFSFile * self);
wxString *wxFSFile_GetLocation(const wxFSFile * self);
wxString *wxFSFile_GetMimeType(const wxFSFile * self);
wxDateTime *wxFSFile_GetModificationTime(const wxFSFile * self);
wxInputStream * wxFSFile_GetStream(const wxFSFile * self);

// CLASS: wxFSVolume
void wxFSVolume_delete(wxFSVolume *self);
wxFSVolume *wxFSVolume_new();
wxFSVolume *wxFSVolume_new1(const wxString * name);
bool wxFSVolume_Create(wxFSVolume * self, const wxString * name);
bool wxFSVolume_IsOk(const wxFSVolume * self);
int wxFSVolume_GetFlags(const wxFSVolume * self);
bool wxFSVolume_IsWritable(const wxFSVolume * self);
wxString *wxFSVolume_GetName(const wxFSVolume * self);
wxString *wxFSVolume_GetDisplayName(const wxFSVolume * self);
wxArrayString *wxFSVolume_GetVolumes(int flags_set, int flags_unset);
void wxFSVolume_CancelSearch();

// CLASS: wxFile
void wxFile_delete(wxFile *self);
wxFile *wxFile_new();
wxFile *wxFile_new2(int fd);
int wxFile_GetLastError(const wxFile * self);
void wxFile_ClearLastError(wxFile * self);
void wxFile_Attach(wxFile * self, int fd);
bool wxFile_Close(wxFile * self);
bool wxFile_Create(wxFile * self, const wxString * filename, bool overwrite, int access);
int wxFile_Detach(wxFile * self);
bool wxFile_Eof(const wxFile * self);
bool wxFile_Flush(wxFile * self);
bool wxFile_IsOpened(const wxFile * self);
bool wxFile_ReadAll(wxFile * self, wxString * str, const wxMBConv * conv);
size_t wxFile_Write(wxFile * self, const void * buffer, size_t count);
bool wxFile_Write1(wxFile * self, const wxString * s, const wxMBConv * conv);
int wxFile_fd(const wxFile * self);
bool wxFile_Exists(const wxString * filename);

// CLASS: wxFileName
void wxFileName_delete(wxFileName *self);
wxFileName *wxFileName_new();
wxFileName *wxFileName_new1(const wxFileName * filename);
bool wxFileName_AppendDir(wxFileName * self, const wxString * dir);
void wxFileName_Assign(wxFileName * self, const wxFileName * filepath);
void wxFileName_AssignCwd(wxFileName * self, const wxString * volume);
void wxFileName_AssignHomeDir(wxFileName * self);
void wxFileName_AssignTempFileName(wxFileName * self, const wxString * prefix);
void wxFileName_AssignTempFileName1(wxFileName * self, const wxString * prefix, wxFile * file_temp);
void wxFileName_AssignTempFileName2(wxFileName * self, const wxString * prefix, wxFFile * file_temp);
void wxFileName_Clear(wxFileName * self);
void wxFileName_ClearExt(wxFileName * self);
bool wxFileName_DirExists(const wxFileName * self);
void wxFileName_DontFollowLink(wxFileName * self);
bool wxFileName_Exists(const wxFileName * self, int flags);
bool wxFileName_FileExists(const wxFileName * self);
size_t wxFileName_GetDirCount(const wxFileName * self);
wxArrayString *wxFileName_GetDirs(const wxFileName * self);
wxString *wxFileName_GetExt(const wxFileName * self);
wxString *wxFileName_GetFullName(const wxFileName * self);
wxString *wxFileName_GetLongPath(const wxFileName * self);
wxDateTime *wxFileName_GetModificationTime(const wxFileName * self);
wxString *wxFileName_GetName(const wxFileName * self);
wxString *wxFileName_GetShortPath(const wxFileName * self);
bool wxFileName_GetTimes(const wxFileName * self, wxDateTime * dt_access, wxDateTime * dt_mod, wxDateTime * dt_create);
wxString *wxFileName_GetVolume(const wxFileName * self);
bool wxFileName_HasExt(const wxFileName * self);
bool wxFileName_HasName(const wxFileName * self);
bool wxFileName_HasVolume(const wxFileName * self);
bool wxFileName_InsertDir(wxFileName * self, size_t before, const wxString * dir);
bool wxFileName_IsDir(const wxFileName * self);
bool wxFileName_IsDirReadable(const wxFileName * self);
bool wxFileName_IsDirWritable(const wxFileName * self);
bool wxFileName_IsFileExecutable(const wxFileName * self);
bool wxFileName_IsFileReadable(const wxFileName * self);
bool wxFileName_IsFileWritable(const wxFileName * self);
bool wxFileName_IsOk(const wxFileName * self);
bool wxFileName_Mkdir(const wxFileName * self, int perm, int flags);
void wxFileName_PrependDir(wxFileName * self, const wxString * dir);
void wxFileName_RemoveDir(wxFileName * self, size_t pos);
void wxFileName_RemoveLastDir(wxFileName * self);
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_ResolveLink(wxFileName * self);
#endif
bool wxFileName_Rmdir(const wxFileName * self, int flags);
bool wxFileName_SetCwd(const wxFileName * self);
void wxFileName_SetEmptyExt(wxFileName * self);
void wxFileName_SetExt(wxFileName * self, const wxString * ext);
void wxFileName_SetFullName(wxFileName * self, const wxString * fullname);
void wxFileName_SetName(wxFileName * self, const wxString * name);
bool wxFileName_SetPermissions(wxFileName * self, int permissions);
bool wxFileName_SetTimes(const wxFileName * self, const wxDateTime * dt_access, const wxDateTime * dt_mod, const wxDateTime * dt_create);
void wxFileName_SetVolume(wxFileName * self, const wxString * volume);
bool wxFileName_ShouldFollowLink(const wxFileName * self);
bool wxFileName_Touch(const wxFileName * self);
wxString *wxFileName_CreateTempFileName(const wxString * prefix, wxFile * file_temp);
wxString *wxFileName_CreateTempFileName1(const wxString * prefix, wxFFile * file_temp);
bool wxFileName_DirExists1(const wxString * dir);
bool wxFileName_Exists1(const wxString * path, int flags);
bool wxFileName_FileExists1(const wxString * file);
wxString *wxFileName_GetCwd(const wxString * volume);
wxString *wxFileName_GetHomeDir();
wxString *wxFileName_GetTempDir();
bool wxFileName_IsDirReadable1(const wxString * dir);
bool wxFileName_IsDirWritable1(const wxString * dir);
bool wxFileName_IsFileExecutable1(const wxString * file);
bool wxFileName_IsFileReadable1(const wxString * file);
bool wxFileName_IsFileWritable1(const wxString * file);
bool wxFileName_Mkdir1(const wxString * dir, int perm, int flags);
bool wxFileName_Rmdir1(const wxString * dir, int flags);
bool wxFileName_SetCwd1(const wxString * cwd);
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_URLToFileName(const wxString * url);
wxString *wxFileName_FileNameToURL(const wxFileName * filename);
#endif
wxString *wxFileName_StripExtension(const wxString * fullname);

// CLASS: wxFileSystem
wxClassInfo *wxFileSystem_CLASSINFO();
wxFileSystem *wxFileSystem_new();
void wxFileSystem_ChangePathTo(wxFileSystem * self, const wxString * location, bool is_dir);
bool wxFileSystem_FindFileInPath(wxFileSystem * self, wxString * p_str, const wxString * path, const wxString * file);
wxString *wxFileSystem_FindFirst(wxFileSystem * self, const wxString * wildcard, int flags);
wxString *wxFileSystem_FindNext(wxFileSystem * self);
wxString *wxFileSystem_GetPath(const wxFileSystem * self);
wxFSFile * wxFileSystem_OpenFile(wxFileSystem * self, const wxString * location, int flags);
void wxFileSystem_AddHandler(wxFileSystemHandler * handler);
wxFileSystemHandler * wxFileSystem_RemoveHandler(wxFileSystemHandler * handler);
wxString *wxFileSystem_FileNameToURL(const wxFileName * filename);
bool wxFileSystem_HasHandlerForPath(const wxString * location);
wxFileName *wxFileSystem_URLToFileName(const wxString * url);

// CLASS: wxFileSystemHandler
wxClassInfo *wxFileSystemHandler_CLASSINFO();
wxFileSystemHandler *wxFileSystemHandler_new();
bool wxFileSystemHandler_CanOpen(wxFileSystemHandler * self, const wxString * location);
wxString *wxFileSystemHandler_FindFirst(wxFileSystemHandler * self, const wxString * wildcard, int flags);
wxString *wxFileSystemHandler_FindNext(wxFileSystemHandler * self);
wxFSFile * wxFileSystemHandler_OpenFile(wxFileSystemHandler * self, wxFileSystem * fs, const wxString * location);
wxString *wxFileSystemHandler_GetMimeTypeFromExt(const wxString * location);

// CLASS: wxFileSystemWatcher
wxClassInfo *wxFileSystemWatcher_CLASSINFO();
wxFileSystemWatcher *wxFileSystemWatcher_new();
bool wxFileSystemWatcher_Add(wxFileSystemWatcher * self, const wxFileName * path, int events);
bool wxFileSystemWatcher_AddTree(wxFileSystemWatcher * self, const wxFileName * path, int events, const wxString * filter);
bool wxFileSystemWatcher_Remove(wxFileSystemWatcher * self, const wxFileName * path);
bool wxFileSystemWatcher_RemoveTree(wxFileSystemWatcher * self, const wxFileName * path);
bool wxFileSystemWatcher_RemoveAll(wxFileSystemWatcher * self);
int wxFileSystemWatcher_GetWatchedPathsCount(const wxFileSystemWatcher * self);
int wxFileSystemWatcher_GetWatchedPaths(const wxFileSystemWatcher * self, wxArrayString * paths);
void wxFileSystemWatcher_SetOwner(wxFileSystemWatcher * self, wxEvtHandler * handler);
// Mix-in(s) to wxFileSystemWatcher
wxTrackable *wxFileSystemWatcher_AsTrackable(wxFileSystemWatcher* obj);

// CLASS: wxFileSystemWatcherEvent
wxClassInfo *wxFileSystemWatcherEvent_CLASSINFO();
wxFileSystemWatcherEvent *wxFileSystemWatcherEvent_new(int change_type, int watchid);
wxFileSystemWatcherEvent *wxFileSystemWatcherEvent_new2(int change_type, const wxFileName * path, const wxFileName * new_path, int watchid);
wxFileName *wxFileSystemWatcherEvent_GetPath(const wxFileSystemWatcherEvent * self);
wxFileName *wxFileSystemWatcherEvent_GetNewPath(const wxFileSystemWatcherEvent * self);
int wxFileSystemWatcherEvent_GetChangeType(const wxFileSystemWatcherEvent * self);
bool wxFileSystemWatcherEvent_IsError(const wxFileSystemWatcherEvent * self);
wxString *wxFileSystemWatcherEvent_GetErrorDescription(const wxFileSystemWatcherEvent * self);
wxString *wxFileSystemWatcherEvent_ToString(const wxFileSystemWatcherEvent * self);

// CLASS: wxFileType
void wxFileType_delete(wxFileType *self);
wxFileType *wxFileType_new(const wxFileTypeInfo * ft_info);
bool wxFileType_GetDescription(const wxFileType * self, wxString * desc);
bool wxFileType_GetExtensions(wxFileType * self, wxArrayString * extensions);
bool wxFileType_GetIcon(const wxFileType * self, wxIconLocation * icon_loc);
bool wxFileType_GetMimeType(const wxFileType * self, wxString * mime_type);
bool wxFileType_GetMimeTypes(const wxFileType * self, wxArrayString * mime_types);
bool wxFileType_GetOpenCommand(wxFileType * self, wxString * command, const MessageParameters * params);
wxString *wxFileType_GetOpenCommand1(const wxFileType * self, const wxString * filename);
bool wxFileType_GetPrintCommand(const wxFileType * self, wxString * command, const MessageParameters * params);
wxString *wxFileType_GetExpandedCommand(const wxFileType * self, const wxString * verb, const wxFileType::MessageParameters * params);
size_t wxFileType_GetAllCommands(const wxFileType * self, wxArrayString * verbs, wxArrayString * commands, const wxFileType::MessageParameters * params);
wxString *wxFileType_ExpandCommand(const wxString * command, const MessageParameters * params);

// CLASS: wxFilterClassFactory
wxClassInfo *wxFilterClassFactory_CLASSINFO();
const wxFilterClassFactory * wxFilterClassFactory_GetNext(const wxFilterClassFactory * self);
wxString *wxFilterClassFactory_GetProtocol(const wxFilterClassFactory * self);
wxFilterInputStream * wxFilterClassFactory_NewStream(const wxFilterClassFactory * self, wxInputStream * stream);
wxFilterOutputStream * wxFilterClassFactory_NewStream1(const wxFilterClassFactory * self, wxOutputStream * stream);
wxFilterInputStream * wxFilterClassFactory_NewStream2(const wxFilterClassFactory * self, wxInputStream * stream);
wxFilterOutputStream * wxFilterClassFactory_NewStream3(const wxFilterClassFactory * self, wxOutputStream * stream);
wxString *wxFilterClassFactory_PopExtension(const wxFilterClassFactory * self, const wxString * location);
void wxFilterClassFactory_PushFront(wxFilterClassFactory * self);
void wxFilterClassFactory_Remove(wxFilterClassFactory * self);
const wxFilterClassFactory * wxFilterClassFactory_GetFirst();

// CLASS: wxHashMap
void wxHashMap_delete(wxHashMap *self);
wxHashMap *wxHashMap_new1(const wxHashMap * map);
void wxHashMap_clear(wxHashMap * self);
bool wxHashMap_empty(const wxHashMap * self);

// CLASS: wxHashSet
void wxHashSet_delete(wxHashSet *self);
wxHashSet *wxHashSet_new1(const wxHashSet * set);
void wxHashSet_clear(wxHashSet * self);
bool wxHashSet_empty(const wxHashSet * self);

// CLASS: wxHashTable
wxClassInfo *wxHashTable_CLASSINFO();
void wxHashTable_BeginFind(wxHashTable * self);
void wxHashTable_Clear(wxHashTable * self);
wxObject * wxHashTable_Delete(wxHashTable * self, long key);
wxObject * wxHashTable_Delete1(wxHashTable * self, const wxString * key);
void wxHashTable_DeleteContents(wxHashTable * self, bool flag);
wxObject * wxHashTable_Get(wxHashTable * self, long key);
wxObject * wxHashTable_Get1(wxHashTable * self, const char * key);
size_t wxHashTable_GetCount(const wxHashTable * self);
wxHashTable::Node * wxHashTable_Next(wxHashTable * self);
void wxHashTable_Put(wxHashTable * self, long key, wxObject * object);
void wxHashTable_Put1(wxHashTable * self, const char * key, wxObject * object);
long wxHashTable_MakeKey(const wxString * string);

// CLASS: wxIconLocation
void wxIconLocation_delete(wxIconLocation *self);
bool wxIconLocation_IsOk(const wxIconLocation * self);
void wxIconLocation_SetFileName(wxIconLocation * self, const wxString * filename);
wxString *wxIconLocation_GetFileName(const wxIconLocation * self);

// CLASS: wxIdleEvent
wxClassInfo *wxIdleEvent_CLASSINFO();
wxIdleEvent *wxIdleEvent_new();
bool wxIdleEvent_MoreRequested(const wxIdleEvent * self);
void wxIdleEvent_RequestMore(wxIdleEvent * self, bool need_more);

// CLASS: wxInitializer
void wxInitializer_delete(wxInitializer *self);
wxInitializer *wxInitializer_new(int argc, wxChar ** argv);
bool wxInitializer_IsOk(const wxInitializer * self);

// CLASS: wxLocale
void wxLocale_delete(wxLocale *self);
wxLocale *wxLocale_new();
wxLocale *wxLocale_new1(int language, int flags);
wxLocale *wxLocale_new2(const wxString * name, const wxString * short_name, const wxString * locale, bool b_load_default);
bool wxLocale_AddCatalog(wxLocale * self, const wxString * domain);
wxString *wxLocale_GetCanonicalName(const wxLocale * self);
wxString *wxLocale_GetHeaderValue(const wxLocale * self, const wxString * header, const wxString * domain);
int wxLocale_GetLanguage(const wxLocale * self);
wxString *wxLocale_GetLocale(const wxLocale * self);
wxString *wxLocale_GetName(const wxLocale * self);
wxString *wxLocale_GetString(const wxLocale * self, const wxString * orig_string, const wxString * domain);
wxString *wxLocale_GetSysName(const wxLocale * self);
bool wxLocale_Init(wxLocale * self, int language, int flags);
bool wxLocale_Init1(wxLocale * self, const wxString * name, const wxString * short_name, const wxString * locale, bool b_load_default);
bool wxLocale_IsLoaded(const wxLocale * self, const wxString * domain);
bool wxLocale_IsOk(const wxLocale * self);
void wxLocale_AddCatalogLookupPathPrefix(const wxString * prefix);
void wxLocale_AddLanguage(const wxLanguageInfo * info);
const wxLanguageInfo * wxLocale_FindLanguageInfo(const wxString * locale);
const wxLanguageInfo * wxLocale_GetLanguageInfo(int lang);
wxString *wxLocale_GetLanguageName(int lang);
wxString *wxLocale_GetLanguageCanonicalName(int lang);
wxString *wxLocale_GetSystemEncodingName();
int wxLocale_GetSystemLanguage();
bool wxLocale_IsAvailable(int lang);

// CLASS: wxLog
void wxLog_delete(wxLog *self);
void wxLog_AddTraceMask(const wxString * mask);
void wxLog_ClearTraceMasks();
wxArrayString *wxLog_GetTraceMasks();
bool wxLog_IsAllowedTraceMask(const wxString * mask);
void wxLog_RemoveTraceMask(const wxString * mask);
void wxLog_DontCreateOnDemand();
wxLog * wxLog_GetActiveTarget();
wxLog * wxLog_SetActiveTarget(wxLog * logtarget);
wxLog * wxLog_SetThreadActiveTarget(wxLog * logger);
void wxLog_FlushActive();
void wxLog_Resume();
void wxLog_Suspend();
bool wxLog_EnableLogging(bool enable);
bool wxLog_IsEnabled();
bool wxLog_GetRepetitionCounting();
void wxLog_SetRepetitionCounting(bool repet_counting);
wxString *wxLog_GetTimestamp();
void wxLog_SetTimestamp(const wxString * format);
void wxLog_DisableTimestamp();
bool wxLog_GetVerbose();
void wxLog_SetVerbose(bool verbose);
wxLogFormatter * wxLog_SetFormatter(wxLog * self, wxLogFormatter * formatter);
void wxLog_Flush(wxLog * self);

// CLASS: wxLogBuffer
void wxLogBuffer_delete(wxLogBuffer *self);
wxLogBuffer *wxLogBuffer_new();
wxString *wxLogBuffer_GetBuffer(const wxLogBuffer * self);

// CLASS: wxLogChain
void wxLogChain_delete(wxLogChain *self);
wxLogChain *wxLogChain_new(wxLog * logger);
void wxLogChain_DetachOldLog(wxLogChain * self);
wxLog * wxLogChain_GetOldLog(const wxLogChain * self);
bool wxLogChain_IsPassingMessages(const wxLogChain * self);
void wxLogChain_PassMessages(wxLogChain * self, bool pass_messages);
void wxLogChain_SetLog(wxLogChain * self, wxLog * logger);

// CLASS: wxLogFormatter
void wxLogFormatter_delete(wxLogFormatter *self);
wxLogFormatter *wxLogFormatter_new();

// CLASS: wxLogInterposer
void wxLogInterposer_delete(wxLogInterposer *self);
wxLogInterposer *wxLogInterposer_new();

// CLASS: wxLogInterposerTemp
void wxLogInterposerTemp_delete(wxLogInterposerTemp *self);
wxLogInterposerTemp *wxLogInterposerTemp_new();

// CLASS: wxLogNull
void wxLogNull_delete(wxLogNull *self);
wxLogNull *wxLogNull_new();

// CLASS: wxLogStderr
void wxLogStderr_delete(wxLogStderr *self);
wxLogStderr *wxLogStderr_new(FILE * fp, const wxMBConv * conv);

// CLASS: wxLogStream
void wxLogStream_delete(wxLogStream *self);
wxLogStream *wxLogStream_new(std::ostream * ostr, const wxMBConv * conv);

// CLASS: wxMemoryBuffer
void wxMemoryBuffer_delete(wxMemoryBuffer *self);
wxMemoryBuffer *wxMemoryBuffer_new(const wxMemoryBuffer * src);
wxMemoryBuffer *wxMemoryBuffer_new1(size_t size);
void wxMemoryBuffer_AppendData(wxMemoryBuffer * self, const void * data, size_t len);
void wxMemoryBuffer_Clear(wxMemoryBuffer * self);
void * wxMemoryBuffer_GetAppendBuf(wxMemoryBuffer * self, size_t size_needed);
size_t wxMemoryBuffer_GetBufSize(const wxMemoryBuffer * self);
void * wxMemoryBuffer_GetData(const wxMemoryBuffer * self);
size_t wxMemoryBuffer_GetDataLen(const wxMemoryBuffer * self);
void * wxMemoryBuffer_GetWriteBuf(wxMemoryBuffer * self, size_t size_needed);
bool wxMemoryBuffer_IsEmpty(const wxMemoryBuffer * self);
void wxMemoryBuffer_SetBufSize(wxMemoryBuffer * self, size_t size);
void wxMemoryBuffer_SetDataLen(wxMemoryBuffer * self, size_t size);
void wxMemoryBuffer_UngetAppendBuf(wxMemoryBuffer * self, size_t size_used);
void wxMemoryBuffer_UngetWriteBuf(wxMemoryBuffer * self, size_t size_used);

// CLASS: wxMemoryFSHandler
wxClassInfo *wxMemoryFSHandler_CLASSINFO();
wxMemoryFSHandler *wxMemoryFSHandler_new();
void wxMemoryFSHandler_AddFile2(const wxString * filename, const wxString * textdata);
void wxMemoryFSHandler_AddFile3(const wxString * filename, const void * binarydata, size_t size);
void wxMemoryFSHandler_AddFileWithMimeType(const wxString * filename, const wxString * textdata, const wxString * mimetype);
void wxMemoryFSHandler_AddFileWithMimeType1(const wxString * filename, const void * binarydata, size_t size, const wxString * mimetype);
void wxMemoryFSHandler_RemoveFile(const wxString * filename);

// CLASS: wxMessageOutput
void wxMessageOutput_delete(wxMessageOutput *self);
wxMessageOutput * wxMessageOutput_Get();
wxMessageOutput * wxMessageOutput_Set(wxMessageOutput * msgout);
void wxMessageOutput_Output(wxMessageOutput * self, const wxString * str);

// CLASS: wxMessageOutputBest
void wxMessageOutputBest_delete(wxMessageOutputBest *self);

// CLASS: wxMessageOutputDebug
void wxMessageOutputDebug_delete(wxMessageOutputDebug *self);
wxMessageOutputDebug *wxMessageOutputDebug_new();

// CLASS: wxMessageOutputStderr
void wxMessageOutputStderr_delete(wxMessageOutputStderr *self);
wxMessageOutputStderr *wxMessageOutputStderr_new(FILE * fp);

// CLASS: wxMimeTypesManager
void wxMimeTypesManager_delete(wxMimeTypesManager *self);
wxMimeTypesManager *wxMimeTypesManager_new();
void wxMimeTypesManager_AddFallbacks(wxMimeTypesManager * self, const wxFileTypeInfo * fallbacks);
wxFileType * wxMimeTypesManager_GetFileTypeFromExtension(wxMimeTypesManager * self, const wxString * extension);
wxFileType * wxMimeTypesManager_GetFileTypeFromMimeType(wxMimeTypesManager * self, const wxString * mime_type);
wxFileType * wxMimeTypesManager_Associate(wxMimeTypesManager * self, const wxFileTypeInfo * ft_info);
bool wxMimeTypesManager_Unassociate(wxMimeTypesManager * self, wxFileType * ft);
size_t wxMimeTypesManager_EnumAllFileTypes(wxMimeTypesManager * self, wxArrayString * mimetypes);
bool wxMimeTypesManager_IsOfType(const wxString * mime_type, const wxString * wildcard);

// CLASS: wxModule
wxClassInfo *wxModule_CLASSINFO();
wxModule *wxModule_new();
void wxModule_OnExit(wxModule * self);
bool wxModule_OnInit(wxModule * self);

// CLASS: wxMultiChoiceDialog
wxClassInfo *wxMultiChoiceDialog_CLASSINFO();
wxMultiChoiceDialog *wxMultiChoiceDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, int n, const wxString * choices, long style, const wxPoint * pos);
wxMultiChoiceDialog *wxMultiChoiceDialog_new1(wxWindow * parent, const wxString * message, const wxString * caption, const wxArrayString * choices, long style, const wxPoint * pos);
wxArrayInt *wxMultiChoiceDialog_GetSelections(const wxMultiChoiceDialog * self);
void wxMultiChoiceDialog_SetSelections(wxMultiChoiceDialog * self, const wxArrayInt * selections);
// Mix-in(s) to wxMultiChoiceDialog
wxTrackable *wxMultiChoiceDialog_AsTrackable(wxMultiChoiceDialog* obj);

// CLASS: wxMutex
void wxMutex_delete(wxMutex *self);

// CLASS: wxMutexLocker
void wxMutexLocker_delete(wxMutexLocker *self);
wxMutexLocker *wxMutexLocker_new(wxMutex * mutex);
bool wxMutexLocker_IsOk(const wxMutexLocker * self);

// CLASS: wxObject
wxClassInfo *wxObject_CLASSINFO();
wxObject *wxObject_new();
wxObject *wxObject_new1(const wxObject * other);
wxClassInfo * wxObject_GetClassInfo(const wxObject * self);
wxObjectRefData * wxObject_GetRefData(const wxObject * self);
bool wxObject_IsKindOf(const wxObject * self, const wxClassInfo * info);
bool wxObject_IsSameAs(const wxObject * self, const wxObject * obj);
void wxObject_Ref(wxObject * self, const wxObject * clone);
void wxObject_SetRefData(wxObject * self, wxObjectRefData * data);
void wxObject_UnRef(wxObject * self);
void wxObject_UnShare(wxObject * self);

// CLASS: wxObjectRefData
void wxObjectRefData_delete(wxObjectRefData *self);

// CLASS: wxPlatformInfo
void wxPlatformInfo_delete(wxPlatformInfo *self);
wxString *wxPlatformInfo_GetCpuArchitectureName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetNativeCpuArchitectureName(const wxPlatformInfo * self);
int wxPlatformInfo_GetOSMajorVersion(const wxPlatformInfo * self);
int wxPlatformInfo_GetOSMinorVersion(const wxPlatformInfo * self);
int wxPlatformInfo_GetOSMicroVersion(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetOperatingSystemDescription(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetDesktopEnvironment(const wxPlatformInfo * self);
int wxPlatformInfo_GetToolkitMajorVersion(const wxPlatformInfo * self);
int wxPlatformInfo_GetToolkitMinorVersion(const wxPlatformInfo * self);
int wxPlatformInfo_GetToolkitMicroVersion(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetArchName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetBitnessName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetEndiannessName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetOperatingSystemFamilyName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetOperatingSystemIdName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetPortIdName(const wxPlatformInfo * self);
wxString *wxPlatformInfo_GetPortIdShortName(const wxPlatformInfo * self);
void wxPlatformInfo_SetOSVersion(wxPlatformInfo * self, int major, int minor, int micro);
void wxPlatformInfo_SetToolkitVersion(wxPlatformInfo * self, int major, int minor, int micro);
void wxPlatformInfo_SetOperatingSystemDescription(wxPlatformInfo * self, const wxString * desc);
void wxPlatformInfo_SetDesktopEnvironment(wxPlatformInfo * self, const wxString * de);
void wxPlatformInfo_SetLinuxDistributionInfo(wxPlatformInfo * self, const wxLinuxDistributionInfo * di);
wxString *wxPlatformInfo_GetOperatingSystemDirectory();
wxPlatformInfo *wxPlatformInfo_new();
bool wxPlatformInfo_CheckOSVersion(const wxPlatformInfo * self, int major, int minor, int micro);
bool wxPlatformInfo_CheckToolkitVersion(const wxPlatformInfo * self, int major, int minor, int micro);
bool wxPlatformInfo_IsOk(const wxPlatformInfo * self);
bool wxPlatformInfo_IsUsingUniversalWidgets(const wxPlatformInfo * self);
wxPlatformInfo *wxPlatformInfo_Get();

// CLASS: wxPosition
void wxPosition_delete(wxPosition *self);
wxPosition *wxPosition_new();
wxPosition *wxPosition_new1(int row, int col);
int wxPosition_GetCol(const wxPosition * self);
int wxPosition_GetColumn(const wxPosition * self);
int wxPosition_GetRow(const wxPosition * self);
void wxPosition_SetCol(wxPosition * self, int column);
void wxPosition_SetColumn(wxPosition * self, int column);
void wxPosition_SetRow(wxPosition * self, int row);

// CLASS: wxPostScriptDC
wxClassInfo *wxPostScriptDC_CLASSINFO();
wxPostScriptDC *wxPostScriptDC_new();
wxPostScriptDC *wxPostScriptDC_new1(const wxPrintData * print_data);

// CLASS: wxPowerEvent
wxClassInfo *wxPowerEvent_CLASSINFO();
wxPowerEvent *wxPowerEvent_new();
void wxPowerEvent_Veto(wxPowerEvent * self);
bool wxPowerEvent_IsVetoed(const wxPowerEvent * self);

// CLASS: wxPowerResource
void wxPowerResource_delete(wxPowerResource *self);

// CLASS: wxPowerResourceBlocker
void wxPowerResourceBlocker_delete(wxPowerResourceBlocker *self);
bool wxPowerResourceBlocker_IsInEffect(const wxPowerResourceBlocker * self);

// CLASS: wxProcess
wxClassInfo *wxProcess_CLASSINFO();
wxProcess *wxProcess_new(wxEvtHandler * parent, int id);
wxProcess *wxProcess_new1(int flags);
bool wxProcess_Activate(const wxProcess * self);
void wxProcess_CloseOutput(wxProcess * self);
void wxProcess_Detach(wxProcess * self);
wxInputStream * wxProcess_GetErrorStream(const wxProcess * self);
wxInputStream * wxProcess_GetInputStream(const wxProcess * self);
wxOutputStream * wxProcess_GetOutputStream(const wxProcess * self);
long wxProcess_GetPid(const wxProcess * self);
bool wxProcess_IsErrorAvailable(const wxProcess * self);
bool wxProcess_IsInputAvailable(const wxProcess * self);
bool wxProcess_IsInputOpened(const wxProcess * self);
void wxProcess_OnTerminate(wxProcess * self, int pid, int status);
void wxProcess_Redirect(wxProcess * self);
bool wxProcess_Exists(int pid);
wxProcess * wxProcess_Open(const wxString * cmd, int flags);
// Mix-in(s) to wxProcess
wxTrackable *wxProcess_AsTrackable(wxProcess* obj);

// CLASS: wxProcessEvent
wxClassInfo *wxProcessEvent_CLASSINFO();
wxProcessEvent *wxProcessEvent_new(int id, int pid, int exitcode);
int wxProcessEvent_GetExitCode(const wxProcessEvent * self);
int wxProcessEvent_GetPid(const wxProcessEvent * self);

// CLASS: wxRecursionGuard
void wxRecursionGuard_delete(wxRecursionGuard *self);
wxRecursionGuard *wxRecursionGuard_new(wxRecursionGuardFlag * flag);
bool wxRecursionGuard_IsInside(const wxRecursionGuard * self);

// CLASS: wxRecursionGuardFlag
void wxRecursionGuardFlag_delete(wxRecursionGuardFlag *self);

// CLASS: wxRefCounter
void wxRefCounter_delete(wxRefCounter *self);
wxRefCounter *wxRefCounter_new();
void wxRefCounter_DecRef(wxRefCounter * self);
int wxRefCounter_GetRefCount(const wxRefCounter * self);
void wxRefCounter_IncRef(wxRefCounter * self);

// CLASS: wxRegEx
void wxRegEx_delete(wxRegEx *self);
wxRegEx *wxRegEx_new();
wxRegEx *wxRegEx_new1(const wxString * expr, int flags);
bool wxRegEx_Compile(wxRegEx * self, const wxString * pattern, int flags);
bool wxRegEx_GetMatch(const wxRegEx * self, size_t * start, size_t * len, size_t index);
wxString *wxRegEx_GetMatch1(const wxRegEx * self, const wxString * text, size_t index);
size_t wxRegEx_GetMatchCount(const wxRegEx * self);
bool wxRegEx_IsValid(const wxRegEx * self);
bool wxRegEx_Matches(const wxRegEx * self, const wxChar * text, int flags);
bool wxRegEx_Matches1(const wxRegEx * self, const wxChar * text, int flags, size_t len);
bool wxRegEx_Matches2(const wxRegEx * self, const wxString * text, int flags);
int wxRegEx_Replace(const wxRegEx * self, wxString * text, const wxString * replacement, size_t max_matches);
int wxRegEx_ReplaceAll(const wxRegEx * self, wxString * text, const wxString * replacement);
int wxRegEx_ReplaceFirst(const wxRegEx * self, wxString * text, const wxString * replacement);
wxString *wxRegEx_QuoteMeta(const wxString * str);
wxString *wxRegEx_ConvertFromBasic(const wxString * bre);
wxVersionInfo *wxRegEx_GetLibraryVersionInfo();

// CLASS: wxRegKey
void wxRegKey_delete(wxRegKey *self);
wxRegKey *wxRegKey_new3(const wxRegKey * key_parent, const wxString * str_key);
void wxRegKey_Close(wxRegKey * self);
bool wxRegKey_Copy(wxRegKey * self, const wxString * sz_new_name);
bool wxRegKey_Copy1(wxRegKey * self, wxRegKey * key_dst);
bool wxRegKey_CopyValue(wxRegKey * self, const wxString * sz_value, wxRegKey * key_dst, const wxString * sz_new_name);
bool wxRegKey_Create(wxRegKey * self, bool b_ok_if_exists);
void wxRegKey_DeleteKey(wxRegKey * self, const wxString * sz_key);
void wxRegKey_DeleteSelf(wxRegKey * self);
void wxRegKey_DeleteValue(wxRegKey * self, const wxString * sz_key);
bool wxRegKey_Exists(const wxRegKey * self);
bool wxRegKey_Export(const wxRegKey * self, const wxString * filename);
bool wxRegKey_Export1(const wxRegKey * self, wxOutputStream * ostr);
bool wxRegKey_GetFirstKey(wxRegKey * self, wxString * str_key_name, long * l_index);
bool wxRegKey_GetFirstValue(wxRegKey * self, wxString * str_value_name, long * l_index);
bool wxRegKey_GetKeyInfo(const wxRegKey * self, size_t * pn_sub_keys, size_t * pn_max_key_len, size_t * pn_values, size_t * pn_max_value_len);
wxString *wxRegKey_GetName(const wxRegKey * self, bool b_short_prefix);
bool wxRegKey_GetNextKey(const wxRegKey * self, wxString * str_key_name, long * l_index);
bool wxRegKey_GetNextValue(const wxRegKey * self, wxString * str_value_name, long * l_index);
bool wxRegKey_HasSubKey(const wxRegKey * self, const wxString * sz_key);
bool wxRegKey_HasSubkeys(const wxRegKey * self);
bool wxRegKey_HasValue(const wxRegKey * self, const wxString * sz_value);
bool wxRegKey_HasValues(const wxRegKey * self);
bool wxRegKey_IsEmpty(const wxRegKey * self);
bool wxRegKey_IsNumericValue(const wxRegKey * self, const wxString * sz_value);
bool wxRegKey_IsOpened(const wxRegKey * self);
wxString *wxRegKey_QueryDefaultValue(const wxRegKey * self);
bool wxRegKey_QueryRawValue(const wxRegKey * self, const wxString * sz_value, wxString * str_value);
bool wxRegKey_QueryValue(const wxRegKey * self, const wxString * sz_value, wxString * str_value, bool raw);
bool wxRegKey_QueryValue1(const wxRegKey * self, const wxString * sz_value, long * pl_value);
bool wxRegKey_QueryValue64(const wxRegKey * self, const wxString * sz_value, wxLongLong_t * pl_value);
bool wxRegKey_QueryValue2(const wxRegKey * self, const wxString * sz_value, wxMemoryBuffer * buf);
bool wxRegKey_Rename(wxRegKey * self, const wxString * sz_new_name);
bool wxRegKey_RenameValue(wxRegKey * self, const wxString * sz_value_old, const wxString * sz_value_new);
void wxRegKey_ReserveMemoryForName(wxRegKey * self, size_t bytes);
void wxRegKey_SetName(wxRegKey * self, const wxString * str_key);
void wxRegKey_SetName2(wxRegKey * self, const wxRegKey * key_parent, const wxString * str_key);
bool wxRegKey_SetValue(wxRegKey * self, const wxString * sz_value, long l_value);
bool wxRegKey_SetValue1(wxRegKey * self, const wxString * sz_value, const wxString * str_value);
bool wxRegKey_SetValue2(wxRegKey * self, const wxString * sz_value, const wxMemoryBuffer * buf);

// CLASS: wxSecretStore
void wxSecretStore_delete(wxSecretStore *self);
wxSecretStore *wxSecretStore_GetDefault();
bool wxSecretStore_IsOk(const wxSecretStore * self, wxString * errmsg);
bool wxSecretStore_Save(wxSecretStore * self, const wxString * service, const wxString * username, const wxSecretValue * password);
bool wxSecretStore_Load(const wxSecretStore * self, const wxString * service, wxString * username, wxSecretValue * password);
bool wxSecretStore_Delete(wxSecretStore * self, const wxString * service);

// CLASS: wxSecretValue
void wxSecretValue_delete(wxSecretValue *self);
wxSecretValue *wxSecretValue_new();
wxSecretValue *wxSecretValue_new1(size_t size, const void * data);
wxSecretValue *wxSecretValue_new2(const wxString * secret);
wxSecretValue *wxSecretValue_new3(const wxSecretValue * other);
bool wxSecretValue_IsOk(const wxSecretValue * self);
size_t wxSecretValue_GetSize(const wxSecretValue * self);
const void * wxSecretValue_GetData(const wxSecretValue * self);
wxString *wxSecretValue_GetAsString(const wxSecretValue * self, const wxMBConv * conv);
void wxSecretValue_Wipe(size_t size, void * data);
void wxSecretValue_WipeString(wxString * str);

// CLASS: wxSemaphore
void wxSemaphore_delete(wxSemaphore *self);
wxSemaphore *wxSemaphore_new(int initialcount, int maxcount);

// CLASS: wxServer
void wxServer_delete(wxServer *self);
wxServer *wxServer_new();
bool wxServer_Create(wxServer * self, const wxString * service);
wxConnectionBase * wxServer_OnAcceptConnection(wxServer * self, const wxString * topic);

// CLASS: wxSharedClientDataContainer
void wxSharedClientDataContainer_delete(wxSharedClientDataContainer *self);
void * wxSharedClientDataContainer_GetClientData(const wxSharedClientDataContainer * self);
wxClientData * wxSharedClientDataContainer_GetClientObject(const wxSharedClientDataContainer * self);
void wxSharedClientDataContainer_SetClientData(wxSharedClientDataContainer * self, void * data);
void wxSharedClientDataContainer_SetClientObject(wxSharedClientDataContainer * self, wxClientData * data);

// CLASS: wxSingleChoiceDialog
wxClassInfo *wxSingleChoiceDialog_CLASSINFO();
wxSingleChoiceDialog *wxSingleChoiceDialog_new(wxWindow * parent, const wxString * message, const wxString * caption, int n, const wxString * choices, void ** client_data, long style, const wxPoint * pos);
wxSingleChoiceDialog *wxSingleChoiceDialog_new1(wxWindow * parent, const wxString * message, const wxString * caption, const wxArrayString * choices, void ** client_data, long style, const wxPoint * pos);
int wxSingleChoiceDialog_GetSelection(const wxSingleChoiceDialog * self);
void * wxSingleChoiceDialog_GetSelectionData(const wxSingleChoiceDialog * self);
wxString *wxSingleChoiceDialog_GetStringSelection(const wxSingleChoiceDialog * self);
void wxSingleChoiceDialog_SetSelection(wxSingleChoiceDialog * self, int selection);
// Mix-in(s) to wxSingleChoiceDialog
wxTrackable *wxSingleChoiceDialog_AsTrackable(wxSingleChoiceDialog* obj);

// CLASS: wxSingleInstanceChecker
void wxSingleInstanceChecker_delete(wxSingleInstanceChecker *self);
wxSingleInstanceChecker *wxSingleInstanceChecker_new();
wxSingleInstanceChecker *wxSingleInstanceChecker_new1(const wxString * name, const wxString * path);
bool wxSingleInstanceChecker_Create(wxSingleInstanceChecker * self, const wxString * name, const wxString * path);
bool wxSingleInstanceChecker_CreateDefault(wxSingleInstanceChecker * self);
bool wxSingleInstanceChecker_IsAnotherRunning(const wxSingleInstanceChecker * self);

// CLASS: wxStackFrame
void wxStackFrame_delete(wxStackFrame *self);
void * wxStackFrame_GetAddress(const wxStackFrame * self);
wxString *wxStackFrame_GetFileName(const wxStackFrame * self);
size_t wxStackFrame_GetLevel(const wxStackFrame * self);
size_t wxStackFrame_GetLine(const wxStackFrame * self);
wxString *wxStackFrame_GetModule(const wxStackFrame * self);
wxString *wxStackFrame_GetName(const wxStackFrame * self);
size_t wxStackFrame_GetOffset(const wxStackFrame * self);
bool wxStackFrame_GetParam(const wxStackFrame * self, size_t n, wxString * type_, wxString * name, wxString * value);
size_t wxStackFrame_GetParamCount(const wxStackFrame * self);
bool wxStackFrame_HasSourceLocation(const wxStackFrame * self);

// CLASS: wxStackWalker
void wxStackWalker_delete(wxStackWalker *self);
wxStackWalker *wxStackWalker_new(const char * argv0);
void wxStackWalker_Walk(wxStackWalker * self, size_t skip, size_t max_depth);
void wxStackWalker_WalkFromException(wxStackWalker * self, size_t max_depth);

// CLASS: wxStandardPaths
void wxStandardPaths_delete(wxStandardPaths *self);
#ifdef __WXMSW__
void wxStandardPaths_DontIgnoreAppSubDir(wxStandardPaths * self);
#endif
wxString *wxStandardPaths_GetAppDocumentsDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetConfigDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetDataDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetDocumentsDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetExecutablePath(const wxStandardPaths * self);
#ifdef __WXGTK__
wxString *wxStandardPaths_GetInstallPrefix(const wxStandardPaths * self);
#endif
wxString *wxStandardPaths_GetLocalDataDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetPluginsDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetResourcesDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetTempDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetUserConfigDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetUserDataDir(const wxStandardPaths * self);
wxString *wxStandardPaths_GetUserLocalDataDir(const wxStandardPaths * self);
#ifdef __WXMSW__
void wxStandardPaths_IgnoreAppSubDir(wxStandardPaths * self, const wxString * subdir_pattern);
void wxStandardPaths_IgnoreAppBuildSubDirs(wxStandardPaths * self);
#endif
#ifdef __WXGTK__
void wxStandardPaths_SetInstallPrefix(wxStandardPaths * self, const wxString * prefix);
#endif
void wxStandardPaths_UseAppInfo(wxStandardPaths * self, int info);
wxStandardPaths * wxStandardPaths_Get();
#ifdef __WXMSW__
wxString *wxStandardPaths_MSWGetShellDir(int csidl);
#endif

// CLASS: wxStopWatch
void wxStopWatch_delete(wxStopWatch *self);
wxStopWatch *wxStopWatch_new();
void wxStopWatch_Pause(wxStopWatch * self);
void wxStopWatch_Resume(wxStopWatch * self);
void wxStopWatch_Start(wxStopWatch * self, long milliseconds);
long wxStopWatch_Time(const wxStopWatch * self);

// CLASS: wxStreamBuffer
void wxStreamBuffer_delete(wxStreamBuffer *self);
wxStreamBuffer *wxStreamBuffer_new1(size_t bufsize, wxInputStream * stream);
wxStreamBuffer *wxStreamBuffer_new2(size_t bufsize, wxOutputStream * stream);
wxStreamBuffer *wxStreamBuffer_new4(const wxStreamBuffer * buffer);
bool wxStreamBuffer_FillBuffer(wxStreamBuffer * self);
void wxStreamBuffer_Fixed(wxStreamBuffer * self, bool fixed);
bool wxStreamBuffer_FlushBuffer(wxStreamBuffer * self);
void wxStreamBuffer_Flushable(wxStreamBuffer * self, bool flushable);
void * wxStreamBuffer_GetBufferEnd(const wxStreamBuffer * self);
void * wxStreamBuffer_GetBufferPos(const wxStreamBuffer * self);
size_t wxStreamBuffer_GetBufferSize(const wxStreamBuffer * self);
void * wxStreamBuffer_GetBufferStart(const wxStreamBuffer * self);
size_t wxStreamBuffer_GetDataLeft(wxStreamBuffer * self);
size_t wxStreamBuffer_GetIntPosition(const wxStreamBuffer * self);
size_t wxStreamBuffer_GetLastAccess(const wxStreamBuffer * self);
size_t wxStreamBuffer_Read(wxStreamBuffer * self, void * buffer, size_t size);
size_t wxStreamBuffer_Read1(wxStreamBuffer * self, wxStreamBuffer * buffer);
void wxStreamBuffer_ResetBuffer(wxStreamBuffer * self);
void wxStreamBuffer_SetBufferIO(wxStreamBuffer * self, void * start, void * end, bool take_ownership);
void wxStreamBuffer_SetBufferIO1(wxStreamBuffer * self, size_t bufsize);
void wxStreamBuffer_SetIntPosition(wxStreamBuffer * self, size_t pos);
wxStreamBase * wxStreamBuffer_Stream(wxStreamBuffer * self);
void wxStreamBuffer_Truncate(wxStreamBuffer * self);
size_t wxStreamBuffer_Write(wxStreamBuffer * self, const void * buffer, size_t size);
size_t wxStreamBuffer_Write1(wxStreamBuffer * self, wxStreamBuffer * buffer);

// CLASS: wxStringClientData
void wxStringClientData_delete(wxStringClientData *self);
wxStringClientData *wxStringClientData_new();
wxStringClientData *wxStringClientData_new1(const wxString * data);
wxString *wxStringClientData_GetData(const wxStringClientData * self);
void wxStringClientData_SetData(wxStringClientData * self, const wxString * data);

// CLASS: wxStringTokenizer
wxClassInfo *wxStringTokenizer_CLASSINFO();
wxStringTokenizer *wxStringTokenizer_new();
size_t wxStringTokenizer_CountTokens(const wxStringTokenizer * self);
wxString *wxStringTokenizer_GetNextToken(wxStringTokenizer * self);
size_t wxStringTokenizer_GetPosition(const wxStringTokenizer * self);
wxString *wxStringTokenizer_GetString(const wxStringTokenizer * self);
bool wxStringTokenizer_HasMoreTokens(const wxStringTokenizer * self);

// CLASS: wxSystemOptions
wxClassInfo *wxSystemOptions_CLASSINFO();
wxSystemOptions *wxSystemOptions_new();
wxString *wxSystemOptions_GetOption(const wxString * name);
int wxSystemOptions_GetOptionInt(const wxString * name);
bool wxSystemOptions_HasOption(const wxString * name);
bool wxSystemOptions_IsFalse(const wxString * name);
void wxSystemOptions_SetOption(const wxString * name, const wxString * value);
void wxSystemOptions_SetOption1(const wxString * name, int value);

// CLASS: wxTarClassFactory
wxClassInfo *wxTarClassFactory_CLASSINFO();

// CLASS: wxTempFFile
void wxTempFFile_delete(wxTempFFile *self);
wxTempFFile *wxTempFFile_new();
wxTempFFile *wxTempFFile_new1(const wxString * str_name);
bool wxTempFFile_Commit(wxTempFFile * self);
void wxTempFFile_Discard(wxTempFFile * self);
bool wxTempFFile_Flush(wxTempFFile * self);
bool wxTempFFile_IsOpened(const wxTempFFile * self);
bool wxTempFFile_Open(wxTempFFile * self, const wxString * str_name);
bool wxTempFFile_Write(wxTempFFile * self, const wxString * str, const wxMBConv * conv);

// CLASS: wxTempFile
void wxTempFile_delete(wxTempFile *self);
wxTempFile *wxTempFile_new();
wxTempFile *wxTempFile_new1(const wxString * str_name);
bool wxTempFile_Commit(wxTempFile * self);
void wxTempFile_Discard(wxTempFile * self);
bool wxTempFile_Flush(wxTempFile * self);
bool wxTempFile_IsOpened(const wxTempFile * self);
bool wxTempFile_Open(wxTempFile * self, const wxString * str_name);
bool wxTempFile_Write(wxTempFile * self, const wxString * str, const wxMBConv * conv);

// CLASS: wxThreadHelper
void wxThreadHelper_delete(wxThreadHelper *self);
void wxThreadHelper_OnDelete(wxThreadHelper * self);
void wxThreadHelper_OnKill(wxThreadHelper * self);
void wxThreadHelper_OnExit(wxThreadHelper * self);
wxThread * wxThreadHelper_GetThread(const wxThreadHelper * self);

// CLASS: wxTimer
wxClassInfo *wxTimer_CLASSINFO();
wxTimer *wxTimer_new();
wxTimer *wxTimer_new1(wxEvtHandler * owner, int id);
int wxTimer_GetId(const wxTimer * self);
int wxTimer_GetInterval(const wxTimer * self);
wxEvtHandler * wxTimer_GetOwner(const wxTimer * self);
bool wxTimer_IsOneShot(const wxTimer * self);
bool wxTimer_IsRunning(const wxTimer * self);
void wxTimer_Notify(wxTimer * self);
void wxTimer_SetOwner(wxTimer * self, wxEvtHandler * owner, int id);
bool wxTimer_Start(wxTimer * self, int milliseconds, bool one_shot);
bool wxTimer_StartOnce(wxTimer * self, int milliseconds);
void wxTimer_Stop(wxTimer * self);
// Mix-in(s) to wxTimer
wxTrackable *wxTimer_AsTrackable(wxTimer* obj);

// CLASS: wxTimerEvent
wxClassInfo *wxTimerEvent_CLASSINFO();
wxTimerEvent *wxTimerEvent_new(wxTimer * timer);
int wxTimerEvent_GetInterval(const wxTimerEvent * self);
wxTimer * wxTimerEvent_GetTimer(const wxTimerEvent * self);

// CLASS: wxTrackable
void wxTrackable_delete(wxTrackable *self);

// CLASS: wxUILocale
void wxUILocale_delete(wxUILocale *self);
bool wxUILocale_UseDefault();
wxUILocale *wxUILocale_GetCurrent();
wxUILocale *wxUILocale_FromTag(const wxString * tag);
void wxUILocale_AddLanguage(const wxLanguageInfo * info);
const wxLanguageInfo * wxUILocale_FindLanguageInfo(const wxString * locale);
const wxLanguageInfo * wxUILocale_FindLanguageInfo1(const wxLocaleIdent * locale_id);
const wxLanguageInfo * wxUILocale_GetLanguageInfo(int lang);
wxString *wxUILocale_GetLanguageName(int lang);
wxString *wxUILocale_GetLanguageCanonicalName(int lang);
int wxUILocale_GetSystemLanguage();
int wxUILocale_GetSystemLocale();
wxUILocale *wxUILocale_new(const wxLocaleIdent * locale_id);
int wxUILocale_CompareStrings(const wxUILocale * self, const wxString * lhs, const wxString * rhs, int flags);
wxString *wxUILocale_GetName(const wxUILocale * self);
wxLocaleIdent *wxUILocale_GetLocaleId(const wxUILocale * self);
wxLayoutDirection wxUILocale_GetLayoutDirection(const wxUILocale * self);
bool wxUILocale_IsSupported(const wxUILocale * self);

// CLASS: wxURI
wxClassInfo *wxURI_CLASSINFO();
wxURI *wxURI_new();
wxURI *wxURI_new1(const wxString * uri);
wxURI *wxURI_new2(const wxURI * uri);
wxString *wxURI_BuildURI(const wxURI * self);
wxString *wxURI_BuildUnescapedURI(const wxURI * self);
bool wxURI_Create(wxURI * self, const wxString * uri);
wxString *wxURI_GetFragment(const wxURI * self);
wxString *wxURI_GetPassword(const wxURI * self);
wxString *wxURI_GetPath(const wxURI * self);
wxString *wxURI_GetPort(const wxURI * self);
wxString *wxURI_GetQuery(const wxURI * self);
wxString *wxURI_GetScheme(const wxURI * self);
wxString *wxURI_GetServer(const wxURI * self);
wxString *wxURI_GetUser(const wxURI * self);
wxString *wxURI_GetUserInfo(const wxURI * self);
bool wxURI_HasFragment(const wxURI * self);
bool wxURI_HasPath(const wxURI * self);
bool wxURI_HasPort(const wxURI * self);
bool wxURI_HasQuery(const wxURI * self);
bool wxURI_HasScheme(const wxURI * self);
bool wxURI_HasServer(const wxURI * self);
bool wxURI_HasUserInfo(const wxURI * self);
bool wxURI_IsReference(const wxURI * self);
void wxURI_Resolve(wxURI * self, const wxURI * base, int flags);
wxString *wxURI_Unescape(const wxString * uri);

// CLASS: wxUniCharRef
void wxUniCharRef_delete(wxUniCharRef *self);

// CLASS: wxVariantData
void wxVariantData_delete(wxVariantData *self);
wxVariantData *wxVariantData_new();
wxVariantData * wxVariantData_Clone(const wxVariantData * self);
void wxVariantData_DecRef(wxVariantData * self);
bool wxVariantData_Eq(const wxVariantData * self, wxVariantData * data);
bool wxVariantData_GetAny(const wxVariantData * self, wxAny * any);
wxString *wxVariantData_GetType(const wxVariantData * self);
wxClassInfo * wxVariantData_GetValueClassInfo(wxVariantData * self);
void wxVariantData_IncRef(wxVariantData * self);
bool wxVariantData_Read(wxVariantData * self, istream * stream);
bool wxVariantData_Read1(wxVariantData * self, wxString * string);
bool wxVariantData_Write(const wxVariantData * self, ostream * stream);
bool wxVariantData_Write1(const wxVariantData * self, wxString * string);

// CLASS: wxVersionInfo
void wxVersionInfo_delete(wxVersionInfo *self);
wxVersionInfo *wxVersionInfo_new(const wxString * name, int major, int minor, int micro, int revision, const wxString * description, const wxString * copyright);
wxString *wxVersionInfo_GetName(const wxVersionInfo * self);
int wxVersionInfo_GetMajor(const wxVersionInfo * self);
int wxVersionInfo_GetMinor(const wxVersionInfo * self);
int wxVersionInfo_GetMicro(const wxVersionInfo * self);
int wxVersionInfo_GetRevision(const wxVersionInfo * self);
wxString *wxVersionInfo_ToString(const wxVersionInfo * self);
wxString *wxVersionInfo_GetVersionString(const wxVersionInfo * self);
bool wxVersionInfo_HasDescription(const wxVersionInfo * self);
wxString *wxVersionInfo_GetDescription(wxVersionInfo * self);
bool wxVersionInfo_HasCopyright(const wxVersionInfo * self);
wxString *wxVersionInfo_GetCopyright(const wxVersionInfo * self);

// CLASS: wxWindowUpdateLocker
void wxWindowUpdateLocker_delete(wxWindowUpdateLocker *self);
wxWindowUpdateLocker *wxWindowUpdateLocker_new();
wxWindowUpdateLocker *wxWindowUpdateLocker_new1(wxWindow * win);
void wxWindowUpdateLocker_Lock(wxWindowUpdateLocker * self, wxWindow * win);

// CLASS: wxZipClassFactory
wxClassInfo *wxZipClassFactory_CLASSINFO();

// CLASS: wxZipNotifier
void wxZipNotifier_delete(wxZipNotifier *self);
void wxZipNotifier_OnEntryUpdated(wxZipNotifier * self, wxZipEntry * entry);

} // extern "C"

