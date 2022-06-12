#include "generated.h"

extern "C" {

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
wxDateTime * wxDateTime_SetFromMSWSysTime(wxDateTime * self, const struct _SYSTEMTIME * st) {
    return &(self->SetFromMSWSysTime(*st));
}
void wxDateTime_GetAsMSWSysTime(const wxDateTime * self, struct _SYSTEMTIME * st) {
    return self->GetAsMSWSysTime(st);
}
int wxDateTime_GetCentury(const wxDateTime * self, const TimeZone * tz) {
    return self->GetCentury(*tz);
}
wxDateTime *wxDateTime_GetDateOnly(const wxDateTime * self) {
    return new wxDateTime(self->GetDateOnly());
}
int wxDateTime_GetWeekBasedYear(const wxDateTime * self, const TimeZone * tz) {
    return self->GetWeekBasedYear(*tz);
}
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
wxDateTime *wxDateTime_Add(const wxDateTime * self, const wxDateSpan * diff) {
    return new wxDateTime(self->Add(*diff));
}
wxDateTime * wxDateTime_Add1(wxDateTime * self, const wxDateSpan * diff) {
    return &(self->Add(*diff));
}
wxDateTime *wxDateTime_Add2(const wxDateTime * self, const wxTimeSpan * diff) {
    return new wxDateTime(self->Add(*diff));
}
wxDateTime * wxDateTime_Add3(wxDateTime * self, const wxTimeSpan * diff) {
    return &(self->Add(*diff));
}
wxDateTime *wxDateTime_Subtract(const wxDateTime * self, const wxTimeSpan * diff) {
    return new wxDateTime(self->Subtract(*diff));
}
wxDateTime * wxDateTime_Subtract1(wxDateTime * self, const wxTimeSpan * diff) {
    return &(self->Subtract(*diff));
}
wxDateTime *wxDateTime_Subtract2(const wxDateTime * self, const wxDateSpan * diff) {
    return new wxDateTime(self->Subtract(*diff));
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
bool wxDateTime_ParseDate(wxDateTime * self, const wxString * date, wxString::const_iterator * end) {
    return self->ParseDate(*date, end);
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
bool wxDateTime_GetFirstWeekDay(WeekDay * first_day) {
    return wxDateTime::GetFirstWeekDay(first_day);
}
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

} // extern "C"

