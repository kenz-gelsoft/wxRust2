#include "generated.h"

extern "C" {

// CLASS: wxKeyEvent
wxClassInfo *wxKeyEvent_CLASSINFO() {
    return wxCLASSINFO(wxKeyEvent);
}
int wxKeyEvent_GetKeyCode(const wxKeyEvent * self) {
    return self->GetKeyCode();
}
bool wxKeyEvent_IsKeyInCategory(const wxKeyEvent * self, int category) {
    return self->IsKeyInCategory(category);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxKeyEvent_IsAutoRepeat(const wxKeyEvent * self) {
    return self->IsAutoRepeat();
}
#endif
wxPoint *wxKeyEvent_GetPosition(const wxKeyEvent * self) {
    return new wxPoint(self->GetPosition());
}
void wxKeyEvent_GetPosition1(const wxKeyEvent * self, wxCoord * x, wxCoord * y) {
    return self->GetPosition(x, y);
}
wxCoord wxKeyEvent_GetX(const wxKeyEvent * self) {
    return self->GetX();
}
wxCoord wxKeyEvent_GetY(const wxKeyEvent * self) {
    return self->GetY();
}
void wxKeyEvent_DoAllowNextEvent(wxKeyEvent * self) {
    return self->DoAllowNextEvent();
}
bool wxKeyEvent_IsNextEventAllowed(const wxKeyEvent * self) {
    return self->IsNextEventAllowed();
}

} // extern "C"

