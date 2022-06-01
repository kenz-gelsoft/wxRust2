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

    // CLASS: wxItemContainer
    void wxItemContainer_delete(wxItemContainer *self);
    int wxItemContainer_Append(wxItemContainer * self, const wxString * item);
    int wxItemContainer_Append1(wxItemContainer * self, const wxString * item, void * client_data);
    int wxItemContainer_Append2(wxItemContainer * self, const wxString * item, wxClientData * client_data);
    int wxItemContainer_Append3(wxItemContainer * self, const wxArrayString * items);
    int wxItemContainer_Append5(wxItemContainer * self, const wxArrayString * items, void ** client_data);
    int wxItemContainer_Append6(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data);
    void wxItemContainer_Clear(wxItemContainer * self);
    bool wxItemContainer_HasClientData(const wxItemContainer * self);
    bool wxItemContainer_HasClientObjectData(const wxItemContainer * self);
    bool wxItemContainer_HasClientUntypedData(const wxItemContainer * self);
    void wxItemContainer_Set(wxItemContainer * self, const wxArrayString * items);
    void wxItemContainer_Set2(wxItemContainer * self, const wxArrayString * items, void ** client_data);
    void wxItemContainer_Set3(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data);

    wxItemContainer *wxChoice_AsItemContainer(wxChoice * obj);

    // CLASS: wxItemContainerImmutable
    void wxItemContainerImmutable_delete(wxItemContainerImmutable *self);
    void wxItemContainerImmutable_SetSelection(wxItemContainerImmutable * self, int n);
    int wxItemContainerImmutable_GetSelection(const wxItemContainerImmutable * self);
    bool wxItemContainerImmutable_SetStringSelection(wxItemContainerImmutable * self, const wxString * string);
    wxString *wxItemContainerImmutable_GetStringSelection(const wxItemContainerImmutable * self);
    void wxItemContainerImmutable_Select(wxItemContainerImmutable * self, int n);
    wxItemContainerImmutable *wxItemContainerImmutable_new();
    bool wxItemContainerImmutable_IsEmpty(const wxItemContainerImmutable * self);
    wxArrayString *wxItemContainerImmutable_GetStrings(const wxItemContainerImmutable * self);
    int wxItemContainerImmutable_FindString(const wxItemContainerImmutable * self, const wxString * string, bool case_sensitive);

    wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox * obj);

    // CLASS: wxTextEntry
    void wxTextEntry_delete(wxTextEntry *self);
    void wxTextEntry_AppendText(wxTextEntry * self, const wxString * text);
    bool wxTextEntry_AutoComplete(wxTextEntry * self, const wxArrayString * choices);
    bool wxTextEntry_AutoComplete1(wxTextEntry * self, wxTextCompleter * completer);
    bool wxTextEntry_AutoCompleteFileNames(wxTextEntry * self);
    bool wxTextEntry_AutoCompleteDirectories(wxTextEntry * self);
    bool wxTextEntry_CanCopy(const wxTextEntry * self);
    bool wxTextEntry_CanCut(const wxTextEntry * self);
    bool wxTextEntry_CanPaste(const wxTextEntry * self);
    bool wxTextEntry_CanRedo(const wxTextEntry * self);
    bool wxTextEntry_CanUndo(const wxTextEntry * self);
    void wxTextEntry_ChangeValue(wxTextEntry * self, const wxString * value);
    void wxTextEntry_Clear(wxTextEntry * self);
    void wxTextEntry_Copy(wxTextEntry * self);
    void wxTextEntry_Cut(wxTextEntry * self);
    #if wxCHECK_VERSION(3, 1, 0)
    void wxTextEntry_ForceUpper(wxTextEntry * self);
    #endif
    long wxTextEntry_GetInsertionPoint(const wxTextEntry * self);
    wxString *wxTextEntry_GetRange(const wxTextEntry * self, long from, long to);
    void wxTextEntry_GetSelection(const wxTextEntry * self, long * from, long * to);
    wxString *wxTextEntry_GetStringSelection(const wxTextEntry * self);
    wxString *wxTextEntry_GetValue(const wxTextEntry * self);
    bool wxTextEntry_IsEditable(const wxTextEntry * self);
    bool wxTextEntry_IsEmpty(const wxTextEntry * self);
    void wxTextEntry_Paste(wxTextEntry * self);
    void wxTextEntry_Redo(wxTextEntry * self);
    void wxTextEntry_Remove(wxTextEntry * self, long from, long to);
    void wxTextEntry_Replace(wxTextEntry * self, long from, long to, const wxString * value);
    void wxTextEntry_SetEditable(wxTextEntry * self, bool editable);
    void wxTextEntry_SetInsertionPoint(wxTextEntry * self, long pos);
    void wxTextEntry_SetInsertionPointEnd(wxTextEntry * self);
    void wxTextEntry_SetSelection(wxTextEntry * self, long from, long to);
    void wxTextEntry_SelectAll(wxTextEntry * self);
    void wxTextEntry_SelectNone(wxTextEntry * self);
    bool wxTextEntry_SetHint(wxTextEntry * self, const wxString * hint);
    wxString *wxTextEntry_GetHint(const wxTextEntry * self);
    bool wxTextEntry_SetMargins(wxTextEntry * self, const wxPoint * pt);
    bool wxTextEntry_SetMargins1(wxTextEntry * self, wxCoord left, wxCoord top);
    wxPoint *wxTextEntry_GetMargins(const wxTextEntry * self);
    void wxTextEntry_SetValue(wxTextEntry * self, const wxString * value);
    void wxTextEntry_Undo(wxTextEntry * self);
    void wxTextEntry_WriteText(wxTextEntry * self, const wxString * text);

    wxTextEntry *wxTextCtrl_AsTextEntry(wxTextCtrl * obj);
}
