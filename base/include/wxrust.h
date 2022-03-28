#pragma once
#include <wx/wx.h>

extern "C" {

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength);
const char *wxString_UTF8Data(wxString *self);
size_t wxString_Len(wxString *self);

int wxRustEntry(int *argc, char **argv);

} // extern "C"
