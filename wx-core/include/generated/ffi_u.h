#pragma once

#include <wx/dataobj.h>
#include <wx/event.h>
#include <wx/uiaction.h>

extern "C" {

// CLASS: wxUIActionSimulator
void wxUIActionSimulator_delete(wxUIActionSimulator *self);
wxUIActionSimulator *wxUIActionSimulator_new();
bool wxUIActionSimulator_MouseMove(wxUIActionSimulator * self, long x, long y);
bool wxUIActionSimulator_MouseMove1(wxUIActionSimulator * self, const wxPoint * point);
bool wxUIActionSimulator_MouseDown(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseUp(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseClick(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseDblClick(wxUIActionSimulator * self, int button);
bool wxUIActionSimulator_MouseDragDrop(wxUIActionSimulator * self, long x1, long y1, long x2, long y2, int button);
bool wxUIActionSimulator_KeyDown(wxUIActionSimulator * self, int keycode, int modifiers);
bool wxUIActionSimulator_KeyUp(wxUIActionSimulator * self, int keycode, int modifiers);
bool wxUIActionSimulator_Char(wxUIActionSimulator * self, int keycode, int modifiers);
#if wxCHECK_VERSION(3, 1, 0)
bool wxUIActionSimulator_Select(wxUIActionSimulator * self, const wxString * text);
#endif
bool wxUIActionSimulator_Text(wxUIActionSimulator * self, const char * text);

// CLASS: wxURLDataObject
void wxURLDataObject_delete(wxURLDataObject *self);
wxURLDataObject *wxURLDataObject_new(const wxString * url);
wxString *wxURLDataObject_GetURL(const wxURLDataObject * self);
void wxURLDataObject_SetURL(wxURLDataObject * self, const wxString * url);

// CLASS: wxUpdateUIEvent
wxClassInfo *wxUpdateUIEvent_CLASSINFO();
wxUpdateUIEvent *wxUpdateUIEvent_new(wxWindowID command_id);
void wxUpdateUIEvent_Check(wxUpdateUIEvent * self, bool check);
void wxUpdateUIEvent_Enable(wxUpdateUIEvent * self, bool enable);
bool wxUpdateUIEvent_GetChecked(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetEnabled(const wxUpdateUIEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxUpdateUIEvent_IsCheckable(const wxUpdateUIEvent * self);
#endif
bool wxUpdateUIEvent_GetSetChecked(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetEnabled(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetShown(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetSetText(const wxUpdateUIEvent * self);
bool wxUpdateUIEvent_GetShown(const wxUpdateUIEvent * self);
wxString *wxUpdateUIEvent_GetText(const wxUpdateUIEvent * self);
void wxUpdateUIEvent_SetText(wxUpdateUIEvent * self, const wxString * text);
void wxUpdateUIEvent_Show(wxUpdateUIEvent * self, bool show);
bool wxUpdateUIEvent_CanUpdate(wxWindow * window);
long wxUpdateUIEvent_GetUpdateInterval();
void wxUpdateUIEvent_ResetUpdateTime();
void wxUpdateUIEvent_SetUpdateInterval(long update_interval);

} // extern "C"

