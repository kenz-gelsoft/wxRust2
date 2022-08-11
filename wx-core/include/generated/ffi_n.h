#pragma once
#include <wx/wx.h>
#include <wx/event.h>
#include <wx/nonownedwnd.h>
#include <wx/notebook.h>

// wxBitmapBundle compatibility hack(for a while)
#if !wxCHECK_VERSION(3, 1, 6)
typedef wxBitmap wxBitmapBundle;
#endif

extern "C" {

// CLASS: wxNonOwnedWindow
wxClassInfo *wxNonOwnedWindow_CLASSINFO();
bool wxNonOwnedWindow_SetShape(wxNonOwnedWindow * self, const wxRegion * region);
bool wxNonOwnedWindow_SetShape1(wxNonOwnedWindow * self, const wxGraphicsPath * path);

// CLASS: wxNotebook
wxClassInfo *wxNotebook_CLASSINFO();
wxNotebook *wxNotebook_new();
wxNotebook *wxNotebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
int wxNotebook_GetRowCount(const wxNotebook * self);
wxColour *wxNotebook_GetThemeBackgroundColour(const wxNotebook * self);
void wxNotebook_SetPadding(wxNotebook * self, const wxSize * padding);

// CLASS: wxNotifyEvent
wxClassInfo *wxNotifyEvent_CLASSINFO();
void wxNotifyEvent_Allow(wxNotifyEvent * self);
bool wxNotifyEvent_IsAllowed(const wxNotifyEvent * self);
void wxNotifyEvent_Veto(wxNotifyEvent * self);

} // extern "C"

