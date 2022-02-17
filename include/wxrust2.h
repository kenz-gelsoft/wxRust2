#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxObject
wxObject *NewObject() {
    return new wxObject();
}
wxObject *NewObject(const wxObject & other) {
    return new wxObject(other);
}

// CLASS: wxEvtHandler
wxEvtHandler *NewEvtHandler() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
wxSize *wxWindow_ClientToWindowSize(const wxWindow & self, const wxSize & size) {
    return new wxSize(self.ClientToWindowSize(size));
}
wxSize *wxWindow_WindowToClientSize(const wxWindow & self, const wxSize & size) {
    return new wxSize(self.WindowToClientSize(size));
}
wxPoint *wxWindow_FromDIP(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.FromDIP(pt));
}
wxSize *wxWindow_ToDIP(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ToDIP(sz));
}
wxPoint *wxWindow_ToDIP(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ToDIP(pt));
}
wxSize *wxWindow_GetBestSize(const wxWindow & self) {
    return new wxSize(self.GetBestSize());
}
wxSize *wxWindow_GetClientSize(const wxWindow & self) {
    return new wxSize(self.GetClientSize());
}
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow & self) {
    return new wxSize(self.GetEffectiveMinSize());
}
wxSize *wxWindow_GetMaxClientSize(const wxWindow & self) {
    return new wxSize(self.GetMaxClientSize());
}
wxSize *wxWindow_GetMaxSize(const wxWindow & self) {
    return new wxSize(self.GetMaxSize());
}
wxSize *wxWindow_GetMinClientSize(const wxWindow & self) {
    return new wxSize(self.GetMinClientSize());
}
wxSize *wxWindow_GetMinSize(const wxWindow & self) {
    return new wxSize(self.GetMinSize());
}
wxSize *wxWindow_GetSize(const wxWindow & self) {
    return new wxSize(self.GetSize());
}
wxSize *wxWindow_GetVirtualSize(const wxWindow & self) {
    return new wxSize(self.GetVirtualSize());
}
wxSize *wxWindow_GetBestVirtualSize(const wxWindow & self) {
    return new wxSize(self.GetBestVirtualSize());
}
wxSize *wxWindow_GetWindowBorderSize(const wxWindow & self) {
    return new wxSize(self.GetWindowBorderSize());
}
wxPoint *wxWindow_GetPosition(const wxWindow & self) {
    return new wxPoint(self.GetPosition());
}
wxPoint *wxWindow_GetScreenPosition(const wxWindow & self) {
    return new wxPoint(self.GetScreenPosition());
}
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow & self) {
    return new wxPoint(self.GetClientAreaOrigin());
}
wxPoint *wxWindow_ClientToScreen(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ClientToScreen(pt));
}
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertDialogToPixels(pt));
}
wxSize *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ConvertDialogToPixels(sz));
}
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertPixelsToDialog(pt));
}
wxSize *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ConvertPixelsToDialog(sz));
}
wxPoint *wxWindow_ScreenToClient(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ScreenToClient(pt));
}
wxSize *wxWindow_GetDPI(const wxWindow & self) {
    return new wxSize(self.GetDPI());
}
wxSize *wxWindow_GetTextExtent(const wxWindow & self, const wxString & string) {
    return new wxSize(self.GetTextExtent(string));
}
wxWindow *NewWindow() {
    return new wxWindow();
}
wxWindow *NewWindow(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxWindow(parent, id, pos, size, style, name);
}

// CLASS: wxControl
wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxControl(parent, id, pos, size, style, validator, name);
}
wxControl *NewControl() {
    return new wxControl();
}
wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, int xlen, int ylen) {
    return new wxSize(self.GetSizeFromTextSize(xlen, ylen));
}
wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, const wxSize & tsize) {
    return new wxSize(self.GetSizeFromTextSize(tsize));
}
wxSize *wxControl_GetSizeFromText(const wxControl & self, const wxString & text) {
    return new wxSize(self.GetSizeFromText(text));
}

// CLASS: wxAnyButton
wxAnyButton *NewAnyButton() {
    return new wxAnyButton();
}
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton & self) {
    return new wxSize(self.GetBitmapMargins());
}

// CLASS: wxButton
wxButton *NewButton() {
    return new wxButton();
}
wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}

// CLASS: wxNonOwnedWindow

// CLASS: wxTopLevelWindow
wxTopLevelWindow *NewTopLevelWindow() {
    return new wxTopLevelWindow();
}
wxTopLevelWindow *NewTopLevelWindow(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxTopLevelWindow(parent, id, title, pos, size, style, name);
}

// CLASS: wxFrame
wxFrame *NewFrame() {
    return new wxFrame();
}
wxFrame *NewFrame(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxFrame(parent, id, title, pos, size, style, name);
}
wxPoint *wxFrame_GetClientAreaOrigin(const wxFrame & self) {
    return new wxPoint(self.GetClientAreaOrigin());
}

// CLASS: wxPoint
wxPoint *NewPoint() {
    return new wxPoint();
}
wxPoint *NewPoint(int x, int y) {
    return new wxPoint(x, y);
}
wxPoint *NewPoint(const wxRealPoint & pt) {
    return new wxPoint(pt);
}

// CLASS: wxSize
wxSize *NewSize() {
    return new wxSize();
}
wxSize *NewSize(int width, int height) {
    return new wxSize(width, height);
}

// CLASS: wxValidator
wxValidator *NewValidator() {
    return new wxValidator();
}

} // namespace wxrust

