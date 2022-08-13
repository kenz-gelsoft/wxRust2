#pragma once

#include <wx/aboutdlg.h>
#include <wx/accel.h>
#include <wx/affinematrix2d.h>
#include <wx/affinematrix2dbase.h>
#include <wx/animate.h>
#include <wx/anybutton.h>
#include <wx/appprogress.h>
#include <wx/artprov.h>
#include <wx/event.h>

extern "C" {

// CLASS: wxAboutDialogInfo
void wxAboutDialogInfo_delete(wxAboutDialogInfo *self);
wxAboutDialogInfo *wxAboutDialogInfo_new();
void wxAboutDialogInfo_AddArtist(wxAboutDialogInfo * self, const wxString * artist);
void wxAboutDialogInfo_AddDeveloper(wxAboutDialogInfo * self, const wxString * developer);
void wxAboutDialogInfo_AddDocWriter(wxAboutDialogInfo * self, const wxString * docwriter);
void wxAboutDialogInfo_AddTranslator(wxAboutDialogInfo * self, const wxString * translator);
wxString *wxAboutDialogInfo_GetName(const wxAboutDialogInfo * self);
bool wxAboutDialogInfo_HasDescription(const wxAboutDialogInfo * self);
wxString *wxAboutDialogInfo_GetDescription(wxAboutDialogInfo * self);
bool wxAboutDialogInfo_HasCopyright(const wxAboutDialogInfo * self);
wxString *wxAboutDialogInfo_GetCopyright(const wxAboutDialogInfo * self);
void wxAboutDialogInfo_SetArtists(wxAboutDialogInfo * self, const wxArrayString * artists);
void wxAboutDialogInfo_SetCopyright(wxAboutDialogInfo * self, const wxString * copyright);
void wxAboutDialogInfo_SetDescription(wxAboutDialogInfo * self, const wxString * desc);
void wxAboutDialogInfo_SetDevelopers(wxAboutDialogInfo * self, const wxArrayString * developers);
void wxAboutDialogInfo_SetDocWriters(wxAboutDialogInfo * self, const wxArrayString * docwriters);
bool wxAboutDialogInfo_HasIcon(const wxAboutDialogInfo * self);
wxIcon *wxAboutDialogInfo_GetIcon(const wxAboutDialogInfo * self);
void wxAboutDialogInfo_SetIcon(wxAboutDialogInfo * self, const wxIcon * icon);
bool wxAboutDialogInfo_HasLicence(const wxAboutDialogInfo * self);
wxString *wxAboutDialogInfo_GetLicence(const wxAboutDialogInfo * self);
void wxAboutDialogInfo_SetLicence(wxAboutDialogInfo * self, const wxString * licence);
void wxAboutDialogInfo_SetLicense(wxAboutDialogInfo * self, const wxString * licence);
void wxAboutDialogInfo_SetName(wxAboutDialogInfo * self, const wxString * name);
void wxAboutDialogInfo_SetTranslators(wxAboutDialogInfo * self, const wxArrayString * translators);
void wxAboutDialogInfo_SetVersion(wxAboutDialogInfo * self, const wxString * version, const wxString * long_version);
wxString *wxAboutDialogInfo_GetVersion(const wxAboutDialogInfo * self);
wxString *wxAboutDialogInfo_GetLongVersion(const wxAboutDialogInfo * self);
bool wxAboutDialogInfo_HasWebSite(const wxAboutDialogInfo * self);
wxString *wxAboutDialogInfo_GetWebSiteURL(const wxAboutDialogInfo * self);
wxString *wxAboutDialogInfo_GetWebSiteDescription(const wxAboutDialogInfo * self);
void wxAboutDialogInfo_SetWebSite(wxAboutDialogInfo * self, const wxString * url, const wxString * desc);
bool wxAboutDialogInfo_HasDevelopers(const wxAboutDialogInfo * self);
wxArrayString *wxAboutDialogInfo_GetDevelopers(const wxAboutDialogInfo * self);
bool wxAboutDialogInfo_HasDocWriters(const wxAboutDialogInfo * self);
wxArrayString *wxAboutDialogInfo_GetDocWriters(const wxAboutDialogInfo * self);
bool wxAboutDialogInfo_HasArtists(const wxAboutDialogInfo * self);
wxArrayString *wxAboutDialogInfo_GetArtists(const wxAboutDialogInfo * self);
bool wxAboutDialogInfo_HasTranslators(const wxAboutDialogInfo * self);
wxArrayString *wxAboutDialogInfo_GetTranslators(const wxAboutDialogInfo * self);

// CLASS: wxAcceleratorEntry
void wxAcceleratorEntry_delete(wxAcceleratorEntry *self);
wxAcceleratorEntry *wxAcceleratorEntry_new(int flags, int key_code, int cmd, wxMenuItem * item);
wxAcceleratorEntry *wxAcceleratorEntry_new1(const wxAcceleratorEntry * entry);
int wxAcceleratorEntry_GetCommand(const wxAcceleratorEntry * self);
int wxAcceleratorEntry_GetFlags(const wxAcceleratorEntry * self);
int wxAcceleratorEntry_GetKeyCode(const wxAcceleratorEntry * self);
wxMenuItem * wxAcceleratorEntry_GetMenuItem(const wxAcceleratorEntry * self);
void wxAcceleratorEntry_Set(wxAcceleratorEntry * self, int flags, int key_code, int cmd, wxMenuItem * item);
bool wxAcceleratorEntry_IsOk(const wxAcceleratorEntry * self);
wxString *wxAcceleratorEntry_ToString(const wxAcceleratorEntry * self);
wxString *wxAcceleratorEntry_ToRawString(const wxAcceleratorEntry * self);
bool wxAcceleratorEntry_FromString(wxAcceleratorEntry * self, const wxString * str);

// CLASS: wxAcceleratorTable
wxClassInfo *wxAcceleratorTable_CLASSINFO();
wxAcceleratorTable *wxAcceleratorTable_new();
bool wxAcceleratorTable_IsOk(const wxAcceleratorTable * self);

// CLASS: wxActivateEvent
wxClassInfo *wxActivateEvent_CLASSINFO();
bool wxActivateEvent_GetActive(const wxActivateEvent * self);

// CLASS: wxAffineMatrix2D
void wxAffineMatrix2D_delete(wxAffineMatrix2D *self);
wxAffineMatrix2D *wxAffineMatrix2D_new();
void wxAffineMatrix2D_Mirror(wxAffineMatrix2D * self, int direction);
void wxAffineMatrix2D_TransformPoint1(const wxAffineMatrix2D * self, wxDouble * x, wxDouble * y);
void wxAffineMatrix2D_TransformDistance1(const wxAffineMatrix2D * self, wxDouble * dx, wxDouble * dy);

// CLASS: wxAffineMatrix2DBase
void wxAffineMatrix2DBase_delete(wxAffineMatrix2DBase *self);
void wxAffineMatrix2DBase_Set(wxAffineMatrix2DBase * self, const wxMatrix2D * mat2_d, const wxPoint2DDouble * tr);
void wxAffineMatrix2DBase_Get(const wxAffineMatrix2DBase * self, wxMatrix2D * mat2_d, wxPoint2DDouble * tr);
void wxAffineMatrix2DBase_Concat(wxAffineMatrix2DBase * self, const wxAffineMatrix2DBase * t);
bool wxAffineMatrix2DBase_Invert(wxAffineMatrix2DBase * self);
bool wxAffineMatrix2DBase_IsIdentity(const wxAffineMatrix2DBase * self);
bool wxAffineMatrix2DBase_IsEqual(const wxAffineMatrix2DBase * self, const wxAffineMatrix2DBase * t);
void wxAffineMatrix2DBase_Mirror(wxAffineMatrix2DBase * self, int direction);
void wxAffineMatrix2DBase_TransformPoint1(const wxAffineMatrix2DBase * self, wxDouble * x, wxDouble * y);
void wxAffineMatrix2DBase_TransformDistance1(const wxAffineMatrix2DBase * self, wxDouble * dx, wxDouble * dy);

// CLASS: wxAnimationCtrl
wxClassInfo *wxAnimationCtrl_CLASSINFO();
wxAnimationCtrl *wxAnimationCtrl_new(wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
bool wxAnimationCtrl_Create(wxAnimationCtrl * self, wxWindow * parent, wxWindowID id, const wxAnimation * anim, const wxPoint * pos, const wxSize * size, long style, const wxString * name);
wxBitmap *wxAnimationCtrl_GetInactiveBitmap(const wxAnimationCtrl * self);
bool wxAnimationCtrl_IsPlaying(const wxAnimationCtrl * self);
bool wxAnimationCtrl_Play(wxAnimationCtrl * self);
void wxAnimationCtrl_SetAnimation(wxAnimationCtrl * self, const wxAnimation * anim);
void wxAnimationCtrl_SetInactiveBitmap(wxAnimationCtrl * self, const wxBitmapBundle * bmp);
void wxAnimationCtrl_Stop(wxAnimationCtrl * self);

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

// CLASS: wxAppProgressIndicator
void wxAppProgressIndicator_delete(wxAppProgressIndicator *self);
wxAppProgressIndicator *wxAppProgressIndicator_new(wxWindow * parent, int max_value);
bool wxAppProgressIndicator_IsAvailable(const wxAppProgressIndicator * self);
void wxAppProgressIndicator_SetValue(wxAppProgressIndicator * self, int value);
void wxAppProgressIndicator_SetRange(wxAppProgressIndicator * self, int range);

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

