#include "wx/include/wxrust.h"
#include "wx/include/wxrust2.h"

namespace wxrust {

// Constructors

// CLASS: wxEvtHandler
wxEvtHandler *NewEvtHandler() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
wxWindow *NewWindow() {
    return new wxWindow();
}

wxWindow *NewWindow(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, long style, const wxString & name) {
    return new wxWindow(parent, id, pos, size, style, name);
}

// CLASS: wxControl
wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, long style, const wxValidator & validator, const wxString & name) {
    return new wxControl(parent, id, pos, size, style, validator, name);
}

wxControl *NewControl() {
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

wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, long style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}

} // namespace wxrust

