#pragma once

#include <wx/gdicmn.h>
#include <wx/radiobox.h>
#include <wx/radiobut.h>
#include <wx/rearrangectrl.h>
#include <wx/region.h>
#include <wx/renderer.h>
#include <wx/richtooltip.h>

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

// CLASS: wxRadioButton
wxClassInfo *wxRadioButton_CLASSINFO();
wxRadioButton *wxRadioButton_new();
wxRadioButton *wxRadioButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxRadioButton_Create(wxRadioButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxRadioButton_GetValue(const wxRadioButton * self);
void wxRadioButton_SetValue(wxRadioButton * self, bool value);
wxRadioButton * wxRadioButton_GetFirstInGroup(const wxRadioButton * self);
wxRadioButton * wxRadioButton_GetLastInGroup(const wxRadioButton * self);
wxRadioButton * wxRadioButton_GetPreviousInGroup(const wxRadioButton * self);
wxRadioButton * wxRadioButton_GetNextInGroup(const wxRadioButton * self);

// CLASS: wxRealPoint
void wxRealPoint_delete(wxRealPoint *self);
wxRealPoint *wxRealPoint_new();
wxRealPoint *wxRealPoint_new1(double x, double y);
wxRealPoint *wxRealPoint_new2(const wxPoint * pt);

// CLASS: wxRearrangeCtrl
wxClassInfo *wxRearrangeCtrl_CLASSINFO();
wxRearrangeCtrl *wxRearrangeCtrl_new();
wxRearrangeCtrl *wxRearrangeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
bool wxRearrangeCtrl_Create(wxRearrangeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
wxRearrangeList * wxRearrangeCtrl_GetList(const wxRearrangeCtrl * self);

// CLASS: wxRearrangeDialog
wxClassInfo *wxRearrangeDialog_CLASSINFO();
wxRearrangeDialog *wxRearrangeDialog_new();
wxRearrangeDialog *wxRearrangeDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name);
bool wxRearrangeDialog_Create(wxRearrangeDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name);
void wxRearrangeDialog_AddExtraControls(wxRearrangeDialog * self, wxWindow * win);
wxRearrangeList * wxRearrangeDialog_GetList(const wxRearrangeDialog * self);
wxArrayInt *wxRearrangeDialog_GetOrder(const wxRearrangeDialog * self);

// CLASS: wxRearrangeList
wxClassInfo *wxRearrangeList_CLASSINFO();
wxRearrangeList *wxRearrangeList_new();
wxRearrangeList *wxRearrangeList_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
bool wxRearrangeList_Create(wxRearrangeList * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name);
wxArrayInt *wxRearrangeList_GetCurrentOrder(const wxRearrangeList * self);
bool wxRearrangeList_CanMoveCurrentUp(const wxRearrangeList * self);
bool wxRearrangeList_CanMoveCurrentDown(const wxRearrangeList * self);
bool wxRearrangeList_MoveCurrentUp(wxRearrangeList * self);
bool wxRearrangeList_MoveCurrentDown(wxRearrangeList * self);
// Mix-in(s) to wxRearrangeList
wxItemContainer *wxRearrangeList_AsItemContainer(wxRearrangeList* obj);

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

// CLASS: wxRegion
wxClassInfo *wxRegion_CLASSINFO();
wxRegion *wxRegion_new();
wxRegion *wxRegion_new1(wxCoord x, wxCoord y, wxCoord width, wxCoord height);
wxRegion *wxRegion_new2(const wxPoint * top_left, const wxPoint * bottom_right);
wxRegion *wxRegion_new3(const wxRect * rect);
wxRegion *wxRegion_new4(const wxRegion * region);
wxRegion *wxRegion_new6(const wxBitmap * bmp);
wxRegion *wxRegion_new7(const wxBitmap * bmp, const wxColour * trans_colour, int tolerance);
void wxRegion_Clear(wxRegion * self);
wxBitmap *wxRegion_ConvertToBitmap(const wxRegion * self);
wxRect *wxRegion_GetBox1(const wxRegion * self);
bool wxRegion_Intersect(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
bool wxRegion_Intersect1(wxRegion * self, const wxRect * rect);
bool wxRegion_Intersect2(wxRegion * self, const wxRegion * region);
bool wxRegion_IsEmpty(const wxRegion * self);
bool wxRegion_IsEqual(const wxRegion * self, const wxRegion * region);
bool wxRegion_Offset(wxRegion * self, wxCoord x, wxCoord y);
bool wxRegion_Offset1(wxRegion * self, const wxPoint * pt);
bool wxRegion_Subtract(wxRegion * self, const wxRect * rect);
bool wxRegion_Subtract1(wxRegion * self, const wxRegion * region);
bool wxRegion_Union(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
bool wxRegion_Union1(wxRegion * self, const wxRect * rect);
bool wxRegion_Union2(wxRegion * self, const wxRegion * region);
bool wxRegion_Union3(wxRegion * self, const wxBitmap * bmp);
bool wxRegion_Union4(wxRegion * self, const wxBitmap * bmp, const wxColour * trans_colour, int tolerance);
bool wxRegion_Xor(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
bool wxRegion_Xor1(wxRegion * self, const wxRect * rect);
bool wxRegion_Xor2(wxRegion * self, const wxRegion * region);

// CLASS: wxRegionIterator
wxClassInfo *wxRegionIterator_CLASSINFO();
wxRegionIterator *wxRegionIterator_new();
wxRegionIterator *wxRegionIterator_new1(const wxRegion * region);
wxCoord wxRegionIterator_GetH(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetHeight(const wxRegionIterator * self);
wxRect *wxRegionIterator_GetRect(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetW(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetWidth(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetX(const wxRegionIterator * self);
wxCoord wxRegionIterator_GetY(const wxRegionIterator * self);
bool wxRegionIterator_HaveRects(const wxRegionIterator * self);
void wxRegionIterator_Reset(wxRegionIterator * self);
void wxRegionIterator_Reset1(wxRegionIterator * self, const wxRegion * region);

// CLASS: wxRendererNative
void wxRendererNative_delete(wxRendererNative *self);
void wxRendererNative_DrawCheckBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawComboBoxDropButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawDropArrow(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawFocusRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawGauge(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int value, int max, int flags);
#endif
void wxRendererNative_DrawItemSelectionRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawItemText(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxString * text, const wxRect * rect, int align, int flags, wxEllipsizeMode ellipsize_mode);
#endif
void wxRendererNative_DrawPushButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawCollapseButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
wxSize *wxRendererNative_GetCollapseButtonSize(wxRendererNative * self, wxWindow * win, wxDC * dc);
#endif
void wxRendererNative_DrawSplitterBorder(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawTreeItemButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawChoice(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawComboBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawTextCtrl(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
void wxRendererNative_DrawRadioBitmap(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawCheckMark(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags);
wxSize *wxRendererNative_GetCheckBoxSize(wxRendererNative * self, wxWindow * win, int flags);
wxSize *wxRendererNative_GetCheckMarkSize(wxRendererNative * self, wxWindow * win);
wxSize *wxRendererNative_GetExpanderSize(wxRendererNative * self, wxWindow * win);
#endif
int wxRendererNative_GetHeaderButtonHeight(wxRendererNative * self, wxWindow * win);
int wxRendererNative_GetHeaderButtonMargin(wxRendererNative * self, wxWindow * win);
wxRendererNative * wxRendererNative_Get();
wxRendererNative * wxRendererNative_GetDefault();
wxRendererNative * wxRendererNative_GetGeneric();
wxRendererNative * wxRendererNative_Load(const wxString * name);
wxRendererNative * wxRendererNative_Set(wxRendererNative * renderer);

// CLASS: wxRichToolTip
void wxRichToolTip_delete(wxRichToolTip *self);
wxRichToolTip *wxRichToolTip_new(const wxString * title, const wxString * message);
void wxRichToolTip_SetBackgroundColour(wxRichToolTip * self, const wxColour * col, const wxColour * col_end);
void wxRichToolTip_SetIcon(wxRichToolTip * self, int icon);
#if wxCHECK_VERSION(3, 2, 0)
void wxRichToolTip_SetIcon1(wxRichToolTip * self, const wxBitmapBundle * icon);
#endif
void wxRichToolTip_SetTitleFont(wxRichToolTip * self, const wxFont * font);
void wxRichToolTip_ShowFor(wxRichToolTip * self, wxWindow * win, const wxRect * rect);

} // extern "C"

