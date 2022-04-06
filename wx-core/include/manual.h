#pragma once
#include <wx/wx.h>

extern "C" {
    int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y);
}
