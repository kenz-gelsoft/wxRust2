#pragma once
#include <wx/wx.h>

extern "C" {
    // WindowList
    wxWindowList *wxWindowList_new();
    void wxWindowList_delete(wxWindowList *self);
    bool wxWindowList_IsEmpty(wxWindowList *self);

    int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y);
}
