#include "generated.h"

extern "C" {

// CLASS: wxClassInfo
void wxClassInfo_delete(wxClassInfo *self) {
    delete self;
}
wxObject * wxClassInfo_CreateObject(const wxClassInfo * self) {
    return self->CreateObject();
}
const wxChar * wxClassInfo_GetBaseClassName1(const wxClassInfo * self) {
    return self->GetBaseClassName1();
}
const wxChar * wxClassInfo_GetBaseClassName2(const wxClassInfo * self) {
    return self->GetBaseClassName2();
}
const wxChar * wxClassInfo_GetClassName(const wxClassInfo * self) {
    return self->GetClassName();
}
int wxClassInfo_GetSize(const wxClassInfo * self) {
    return self->GetSize();
}
bool wxClassInfo_IsDynamic(const wxClassInfo * self) {
    return self->IsDynamic();
}
bool wxClassInfo_IsKindOf(const wxClassInfo * self, const wxClassInfo * info) {
    return self->IsKindOf(info);
}
wxClassInfo * wxClassInfo_FindClass(const wxString * class_name) {
    return wxClassInfo::FindClass(*class_name);
}

// CLASS: wxDateTime
void wxDateTime_delete(wxDateTime *self) {
    delete self;
}
wxDateTime *wxDateTime_new() {
    return new wxDateTime();
}
wxDateTime *wxDateTime_new1(const wxDateTime * date) {
    return new wxDateTime(*date);
}
wxDateTime *wxDateTime_new4(double jdn) {
    return new wxDateTime(jdn);
}
wxDateTime * wxDateTime_ResetTime(wxDateTime * self) {
    return &(self->ResetTime());
}
wxDateTime * wxDateTime_Set2(wxDateTime * self, const Tm * tm) {
    return &(self->Set(*tm));
}
wxDateTime * wxDateTime_Set3(wxDateTime * self, double jdn) {
    return &(self->Set(jdn));
}
wxDateTime * wxDateTime_SetToCurrent(wxDateTime * self) {
    return &(self->SetToCurrent());
}
wxDateTime * wxDateTime_SetYear(wxDateTime * self, int year) {
    return &(self->SetYear(year));
}
#ifdef __WXMSW__
wxDateTime * wxDateTime_SetFromMSWSysTime(wxDateTime * self, const struct _SYSTEMTIME * st) {
    return &(self->SetFromMSWSysTime(*st));
}
void wxDateTime_GetAsMSWSysTime(const wxDateTime * self, struct _SYSTEMTIME * st) {
    return self->GetAsMSWSysTime(st);
}
#endif
int wxDateTime_GetCentury(const wxDateTime * self, const TimeZone * tz) {
    return self->GetCentury(*tz);
}
wxDateTime *wxDateTime_GetDateOnly(const wxDateTime * self) {
    return new wxDateTime(self->GetDateOnly());
}
#if wxCHECK_VERSION(3, 1, 0)
int wxDateTime_GetWeekBasedYear(const wxDateTime * self, const TimeZone * tz) {
    return self->GetWeekBasedYear(*tz);
}
#endif
int wxDateTime_GetYear(const wxDateTime * self, const TimeZone * tz) {
    return self->GetYear(*tz);
}
bool wxDateTime_IsValid(const wxDateTime * self) {
    return self->IsValid();
}
bool wxDateTime_IsEarlierThan(const wxDateTime * self, const wxDateTime * datetime) {
    return self->IsEarlierThan(*datetime);
}
bool wxDateTime_IsEqualTo(const wxDateTime * self, const wxDateTime * datetime) {
    return self->IsEqualTo(*datetime);
}
bool wxDateTime_IsEqualUpTo(const wxDateTime * self, const wxDateTime * dt, const wxTimeSpan * ts) {
    return self->IsEqualUpTo(*dt, *ts);
}
bool wxDateTime_IsLaterThan(const wxDateTime * self, const wxDateTime * datetime) {
    return self->IsLaterThan(*datetime);
}
bool wxDateTime_IsSameDate(const wxDateTime * self, const wxDateTime * dt) {
    return self->IsSameDate(*dt);
}
bool wxDateTime_IsSameTime(const wxDateTime * self, const wxDateTime * dt) {
    return self->IsSameTime(*dt);
}
bool wxDateTime_IsStrictlyBetween(const wxDateTime * self, const wxDateTime * t1, const wxDateTime * t2) {
    return self->IsStrictlyBetween(*t1, *t2);
}
bool wxDateTime_IsBetween(const wxDateTime * self, const wxDateTime * t1, const wxDateTime * t2) {
    return self->IsBetween(*t1, *t2);
}
wxDateTime * wxDateTime_Add1(wxDateTime * self, const wxDateSpan * diff) {
    return &(self->Add(*diff));
}
wxDateTime * wxDateTime_Add3(wxDateTime * self, const wxTimeSpan * diff) {
    return &(self->Add(*diff));
}
wxDateTime * wxDateTime_Subtract1(wxDateTime * self, const wxTimeSpan * diff) {
    return &(self->Subtract(*diff));
}
wxDateTime * wxDateTime_Subtract3(wxDateTime * self, const wxDateSpan * diff) {
    return &(self->Subtract(*diff));
}
wxString *wxDateTime_Format(const wxDateTime * self, const wxString * format, const TimeZone * tz) {
    return new wxString(self->Format(*format, *tz));
}
wxString *wxDateTime_FormatDate(const wxDateTime * self) {
    return new wxString(self->FormatDate());
}
wxString *wxDateTime_FormatISODate(const wxDateTime * self) {
    return new wxString(self->FormatISODate());
}
wxString *wxDateTime_FormatISOTime(const wxDateTime * self) {
    return new wxString(self->FormatISOTime());
}
wxString *wxDateTime_FormatTime(const wxDateTime * self) {
    return new wxString(self->FormatTime());
}
bool wxDateTime_ParseDateTime(wxDateTime * self, const wxString * datetime, wxString::const_iterator * end) {
    return self->ParseDateTime(*datetime, end);
}
bool wxDateTime_ParseFormat(wxDateTime * self, const wxString * date, const wxString * format, const wxDateTime * date_def, wxString::const_iterator * end) {
    return self->ParseFormat(*date, *format, *date_def, end);
}
bool wxDateTime_ParseISODate(wxDateTime * self, const wxString * date) {
    return self->ParseISODate(*date);
}
bool wxDateTime_ParseISOTime(wxDateTime * self, const wxString * date) {
    return self->ParseISOTime(*date);
}
bool wxDateTime_ParseRfc822Date(wxDateTime * self, const wxString * date, wxString::const_iterator * end) {
    return self->ParseRfc822Date(*date, end);
}
bool wxDateTime_ParseTime(wxDateTime * self, const wxString * time, wxString::const_iterator * end) {
    return self->ParseTime(*time, end);
}
double wxDateTime_GetJDN(const wxDateTime * self) {
    return self->GetJDN();
}
double wxDateTime_GetJulianDayNumber(const wxDateTime * self) {
    return self->GetJulianDayNumber();
}
double wxDateTime_GetMJD(const wxDateTime * self) {
    return self->GetMJD();
}
double wxDateTime_GetModifiedJulianDayNumber(const wxDateTime * self) {
    return self->GetModifiedJulianDayNumber();
}
double wxDateTime_GetRataDie(const wxDateTime * self) {
    return self->GetRataDie();
}
wxDateTime *wxDateTime_FromTimezone(const wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return new wxDateTime(self->FromTimezone(*tz, no_dst));
}
wxDateTime * wxDateTime_MakeFromTimezone(wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return &(self->MakeFromTimezone(*tz, no_dst));
}
wxDateTime * wxDateTime_MakeTimezone(wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return &(self->MakeTimezone(*tz, no_dst));
}
wxDateTime * wxDateTime_MakeUTC(wxDateTime * self, bool no_dst) {
    return &(self->MakeUTC(no_dst));
}
wxDateTime *wxDateTime_ToTimezone(const wxDateTime * self, const TimeZone * tz, bool no_dst) {
    return new wxDateTime(self->ToTimezone(*tz, no_dst));
}
wxDateTime *wxDateTime_ToUTC(const wxDateTime * self, bool no_dst) {
    return new wxDateTime(self->ToUTC(no_dst));
}
int wxDateTime_ConvertYearToBC(int year) {
    return wxDateTime::ConvertYearToBC(year);
}
void wxDateTime_GetAmPmStrings(wxString * am, wxString * pm) {
    return wxDateTime::GetAmPmStrings(am, pm);
}
int wxDateTime_GetCentury1(int year) {
    return wxDateTime::GetCentury(year);
}
tm * wxDateTime_GetTmNow1() {
    return wxDateTime::GetTmNow();
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxDateTime_GetFirstWeekDay(WeekDay * first_day) {
    return wxDateTime::GetFirstWeekDay(first_day);
}
#endif
wxDateTime *wxDateTime_Now() {
    return new wxDateTime(wxDateTime::Now());
}
wxDateTime *wxDateTime_Today() {
    return new wxDateTime(wxDateTime::Today());
}
wxDateTime *wxDateTime_UNow() {
    return new wxDateTime(wxDateTime::UNow());
}

// CLASS: wxEvent
wxEvent * wxEvent_Clone(const wxEvent * self) {
    return self->Clone();
}
wxObject * wxEvent_GetEventObject(const wxEvent * self) {
    return self->GetEventObject();
}
int wxEvent_GetId(const wxEvent * self) {
    return self->GetId();
}
wxObject * wxEvent_GetEventUserData(const wxEvent * self) {
    return self->GetEventUserData();
}
bool wxEvent_GetSkipped(const wxEvent * self) {
    return self->GetSkipped();
}
long wxEvent_GetTimestamp(const wxEvent * self) {
    return self->GetTimestamp();
}
bool wxEvent_IsCommandEvent(const wxEvent * self) {
    return self->IsCommandEvent();
}
void wxEvent_ResumePropagation(wxEvent * self, int propagation_level) {
    return self->ResumePropagation(propagation_level);
}
void wxEvent_SetEventObject(wxEvent * self, wxObject * object) {
    return self->SetEventObject(object);
}
void wxEvent_SetId(wxEvent * self, int id) {
    return self->SetId(id);
}
void wxEvent_SetTimestamp(wxEvent * self, long time_stamp) {
    return self->SetTimestamp(time_stamp);
}
bool wxEvent_ShouldPropagate(const wxEvent * self) {
    return self->ShouldPropagate();
}
void wxEvent_Skip(wxEvent * self, bool skip) {
    return self->Skip(skip);
}
int wxEvent_StopPropagation(wxEvent * self) {
    return self->StopPropagation();
}

// CLASS: wxEvtHandler
void wxEvtHandler_QueueEvent(wxEvtHandler * self, wxEvent * event) {
    return self->QueueEvent(event);
}
void wxEvtHandler_AddPendingEvent(wxEvtHandler * self, const wxEvent * event) {
    return self->AddPendingEvent(*event);
}
bool wxEvtHandler_ProcessEvent(wxEvtHandler * self, wxEvent * event) {
    return self->ProcessEvent(*event);
}
bool wxEvtHandler_ProcessEventLocally(wxEvtHandler * self, wxEvent * event) {
    return self->ProcessEventLocally(*event);
}
bool wxEvtHandler_SafelyProcessEvent(wxEvtHandler * self, wxEvent * event) {
    return self->SafelyProcessEvent(*event);
}
void wxEvtHandler_ProcessPendingEvents(wxEvtHandler * self) {
    return self->ProcessPendingEvents();
}
void wxEvtHandler_DeletePendingEvents(wxEvtHandler * self) {
    return self->DeletePendingEvents();
}
wxClientData * wxEvtHandler_GetClientObject(const wxEvtHandler * self) {
    return self->GetClientObject();
}
void wxEvtHandler_SetClientObject(wxEvtHandler * self, wxClientData * data) {
    return self->SetClientObject(data);
}
bool wxEvtHandler_GetEvtHandlerEnabled(const wxEvtHandler * self) {
    return self->GetEvtHandlerEnabled();
}
wxEvtHandler * wxEvtHandler_GetNextHandler(const wxEvtHandler * self) {
    return self->GetNextHandler();
}
wxEvtHandler * wxEvtHandler_GetPreviousHandler(const wxEvtHandler * self) {
    return self->GetPreviousHandler();
}
void wxEvtHandler_SetEvtHandlerEnabled(wxEvtHandler * self, bool enabled) {
    return self->SetEvtHandlerEnabled(enabled);
}
void wxEvtHandler_SetNextHandler(wxEvtHandler * self, wxEvtHandler * handler) {
    return self->SetNextHandler(handler);
}
void wxEvtHandler_SetPreviousHandler(wxEvtHandler * self, wxEvtHandler * handler) {
    return self->SetPreviousHandler(handler);
}
void wxEvtHandler_Unlink(wxEvtHandler * self) {
    return self->Unlink();
}
bool wxEvtHandler_IsUnlinked(const wxEvtHandler * self) {
    return self->IsUnlinked();
}
void wxEvtHandler_AddFilter(wxEventFilter * filter) {
    return wxEvtHandler::AddFilter(filter);
}
void wxEvtHandler_RemoveFilter(wxEventFilter * filter) {
    return wxEvtHandler::RemoveFilter(filter);
}
wxEvtHandler *wxEvtHandler_new() {
    return new wxEvtHandler();
}

// CLASS: wxFileName
void wxFileName_delete(wxFileName *self) {
    delete self;
}
wxFileName *wxFileName_new() {
    return new wxFileName();
}
wxFileName *wxFileName_new1(const wxFileName * filename) {
    return new wxFileName(*filename);
}
bool wxFileName_AppendDir(wxFileName * self, const wxString * dir) {
    return self->AppendDir(*dir);
}
void wxFileName_Assign(wxFileName * self, const wxFileName * filepath) {
    return self->Assign(*filepath);
}
void wxFileName_AssignCwd(wxFileName * self, const wxString * volume) {
    return self->AssignCwd(*volume);
}
void wxFileName_AssignHomeDir(wxFileName * self) {
    return self->AssignHomeDir();
}
void wxFileName_AssignTempFileName(wxFileName * self, const wxString * prefix) {
    return self->AssignTempFileName(*prefix);
}
void wxFileName_AssignTempFileName1(wxFileName * self, const wxString * prefix, wxFile * file_temp) {
    return self->AssignTempFileName(*prefix, file_temp);
}
void wxFileName_AssignTempFileName2(wxFileName * self, const wxString * prefix, wxFFile * file_temp) {
    return self->AssignTempFileName(*prefix, file_temp);
}
void wxFileName_Clear(wxFileName * self) {
    return self->Clear();
}
void wxFileName_ClearExt(wxFileName * self) {
    return self->ClearExt();
}
bool wxFileName_DirExists(const wxFileName * self) {
    return self->DirExists();
}
void wxFileName_DontFollowLink(wxFileName * self) {
    return self->DontFollowLink();
}
bool wxFileName_Exists(const wxFileName * self, int flags) {
    return self->Exists(flags);
}
bool wxFileName_FileExists(const wxFileName * self) {
    return self->FileExists();
}
size_t wxFileName_GetDirCount(const wxFileName * self) {
    return self->GetDirCount();
}
wxArrayString *wxFileName_GetDirs(const wxFileName * self) {
    return new wxArrayString(self->GetDirs());
}
wxString *wxFileName_GetExt(const wxFileName * self) {
    return new wxString(self->GetExt());
}
wxString *wxFileName_GetFullName(const wxFileName * self) {
    return new wxString(self->GetFullName());
}
wxString *wxFileName_GetLongPath(const wxFileName * self) {
    return new wxString(self->GetLongPath());
}
wxDateTime *wxFileName_GetModificationTime(const wxFileName * self) {
    return new wxDateTime(self->GetModificationTime());
}
wxString *wxFileName_GetName(const wxFileName * self) {
    return new wxString(self->GetName());
}
wxString *wxFileName_GetShortPath(const wxFileName * self) {
    return new wxString(self->GetShortPath());
}
bool wxFileName_GetTimes(const wxFileName * self, wxDateTime * dt_access, wxDateTime * dt_mod, wxDateTime * dt_create) {
    return self->GetTimes(dt_access, dt_mod, dt_create);
}
wxString *wxFileName_GetVolume(const wxFileName * self) {
    return new wxString(self->GetVolume());
}
bool wxFileName_HasExt(const wxFileName * self) {
    return self->HasExt();
}
bool wxFileName_HasName(const wxFileName * self) {
    return self->HasName();
}
bool wxFileName_HasVolume(const wxFileName * self) {
    return self->HasVolume();
}
bool wxFileName_InsertDir(wxFileName * self, size_t before, const wxString * dir) {
    return self->InsertDir(before, *dir);
}
bool wxFileName_IsDir(const wxFileName * self) {
    return self->IsDir();
}
bool wxFileName_IsDirReadable(const wxFileName * self) {
    return self->IsDirReadable();
}
bool wxFileName_IsDirWritable(const wxFileName * self) {
    return self->IsDirWritable();
}
bool wxFileName_IsFileExecutable(const wxFileName * self) {
    return self->IsFileExecutable();
}
bool wxFileName_IsFileReadable(const wxFileName * self) {
    return self->IsFileReadable();
}
bool wxFileName_IsFileWritable(const wxFileName * self) {
    return self->IsFileWritable();
}
bool wxFileName_IsOk(const wxFileName * self) {
    return self->IsOk();
}
bool wxFileName_Mkdir(const wxFileName * self, int perm, int flags) {
    return self->Mkdir(perm, flags);
}
void wxFileName_PrependDir(wxFileName * self, const wxString * dir) {
    return self->PrependDir(*dir);
}
void wxFileName_RemoveDir(wxFileName * self, size_t pos) {
    return self->RemoveDir(pos);
}
void wxFileName_RemoveLastDir(wxFileName * self) {
    return self->RemoveLastDir();
}
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_ResolveLink(wxFileName * self) {
    return new wxFileName(self->ResolveLink());
}
#endif
bool wxFileName_Rmdir(const wxFileName * self, int flags) {
    return self->Rmdir(flags);
}
bool wxFileName_SetCwd(const wxFileName * self) {
    return self->SetCwd();
}
void wxFileName_SetEmptyExt(wxFileName * self) {
    return self->SetEmptyExt();
}
void wxFileName_SetExt(wxFileName * self, const wxString * ext) {
    return self->SetExt(*ext);
}
void wxFileName_SetFullName(wxFileName * self, const wxString * fullname) {
    return self->SetFullName(*fullname);
}
void wxFileName_SetName(wxFileName * self, const wxString * name) {
    return self->SetName(*name);
}
bool wxFileName_SetPermissions(wxFileName * self, int permissions) {
    return self->SetPermissions(permissions);
}
bool wxFileName_SetTimes(const wxFileName * self, const wxDateTime * dt_access, const wxDateTime * dt_mod, const wxDateTime * dt_create) {
    return self->SetTimes(dt_access, dt_mod, dt_create);
}
void wxFileName_SetVolume(wxFileName * self, const wxString * volume) {
    return self->SetVolume(*volume);
}
bool wxFileName_ShouldFollowLink(const wxFileName * self) {
    return self->ShouldFollowLink();
}
bool wxFileName_Touch(const wxFileName * self) {
    return self->Touch();
}
wxString *wxFileName_CreateTempFileName(const wxString * prefix, wxFile * file_temp) {
    return new wxString(wxFileName::CreateTempFileName(*prefix, file_temp));
}
wxString *wxFileName_CreateTempFileName1(const wxString * prefix, wxFFile * file_temp) {
    return new wxString(wxFileName::CreateTempFileName(*prefix, file_temp));
}
bool wxFileName_DirExists1(const wxString * dir) {
    return wxFileName::DirExists(*dir);
}
bool wxFileName_Exists1(const wxString * path, int flags) {
    return wxFileName::Exists(*path, flags);
}
bool wxFileName_FileExists1(const wxString * file) {
    return wxFileName::FileExists(*file);
}
wxString *wxFileName_GetCwd(const wxString * volume) {
    return new wxString(wxFileName::GetCwd(*volume));
}
wxString *wxFileName_GetHomeDir() {
    return new wxString(wxFileName::GetHomeDir());
}
wxString *wxFileName_GetTempDir() {
    return new wxString(wxFileName::GetTempDir());
}
bool wxFileName_IsDirReadable1(const wxString * dir) {
    return wxFileName::IsDirReadable(*dir);
}
bool wxFileName_IsDirWritable1(const wxString * dir) {
    return wxFileName::IsDirWritable(*dir);
}
bool wxFileName_IsFileExecutable1(const wxString * file) {
    return wxFileName::IsFileExecutable(*file);
}
bool wxFileName_IsFileReadable1(const wxString * file) {
    return wxFileName::IsFileReadable(*file);
}
bool wxFileName_IsFileWritable1(const wxString * file) {
    return wxFileName::IsFileWritable(*file);
}
bool wxFileName_Mkdir1(const wxString * dir, int perm, int flags) {
    return wxFileName::Mkdir(*dir, perm, flags);
}
bool wxFileName_Rmdir1(const wxString * dir, int flags) {
    return wxFileName::Rmdir(*dir, flags);
}
bool wxFileName_SetCwd1(const wxString * cwd) {
    return wxFileName::SetCwd(*cwd);
}
#if wxCHECK_VERSION(3, 1, 0)
wxFileName *wxFileName_URLToFileName(const wxString * url) {
    return new wxFileName(wxFileName::URLToFileName(*url));
}
wxString *wxFileName_FileNameToURL(const wxFileName * filename) {
    return new wxString(wxFileName::FileNameToURL(*filename));
}
#endif
wxString *wxFileName_StripExtension(const wxString * fullname) {
    return new wxString(wxFileName::StripExtension(*fullname));
}

// CLASS: wxObject
wxObject *wxObject_new() {
    return new wxObject();
}
wxObject *wxObject_new1(const wxObject * other) {
    return new wxObject(*other);
}
wxClassInfo * wxObject_GetClassInfo(const wxObject * self) {
    return self->GetClassInfo();
}
wxObjectRefData * wxObject_GetRefData(const wxObject * self) {
    return self->GetRefData();
}
bool wxObject_IsKindOf(const wxObject * self, const wxClassInfo * info) {
    return self->IsKindOf(info);
}
bool wxObject_IsSameAs(const wxObject * self, const wxObject * obj) {
    return self->IsSameAs(*obj);
}
void wxObject_Ref(wxObject * self, const wxObject * clone) {
    return self->Ref(*clone);
}
void wxObject_SetRefData(wxObject * self, wxObjectRefData * data) {
    return self->SetRefData(data);
}
void wxObject_UnRef(wxObject * self) {
    return self->UnRef();
}
void wxObject_UnShare(wxObject * self) {
    return self->UnShare();
}

// CLASS: wxStandardPaths
void wxStandardPaths_delete(wxStandardPaths *self) {
    delete self;
}
#ifdef __WXMSW__
void wxStandardPaths_DontIgnoreAppSubDir(wxStandardPaths * self) {
    return self->DontIgnoreAppSubDir();
}
#endif
wxString *wxStandardPaths_GetAppDocumentsDir(const wxStandardPaths * self) {
    return new wxString(self->GetAppDocumentsDir());
}
wxString *wxStandardPaths_GetConfigDir(const wxStandardPaths * self) {
    return new wxString(self->GetConfigDir());
}
wxString *wxStandardPaths_GetDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetDataDir());
}
wxString *wxStandardPaths_GetDocumentsDir(const wxStandardPaths * self) {
    return new wxString(self->GetDocumentsDir());
}
wxString *wxStandardPaths_GetExecutablePath(const wxStandardPaths * self) {
    return new wxString(self->GetExecutablePath());
}
#ifdef __WXGTK__
wxString *wxStandardPaths_GetInstallPrefix(const wxStandardPaths * self) {
    return new wxString(self->GetInstallPrefix());
}
#endif
wxString *wxStandardPaths_GetLocalDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetLocalDataDir());
}
wxString *wxStandardPaths_GetPluginsDir(const wxStandardPaths * self) {
    return new wxString(self->GetPluginsDir());
}
wxString *wxStandardPaths_GetResourcesDir(const wxStandardPaths * self) {
    return new wxString(self->GetResourcesDir());
}
wxString *wxStandardPaths_GetTempDir(const wxStandardPaths * self) {
    return new wxString(self->GetTempDir());
}
wxString *wxStandardPaths_GetUserConfigDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserConfigDir());
}
wxString *wxStandardPaths_GetUserDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserDataDir());
}
wxString *wxStandardPaths_GetUserLocalDataDir(const wxStandardPaths * self) {
    return new wxString(self->GetUserLocalDataDir());
}
#ifdef __WXMSW__
void wxStandardPaths_IgnoreAppSubDir(wxStandardPaths * self, const wxString * subdir_pattern) {
    return self->IgnoreAppSubDir(*subdir_pattern);
}
void wxStandardPaths_IgnoreAppBuildSubDirs(wxStandardPaths * self) {
    return self->IgnoreAppBuildSubDirs();
}
#endif
#ifdef __WXGTK__
void wxStandardPaths_SetInstallPrefix(wxStandardPaths * self, const wxString * prefix) {
    return self->SetInstallPrefix(*prefix);
}
#endif
void wxStandardPaths_UseAppInfo(wxStandardPaths * self, int info) {
    return self->UseAppInfo(info);
}
wxStandardPaths * wxStandardPaths_Get() {
    return &(wxStandardPaths::Get());
}
#ifdef __WXMSW__
wxString *wxStandardPaths_MSWGetShellDir(int csidl) {
    return new wxString(wxStandardPaths::MSWGetShellDir(csidl));
}
#endif

// CLASS: wxTimer
wxTimer *wxTimer_new() {
    return new wxTimer();
}
wxTimer *wxTimer_new1(wxEvtHandler * owner, int id) {
    return new wxTimer(owner, id);
}
int wxTimer_GetId(const wxTimer * self) {
    return self->GetId();
}
int wxTimer_GetInterval(const wxTimer * self) {
    return self->GetInterval();
}
wxEvtHandler * wxTimer_GetOwner(const wxTimer * self) {
    return self->GetOwner();
}
bool wxTimer_IsOneShot(const wxTimer * self) {
    return self->IsOneShot();
}
bool wxTimer_IsRunning(const wxTimer * self) {
    return self->IsRunning();
}
void wxTimer_Notify(wxTimer * self) {
    return self->Notify();
}
void wxTimer_SetOwner(wxTimer * self, wxEvtHandler * owner, int id) {
    return self->SetOwner(owner, id);
}
bool wxTimer_Start(wxTimer * self, int milliseconds, bool one_shot) {
    return self->Start(milliseconds, one_shot);
}
bool wxTimer_StartOnce(wxTimer * self, int milliseconds) {
    return self->StartOnce(milliseconds);
}
void wxTimer_Stop(wxTimer * self) {
    return self->Stop();
}

// CLASS: wxTimerEvent
wxTimerEvent *wxTimerEvent_new(wxTimer * timer) {
    return new wxTimerEvent(*timer);
}
int wxTimerEvent_GetInterval(const wxTimerEvent * self) {
    return self->GetInterval();
}
wxTimer * wxTimerEvent_GetTimer(const wxTimerEvent * self) {
    return &(self->GetTimer());
}

} // extern "C"

