#pragma once
#include <wx/wx.h>

#include "rust/cxx.h"
#include "wx/src/generated.rs.h"


namespace wxrust {

// CLASS: wxObject
wxObject *NewObject();
wxObject *NewObject1(const wxObject & other);

// CLASS: wxEvtHandler
wxEvtHandler *NewEvtHandler();

// CLASS: wxWindow
wxWindow *NewWindow();
wxWindow *NewWindow1(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name);

// CLASS: wxControl
wxControl *NewControl(wxWindow * parent, wxWindowID id, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name);
wxControl *NewControl1();

// CLASS: wxAnyButton
wxAnyButton *NewAnyButton();

// CLASS: wxButton
wxButton *NewButton();
wxButton *NewButton1(wxWindow * parent, wxWindowID id, const wxString & label, const wxPoint & pos, const wxSize & size, int32_t style, const wxValidator & validator, const wxString & name);

// CLASS: wxNonOwnedWindow

// CLASS: wxTopLevelWindow
wxTopLevelWindow *NewTopLevelWindow();
wxTopLevelWindow *NewTopLevelWindow1(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name);

// CLASS: wxFrame
wxFrame *NewFrame();
wxFrame *NewFrame1(wxWindow * parent, wxWindowID id, const wxString & title, const wxPoint & pos, const wxSize & size, int32_t style, const wxString & name);

// CLASS: wxPoint
wxPoint *NewPoint();
wxPoint *NewPoint1(int x, int y);
wxPoint *NewPoint2(const wxRealPoint & pt);

// CLASS: wxSize
wxSize *NewSize();
wxSize *NewSize1(int width, int height);

} // namespace wxrust

