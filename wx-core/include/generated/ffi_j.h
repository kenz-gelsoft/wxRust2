#pragma once

#include <wx/event.h>
#include <wx/imagjpeg.h>

extern "C" {

// CLASS: wxJPEGHandler
wxClassInfo *wxJPEGHandler_CLASSINFO();
wxJPEGHandler *wxJPEGHandler_new();

// CLASS: wxJoystickEvent
wxClassInfo *wxJoystickEvent_CLASSINFO();
bool wxJoystickEvent_ButtonDown(const wxJoystickEvent * self, int button);
bool wxJoystickEvent_ButtonIsDown(const wxJoystickEvent * self, int button);
bool wxJoystickEvent_ButtonUp(const wxJoystickEvent * self, int button);
int wxJoystickEvent_GetButtonChange(const wxJoystickEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxJoystickEvent_GetButtonOrdinal(const wxJoystickEvent * self);
#endif
int wxJoystickEvent_GetButtonState(const wxJoystickEvent * self);
int wxJoystickEvent_GetJoystick(const wxJoystickEvent * self);
wxPoint *wxJoystickEvent_GetPosition(const wxJoystickEvent * self);
int wxJoystickEvent_GetZPosition(const wxJoystickEvent * self);
bool wxJoystickEvent_IsButton(const wxJoystickEvent * self);
bool wxJoystickEvent_IsMove(const wxJoystickEvent * self);
bool wxJoystickEvent_IsZMove(const wxJoystickEvent * self);

} // extern "C"

