#include "manual.h"

// WindowList
wxSizerItemList *wxSizerItemList_new() {
    return new wxSizerItemList();
}
void wxSizerItemList_delete(wxSizerItemList *self) {
    delete self;
}
bool wxSizerItemList_IsEmpty(wxSizerItemList *self) {
    return self->IsEmpty();
}

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

// CLASS: wxTextEntry
void wxTextEntry_delete(wxTextEntry *self) {
    delete self;
}
void wxTextEntry_AppendText(wxTextEntry * self, const wxString * text) {
    return self->AppendText(*text);
}
bool wxTextEntry_AutoComplete(wxTextEntry * self, const wxArrayString * choices) {
    return self->AutoComplete(*choices);
}
bool wxTextEntry_AutoComplete1(wxTextEntry * self, wxTextCompleter * completer) {
    return self->AutoComplete(completer);
}
bool wxTextEntry_AutoCompleteFileNames(wxTextEntry * self) {
    return self->AutoCompleteFileNames();
}
bool wxTextEntry_AutoCompleteDirectories(wxTextEntry * self) {
    return self->AutoCompleteDirectories();
}
bool wxTextEntry_CanCopy(const wxTextEntry * self) {
    return self->CanCopy();
}
bool wxTextEntry_CanCut(const wxTextEntry * self) {
    return self->CanCut();
}
bool wxTextEntry_CanPaste(const wxTextEntry * self) {
    return self->CanPaste();
}
bool wxTextEntry_CanRedo(const wxTextEntry * self) {
    return self->CanRedo();
}
bool wxTextEntry_CanUndo(const wxTextEntry * self) {
    return self->CanUndo();
}
void wxTextEntry_ChangeValue(wxTextEntry * self, const wxString * value) {
    return self->ChangeValue(*value);
}
void wxTextEntry_Clear(wxTextEntry * self) {
    return self->Clear();
}
void wxTextEntry_Copy(wxTextEntry * self) {
    return self->Copy();
}
void wxTextEntry_Cut(wxTextEntry * self) {
    return self->Cut();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxTextEntry_ForceUpper(wxTextEntry * self) {
    return self->ForceUpper();
}
#endif
long wxTextEntry_GetInsertionPoint(const wxTextEntry * self) {
    return self->GetInsertionPoint();
}
wxString *wxTextEntry_GetRange(const wxTextEntry * self, long from, long to) {
    return new wxString(self->GetRange(from, to));
}
void wxTextEntry_GetSelection(const wxTextEntry * self, long * from, long * to) {
    return self->GetSelection(from, to);
}
wxString *wxTextEntry_GetStringSelection(const wxTextEntry * self) {
    return new wxString(self->GetStringSelection());
}
wxString *wxTextEntry_GetValue(const wxTextEntry * self) {
    return new wxString(self->GetValue());
}
bool wxTextEntry_IsEditable(const wxTextEntry * self) {
    return self->IsEditable();
}
bool wxTextEntry_IsEmpty(const wxTextEntry * self) {
    return self->IsEmpty();
}
void wxTextEntry_Paste(wxTextEntry * self) {
    return self->Paste();
}
void wxTextEntry_Redo(wxTextEntry * self) {
    return self->Redo();
}
void wxTextEntry_Remove(wxTextEntry * self, long from, long to) {
    return self->Remove(from, to);
}
void wxTextEntry_Replace(wxTextEntry * self, long from, long to, const wxString * value) {
    return self->Replace(from, to, *value);
}
void wxTextEntry_SetEditable(wxTextEntry * self, bool editable) {
    return self->SetEditable(editable);
}
void wxTextEntry_SetInsertionPoint(wxTextEntry * self, long pos) {
    return self->SetInsertionPoint(pos);
}
void wxTextEntry_SetInsertionPointEnd(wxTextEntry * self) {
    return self->SetInsertionPointEnd();
}
void wxTextEntry_SetSelection(wxTextEntry * self, long from, long to) {
    return self->SetSelection(from, to);
}
void wxTextEntry_SelectAll(wxTextEntry * self) {
    return self->SelectAll();
}
void wxTextEntry_SelectNone(wxTextEntry * self) {
    return self->SelectNone();
}
bool wxTextEntry_SetHint(wxTextEntry * self, const wxString * hint) {
    return self->SetHint(*hint);
}
wxString *wxTextEntry_GetHint(const wxTextEntry * self) {
    return new wxString(self->GetHint());
}
bool wxTextEntry_SetMargins(wxTextEntry * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
bool wxTextEntry_SetMargins1(wxTextEntry * self, wxCoord left, wxCoord top) {
    return self->SetMargins(left, top);
}
wxPoint *wxTextEntry_GetMargins(const wxTextEntry * self) {
    return new wxPoint(self->GetMargins());
}
void wxTextEntry_SetValue(wxTextEntry * self, const wxString * value) {
    return self->SetValue(*value);
}
void wxTextEntry_Undo(wxTextEntry * self) {
    return self->Undo();
}
void wxTextEntry_WriteText(wxTextEntry * self, const wxString * text) {
    return self->WriteText(*text);
}

wxTextEntry *wxTextCtrl_AsTextEntry(wxTextCtrl * obj) {
    return static_cast<wxTextEntry *>(obj);
}
