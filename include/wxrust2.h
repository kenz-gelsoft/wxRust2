#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxObject
inline wxObject *NewObject() {
    return new wxObject();
}
inline wxObject *NewObject(const wxObject & other) {
    return new wxObject(other);
}

// CLASS: wxEvtHandler
inline void wxEvtHandler_AddFilter(wxEventFilter * filter) {
    return wxEvtHandler::AddFilter(filter);
}
inline void wxEvtHandler_RemoveFilter(wxEventFilter * filter) {
    return wxEvtHandler::RemoveFilter(filter);
}
inline wxEvtHandler *NewEvtHandler() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
inline wxPoint *wxWindow_FromDIP(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.FromDIP(pt));
}
inline wxPoint *wxWindow_ToDIP(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ToDIP(pt));
}
inline wxSize wxWindow_FromDIP(const wxSize & sz, const wxWindow * w) {
    return wxWindow::FromDIP(sz, w);
}
inline wxPoint *wxWindow_FromDIP(const wxPoint & pt, const wxWindow * w) {
    return new wxPoint(wxWindow::FromDIP(pt, w));
}
inline int wxWindow_FromDIP(int d, const wxWindow * w) {
    return wxWindow::FromDIP(d, w);
}
inline wxSize wxWindow_ToDIP(const wxSize & sz, const wxWindow * w) {
    return wxWindow::ToDIP(sz, w);
}
inline wxPoint *wxWindow_ToDIP(const wxPoint & pt, const wxWindow * w) {
    return new wxPoint(wxWindow::ToDIP(pt, w));
}
inline int wxWindow_ToDIP(int d, const wxWindow * w) {
    return wxWindow::ToDIP(d, w);
}
inline wxPoint *wxWindow_GetPosition(const wxWindow & self) {
    return new wxPoint(self.GetPosition());
}
inline wxPoint *wxWindow_GetScreenPosition(const wxWindow & self) {
    return new wxPoint(self.GetScreenPosition());
}
inline wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow & self) {
    return new wxPoint(self.GetClientAreaOrigin());
}
inline wxPoint *wxWindow_ClientToScreen(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ClientToScreen(pt));
}
inline wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertDialogToPixels(pt));
}
inline wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertPixelsToDialog(pt));
}
inline wxPoint *wxWindow_ScreenToClient(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ScreenToClient(pt));
}
inline wxWindow * wxWindow_FindFocus() {
    return wxWindow::FindFocus();
}
inline wxWindow * wxWindow_FindWindowById(int32_t id, const wxWindow * parent) {
    return wxWindow::FindWindowById(id, parent);
}
inline wxWindow * wxWindow_FindWindowByLabel(const wxString & label, const wxWindow * parent) {
    return wxWindow::FindWindowByLabel(label, parent);
}
inline wxWindow * wxWindow_FindWindowByName(const wxString & name, const wxWindow * parent) {
    return wxWindow::FindWindowByName(name, parent);
}
inline wxWindow * wxWindow_GetCapture() {
    return wxWindow::GetCapture();
}
inline wxWindowID wxWindow_NewControlId(int count) {
    return wxWindow::NewControlId(count);
}
inline void wxWindow_UnreserveControlId(wxWindowID id, int count) {
    return wxWindow::UnreserveControlId(id, count);
}
inline wxWindow *NewWindow() {
    return new wxWindow();
}
inline wxWindow *NewWindow(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxWindow(parent, id, pos, size, style, name);
}

// CLASS: wxControl
inline wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxControl(parent, id, pos, size, style, validator, name);
}
inline wxControl *NewControl() {
    return new wxControl();
}

// CLASS: wxAnyButton
inline wxAnyButton *NewAnyButton() {
    return new wxAnyButton();
}

// CLASS: wxButton
inline wxButton *NewButton() {
    return new wxButton();
}
inline wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}
inline wxSize wxButton_GetDefaultSize(wxWindow * win) {
    return wxButton::GetDefaultSize(win);
}

// CLASS: wxNonOwnedWindow

// CLASS: wxTopLevelWindow
inline wxTopLevelWindow *NewTopLevelWindow() {
    return new wxTopLevelWindow();
}
inline wxTopLevelWindow *NewTopLevelWindow(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxTopLevelWindow(parent, id, title, pos, size, style, name);
}
inline wxSize wxTopLevelWindow_GetDefaultSize() {
    return wxTopLevelWindow::GetDefaultSize();
}

// CLASS: wxFrame
inline wxFrame *NewFrame() {
    return new wxFrame();
}
inline wxFrame *NewFrame(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxFrame(parent, id, title, pos, size, style, name);
}
inline wxPoint *wxFrame_GetClientAreaOrigin(const wxFrame & self) {
    return new wxPoint(self.GetClientAreaOrigin());
}

// CLASS: wxPoint
inline wxPoint *NewPoint() {
    return new wxPoint();
}
inline wxPoint *NewPoint(int x, int y) {
    return new wxPoint(x, y);
}
inline wxPoint *NewPoint(const wxRealPoint & pt) {
    return new wxPoint(pt);
}

// CLASS: wxSize
inline wxSize NewSize() {
    return wxSize();
}
inline wxSize NewSize(int width, int height) {
    return wxSize(width, height);
}

// CLASS: wxValidator
inline wxValidator *NewValidator() {
    return new wxValidator();
}
inline void wxValidator_SuppressBellOnError(bool suppress) {
    return wxValidator::SuppressBellOnError(suppress);
}
inline bool wxValidator_IsSilent() {
    return wxValidator::IsSilent();
}

} // namespace wxrust

