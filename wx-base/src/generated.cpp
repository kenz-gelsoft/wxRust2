#include "generated.h"

extern "C" {

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
bool wxEvtHandler_SearchEventTable(wxEvtHandler * self, wxEventTable * table, wxEvent * event) {
    return self->SearchEventTable(*table, *event);
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

} // extern "C"

