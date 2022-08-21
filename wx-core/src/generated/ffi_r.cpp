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

// CLASS: wxRadioButton
wxClassInfo *wxRadioButton_CLASSINFO() {
    return wxCLASSINFO(wxRadioButton);
}
wxRadioButton *wxRadioButton_new() {
    return new wxRadioButton();
}
wxRadioButton *wxRadioButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxRadioButton(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxRadioButton_Create(wxRadioButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxRadioButton_GetValue(const wxRadioButton * self) {
    return self->GetValue();
}
void wxRadioButton_SetValue(wxRadioButton * self, bool value) {
    return self->SetValue(value);
}
#if wxCHECK_VERSION(3, 1, 0)
wxRadioButton * wxRadioButton_GetFirstInGroup(const wxRadioButton * self) {
    return self->GetFirstInGroup();
}
wxRadioButton * wxRadioButton_GetLastInGroup(const wxRadioButton * self) {
    return self->GetLastInGroup();
}
wxRadioButton * wxRadioButton_GetPreviousInGroup(const wxRadioButton * self) {
    return self->GetPreviousInGroup();
}
wxRadioButton * wxRadioButton_GetNextInGroup(const wxRadioButton * self) {
    return self->GetNextInGroup();
}
#endif

// CLASS: wxRealPoint
void wxRealPoint_delete(wxRealPoint *self) {
    delete self;
}
wxRealPoint *wxRealPoint_new() {
    return new wxRealPoint();
}
wxRealPoint *wxRealPoint_new1(double x, double y) {
    return new wxRealPoint(x, y);
}
wxRealPoint *wxRealPoint_new2(const wxPoint * pt) {
    return new wxRealPoint(*pt);
}

// CLASS: wxRearrangeCtrl
wxClassInfo *wxRearrangeCtrl_CLASSINFO() {
    return wxCLASSINFO(wxRearrangeCtrl);
}
wxRearrangeCtrl *wxRearrangeCtrl_new() {
    return new wxRearrangeCtrl();
}
wxRearrangeCtrl *wxRearrangeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return new wxRearrangeCtrl(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
bool wxRearrangeCtrl_Create(wxRearrangeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
wxRearrangeList * wxRearrangeCtrl_GetList(const wxRearrangeCtrl * self) {
    return self->GetList();
}

// CLASS: wxRearrangeDialog
wxClassInfo *wxRearrangeDialog_CLASSINFO() {
    return wxCLASSINFO(wxRearrangeDialog);
}
wxRearrangeDialog *wxRearrangeDialog_new() {
    return new wxRearrangeDialog();
}
wxRearrangeDialog *wxRearrangeDialog_new1(wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name) {
    return new wxRearrangeDialog(parent, *message, *title, *order, *items, *pos, *name);
}
bool wxRearrangeDialog_Create(wxRearrangeDialog * self, wxWindow * parent, const wxString * message, const wxString * title, const wxArrayInt * order, const wxArrayString * items, const wxPoint * pos, const wxString * name) {
    return self->Create(parent, *message, *title, *order, *items, *pos, *name);
}
void wxRearrangeDialog_AddExtraControls(wxRearrangeDialog * self, wxWindow * win) {
    return self->AddExtraControls(win);
}
wxRearrangeList * wxRearrangeDialog_GetList(const wxRearrangeDialog * self) {
    return self->GetList();
}
wxArrayInt *wxRearrangeDialog_GetOrder(const wxRearrangeDialog * self) {
    return new wxArrayInt(self->GetOrder());
}

// CLASS: wxRearrangeList
wxClassInfo *wxRearrangeList_CLASSINFO() {
    return wxCLASSINFO(wxRearrangeList);
}
wxRearrangeList *wxRearrangeList_new() {
    return new wxRearrangeList();
}
wxRearrangeList *wxRearrangeList_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return new wxRearrangeList(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
bool wxRearrangeList_Create(wxRearrangeList * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayInt * order, const wxArrayString * items, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *order, *items, style, *validator, *name);
}
wxArrayInt *wxRearrangeList_GetCurrentOrder(const wxRearrangeList * self) {
    return new wxArrayInt(self->GetCurrentOrder());
}
bool wxRearrangeList_CanMoveCurrentUp(const wxRearrangeList * self) {
    return self->CanMoveCurrentUp();
}
bool wxRearrangeList_CanMoveCurrentDown(const wxRearrangeList * self) {
    return self->CanMoveCurrentDown();
}
bool wxRearrangeList_MoveCurrentUp(wxRearrangeList * self) {
    return self->MoveCurrentUp();
}
bool wxRearrangeList_MoveCurrentDown(wxRearrangeList * self) {
    return self->MoveCurrentDown();
}
// Mix-in(s) to wxRearrangeList
wxItemContainer *wxRearrangeList_AsItemContainer(wxRearrangeList* obj) {
    return static_cast<wxItemContainer*>(obj);
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

// CLASS: wxRegion
wxClassInfo *wxRegion_CLASSINFO() {
    return wxCLASSINFO(wxRegion);
}
wxRegion *wxRegion_new() {
    return new wxRegion();
}
wxRegion *wxRegion_new1(wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return new wxRegion(x, y, width, height);
}
wxRegion *wxRegion_new2(const wxPoint * top_left, const wxPoint * bottom_right) {
    return new wxRegion(*top_left, *bottom_right);
}
wxRegion *wxRegion_new3(const wxRect * rect) {
    return new wxRegion(*rect);
}
wxRegion *wxRegion_new4(const wxRegion * region) {
    return new wxRegion(*region);
}
wxRegion *wxRegion_new6(const wxBitmap * bmp) {
    return new wxRegion(*bmp);
}
wxRegion *wxRegion_new7(const wxBitmap * bmp, const wxColour * trans_colour, int tolerance) {
    return new wxRegion(*bmp, *trans_colour, tolerance);
}
void wxRegion_Clear(wxRegion * self) {
    return self->Clear();
}
wxBitmap *wxRegion_ConvertToBitmap(const wxRegion * self) {
    return new wxBitmap(self->ConvertToBitmap());
}
wxRect *wxRegion_GetBox1(const wxRegion * self) {
    return new wxRect(self->GetBox());
}
bool wxRegion_Intersect(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->Intersect(x, y, width, height);
}
bool wxRegion_Intersect1(wxRegion * self, const wxRect * rect) {
    return self->Intersect(*rect);
}
bool wxRegion_Intersect2(wxRegion * self, const wxRegion * region) {
    return self->Intersect(*region);
}
bool wxRegion_IsEmpty(const wxRegion * self) {
    return self->IsEmpty();
}
bool wxRegion_IsEqual(const wxRegion * self, const wxRegion * region) {
    return self->IsEqual(*region);
}
bool wxRegion_Offset(wxRegion * self, wxCoord x, wxCoord y) {
    return self->Offset(x, y);
}
bool wxRegion_Offset1(wxRegion * self, const wxPoint * pt) {
    return self->Offset(*pt);
}
bool wxRegion_Subtract(wxRegion * self, const wxRect * rect) {
    return self->Subtract(*rect);
}
bool wxRegion_Subtract1(wxRegion * self, const wxRegion * region) {
    return self->Subtract(*region);
}
bool wxRegion_Union(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->Union(x, y, width, height);
}
bool wxRegion_Union1(wxRegion * self, const wxRect * rect) {
    return self->Union(*rect);
}
bool wxRegion_Union2(wxRegion * self, const wxRegion * region) {
    return self->Union(*region);
}
bool wxRegion_Union3(wxRegion * self, const wxBitmap * bmp) {
    return self->Union(*bmp);
}
bool wxRegion_Union4(wxRegion * self, const wxBitmap * bmp, const wxColour * trans_colour, int tolerance) {
    return self->Union(*bmp, *trans_colour, tolerance);
}
bool wxRegion_Xor(wxRegion * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->Xor(x, y, width, height);
}
bool wxRegion_Xor1(wxRegion * self, const wxRect * rect) {
    return self->Xor(*rect);
}
bool wxRegion_Xor2(wxRegion * self, const wxRegion * region) {
    return self->Xor(*region);
}

// CLASS: wxRegionIterator
wxClassInfo *wxRegionIterator_CLASSINFO() {
    return wxCLASSINFO(wxRegionIterator);
}
wxRegionIterator *wxRegionIterator_new() {
    return new wxRegionIterator();
}
wxRegionIterator *wxRegionIterator_new1(const wxRegion * region) {
    return new wxRegionIterator(*region);
}
wxCoord wxRegionIterator_GetH(const wxRegionIterator * self) {
    return self->GetH();
}
wxCoord wxRegionIterator_GetHeight(const wxRegionIterator * self) {
    return self->GetHeight();
}
wxRect *wxRegionIterator_GetRect(const wxRegionIterator * self) {
    return new wxRect(self->GetRect());
}
wxCoord wxRegionIterator_GetW(const wxRegionIterator * self) {
    return self->GetW();
}
wxCoord wxRegionIterator_GetWidth(const wxRegionIterator * self) {
    return self->GetWidth();
}
wxCoord wxRegionIterator_GetX(const wxRegionIterator * self) {
    return self->GetX();
}
wxCoord wxRegionIterator_GetY(const wxRegionIterator * self) {
    return self->GetY();
}
bool wxRegionIterator_HaveRects(const wxRegionIterator * self) {
    return self->HaveRects();
}
void wxRegionIterator_Reset(wxRegionIterator * self) {
    return self->Reset();
}
void wxRegionIterator_Reset1(wxRegionIterator * self, const wxRegion * region) {
    return self->Reset(*region);
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
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawGauge(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int value, int max, int flags) {
    return self->DrawGauge(win, *dc, *rect, value, max, flags);
}
#endif
void wxRendererNative_DrawItemSelectionRect(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawItemSelectionRect(win, *dc, *rect, flags);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawItemText(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxString * text, const wxRect * rect, int align, int flags, wxEllipsizeMode ellipsize_mode) {
    return self->DrawItemText(win, *dc, *text, *rect, align, flags, ellipsize_mode);
}
#endif
void wxRendererNative_DrawPushButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawPushButton(win, *dc, *rect, flags);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxRendererNative_DrawCollapseButton(wxRendererNative * self, wxWindow * win, wxDC * dc, const wxRect * rect, int flags) {
    return self->DrawCollapseButton(win, *dc, *rect, flags);
}
wxSize *wxRendererNative_GetCollapseButtonSize(wxRendererNative * self, wxWindow * win, wxDC * dc) {
    return new wxSize(self->GetCollapseButtonSize(win, *dc));
}
#endif
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
#if wxCHECK_VERSION(3, 1, 0)
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
#endif
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

// CLASS: wxRichToolTip
void wxRichToolTip_delete(wxRichToolTip *self) {
    delete self;
}
wxRichToolTip *wxRichToolTip_new(const wxString * title, const wxString * message) {
    return new wxRichToolTip(*title, *message);
}
void wxRichToolTip_SetBackgroundColour(wxRichToolTip * self, const wxColour * col, const wxColour * col_end) {
    return self->SetBackgroundColour(*col, *col_end);
}
void wxRichToolTip_SetIcon(wxRichToolTip * self, int icon) {
    return self->SetIcon(icon);
}
#if wxCHECK_VERSION(3, 2, 0)
void wxRichToolTip_SetIcon1(wxRichToolTip * self, const wxBitmapBundle * icon) {
    return self->SetIcon(*icon);
}
#endif
void wxRichToolTip_SetTitleFont(wxRichToolTip * self, const wxFont * font) {
    return self->SetTitleFont(*font);
}
void wxRichToolTip_ShowFor(wxRichToolTip * self, wxWindow * win, const wxRect * rect) {
    return self->ShowFor(win, rect);
}

} // extern "C"

