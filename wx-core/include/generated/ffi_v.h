#pragma once

#include <wx/docview.h>
#include <wx/validate.h>
#include <wx/vlbox.h>
#include <wx/vscroll.h>

extern "C" {

// CLASS: wxVListBox
wxClassInfo *wxVListBox_CLASSINFO();
void wxVListBox_Clear(wxVListBox * self);
bool wxVListBox_Create(wxVListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxVListBox_DeselectAll(wxVListBox * self);
int wxVListBox_GetFirstSelected(const wxVListBox * self, unsigned long * cookie);
size_t wxVListBox_GetItemCount(const wxVListBox * self);
wxPoint *wxVListBox_GetMargins(const wxVListBox * self);
wxRect *wxVListBox_GetItemRect(const wxVListBox * self, size_t item);
int wxVListBox_GetNextSelected(const wxVListBox * self, unsigned long * cookie);
size_t wxVListBox_GetSelectedCount(const wxVListBox * self);
int wxVListBox_GetSelection(const wxVListBox * self);
wxColour *wxVListBox_GetSelectionBackground(const wxVListBox * self);
bool wxVListBox_HasMultipleSelection(const wxVListBox * self);
bool wxVListBox_IsCurrent(const wxVListBox * self, size_t item);
bool wxVListBox_IsSelected(const wxVListBox * self, size_t item);
bool wxVListBox_Select(wxVListBox * self, size_t item, bool select);
bool wxVListBox_SelectAll(wxVListBox * self);
bool wxVListBox_SelectRange(wxVListBox * self, size_t from, size_t to);
void wxVListBox_SetItemCount(wxVListBox * self, size_t count);
void wxVListBox_SetMargins(wxVListBox * self, const wxPoint * pt);
void wxVListBox_SetMargins1(wxVListBox * self, wxCoord x, wxCoord y);
void wxVListBox_SetSelection(wxVListBox * self, int selection);
void wxVListBox_SetSelectionBackground(wxVListBox * self, const wxColour * col);
void wxVListBox_Toggle(wxVListBox * self, size_t item);

// CLASS: wxVScrolledWindow
wxClassInfo *wxVScrolledWindow_CLASSINFO();
bool wxVScrolledWindow_Create(wxVScrolledWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxValidator
wxClassInfo *wxValidator_CLASSINFO();
wxValidator *wxValidator_new();
wxObject * wxValidator_Clone(const wxValidator * self);
wxWindow * wxValidator_GetWindow(const wxValidator * self);
void wxValidator_SetWindow(wxValidator * self, wxWindow * window);
bool wxValidator_TransferFromWindow(wxValidator * self);
bool wxValidator_TransferToWindow(wxValidator * self);
bool wxValidator_Validate(wxValidator * self, wxWindow * parent);
void wxValidator_SuppressBellOnError(bool suppress);
bool wxValidator_IsSilent();

// CLASS: wxView
wxClassInfo *wxView_CLASSINFO();
void wxView_Activate(wxView * self, bool activate);
bool wxView_Close(wxView * self, bool delete_window);
wxDocument * wxView_GetDocument(const wxView * self);
wxDocManager * wxView_GetDocumentManager(const wxView * self);
wxWindow * wxView_GetFrame(const wxView * self);
wxString *wxView_GetViewName(const wxView * self);
void wxView_OnActivateView(wxView * self, bool activate, wxView * active_view, wxView * deactive_view);
void wxView_OnChangeFilename(wxView * self);
bool wxView_OnClose(wxView * self, bool delete_window);
void wxView_OnClosingDocument(wxView * self);
bool wxView_OnCreate(wxView * self, wxDocument * doc, long flags);
wxPrintout * wxView_OnCreatePrintout(wxView * self);
void wxView_OnDraw(wxView * self, wxDC * dc);
void wxView_OnUpdate(wxView * self, wxView * sender, wxObject * hint);
void wxView_SetDocument(wxView * self, wxDocument * doc);
void wxView_SetFrame(wxView * self, wxWindow * frame);
void wxView_SetViewName(wxView * self, const wxString * name);

} // extern "C"

