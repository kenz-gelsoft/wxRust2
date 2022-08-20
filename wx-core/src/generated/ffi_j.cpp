#include "generated.h"

extern "C" {

// CLASS: wxJPEGHandler
wxClassInfo *wxJPEGHandler_CLASSINFO() {
    return wxCLASSINFO(wxJPEGHandler);
}
wxJPEGHandler *wxJPEGHandler_new() {
    return new wxJPEGHandler();
}

// CLASS: wxJoystick
wxClassInfo *wxJoystick_CLASSINFO() {
    return wxCLASSINFO(wxJoystick);
}
wxJoystick *wxJoystick_new(int joystick) {
    return new wxJoystick(joystick);
}
int wxJoystick_GetButtonState(const wxJoystick * self) {
    return self->GetButtonState();
}
bool wxJoystick_GetButtonState1(const wxJoystick * self, unsigned int id) {
    return self->GetButtonState(id);
}
int wxJoystick_GetManufacturerId(const wxJoystick * self) {
    return self->GetManufacturerId();
}
int wxJoystick_GetMovementThreshold(const wxJoystick * self) {
    return self->GetMovementThreshold();
}
int wxJoystick_GetNumberAxes(const wxJoystick * self) {
    return self->GetNumberAxes();
}
int wxJoystick_GetNumberButtons(const wxJoystick * self) {
    return self->GetNumberButtons();
}
int wxJoystick_GetPOVCTSPosition(const wxJoystick * self) {
    return self->GetPOVCTSPosition();
}
int wxJoystick_GetPOVPosition(const wxJoystick * self) {
    return self->GetPOVPosition();
}
int wxJoystick_GetPollingMax(const wxJoystick * self) {
    return self->GetPollingMax();
}
int wxJoystick_GetPollingMin(const wxJoystick * self) {
    return self->GetPollingMin();
}
wxPoint *wxJoystick_GetPosition(const wxJoystick * self) {
    return new wxPoint(self->GetPosition());
}
int wxJoystick_GetPosition1(const wxJoystick * self, unsigned int axis) {
    return self->GetPosition(axis);
}
int wxJoystick_GetProductId(const wxJoystick * self) {
    return self->GetProductId();
}
wxString *wxJoystick_GetProductName(const wxJoystick * self) {
    return new wxString(self->GetProductName());
}
int wxJoystick_GetRudderMax(const wxJoystick * self) {
    return self->GetRudderMax();
}
int wxJoystick_GetRudderMin(const wxJoystick * self) {
    return self->GetRudderMin();
}
int wxJoystick_GetRudderPosition(const wxJoystick * self) {
    return self->GetRudderPosition();
}
int wxJoystick_GetUMax(const wxJoystick * self) {
    return self->GetUMax();
}
int wxJoystick_GetUMin(const wxJoystick * self) {
    return self->GetUMin();
}
int wxJoystick_GetUPosition(const wxJoystick * self) {
    return self->GetUPosition();
}
int wxJoystick_GetVMax(const wxJoystick * self) {
    return self->GetVMax();
}
int wxJoystick_GetVMin(const wxJoystick * self) {
    return self->GetVMin();
}
int wxJoystick_GetVPosition(const wxJoystick * self) {
    return self->GetVPosition();
}
int wxJoystick_GetXMax(const wxJoystick * self) {
    return self->GetXMax();
}
int wxJoystick_GetXMin(const wxJoystick * self) {
    return self->GetXMin();
}
int wxJoystick_GetYMax(const wxJoystick * self) {
    return self->GetYMax();
}
int wxJoystick_GetYMin(const wxJoystick * self) {
    return self->GetYMin();
}
int wxJoystick_GetZMax(const wxJoystick * self) {
    return self->GetZMax();
}
int wxJoystick_GetZMin(const wxJoystick * self) {
    return self->GetZMin();
}
int wxJoystick_GetZPosition(const wxJoystick * self) {
    return self->GetZPosition();
}
bool wxJoystick_HasPOV(const wxJoystick * self) {
    return self->HasPOV();
}
bool wxJoystick_HasPOV4Dir(const wxJoystick * self) {
    return self->HasPOV4Dir();
}
bool wxJoystick_HasPOVCTS(const wxJoystick * self) {
    return self->HasPOVCTS();
}
bool wxJoystick_HasRudder(const wxJoystick * self) {
    return self->HasRudder();
}
bool wxJoystick_HasU(const wxJoystick * self) {
    return self->HasU();
}
bool wxJoystick_HasV(const wxJoystick * self) {
    return self->HasV();
}
bool wxJoystick_HasZ(const wxJoystick * self) {
    return self->HasZ();
}
bool wxJoystick_IsOk(const wxJoystick * self) {
    return self->IsOk();
}
bool wxJoystick_ReleaseCapture(wxJoystick * self) {
    return self->ReleaseCapture();
}
bool wxJoystick_SetCapture(wxJoystick * self, wxWindow * win, int polling_freq) {
    return self->SetCapture(win, polling_freq);
}
void wxJoystick_SetMovementThreshold(wxJoystick * self, int threshold) {
    return self->SetMovementThreshold(threshold);
}
int wxJoystick_GetNumberJoysticks() {
    return wxJoystick::GetNumberJoysticks();
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

