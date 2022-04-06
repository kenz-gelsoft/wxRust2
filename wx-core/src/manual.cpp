#include "manual.h"

int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y) {
    return wxMessageBox(*message, *caption, style, parent, x, y);
}