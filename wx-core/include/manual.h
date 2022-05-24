#pragma once
#include <wx/wx.h>

extern "C" {
    // SizerItemList
    wxSizerItemList *wxSizerItemList_new();
    void wxSizerItemList_delete(wxSizerItemList *self);
    bool wxSizerItemList_IsEmpty(wxSizerItemList *self);

    // WindowList
    wxWindowList *wxWindowList_new();
    void wxWindowList_delete(wxWindowList *self);
    bool wxWindowList_IsEmpty(wxWindowList *self);

    int wxRustMessageBox(const wxString *message, const wxString *caption, int style, wxWindow *parent, int x, int y);

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
