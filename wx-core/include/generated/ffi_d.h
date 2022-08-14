#pragma once

#include <wx/dataobj.h>
#include <wx/dataview.h>
#include <wx/datectrl.h>
#include <wx/dateevt.h>
#include <wx/dc.h>
#include <wx/dialog.h>
#include <wx/dirdlg.h>
#include <wx/display.h>
#include <wx/dnd.h>
#include <wx/dragimag.h>
#include <wx/event.h>
#include <wx/filepicker.h>
#include <wx/overlay.h>
#include <wx/renderer.h>

extern "C" {

// CLASS: wxDC
wxClassInfo *wxDC_CLASSINFO();
wxCoord wxDC_DeviceToLogicalX(const wxDC * self, wxCoord x);
wxCoord wxDC_DeviceToLogicalXRel(const wxDC * self, wxCoord x);
wxCoord wxDC_DeviceToLogicalY(const wxDC * self, wxCoord y);
wxCoord wxDC_DeviceToLogicalYRel(const wxDC * self, wxCoord y);
wxCoord wxDC_LogicalToDeviceX(const wxDC * self, wxCoord x);
wxCoord wxDC_LogicalToDeviceXRel(const wxDC * self, wxCoord x);
wxCoord wxDC_LogicalToDeviceY(const wxDC * self, wxCoord y);
wxCoord wxDC_LogicalToDeviceYRel(const wxDC * self, wxCoord y);
wxPoint *wxDC_DeviceToLogical(const wxDC * self, wxCoord x, wxCoord y);
wxPoint *wxDC_DeviceToLogical1(const wxDC * self, const wxPoint * pt);
wxSize *wxDC_DeviceToLogicalRel(const wxDC * self, int x, int y);
wxSize *wxDC_DeviceToLogicalRel1(const wxDC * self, const wxSize * dim);
wxPoint *wxDC_LogicalToDevice(const wxDC * self, wxCoord x, wxCoord y);
wxPoint *wxDC_LogicalToDevice1(const wxDC * self, const wxPoint * pt);
wxSize *wxDC_LogicalToDeviceRel(const wxDC * self, int x, int y);
wxSize *wxDC_LogicalToDeviceRel1(const wxDC * self, const wxSize * dim);
void wxDC_Clear(wxDC * self);
void wxDC_DrawArc(wxDC * self, wxCoord x_start, wxCoord y_start, wxCoord x_end, wxCoord y_end, wxCoord xc, wxCoord yc);
void wxDC_DrawArc1(wxDC * self, const wxPoint * pt_start, const wxPoint * pt_end, const wxPoint * centre);
void wxDC_DrawBitmap(wxDC * self, const wxBitmap * bitmap, wxCoord x, wxCoord y, bool use_mask);
void wxDC_DrawBitmap1(wxDC * self, const wxBitmap * bmp, const wxPoint * pt, bool use_mask);
void wxDC_DrawCheckMark(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
void wxDC_DrawCheckMark1(wxDC * self, const wxRect * rect);
void wxDC_DrawCircle(wxDC * self, wxCoord x, wxCoord y, wxCoord radius);
void wxDC_DrawCircle1(wxDC * self, const wxPoint * pt, wxCoord radius);
void wxDC_DrawEllipse(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
void wxDC_DrawEllipse1(wxDC * self, const wxPoint * pt, const wxSize * size);
void wxDC_DrawEllipse2(wxDC * self, const wxRect * rect);
void wxDC_DrawEllipticArc(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height, double start, double end);
void wxDC_DrawEllipticArc1(wxDC * self, const wxPoint * pt, const wxSize * sz, double sa, double ea);
void wxDC_DrawIcon(wxDC * self, const wxIcon * icon, wxCoord x, wxCoord y);
void wxDC_DrawIcon1(wxDC * self, const wxIcon * icon, const wxPoint * pt);
void wxDC_DrawLabel(wxDC * self, const wxString * text, const wxBitmap * bitmap, const wxRect * rect, int alignment, int index_accel, wxRect * rect_bounding);
void wxDC_DrawLabel1(wxDC * self, const wxString * text, const wxRect * rect, int alignment, int index_accel);
void wxDC_DrawLine(wxDC * self, wxCoord x1, wxCoord y1, wxCoord x2, wxCoord y2);
void wxDC_DrawLine1(wxDC * self, const wxPoint * pt1, const wxPoint * pt2);
void wxDC_DrawLines1(wxDC * self, const wxPointList * points, wxCoord xoffset, wxCoord yoffset);
void wxDC_DrawPoint(wxDC * self, wxCoord x, wxCoord y);
void wxDC_DrawPoint1(wxDC * self, const wxPoint * pt);
void wxDC_DrawRectangle(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
void wxDC_DrawRectangle1(wxDC * self, const wxPoint * pt, const wxSize * sz);
void wxDC_DrawRectangle2(wxDC * self, const wxRect * rect);
void wxDC_DrawRotatedText(wxDC * self, const wxString * text, wxCoord x, wxCoord y, double angle);
void wxDC_DrawRotatedText1(wxDC * self, const wxString * text, const wxPoint * point, double angle);
void wxDC_DrawRoundedRectangle(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height, double radius);
void wxDC_DrawRoundedRectangle1(wxDC * self, const wxPoint * pt, const wxSize * sz, double radius);
void wxDC_DrawRoundedRectangle2(wxDC * self, const wxRect * rect, double radius);
void wxDC_DrawSpline1(wxDC * self, const wxPointList * points);
void wxDC_DrawSpline2(wxDC * self, wxCoord x1, wxCoord y1, wxCoord x2, wxCoord y2, wxCoord x3, wxCoord y3);
void wxDC_DrawText(wxDC * self, const wxString * text, wxCoord x, wxCoord y);
void wxDC_DrawText1(wxDC * self, const wxString * text, const wxPoint * pt);
void wxDC_GradientFillConcentric(wxDC * self, const wxRect * rect, const wxColour * initial_colour, const wxColour * dest_colour);
void wxDC_GradientFillConcentric1(wxDC * self, const wxRect * rect, const wxColour * initial_colour, const wxColour * dest_colour, const wxPoint * circle_center);
void wxDC_GradientFillLinear(wxDC * self, const wxRect * rect, const wxColour * initial_colour, const wxColour * dest_colour, wxDirection n_direction);
void wxDC_CrossHair(wxDC * self, wxCoord x, wxCoord y);
void wxDC_CrossHair1(wxDC * self, const wxPoint * pt);
void wxDC_DestroyClippingRegion(wxDC * self);
bool wxDC_GetClippingBox(const wxDC * self, wxCoord * x, wxCoord * y, wxCoord * width, wxCoord * height);
bool wxDC_GetClippingBox1(const wxDC * self, wxRect * rect);
void wxDC_SetClippingRegion(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height);
void wxDC_SetClippingRegion1(wxDC * self, const wxPoint * pt, const wxSize * sz);
void wxDC_SetClippingRegion2(wxDC * self, const wxRect * rect);
void wxDC_SetDeviceClippingRegion(wxDC * self, const wxRegion * region);
wxCoord wxDC_GetCharHeight(const wxDC * self);
wxCoord wxDC_GetCharWidth(const wxDC * self);
void wxDC_GetMultiLineTextExtent(const wxDC * self, const wxString * string, wxCoord * w, wxCoord * h, wxCoord * height_line, const wxFont * font);
wxSize *wxDC_GetMultiLineTextExtent1(const wxDC * self, const wxString * string);
bool wxDC_GetPartialTextExtents(const wxDC * self, const wxString * text, wxArrayInt * widths);
void wxDC_GetTextExtent(const wxDC * self, const wxString * string, wxCoord * w, wxCoord * h, wxCoord * descent, wxCoord * external_leading, const wxFont * font);
wxSize *wxDC_GetTextExtent1(const wxDC * self, const wxString * string);
int wxDC_GetBackgroundMode(const wxDC * self);
wxFont *wxDC_GetFont(const wxDC * self);
wxLayoutDirection wxDC_GetLayoutDirection(const wxDC * self);
wxColour *wxDC_GetTextBackground(const wxDC * self);
wxColour *wxDC_GetTextForeground(const wxDC * self);
void wxDC_SetBackgroundMode(wxDC * self, int mode);
void wxDC_SetFont(wxDC * self, const wxFont * font);
void wxDC_SetTextBackground(wxDC * self, const wxColour * colour);
void wxDC_SetTextForeground(wxDC * self, const wxColour * colour);
void wxDC_SetLayoutDirection(wxDC * self, wxLayoutDirection dir);
void wxDC_CalcBoundingBox(wxDC * self, wxCoord x, wxCoord y);
wxCoord wxDC_MaxX(const wxDC * self);
wxCoord wxDC_MaxY(const wxDC * self);
wxCoord wxDC_MinX(const wxDC * self);
wxCoord wxDC_MinY(const wxDC * self);
void wxDC_ResetBoundingBox(wxDC * self);
bool wxDC_StartDoc(wxDC * self, const wxString * message);
void wxDC_StartPage(wxDC * self);
void wxDC_EndDoc(wxDC * self);
void wxDC_EndPage(wxDC * self);
wxBrush *wxDC_GetBackground(const wxDC * self);
wxBrush *wxDC_GetBrush(const wxDC * self);
void wxDC_SetBackground(wxDC * self, const wxBrush * brush);
void wxDC_SetBrush(wxDC * self, const wxBrush * brush);
void wxDC_SetPen(wxDC * self, const wxPen * pen);
void wxDC_CopyAttributes(wxDC * self, const wxDC * dc);
double wxDC_GetContentScaleFactor(const wxDC * self);
int wxDC_GetDepth(const wxDC * self);
wxPoint *wxDC_GetDeviceOrigin(const wxDC * self);
bool wxDC_GetPixel(const wxDC * self, wxCoord x, wxCoord y, wxColour * colour);
wxSize *wxDC_GetPPI(const wxDC * self);
#if wxCHECK_VERSION(3, 1, 7)
wxSize *wxDC_FromDIP(const wxDC * self, const wxSize * sz);
wxPoint *wxDC_FromDIP1(const wxDC * self, const wxPoint * pt);
int wxDC_FromDIP2(const wxDC * self, int d);
wxSize *wxDC_ToDIP(const wxDC * self, const wxSize * sz);
wxPoint *wxDC_ToDIP1(const wxDC * self, const wxPoint * pt);
int wxDC_ToDIP2(const wxDC * self, int d);
#endif
void wxDC_GetSize(const wxDC * self, wxCoord * width, wxCoord * height);
wxSize *wxDC_GetSize1(const wxDC * self);
void wxDC_GetSizeMM(const wxDC * self, wxCoord * width, wxCoord * height);
wxSize *wxDC_GetSizeMM1(const wxDC * self);
void wxDC_GetUserScale(const wxDC * self, double * x, double * y);
bool wxDC_IsOk(const wxDC * self);
void wxDC_SetAxisOrientation(wxDC * self, bool x_left_right, bool y_bottom_up);
void wxDC_SetDeviceOrigin(wxDC * self, wxCoord x, wxCoord y);
void wxDC_SetPalette(wxDC * self, const wxPalette * palette);
void wxDC_SetUserScale(wxDC * self, double x_scale, double y_scale);
bool wxDC_CanUseTransformMatrix(const wxDC * self);
bool wxDC_SetTransformMatrix(wxDC * self, const wxAffineMatrix2D * matrix);
wxAffineMatrix2D *wxDC_GetTransformMatrix(const wxDC * self);
void wxDC_ResetTransformMatrix(wxDC * self);
bool wxDC_CanDrawBitmap(const wxDC * self);
bool wxDC_CanGetTextExtent(const wxDC * self);
void * wxDC_GetHandle(const wxDC * self);
wxBitmap *wxDC_GetAsBitmap(const wxDC * self, const wxRect * subrect);
void wxDC_SetLogicalScale(wxDC * self, double x, double y);
void wxDC_GetLogicalScale(const wxDC * self, double * x, double * y);
void wxDC_SetLogicalOrigin(wxDC * self, wxCoord x, wxCoord y);
void wxDC_GetLogicalOrigin(const wxDC * self, wxCoord * x, wxCoord * y);
wxPoint *wxDC_GetLogicalOrigin1(const wxDC * self);
wxGraphicsContext * wxDC_GetGraphicsContext(const wxDC * self);
void wxDC_SetGraphicsContext(wxDC * self, wxGraphicsContext * ctx);

// CLASS: wxDCBrushChanger
void wxDCBrushChanger_delete(wxDCBrushChanger *self);
wxDCBrushChanger *wxDCBrushChanger_new(wxDC * dc, const wxBrush * brush);

// CLASS: wxDCClipper
void wxDCClipper_delete(wxDCClipper *self);
wxDCClipper *wxDCClipper_new(wxDC * dc, const wxRegion * region);
wxDCClipper *wxDCClipper_new1(wxDC * dc, const wxRect * rect);
wxDCClipper *wxDCClipper_new2(wxDC * dc, wxCoord x, wxCoord y, wxCoord w, wxCoord h);

// CLASS: wxDCFontChanger
void wxDCFontChanger_delete(wxDCFontChanger *self);
wxDCFontChanger *wxDCFontChanger_new(wxDC * dc);
wxDCFontChanger *wxDCFontChanger_new1(wxDC * dc, const wxFont * font);
void wxDCFontChanger_Set(wxDCFontChanger * self, const wxFont * font);

// CLASS: wxDCOverlay
void wxDCOverlay_delete(wxDCOverlay *self);
wxDCOverlay *wxDCOverlay_new(wxOverlay * overlay, wxDC * dc, int x, int y, int width, int height);
wxDCOverlay *wxDCOverlay_new1(wxOverlay * overlay, wxDC * dc);
void wxDCOverlay_Clear(wxDCOverlay * self);

// CLASS: wxDCPenChanger
void wxDCPenChanger_delete(wxDCPenChanger *self);
wxDCPenChanger *wxDCPenChanger_new(wxDC * dc, const wxPen * pen);

// CLASS: wxDCTextBgColourChanger
void wxDCTextBgColourChanger_delete(wxDCTextBgColourChanger *self);
wxDCTextBgColourChanger *wxDCTextBgColourChanger_new(wxDC * dc);
wxDCTextBgColourChanger *wxDCTextBgColourChanger_new1(wxDC * dc, const wxColour * col);
void wxDCTextBgColourChanger_Set(wxDCTextBgColourChanger * self, const wxColour * col);

// CLASS: wxDCTextBgModeChanger
void wxDCTextBgModeChanger_delete(wxDCTextBgModeChanger *self);

// CLASS: wxDCTextColourChanger
void wxDCTextColourChanger_delete(wxDCTextColourChanger *self);
wxDCTextColourChanger *wxDCTextColourChanger_new(wxDC * dc);
wxDCTextColourChanger *wxDCTextColourChanger_new1(wxDC * dc, const wxColour * col);
void wxDCTextColourChanger_Set(wxDCTextColourChanger * self, const wxColour * col);

// CLASS: wxDPIChangedEvent
wxClassInfo *wxDPIChangedEvent_CLASSINFO();
wxSize *wxDPIChangedEvent_GetOldDPI(const wxDPIChangedEvent * self);
wxSize *wxDPIChangedEvent_GetNewDPI(const wxDPIChangedEvent * self);
int wxDPIChangedEvent_ScaleX(const wxDPIChangedEvent * self, int x);
int wxDPIChangedEvent_ScaleY(const wxDPIChangedEvent * self, int y);

// CLASS: wxDataFormat
void wxDataFormat_delete(wxDataFormat *self);
wxDataFormat *wxDataFormat_new1(const wxString * format);
wxString *wxDataFormat_GetId(const wxDataFormat * self);
void wxDataFormat_SetId(wxDataFormat * self, const wxString * format);

// CLASS: wxDataObject
void wxDataObject_delete(wxDataObject *self);
bool wxDataObject_GetDataHere(const wxDataObject * self, const wxDataFormat * format, void * buf);
size_t wxDataObject_GetDataSize(const wxDataObject * self, const wxDataFormat * format);
bool wxDataObject_SetData(wxDataObject * self, const wxDataFormat * format, size_t len, const void * buf);

// CLASS: wxDataObjectComposite
void wxDataObjectComposite_delete(wxDataObjectComposite *self);
wxDataObjectComposite *wxDataObjectComposite_new();
void wxDataObjectComposite_Add(wxDataObjectComposite * self, wxDataObjectSimple * data_object, bool preferred);
wxDataFormat *wxDataObjectComposite_GetReceivedFormat(const wxDataObjectComposite * self);

// CLASS: wxDataObjectSimple
void wxDataObjectSimple_delete(wxDataObjectSimple *self);
wxDataObjectSimple *wxDataObjectSimple_new(const wxDataFormat * format);
bool wxDataObjectSimple_GetDataHere(const wxDataObjectSimple * self, void * buf);
size_t wxDataObjectSimple_GetDataSize(const wxDataObjectSimple * self);
bool wxDataObjectSimple_SetData(wxDataObjectSimple * self, size_t len, const void * buf);
void wxDataObjectSimple_SetFormat(wxDataObjectSimple * self, const wxDataFormat * format);

// CLASS: wxDataViewBitmapRenderer
wxClassInfo *wxDataViewBitmapRenderer_CLASSINFO();
wxString *wxDataViewBitmapRenderer_GetDefaultType();

// CLASS: wxDataViewCheckIconTextRenderer
wxClassInfo *wxDataViewCheckIconTextRenderer_CLASSINFO();
wxString *wxDataViewCheckIconTextRenderer_GetDefaultType();
void wxDataViewCheckIconTextRenderer_Allow3rdStateForUser(wxDataViewCheckIconTextRenderer * self, bool allow);

// CLASS: wxDataViewChoiceByIndexRenderer
wxClassInfo *wxDataViewChoiceByIndexRenderer_CLASSINFO();

// CLASS: wxDataViewChoiceRenderer
wxClassInfo *wxDataViewChoiceRenderer_CLASSINFO();
wxString *wxDataViewChoiceRenderer_GetChoice(const wxDataViewChoiceRenderer * self, size_t index);
wxArrayString *wxDataViewChoiceRenderer_GetChoices(const wxDataViewChoiceRenderer * self);

// CLASS: wxDataViewColumn
void wxDataViewColumn_delete(wxDataViewColumn *self);
wxDataViewColumn *wxDataViewColumn_new(const wxString * title, wxDataViewRenderer * renderer, unsigned int model_column, int width, wxAlignment align, int flags);
wxDataViewColumn *wxDataViewColumn_new1(const wxBitmapBundle * bitmap, wxDataViewRenderer * renderer, unsigned int model_column, int width, wxAlignment align, int flags);
unsigned int wxDataViewColumn_GetModelColumn(const wxDataViewColumn * self);
wxDataViewCtrl * wxDataViewColumn_GetOwner(const wxDataViewColumn * self);
wxDataViewRenderer * wxDataViewColumn_GetRenderer(const wxDataViewColumn * self);

// CLASS: wxDataViewCtrl
wxClassInfo *wxDataViewCtrl_CLASSINFO();
wxDataViewCtrl *wxDataViewCtrl_new();
wxDataViewCtrl *wxDataViewCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDataViewCtrl_AllowMultiColumnSort(wxDataViewCtrl * self, bool allow);
bool wxDataViewCtrl_Create(wxDataViewCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDataViewCtrl_AppendColumn(wxDataViewCtrl * self, wxDataViewColumn * col);
bool wxDataViewCtrl_PrependColumn(wxDataViewCtrl * self, wxDataViewColumn * col);
bool wxDataViewCtrl_InsertColumn(wxDataViewCtrl * self, unsigned int pos, wxDataViewColumn * col);
bool wxDataViewCtrl_AssociateModel(wxDataViewCtrl * self, wxDataViewModel * model);
bool wxDataViewCtrl_ClearColumns(wxDataViewCtrl * self);
void wxDataViewCtrl_Collapse(wxDataViewCtrl * self, const wxDataViewItem * item);
bool wxDataViewCtrl_DeleteColumn(wxDataViewCtrl * self, wxDataViewColumn * column);
void wxDataViewCtrl_EditItem(wxDataViewCtrl * self, const wxDataViewItem * item, const wxDataViewColumn * column);
bool wxDataViewCtrl_EnableDragSource(wxDataViewCtrl * self, const wxDataFormat * format);
bool wxDataViewCtrl_EnableDropTargets(wxDataViewCtrl * self, const wxVector< wxDataFormat > * formats);
bool wxDataViewCtrl_EnableDropTarget(wxDataViewCtrl * self, const wxDataFormat * format);
void wxDataViewCtrl_EnsureVisible(wxDataViewCtrl * self, const wxDataViewItem * item, const wxDataViewColumn * column);
void wxDataViewCtrl_Expand(wxDataViewCtrl * self, const wxDataViewItem * item);
void wxDataViewCtrl_ExpandAncestors(wxDataViewCtrl * self, const wxDataViewItem * item);
void wxDataViewCtrl_ExpandChildren(wxDataViewCtrl * self, const wxDataViewItem * item);
wxDataViewColumn * wxDataViewCtrl_GetColumn(const wxDataViewCtrl * self, unsigned int pos);
unsigned int wxDataViewCtrl_GetColumnCount(const wxDataViewCtrl * self);
int wxDataViewCtrl_GetColumnPosition(const wxDataViewCtrl * self, const wxDataViewColumn * column);
wxDataViewColumn * wxDataViewCtrl_GetExpanderColumn(const wxDataViewCtrl * self);
wxDataViewItem *wxDataViewCtrl_GetCurrentItem(const wxDataViewCtrl * self);
wxDataViewColumn * wxDataViewCtrl_GetCurrentColumn(const wxDataViewCtrl * self);
int wxDataViewCtrl_GetIndent(const wxDataViewCtrl * self);
wxRect *wxDataViewCtrl_GetItemRect(const wxDataViewCtrl * self, const wxDataViewItem * item, const wxDataViewColumn * col);
wxWindow * wxDataViewCtrl_GetMainWindow(wxDataViewCtrl * self);
wxDataViewModel * wxDataViewCtrl_GetModel(wxDataViewCtrl * self);
int wxDataViewCtrl_GetSelectedItemsCount(const wxDataViewCtrl * self);
wxDataViewItem *wxDataViewCtrl_GetSelection(const wxDataViewCtrl * self);
int wxDataViewCtrl_GetSelections(const wxDataViewCtrl * self, wxDataViewItemArray * sel);
wxDataViewColumn * wxDataViewCtrl_GetSortingColumn(const wxDataViewCtrl * self);
bool wxDataViewCtrl_HasSelection(const wxDataViewCtrl * self);
void wxDataViewCtrl_HitTest(const wxDataViewCtrl * self, const wxPoint * point, wxDataViewItem * item, wxDataViewColumn *& col);
bool wxDataViewCtrl_IsExpanded(const wxDataViewCtrl * self, const wxDataViewItem * item);
bool wxDataViewCtrl_IsMultiColumnSortAllowed(const wxDataViewCtrl * self);
bool wxDataViewCtrl_IsSelected(const wxDataViewCtrl * self, const wxDataViewItem * item);
void wxDataViewCtrl_Select(wxDataViewCtrl * self, const wxDataViewItem * item);
void wxDataViewCtrl_SelectAll(wxDataViewCtrl * self);
bool wxDataViewCtrl_SetAlternateRowColour(wxDataViewCtrl * self, const wxColour * colour);
void wxDataViewCtrl_SetExpanderColumn(wxDataViewCtrl * self, wxDataViewColumn * col);
void wxDataViewCtrl_SetCurrentItem(wxDataViewCtrl * self, const wxDataViewItem * item);
bool wxDataViewCtrl_SetHeaderAttr(wxDataViewCtrl * self, const wxItemAttr * attr);
void wxDataViewCtrl_SetIndent(wxDataViewCtrl * self, int indent);
void wxDataViewCtrl_SetSelections(wxDataViewCtrl * self, const wxDataViewItemArray * sel);
void wxDataViewCtrl_Unselect(wxDataViewCtrl * self, const wxDataViewItem * item);
void wxDataViewCtrl_UnselectAll(wxDataViewCtrl * self);
bool wxDataViewCtrl_SetRowHeight(wxDataViewCtrl * self, int row_height);
void wxDataViewCtrl_ToggleSortByColumn(wxDataViewCtrl * self, int column);
int wxDataViewCtrl_GetCountPerPage(const wxDataViewCtrl * self);
wxDataViewItem *wxDataViewCtrl_GetTopItem(const wxDataViewCtrl * self);

// CLASS: wxDataViewCustomRenderer
wxClassInfo *wxDataViewCustomRenderer_CLASSINFO();
wxString *wxDataViewCustomRenderer_GetDefaultType();
bool wxDataViewCustomRenderer_ActivateCell(wxDataViewCustomRenderer * self, const wxRect * cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col, const wxMouseEvent * mouse_event);
wxDataViewItemAttr *wxDataViewCustomRenderer_GetAttr(const wxDataViewCustomRenderer * self);
wxSize *wxDataViewCustomRenderer_GetSize(const wxDataViewCustomRenderer * self);
bool wxDataViewCustomRenderer_StartDrag(wxDataViewCustomRenderer * self, const wxPoint * cursor, const wxRect * cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col);

// CLASS: wxDataViewDateRenderer
wxClassInfo *wxDataViewDateRenderer_CLASSINFO();
wxString *wxDataViewDateRenderer_GetDefaultType();

// CLASS: wxDataViewEvent
wxClassInfo *wxDataViewEvent_CLASSINFO();
wxDataViewEvent *wxDataViewEvent_new();
wxDataViewEvent *wxDataViewEvent_new3(const wxDataViewEvent * event);
int wxDataViewEvent_GetColumn(const wxDataViewEvent * self);
wxDataViewColumn * wxDataViewEvent_GetDataViewColumn(const wxDataViewEvent * self);
wxDataViewModel * wxDataViewEvent_GetModel(const wxDataViewEvent * self);
wxPoint *wxDataViewEvent_GetPosition(const wxDataViewEvent * self);
bool wxDataViewEvent_IsEditCancelled(const wxDataViewEvent * self);
void wxDataViewEvent_SetColumn(wxDataViewEvent * self, int col);
void wxDataViewEvent_SetValue(wxDataViewEvent * self, const wxVariant * value);
void wxDataViewEvent_SetDataObject(wxDataViewEvent * self, wxDataObject * obj);
wxDataFormat *wxDataViewEvent_GetDataFormat(const wxDataViewEvent * self);
size_t wxDataViewEvent_GetDataSize(const wxDataViewEvent * self);
void * wxDataViewEvent_GetDataBuffer(const wxDataViewEvent * self);
void wxDataViewEvent_SetDragFlags(wxDataViewEvent * self, int flags);
int wxDataViewEvent_GetCacheFrom(const wxDataViewEvent * self);
int wxDataViewEvent_GetCacheTo(const wxDataViewEvent * self);
int wxDataViewEvent_GetProposedDropIndex(const wxDataViewEvent * self);
wxDataViewItem *wxDataViewEvent_GetItem(const wxDataViewEvent * self);
void wxDataViewEvent_SetPosition(wxDataViewEvent * self, int x, int y);
void wxDataViewEvent_SetCache(wxDataViewEvent * self, int from, int to);
wxDataObject * wxDataViewEvent_GetDataObject(const wxDataViewEvent * self);
void wxDataViewEvent_SetDataFormat(wxDataViewEvent * self, const wxDataFormat * format);
void wxDataViewEvent_SetDataSize(wxDataViewEvent * self, size_t size);
void wxDataViewEvent_SetDataBuffer(wxDataViewEvent * self, void * buf);
int wxDataViewEvent_GetDragFlags(const wxDataViewEvent * self);

// CLASS: wxDataViewIconText
wxClassInfo *wxDataViewIconText_CLASSINFO();
wxDataViewIconText *wxDataViewIconText_new(const wxString * text, const wxBitmapBundle * bitmap);
wxDataViewIconText *wxDataViewIconText_new1(const wxDataViewIconText * other);
wxBitmapBundle *wxDataViewIconText_GetBitmapBundle(const wxDataViewIconText * self);
wxIcon *wxDataViewIconText_GetIcon(const wxDataViewIconText * self);
wxString *wxDataViewIconText_GetText(const wxDataViewIconText * self);
void wxDataViewIconText_SetBitmapBundle(wxDataViewIconText * self, const wxBitmapBundle * bitmap);
void wxDataViewIconText_SetIcon(wxDataViewIconText * self, const wxIcon * icon);
void wxDataViewIconText_SetText(wxDataViewIconText * self, const wxString * text);

// CLASS: wxDataViewIconTextRenderer
wxClassInfo *wxDataViewIconTextRenderer_CLASSINFO();
wxString *wxDataViewIconTextRenderer_GetDefaultType();

// CLASS: wxDataViewIndexListModel
wxDataViewItem *wxDataViewIndexListModel_GetItem(const wxDataViewIndexListModel * self, unsigned int row);
void wxDataViewIndexListModel_Reset(wxDataViewIndexListModel * self, unsigned int new_size);
void wxDataViewIndexListModel_RowAppended(wxDataViewIndexListModel * self);
void wxDataViewIndexListModel_RowChanged(wxDataViewIndexListModel * self, unsigned int row);
void wxDataViewIndexListModel_RowDeleted(wxDataViewIndexListModel * self, unsigned int row);
void wxDataViewIndexListModel_RowInserted(wxDataViewIndexListModel * self, unsigned int before);
void wxDataViewIndexListModel_RowPrepended(wxDataViewIndexListModel * self);
void wxDataViewIndexListModel_RowValueChanged(wxDataViewIndexListModel * self, unsigned int row, unsigned int col);
void wxDataViewIndexListModel_RowsDeleted(wxDataViewIndexListModel * self, const wxArrayInt * rows);

// CLASS: wxDataViewItem
void wxDataViewItem_delete(wxDataViewItem *self);
wxDataViewItem *wxDataViewItem_new();
wxDataViewItem *wxDataViewItem_new1(const wxDataViewItem * item);
wxDataViewItem *wxDataViewItem_new2(void * id);
void * wxDataViewItem_GetID(const wxDataViewItem * self);
bool wxDataViewItem_IsOk(const wxDataViewItem * self);

// CLASS: wxDataViewItemAttr
void wxDataViewItemAttr_delete(wxDataViewItemAttr *self);
wxDataViewItemAttr *wxDataViewItemAttr_new();
void wxDataViewItemAttr_SetBold(wxDataViewItemAttr * self, bool set);
void wxDataViewItemAttr_SetColour(wxDataViewItemAttr * self, const wxColour * colour);
void wxDataViewItemAttr_SetBackgroundColour(wxDataViewItemAttr * self, const wxColour * colour);
void wxDataViewItemAttr_SetItalic(wxDataViewItemAttr * self, bool set);
void wxDataViewItemAttr_SetStrikethrough(wxDataViewItemAttr * self, bool set);
bool wxDataViewItemAttr_HasColour(const wxDataViewItemAttr * self);
wxColour *wxDataViewItemAttr_GetColour(const wxDataViewItemAttr * self);
bool wxDataViewItemAttr_HasFont(const wxDataViewItemAttr * self);
bool wxDataViewItemAttr_GetBold(const wxDataViewItemAttr * self);
bool wxDataViewItemAttr_GetItalic(const wxDataViewItemAttr * self);
bool wxDataViewItemAttr_HasBackgroundColour(const wxDataViewItemAttr * self);
wxColour *wxDataViewItemAttr_GetBackgroundColour(const wxDataViewItemAttr * self);
bool wxDataViewItemAttr_IsDefault(const wxDataViewItemAttr * self);
wxFont *wxDataViewItemAttr_GetEffectiveFont(const wxDataViewItemAttr * self, const wxFont * font);

// CLASS: wxDataViewListCtrl
wxClassInfo *wxDataViewListCtrl_CLASSINFO();
int wxDataViewListCtrl_GetSelectedRow(const wxDataViewListCtrl * self);
void wxDataViewListCtrl_DeleteAllItems(wxDataViewListCtrl * self);
unsigned int wxDataViewListCtrl_GetItemCount(const wxDataViewListCtrl * self);
void wxDataViewListCtrl_SetValue(wxDataViewListCtrl * self, const wxVariant * value, unsigned int row, unsigned int col);
void wxDataViewListCtrl_GetValue(wxDataViewListCtrl * self, wxVariant * value, unsigned int row, unsigned int col);
void wxDataViewListCtrl_SetTextValue(wxDataViewListCtrl * self, const wxString * value, unsigned int row, unsigned int col);
wxString *wxDataViewListCtrl_GetTextValue(const wxDataViewListCtrl * self, unsigned int row, unsigned int col);
void wxDataViewListCtrl_SetToggleValue(wxDataViewListCtrl * self, bool value, unsigned int row, unsigned int col);
bool wxDataViewListCtrl_GetToggleValue(const wxDataViewListCtrl * self, unsigned int row, unsigned int col);
wxDataViewListCtrl *wxDataViewListCtrl_new();
wxDataViewListCtrl *wxDataViewListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator);
bool wxDataViewListCtrl_Create(wxDataViewListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator);
const wxDataViewListStore * wxDataViewListCtrl_GetStore1(const wxDataViewListCtrl * self);
int wxDataViewListCtrl_ItemToRow(const wxDataViewListCtrl * self, const wxDataViewItem * item);
wxDataViewItem *wxDataViewListCtrl_RowToItem(const wxDataViewListCtrl * self, int row);

// CLASS: wxDataViewListModel
bool wxDataViewListModel_GetAttrByRow(const wxDataViewListModel * self, unsigned int row, unsigned int col, wxDataViewItemAttr * attr);
bool wxDataViewListModel_IsEnabledByRow(const wxDataViewListModel * self, unsigned int row, unsigned int col);
unsigned int wxDataViewListModel_GetCount(const wxDataViewListModel * self);
unsigned int wxDataViewListModel_GetRow(const wxDataViewListModel * self, const wxDataViewItem * item);
void wxDataViewListModel_GetValueByRow(const wxDataViewListModel * self, wxVariant * variant, unsigned int row, unsigned int col);
bool wxDataViewListModel_SetValueByRow(wxDataViewListModel * self, const wxVariant * variant, unsigned int row, unsigned int col);

// CLASS: wxDataViewListStore
wxDataViewListStore *wxDataViewListStore_new();
void wxDataViewListStore_PrependColumn(wxDataViewListStore * self, const wxString * varianttype);
void wxDataViewListStore_InsertColumn(wxDataViewListStore * self, unsigned int pos, const wxString * varianttype);
void wxDataViewListStore_AppendColumn(wxDataViewListStore * self, const wxString * varianttype);
void wxDataViewListStore_DeleteAllItems(wxDataViewListStore * self);
unsigned int wxDataViewListStore_GetItemCount(const wxDataViewListStore * self);

// CLASS: wxDataViewModel
void wxDataViewModel_AddNotifier(wxDataViewModel * self, wxDataViewModelNotifier * notifier);
bool wxDataViewModel_ChangeValue(wxDataViewModel * self, const wxVariant * variant, const wxDataViewItem * item, unsigned int col);
bool wxDataViewModel_Cleared(wxDataViewModel * self);
int wxDataViewModel_Compare(const wxDataViewModel * self, const wxDataViewItem * item1, const wxDataViewItem * item2, unsigned int column, bool ascending);
bool wxDataViewModel_GetAttr(const wxDataViewModel * self, const wxDataViewItem * item, unsigned int col, wxDataViewItemAttr * attr);
bool wxDataViewModel_IsEnabled(const wxDataViewModel * self, const wxDataViewItem * item, unsigned int col);
unsigned int wxDataViewModel_GetChildren(const wxDataViewModel * self, const wxDataViewItem * item, wxDataViewItemArray * children);
wxDataViewItem *wxDataViewModel_GetParent(const wxDataViewModel * self, const wxDataViewItem * item);
void wxDataViewModel_GetValue(const wxDataViewModel * self, wxVariant * variant, const wxDataViewItem * item, unsigned int col);
bool wxDataViewModel_HasContainerColumns(const wxDataViewModel * self, const wxDataViewItem * item);
bool wxDataViewModel_HasDefaultCompare(const wxDataViewModel * self);
bool wxDataViewModel_IsContainer(const wxDataViewModel * self, const wxDataViewItem * item);
bool wxDataViewModel_ItemAdded(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItem * item);
bool wxDataViewModel_ItemChanged(wxDataViewModel * self, const wxDataViewItem * item);
bool wxDataViewModel_ItemDeleted(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItem * item);
bool wxDataViewModel_ItemsAdded(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItemArray * items);
bool wxDataViewModel_ItemsChanged(wxDataViewModel * self, const wxDataViewItemArray * items);
bool wxDataViewModel_ItemsDeleted(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItemArray * items);
void wxDataViewModel_RemoveNotifier(wxDataViewModel * self, wxDataViewModelNotifier * notifier);
void wxDataViewModel_Resort(wxDataViewModel * self);
bool wxDataViewModel_SetValue(wxDataViewModel * self, const wxVariant * variant, const wxDataViewItem * item, unsigned int col);
bool wxDataViewModel_ValueChanged(wxDataViewModel * self, const wxDataViewItem * item, unsigned int col);
bool wxDataViewModel_IsListModel(const wxDataViewModel * self);
bool wxDataViewModel_IsVirtualListModel(const wxDataViewModel * self);

// CLASS: wxDataViewModelNotifier
void wxDataViewModelNotifier_delete(wxDataViewModelNotifier *self);
bool wxDataViewModelNotifier_Cleared(wxDataViewModelNotifier * self);
wxDataViewModel * wxDataViewModelNotifier_GetOwner(const wxDataViewModelNotifier * self);
bool wxDataViewModelNotifier_ItemAdded(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItem * item);
bool wxDataViewModelNotifier_ItemChanged(wxDataViewModelNotifier * self, const wxDataViewItem * item);
bool wxDataViewModelNotifier_ItemDeleted(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItem * item);
bool wxDataViewModelNotifier_ItemsAdded(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItemArray * items);
bool wxDataViewModelNotifier_ItemsChanged(wxDataViewModelNotifier * self, const wxDataViewItemArray * items);
bool wxDataViewModelNotifier_ItemsDeleted(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItemArray * items);
void wxDataViewModelNotifier_Resort(wxDataViewModelNotifier * self);
void wxDataViewModelNotifier_SetOwner(wxDataViewModelNotifier * self, wxDataViewModel * owner);
bool wxDataViewModelNotifier_ValueChanged(wxDataViewModelNotifier * self, const wxDataViewItem * item, unsigned int col);

// CLASS: wxDataViewProgressRenderer
wxClassInfo *wxDataViewProgressRenderer_CLASSINFO();
wxString *wxDataViewProgressRenderer_GetDefaultType();

// CLASS: wxDataViewRenderer
wxClassInfo *wxDataViewRenderer_CLASSINFO();
void wxDataViewRenderer_EnableEllipsize(wxDataViewRenderer * self, wxEllipsizeMode mode);
void wxDataViewRenderer_DisableEllipsize(wxDataViewRenderer * self);
int wxDataViewRenderer_GetAlignment(const wxDataViewRenderer * self);
wxEllipsizeMode wxDataViewRenderer_GetEllipsizeMode(const wxDataViewRenderer * self);
wxDataViewColumn * wxDataViewRenderer_GetOwner(const wxDataViewRenderer * self);
bool wxDataViewRenderer_GetValue(const wxDataViewRenderer * self, wxVariant * value);
wxString *wxDataViewRenderer_GetVariantType(const wxDataViewRenderer * self);
#if wxCHECK_VERSION(3, 1, 7)
bool wxDataViewRenderer_IsCompatibleVariantType(const wxDataViewRenderer * self, const wxString * variant_type);
#endif
void wxDataViewRenderer_SetAlignment(wxDataViewRenderer * self, int align);
void wxDataViewRenderer_SetOwner(wxDataViewRenderer * self, wxDataViewColumn * owner);
bool wxDataViewRenderer_SetValue(wxDataViewRenderer * self, const wxVariant * value);
void wxDataViewRenderer_SetValueAdjuster(wxDataViewRenderer * self, wxDataViewValueAdjuster * transformer);
bool wxDataViewRenderer_Validate(wxDataViewRenderer * self, wxVariant * value);
bool wxDataViewRenderer_HasEditorCtrl(const wxDataViewRenderer * self);
bool wxDataViewRenderer_GetValueFromEditorCtrl(wxDataViewRenderer * self, wxWindow * editor, wxVariant * value);
void wxDataViewRenderer_CancelEditing(wxDataViewRenderer * self);
bool wxDataViewRenderer_FinishEditing(wxDataViewRenderer * self);
wxWindow * wxDataViewRenderer_GetEditorCtrl(wxDataViewRenderer * self);

// CLASS: wxDataViewSpinRenderer
wxClassInfo *wxDataViewSpinRenderer_CLASSINFO();

// CLASS: wxDataViewTextRenderer
wxClassInfo *wxDataViewTextRenderer_CLASSINFO();
wxString *wxDataViewTextRenderer_GetDefaultType();
void wxDataViewTextRenderer_EnableMarkup(wxDataViewTextRenderer * self, bool enable);

// CLASS: wxDataViewToggleRenderer
wxClassInfo *wxDataViewToggleRenderer_CLASSINFO();
wxString *wxDataViewToggleRenderer_GetDefaultType();
void wxDataViewToggleRenderer_ShowAsRadio(wxDataViewToggleRenderer * self);

// CLASS: wxDataViewTreeCtrl
wxClassInfo *wxDataViewTreeCtrl_CLASSINFO();
wxDataViewTreeCtrl *wxDataViewTreeCtrl_new();
wxDataViewTreeCtrl *wxDataViewTreeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator);
wxDataViewItem *wxDataViewTreeCtrl_AppendContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, int expanded, wxClientData * data);
wxDataViewItem *wxDataViewTreeCtrl_AppendItem(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, wxClientData * data);
bool wxDataViewTreeCtrl_Create(wxDataViewTreeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator);
void wxDataViewTreeCtrl_DeleteAllItems(wxDataViewTreeCtrl * self);
void wxDataViewTreeCtrl_DeleteChildren(wxDataViewTreeCtrl * self, const wxDataViewItem * item);
void wxDataViewTreeCtrl_DeleteItem(wxDataViewTreeCtrl * self, const wxDataViewItem * item);
int wxDataViewTreeCtrl_GetChildCount(const wxDataViewTreeCtrl * self, const wxDataViewItem * parent);
wxImageList * wxDataViewTreeCtrl_GetImageList(wxDataViewTreeCtrl * self);
wxClientData * wxDataViewTreeCtrl_GetItemData(const wxDataViewTreeCtrl * self, const wxDataViewItem * item);
wxIcon *wxDataViewTreeCtrl_GetItemExpandedIcon(const wxDataViewTreeCtrl * self, const wxDataViewItem * item);
wxIcon *wxDataViewTreeCtrl_GetItemIcon(const wxDataViewTreeCtrl * self, const wxDataViewItem * item);
wxString *wxDataViewTreeCtrl_GetItemText(const wxDataViewTreeCtrl * self, const wxDataViewItem * item);
wxDataViewItem *wxDataViewTreeCtrl_GetNthChild(const wxDataViewTreeCtrl * self, const wxDataViewItem * parent, unsigned int pos);
const wxDataViewTreeStore * wxDataViewTreeCtrl_GetStore1(const wxDataViewTreeCtrl * self);
wxDataViewItem *wxDataViewTreeCtrl_InsertContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, int icon, int expanded, wxClientData * data);
wxDataViewItem *wxDataViewTreeCtrl_InsertItem(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, int icon, wxClientData * data);
bool wxDataViewTreeCtrl_IsContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * item);
wxDataViewItem *wxDataViewTreeCtrl_PrependContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, int expanded, wxClientData * data);
wxDataViewItem *wxDataViewTreeCtrl_PrependItem(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, wxClientData * data);
void wxDataViewTreeCtrl_SetImageList(wxDataViewTreeCtrl * self, wxImageList * imagelist);
void wxDataViewTreeCtrl_SetItemData(wxDataViewTreeCtrl * self, const wxDataViewItem * item, wxClientData * data);
void wxDataViewTreeCtrl_SetItemExpandedIcon(wxDataViewTreeCtrl * self, const wxDataViewItem * item, const wxBitmapBundle * icon);
void wxDataViewTreeCtrl_SetItemIcon(wxDataViewTreeCtrl * self, const wxDataViewItem * item, const wxBitmapBundle * icon);
void wxDataViewTreeCtrl_SetItemText(wxDataViewTreeCtrl * self, const wxDataViewItem * item, const wxString * text);

// CLASS: wxDataViewTreeStore
wxDataViewTreeStore *wxDataViewTreeStore_new();
wxDataViewItem *wxDataViewTreeStore_AppendContainer(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, const wxBitmapBundle * expanded, wxClientData * data);
wxDataViewItem *wxDataViewTreeStore_AppendItem(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, wxClientData * data);
void wxDataViewTreeStore_DeleteAllItems(wxDataViewTreeStore * self);
void wxDataViewTreeStore_DeleteChildren(wxDataViewTreeStore * self, const wxDataViewItem * item);
void wxDataViewTreeStore_DeleteItem(wxDataViewTreeStore * self, const wxDataViewItem * item);
int wxDataViewTreeStore_GetChildCount(const wxDataViewTreeStore * self, const wxDataViewItem * parent);
wxClientData * wxDataViewTreeStore_GetItemData(const wxDataViewTreeStore * self, const wxDataViewItem * item);
wxIcon *wxDataViewTreeStore_GetItemExpandedIcon(const wxDataViewTreeStore * self, const wxDataViewItem * item);
wxIcon *wxDataViewTreeStore_GetItemIcon(const wxDataViewTreeStore * self, const wxDataViewItem * item);
wxString *wxDataViewTreeStore_GetItemText(const wxDataViewTreeStore * self, const wxDataViewItem * item);
wxDataViewItem *wxDataViewTreeStore_GetNthChild(const wxDataViewTreeStore * self, const wxDataViewItem * parent, unsigned int pos);
wxDataViewItem *wxDataViewTreeStore_InsertContainer(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, const wxBitmapBundle * icon, const wxBitmapBundle * expanded, wxClientData * data);
wxDataViewItem *wxDataViewTreeStore_InsertItem(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, const wxBitmapBundle * icon, wxClientData * data);
wxDataViewItem *wxDataViewTreeStore_PrependContainer(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, const wxBitmapBundle * expanded, wxClientData * data);
wxDataViewItem *wxDataViewTreeStore_PrependItem(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, wxClientData * data);
void wxDataViewTreeStore_SetItemData(wxDataViewTreeStore * self, const wxDataViewItem * item, wxClientData * data);
void wxDataViewTreeStore_SetItemExpandedIcon(wxDataViewTreeStore * self, const wxDataViewItem * item, const wxBitmapBundle * icon);
void wxDataViewTreeStore_SetItemIcon(wxDataViewTreeStore * self, const wxDataViewItem * item, const wxBitmapBundle * icon);

// CLASS: wxDataViewValueAdjuster
void wxDataViewValueAdjuster_delete(wxDataViewValueAdjuster *self);

// CLASS: wxDataViewVirtualListModel
wxDataViewItem *wxDataViewVirtualListModel_GetItem(const wxDataViewVirtualListModel * self, unsigned int row);
void wxDataViewVirtualListModel_Reset(wxDataViewVirtualListModel * self, unsigned int new_size);
void wxDataViewVirtualListModel_RowAppended(wxDataViewVirtualListModel * self);
void wxDataViewVirtualListModel_RowChanged(wxDataViewVirtualListModel * self, unsigned int row);
void wxDataViewVirtualListModel_RowDeleted(wxDataViewVirtualListModel * self, unsigned int row);
void wxDataViewVirtualListModel_RowInserted(wxDataViewVirtualListModel * self, unsigned int before);
void wxDataViewVirtualListModel_RowPrepended(wxDataViewVirtualListModel * self);
void wxDataViewVirtualListModel_RowValueChanged(wxDataViewVirtualListModel * self, unsigned int row, unsigned int col);
void wxDataViewVirtualListModel_RowsDeleted(wxDataViewVirtualListModel * self, const wxArrayInt * rows);

// CLASS: wxDateEvent
wxClassInfo *wxDateEvent_CLASSINFO();
wxDateEvent *wxDateEvent_new();
wxDateTime *wxDateEvent_GetDate(const wxDateEvent * self);
void wxDateEvent_SetDate(wxDateEvent * self, const wxDateTime * date);

// CLASS: wxDatePickerCtrl
wxClassInfo *wxDatePickerCtrl_CLASSINFO();
wxDatePickerCtrl *wxDatePickerCtrl_new();
wxDatePickerCtrl *wxDatePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_Create(wxDatePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDatePickerCtrl_GetRange(const wxDatePickerCtrl * self, wxDateTime * dt1, wxDateTime * dt2);
wxDateTime *wxDatePickerCtrl_GetValue(const wxDatePickerCtrl * self);
#if wxCHECK_VERSION(3, 1, 0)
void wxDatePickerCtrl_SetNullText(wxDatePickerCtrl * self, const wxString * text);
#endif
void wxDatePickerCtrl_SetRange(wxDatePickerCtrl * self, const wxDateTime * dt1, const wxDateTime * dt2);
void wxDatePickerCtrl_SetValue(wxDatePickerCtrl * self, const wxDateTime * dt);

// CLASS: wxDelegateRendererNative
void wxDelegateRendererNative_delete(wxDelegateRendererNative *self);
wxDelegateRendererNative *wxDelegateRendererNative_new();
wxDelegateRendererNative *wxDelegateRendererNative_new1(wxRendererNative * renderer_native);

// CLASS: wxDialog
wxClassInfo *wxDialog_CLASSINFO();
wxDialog *wxDialog_new();
wxDialog *wxDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
void wxDialog_AddMainButtonId(wxDialog * self, wxWindowID id);
bool wxDialog_CanDoLayoutAdaptation(wxDialog * self);
void wxDialog_Centre(wxDialog * self, int direction);
bool wxDialog_Create(wxDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxSizer * wxDialog_CreateButtonSizer(wxDialog * self, long flags);
wxSizer * wxDialog_CreateSeparatedButtonSizer(wxDialog * self, long flags);
wxSizer * wxDialog_CreateSeparatedSizer(wxDialog * self, wxSizer * sizer);
wxStdDialogButtonSizer * wxDialog_CreateStdDialogButtonSizer(wxDialog * self, long flags);
#if wxCHECK_VERSION(3, 1, 0)
wxSizer * wxDialog_CreateTextSizer(wxDialog * self, const wxString * message, int width_max);
#endif
bool wxDialog_DoLayoutAdaptation(wxDialog * self);
void wxDialog_EndModal(wxDialog * self, int ret_code);
int wxDialog_GetAffirmativeId(const wxDialog * self);
wxWindow * wxDialog_GetContentWindow(const wxDialog * self);
int wxDialog_GetEscapeId(const wxDialog * self);
bool wxDialog_GetLayoutAdaptationDone(const wxDialog * self);
int wxDialog_GetLayoutAdaptationLevel(const wxDialog * self);
wxArrayInt * wxDialog_GetMainButtonIds(wxDialog * self);
int wxDialog_GetReturnCode(const wxDialog * self);
bool wxDialog_IsMainButtonId(const wxDialog * self, wxWindowID id);
bool wxDialog_IsModal(const wxDialog * self);
void wxDialog_SetAffirmativeId(wxDialog * self, int id);
void wxDialog_SetEscapeId(wxDialog * self, int id);
void wxDialog_SetIcon(wxDialog * self, const wxIcon * icon);
void wxDialog_SetLayoutAdaptationDone(wxDialog * self, bool done);
void wxDialog_SetLayoutAdaptationLevel(wxDialog * self, int level);
void wxDialog_SetReturnCode(wxDialog * self, int ret_code);
int wxDialog_ShowModal(wxDialog * self);
void wxDialog_ShowWindowModal(wxDialog * self);
void wxDialog_EnableLayoutAdaptation(bool enable);
wxDialogLayoutAdapter * wxDialog_GetLayoutAdapter();
bool wxDialog_IsLayoutAdaptationEnabled();
wxDialogLayoutAdapter * wxDialog_SetLayoutAdapter(wxDialogLayoutAdapter * adapter);

// CLASS: wxDialogLayoutAdapter
void wxDialogLayoutAdapter_delete(wxDialogLayoutAdapter *self);
bool wxDialogLayoutAdapter_CanDoLayoutAdaptation(wxDialogLayoutAdapter * self, wxDialog * dialog);
bool wxDialogLayoutAdapter_DoLayoutAdaptation(wxDialogLayoutAdapter * self, wxDialog * dialog);

// CLASS: wxDirDialog
wxClassInfo *wxDirDialog_CLASSINFO();
wxDirDialog *wxDirDialog_new(wxWindow * parent, const wxString * message, const wxString * default_path, long style, const wxPoint * pos, const wxSize * size, const wxString * name);
wxString *wxDirDialog_GetMessage(const wxDirDialog * self);
wxString *wxDirDialog_GetPath(const wxDirDialog * self);
void wxDirDialog_GetPaths(const wxDirDialog * self, wxArrayString * paths);
void wxDirDialog_SetMessage(wxDirDialog * self, const wxString * message);
void wxDirDialog_SetPath(wxDirDialog * self, const wxString * path);

// CLASS: wxDirPickerCtrl
wxClassInfo *wxDirPickerCtrl_CLASSINFO();
wxDirPickerCtrl *wxDirPickerCtrl_new();
wxDirPickerCtrl *wxDirPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
bool wxDirPickerCtrl_Create(wxDirPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name);
wxFileName *wxDirPickerCtrl_GetDirName(const wxDirPickerCtrl * self);
wxString *wxDirPickerCtrl_GetPath(const wxDirPickerCtrl * self);
void wxDirPickerCtrl_SetDirName(wxDirPickerCtrl * self, const wxFileName * dirname);
void wxDirPickerCtrl_SetInitialDirectory(wxDirPickerCtrl * self, const wxString * dir);
void wxDirPickerCtrl_SetPath(wxDirPickerCtrl * self, const wxString * dirname);

// CLASS: wxDisplay
void wxDisplay_delete(wxDisplay *self);
wxDisplay *wxDisplay_new();
wxDisplay *wxDisplay_new1(unsigned int index);
wxDisplay *wxDisplay_new2(const wxWindow * window);
bool wxDisplay_ChangeMode(wxDisplay * self, const wxVideoMode * mode);
wxRect *wxDisplay_GetClientArea(const wxDisplay * self);
wxRect *wxDisplay_GetGeometry(const wxDisplay * self);
wxString *wxDisplay_GetName(const wxDisplay * self);
wxSize *wxDisplay_GetPPI(const wxDisplay * self);
double wxDisplay_GetScaleFactor(const wxDisplay * self);
bool wxDisplay_IsPrimary(const wxDisplay * self);
unsigned int wxDisplay_GetCount();
int wxDisplay_GetFromPoint(const wxPoint * pt);
int wxDisplay_GetFromWindow(const wxWindow * win);
int wxDisplay_GetStdPPIValue();
wxSize *wxDisplay_GetStdPPI();

// CLASS: wxDisplayChangedEvent
wxClassInfo *wxDisplayChangedEvent_CLASSINFO();
wxDisplayChangedEvent *wxDisplayChangedEvent_new();

// CLASS: wxDragImage
wxClassInfo *wxDragImage_CLASSINFO();
wxDragImage *wxDragImage_new();
wxDragImage *wxDragImage_new1(const wxBitmap * image, const wxCursor * cursor);
wxDragImage *wxDragImage_new2(const wxIcon * image, const wxCursor * cursor);
wxDragImage *wxDragImage_new3(const wxString * text, const wxCursor * cursor);
wxDragImage *wxDragImage_new4(const wxTreeCtrl * tree_ctrl, wxTreeItemId * id);
wxDragImage *wxDragImage_new5(const wxListCtrl * list_ctrl, long id);
bool wxDragImage_BeginDrag(wxDragImage * self, const wxPoint * hotspot, wxWindow * window, bool full_screen, wxRect * rect);
bool wxDragImage_BeginDrag1(wxDragImage * self, const wxPoint * hotspot, wxWindow * window, wxWindow * bounding_window);
#ifndef __WXMSW__
bool wxDragImage_DoDrawImage(const wxDragImage * self, wxDC * dc, const wxPoint * pos);
#endif
bool wxDragImage_EndDrag(wxDragImage * self);
#ifndef __WXMSW__
wxRect *wxDragImage_GetImageRect(const wxDragImage * self, const wxPoint * pos);
#endif
bool wxDragImage_Hide(wxDragImage * self);
bool wxDragImage_Move(wxDragImage * self, const wxPoint * pt);
bool wxDragImage_Show(wxDragImage * self);
#ifndef __WXMSW__
bool wxDragImage_UpdateBackingFromWindow(const wxDragImage * self, wxDC * window_dc, wxMemoryDC * dest_dc, const wxRect * source_rect, const wxRect * dest_rect);
#endif

// CLASS: wxDropFilesEvent
wxClassInfo *wxDropFilesEvent_CLASSINFO();
int wxDropFilesEvent_GetNumberOfFiles(const wxDropFilesEvent * self);
wxPoint *wxDropFilesEvent_GetPosition(const wxDropFilesEvent * self);

// CLASS: wxDropSource
void wxDropSource_delete(wxDropSource *self);
#ifndef __WXGTK__
wxDropSource *wxDropSource_new(wxWindow * win, const wxCursor * icon_copy, const wxCursor * icon_move, const wxCursor * icon_none);
wxDropSource *wxDropSource_new1(wxDataObject * data, wxWindow * win, const wxCursor * icon_copy, const wxCursor * icon_move, const wxCursor * icon_none);
#endif
#ifdef __WXGTK__
wxDropSource *wxDropSource_new2(wxWindow * win, const wxIcon * icon_copy, const wxIcon * icon_move, const wxIcon * icon_none);
wxDropSource *wxDropSource_new3(wxDataObject * data, wxWindow * win, const wxIcon * icon_copy, const wxIcon * icon_move, const wxIcon * icon_none);
#endif
wxDataObject * wxDropSource_GetDataObject(wxDropSource * self);
void wxDropSource_SetData(wxDropSource * self, wxDataObject * data);

// CLASS: wxDropTarget
void wxDropTarget_delete(wxDropTarget *self);
bool wxDropTarget_GetData(wxDropTarget * self);
bool wxDropTarget_OnDrop(wxDropTarget * self, wxCoord x, wxCoord y);
void wxDropTarget_OnLeave(wxDropTarget * self);
wxDataObject * wxDropTarget_GetDataObject(const wxDropTarget * self);
void wxDropTarget_SetDataObject(wxDropTarget * self, wxDataObject * data);

} // extern "C"

