#pragma once

#include <wx/event.h>
#include <wx/imagjpeg.h>
#include <wx/joystick.h>

extern "C" {

// CLASS: wxJPEGHandler
wxClassInfo *wxJPEGHandler_CLASSINFO();
wxJPEGHandler *wxJPEGHandler_new();

// CLASS: wxJoystick
wxClassInfo *wxJoystick_CLASSINFO();
wxJoystick *wxJoystick_new(int joystick);
int wxJoystick_GetButtonState(const wxJoystick * self);
bool wxJoystick_GetButtonState1(const wxJoystick * self, unsigned int id);
int wxJoystick_GetManufacturerId(const wxJoystick * self);
int wxJoystick_GetMovementThreshold(const wxJoystick * self);
int wxJoystick_GetNumberAxes(const wxJoystick * self);
int wxJoystick_GetNumberButtons(const wxJoystick * self);
int wxJoystick_GetPOVCTSPosition(const wxJoystick * self);
int wxJoystick_GetPOVPosition(const wxJoystick * self);
int wxJoystick_GetPollingMax(const wxJoystick * self);
int wxJoystick_GetPollingMin(const wxJoystick * self);
wxPoint *wxJoystick_GetPosition(const wxJoystick * self);
int wxJoystick_GetPosition1(const wxJoystick * self, unsigned int axis);
int wxJoystick_GetProductId(const wxJoystick * self);
wxString *wxJoystick_GetProductName(const wxJoystick * self);
int wxJoystick_GetRudderMax(const wxJoystick * self);
int wxJoystick_GetRudderMin(const wxJoystick * self);
int wxJoystick_GetRudderPosition(const wxJoystick * self);
int wxJoystick_GetUMax(const wxJoystick * self);
int wxJoystick_GetUMin(const wxJoystick * self);
int wxJoystick_GetUPosition(const wxJoystick * self);
int wxJoystick_GetVMax(const wxJoystick * self);
int wxJoystick_GetVMin(const wxJoystick * self);
int wxJoystick_GetVPosition(const wxJoystick * self);
int wxJoystick_GetXMax(const wxJoystick * self);
int wxJoystick_GetXMin(const wxJoystick * self);
int wxJoystick_GetYMax(const wxJoystick * self);
int wxJoystick_GetYMin(const wxJoystick * self);
int wxJoystick_GetZMax(const wxJoystick * self);
int wxJoystick_GetZMin(const wxJoystick * self);
int wxJoystick_GetZPosition(const wxJoystick * self);
bool wxJoystick_HasPOV(const wxJoystick * self);
bool wxJoystick_HasPOV4Dir(const wxJoystick * self);
bool wxJoystick_HasPOVCTS(const wxJoystick * self);
bool wxJoystick_HasRudder(const wxJoystick * self);
bool wxJoystick_HasU(const wxJoystick * self);
bool wxJoystick_HasV(const wxJoystick * self);
bool wxJoystick_HasZ(const wxJoystick * self);
bool wxJoystick_IsOk(const wxJoystick * self);
bool wxJoystick_ReleaseCapture(wxJoystick * self);
bool wxJoystick_SetCapture(wxJoystick * self, wxWindow * win, int polling_freq);
void wxJoystick_SetMovementThreshold(wxJoystick * self, int threshold);
int wxJoystick_GetNumberJoysticks();

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

