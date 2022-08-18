#pragma once

#include <wx/dirctrl.h>
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
#if wxCHECK_VERSION(3, 1, 0)
wxGraphicsContext * wxGraphicsContext_CreateFromUnknownDC(wxDC * dc);
#endif
wxGraphicsContext * wxGraphicsContext_Create5(wxImage * image);
wxGraphicsContext * wxGraphicsContext_CreateFromNative(void * context);
wxGraphicsContext * wxGraphicsContext_CreateFromNativeWindow(void * window);
wxGraphicsContext * wxGraphicsContext_Create6();
void wxGraphicsContext_ResetClip(wxGraphicsContext * self);
void wxGraphicsContext_Clip(wxGraphicsContext * self, const wxRegion * region);
#if wxCHECK_VERSION(3, 1, 0)
void wxGraphicsContext_GetClipBox(wxGraphicsContext * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h);
#endif
wxGraphicsMatrix *wxGraphicsContext_CreateMatrix1(const wxGraphicsContext * self, const wxAffineMatrix2DBase * mat);
void wxGraphicsContext_ConcatTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix);
wxGraphicsMatrix *wxGraphicsContext_GetTransform(const wxGraphicsContext * self);
void wxGraphicsContext_SetTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix);
wxGraphicsBrush *wxGraphicsContext_CreateBrush(const wxGraphicsContext * self, const wxBrush * brush);
void wxGraphicsContext_SetBrush(wxGraphicsContext * self, const wxBrush * brush);
void wxGraphicsContext_SetBrush1(wxGraphicsContext * self, const wxGraphicsBrush * brush);
wxGraphicsPen *wxGraphicsContext_CreatePen(const wxGraphicsContext * self, const wxPen * pen);
#if wxCHECK_VERSION(3, 1, 0)
wxGraphicsPen *wxGraphicsContext_CreatePen1(const wxGraphicsContext * self, const wxGraphicsPenInfo * info);
#endif
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
#if wxCHECK_VERSION(3, 1, 0)
void wxGraphicsContext_GetDPI(const wxGraphicsContext * self, wxDouble * dpi_x, wxDouble * dpi_y);
wxWindow * wxGraphicsContext_GetWindow(const wxGraphicsContext * self);
#endif
bool wxGraphicsContext_ShouldOffset(const wxGraphicsContext * self);
void wxGraphicsContext_EnableOffset(wxGraphicsContext * self, bool enable);
void wxGraphicsContext_DisableOffset(wxGraphicsContext * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxGraphicsContext_OffsetEnabled(const wxGraphicsContext * self);
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxSize *wxGraphicsContext_FromDIP(const wxGraphicsContext * self, const wxSize * sz);
wxPoint *wxGraphicsContext_FromDIP1(const wxGraphicsContext * self, const wxPoint * pt);
int wxGraphicsContext_FromDIP2(const wxGraphicsContext * self, int d);
wxSize *wxGraphicsContext_ToDIP(const wxGraphicsContext * self, const wxSize * sz);
wxPoint *wxGraphicsContext_ToDIP1(const wxGraphicsContext * self, const wxPoint * pt);
int wxGraphicsContext_ToDIP2(const wxGraphicsContext * self, int d);
#endif

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
#if wxCHECK_VERSION(3, 1, 0)
wxGraphicsPen *wxGraphicsRenderer_CreatePen(wxGraphicsRenderer * self, const wxGraphicsPenInfo * info);
wxString *wxGraphicsRenderer_GetName(const wxGraphicsRenderer * self);
void wxGraphicsRenderer_GetVersion(const wxGraphicsRenderer * self, int * major, int * minor, int * micro);
#endif
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

// CLASS: wxGridEditorCreatedEvent
wxClassInfo *wxGridEditorCreatedEvent_CLASSINFO();
wxGridEditorCreatedEvent *wxGridEditorCreatedEvent_new();
#if wxCHECK_VERSION(3, 1, 0)
int wxGridEditorCreatedEvent_GetCol(const wxGridEditorCreatedEvent * self);
#endif
wxControl * wxGridEditorCreatedEvent_GetControl(wxGridEditorCreatedEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxGridEditorCreatedEvent_GetRow(const wxGridEditorCreatedEvent * self);
wxWindow * wxGridEditorCreatedEvent_GetWindow(const wxGridEditorCreatedEvent * self);
#endif
void wxGridEditorCreatedEvent_SetCol(wxGridEditorCreatedEvent * self, int col);
void wxGridEditorCreatedEvent_SetControl(wxGridEditorCreatedEvent * self, wxControl * ctrl);
void wxGridEditorCreatedEvent_SetRow(wxGridEditorCreatedEvent * self, int row);
#if wxCHECK_VERSION(3, 1, 0)
void wxGridEditorCreatedEvent_SetWindow(wxGridEditorCreatedEvent * self, wxWindow * window);
#endif

// CLASS: wxGridEvent
wxClassInfo *wxGridEvent_CLASSINFO();
wxGridEvent *wxGridEvent_new();
bool wxGridEvent_AltDown(const wxGridEvent * self);
bool wxGridEvent_ControlDown(const wxGridEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxGridEvent_GetCol(const wxGridEvent * self);
wxPoint *wxGridEvent_GetPosition(const wxGridEvent * self);
int wxGridEvent_GetRow(const wxGridEvent * self);
#endif
bool wxGridEvent_MetaDown(const wxGridEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxGridEvent_Selecting(const wxGridEvent * self);
#endif
bool wxGridEvent_ShiftDown(const wxGridEvent * self);

// CLASS: wxGridRangeSelectEvent
wxClassInfo *wxGridRangeSelectEvent_CLASSINFO();
wxGridRangeSelectEvent *wxGridRangeSelectEvent_new();
bool wxGridRangeSelectEvent_AltDown(const wxGridRangeSelectEvent * self);
bool wxGridRangeSelectEvent_ControlDown(const wxGridRangeSelectEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
int wxGridRangeSelectEvent_GetBottomRow(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetLeftCol(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetRightCol(const wxGridRangeSelectEvent * self);
int wxGridRangeSelectEvent_GetTopRow(const wxGridRangeSelectEvent * self);
#endif
bool wxGridRangeSelectEvent_MetaDown(const wxGridRangeSelectEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxGridRangeSelectEvent_Selecting(const wxGridRangeSelectEvent * self);
#endif
bool wxGridRangeSelectEvent_ShiftDown(const wxGridRangeSelectEvent * self);

// CLASS: wxGridSizeEvent
wxClassInfo *wxGridSizeEvent_CLASSINFO();
wxGridSizeEvent *wxGridSizeEvent_new();
bool wxGridSizeEvent_AltDown(const wxGridSizeEvent * self);
bool wxGridSizeEvent_ControlDown(const wxGridSizeEvent * self);
#if wxCHECK_VERSION(3, 1, 0)
wxPoint *wxGridSizeEvent_GetPosition(const wxGridSizeEvent * self);
int wxGridSizeEvent_GetRowOrCol(const wxGridSizeEvent * self);
#endif
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
#if wxCHECK_VERSION(3, 1, 0)
wxString *wxGridTableBase_GetCornerLabelValue(const wxGridTableBase * self);
#endif
void wxGridTableBase_SetRowLabelValue(wxGridTableBase * self, int row, const wxString * label);
void wxGridTableBase_SetColLabelValue(wxGridTableBase * self, int col, const wxString * label);
void wxGridTableBase_SetAttrProvider(wxGridTableBase * self, wxGridCellAttrProvider * attr_provider);
wxGridCellAttrProvider * wxGridTableBase_GetAttrProvider(const wxGridTableBase * self);
void wxGridTableBase_SetAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row, int col);
void wxGridTableBase_SetRowAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row);
void wxGridTableBase_SetColAttr(wxGridTableBase * self, wxGridCellAttr * attr, int col);
bool wxGridTableBase_CanHaveAttributes(wxGridTableBase * self);
#if wxCHECK_VERSION(3, 1, 0)
bool wxGridTableBase_CanMeasureColUsingSameAttr(const wxGridTableBase * self, int col);
#endif
int wxGridTableBase_GetNumberRows(wxGridTableBase * self);
int wxGridTableBase_GetNumberCols(wxGridTableBase * self);
int wxGridTableBase_GetRowsCount(const wxGridTableBase * self);
int wxGridTableBase_GetColsCount(const wxGridTableBase * self);

// CLASS: wxGridUpdateLocker
void wxGridUpdateLocker_delete(wxGridUpdateLocker *self);
wxGridUpdateLocker *wxGridUpdateLocker_new(wxGrid * grid);
void wxGridUpdateLocker_Create(wxGridUpdateLocker * self, wxGrid * grid);

} // extern "C"

