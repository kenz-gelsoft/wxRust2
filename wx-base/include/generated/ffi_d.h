#pragma once
#include <wx/wx.h>
#include <wx/datetime.h>

typedef wxDateTime::TimeZone TimeZone;
typedef wxDateTime::Tm       Tm;
typedef wxDateTime::WeekDay  WeekDay;

extern "C" {

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

} // extern "C"

