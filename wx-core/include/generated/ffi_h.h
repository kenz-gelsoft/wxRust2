#pragma once

#include <wx/headercol.h>
#include <wx/headerctrl.h>
#include <wx/hyperlink.h>

extern "C" {

// CLASS: wxHeaderColumn
void wxHeaderColumn_delete(wxHeaderColumn *self);
wxString *wxHeaderColumn_GetTitle(const wxHeaderColumn * self);
wxBitmap *wxHeaderColumn_GetBitmap(const wxHeaderColumn * self);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxHeaderColumn_GetBitmapBundle(const wxHeaderColumn * self);
#endif
int wxHeaderColumn_GetWidth(const wxHeaderColumn * self);
int wxHeaderColumn_GetMinWidth(const wxHeaderColumn * self);
wxAlignment wxHeaderColumn_GetAlignment(const wxHeaderColumn * self);
int wxHeaderColumn_GetFlags(const wxHeaderColumn * self);
bool wxHeaderColumn_HasFlag(const wxHeaderColumn * self, int flag);
bool wxHeaderColumn_IsResizeable(const wxHeaderColumn * self);
bool wxHeaderColumn_IsSortable(const wxHeaderColumn * self);
bool wxHeaderColumn_IsReorderable(const wxHeaderColumn * self);
bool wxHeaderColumn_IsHidden(const wxHeaderColumn * self);
bool wxHeaderColumn_IsShown(const wxHeaderColumn * self);
bool wxHeaderColumn_IsSortKey(const wxHeaderColumn * self);
bool wxHeaderColumn_IsSortOrderAscending(const wxHeaderColumn * self);

// CLASS: wxHeaderColumnSimple
void wxHeaderColumnSimple_delete(wxHeaderColumnSimple *self);
wxHeaderColumnSimple *wxHeaderColumnSimple_new(const wxString * title, int width, wxAlignment align, int flags);
wxHeaderColumnSimple *wxHeaderColumnSimple_new1(const wxBitmapBundle * bitmap, int width, wxAlignment align, int flags);

// CLASS: wxHeaderCtrl
wxClassInfo *wxHeaderCtrl_CLASSINFO();
bool wxHeaderCtrl_Create(wxHeaderCtrl * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxHeaderCtrl_SetColumnCount(wxHeaderCtrl * self, unsigned int count);
unsigned int wxHeaderCtrl_GetColumnCount(const wxHeaderCtrl * self);
bool wxHeaderCtrl_IsEmpty(const wxHeaderCtrl * self);
void wxHeaderCtrl_UpdateColumn(wxHeaderCtrl * self, unsigned int idx);
void wxHeaderCtrl_SetColumnsOrder(wxHeaderCtrl * self, const wxArrayInt * order);
wxArrayInt *wxHeaderCtrl_GetColumnsOrder(const wxHeaderCtrl * self);
unsigned int wxHeaderCtrl_GetColumnAt(const wxHeaderCtrl * self, unsigned int pos);
unsigned int wxHeaderCtrl_GetColumnPos(const wxHeaderCtrl * self, unsigned int idx);
void wxHeaderCtrl_ResetColumnsOrder(wxHeaderCtrl * self);
bool wxHeaderCtrl_ShowColumnsMenu(wxHeaderCtrl * self, const wxPoint * pt, const wxString * title);
void wxHeaderCtrl_AddColumnsItems(wxHeaderCtrl * self, wxMenu * menu, int id_columns_base);
bool wxHeaderCtrl_ShowCustomizeDialog(wxHeaderCtrl * self);
int wxHeaderCtrl_GetColumnTitleWidth(wxHeaderCtrl * self, const wxHeaderColumn * col);
#if wxCHECK_VERSION(3, 1, 0)
int wxHeaderCtrl_GetColumnTitleWidth1(wxHeaderCtrl * self, unsigned int idx);
#endif
void wxHeaderCtrl_MoveColumnInOrderArray(wxArrayInt * order, unsigned int idx, unsigned int pos);

// CLASS: wxHeaderCtrlSimple
wxClassInfo *wxHeaderCtrlSimple_CLASSINFO();
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new();
wxHeaderCtrlSimple *wxHeaderCtrlSimple_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxHeaderCtrlSimple_InsertColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col, unsigned int idx);
void wxHeaderCtrlSimple_AppendColumn(wxHeaderCtrlSimple * self, const wxHeaderColumnSimple * col);
void wxHeaderCtrlSimple_DeleteColumn(wxHeaderCtrlSimple * self, unsigned int idx);
void wxHeaderCtrlSimple_ShowColumn(wxHeaderCtrlSimple * self, unsigned int idx, bool show);
void wxHeaderCtrlSimple_HideColumn(wxHeaderCtrlSimple * self, unsigned int idx);
void wxHeaderCtrlSimple_ShowSortIndicator(wxHeaderCtrlSimple * self, unsigned int idx, bool sort_order);
void wxHeaderCtrlSimple_RemoveSortIndicator(wxHeaderCtrlSimple * self);

// CLASS: wxHyperlinkCtrl
wxClassInfo *wxHyperlinkCtrl_CLASSINFO();
wxHyperlinkCtrl *wxHyperlinkCtrl_new();
wxHyperlinkCtrl *wxHyperlinkCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxHyperlinkCtrl_Create(wxHyperlinkCtrl * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxString * url, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxColour *wxHyperlinkCtrl_GetHoverColour(const wxHyperlinkCtrl * self);
wxColour *wxHyperlinkCtrl_GetNormalColour(const wxHyperlinkCtrl * self);
wxString *wxHyperlinkCtrl_GetURL(const wxHyperlinkCtrl * self);
bool wxHyperlinkCtrl_GetVisited(const wxHyperlinkCtrl * self);
wxColour *wxHyperlinkCtrl_GetVisitedColour(const wxHyperlinkCtrl * self);
void wxHyperlinkCtrl_SetHoverColour(wxHyperlinkCtrl * self, const wxColour * colour);
void wxHyperlinkCtrl_SetNormalColour(wxHyperlinkCtrl * self, const wxColour * colour);
void wxHyperlinkCtrl_SetURL(wxHyperlinkCtrl * self, const wxString * url);
void wxHyperlinkCtrl_SetVisited(wxHyperlinkCtrl * self, bool visited);
void wxHyperlinkCtrl_SetVisitedColour(wxHyperlinkCtrl * self, const wxColour * colour);

} // extern "C"

