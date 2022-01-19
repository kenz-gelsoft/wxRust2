#include "wx/include/wxrust.h"

namespace wxrust {

// Constructors

// CLASS: wxButton
wxButton *NewButton(self: Pin<&mut wxButton>) {
    return new wxButton(self: Pin<&mut wxButton>);
}

wxButton *NewButton(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString) {
    return new wxButton(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);
}

} // namespace wxrust

