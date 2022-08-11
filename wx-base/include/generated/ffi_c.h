#pragma once
#include <wx/wx.h>
#include <wx/object.h>

typedef wxDateTime::TimeZone TimeZone;
typedef wxDateTime::Tm       Tm;
typedef wxDateTime::WeekDay  WeekDay;

extern "C" {

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

} // extern "C"

