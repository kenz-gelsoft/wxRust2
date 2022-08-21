#pragma once

#include <wx/dcscreen.h>
#include <wx/dcsvg.h>
#include <wx/event.h>
#include <wx/gdicmn.h>
#include <wx/headercol.h>
#include <wx/laywin.h>
#include <wx/preferences.h>
#include <wx/sashwin.h>
#include <wx/scrolbar.h>
#include <wx/settings.h>
#include <wx/simplebook.h>
#include <wx/sizer.h>
#include <wx/slider.h>
#include <wx/sound.h>
#include <wx/spinbutt.h>
#include <wx/spinctrl.h>
#include <wx/splash.h>
#include <wx/splitter.h>
#include <wx/srchctrl.h>
#include <wx/statbmp.h>
#include <wx/statbox.h>
#include <wx/statline.h>
#include <wx/stattext.h>
#include <wx/statusbr.h>

extern "C" {

// CLASS: wxSVGFileDC
wxClassInfo *wxSVGFileDC_CLASSINFO();
wxSVGFileDC *wxSVGFileDC_new(const wxString * filename, int width, int height, double dpi, const wxString * title);
void wxSVGFileDC_Clear(wxSVGFileDC * self);
void wxSVGFileDC_SetBitmapHandler(wxSVGFileDC * self, wxSVGBitmapHandler * handler);
void wxSVGFileDC_DestroyClippingRegion(wxSVGFileDC * self);
void wxSVGFileDC_CrossHair(wxSVGFileDC * self, wxCoord x, wxCoord y);
bool wxSVGFileDC_GetPixel(const wxSVGFileDC * self, wxCoord x, wxCoord y, wxColour * colour);
void wxSVGFileDC_SetPalette(wxSVGFileDC * self, const wxPalette * palette);
int wxSVGFileDC_GetDepth(const wxSVGFileDC * self);
bool wxSVGFileDC_StartDoc(wxSVGFileDC * self, const wxString * message);
void wxSVGFileDC_EndDoc(wxSVGFileDC * self);
void wxSVGFileDC_StartPage(wxSVGFileDC * self);
void wxSVGFileDC_EndPage(wxSVGFileDC * self);

// CLASS: wxSashEvent
wxClassInfo *wxSashEvent_CLASSINFO();
wxRect *wxSashEvent_GetDragRect(const wxSashEvent * self);
void wxSashEvent_SetDragRect(wxSashEvent * self, const wxRect * rect);

// CLASS: wxSashLayoutWindow
wxClassInfo *wxSashLayoutWindow_CLASSINFO();
wxSashLayoutWindow *wxSashLayoutWindow_new();
wxSashLayoutWindow *wxSashLayoutWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSashLayoutWindow_Create(wxSashLayoutWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxSashLayoutWindow_OnCalculateLayout(wxSashLayoutWindow * self, wxCalculateLayoutEvent * event);
void wxSashLayoutWindow_OnQueryLayoutInfo(wxSashLayoutWindow * self, wxQueryLayoutInfoEvent * event);
void wxSashLayoutWindow_SetDefaultSize(wxSashLayoutWindow * self, const wxSize * size);

// CLASS: wxSashWindow
wxClassInfo *wxSashWindow_CLASSINFO();
wxSashWindow *wxSashWindow_new();
wxSashWindow *wxSashWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
int wxSashWindow_GetMaximumSizeX(const wxSashWindow * self);
int wxSashWindow_GetMaximumSizeY(const wxSashWindow * self);
int wxSashWindow_GetMinimumSizeX(const wxSashWindow * self);
int wxSashWindow_GetMinimumSizeY(const wxSashWindow * self);
void wxSashWindow_SetMaximumSizeX(wxSashWindow * self, int min);
void wxSashWindow_SetMaximumSizeY(wxSashWindow * self, int min);
void wxSashWindow_SetMinimumSizeX(wxSashWindow * self, int min);
void wxSashWindow_SetMinimumSizeY(wxSashWindow * self, int min);
void wxSashWindow_SetDefaultBorderSize(wxSashWindow * self, int width);
int wxSashWindow_GetDefaultBorderSize(const wxSashWindow * self);
void wxSashWindow_SetExtraBorderSize(wxSashWindow * self, int width);
int wxSashWindow_GetExtraBorderSize(const wxSashWindow * self);
void wxSashWindow_SizeWindows(wxSashWindow * self);

// CLASS: wxScreenDC
wxClassInfo *wxScreenDC_CLASSINFO();
wxScreenDC *wxScreenDC_new();
bool wxScreenDC_EndDrawingOnTop();
bool wxScreenDC_StartDrawingOnTop(wxWindow * window);
bool wxScreenDC_StartDrawingOnTop1(wxRect * rect);

// CLASS: wxScrollBar
wxClassInfo *wxScrollBar_CLASSINFO();
wxScrollBar *wxScrollBar_new();
wxScrollBar *wxScrollBar_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxScrollBar_Create(wxScrollBar * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxScrollBar_GetPageSize(const wxScrollBar * self);
int wxScrollBar_GetRange(const wxScrollBar * self);
int wxScrollBar_GetThumbPosition(const wxScrollBar * self);
int wxScrollBar_GetThumbSize(const wxScrollBar * self);
void wxScrollBar_SetThumbPosition(wxScrollBar * self, int view_start);
bool wxScrollBar_IsVertical(const wxScrollBar * self);

// CLASS: wxScrollEvent
wxClassInfo *wxScrollEvent_CLASSINFO();
int wxScrollEvent_GetOrientation(const wxScrollEvent * self);
int wxScrollEvent_GetPosition(const wxScrollEvent * self);
void wxScrollEvent_SetOrientation(wxScrollEvent * self, int orient);
void wxScrollEvent_SetPosition(wxScrollEvent * self, int pos);

// CLASS: wxScrollWinEvent
wxClassInfo *wxScrollWinEvent_CLASSINFO();
int wxScrollWinEvent_GetOrientation(const wxScrollWinEvent * self);
int wxScrollWinEvent_GetPosition(const wxScrollWinEvent * self);
void wxScrollWinEvent_SetOrientation(wxScrollWinEvent * self, int orient);
void wxScrollWinEvent_SetPosition(wxScrollWinEvent * self, int pos);

// CLASS: wxSearchCtrl
wxClassInfo *wxSearchCtrl_CLASSINFO();
wxSearchCtrl *wxSearchCtrl_new();
wxSearchCtrl *wxSearchCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxSearchCtrl_Create(wxSearchCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxMenu * wxSearchCtrl_GetMenu(wxSearchCtrl * self);
bool wxSearchCtrl_IsSearchButtonVisible(const wxSearchCtrl * self);
bool wxSearchCtrl_IsCancelButtonVisible(const wxSearchCtrl * self);
void wxSearchCtrl_SetMenu(wxSearchCtrl * self, wxMenu * menu);
void wxSearchCtrl_ShowCancelButton(wxSearchCtrl * self, bool show);
void wxSearchCtrl_ShowSearchButton(wxSearchCtrl * self, bool show);
void wxSearchCtrl_SetDescriptiveText(wxSearchCtrl * self, const wxString * text);
wxString *wxSearchCtrl_GetDescriptiveText(const wxSearchCtrl * self);
// Mix-in(s) to wxSearchCtrl
wxTextEntryBase *wxSearchCtrl_AsTextEntry(wxSearchCtrl* obj);

// CLASS: wxSetCursorEvent
wxClassInfo *wxSetCursorEvent_CLASSINFO();
wxSetCursorEvent *wxSetCursorEvent_new(wxCoord x, wxCoord y);
wxCursor *wxSetCursorEvent_GetCursor(const wxSetCursorEvent * self);
wxCoord wxSetCursorEvent_GetX(const wxSetCursorEvent * self);
wxCoord wxSetCursorEvent_GetY(const wxSetCursorEvent * self);
bool wxSetCursorEvent_HasCursor(const wxSetCursorEvent * self);
void wxSetCursorEvent_SetCursor(wxSetCursorEvent * self, const wxCursor * cursor);

// CLASS: wxSettableHeaderColumn
void wxSettableHeaderColumn_delete(wxSettableHeaderColumn *self);
void wxSettableHeaderColumn_SetTitle(wxSettableHeaderColumn * self, const wxString * title);
void wxSettableHeaderColumn_SetBitmap(wxSettableHeaderColumn * self, const wxBitmapBundle * bitmap);
void wxSettableHeaderColumn_SetWidth(wxSettableHeaderColumn * self, int width);
void wxSettableHeaderColumn_SetMinWidth(wxSettableHeaderColumn * self, int min_width);
void wxSettableHeaderColumn_SetAlignment(wxSettableHeaderColumn * self, wxAlignment align);
void wxSettableHeaderColumn_SetFlags(wxSettableHeaderColumn * self, int flags);
void wxSettableHeaderColumn_ChangeFlag(wxSettableHeaderColumn * self, int flag, bool set);
void wxSettableHeaderColumn_SetFlag(wxSettableHeaderColumn * self, int flag);
void wxSettableHeaderColumn_ClearFlag(wxSettableHeaderColumn * self, int flag);
void wxSettableHeaderColumn_ToggleFlag(wxSettableHeaderColumn * self, int flag);
void wxSettableHeaderColumn_SetResizeable(wxSettableHeaderColumn * self, bool resizable);
void wxSettableHeaderColumn_SetSortable(wxSettableHeaderColumn * self, bool sortable);
void wxSettableHeaderColumn_SetReorderable(wxSettableHeaderColumn * self, bool reorderable);
void wxSettableHeaderColumn_SetHidden(wxSettableHeaderColumn * self, bool hidden);
void wxSettableHeaderColumn_UnsetAsSortKey(wxSettableHeaderColumn * self);
void wxSettableHeaderColumn_SetSortOrder(wxSettableHeaderColumn * self, bool ascending);
void wxSettableHeaderColumn_ToggleSortOrder(wxSettableHeaderColumn * self);

// CLASS: wxShowEvent
wxClassInfo *wxShowEvent_CLASSINFO();
wxShowEvent *wxShowEvent_new(int winid, bool show);
void wxShowEvent_SetShow(wxShowEvent * self, bool show);
bool wxShowEvent_IsShown(const wxShowEvent * self);

// CLASS: wxSimplebook
wxClassInfo *wxSimplebook_CLASSINFO();
wxSimplebook *wxSimplebook_new();
wxSimplebook *wxSimplebook_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSimplebook_Create(wxSimplebook * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSimplebook_ShowNewPage(wxSimplebook * self, wxWindow * page);

// CLASS: wxSize
void wxSize_delete(wxSize *self);
wxSize *wxSize_new();
wxSize *wxSize_new1(int width, int height);
void wxSize_DecBy(wxSize * self, const wxPoint * pt);
void wxSize_DecBy1(wxSize * self, const wxSize * size);
void wxSize_DecBy2(wxSize * self, int dx, int dy);
void wxSize_DecBy3(wxSize * self, int d);
void wxSize_DecTo(wxSize * self, const wxSize * size);
void wxSize_DecToIfSpecified(wxSize * self, const wxSize * size);
int wxSize_GetHeight(const wxSize * self);
int wxSize_GetWidth(const wxSize * self);
void wxSize_IncBy(wxSize * self, const wxPoint * pt);
void wxSize_IncBy1(wxSize * self, const wxSize * size);
void wxSize_IncBy2(wxSize * self, int dx, int dy);
void wxSize_IncBy3(wxSize * self, int d);
void wxSize_IncTo(wxSize * self, const wxSize * size);
bool wxSize_IsFullySpecified(const wxSize * self);
void wxSize_Set(wxSize * self, int width, int height);
void wxSize_SetDefaults(wxSize * self, const wxSize * size_default);
void wxSize_SetHeight(wxSize * self, int height);
void wxSize_SetWidth(wxSize * self, int width);

// CLASS: wxSizeEvent
wxClassInfo *wxSizeEvent_CLASSINFO();
wxSizeEvent *wxSizeEvent_new(const wxSize * sz, int id);
wxSize *wxSizeEvent_GetSize(const wxSizeEvent * self);
wxRect *wxSizeEvent_GetRect(const wxSizeEvent * self);

// CLASS: wxSizer
wxClassInfo *wxSizer_CLASSINFO();
wxSizerItem * wxSizer_Add(wxSizer * self, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Add5(wxSizer * self, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Add6(wxSizer * self, wxSizerItem * item);
wxSizerItem * wxSizer_AddSpacer(wxSizer * self, int size);
wxSizerItem * wxSizer_AddStretchSpacer(wxSizer * self, int prop);
wxSize *wxSizer_CalcMin(wxSizer * self);
void wxSizer_Clear(wxSizer * self, bool delete_windows);
wxSize *wxSizer_ComputeFittingClientSize(wxSizer * self, wxWindow * window);
wxSize *wxSizer_ComputeFittingWindowSize(wxSizer * self, wxWindow * window);
bool wxSizer_Detach(wxSizer * self, wxWindow * window);
bool wxSizer_Detach1(wxSizer * self, wxSizer * sizer);
bool wxSizer_Detach2(wxSizer * self, int index);
wxSize *wxSizer_Fit(wxSizer * self, wxWindow * window);
void wxSizer_FitInside(wxSizer * self, wxWindow * window);
bool wxSizer_InformFirstDirection(wxSizer * self, int direction, int size, int available_other_dir);
wxSizerItemList * wxSizer_GetChildren(wxSizer * self);
wxWindow * wxSizer_GetContainingWindow(const wxSizer * self);
void wxSizer_SetContainingWindow(wxSizer * self, wxWindow * window);
size_t wxSizer_GetItemCount(const wxSizer * self);
wxSizerItem * wxSizer_GetItem(wxSizer * self, wxWindow * window, bool recursive);
wxSizerItem * wxSizer_GetItem1(wxSizer * self, wxSizer * sizer, bool recursive);
wxSizerItem * wxSizer_GetItem2(wxSizer * self, size_t index);
wxSizerItem * wxSizer_GetItemById(wxSizer * self, int id, bool recursive);
wxSize *wxSizer_GetMinSize(wxSizer * self);
wxPoint *wxSizer_GetPosition(const wxSizer * self);
wxSize *wxSizer_GetSize(const wxSizer * self);
bool wxSizer_Hide(wxSizer * self, wxWindow * window, bool recursive);
bool wxSizer_Hide1(wxSizer * self, wxSizer * sizer, bool recursive);
bool wxSizer_Hide2(wxSizer * self, size_t index);
wxSizerItem * wxSizer_Insert(wxSizer * self, size_t index, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert1(wxSizer * self, size_t index, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert2(wxSizer * self, size_t index, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert3(wxSizer * self, size_t index, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert4(wxSizer * self, size_t index, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Insert5(wxSizer * self, size_t index, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Insert6(wxSizer * self, size_t index, wxSizerItem * item);
wxSizerItem * wxSizer_InsertSpacer(wxSizer * self, size_t index, int size);
wxSizerItem * wxSizer_InsertStretchSpacer(wxSizer * self, size_t index, int prop);
bool wxSizer_IsEmpty(const wxSizer * self);
bool wxSizer_IsShown(const wxSizer * self, wxWindow * window);
bool wxSizer_IsShown1(const wxSizer * self, wxSizer * sizer);
bool wxSizer_IsShown2(const wxSizer * self, size_t index);
void wxSizer_Layout(wxSizer * self);
wxSizerItem * wxSizer_Prepend(wxSizer * self, wxWindow * window, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend1(wxSizer * self, wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend2(wxSizer * self, wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend3(wxSizer * self, wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend4(wxSizer * self, int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem * wxSizer_Prepend5(wxSizer * self, int width, int height, const wxSizerFlags * flags);
wxSizerItem * wxSizer_Prepend6(wxSizer * self, wxSizerItem * item);
wxSizerItem * wxSizer_PrependSpacer(wxSizer * self, int size);
wxSizerItem * wxSizer_PrependStretchSpacer(wxSizer * self, int prop);
#if wxCHECK_VERSION(3, 1, 0)
void wxSizer_RepositionChildren(wxSizer * self, const wxSize * min_size);
#endif
bool wxSizer_Remove1(wxSizer * self, wxSizer * sizer);
bool wxSizer_Remove2(wxSizer * self, int index);
bool wxSizer_Replace(wxSizer * self, wxWindow * oldwin, wxWindow * newwin, bool recursive);
bool wxSizer_Replace1(wxSizer * self, wxSizer * oldsz, wxSizer * newsz, bool recursive);
bool wxSizer_Replace2(wxSizer * self, size_t index, wxSizerItem * newitem);
void wxSizer_SetDimension(wxSizer * self, int x, int y, int width, int height);
void wxSizer_SetDimension1(wxSizer * self, const wxPoint * pos, const wxSize * size);
bool wxSizer_SetItemMinSize(wxSizer * self, wxWindow * window, int width, int height);
bool wxSizer_SetItemMinSize1(wxSizer * self, wxWindow * window, const wxSize * size);
bool wxSizer_SetItemMinSize2(wxSizer * self, wxSizer * sizer, int width, int height);
bool wxSizer_SetItemMinSize3(wxSizer * self, wxSizer * sizer, const wxSize * size);
bool wxSizer_SetItemMinSize4(wxSizer * self, size_t index, int width, int height);
bool wxSizer_SetItemMinSize5(wxSizer * self, size_t index, const wxSize * size);
void wxSizer_SetMinSize(wxSizer * self, const wxSize * size);
void wxSizer_SetMinSize1(wxSizer * self, int width, int height);
void wxSizer_SetSizeHints(wxSizer * self, wxWindow * window);
bool wxSizer_Show(wxSizer * self, wxWindow * window, bool show, bool recursive);
bool wxSizer_Show1(wxSizer * self, wxSizer * sizer, bool show, bool recursive);
bool wxSizer_Show2(wxSizer * self, size_t index, bool show);
void wxSizer_ShowItems(wxSizer * self, bool show);

// CLASS: wxSizerFlags
void wxSizerFlags_delete(wxSizerFlags *self);
wxSizerFlags *wxSizerFlags_new(int proportion);
wxSizerFlags * wxSizerFlags_Align(wxSizerFlags * self, int alignment);
wxSizerFlags * wxSizerFlags_Border(wxSizerFlags * self, int direction, int borderinpixels);
wxSizerFlags * wxSizerFlags_Border1(wxSizerFlags * self, int direction);
wxSizerFlags * wxSizerFlags_Bottom(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Center(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Centre(wxSizerFlags * self);
#if wxCHECK_VERSION(3, 1, 0)
wxSizerFlags * wxSizerFlags_CenterHorizontal(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CenterVertical(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CentreHorizontal(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_CentreVertical(wxSizerFlags * self);
#endif
wxSizerFlags * wxSizerFlags_DoubleBorder(wxSizerFlags * self, int direction);
wxSizerFlags * wxSizerFlags_DoubleHorzBorder(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Expand(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_FixedMinSize(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_ReserveSpaceEvenIfHidden(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Left(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Proportion(wxSizerFlags * self, int proportion);
wxSizerFlags * wxSizerFlags_Right(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Shaped(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_Top(wxSizerFlags * self);
wxSizerFlags * wxSizerFlags_TripleBorder(wxSizerFlags * self, int direction);
#if wxCHECK_VERSION(3, 1, 0)
void wxSizerFlags_DisableConsistencyChecks();
#endif
int wxSizerFlags_GetDefaultBorder();

// CLASS: wxSizerItem
wxClassInfo *wxSizerItem_CLASSINFO();
wxSizerItem *wxSizerItem_new(int width, int height, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem *wxSizerItem_new1(wxWindow * window, const wxSizerFlags * flags);
wxSizerItem *wxSizerItem_new2(wxWindow * window, int proportion, int flag, int border, wxObject * user_data);
wxSizerItem *wxSizerItem_new3(wxSizer * sizer, const wxSizerFlags * flags);
wxSizerItem *wxSizerItem_new4(wxSizer * sizer, int proportion, int flag, int border, wxObject * user_data);
void wxSizerItem_AssignWindow(wxSizerItem * self, wxWindow * window);
void wxSizerItem_AssignSizer(wxSizerItem * self, wxSizer * sizer);
void wxSizerItem_AssignSpacer(wxSizerItem * self, const wxSize * size);
void wxSizerItem_AssignSpacer1(wxSizerItem * self, int w, int h);
wxSize *wxSizerItem_CalcMin(wxSizerItem * self);
void wxSizerItem_DeleteWindows(wxSizerItem * self);
void wxSizerItem_DetachSizer(wxSizerItem * self);
int wxSizerItem_GetBorder(const wxSizerItem * self);
int wxSizerItem_GetFlag(const wxSizerItem * self);
int wxSizerItem_GetId(const wxSizerItem * self);
wxSize *wxSizerItem_GetMinSize(const wxSizerItem * self);
void wxSizerItem_SetMinSize(wxSizerItem * self, const wxSize * size);
void wxSizerItem_SetMinSize1(wxSizerItem * self, int x, int y);
wxPoint *wxSizerItem_GetPosition(const wxSizerItem * self);
int wxSizerItem_GetProportion(const wxSizerItem * self);
wxRect *wxSizerItem_GetRect(wxSizerItem * self);
wxSize *wxSizerItem_GetSize(const wxSizerItem * self);
wxSizer * wxSizerItem_GetSizer(const wxSizerItem * self);
wxSize *wxSizerItem_GetSpacer(const wxSizerItem * self);
wxObject * wxSizerItem_GetUserData(const wxSizerItem * self);
wxWindow * wxSizerItem_GetWindow(const wxSizerItem * self);
bool wxSizerItem_IsShown(const wxSizerItem * self);
bool wxSizerItem_IsSizer(const wxSizerItem * self);
bool wxSizerItem_IsSpacer(const wxSizerItem * self);
bool wxSizerItem_IsWindow(const wxSizerItem * self);
void wxSizerItem_SetBorder(wxSizerItem * self, int border);
void wxSizerItem_SetDimension(wxSizerItem * self, const wxPoint * pos, const wxSize * size);
void wxSizerItem_SetFlag(wxSizerItem * self, int flag);
void wxSizerItem_SetId(wxSizerItem * self, int id);
void wxSizerItem_SetInitSize(wxSizerItem * self, int x, int y);
void wxSizerItem_SetProportion(wxSizerItem * self, int proportion);
void wxSizerItem_SetRatio(wxSizerItem * self, int width, int height);
void wxSizerItem_SetUserData(wxSizerItem * self, wxObject * user_data);
void wxSizerItem_Show(wxSizerItem * self, bool show);

// CLASS: wxSlider
wxClassInfo *wxSlider_CLASSINFO();
wxSlider *wxSlider_new();
wxSlider *wxSlider_new1(wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
void wxSlider_ClearSel(wxSlider * self);
void wxSlider_ClearTicks(wxSlider * self);
bool wxSlider_Create(wxSlider * self, wxWindow * parent, wxWindowID id, int value, int min_value, int max_value, const wxPoint * point, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxSlider_GetLineSize(const wxSlider * self);
int wxSlider_GetMax(const wxSlider * self);
int wxSlider_GetMin(const wxSlider * self);
int wxSlider_GetPageSize(const wxSlider * self);
int wxSlider_GetSelEnd(const wxSlider * self);
int wxSlider_GetSelStart(const wxSlider * self);
int wxSlider_GetThumbLength(const wxSlider * self);
int wxSlider_GetTickFreq(const wxSlider * self);
int wxSlider_GetValue(const wxSlider * self);
void wxSlider_SetLineSize(wxSlider * self, int line_size);
void wxSlider_SetMin(wxSlider * self, int min_value);
void wxSlider_SetMax(wxSlider * self, int max_value);
void wxSlider_SetPageSize(wxSlider * self, int page_size);
void wxSlider_SetRange(wxSlider * self, int min_value, int max_value);
void wxSlider_SetSelection(wxSlider * self, int start_pos, int end_pos);
void wxSlider_SetThumbLength(wxSlider * self, int len);
void wxSlider_SetTick(wxSlider * self, int tick_pos);
void wxSlider_SetTickFreq(wxSlider * self, int freq);
void wxSlider_SetValue(wxSlider * self, int value);

// CLASS: wxSound
wxClassInfo *wxSound_CLASSINFO();
wxSound *wxSound_new();
wxSound *wxSound_new1(const wxString * file_name, bool is_resource);
wxSound *wxSound_new2(size_t size, const void * data);
bool wxSound_Create(wxSound * self, const wxString * file_name, bool is_resource);
bool wxSound_Create1(wxSound * self, size_t size, const void * data);
bool wxSound_IsOk(const wxSound * self);
#ifndef __WXMSW__
bool wxSound_IsPlaying();
#endif
void wxSound_Stop();

// CLASS: wxSpinButton
wxClassInfo *wxSpinButton_CLASSINFO();
wxSpinButton *wxSpinButton_new();
wxSpinButton *wxSpinButton_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSpinButton_Create(wxSpinButton * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
#if wxCHECK_VERSION(3, 1, 0)
int wxSpinButton_GetIncrement(const wxSpinButton * self);
#endif
int wxSpinButton_GetMax(const wxSpinButton * self);
int wxSpinButton_GetMin(const wxSpinButton * self);
int wxSpinButton_GetValue(const wxSpinButton * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxSpinButton_SetIncrement(wxSpinButton * self, int value);
#endif
void wxSpinButton_SetRange(wxSpinButton * self, int min, int max);
void wxSpinButton_SetValue(wxSpinButton * self, int value);

// CLASS: wxSpinCtrl
wxClassInfo *wxSpinCtrl_CLASSINFO();
wxSpinCtrl *wxSpinCtrl_new();
wxSpinCtrl *wxSpinCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name);
bool wxSpinCtrl_Create(wxSpinCtrl * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, int min, int max, int initial, const wxString * name);
int wxSpinCtrl_GetBase(const wxSpinCtrl * self);
int wxSpinCtrl_GetMax(const wxSpinCtrl * self);
int wxSpinCtrl_GetMin(const wxSpinCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrl_GetTextValue(const wxSpinCtrl * self);
#endif
int wxSpinCtrl_GetValue(const wxSpinCtrl * self);
int wxSpinCtrl_GetIncrement(const wxSpinCtrl * self);
bool wxSpinCtrl_SetBase(wxSpinCtrl * self, int base);
void wxSpinCtrl_SetRange(wxSpinCtrl * self, int min_val, int max_val);
void wxSpinCtrl_SetSelection(wxSpinCtrl * self, long from, long to);
void wxSpinCtrl_SetValue(wxSpinCtrl * self, const wxString * text);
void wxSpinCtrl_SetValue1(wxSpinCtrl * self, int value);
void wxSpinCtrl_SetIncrement(wxSpinCtrl * self, int value);

// CLASS: wxSpinCtrlDouble
wxClassInfo *wxSpinCtrlDouble_CLASSINFO();
wxSpinCtrlDouble *wxSpinCtrlDouble_new();
wxSpinCtrlDouble *wxSpinCtrlDouble_new1(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name);
bool wxSpinCtrlDouble_Create(wxSpinCtrlDouble * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, long style, double min, double max, double initial, double inc, const wxString * name);
unsigned int wxSpinCtrlDouble_GetDigits(const wxSpinCtrlDouble * self);
double wxSpinCtrlDouble_GetIncrement(const wxSpinCtrlDouble * self);
double wxSpinCtrlDouble_GetMax(const wxSpinCtrlDouble * self);
double wxSpinCtrlDouble_GetMin(const wxSpinCtrlDouble * self);
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxSpinCtrlDouble_GetTextValue(const wxSpinCtrlDouble * self);
#endif
double wxSpinCtrlDouble_GetValue(const wxSpinCtrlDouble * self);
void wxSpinCtrlDouble_SetDigits(wxSpinCtrlDouble * self, unsigned int digits);
void wxSpinCtrlDouble_SetIncrement(wxSpinCtrlDouble * self, double inc);
void wxSpinCtrlDouble_SetRange(wxSpinCtrlDouble * self, double min_val, double max_val);
void wxSpinCtrlDouble_SetValue(wxSpinCtrlDouble * self, const wxString * text);
void wxSpinCtrlDouble_SetValue1(wxSpinCtrlDouble * self, double value);

// CLASS: wxSpinDoubleEvent
wxClassInfo *wxSpinDoubleEvent_CLASSINFO();
wxSpinDoubleEvent *wxSpinDoubleEvent_new1(const wxSpinDoubleEvent * event);
double wxSpinDoubleEvent_GetValue(const wxSpinDoubleEvent * self);
void wxSpinDoubleEvent_SetValue(wxSpinDoubleEvent * self, double value);

// CLASS: wxSpinEvent
wxClassInfo *wxSpinEvent_CLASSINFO();
int wxSpinEvent_GetPosition(const wxSpinEvent * self);
void wxSpinEvent_SetPosition(wxSpinEvent * self, int pos);

// CLASS: wxSplashScreen
wxClassInfo *wxSplashScreen_CLASSINFO();
wxSplashScreen *wxSplashScreen_new(const wxBitmap * bitmap, long splash_style, int milliseconds, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style);
long wxSplashScreen_GetSplashStyle(const wxSplashScreen * self);
wxSplashScreenWindow * wxSplashScreen_GetSplashWindow(const wxSplashScreen * self);
int wxSplashScreen_GetTimeout(const wxSplashScreen * self);
void wxSplashScreen_OnCloseWindow(wxSplashScreen * self, wxCloseEvent * event);

// CLASS: wxSplitterEvent
wxClassInfo *wxSplitterEvent_CLASSINFO();
int wxSplitterEvent_GetSashPosition(const wxSplitterEvent * self);
wxWindow * wxSplitterEvent_GetWindowBeingRemoved(const wxSplitterEvent * self);
int wxSplitterEvent_GetX(const wxSplitterEvent * self);
int wxSplitterEvent_GetY(const wxSplitterEvent * self);
void wxSplitterEvent_SetSashPosition(wxSplitterEvent * self, int pos);
#if wxCHECK_VERSION(3, 1, 0)
void wxSplitterEvent_SetSize(wxSplitterEvent * self, int old_size, int new_size);
int wxSplitterEvent_GetOldSize(const wxSplitterEvent * self);
#endif

// CLASS: wxSplitterWindow
wxClassInfo *wxSplitterWindow_CLASSINFO();
wxSplitterWindow *wxSplitterWindow_new();
wxSplitterWindow *wxSplitterWindow_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxSplitterWindow_Create(wxSplitterWindow * self, wxWindow * parent, wxWindowID id, const wxPoint * point, const wxSize * size, long style, const wxString * name);
int wxSplitterWindow_GetMinimumPaneSize(const wxSplitterWindow * self);
double wxSplitterWindow_GetSashGravity(const wxSplitterWindow * self);
int wxSplitterWindow_GetSashPosition(const wxSplitterWindow * self);
int wxSplitterWindow_GetSashSize(const wxSplitterWindow * self);
int wxSplitterWindow_GetDefaultSashSize(const wxSplitterWindow * self);
wxWindow * wxSplitterWindow_GetWindow1(const wxSplitterWindow * self);
wxWindow * wxSplitterWindow_GetWindow2(const wxSplitterWindow * self);
void wxSplitterWindow_Initialize(wxSplitterWindow * self, wxWindow * window);
bool wxSplitterWindow_IsSashInvisible(const wxSplitterWindow * self);
bool wxSplitterWindow_IsSplit(const wxSplitterWindow * self);
void wxSplitterWindow_OnDoubleClickSash(wxSplitterWindow * self, int x, int y);
bool wxSplitterWindow_OnSashPositionChange(wxSplitterWindow * self, int new_sash_position);
void wxSplitterWindow_OnUnsplit(wxSplitterWindow * self, wxWindow * removed);
bool wxSplitterWindow_ReplaceWindow(wxSplitterWindow * self, wxWindow * win_old, wxWindow * win_new);
void wxSplitterWindow_SetMinimumPaneSize(wxSplitterWindow * self, int pane_size);
void wxSplitterWindow_SetSashGravity(wxSplitterWindow * self, double gravity);
void wxSplitterWindow_SetSashPosition(wxSplitterWindow * self, int position, bool redraw);
void wxSplitterWindow_SetSplitMode(wxSplitterWindow * self, int mode);
void wxSplitterWindow_SetSashInvisible(wxSplitterWindow * self, bool invisible);
bool wxSplitterWindow_SplitHorizontally(wxSplitterWindow * self, wxWindow * window1, wxWindow * window2, int sash_position);
bool wxSplitterWindow_SplitVertically(wxSplitterWindow * self, wxWindow * window1, wxWindow * window2, int sash_position);
bool wxSplitterWindow_Unsplit(wxSplitterWindow * self, wxWindow * to_remove);
void wxSplitterWindow_UpdateSize(wxSplitterWindow * self);

// CLASS: wxStaticBitmap
wxClassInfo *wxStaticBitmap_CLASSINFO();
wxStaticBitmap *wxStaticBitmap_new();
wxStaticBitmap *wxStaticBitmap_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticBitmap_Create(wxStaticBitmap * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxBitmap *wxStaticBitmap_GetBitmap(const wxStaticBitmap * self);
wxIcon *wxStaticBitmap_GetIcon(const wxStaticBitmap * self);
void wxStaticBitmap_SetBitmap(wxStaticBitmap * self, const wxBitmapBundle * label);
void wxStaticBitmap_SetIcon(wxStaticBitmap * self, const wxIcon * label);

// CLASS: wxStaticBox
wxClassInfo *wxStaticBox_CLASSINFO();
wxStaticBox *wxStaticBox_new();
wxStaticBox *wxStaticBox_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticBox_Create(wxStaticBox * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);

// CLASS: wxStaticBoxSizer
wxClassInfo *wxStaticBoxSizer_CLASSINFO();
wxStaticBoxSizer *wxStaticBoxSizer_new(wxStaticBox * box_, int orient);
wxStaticBoxSizer *wxStaticBoxSizer_new1(int orient, wxWindow * parent, const wxString * label);
wxStaticBox * wxStaticBoxSizer_GetStaticBox(const wxStaticBoxSizer * self);

// CLASS: wxStaticLine
wxClassInfo *wxStaticLine_CLASSINFO();
wxStaticLine *wxStaticLine_new();
wxStaticLine *wxStaticLine_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticLine_Create(wxStaticLine * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticLine_IsVertical(const wxStaticLine * self);
int wxStaticLine_GetDefaultSize();

// CLASS: wxStaticText
wxClassInfo *wxStaticText_CLASSINFO();
wxStaticText *wxStaticText_new();
wxStaticText *wxStaticText_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticText_Create(wxStaticText * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxStaticText_IsEllipsized(const wxStaticText * self);
void wxStaticText_Wrap(wxStaticText * self, int width);

// CLASS: wxStatusBar
wxClassInfo *wxStatusBar_CLASSINFO();
wxStatusBar *wxStatusBar_new();
wxStatusBar *wxStatusBar_new1(wxWindow * parent, wxWindowID id, long style, const wxString * name);
bool wxStatusBar_Create(wxStatusBar * self, wxWindow * parent, wxWindowID id, long style, const wxString * name);
bool wxStatusBar_GetFieldRect(const wxStatusBar * self, int i, wxRect * rect);
int wxStatusBar_GetFieldsCount(const wxStatusBar * self);
wxStatusBarPane *wxStatusBar_GetField(const wxStatusBar * self, int n);
wxSize *wxStatusBar_GetBorders(const wxStatusBar * self);
wxString *wxStatusBar_GetStatusText(const wxStatusBar * self, int i);
int wxStatusBar_GetStatusWidth(const wxStatusBar * self, int n);
int wxStatusBar_GetStatusStyle(const wxStatusBar * self, int n);
void wxStatusBar_PopStatusText(wxStatusBar * self, int field);
void wxStatusBar_PushStatusText(wxStatusBar * self, const wxString * string, int field);
void wxStatusBar_SetFieldsCount(wxStatusBar * self, int number, const int * widths);
void wxStatusBar_SetMinHeight(wxStatusBar * self, int height);
void wxStatusBar_SetStatusStyles(wxStatusBar * self, int n, const int * styles);
void wxStatusBar_SetStatusText(wxStatusBar * self, const wxString * text, int i);
void wxStatusBar_SetStatusWidths(wxStatusBar * self, int n, const int * widths_field);

// CLASS: wxStatusBarPane
void wxStatusBarPane_delete(wxStatusBarPane *self);
wxStatusBarPane *wxStatusBarPane_new(int style, int width);
int wxStatusBarPane_GetWidth(const wxStatusBarPane * self);
int wxStatusBarPane_GetStyle(const wxStatusBarPane * self);
wxString *wxStatusBarPane_GetText(const wxStatusBarPane * self);

// CLASS: wxStdDialogButtonSizer
wxClassInfo *wxStdDialogButtonSizer_CLASSINFO();
wxStdDialogButtonSizer *wxStdDialogButtonSizer_new();
void wxStdDialogButtonSizer_AddButton(wxStdDialogButtonSizer * self, wxButton * button);
void wxStdDialogButtonSizer_Realize(wxStdDialogButtonSizer * self);
void wxStdDialogButtonSizer_SetAffirmativeButton(wxStdDialogButtonSizer * self, wxButton * button);
void wxStdDialogButtonSizer_SetCancelButton(wxStdDialogButtonSizer * self, wxButton * button);
void wxStdDialogButtonSizer_SetNegativeButton(wxStdDialogButtonSizer * self, wxButton * button);

// CLASS: wxStockPreferencesPage
void wxStockPreferencesPage_delete(wxStockPreferencesPage *self);

// CLASS: wxSysColourChangedEvent
wxClassInfo *wxSysColourChangedEvent_CLASSINFO();
wxSysColourChangedEvent *wxSysColourChangedEvent_new();

// CLASS: wxSystemSettings
void wxSystemSettings_delete(wxSystemSettings *self);
wxSystemSettings *wxSystemSettings_new();

} // extern "C"

