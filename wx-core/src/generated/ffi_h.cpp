#include "generated.h"

extern "C" {

// CLASS: wxHeaderColumn
void wxHeaderColumn_delete(wxHeaderColumn *self) {
    delete self;
}
wxString *wxHeaderColumn_GetTitle(const wxHeaderColumn * self) {
    return new wxString(self->GetTitle());
}
wxBitmap *wxHeaderColumn_GetBitmap(const wxHeaderColumn * self) {
    return new wxBitmap(self->GetBitmap());
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxHeaderColumn_GetBitmapBundle(const wxHeaderColumn * self) {
    return new wxBitmapBundle(self->GetBitmapBundle());
}
#endif
int wxHeaderColumn_GetWidth(const wxHeaderColumn * self) {
    return self->GetWidth();
}
int wxHeaderColumn_GetMinWidth(const wxHeaderColumn * self) {
    return self->GetMinWidth();
}
wxAlignment wxHeaderColumn_GetAlignment(const wxHeaderColumn * self) {
    return self->GetAlignment();
}
int wxHeaderColumn_GetFlags(const wxHeaderColumn * self) {
    return self->GetFlags();
}
bool wxHeaderColumn_HasFlag(const wxHeaderColumn * self, int flag) {
    return self->HasFlag(flag);
}
bool wxHeaderColumn_IsResizeable(const wxHeaderColumn * self) {
    return self->IsResizeable();
}
bool wxHeaderColumn_IsSortable(const wxHeaderColumn * self) {
    return self->IsSortable();
}
bool wxHeaderColumn_IsReorderable(const wxHeaderColumn * self) {
    return self->IsReorderable();
}
bool wxHeaderColumn_IsHidden(const wxHeaderColumn * self) {
    return self->IsHidden();
}
bool wxHeaderColumn_IsShown(const wxHeaderColumn * self) {
    return self->IsShown();
}
bool wxHeaderColumn_IsSortKey(const wxHeaderColumn * self) {
    return self->IsSortKey();
}
bool wxHeaderColumn_IsSortOrderAscending(const wxHeaderColumn * self) {
    return self->IsSortOrderAscending();
}

// CLASS: wxHeaderColumnSimple
void wxHeaderColumnSimple_delete(wxHeaderColumnSimple *self) {
    delete self;
}
wxHeaderColumnSimple *wxHeaderColumnSimple_new(const wxString * title, int width, wxAlignment align, int flags) {
    return new wxHeaderColumnSimple(*title, width, align, flags);
}
wxHeaderColumnSimple *wxHeaderColumnSimple_new1(const wxBitmapBundle * bitmap, int width, wxAlignment align, int flags) {
    return new wxHeaderColumnSimple(*bitmap, width, align, flags);
}

// CLASS: wxHeaderCtrl
wxClassInfo *wxHeaderCtrl_CLASSINFO() {
    return wxCLASSINFO(wxHeaderCtrl);
}
bool wxHeaderCtrl_Create(wxHeaderCtrl * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, winid, *pos, *size, style, *name);
}
void wxHeaderCtrl_SetColumnCount(wxHeaderCtrl * self, unsigned int count) {
    return self->SetColumnCount(count);
}
unsigned int wxHeaderCtrl_GetColumnCount(const wxHeaderCtrl * self) {
    return self->GetColumnCount();
}
bool wxHeaderCtrl_IsEmpty(const wxHeaderCtrl * self) {
    return self->IsEmpty();
}
void wxHeaderCtrl_UpdateColumn(wxHeaderCtrl * self, unsigned int idx) {
    return self->UpdateColumn(idx);
}
void wxHeaderCtrl_SetColumnsOrder(wxHeaderCtrl * self, const wxArrayInt * order) {
    return self->SetColumnsOrder(*order);
}
wxArrayInt *wxHeaderCtrl_GetColumnsOrder(const wxHeaderCtrl * self) {
    return new wxArrayInt(self->GetColumnsOrder());
}
unsigned int wxHeaderCtrl_GetColumnAt(const wxHeaderCtrl * self, unsigned int pos) {
    return self->GetColumnAt(pos);
}
unsigned int wxHeaderCtrl_GetColumnPos(const wxHeaderCtrl * self, unsigned int idx) {
    return self->GetColumnPos(idx);
}
void wxHeaderCtrl_ResetColumnsOrder(wxHeaderCtrl * self) {
    return self->ResetColumnsOrder();
}
bool wxHeaderCtrl_ShowColumnsMenu(wxHeaderCtrl * self, const wxPoint * pt, const wxString * title) {
    return self->ShowColumnsMenu(*pt, *title);
}
void wxHeaderCtrl_AddColumnsItems(wxHeaderCtrl * self, wxMenu * menu, int id_columns_base) {
    return self->AddColumnsItems(*menu, id_columns_base);
}
bool wxHeaderCtrl_ShowCustomizeDialog(wxHeaderCtrl * self) {
    return self->ShowCustomizeDialog();
}
int wxHeaderCtrl_GetColumnTitleWidth(wxHeaderCtrl * self, const wxHeaderColumn * col) {
    return self->GetColumnTitleWidth(*col);
}
#if wxCHECK_VERSION(3, 1, 0)
int wxHeaderCtrl_GetColumnTitleWidth1(wxHeaderCtrl * self, unsigned int idx) {
    return self->GetColumnTitleWidth(idx);
}
#endif
void wxHeaderCtrl_MoveColumnInOrderArray(wxArrayInt * order, unsigned int idx, unsigned int pos) {
    return wxHeaderCtrl::MoveColumnInOrderArray(*order, idx, pos);
}

// CLASS: wxHeaderCtrlSimple
wxClassInfo *wxHeaderCtrlSimple_CLASSINFO() {
    return wxCLASSINFO(wxHeaderCtrlSimple);
}
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new() {
    return new wxHeaderCtrlSimple();
}
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxHeaderCtrlSimple(parent, winid, *pos, *size, style, *name);
}
void wxHeaderCtrlSimple_InsertColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col, unsigned int idx) {
    return self->InsertColumn(*col, idx);
}
void wxHeaderCtrlSimple_AppendColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col) {
    return self->AppendColumn(*col);
}
void wxHeaderCtrlSimple_DeleteColumn(wxHeaderCtrlSimple * self, unsigned int idx) {
    return self->DeleteColumn(idx);
}
void wxHeaderCtrlSimple_ShowColumn(wxHeaderCtrlSimple * self, unsigned int idx, bool show) {
    return self->ShowColumn(idx, show);
}
void wxHeaderCtrlSimple_HideColumn(wxHeaderCtrlSimple * self, unsigned int idx) {
    return self->HideColumn(idx);
}
void wxHeaderCtrlSimple_ShowSortIndicator(wxHeaderCtrlSimple * self, unsigned int idx, bool sort_order) {
    return self->ShowSortIndicator(idx, sort_order);
}
void wxHeaderCtrlSimple_RemoveSortIndicator(wxHeaderCtrlSimple * self) {
    return self->RemoveSortIndicator();
}

// CLASS: wxHyperlinkCtrl
wxClassInfo *wxHyperlinkCtrl_CLASSINFO() {
    return wxCLASSINFO(wxHyperlinkCtrl);
}
wxHyperlinkCtrl *wxHyperlinkCtrl_new() {
    return new wxHyperlinkCtrl();
}
wxHyperlinkCtrl *wxHyperlinkCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxHyperlinkCtrl(parent, id, *label, *url, *pos, *size, style, *name);
}
bool wxHyperlinkCtrl_Create(wxHyperlinkCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *label, *url, *pos, *size, style, *name);
}
wxColour *wxHyperlinkCtrl_GetHoverColour(const wxHyperlinkCtrl * self) {
    return new wxColour(self->GetHoverColour());
}
wxColour *wxHyperlinkCtrl_GetNormalColour(const wxHyperlinkCtrl * self) {
    return new wxColour(self->GetNormalColour());
}
wxString *wxHyperlinkCtrl_GetURL(const wxHyperlinkCtrl * self) {
    return new wxString(self->GetURL());
}
bool wxHyperlinkCtrl_GetVisited(const wxHyperlinkCtrl * self) {
    return self->GetVisited();
}
wxColour *wxHyperlinkCtrl_GetVisitedColour(const wxHyperlinkCtrl * self) {
    return new wxColour(self->GetVisitedColour());
}
void wxHyperlinkCtrl_SetHoverColour(wxHyperlinkCtrl * self, const wxColour * colour) {
    return self->SetHoverColour(*colour);
}
void wxHyperlinkCtrl_SetNormalColour(wxHyperlinkCtrl * self, const wxColour * colour) {
    return self->SetNormalColour(*colour);
}
void wxHyperlinkCtrl_SetURL(wxHyperlinkCtrl * self, const wxString * url) {
    return self->SetURL(*url);
}
void wxHyperlinkCtrl_SetVisited(wxHyperlinkCtrl * self, bool visited) {
    return self->SetVisited(visited);
}
void wxHyperlinkCtrl_SetVisitedColour(wxHyperlinkCtrl * self, const wxColour * colour) {
    return self->SetVisitedColour(*colour);
}

} // extern "C"

