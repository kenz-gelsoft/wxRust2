#pragma once

#include <wx/timer.h>

extern "C" {

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

// CLASS: wxTimerEvent
wxClassInfo *wxTimerEvent_CLASSINFO();
wxTimerEvent *wxTimerEvent_new(wxTimer * timer);
int wxTimerEvent_GetInterval(const wxTimerEvent * self);
wxTimer * wxTimerEvent_GetTimer(const wxTimerEvent * self);

} // extern "C"

