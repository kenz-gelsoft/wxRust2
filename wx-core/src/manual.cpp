#include "manual.h"

// WindowList
wxWindowList *wxWindowList_new() {
    return new wxWindowList();
}
void wxWindowList_delete(wxWindowList *self) {
    delete self;
}
bool wxWindowList_IsEmpty(wxWindowList *self) {
    return self->IsEmpty();
}

int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y) {
    return wxMessageBox(*message, *caption, style, parent, x, y);
}
