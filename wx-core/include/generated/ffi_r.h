#pragma once

#include <wx/gdicmn.h>
#include <wx/radiobox.h>
#include <wx/renderer.h>

extern "C" {

// CLASS: wxRadioBox
wxClassInfo *wxRadioBox_CLASSINFO();
wxRadioBox *wxRadioBox_new();
wxRadioBox *wxRadioBox_new2(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name);
bool wxRadioBox_Create1(wxRadioBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name);
bool wxRadioBox_Enable(wxRadioBox * self, unsigned int n, bool enable);
unsigned int wxRadioBox_GetColumnCount(const wxRadioBox * self);
int wxRadioBox_GetItemFromPoint(const wxRadioBox * self, const wxPoint * pt);
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxRadioBox_GetItemHelpText(const wxRadioBox * self, unsigned int item);
#endif
wxToolTip * wxRadioBox_GetItemToolTip(const wxRadioBox * self, unsigned int item);
unsigned int wxRadioBox_GetRowCount(const wxRadioBox * self);
bool wxRadioBox_IsItemEnabled(const wxRadioBox * self, unsigned int n);
bool wxRadioBox_IsItemShown(const wxRadioBox * self, unsigned int n);
void wxRadioBox_SetItemHelpText(wxRadioBox * self, unsigned int item, const wxString * helptext);
void wxRadioBox_SetItemToolTip(wxRadioBox * self, unsigned int item, const wxString * text);
bool wxRadioBox_Show(wxRadioBox * self, unsigned int item, bool show);
// Mix-in(s) to wxRadioBox
wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox* obj);

// CLASS: wxRect
void wxRect_delete(wxRect *self);
wxRect *wxRect_new();
wxRect *wxRect_new1(int x, int y, int width, int height);
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right);
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size);
wxRect *wxRect_new4(const wxSize * size);
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir);
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir);
bool wxRect_Contains(const wxRect * self, int x, int y);
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt);
bool wxRect_Contains2(const wxRect * self, const wxRect * rect);
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy);
int wxRect_GetBottom(const wxRect * self);
wxPoint *wxRect_GetBottomLeft(const wxRect * self);
wxPoint *wxRect_GetBottomRight(const wxRect * self);
int wxRect_GetHeight(const wxRect * self);
int wxRect_GetLeft(const wxRect * self);
wxPoint *wxRect_GetPosition(const wxRect * self);
int wxRect_GetRight(const wxRect * self);
wxSize *wxRect_GetSize(const wxRect * self);
int wxRect_GetTop(const wxRect * self);
wxPoint *wxRect_GetTopLeft(const wxRect * self);
wxPoint *wxRect_GetTopRight(const wxRect * self);
int wxRect_GetWidth(const wxRect * self);
int wxRect_GetX(const wxRect * self);
int wxRect_GetY(const wxRect * self);
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy);
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect);
bool wxRect_Intersects(const wxRect * self, const wxRect * rect);
bool wxRect_IsEmpty(const wxRect * self);
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy);
void wxRect_Offset1(wxRect * self, const wxPoint * pt);
void wxRect_SetHeight(wxRect * self, int height);
void wxRect_SetPosition(wxRect * self, const wxPoint * pos);
void wxRect_SetSize(wxRect * self, const wxSize * s);
void wxRect_SetWidth(wxRect * self, int width);
void wxRect_SetX(wxRect * self, int x);
void wxRect_SetY(wxRect * self, int y);
void wxRect_SetLeft(wxRect * self, int left);
void wxRect_SetRight(wxRect * self, int right);
void wxRect_SetTop(wxRect * self, int top);
void wxRect_SetBottom(wxRect * self, int bottom);
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p);
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p);
void wxRect_SetTopRight(wxRect * self, const wxPoint * p);
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p);
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect);

// CLASS: wxRendererNative
void wxRendererNative_delete(wxRendererNative *self);
void wxRendererNative_DrawCheckBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawComboBoxDropButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawDropArrow(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawFocusRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawGauge(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int value, int max, int flags);
void wxRendererNative_DrawItemSelectionRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawItemText(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxString * text, const wxRect * rect, int align, int flags, wxEllipsizeMode ellipsize_mode);
void wxRendererNative_DrawPushButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawCollapseButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
wxSize *wxRendererNative_GetCollapseButtonSize(wxRendererNative * self, wxWindow * win, wxDC * dc);
void wxRendererNative_DrawSplitterBorder(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawTreeItemButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawChoice(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawComboBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawTextCtrl(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawRadioBitmap(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawCheckMark(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
wxSize *wxRendererNative_GetCheckBoxSize(wxRendererNative * self, wxWindow * win, int flags);
wxSize *wxRendererNative_GetCheckMarkSize(wxRendererNative * self, wxWindow * win);
wxSize *wxRendererNative_GetExpanderSize(wxRendererNative * self, wxWindow * win);
int wxRendererNative_GetHeaderButtonHeight(wxRendererNative * self, wxWindow * win);
int wxRendererNative_GetHeaderButtonMargin(wxRendererNative * self, wxWindow * win);
wxRendererNative * wxRendererNative_Get();
wxRendererNative * wxRendererNative_GetDefault();
wxRendererNative * wxRendererNative_GetGeneric();
wxRendererNative * wxRendererNative_Load(const wxString * name);
wxRendererNative * wxRendererNative_Set(wxRendererNative * renderer);

} // extern "C"

