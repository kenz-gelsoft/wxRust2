#include "generated.h"

extern "C" {

// CLASS: wxAnyButton
wxClassInfo *wxAnyButton_CLASSINFO() {
    return wxCLASSINFO(wxAnyButton);
}
wxAnyButton *wxAnyButton_new() {
    return new wxAnyButton();
}
wxBitmap *wxAnyButton_GetBitmap(const wxAnyButton * self) {
    return new wxBitmap(self->GetBitmap());
}
wxBitmap *wxAnyButton_GetBitmapCurrent(const wxAnyButton * self) {
    return new wxBitmap(self->GetBitmapCurrent());
}
wxBitmap *wxAnyButton_GetBitmapDisabled(const wxAnyButton * self) {
    return new wxBitmap(self->GetBitmapDisabled());
}
wxBitmap *wxAnyButton_GetBitmapFocus(const wxAnyButton * self) {
    return new wxBitmap(self->GetBitmapFocus());
}
wxBitmap *wxAnyButton_GetBitmapLabel(const wxAnyButton * self) {
    return new wxBitmap(self->GetBitmapLabel());
}
wxBitmap *wxAnyButton_GetBitmapPressed(const wxAnyButton * self) {
    return new wxBitmap(self->GetBitmapPressed());
}
void wxAnyButton_SetBitmap(wxAnyButton * self, const wxBitmapBundle * bitmap, wxDirection dir) {
    return self->SetBitmap(*bitmap, dir);
}
void wxAnyButton_SetBitmapCurrent(wxAnyButton * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmapCurrent(*bitmap);
}
void wxAnyButton_SetBitmapDisabled(wxAnyButton * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmapDisabled(*bitmap);
}
void wxAnyButton_SetBitmapFocus(wxAnyButton * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmapFocus(*bitmap);
}
void wxAnyButton_SetBitmapLabel(wxAnyButton * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmapLabel(*bitmap);
}
void wxAnyButton_SetBitmapPressed(wxAnyButton * self, const wxBitmapBundle * bitmap) {
    return self->SetBitmapPressed(*bitmap);
}
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton * self) {
    return new wxSize(self->GetBitmapMargins());
}
void wxAnyButton_SetBitmapMargins(wxAnyButton * self, wxCoord x, wxCoord y) {
    return self->SetBitmapMargins(x, y);
}
void wxAnyButton_SetBitmapMargins1(wxAnyButton * self, const wxSize * sz) {
    return self->SetBitmapMargins(*sz);
}
void wxAnyButton_SetBitmapPosition(wxAnyButton * self, wxDirection dir) {
    return self->SetBitmapPosition(dir);
}

// CLASS: wxArtProvider
wxClassInfo *wxArtProvider_CLASSINFO() {
    return wxCLASSINFO(wxArtProvider);
}
bool wxArtProvider_Delete(wxArtProvider * provider) {
    return wxArtProvider::Delete(provider);
}
wxBitmap *wxArtProvider_GetBitmap(const wxArtID * id, const wxArtClient * client, const wxSize * size) {
    return new wxBitmap(wxArtProvider::GetBitmap(*id, *client, *size));
}
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxArtProvider_GetBitmapBundle(const wxArtID * id, const wxArtClient * client, const wxSize * size) {
    return new wxBitmapBundle(wxArtProvider::GetBitmapBundle(*id, *client, *size));
}
#endif
wxIcon *wxArtProvider_GetIcon(const wxArtID * id, const wxArtClient * client, const wxSize * size) {
    return new wxIcon(wxArtProvider::GetIcon(*id, *client, *size));
}
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxArtProvider_GetNativeDIPSizeHint(const wxArtClient * client) {
    return new wxSize(wxArtProvider::GetNativeDIPSizeHint(*client));
}
wxSize *wxArtProvider_GetNativeSizeHint(const wxArtClient * client, wxWindow * win) {
    return new wxSize(wxArtProvider::GetNativeSizeHint(*client, win));
}
wxSize *wxArtProvider_GetDIPSizeHint(const wxArtClient * client) {
    return new wxSize(wxArtProvider::GetDIPSizeHint(*client));
}
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxSize *wxArtProvider_GetSizeHint(const wxArtClient * client, wxWindow * win) {
    return new wxSize(wxArtProvider::GetSizeHint(*client, win));
}
#endif
bool wxArtProvider_HasNativeProvider() {
    return wxArtProvider::HasNativeProvider();
}
bool wxArtProvider_Pop() {
    return wxArtProvider::Pop();
}
void wxArtProvider_Push(wxArtProvider * provider) {
    return wxArtProvider::Push(provider);
}
void wxArtProvider_PushBack(wxArtProvider * provider) {
    return wxArtProvider::PushBack(provider);
}
bool wxArtProvider_Remove(wxArtProvider * provider) {
    return wxArtProvider::Remove(provider);
}
wxArtID *wxArtProvider_GetMessageBoxIconId(int flags) {
    return new wxArtID(wxArtProvider::GetMessageBoxIconId(flags));
}
wxIcon *wxArtProvider_GetMessageBoxIcon(int flags) {
    return new wxIcon(wxArtProvider::GetMessageBoxIcon(flags));
}

} // extern "C"

