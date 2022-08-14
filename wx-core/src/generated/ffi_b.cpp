#include "generated.h"

extern "C" {

// CLASS: wxBannerWindow
wxClassInfo *wxBannerWindow_CLASSINFO() {
    return wxCLASSINFO(wxBannerWindow);
}
wxBannerWindow *wxBannerWindow_new() {
    return new wxBannerWindow();
}
wxBannerWindow *wxBannerWindow_new2(wxWindow * parent, wxWindowID winid, wxDirection dir, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return new wxBannerWindow(parent, winid, dir, *pos, *size, style, *name);
}
bool wxBannerWindow_Create(wxBannerWindow * self, wxWindow * parent, wxWindowID winid, wxDirection dir, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, winid, dir, *pos, *size, style, *name);
}
void wxBannerWindow_SetBitmap(wxBannerWindow * self, const wxBitmapBundle * bmp) {
    return self->SetBitmap(*bmp);
}
void wxBannerWindow_SetText(wxBannerWindow * self, const wxString * title, const wxString * message) {
    return self->SetText(*title, *message);
}
void wxBannerWindow_SetGradient(wxBannerWindow * self, const wxColour * start, const wxColour * end) {
    return self->SetGradient(*start, *end);
}

// CLASS: wxBitmap
wxClassInfo *wxBitmap_CLASSINFO() {
    return wxCLASSINFO(wxBitmap);
}
wxBitmap *wxBitmap_new() {
    return new wxBitmap();
}
wxBitmap *wxBitmap_new1(const wxBitmap * bitmap) {
    return new wxBitmap(*bitmap);
}
wxBitmap *wxBitmap_new3(int width, int height, int depth) {
    return new wxBitmap(width, height, depth);
}
wxBitmap *wxBitmap_new4(const wxSize * sz, int depth) {
    return new wxBitmap(*sz, depth);
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new5(int width, int height, const wxDC * dc) {
    return new wxBitmap(width, height, *dc);
}
#endif
wxBitmap *wxBitmap_new6(const char *const * bits) {
    return new wxBitmap(bits);
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new8(const wxImage * img, int depth) {
    return new wxBitmap(*img, depth);
}
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxBitmap *wxBitmap_new9(const wxImage * img, const wxDC * dc) {
    return new wxBitmap(*img, *dc);
}
#endif
#if wxCHECK_VERSION(3, 1, 0)
wxBitmap *wxBitmap_new10(const wxCursor * cursor) {
    return new wxBitmap(*cursor);
}
#endif
bool wxBitmap_CopyFromIcon(wxBitmap * self, const wxIcon * icon) {
    return self->CopyFromIcon(*icon);
}
bool wxBitmap_Create(wxBitmap * self, int width, int height, int depth) {
    return self->Create(width, height, depth);
}
bool wxBitmap_Create1(wxBitmap * self, const wxSize * sz, int depth) {
    return self->Create(*sz, depth);
}
bool wxBitmap_Create2(wxBitmap * self, int width, int height, const wxDC * dc) {
    return self->Create(width, height, *dc);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmap_CreateWithDIPSize(wxBitmap * self, const wxSize * size, double scale, int depth) {
    return self->CreateWithDIPSize(*size, scale, depth);
}
bool wxBitmap_CreateWithDIPSize1(wxBitmap * self, int width, int height, double scale, int depth) {
    return self->CreateWithDIPSize(width, height, scale, depth);
}
#endif
bool wxBitmap_CreateScaled(wxBitmap * self, int width, int height, int depth, double logical_scale) {
    return self->CreateScaled(width, height, depth, logical_scale);
}
int wxBitmap_GetDepth(const wxBitmap * self) {
    return self->GetDepth();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxBitmap_GetDIPSize(const wxBitmap * self) {
    return new wxSize(self->GetDIPSize());
}
#endif
int wxBitmap_GetHeight(const wxBitmap * self) {
    return self->GetHeight();
}
#if wxCHECK_VERSION(3, 1, 0)
double wxBitmap_GetLogicalHeight(const wxBitmap * self) {
    return self->GetLogicalHeight();
}
wxSize *wxBitmap_GetLogicalSize(const wxBitmap * self) {
    return new wxSize(self->GetLogicalSize());
}
double wxBitmap_GetLogicalWidth(const wxBitmap * self) {
    return self->GetLogicalWidth();
}
#endif
wxMask * wxBitmap_GetMask(const wxBitmap * self) {
    return self->GetMask();
}
wxPalette * wxBitmap_GetPalette(const wxBitmap * self) {
    return self->GetPalette();
}
wxBitmap *wxBitmap_GetSubBitmap(const wxBitmap * self, const wxRect * rect) {
    return new wxBitmap(self->GetSubBitmap(*rect));
}
double wxBitmap_GetScaleFactor(const wxBitmap * self) {
    return self->GetScaleFactor();
}
double wxBitmap_GetScaledHeight(const wxBitmap * self) {
    return self->GetScaledHeight();
}
wxSize *wxBitmap_GetScaledSize(const wxBitmap * self) {
    return new wxSize(self->GetScaledSize());
}
double wxBitmap_GetScaledWidth(const wxBitmap * self) {
    return self->GetScaledWidth();
}
wxSize *wxBitmap_GetSize(const wxBitmap * self) {
    return new wxSize(self->GetSize());
}
int wxBitmap_GetWidth(const wxBitmap * self) {
    return self->GetWidth();
}
bool wxBitmap_HasAlpha(const wxBitmap * self) {
    return self->HasAlpha();
}
bool wxBitmap_IsOk(const wxBitmap * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetDepth(wxBitmap * self, int depth) {
    return self->SetDepth(depth);
}
void wxBitmap_SetHeight(wxBitmap * self, int height) {
    return self->SetHeight(height);
}
#endif
#if wxCHECK_VERSION(3, 1, 0)
void wxBitmap_SetScaleFactor(wxBitmap * self, double scale) {
    return self->SetScaleFactor(scale);
}
#endif
void wxBitmap_SetMask(wxBitmap * self, wxMask * mask) {
    return self->SetMask(mask);
}
void wxBitmap_SetPalette(wxBitmap * self, const wxPalette * palette) {
    return self->SetPalette(*palette);
}
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmap_SetWidth(wxBitmap * self, int width) {
    return self->SetWidth(width);
}
#endif
void wxBitmap_AddHandler(wxBitmapHandler * handler) {
    return wxBitmap::AddHandler(handler);
}
void wxBitmap_CleanUpHandlers() {
    return wxBitmap::CleanUpHandlers();
}
#ifndef __WXMSW__
wxBitmapHandler * wxBitmap_FindHandler(const wxString * name) {
    return wxBitmap::FindHandler(*name);
}
#endif
void wxBitmap_InitStandardHandlers() {
    return wxBitmap::InitStandardHandlers();
}
void wxBitmap_InsertHandler(wxBitmapHandler * handler) {
    return wxBitmap::InsertHandler(handler);
}
wxBitmap *wxBitmap_NewFromPNGData(const void * data, size_t size) {
    return new wxBitmap(wxBitmap::NewFromPNGData(data, size));
}
bool wxBitmap_RemoveHandler(const wxString * name) {
    return wxBitmap::RemoveHandler(*name);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxBitmap_Rescale(wxBitmap * bmp, const wxSize * size_needed) {
    return wxBitmap::Rescale(*bmp, *size_needed);
}
#endif

// CLASS: wxBitmapBundle
void wxBitmapBundle_delete(wxBitmapBundle *self) {
    delete self;
}
wxBitmapBundle *wxBitmapBundle_new() {
    return new wxBitmapBundle();
}
wxBitmapBundle *wxBitmapBundle_new1(const wxBitmap * bitmap) {
    return new wxBitmapBundle(*bitmap);
}
wxBitmapBundle *wxBitmapBundle_new2(const wxIcon * icon) {
    return new wxBitmapBundle(*icon);
}
wxBitmapBundle *wxBitmapBundle_new3(const wxImage * image) {
    return new wxBitmapBundle(*image);
}
#if wxCHECK_VERSION(3, 2, 0)
wxBitmapBundle *wxBitmapBundle_new4(const char *const * xpm) {
    return new wxBitmapBundle(xpm);
}
#endif
wxBitmapBundle *wxBitmapBundle_new5(const wxBitmapBundle * other) {
    return new wxBitmapBundle(*other);
}
#if wxCHECK_VERSION(3, 1, 7)
void wxBitmapBundle_Clear(wxBitmapBundle * self) {
    return self->Clear();
}
#endif
bool wxBitmapBundle_IsOk(const wxBitmapBundle * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxBitmapBundle_GetDefaultSize(const wxBitmapBundle * self) {
    return new wxSize(self->GetDefaultSize());
}
wxSize *wxBitmapBundle_GetPreferredBitmapSizeAtScale(const wxBitmapBundle * self, double scale) {
    return new wxSize(self->GetPreferredBitmapSizeAtScale(scale));
}
wxSize *wxBitmapBundle_GetPreferredBitmapSizeFor(const wxBitmapBundle * self, const wxWindow * window) {
    return new wxSize(self->GetPreferredBitmapSizeFor(window));
}
wxSize *wxBitmapBundle_GetPreferredLogicalSizeFor(const wxBitmapBundle * self, const wxWindow * window) {
    return new wxSize(self->GetPreferredLogicalSizeFor(window));
}
wxBitmap *wxBitmapBundle_GetBitmap(const wxBitmapBundle * self, const wxSize * size) {
    return new wxBitmap(self->GetBitmap(*size));
}
wxBitmap *wxBitmapBundle_GetBitmapFor(const wxBitmapBundle * self, const wxWindow * window) {
    return new wxBitmap(self->GetBitmapFor(window));
}
wxIcon *wxBitmapBundle_GetIcon(const wxBitmapBundle * self, const wxSize * size) {
    return new wxIcon(self->GetIcon(*size));
}
wxIcon *wxBitmapBundle_GetIconFor(const wxBitmapBundle * self, const wxWindow * window) {
    return new wxIcon(self->GetIconFor(window));
}
#endif
bool wxBitmapBundle_IsSameAs(const wxBitmapBundle * self, const wxBitmapBundle * other) {
    return self->IsSameAs(*other);
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxBitmapBundle_FromBitmaps1(const wxBitmap * bitmap1, const wxBitmap * bitmap2) {
    return new wxBitmapBundle(wxBitmapBundle::FromBitmaps(*bitmap1, *bitmap2));
}
wxBitmapBundle *wxBitmapBundle_FromBitmap(const wxBitmap * bitmap) {
    return new wxBitmapBundle(wxBitmapBundle::FromBitmap(*bitmap));
}
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxBitmapBundle *wxBitmapBundle_FromIconBundle(const wxIconBundle * icon_bundle) {
    return new wxBitmapBundle(wxBitmapBundle::FromIconBundle(*icon_bundle));
}
#endif
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxBitmapBundle_FromImage(const wxImage * image) {
    return new wxBitmapBundle(wxBitmapBundle::FromImage(*image));
}
wxBitmapBundle *wxBitmapBundle_FromImpl(wxBitmapBundleImpl * impl_) {
    return new wxBitmapBundle(wxBitmapBundle::FromImpl(impl_));
}
wxBitmapBundle *wxBitmapBundle_FromResources(const wxString * name) {
    return new wxBitmapBundle(wxBitmapBundle::FromResources(*name));
}
wxBitmapBundle *wxBitmapBundle_FromFiles(const wxString * path, const wxString * filename, const wxString * extension) {
    return new wxBitmapBundle(wxBitmapBundle::FromFiles(*path, *filename, *extension));
}
wxBitmapBundle *wxBitmapBundle_FromFiles1(const wxString * fullpathname) {
    return new wxBitmapBundle(wxBitmapBundle::FromFiles(*fullpathname));
}
wxBitmapBundle *wxBitmapBundle_FromSVG1(const char * data, const wxSize * size_def) {
    return new wxBitmapBundle(wxBitmapBundle::FromSVG(data, *size_def));
}
wxBitmapBundle *wxBitmapBundle_FromSVGFile(const wxString * path, const wxSize * size_def) {
    return new wxBitmapBundle(wxBitmapBundle::FromSVGFile(*path, *size_def));
}
wxBitmapBundle *wxBitmapBundle_FromSVGResource(const wxString * name, const wxSize * size_def) {
    return new wxBitmapBundle(wxBitmapBundle::FromSVGResource(*name, *size_def));
}
#endif

// CLASS: wxBitmapBundleImpl
wxSize *wxBitmapBundleImpl_GetDefaultSize(const wxBitmapBundleImpl * self) {
    return new wxSize(self->GetDefaultSize());
}
wxSize *wxBitmapBundleImpl_GetPreferredBitmapSizeAtScale(const wxBitmapBundleImpl * self, double scale) {
    return new wxSize(self->GetPreferredBitmapSizeAtScale(scale));
}
wxBitmap *wxBitmapBundleImpl_GetBitmap(wxBitmapBundleImpl * self, const wxSize * size) {
    return new wxBitmap(self->GetBitmap(*size));
}

// CLASS: wxBitmapButton
wxClassInfo *wxBitmapButton_CLASSINFO() {
    return wxCLASSINFO(wxBitmapButton);
}
wxBitmapButton *wxBitmapButton_new() {
    return new wxBitmapButton();
}
wxBitmapButton *wxBitmapButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxBitmapButton(parent, id, *bitmap, *pos, *size, style, *validator, *name);
}
bool wxBitmapButton_Create(wxBitmapButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * bitmap, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *bitmap, *pos, *size, style, *validator, *name);
}
#if wxCHECK_VERSION(3, 1, 0)
bool wxBitmapButton_CreateCloseButton(wxBitmapButton * self, wxWindow * parent, wxWindowID winid, const wxString * name) {
    return self->CreateCloseButton(parent, winid, *name);
}
wxBitmapButton * wxBitmapButton_NewCloseButton(wxWindow * parent, wxWindowID winid, const wxString * name) {
    return wxBitmapButton::NewCloseButton(parent, winid, *name);
}
#endif

// CLASS: wxBitmapComboBox
wxClassInfo *wxBitmapComboBox_CLASSINFO() {
    return wxCLASSINFO(wxBitmapComboBox);
}
wxBitmapComboBox *wxBitmapComboBox_new() {
    return new wxBitmapComboBox();
}
wxBitmapComboBox *wxBitmapComboBox_new2(wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return new wxBitmapComboBox(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
int wxBitmapComboBox_Append(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap) {
    return self->Append(*item, *bitmap);
}
int wxBitmapComboBox_Append1(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, void * client_data) {
    return self->Append(*item, *bitmap, client_data);
}
int wxBitmapComboBox_Append2(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, wxClientData * client_data) {
    return self->Append(*item, *bitmap, client_data);
}
bool wxBitmapComboBox_Create1(wxBitmapComboBox * self, wxWindow * parent, wxWindowID id, const wxString * value, const wxPoint * pos, const wxSize * size, const wxArrayString * choices, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *value, *pos, *size, *choices, style, *validator, *name);
}
wxSize *wxBitmapComboBox_GetBitmapSize(const wxBitmapComboBox * self) {
    return new wxSize(self->GetBitmapSize());
}
wxBitmap *wxBitmapComboBox_GetItemBitmap(const wxBitmapComboBox * self, unsigned int n) {
    return new wxBitmap(self->GetItemBitmap(n));
}
int wxBitmapComboBox_Insert(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, unsigned int pos) {
    return self->Insert(*item, *bitmap, pos);
}
int wxBitmapComboBox_Insert1(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, unsigned int pos, void * client_data) {
    return self->Insert(*item, *bitmap, pos, client_data);
}
int wxBitmapComboBox_Insert2(wxBitmapComboBox * self, const wxString * item, const wxBitmap * bitmap, unsigned int pos, wxClientData * client_data) {
    return self->Insert(*item, *bitmap, pos, client_data);
}
void wxBitmapComboBox_SetItemBitmap(wxBitmapComboBox * self, unsigned int n, const wxBitmapBundle * bitmap) {
    return self->SetItemBitmap(n, *bitmap);
}
// Mix-in(s) to wxBitmapComboBox
wxItemContainer *wxBitmapComboBox_AsItemContainer(wxBitmapComboBox* obj) {
    return static_cast<wxItemContainer*>(obj);
}
wxTextEntryBase *wxBitmapComboBox_AsTextEntry(wxBitmapComboBox* obj) {
    return static_cast<wxTextEntryBase*>(obj);
}

// CLASS: wxBitmapDataObject
void wxBitmapDataObject_delete(wxBitmapDataObject *self) {
    delete self;
}
wxBitmapDataObject *wxBitmapDataObject_new(const wxBitmap * bitmap) {
    return new wxBitmapDataObject(*bitmap);
}
wxBitmap *wxBitmapDataObject_GetBitmap(const wxBitmapDataObject * self) {
    return new wxBitmap(self->GetBitmap());
}
void wxBitmapDataObject_SetBitmap(wxBitmapDataObject * self, const wxBitmap * bitmap) {
    return self->SetBitmap(*bitmap);
}

// CLASS: wxBitmapHandler
wxClassInfo *wxBitmapHandler_CLASSINFO() {
    return wxCLASSINFO(wxBitmapHandler);
}
wxBitmapHandler *wxBitmapHandler_new() {
    return new wxBitmapHandler();
}
wxString *wxBitmapHandler_GetExtension(const wxBitmapHandler * self) {
    return new wxString(self->GetExtension());
}
wxString *wxBitmapHandler_GetName(const wxBitmapHandler * self) {
    return new wxString(self->GetName());
}
void wxBitmapHandler_SetExtension(wxBitmapHandler * self, const wxString * extension) {
    return self->SetExtension(*extension);
}
void wxBitmapHandler_SetName(wxBitmapHandler * self, const wxString * name) {
    return self->SetName(*name);
}

// CLASS: wxBitmapToggleButton
wxClassInfo *wxBitmapToggleButton_CLASSINFO() {
    return wxCLASSINFO(wxBitmapToggleButton);
}
wxBitmapToggleButton *wxBitmapToggleButton_new() {
    return new wxBitmapToggleButton();
}
wxBitmapToggleButton *wxBitmapToggleButton_new1(wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name) {
    return new wxBitmapToggleButton(parent, id, *label, *pos, *size, style, *val, *name);
}
bool wxBitmapToggleButton_Create(wxBitmapToggleButton * self, wxWindow * parent, wxWindowID id, const wxBitmapBundle * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * val, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *val, *name);
}

// CLASS: wxBookCtrlBase
wxClassInfo *wxBookCtrlBase_CLASSINFO() {
    return wxCLASSINFO(wxBookCtrlBase);
}
int wxBookCtrlBase_GetPageImage(const wxBookCtrlBase * self, size_t n_page) {
    return self->GetPageImage(n_page);
}
bool wxBookCtrlBase_SetPageImage(wxBookCtrlBase * self, size_t page, int image) {
    return self->SetPageImage(page, image);
}
wxString *wxBookCtrlBase_GetPageText(const wxBookCtrlBase * self, size_t n_page) {
    return new wxString(self->GetPageText(n_page));
}
bool wxBookCtrlBase_SetPageText(wxBookCtrlBase * self, size_t page, const wxString * text) {
    return self->SetPageText(page, *text);
}
int wxBookCtrlBase_GetSelection(const wxBookCtrlBase * self) {
    return self->GetSelection();
}
wxWindow * wxBookCtrlBase_GetCurrentPage(const wxBookCtrlBase * self) {
    return self->GetCurrentPage();
}
int wxBookCtrlBase_SetSelection(wxBookCtrlBase * self, size_t page) {
    return self->SetSelection(page);
}
void wxBookCtrlBase_AdvanceSelection(wxBookCtrlBase * self, bool forward) {
    return self->AdvanceSelection(forward);
}
int wxBookCtrlBase_ChangeSelection(wxBookCtrlBase * self, size_t page) {
    return self->ChangeSelection(page);
}
int wxBookCtrlBase_FindPage(const wxBookCtrlBase * self, const wxWindow * page) {
    return self->FindPage(page);
}
void wxBookCtrlBase_SetPageSize(wxBookCtrlBase * self, const wxSize * size) {
    return self->SetPageSize(*size);
}
int wxBookCtrlBase_HitTest(const wxBookCtrlBase * self, const wxPoint * pt, long * flags) {
    return self->HitTest(*pt, flags);
}
bool wxBookCtrlBase_AddPage(wxBookCtrlBase * self, wxWindow * page, const wxString * text, bool select, int image_id) {
    return self->AddPage(page, *text, select, image_id);
}
bool wxBookCtrlBase_DeleteAllPages(wxBookCtrlBase * self) {
    return self->DeleteAllPages();
}
bool wxBookCtrlBase_DeletePage(wxBookCtrlBase * self, size_t page) {
    return self->DeletePage(page);
}
bool wxBookCtrlBase_InsertPage(wxBookCtrlBase * self, size_t index, wxWindow * page, const wxString * text, bool select, int image_id) {
    return self->InsertPage(index, page, *text, select, image_id);
}
bool wxBookCtrlBase_RemovePage(wxBookCtrlBase * self, size_t page) {
    return self->RemovePage(page);
}
size_t wxBookCtrlBase_GetPageCount(const wxBookCtrlBase * self) {
    return self->GetPageCount();
}
wxWindow * wxBookCtrlBase_GetPage(const wxBookCtrlBase * self, size_t page) {
    return self->GetPage(page);
}
bool wxBookCtrlBase_Create(wxBookCtrlBase * self, wxWindow * parent, wxWindowID winid, const wxPoint * pos, const wxSize * size, long style, const wxString * name) {
    return self->Create(parent, winid, *pos, *size, style, *name);
}

// CLASS: wxBookCtrlEvent
wxClassInfo *wxBookCtrlEvent_CLASSINFO() {
    return wxCLASSINFO(wxBookCtrlEvent);
}
int wxBookCtrlEvent_GetOldSelection(const wxBookCtrlEvent * self) {
    return self->GetOldSelection();
}
int wxBookCtrlEvent_GetSelection(const wxBookCtrlEvent * self) {
    return self->GetSelection();
}
void wxBookCtrlEvent_SetOldSelection(wxBookCtrlEvent * self, int page) {
    return self->SetOldSelection(page);
}
void wxBookCtrlEvent_SetSelection(wxBookCtrlEvent * self, int page) {
    return self->SetSelection(page);
}

// CLASS: wxBoxSizer
wxClassInfo *wxBoxSizer_CLASSINFO() {
    return wxCLASSINFO(wxBoxSizer);
}
wxBoxSizer *wxBoxSizer_new(int orient) {
    return new wxBoxSizer(orient);
}
int wxBoxSizer_GetOrientation(const wxBoxSizer * self) {
    return self->GetOrientation();
}
void wxBoxSizer_SetOrientation(wxBoxSizer * self, int orient) {
    return self->SetOrientation(orient);
}

// CLASS: wxBrush
wxClassInfo *wxBrush_CLASSINFO() {
    return wxCLASSINFO(wxBrush);
}
wxBrush *wxBrush_new() {
    return new wxBrush();
}
wxBrush *wxBrush_new2(const wxBitmap * stipple_bitmap) {
    return new wxBrush(*stipple_bitmap);
}
wxBrush *wxBrush_new3(const wxBrush * brush) {
    return new wxBrush(*brush);
}
wxColour *wxBrush_GetColour(const wxBrush * self) {
    return new wxColour(self->GetColour());
}
wxBitmap * wxBrush_GetStipple(const wxBrush * self) {
    return self->GetStipple();
}
bool wxBrush_IsHatch(const wxBrush * self) {
    return self->IsHatch();
}
bool wxBrush_IsOk(const wxBrush * self) {
    return self->IsOk();
}
bool wxBrush_IsNonTransparent(const wxBrush * self) {
    return self->IsNonTransparent();
}
bool wxBrush_IsTransparent(const wxBrush * self) {
    return self->IsTransparent();
}
void wxBrush_SetColour(wxBrush * self, const wxColour * colour) {
    return self->SetColour(*colour);
}
void wxBrush_SetStipple(wxBrush * self, const wxBitmap * bitmap) {
    return self->SetStipple(*bitmap);
}

// CLASS: wxBrushList
void wxBrushList_delete(wxBrushList *self) {
    delete self;
}

// CLASS: wxBusyCursor
void wxBusyCursor_delete(wxBusyCursor *self) {
    delete self;
}
wxBusyCursor *wxBusyCursor_new(const wxCursor * cursor) {
    return new wxBusyCursor(cursor);
}

// CLASS: wxBusyInfo
void wxBusyInfo_delete(wxBusyInfo *self) {
    delete self;
}
wxBusyInfo *wxBusyInfo_new(const wxBusyInfoFlags * flags) {
    return new wxBusyInfo(*flags);
}
wxBusyInfo *wxBusyInfo_new1(const wxString * msg, wxWindow * parent) {
    return new wxBusyInfo(*msg, parent);
}
void wxBusyInfo_UpdateText(wxBusyInfo * self, const wxString * str) {
    return self->UpdateText(*str);
}
void wxBusyInfo_UpdateLabel(wxBusyInfo * self, const wxString * str) {
    return self->UpdateLabel(*str);
}

// CLASS: wxButton
wxClassInfo *wxButton_CLASSINFO() {
    return wxCLASSINFO(wxButton);
}
wxButton *wxButton_new() {
    return new wxButton();
}
wxButton *wxButton_new1(wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return new wxButton(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxButton_Create(wxButton * self, wxWindow * parent, wxWindowID id, const wxString * label, const wxPoint * pos, const wxSize * size, long style, const wxValidator * validator, const wxString * name) {
    return self->Create(parent, id, *label, *pos, *size, style, *validator, *name);
}
bool wxButton_GetAuthNeeded(const wxButton * self) {
    return self->GetAuthNeeded();
}
void wxButton_SetAuthNeeded(wxButton * self, bool needed) {
    return self->SetAuthNeeded(needed);
}
wxWindow * wxButton_SetDefault(wxButton * self) {
    return self->SetDefault();
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxButton_GetDefaultSize(wxWindow * win) {
    return new wxSize(wxButton::GetDefaultSize(win));
}
#endif

} // extern "C"

