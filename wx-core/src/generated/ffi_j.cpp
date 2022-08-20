#include "generated.h"

extern "C" {

// CLASS: wxJPEGHandler
wxClassInfo *wxJPEGHandler_CLASSINFO() {
    return wxCLASSINFO(wxJPEGHandler);
}
wxJPEGHandler *wxJPEGHandler_new() {
    return new wxJPEGHandler();
}

// CLASS: wxJoystickEvent
wxClassInfo *wxJoystickEvent_CLASSINFO() {
    return wxCLASSINFO(wxJoystickEvent);
}
bool wxJoystickEvent_ButtonDown(const wxJoystickEvent * self, int button) {
    return self->ButtonDown(button);
}
bool wxJoystickEvent_ButtonIsDown(const wxJoystickEvent * self, int button) {
    return self->ButtonIsDown(button);
}
bool wxJoystickEvent_ButtonUp(const wxJoystickEvent * self, int button) {
    return self->ButtonUp(button);
}
int wxJoystickEvent_GetButtonChange(const wxJoystickEvent * self) {
    return self->GetButtonChange();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxJoystickEvent_GetButtonOrdinal(const wxJoystickEvent * self) {
    return self->GetButtonOrdinal();
}
#endif
int wxJoystickEvent_GetButtonState(const wxJoystickEvent * self) {
    return self->GetButtonState();
}
int wxJoystickEvent_GetJoystick(const wxJoystickEvent * self) {
    return self->GetJoystick();
}
wxPoint *wxJoystickEvent_GetPosition(const wxJoystickEvent * self) {
    return new wxPoint(self->GetPosition());
}
int wxJoystickEvent_GetZPosition(const wxJoystickEvent * self) {
    return self->GetZPosition();
}
bool wxJoystickEvent_IsButton(const wxJoystickEvent * self) {
    return self->IsButton();
}
bool wxJoystickEvent_IsMove(const wxJoystickEvent * self) {
    return self->IsMove();
}
bool wxJoystickEvent_IsZMove(const wxJoystickEvent * self) {
    return self->IsZMove();
}

} // extern "C"

