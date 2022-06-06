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

    int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y);

    // wxItemContainer
    wxItemContainer *wxChoice_AsItemContainer(wxChoice * obj);
    wxItemContainer *wxComboBox_AsItemContainer(wxComboBox * obj);

    wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox * obj);

    wxTextEntry *wxComboBox_AsTextEntry(wxComboBox * obj);
    wxTextEntry *wxTextCtrl_AsTextEntry(wxTextCtrl * obj);
}
