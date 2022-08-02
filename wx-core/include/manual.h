#pragma once
#include <wx/wx.h>

#define DECL_WX_LIST_BINDING(wxclass)                   \
    wxclass##List *wxclass##List_new();                 \
    void wxclass##List_delete(wxclass##List *self);     \
    size_t wxclass##List_GetCount(wxclass##List *self); \
    bool wxclass##List_IsEmpty(wxclass##List *self);

extern "C" {
    // wxList<T>
    DECL_WX_LIST_BINDING(wxSizerItem);
    DECL_WX_LIST_BINDING(wxWindow);

    // wxBitmapBundle compatibility hack(for a while)
    void *wxBitmapBundle_From(wxBitmap *bitmap);

    int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y);
}
