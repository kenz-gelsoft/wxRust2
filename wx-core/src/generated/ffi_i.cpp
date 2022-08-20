#include "generated.h"

extern "C" {

// CLASS: wxIFFHandler
wxClassInfo *wxIFFHandler_CLASSINFO() {
    return wxCLASSINFO(wxIFFHandler);
}
wxIFFHandler *wxIFFHandler_new() {
    return new wxIFFHandler();
}

// CLASS: wxIcon
wxClassInfo *wxIcon_CLASSINFO() {
    return wxCLASSINFO(wxIcon);
}
wxIcon *wxIcon_new() {
    return new wxIcon();
}
wxIcon *wxIcon_new1(const wxIcon * icon) {
    return new wxIcon(*icon);
}
wxIcon *wxIcon_new3(const char *const * bits) {
    return new wxIcon(bits);
}
wxIcon *wxIcon_new5(const wxIconLocation * loc) {
    return new wxIcon(*loc);
}
void wxIcon_CopyFromBitmap(wxIcon * self, const wxBitmap * bmp) {
    return self->CopyFromBitmap(*bmp);
}
int wxIcon_GetDepth(const wxIcon * self) {
    return self->GetDepth();
}
int wxIcon_GetHeight(const wxIcon * self) {
    return self->GetHeight();
}
#if wxCHECK_VERSION(3, 1, 0)
double wxIcon_GetLogicalHeight(const wxIcon * self) {
    return self->GetLogicalHeight();
}
wxSize *wxIcon_GetLogicalSize(const wxIcon * self) {
    return new wxSize(self->GetLogicalSize());
}
double wxIcon_GetLogicalWidth(const wxIcon * self) {
    return self->GetLogicalWidth();
}
#endif
double wxIcon_GetScaleFactor(const wxIcon * self) {
    return self->GetScaleFactor();
}
wxSize *wxIcon_GetSize(const wxIcon * self) {
    return new wxSize(self->GetSize());
}
int wxIcon_GetWidth(const wxIcon * self) {
    return self->GetWidth();
}
bool wxIcon_IsOk(const wxIcon * self) {
    return self->IsOk();
}
#if wxCHECK_VERSION(3, 1, 7)
void wxIcon_SetDepth(wxIcon * self, int depth) {
    return self->SetDepth(depth);
}
void wxIcon_SetHeight(wxIcon * self, int height) {
    return self->SetHeight(height);
}
void wxIcon_SetWidth(wxIcon * self, int width) {
    return self->SetWidth(width);
}
#endif

// CLASS: wxIconBundle
wxClassInfo *wxIconBundle_CLASSINFO() {
    return wxCLASSINFO(wxIconBundle);
}
wxIconBundle *wxIconBundle_new() {
    return new wxIconBundle();
}
wxIconBundle *wxIconBundle_new3(const wxIcon * icon) {
    return new wxIconBundle(*icon);
}
wxIconBundle *wxIconBundle_new5(const wxIconBundle * ic) {
    return new wxIconBundle(*ic);
}
void wxIconBundle_AddIcon3(wxIconBundle * self, const wxIcon * icon) {
    return self->AddIcon(*icon);
}
wxIcon *wxIconBundle_GetIcon(const wxIconBundle * self, const wxSize * size, int flags) {
    return new wxIcon(self->GetIcon(*size, flags));
}
wxIcon *wxIconBundle_GetIcon1(const wxIconBundle * self, wxCoord size, int flags) {
    return new wxIcon(self->GetIcon(size, flags));
}
wxIcon *wxIconBundle_GetIconOfExactSize(const wxIconBundle * self, const wxSize * size) {
    return new wxIcon(self->GetIconOfExactSize(*size));
}
size_t wxIconBundle_GetIconCount(const wxIconBundle * self) {
    return self->GetIconCount();
}
wxIcon *wxIconBundle_GetIconByIndex(const wxIconBundle * self, size_t n) {
    return new wxIcon(self->GetIconByIndex(n));
}
bool wxIconBundle_IsEmpty(const wxIconBundle * self) {
    return self->IsEmpty();
}

// CLASS: wxIconizeEvent
wxClassInfo *wxIconizeEvent_CLASSINFO() {
    return wxCLASSINFO(wxIconizeEvent);
}
wxIconizeEvent *wxIconizeEvent_new(int id, bool iconized) {
    return new wxIconizeEvent(id, iconized);
}
bool wxIconizeEvent_IsIconized(const wxIconizeEvent * self) {
    return self->IsIconized();
}

// CLASS: wxIdManager
void wxIdManager_delete(wxIdManager *self) {
    delete self;
}
wxWindowID wxIdManager_ReserveId(int count) {
    return wxIdManager::ReserveId(count);
}
void wxIdManager_UnreserveId(wxWindowID id, int count) {
    return wxIdManager::UnreserveId(id, count);
}

// CLASS: wxImage
wxClassInfo *wxImage_CLASSINFO() {
    return wxCLASSINFO(wxImage);
}
wxImage *wxImage_Copy(const wxImage * self) {
    return new wxImage(self->Copy());
}
bool wxImage_Create(wxImage * self, int width, int height, bool clear) {
    return self->Create(width, height, clear);
}
bool wxImage_Create1(wxImage * self, const wxSize * sz, bool clear) {
    return self->Create(*sz, clear);
}
bool wxImage_Create2(wxImage * self, int width, int height, unsigned char * data, bool static_data) {
    return self->Create(width, height, data, static_data);
}
bool wxImage_Create3(wxImage * self, const wxSize * sz, unsigned char * data, bool static_data) {
    return self->Create(*sz, data, static_data);
}
bool wxImage_Create4(wxImage * self, int width, int height, unsigned char * data, unsigned char * alpha, bool static_data) {
    return self->Create(width, height, data, alpha, static_data);
}
bool wxImage_Create5(wxImage * self, const wxSize * sz, unsigned char * data, unsigned char * alpha, bool static_data) {
    return self->Create(*sz, data, alpha, static_data);
}
void wxImage_Destroy(wxImage * self) {
    return self->Destroy();
}
void wxImage_InitAlpha(wxImage * self) {
    return self->InitAlpha();
}
wxImage *wxImage_Blur(const wxImage * self, int blur_radius) {
    return new wxImage(self->Blur(blur_radius));
}
wxImage *wxImage_BlurHorizontal(const wxImage * self, int blur_radius) {
    return new wxImage(self->BlurHorizontal(blur_radius));
}
wxImage *wxImage_BlurVertical(const wxImage * self, int blur_radius) {
    return new wxImage(self->BlurVertical(blur_radius));
}
wxImage *wxImage_Mirror(const wxImage * self, bool horizontally) {
    return new wxImage(self->Mirror(horizontally));
}
wxImage * wxImage_Resize(wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue) {
    return &(self->Resize(*size, *pos, red, green, blue));
}
wxImage *wxImage_Rotate(const wxImage * self, double angle, const wxPoint * rotation_centre, bool interpolating, wxPoint * offset_after_rotation) {
    return new wxImage(self->Rotate(angle, *rotation_centre, interpolating, offset_after_rotation));
}
wxImage *wxImage_Rotate90(const wxImage * self, bool clockwise) {
    return new wxImage(self->Rotate90(clockwise));
}
wxImage *wxImage_Rotate180(const wxImage * self) {
    return new wxImage(self->Rotate180());
}
void wxImage_RotateHue(wxImage * self, double angle) {
    return self->RotateHue(angle);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxImage_ChangeSaturation(wxImage * self, double factor) {
    return self->ChangeSaturation(factor);
}
void wxImage_ChangeBrightness(wxImage * self, double factor) {
    return self->ChangeBrightness(factor);
}
void wxImage_ChangeHSV(wxImage * self, double angle_h, double factor_s, double factor_v) {
    return self->ChangeHSV(angle_h, factor_s, factor_v);
}
#endif
wxImage *wxImage_Size(const wxImage * self, const wxSize * size, const wxPoint * pos, int red, int green, int blue) {
    return new wxImage(self->Size(*size, *pos, red, green, blue));
}
wxImage *wxImage_ConvertToGreyscale(const wxImage * self, double weight_r, double weight_g, double weight_b) {
    return new wxImage(self->ConvertToGreyscale(weight_r, weight_g, weight_b));
}
wxImage *wxImage_ConvertToGreyscale1(const wxImage * self) {
    return new wxImage(self->ConvertToGreyscale());
}
#if wxCHECK_VERSION(3, 1, 0)
wxImage *wxImage_ChangeLightness(const wxImage * self, int alpha) {
    return new wxImage(self->ChangeLightness(alpha));
}
#endif
unsigned char * wxImage_GetAlpha(const wxImage * self) {
    return self->GetAlpha();
}
unsigned char * wxImage_GetData(const wxImage * self) {
    return self->GetData();
}
int wxImage_GetWidth(const wxImage * self) {
    return self->GetWidth();
}
int wxImage_GetHeight(const wxImage * self) {
    return self->GetHeight();
}
wxSize *wxImage_GetSize(const wxImage * self) {
    return new wxSize(self->GetSize());
}
wxString *wxImage_GetOption(const wxImage * self, const wxString * name) {
    return new wxString(self->GetOption(*name));
}
int wxImage_GetOptionInt(const wxImage * self, const wxString * name) {
    return self->GetOptionInt(*name);
}
bool wxImage_GetOrFindMaskColour(const wxImage * self, unsigned char * r, unsigned char * g, unsigned char * b) {
    return self->GetOrFindMaskColour(r, g, b);
}
wxImage *wxImage_GetSubImage(const wxImage * self, const wxRect * rect) {
    return new wxImage(self->GetSubImage(*rect));
}
bool wxImage_HasAlpha(const wxImage * self) {
    return self->HasAlpha();
}
bool wxImage_HasMask(const wxImage * self) {
    return self->HasMask();
}
bool wxImage_HasOption(const wxImage * self, const wxString * name) {
    return self->HasOption(*name);
}
bool wxImage_IsOk(const wxImage * self) {
    return self->IsOk();
}
bool wxImage_LoadFile2(wxImage * self, const wxString * name, const wxString * mimetype, int index) {
    return self->LoadFile(*name, *mimetype, index);
}
bool wxImage_LoadFile3(wxImage * self, wxInputStream * stream, const wxString * mimetype, int index) {
    return self->LoadFile(*stream, *mimetype, index);
}
bool wxImage_SaveFile(const wxImage * self, wxOutputStream * stream, const wxString * mimetype) {
    return self->SaveFile(*stream, *mimetype);
}
bool wxImage_SaveFile2(const wxImage * self, const wxString * name, const wxString * mimetype) {
    return self->SaveFile(*name, *mimetype);
}
bool wxImage_SaveFile3(const wxImage * self, const wxString * name) {
    return self->SaveFile(*name);
}
void wxImage_SetAlpha(wxImage * self, unsigned char * alpha, bool static_data) {
    return self->SetAlpha(alpha, static_data);
}
void wxImage_ClearAlpha(wxImage * self) {
    return self->ClearAlpha();
}
void wxImage_SetData(wxImage * self, unsigned char * data, bool static_data) {
    return self->SetData(data, static_data);
}
void wxImage_SetData1(wxImage * self, unsigned char * data, int new_width, int new_height, bool static_data) {
    return self->SetData(data, new_width, new_height, static_data);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxImage_SetLoadFlags(wxImage * self, int flags) {
    return self->SetLoadFlags(flags);
}
#endif
void wxImage_SetMask(wxImage * self, bool has_mask) {
    return self->SetMask(has_mask);
}
void wxImage_SetOption(wxImage * self, const wxString * name, const wxString * value) {
    return self->SetOption(*name, *value);
}
void wxImage_SetOption1(wxImage * self, const wxString * name, int value) {
    return self->SetOption(*name, value);
}
void wxImage_SetPalette(wxImage * self, const wxPalette * palette) {
    return self->SetPalette(*palette);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxImage_SetDefaultLoadFlags(int flags) {
    return wxImage::SetDefaultLoadFlags(flags);
}
int wxImage_GetLoadFlags(const wxImage * self) {
    return self->GetLoadFlags();
}
#endif
void wxImage_AddHandler(wxImageHandler * handler) {
    return wxImage::AddHandler(handler);
}
void wxImage_CleanUpHandlers() {
    return wxImage::CleanUpHandlers();
}
wxImageHandler * wxImage_FindHandler(const wxString * name) {
    return wxImage::FindHandler(*name);
}
wxImageHandler * wxImage_FindHandlerMime(const wxString * mimetype) {
    return wxImage::FindHandlerMime(*mimetype);
}
void wxImage_InitStandardHandlers() {
    return wxImage::InitStandardHandlers();
}
void wxImage_InsertHandler(wxImageHandler * handler) {
    return wxImage::InsertHandler(handler);
}
bool wxImage_RemoveHandler(const wxString * name) {
    return wxImage::RemoveHandler(*name);
}
bool wxImage_CanRead(const wxString * filename) {
    return wxImage::CanRead(*filename);
}
bool wxImage_CanRead1(wxInputStream * stream) {
    return wxImage::CanRead(*stream);
}
#if wxCHECK_VERSION(3, 1, 0)
int wxImage_GetDefaultLoadFlags() {
    return wxImage::GetDefaultLoadFlags();
}
#endif
wxString *wxImage_GetImageExtWildcard() {
    return new wxString(wxImage::GetImageExtWildcard());
}
wxImage *wxImage_new() {
    return new wxImage();
}
wxImage *wxImage_new1(int width, int height, bool clear) {
    return new wxImage(width, height, clear);
}
wxImage *wxImage_new2(const wxSize * sz, bool clear) {
    return new wxImage(*sz, clear);
}
wxImage *wxImage_new3(int width, int height, unsigned char * data, bool static_data) {
    return new wxImage(width, height, data, static_data);
}
wxImage *wxImage_new4(const wxSize * sz, unsigned char * data, bool static_data) {
    return new wxImage(*sz, data, static_data);
}
wxImage *wxImage_new5(int width, int height, unsigned char * data, unsigned char * alpha, bool static_data) {
    return new wxImage(width, height, data, alpha, static_data);
}
wxImage *wxImage_new6(const wxSize * sz, unsigned char * data, unsigned char * alpha, bool static_data) {
    return new wxImage(*sz, data, alpha, static_data);
}
wxImage *wxImage_new7(const char *const * xpm_data) {
    return new wxImage(xpm_data);
}
wxImage *wxImage_new9(const wxString * name, const wxString * mimetype, int index) {
    return new wxImage(*name, *mimetype, index);
}
wxImage *wxImage_new11(wxInputStream * stream, const wxString * mimetype, int index) {
    return new wxImage(*stream, *mimetype, index);
}

// CLASS: wxImageHandler
wxClassInfo *wxImageHandler_CLASSINFO() {
    return wxCLASSINFO(wxImageHandler);
}
bool wxImageHandler_CanRead(wxImageHandler * self, wxInputStream * stream) {
    return self->CanRead(*stream);
}
bool wxImageHandler_CanRead1(wxImageHandler * self, const wxString * filename) {
    return self->CanRead(*filename);
}
wxString *wxImageHandler_GetExtension(const wxImageHandler * self) {
    return new wxString(self->GetExtension());
}
wxArrayString *wxImageHandler_GetAltExtensions(const wxImageHandler * self) {
    return new wxArrayString(self->GetAltExtensions());
}
int wxImageHandler_GetImageCount(wxImageHandler * self, wxInputStream * stream) {
    return self->GetImageCount(*stream);
}
wxString *wxImageHandler_GetMimeType(const wxImageHandler * self) {
    return new wxString(self->GetMimeType());
}
wxString *wxImageHandler_GetName(const wxImageHandler * self) {
    return new wxString(self->GetName());
}
bool wxImageHandler_LoadFile(wxImageHandler * self, wxImage * image, wxInputStream * stream, bool verbose, int index) {
    return self->LoadFile(image, *stream, verbose, index);
}
bool wxImageHandler_SaveFile(wxImageHandler * self, wxImage * image, wxOutputStream * stream, bool verbose) {
    return self->SaveFile(image, *stream, verbose);
}
void wxImageHandler_SetExtension(wxImageHandler * self, const wxString * extension) {
    return self->SetExtension(*extension);
}
void wxImageHandler_SetAltExtensions(wxImageHandler * self, const wxArrayString * extensions) {
    return self->SetAltExtensions(*extensions);
}
void wxImageHandler_SetMimeType(wxImageHandler * self, const wxString * mimetype) {
    return self->SetMimeType(*mimetype);
}
void wxImageHandler_SetName(wxImageHandler * self, const wxString * name) {
    return self->SetName(*name);
}

// CLASS: wxImageList
wxClassInfo *wxImageList_CLASSINFO() {
    return wxCLASSINFO(wxImageList);
}
wxImageList *wxImageList_new() {
    return new wxImageList();
}
wxImageList *wxImageList_new1(int width, int height, bool mask, int initial_count) {
    return new wxImageList(width, height, mask, initial_count);
}
int wxImageList_Add(wxImageList * self, const wxBitmap * bitmap, const wxBitmap * mask) {
    return self->Add(*bitmap, *mask);
}
int wxImageList_Add1(wxImageList * self, const wxBitmap * bitmap, const wxColour * mask_colour) {
    return self->Add(*bitmap, *mask_colour);
}
int wxImageList_Add2(wxImageList * self, const wxIcon * icon) {
    return self->Add(*icon);
}
bool wxImageList_Create(wxImageList * self, int width, int height, bool mask, int initial_count) {
    return self->Create(width, height, mask, initial_count);
}
#if wxCHECK_VERSION(3, 1, 0)
void wxImageList_Destroy(wxImageList * self) {
    return self->Destroy();
}
#endif
bool wxImageList_Draw(wxImageList * self, int index, wxDC * dc, int x, int y, int flags, bool solid_background) {
    return self->Draw(index, *dc, x, y, flags, solid_background);
}
wxBitmap *wxImageList_GetBitmap(const wxImageList * self, int index) {
    return new wxBitmap(self->GetBitmap(index));
}
wxIcon *wxImageList_GetIcon(const wxImageList * self, int index) {
    return new wxIcon(self->GetIcon(index));
}
int wxImageList_GetImageCount(const wxImageList * self) {
    return self->GetImageCount();
}
bool wxImageList_GetSize(const wxImageList * self, int index, int * width, int * height) {
    return self->GetSize(index, *width, *height);
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxImageList_GetSize1(const wxImageList * self) {
    return new wxSize(self->GetSize());
}
#endif
bool wxImageList_Remove(wxImageList * self, int index) {
    return self->Remove(index);
}
bool wxImageList_RemoveAll(wxImageList * self) {
    return self->RemoveAll();
}
bool wxImageList_Replace(wxImageList * self, int index, const wxBitmap * bitmap, const wxBitmap * mask) {
    return self->Replace(index, *bitmap, *mask);
}
bool wxImageList_Replace1(wxImageList * self, int index, const wxIcon * icon) {
    return self->Replace(index, *icon);
}

// CLASS: wxInfoBar
wxClassInfo *wxInfoBar_CLASSINFO() {
    return wxCLASSINFO(wxInfoBar);
}
void wxInfoBar_SetEffectDuration(wxInfoBar * self, int duration) {
    return self->SetEffectDuration(duration);
}
int wxInfoBar_GetEffectDuration(const wxInfoBar * self) {
    return self->GetEffectDuration();
}
wxInfoBar *wxInfoBar_new() {
    return new wxInfoBar();
}
wxInfoBar *wxInfoBar_new1(wxWindow * parent, wxWindowID winid) {
    return new wxInfoBar(parent, winid);
}
bool wxInfoBar_Create(wxInfoBar * self, wxWindow * parent, wxWindowID winid) {
    return self->Create(parent, winid);
}
void wxInfoBar_AddButton(wxInfoBar * self, wxWindowID btnid, const wxString * label) {
    return self->AddButton(btnid, *label);
}
void wxInfoBar_Dismiss(wxInfoBar * self) {
    return self->Dismiss();
}
void wxInfoBar_RemoveButton(wxInfoBar * self, wxWindowID btnid) {
    return self->RemoveButton(btnid);
}
void wxInfoBar_ShowMessage(wxInfoBar * self, const wxString * msg, int flags) {
    return self->ShowMessage(*msg, flags);
}
#if wxCHECK_VERSION(3, 1, 0)
size_t wxInfoBar_GetButtonCount(const wxInfoBar * self) {
    return self->GetButtonCount();
}
wxWindowID wxInfoBar_GetButtonId(const wxInfoBar * self, size_t idx) {
    return self->GetButtonId(idx);
}
bool wxInfoBar_HasButtonId(const wxInfoBar * self, wxWindowID btnid) {
    return self->HasButtonId(btnid);
}
#endif

// CLASS: wxInitDialogEvent
wxClassInfo *wxInitDialogEvent_CLASSINFO() {
    return wxCLASSINFO(wxInitDialogEvent);
}
wxInitDialogEvent *wxInitDialogEvent_new(int id) {
    return new wxInitDialogEvent(id);
}

// CLASS: wxItemContainer
void wxItemContainer_delete(wxItemContainer *self) {
    delete self;
}
int wxItemContainer_Append(wxItemContainer * self, const wxString * item) {
    return self->Append(*item);
}
int wxItemContainer_Append1(wxItemContainer * self, const wxString * item, void * client_data) {
    return self->Append(*item, client_data);
}
int wxItemContainer_Append2(wxItemContainer * self, const wxString * item, wxClientData * client_data) {
    return self->Append(*item, client_data);
}
int wxItemContainer_Append3(wxItemContainer * self, const wxArrayString * items) {
    return self->Append(*items);
}
int wxItemContainer_Append5(wxItemContainer * self, const wxArrayString * items, void ** client_data) {
    return self->Append(*items, client_data);
}
int wxItemContainer_Append6(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data) {
    return self->Append(*items, client_data);
}
int wxItemContainer_Append7(wxItemContainer * self, unsigned int n, const wxString * items) {
    return self->Append(n, items);
}
int wxItemContainer_Append8(wxItemContainer * self, unsigned int n, const wxString * items, void ** client_data) {
    return self->Append(n, items, client_data);
}
int wxItemContainer_Append9(wxItemContainer * self, unsigned int n, const wxString * items, wxClientData ** client_data) {
    return self->Append(n, items, client_data);
}
void wxItemContainer_Clear(wxItemContainer * self) {
    return self->Clear();
}
void wxItemContainer_Delete(wxItemContainer * self, unsigned int n) {
    return self->Delete(n);
}
wxClientData * wxItemContainer_DetachClientObject(wxItemContainer * self, unsigned int n) {
    return self->DetachClientObject(n);
}
bool wxItemContainer_HasClientData(const wxItemContainer * self) {
    return self->HasClientData();
}
bool wxItemContainer_HasClientObjectData(const wxItemContainer * self) {
    return self->HasClientObjectData();
}
bool wxItemContainer_HasClientUntypedData(const wxItemContainer * self) {
    return self->HasClientUntypedData();
}
void * wxItemContainer_GetClientData(const wxItemContainer * self, unsigned int n) {
    return self->GetClientData(n);
}
wxClientData * wxItemContainer_GetClientObject(const wxItemContainer * self, unsigned int n) {
    return self->GetClientObject(n);
}
void wxItemContainer_SetClientData(wxItemContainer * self, unsigned int n, void * data) {
    return self->SetClientData(n, data);
}
void wxItemContainer_SetClientObject(wxItemContainer * self, unsigned int n, wxClientData * data) {
    return self->SetClientObject(n, data);
}
int wxItemContainer_Insert(wxItemContainer * self, const wxString * item, unsigned int pos) {
    return self->Insert(*item, pos);
}
int wxItemContainer_Insert1(wxItemContainer * self, const wxString * item, unsigned int pos, void * client_data) {
    return self->Insert(*item, pos, client_data);
}
int wxItemContainer_Insert2(wxItemContainer * self, const wxString * item, unsigned int pos, wxClientData * client_data) {
    return self->Insert(*item, pos, client_data);
}
int wxItemContainer_Insert3(wxItemContainer * self, const wxArrayString * items, unsigned int pos) {
    return self->Insert(*items, pos);
}
int wxItemContainer_Insert5(wxItemContainer * self, const wxArrayString * items, unsigned int pos, void ** client_data) {
    return self->Insert(*items, pos, client_data);
}
int wxItemContainer_Insert6(wxItemContainer * self, const wxArrayString * items, unsigned int pos, wxClientData ** client_data) {
    return self->Insert(*items, pos, client_data);
}
int wxItemContainer_Insert7(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos) {
    return self->Insert(n, items, pos);
}
int wxItemContainer_Insert8(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos, void ** client_data) {
    return self->Insert(n, items, pos, client_data);
}
int wxItemContainer_Insert9(wxItemContainer * self, unsigned int n, const wxString * items, unsigned int pos, wxClientData ** client_data) {
    return self->Insert(n, items, pos, client_data);
}
void wxItemContainer_Set(wxItemContainer * self, const wxArrayString * items) {
    return self->Set(*items);
}
void wxItemContainer_Set2(wxItemContainer * self, const wxArrayString * items, void ** client_data) {
    return self->Set(*items, client_data);
}
void wxItemContainer_Set3(wxItemContainer * self, const wxArrayString * items, wxClientData ** client_data) {
    return self->Set(*items, client_data);
}
void wxItemContainer_Set4(wxItemContainer * self, unsigned int n, const wxString * items) {
    return self->Set(n, items);
}
void wxItemContainer_Set5(wxItemContainer * self, unsigned int n, const wxString * items, void ** client_data) {
    return self->Set(n, items, client_data);
}
void wxItemContainer_Set6(wxItemContainer * self, unsigned int n, const wxString * items, wxClientData ** client_data) {
    return self->Set(n, items, client_data);
}

// CLASS: wxItemContainerImmutable
void wxItemContainerImmutable_delete(wxItemContainerImmutable *self) {
    delete self;
}
void wxItemContainerImmutable_SetSelection(wxItemContainerImmutable * self, int n) {
    return self->SetSelection(n);
}
int wxItemContainerImmutable_GetSelection(const wxItemContainerImmutable * self) {
    return self->GetSelection();
}
bool wxItemContainerImmutable_SetStringSelection(wxItemContainerImmutable * self, const wxString * string) {
    return self->SetStringSelection(*string);
}
wxString *wxItemContainerImmutable_GetStringSelection(const wxItemContainerImmutable * self) {
    return new wxString(self->GetStringSelection());
}
void wxItemContainerImmutable_Select(wxItemContainerImmutable * self, int n) {
    return self->Select(n);
}
unsigned int wxItemContainerImmutable_GetCount(const wxItemContainerImmutable * self) {
    return self->GetCount();
}
bool wxItemContainerImmutable_IsEmpty(const wxItemContainerImmutable * self) {
    return self->IsEmpty();
}
wxString *wxItemContainerImmutable_GetString(const wxItemContainerImmutable * self, unsigned int n) {
    return new wxString(self->GetString(n));
}
wxArrayString *wxItemContainerImmutable_GetStrings(const wxItemContainerImmutable * self) {
    return new wxArrayString(self->GetStrings());
}
void wxItemContainerImmutable_SetString(wxItemContainerImmutable * self, unsigned int n, const wxString * string) {
    return self->SetString(n, *string);
}
int wxItemContainerImmutable_FindString(const wxItemContainerImmutable * self, const wxString * string, bool case_sensitive) {
    return self->FindString(*string, case_sensitive);
}

} // extern "C"

