#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxButton
wxButton *NewButton(self: Pin<&mut wxButton>);
wxButton *NewButton(self: Pin<&mut wxButton>, parent: *mut wxWindow, id: i32, label: &wxString, pos: &wxPoint, size: &wxSize, style: i32, validator: &wxValidator, name: &wxString);

} // namespace wxrust

