#include "generated.h"

extern "C" {

// CLASS: wxDC
wxClassInfo *wxDC_CLASSINFO() {
    return wxCLASSINFO(wxDC);
}
wxCoord wxDC_DeviceToLogicalX(const wxDC * self, wxCoord x) {
    return self->DeviceToLogicalX(x);
}
wxCoord wxDC_DeviceToLogicalXRel(const wxDC * self, wxCoord x) {
    return self->DeviceToLogicalXRel(x);
}
wxCoord wxDC_DeviceToLogicalY(const wxDC * self, wxCoord y) {
    return self->DeviceToLogicalY(y);
}
wxCoord wxDC_DeviceToLogicalYRel(const wxDC * self, wxCoord y) {
    return self->DeviceToLogicalYRel(y);
}
wxCoord wxDC_LogicalToDeviceX(const wxDC * self, wxCoord x) {
    return self->LogicalToDeviceX(x);
}
wxCoord wxDC_LogicalToDeviceXRel(const wxDC * self, wxCoord x) {
    return self->LogicalToDeviceXRel(x);
}
wxCoord wxDC_LogicalToDeviceY(const wxDC * self, wxCoord y) {
    return self->LogicalToDeviceY(y);
}
wxCoord wxDC_LogicalToDeviceYRel(const wxDC * self, wxCoord y) {
    return self->LogicalToDeviceYRel(y);
}
wxPoint *wxDC_DeviceToLogical(const wxDC * self, wxCoord x, wxCoord y) {
    return new wxPoint(self->DeviceToLogical(x, y));
}
wxPoint *wxDC_DeviceToLogical1(const wxDC * self, const wxPoint * pt) {
    return new wxPoint(self->DeviceToLogical(*pt));
}
wxSize *wxDC_DeviceToLogicalRel(const wxDC * self, int x, int y) {
    return new wxSize(self->DeviceToLogicalRel(x, y));
}
wxSize *wxDC_DeviceToLogicalRel1(const wxDC * self, const wxSize * dim) {
    return new wxSize(self->DeviceToLogicalRel(*dim));
}
wxPoint *wxDC_LogicalToDevice(const wxDC * self, wxCoord x, wxCoord y) {
    return new wxPoint(self->LogicalToDevice(x, y));
}
wxPoint *wxDC_LogicalToDevice1(const wxDC * self, const wxPoint * pt) {
    return new wxPoint(self->LogicalToDevice(*pt));
}
wxSize *wxDC_LogicalToDeviceRel(const wxDC * self, int x, int y) {
    return new wxSize(self->LogicalToDeviceRel(x, y));
}
wxSize *wxDC_LogicalToDeviceRel1(const wxDC * self, const wxSize * dim) {
    return new wxSize(self->LogicalToDeviceRel(*dim));
}
void wxDC_Clear(wxDC * self) {
    return self->Clear();
}
void wxDC_DrawArc(wxDC * self, wxCoord x_start, wxCoord y_start, wxCoord x_end, wxCoord y_end, wxCoord xc, wxCoord yc) {
    return self->DrawArc(x_start, y_start, x_end, y_end, xc, yc);
}
void wxDC_DrawArc1(wxDC * self, const wxPoint * pt_start, const wxPoint * pt_end, const wxPoint * centre) {
    return self->DrawArc(*pt_start, *pt_end, *centre);
}
void wxDC_DrawBitmap(wxDC * self, const wxBitmap * bitmap, wxCoord x, wxCoord y, bool use_mask) {
    return self->DrawBitmap(*bitmap, x, y, use_mask);
}
void wxDC_DrawBitmap1(wxDC * self, const wxBitmap * bmp, const wxPoint * pt, bool use_mask) {
    return self->DrawBitmap(*bmp, *pt, use_mask);
}
void wxDC_DrawCheckMark(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->DrawCheckMark(x, y, width, height);
}
void wxDC_DrawCheckMark1(wxDC * self, const wxRect * rect) {
    return self->DrawCheckMark(*rect);
}
void wxDC_DrawCircle(wxDC * self, wxCoord x, wxCoord y, wxCoord radius) {
    return self->DrawCircle(x, y, radius);
}
void wxDC_DrawCircle1(wxDC * self, const wxPoint * pt, wxCoord radius) {
    return self->DrawCircle(*pt, radius);
}
void wxDC_DrawEllipse(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->DrawEllipse(x, y, width, height);
}
void wxDC_DrawEllipse1(wxDC * self, const wxPoint * pt, const wxSize * size) {
    return self->DrawEllipse(*pt, *size);
}
void wxDC_DrawEllipse2(wxDC * self, const wxRect * rect) {
    return self->DrawEllipse(*rect);
}
void wxDC_DrawEllipticArc(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height, double start, double end) {
    return self->DrawEllipticArc(x, y, width, height, start, end);
}
void wxDC_DrawEllipticArc1(wxDC * self, const wxPoint * pt, const wxSize * sz, double sa, double ea) {
    return self->DrawEllipticArc(*pt, *sz, sa, ea);
}
void wxDC_DrawIcon(wxDC * self, const wxIcon * icon, wxCoord x, wxCoord y) {
    return self->DrawIcon(*icon, x, y);
}
void wxDC_DrawIcon1(wxDC * self, const wxIcon * icon, const wxPoint * pt) {
    return self->DrawIcon(*icon, *pt);
}
void wxDC_DrawLabel(wxDC * self, const wxString * text, const wxBitmap * bitmap, const wxRect * rect, int alignment, int index_accel, wxRect * rect_bounding) {
    return self->DrawLabel(*text, *bitmap, *rect, alignment, index_accel, rect_bounding);
}
void wxDC_DrawLabel1(wxDC * self, const wxString * text, const wxRect * rect, int alignment, int index_accel) {
    return self->DrawLabel(*text, *rect, alignment, index_accel);
}
void wxDC_DrawLine(wxDC * self, wxCoord x1, wxCoord y1, wxCoord x2, wxCoord y2) {
    return self->DrawLine(x1, y1, x2, y2);
}
void wxDC_DrawLine1(wxDC * self, const wxPoint * pt1, const wxPoint * pt2) {
    return self->DrawLine(*pt1, *pt2);
}
void wxDC_DrawLines1(wxDC * self, const wxPointList * points, wxCoord xoffset, wxCoord yoffset) {
    return self->DrawLines(points, xoffset, yoffset);
}
void wxDC_DrawPoint(wxDC * self, wxCoord x, wxCoord y) {
    return self->DrawPoint(x, y);
}
void wxDC_DrawPoint1(wxDC * self, const wxPoint * pt) {
    return self->DrawPoint(*pt);
}
void wxDC_DrawRectangle(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->DrawRectangle(x, y, width, height);
}
void wxDC_DrawRectangle1(wxDC * self, const wxPoint * pt, const wxSize * sz) {
    return self->DrawRectangle(*pt, *sz);
}
void wxDC_DrawRectangle2(wxDC * self, const wxRect * rect) {
    return self->DrawRectangle(*rect);
}
void wxDC_DrawRotatedText(wxDC * self, const wxString * text, wxCoord x, wxCoord y, double angle) {
    return self->DrawRotatedText(*text, x, y, angle);
}
void wxDC_DrawRotatedText1(wxDC * self, const wxString * text, const wxPoint * point, double angle) {
    return self->DrawRotatedText(*text, *point, angle);
}
void wxDC_DrawRoundedRectangle(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height, double radius) {
    return self->DrawRoundedRectangle(x, y, width, height, radius);
}
void wxDC_DrawRoundedRectangle1(wxDC * self, const wxPoint * pt, const wxSize * sz, double radius) {
    return self->DrawRoundedRectangle(*pt, *sz, radius);
}
void wxDC_DrawRoundedRectangle2(wxDC * self, const wxRect * rect, double radius) {
    return self->DrawRoundedRectangle(*rect, radius);
}
void wxDC_DrawSpline1(wxDC * self, const wxPointList * points) {
    return self->DrawSpline(points);
}
void wxDC_DrawSpline2(wxDC * self, wxCoord x1, wxCoord y1, wxCoord x2, wxCoord y2, wxCoord x3, wxCoord y3) {
    return self->DrawSpline(x1, y1, x2, y2, x3, y3);
}
void wxDC_DrawText(wxDC * self, const wxString * text, wxCoord x, wxCoord y) {
    return self->DrawText(*text, x, y);
}
void wxDC_DrawText1(wxDC * self, const wxString * text, const wxPoint * pt) {
    return self->DrawText(*text, *pt);
}
void wxDC_GradientFillConcentric(wxDC * self, const wxRect * rect, const wxColour * initial_colour, const wxColour * dest_colour) {
    return self->GradientFillConcentric(*rect, *initial_colour, *dest_colour);
}
void wxDC_GradientFillConcentric1(wxDC * self, const wxRect * rect, const wxColour * initial_colour, const wxColour * dest_colour, const wxPoint * circle_center) {
    return self->GradientFillConcentric(*rect, *initial_colour, *dest_colour, *circle_center);
}
void wxDC_GradientFillLinear(wxDC * self, const wxRect * rect, const wxColour * initial_colour, const wxColour * dest_colour, wxDirection n_direction) {
    return self->GradientFillLinear(*rect, *initial_colour, *dest_colour, n_direction);
}
void wxDC_CrossHair(wxDC * self, wxCoord x, wxCoord y) {
    return self->CrossHair(x, y);
}
void wxDC_CrossHair1(wxDC * self, const wxPoint * pt) {
    return self->CrossHair(*pt);
}
void wxDC_DestroyClippingRegion(wxDC * self) {
    return self->DestroyClippingRegion();
}
bool wxDC_GetClippingBox(const wxDC * self, wxCoord * x, wxCoord * y, wxCoord * width, wxCoord * height) {
    return self->GetClippingBox(x, y, width, height);
}
bool wxDC_GetClippingBox1(const wxDC * self, wxRect * rect) {
    return self->GetClippingBox(*rect);
}
void wxDC_SetClippingRegion(wxDC * self, wxCoord x, wxCoord y, wxCoord width, wxCoord height) {
    return self->SetClippingRegion(x, y, width, height);
}
void wxDC_SetClippingRegion1(wxDC * self, const wxPoint * pt, const wxSize * sz) {
    return self->SetClippingRegion(*pt, *sz);
}
void wxDC_SetClippingRegion2(wxDC * self, const wxRect * rect) {
    return self->SetClippingRegion(*rect);
}
void wxDC_SetDeviceClippingRegion(wxDC * self, const wxRegion * region) {
    return self->SetDeviceClippingRegion(*region);
}
wxCoord wxDC_GetCharHeight(const wxDC * self) {
    return self->GetCharHeight();
}
wxCoord wxDC_GetCharWidth(const wxDC * self) {
    return self->GetCharWidth();
}
void wxDC_GetMultiLineTextExtent(const wxDC * self, const wxString * string, wxCoord * w, wxCoord * h, wxCoord * height_line, const wxFont * font) {
    return self->GetMultiLineTextExtent(*string, w, h, height_line, font);
}
wxSize *wxDC_GetMultiLineTextExtent1(const wxDC * self, const wxString * string) {
    return new wxSize(self->GetMultiLineTextExtent(*string));
}
bool wxDC_GetPartialTextExtents(const wxDC * self, const wxString * text, wxArrayInt * widths) {
    return self->GetPartialTextExtents(*text, *widths);
}
void wxDC_GetTextExtent(const wxDC * self, const wxString * string, wxCoord * w, wxCoord * h, wxCoord * descent, wxCoord * external_leading, const wxFont * font) {
    return self->GetTextExtent(*string, w, h, descent, external_leading, font);
}
wxSize *wxDC_GetTextExtent1(const wxDC * self, const wxString * string) {
    return new wxSize(self->GetTextExtent(*string));
}
int wxDC_GetBackgroundMode(const wxDC * self) {
    return self->GetBackgroundMode();
}
wxFont *wxDC_GetFont(const wxDC * self) {
    return new wxFont(self->GetFont());
}
wxLayoutDirection wxDC_GetLayoutDirection(const wxDC * self) {
    return self->GetLayoutDirection();
}
wxColour *wxDC_GetTextBackground(const wxDC * self) {
    return new wxColour(self->GetTextBackground());
}
wxColour *wxDC_GetTextForeground(const wxDC * self) {
    return new wxColour(self->GetTextForeground());
}
void wxDC_SetBackgroundMode(wxDC * self, int mode) {
    return self->SetBackgroundMode(mode);
}
void wxDC_SetFont(wxDC * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxDC_SetTextBackground(wxDC * self, const wxColour * colour) {
    return self->SetTextBackground(*colour);
}
void wxDC_SetTextForeground(wxDC * self, const wxColour * colour) {
    return self->SetTextForeground(*colour);
}
void wxDC_SetLayoutDirection(wxDC * self, wxLayoutDirection dir) {
    return self->SetLayoutDirection(dir);
}
void wxDC_CalcBoundingBox(wxDC * self, wxCoord x, wxCoord y) {
    return self->CalcBoundingBox(x, y);
}
wxCoord wxDC_MaxX(const wxDC * self) {
    return self->MaxX();
}
wxCoord wxDC_MaxY(const wxDC * self) {
    return self->MaxY();
}
wxCoord wxDC_MinX(const wxDC * self) {
    return self->MinX();
}
wxCoord wxDC_MinY(const wxDC * self) {
    return self->MinY();
}
void wxDC_ResetBoundingBox(wxDC * self) {
    return self->ResetBoundingBox();
}
bool wxDC_StartDoc(wxDC * self, const wxString * message) {
    return self->StartDoc(*message);
}
void wxDC_StartPage(wxDC * self) {
    return self->StartPage();
}
void wxDC_EndDoc(wxDC * self) {
    return self->EndDoc();
}
void wxDC_EndPage(wxDC * self) {
    return self->EndPage();
}
wxBrush *wxDC_GetBackground(const wxDC * self) {
    return new wxBrush(self->GetBackground());
}
wxBrush *wxDC_GetBrush(const wxDC * self) {
    return new wxBrush(self->GetBrush());
}
void wxDC_SetBackground(wxDC * self, const wxBrush * brush) {
    return self->SetBackground(*brush);
}
void wxDC_SetBrush(wxDC * self, const wxBrush * brush) {
    return self->SetBrush(*brush);
}
void wxDC_SetPen(wxDC * self, const wxPen * pen) {
    return self->SetPen(*pen);
}
void wxDC_CopyAttributes(wxDC * self, const wxDC * dc) {
    return self->CopyAttributes(*dc);
}
double wxDC_GetContentScaleFactor(const wxDC * self) {
    return self->GetContentScaleFactor();
}
int wxDC_GetDepth(const wxDC * self) {
    return self->GetDepth();
}
wxPoint *wxDC_GetDeviceOrigin(const wxDC * self) {
    return new wxPoint(self->GetDeviceOrigin());
}
bool wxDC_GetPixel(const wxDC * self, wxCoord x, wxCoord y, wxColour * colour) {
    return self->GetPixel(x, y, colour);
}
wxSize *wxDC_GetPPI(const wxDC * self) {
    return new wxSize(self->GetPPI());
}
wxSize *wxDC_FromDIP(const wxDC * self, const wxSize * sz) {
    return new wxSize(self->FromDIP(*sz));
}
wxPoint *wxDC_FromDIP1(const wxDC * self, const wxPoint * pt) {
    return new wxPoint(self->FromDIP(*pt));
}
int wxDC_FromDIP2(const wxDC * self, int d) {
    return self->FromDIP(d);
}
wxSize *wxDC_ToDIP(const wxDC * self, const wxSize * sz) {
    return new wxSize(self->ToDIP(*sz));
}
wxPoint *wxDC_ToDIP1(const wxDC * self, const wxPoint * pt) {
    return new wxPoint(self->ToDIP(*pt));
}
int wxDC_ToDIP2(const wxDC * self, int d) {
    return self->ToDIP(d);
}
void wxDC_GetSize(const wxDC * self, wxCoord * width, wxCoord * height) {
    return self->GetSize(width, height);
}
wxSize *wxDC_GetSize1(const wxDC * self) {
    return new wxSize(self->GetSize());
}
void wxDC_GetSizeMM(const wxDC * self, wxCoord * width, wxCoord * height) {
    return self->GetSizeMM(width, height);
}
wxSize *wxDC_GetSizeMM1(const wxDC * self) {
    return new wxSize(self->GetSizeMM());
}
void wxDC_GetUserScale(const wxDC * self, double * x, double * y) {
    return self->GetUserScale(x, y);
}
bool wxDC_IsOk(const wxDC * self) {
    return self->IsOk();
}
void wxDC_SetAxisOrientation(wxDC * self, bool x_left_right, bool y_bottom_up) {
    return self->SetAxisOrientation(x_left_right, y_bottom_up);
}
void wxDC_SetDeviceOrigin(wxDC * self, wxCoord x, wxCoord y) {
    return self->SetDeviceOrigin(x, y);
}
void wxDC_SetPalette(wxDC * self, const wxPalette * palette) {
    return self->SetPalette(*palette);
}
void wxDC_SetUserScale(wxDC * self, double x_scale, double y_scale) {
    return self->SetUserScale(x_scale, y_scale);
}
bool wxDC_CanUseTransformMatrix(const wxDC * self) {
    return self->CanUseTransformMatrix();
}
bool wxDC_SetTransformMatrix(wxDC * self, const wxAffineMatrix2D * matrix) {
    return self->SetTransformMatrix(*matrix);
}
wxAffineMatrix2D *wxDC_GetTransformMatrix(const wxDC * self) {
    return new wxAffineMatrix2D(self->GetTransformMatrix());
}
void wxDC_ResetTransformMatrix(wxDC * self) {
    return self->ResetTransformMatrix();
}
bool wxDC_CanDrawBitmap(const wxDC * self) {
    return self->CanDrawBitmap();
}
bool wxDC_CanGetTextExtent(const wxDC * self) {
    return self->CanGetTextExtent();
}
void * wxDC_GetHandle(const wxDC * self) {
    return self->GetHandle();
}
wxBitmap *wxDC_GetAsBitmap(const wxDC * self, const wxRect * subrect) {
    return new wxBitmap(self->GetAsBitmap(subrect));
}
void wxDC_SetLogicalScale(wxDC * self, double x, double y) {
    return self->SetLogicalScale(x, y);
}
void wxDC_GetLogicalScale(const wxDC * self, double * x, double * y) {
    return self->GetLogicalScale(x, y);
}
void wxDC_SetLogicalOrigin(wxDC * self, wxCoord x, wxCoord y) {
    return self->SetLogicalOrigin(x, y);
}
void wxDC_GetLogicalOrigin(const wxDC * self, wxCoord * x, wxCoord * y) {
    return self->GetLogicalOrigin(x, y);
}
wxPoint *wxDC_GetLogicalOrigin1(const wxDC * self) {
    return new wxPoint(self->GetLogicalOrigin());
}
wxGraphicsContext * wxDC_GetGraphicsContext(const wxDC * self) {
    return self->GetGraphicsContext();
}
void wxDC_SetGraphicsContext(wxDC * self, wxGraphicsContext * ctx) {
    return self->SetGraphicsContext(ctx);
}

// CLASS: wxDCBrushChanger
void wxDCBrushChanger_delete(wxDCBrushChanger *self) {
    delete self;
}
wxDCBrushChanger *wxDCBrushChanger_new(wxDC * dc, const wxBrush * brush) {
    return new wxDCBrushChanger(*dc, *brush);
}

// CLASS: wxDCClipper
void wxDCClipper_delete(wxDCClipper *self) {
    delete self;
}
wxDCClipper *wxDCClipper_new(wxDC * dc, const wxRegion * region) {
    return new wxDCClipper(*dc, *region);
}
wxDCClipper *wxDCClipper_new1(wxDC * dc, const wxRect * rect) {
    return new wxDCClipper(*dc, *rect);
}
wxDCClipper *wxDCClipper_new2(wxDC * dc, wxCoord x, wxCoord y, wxCoord w, wxCoord h) {
    return new wxDCClipper(*dc, x, y, w, h);
}

// CLASS: wxDCFontChanger
void wxDCFontChanger_delete(wxDCFontChanger *self) {
    delete self;
}
wxDCFontChanger *wxDCFontChanger_new(wxDC * dc) {
    return new wxDCFontChanger(*dc);
}
wxDCFontChanger *wxDCFontChanger_new1(wxDC * dc, const wxFont * font) {
    return new wxDCFontChanger(*dc, *font);
}
void wxDCFontChanger_Set(wxDCFontChanger * self, const wxFont * font) {
    return self->Set(*font);
}

// CLASS: wxDCOverlay
void wxDCOverlay_delete(wxDCOverlay *self) {
    delete self;
}
wxDCOverlay *wxDCOverlay_new(wxOverlay * overlay, wxDC * dc, int x, int y, int width, int height) {
    return new wxDCOverlay(*overlay, dc, x, y, width, height);
}
wxDCOverlay *wxDCOverlay_new1(wxOverlay * overlay, wxDC * dc) {
    return new wxDCOverlay(*overlay, dc);
}
void wxDCOverlay_Clear(wxDCOverlay * self) {
    return self->Clear();
}

// CLASS: wxDCPenChanger
void wxDCPenChanger_delete(wxDCPenChanger *self) {
    delete self;
}
wxDCPenChanger *wxDCPenChanger_new(wxDC * dc, const wxPen * pen) {
    return new wxDCPenChanger(*dc, *pen);
}

// CLASS: wxDCTextBgColourChanger
void wxDCTextBgColourChanger_delete(wxDCTextBgColourChanger *self) {
    delete self;
}
wxDCTextBgColourChanger *wxDCTextBgColourChanger_new(wxDC * dc) {
    return new wxDCTextBgColourChanger(*dc);
}
wxDCTextBgColourChanger *wxDCTextBgColourChanger_new1(wxDC * dc, const wxColour * col) {
    return new wxDCTextBgColourChanger(*dc, *col);
}
void wxDCTextBgColourChanger_Set(wxDCTextBgColourChanger * self, const wxColour * col) {
    return self->Set(*col);
}

// CLASS: wxDCTextBgModeChanger
void wxDCTextBgModeChanger_delete(wxDCTextBgModeChanger *self) {
    delete self;
}

// CLASS: wxDCTextColourChanger
void wxDCTextColourChanger_delete(wxDCTextColourChanger *self) {
    delete self;
}
wxDCTextColourChanger *wxDCTextColourChanger_new(wxDC * dc) {
    return new wxDCTextColourChanger(*dc);
}
wxDCTextColourChanger *wxDCTextColourChanger_new1(wxDC * dc, const wxColour * col) {
    return new wxDCTextColourChanger(*dc, *col);
}
void wxDCTextColourChanger_Set(wxDCTextColourChanger * self, const wxColour * col) {
    return self->Set(*col);
}

// CLASS: wxDPIChangedEvent
wxClassInfo *wxDPIChangedEvent_CLASSINFO() {
    return wxCLASSINFO(wxDPIChangedEvent);
}
wxSize *wxDPIChangedEvent_GetOldDPI(const wxDPIChangedEvent * self) {
    return new wxSize(self->GetOldDPI());
}
wxSize *wxDPIChangedEvent_GetNewDPI(const wxDPIChangedEvent * self) {
    return new wxSize(self->GetNewDPI());
}
wxSize *wxDPIChangedEvent_Scale(const wxDPIChangedEvent * self, wxSize sz) {
    return new wxSize(self->Scale(sz));
}
int wxDPIChangedEvent_ScaleX(const wxDPIChangedEvent * self, int x) {
    return self->ScaleX(x);
}
int wxDPIChangedEvent_ScaleY(const wxDPIChangedEvent * self, int y) {
    return self->ScaleY(y);
}

// CLASS: wxDataFormat
void wxDataFormat_delete(wxDataFormat *self) {
    delete self;
}
wxDataFormat *wxDataFormat_new1(const wxString * format) {
    return new wxDataFormat(*format);
}
wxString *wxDataFormat_GetId(const wxDataFormat * self) {
    return new wxString(self->GetId());
}
void wxDataFormat_SetId(wxDataFormat * self, const wxString * format) {
    return self->SetId(*format);
}

// CLASS: wxDataObject
void wxDataObject_delete(wxDataObject *self) {
    delete self;
}
bool wxDataObject_GetDataHere(const wxDataObject * self, const wxDataFormat * format, void * buf) {
    return self->GetDataHere(*format, buf);
}
size_t wxDataObject_GetDataSize(const wxDataObject * self, const wxDataFormat * format) {
    return self->GetDataSize(*format);
}
bool wxDataObject_SetData(wxDataObject * self, const wxDataFormat * format, size_t len, const void * buf) {
    return self->SetData(*format, len, buf);
}

// CLASS: wxDataObjectComposite
void wxDataObjectComposite_delete(wxDataObjectComposite *self) {
    delete self;
}
wxDataObjectComposite *wxDataObjectComposite_new() {
    return new wxDataObjectComposite();
}
void wxDataObjectComposite_Add(wxDataObjectComposite * self, wxDataObjectSimple * data_object, bool preferred) {
    return self->Add(data_object, preferred);
}
wxDataFormat *wxDataObjectComposite_GetReceivedFormat(const wxDataObjectComposite * self) {
    return new wxDataFormat(self->GetReceivedFormat());
}

// CLASS: wxDataObjectSimple
void wxDataObjectSimple_delete(wxDataObjectSimple *self) {
    delete self;
}
wxDataObjectSimple *wxDataObjectSimple_new(const wxDataFormat * format) {
    return new wxDataObjectSimple(*format);
}
bool wxDataObjectSimple_GetDataHere(const wxDataObjectSimple * self, void * buf) {
    return self->GetDataHere(buf);
}
size_t wxDataObjectSimple_GetDataSize(const wxDataObjectSimple * self) {
    return self->GetDataSize();
}
bool wxDataObjectSimple_SetData(wxDataObjectSimple * self, size_t len, const void * buf) {
    return self->SetData(len, buf);
}
void wxDataObjectSimple_SetFormat(wxDataObjectSimple * self, const wxDataFormat * format) {
    return self->SetFormat(*format);
}

// CLASS: wxDataViewBitmapRenderer
wxClassInfo *wxDataViewBitmapRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewBitmapRenderer);
}
wxString *wxDataViewBitmapRenderer_GetDefaultType() {
    return new wxString(wxDataViewBitmapRenderer::GetDefaultType());
}

// CLASS: wxDataViewCheckIconTextRenderer
wxClassInfo *wxDataViewCheckIconTextRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewCheckIconTextRenderer);
}
wxString *wxDataViewCheckIconTextRenderer_GetDefaultType() {
    return new wxString(wxDataViewCheckIconTextRenderer::GetDefaultType());
}
void wxDataViewCheckIconTextRenderer_Allow3rdStateForUser(wxDataViewCheckIconTextRenderer * self, bool allow) {
    return self->Allow3rdStateForUser(allow);
}

// CLASS: wxDataViewChoiceByIndexRenderer
wxClassInfo *wxDataViewChoiceByIndexRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewChoiceByIndexRenderer);
}

// CLASS: wxDataViewChoiceRenderer
wxClassInfo *wxDataViewChoiceRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewChoiceRenderer);
}
wxString *wxDataViewChoiceRenderer_GetChoice(const wxDataViewChoiceRenderer * self, size_t index) {
    return new wxString(self->GetChoice(index));
}
wxArrayString *wxDataViewChoiceRenderer_GetChoices(const wxDataViewChoiceRenderer * self) {
    return new wxArrayString(self->GetChoices());
}

// CLASS: wxDataViewColumn
void wxDataViewColumn_delete(wxDataViewColumn *self) {
    delete self;
}
wxDataViewColumn *wxDataViewColumn_new(const wxString * title, wxDataViewRenderer * renderer, unsigned int model_column, int width, wxAlignment align, int flags) {
    return new wxDataViewColumn(*title, renderer, model_column, width, align, flags);
}
wxDataViewColumn *wxDataViewColumn_new1(const wxBitmapBundle * bitmap, wxDataViewRenderer * renderer, unsigned int model_column, int width, wxAlignment align, int flags) {
    return new wxDataViewColumn(*bitmap, renderer, model_column, width, align, flags);
}
unsigned int wxDataViewColumn_GetModelColumn(const wxDataViewColumn * self) {
    return self->GetModelColumn();
}
wxDataViewCtrl * wxDataViewColumn_GetOwner(const wxDataViewColumn * self) {
    return self->GetOwner();
}
wxDataViewRenderer * wxDataViewColumn_GetRenderer(const wxDataViewColumn * self) {
    return self->GetRenderer();
}

// CLASS: wxDataViewCtrl
wxClassInfo *wxDataViewCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDataViewCtrl);
}
wxDataViewCtrl *wxDataViewCtrl_new() {
    return new wxDataViewCtrl();
}
wxDataViewCtrl *wxDataViewCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxDataViewCtrl(parent, id, *pos, *size, style, *validator, *name);
}
bool wxDataViewCtrl_AllowMultiColumnSort(wxDataViewCtrl * self, bool allow) {
    return self->AllowMultiColumnSort(allow);
}
bool wxDataViewCtrl_Create(wxDataViewCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *pos, *size, style, *validator, *name);
}
bool wxDataViewCtrl_AppendColumn(wxDataViewCtrl * self, wxDataViewColumn * col) {
    return self->AppendColumn(col);
}
bool wxDataViewCtrl_PrependColumn(wxDataViewCtrl * self, wxDataViewColumn * col) {
    return self->PrependColumn(col);
}
bool wxDataViewCtrl_InsertColumn(wxDataViewCtrl * self, unsigned int pos, wxDataViewColumn * col) {
    return self->InsertColumn(pos, col);
}
bool wxDataViewCtrl_AssociateModel(wxDataViewCtrl * self, wxDataViewModel * model) {
    return self->AssociateModel(model);
}
bool wxDataViewCtrl_ClearColumns(wxDataViewCtrl * self) {
    return self->ClearColumns();
}
void wxDataViewCtrl_Collapse(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->Collapse(*item);
}
bool wxDataViewCtrl_DeleteColumn(wxDataViewCtrl * self, wxDataViewColumn * column) {
    return self->DeleteColumn(column);
}
void wxDataViewCtrl_EditItem(wxDataViewCtrl * self, const wxDataViewItem * item, const wxDataViewColumn * column) {
    return self->EditItem(*item, column);
}
bool wxDataViewCtrl_EnableDragSource(wxDataViewCtrl * self, const wxDataFormat * format) {
    return self->EnableDragSource(*format);
}
bool wxDataViewCtrl_EnableDropTargets(wxDataViewCtrl * self, const wxVector< wxDataFormat > * formats) {
    return self->EnableDropTargets(*formats);
}
bool wxDataViewCtrl_EnableDropTarget(wxDataViewCtrl * self, const wxDataFormat * format) {
    return self->EnableDropTarget(*format);
}
void wxDataViewCtrl_EnsureVisible(wxDataViewCtrl * self, const wxDataViewItem * item, const wxDataViewColumn * column) {
    return self->EnsureVisible(*item, column);
}
void wxDataViewCtrl_Expand(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->Expand(*item);
}
void wxDataViewCtrl_ExpandAncestors(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->ExpandAncestors(*item);
}
void wxDataViewCtrl_ExpandChildren(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->ExpandChildren(*item);
}
wxDataViewColumn * wxDataViewCtrl_GetColumn(const wxDataViewCtrl * self, unsigned int pos) {
    return self->GetColumn(pos);
}
unsigned int wxDataViewCtrl_GetColumnCount(const wxDataViewCtrl * self) {
    return self->GetColumnCount();
}
int wxDataViewCtrl_GetColumnPosition(const wxDataViewCtrl * self, const wxDataViewColumn * column) {
    return self->GetColumnPosition(column);
}
wxDataViewColumn * wxDataViewCtrl_GetExpanderColumn(const wxDataViewCtrl * self) {
    return self->GetExpanderColumn();
}
wxDataViewItem *wxDataViewCtrl_GetCurrentItem(const wxDataViewCtrl * self) {
    return new wxDataViewItem(self->GetCurrentItem());
}
wxDataViewColumn * wxDataViewCtrl_GetCurrentColumn(const wxDataViewCtrl * self) {
    return self->GetCurrentColumn();
}
int wxDataViewCtrl_GetIndent(const wxDataViewCtrl * self) {
    return self->GetIndent();
}
wxRect *wxDataViewCtrl_GetItemRect(const wxDataViewCtrl * self, const wxDataViewItem * item, const wxDataViewColumn * col) {
    return new wxRect(self->GetItemRect(*item, col));
}
wxWindow * wxDataViewCtrl_GetMainWindow(wxDataViewCtrl * self) {
    return self->GetMainWindow();
}
wxDataViewModel * wxDataViewCtrl_GetModel(wxDataViewCtrl * self) {
    return self->GetModel();
}
int wxDataViewCtrl_GetSelectedItemsCount(const wxDataViewCtrl * self) {
    return self->GetSelectedItemsCount();
}
wxDataViewItem *wxDataViewCtrl_GetSelection(const wxDataViewCtrl * self) {
    return new wxDataViewItem(self->GetSelection());
}
int wxDataViewCtrl_GetSelections(const wxDataViewCtrl * self, wxDataViewItemArray * sel) {
    return self->GetSelections(*sel);
}
wxDataViewColumn * wxDataViewCtrl_GetSortingColumn(const wxDataViewCtrl * self) {
    return self->GetSortingColumn();
}
bool wxDataViewCtrl_HasSelection(const wxDataViewCtrl * self) {
    return self->HasSelection();
}
void wxDataViewCtrl_HitTest(const wxDataViewCtrl * self, const wxPoint * point, wxDataViewItem * item, wxDataViewColumn *& col) {
    return self->HitTest(*point, *item, col);
}
bool wxDataViewCtrl_IsExpanded(const wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->IsExpanded(*item);
}
bool wxDataViewCtrl_IsMultiColumnSortAllowed(const wxDataViewCtrl * self) {
    return self->IsMultiColumnSortAllowed();
}
bool wxDataViewCtrl_IsSelected(const wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->IsSelected(*item);
}
void wxDataViewCtrl_Select(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->Select(*item);
}
void wxDataViewCtrl_SelectAll(wxDataViewCtrl * self) {
    return self->SelectAll();
}
bool wxDataViewCtrl_SetAlternateRowColour(wxDataViewCtrl * self, const wxColour * colour) {
    return self->SetAlternateRowColour(*colour);
}
void wxDataViewCtrl_SetExpanderColumn(wxDataViewCtrl * self, wxDataViewColumn * col) {
    return self->SetExpanderColumn(col);
}
void wxDataViewCtrl_SetCurrentItem(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->SetCurrentItem(*item);
}
bool wxDataViewCtrl_SetHeaderAttr(wxDataViewCtrl * self, const wxItemAttr * attr) {
    return self->SetHeaderAttr(*attr);
}
void wxDataViewCtrl_SetIndent(wxDataViewCtrl * self, int indent) {
    return self->SetIndent(indent);
}
void wxDataViewCtrl_SetSelections(wxDataViewCtrl * self, const wxDataViewItemArray * sel) {
    return self->SetSelections(*sel);
}
void wxDataViewCtrl_Unselect(wxDataViewCtrl * self, const wxDataViewItem * item) {
    return self->Unselect(*item);
}
void wxDataViewCtrl_UnselectAll(wxDataViewCtrl * self) {
    return self->UnselectAll();
}
bool wxDataViewCtrl_SetRowHeight(wxDataViewCtrl * self, int row_height) {
    return self->SetRowHeight(row_height);
}
void wxDataViewCtrl_ToggleSortByColumn(wxDataViewCtrl * self, int column) {
    return self->ToggleSortByColumn(column);
}
int wxDataViewCtrl_GetCountPerPage(const wxDataViewCtrl * self) {
    return self->GetCountPerPage();
}
wxDataViewItem *wxDataViewCtrl_GetTopItem(const wxDataViewCtrl * self) {
    return new wxDataViewItem(self->GetTopItem());
}

// CLASS: wxDataViewCustomRenderer
wxClassInfo *wxDataViewCustomRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewCustomRenderer);
}
wxString *wxDataViewCustomRenderer_GetDefaultType() {
    return new wxString(wxDataViewCustomRenderer::GetDefaultType());
}
bool wxDataViewCustomRenderer_ActivateCell(wxDataViewCustomRenderer * self, const wxRect * cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col, const wxMouseEvent * mouse_event) {
    return self->ActivateCell(*cell, model, *item, col, mouse_event);
}
wxDataViewItemAttr *wxDataViewCustomRenderer_GetAttr(const wxDataViewCustomRenderer * self) {
    return new wxDataViewItemAttr(self->GetAttr());
}
wxSize *wxDataViewCustomRenderer_GetSize(const wxDataViewCustomRenderer * self) {
    return new wxSize(self->GetSize());
}
bool wxDataViewCustomRenderer_LeftClick(wxDataViewCustomRenderer * self, wxPoint cursor, wxRect cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col) {
    return self->LeftClick(cursor, cell, model, *item, col);
}
bool wxDataViewCustomRenderer_Activate(wxDataViewCustomRenderer * self, wxRect cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col) {
    return self->Activate(cell, model, *item, col);
}
bool wxDataViewCustomRenderer_Render(wxDataViewCustomRenderer * self, wxRect cell, wxDC * dc, int state) {
    return self->Render(cell, dc, state);
}
void wxDataViewCustomRenderer_RenderText(wxDataViewCustomRenderer * self, const wxString * text, int xoffset, wxRect cell, wxDC * dc, int state) {
    return self->RenderText(*text, xoffset, cell, dc, state);
}
bool wxDataViewCustomRenderer_StartDrag(wxDataViewCustomRenderer * self, const wxPoint * cursor, const wxRect * cell, wxDataViewModel * model, const wxDataViewItem * item, unsigned int col) {
    return self->StartDrag(*cursor, *cell, model, *item, col);
}

// CLASS: wxDataViewDateRenderer
wxClassInfo *wxDataViewDateRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewDateRenderer);
}
wxString *wxDataViewDateRenderer_GetDefaultType() {
    return new wxString(wxDataViewDateRenderer::GetDefaultType());
}

// CLASS: wxDataViewEvent
wxClassInfo *wxDataViewEvent_CLASSINFO() {
    return wxCLASSINFO(wxDataViewEvent);
}
wxDataViewEvent *wxDataViewEvent_new() {
    return new wxDataViewEvent();
}
wxDataViewEvent *wxDataViewEvent_new3(const wxDataViewEvent * event) {
    return new wxDataViewEvent(*event);
}
int wxDataViewEvent_GetColumn(const wxDataViewEvent * self) {
    return self->GetColumn();
}
wxDataViewColumn * wxDataViewEvent_GetDataViewColumn(const wxDataViewEvent * self) {
    return self->GetDataViewColumn();
}
wxDataViewModel * wxDataViewEvent_GetModel(const wxDataViewEvent * self) {
    return self->GetModel();
}
wxPoint *wxDataViewEvent_GetPosition(const wxDataViewEvent * self) {
    return new wxPoint(self->GetPosition());
}
bool wxDataViewEvent_IsEditCancelled(const wxDataViewEvent * self) {
    return self->IsEditCancelled();
}
void wxDataViewEvent_SetColumn(wxDataViewEvent * self, int col) {
    return self->SetColumn(col);
}
void wxDataViewEvent_SetDataViewColumn(wxDataViewEvent * self, wxDataViewColumn * col) {
    return self->SetDataViewColumn(col);
}
void wxDataViewEvent_SetModel(wxDataViewEvent * self, wxDataViewModel * model) {
    return self->SetModel(model);
}
void wxDataViewEvent_SetValue(wxDataViewEvent * self, const wxVariant * value) {
    return self->SetValue(*value);
}
void wxDataViewEvent_SetDataObject(wxDataViewEvent * self, wxDataObject * obj) {
    return self->SetDataObject(obj);
}
wxDataFormat *wxDataViewEvent_GetDataFormat(const wxDataViewEvent * self) {
    return new wxDataFormat(self->GetDataFormat());
}
size_t wxDataViewEvent_GetDataSize(const wxDataViewEvent * self) {
    return self->GetDataSize();
}
void * wxDataViewEvent_GetDataBuffer(const wxDataViewEvent * self) {
    return self->GetDataBuffer();
}
void wxDataViewEvent_SetDragFlags(wxDataViewEvent * self, int flags) {
    return self->SetDragFlags(flags);
}
int wxDataViewEvent_GetCacheFrom(const wxDataViewEvent * self) {
    return self->GetCacheFrom();
}
int wxDataViewEvent_GetCacheTo(const wxDataViewEvent * self) {
    return self->GetCacheTo();
}
int wxDataViewEvent_GetProposedDropIndex(const wxDataViewEvent * self) {
    return self->GetProposedDropIndex();
}
wxDataViewItem *wxDataViewEvent_GetItem(const wxDataViewEvent * self) {
    return new wxDataViewItem(self->GetItem());
}
void wxDataViewEvent_SetItem(wxDataViewEvent * self, const wxDataViewItem * item) {
    return self->SetItem(*item);
}
void wxDataViewEvent_SetPosition(wxDataViewEvent * self, int x, int y) {
    return self->SetPosition(x, y);
}
void wxDataViewEvent_SetCache(wxDataViewEvent * self, int from, int to) {
    return self->SetCache(from, to);
}
wxDataObject * wxDataViewEvent_GetDataObject(const wxDataViewEvent * self) {
    return self->GetDataObject();
}
void wxDataViewEvent_SetDataFormat(wxDataViewEvent * self, const wxDataFormat * format) {
    return self->SetDataFormat(*format);
}
void wxDataViewEvent_SetDataSize(wxDataViewEvent * self, size_t size) {
    return self->SetDataSize(size);
}
void wxDataViewEvent_SetDataBuffer(wxDataViewEvent * self, void * buf) {
    return self->SetDataBuffer(buf);
}
int wxDataViewEvent_GetDragFlags(const wxDataViewEvent * self) {
    return self->GetDragFlags();
}

// CLASS: wxDataViewIconText
wxClassInfo *wxDataViewIconText_CLASSINFO() {
    return wxCLASSINFO(wxDataViewIconText);
}
wxDataViewIconText *wxDataViewIconText_new(const wxString * text, const wxBitmapBundle * bitmap) {
    return new wxDataViewIconText(*text, *bitmap);
}
wxDataViewIconText *wxDataViewIconText_new1(const wxDataViewIconText * other) {
    return new wxDataViewIconText(*other);
}
wxBitmapBundle *wxDataViewIconText_GetBitmapBundle(const wxDataViewIconText * self) {
    return new wxBitmapBundle(self->GetBitmapBundle());
}
wxIcon *wxDataViewIconText_GetIcon(const wxDataViewIconText * self) {
    return new wxIcon(self->GetIcon());
}
wxString *wxDataViewIconText_GetText(const wxDataViewIconText * self) {
    return new wxString(self->GetText());
}
void wxDataViewIconText_SetBitmapBundle(wxDataViewIconText * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmapBundle(*bitmap);
}
void wxDataViewIconText_SetIcon(wxDataViewIconText * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxDataViewIconText_SetText(wxDataViewIconText * self, const wxString * text) {
    return self->SetText(*text);
}

// CLASS: wxDataViewIconTextRenderer
wxClassInfo *wxDataViewIconTextRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewIconTextRenderer);
}
wxString *wxDataViewIconTextRenderer_GetDefaultType() {
    return new wxString(wxDataViewIconTextRenderer::GetDefaultType());
}

// CLASS: wxDataViewIndexListModel
wxDataViewItem *wxDataViewIndexListModel_GetItem(const wxDataViewIndexListModel * self, unsigned int row) {
    return new wxDataViewItem(self->GetItem(row));
}
void wxDataViewIndexListModel_Reset(wxDataViewIndexListModel * self, unsigned int new_size) {
    return self->Reset(new_size);
}
void wxDataViewIndexListModel_RowAppended(wxDataViewIndexListModel * self) {
    return self->RowAppended();
}
void wxDataViewIndexListModel_RowChanged(wxDataViewIndexListModel * self, unsigned int row) {
    return self->RowChanged(row);
}
void wxDataViewIndexListModel_RowDeleted(wxDataViewIndexListModel * self, unsigned int row) {
    return self->RowDeleted(row);
}
void wxDataViewIndexListModel_RowInserted(wxDataViewIndexListModel * self, unsigned int before) {
    return self->RowInserted(before);
}
void wxDataViewIndexListModel_RowPrepended(wxDataViewIndexListModel * self) {
    return self->RowPrepended();
}
void wxDataViewIndexListModel_RowValueChanged(wxDataViewIndexListModel * self, unsigned int row, unsigned int col) {
    return self->RowValueChanged(row, col);
}
void wxDataViewIndexListModel_RowsDeleted(wxDataViewIndexListModel * self, const wxArrayInt * rows) {
    return self->RowsDeleted(*rows);
}

// CLASS: wxDataViewItem
void wxDataViewItem_delete(wxDataViewItem *self) {
    delete self;
}
wxDataViewItem *wxDataViewItem_new() {
    return new wxDataViewItem();
}
wxDataViewItem *wxDataViewItem_new1(const wxDataViewItem * item) {
    return new wxDataViewItem(*item);
}
wxDataViewItem *wxDataViewItem_new2(void * id) {
    return new wxDataViewItem(id);
}
void * wxDataViewItem_GetID(const wxDataViewItem * self) {
    return self->GetID();
}
bool wxDataViewItem_IsOk(const wxDataViewItem * self) {
    return self->IsOk();
}

// CLASS: wxDataViewItemAttr
void wxDataViewItemAttr_delete(wxDataViewItemAttr *self) {
    delete self;
}
wxDataViewItemAttr *wxDataViewItemAttr_new() {
    return new wxDataViewItemAttr();
}
void wxDataViewItemAttr_SetBold(wxDataViewItemAttr * self, bool set) {
    return self->SetBold(set);
}
void wxDataViewItemAttr_SetColour(wxDataViewItemAttr * self, const wxColour * colour) {
    return self->SetColour(*colour);
}
void wxDataViewItemAttr_SetBackgroundColour(wxDataViewItemAttr * self, const wxColour * colour) {
    return self->SetBackgroundColour(*colour);
}
void wxDataViewItemAttr_SetItalic(wxDataViewItemAttr * self, bool set) {
    return self->SetItalic(set);
}
void wxDataViewItemAttr_SetStrikethrough(wxDataViewItemAttr * self, bool set) {
    return self->SetStrikethrough(set);
}
bool wxDataViewItemAttr_HasColour(const wxDataViewItemAttr * self) {
    return self->HasColour();
}
wxColour *wxDataViewItemAttr_GetColour(const wxDataViewItemAttr * self) {
    return new wxColour(self->GetColour());
}
bool wxDataViewItemAttr_HasFont(const wxDataViewItemAttr * self) {
    return self->HasFont();
}
bool wxDataViewItemAttr_GetBold(const wxDataViewItemAttr * self) {
    return self->GetBold();
}
bool wxDataViewItemAttr_GetItalic(const wxDataViewItemAttr * self) {
    return self->GetItalic();
}
bool wxDataViewItemAttr_HasBackgroundColour(const wxDataViewItemAttr * self) {
    return self->HasBackgroundColour();
}
wxColour *wxDataViewItemAttr_GetBackgroundColour(const wxDataViewItemAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
bool wxDataViewItemAttr_IsDefault(const wxDataViewItemAttr * self) {
    return self->IsDefault();
}
wxFont *wxDataViewItemAttr_GetEffectiveFont(const wxDataViewItemAttr * self, const wxFont * font) {
    return new wxFont(self->GetEffectiveFont(*font));
}

// CLASS: wxDataViewListCtrl
wxClassInfo *wxDataViewListCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDataViewListCtrl);
}
int wxDataViewListCtrl_GetSelectedRow(const wxDataViewListCtrl * self) {
    return self->GetSelectedRow();
}
void wxDataViewListCtrl_DeleteAllItems(wxDataViewListCtrl * self) {
    return self->DeleteAllItems();
}
unsigned int wxDataViewListCtrl_GetItemCount(const wxDataViewListCtrl * self) {
    return self->GetItemCount();
}
void wxDataViewListCtrl_SetValue(wxDataViewListCtrl * self, const wxVariant * value, unsigned int row, unsigned int col) {
    return self->SetValue(*value, row, col);
}
void wxDataViewListCtrl_GetValue(wxDataViewListCtrl * self, wxVariant * value, unsigned int row, unsigned int col) {
    return self->GetValue(*value, row, col);
}
void wxDataViewListCtrl_SetTextValue(wxDataViewListCtrl * self, const wxString * value, unsigned int row, unsigned int col) {
    return self->SetTextValue(*value, row, col);
}
wxString *wxDataViewListCtrl_GetTextValue(const wxDataViewListCtrl * self, unsigned int row, unsigned int col) {
    return new wxString(self->GetTextValue(row, col));
}
void wxDataViewListCtrl_SetToggleValue(wxDataViewListCtrl * self, bool value, unsigned int row, unsigned int col) {
    return self->SetToggleValue(value, row, col);
}
bool wxDataViewListCtrl_GetToggleValue(const wxDataViewListCtrl * self, unsigned int row, unsigned int col) {
    return self->GetToggleValue(row, col);
}
wxDataViewListCtrl *wxDataViewListCtrl_new() {
    return new wxDataViewListCtrl();
}
wxDataViewListCtrl *wxDataViewListCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator) {
    return new wxDataViewListCtrl(parent, id, *pos, *size, style, *validator);
}
bool wxDataViewListCtrl_Create(wxDataViewListCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator) {
    return self->Create(parent, id, *pos, *size, style, *validator);
}
wxDataViewListStore * wxDataViewListCtrl_GetStore(wxDataViewListCtrl * self) {
    return self->GetStore();
}
const wxDataViewListStore * wxDataViewListCtrl_GetStore1(const wxDataViewListCtrl * self) {
    return self->GetStore();
}
int wxDataViewListCtrl_ItemToRow(const wxDataViewListCtrl * self, const wxDataViewItem * item) {
    return self->ItemToRow(*item);
}
wxDataViewItem *wxDataViewListCtrl_RowToItem(const wxDataViewListCtrl * self, int row) {
    return new wxDataViewItem(self->RowToItem(row));
}

// CLASS: wxDataViewListModel
bool wxDataViewListModel_GetAttrByRow(const wxDataViewListModel * self, unsigned int row, unsigned int col, wxDataViewItemAttr * attr) {
    return self->GetAttrByRow(row, col, *attr);
}
bool wxDataViewListModel_IsEnabledByRow(const wxDataViewListModel * self, unsigned int row, unsigned int col) {
    return self->IsEnabledByRow(row, col);
}
unsigned int wxDataViewListModel_GetCount(const wxDataViewListModel * self) {
    return self->GetCount();
}
unsigned int wxDataViewListModel_GetRow(const wxDataViewListModel * self, const wxDataViewItem * item) {
    return self->GetRow(*item);
}
void wxDataViewListModel_GetValueByRow(const wxDataViewListModel * self, wxVariant * variant, unsigned int row, unsigned int col) {
    return self->GetValueByRow(*variant, row, col);
}
bool wxDataViewListModel_SetValueByRow(wxDataViewListModel * self, const wxVariant * variant, unsigned int row, unsigned int col) {
    return self->SetValueByRow(*variant, row, col);
}

// CLASS: wxDataViewListStore
wxDataViewListStore *wxDataViewListStore_new() {
    return new wxDataViewListStore();
}
void wxDataViewListStore_PrependColumn(wxDataViewListStore * self, const wxString * varianttype) {
    return self->PrependColumn(*varianttype);
}
void wxDataViewListStore_InsertColumn(wxDataViewListStore * self, unsigned int pos, const wxString * varianttype) {
    return self->InsertColumn(pos, *varianttype);
}
void wxDataViewListStore_AppendColumn(wxDataViewListStore * self, const wxString * varianttype) {
    return self->AppendColumn(*varianttype);
}
void wxDataViewListStore_DeleteAllItems(wxDataViewListStore * self) {
    return self->DeleteAllItems();
}
unsigned int wxDataViewListStore_GetItemCount(const wxDataViewListStore * self) {
    return self->GetItemCount();
}

// CLASS: wxDataViewModel
void wxDataViewModel_AddNotifier(wxDataViewModel * self, wxDataViewModelNotifier * notifier) {
    return self->AddNotifier(notifier);
}
bool wxDataViewModel_ChangeValue(wxDataViewModel * self, const wxVariant * variant, const wxDataViewItem * item, unsigned int col) {
    return self->ChangeValue(*variant, *item, col);
}
bool wxDataViewModel_Cleared(wxDataViewModel * self) {
    return self->Cleared();
}
int wxDataViewModel_Compare(const wxDataViewModel * self, const wxDataViewItem * item1, const wxDataViewItem * item2, unsigned int column, bool ascending) {
    return self->Compare(*item1, *item2, column, ascending);
}
bool wxDataViewModel_GetAttr(const wxDataViewModel * self, const wxDataViewItem * item, unsigned int col, wxDataViewItemAttr * attr) {
    return self->GetAttr(*item, col, *attr);
}
bool wxDataViewModel_IsEnabled(const wxDataViewModel * self, const wxDataViewItem * item, unsigned int col) {
    return self->IsEnabled(*item, col);
}
unsigned int wxDataViewModel_GetChildren(const wxDataViewModel * self, const wxDataViewItem * item, wxDataViewItemArray * children) {
    return self->GetChildren(*item, *children);
}
wxDataViewItem *wxDataViewModel_GetParent(const wxDataViewModel * self, const wxDataViewItem * item) {
    return new wxDataViewItem(self->GetParent(*item));
}
void wxDataViewModel_GetValue(const wxDataViewModel * self, wxVariant * variant, const wxDataViewItem * item, unsigned int col) {
    return self->GetValue(*variant, *item, col);
}
bool wxDataViewModel_HasContainerColumns(const wxDataViewModel * self, const wxDataViewItem * item) {
    return self->HasContainerColumns(*item);
}
bool wxDataViewModel_HasDefaultCompare(const wxDataViewModel * self) {
    return self->HasDefaultCompare();
}
bool wxDataViewModel_IsContainer(const wxDataViewModel * self, const wxDataViewItem * item) {
    return self->IsContainer(*item);
}
bool wxDataViewModel_ItemAdded(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItem * item) {
    return self->ItemAdded(*parent, *item);
}
bool wxDataViewModel_ItemChanged(wxDataViewModel * self, const wxDataViewItem * item) {
    return self->ItemChanged(*item);
}
bool wxDataViewModel_ItemDeleted(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItem * item) {
    return self->ItemDeleted(*parent, *item);
}
bool wxDataViewModel_ItemsAdded(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItemArray * items) {
    return self->ItemsAdded(*parent, *items);
}
bool wxDataViewModel_ItemsChanged(wxDataViewModel * self, const wxDataViewItemArray * items) {
    return self->ItemsChanged(*items);
}
bool wxDataViewModel_ItemsDeleted(wxDataViewModel * self, const wxDataViewItem * parent, const wxDataViewItemArray * items) {
    return self->ItemsDeleted(*parent, *items);
}
void wxDataViewModel_RemoveNotifier(wxDataViewModel * self, wxDataViewModelNotifier * notifier) {
    return self->RemoveNotifier(notifier);
}
void wxDataViewModel_Resort(wxDataViewModel * self) {
    return self->Resort();
}
bool wxDataViewModel_SetValue(wxDataViewModel * self, const wxVariant * variant, const wxDataViewItem * item, unsigned int col) {
    return self->SetValue(*variant, *item, col);
}
bool wxDataViewModel_ValueChanged(wxDataViewModel * self, const wxDataViewItem * item, unsigned int col) {
    return self->ValueChanged(*item, col);
}
bool wxDataViewModel_IsListModel(const wxDataViewModel * self) {
    return self->IsListModel();
}
bool wxDataViewModel_IsVirtualListModel(const wxDataViewModel * self) {
    return self->IsVirtualListModel();
}

// CLASS: wxDataViewModelNotifier
void wxDataViewModelNotifier_delete(wxDataViewModelNotifier *self) {
    delete self;
}
bool wxDataViewModelNotifier_Cleared(wxDataViewModelNotifier * self) {
    return self->Cleared();
}
wxDataViewModel * wxDataViewModelNotifier_GetOwner(const wxDataViewModelNotifier * self) {
    return self->GetOwner();
}
bool wxDataViewModelNotifier_ItemAdded(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItem * item) {
    return self->ItemAdded(*parent, *item);
}
bool wxDataViewModelNotifier_ItemChanged(wxDataViewModelNotifier * self, const wxDataViewItem * item) {
    return self->ItemChanged(*item);
}
bool wxDataViewModelNotifier_ItemDeleted(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItem * item) {
    return self->ItemDeleted(*parent, *item);
}
bool wxDataViewModelNotifier_ItemsAdded(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItemArray * items) {
    return self->ItemsAdded(*parent, *items);
}
bool wxDataViewModelNotifier_ItemsChanged(wxDataViewModelNotifier * self, const wxDataViewItemArray * items) {
    return self->ItemsChanged(*items);
}
bool wxDataViewModelNotifier_ItemsDeleted(wxDataViewModelNotifier * self, const wxDataViewItem * parent, const wxDataViewItemArray * items) {
    return self->ItemsDeleted(*parent, *items);
}
void wxDataViewModelNotifier_Resort(wxDataViewModelNotifier * self) {
    return self->Resort();
}
void wxDataViewModelNotifier_SetOwner(wxDataViewModelNotifier * self, wxDataViewModel * owner) {
    return self->SetOwner(owner);
}
bool wxDataViewModelNotifier_ValueChanged(wxDataViewModelNotifier * self, const wxDataViewItem * item, unsigned int col) {
    return self->ValueChanged(*item, col);
}

// CLASS: wxDataViewProgressRenderer
wxClassInfo *wxDataViewProgressRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewProgressRenderer);
}
wxString *wxDataViewProgressRenderer_GetDefaultType() {
    return new wxString(wxDataViewProgressRenderer::GetDefaultType());
}

// CLASS: wxDataViewRenderer
wxClassInfo *wxDataViewRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewRenderer);
}
void wxDataViewRenderer_EnableEllipsize(wxDataViewRenderer * self, wxEllipsizeMode mode) {
    return self->EnableEllipsize(mode);
}
void wxDataViewRenderer_DisableEllipsize(wxDataViewRenderer * self) {
    return self->DisableEllipsize();
}
int wxDataViewRenderer_GetAlignment(const wxDataViewRenderer * self) {
    return self->GetAlignment();
}
wxEllipsizeMode wxDataViewRenderer_GetEllipsizeMode(const wxDataViewRenderer * self) {
    return self->GetEllipsizeMode();
}
wxDataViewColumn * wxDataViewRenderer_GetOwner(const wxDataViewRenderer * self) {
    return self->GetOwner();
}
bool wxDataViewRenderer_GetValue(const wxDataViewRenderer * self, wxVariant * value) {
    return self->GetValue(*value);
}
wxString *wxDataViewRenderer_GetVariantType(const wxDataViewRenderer * self) {
    return new wxString(self->GetVariantType());
}
bool wxDataViewRenderer_IsCompatibleVariantType(const wxDataViewRenderer * self, const wxString * variant_type) {
    return self->IsCompatibleVariantType(*variant_type);
}
void wxDataViewRenderer_SetAlignment(wxDataViewRenderer * self, int align) {
    return self->SetAlignment(align);
}
void wxDataViewRenderer_SetOwner(wxDataViewRenderer * self, wxDataViewColumn * owner) {
    return self->SetOwner(owner);
}
bool wxDataViewRenderer_SetValue(wxDataViewRenderer * self, const wxVariant * value) {
    return self->SetValue(*value);
}
void wxDataViewRenderer_SetValueAdjuster(wxDataViewRenderer * self, wxDataViewValueAdjuster * transformer) {
    return self->SetValueAdjuster(transformer);
}
bool wxDataViewRenderer_Validate(wxDataViewRenderer * self, wxVariant * value) {
    return self->Validate(*value);
}
bool wxDataViewRenderer_HasEditorCtrl(const wxDataViewRenderer * self) {
    return self->HasEditorCtrl();
}
wxWindow * wxDataViewRenderer_CreateEditorCtrl(wxDataViewRenderer * self, wxWindow * parent, wxRect label_rect, const wxVariant * value) {
    return self->CreateEditorCtrl(parent, label_rect, *value);
}
bool wxDataViewRenderer_GetValueFromEditorCtrl(wxDataViewRenderer * self, wxWindow * editor, wxVariant * value) {
    return self->GetValueFromEditorCtrl(editor, *value);
}
bool wxDataViewRenderer_StartEditing(wxDataViewRenderer * self, const wxDataViewItem * item, wxRect label_rect) {
    return self->StartEditing(*item, label_rect);
}
void wxDataViewRenderer_CancelEditing(wxDataViewRenderer * self) {
    return self->CancelEditing();
}
bool wxDataViewRenderer_FinishEditing(wxDataViewRenderer * self) {
    return self->FinishEditing();
}
wxWindow * wxDataViewRenderer_GetEditorCtrl(wxDataViewRenderer * self) {
    return self->GetEditorCtrl();
}

// CLASS: wxDataViewSpinRenderer
wxClassInfo *wxDataViewSpinRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewSpinRenderer);
}

// CLASS: wxDataViewTextRenderer
wxClassInfo *wxDataViewTextRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewTextRenderer);
}
wxString *wxDataViewTextRenderer_GetDefaultType() {
    return new wxString(wxDataViewTextRenderer::GetDefaultType());
}
void wxDataViewTextRenderer_EnableMarkup(wxDataViewTextRenderer * self, bool enable) {
    return self->EnableMarkup(enable);
}

// CLASS: wxDataViewToggleRenderer
wxClassInfo *wxDataViewToggleRenderer_CLASSINFO() {
    return wxCLASSINFO(wxDataViewToggleRenderer);
}
wxString *wxDataViewToggleRenderer_GetDefaultType() {
    return new wxString(wxDataViewToggleRenderer::GetDefaultType());
}
void wxDataViewToggleRenderer_ShowAsRadio(wxDataViewToggleRenderer * self) {
    return self->ShowAsRadio();
}

// CLASS: wxDataViewTreeCtrl
wxClassInfo *wxDataViewTreeCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDataViewTreeCtrl);
}
wxDataViewTreeCtrl *wxDataViewTreeCtrl_new() {
    return new wxDataViewTreeCtrl();
}
wxDataViewTreeCtrl *wxDataViewTreeCtrl_new1(wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator) {
    return new wxDataViewTreeCtrl(parent, id, *pos, *size, style, *validator);
}
wxDataViewItem *wxDataViewTreeCtrl_AppendContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, int expanded, wxClientData * data) {
    return new wxDataViewItem(self->AppendContainer(*parent, *text, icon, expanded, data));
}
wxDataViewItem *wxDataViewTreeCtrl_AppendItem(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, wxClientData * data) {
    return new wxDataViewItem(self->AppendItem(*parent, *text, icon, data));
}
bool wxDataViewTreeCtrl_Create(wxDataViewTreeCtrl * self, wxWindow * parent, wxWindowID id, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator) {
    return self->Create(parent, id, *pos, *size, style, *validator);
}
void wxDataViewTreeCtrl_DeleteAllItems(wxDataViewTreeCtrl * self) {
    return self->DeleteAllItems();
}
void wxDataViewTreeCtrl_DeleteChildren(wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return self->DeleteChildren(*item);
}
void wxDataViewTreeCtrl_DeleteItem(wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return self->DeleteItem(*item);
}
int wxDataViewTreeCtrl_GetChildCount(const wxDataViewTreeCtrl * self, const wxDataViewItem * parent) {
    return self->GetChildCount(*parent);
}
wxImageList * wxDataViewTreeCtrl_GetImageList(wxDataViewTreeCtrl * self) {
    return self->GetImageList();
}
wxClientData * wxDataViewTreeCtrl_GetItemData(const wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return self->GetItemData(*item);
}
wxIcon *wxDataViewTreeCtrl_GetItemExpandedIcon(const wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return new wxIcon(self->GetItemExpandedIcon(*item));
}
wxIcon *wxDataViewTreeCtrl_GetItemIcon(const wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return new wxIcon(self->GetItemIcon(*item));
}
wxDataViewItem *wxDataViewTreeCtrl_GetItemParent(const wxDataViewTreeCtrl * self, wxDataViewItem item) {
    return new wxDataViewItem(self->GetItemParent(item));
}
wxString *wxDataViewTreeCtrl_GetItemText(const wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return new wxString(self->GetItemText(*item));
}
wxDataViewItem *wxDataViewTreeCtrl_GetNthChild(const wxDataViewTreeCtrl * self, const wxDataViewItem * parent, unsigned int pos) {
    return new wxDataViewItem(self->GetNthChild(*parent, pos));
}
wxDataViewTreeStore * wxDataViewTreeCtrl_GetStore(wxDataViewTreeCtrl * self) {
    return self->GetStore();
}
const wxDataViewTreeStore * wxDataViewTreeCtrl_GetStore1(const wxDataViewTreeCtrl * self) {
    return self->GetStore();
}
wxDataViewItem *wxDataViewTreeCtrl_InsertContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, int icon, int expanded, wxClientData * data) {
    return new wxDataViewItem(self->InsertContainer(*parent, *previous, *text, icon, expanded, data));
}
wxDataViewItem *wxDataViewTreeCtrl_InsertItem(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, int icon, wxClientData * data) {
    return new wxDataViewItem(self->InsertItem(*parent, *previous, *text, icon, data));
}
bool wxDataViewTreeCtrl_IsContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * item) {
    return self->IsContainer(*item);
}
wxDataViewItem *wxDataViewTreeCtrl_PrependContainer(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, int expanded, wxClientData * data) {
    return new wxDataViewItem(self->PrependContainer(*parent, *text, icon, expanded, data));
}
wxDataViewItem *wxDataViewTreeCtrl_PrependItem(wxDataViewTreeCtrl * self, const wxDataViewItem * parent, const wxString * text, int icon, wxClientData * data) {
    return new wxDataViewItem(self->PrependItem(*parent, *text, icon, data));
}
void wxDataViewTreeCtrl_SetImageList(wxDataViewTreeCtrl * self, wxImageList * imagelist) {
    return self->SetImageList(imagelist);
}
void wxDataViewTreeCtrl_SetItemData(wxDataViewTreeCtrl * self, const wxDataViewItem * item, wxClientData * data) {
    return self->SetItemData(*item, data);
}
void wxDataViewTreeCtrl_SetItemExpandedIcon(wxDataViewTreeCtrl * self, const wxDataViewItem * item, const wxBitmapBundle * icon) {
    return self->SetItemExpandedIcon(*item, *icon);
}
void wxDataViewTreeCtrl_SetItemIcon(wxDataViewTreeCtrl * self, const wxDataViewItem * item, const wxBitmapBundle * icon) {
    return self->SetItemIcon(*item, *icon);
}
void wxDataViewTreeCtrl_SetItemText(wxDataViewTreeCtrl * self, const wxDataViewItem * item, const wxString * text) {
    return self->SetItemText(*item, *text);
}

// CLASS: wxDataViewTreeStore
wxDataViewTreeStore *wxDataViewTreeStore_new() {
    return new wxDataViewTreeStore();
}
wxDataViewItem *wxDataViewTreeStore_AppendContainer(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, const wxBitmapBundle * expanded, wxClientData * data) {
    return new wxDataViewItem(self->AppendContainer(*parent, *text, *icon, *expanded, data));
}
wxDataViewItem *wxDataViewTreeStore_AppendItem(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, wxClientData * data) {
    return new wxDataViewItem(self->AppendItem(*parent, *text, *icon, data));
}
void wxDataViewTreeStore_DeleteAllItems(wxDataViewTreeStore * self) {
    return self->DeleteAllItems();
}
void wxDataViewTreeStore_DeleteChildren(wxDataViewTreeStore * self, const wxDataViewItem * item) {
    return self->DeleteChildren(*item);
}
void wxDataViewTreeStore_DeleteItem(wxDataViewTreeStore * self, const wxDataViewItem * item) {
    return self->DeleteItem(*item);
}
int wxDataViewTreeStore_GetChildCount(const wxDataViewTreeStore * self, const wxDataViewItem * parent) {
    return self->GetChildCount(*parent);
}
wxClientData * wxDataViewTreeStore_GetItemData(const wxDataViewTreeStore * self, const wxDataViewItem * item) {
    return self->GetItemData(*item);
}
wxIcon *wxDataViewTreeStore_GetItemExpandedIcon(const wxDataViewTreeStore * self, const wxDataViewItem * item) {
    return new wxIcon(self->GetItemExpandedIcon(*item));
}
wxIcon *wxDataViewTreeStore_GetItemIcon(const wxDataViewTreeStore * self, const wxDataViewItem * item) {
    return new wxIcon(self->GetItemIcon(*item));
}
wxString *wxDataViewTreeStore_GetItemText(const wxDataViewTreeStore * self, const wxDataViewItem * item) {
    return new wxString(self->GetItemText(*item));
}
wxDataViewItem *wxDataViewTreeStore_GetNthChild(const wxDataViewTreeStore * self, const wxDataViewItem * parent, unsigned int pos) {
    return new wxDataViewItem(self->GetNthChild(*parent, pos));
}
wxDataViewItem *wxDataViewTreeStore_InsertContainer(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, const wxBitmapBundle * icon, const wxBitmapBundle * expanded, wxClientData * data) {
    return new wxDataViewItem(self->InsertContainer(*parent, *previous, *text, *icon, *expanded, data));
}
wxDataViewItem *wxDataViewTreeStore_InsertItem(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxDataViewItem * previous, const wxString * text, const wxBitmapBundle * icon, wxClientData * data) {
    return new wxDataViewItem(self->InsertItem(*parent, *previous, *text, *icon, data));
}
wxDataViewItem *wxDataViewTreeStore_PrependContainer(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, const wxBitmapBundle * expanded, wxClientData * data) {
    return new wxDataViewItem(self->PrependContainer(*parent, *text, *icon, *expanded, data));
}
wxDataViewItem *wxDataViewTreeStore_PrependItem(wxDataViewTreeStore * self, const wxDataViewItem * parent, const wxString * text, const wxBitmapBundle * icon, wxClientData * data) {
    return new wxDataViewItem(self->PrependItem(*parent, *text, *icon, data));
}
void wxDataViewTreeStore_SetItemData(wxDataViewTreeStore * self, const wxDataViewItem * item, wxClientData * data) {
    return self->SetItemData(*item, data);
}
void wxDataViewTreeStore_SetItemExpandedIcon(wxDataViewTreeStore * self, const wxDataViewItem * item, const wxBitmapBundle * icon) {
    return self->SetItemExpandedIcon(*item, *icon);
}
void wxDataViewTreeStore_SetItemIcon(wxDataViewTreeStore * self, const wxDataViewItem * item, const wxBitmapBundle * icon) {
    return self->SetItemIcon(*item, *icon);
}

// CLASS: wxDataViewValueAdjuster
void wxDataViewValueAdjuster_delete(wxDataViewValueAdjuster *self) {
    delete self;
}

// CLASS: wxDataViewVirtualListModel
wxDataViewItem *wxDataViewVirtualListModel_GetItem(const wxDataViewVirtualListModel * self, unsigned int row) {
    return new wxDataViewItem(self->GetItem(row));
}
void wxDataViewVirtualListModel_Reset(wxDataViewVirtualListModel * self, unsigned int new_size) {
    return self->Reset(new_size);
}
void wxDataViewVirtualListModel_RowAppended(wxDataViewVirtualListModel * self) {
    return self->RowAppended();
}
void wxDataViewVirtualListModel_RowChanged(wxDataViewVirtualListModel * self, unsigned int row) {
    return self->RowChanged(row);
}
void wxDataViewVirtualListModel_RowDeleted(wxDataViewVirtualListModel * self, unsigned int row) {
    return self->RowDeleted(row);
}
void wxDataViewVirtualListModel_RowInserted(wxDataViewVirtualListModel * self, unsigned int before) {
    return self->RowInserted(before);
}
void wxDataViewVirtualListModel_RowPrepended(wxDataViewVirtualListModel * self) {
    return self->RowPrepended();
}
void wxDataViewVirtualListModel_RowValueChanged(wxDataViewVirtualListModel * self, unsigned int row, unsigned int col) {
    return self->RowValueChanged(row, col);
}
void wxDataViewVirtualListModel_RowsDeleted(wxDataViewVirtualListModel * self, const wxArrayInt * rows) {
    return self->RowsDeleted(*rows);
}

// CLASS: wxDateEvent
wxClassInfo *wxDateEvent_CLASSINFO() {
    return wxCLASSINFO(wxDateEvent);
}
wxDateEvent *wxDateEvent_new() {
    return new wxDateEvent();
}
wxDateTime *wxDateEvent_GetDate(const wxDateEvent * self) {
    return new wxDateTime(self->GetDate());
}
void wxDateEvent_SetDate(wxDateEvent * self, const wxDateTime * date) {
    return self->SetDate(*date);
}

// CLASS: wxDatePickerCtrl
wxClassInfo *wxDatePickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDatePickerCtrl);
}
wxDatePickerCtrl *wxDatePickerCtrl_new() {
    return new wxDatePickerCtrl();
}
wxDatePickerCtrl *wxDatePickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxDatePickerCtrl(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxDatePickerCtrl_Create(wxDatePickerCtrl * self, wxWindow * parent, wxWindowID id, const wxDateTime * dt, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *dt, *pos, *size, style, *validator, *name);
}
bool wxDatePickerCtrl_GetRange(const wxDatePickerCtrl * self, wxDateTime * dt1, wxDateTime * dt2) {
    return self->GetRange(dt1, dt2);
}
wxDateTime *wxDatePickerCtrl_GetValue(const wxDatePickerCtrl * self) {
    return new wxDateTime(self->GetValue());
}
#if wxCHECK_VERSION(3, 1, 0)
void wxDatePickerCtrl_SetNullText(wxDatePickerCtrl * self, const wxString * text) {
    return self->SetNullText(*text);
}
#endif
void wxDatePickerCtrl_SetRange(wxDatePickerCtrl * self, const wxDateTime * dt1, const wxDateTime * dt2) {
    return self->SetRange(*dt1, *dt2);
}
void wxDatePickerCtrl_SetValue(wxDatePickerCtrl * self, const wxDateTime * dt) {
    return self->SetValue(*dt);
}

// CLASS: wxDelegateRendererNative
void wxDelegateRendererNative_delete(wxDelegateRendererNative *self) {
    delete self;
}
wxDelegateRendererNative *wxDelegateRendererNative_new() {
    return new wxDelegateRendererNative();
}
wxDelegateRendererNative *wxDelegateRendererNative_new1(wxRendererNative * renderer_native) {
    return new wxDelegateRendererNative(*renderer_native);
}

// CLASS: wxDialog
wxClassInfo *wxDialog_CLASSINFO() {
    return wxCLASSINFO(wxDialog);
}
wxDialog *wxDialog_new() {
    return new wxDialog();
}
wxDialog *wxDialog_new1(wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxDialog(parent, id, *title, *pos, *size, style, *name);
}
void wxDialog_AddMainButtonId(wxDialog * self, wxWindowID id) {
    return self->AddMainButtonId(id);
}
bool wxDialog_CanDoLayoutAdaptation(wxDialog * self) {
    return self->CanDoLayoutAdaptation();
}
void wxDialog_Centre(wxDialog * self, int direction) {
    return self->Centre(direction);
}
bool wxDialog_Create(wxDialog * self, wxWindow * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *title, *pos, *size, style, *name);
}
wxSizer * wxDialog_CreateButtonSizer(wxDialog * self, long flags) {
    return self->CreateButtonSizer(flags);
}
wxSizer * wxDialog_CreateSeparatedButtonSizer(wxDialog * self, long flags) {
    return self->CreateSeparatedButtonSizer(flags);
}
wxSizer * wxDialog_CreateSeparatedSizer(wxDialog * self, wxSizer * sizer) {
    return self->CreateSeparatedSizer(sizer);
}
wxStdDialogButtonSizer * wxDialog_CreateStdDialogButtonSizer(wxDialog * self, long flags) {
    return self->CreateStdDialogButtonSizer(flags);
}
#if wxCHECK_VERSION(3, 1, 0)
wxSizer * wxDialog_CreateTextSizer(wxDialog * self, const wxString * message, int width_max) {
    return self->CreateTextSizer(*message, width_max);
}
#endif
bool wxDialog_DoLayoutAdaptation(wxDialog * self) {
    return self->DoLayoutAdaptation();
}
void wxDialog_EndModal(wxDialog * self, int ret_code) {
    return self->EndModal(ret_code);
}
int wxDialog_GetAffirmativeId(const wxDialog * self) {
    return self->GetAffirmativeId();
}
wxWindow * wxDialog_GetContentWindow(const wxDialog * self) {
    return self->GetContentWindow();
}
int wxDialog_GetEscapeId(const wxDialog * self) {
    return self->GetEscapeId();
}
bool wxDialog_GetLayoutAdaptationDone(const wxDialog * self) {
    return self->GetLayoutAdaptationDone();
}
int wxDialog_GetLayoutAdaptationLevel(const wxDialog * self) {
    return self->GetLayoutAdaptationLevel();
}
wxArrayInt * wxDialog_GetMainButtonIds(wxDialog * self) {
    return &(self->GetMainButtonIds());
}
int wxDialog_GetReturnCode(const wxDialog * self) {
    return self->GetReturnCode();
}
bool wxDialog_IsMainButtonId(const wxDialog * self, wxWindowID id) {
    return self->IsMainButtonId(id);
}
bool wxDialog_IsModal(const wxDialog * self) {
    return self->IsModal();
}
void wxDialog_SetAffirmativeId(wxDialog * self, int id) {
    return self->SetAffirmativeId(id);
}
void wxDialog_SetEscapeId(wxDialog * self, int id) {
    return self->SetEscapeId(id);
}
void wxDialog_SetIcon(wxDialog * self, const wxIcon * icon) {
    return self->SetIcon(*icon);
}
void wxDialog_SetLayoutAdaptationDone(wxDialog * self, bool done) {
    return self->SetLayoutAdaptationDone(done);
}
void wxDialog_SetLayoutAdaptationLevel(wxDialog * self, int level) {
    return self->SetLayoutAdaptationLevel(level);
}
void wxDialog_SetReturnCode(wxDialog * self, int ret_code) {
    return self->SetReturnCode(ret_code);
}
int wxDialog_ShowModal(wxDialog * self) {
    return self->ShowModal();
}
void wxDialog_ShowWindowModal(wxDialog * self) {
    return self->ShowWindowModal();
}
void wxDialog_EnableLayoutAdaptation(bool enable) {
    return wxDialog::EnableLayoutAdaptation(enable);
}
wxDialogLayoutAdapter * wxDialog_GetLayoutAdapter() {
    return wxDialog::GetLayoutAdapter();
}
bool wxDialog_IsLayoutAdaptationEnabled() {
    return wxDialog::IsLayoutAdaptationEnabled();
}
wxDialogLayoutAdapter * wxDialog_SetLayoutAdapter(wxDialogLayoutAdapter * adapter) {
    return wxDialog::SetLayoutAdapter(adapter);
}

// CLASS: wxDialogLayoutAdapter
void wxDialogLayoutAdapter_delete(wxDialogLayoutAdapter *self) {
    delete self;
}
bool wxDialogLayoutAdapter_CanDoLayoutAdaptation(wxDialogLayoutAdapter * self, wxDialog * dialog) {
    return self->CanDoLayoutAdaptation(dialog);
}
bool wxDialogLayoutAdapter_DoLayoutAdaptation(wxDialogLayoutAdapter * self, wxDialog * dialog) {
    return self->DoLayoutAdaptation(dialog);
}

// CLASS: wxDirDialog
wxClassInfo *wxDirDialog_CLASSINFO() {
    return wxCLASSINFO(wxDirDialog);
}
wxDirDialog *wxDirDialog_new(wxWindow * parent, const wxString * message, const wxString * default_path, long style, const wxPoint * pos, const wxSize * size, const wxString * name) {
    return new wxDirDialog(parent, *message, *default_path, style, *pos, *size, *name);
}
wxString *wxDirDialog_GetMessage(const wxDirDialog * self) {
    return new wxString(self->GetMessage());
}
wxString *wxDirDialog_GetPath(const wxDirDialog * self) {
    return new wxString(self->GetPath());
}
void wxDirDialog_GetPaths(const wxDirDialog * self, wxArrayString * paths) {
    return self->GetPaths(*paths);
}
void wxDirDialog_SetMessage(wxDirDialog * self, const wxString * message) {
    return self->SetMessage(*message);
}
void wxDirDialog_SetPath(wxDirDialog * self, const wxString * path) {
    return self->SetPath(*path);
}

// CLASS: wxDirPickerCtrl
wxClassInfo *wxDirPickerCtrl_CLASSINFO() {
    return wxCLASSINFO(wxDirPickerCtrl);
}
wxDirPickerCtrl *wxDirPickerCtrl_new() {
    return new wxDirPickerCtrl();
}
wxDirPickerCtrl *wxDirPickerCtrl_new1(wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxDirPickerCtrl(parent, id, *path, *message, *pos, *size, style, *validator, *name);
}
bool wxDirPickerCtrl_Create(wxDirPickerCtrl * self, wxWindow * parent, wxWindowID id, const wxString * path, const wxString * message, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *path, *message, *pos, *size, style, *validator, *name);
}
wxFileName *wxDirPickerCtrl_GetDirName(const wxDirPickerCtrl * self) {
    return new wxFileName(self->GetDirName());
}
wxString *wxDirPickerCtrl_GetPath(const wxDirPickerCtrl * self) {
    return new wxString(self->GetPath());
}
void wxDirPickerCtrl_SetDirName(wxDirPickerCtrl * self, const wxFileName * dirname) {
    return self->SetDirName(*dirname);
}
void wxDirPickerCtrl_SetInitialDirectory(wxDirPickerCtrl * self, const wxString * dir) {
    return self->SetInitialDirectory(*dir);
}
void wxDirPickerCtrl_SetPath(wxDirPickerCtrl * self, const wxString * dirname) {
    return self->SetPath(*dirname);
}

// CLASS: wxDisplay
void wxDisplay_delete(wxDisplay *self) {
    delete self;
}
wxDisplay *wxDisplay_new() {
    return new wxDisplay();
}
wxDisplay *wxDisplay_new1(unsigned int index) {
    return new wxDisplay(index);
}
wxDisplay *wxDisplay_new2(const wxWindow * window) {
    return new wxDisplay(window);
}
bool wxDisplay_ChangeMode(wxDisplay * self, const wxVideoMode * mode) {
    return self->ChangeMode(*mode);
}
wxRect *wxDisplay_GetClientArea(const wxDisplay * self) {
    return new wxRect(self->GetClientArea());
}
wxRect *wxDisplay_GetGeometry(const wxDisplay * self) {
    return new wxRect(self->GetGeometry());
}
wxString *wxDisplay_GetName(const wxDisplay * self) {
    return new wxString(self->GetName());
}
wxSize *wxDisplay_GetPPI(const wxDisplay * self) {
    return new wxSize(self->GetPPI());
}
double wxDisplay_GetScaleFactor(const wxDisplay * self) {
    return self->GetScaleFactor();
}
bool wxDisplay_IsPrimary(const wxDisplay * self) {
    return self->IsPrimary();
}
unsigned int wxDisplay_GetCount() {
    return wxDisplay::GetCount();
}
int wxDisplay_GetFromPoint(const wxPoint * pt) {
    return wxDisplay::GetFromPoint(*pt);
}
int wxDisplay_GetFromWindow(const wxWindow * win) {
    return wxDisplay::GetFromWindow(win);
}
int wxDisplay_GetStdPPIValue() {
    return wxDisplay::GetStdPPIValue();
}
wxSize *wxDisplay_GetStdPPI() {
    return new wxSize(wxDisplay::GetStdPPI());
}

// CLASS: wxDisplayChangedEvent
wxClassInfo *wxDisplayChangedEvent_CLASSINFO() {
    return wxCLASSINFO(wxDisplayChangedEvent);
}
wxDisplayChangedEvent *wxDisplayChangedEvent_new() {
    return new wxDisplayChangedEvent();
}

// CLASS: wxDocChildFrame
wxClassInfo *wxDocChildFrame_CLASSINFO() {
    return wxCLASSINFO(wxDocChildFrame);
}
wxDocChildFrame *wxDocChildFrame_new(wxDocument * doc, wxView * view, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxDocChildFrame(doc, view, parent, id, *title, *pos, *size, style, *name);
}
wxDocument * wxDocChildFrame_GetDocument(const wxDocChildFrame * self) {
    return self->GetDocument();
}
wxView * wxDocChildFrame_GetView(const wxDocChildFrame * self) {
    return self->GetView();
}
void wxDocChildFrame_SetDocument(wxDocChildFrame * self, wxDocument * doc) {
    return self->SetDocument(doc);
}
void wxDocChildFrame_SetView(wxDocChildFrame * self, wxView * view) {
    return self->SetView(view);
}

// CLASS: wxDocMDIChildFrame
wxClassInfo *wxDocMDIChildFrame_CLASSINFO() {
    return wxCLASSINFO(wxDocMDIChildFrame);
}
wxDocMDIChildFrame *wxDocMDIChildFrame_new(wxDocument * doc, wxView * view, wxMDIParentFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxDocMDIChildFrame(doc, view, parent, id, *title, *pos, *size, style, *name);
}
wxDocument * wxDocMDIChildFrame_GetDocument(const wxDocMDIChildFrame * self) {
    return self->GetDocument();
}
wxView * wxDocMDIChildFrame_GetView(const wxDocMDIChildFrame * self) {
    return self->GetView();
}
void wxDocMDIChildFrame_SetDocument(wxDocMDIChildFrame * self, wxDocument * doc) {
    return self->SetDocument(doc);
}
void wxDocMDIChildFrame_SetView(wxDocMDIChildFrame * self, wxView * view) {
    return self->SetView(view);
}

// CLASS: wxDocMDIParentFrame
wxClassInfo *wxDocMDIParentFrame_CLASSINFO() {
    return wxCLASSINFO(wxDocMDIParentFrame);
}
wxDocMDIParentFrame *wxDocMDIParentFrame_new() {
    return new wxDocMDIParentFrame();
}
wxDocMDIParentFrame *wxDocMDIParentFrame_new1(wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxDocMDIParentFrame(manager, parent, id, *title, *pos, *size, style, *name);
}
bool wxDocMDIParentFrame_Create(wxDocMDIParentFrame * self, wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(manager, parent, id, *title, *pos, *size, style, *name);
}

// CLASS: wxDocManager
wxClassInfo *wxDocManager_CLASSINFO() {
    return wxCLASSINFO(wxDocManager);
}
wxDocManager *wxDocManager_new(long flags, bool initialize) {
    return new wxDocManager(flags, initialize);
}
void wxDocManager_ActivateView(wxDocManager * self, wxView * doc, bool activate) {
    return self->ActivateView(doc, activate);
}
void wxDocManager_AddDocument(wxDocManager * self, wxDocument * doc) {
    return self->AddDocument(doc);
}
void wxDocManager_AddFileToHistory(wxDocManager * self, const wxString * filename) {
    return self->AddFileToHistory(*filename);
}
void wxDocManager_AssociateTemplate(wxDocManager * self, wxDocTemplate * temp) {
    return self->AssociateTemplate(temp);
}
wxDocTemplate * wxDocManager_FindTemplate(wxDocManager * self, const wxClassInfo * classinfo) {
    return self->FindTemplate(classinfo);
}
wxDocument * wxDocManager_FindDocumentByPath(const wxDocManager * self, const wxString * path) {
    return self->FindDocumentByPath(*path);
}
bool wxDocManager_CloseDocument(wxDocManager * self, wxDocument * doc, bool force) {
    return self->CloseDocument(doc, force);
}
bool wxDocManager_CloseDocuments(wxDocManager * self, bool force) {
    return self->CloseDocuments(force);
}
wxDocument * wxDocManager_CreateDocument(wxDocManager * self, const wxString * path, long flags) {
    return self->CreateDocument(*path, flags);
}
wxDocument * wxDocManager_CreateNewDocument(wxDocManager * self) {
    return self->CreateNewDocument();
}
wxView * wxDocManager_CreateView(wxDocManager * self, wxDocument * doc, long flags) {
    return self->CreateView(doc, flags);
}
void wxDocManager_DisassociateTemplate(wxDocManager * self, wxDocTemplate * temp) {
    return self->DisassociateTemplate(temp);
}
void wxDocManager_FileHistoryAddFilesToMenu(wxDocManager * self) {
    return self->FileHistoryAddFilesToMenu();
}
void wxDocManager_FileHistoryAddFilesToMenu1(wxDocManager * self, wxMenu * menu) {
    return self->FileHistoryAddFilesToMenu(menu);
}
void wxDocManager_FileHistoryLoad(wxDocManager * self, const wxConfigBase * config) {
    return self->FileHistoryLoad(*config);
}
void wxDocManager_FileHistoryRemoveMenu(wxDocManager * self, wxMenu * menu) {
    return self->FileHistoryRemoveMenu(menu);
}
void wxDocManager_FileHistorySave(wxDocManager * self, wxConfigBase * resource_file) {
    return self->FileHistorySave(*resource_file);
}
void wxDocManager_FileHistoryUseMenu(wxDocManager * self, wxMenu * menu) {
    return self->FileHistoryUseMenu(menu);
}
wxDocTemplate * wxDocManager_FindTemplateForPath(wxDocManager * self, const wxString * path) {
    return self->FindTemplateForPath(*path);
}
wxView * wxDocManager_GetAnyUsableView(const wxDocManager * self) {
    return self->GetAnyUsableView();
}
wxDocument * wxDocManager_GetCurrentDocument(const wxDocManager * self) {
    return self->GetCurrentDocument();
}
wxView * wxDocManager_GetCurrentView(const wxDocManager * self) {
    return self->GetCurrentView();
}
wxFileHistory * wxDocManager_GetFileHistory(const wxDocManager * self) {
    return self->GetFileHistory();
}
size_t wxDocManager_GetHistoryFilesCount(const wxDocManager * self) {
    return self->GetHistoryFilesCount();
}
wxString *wxDocManager_GetLastDirectory(const wxDocManager * self) {
    return new wxString(self->GetLastDirectory());
}
int wxDocManager_GetMaxDocsOpen(const wxDocManager * self) {
    return self->GetMaxDocsOpen();
}
bool wxDocManager_Initialize(wxDocManager * self) {
    return self->Initialize();
}
wxString *wxDocManager_MakeNewDocumentName(wxDocManager * self) {
    return new wxString(self->MakeNewDocumentName());
}
wxFileHistory * wxDocManager_OnCreateFileHistory(wxDocManager * self) {
    return self->OnCreateFileHistory();
}
void wxDocManager_OnFileClose(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileClose(*event);
}
void wxDocManager_OnFileCloseAll(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileCloseAll(*event);
}
void wxDocManager_OnFileNew(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileNew(*event);
}
void wxDocManager_OnFileOpen(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileOpen(*event);
}
void wxDocManager_OnFileRevert(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileRevert(*event);
}
void wxDocManager_OnFileSave(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileSave(*event);
}
void wxDocManager_OnFileSaveAs(wxDocManager * self, wxCommandEvent * event) {
    return self->OnFileSaveAs(*event);
}
void wxDocManager_RemoveDocument(wxDocManager * self, wxDocument * doc) {
    return self->RemoveDocument(doc);
}
wxDocTemplate * wxDocManager_SelectDocumentPath(wxDocManager * self, wxDocTemplate ** templates, int no_templates, wxString * path, long flags, bool save) {
    return self->SelectDocumentPath(templates, no_templates, *path, flags, save);
}
wxDocTemplate * wxDocManager_SelectDocumentType(wxDocManager * self, wxDocTemplate ** templates, int no_templates, bool sort) {
    return self->SelectDocumentType(templates, no_templates, sort);
}
wxDocTemplate * wxDocManager_SelectViewType(wxDocManager * self, wxDocTemplate ** templates, int no_templates, bool sort) {
    return self->SelectViewType(templates, no_templates, sort);
}
void wxDocManager_SetLastDirectory(wxDocManager * self, const wxString * dir) {
    return self->SetLastDirectory(*dir);
}
void wxDocManager_SetMaxDocsOpen(wxDocManager * self, int n) {
    return self->SetMaxDocsOpen(n);
}

// CLASS: wxDocParentFrame
wxClassInfo *wxDocParentFrame_CLASSINFO() {
    return wxCLASSINFO(wxDocParentFrame);
}
wxDocParentFrame *wxDocParentFrame_new() {
    return new wxDocParentFrame();
}
wxDocParentFrame *wxDocParentFrame_new1(wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxDocParentFrame(manager, parent, id, *title, *pos, *size, style, *name);
}
bool wxDocParentFrame_Create(wxDocParentFrame * self, wxDocManager * manager, wxFrame * parent, wxWindowID id, const wxString * title, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(manager, parent, id, *title, *pos, *size, style, *name);
}
wxDocManager * wxDocParentFrame_GetDocumentManager(const wxDocParentFrame * self) {
    return self->GetDocumentManager();
}

// CLASS: wxDocTemplate
wxClassInfo *wxDocTemplate_CLASSINFO() {
    return wxCLASSINFO(wxDocTemplate);
}
wxDocTemplate *wxDocTemplate_new(wxDocManager * manager, const wxString * descr, const wxString * filter, const wxString * dir, const wxString * ext, const wxString * doc_type_name, const wxString * view_type_name, wxClassInfo * doc_class_info, wxClassInfo * view_class_info, long flags) {
    return new wxDocTemplate(manager, *descr, *filter, *dir, *ext, *doc_type_name, *view_type_name, doc_class_info, view_class_info, flags);
}
wxDocument * wxDocTemplate_CreateDocument(wxDocTemplate * self, const wxString * path, long flags) {
    return self->CreateDocument(*path, flags);
}
wxView * wxDocTemplate_CreateView(wxDocTemplate * self, wxDocument * doc, long flags) {
    return self->CreateView(doc, flags);
}
bool wxDocTemplate_FileMatchesTemplate(wxDocTemplate * self, const wxString * path) {
    return self->FileMatchesTemplate(*path);
}
wxString *wxDocTemplate_GetDefaultExtension(const wxDocTemplate * self) {
    return new wxString(self->GetDefaultExtension());
}
wxString *wxDocTemplate_GetDescription(const wxDocTemplate * self) {
    return new wxString(self->GetDescription());
}
wxString *wxDocTemplate_GetDirectory(const wxDocTemplate * self) {
    return new wxString(self->GetDirectory());
}
wxClassInfo * wxDocTemplate_GetDocClassInfo(const wxDocTemplate * self) {
    return self->GetDocClassInfo();
}
wxDocManager * wxDocTemplate_GetDocumentManager(const wxDocTemplate * self) {
    return self->GetDocumentManager();
}
wxString *wxDocTemplate_GetDocumentName(const wxDocTemplate * self) {
    return new wxString(self->GetDocumentName());
}
wxString *wxDocTemplate_GetFileFilter(const wxDocTemplate * self) {
    return new wxString(self->GetFileFilter());
}
long wxDocTemplate_GetFlags(const wxDocTemplate * self) {
    return self->GetFlags();
}
wxClassInfo * wxDocTemplate_GetViewClassInfo(const wxDocTemplate * self) {
    return self->GetViewClassInfo();
}
wxString *wxDocTemplate_GetViewName(const wxDocTemplate * self) {
    return new wxString(self->GetViewName());
}
bool wxDocTemplate_InitDocument(wxDocTemplate * self, wxDocument * doc, const wxString * path, long flags) {
    return self->InitDocument(doc, *path, flags);
}
bool wxDocTemplate_IsVisible(const wxDocTemplate * self) {
    return self->IsVisible();
}
void wxDocTemplate_SetDefaultExtension(wxDocTemplate * self, const wxString * ext) {
    return self->SetDefaultExtension(*ext);
}
void wxDocTemplate_SetDescription(wxDocTemplate * self, const wxString * descr) {
    return self->SetDescription(*descr);
}
void wxDocTemplate_SetDirectory(wxDocTemplate * self, const wxString * dir) {
    return self->SetDirectory(*dir);
}
void wxDocTemplate_SetDocumentManager(wxDocTemplate * self, wxDocManager * manager) {
    return self->SetDocumentManager(manager);
}
void wxDocTemplate_SetFileFilter(wxDocTemplate * self, const wxString * filter) {
    return self->SetFileFilter(*filter);
}
void wxDocTemplate_SetFlags(wxDocTemplate * self, long flags) {
    return self->SetFlags(flags);
}

// CLASS: wxDocument
wxClassInfo *wxDocument_CLASSINFO() {
    return wxCLASSINFO(wxDocument);
}
wxDocument *wxDocument_new(wxDocument * parent) {
    return new wxDocument(parent);
}
bool wxDocument_AddView(wxDocument * self, wxView * view) {
    return self->AddView(view);
}
bool wxDocument_AlreadySaved(const wxDocument * self) {
    return self->AlreadySaved();
}
bool wxDocument_Close(wxDocument * self) {
    return self->Close();
}
bool wxDocument_DeleteAllViews(wxDocument * self) {
    return self->DeleteAllViews();
}
bool wxDocument_DeleteContents(wxDocument * self) {
    return self->DeleteContents();
}
wxCommandProcessor * wxDocument_GetCommandProcessor(const wxDocument * self) {
    return self->GetCommandProcessor();
}
wxDocManager * wxDocument_GetDocumentManager(const wxDocument * self) {
    return self->GetDocumentManager();
}
wxString *wxDocument_GetDocumentName(const wxDocument * self) {
    return new wxString(self->GetDocumentName());
}
bool wxDocument_GetDocumentSaved(const wxDocument * self) {
    return self->GetDocumentSaved();
}
wxDocTemplate * wxDocument_GetDocumentTemplate(const wxDocument * self) {
    return self->GetDocumentTemplate();
}
wxWindow * wxDocument_GetDocumentWindow(const wxDocument * self) {
    return self->GetDocumentWindow();
}
wxString *wxDocument_GetFilename(const wxDocument * self) {
    return new wxString(self->GetFilename());
}
wxView * wxDocument_GetFirstView(const wxDocument * self) {
    return self->GetFirstView();
}
wxString *wxDocument_GetTitle(const wxDocument * self) {
    return new wxString(self->GetTitle());
}
wxString *wxDocument_GetUserReadableName(const wxDocument * self) {
    return new wxString(self->GetUserReadableName());
}
bool wxDocument_IsChildDocument(const wxDocument * self) {
    return self->IsChildDocument();
}
bool wxDocument_IsModified(const wxDocument * self) {
    return self->IsModified();
}
void wxDocument_Modify(wxDocument * self, bool modify) {
    return self->Modify(modify);
}
void wxDocument_OnChangedViewList(wxDocument * self) {
    return self->OnChangedViewList();
}
bool wxDocument_OnCloseDocument(wxDocument * self) {
    return self->OnCloseDocument();
}
bool wxDocument_OnCreate(wxDocument * self, const wxString * path, long flags) {
    return self->OnCreate(*path, flags);
}
wxCommandProcessor * wxDocument_OnCreateCommandProcessor(wxDocument * self) {
    return self->OnCreateCommandProcessor();
}
bool wxDocument_OnNewDocument(wxDocument * self) {
    return self->OnNewDocument();
}
bool wxDocument_OnOpenDocument(wxDocument * self, const wxString * filename) {
    return self->OnOpenDocument(*filename);
}
bool wxDocument_OnSaveDocument(wxDocument * self, const wxString * filename) {
    return self->OnSaveDocument(*filename);
}
bool wxDocument_OnSaveModified(wxDocument * self) {
    return self->OnSaveModified();
}
bool wxDocument_RemoveView(wxDocument * self, wxView * view) {
    return self->RemoveView(view);
}
bool wxDocument_Save(wxDocument * self) {
    return self->Save();
}
bool wxDocument_SaveAs(wxDocument * self) {
    return self->SaveAs();
}
bool wxDocument_Revert(wxDocument * self) {
    return self->Revert();
}
void wxDocument_SetCommandProcessor(wxDocument * self, wxCommandProcessor * processor) {
    return self->SetCommandProcessor(processor);
}
void wxDocument_SetDocumentName(wxDocument * self, const wxString * name) {
    return self->SetDocumentName(*name);
}
void wxDocument_SetDocumentTemplate(wxDocument * self, wxDocTemplate * templ) {
    return self->SetDocumentTemplate(templ);
}
void wxDocument_SetDocumentSaved(wxDocument * self, bool saved) {
    return self->SetDocumentSaved(saved);
}
void wxDocument_SetFilename(wxDocument * self, const wxString * filename, bool notify_views) {
    return self->SetFilename(*filename, notify_views);
}
void wxDocument_OnChangeFilename(wxDocument * self, bool notify_views) {
    return self->OnChangeFilename(notify_views);
}
void wxDocument_SetTitle(wxDocument * self, const wxString * title) {
    return self->SetTitle(*title);
}
void wxDocument_UpdateAllViews(wxDocument * self, wxView * sender, wxObject * hint) {
    return self->UpdateAllViews(sender, hint);
}

// CLASS: wxDragImage
wxClassInfo *wxDragImage_CLASSINFO() {
    return wxCLASSINFO(wxDragImage);
}
wxDragImage *wxDragImage_new() {
    return new wxDragImage();
}
wxDragImage *wxDragImage_new1(const wxBitmap * image, const wxCursor * cursor) {
    return new wxDragImage(*image, *cursor);
}
wxDragImage *wxDragImage_new2(const wxIcon * image, const wxCursor * cursor) {
    return new wxDragImage(*image, *cursor);
}
wxDragImage *wxDragImage_new3(const wxString * text, const wxCursor * cursor) {
    return new wxDragImage(*text, *cursor);
}
wxDragImage *wxDragImage_new4(const wxTreeCtrl * tree_ctrl, wxTreeItemId * id) {
    return new wxDragImage(*tree_ctrl, *id);
}
wxDragImage *wxDragImage_new5(const wxListCtrl * list_ctrl, long id) {
    return new wxDragImage(*list_ctrl, id);
}
bool wxDragImage_BeginDrag(wxDragImage * self, const wxPoint * hotspot, wxWindow * window, bool full_screen, wxRect * rect) {
    return self->BeginDrag(*hotspot, window, full_screen, rect);
}
bool wxDragImage_BeginDrag1(wxDragImage * self, const wxPoint * hotspot, wxWindow * window, wxWindow * bounding_window) {
    return self->BeginDrag(*hotspot, window, bounding_window);
}
bool wxDragImage_DoDrawImage(const wxDragImage * self, wxDC * dc, const wxPoint * pos) {
    return self->DoDrawImage(*dc, *pos);
}
bool wxDragImage_EndDrag(wxDragImage * self) {
    return self->EndDrag();
}
wxRect *wxDragImage_GetImageRect(const wxDragImage * self, const wxPoint * pos) {
    return new wxRect(self->GetImageRect(*pos));
}
bool wxDragImage_Hide(wxDragImage * self) {
    return self->Hide();
}
bool wxDragImage_Move(wxDragImage * self, const wxPoint * pt) {
    return self->Move(*pt);
}
bool wxDragImage_Show(wxDragImage * self) {
    return self->Show();
}
bool wxDragImage_UpdateBackingFromWindow(const wxDragImage * self, wxDC * window_dc, wxMemoryDC * dest_dc, const wxRect * source_rect, const wxRect * dest_rect) {
    return self->UpdateBackingFromWindow(*window_dc, *dest_dc, *source_rect, *dest_rect);
}

// CLASS: wxDropFilesEvent
wxClassInfo *wxDropFilesEvent_CLASSINFO() {
    return wxCLASSINFO(wxDropFilesEvent);
}
int wxDropFilesEvent_GetNumberOfFiles(const wxDropFilesEvent * self) {
    return self->GetNumberOfFiles();
}
wxPoint *wxDropFilesEvent_GetPosition(const wxDropFilesEvent * self) {
    return new wxPoint(self->GetPosition());
}

// CLASS: wxDropSource
void wxDropSource_delete(wxDropSource *self) {
    delete self;
}
wxDropSource *wxDropSource_new(wxWindow * win, const wxCursor * icon_copy, const wxCursor * icon_move, const wxCursor * icon_none) {
    return new wxDropSource(win, *icon_copy, *icon_move, *icon_none);
}
wxDropSource *wxDropSource_new1(wxDataObject * data, wxWindow * win, const wxCursor * icon_copy, const wxCursor * icon_move, const wxCursor * icon_none) {
    return new wxDropSource(*data, win, *icon_copy, *icon_move, *icon_none);
}
wxDataObject * wxDropSource_GetDataObject(wxDropSource * self) {
    return self->GetDataObject();
}
void wxDropSource_SetData(wxDropSource * self, wxDataObject * data) {
    return self->SetData(*data);
}

// CLASS: wxDropTarget
void wxDropTarget_delete(wxDropTarget *self) {
    delete self;
}
wxDropTarget *wxDropTarget_new(wxDataObject * data) {
    return new wxDropTarget(data);
}
bool wxDropTarget_GetData(wxDropTarget * self) {
    return self->GetData();
}
bool wxDropTarget_OnDrop(wxDropTarget * self, wxCoord x, wxCoord y) {
    return self->OnDrop(x, y);
}
void wxDropTarget_OnLeave(wxDropTarget * self) {
    return self->OnLeave();
}
wxDataObject * wxDropTarget_GetDataObject(const wxDropTarget * self) {
    return self->GetDataObject();
}
void wxDropTarget_SetDataObject(wxDropTarget * self, wxDataObject * data) {
    return self->SetDataObject(data);
}

} // extern "C"

