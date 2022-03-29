#pragma once
#include <wx/wx.h>

extern "C" {

// CLASS: wxObject
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

// CLASS: wxEvtHandler
void wxEvtHandler_QueueEvent(wxEvtHandler * self, wxEvent * event);
void wxEvtHandler_AddPendingEvent(wxEvtHandler * self, const wxEvent * event);
bool wxEvtHandler_ProcessEvent(wxEvtHandler * self, wxEvent * event);
bool wxEvtHandler_ProcessEventLocally(wxEvtHandler * self, wxEvent * event);
bool wxEvtHandler_SafelyProcessEvent(wxEvtHandler * self, wxEvent * event);
void wxEvtHandler_ProcessPendingEvents(wxEvtHandler * self);
void wxEvtHandler_DeletePendingEvents(wxEvtHandler * self);
bool wxEvtHandler_SearchEventTable(wxEvtHandler * self, wxEventTable * table, wxEvent * event);
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

} // extern "C"

