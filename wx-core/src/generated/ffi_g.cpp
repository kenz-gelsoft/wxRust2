#include "generated.h"

extern "C" {

// CLASS: wxGDIObject
wxClassInfo *wxGDIObject_CLASSINFO() {
    return wxCLASSINFO(wxGDIObject);
}

// CLASS: wxGauge
wxClassInfo *wxGauge_CLASSINFO() {
    return wxCLASSINFO(wxGauge);
}
wxGauge *wxGauge_new() {
    return new wxGauge();
}
wxGauge *wxGauge_new1(wxWindow * parent, wxWindowID id, int range, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxGauge(parent, id, range, *pos, *size, style, *validator, *name);
}
bool wxGauge_Create(wxGauge * self, wxWindow * parent, wxWindowID id, int range, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, range, *pos, *size, style, *validator, *name);
}
int wxGauge_GetRange(const wxGauge * self) {
    return self->GetRange();
}
int wxGauge_GetValue(const wxGauge * self) {
    return self->GetValue();
}
bool wxGauge_IsVertical(const wxGauge * self) {
    return self->IsVertical();
}
void wxGauge_Pulse(wxGauge * self) {
    return self->Pulse();
}
void wxGauge_SetRange(wxGauge * self, int range) {
    return self->SetRange(range);
}
void wxGauge_SetValue(wxGauge * self, int pos) {
    return self->SetValue(pos);
}

// CLASS: wxGenericDirCtrl
wxClassInfo *wxGenericDirCtrl_CLASSINFO() {
    return wxCLASSINFO(wxGenericDirCtrl);
}
wxGenericDirCtrl *wxGenericDirCtrl_new() {
    return new wxGenericDirCtrl();
}
wxGenericDirCtrl *wxGenericDirCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * dir, const wxPoint * pos, const wxSize * size, long style, const wxString * filter, int default_filter, const wxString * name) {
    return new wxGenericDirCtrl(parent, id, *dir, *pos, *size, style, *filter, default_filter, *name);
}
bool wxGenericDirCtrl_CollapsePath(wxGenericDirCtrl * self, const wxString * path) {
    return self->CollapsePath(*path);
}
void wxGenericDirCtrl_CollapseTree(wxGenericDirCtrl * self) {
    return self->CollapseTree();
}
bool wxGenericDirCtrl_Create(wxGenericDirCtrl * self, wxWindow * parent, wxWindowID id, const wxString * dir, const wxPoint * pos, const wxSize * size, long style, const wxString * filter, int default_filter, const wxString * name) {
    return self->Create(parent, id, *dir, *pos, *size, style, *filter, default_filter, *name);
}
bool wxGenericDirCtrl_ExpandPath(wxGenericDirCtrl * self, const wxString * path) {
    return self->ExpandPath(*path);
}
wxString *wxGenericDirCtrl_GetDefaultPath(const wxGenericDirCtrl * self) {
    return new wxString(self->GetDefaultPath());
}
wxString *wxGenericDirCtrl_GetFilePath(const wxGenericDirCtrl * self) {
    return new wxString(self->GetFilePath());
}
void wxGenericDirCtrl_GetFilePaths(const wxGenericDirCtrl * self, wxArrayString * paths) {
    return self->GetFilePaths(*paths);
}
wxString *wxGenericDirCtrl_GetFilter(const wxGenericDirCtrl * self) {
    return new wxString(self->GetFilter());
}
int wxGenericDirCtrl_GetFilterIndex(const wxGenericDirCtrl * self) {
    return self->GetFilterIndex();
}
wxDirFilterListCtrl * wxGenericDirCtrl_GetFilterListCtrl(const wxGenericDirCtrl * self) {
    return self->GetFilterListCtrl();
}
wxString *wxGenericDirCtrl_GetPath(const wxGenericDirCtrl * self) {
    return new wxString(self->GetPath());
}
void wxGenericDirCtrl_GetPaths(const wxGenericDirCtrl * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
wxTreeCtrl * wxGenericDirCtrl_GetTreeCtrl(const wxGenericDirCtrl * self) {
    return self->GetTreeCtrl();
}
void wxGenericDirCtrl_Init(wxGenericDirCtrl * self) {
    return self->Init();
}
void wxGenericDirCtrl_ReCreateTree(wxGenericDirCtrl * self) {
    return self->ReCreateTree();
}
void wxGenericDirCtrl_SetDefaultPath(wxGenericDirCtrl * self, const wxString * path) {
    return self->SetDefaultPath(*path);
}
void wxGenericDirCtrl_SetFilter(wxGenericDirCtrl * self, const wxString * filter) {
    return self->SetFilter(*filter);
}
void wxGenericDirCtrl_SetFilterIndex(wxGenericDirCtrl * self, int n) {
    return self->SetFilterIndex(n);
}
void wxGenericDirCtrl_SetPath(wxGenericDirCtrl * self, const wxString * path) {
    return self->SetPath(*path);
}
void wxGenericDirCtrl_ShowHidden(wxGenericDirCtrl * self, bool show) {
    return self->ShowHidden(show);
}
void wxGenericDirCtrl_SelectPath(wxGenericDirCtrl * self, const wxString * path, bool select) {
    return self->SelectPath(*path, select);
}
void wxGenericDirCtrl_SelectPaths(wxGenericDirCtrl * self, const wxArrayString * paths) {
    return self->SelectPaths(*paths);
}
void wxGenericDirCtrl_UnselectAll(wxGenericDirCtrl * self) {
    return self->UnselectAll();
}

// CLASS: wxGridSizer
wxClassInfo *wxGridSizer_CLASSINFO() {
    return wxCLASSINFO(wxGridSizer);
}
wxGridSizer *wxGridSizer_new(int cols, int vgap, int hgap) {
    return new wxGridSizer(cols, vgap, hgap);
}
wxGridSizer *wxGridSizer_new1(int cols, const wxSize * gap) {
    return new wxGridSizer(cols, *gap);
}
wxGridSizer *wxGridSizer_new2(int rows, int cols, int vgap, int hgap) {
    return new wxGridSizer(rows, cols, vgap, hgap);
}
wxGridSizer *wxGridSizer_new3(int rows, int cols, const wxSize * gap) {
    return new wxGridSizer(rows, cols, *gap);
}
int wxGridSizer_GetCols(const wxGridSizer * self) {
    return self->GetCols();
}
int wxGridSizer_GetRows(const wxGridSizer * self) {
    return self->GetRows();
}
int wxGridSizer_GetEffectiveColsCount(const wxGridSizer * self) {
    return self->GetEffectiveColsCount();
}
int wxGridSizer_GetEffectiveRowsCount(const wxGridSizer * self) {
    return self->GetEffectiveRowsCount();
}
int wxGridSizer_GetHGap(const wxGridSizer * self) {
    return self->GetHGap();
}
int wxGridSizer_GetVGap(const wxGridSizer * self) {
    return self->GetVGap();
}
void wxGridSizer_SetCols(wxGridSizer * self, int cols) {
    return self->SetCols(cols);
}
void wxGridSizer_SetHGap(wxGridSizer * self, int gap) {
    return self->SetHGap(gap);
}
void wxGridSizer_SetRows(wxGridSizer * self, int rows) {
    return self->SetRows(rows);
}
void wxGridSizer_SetVGap(wxGridSizer * self, int gap) {
    return self->SetVGap(gap);
}

} // extern "C"

