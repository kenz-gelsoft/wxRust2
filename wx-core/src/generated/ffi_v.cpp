#include "generated.h"

extern "C" {

// CLASS: wxVListBox
wxClassInfo *wxVListBox_CLASSINFO() {
    return wxCLASSINFO(wxVListBox);
}
void wxVListBox_Clear(wxVListBox * self) {
    return self->Clear();
}
bool wxVListBox_Create(wxVListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
bool wxVListBox_DeselectAll(wxVListBox * self) {
    return self->DeselectAll();
}
int wxVListBox_GetFirstSelected(const wxVListBox * self, unsigned long * cookie) {
    return self->GetFirstSelected(*cookie);
}
size_t wxVListBox_GetItemCount(const wxVListBox * self) {
    return self->GetItemCount();
}
wxPoint *wxVListBox_GetMargins(const wxVListBox * self) {
    return new wxPoint(self->GetMargins());
}
wxRect *wxVListBox_GetItemRect(const wxVListBox * self, size_t item) {
    return new wxRect(self->GetItemRect(item));
}
int wxVListBox_GetNextSelected(const wxVListBox * self, unsigned long * cookie) {
    return self->GetNextSelected(*cookie);
}
size_t wxVListBox_GetSelectedCount(const wxVListBox * self) {
    return self->GetSelectedCount();
}
int wxVListBox_GetSelection(const wxVListBox * self) {
    return self->GetSelection();
}
wxColour *wxVListBox_GetSelectionBackground(const wxVListBox * self) {
    return new wxColour(self->GetSelectionBackground());
}
bool wxVListBox_HasMultipleSelection(const wxVListBox * self) {
    return self->HasMultipleSelection();
}
bool wxVListBox_IsCurrent(const wxVListBox * self, size_t item) {
    return self->IsCurrent(item);
}
bool wxVListBox_IsSelected(const wxVListBox * self, size_t item) {
    return self->IsSelected(item);
}
bool wxVListBox_Select(wxVListBox * self, size_t item, bool select) {
    return self->Select(item, select);
}
bool wxVListBox_SelectAll(wxVListBox * self) {
    return self->SelectAll();
}
bool wxVListBox_SelectRange(wxVListBox * self, size_t from, size_t to) {
    return self->SelectRange(from, to);
}
void wxVListBox_SetItemCount(wxVListBox * self, size_t count) {
    return self->SetItemCount(count);
}
void wxVListBox_SetMargins(wxVListBox * self, const wxPoint * pt) {
    return self->SetMargins(*pt);
}
void wxVListBox_SetMargins1(wxVListBox * self, wxCoord x, wxCoord y) {
    return self->SetMargins(x, y);
}
void wxVListBox_SetSelection(wxVListBox * self, int selection) {
    return self->SetSelection(selection);
}
void wxVListBox_SetSelectionBackground(wxVListBox * self, const wxColour * col) {
    return self->SetSelectionBackground(*col);
}
void wxVListBox_Toggle(wxVListBox * self, size_t item) {
    return self->Toggle(item);
}

// CLASS: wxVScrolledWindow
wxClassInfo *wxVScrolledWindow_CLASSINFO() {
    return wxCLASSINFO(wxVScrolledWindow);
}
bool wxVScrolledWindow_Create(wxVScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}

// CLASS: wxValidator
wxClassInfo *wxValidator_CLASSINFO() {
    return wxCLASSINFO(wxValidator);
}
wxValidator *wxValidator_new() {
    return new wxValidator();
}
wxObject * wxValidator_Clone(const wxValidator * self) {
    return self->Clone();
}
wxWindow * wxValidator_GetWindow(const wxValidator * self) {
    return self->GetWindow();
}
void wxValidator_SetWindow(wxValidator * self, wxWindow * window) {
    return self->SetWindow(window);
}
bool wxValidator_TransferFromWindow(wxValidator * self) {
    return self->TransferFromWindow();
}
bool wxValidator_TransferToWindow(wxValidator * self) {
    return self->TransferToWindow();
}
bool wxValidator_Validate(wxValidator * self, wxWindow * parent) {
    return self->Validate(parent);
}
void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}

// CLASS: wxView
wxClassInfo *wxView_CLASSINFO() {
    return wxCLASSINFO(wxView);
}
void wxView_Activate(wxView * self, bool activate) {
    return self->Activate(activate);
}
bool wxView_Close(wxView * self, bool delete_window) {
    return self->Close(delete_window);
}
wxDocument * wxView_GetDocument(const wxView * self) {
    return self->GetDocument();
}
wxDocManager * wxView_GetDocumentManager(const wxView * self) {
    return self->GetDocumentManager();
}
wxWindow * wxView_GetFrame(const wxView * self) {
    return self->GetFrame();
}
wxString *wxView_GetViewName(const wxView * self) {
    return new wxString(self->GetViewName());
}
void wxView_OnActivateView(wxView * self, bool activate, wxView * active_view, wxView * deactive_view) {
    return self->OnActivateView(activate, active_view, deactive_view);
}
void wxView_OnChangeFilename(wxView * self) {
    return self->OnChangeFilename();
}
bool wxView_OnClose(wxView * self, bool delete_window) {
    return self->OnClose(delete_window);
}
void wxView_OnClosingDocument(wxView * self) {
    return self->OnClosingDocument();
}
bool wxView_OnCreate(wxView * self, wxDocument * doc, long flags) {
    return self->OnCreate(doc, flags);
}
wxPrintout * wxView_OnCreatePrintout(wxView * self) {
    return self->OnCreatePrintout();
}
void wxView_OnDraw(wxView * self, wxDC * dc) {
    return self->OnDraw(dc);
}
void wxView_OnUpdate(wxView * self, wxView * sender, wxObject * hint) {
    return self->OnUpdate(sender, hint);
}
void wxView_SetDocument(wxView * self, wxDocument * doc) {
    return self->SetDocument(doc);
}
void wxView_SetFrame(wxView * self, wxWindow * frame) {
    return self->SetFrame(frame);
}
void wxView_SetViewName(wxView * self, const wxString * name) {
    return self->SetViewName(*name);
}

} // extern "C"

