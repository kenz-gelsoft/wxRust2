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
inline wxEvtHandler *NewEvtHandler() {
    return new wxEvtHandler();
}

// CLASS: wxWindow
inline wxSize *wxWindow_ClientToWindowSize(const wxWindow & self, const wxSize & size) {
    return new wxSize(self.ClientToWindowSize(size));
}
inline wxSize *wxWindow_WindowToClientSize(const wxWindow & self, const wxSize & size) {
    return new wxSize(self.WindowToClientSize(size));
}
inline wxPoint *wxWindow_FromDIP(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.FromDIP(pt));
}
inline wxSize *wxWindow_ToDIP(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ToDIP(sz));
}
inline wxPoint *wxWindow_ToDIP(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ToDIP(pt));
}
inline wxSize *wxWindow_GetBestSize(const wxWindow & self) {
    return new wxSize(self.GetBestSize());
}
inline wxSize *wxWindow_GetClientSize(const wxWindow & self) {
    return new wxSize(self.GetClientSize());
}
inline wxSize *wxWindow_GetEffectiveMinSize(const wxWindow & self) {
    return new wxSize(self.GetEffectiveMinSize());
}
inline wxSize *wxWindow_GetMaxClientSize(const wxWindow & self) {
    return new wxSize(self.GetMaxClientSize());
}
inline wxSize *wxWindow_GetMaxSize(const wxWindow & self) {
    return new wxSize(self.GetMaxSize());
}
inline wxSize *wxWindow_GetMinClientSize(const wxWindow & self) {
    return new wxSize(self.GetMinClientSize());
}
inline wxSize *wxWindow_GetMinSize(const wxWindow & self) {
    return new wxSize(self.GetMinSize());
}
inline wxSize *wxWindow_GetSize(const wxWindow & self) {
    return new wxSize(self.GetSize());
}
inline wxSize *wxWindow_GetVirtualSize(const wxWindow & self) {
    return new wxSize(self.GetVirtualSize());
}
inline wxSize *wxWindow_GetBestVirtualSize(const wxWindow & self) {
    return new wxSize(self.GetBestVirtualSize());
}
inline wxSize *wxWindow_GetWindowBorderSize(const wxWindow & self) {
    return new wxSize(self.GetWindowBorderSize());
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
inline wxSize *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ConvertDialogToPixels(sz));
}
inline wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ConvertPixelsToDialog(pt));
}
inline wxSize *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxSize & sz) {
    return new wxSize(self.ConvertPixelsToDialog(sz));
}
inline wxPoint *wxWindow_ScreenToClient(const wxWindow & self, const wxPoint & pt) {
    return new wxPoint(self.ScreenToClient(pt));
}
inline wxSize *wxWindow_GetDPI(const wxWindow & self) {
    return new wxSize(self.GetDPI());
}
inline wxSize *wxWindow_GetTextExtent(const wxWindow & self, const wxString & string) {
    return new wxSize(self.GetTextExtent(string));
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
inline wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, int xlen, int ylen) {
    return new wxSize(self.GetSizeFromTextSize(xlen, ylen));
}
inline wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, const wxSize & tsize) {
    return new wxSize(self.GetSizeFromTextSize(tsize));
}
inline wxSize *wxControl_GetSizeFromText(const wxControl & self, const wxString & text) {
    return new wxSize(self.GetSizeFromText(text));
}

// CLASS: wxAnyButton
inline wxAnyButton *NewAnyButton() {
    return new wxAnyButton();
}
inline wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton & self) {
    return new wxSize(self.GetBitmapMargins());
}

// CLASS: wxButton
inline wxButton *NewButton() {
    return new wxButton();
}
inline wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name) {
    return new wxButton(parent, id, label, pos, size, style, validator, name);
}

// CLASS: wxNonOwnedWindow

// CLASS: wxTopLevelWindow
inline wxTopLevelWindow *NewTopLevelWindow() {
    return new wxTopLevelWindow();
}
inline wxTopLevelWindow *NewTopLevelWindow(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name) {
    return new wxTopLevelWindow(parent, id, title, pos, size, style, name);
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
inline wxSize *NewSize() {
    return new wxSize();
}
inline wxSize *NewSize(int width, int height) {
    return new wxSize(width, height);
}

// CLASS: wxValidator
inline wxValidator *NewValidator() {
    return new wxValidator();
}

} // namespace wxrust

