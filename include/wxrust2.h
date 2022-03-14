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
inline wxWindow * wxWindow_FindWindow(const wxWindow & self, int32_t id) {
    return self.FindWindow(id);
}
inline void wxWindow_SetSize(wxWindow & self, const wxRect & rect) {
    return self.SetSize(rect);
}
inline wxSize wxWindow_FromDIP(const wxSize & sz, const wxWindow * w) {
    return wxWindow::FromDIP(sz, w);
}
inline wxPoint wxWindow_FromDIP(const wxPoint & pt, const wxWindow * w) {
    return wxWindow::FromDIP(pt, w);
}
inline int wxWindow_FromDIP(int d, const wxWindow * w) {
    return wxWindow::FromDIP(d, w);
}
inline wxSize wxWindow_ToDIP(const wxSize & sz, const wxWindow * w) {
    return wxWindow::ToDIP(sz, w);
}
inline wxPoint wxWindow_ToDIP(const wxPoint & pt, const wxWindow * w) {
    return wxWindow::ToDIP(pt, w);
}
inline int wxWindow_ToDIP(int d, const wxWindow * w) {
    return wxWindow::ToDIP(d, w);
}
inline int32_t wxWindow_GetExtraStyle(const wxWindow & self) {
    return self.GetExtraStyle();
}
inline int32_t wxWindow_GetWindowStyleFlag(const wxWindow & self) {
    return self.GetWindowStyleFlag();
}
inline int32_t wxWindow_GetWindowStyle(const wxWindow & self) {
    return self.GetWindowStyle();
}
inline void wxWindow_SetExtraStyle(wxWindow & self, int32_t ex_style) {
    return self.SetExtraStyle(ex_style);
}
inline void wxWindow_SetWindowStyleFlag(wxWindow & self, int32_t style) {
    return self.SetWindowStyleFlag(style);
}
inline void wxWindow_SetWindowStyle(wxWindow & self, int32_t style) {
    return self.SetWindowStyle(style);
}
inline bool wxWindow_IsExposed(const wxWindow & self, wxPoint & pt) {
    return self.IsExposed(pt);
}
inline bool wxWindow_IsExposed(const wxWindow & self, wxRect & rect) {
    return self.IsExposed(rect);
}
inline rust::String wxWindow_GetHelpText(const wxWindow & self) {
    return rust::String(self.GetHelpText().utf8_str());
}
inline rust::String wxWindow_GetToolTipText(const wxWindow & self) {
    return rust::String(self.GetToolTipText().utf8_str());
}
inline rust::String wxWindow_GetLabel(const wxWindow & self) {
    return rust::String(self.GetLabel().utf8_str());
}
inline rust::String wxWindow_GetName(const wxWindow & self) {
    return rust::String(self.GetName().utf8_str());
}
inline void wxWindow_UpdateWindowUI(wxWindow & self, int32_t flags) {
    return self.UpdateWindowUI(flags);
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
inline bool wxWindow_Create(wxWindow & self, wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return self.Create(parent, id, pos, size, style, name);
}

// CLASS: wxControl
inline wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxControl(parent, id, pos, size, style, validator, name);
}
inline wxControl *NewControl() {
    return new wxControl();
}
inline bool wxControl_Create(wxControl & self, wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return self.Create(parent, id, pos, size, style, validator, name);
}
inline rust::String wxControl_GetLabel(const wxControl & self) {
    return rust::String(self.GetLabel().utf8_str());
}
inline rust::String wxControl_GetLabelText(const wxControl & self) {
    return rust::String(self.GetLabelText().utf8_str());
}
inline rust::String wxControl_GetLabelText(const wxString & label) {
    return rust::String(wxControl::GetLabelText(label).utf8_str());
}
inline rust::String wxControl_RemoveMnemonics(const wxString & str) {
    return rust::String(wxControl::RemoveMnemonics(str).utf8_str());
}
inline rust::String wxControl_EscapeMnemonics(const wxString & text) {
    return rust::String(wxControl::EscapeMnemonics(text).utf8_str());
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
inline bool wxButton_Create(wxButton & self, wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return self.Create(parent, id, label, pos, size, style, validator, name);
}
inline rust::String wxButton_GetLabel(const wxButton & self) {
    return rust::String(self.GetLabel().utf8_str());
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
inline bool wxTopLevelWindow_Create(wxTopLevelWindow & self, wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return self.Create(parent, id, title, pos, size, style, name);
}
inline rust::String wxTopLevelWindow_GetTitle(const wxTopLevelWindow & self) {
    return rust::String(self.GetTitle().utf8_str());
}
inline bool wxTopLevelWindow_ShowFullScreen(wxTopLevelWindow & self, bool show, int32_t style) {
    return self.ShowFullScreen(show, style);
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
inline bool wxFrame_Create(wxFrame & self, wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return self.Create(parent, id, title, pos, size, style, name);
}
inline wxStatusBar * wxFrame_CreateStatusBar(wxFrame & self, int number, int32_t style, wxWindowID id, const wxString & name) {
    return self.CreateStatusBar(number, style, id, name);
}
inline wxToolBar * wxFrame_CreateToolBar(wxFrame & self, int32_t style, wxWindowID id, const wxString & name) {
    return self.CreateToolBar(style, id, name);
}
inline wxStatusBar * wxFrame_OnCreateStatusBar(wxFrame & self, int number, int32_t style, wxWindowID id, const wxString & name) {
    return self.OnCreateStatusBar(number, style, id, name);
}
inline wxToolBar * wxFrame_OnCreateToolBar(wxFrame & self, int32_t style, wxWindowID id, const wxString & name) {
    return self.OnCreateToolBar(style, id, name);
}

// CLASS: wxPoint
inline wxPoint NewPoint() {
    return wxPoint();
}
inline wxPoint NewPoint(int x, int y) {
    return wxPoint(x, y);
}
inline wxPoint NewPoint(const wxRealPoint & pt) {
    return wxPoint(pt);
}

// CLASS: wxRect
inline wxRect NewRect() {
    return wxRect();
}
inline wxRect NewRect(int x, int y, int width, int height) {
    return wxRect(x, y, width, height);
}
inline wxRect NewRect(const wxPoint & top_left, const wxPoint & bottom_right) {
    return wxRect(top_left, bottom_right);
}
inline wxRect NewRect(const wxPoint & pos, const wxSize & size) {
    return wxRect(pos, size);
}
inline wxRect NewRect(const wxSize & size) {
    return wxRect(size);
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

