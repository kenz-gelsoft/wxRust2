#include "wxrust.h"

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}

int wxRustEntry(int *argc, char **argv) {
    return wxEntry(*argc, argv);
}
