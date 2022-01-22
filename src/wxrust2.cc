#include "wx/include/wxrust.h"
#include "wx/include/wxrust2.h"

namespace wxrust {

// Constructors

// CLASS: wxButton
wxButton *NewButton() {
    return new wxButton();
}

wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, long style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}

} // namespace wxrust

