#pragma once

#include <wx/anybutton.h>
#include <wx/artprov.h>

extern "C" {

// CLASS: wxAnyButton
wxClassInfo *wxAnyButton_CLASSINFO();
wxAnyButton *wxAnyButton_new();
wxBitmap *wxAnyButton_GetBitmap(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapCurrent(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapDisabled(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapFocus(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapLabel(const wxAnyButton * self);
wxBitmap *wxAnyButton_GetBitmapPressed(const wxAnyButton * self);
void wxAnyButton_SetBitmap(wxAnyButton * self, const wxBitmapBundle * bitmap, wxDirection dir);
void wxAnyButton_SetBitmapCurrent(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapDisabled(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapFocus(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapLabel(wxAnyButton * self, const wxBitmapBundle * bitmap);
void wxAnyButton_SetBitmapPressed(wxAnyButton * self, const wxBitmapBundle * bitmap);
wxSize *wxAnyButton_GetBitmapMargins(wxAnyButton * self);
void wxAnyButton_SetBitmapMargins(wxAnyButton * self, wxCoord x, wxCoord y);
void wxAnyButton_SetBitmapMargins1(wxAnyButton * self, const wxSize * sz);
void wxAnyButton_SetBitmapPosition(wxAnyButton * self, wxDirection dir);

// CLASS: wxArtProvider
wxClassInfo *wxArtProvider_CLASSINFO();
bool wxArtProvider_Delete(wxArtProvider * provider);
wxBitmap *wxArtProvider_GetBitmap(const wxArtID * id, const wxArtClient * client, const wxSize * size);
#if wxCHECK_VERSION(3, 1, 0)
wxBitmapBundle *wxArtProvider_GetBitmapBundle(const wxArtID * id, const wxArtClient * client, const wxSize * size);
#endif
wxIcon *wxArtProvider_GetIcon(const wxArtID * id, const wxArtClient * client, const wxSize * size);
#if wxCHECK_VERSION(3, 1, 0)
wxSize *wxArtProvider_GetNativeDIPSizeHint(const wxArtClient * client);
wxSize *wxArtProvider_GetNativeSizeHint(const wxArtClient * client, wxWindow * win);
wxSize *wxArtProvider_GetDIPSizeHint(const wxArtClient * client);
#endif
#if wxCHECK_VERSION(3, 1, 7)
wxSize *wxArtProvider_GetSizeHint(const wxArtClient * client, wxWindow * win);
#endif
bool wxArtProvider_HasNativeProvider();
bool wxArtProvider_Pop();
void wxArtProvider_Push(wxArtProvider * provider);
void wxArtProvider_PushBack(wxArtProvider * provider);
bool wxArtProvider_Remove(wxArtProvider * provider);
wxArtID *wxArtProvider_GetMessageBoxIconId(int flags);
wxIcon *wxArtProvider_GetMessageBoxIcon(int flags);

} // extern "C"

