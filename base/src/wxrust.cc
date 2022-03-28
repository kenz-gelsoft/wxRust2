#include "wxrust.h"

// String
wxString *wxString_new(const unsigned char *psz, const size_t nLength) {
    return new wxString(psz, wxConvUTF8, nLength);
}
const char *wxString_UTF8Data(wxString *self) {
    return self->ToUTF8().data();
}
size_t wxString_Len(wxString *self) {
    return self->Len();
}

int wxRustEntry(int *argc, char **argv) {
    return wxEntry(*argc, argv);
}
