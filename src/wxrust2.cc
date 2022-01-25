#include "wx/include/wxrust.h"
#include "wx/include/wxrust2.h"

namespace wxrust {

// Constructors

// CLASS: wxObject
wxObject *NewObject() {
    return new wxObject();
}
wxObject *NewObject1(const wxObject & other) {
    return new wxObject(other);
}

// CLASS: wxEvtHandler
wxEvtHandler *NewEvtHandler() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
wxWindow *NewWindow() {
    return new wxWindow();
}
wxWindow *NewWindow1(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxWindow(parent, id, pos, size, style, name);
}

// CLASS: wxControl
wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxControl(parent, id, pos, size, style, validator, name);
}
wxControl *NewControl1() {
    return new wxControl();
}

// CLASS: wxAnyButton
wxAnyButton *NewAnyButton() {
    return new wxAnyButton();
}

// CLASS: wxButton
wxButton *NewButton() {
    return new wxButton();
}
wxButton *NewButton1(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}

} // namespace wxrust

