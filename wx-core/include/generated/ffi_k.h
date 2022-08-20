#pragma once

#include <wx/event.h>

extern "C" {

// CLASS: wxKeyEvent
wxClassInfo *wxKeyEvent_CLASSINFO();
int wxKeyEvent_GetKeyCode(const wxKeyEvent * self);
bool wxKeyEvent_IsKeyInCategory(const wxKeyEvent * self, int category);
bool wxKeyEvent_IsAutoRepeat(const wxKeyEvent * self);
wxPoint *wxKeyEvent_GetPosition(const wxKeyEvent * self);
void wxKeyEvent_GetPosition1(const wxKeyEvent * self, wxCoord * x, wxCoord * y);
wxCoord wxKeyEvent_GetX(const wxKeyEvent * self);
wxCoord wxKeyEvent_GetY(const wxKeyEvent * self);
void wxKeyEvent_DoAllowNextEvent(wxKeyEvent * self);
bool wxKeyEvent_IsNextEventAllowed(const wxKeyEvent * self);

} // extern "C"

