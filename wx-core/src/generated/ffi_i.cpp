#include "generated.h"

extern "C" {

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

