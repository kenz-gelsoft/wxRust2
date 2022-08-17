#include "generated.h"

extern "C" {

// CLASS: wxGBPosition
void wxGBPosition_delete(wxGBPosition *self) {
    delete self;
}
wxGBPosition *wxGBPosition_new() {
    return new wxGBPosition();
}
wxGBPosition *wxGBPosition_new1(int row, int col) {
    return new wxGBPosition(row, col);
}
int wxGBPosition_GetCol(const wxGBPosition * self) {
    return self->GetCol();
}
int wxGBPosition_GetRow(const wxGBPosition * self) {
    return self->GetRow();
}
void wxGBPosition_SetCol(wxGBPosition * self, int col) {
    return self->SetCol(col);
}
void wxGBPosition_SetRow(wxGBPosition * self, int row) {
    return self->SetRow(row);
}

// CLASS: wxGBSizerItem
wxClassInfo *wxGBSizerItem_CLASSINFO() {
    return wxCLASSINFO(wxGBSizerItem);
}
wxGBSizerItem *wxGBSizerItem_new(int width, int height, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data) {
    return new wxGBSizerItem(width, height, *pos, *span, flag, border, user_data);
}
wxGBSizerItem *wxGBSizerItem_new1(wxWindow * window, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data) {
    return new wxGBSizerItem(window, *pos, *span, flag, border, user_data);
}
wxGBSizerItem *wxGBSizerItem_new2(wxSizer * sizer, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data) {
    return new wxGBSizerItem(sizer, *pos, *span, flag, border, user_data);
}
void wxGBSizerItem_GetEndPos(wxGBSizerItem * self, int * row, int * col) {
    return self->GetEndPos(*row, *col);
}
wxGBPosition *wxGBSizerItem_GetPos(const wxGBSizerItem * self) {
    return new wxGBPosition(self->GetPos());
}
void wxGBSizerItem_GetPos1(const wxGBSizerItem * self, int * row, int * col) {
    return self->GetPos(*row, *col);
}
wxGBSpan *wxGBSizerItem_GetSpan(const wxGBSizerItem * self) {
    return new wxGBSpan(self->GetSpan());
}
void wxGBSizerItem_GetSpan1(const wxGBSizerItem * self, int * rowspan, int * colspan) {
    return self->GetSpan(*rowspan, *colspan);
}
bool wxGBSizerItem_Intersects(wxGBSizerItem * self, const wxGBSizerItem * other) {
    return self->Intersects(*other);
}
bool wxGBSizerItem_Intersects1(wxGBSizerItem * self, const wxGBPosition * pos, const wxGBSpan * span) {
    return self->Intersects(*pos, *span);
}
bool wxGBSizerItem_SetPos(wxGBSizerItem * self, const wxGBPosition * pos) {
    return self->SetPos(*pos);
}
bool wxGBSizerItem_SetSpan(wxGBSizerItem * self, const wxGBSpan * span) {
    return self->SetSpan(*span);
}
wxGridBagSizer * wxGBSizerItem_GetGBSizer(const wxGBSizerItem * self) {
    return self->GetGBSizer();
}
void wxGBSizerItem_SetGBSizer(wxGBSizerItem * self, wxGridBagSizer * sizer) {
    return self->SetGBSizer(sizer);
}

// CLASS: wxGBSpan
void wxGBSpan_delete(wxGBSpan *self) {
    delete self;
}
wxGBSpan *wxGBSpan_new() {
    return new wxGBSpan();
}
wxGBSpan *wxGBSpan_new1(int rowspan, int colspan) {
    return new wxGBSpan(rowspan, colspan);
}
int wxGBSpan_GetColspan(const wxGBSpan * self) {
    return self->GetColspan();
}
int wxGBSpan_GetRowspan(const wxGBSpan * self) {
    return self->GetRowspan();
}
void wxGBSpan_SetColspan(wxGBSpan * self, int colspan) {
    return self->SetColspan(colspan);
}
void wxGBSpan_SetRowspan(wxGBSpan * self, int rowspan) {
    return self->SetRowspan(rowspan);
}

// CLASS: wxGDIObject
wxClassInfo *wxGDIObject_CLASSINFO() {
    return wxCLASSINFO(wxGDIObject);
}

// CLASS: wxGIFHandler
wxClassInfo *wxGIFHandler_CLASSINFO() {
    return wxCLASSINFO(wxGIFHandler);
}
wxGIFHandler *wxGIFHandler_new() {
    return new wxGIFHandler();
}
bool wxGIFHandler_SaveAnimation(wxGIFHandler * self, const wxImageArray * images, wxOutputStream * stream, bool verbose, int delay_milli_secs) {
    return self->SaveAnimation(*images, stream, verbose, delay_milli_secs);
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

// CLASS: wxGenericAboutDialog
void wxGenericAboutDialog_delete(wxGenericAboutDialog *self) {
    delete self;
}
wxGenericAboutDialog *wxGenericAboutDialog_new() {
    return new wxGenericAboutDialog();
}
wxGenericAboutDialog *wxGenericAboutDialog_new1(const wxAboutDialogInfo * info, wxWindow * parent) {
    return new wxGenericAboutDialog(*info, parent);
}
bool wxGenericAboutDialog_Create(wxGenericAboutDialog * self, const wxAboutDialogInfo * info, wxWindow * parent) {
    return self->Create(*info, parent);
}

// CLASS: wxGenericAnimationCtrl
wxClassInfo *wxGenericAnimationCtrl_CLASSINFO() {
    return wxCLASSINFO(wxGenericAnimationCtrl);
}
wxGenericAnimationCtrl *wxGenericAnimationCtrl_new(wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxGenericAnimationCtrl(parent, id, *anim, *pos, *size, style, *name);
}
bool wxGenericAnimationCtrl_Create(wxGenericAnimationCtrl * self, wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, id, *anim, *pos, *size, style, *name);
}
void wxGenericAnimationCtrl_DrawCurrentFrame(wxGenericAnimationCtrl * self, wxDC * dc) {
    return self->DrawCurrentFrame(*dc);
}
wxBitmap * wxGenericAnimationCtrl_GetBackingStore(wxGenericAnimationCtrl * self) {
    return &(self->GetBackingStore());
}
bool wxGenericAnimationCtrl_Play(wxGenericAnimationCtrl * self, bool looped) {
    return self->Play(looped);
}
void wxGenericAnimationCtrl_SetUseWindowBackgroundColour(wxGenericAnimationCtrl * self, bool use_win_background) {
    return self->SetUseWindowBackgroundColour(use_win_background);
}
bool wxGenericAnimationCtrl_IsUsingWindowBackgroundColour(const wxGenericAnimationCtrl * self) {
    return self->IsUsingWindowBackgroundColour();
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

// CLASS: wxGenericProgressDialog
wxClassInfo *wxGenericProgressDialog_CLASSINFO() {
    return wxCLASSINFO(wxGenericProgressDialog);
}
wxGenericProgressDialog *wxGenericProgressDialog_new(const wxString * title, const wxString * message, int maximum, wxWindow * parent, int style) {
    return new wxGenericProgressDialog(*title, *message, maximum, parent, style);
}
int wxGenericProgressDialog_GetValue(const wxGenericProgressDialog * self) {
    return self->GetValue();
}
int wxGenericProgressDialog_GetRange(const wxGenericProgressDialog * self) {
    return self->GetRange();
}
wxString *wxGenericProgressDialog_GetMessage(const wxGenericProgressDialog * self) {
    return new wxString(self->GetMessage());
}
bool wxGenericProgressDialog_Pulse(wxGenericProgressDialog * self, const wxString * newmsg, bool * skip) {
    return self->Pulse(*newmsg, skip);
}
void wxGenericProgressDialog_Resume(wxGenericProgressDialog * self) {
    return self->Resume();
}
void wxGenericProgressDialog_SetRange(wxGenericProgressDialog * self, int maximum) {
    return self->SetRange(maximum);
}
bool wxGenericProgressDialog_WasCancelled(const wxGenericProgressDialog * self) {
    return self->WasCancelled();
}
bool wxGenericProgressDialog_WasSkipped(const wxGenericProgressDialog * self) {
    return self->WasSkipped();
}
bool wxGenericProgressDialog_Update(wxGenericProgressDialog * self, int value, const wxString * newmsg, bool * skip) {
    return self->Update(value, *newmsg, skip);
}

// CLASS: wxGenericValidator
wxClassInfo *wxGenericValidator_CLASSINFO() {
    return wxCLASSINFO(wxGenericValidator);
}
wxGenericValidator *wxGenericValidator_new(const wxGenericValidator * validator) {
    return new wxGenericValidator(*validator);
}
wxGenericValidator *wxGenericValidator_new1(bool * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new2(wxString * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new3(int * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new4(wxArrayInt * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new5(wxDateTime * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new6(wxFileName * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new7(float * val_ptr) {
    return new wxGenericValidator(val_ptr);
}
wxGenericValidator *wxGenericValidator_new8(double * val_ptr) {
    return new wxGenericValidator(val_ptr);
}

// CLASS: wxGestureEvent
wxClassInfo *wxGestureEvent_CLASSINFO() {
    return wxCLASSINFO(wxGestureEvent);
}
wxPoint *wxGestureEvent_GetPosition(const wxGestureEvent * self) {
    return new wxPoint(self->GetPosition());
}
bool wxGestureEvent_IsGestureStart(const wxGestureEvent * self) {
    return self->IsGestureStart();
}
bool wxGestureEvent_IsGestureEnd(const wxGestureEvent * self) {
    return self->IsGestureEnd();
}
void wxGestureEvent_SetPosition(wxGestureEvent * self, const wxPoint * pos) {
    return self->SetPosition(*pos);
}
void wxGestureEvent_SetGestureStart(wxGestureEvent * self, bool is_start) {
    return self->SetGestureStart(is_start);
}
void wxGestureEvent_SetGestureEnd(wxGestureEvent * self, bool is_end) {
    return self->SetGestureEnd(is_end);
}

// CLASS: wxGraphicsBrush
wxClassInfo *wxGraphicsBrush_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsBrush);
}

// CLASS: wxGraphicsContext
wxClassInfo *wxGraphicsContext_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsContext);
}
wxGraphicsContext * wxGraphicsContext_Create(wxWindow * window) {
    return wxGraphicsContext::Create(window);
}
wxGraphicsContext * wxGraphicsContext_Create1(const wxWindowDC * window_dc) {
    return wxGraphicsContext::Create(*window_dc);
}
wxGraphicsContext * wxGraphicsContext_Create2(const wxMemoryDC * memory_dc) {
    return wxGraphicsContext::Create(*memory_dc);
}
wxGraphicsContext * wxGraphicsContext_Create3(const wxPrinterDC * printer_dc) {
    return wxGraphicsContext::Create(*printer_dc);
}
#ifdef __WXMSW__
wxGraphicsContext * wxGraphicsContext_Create4(const wxEnhMetaFileDC * meta_file_dc) {
    return wxGraphicsContext::Create(*meta_file_dc);
}
#endif
wxGraphicsContext * wxGraphicsContext_CreateFromUnknownDC(wxDC * dc) {
    return wxGraphicsContext::CreateFromUnknownDC(*dc);
}
wxGraphicsContext * wxGraphicsContext_Create5(wxImage * image) {
    return wxGraphicsContext::Create(*image);
}
wxGraphicsContext * wxGraphicsContext_CreateFromNative(void * context) {
    return wxGraphicsContext::CreateFromNative(context);
}
wxGraphicsContext * wxGraphicsContext_CreateFromNativeWindow(void * window) {
    return wxGraphicsContext::CreateFromNativeWindow(window);
}
wxGraphicsContext * wxGraphicsContext_Create6() {
    return wxGraphicsContext::Create();
}
void wxGraphicsContext_ResetClip(wxGraphicsContext * self) {
    return self->ResetClip();
}
void wxGraphicsContext_Clip(wxGraphicsContext * self, const wxRegion * region) {
    return self->Clip(*region);
}
void wxGraphicsContext_GetClipBox(wxGraphicsContext * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h) {
    return self->GetClipBox(x, y, w, h);
}
wxGraphicsMatrix *wxGraphicsContext_CreateMatrix1(const wxGraphicsContext * self, const wxAffineMatrix2DBase * mat) {
    return new wxGraphicsMatrix(self->CreateMatrix(*mat));
}
void wxGraphicsContext_ConcatTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix) {
    return self->ConcatTransform(*matrix);
}
wxGraphicsMatrix *wxGraphicsContext_GetTransform(const wxGraphicsContext * self) {
    return new wxGraphicsMatrix(self->GetTransform());
}
void wxGraphicsContext_SetTransform(wxGraphicsContext * self, const wxGraphicsMatrix * matrix) {
    return self->SetTransform(*matrix);
}
wxGraphicsBrush *wxGraphicsContext_CreateBrush(const wxGraphicsContext * self, const wxBrush * brush) {
    return new wxGraphicsBrush(self->CreateBrush(*brush));
}
void wxGraphicsContext_SetBrush(wxGraphicsContext * self, const wxBrush * brush) {
    return self->SetBrush(*brush);
}
void wxGraphicsContext_SetBrush1(wxGraphicsContext * self, const wxGraphicsBrush * brush) {
    return self->SetBrush(*brush);
}
wxGraphicsPen *wxGraphicsContext_CreatePen(const wxGraphicsContext * self, const wxPen * pen) {
    return new wxGraphicsPen(self->CreatePen(*pen));
}
wxGraphicsPen *wxGraphicsContext_CreatePen1(const wxGraphicsContext * self, const wxGraphicsPenInfo * info) {
    return new wxGraphicsPen(self->CreatePen(*info));
}
void wxGraphicsContext_SetPen(wxGraphicsContext * self, const wxPen * pen) {
    return self->SetPen(*pen);
}
void wxGraphicsContext_SetPen1(wxGraphicsContext * self, const wxGraphicsPen * pen) {
    return self->SetPen(*pen);
}
wxGraphicsPath *wxGraphicsContext_CreatePath(const wxGraphicsContext * self) {
    return new wxGraphicsPath(self->CreatePath());
}
void wxGraphicsContext_StrokeLines(wxGraphicsContext * self, size_t n, const wxPoint2DDouble * begin_points, const wxPoint2DDouble * end_points) {
    return self->StrokeLines(n, begin_points, end_points);
}
void wxGraphicsContext_StrokeLines1(wxGraphicsContext * self, size_t n, const wxPoint2DDouble * points) {
    return self->StrokeLines(n, points);
}
void wxGraphicsContext_StrokePath(wxGraphicsContext * self, const wxGraphicsPath * path) {
    return self->StrokePath(*path);
}
wxGraphicsFont *wxGraphicsContext_CreateFont(const wxGraphicsContext * self, const wxFont * font, const wxColour * col) {
    return new wxGraphicsFont(self->CreateFont(*font, *col));
}
wxGraphicsFont *wxGraphicsContext_CreateFont1(const wxGraphicsContext * self, double size_in_pixels, const wxString * facename, int flags, const wxColour * col) {
    return new wxGraphicsFont(self->CreateFont(size_in_pixels, *facename, flags, *col));
}
void wxGraphicsContext_SetFont(wxGraphicsContext * self, const wxFont * font, const wxColour * colour) {
    return self->SetFont(*font, *colour);
}
void wxGraphicsContext_SetFont1(wxGraphicsContext * self, const wxGraphicsFont * font) {
    return self->SetFont(*font);
}
void wxGraphicsContext_GetPartialTextExtents(const wxGraphicsContext * self, const wxString * text, wxArrayDouble * widths) {
    return self->GetPartialTextExtents(*text, *widths);
}
void wxGraphicsContext_GetTextExtent(const wxGraphicsContext * self, const wxString * text, wxDouble * width, wxDouble * height, wxDouble * descent, wxDouble * external_leading) {
    return self->GetTextExtent(*text, width, height, descent, external_leading);
}
bool wxGraphicsContext_StartDoc(wxGraphicsContext * self, const wxString * message) {
    return self->StartDoc(*message);
}
void wxGraphicsContext_EndDoc(wxGraphicsContext * self) {
    return self->EndDoc();
}
void wxGraphicsContext_EndPage(wxGraphicsContext * self) {
    return self->EndPage();
}
void wxGraphicsContext_EndLayer(wxGraphicsContext * self) {
    return self->EndLayer();
}
void wxGraphicsContext_PushState(wxGraphicsContext * self) {
    return self->PushState();
}
void wxGraphicsContext_PopState(wxGraphicsContext * self) {
    return self->PopState();
}
void wxGraphicsContext_Flush(wxGraphicsContext * self) {
    return self->Flush();
}
void * wxGraphicsContext_GetNativeContext(wxGraphicsContext * self) {
    return self->GetNativeContext();
}
void wxGraphicsContext_GetSize(const wxGraphicsContext * self, wxDouble * width, wxDouble * height) {
    return self->GetSize(width, height);
}
void wxGraphicsContext_GetDPI(const wxGraphicsContext * self, wxDouble * dpi_x, wxDouble * dpi_y) {
    return self->GetDPI(dpi_x, dpi_y);
}
wxWindow * wxGraphicsContext_GetWindow(const wxGraphicsContext * self) {
    return self->GetWindow();
}
bool wxGraphicsContext_ShouldOffset(const wxGraphicsContext * self) {
    return self->ShouldOffset();
}
void wxGraphicsContext_EnableOffset(wxGraphicsContext * self, bool enable) {
    return self->EnableOffset(enable);
}
void wxGraphicsContext_DisableOffset(wxGraphicsContext * self) {
    return self->DisableOffset();
}
bool wxGraphicsContext_OffsetEnabled(const wxGraphicsContext * self) {
    return self->OffsetEnabled();
}
wxSize *wxGraphicsContext_FromDIP(const wxGraphicsContext * self, const wxSize * sz) {
    return new wxSize(self->FromDIP(*sz));
}
wxPoint *wxGraphicsContext_FromDIP1(const wxGraphicsContext * self, const wxPoint * pt) {
    return new wxPoint(self->FromDIP(*pt));
}
int wxGraphicsContext_FromDIP2(const wxGraphicsContext * self, int d) {
    return self->FromDIP(d);
}
wxSize *wxGraphicsContext_ToDIP(const wxGraphicsContext * self, const wxSize * sz) {
    return new wxSize(self->ToDIP(*sz));
}
wxPoint *wxGraphicsContext_ToDIP1(const wxGraphicsContext * self, const wxPoint * pt) {
    return new wxPoint(self->ToDIP(*pt));
}
int wxGraphicsContext_ToDIP2(const wxGraphicsContext * self, int d) {
    return self->ToDIP(d);
}

// CLASS: wxGraphicsFont
wxClassInfo *wxGraphicsFont_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsFont);
}

// CLASS: wxGraphicsGradientStop
void wxGraphicsGradientStop_delete(wxGraphicsGradientStop *self) {
    delete self;
}
wxColour *wxGraphicsGradientStop_GetColour(const wxGraphicsGradientStop * self) {
    return new wxColour(self->GetColour());
}
void wxGraphicsGradientStop_SetColour(wxGraphicsGradientStop * self, const wxColour * col) {
    return self->SetColour(*col);
}

// CLASS: wxGraphicsGradientStops
void wxGraphicsGradientStops_delete(wxGraphicsGradientStops *self) {
    delete self;
}
wxGraphicsGradientStops *wxGraphicsGradientStops_new(wxColour start_col, wxColour end_col) {
    return new wxGraphicsGradientStops(start_col, end_col);
}
void wxGraphicsGradientStops_Add(wxGraphicsGradientStops * self, const wxGraphicsGradientStop * stop) {
    return self->Add(*stop);
}
size_t wxGraphicsGradientStops_GetCount(const wxGraphicsGradientStops * self) {
    return self->GetCount();
}
void wxGraphicsGradientStops_SetStartColour(wxGraphicsGradientStops * self, wxColour col) {
    return self->SetStartColour(col);
}
wxColour *wxGraphicsGradientStops_GetStartColour(const wxGraphicsGradientStops * self) {
    return new wxColour(self->GetStartColour());
}
void wxGraphicsGradientStops_SetEndColour(wxGraphicsGradientStops * self, wxColour col) {
    return self->SetEndColour(col);
}
wxColour *wxGraphicsGradientStops_GetEndColour(const wxGraphicsGradientStops * self) {
    return new wxColour(self->GetEndColour());
}

// CLASS: wxGraphicsMatrix
wxClassInfo *wxGraphicsMatrix_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsMatrix);
}
void wxGraphicsMatrix_Concat(wxGraphicsMatrix * self, const wxGraphicsMatrix * t) {
    return self->Concat(t);
}
void wxGraphicsMatrix_Get(const wxGraphicsMatrix * self, wxDouble * a, wxDouble * b, wxDouble * c, wxDouble * d, wxDouble * tx, wxDouble * ty) {
    return self->Get(a, b, c, d, tx, ty);
}
void * wxGraphicsMatrix_GetNativeMatrix(const wxGraphicsMatrix * self) {
    return self->GetNativeMatrix();
}
void wxGraphicsMatrix_Invert(wxGraphicsMatrix * self) {
    return self->Invert();
}
bool wxGraphicsMatrix_IsEqual1(const wxGraphicsMatrix * self, const wxGraphicsMatrix * t) {
    return self->IsEqual(*t);
}
bool wxGraphicsMatrix_IsIdentity(const wxGraphicsMatrix * self) {
    return self->IsIdentity();
}
void wxGraphicsMatrix_TransformDistance(const wxGraphicsMatrix * self, wxDouble * dx, wxDouble * dy) {
    return self->TransformDistance(dx, dy);
}
void wxGraphicsMatrix_TransformPoint(const wxGraphicsMatrix * self, wxDouble * x, wxDouble * y) {
    return self->TransformPoint(x, y);
}

// CLASS: wxGraphicsObject
wxClassInfo *wxGraphicsObject_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsObject);
}
wxGraphicsRenderer * wxGraphicsObject_GetRenderer(const wxGraphicsObject * self) {
    return self->GetRenderer();
}
bool wxGraphicsObject_IsNull(const wxGraphicsObject * self) {
    return self->IsNull();
}

// CLASS: wxGraphicsPath
wxClassInfo *wxGraphicsPath_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsPath);
}
void wxGraphicsPath_AddCurveToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * c1, const wxPoint2DDouble * c2, const wxPoint2DDouble * e) {
    return self->AddCurveToPoint(*c1, *c2, *e);
}
void wxGraphicsPath_AddLineToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * p) {
    return self->AddLineToPoint(*p);
}
void wxGraphicsPath_AddPath(wxGraphicsPath * self, const wxGraphicsPath * path) {
    return self->AddPath(*path);
}
void wxGraphicsPath_CloseSubpath(wxGraphicsPath * self) {
    return self->CloseSubpath();
}
void wxGraphicsPath_GetBox1(const wxGraphicsPath * self, wxDouble * x, wxDouble * y, wxDouble * w, wxDouble * h) {
    return self->GetBox(x, y, w, h);
}
void wxGraphicsPath_GetCurrentPoint(const wxGraphicsPath * self, wxDouble * x, wxDouble * y) {
    return self->GetCurrentPoint(x, y);
}
void * wxGraphicsPath_GetNativePath(const wxGraphicsPath * self) {
    return self->GetNativePath();
}
void wxGraphicsPath_MoveToPoint1(wxGraphicsPath * self, const wxPoint2DDouble * p) {
    return self->MoveToPoint(*p);
}
void wxGraphicsPath_Transform(wxGraphicsPath * self, const wxGraphicsMatrix * matrix) {
    return self->Transform(*matrix);
}
void wxGraphicsPath_UnGetNativePath(const wxGraphicsPath * self, void * p) {
    return self->UnGetNativePath(p);
}

// CLASS: wxGraphicsPen
wxClassInfo *wxGraphicsPen_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsPen);
}

// CLASS: wxGraphicsRenderer
wxClassInfo *wxGraphicsRenderer_CLASSINFO() {
    return wxCLASSINFO(wxGraphicsRenderer);
}
wxGraphicsContext * wxGraphicsRenderer_CreateContext(wxGraphicsRenderer * self, wxWindow * window) {
    return self->CreateContext(window);
}
wxGraphicsContext * wxGraphicsRenderer_CreateContext1(wxGraphicsRenderer * self, const wxWindowDC * window_dc) {
    return self->CreateContext(*window_dc);
}
wxGraphicsContext * wxGraphicsRenderer_CreateContext2(wxGraphicsRenderer * self, const wxMemoryDC * memory_dc) {
    return self->CreateContext(*memory_dc);
}
wxGraphicsContext * wxGraphicsRenderer_CreateContext3(wxGraphicsRenderer * self, const wxPrinterDC * printer_dc) {
    return self->CreateContext(*printer_dc);
}
#ifdef __WXMSW__
wxGraphicsContext * wxGraphicsRenderer_CreateContext4(wxGraphicsRenderer * self, const wxEnhMetaFileDC * meta_file_dc) {
    return self->CreateContext(*meta_file_dc);
}
#endif
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromUnknownDC(wxGraphicsRenderer * self, wxDC * dc) {
    return self->CreateContextFromUnknownDC(*dc);
}
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromImage(wxGraphicsRenderer * self, wxImage * image) {
    return self->CreateContextFromImage(*image);
}
wxGraphicsBrush *wxGraphicsRenderer_CreateBrush(wxGraphicsRenderer * self, const wxBrush * brush) {
    return new wxGraphicsBrush(self->CreateBrush(*brush));
}
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromNativeContext(wxGraphicsRenderer * self, void * context) {
    return self->CreateContextFromNativeContext(context);
}
wxGraphicsContext * wxGraphicsRenderer_CreateContextFromNativeWindow(wxGraphicsRenderer * self, void * window) {
    return self->CreateContextFromNativeWindow(window);
}
wxGraphicsContext * wxGraphicsRenderer_CreateMeasuringContext(wxGraphicsRenderer * self) {
    return self->CreateMeasuringContext();
}
wxGraphicsFont *wxGraphicsRenderer_CreateFont(wxGraphicsRenderer * self, const wxFont * font, const wxColour * col) {
    return new wxGraphicsFont(self->CreateFont(*font, *col));
}
wxGraphicsFont *wxGraphicsRenderer_CreateFont1(wxGraphicsRenderer * self, double size_in_pixels, const wxString * facename, int flags, const wxColour * col) {
    return new wxGraphicsFont(self->CreateFont(size_in_pixels, *facename, flags, *col));
}
wxGraphicsFont *wxGraphicsRenderer_CreateFontAtDPI(wxGraphicsRenderer * self, const wxFont * font, const wxRealPoint * dpi, const wxColour * col) {
    return new wxGraphicsFont(self->CreateFontAtDPI(*font, *dpi, *col));
}
wxGraphicsPath *wxGraphicsRenderer_CreatePath(wxGraphicsRenderer * self) {
    return new wxGraphicsPath(self->CreatePath());
}
wxGraphicsPen *wxGraphicsRenderer_CreatePen(wxGraphicsRenderer * self, const wxGraphicsPenInfo * info) {
    return new wxGraphicsPen(self->CreatePen(*info));
}
wxString *wxGraphicsRenderer_GetName(const wxGraphicsRenderer * self) {
    return new wxString(self->GetName());
}
void wxGraphicsRenderer_GetVersion(const wxGraphicsRenderer * self, int * major, int * minor, int * micro) {
    return self->GetVersion(major, minor, micro);
}
wxGraphicsRenderer * wxGraphicsRenderer_GetDefaultRenderer() {
    return wxGraphicsRenderer::GetDefaultRenderer();
}
wxGraphicsRenderer * wxGraphicsRenderer_GetCairoRenderer() {
    return wxGraphicsRenderer::GetCairoRenderer();
}
#ifdef __WXMSW__
wxGraphicsRenderer * wxGraphicsRenderer_GetGDIPlusRenderer() {
    return wxGraphicsRenderer::GetGDIPlusRenderer();
}
wxGraphicsRenderer * wxGraphicsRenderer_GetDirect2DRenderer() {
    return wxGraphicsRenderer::GetDirect2DRenderer();
}
#endif

// CLASS: wxGridBagSizer
wxClassInfo *wxGridBagSizer_CLASSINFO() {
    return wxCLASSINFO(wxGridBagSizer);
}
wxGridBagSizer *wxGridBagSizer_new(int vgap, int hgap) {
    return new wxGridBagSizer(vgap, hgap);
}
wxSizerItem * wxGridBagSizer_Add(wxGridBagSizer * self, wxWindow * window, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data) {
    return self->Add(window, *pos, *span, flag, border, user_data);
}
wxSizerItem * wxGridBagSizer_Add1(wxGridBagSizer * self, wxSizer * sizer, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data) {
    return self->Add(sizer, *pos, *span, flag, border, user_data);
}
wxSizerItem * wxGridBagSizer_Add2(wxGridBagSizer * self, wxGBSizerItem * item) {
    return self->Add(item);
}
wxSizerItem * wxGridBagSizer_Add3(wxGridBagSizer * self, int width, int height, const wxGBPosition * pos, const wxGBSpan * span, int flag, int border, wxObject * user_data) {
    return self->Add(width, height, *pos, *span, flag, border, user_data);
}
bool wxGridBagSizer_CheckForIntersection(wxGridBagSizer * self, wxGBSizerItem * item, wxGBSizerItem * exclude_item) {
    return self->CheckForIntersection(item, exclude_item);
}
bool wxGridBagSizer_CheckForIntersection1(wxGridBagSizer * self, const wxGBPosition * pos, const wxGBSpan * span, wxGBSizerItem * exclude_item) {
    return self->CheckForIntersection(*pos, *span, exclude_item);
}
wxGBSizerItem * wxGridBagSizer_FindItem(wxGridBagSizer * self, wxWindow * window) {
    return self->FindItem(window);
}
wxGBSizerItem * wxGridBagSizer_FindItem1(wxGridBagSizer * self, wxSizer * sizer) {
    return self->FindItem(sizer);
}
wxGBSizerItem * wxGridBagSizer_FindItemAtPoint(wxGridBagSizer * self, const wxPoint * pt) {
    return self->FindItemAtPoint(*pt);
}
wxGBSizerItem * wxGridBagSizer_FindItemAtPosition(wxGridBagSizer * self, const wxGBPosition * pos) {
    return self->FindItemAtPosition(*pos);
}
wxGBSizerItem * wxGridBagSizer_FindItemWithData(wxGridBagSizer * self, const wxObject * user_data) {
    return self->FindItemWithData(user_data);
}
wxSize *wxGridBagSizer_GetCellSize(const wxGridBagSizer * self, int row, int col) {
    return new wxSize(self->GetCellSize(row, col));
}
wxSize *wxGridBagSizer_GetEmptyCellSize(const wxGridBagSizer * self) {
    return new wxSize(self->GetEmptyCellSize());
}
wxGBPosition *wxGridBagSizer_GetItemPosition(wxGridBagSizer * self, wxWindow * window) {
    return new wxGBPosition(self->GetItemPosition(window));
}
wxGBPosition *wxGridBagSizer_GetItemPosition1(wxGridBagSizer * self, wxSizer * sizer) {
    return new wxGBPosition(self->GetItemPosition(sizer));
}
wxGBPosition *wxGridBagSizer_GetItemPosition2(wxGridBagSizer * self, size_t index) {
    return new wxGBPosition(self->GetItemPosition(index));
}
wxGBSpan *wxGridBagSizer_GetItemSpan(wxGridBagSizer * self, wxWindow * window) {
    return new wxGBSpan(self->GetItemSpan(window));
}
wxGBSpan *wxGridBagSizer_GetItemSpan1(wxGridBagSizer * self, wxSizer * sizer) {
    return new wxGBSpan(self->GetItemSpan(sizer));
}
wxGBSpan *wxGridBagSizer_GetItemSpan2(wxGridBagSizer * self, size_t index) {
    return new wxGBSpan(self->GetItemSpan(index));
}
void wxGridBagSizer_SetEmptyCellSize(wxGridBagSizer * self, const wxSize * sz) {
    return self->SetEmptyCellSize(*sz);
}
bool wxGridBagSizer_SetItemPosition(wxGridBagSizer * self, wxWindow * window, const wxGBPosition * pos) {
    return self->SetItemPosition(window, *pos);
}
bool wxGridBagSizer_SetItemPosition1(wxGridBagSizer * self, wxSizer * sizer, const wxGBPosition * pos) {
    return self->SetItemPosition(sizer, *pos);
}
bool wxGridBagSizer_SetItemPosition2(wxGridBagSizer * self, size_t index, const wxGBPosition * pos) {
    return self->SetItemPosition(index, *pos);
}
bool wxGridBagSizer_SetItemSpan(wxGridBagSizer * self, wxWindow * window, const wxGBSpan * span) {
    return self->SetItemSpan(window, *span);
}
bool wxGridBagSizer_SetItemSpan1(wxGridBagSizer * self, wxSizer * sizer, const wxGBSpan * span) {
    return self->SetItemSpan(sizer, *span);
}
bool wxGridBagSizer_SetItemSpan2(wxGridBagSizer * self, size_t index, const wxGBSpan * span) {
    return self->SetItemSpan(index, *span);
}

// CLASS: wxGridCellAttr
wxGridCellAttr *wxGridCellAttr_new(wxGridCellAttr * attr_default) {
    return new wxGridCellAttr(attr_default);
}
wxGridCellAttr *wxGridCellAttr_new1(const wxColour * col_text, const wxColour * col_back, const wxFont * font, int h_align, int v_align) {
    return new wxGridCellAttr(*col_text, *col_back, *font, h_align, v_align);
}
wxGridCellAttr * wxGridCellAttr_Clone(const wxGridCellAttr * self) {
    return self->Clone();
}
void wxGridCellAttr_DecRef(wxGridCellAttr * self) {
    return self->DecRef();
}
void wxGridCellAttr_GetAlignment(const wxGridCellAttr * self, int * h_align, int * v_align) {
    return self->GetAlignment(h_align, v_align);
}
wxColour *wxGridCellAttr_GetBackgroundColour(const wxGridCellAttr * self) {
    return new wxColour(self->GetBackgroundColour());
}
wxGridCellEditor * wxGridCellAttr_GetEditor(const wxGridCellAttr * self, const wxGrid * grid, int row, int col) {
    return self->GetEditor(grid, row, col);
}
wxFont *wxGridCellAttr_GetFont(const wxGridCellAttr * self) {
    return new wxFont(self->GetFont());
}
void wxGridCellAttr_GetNonDefaultAlignment(const wxGridCellAttr * self, int * h_align, int * v_align) {
    return self->GetNonDefaultAlignment(h_align, v_align);
}
wxGridCellRenderer * wxGridCellAttr_GetRenderer(const wxGridCellAttr * self, const wxGrid * grid, int row, int col) {
    return self->GetRenderer(grid, row, col);
}
wxColour *wxGridCellAttr_GetTextColour(const wxGridCellAttr * self) {
    return new wxColour(self->GetTextColour());
}
bool wxGridCellAttr_HasAlignment(const wxGridCellAttr * self) {
    return self->HasAlignment();
}
bool wxGridCellAttr_HasBackgroundColour(const wxGridCellAttr * self) {
    return self->HasBackgroundColour();
}
bool wxGridCellAttr_HasEditor(const wxGridCellAttr * self) {
    return self->HasEditor();
}
bool wxGridCellAttr_HasFont(const wxGridCellAttr * self) {
    return self->HasFont();
}
bool wxGridCellAttr_HasRenderer(const wxGridCellAttr * self) {
    return self->HasRenderer();
}
bool wxGridCellAttr_HasTextColour(const wxGridCellAttr * self) {
    return self->HasTextColour();
}
void wxGridCellAttr_IncRef(wxGridCellAttr * self) {
    return self->IncRef();
}
bool wxGridCellAttr_IsReadOnly(const wxGridCellAttr * self) {
    return self->IsReadOnly();
}
void wxGridCellAttr_SetAlignment(wxGridCellAttr * self, int h_align, int v_align) {
    return self->SetAlignment(h_align, v_align);
}
void wxGridCellAttr_SetBackgroundColour(wxGridCellAttr * self, const wxColour * col_back) {
    return self->SetBackgroundColour(*col_back);
}
void wxGridCellAttr_SetDefAttr(wxGridCellAttr * self, wxGridCellAttr * def_attr) {
    return self->SetDefAttr(def_attr);
}
void wxGridCellAttr_SetEditor(wxGridCellAttr * self, wxGridCellEditor * editor) {
    return self->SetEditor(editor);
}
void wxGridCellAttr_SetFont(wxGridCellAttr * self, const wxFont * font) {
    return self->SetFont(*font);
}
void wxGridCellAttr_SetReadOnly(wxGridCellAttr * self, bool is_read_only) {
    return self->SetReadOnly(is_read_only);
}
void wxGridCellAttr_SetRenderer(wxGridCellAttr * self, wxGridCellRenderer * renderer) {
    return self->SetRenderer(renderer);
}
void wxGridCellAttr_SetTextColour(wxGridCellAttr * self, const wxColour * col_text) {
    return self->SetTextColour(*col_text);
}
void wxGridCellAttr_MergeWith(wxGridCellAttr * self, wxGridCellAttr * mergefrom) {
    return self->MergeWith(mergefrom);
}
void wxGridCellAttr_SetSize(wxGridCellAttr * self, int num_rows, int num_cols) {
    return self->SetSize(num_rows, num_cols);
}
void wxGridCellAttr_SetFitMode(wxGridCellAttr * self, wxGridFitMode fit_mode) {
    return self->SetFitMode(fit_mode);
}
void wxGridCellAttr_SetOverflow(wxGridCellAttr * self, bool allow) {
    return self->SetOverflow(allow);
}
bool wxGridCellAttr_HasReadWriteMode(const wxGridCellAttr * self) {
    return self->HasReadWriteMode();
}
bool wxGridCellAttr_HasOverflowMode(const wxGridCellAttr * self) {
    return self->HasOverflowMode();
}
bool wxGridCellAttr_HasSize(const wxGridCellAttr * self) {
    return self->HasSize();
}
void wxGridCellAttr_GetSize(const wxGridCellAttr * self, int * num_rows, int * num_cols) {
    return self->GetSize(num_rows, num_cols);
}
wxGridFitMode *wxGridCellAttr_GetFitMode(const wxGridCellAttr * self) {
    return new wxGridFitMode(self->GetFitMode());
}
bool wxGridCellAttr_GetOverflow(const wxGridCellAttr * self) {
    return self->GetOverflow();
}
bool wxGridCellAttr_CanOverflow(const wxGridCellAttr * self) {
    return self->CanOverflow();
}
// Mix-in(s) to wxGridCellAttr
wxRefCounter *wxGridCellAttr_AsRefCounter(wxGridCellAttr* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellAutoWrapStringEditor
wxGridCellAutoWrapStringEditor *wxGridCellAutoWrapStringEditor_new() {
    return new wxGridCellAutoWrapStringEditor();
}
// Mix-in(s) to wxGridCellAutoWrapStringEditor
wxRefCounter *wxGridCellAutoWrapStringEditor_AsRefCounter(wxGridCellAutoWrapStringEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellAutoWrapStringRenderer
wxGridCellAutoWrapStringRenderer *wxGridCellAutoWrapStringRenderer_new() {
    return new wxGridCellAutoWrapStringRenderer();
}
// Mix-in(s) to wxGridCellAutoWrapStringRenderer
wxRefCounter *wxGridCellAutoWrapStringRenderer_AsRefCounter(wxGridCellAutoWrapStringRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellBoolEditor
wxGridCellBoolEditor *wxGridCellBoolEditor_new() {
    return new wxGridCellBoolEditor();
}
bool wxGridCellBoolEditor_IsTrueValue(const wxString * value) {
    return wxGridCellBoolEditor::IsTrueValue(*value);
}
void wxGridCellBoolEditor_UseStringValues(const wxString * value_true, const wxString * value_false) {
    return wxGridCellBoolEditor::UseStringValues(*value_true, *value_false);
}
// Mix-in(s) to wxGridCellBoolEditor
wxRefCounter *wxGridCellBoolEditor_AsRefCounter(wxGridCellBoolEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellBoolRenderer
wxGridCellBoolRenderer *wxGridCellBoolRenderer_new() {
    return new wxGridCellBoolRenderer();
}
// Mix-in(s) to wxGridCellBoolRenderer
wxRefCounter *wxGridCellBoolRenderer_AsRefCounter(wxGridCellBoolRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellChoiceEditor
wxGridCellChoiceEditor *wxGridCellChoiceEditor_new1(const wxArrayString * choices, bool allow_others) {
    return new wxGridCellChoiceEditor(*choices, allow_others);
}
void wxGridCellChoiceEditor_SetParameters(wxGridCellChoiceEditor * self, const wxString * params) {
    return self->SetParameters(*params);
}
// Mix-in(s) to wxGridCellChoiceEditor
wxRefCounter *wxGridCellChoiceEditor_AsRefCounter(wxGridCellChoiceEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellDateEditor
wxGridCellDateEditor *wxGridCellDateEditor_new(const wxString * format) {
    return new wxGridCellDateEditor(*format);
}
// Mix-in(s) to wxGridCellDateEditor
wxRefCounter *wxGridCellDateEditor_AsRefCounter(wxGridCellDateEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellDateRenderer
wxGridCellDateRenderer *wxGridCellDateRenderer_new(const wxString * outformat) {
    return new wxGridCellDateRenderer(*outformat);
}
void wxGridCellDateRenderer_SetParameters(wxGridCellDateRenderer * self, const wxString * params) {
    return self->SetParameters(*params);
}
// Mix-in(s) to wxGridCellDateRenderer
wxRefCounter *wxGridCellDateRenderer_AsRefCounter(wxGridCellDateRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellDateTimeRenderer
wxGridCellDateTimeRenderer *wxGridCellDateTimeRenderer_new(const wxString * outformat, const wxString * informat) {
    return new wxGridCellDateTimeRenderer(*outformat, *informat);
}
// Mix-in(s) to wxGridCellDateTimeRenderer
wxRefCounter *wxGridCellDateTimeRenderer_AsRefCounter(wxGridCellDateTimeRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellEditor
void wxGridCellEditor_BeginEdit(wxGridCellEditor * self, int row, int col, wxGrid * grid) {
    return self->BeginEdit(row, col, grid);
}
wxGridCellEditor * wxGridCellEditor_Clone(const wxGridCellEditor * self) {
    return self->Clone();
}
void wxGridCellEditor_Create(wxGridCellEditor * self, wxWindow * parent, wxWindowID id, wxEvtHandler * evt_handler) {
    return self->Create(parent, id, evt_handler);
}
void wxGridCellEditor_Destroy(wxGridCellEditor * self) {
    return self->Destroy();
}
bool wxGridCellEditor_EndEdit(wxGridCellEditor * self, int row, int col, const wxGrid * grid, const wxString * oldval, wxString * newval) {
    return self->EndEdit(row, col, grid, *oldval, newval);
}
void wxGridCellEditor_ApplyEdit(wxGridCellEditor * self, int row, int col, wxGrid * grid) {
    return self->ApplyEdit(row, col, grid);
}
void wxGridCellEditor_HandleReturn(wxGridCellEditor * self, wxKeyEvent * event) {
    return self->HandleReturn(*event);
}
bool wxGridCellEditor_IsCreated(wxGridCellEditor * self) {
    return self->IsCreated();
}
void wxGridCellEditor_PaintBackground(wxGridCellEditor * self, wxDC * dc, const wxRect * rect_cell, const wxGridCellAttr * attr) {
    return self->PaintBackground(*dc, *rect_cell, *attr);
}
void wxGridCellEditor_Reset(wxGridCellEditor * self) {
    return self->Reset();
}
void wxGridCellEditor_SetSize(wxGridCellEditor * self, const wxRect * rect) {
    return self->SetSize(*rect);
}
void wxGridCellEditor_Show(wxGridCellEditor * self, bool show, wxGridCellAttr * attr) {
    return self->Show(show, attr);
}
void wxGridCellEditor_StartingClick(wxGridCellEditor * self) {
    return self->StartingClick();
}
void wxGridCellEditor_StartingKey(wxGridCellEditor * self, wxKeyEvent * event) {
    return self->StartingKey(*event);
}
bool wxGridCellEditor_IsAcceptedKey(wxGridCellEditor * self, wxKeyEvent * event) {
    return self->IsAcceptedKey(*event);
}
wxString *wxGridCellEditor_GetValue(const wxGridCellEditor * self) {
    return new wxString(self->GetValue());
}
wxWindow * wxGridCellEditor_GetWindow(const wxGridCellEditor * self) {
    return self->GetWindow();
}
void wxGridCellEditor_SetWindow(wxGridCellEditor * self, wxWindow * window) {
    return self->SetWindow(window);
}
wxControl * wxGridCellEditor_GetControl(wxGridCellEditor * self) {
    return self->GetControl();
}
void wxGridCellEditor_SetControl(wxGridCellEditor * self, wxControl * control) {
    return self->SetControl(control);
}
void wxGridCellEditor_DoActivate(wxGridCellEditor * self, int row, int col, wxGrid * grid) {
    return self->DoActivate(row, col, grid);
}
// Mix-in(s) to wxGridCellEditor
wxRefCounter *wxGridCellEditor_AsRefCounter(wxGridCellEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellEnumEditor
wxGridCellEnumEditor *wxGridCellEnumEditor_new(const wxString * choices) {
    return new wxGridCellEnumEditor(*choices);
}
// Mix-in(s) to wxGridCellEnumEditor
wxRefCounter *wxGridCellEnumEditor_AsRefCounter(wxGridCellEnumEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellEnumRenderer
wxGridCellEnumRenderer *wxGridCellEnumRenderer_new(const wxString * choices) {
    return new wxGridCellEnumRenderer(*choices);
}
void wxGridCellEnumRenderer_SetParameters(wxGridCellEnumRenderer * self, const wxString * params) {
    return self->SetParameters(*params);
}
// Mix-in(s) to wxGridCellEnumRenderer
wxRefCounter *wxGridCellEnumRenderer_AsRefCounter(wxGridCellEnumRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellFloatEditor
wxGridCellFloatEditor *wxGridCellFloatEditor_new(int width, int precision, int format) {
    return new wxGridCellFloatEditor(width, precision, format);
}
// Mix-in(s) to wxGridCellFloatEditor
wxRefCounter *wxGridCellFloatEditor_AsRefCounter(wxGridCellFloatEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellFloatRenderer
wxGridCellFloatRenderer *wxGridCellFloatRenderer_new(int width, int precision, int format) {
    return new wxGridCellFloatRenderer(width, precision, format);
}
int wxGridCellFloatRenderer_GetFormat(const wxGridCellFloatRenderer * self) {
    return self->GetFormat();
}
int wxGridCellFloatRenderer_GetPrecision(const wxGridCellFloatRenderer * self) {
    return self->GetPrecision();
}
int wxGridCellFloatRenderer_GetWidth(const wxGridCellFloatRenderer * self) {
    return self->GetWidth();
}
void wxGridCellFloatRenderer_SetFormat(wxGridCellFloatRenderer * self, int format) {
    return self->SetFormat(format);
}
void wxGridCellFloatRenderer_SetParameters(wxGridCellFloatRenderer * self, const wxString * params) {
    return self->SetParameters(*params);
}
void wxGridCellFloatRenderer_SetPrecision(wxGridCellFloatRenderer * self, int precision) {
    return self->SetPrecision(precision);
}
void wxGridCellFloatRenderer_SetWidth(wxGridCellFloatRenderer * self, int width) {
    return self->SetWidth(width);
}
// Mix-in(s) to wxGridCellFloatRenderer
wxRefCounter *wxGridCellFloatRenderer_AsRefCounter(wxGridCellFloatRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellNumberEditor
wxGridCellNumberEditor *wxGridCellNumberEditor_new(int min, int max) {
    return new wxGridCellNumberEditor(min, max);
}
// Mix-in(s) to wxGridCellNumberEditor
wxRefCounter *wxGridCellNumberEditor_AsRefCounter(wxGridCellNumberEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellNumberRenderer
wxGridCellNumberRenderer *wxGridCellNumberRenderer_new() {
    return new wxGridCellNumberRenderer();
}
// Mix-in(s) to wxGridCellNumberRenderer
wxRefCounter *wxGridCellNumberRenderer_AsRefCounter(wxGridCellNumberRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellRenderer
wxGridCellRenderer * wxGridCellRenderer_Clone(const wxGridCellRenderer * self) {
    return self->Clone();
}
void wxGridCellRenderer_Draw(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, const wxRect * rect, int row, int col, bool is_selected) {
    return self->Draw(*grid, *attr, *dc, *rect, row, col, is_selected);
}
wxSize *wxGridCellRenderer_GetBestSize(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col) {
    return new wxSize(self->GetBestSize(*grid, *attr, *dc, row, col));
}
int wxGridCellRenderer_GetBestHeight(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col, int width) {
    return self->GetBestHeight(*grid, *attr, *dc, row, col, width);
}
int wxGridCellRenderer_GetBestWidth(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc, int row, int col, int height) {
    return self->GetBestWidth(*grid, *attr, *dc, row, col, height);
}
wxSize *wxGridCellRenderer_GetMaxBestSize(wxGridCellRenderer * self, wxGrid * grid, wxGridCellAttr * attr, wxDC * dc) {
    return new wxSize(self->GetMaxBestSize(*grid, *attr, *dc));
}
// Mix-in(s) to wxGridCellRenderer
wxRefCounter *wxGridCellRenderer_AsRefCounter(wxGridCellRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellStringRenderer
wxGridCellStringRenderer *wxGridCellStringRenderer_new() {
    return new wxGridCellStringRenderer();
}
// Mix-in(s) to wxGridCellStringRenderer
wxRefCounter *wxGridCellStringRenderer_AsRefCounter(wxGridCellStringRenderer* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridCellTextEditor
wxGridCellTextEditor *wxGridCellTextEditor_new(size_t max_chars) {
    return new wxGridCellTextEditor(max_chars);
}
void wxGridCellTextEditor_SetParameters(wxGridCellTextEditor * self, const wxString * params) {
    return self->SetParameters(*params);
}
void wxGridCellTextEditor_SetValidator(wxGridCellTextEditor * self, const wxValidator * validator) {
    return self->SetValidator(*validator);
}
// Mix-in(s) to wxGridCellTextEditor
wxRefCounter *wxGridCellTextEditor_AsRefCounter(wxGridCellTextEditor* obj) {
    return static_cast<wxRefCounter*>(obj);
}

// CLASS: wxGridEditorCreatedEvent
wxClassInfo *wxGridEditorCreatedEvent_CLASSINFO() {
    return wxCLASSINFO(wxGridEditorCreatedEvent);
}
wxGridEditorCreatedEvent *wxGridEditorCreatedEvent_new() {
    return new wxGridEditorCreatedEvent();
}
int wxGridEditorCreatedEvent_GetCol(const wxGridEditorCreatedEvent * self) {
    return self->GetCol();
}
wxControl * wxGridEditorCreatedEvent_GetControl(wxGridEditorCreatedEvent * self) {
    return self->GetControl();
}
int wxGridEditorCreatedEvent_GetRow(const wxGridEditorCreatedEvent * self) {
    return self->GetRow();
}
wxWindow * wxGridEditorCreatedEvent_GetWindow(const wxGridEditorCreatedEvent * self) {
    return self->GetWindow();
}
void wxGridEditorCreatedEvent_SetCol(wxGridEditorCreatedEvent * self, int col) {
    return self->SetCol(col);
}
void wxGridEditorCreatedEvent_SetControl(wxGridEditorCreatedEvent * self, wxControl * ctrl) {
    return self->SetControl(ctrl);
}
void wxGridEditorCreatedEvent_SetRow(wxGridEditorCreatedEvent * self, int row) {
    return self->SetRow(row);
}
void wxGridEditorCreatedEvent_SetWindow(wxGridEditorCreatedEvent * self, wxWindow * window) {
    return self->SetWindow(window);
}

// CLASS: wxGridEvent
wxClassInfo *wxGridEvent_CLASSINFO() {
    return wxCLASSINFO(wxGridEvent);
}
wxGridEvent *wxGridEvent_new() {
    return new wxGridEvent();
}
bool wxGridEvent_AltDown(const wxGridEvent * self) {
    return self->AltDown();
}
bool wxGridEvent_ControlDown(const wxGridEvent * self) {
    return self->ControlDown();
}
int wxGridEvent_GetCol(const wxGridEvent * self) {
    return self->GetCol();
}
wxPoint *wxGridEvent_GetPosition(const wxGridEvent * self) {
    return new wxPoint(self->GetPosition());
}
int wxGridEvent_GetRow(const wxGridEvent * self) {
    return self->GetRow();
}
bool wxGridEvent_MetaDown(const wxGridEvent * self) {
    return self->MetaDown();
}
bool wxGridEvent_Selecting(const wxGridEvent * self) {
    return self->Selecting();
}
bool wxGridEvent_ShiftDown(const wxGridEvent * self) {
    return self->ShiftDown();
}

// CLASS: wxGridFitMode
void wxGridFitMode_delete(wxGridFitMode *self) {
    delete self;
}
wxGridFitMode *wxGridFitMode_new() {
    return new wxGridFitMode();
}
bool wxGridFitMode_IsSpecified(const wxGridFitMode * self) {
    return self->IsSpecified();
}
bool wxGridFitMode_IsClip(const wxGridFitMode * self) {
    return self->IsClip();
}
bool wxGridFitMode_IsOverflow(const wxGridFitMode * self) {
    return self->IsOverflow();
}
wxEllipsizeMode wxGridFitMode_GetEllipsizeMode(const wxGridFitMode * self) {
    return self->GetEllipsizeMode();
}
wxGridFitMode *wxGridFitMode_Clip() {
    return new wxGridFitMode(wxGridFitMode::Clip());
}
wxGridFitMode *wxGridFitMode_Overflow() {
    return new wxGridFitMode(wxGridFitMode::Overflow());
}
wxGridFitMode *wxGridFitMode_Ellipsize(wxEllipsizeMode ellipsize) {
    return new wxGridFitMode(wxGridFitMode::Ellipsize(ellipsize));
}

// CLASS: wxGridRangeSelectEvent
wxClassInfo *wxGridRangeSelectEvent_CLASSINFO() {
    return wxCLASSINFO(wxGridRangeSelectEvent);
}
wxGridRangeSelectEvent *wxGridRangeSelectEvent_new() {
    return new wxGridRangeSelectEvent();
}
bool wxGridRangeSelectEvent_AltDown(const wxGridRangeSelectEvent * self) {
    return self->AltDown();
}
bool wxGridRangeSelectEvent_ControlDown(const wxGridRangeSelectEvent * self) {
    return self->ControlDown();
}
int wxGridRangeSelectEvent_GetBottomRow(const wxGridRangeSelectEvent * self) {
    return self->GetBottomRow();
}
int wxGridRangeSelectEvent_GetLeftCol(const wxGridRangeSelectEvent * self) {
    return self->GetLeftCol();
}
int wxGridRangeSelectEvent_GetRightCol(const wxGridRangeSelectEvent * self) {
    return self->GetRightCol();
}
int wxGridRangeSelectEvent_GetTopRow(const wxGridRangeSelectEvent * self) {
    return self->GetTopRow();
}
bool wxGridRangeSelectEvent_MetaDown(const wxGridRangeSelectEvent * self) {
    return self->MetaDown();
}
bool wxGridRangeSelectEvent_Selecting(const wxGridRangeSelectEvent * self) {
    return self->Selecting();
}
bool wxGridRangeSelectEvent_ShiftDown(const wxGridRangeSelectEvent * self) {
    return self->ShiftDown();
}

// CLASS: wxGridSizeEvent
wxClassInfo *wxGridSizeEvent_CLASSINFO() {
    return wxCLASSINFO(wxGridSizeEvent);
}
wxGridSizeEvent *wxGridSizeEvent_new() {
    return new wxGridSizeEvent();
}
bool wxGridSizeEvent_AltDown(const wxGridSizeEvent * self) {
    return self->AltDown();
}
bool wxGridSizeEvent_ControlDown(const wxGridSizeEvent * self) {
    return self->ControlDown();
}
wxPoint *wxGridSizeEvent_GetPosition(const wxGridSizeEvent * self) {
    return new wxPoint(self->GetPosition());
}
int wxGridSizeEvent_GetRowOrCol(const wxGridSizeEvent * self) {
    return self->GetRowOrCol();
}
bool wxGridSizeEvent_MetaDown(const wxGridSizeEvent * self) {
    return self->MetaDown();
}
bool wxGridSizeEvent_ShiftDown(const wxGridSizeEvent * self) {
    return self->ShiftDown();
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

// CLASS: wxGridTableBase
wxClassInfo *wxGridTableBase_CLASSINFO() {
    return wxCLASSINFO(wxGridTableBase);
}
bool wxGridTableBase_IsEmptyCell(wxGridTableBase * self, int row, int col) {
    return self->IsEmptyCell(row, col);
}
bool wxGridTableBase_IsEmpty(wxGridTableBase * self, const wxGridCellCoords * coords) {
    return self->IsEmpty(*coords);
}
wxString *wxGridTableBase_GetValue(wxGridTableBase * self, int row, int col) {
    return new wxString(self->GetValue(row, col));
}
void wxGridTableBase_SetValue(wxGridTableBase * self, int row, int col, const wxString * value) {
    return self->SetValue(row, col, *value);
}
wxString *wxGridTableBase_GetTypeName(wxGridTableBase * self, int row, int col) {
    return new wxString(self->GetTypeName(row, col));
}
bool wxGridTableBase_CanGetValueAs(wxGridTableBase * self, int row, int col, const wxString * type_name) {
    return self->CanGetValueAs(row, col, *type_name);
}
bool wxGridTableBase_CanSetValueAs(wxGridTableBase * self, int row, int col, const wxString * type_name) {
    return self->CanSetValueAs(row, col, *type_name);
}
long wxGridTableBase_GetValueAsLong(wxGridTableBase * self, int row, int col) {
    return self->GetValueAsLong(row, col);
}
double wxGridTableBase_GetValueAsDouble(wxGridTableBase * self, int row, int col) {
    return self->GetValueAsDouble(row, col);
}
bool wxGridTableBase_GetValueAsBool(wxGridTableBase * self, int row, int col) {
    return self->GetValueAsBool(row, col);
}
void * wxGridTableBase_GetValueAsCustom(wxGridTableBase * self, int row, int col, const wxString * type_name) {
    return self->GetValueAsCustom(row, col, *type_name);
}
void wxGridTableBase_SetValueAsLong(wxGridTableBase * self, int row, int col, long value) {
    return self->SetValueAsLong(row, col, value);
}
void wxGridTableBase_SetValueAsDouble(wxGridTableBase * self, int row, int col, double value) {
    return self->SetValueAsDouble(row, col, value);
}
void wxGridTableBase_SetValueAsBool(wxGridTableBase * self, int row, int col, bool value) {
    return self->SetValueAsBool(row, col, value);
}
void wxGridTableBase_SetValueAsCustom(wxGridTableBase * self, int row, int col, const wxString * type_name, void * value) {
    return self->SetValueAsCustom(row, col, *type_name, value);
}
void wxGridTableBase_SetView(wxGridTableBase * self, wxGrid * grid) {
    return self->SetView(grid);
}
wxGrid * wxGridTableBase_GetView(const wxGridTableBase * self) {
    return self->GetView();
}
void wxGridTableBase_Clear(wxGridTableBase * self) {
    return self->Clear();
}
bool wxGridTableBase_InsertRows(wxGridTableBase * self, size_t pos, size_t num_rows) {
    return self->InsertRows(pos, num_rows);
}
bool wxGridTableBase_AppendRows(wxGridTableBase * self, size_t num_rows) {
    return self->AppendRows(num_rows);
}
bool wxGridTableBase_DeleteRows(wxGridTableBase * self, size_t pos, size_t num_rows) {
    return self->DeleteRows(pos, num_rows);
}
bool wxGridTableBase_InsertCols(wxGridTableBase * self, size_t pos, size_t num_cols) {
    return self->InsertCols(pos, num_cols);
}
bool wxGridTableBase_AppendCols(wxGridTableBase * self, size_t num_cols) {
    return self->AppendCols(num_cols);
}
bool wxGridTableBase_DeleteCols(wxGridTableBase * self, size_t pos, size_t num_cols) {
    return self->DeleteCols(pos, num_cols);
}
wxString *wxGridTableBase_GetRowLabelValue(wxGridTableBase * self, int row) {
    return new wxString(self->GetRowLabelValue(row));
}
wxString *wxGridTableBase_GetColLabelValue(wxGridTableBase * self, int col) {
    return new wxString(self->GetColLabelValue(col));
}
wxString *wxGridTableBase_GetCornerLabelValue(const wxGridTableBase * self) {
    return new wxString(self->GetCornerLabelValue());
}
void wxGridTableBase_SetRowLabelValue(wxGridTableBase * self, int row, const wxString * label) {
    return self->SetRowLabelValue(row, *label);
}
void wxGridTableBase_SetColLabelValue(wxGridTableBase * self, int col, const wxString * label) {
    return self->SetColLabelValue(col, *label);
}
void wxGridTableBase_SetAttrProvider(wxGridTableBase * self, wxGridCellAttrProvider * attr_provider) {
    return self->SetAttrProvider(attr_provider);
}
wxGridCellAttrProvider * wxGridTableBase_GetAttrProvider(const wxGridTableBase * self) {
    return self->GetAttrProvider();
}
void wxGridTableBase_SetAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row, int col) {
    return self->SetAttr(attr, row, col);
}
void wxGridTableBase_SetRowAttr(wxGridTableBase * self, wxGridCellAttr * attr, int row) {
    return self->SetRowAttr(attr, row);
}
void wxGridTableBase_SetColAttr(wxGridTableBase * self, wxGridCellAttr * attr, int col) {
    return self->SetColAttr(attr, col);
}
bool wxGridTableBase_CanHaveAttributes(wxGridTableBase * self) {
    return self->CanHaveAttributes();
}
bool wxGridTableBase_CanMeasureColUsingSameAttr(const wxGridTableBase * self, int col) {
    return self->CanMeasureColUsingSameAttr(col);
}
int wxGridTableBase_GetNumberRows(wxGridTableBase * self) {
    return self->GetNumberRows();
}
int wxGridTableBase_GetNumberCols(wxGridTableBase * self) {
    return self->GetNumberCols();
}
int wxGridTableBase_GetRowsCount(const wxGridTableBase * self) {
    return self->GetRowsCount();
}
int wxGridTableBase_GetColsCount(const wxGridTableBase * self) {
    return self->GetColsCount();
}

// CLASS: wxGridUpdateLocker
void wxGridUpdateLocker_delete(wxGridUpdateLocker *self) {
    delete self;
}
wxGridUpdateLocker *wxGridUpdateLocker_new(wxGrid * grid) {
    return new wxGridUpdateLocker(grid);
}
void wxGridUpdateLocker_Create(wxGridUpdateLocker * self, wxGrid * grid) {
    return self->Create(grid);
}

} // extern "C"

