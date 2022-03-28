#pragma once
#include <wx/wx.h>

extern "C" {

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength);

int wxRustEntry(int *argc, char **argv);

} // extern "C"
