#pragma once

#include <wx/msgout.h>

extern "C" {

// CLASS: wxMessageOutput
void wxMessageOutput_delete(wxMessageOutput *self);
wxMessageOutput * wxMessageOutput_Get();
wxMessageOutput * wxMessageOutput_Set(wxMessageOutput * msgout);
void wxMessageOutput_Output(wxMessageOutput * self, const wxString * str);

} // extern "C"

