#include "generated.h"

extern "C" {

// CLASS: wxTimer
wxClassInfo *wxTimer_CLASSINFO() {
    return wxCLASSINFO(wxTimer);
}
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
wxClassInfo *wxTimerEvent_CLASSINFO() {
    return wxCLASSINFO(wxTimerEvent);
}
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

