#pragma once
#include <wx/wx.h>

extern "C" {

// CLASS: wxEvent
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

} // extern "C"

