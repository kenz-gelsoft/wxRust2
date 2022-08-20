#include "generated.h"

extern "C" {

// CLASS: wxLayoutAlgorithm
wxClassInfo *wxLayoutAlgorithm_CLASSINFO() {
    return wxCLASSINFO(wxLayoutAlgorithm);
}
wxLayoutAlgorithm *wxLayoutAlgorithm_new() {
    return new wxLayoutAlgorithm();
}
bool wxLayoutAlgorithm_LayoutFrame(wxLayoutAlgorithm * self, wxFrame * frame, wxWindow * main_window) {
    return self->LayoutFrame(frame, main_window);
}
bool wxLayoutAlgorithm_LayoutMDIFrame(wxLayoutAlgorithm * self, wxMDIParentFrame * frame, wxRect * rect) {
    return self->LayoutMDIFrame(frame, rect);
}
bool wxLayoutAlgorithm_LayoutWindow(wxLayoutAlgorithm * self, wxWindow * parent, wxWindow * main_window) {
    return self->LayoutWindow(parent, main_window);
}

// CLASS: wxListBox
wxClassInfo *wxListBox_CLASSINFO() {
    return wxCLASSINFO(wxListBox);
}
wxListBox *wxListBox_new() {
    return new wxListBox();
}
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxListBox(parent, id, *pos, *size, *choices, style, *validator, *name);
}
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, *choices, style, *validator, *name);
}
void wxListBox_Deselect(wxListBox * self, int n) {
    return self->Deselect(n);
}
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select) {
    return self->SetStringSelection(*s, select);
}
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s) {
    return self->SetStringSelection(*s);
}
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections) {
    return self->GetSelections(*selections);
}
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point) {
    return self->HitTest(*point);
}
int wxListBox_HitTest1(const wxListBox * self, int x, int y) {
    return self->HitTest(x, y);
}
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos) {
    return self->InsertItems(*items, pos);
}
bool wxListBox_IsSelected(const wxListBox * self, int n) {
    return self->IsSelected(n);
}
void wxListBox_SetFirstItem(wxListBox * self, int n) {
    return self->SetFirstItem(n);
}
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string) {
    return self->SetFirstItem(*string);
}
void wxListBox_EnsureVisible(wxListBox * self, int n) {
    return self->EnsureVisible(n);
}
bool wxListBox_IsSorted(const wxListBox * self) {
    return self->IsSorted();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self) {
    return self->GetCountPerPage();
}
int wxListBox_GetTopItem(const wxListBox * self) {
    return self->GetTopItem();
}
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}

// CLASS: wxListCtrl
wxClassInfo *wxListCtrl_CLASSINFO() {
    return wxCLASSINFO(wxListCtrl);
}
wxListCtrl *wxListCtrl_new() {
    return new wxListCtrl();
}
wxListCtrl *wxListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxListCtrl(parent, id, *pos, *size, style, *validator, *name);
}
bool wxListCtrl_Arrange(wxListCtrl * self, int flag) {
    return self->Arrange(flag);
}
void wxListCtrl_AssignImageList(wxListCtrl * self, wxImageList * image_list, int which) {
    return self->AssignImageList(image_list, which);
}
void wxListCtrl_ClearAll(wxListCtrl * self) {
    return self->ClearAll();
}
bool wxListCtrl_Create(wxListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
bool wxListCtrl_DeleteAllColumns(wxListCtrl * self) {
    return self->DeleteAllColumns();
}
bool wxListCtrl_DeleteAllItems(wxListCtrl * self) {
    return self->DeleteAllItems();
}
bool wxListCtrl_DeleteColumn(wxListCtrl * self, int col) {
    return self->DeleteColumn(col);
}
bool wxListCtrl_DeleteItem(wxListCtrl * self, long item) {
    return self->DeleteItem(item);
}
wxTextCtrl * wxListCtrl_EditLabel(wxListCtrl * self, long item, wxClassInfo * text_control_class) {
    return self->EditLabel(item, text_control_class);
}
void wxListCtrl_EnableAlternateRowColours(wxListCtrl * self, bool enable) {
    return self->EnableAlternateRowColours(enable);
}
void wxListCtrl_EnableBellOnNoMatch(wxListCtrl * self, bool on) {
    return self->EnableBellOnNoMatch(on);
}
bool wxListCtrl_EndEditLabel(wxListCtrl * self, bool cancel) {
    return self->EndEditLabel(cancel);
}
bool wxListCtrl_EnsureVisible(wxListCtrl * self, long item) {
    return self->EnsureVisible(item);
}
long wxListCtrl_FindItem(wxListCtrl * self, long start, const wxString * str, bool partial) {
    return self->FindItem(start, *str, partial);
}
long wxListCtrl_FindItem2(wxListCtrl * self, long start, const wxPoint * pt, int direction) {
    return self->FindItem(start, *pt, direction);
}
bool wxListCtrl_GetColumn(const wxListCtrl * self, int col, wxListItem * item) {
    return self->GetColumn(col, *item);
}
int wxListCtrl_GetColumnCount(const wxListCtrl * self) {
    return self->GetColumnCount();
}
#if wxCHECK_VERSION(3, 1, 0)
int wxListCtrl_GetColumnIndexFromOrder(const wxListCtrl * self, int pos) {
    return self->GetColumnIndexFromOrder(pos);
}
int wxListCtrl_GetColumnOrder(const wxListCtrl * self, int col) {
    return self->GetColumnOrder(col);
}
#endif
int wxListCtrl_GetColumnWidth(const wxListCtrl * self, int col) {
    return self->GetColumnWidth(col);
}
#if wxCHECK_VERSION(3, 1, 0)
wxArrayInt *wxListCtrl_GetColumnsOrder(const wxListCtrl * self) {
    return new wxArrayInt(self->GetColumnsOrder());
}
#endif
int wxListCtrl_GetCountPerPage(const wxListCtrl * self) {
    return self->GetCountPerPage();
}
wxTextCtrl * wxListCtrl_GetEditControl(const wxListCtrl * self) {
    return self->GetEditControl();
}
wxImageList * wxListCtrl_GetImageList(const wxListCtrl * self, int which) {
    return self->GetImageList(which);
}
bool wxListCtrl_GetItem(const wxListCtrl * self, wxListItem * info) {
    return self->GetItem(*info);
}
wxColour *wxListCtrl_GetItemBackgroundColour(const wxListCtrl * self, long item) {
    return new wxColour(self->GetItemBackgroundColour(item));
}
int wxListCtrl_GetItemCount(const wxListCtrl * self) {
    return self->GetItemCount();
}
wxFont *wxListCtrl_GetItemFont(const wxListCtrl * self, long item) {
    return new wxFont(self->GetItemFont(item));
}
bool wxListCtrl_GetItemPosition(const wxListCtrl * self, long item, wxPoint * pos) {
    return self->GetItemPosition(item, *pos);
}
bool wxListCtrl_GetItemRect(const wxListCtrl * self, long item, wxRect * rect, int code) {
    return self->GetItemRect(item, *rect, code);
}
wxSize *wxListCtrl_GetItemSpacing(const wxListCtrl * self) {
    return new wxSize(self->GetItemSpacing());
}
int wxListCtrl_GetItemState(const wxListCtrl * self, long item, long state_mask) {
    return self->GetItemState(item, state_mask);
}
wxString *wxListCtrl_GetItemText(const wxListCtrl * self, long item, int col) {
    return new wxString(self->GetItemText(item, col));
}
wxColour *wxListCtrl_GetItemTextColour(const wxListCtrl * self, long item) {
    return new wxColour(self->GetItemTextColour(item));
}
long wxListCtrl_GetNextItem(const wxListCtrl * self, long item, int geometry, int state) {
    return self->GetNextItem(item, geometry, state);
}
int wxListCtrl_GetSelectedItemCount(const wxListCtrl * self) {
    return self->GetSelectedItemCount();
}
bool wxListCtrl_GetSubItemRect(const wxListCtrl * self, long item, long sub_item, wxRect * rect, int code) {
    return self->GetSubItemRect(item, sub_item, *rect, code);
}
wxColour *wxListCtrl_GetTextColour(const wxListCtrl * self) {
    return new wxColour(self->GetTextColour());
}
long wxListCtrl_GetTopItem(const wxListCtrl * self) {
    return self->GetTopItem();
}
wxRect *wxListCtrl_GetViewRect(const wxListCtrl * self) {
    return new wxRect(self->GetViewRect());
}
void wxListCtrl_SetAlternateRowColour(wxListCtrl * self, const wxColour * colour) {
    return self->SetAlternateRowColour(*colour);
}
#if wxCHECK_VERSION(3, 1, 0)
wxColour *wxListCtrl_GetAlternateRowColour(const wxListCtrl * self) {
    return new wxColour(self->GetAlternateRowColour());
}
#endif
long wxListCtrl_HitTest(const wxListCtrl * self, const wxPoint * point, int * flags, long * ptr_sub_item) {
    return self->HitTest(*point, *flags, ptr_sub_item);
}
bool wxListCtrl_InReportView(const wxListCtrl * self) {
    return self->InReportView();
}
long wxListCtrl_InsertColumn(wxListCtrl * self, long col, const wxListItem * info) {
    return self->InsertColumn(col, *info);
}
long wxListCtrl_InsertColumn1(wxListCtrl * self, long col, const wxString * heading, int format, int width) {
    return self->InsertColumn(col, *heading, format, width);
}
long wxListCtrl_InsertItem(wxListCtrl * self, wxListItem * info) {
    return self->InsertItem(*info);
}
long wxListCtrl_InsertItem1(wxListCtrl * self, long index, const wxString * label) {
    return self->InsertItem(index, *label);
}
long wxListCtrl_InsertItem2(wxListCtrl * self, long index, int image_index) {
    return self->InsertItem(index, image_index);
}
long wxListCtrl_InsertItem3(wxListCtrl * self, long index, const wxString * label, int image_index) {
    return self->InsertItem(index, *label, image_index);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxListCtrl_IsEmpty(const wxListCtrl * self) {
    return self->IsEmpty();
}
#endif
bool wxListCtrl_IsVirtual(const wxListCtrl * self) {
    return self->IsVirtual();
}
void wxListCtrl_RefreshItem(wxListCtrl * self, long item) {
    return self->RefreshItem(item);
}
void wxListCtrl_RefreshItems(wxListCtrl * self, long item_from, long item_to) {
    return self->RefreshItems(item_from, item_to);
}
bool wxListCtrl_ScrollList(wxListCtrl * self, int dx, int dy) {
    return self->ScrollList(dx, dy);
}
bool wxListCtrl_SetColumn(wxListCtrl * self, int col, wxListItem * item) {
    return self->SetColumn(col, *item);
}
bool wxListCtrl_SetColumnWidth(wxListCtrl * self, int col, int width) {
    return self->SetColumnWidth(col, width);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxListCtrl_SetColumnsOrder(wxListCtrl * self, const wxArrayInt * orders) {
    return self->SetColumnsOrder(*orders);
}
bool wxListCtrl_SetHeaderAttr(wxListCtrl * self, const wxItemAttr * attr) {
    return self->SetHeaderAttr(*attr);
}
#endif
void wxListCtrl_SetImageList(wxListCtrl * self, wxImageList * image_list, int which) {
    return self->SetImageList(image_list, which);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxListCtrl_SetNormalImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images) {
    return self->SetNormalImages(*images);
}
void wxListCtrl_SetSmallImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images) {
    return self->SetSmallImages(*images);
}
bool wxListCtrl_IsVisible(const wxListCtrl * self, long item) {
    return self->IsVisible(item);
}
#endif
bool wxListCtrl_SetItem(wxListCtrl * self, wxListItem * info) {
    return self->SetItem(*info);
}
bool wxListCtrl_SetItem1(wxListCtrl * self, long index, int column, const wxString * label, int image_id) {
    return self->SetItem(index, column, *label, image_id);
}
void wxListCtrl_SetItemBackgroundColour(wxListCtrl * self, long item, const wxColour * col) {
    return self->SetItemBackgroundColour(item, *col);
}
bool wxListCtrl_SetItemColumnImage(wxListCtrl * self, long item, long column, int image) {
    return self->SetItemColumnImage(item, column, image);
}
void wxListCtrl_SetItemCount(wxListCtrl * self, long count) {
    return self->SetItemCount(count);
}
bool wxListCtrl_SetItemData(wxListCtrl * self, long item, long data) {
    return self->SetItemData(item, data);
}
void wxListCtrl_SetItemFont(wxListCtrl * self, long item, const wxFont * font) {
    return self->SetItemFont(item, *font);
}
bool wxListCtrl_SetItemImage(wxListCtrl * self, long item, int image, int sel_image) {
    return self->SetItemImage(item, image, sel_image);
}
bool wxListCtrl_SetItemPosition(wxListCtrl * self, long item, const wxPoint * pos) {
    return self->SetItemPosition(item, *pos);
}
bool wxListCtrl_SetItemState(wxListCtrl * self, long item, long state, long state_mask) {
    return self->SetItemState(item, state, state_mask);
}
void wxListCtrl_SetItemText(wxListCtrl * self, long item, const wxString * text) {
    return self->SetItemText(item, *text);
}
void wxListCtrl_SetItemTextColour(wxListCtrl * self, long item, const wxColour * col) {
    return self->SetItemTextColour(item, *col);
}
void wxListCtrl_SetSingleStyle(wxListCtrl * self, long style, bool add) {
    return self->SetSingleStyle(style, add);
}
void wxListCtrl_SetTextColour(wxListCtrl * self, const wxColour * col) {
    return self->SetTextColour(*col);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxListCtrl_HasCheckBoxes(const wxListCtrl * self) {
    return self->HasCheckBoxes();
}
bool wxListCtrl_EnableCheckBoxes(wxListCtrl * self, bool enable) {
    return self->EnableCheckBoxes(enable);
}
bool wxListCtrl_IsItemChecked(const wxListCtrl * self, long item) {
    return self->IsItemChecked(item);
}
void wxListCtrl_CheckItem(wxListCtrl * self, long item, bool check) {
    return self->CheckItem(item, check);
}
void wxListCtrl_ExtendRulesAndAlternateColour(wxListCtrl * self, bool extend) {
    return self->ExtendRulesAndAlternateColour(extend);
}
void wxListCtrl_ShowSortIndicator(wxListCtrl * self, int col, bool ascending) {
    return self->ShowSortIndicator(col, ascending);
}
void wxListCtrl_RemoveSortIndicator(wxListCtrl * self) {
    return self->RemoveSortIndicator();
}
int wxListCtrl_GetSortIndicator(const wxListCtrl * self) {
    return self->GetSortIndicator();
}
bool wxListCtrl_GetUpdatedAscendingSortIndicator(const wxListCtrl * self, int col) {
    return self->GetUpdatedAscendingSortIndicator(col);
}
bool wxListCtrl_IsAscendingSortIndicator(const wxListCtrl * self) {
    return self->IsAscendingSortIndicator();
}
#endif

// CLASS: wxListEvent
wxClassInfo *wxListEvent_CLASSINFO() {
    return wxCLASSINFO(wxListEvent);
}
long wxListEvent_GetCacheFrom(const wxListEvent * self) {
    return self->GetCacheFrom();
}
long wxListEvent_GetCacheTo(const wxListEvent * self) {
    return self->GetCacheTo();
}
int wxListEvent_GetColumn(const wxListEvent * self) {
    return self->GetColumn();
}
int wxListEvent_GetImage(const wxListEvent * self) {
    return self->GetImage();
}
long wxListEvent_GetIndex(const wxListEvent * self) {
    return self->GetIndex();
}
wxListItem *wxListEvent_GetItem(const wxListEvent * self) {
    return new wxListItem(self->GetItem());
}
int wxListEvent_GetKeyCode(const wxListEvent * self) {
    return self->GetKeyCode();
}
wxString *wxListEvent_GetLabel(const wxListEvent * self) {
    return new wxString(self->GetLabel());
}
long wxListEvent_GetMask(const wxListEvent * self) {
    return self->GetMask();
}
wxPoint *wxListEvent_GetPoint(const wxListEvent * self) {
    return new wxPoint(self->GetPoint());
}
wxString *wxListEvent_GetText(const wxListEvent * self) {
    return new wxString(self->GetText());
}
bool wxListEvent_IsEditCancelled(const wxListEvent * self) {
    return self->IsEditCancelled();
}
void wxListEvent_SetKeyCode(wxListEvent * self, int code) {
    return self->SetKeyCode(code);
}
void wxListEvent_SetIndex(wxListEvent * self, long index) {
    return self->SetIndex(index);
}
void wxListEvent_SetColumn(wxListEvent * self, int col) {
    return self->SetColumn(col);
}
void wxListEvent_SetPoint(wxListEvent * self, const wxPoint * point) {
    return self->SetPoint(*point);
}
void wxListEvent_SetItem(wxListEvent * self, const wxListItem * item) {
    return self->SetItem(*item);
}
void wxListEvent_SetCacheFrom(wxListEvent * self, long cache_from) {
    return self->SetCacheFrom(cache_from);
}
void wxListEvent_SetCacheTo(wxListEvent * self, long cache_to) {
    return self->SetCacheTo(cache_to);
}

// CLASS: wxListItem
wxClassInfo *wxListItem_CLASSINFO() {
    return wxCLASSINFO(wxListItem);
}
wxListItem *wxListItem_new() {
    return new wxListItem();
}
void wxListItem_Clear(wxListItem * self) {
    return self->Clear();
}
wxColour *wxListItem_GetBackgroundColour(const wxListItem * self) {
    return new wxColour(self->GetBackgroundColour());
}
int wxListItem_GetColumn(const wxListItem * self) {
    return self->GetColumn();
}
wxFont *wxListItem_GetFont(const wxListItem * self) {
    return new wxFont(self->GetFont());
}
long wxListItem_GetId(const wxListItem * self) {
    return self->GetId();
}
int wxListItem_GetImage(const wxListItem * self) {
    return self->GetImage();
}
long wxListItem_GetMask(const wxListItem * self) {
    return self->GetMask();
}
long wxListItem_GetState(const wxListItem * self) {
    return self->GetState();
}
wxString *wxListItem_GetText(const wxListItem * self) {
    return new wxString(self->GetText());
}
wxColour *wxListItem_GetTextColour(const wxListItem * self) {
    return new wxColour(self->GetTextColour());
}
int wxListItem_GetWidth(const wxListItem * self) {
    return self->GetWidth();
}
void wxListItem_SetBackgroundColour(wxListItem * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxListItem_SetColumn(wxListItem * self, int col) {
    return self->SetColumn(col);
}
void wxListItem_SetData(wxListItem * self, long data) {
    return self->SetData(data);
}
void wxListItem_SetData1(wxListItem * self, void * data) {
    return self->SetData(data);
}
void wxListItem_SetFont(wxListItem * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxListItem_SetId(wxListItem * self, long id) {
    return self->SetId(id);
}
void wxListItem_SetImage(wxListItem * self, int image) {
    return self->SetImage(image);
}
void wxListItem_SetMask(wxListItem * self, long mask) {
    return self->SetMask(mask);
}
void wxListItem_SetState(wxListItem * self, long state) {
    return self->SetState(state);
}
void wxListItem_SetStateMask(wxListItem * self, long state_mask) {
    return self->SetStateMask(state_mask);
}
void wxListItem_SetText(wxListItem * self, const wxString * text) {
    return self->SetText(*text);
}
void wxListItem_SetTextColour(wxListItem * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
void wxListItem_SetWidth(wxListItem * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxListView
wxClassInfo *wxListView_CLASSINFO() {
    return wxCLASSINFO(wxListView);
}
wxListView *wxListView_new() {
    return new wxListView();
}
wxListView *wxListView_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxListView(parent, winid, *pos, *size, style, *validator, *name);
}
void wxListView_ClearColumnImage(wxListView * self, int col) {
    return self->ClearColumnImage(col);
}
void wxListView_Focus(wxListView * self, long index) {
    return self->Focus(index);
}
long wxListView_GetFirstSelected(const wxListView * self) {
    return self->GetFirstSelected();
}
long wxListView_GetFocusedItem(const wxListView * self) {
    return self->GetFocusedItem();
}
long wxListView_GetNextSelected(const wxListView * self, long item) {
    return self->GetNextSelected(item);
}
bool wxListView_IsSelected(const wxListView * self, long index) {
    return self->IsSelected(index);
}
void wxListView_Select(wxListView * self, long n, bool on) {
    return self->Select(n, on);
}
void wxListView_SetColumnImage(wxListView * self, int col, int image) {
    return self->SetColumnImage(col, image);
}

// CLASS: wxListbook
wxClassInfo *wxListbook_CLASSINFO() {
    return wxCLASSINFO(wxListbook);
}
wxListbook *wxListbook_new() {
    return new wxListbook();
}
wxListbook *wxListbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxListbook(parent, id, *pos, *size, style, *name);
}
bool wxListbook_Create(wxListbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
wxListView * wxListbook_GetListView(const wxListbook * self) {
    return self->GetListView();
}

} // extern "C"

