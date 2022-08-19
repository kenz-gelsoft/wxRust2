#include "generated.h"

extern "C" {

// CLASS: wxSearchCtrl
wxClassInfo *wxSearchCtrl_CLASSINFO() {
    return wxCLASSINFO(wxSearchCtrl);
}
wxSearchCtrl *wxSearchCtrl_new() {
    return new wxSearchCtrl();
}
wxSearchCtrl *wxSearchCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxSearchCtrl(parent, id, *value, *pos, *size, style, *validator, *name);
}
bool wxSearchCtrl_Create(wxSearchCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, *validator, *name);
}
wxMenu * wxSearchCtrl_GetMenu(wxSearchCtrl * self) {
    return self->GetMenu();
}
bool wxSearchCtrl_IsSearchButtonVisible(const wxSearchCtrl * self) {
    return self->IsSearchButtonVisible();
}
bool wxSearchCtrl_IsCancelButtonVisible(const wxSearchCtrl * self) {
    return self->IsCancelButtonVisible();
}
void wxSearchCtrl_SetMenu(wxSearchCtrl * self, wxMenu * menu) {
    return self->SetMenu(menu);
}
void wxSearchCtrl_ShowCancelButton(wxSearchCtrl * self, bool show) {
    return self->ShowCancelButton(show);
}
void wxSearchCtrl_ShowSearchButton(wxSearchCtrl * self, bool show) {
    return self->ShowSearchButton(show);
}
void wxSearchCtrl_SetDescriptiveText(wxSearchCtrl * self, const wxString * text) {
    return self->SetDescriptiveText(*text);
}
wxString *wxSearchCtrl_GetDescriptiveText(const wxSearchCtrl * self) {
    return new wxString(self->GetDescriptiveText());
}
// Mix-in(s) to wxSearchCtrl
wxTextEntryBase *wxSearchCtrl_AsTextEntry(wxSearchCtrl* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}

// CLASS: wxSettableHeaderColumn
void wxSettableHeaderColumn_delete(wxSettableHeaderColumn *self) {
    delete self;
}
void wxSettableHeaderColumn_SetTitle(wxSettableHeaderColumn * self, const wxString * title) {
    return self->SetTitle(*title);
}
void wxSettableHeaderColumn_SetBitmap(wxSettableHeaderColumn * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmap(*bitmap);
}
void wxSettableHeaderColumn_SetWidth(wxSettableHeaderColumn * self, int width) {
    return self->SetWidth(width);
}
void wxSettableHeaderColumn_SetMinWidth(wxSettableHeaderColumn * self, int min_width) {
    return self->SetMinWidth(min_width);
}
void wxSettableHeaderColumn_SetAlignment(wxSettableHeaderColumn * self, wxAlignment align) {
    return self->SetAlignment(align);
}
void wxSettableHeaderColumn_SetFlags(wxSettableHeaderColumn * self, int flags) {
    return self->SetFlags(flags);
}
void wxSettableHeaderColumn_ChangeFlag(wxSettableHeaderColumn * self, int flag, bool set) {
    return self->ChangeFlag(flag, set);
}
void wxSettableHeaderColumn_SetFlag(wxSettableHeaderColumn * self, int flag) {
    return self->SetFlag(flag);
}
void wxSettableHeaderColumn_ClearFlag(wxSettableHeaderColumn * self, int flag) {
    return self->ClearFlag(flag);
}
void wxSettableHeaderColumn_ToggleFlag(wxSettableHeaderColumn * self, int flag) {
    return self->ToggleFlag(flag);
}
void wxSettableHeaderColumn_SetResizeable(wxSettableHeaderColumn * self, bool resizable) {
    return self->SetResizeable(resizable);
}
void wxSettableHeaderColumn_SetSortable(wxSettableHeaderColumn * self, bool sortable) {
    return self->SetSortable(sortable);
}
void wxSettableHeaderColumn_SetReorderable(wxSettableHeaderColumn * self, bool reorderable) {
    return self->SetReorderable(reorderable);
}
void wxSettableHeaderColumn_SetHidden(wxSettableHeaderColumn * self, bool hidden) {
    return self->SetHidden(hidden);
}
void wxSettableHeaderColumn_UnsetAsSortKey(wxSettableHeaderColumn * self) {
    return self->UnsetAsSortKey();
}
void wxSettableHeaderColumn_SetSortOrder(wxSettableHeaderColumn * self, bool ascending) {
    return self->SetSortOrder(ascending);
}
void wxSettableHeaderColumn_ToggleSortOrder(wxSettableHeaderColumn * self) {
    return self->ToggleSortOrder();
}

// CLASS: wxSize
void wxSize_delete(wxSize *self) {
    delete self;
}
wxSize *wxSize_new() {
    return new wxSize();
}
wxSize *wxSize_new1(int width, int height) {
    return new wxSize(width, height);
}
void wxSize_DecBy(wxSize * self, const wxPoint * pt) {
    return self->DecBy(*pt);
}
void wxSize_DecBy1(wxSize * self, const wxSize * size) {
    return self->DecBy(*size);
}
void wxSize_DecBy2(wxSize * self, int dx, int dy) {
    return self->DecBy(dx, dy);
}
void wxSize_DecBy3(wxSize * self, int d) {
    return self->DecBy(d);
}
void wxSize_DecTo(wxSize * self, const wxSize * size) {
    return self->DecTo(*size);
}
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size) {
    return self->DecToIfSpecified(*size);
}
int wxSize_GetHeight(const wxSize * self) {
    return self->GetHeight();
}
int wxSize_GetWidth(const wxSize * self) {
    return self->GetWidth();
}
void wxSize_IncBy(wxSize * self, const wxPoint * pt) {
    return self->IncBy(*pt);
}
void wxSize_IncBy1(wxSize * self, const wxSize * size) {
    return self->IncBy(*size);
}
void wxSize_IncBy2(wxSize * self, int dx, int dy) {
    return self->IncBy(dx, dy);
}
void wxSize_IncBy3(wxSize * self, int d) {
    return self->IncBy(d);
}
void wxSize_IncTo(wxSize * self, const wxSize * size) {
    return self->IncTo(*size);
}
bool wxSize_IsFullySpecified(const wxSize * self) {
    return self->IsFullySpecified();
}
void wxSize_Set(wxSize * self, int width, int height) {
    return self->Set(width, height);
}
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default) {
    return self->SetDefaults(*size_default);
}
void wxSize_SetHeight(wxSize * self, int height) {
    return self->SetHeight(height);
}
void wxSize_SetWidth(wxSize * self, int width) {
    return self->SetWidth(width);
}

// CLASS: wxSizer
wxClassInfo *wxSizer_CLASSINFO() {
    return wxCLASSINFO(wxSizer);
}
wxSizerItem * wxSizer_Add(wxSizer * self, wxWindow * window, const wxSizerFlags * flags) {
    return self->Add(window, *flags);
}
wxSizerItem * wxSizer_Add1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Add(sizer, *flags);
}
wxSizerItem * wxSizer_Add3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Add(width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Add5(wxSizer * self, int width, int height, const wxSizerFlags * flags) {
    return self->Add(width, height, *flags);
}
wxSizerItem * wxSizer_Add6(wxSizer * self, wxSizerItem * item) {
    return self->Add(item);
}
wxSizerItem * wxSizer_AddSpacer(wxSizer * self, int size) {
    return self->AddSpacer(size);
}
wxSizerItem * wxSizer_AddStretchSpacer(wxSizer * self, int prop) {
    return self->AddStretchSpacer(prop);
}
wxSize *wxSizer_CalcMin(wxSizer * self) {
    return new wxSize(self->CalcMin());
}
void wxSizer_Clear(wxSizer * self, bool delete_windows) {
    return self->Clear(delete_windows);
}
wxSize *wxSizer_ComputeFittingClientSize(wxSizer * self, wxWindow * window) {
    return new wxSize(self->ComputeFittingClientSize(window));
}
wxSize *wxSizer_ComputeFittingWindowSize(wxSizer * self, wxWindow * window) {
    return new wxSize(self->ComputeFittingWindowSize(window));
}
bool wxSizer_Detach(wxSizer * self, wxWindow * window) {
    return self->Detach(window);
}
bool wxSizer_Detach1(wxSizer * self, wxSizer * sizer) {
    return self->Detach(sizer);
}
bool wxSizer_Detach2(wxSizer * self, int index) {
    return self->Detach(index);
}
wxSize *wxSizer_Fit(wxSizer * self, wxWindow * window) {
    return new wxSize(self->Fit(window));
}
void wxSizer_FitInside(wxSizer * self, wxWindow * window) {
    return self->FitInside(window);
}
bool wxSizer_InformFirstDirection(wxSizer * self, int direction, int size, int available_other_dir) {
    return self->InformFirstDirection(direction, size, available_other_dir);
}
wxSizerItemList * wxSizer_GetChildren(wxSizer * self) {
    return &(self->GetChildren());
}
wxWindow * wxSizer_GetContainingWindow(const wxSizer * self) {
    return self->GetContainingWindow();
}
void wxSizer_SetContainingWindow(wxSizer * self, wxWindow * window) {
    return self->SetContainingWindow(window);
}
size_t wxSizer_GetItemCount(const wxSizer * self) {
    return self->GetItemCount();
}
wxSizerItem * wxSizer_GetItem(wxSizer * self, wxWindow * window, bool recursive) {
    return self->GetItem(window, recursive);
}
wxSizerItem * wxSizer_GetItem1(wxSizer * self, wxSizer * sizer, bool recursive) {
    return self->GetItem(sizer, recursive);
}
wxSizerItem * wxSizer_GetItem2(wxSizer * self, size_t index) {
    return self->GetItem(index);
}
wxSizerItem * wxSizer_GetItemById(wxSizer * self, int id, bool recursive) {
    return self->GetItemById(id, recursive);
}
wxSize *wxSizer_GetMinSize(wxSizer * self) {
    return new wxSize(self->GetMinSize());
}
wxPoint *wxSizer_GetPosition(const wxSizer * self) {
    return new wxPoint(self->GetPosition());
}
wxSize *wxSizer_GetSize(const wxSizer * self) {
    return new wxSize(self->GetSize());
}
bool wxSizer_Hide(wxSizer * self, wxWindow * window, bool recursive) {
    return self->Hide(window, recursive);
}
bool wxSizer_Hide1(wxSizer * self, wxSizer * sizer, bool recursive) {
    return self->Hide(sizer, recursive);
}
bool wxSizer_Hide2(wxSizer * self, size_t index) {
    return self->Hide(index);
}
wxSizerItem * wxSizer_Insert(wxSizer * self, size_t index, wxWindow * window, const wxSizerFlags * flags) {
    return self->Insert(index, window, *flags);
}
wxSizerItem * wxSizer_Insert1(wxSizer * self, size_t index, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Insert(index, window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Insert2(wxSizer * self, size_t index, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Insert(index, sizer, *flags);
}
wxSizerItem * wxSizer_Insert3(wxSizer * self, size_t index, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Insert(index, sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Insert4(wxSizer * self, size_t index, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Insert(index, width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Insert5(wxSizer * self, size_t index, int width, int height, const wxSizerFlags * flags) {
    return self->Insert(index, width, height, *flags);
}
wxSizerItem * wxSizer_Insert6(wxSizer * self, size_t index, wxSizerItem * item) {
    return self->Insert(index, item);
}
wxSizerItem * wxSizer_InsertSpacer(wxSizer * self, size_t index, int size) {
    return self->InsertSpacer(index, size);
}
wxSizerItem * wxSizer_InsertStretchSpacer(wxSizer * self, size_t index, int prop) {
    return self->InsertStretchSpacer(index, prop);
}
bool wxSizer_IsEmpty(const wxSizer * self) {
    return self->IsEmpty();
}
bool wxSizer_IsShown(const wxSizer * self, wxWindow * window) {
    return self->IsShown(window);
}
bool wxSizer_IsShown1(const wxSizer * self, wxSizer * sizer) {
    return self->IsShown(sizer);
}
bool wxSizer_IsShown2(const wxSizer * self, size_t index) {
    return self->IsShown(index);
}
void wxSizer_Layout(wxSizer * self) {
    return self->Layout();
}
wxSizerItem * wxSizer_Prepend(wxSizer * self, wxWindow * window, const wxSizerFlags * flags) {
    return self->Prepend(window, *flags);
}
wxSizerItem * wxSizer_Prepend1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(window, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags) {
    return self->Prepend(sizer, *flags);
}
wxSizerItem * wxSizer_Prepend3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(sizer, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return self->Prepend(width, height, proportion, flag, border, user_data);
}
wxSizerItem * wxSizer_Prepend5(wxSizer * self, int width, int height, const wxSizerFlags * flags) {
    return self->Prepend(width, height, *flags);
}
wxSizerItem * wxSizer_Prepend6(wxSizer * self, wxSizerItem * item) {
    return self->Prepend(item);
}
wxSizerItem * wxSizer_PrependSpacer(wxSizer * self, int size) {
    return self->PrependSpacer(size);
}
wxSizerItem * wxSizer_PrependStretchSpacer(wxSizer * self, int prop) {
    return self->PrependStretchSpacer(prop);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxSizer_RepositionChildren(wxSizer * self, const wxSize * min_size) {
    return self->RepositionChildren(*min_size);
}
#endif
bool wxSizer_Remove1(wxSizer * self, wxSizer * sizer) {
    return self->Remove(sizer);
}
bool wxSizer_Remove2(wxSizer * self, int index) {
    return self->Remove(index);
}
bool wxSizer_Replace(wxSizer * self, wxWindow * oldwin, wxWindow * newwin, bool recursive) {
    return self->Replace(oldwin, newwin, recursive);
}
bool wxSizer_Replace1(wxSizer * self, wxSizer * oldsz, wxSizer * newsz, bool recursive) {
    return self->Replace(oldsz, newsz, recursive);
}
bool wxSizer_Replace2(wxSizer * self, size_t index, wxSizerItem * newitem) {
    return self->Replace(index, newitem);
}
void wxSizer_SetDimension(wxSizer * self, int x, int y, int width, int height) {
    return self->SetDimension(x, y, width, height);
}
void wxSizer_SetDimension1(wxSizer * self, const wxPoint * pos, const wxSize * size) {
    return self->SetDimension(*pos, *size);
}
bool wxSizer_SetItemMinSize(wxSizer * self, wxWindow * window, int width, int height) {
    return self->SetItemMinSize(window, width, height);
}
bool wxSizer_SetItemMinSize1(wxSizer * self, wxWindow * window, const wxSize * size) {
    return self->SetItemMinSize(window, *size);
}
bool wxSizer_SetItemMinSize2(wxSizer * self, wxSizer * sizer, int width, int height) {
    return self->SetItemMinSize(sizer, width, height);
}
bool wxSizer_SetItemMinSize3(wxSizer * self, wxSizer * sizer, const wxSize * size) {
    return self->SetItemMinSize(sizer, *size);
}
bool wxSizer_SetItemMinSize4(wxSizer * self, size_t index, int width, int height) {
    return self->SetItemMinSize(index, width, height);
}
bool wxSizer_SetItemMinSize5(wxSizer * self, size_t index, const wxSize * size) {
    return self->SetItemMinSize(index, *size);
}
void wxSizer_SetMinSize(wxSizer * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxSizer_SetMinSize1(wxSizer * self, int width, int height) {
    return self->SetMinSize(width, height);
}
void wxSizer_SetSizeHints(wxSizer * self, wxWindow * window) {
    return self->SetSizeHints(window);
}
bool wxSizer_Show(wxSizer * self, wxWindow * window, bool show, bool recursive) {
    return self->Show(window, show, recursive);
}
bool wxSizer_Show1(wxSizer * self, wxSizer * sizer, bool show, bool recursive) {
    return self->Show(sizer, show, recursive);
}
bool wxSizer_Show2(wxSizer * self, size_t index, bool show) {
    return self->Show(index, show);
}
void wxSizer_ShowItems(wxSizer * self, bool show) {
    return self->ShowItems(show);
}

// CLASS: wxSizerFlags
void wxSizerFlags_delete(wxSizerFlags *self) {
    delete self;
}
wxSizerFlags *wxSizerFlags_new(int proportion) {
    return new wxSizerFlags(proportion);
}
wxSizerFlags * wxSizerFlags_Align(wxSizerFlags * self, int alignment) {
    return &(self->Align(alignment));
}
wxSizerFlags * wxSizerFlags_Border(wxSizerFlags * self, int direction, int borderinpixels) {
    return &(self->Border(direction, borderinpixels));
}
wxSizerFlags * wxSizerFlags_Border1(wxSizerFlags * self, int direction) {
    return &(self->Border(direction));
}
wxSizerFlags * wxSizerFlags_Bottom(wxSizerFlags * self) {
    return &(self->Bottom());
}
wxSizerFlags * wxSizerFlags_Center(wxSizerFlags * self) {
    return &(self->Center());
}
wxSizerFlags * wxSizerFlags_Centre(wxSizerFlags * self) {
    return &(self->Centre());
}
#if wxCHECK_VERSION(3, 1, 0)
wxSizerFlags * wxSizerFlags_CenterHorizontal(wxSizerFlags * self) {
    return &(self->CenterHorizontal());
}
wxSizerFlags * wxSizerFlags_CenterVertical(wxSizerFlags * self) {
    return &(self->CenterVertical());
}
wxSizerFlags * wxSizerFlags_CentreHorizontal(wxSizerFlags * self) {
    return &(self->CentreHorizontal());
}
wxSizerFlags * wxSizerFlags_CentreVertical(wxSizerFlags * self) {
    return &(self->CentreVertical());
}
#endif
wxSizerFlags * wxSizerFlags_DoubleBorder(wxSizerFlags * self, int direction) {
    return &(self->DoubleBorder(direction));
}
wxSizerFlags * wxSizerFlags_DoubleHorzBorder(wxSizerFlags * self) {
    return &(self->DoubleHorzBorder());
}
wxSizerFlags * wxSizerFlags_Expand(wxSizerFlags * self) {
    return &(self->Expand());
}
wxSizerFlags * wxSizerFlags_FixedMinSize(wxSizerFlags * self) {
    return &(self->FixedMinSize());
}
wxSizerFlags * wxSizerFlags_ReserveSpaceEvenIfHidden(wxSizerFlags * self) {
    return &(self->ReserveSpaceEvenIfHidden());
}
wxSizerFlags * wxSizerFlags_Left(wxSizerFlags * self) {
    return &(self->Left());
}
wxSizerFlags * wxSizerFlags_Proportion(wxSizerFlags * self, int proportion) {
    return &(self->Proportion(proportion));
}
wxSizerFlags * wxSizerFlags_Right(wxSizerFlags * self) {
    return &(self->Right());
}
wxSizerFlags * wxSizerFlags_Shaped(wxSizerFlags * self) {
    return &(self->Shaped());
}
wxSizerFlags * wxSizerFlags_Top(wxSizerFlags * self) {
    return &(self->Top());
}
wxSizerFlags * wxSizerFlags_TripleBorder(wxSizerFlags * self, int direction) {
    return &(self->TripleBorder(direction));
}
#if wxCHECK_VERSION(3, 1, 0)
void wxSizerFlags_DisableConsistencyChecks() {
    return wxSizerFlags::DisableConsistencyChecks();
}
#endif
int wxSizerFlags_GetDefaultBorder() {
    return wxSizerFlags::GetDefaultBorder();
}

// CLASS: wxSizerItem
wxClassInfo *wxSizerItem_CLASSINFO() {
    return wxCLASSINFO(wxSizerItem);
}
wxSizerItem *wxSizerItem_new(int width, int height, int proportion, int flag, int border, wxObject * user_data) {
    return new wxSizerItem(width, height, proportion, flag, border, user_data);
}
wxSizerItem *wxSizerItem_new1(wxWindow * window, const wxSizerFlags * flags) {
    return new wxSizerItem(window, *flags);
}
wxSizerItem *wxSizerItem_new2(wxWindow * window, int proportion, int flag, int border, wxObject * user_data) {
    return new wxSizerItem(window, proportion, flag, border, user_data);
}
wxSizerItem *wxSizerItem_new3(wxSizer * sizer, const wxSizerFlags * flags) {
    return new wxSizerItem(sizer, *flags);
}
wxSizerItem *wxSizerItem_new4(wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data) {
    return new wxSizerItem(sizer, proportion, flag, border, user_data);
}
void wxSizerItem_AssignWindow(wxSizerItem * self, wxWindow * window) {
    return self->AssignWindow(window);
}
void wxSizerItem_AssignSizer(wxSizerItem * self, wxSizer * sizer) {
    return self->AssignSizer(sizer);
}
void wxSizerItem_AssignSpacer(wxSizerItem * self, const wxSize * size) {
    return self->AssignSpacer(*size);
}
void wxSizerItem_AssignSpacer1(wxSizerItem * self, int w, int h) {
    return self->AssignSpacer(w, h);
}
wxSize *wxSizerItem_CalcMin(wxSizerItem * self) {
    return new wxSize(self->CalcMin());
}
void wxSizerItem_DeleteWindows(wxSizerItem * self) {
    return self->DeleteWindows();
}
void wxSizerItem_DetachSizer(wxSizerItem * self) {
    return self->DetachSizer();
}
int wxSizerItem_GetBorder(const wxSizerItem * self) {
    return self->GetBorder();
}
int wxSizerItem_GetFlag(const wxSizerItem * self) {
    return self->GetFlag();
}
int wxSizerItem_GetId(const wxSizerItem * self) {
    return self->GetId();
}
wxSize *wxSizerItem_GetMinSize(const wxSizerItem * self) {
    return new wxSize(self->GetMinSize());
}
void wxSizerItem_SetMinSize(wxSizerItem * self, const wxSize * size) {
    return self->SetMinSize(*size);
}
void wxSizerItem_SetMinSize1(wxSizerItem * self, int x, int y) {
    return self->SetMinSize(x, y);
}
wxPoint *wxSizerItem_GetPosition(const wxSizerItem * self) {
    return new wxPoint(self->GetPosition());
}
int wxSizerItem_GetProportion(const wxSizerItem * self) {
    return self->GetProportion();
}
wxRect *wxSizerItem_GetRect(wxSizerItem * self) {
    return new wxRect(self->GetRect());
}
wxSize *wxSizerItem_GetSize(const wxSizerItem * self) {
    return new wxSize(self->GetSize());
}
wxSizer * wxSizerItem_GetSizer(const wxSizerItem * self) {
    return self->GetSizer();
}
wxSize *wxSizerItem_GetSpacer(const wxSizerItem * self) {
    return new wxSize(self->GetSpacer());
}
wxObject * wxSizerItem_GetUserData(const wxSizerItem * self) {
    return self->GetUserData();
}
wxWindow * wxSizerItem_GetWindow(const wxSizerItem * self) {
    return self->GetWindow();
}
bool wxSizerItem_IsShown(const wxSizerItem * self) {
    return self->IsShown();
}
bool wxSizerItem_IsSizer(const wxSizerItem * self) {
    return self->IsSizer();
}
bool wxSizerItem_IsSpacer(const wxSizerItem * self) {
    return self->IsSpacer();
}
bool wxSizerItem_IsWindow(const wxSizerItem * self) {
    return self->IsWindow();
}
void wxSizerItem_SetBorder(wxSizerItem * self, int border) {
    return self->SetBorder(border);
}
void wxSizerItem_SetDimension(wxSizerItem * self, const wxPoint * pos, const wxSize * size) {
    return self->SetDimension(*pos, *size);
}
void wxSizerItem_SetFlag(wxSizerItem * self, int flag) {
    return self->SetFlag(flag);
}
void wxSizerItem_SetId(wxSizerItem * self, int id) {
    return self->SetId(id);
}
void wxSizerItem_SetInitSize(wxSizerItem * self, int x, int y) {
    return self->SetInitSize(x, y);
}
void wxSizerItem_SetProportion(wxSizerItem * self, int proportion) {
    return self->SetProportion(proportion);
}
void wxSizerItem_SetRatio(wxSizerItem * self, int width, int height) {
    return self->SetRatio(width, height);
}
void wxSizerItem_SetUserData(wxSizerItem * self, wxObject * user_data) {
    return self->SetUserData(user_data);
}
void wxSizerItem_Show(wxSizerItem * self, bool show) {
    return self->Show(show);
}

// CLASS: wxSlider
wxClassInfo *wxSlider_CLASSINFO() {
    return wxCLASSINFO(wxSlider);
}
wxSlider *wxSlider_new() {
    return new wxSlider();
}
wxSlider *wxSlider_new1(wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxSlider(parent, id, value, min_value, max_value, *pos, *size, style, *validator, *name);
}
void wxSlider_ClearSel(wxSlider * self) {
    return self->ClearSel();
}
void wxSlider_ClearTicks(wxSlider * self) {
    return self->ClearTicks();
}
bool wxSlider_Create(wxSlider * self, wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * point, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, value, min_value, max_value, *point, *size, style, *validator, *name);
}
int wxSlider_GetLineSize(const wxSlider * self) {
    return self->GetLineSize();
}
int wxSlider_GetMax(const wxSlider * self) {
    return self->GetMax();
}
int wxSlider_GetMin(const wxSlider * self) {
    return self->GetMin();
}
int wxSlider_GetPageSize(const wxSlider * self) {
    return self->GetPageSize();
}
int wxSlider_GetSelEnd(const wxSlider * self) {
    return self->GetSelEnd();
}
int wxSlider_GetSelStart(const wxSlider * self) {
    return self->GetSelStart();
}
int wxSlider_GetThumbLength(const wxSlider * self) {
    return self->GetThumbLength();
}
int wxSlider_GetTickFreq(const wxSlider * self) {
    return self->GetTickFreq();
}
int wxSlider_GetValue(const wxSlider * self) {
    return self->GetValue();
}
void wxSlider_SetLineSize(wxSlider * self, int line_size) {
    return self->SetLineSize(line_size);
}
void wxSlider_SetMin(wxSlider * self, int min_value) {
    return self->SetMin(min_value);
}
void wxSlider_SetMax(wxSlider * self, int max_value) {
    return self->SetMax(max_value);
}
void wxSlider_SetPageSize(wxSlider * self, int page_size) {
    return self->SetPageSize(page_size);
}
void wxSlider_SetRange(wxSlider * self, int min_value, int max_value) {
    return self->SetRange(min_value, max_value);
}
void wxSlider_SetSelection(wxSlider * self, int start_pos, int end_pos) {
    return self->SetSelection(start_pos, end_pos);
}
void wxSlider_SetThumbLength(wxSlider * self, int len) {
    return self->SetThumbLength(len);
}
void wxSlider_SetTick(wxSlider * self, int tick_pos) {
    return self->SetTick(tick_pos);
}
void wxSlider_SetTickFreq(wxSlider * self, int freq) {
    return self->SetTickFreq(freq);
}
void wxSlider_SetValue(wxSlider * self, int value) {
    return self->SetValue(value);
}

// CLASS: wxSpinButton
wxClassInfo *wxSpinButton_CLASSINFO() {
    return wxCLASSINFO(wxSpinButton);
}
wxSpinButton *wxSpinButton_new() {
    return new wxSpinButton();
}
wxSpinButton *wxSpinButton_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxSpinButton(parent, id, *pos, *size, style, *name);
}
bool wxSpinButton_Create(wxSpinButton * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *name);
}
#if wxCHECK_VERSION(3, 1, 0)
int wxSpinButton_GetIncrement(const wxSpinButton * self) {
    return self->GetIncrement();
}
#endif
int wxSpinButton_GetMax(const wxSpinButton * self) {
    return self->GetMax();
}
int wxSpinButton_GetMin(const wxSpinButton * self) {
    return self->GetMin();
}
int wxSpinButton_GetValue(const wxSpinButton * self) {
    return self->GetValue();
}
#if wxCHECK_VERSION(3, 1, 0)
void wxSpinButton_SetIncrement(wxSpinButton * self, int value) {
    return self->SetIncrement(value);
}
#endif
void wxSpinButton_SetRange(wxSpinButton * self, int min, int max) {
    return self->SetRange(min, max);
}
void wxSpinButton_SetValue(wxSpinButton * self, int value) {
    return self->SetValue(value);
}

// CLASS: wxSpinCtrl
wxClassInfo *wxSpinCtrl_CLASSINFO() {
    return wxCLASSINFO(wxSpinCtrl);
}
wxSpinCtrl *wxSpinCtrl_new() {
    return new wxSpinCtrl();
}
wxSpinCtrl *wxSpinCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name) {
    return new wxSpinCtrl(parent, id, *value, *pos, *size, style, min, max, initial, *name);
}
bool wxSpinCtrl_Create(wxSpinCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, min, max, initial, *name);
}
int wxSpinCtrl_GetBase(const wxSpinCtrl * self) {
    return self->GetBase();
}
int wxSpinCtrl_GetMax(const wxSpinCtrl * self) {
    return self->GetMax();
}
int wxSpinCtrl_GetMin(const wxSpinCtrl * self) {
    return self->GetMin();
}
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrl_GetTextValue(const wxSpinCtrl * self) {
    return new wxString(self->GetTextValue());
}
#endif
int wxSpinCtrl_GetValue(const wxSpinCtrl * self) {
    return self->GetValue();
}
int wxSpinCtrl_GetIncrement(const wxSpinCtrl * self) {
    return self->GetIncrement();
}
bool wxSpinCtrl_SetBase(wxSpinCtrl * self, int base) {
    return self->SetBase(base);
}
void wxSpinCtrl_SetRange(wxSpinCtrl * self, int min_val, int max_val) {
    return self->SetRange(min_val, max_val);
}
void wxSpinCtrl_SetSelection(wxSpinCtrl * self, long from, long to) {
    return self->SetSelection(from, to);
}
void wxSpinCtrl_SetValue(wxSpinCtrl * self, const wxString * text) {
    return self->SetValue(*text);
}
void wxSpinCtrl_SetValue1(wxSpinCtrl * self, int value) {
    return self->SetValue(value);
}
void wxSpinCtrl_SetIncrement(wxSpinCtrl * self, int value) {
    return self->SetIncrement(value);
}

// CLASS: wxSpinCtrlDouble
wxClassInfo *wxSpinCtrlDouble_CLASSINFO() {
    return wxCLASSINFO(wxSpinCtrlDouble);
}
wxSpinCtrlDouble *wxSpinCtrlDouble_new() {
    return new wxSpinCtrlDouble();
}
wxSpinCtrlDouble *wxSpinCtrlDouble_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name) {
    return new wxSpinCtrlDouble(parent, id, *value, *pos, *size, style, min, max, initial, inc, *name);
}
bool wxSpinCtrlDouble_Create(wxSpinCtrlDouble * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, style, min, max, initial, inc, *name);
}
unsigned int wxSpinCtrlDouble_GetDigits(const wxSpinCtrlDouble * self) {
    return self->GetDigits();
}
double wxSpinCtrlDouble_GetIncrement(const wxSpinCtrlDouble * self) {
    return self->GetIncrement();
}
double wxSpinCtrlDouble_GetMax(const wxSpinCtrlDouble * self) {
    return self->GetMax();
}
double wxSpinCtrlDouble_GetMin(const wxSpinCtrlDouble * self) {
    return self->GetMin();
}
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrlDouble_GetTextValue(const wxSpinCtrlDouble * self) {
    return new wxString(self->GetTextValue());
}
#endif
double wxSpinCtrlDouble_GetValue(const wxSpinCtrlDouble * self) {
    return self->GetValue();
}
void wxSpinCtrlDouble_SetDigits(wxSpinCtrlDouble * self, unsigned int digits) {
    return self->SetDigits(digits);
}
void wxSpinCtrlDouble_SetIncrement(wxSpinCtrlDouble * self, double inc) {
    return self->SetIncrement(inc);
}
void wxSpinCtrlDouble_SetRange(wxSpinCtrlDouble * self, double min_val, double max_val) {
    return self->SetRange(min_val, max_val);
}
void wxSpinCtrlDouble_SetValue(wxSpinCtrlDouble * self, const wxString * text) {
    return self->SetValue(*text);
}
void wxSpinCtrlDouble_SetValue1(wxSpinCtrlDouble * self, double value) {
    return self->SetValue(value);
}

// CLASS: wxStaticBitmap
wxClassInfo *wxStaticBitmap_CLASSINFO() {
    return wxCLASSINFO(wxStaticBitmap);
}
wxStaticBitmap *wxStaticBitmap_new() {
    return new wxStaticBitmap();
}
wxStaticBitmap *wxStaticBitmap_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticBitmap(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticBitmap_Create(wxStaticBitmap * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
wxBitmap *wxStaticBitmap_GetBitmap(const wxStaticBitmap * self) {
    return new wxBitmap(self->GetBitmap());
}
wxIcon *wxStaticBitmap_GetIcon(const wxStaticBitmap * self) {
    return new wxIcon(self->GetIcon());
}
void wxStaticBitmap_SetBitmap(wxStaticBitmap * self, const wxBitmapBundle * label) {
    return self->SetBitmap(*label);
}
void wxStaticBitmap_SetIcon(wxStaticBitmap * self, const wxIcon * label) {
    return self->SetIcon(*label);
}

// CLASS: wxStaticBox
wxClassInfo *wxStaticBox_CLASSINFO() {
    return wxCLASSINFO(wxStaticBox);
}
wxStaticBox *wxStaticBox_new() {
    return new wxStaticBox();
}
wxStaticBox *wxStaticBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticBox(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticBox_Create(wxStaticBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}

// CLASS: wxStaticBoxSizer
wxClassInfo *wxStaticBoxSizer_CLASSINFO() {
    return wxCLASSINFO(wxStaticBoxSizer);
}
wxStaticBoxSizer *wxStaticBoxSizer_new(wxStaticBox * box_, int orient) {
    return new wxStaticBoxSizer(box_, orient);
}
wxStaticBoxSizer *wxStaticBoxSizer_new1(int orient, wxWindow * parent, const wxString * label) {
    return new wxStaticBoxSizer(orient, parent, *label);
}
wxStaticBox * wxStaticBoxSizer_GetStaticBox(const wxStaticBoxSizer * self) {
    return self->GetStaticBox();
}

// CLASS: wxStaticText
wxClassInfo *wxStaticText_CLASSINFO() {
    return wxCLASSINFO(wxStaticText);
}
wxStaticText *wxStaticText_new() {
    return new wxStaticText();
}
wxStaticText *wxStaticText_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxStaticText(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticText_Create(wxStaticText * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *name);
}
bool wxStaticText_IsEllipsized(const wxStaticText * self) {
    return self->IsEllipsized();
}
void wxStaticText_Wrap(wxStaticText * self, int width) {
    return self->Wrap(width);
}

} // extern "C"

