#include "generated.h"

extern "C" {

// CLASS: wxRadioBox
wxClassInfo *wxRadioBox_CLASSINFO() {
    return wxCLASSINFO(wxRadioBox);
}
wxRadioBox *wxRadioBox_new() {
    return new wxRadioBox();
}
wxRadioBox *wxRadioBox_new2(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name) {
    return new wxRadioBox(parent, id, *label, *pos, *size, *choices, major_dimension, style, *validator, *name);
}
bool wxRadioBox_Create1(wxRadioBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, int major_dimension, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, *choices, major_dimension, style, *validator, *name);
}
bool wxRadioBox_Enable(wxRadioBox * self, unsigned int n, bool enable) {
    return self->Enable(n, enable);
}
unsigned int wxRadioBox_GetColumnCount(const wxRadioBox * self) {
    return self->GetColumnCount();
}
int wxRadioBox_GetItemFromPoint(const wxRadioBox * self, const wxPoint * pt) {
    return self->GetItemFromPoint(*pt);
}
#if wxCHECK_VERSION(3, 1, 7)
wxString *wxRadioBox_GetItemHelpText(const wxRadioBox * self, unsigned int item) {
    return new wxString(self->GetItemHelpText(item));
}
#endif
wxToolTip * wxRadioBox_GetItemToolTip(const wxRadioBox * self, unsigned int item) {
    return self->GetItemToolTip(item);
}
unsigned int wxRadioBox_GetRowCount(const wxRadioBox * self) {
    return self->GetRowCount();
}
bool wxRadioBox_IsItemEnabled(const wxRadioBox * self, unsigned int n) {
    return self->IsItemEnabled(n);
}
bool wxRadioBox_IsItemShown(const wxRadioBox * self, unsigned int n) {
    return self->IsItemShown(n);
}
void wxRadioBox_SetItemHelpText(wxRadioBox * self, unsigned int item, const wxString * helptext) {
    return self->SetItemHelpText(item, *helptext);
}
void wxRadioBox_SetItemToolTip(wxRadioBox * self, unsigned int item, const wxString * text) {
    return self->SetItemToolTip(item, *text);
}
bool wxRadioBox_Show(wxRadioBox * self, unsigned int item, bool show) {
    return self->Show(item, show);
}
// Mix-in(s) to wxRadioBox
wxItemContainerImmutable *wxRadioBox_AsItemContainerImmutable(wxRadioBox* obj) {
    return static_cast<wxItemContainerImmutable*>(obj);
}

// CLASS: wxRect
void wxRect_delete(wxRect *self) {
    delete self;
}
wxRect *wxRect_new() {
    return new wxRect();
}
wxRect *wxRect_new1(int x, int y, int width, int height) {
    return new wxRect(x, y, width, height);
}
wxRect *wxRect_new2(const wxPoint * top_left, const wxPoint * bottom_right) {
    return new wxRect(*top_left, *bottom_right);
}
wxRect *wxRect_new3(const wxPoint * pos, const wxSize * size) {
    return new wxRect(*pos, *size);
}
wxRect *wxRect_new4(const wxSize * size) {
    return new wxRect(*size);
}
wxRect *wxRect_CentreIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(self->CentreIn(*r, dir));
}
wxRect *wxRect_CenterIn(const wxRect * self, const wxRect * r, int dir) {
    return new wxRect(self->CenterIn(*r, dir));
}
bool wxRect_Contains(const wxRect * self, int x, int y) {
    return self->Contains(x, y);
}
bool wxRect_Contains1(const wxRect * self, const wxPoint * pt) {
    return self->Contains(*pt);
}
bool wxRect_Contains2(const wxRect * self, const wxRect * rect) {
    return self->Contains(*rect);
}
wxRect *wxRect_Deflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(self->Deflate(dx, dy));
}
int wxRect_GetBottom(const wxRect * self) {
    return self->GetBottom();
}
wxPoint *wxRect_GetBottomLeft(const wxRect * self) {
    return new wxPoint(self->GetBottomLeft());
}
wxPoint *wxRect_GetBottomRight(const wxRect * self) {
    return new wxPoint(self->GetBottomRight());
}
int wxRect_GetHeight(const wxRect * self) {
    return self->GetHeight();
}
int wxRect_GetLeft(const wxRect * self) {
    return self->GetLeft();
}
wxPoint *wxRect_GetPosition(const wxRect * self) {
    return new wxPoint(self->GetPosition());
}
int wxRect_GetRight(const wxRect * self) {
    return self->GetRight();
}
wxSize *wxRect_GetSize(const wxRect * self) {
    return new wxSize(self->GetSize());
}
int wxRect_GetTop(const wxRect * self) {
    return self->GetTop();
}
wxPoint *wxRect_GetTopLeft(const wxRect * self) {
    return new wxPoint(self->GetTopLeft());
}
wxPoint *wxRect_GetTopRight(const wxRect * self) {
    return new wxPoint(self->GetTopRight());
}
int wxRect_GetWidth(const wxRect * self) {
    return self->GetWidth();
}
int wxRect_GetX(const wxRect * self) {
    return self->GetX();
}
int wxRect_GetY(const wxRect * self) {
    return self->GetY();
}
wxRect *wxRect_Inflate3(const wxRect * self, wxCoord dx, wxCoord dy) {
    return new wxRect(self->Inflate(dx, dy));
}
wxRect *wxRect_Intersect1(const wxRect * self, const wxRect * rect) {
    return new wxRect(self->Intersect(*rect));
}
bool wxRect_Intersects(const wxRect * self, const wxRect * rect) {
    return self->Intersects(*rect);
}
bool wxRect_IsEmpty(const wxRect * self) {
    return self->IsEmpty();
}
void wxRect_Offset(wxRect * self, wxCoord dx, wxCoord dy) {
    return self->Offset(dx, dy);
}
void wxRect_Offset1(wxRect * self, const wxPoint * pt) {
    return self->Offset(*pt);
}
void wxRect_SetHeight(wxRect * self, int height) {
    return self->SetHeight(height);
}
void wxRect_SetPosition(wxRect * self, const wxPoint * pos) {
    return self->SetPosition(*pos);
}
void wxRect_SetSize(wxRect * self, const wxSize * s) {
    return self->SetSize(*s);
}
void wxRect_SetWidth(wxRect * self, int width) {
    return self->SetWidth(width);
}
void wxRect_SetX(wxRect * self, int x) {
    return self->SetX(x);
}
void wxRect_SetY(wxRect * self, int y) {
    return self->SetY(y);
}
void wxRect_SetLeft(wxRect * self, int left) {
    return self->SetLeft(left);
}
void wxRect_SetRight(wxRect * self, int right) {
    return self->SetRight(right);
}
void wxRect_SetTop(wxRect * self, int top) {
    return self->SetTop(top);
}
void wxRect_SetBottom(wxRect * self, int bottom) {
    return self->SetBottom(bottom);
}
void wxRect_SetTopLeft(wxRect * self, const wxPoint * p) {
    return self->SetTopLeft(*p);
}
void wxRect_SetBottomRight(wxRect * self, const wxPoint * p) {
    return self->SetBottomRight(*p);
}
void wxRect_SetTopRight(wxRect * self, const wxPoint * p) {
    return self->SetTopRight(*p);
}
void wxRect_SetBottomLeft(wxRect * self, const wxPoint * p) {
    return self->SetBottomLeft(*p);
}
wxRect *wxRect_Union(const wxRect * self, const wxRect * rect) {
    return new wxRect(self->Union(*rect));
}

// CLASS: wxRendererNative
void wxRendererNative_delete(wxRendererNative *self) {
    delete self;
}
void wxRendererNative_DrawCheckBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCheckBox(win, *dc, *rect, flags);
}
void wxRendererNative_DrawComboBoxDropButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawComboBoxDropButton(win, *dc, *rect, flags);
}
void wxRendererNative_DrawDropArrow(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawDropArrow(win, *dc, *rect, flags);
}
void wxRendererNative_DrawFocusRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawFocusRect(win, *dc, *rect, flags);
}
void wxRendererNative_DrawGauge(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int value, int max, int flags) {
    return self->DrawGauge(win, *dc, *rect, value, max, flags);
}
void wxRendererNative_DrawItemSelectionRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawItemSelectionRect(win, *dc, *rect, flags);
}
void wxRendererNative_DrawItemText(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxString * text, const wxRect * rect, int align, int flags, wxEllipsizeMode ellipsize_mode) {
    return self->DrawItemText(win, *dc, *text, *rect, align, flags, ellipsize_mode);
}
void wxRendererNative_DrawPushButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawPushButton(win, *dc, *rect, flags);
}
void wxRendererNative_DrawCollapseButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCollapseButton(win, *dc, *rect, flags);
}
wxSize *wxRendererNative_GetCollapseButtonSize(wxRendererNative * self, wxWindow * win, wxDC * dc) {
    return new wxSize(self->GetCollapseButtonSize(win, *dc));
}
void wxRendererNative_DrawSplitterBorder(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawSplitterBorder(win, *dc, *rect, flags);
}
void wxRendererNative_DrawTreeItemButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawTreeItemButton(win, *dc, *rect, flags);
}
void wxRendererNative_DrawChoice(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawChoice(win, *dc, *rect, flags);
}
void wxRendererNative_DrawComboBox(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawComboBox(win, *dc, *rect, flags);
}
void wxRendererNative_DrawTextCtrl(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawTextCtrl(win, *dc, *rect, flags);
}
void wxRendererNative_DrawRadioBitmap(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawRadioBitmap(win, *dc, *rect, flags);
}
void wxRendererNative_DrawCheckMark(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCheckMark(win, *dc, *rect, flags);
}
wxSize *wxRendererNative_GetCheckBoxSize(wxRendererNative * self, wxWindow * win, int flags) {
    return new wxSize(self->GetCheckBoxSize(win, flags));
}
wxSize *wxRendererNative_GetCheckMarkSize(wxRendererNative * self, wxWindow * win) {
    return new wxSize(self->GetCheckMarkSize(win));
}
wxSize *wxRendererNative_GetExpanderSize(wxRendererNative * self, wxWindow * win) {
    return new wxSize(self->GetExpanderSize(win));
}
int wxRendererNative_GetHeaderButtonHeight(wxRendererNative * self, wxWindow * win) {
    return self->GetHeaderButtonHeight(win);
}
int wxRendererNative_GetHeaderButtonMargin(wxRendererNative * self, wxWindow * win) {
    return self->GetHeaderButtonMargin(win);
}
wxRendererNative * wxRendererNative_Get() {
    return &(wxRendererNative::Get());
}
wxRendererNative * wxRendererNative_GetDefault() {
    return &(wxRendererNative::GetDefault());
}
wxRendererNative * wxRendererNative_GetGeneric() {
    return &(wxRendererNative::GetGeneric());
}
wxRendererNative * wxRendererNative_Load(const wxString * name) {
    return wxRendererNative::Load(*name);
}
wxRendererNative * wxRendererNative_Set(wxRendererNative * renderer) {
    return wxRendererNative::Set(renderer);
}

} // extern "C"

