#pragma once

#include <wx/animate.h>
#include <wx/dirctrl.h>
#include <wx/event.h>
#include <wx/gauge.h>
#include <wx/gbsizer.h>
#include <wx/gdiobj.h>
#include <wx/generic/aboutdlgg.h>
#include <wx/graphics.h>
#include <wx/grid.h>
#include <wx/imaggif.h>
#include <wx/progdlg.h>
#include <wx/sizer.h>
#include <wx/valgen.h>

extern "C" {

// CLASS: wxGBPosition
void wxGBPosition_delete(wxGBPosition *self);
wxGBPosition *wxGBPosition_new();
wxGBPosition *wxGBPosition_new1(int row, int col);
int wxGBPosition_GetCol(const wxGBPosition * self);
int wxGBPosition_GetRow(const wxGBPosition * self);
void wxGBPosition_SetCol(wxGBPosition * self, int col);
void wxGBPosition_SetRow(wxGBPosition * self, int row);

// CLASS: wxGBSizerItem
wxClassInfo *wxGBSizerItem_CLASSINFO();
wxGBSizerItem *wxGBSizerItem_new(int width, int height, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data);
wxGBSizerItem *wxGBSizerItem_new1(wxWindow * window, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data);
wxGBSizerItem *wxGBSizerItem_new2(wxSizer * sizer, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data);
void wxGBSizerItem_GetEndPos(wxGBSizerItem * self, int * row, int * col);
wxGBPosition *wxGBSizerItem_GetPos(const wxGBSizerItem * self);
void wxGBSizerItem_GetPos1(const wxGBSizerItem * self, int * row, int * col);
wxGBSpan *wxGBSizerItem_GetSpan(const wxGBSizerItem * self);
void wxGBSizerItem_GetSpan1(const wxGBSizerItem * self, int * rowspan, int * colspan);
bool wxGBSizerItem_Intersects(wxGBSizerItem * self, const wxGBSizerItem * other);
bool wxGBSizerItem_Intersects1(wxGBSizerItem * self, const wxGBPosition * pos, const wxGBSpan * span);
bool wxGBSizerItem_SetPos(wxGBSizerItem * self, const wxGBPosition * pos);
bool wxGBSizerItem_SetSpan(wxGBSizerItem * self, const wxGBSpan * span);
wxGridBagSizer * wxGBSizerItem_GetGBSizer(const wxGBSizerItem * self);
void wxGBSizerItem_SetGBSizer(wxGBSizerItem * self, wxGridBagSizer * sizer);

// CLASS: wxGBSpan
void wxGBSpan_delete(wxGBSpan *self);
wxGBSpan *wxGBSpan_new();
wxGBSpan *wxGBSpan_new1(int rowspan, int colspan);
int wxGBSpan_GetColspan(const wxGBSpan * self);
int wxGBSpan_GetRowspan(const wxGBSpan * self);
void wxGBSpan_SetColspan(wxGBSpan * self, int colspan);
void wxGBSpan_SetRowspan(wxGBSpan * self, int rowspan);

// CLASS: wxGDIObject
wxClassInfo *wxGDIObject_CLASSINFO();

// CLASS: wxGIFHandler
wxClassInfo *wxGIFHandler_CLASSINFO();
wxGIFHandler *wxGIFHandler_new();
bool wxGIFHandler_SaveAnimation(wxGIFHandler * self, const wxImageArray * images, wxOutputStream * stream, bool verbose, int delay_milli_secs);

// CLASS: wxGauge
wxClassInfo *wxGauge_CLASSINFO();
wxGauge *wxGauge_new();
wxGauge *wxGauge_new1(wxWindow * parent, wxWindowID id, int range, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxGauge_Create(wxGauge * self, wxWindow * parent, wxWindowID id, int range, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
int wxGauge_GetRange(const wxGauge * self);
int wxGauge_GetValue(const wxGauge * self);
bool wxGauge_IsVertical(const wxGauge * self);
void wxGauge_Pulse(wxGauge * self);
void wxGauge_SetRange(wxGauge * self, int range);
void wxGauge_SetValue(wxGauge * self, int pos);

// CLASS: wxGenericAboutDialog
void wxGenericAboutDialog_delete(wxGenericAboutDialog *self);
wxGenericAboutDialog *wxGenericAboutDialog_new();
wxGenericAboutDialog *wxGenericAboutDialog_new1(const wxAboutDialogInfo * info, wxWindow * parent);
bool wxGenericAboutDialog_Create(wxGenericAboutDialog * self, const wxAboutDialogInfo * info, wxWindow * parent);

// CLASS: wxGenericAnimationCtrl
wxClassInfo *wxGenericAnimationCtrl_CLASSINFO();
wxGenericAnimationCtrl *wxGenericAnimationCtrl_new(wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxGenericAnimationCtrl_Create(wxGenericAnimationCtrl * self, wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxGenericAnimationCtrl_DrawCurrentFrame(wxGenericAnimationCtrl * self, wxDC * dc);
wxBitmap * wxGenericAnimationCtrl_GetBackingStore(wxGenericAnimationCtrl * self);
bool wxGenericAnimationCtrl_Play(wxGenericAnimationCtrl * self, bool looped);
void wxGenericAnimationCtrl_SetUseWindowBackgroundColour(wxGenericAnimationCtrl * self, bool use_win_background);
bool wxGenericAnimationCtrl_IsUsingWindowBackgroundColour(const wxGenericAnimationCtrl * self);

// CLASS: wxGenericDirCtrl
wxClassInfo *wxGenericDirCtrl_CLASSINFO();
wxGenericDirCtrl *wxGenericDirCtrl_new();
wxGenericDirCtrl *wxGenericDirCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * dir, const wxPoint * pos, const wxSize * size, long style, const wxString * filter, int default_filter, const wxString * name);
bool wxGenericDirCtrl_CollapsePath(wxGenericDirCtrl * self, const wxString * path);
void wxGenericDirCtrl_CollapseTree(wxGenericDirCtrl * self);
bool wxGenericDirCtrl_Create(wxGenericDirCtrl * self, wxWindow * parent, wxWindowID id, const wxString * dir, const wxPoint * pos, const wxSize * size, long style, const wxString * filter, int default_filter, const wxString * name);
bool wxGenericDirCtrl_ExpandPath(wxGenericDirCtrl * self, const wxString * path);
wxString *wxGenericDirCtrl_GetDefaultPath(const wxGenericDirCtrl * self);
wxString *wxGenericDirCtrl_GetFilePath(const wxGenericDirCtrl * self);
void wxGenericDirCtrl_GetFilePaths(const wxGenericDirCtrl * self, wxArrayString * paths);
wxString *wxGenericDirCtrl_GetFilter(const wxGenericDirCtrl * self);
int wxGenericDirCtrl_GetFilterIndex(const wxGenericDirCtrl * self);
wxDirFilterListCtrl * wxGenericDirCtrl_GetFilterListCtrl(const wxGenericDirCtrl * self);
wxString *wxGenericDirCtrl_GetPath(const wxGenericDirCtrl * self);
void wxGenericDirCtrl_GetPaths(const wxGenericDirCtrl * self, wxArrayString * paths);
wxTreeCtrl * wxGenericDirCtrl_GetTreeCtrl(const wxGenericDirCtrl * self);
void wxGenericDirCtrl_Init(wxGenericDirCtrl * self);
void wxGenericDirCtrl_ReCreateTree(wxGenericDirCtrl * self);
void wxGenericDirCtrl_SetDefaultPath(wxGenericDirCtrl * self, const wxString * path);
void wxGenericDirCtrl_SetFilter(wxGenericDirCtrl * self, const wxString * filter);
void wxGenericDirCtrl_SetFilterIndex(wxGenericDirCtrl * self, int n);
void wxGenericDirCtrl_SetPath(wxGenericDirCtrl * self, const wxString * path);
void wxGenericDirCtrl_ShowHidden(wxGenericDirCtrl * self, bool show);
void wxGenericDirCtrl_SelectPath(wxGenericDirCtrl * self, const wxString * path, bool select);
void wxGenericDirCtrl_SelectPaths(wxGenericDirCtrl * self, const wxArrayString * paths);
void wxGenericDirCtrl_UnselectAll(wxGenericDirCtrl * self);

// CLASS: wxGenericProgressDialog
wxClassInfo *wxGenericProgressDialog_CLASSINFO();
wxGenericProgressDialog *wxGenericProgressDialog_new(const wxString * title, const wxString * message, int maximum, wxWindow * parent, int style);
int wxGenericProgressDialog_GetValue(const wxGenericProgressDialog * self);
int wxGenericProgressDialog_GetRange(const wxGenericProgressDialog * self);
wxString *wxGenericProgressDialog_GetMessage(const wxGenericProgressDialog * self);
bool wxGenericProgressDialog_Pulse(wxGenericProgressDialog * self, const wxString * newmsg, bool * skip);
void wxGenericProgressDialog_Resume(wxGenericProgressDialog * self);
void wxGenericProgressDialog_SetRange(wxGenericProgressDialog * self, int maximum);
bool wxGenericProgressDialog_WasCancelled(const wxGenericProgressDialog * self);
bool wxGenericProgressDialog_WasSkipped(const wxGenericProgressDialog * self);
bool wxGenericProgressDialog_Update(wxGenericProgressDialog * self, int value, const wxString * newmsg, bool * skip);

// CLASS: wxGenericValidator
wxClassInfo *wxGenericValidator_CLASSINFO();
wxGenericValidator *wxGenericValidator_new(const wxGenericValidator * validator);
wxGenericValidator *wxGenericValidator_new1(bool * val_ptr);
wxGenericValidator *wxGenericValidator_new2(wxString * val_ptr);
wxGenericValidator *wxGenericValidator_new3(int * val_ptr);
wxGenericValidator *wxGenericValidator_new4(wxArrayInt * val_ptr);
wxGenericValidator *wxGenericValidator_new5(wxDateTime * val_ptr);
wxGenericValidator *wxGenericValidator_new6(wxFileName * val_ptr);
wxGenericValidator *wxGenericValidator_new7(float * val_ptr);
wxGenericValidator *wxGenericValidator_new8(double * val_ptr);

// CLASS: wxGestureEvent
wxClassInfo *wxGestureEvent_CLASSINFO();
wxPoint *wxGestureEvent_GetPosition(const wxGestureEvent * self);
bool wxGestureEvent_IsGestureStart(const wxGestureEvent * self);
bool wxGestureEvent_IsGestureEnd(const wxGestureEvent * self);
void wxGestureEvent_SetPosition(wxGestureEvent * self, const wxPoint * pos);
void wxGestureEvent_SetGestureStart(wxGestureEvent * self, bool is_start);
void wxGestureEvent_SetGestureEnd(wxGestureEvent * self, bool is_end);

// CLASS: wxGraphicsBrush
wxClassInfo *wxGraphicsBrush_CLASSINFO();

// CLASS: wxGraphicsContext
wxClassInfo *wxGraphicsContext_CLASSINFO();
wxGraphicsContext * wxGraphicsContext_Create(wxWindow * window);
wxGraphicsContext * wxGraphicsContext_Create1(const wxWindowDC * window_dc);
wxGraphicsContext * wxGraphicsContext_Create2(const wxMemoryDC * memory_dc);
wxGraphicsContext * wxGraphicsContext_Create3(const wxPrinterDC * printer_dc);
#ifdef __WXMSW__
wxGraphicsContext * wxGraphicsContext_Create4(const wxEnhMetaFileDC * meta_file_dc);
#endif
wxGraphicsContext * wxGraphicsContext_CreateFromUnknownDC(wxDC * dc);
wxGraphicsContext * wxGraphicsContext_Create5(wxImage * image);
wxGraphicsContext * wxGraphicsContext_CreateFromNative(void * context);
wxGraphicsContext * wxGraphicsContext_CreateFromNativeWindow(void * window);
wxGraphicsContext * wxGraphicsContext_Create6();
void wxGraphicsContext_ResetClip(wxGraphicsContext * self);
void wxGraphicsContext_Clip(wxGraphicsContext * self, const wxRegion * region);
void wxGraphicsContext_GetClipBox(wxGraphicsContext * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h);
wxGraphicsMatrix *wxGraphicsContext_CreateMatrix1(const wxGraphicsContext * self, const wxAffineMatrix2DBase * mat);
void wxGraphicsContext_ConcatTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix);
wxGraphicsMatrix *wxGraphicsContext_GetTransform(const wxGraphicsContext * self);
void wxGraphicsContext_SetTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix);
wxGraphicsBrush *wxGraphicsContext_CreateBrush(const wxGraphicsContext * self, const wxBrush * brush);
void wxGraphicsContext_SetBrush(wxGraphicsContext * self, const wxBrush * brush);
void wxGraphicsContext_SetBrush1(wxGraphicsContext * self, const wxGraphicsBrush * brush);
wxGraphicsPen *wxGraphicsContext_CreatePen(const wxGraphicsContext * self, const wxPen * pen);
wxGraphicsPen *wxGraphicsContext_CreatePen1(const wxGraphicsContext * self, const wxGraphicsPenInfo * info);
void wxGraphicsContext_SetPen(wxGraphicsContext * self, const wxPen * pen);
void wxGraphicsContext_SetPen1(wxGraphicsContext * self, const wxGraphicsPen * pen);
wxGraphicsPath *wxGraphicsContext_CreatePath(const wxGraphicsContext * self);
void wxGraphicsContext_StrokeLines(wxGraphicsContext * self, size_t n, const wxPoint2DDouble * begin_points, const wxPoint2DDouble * end_points);
void wxGraphicsContext_StrokeLines1(wxGraphicsContext * self, size_t n, const wxPoint2DDouble * points);
void wxGraphicsContext_StrokePath(wxGraphicsContext * self, const wxGraphicsPath * path);
wxGraphicsFont *wxGraphicsContext_CreateFont(const wxGraphicsContext * self, const wxFont * font, const wxColour * col);
wxGraphicsFont *wxGraphicsContext_CreateFont1(const wxGraphicsContext * self, double size_in_pixels, const wxString * facename, int flags, const wxColour * col);
void wxGraphicsContext_SetFont(wxGraphicsContext * self, const wxFont * font, const wxColour * colour);
void wxGraphicsContext_SetFont1(wxGraphicsContext * self, const wxGraphicsFont * font);
void wxGraphicsContext_GetPartialTextExtents(const wxGraphicsContext * self, const wxString * text, wxArrayDouble * widths);
void wxGraphicsContext_GetTextExtent(const wxGraphicsContext * self, const wxString * text, wxDouble * width, wxDouble * height, wxDouble * descent, wxDouble * external_leading);
bool wxGraphicsContext_StartDoc(wxGraphicsContext * self, const wxString * message);
void wxGraphicsContext_EndDoc(wxGraphicsContext * self);
void wxGraphicsContext_EndPage(wxGraphicsContext * self);
void wxGraphicsContext_EndLayer(wxGraphicsContext * self);
void wxGraphicsContext_PushState(wxGraphicsContext * self);
void wxGraphicsContext_PopState(wxGraphicsContext * self);
void wxGraphicsContext_Flush(wxGraphicsContext * self);
void * wxGraphicsContext_GetNativeContext(wxGraphicsContext * self);
void wxGraphicsContext_GetSize(const wxGraphicsContext * self, wxDouble * width, wxDouble * height);
void wxGraphicsContext_GetDPI(const wxGraphicsContext * self, wxDouble * dpi_x, wxDouble * dpi_y);
wxWindow * wxGraphicsContext_GetWindow(const wxGraphicsContext * self);
bool wxGraphicsContext_ShouldOffset(const wxGraphicsContext * self);
void wxGraphicsContext_EnableOffset(wxGraphicsContext * self, bool enable);
void wxGraphicsContext_DisableOffset(wxGraphicsContext * self);
bool wxGraphicsContext_OffsetEnabled(const wxGraphicsContext * self);
wxSize *wxGraphicsContext_FromDIP(const wxGraphicsContext * self, const wxSize * sz);
wxPoint *wxGraphicsContext_FromDIP1(const wxGraphicsContext * self, const wxPoint * pt);
int wxGraphicsContext_FromDIP2(const wxGraphicsContext * self, int d);
wxSize *wxGraphicsContext_ToDIP(const wxGraphicsContext * self, const wxSize * sz);
wxPoint *wxGraphicsContext_ToDIP1(const wxGraphicsContext * self, const wxPoint * pt);
int wxGraphicsContext_ToDIP2(const wxGraphicsContext * self, int d);

// CLASS: wxGraphicsFont
wxClassInfo *wxGraphicsFont_CLASSINFO();

// CLASS: wxGraphicsGradientStop
void wxGraphicsGradientStop_delete(wxGraphicsGradientStop *self);
wxColour *wxGraphicsGradientStop_GetColour(const wxGraphicsGradientStop * self);
void wxGraphicsGradientStop_SetColour(wxGraphicsGradientStop * self, const wxColour * col);

// CLASS: wxGraphicsGradientStops
void wxGraphicsGradientStops_delete(wxGraphicsGradientStops *self);
void wxGraphicsGradientStops_Add(wxGraphicsGradientStops * self, const wxGraphicsGradientStop * stop);
size_t wxGraphicsGradientStops_GetCount(const wxGraphicsGradientStops * self);
wxColour *wxGraphicsGradientStops_GetStartColour(const wxGraphicsGradientStops * self);
wxColour *wxGraphicsGradientStops_GetEndColour(const wxGraphicsGradientStops * self);

// CLASS: wxGraphicsMatrix
wxClassInfo *wxGraphicsMatrix_CLASSINFO();
void wxGraphicsMatrix_Concat(wxGraphicsMatrix * self, const wxGraphicsMatrix * t);
void wxGraphicsMatrix_Get(const wxGraphicsMatrix * self, wxDouble * a, wxDouble * b, wxDouble * c, wxDouble * d, wxDouble * tx, wxDouble * ty);
void * wxGraphicsMatrix_GetNativeMatrix(const wxGraphicsMatrix * self);
void wxGraphicsMatrix_Invert(wxGraphicsMatrix * self);
bool wxGraphicsMatrix_IsEqual1(const wxGraphicsMatrix * self, const wxGraphicsMatrix * t);
bool wxGraphicsMatrix_IsIdentity(const wxGraphicsMatrix * self);
void wxGraphicsMatrix_TransformDistance(const wxGraphicsMatrix * self, wxDouble * dx, wxDouble * dy);
void wxGraphicsMatrix_TransformPoint(const wxGraphicsMatrix * self, wxDouble * x, wxDouble * y);

// CLASS: wxGraphicsObject
wxClassInfo *wxGraphicsObject_CLASSINFO();
wxGraphicsRenderer * wxGraphicsObject_GetRenderer(const wxGraphicsObject * self);
bool wxGraphicsObject_IsNull(const wxGraphicsObject * self);

// CLASS: wxGraphicsPath
wxClassInfo *wxGraphicsPath_CLASSINFO();
void wxGraphicsPath_AddCurveToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * c1, const wxPoint2DDouble * c2, const wxPoint2DDouble * e);
void wxGraphicsPath_AddLineToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * p);
void wxGraphicsPath_AddPath(wxGraphicsPath * self, const wxGraphicsPath * path);
void wxGraphicsPath_CloseSubpath(wxGraphicsPath * self);
void wxGraphicsPath_GetBox1(const wxGraphicsPath * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h);
void wxGraphicsPath_GetCurrentPoint(const wxGraphicsPath * self, wxDouble * x, wxDouble * y);
void * wxGraphicsPath_GetNativePath(const wxGraphicsPath * self);
void wxGraphicsPath_MoveToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * p);
void wxGraphicsPath_Transform(wxGraphicsPath * self, const wxGraphicsMatrix * matrix);
void wxGraphicsPath_UnGetNativePath(const wxGraphicsPath * self, void * p);

// CLASS: wxGraphicsPen
wxClassInfo *wxGraphicsPen_CLASSINFO();

// CLASS: wxGraphicsRenderer
wxClassInfo *wxGraphicsRenderer_CLASSINFO();
wxGraphicsContext * wxGraphicsRenderer_CreateContext(wxGraphicsRenderer * self, wxWindow * window);
wxGraphicsContext * wxGraphicsRenderer_CreateContext1(wxGraphicsRenderer * self, const wxWindowDC * window_dc);
wxGraphicsContext * wxGraphicsRenderer_CreateContext2(wxGraphicsRenderer * self, const wxMemoryDC * memory_dc);
wxGraphicsContext * wxGraphicsRenderer_CreateContext3(wxGraphicsRenderer * self, const wxPrinterDC * printer_dc);
#ifdef __WXMSW__
wxGraphicsContext * wxGraphicsRenderer_CreateContext4(wxGraphicsRenderer * self, const wxEnhMetaFileDC * meta_file_dc);
#endif
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromUnknownDC(wxGraphicsRenderer * self, wxDC * dc);
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromImage(wxGraphicsRenderer * self, wxImage * image);
wxGraphicsBrush *wxGraphicsRenderer_CreateBrush(wxGraphicsRenderer * self, const wxBrush * brush);
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromNativeContext(wxGraphicsRenderer * self, void * context);
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromNativeWindow(wxGraphicsRenderer * self, void * window);
wxGraphicsContext * wxGraphicsRenderer_CreateMeasuringContext(wxGraphicsRenderer * self);
wxGraphicsFont *wxGraphicsRenderer_CreateFont(wxGraphicsRenderer * self, const wxFont * font, const wxColour * col);
wxGraphicsFont *wxGraphicsRenderer_CreateFont1(wxGraphicsRenderer * self, double size_in_pixels, const wxString * facename, int flags, const wxColour * col);
wxGraphicsFont *wxGraphicsRenderer_CreateFontAtDPI(wxGraphicsRenderer * self, const wxFont * font, const wxRealPoint * dpi, const wxColour * col);
wxGraphicsPath *wxGraphicsRenderer_CreatePath(wxGraphicsRenderer * self);
wxGraphicsPen *wxGraphicsRenderer_CreatePen(wxGraphicsRenderer * self, const wxGraphicsPenInfo * info);
wxString *wxGraphicsRenderer_GetName(const wxGraphicsRenderer * self);
void wxGraphicsRenderer_GetVersion(const wxGraphicsRenderer * self, int * major, int * minor, int * micro);
wxGraphicsRenderer * wxGraphicsRenderer_GetDefaultRenderer();
wxGraphicsRenderer * wxGraphicsRenderer_GetCairoRenderer();
#ifdef __WXMSW__
wxGraphicsRenderer * wxGraphicsRenderer_GetGDIPlusRenderer();
wxGraphicsRenderer * wxGraphicsRenderer_GetDirect2DRenderer();
#endif

// CLASS: wxGridBagSizer
wxClassInfo *wxGridBagSizer_CLASSINFO();
wxGridBagSizer *wxGridBagSizer_new(int vgap, int hgap);
wxSizerItem * wxGridBagSizer_Add(wxGridBagSizer * self, wxWindow * window, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data);
wxSizerItem * wxGridBagSizer_Add1(wxGridBagSizer * self, wxSizer * sizer, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data);
wxSizerItem * wxGridBagSizer_Add2(wxGridBagSizer * self, wxGBSizerItem * item);
wxSizerItem * wxGridBagSizer_Add3(wxGridBagSizer * self, int width, int height, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data);
bool wxGridBagSizer_CheckForIntersection(wxGridBagSizer * self, wxGBSizerItem * item, wxGBSizerItem * exclude_item);
bool wxGridBagSizer_CheckForIntersection1(wxGridBagSizer * self, const wxGBPosition * pos, const wxGBSpan * span, wxGBSizerItem * exclude_item);
wxGBSizerItem * wxGridBagSizer_FindItem(wxGridBagSizer * self, wxWindow * window);
wxGBSizerItem * wxGridBagSizer_FindItem1(wxGridBagSizer * self, wxSizer * sizer);
wxGBSizerItem * wxGridBagSizer_FindItemAtPoint(wxGridBagSizer * self, const wxPoint * pt);
wxGBSizerItem * wxGridBagSizer_FindItemAtPosition(wxGridBagSizer * self, const wxGBPosition * pos);
wxGBSizerItem * wxGridBagSizer_FindItemWithData(wxGridBagSizer * self, const wxObject * user_data);
wxSize *wxGridBagSizer_GetCellSize(const wxGridBagSizer * self, int row, int col);
wxSize *wxGridBagSizer_GetEmptyCellSize(const wxGridBagSizer * self);
wxGBPosition *wxGridBagSizer_GetItemPosition(wxGridBagSizer * self, wxWindow * window);
wxGBPosition *wxGridBagSizer_GetItemPosition1(wxGridBagSizer * self, wxSizer * sizer);
wxGBPosition *wxGridBagSizer_GetItemPosition2(wxGridBagSizer * self, size_t index);
wxGBSpan *wxGridBagSizer_GetItemSpan(wxGridBagSizer * self, wxWindow * window);
wxGBSpan *wxGridBagSizer_GetItemSpan1(wxGridBagSizer * self, wxSizer * sizer);
wxGBSpan *wxGridBagSizer_GetItemSpan2(wxGridBagSizer * self, size_t index);
void wxGridBagSizer_SetEmptyCellSize(wxGridBagSizer * self, const wxSize * sz);
bool wxGridBagSizer_SetItemPosition(wxGridBagSizer * self, wxWindow * window, const wxGBPosition * pos);
bool wxGridBagSizer_SetItemPosition1(wxGridBagSizer * self, wxSizer * sizer, const wxGBPosition * pos);
bool wxGridBagSizer_SetItemPosition2(wxGridBagSizer * self, size_t index, const wxGBPosition * pos);
bool wxGridBagSizer_SetItemSpan(wxGridBagSizer * self, wxWindow * window, const wxGBSpan * span);
bool wxGridBagSizer_SetItemSpan1(wxGridBagSizer * self, wxSizer * sizer, const wxGBSpan * span);
bool wxGridBagSizer_SetItemSpan2(wxGridBagSizer * self, size_t index, const wxGBSpan * span);

// CLASS: wxGridCellAttr
wxGridCellAttr *wxGridCellAttr_new(wxGridCellAttr * attr_default);
wxGridCellAttr *wxGridCellAttr_new1(const wxColour * col_text, const wxColour * col_back, const wxFont * font, int h_align, int v_align);
wxGridCellAttr * wxGridCellAttr_Clone(const wxGridCellAttr * self);
void wxGridCellAttr_DecRef(wxGridCellAttr * self);
void wxGridCellAttr_GetAlignment(const wxGridCellAttr * self, int * h_align, int * v_align);
wxColour *wxGridCellAttr_GetBackgroundColour(const wxGridCellAttr * self);
wxGridCellEditor * wxGridCellAttr_GetEditor(const wxGridCellAttr * self, const wxGrid * grid, int row, int col);
wxFont *wxGridCellAttr_GetFont(const wxGridCellAttr * self);
void wxGridCellAttr_GetNonDefaultAlignment(const wxGridCellAttr * self, int * h_align, int * v_align);
wxGridCellRenderer * wxGridCellAttr_GetRenderer(const wxGridCellAttr * self, const wxGrid * grid, int row, int col);
wxColour *wxGridCellAttr_GetTextColour(const wxGridCellAttr * self);
bool wxGridCellAttr_HasAlignment(const wxGridCellAttr * self);
bool wxGridCellAttr_HasBackgroundColour(const wxGridCellAttr * self);
bool wxGridCellAttr_HasEditor(const wxGridCellAttr * self);
bool wxGridCellAttr_HasFont(const wxGridCellAttr * self);
bool wxGridCellAttr_HasRenderer(const wxGridCellAttr * self);
bool wxGridCellAttr_HasTextColour(const wxGridCellAttr * self);
void wxGridCellAttr_IncRef(wxGridCellAttr * self);
bool wxGridCellAttr_IsReadOnly(const wxGridCellAttr * self);
void wxGridCellAttr_SetAlignment(wxGridCellAttr * self, int h_align, int v_align);
void wxGridCellAttr_SetBackgroundColour(wxGridCellAttr * self, const wxColour * col_back);
void wxGridCellAttr_SetDefAttr(wxGridCellAttr * self, wxGridCellAttr * def_attr);
void wxGridCellAttr_SetEditor(wxGridCellAttr * self, wxGridCellEditor * editor);
void wxGridCellAttr_SetFont(wxGridCellAttr * self, const wxFont * font);
void wxGridCellAttr_SetReadOnly(wxGridCellAttr * self, bool is_read_only);
void wxGridCellAttr_SetRenderer(wxGridCellAttr * self, wxGridCellRenderer * renderer);
void wxGridCellAttr_SetTextColour(wxGridCellAttr * self, const wxColour * col_text);
void wxGridCellAttr_MergeWith(wxGridCellAttr * self, wxGridCellAttr * mergefrom);
void wxGridCellAttr_SetSize(wxGridCellAttr * self, int num_rows, int num_cols);
void wxGridCellAttr_SetOverflow(wxGridCellAttr * self, bool allow);
bool wxGridCellAttr_HasReadWriteMode(const wxGridCellAttr * self);
bool wxGridCellAttr_HasOverflowMode(const wxGridCellAttr * self);
bool wxGridCellAttr_HasSize(const wxGridCellAttr * self);
void wxGridCellAttr_GetSize(const wxGridCellAttr * self, int * num_rows, int * num_cols);
wxGridFitMode *wxGridCellAttr_GetFitMode(const wxGridCellAttr * self);
bool wxGridCellAttr_GetOverflow(const wxGridCellAttr * self);
bool wxGridCellAttr_CanOverflow(const wxGridCellAttr * self);
// Mix-in(s) to wxGridCellAttr
wxRefCounter *wxGridCellAttr_AsRefCounter(wxGridCellAttr* obj);

// CLASS: wxGridCellAutoWrapStringEditor
wxGridCellAutoWrapStringEditor *wxGridCellAutoWrapStringEditor_new();
// Mix-in(s) to wxGridCellAutoWrapStringEditor
wxRefCounter *wxGridCellAutoWrapStringEditor_AsRefCounter(wxGridCellAutoWrapStringEditor* obj);

// CLASS: wxGridCellAutoWrapStringRenderer
wxGridCellAutoWrapStringRenderer *wxGridCellAutoWrapStringRenderer_new();
// Mix-in(s) to wxGridCellAutoWrapStringRenderer
wxRefCounter *wxGridCellAutoWrapStringRenderer_AsRefCounter(wxGridCellAutoWrapStringRenderer* obj);

// CLASS: wxGridCellBoolEditor
wxGridCellBoolEditor *wxGridCellBoolEditor_new();
bool wxGridCellBoolEditor_IsTrueValue(const wxString * value);
void wxGridCellBoolEditor_UseStringValues(const wxString * value_true, const wxString * value_false);
// Mix-in(s) to wxGridCellBoolEditor
wxRefCounter *wxGridCellBoolEditor_AsRefCounter(wxGridCellBoolEditor* obj);

// CLASS: wxGridCellBoolRenderer
wxGridCellBoolRenderer *wxGridCellBoolRenderer_new();
// Mix-in(s) to wxGridCellBoolRenderer
wxRefCounter *wxGridCellBoolRenderer_AsRefCounter(wxGridCellBoolRenderer* obj);

// CLASS: wxGridCellChoiceEditor
wxGridCellChoiceEditor *wxGridCellChoiceEditor_new1(const wxArrayString * choices, bool allow_others);
void wxGridCellChoiceEditor_SetParameters(wxGridCellChoiceEditor * self, const wxString * params);
// Mix-in(s) to wxGridCellChoiceEditor
wxRefCounter *wxGridCellChoiceEditor_AsRefCounter(wxGridCellChoiceEditor* obj);

// CLASS: wxGridCellDateEditor
wxGridCellDateEditor *wxGridCellDateEditor_new(const wxString * format);
// Mix-in(s) to wxGridCellDateEditor
wxRefCounter *wxGridCellDateEditor_AsRefCounter(wxGridCellDateEditor* obj);

// CLASS: wxGridCellDateRenderer
wxGridCellDateRenderer *wxGridCellDateRenderer_new(const wxString * outformat);
void wxGridCellDateRenderer_SetParameters(wxGridCellDateRenderer * self, const wxString * params);
// Mix-in(s) to wxGridCellDateRenderer
wxRefCounter *wxGridCellDateRenderer_AsRefCounter(wxGridCellDateRenderer* obj);

// CLASS: wxGridCellDateTimeRenderer
wxGridCellDateTimeRenderer *wxGridCellDateTimeRenderer_new(const wxString * outformat, const wxString * informat);
// Mix-in(s) to wxGridCellDateTimeRenderer
wxRefCounter *wxGridCellDateTimeRenderer_AsRefCounter(wxGridCellDateTimeRenderer* obj);

// CLASS: wxGridCellEditor
void wxGridCellEditor_BeginEdit(wxGridCellEditor * self, int row, int col, wxGrid * grid);
wxGridCellEditor * wxGridCellEditor_Clone(const wxGridCellEditor * self);
void wxGridCellEditor_Create(wxGridCellEditor * self, wxWindow * parent, wxWindowID id, wxEvtHandler * evt_handler);
void wxGridCellEditor_Destroy(wxGridCellEditor * self);
bool wxGridCellEditor_EndEdit(wxGridCellEditor * self, int row, int col, const wxGrid * grid, const wxString * oldval, wxString * newval);
void wxGridCellEditor_ApplyEdit(wxGridCellEditor * self, int row, int col, wxGrid * grid);
void wxGridCellEditor_HandleReturn(wxGridCellEditor * self, wxKeyEvent * event);
bool wxGridCellEditor_IsCreated(wxGridCellEditor * self);
void wxGridCellEditor_PaintBackground(wxGridCellEditor * self, wxDC * dc, const wxRect * rect_cell, const wxGridCellAttr * attr);
void wxGridCellEditor_Reset(wxGridCellEditor * self);
void wxGridCellEditor_SetSize(wxGridCellEditor * self, const wxRect * rect);
void wxGridCellEditor_Show(wxGridCellEditor * self, bool show, wxGridCellAttr * attr);
void wxGridCellEditor_StartingClick(wxGridCellEditor * self);
void wxGridCellEditor_StartingKey(wxGridCellEditor * self, wxKeyEvent * event);
bool wxGridCellEditor_IsAcceptedKey(wxGridCellEditor * self, wxKeyEvent * event);
wxString *wxGridCellEditor_GetValue(const wxGridCellEditor * self);
wxWindow * wxGridCellEditor_GetWindow(const wxGridCellEditor * self);
void wxGridCellEditor_SetWindow(wxGridCellEditor * self, wxWindow * window);
wxControl * wxGridCellEditor_GetControl(wxGridCellEditor * self);
void wxGridCellEditor_SetControl(wxGridCellEditor * self, wxControl * control);
void wxGridCellEditor_DoActivate(wxGridCellEditor * self, int row, int col, wxGrid * grid);
// Mix-in(s) to wxGridCellEditor
wxRefCounter *wxGridCellEditor_AsRefCounter(wxGridCellEditor* obj);

// CLASS: wxGridCellEnumEditor
wxGridCellEnumEditor *wxGridCellEnumEditor_new(const wxString * choices);
// Mix-in(s) to wxGridCellEnumEditor
wxRefCounter *wxGridCellEnumEditor_AsRefCounter(wxGridCellEnumEditor* obj);

// CLASS: wxGridCellEnumRenderer
wxGridCellEnumRenderer *wxGridCellEnumRenderer_new(const wxString * choices);
void wxGridCellEnumRenderer_SetParameters(wxGridCellEnumRenderer * self, const wxString * params);
// Mix-in(s) to wxGridCellEnumRenderer
wxRefCounter *wxGridCellEnumRenderer_AsRefCounter(wxGridCellEnumRenderer* obj);

// CLASS: wxGridCellFloatEditor
wxGridCellFloatEditor *wxGridCellFloatEditor_new(int width, int precision, int format);
// Mix-in(s) to wxGridCellFloatEditor
wxRefCounter *wxGridCellFloatEditor_AsRefCounter(wxGridCellFloatEditor* obj);

// CLASS: wxGridCellFloatRenderer
wxGridCellFloatRenderer *wxGridCellFloatRenderer_new(int width, int precision, int format);
int wxGridCellFloatRenderer_GetFormat(const wxGridCellFloatRenderer * self);
int wxGridCellFloatRenderer_GetPrecision(const wxGridCellFloatRenderer * self);
int wxGridCellFloatRenderer_GetWidth(const wxGridCellFloatRenderer * self);
void wxGridCellFloatRenderer_SetFormat(wxGridCellFloatRenderer * self, int format);
void wxGridCellFloatRenderer_SetParameters(wxGridCellFloatRenderer * self, const wxString * params);
void wxGridCellFloatRenderer_SetPrecision(wxGridCellFloatRenderer * self, int precision);
void wxGridCellFloatRenderer_SetWidth(wxGridCellFloatRenderer * self, int width);
// Mix-in(s) to wxGridCellFloatRenderer
wxRefCounter *wxGridCellFloatRenderer_AsRefCounter(wxGridCellFloatRenderer* obj);

// CLASS: wxGridCellNumberEditor
wxGridCellNumberEditor *wxGridCellNumberEditor_new(int min, int max);
// Mix-in(s) to wxGridCellNumberEditor
wxRefCounter *wxGridCellNumberEditor_AsRefCounter(wxGridCellNumberEditor* obj);

// CLASS: wxGridCellNumberRenderer
wxGridCellNumberRenderer *wxGridCellNumberRenderer_new();
// Mix-in(s) to wxGridCellNumberRenderer
wxRefCounter *wxGridCellNumberRenderer_AsRefCounter(wxGridCellNumberRenderer* obj);

// CLASS: wxGridCellRenderer
wxGridCellRenderer * wxGridCellRenderer_Clone(const wxGridCellRenderer * self);
void wxGridCellRenderer_Draw(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, const wxRect * rect, int row, int col, bool is_selected);
wxSize *wxGridCellRenderer_GetBestSize(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col);
int wxGridCellRenderer_GetBestHeight(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col, int width);
int wxGridCellRenderer_GetBestWidth(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col, int height);
wxSize *wxGridCellRenderer_GetMaxBestSize(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc);
// Mix-in(s) to wxGridCellRenderer
wxRefCounter *wxGridCellRenderer_AsRefCounter(wxGridCellRenderer* obj);

// CLASS: wxGridCellStringRenderer
wxGridCellStringRenderer *wxGridCellStringRenderer_new();
// Mix-in(s) to wxGridCellStringRenderer
wxRefCounter *wxGridCellStringRenderer_AsRefCounter(wxGridCellStringRenderer* obj);

// CLASS: wxGridCellTextEditor
wxGridCellTextEditor *wxGridCellTextEditor_new(size_t max_chars);
void wxGridCellTextEditor_SetParameters(wxGridCellTextEditor * self, const wxString * params);
void wxGridCellTextEditor_SetValidator(wxGridCellTextEditor * self, const wxValidator * validator);
// Mix-in(s) to wxGridCellTextEditor
wxRefCounter *wxGridCellTextEditor_AsRefCounter(wxGridCellTextEditor* obj);

// CLASS: wxGridEditorCreatedEvent
wxClassInfo *wxGridEditorCreatedEvent_CLASSINFO();
wxGridEditorCreatedEvent *wxGridEditorCreatedEvent_new();
int wxGridEditorCreatedEvent_GetCol(const wxGridEditorCreatedEvent * self);
wxControl * wxGridEditorCreatedEvent_GetControl(wxGridEditorCreatedEvent * self);
int wxGridEditorCreatedEvent_GetRow(const wxGridEditorCreatedEvent * self);
wxWindow * wxGridEditorCreatedEvent_GetWindow(const wxGridEditorCreatedEvent * self);
void wxGridEditorCreatedEvent_SetCol(wxGridEditorCreatedEvent * self, int col);
void wxGridEditorCreatedEvent_SetControl(wxGridEditorCreatedEvent * self, wxControl * ctrl);
void wxGridEditorCreatedEvent_SetRow(wxGridEditorCreatedEvent * self, int row);
void wxGridEditorCreatedEvent_SetWindow(wxGridEditorCreatedEvent * self, wxWindow * window);

// CLASS: wxGridEvent
wxClassInfo *wxGridEvent_CLASSINFO();
wxGridEvent *wxGridEvent_new();
bool wxGridEvent_AltDown(const wxGridEvent * self);
bool wxGridEvent_ControlDown(const wxGridEvent * self);
int wxGridEvent_GetCol(const wxGridEvent * self);
wxPoint *wxGridEvent_GetPosition(const wxGridEvent * self);
int wxGridEvent_GetRow(const wxGridEvent * self);
bool wxGridEvent_MetaDown(const wxGridEvent * self);
bool wxGridEvent_Selecting(const wxGridEvent * self);
bool wxGridEvent_ShiftDown(const wxGridEvent * self);

// CLASS: wxGridFitMode
void wxGridFitMode_delete(wxGridFitMode *self);
wxGridFitMode *wxGridFitMode_new();
bool wxGridFitMode_IsSpecified(const wxGridFitMode * self);
bool wxGridFitMode_IsClip(const wxGridFitMode * self);
bool wxGridFitMode_IsOverflow(const wxGridFitMode * self);
wxEllipsizeMode wxGridFitMode_GetEllipsizeMode(const wxGridFitMode * self);
wxGridFitMode *wxGridFitMode_Clip();
wxGridFitMode *wxGridFitMode_Overflow();
wxGridFitMode *wxGridFitMode_Ellipsize(wxEllipsizeMode ellipsize);

// CLASS: wxGridRangeSelectEvent
wxClassInfo *wxGridRangeSelectEvent_CLASSINFO();
wxGridRangeSelectEvent *wxGridRangeSelectEvent_new();
bool wxGridRangeSelectEvent_AltDown(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_ControlDown(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetBottomRow(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetLeftCol(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetRightCol(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetTopRow(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_MetaDown(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_Selecting(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_ShiftDown(const wxGridRangeSelectEvent * self);

// CLASS: wxGridSizeEvent
wxClassInfo *wxGridSizeEvent_CLASSINFO();
wxGridSizeEvent *wxGridSizeEvent_new();
bool wxGridSizeEvent_AltDown(const wxGridSizeEvent * self);
bool wxGridSizeEvent_ControlDown(const wxGridSizeEvent * self);
wxPoint *wxGridSizeEvent_GetPosition(const wxGridSizeEvent * self);
int wxGridSizeEvent_GetRowOrCol(const wxGridSizeEvent * self);
bool wxGridSizeEvent_MetaDown(const wxGridSizeEvent * self);
bool wxGridSizeEvent_ShiftDown(const wxGridSizeEvent * self);

// CLASS: wxGridSizer
wxClassInfo *wxGridSizer_CLASSINFO();
wxGridSizer *wxGridSizer_new(int cols, int vgap, int hgap);
wxGridSizer *wxGridSizer_new1(int cols, const wxSize * gap);
wxGridSizer *wxGridSizer_new2(int rows, int cols, int vgap, int hgap);
wxGridSizer *wxGridSizer_new3(int rows, int cols, const wxSize * gap);
int wxGridSizer_GetCols(const wxGridSizer * self);
int wxGridSizer_GetRows(const wxGridSizer * self);
int wxGridSizer_GetEffectiveColsCount(const wxGridSizer * self);
int wxGridSizer_GetEffectiveRowsCount(const wxGridSizer * self);
int wxGridSizer_GetHGap(const wxGridSizer * self);
int wxGridSizer_GetVGap(const wxGridSizer * self);
void wxGridSizer_SetCols(wxGridSizer * self, int cols);
void wxGridSizer_SetHGap(wxGridSizer * self, int gap);
void wxGridSizer_SetRows(wxGridSizer * self, int rows);
void wxGridSizer_SetVGap(wxGridSizer * self, int gap);

// CLASS: wxGridTableBase
wxClassInfo *wxGridTableBase_CLASSINFO();
bool wxGridTableBase_IsEmptyCell(wxGridTableBase * self, int row, int col);
bool wxGridTableBase_IsEmpty(wxGridTableBase * self, const wxGridCellCoords * coords);
wxString *wxGridTableBase_GetValue(wxGridTableBase * self, int row, int col);
void wxGridTableBase_SetValue(wxGridTableBase * self, int row, int col, const wxString * value);
wxString *wxGridTableBase_GetTypeName(wxGridTableBase * self, int row, int col);
bool wxGridTableBase_CanGetValueAs(wxGridTableBase * self, int row, int col, const wxString * type_name);
bool wxGridTableBase_CanSetValueAs(wxGridTableBase * self, int row, int col, const wxString * type_name);
long wxGridTableBase_GetValueAsLong(wxGridTableBase * self, int row, int col);
double wxGridTableBase_GetValueAsDouble(wxGridTableBase * self, int row, int col);
bool wxGridTableBase_GetValueAsBool(wxGridTableBase * self, int row, int col);
void * wxGridTableBase_GetValueAsCustom(wxGridTableBase * self, int row, int col, const wxString * type_name);
void wxGridTableBase_SetValueAsLong(wxGridTableBase * self, int row, int col, long value);
void wxGridTableBase_SetValueAsDouble(wxGridTableBase * self, int row, int col, double value);
void wxGridTableBase_SetValueAsBool(wxGridTableBase * self, int row, int col, bool value);
void wxGridTableBase_SetValueAsCustom(wxGridTableBase * self, int row, int col, const wxString * type_name, void * value);
void wxGridTableBase_SetView(wxGridTableBase * self, wxGrid * grid);
wxGrid * wxGridTableBase_GetView(const wxGridTableBase * self);
void wxGridTableBase_Clear(wxGridTableBase * self);
bool wxGridTableBase_InsertRows(wxGridTableBase * self, size_t pos, size_t num_rows);
bool wxGridTableBase_AppendRows(wxGridTableBase * self, size_t num_rows);
bool wxGridTableBase_DeleteRows(wxGridTableBase * self, size_t pos, size_t num_rows);
bool wxGridTableBase_InsertCols(wxGridTableBase * self, size_t pos, size_t num_cols);
bool wxGridTableBase_AppendCols(wxGridTableBase * self, size_t num_cols);
bool wxGridTableBase_DeleteCols(wxGridTableBase * self, size_t pos, size_t num_cols);
wxString *wxGridTableBase_GetRowLabelValue(wxGridTableBase * self, int row);
wxString *wxGridTableBase_GetColLabelValue(wxGridTableBase * self, int col);
wxString *wxGridTableBase_GetCornerLabelValue(const wxGridTableBase * self);
void wxGridTableBase_SetRowLabelValue(wxGridTableBase * self, int row, const wxString * label);
void wxGridTableBase_SetColLabelValue(wxGridTableBase * self, int col, const wxString * label);
void wxGridTableBase_SetAttrProvider(wxGridTableBase * self, wxGridCellAttrProvider * attr_provider);
wxGridCellAttrProvider * wxGridTableBase_GetAttrProvider(const wxGridTableBase * self);
void wxGridTableBase_SetAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row, int col);
void wxGridTableBase_SetRowAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row);
void wxGridTableBase_SetColAttr(wxGridTableBase * self, wxGridCellAttr * attr, int col);
bool wxGridTableBase_CanHaveAttributes(wxGridTableBase * self);
bool wxGridTableBase_CanMeasureColUsingSameAttr(const wxGridTableBase * self, int col);
int wxGridTableBase_GetNumberRows(wxGridTableBase * self);
int wxGridTableBase_GetNumberCols(wxGridTableBase * self);
int wxGridTableBase_GetRowsCount(const wxGridTableBase * self);
int wxGridTableBase_GetColsCount(const wxGridTableBase * self);

// CLASS: wxGridUpdateLocker
void wxGridUpdateLocker_delete(wxGridUpdateLocker *self);
wxGridUpdateLocker *wxGridUpdateLocker_new(wxGrid * grid);
void wxGridUpdateLocker_Create(wxGridUpdateLocker * self, wxGrid * grid);

} // extern "C"

