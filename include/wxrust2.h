#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxObject
wxObject *NewObject();
wxObject *NewObject(const wxObject & other);

// CLASS: wxEvtHandler
wxEvtHandler *NewEvtHandler();

// CLASS: wxWindow
wxSize *wxWindow_ClientToWindowSize(const wxWindow & self, const wxSize & size);
wxSize *wxWindow_WindowToClientSize(const wxWindow & self, const wxSize & size);
wxPoint *wxWindow_FromDIP(const wxWindow & self, const wxPoint & pt);
wxSize *wxWindow_ToDIP(const wxWindow & self, const wxSize & sz);
wxPoint *wxWindow_ToDIP(const wxWindow & self, const wxPoint & pt);
wxSize *wxWindow_GetBestSize(const wxWindow & self);
wxSize *wxWindow_GetClientSize(const wxWindow & self);
wxSize *wxWindow_GetEffectiveMinSize(const wxWindow & self);
wxSize *wxWindow_GetMaxClientSize(const wxWindow & self);
wxSize *wxWindow_GetMaxSize(const wxWindow & self);
wxSize *wxWindow_GetMinClientSize(const wxWindow & self);
wxSize *wxWindow_GetMinSize(const wxWindow & self);
wxSize *wxWindow_GetSize(const wxWindow & self);
wxSize *wxWindow_GetVirtualSize(const wxWindow & self);
wxSize *wxWindow_GetBestVirtualSize(const wxWindow & self);
wxSize *wxWindow_GetWindowBorderSize(const wxWindow & self);
wxPoint *wxWindow_GetPosition(const wxWindow & self);
wxPoint *wxWindow_GetScreenPosition(const wxWindow & self);
wxPoint *wxWindow_GetClientAreaOrigin(const wxWindow & self);
wxPoint *wxWindow_ClientToScreen(const wxWindow & self, const wxPoint & pt);
wxPoint *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxPoint & pt);
wxSize *wxWindow_ConvertDialogToPixels(const wxWindow & self, const wxSize & sz);
wxPoint *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxPoint & pt);
wxSize *wxWindow_ConvertPixelsToDialog(const wxWindow & self, const wxSize & sz);
wxPoint *wxWindow_ScreenToClient(const wxWindow & self, const wxPoint & pt);
wxSize *wxWindow_GetDPI(const wxWindow & self);
wxSize *wxWindow_GetTextExtent(const wxWindow & self, const wxString & string);
wxWindow *NewWindow();
wxWindow *NewWindow(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name);

// CLASS: wxControl
wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name);
wxControl *NewControl();
wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, int xlen, int ylen);
wxSize *wxControl_GetSizeFromTextSize(const wxControl & self, const wxSize & tsize);
wxSize *wxControl_GetSizeFromText(const wxControl & self, const wxString & text);

// CLASS: wxAnyButton
wxAnyButton *NewAnyButton();
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton & self);

// CLASS: wxButton
wxButton *NewButton();
wxButton *NewButton(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name);

// CLASS: wxNonOwnedWindow

// CLASS: wxTopLevelWindow
wxTopLevelWindow *NewTopLevelWindow();
wxTopLevelWindow *NewTopLevelWindow(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name);

// CLASS: wxFrame
wxFrame *NewFrame();
wxFrame *NewFrame(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name);
wxPoint *wxFrame_GetClientAreaOrigin(const wxFrame & self);

// CLASS: wxPoint
wxPoint *NewPoint();
wxPoint *NewPoint(int x, int y);
wxPoint *NewPoint(const wxRealPoint & pt);

// CLASS: wxSize
wxSize *NewSize();
wxSize *NewSize(int width, int height);

// CLASS: wxValidator
wxValidator *NewValidator();

} // namespace wxrust

