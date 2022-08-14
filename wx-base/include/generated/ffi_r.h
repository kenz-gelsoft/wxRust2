#pragma once

#include <wx/object.h>

extern "C" {

// CLASS: wxRefCounter
wxRefCounter *wxRefCounter_new();
void wxRefCounter_DecRef(wxRefCounter * self);
int wxRefCounter_GetRefCount(const wxRefCounter * self);
void wxRefCounter_IncRef(wxRefCounter * self);

} // extern "C"

