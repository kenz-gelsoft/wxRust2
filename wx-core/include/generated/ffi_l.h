#pragma once

#include <wx/laywin.h>
#include <wx/listbook.h>
#include <wx/listbox.h>
#include <wx/listctrl.h>

extern "C" {

// CLASS: wxLayoutAlgorithm
wxClassInfo *wxLayoutAlgorithm_CLASSINFO();
wxLayoutAlgorithm *wxLayoutAlgorithm_new();
bool wxLayoutAlgorithm_LayoutFrame(wxLayoutAlgorithm * self, wxFrame * frame, wxWindow * main_window);
bool wxLayoutAlgorithm_LayoutWindow(wxLayoutAlgorithm * self, wxWindow * parent, wxWindow * main_window);

// CLASS: wxListBox
wxClassInfo *wxListBox_CLASSINFO();
wxListBox *wxListBox_new();
wxListBox *wxListBox_new2(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
bool wxListBox_Create1(wxListBox * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name);
void wxListBox_Deselect(wxListBox * self, int n);
bool wxListBox_SetStringSelection(wxListBox * self, const wxString * s, bool select);
bool wxListBox_SetStringSelection1(wxListBox * self, const wxString * s);
int wxListBox_GetSelections(const wxListBox * self, wxArrayInt * selections);
int wxListBox_HitTest(const wxListBox * self, const wxPoint * point);
int wxListBox_HitTest1(const wxListBox * self, int x, int y);
void wxListBox_InsertItems1(wxListBox * self, const wxArrayString * items, unsigned int pos);
bool wxListBox_IsSelected(const wxListBox * self, int n);
void wxListBox_SetFirstItem(wxListBox * self, int n);
void wxListBox_SetFirstItem1(wxListBox * self, const wxString * string);
void wxListBox_EnsureVisible(wxListBox * self, int n);
bool wxListBox_IsSorted(const wxListBox * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxListBox_GetCountPerPage(const wxListBox * self);
int wxListBox_GetTopItem(const wxListBox * self);
#endif
// Mix-in(s) to wxListBox
wxItemContainer *wxListBox_AsItemContainer(wxListBox* obj);

// CLASS: wxListCtrl
wxClassInfo *wxListCtrl_CLASSINFO();
wxListCtrl *wxListCtrl_new();
wxListCtrl *wxListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxListCtrl_Arrange(wxListCtrl * self, int flag);
void wxListCtrl_AssignImageList(wxListCtrl * self, wxImageList * image_list, int which);
void wxListCtrl_ClearAll(wxListCtrl * self);
bool wxListCtrl_Create(wxListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxListCtrl_DeleteAllColumns(wxListCtrl * self);
bool wxListCtrl_DeleteAllItems(wxListCtrl * self);
bool wxListCtrl_DeleteColumn(wxListCtrl * self, int col);
bool wxListCtrl_DeleteItem(wxListCtrl * self, long item);
wxTextCtrl * wxListCtrl_EditLabel(wxListCtrl * self, long item, wxClassInfo * text_control_class);
void wxListCtrl_EnableAlternateRowColours(wxListCtrl * self, bool enable);
void wxListCtrl_EnableBellOnNoMatch(wxListCtrl * self, bool on);
bool wxListCtrl_EndEditLabel(wxListCtrl * self, bool cancel);
bool wxListCtrl_EnsureVisible(wxListCtrl * self, long item);
long wxListCtrl_FindItem(wxListCtrl * self, long start, const wxString * str, bool partial);
long wxListCtrl_FindItem2(wxListCtrl * self, long start, const wxPoint * pt, int direction);
bool wxListCtrl_GetColumn(const wxListCtrl * self, int col, wxListItem * item);
int wxListCtrl_GetColumnCount(const wxListCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxListCtrl_GetColumnIndexFromOrder(const wxListCtrl * self, int pos);
int wxListCtrl_GetColumnOrder(const wxListCtrl * self, int col);
#endif
int wxListCtrl_GetColumnWidth(const wxListCtrl * self, int col);
#if wxCHECK_VERSION(3, 1, 0)
wxArrayInt *wxListCtrl_GetColumnsOrder(const wxListCtrl * self);
#endif
int wxListCtrl_GetCountPerPage(const wxListCtrl * self);
wxTextCtrl * wxListCtrl_GetEditControl(const wxListCtrl * self);
wxImageList * wxListCtrl_GetImageList(const wxListCtrl * self, int which);
bool wxListCtrl_GetItem(const wxListCtrl * self, wxListItem * info);
wxColour *wxListCtrl_GetItemBackgroundColour(const wxListCtrl * self, long item);
int wxListCtrl_GetItemCount(const wxListCtrl * self);
wxFont *wxListCtrl_GetItemFont(const wxListCtrl * self, long item);
bool wxListCtrl_GetItemPosition(const wxListCtrl * self, long item, wxPoint * pos);
bool wxListCtrl_GetItemRect(const wxListCtrl * self, long item, wxRect * rect, int code);
wxSize *wxListCtrl_GetItemSpacing(const wxListCtrl * self);
int wxListCtrl_GetItemState(const wxListCtrl * self, long item, long state_mask);
wxString *wxListCtrl_GetItemText(const wxListCtrl * self, long item, int col);
wxColour *wxListCtrl_GetItemTextColour(const wxListCtrl * self, long item);
long wxListCtrl_GetNextItem(const wxListCtrl * self, long item, int geometry, int state);
int wxListCtrl_GetSelectedItemCount(const wxListCtrl * self);
bool wxListCtrl_GetSubItemRect(const wxListCtrl * self, long item, long sub_item, wxRect * rect, int code);
wxColour *wxListCtrl_GetTextColour(const wxListCtrl * self);
long wxListCtrl_GetTopItem(const wxListCtrl * self);
wxRect *wxListCtrl_GetViewRect(const wxListCtrl * self);
void wxListCtrl_SetAlternateRowColour(wxListCtrl * self, const wxColour * colour);
#if wxCHECK_VERSION(3, 1, 0)
wxColour *wxListCtrl_GetAlternateRowColour(const wxListCtrl * self);
#endif
long wxListCtrl_HitTest(const wxListCtrl * self, const wxPoint * point, int * flags, long * ptr_sub_item);
bool wxListCtrl_InReportView(const wxListCtrl * self);
long wxListCtrl_InsertColumn(wxListCtrl * self, long col, const wxListItem * info);
long wxListCtrl_InsertColumn1(wxListCtrl * self, long col, const wxString * heading, int format, int width);
long wxListCtrl_InsertItem(wxListCtrl * self, wxListItem * info);
long wxListCtrl_InsertItem1(wxListCtrl * self, long index, const wxString * label);
long wxListCtrl_InsertItem2(wxListCtrl * self, long index, int image_index);
long wxListCtrl_InsertItem3(wxListCtrl * self, long index, const wxString * label, int image_index);
#if wxCHECK_VERSION(3, 1, 0)
bool wxListCtrl_IsEmpty(const wxListCtrl * self);
#endif
bool wxListCtrl_IsVirtual(const wxListCtrl * self);
void wxListCtrl_RefreshItem(wxListCtrl * self, long item);
void wxListCtrl_RefreshItems(wxListCtrl * self, long item_from, long item_to);
bool wxListCtrl_ScrollList(wxListCtrl * self, int dx, int dy);
bool wxListCtrl_SetColumn(wxListCtrl * self, int col, wxListItem * item);
bool wxListCtrl_SetColumnWidth(wxListCtrl * self, int col, int width);
#if wxCHECK_VERSION(3, 1, 0)
bool wxListCtrl_SetColumnsOrder(wxListCtrl * self, const wxArrayInt * orders);
bool wxListCtrl_SetHeaderAttr(wxListCtrl * self, const wxItemAttr * attr);
#endif
void wxListCtrl_SetImageList(wxListCtrl * self, wxImageList * image_list, int which);
#if wxCHECK_VERSION(3, 1, 0)
void wxListCtrl_SetNormalImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images);
void wxListCtrl_SetSmallImages(wxListCtrl * self, const wxVector< wxBitmapBundle > * images);
bool wxListCtrl_IsVisible(const wxListCtrl * self, long item);
#endif
bool wxListCtrl_SetItem(wxListCtrl * self, wxListItem * info);
bool wxListCtrl_SetItem1(wxListCtrl * self, long index, int column, const wxString * label, int image_id);
void wxListCtrl_SetItemBackgroundColour(wxListCtrl * self, long item, const wxColour * col);
bool wxListCtrl_SetItemColumnImage(wxListCtrl * self, long item, long column, int image);
void wxListCtrl_SetItemCount(wxListCtrl * self, long count);
bool wxListCtrl_SetItemData(wxListCtrl * self, long item, long data);
void wxListCtrl_SetItemFont(wxListCtrl * self, long item, const wxFont * font);
bool wxListCtrl_SetItemImage(wxListCtrl * self, long item, int image, int sel_image);
bool wxListCtrl_SetItemPosition(wxListCtrl * self, long item, const wxPoint * pos);
bool wxListCtrl_SetItemState(wxListCtrl * self, long item, long state, long state_mask);
void wxListCtrl_SetItemText(wxListCtrl * self, long item, const wxString * text);
void wxListCtrl_SetItemTextColour(wxListCtrl * self, long item, const wxColour * col);
void wxListCtrl_SetSingleStyle(wxListCtrl * self, long style, bool add);
void wxListCtrl_SetTextColour(wxListCtrl * self, const wxColour * col);
#if wxCHECK_VERSION(3, 1, 0)
bool wxListCtrl_HasCheckBoxes(const wxListCtrl * self);
bool wxListCtrl_EnableCheckBoxes(wxListCtrl * self, bool enable);
bool wxListCtrl_IsItemChecked(const wxListCtrl * self, long item);
void wxListCtrl_CheckItem(wxListCtrl * self, long item, bool check);
void wxListCtrl_ExtendRulesAndAlternateColour(wxListCtrl * self, bool extend);
void wxListCtrl_ShowSortIndicator(wxListCtrl * self, int col, bool ascending);
void wxListCtrl_RemoveSortIndicator(wxListCtrl * self);
int wxListCtrl_GetSortIndicator(const wxListCtrl * self);
bool wxListCtrl_GetUpdatedAscendingSortIndicator(const wxListCtrl * self, int col);
bool wxListCtrl_IsAscendingSortIndicator(const wxListCtrl * self);
#endif

// CLASS: wxListEvent
wxClassInfo *wxListEvent_CLASSINFO();
long wxListEvent_GetCacheFrom(const wxListEvent * self);
long wxListEvent_GetCacheTo(const wxListEvent * self);
int wxListEvent_GetColumn(const wxListEvent * self);
int wxListEvent_GetImage(const wxListEvent * self);
long wxListEvent_GetIndex(const wxListEvent * self);
wxListItem *wxListEvent_GetItem(const wxListEvent * self);
int wxListEvent_GetKeyCode(const wxListEvent * self);
wxString *wxListEvent_GetLabel(const wxListEvent * self);
long wxListEvent_GetMask(const wxListEvent * self);
wxPoint *wxListEvent_GetPoint(const wxListEvent * self);
wxString *wxListEvent_GetText(const wxListEvent * self);
bool wxListEvent_IsEditCancelled(const wxListEvent * self);
void wxListEvent_SetKeyCode(wxListEvent * self, int code);
void wxListEvent_SetIndex(wxListEvent * self, long index);
void wxListEvent_SetColumn(wxListEvent * self, int col);
void wxListEvent_SetPoint(wxListEvent * self, const wxPoint * point);
void wxListEvent_SetItem(wxListEvent * self, const wxListItem * item);
void wxListEvent_SetCacheFrom(wxListEvent * self, long cache_from);
void wxListEvent_SetCacheTo(wxListEvent * self, long cache_to);

// CLASS: wxListItem
wxClassInfo *wxListItem_CLASSINFO();
wxListItem *wxListItem_new();
void wxListItem_Clear(wxListItem * self);
wxColour *wxListItem_GetBackgroundColour(const wxListItem * self);
int wxListItem_GetColumn(const wxListItem * self);
wxFont *wxListItem_GetFont(const wxListItem * self);
long wxListItem_GetId(const wxListItem * self);
int wxListItem_GetImage(const wxListItem * self);
long wxListItem_GetMask(const wxListItem * self);
long wxListItem_GetState(const wxListItem * self);
wxString *wxListItem_GetText(const wxListItem * self);
wxColour *wxListItem_GetTextColour(const wxListItem * self);
int wxListItem_GetWidth(const wxListItem * self);
void wxListItem_SetBackgroundColour(wxListItem * self, const wxColour * col_back);
void wxListItem_SetColumn(wxListItem * self, int col);
void wxListItem_SetData(wxListItem * self, long data);
void wxListItem_SetData1(wxListItem * self, void * data);
void wxListItem_SetFont(wxListItem * self, const wxFont * font);
void wxListItem_SetId(wxListItem * self, long id);
void wxListItem_SetImage(wxListItem * self, int image);
void wxListItem_SetMask(wxListItem * self, long mask);
void wxListItem_SetState(wxListItem * self, long state);
void wxListItem_SetStateMask(wxListItem * self, long state_mask);
void wxListItem_SetText(wxListItem * self, const wxString * text);
void wxListItem_SetTextColour(wxListItem * self, const wxColour * col_text);
void wxListItem_SetWidth(wxListItem * self, int width);

// CLASS: wxListView
wxClassInfo *wxListView_CLASSINFO();
wxListView *wxListView_new();
wxListView *wxListView_new1(wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxListView_ClearColumnImage(wxListView * self, int col);
void wxListView_Focus(wxListView * self, long index);
long wxListView_GetFirstSelected(const wxListView * self);
long wxListView_GetFocusedItem(const wxListView * self);
long wxListView_GetNextSelected(const wxListView * self, long item);
bool wxListView_IsSelected(const wxListView * self, long index);
void wxListView_Select(wxListView * self, long n, bool on);
void wxListView_SetColumnImage(wxListView * self, int col, int image);

// CLASS: wxListbook
wxClassInfo *wxListbook_CLASSINFO();
wxListbook *wxListbook_new();
wxListbook *wxListbook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxListbook_Create(wxListbook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxListView * wxListbook_GetListView(const wxListbook * self);

} // extern "C"

