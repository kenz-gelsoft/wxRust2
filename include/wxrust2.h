#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxAnyButton
wxAnyButton *NewAnyButton();

// CLASS: wxButton
wxButton *NewButton();
wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, long style, const wxValidator & validator, const wxString & name);

// CLASS: wxWindow
wxWindow *NewWindow();
wxWindow *NewWindow(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, long style, const wxString & name);

} // namespace wxrust

